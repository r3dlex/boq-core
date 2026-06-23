//! Fixture-backed DQE French quantity estimate overlay.
//!
//! This module carries French DQE (Détail Quantitatif Estimatif) quantity-estimate
//! evidence into the canonical multi-standard annotation model. It is an overlay only:
//! applying a table does not promote parser support status, does not grant Obra adapter support, does not claim complete DQE coverage, and does not acquire external French
//! catalog or quantity-estimate data. Synthetic fixture codes use `DQE-SYN-NNN` and
//! `DQE-QTY-SYN-NNN` namespaces to avoid presenting minimal test data as official content.

use std::collections::BTreeMap;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::VERSION;
use crate::error::ValidationFinding;
use crate::model::{
    BoqItem, BoqNode, ClassificationReference, ClassificationSystem, GaebDocument, GaebFormat,
    MultiStandardAnnotations, QuantityReference, QuantityReferenceKind, ReferenceConfidence,
    RichText, SourceProvenance,
};

/// A fixture-backed DQE quantity-estimate table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DqeEstimateTable {
    /// Mapping source provenance.
    pub source: SourceProvenance,
    items: Vec<DqeEstimateItem>,
}

/// A deterministic text-to-DQE quantity mapping item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DqeEstimateItem {
    /// Case-insensitive text fragment that triggers the item.
    pub match_text: String,
    /// Synthetic DQE fixture code such as `DQE-SYN-001`.
    pub dqe_code: String,
    /// Human-readable DQE label.
    pub label: String,
    /// Synthetic quantity reference such as `DQE-QTY-SYN-001`.
    pub quantity_reference: String,
    /// Quantity evidence from the synthetic fixture.
    pub quantity: Decimal,
    /// Unit label for the quantity evidence.
    pub unit: String,
    /// DQE estimate basis carried as metadata, for example `avant-metre`.
    pub estimate_basis: String,
    /// Optional calculation note preserved as evidence; it is not evaluated.
    pub calculation_note: Option<String>,
    /// Optional loss-finding code to attach when the fixture intentionally records a gap.
    pub loss_finding_code: Option<String>,
    /// Optional loss-finding message paired with [`Self::loss_finding_code`].
    pub loss_finding_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DqeEstimateFixture {
    source_uri: Option<String>,
    items: Vec<DqeEstimateItemFixture>,
}

#[derive(Debug, Deserialize)]
struct DqeEstimateItemFixture {
    match_text: String,
    dqe_code: String,
    label: String,
    quantity_reference: String,
    quantity: String,
    unit: String,
    estimate_basis: String,
    calculation_note: Option<String>,
    loss_finding_code: Option<String>,
    loss_finding_message: Option<String>,
}

impl DqeEstimateTable {
    /// Loads a DQE quantity-estimate table from a deterministic JSON fixture.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the JSON is malformed or any item is
    /// empty/invalid. Only synthetic fixture namespaces are accepted.
    pub fn from_json_str(input: &str) -> Result<Self, ValidationFinding> {
        let fixture: DqeEstimateFixture = serde_json::from_str(input).map_err(|error| {
            ValidationFinding::warning(
                "dqe_estimate_invalid_json",
                format!("DQE fixture could not be decoded: {error}"),
            )
        })?;

        if fixture.items.is_empty() {
            return Err(ValidationFinding::warning(
                "dqe_estimate_empty",
                "DQE fixture must contain at least one item",
            ));
        }

        let items = fixture
            .items
            .into_iter()
            .enumerate()
            .map(parse_fixture_item)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            source: SourceProvenance {
                source_uri: fixture.source_uri,
                source_format: GaebFormat::GaebXml,
                gaeb_version: None,
                phase: None,
                checksum: None,
                parser_version: VERSION.to_owned(),
            },
            items,
        })
    }

    /// Returns the deterministic DQE items.
    #[must_use]
    pub fn items(&self) -> &[DqeEstimateItem] {
        &self.items
    }
}

fn parse_fixture_item(
    (index, item): (usize, DqeEstimateItemFixture),
) -> Result<DqeEstimateItem, ValidationFinding> {
    let location = format!("items[{index}]");
    let match_text = required_trimmed(
        &item.match_text,
        "dqe_estimate_empty_match",
        "DQE item match_text must not be empty",
        &location,
    )?;
    let dqe_code = item.dqe_code.trim();
    if !is_valid_dqe_code(dqe_code) {
        return Err(ValidationFinding::warning(
            "dqe_estimate_invalid_dqe_code",
            "DQE code must use the synthetic DQE-SYN-NNN fixture namespace",
        )
        .at(format!("{location}.dqe_code")));
    }
    let label = required_trimmed(
        &item.label,
        "dqe_estimate_empty_label",
        "DQE label must not be empty",
        &format!("{location}.label"),
    )?;
    let quantity_reference = item.quantity_reference.trim();
    if !is_valid_quantity_reference(quantity_reference) {
        return Err(ValidationFinding::warning(
            "dqe_estimate_invalid_quantity_reference",
            "DQE quantity reference must use the synthetic DQE-QTY-SYN-NNN fixture namespace",
        )
        .at(format!("{location}.quantity_reference")));
    }
    let quantity = parse_decimal(
        &item.quantity,
        "dqe_estimate_invalid_quantity",
        "DQE quantity must be a non-negative decimal",
        &format!("{location}.quantity"),
    )?;
    let unit = required_trimmed(
        &item.unit,
        "dqe_estimate_empty_unit",
        "DQE quantity unit must not be empty",
        &format!("{location}.unit"),
    )?;
    let estimate_basis = required_trimmed(
        &item.estimate_basis,
        "dqe_estimate_empty_basis",
        "DQE estimate_basis must not be empty",
        &format!("{location}.estimate_basis"),
    )?;

    Ok(DqeEstimateItem {
        match_text,
        dqe_code: dqe_code.to_owned(),
        label,
        quantity_reference: quantity_reference.to_owned(),
        quantity,
        unit,
        estimate_basis,
        calculation_note: optional_trimmed(item.calculation_note),
        loss_finding_code: optional_trimmed(item.loss_finding_code),
        loss_finding_message: optional_trimmed(item.loss_finding_message),
    })
}

fn required_trimmed(
    value: &str,
    code: &'static str,
    message: &'static str,
    location: &str,
) -> Result<String, ValidationFinding> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationFinding::warning(code, message.to_owned()).at(location.to_owned()));
    }
    Ok(value.to_owned())
}

fn optional_trimmed(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

/// Applies a fixture-backed DQE overlay to a parsed document.
///
/// The document support status and capabilities are left unchanged. Any loss
/// findings are appended to the document and also returned to the caller.
#[must_use]
pub fn apply_dqe_overlay(
    document: &mut GaebDocument,
    table: &DqeEstimateTable,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    apply_to_nodes(&mut document.boq.nodes, table, &mut findings);
    document.findings.extend(findings.clone());
    findings
}

fn apply_to_nodes(
    nodes: &mut [BoqNode],
    table: &DqeEstimateTable,
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
    table: &DqeEstimateTable,
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
    for (index, dqe_item) in table.items().iter().enumerate() {
        if !matches_dqe_item(&text, dqe_item) {
            continue;
        }

        let classification_exists = has_dqe_classification(&annotations, &dqe_item.dqe_code);
        let quantity_exists = has_quantity_reference(&annotations, &dqe_item.quantity_reference);
        if classification_exists && quantity_exists {
            continue;
        }

        if !classification_exists {
            annotations
                .classifications
                .push(dqe_classification(dqe_item, &table.source, index));
            changed = true;
        }

        let item_findings = dqe_item.loss_finding().into_iter().collect::<Vec<_>>();
        if quantity_exists {
            for finding in item_findings {
                if !has_loss_finding(&annotations, &finding) {
                    findings.push(finding.clone());
                    annotations.loss_findings.push(finding);
                    changed = true;
                }
            }
        } else {
            findings.extend(item_findings.clone());
            for finding in &item_findings {
                if !has_loss_finding(&annotations, finding) {
                    annotations.loss_findings.push(finding.clone());
                }
            }
            annotations.quantity_references.push(quantity_reference(
                dqe_item,
                &table.source,
                item_findings,
            ));
            changed = true;
        }

        if !annotations.provenance.contains(&table.source) {
            annotations.provenance.push(table.source.clone());
            changed = true;
        }
    }

    if changed {
        if let Err(finding) = item.set_multi_standard(annotations) {
            findings.push(finding);
        }
    }
}

fn dqe_classification(
    item: &DqeEstimateItem,
    source: &SourceProvenance,
    rule_index: usize,
) -> ClassificationReference {
    let mut metadata = BTreeMap::new();
    metadata.insert("match_text".to_owned(), serde_json::json!(item.match_text));
    metadata.insert("rule_index".to_owned(), serde_json::json!(rule_index));
    metadata.insert(
        "quantity_reference".to_owned(),
        serde_json::json!(item.quantity_reference),
    );
    metadata.insert(
        "estimate_basis".to_owned(),
        serde_json::json!(item.estimate_basis),
    );

    ClassificationReference {
        system: ClassificationSystem::Dqe,
        code: item.dqe_code.clone(),
        label: Some(item.label.clone()),
        confidence: ReferenceConfidence::Derived,
        source: Some(source.clone()),
        metadata,
    }
}

fn quantity_reference(
    item: &DqeEstimateItem,
    source: &SourceProvenance,
    findings: Vec<ValidationFinding>,
) -> QuantityReference {
    let mut metadata = BTreeMap::new();
    metadata.insert("match_text".to_owned(), serde_json::json!(item.match_text));
    metadata.insert("dqe_code".to_owned(), serde_json::json!(item.dqe_code));
    metadata.insert(
        "estimate_basis".to_owned(),
        serde_json::json!(item.estimate_basis),
    );
    if let Some(note) = &item.calculation_note {
        metadata.insert("calculation_note".to_owned(), serde_json::json!(note));
    }

    QuantityReference {
        kind: QuantityReferenceKind::External,
        reference: item.quantity_reference.clone(),
        quantity: Some(item.quantity),
        unit: Some(item.unit.clone()),
        source: Some(source.clone()),
        findings,
        metadata,
    }
}

impl DqeEstimateItem {
    fn loss_finding(&self) -> Option<ValidationFinding> {
        let code = self.loss_finding_code.as_ref()?;
        let message = self.loss_finding_message.as_ref().map_or_else(
            || "DQE quantity-estimate evidence was preserved as a loss finding".to_owned(),
            Clone::clone,
        );
        Some(ValidationFinding::warning(code.clone(), message).at(self.quantity_reference.clone()))
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

fn matches_dqe_item(text: &str, item: &DqeEstimateItem) -> bool {
    text.to_lowercase()
        .contains(&item.match_text.to_lowercase())
}

fn has_dqe_classification(annotations: &MultiStandardAnnotations, code: &str) -> bool {
    annotations
        .classifications
        .iter()
        .any(|reference| reference.system == ClassificationSystem::Dqe && reference.code == code)
}

fn has_quantity_reference(annotations: &MultiStandardAnnotations, reference: &str) -> bool {
    annotations
        .quantity_references
        .iter()
        .any(|quantity| quantity.reference == reference)
}

fn has_loss_finding(annotations: &MultiStandardAnnotations, finding: &ValidationFinding) -> bool {
    annotations
        .loss_findings
        .iter()
        .any(|existing| existing.code == finding.code && existing.location == finding.location)
}

fn parse_decimal(
    value: &str,
    code: &'static str,
    message: &'static str,
    location: &str,
) -> Result<Decimal, ValidationFinding> {
    let parsed = Decimal::from_str(value.trim()).map_err(|_| {
        ValidationFinding::warning(code, message.to_owned()).at(location.to_owned())
    })?;
    if parsed < Decimal::ZERO {
        return Err(ValidationFinding::warning(code, message.to_owned()).at(location.to_owned()));
    }
    Ok(parsed)
}

fn is_valid_dqe_code(code: &str) -> bool {
    let Some(number) = code.strip_prefix("DQE-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_valid_quantity_reference(reference: &str) -> bool {
    let Some(number) = reference.strip_prefix("DQE-QTY-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}
