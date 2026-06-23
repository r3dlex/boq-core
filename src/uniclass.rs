//! Fixture-backed Uniclass classification overlay.
//!
//! This module carries Uniclass classification evidence into the
//! canonical multi-standard annotation model. It is an overlay only: applying a
//! mapping table does not promote parser support status, does not grant Obra adapter support,
//! does not claim complete Uniclass coverage, and does not acquire external Uniclass catalog data.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::VERSION;
use crate::error::ValidationFinding;
use crate::model::{
    BoqItem, BoqNode, ClassificationReference, ClassificationSystem, GaebDocument, GaebFormat,
    MultiStandardAnnotations, ReferenceConfidence, RichText, SourceProvenance,
};

/// A fixture-backed Uniclass mapping table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniclassMappingTable {
    /// Mapping source provenance.
    pub source: SourceProvenance,
    rules: Vec<UniclassMappingRule>,
}

/// A single deterministic text-to-Uniclass mapping rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniclassMappingRule {
    /// Case-insensitive text fragment that triggers the rule.
    pub match_text: String,
    /// Uniclass table code, for example `Ss_25_10_30`.
    pub code: String,
    /// Human-readable Uniclass label.
    pub label: String,
}

#[derive(Debug, Deserialize)]
struct UniclassMappingFixture {
    source_uri: Option<String>,
    rules: Vec<UniclassMappingRuleFixture>,
}

#[derive(Debug, Deserialize)]
struct UniclassMappingRuleFixture {
    match_text: String,
    code: String,
    label: String,
}

impl UniclassMappingTable {
    /// Loads a Uniclass mapping table from a deterministic JSON fixture.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the JSON is malformed or any rule is
    /// empty/invalid.
    pub fn from_json_str(input: &str) -> Result<Self, ValidationFinding> {
        let fixture: UniclassMappingFixture = serde_json::from_str(input).map_err(|error| {
            ValidationFinding::warning(
                "uniclass_mapping_invalid_json",
                format!("Uniclass mapping fixture could not be decoded: {error}"),
            )
        })?;

        if fixture.rules.is_empty() {
            return Err(ValidationFinding::warning(
                "uniclass_mapping_empty",
                "Uniclass mapping fixture must contain at least one rule",
            ));
        }

        let mut rules = Vec::with_capacity(fixture.rules.len());
        for (index, rule) in fixture.rules.into_iter().enumerate() {
            let location = format!("rules[{index}]");
            let match_text = rule.match_text.trim();
            if match_text.is_empty() {
                return Err(ValidationFinding::warning(
                    "uniclass_mapping_empty_match",
                    "Uniclass mapping rule match_text must not be empty",
                )
                .at(location));
            }
            let code = rule.code.trim();
            if !is_valid_uniclass_code(code) {
                return Err(ValidationFinding::warning(
                    "uniclass_mapping_invalid_code",
                    "Uniclass mapping rule code must use a synthetic Uniclass table code format such as Ss_25_10_30",
                )
                .at(format!("{location}.code")));
            }
            let label = rule.label.trim();
            if label.is_empty() {
                return Err(ValidationFinding::warning(
                    "uniclass_mapping_empty_label",
                    "Uniclass mapping rule label must not be empty",
                )
                .at(format!("{location}.label")));
            }

            rules.push(UniclassMappingRule {
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
    pub fn rules(&self) -> &[UniclassMappingRule] {
        &self.rules
    }
}

/// Applies a fixture-backed Uniclass overlay to a parsed document.
///
/// The document support status and capabilities are left unchanged. Any loss
/// findings are appended to the document and also returned to the caller.
#[must_use]
pub fn apply_classification_overlay(
    document: &mut GaebDocument,
    table: &UniclassMappingTable,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    apply_to_nodes(&mut document.boq.nodes, table, &mut findings);
    document.findings.extend(findings.clone());
    findings
}

fn apply_to_nodes(
    nodes: &mut [BoqNode],
    table: &UniclassMappingTable,
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
    table: &UniclassMappingTable,
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
            system: ClassificationSystem::Uniclass,
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

fn matches_rule(text: &str, rule: &UniclassMappingRule) -> bool {
    text.to_lowercase()
        .contains(&rule.match_text.to_lowercase())
}

fn has_classification(annotations: &MultiStandardAnnotations, code: &str) -> bool {
    annotations.classifications.iter().any(|classification| {
        classification.system == ClassificationSystem::Uniclass && classification.code == code
    })
}

fn is_valid_uniclass_code(code: &str) -> bool {
    let mut parts = code.split('_');
    let Some(prefix) = parts.next() else {
        return false;
    };
    if prefix.len() != 2 || !prefix.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        return false;
    }

    let segments: Vec<&str> = parts.collect();
    (2..=5).contains(&segments.len())
        && segments
            .iter()
            .all(|segment| segment.len() == 2 && segment.bytes().all(|byte| byte.is_ascii_digit()))
}
