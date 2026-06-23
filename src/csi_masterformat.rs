//! Fixture-backed CSI MasterFormat classification overlay.
//!
//! This module carries CSI MasterFormat classification evidence into the
//! canonical multi-standard annotation model. It is an overlay only: applying a
//! mapping table does not promote parser support status, does not grant Obra adapter support,
//! and does not claim complete MasterFormat coverage.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::VERSION;
use crate::error::ValidationFinding;
use crate::model::{
    BoqItem, BoqNode, ClassificationReference, ClassificationSystem, GaebDocument, GaebFormat,
    MultiStandardAnnotations, ReferenceConfidence, RichText, SourceProvenance,
};

/// A fixture-backed CSI MasterFormat mapping table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterFormatMappingTable {
    /// Mapping source provenance.
    pub source: SourceProvenance,
    rules: Vec<MasterFormatMappingRule>,
}

/// A single deterministic text-to-MasterFormat mapping rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterFormatMappingRule {
    /// Case-insensitive text fragment that triggers the rule.
    pub match_text: String,
    /// MasterFormat section code, for example `03 30 00`.
    pub code: String,
    /// Human-readable MasterFormat label.
    pub label: String,
}

#[derive(Debug, Deserialize)]
struct MasterFormatMappingFixture {
    source_uri: Option<String>,
    rules: Vec<MasterFormatMappingRuleFixture>,
}

#[derive(Debug, Deserialize)]
struct MasterFormatMappingRuleFixture {
    match_text: String,
    code: String,
    label: String,
}

impl MasterFormatMappingTable {
    /// Loads a CSI MasterFormat mapping table from a deterministic JSON fixture.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the JSON is malformed or any rule is
    /// empty/invalid.
    pub fn from_json_str(input: &str) -> Result<Self, ValidationFinding> {
        let fixture: MasterFormatMappingFixture = serde_json::from_str(input).map_err(|error| {
            ValidationFinding::warning(
                "masterformat_mapping_invalid_json",
                format!("CSI MasterFormat mapping fixture could not be decoded: {error}"),
            )
        })?;

        if fixture.rules.is_empty() {
            return Err(ValidationFinding::warning(
                "masterformat_mapping_empty",
                "CSI MasterFormat mapping fixture must contain at least one rule",
            ));
        }

        let mut rules = Vec::with_capacity(fixture.rules.len());
        for (index, rule) in fixture.rules.into_iter().enumerate() {
            let location = format!("rules[{index}]");
            let match_text = rule.match_text.trim();
            if match_text.is_empty() {
                return Err(ValidationFinding::warning(
                    "masterformat_mapping_empty_match",
                    "CSI MasterFormat mapping rule match_text must not be empty",
                )
                .at(location));
            }
            let code = rule.code.trim();
            if !is_valid_masterformat_code(code) {
                return Err(ValidationFinding::warning(
                    "masterformat_mapping_invalid_code",
                    "CSI MasterFormat mapping rule code must use NN NN NN section format",
                )
                .at(format!("{location}.code")));
            }
            let label = rule.label.trim();
            if label.is_empty() {
                return Err(ValidationFinding::warning(
                    "masterformat_mapping_empty_label",
                    "CSI MasterFormat mapping rule label must not be empty",
                )
                .at(format!("{location}.label")));
            }

            rules.push(MasterFormatMappingRule {
                match_text: match_text.to_owned(),
                code: code.to_owned(),
                label: label.to_owned(),
            });
        }

        Ok(Self {
            source: SourceProvenance {
                source_uri: fixture.source_uri,
                source_format: GaebFormat::GaebXml,
                gaeb_version: None,
                phase: None,
                checksum: None,
                parser_version: VERSION.to_owned(),
            },
            rules,
        })
    }

    /// Returns the deterministic mapping rules.
    #[must_use]
    pub fn rules(&self) -> &[MasterFormatMappingRule] {
        &self.rules
    }
}

/// Applies a fixture-backed CSI MasterFormat overlay to a parsed document.
///
/// The document support status and capabilities are left unchanged. Any loss
/// findings are appended to the document and also returned to the caller.
#[must_use]
pub fn apply_classification_overlay(
    document: &mut GaebDocument,
    table: &MasterFormatMappingTable,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    apply_to_nodes(&mut document.boq.nodes, table, &mut findings);
    document.findings.extend(findings.clone());
    findings
}

fn apply_to_nodes(
    nodes: &mut [BoqNode],
    table: &MasterFormatMappingTable,
    findings: &mut Vec<ValidationFinding>,
) {
    for node in nodes {
        if let Some(item) = &mut node.item {
            apply_to_item(item, table, findings);
        }
        apply_to_nodes(&mut node.children, table, findings);
    }
}

fn apply_to_item(
    item: &mut BoqItem,
    table: &MasterFormatMappingTable,
    findings: &mut Vec<ValidationFinding>,
) {
    let text = item_search_text(item);
    let mut annotations = match item.try_multi_standard() {
        Ok(value) => value,
        Err(finding) => {
            findings.push(finding);
            return;
        }
    };

    let mut changed = false;
    for (index, rule) in table.rules().iter().enumerate() {
        if !matches_rule(&text, rule) || has_classification(&annotations, &rule.code) {
            continue;
        }

        let mut metadata = BTreeMap::new();
        metadata.insert("match_text".to_owned(), serde_json::json!(rule.match_text));
        metadata.insert("rule_index".to_owned(), serde_json::json!(index));

        annotations.classifications.push(ClassificationReference {
            system: ClassificationSystem::CsiMasterFormat,
            code: rule.code.clone(),
            label: Some(rule.label.clone()),
            confidence: ReferenceConfidence::Derived,
            source: Some(table.source.clone()),
            metadata,
        });
        if !annotations.provenance.contains(&table.source) {
            annotations.provenance.push(table.source.clone());
        }
        changed = true;
    }

    if changed {
        if let Err(finding) = item.set_multi_standard(annotations) {
            findings.push(finding);
        }
    }
}

fn item_search_text(item: &BoqItem) -> String {
    let mut text = item.short_text.clone();
    if let Some(long_text) = &item.long_text {
        text.push(' ');
        text.push_str(&rich_text_search_text(long_text));
    }
    text
}

fn rich_text_search_text(value: &RichText) -> String {
    match value {
        RichText::Plain(text) | RichText::XhtmlFragment(text) => text.clone(),
        RichText::Mixed(fragments) => fragments
            .iter()
            .map(|fragment| match fragment {
                crate::model::RichTextFragment::Text(text)
                | crate::model::RichTextFragment::Table(text)
                | crate::model::RichTextFragment::Unknown(text) => text.clone(),
                crate::model::RichTextFragment::Image { description, .. } => {
                    description.clone().unwrap_or_default()
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn matches_rule(text: &str, rule: &MasterFormatMappingRule) -> bool {
    text.to_lowercase()
        .contains(&rule.match_text.to_lowercase())
}

fn has_classification(annotations: &MultiStandardAnnotations, code: &str) -> bool {
    annotations.classifications.iter().any(|classification| {
        classification.system == ClassificationSystem::CsiMasterFormat
            && classification.code == code
    })
}

fn is_valid_masterformat_code(code: &str) -> bool {
    let bytes = code.as_bytes();
    bytes.len() == 8
        && bytes[0].is_ascii_digit()
        && bytes[1].is_ascii_digit()
        && bytes[2] == b' '
        && bytes[3].is_ascii_digit()
        && bytes[4].is_ascii_digit()
        && bytes[5] == b' '
        && bytes[6].is_ascii_digit()
        && bytes[7].is_ascii_digit()
}
