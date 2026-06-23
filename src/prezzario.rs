//! Fixture-backed Computo Metrico and Prezzario overlay.
//!
//! This module carries Italian Prezzario price-list and Computo Metrico
//! quantity evidence into the canonical multi-standard annotation model. It is
//! an overlay only: applying a table does not promote parser support status,
//! does not grant Obra adapter support, does not claim complete Prezzario or
//! Computo Metrico coverage, and does not acquire external regional catalog data.
//! Synthetic fixture codes use `PREZZARIO-SYN-NNN` and `COMPUTO-SYN-NNN` namespaces
//! to avoid presenting minimal test data as official regional content.

use std::collections::BTreeMap;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::VERSION;
use crate::error::ValidationFinding;
use crate::model::{
    BoqItem, BoqNode, CatalogSystem, GaebDocument, GaebFormat, MultiStandardAnnotations,
    PriceCatalogReference, QuantityReference, QuantityReferenceKind, RichText, SourceProvenance,
};

/// A fixture-backed Italian Prezzario/Computo table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrezzarioComputoTable {
    /// Mapping source provenance.
    pub source: SourceProvenance,
    items: Vec<PrezzarioComputoItem>,
}

/// A deterministic text-to-Prezzario and Computo mapping item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrezzarioComputoItem {
    /// Case-insensitive text fragment that triggers the item.
    pub match_text: String,
    /// Synthetic Prezzario fixture code such as `PREZZARIO-SYN-001`.
    pub prezzario_code: String,
    /// Human-readable price-list label.
    pub label: String,
    /// Unit price evidence from the synthetic fixture.
    pub unit_price: Decimal,
    /// Three-letter currency code for the price evidence.
    pub currency: String,
    /// Synthetic Computo Metrico quantity reference such as `COMPUTO-SYN-001`.
    pub quantity_reference: String,
    /// Quantity evidence from the synthetic fixture.
    pub quantity: Decimal,
    /// Unit label for the quantity evidence.
    pub unit: String,
    /// Optional formula source preserved as evidence; it is not evaluated.
    pub computo_formula: Option<String>,
    /// Optional loss-finding code to attach when the fixture intentionally records a gap.
    pub loss_finding_code: Option<String>,
    /// Optional loss-finding message paired with [`Self::loss_finding_code`].
    pub loss_finding_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PrezzarioComputoFixture {
    source_uri: Option<String>,
    items: Vec<PrezzarioComputoItemFixture>,
}

#[derive(Debug, Deserialize)]
struct PrezzarioComputoItemFixture {
    match_text: String,
    prezzario_code: String,
    label: String,
    unit_price: String,
    currency: String,
    quantity_reference: String,
    quantity: String,
    unit: String,
    computo_formula: Option<String>,
    loss_finding_code: Option<String>,
    loss_finding_message: Option<String>,
}

impl PrezzarioComputoTable {
    /// Loads a Computo Metrico/Prezzario table from a deterministic JSON fixture.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the JSON is malformed or any item is
    /// empty/invalid. Only synthetic fixture namespaces are accepted.
    pub fn from_json_str(input: &str) -> Result<Self, ValidationFinding> {
        let fixture: PrezzarioComputoFixture = serde_json::from_str(input).map_err(|error| {
            ValidationFinding::warning(
                "prezzario_computo_invalid_json",
                format!("Computo Metrico/Prezzario fixture could not be decoded: {error}"),
            )
        })?;

        if fixture.items.is_empty() {
            return Err(ValidationFinding::warning(
                "prezzario_computo_empty",
                "Computo Metrico/Prezzario fixture must contain at least one item",
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

    /// Returns the deterministic Computo Metrico/Prezzario items.
    #[must_use]
    pub fn items(&self) -> &[PrezzarioComputoItem] {
        &self.items
    }
}

fn parse_fixture_item(
    (index, item): (usize, PrezzarioComputoItemFixture),
) -> Result<PrezzarioComputoItem, ValidationFinding> {
    let location = format!("items[{index}]");
    let match_text = required_trimmed(
        &item.match_text,
        "prezzario_computo_empty_match",
        "Computo Metrico/Prezzario item match_text must not be empty",
        &location,
    )?;
    let prezzario_code = item.prezzario_code.trim();
    if !is_valid_prezzario_code(prezzario_code) {
        return Err(ValidationFinding::warning(
            "prezzario_computo_invalid_prezzario_code",
            "Prezzario item code must use the synthetic PREZZARIO-SYN-NNN fixture namespace",
        )
        .at(format!("{location}.prezzario_code")));
    }
    let label = required_trimmed(
        &item.label,
        "prezzario_computo_empty_label",
        "Prezzario item label must not be empty",
        &format!("{location}.label"),
    )?;
    let unit_price = parse_decimal(
        &item.unit_price,
        "prezzario_computo_invalid_unit_price",
        "Prezzario item unit_price must be a non-negative decimal",
        &format!("{location}.unit_price"),
    )?;
    let currency = item.currency.trim();
    if !is_valid_currency(currency) {
        return Err(ValidationFinding::warning(
            "prezzario_computo_invalid_currency",
            "Prezzario item currency must use a three-letter uppercase code",
        )
        .at(format!("{location}.currency")));
    }
    let quantity_reference = item.quantity_reference.trim();
    if !is_valid_computo_reference(quantity_reference) {
        return Err(ValidationFinding::warning(
            "prezzario_computo_invalid_quantity_reference",
            "Computo quantity reference must use the synthetic COMPUTO-SYN-NNN fixture namespace",
        )
        .at(format!("{location}.quantity_reference")));
    }
    let quantity = parse_decimal(
        &item.quantity,
        "prezzario_computo_invalid_quantity",
        "Computo quantity must be a non-negative decimal",
        &format!("{location}.quantity"),
    )?;
    let unit = required_trimmed(
        &item.unit,
        "prezzario_computo_empty_unit",
        "Computo quantity unit must not be empty",
        &format!("{location}.unit"),
    )?;

    Ok(PrezzarioComputoItem {
        match_text,
        prezzario_code: prezzario_code.to_owned(),
        label,
        unit_price,
        currency: currency.to_owned(),
        quantity_reference: quantity_reference.to_owned(),
        quantity,
        unit,
        computo_formula: optional_trimmed(item.computo_formula),
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

/// Applies a fixture-backed Computo Metrico/Prezzario overlay to a parsed document.
///
/// The document support status and capabilities are left unchanged. Any loss
/// findings are appended to the document and also returned to the caller.
#[must_use]
pub fn apply_computo_overlay(
    document: &mut GaebDocument,
    table: &PrezzarioComputoTable,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    apply_to_nodes(&mut document.boq.nodes, table, &mut findings);
    document.findings.extend(findings.clone());
    findings
}

fn apply_to_nodes(
    nodes: &mut [BoqNode],
    table: &PrezzarioComputoTable,
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
    table: &PrezzarioComputoTable,
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
    for (index, computo_item) in table.items().iter().enumerate() {
        if !matches_computo_item(&text, computo_item)
            || has_prezzario_reference(&annotations, &computo_item.prezzario_code)
            || has_quantity_reference(&annotations, &computo_item.quantity_reference)
        {
            continue;
        }

        let mut price_metadata = BTreeMap::new();
        price_metadata.insert(
            "match_text".to_owned(),
            serde_json::json!(computo_item.match_text),
        );
        price_metadata.insert("rule_index".to_owned(), serde_json::json!(index));
        price_metadata.insert(
            "quantity_reference".to_owned(),
            serde_json::json!(computo_item.quantity_reference),
        );

        annotations
            .price_catalog_references
            .push(PriceCatalogReference {
                system: CatalogSystem::Prezzario,
                code: computo_item.prezzario_code.clone(),
                label: Some(computo_item.label.clone()),
                unit_price: Some(computo_item.unit_price),
                currency: Some(computo_item.currency.clone()),
                source: Some(table.source.clone()),
                metadata: price_metadata,
            });

        let mut quantity_metadata = BTreeMap::new();
        quantity_metadata.insert(
            "match_text".to_owned(),
            serde_json::json!(computo_item.match_text),
        );
        quantity_metadata.insert(
            "prezzario_code".to_owned(),
            serde_json::json!(computo_item.prezzario_code),
        );
        if let Some(formula) = &computo_item.computo_formula {
            quantity_metadata.insert("computo_formula".to_owned(), serde_json::json!(formula));
        }

        let item_findings = computo_item.loss_finding().into_iter().collect::<Vec<_>>();
        findings.extend(item_findings.clone());
        annotations.loss_findings.extend(item_findings.clone());
        annotations.quantity_references.push(QuantityReference {
            kind: QuantityReferenceKind::Measurement,
            reference: computo_item.quantity_reference.clone(),
            quantity: Some(computo_item.quantity),
            unit: Some(computo_item.unit.clone()),
            source: Some(table.source.clone()),
            findings: item_findings,
            metadata: quantity_metadata,
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

impl PrezzarioComputoItem {
    fn loss_finding(&self) -> Option<ValidationFinding> {
        let code = self.loss_finding_code.as_ref()?;
        let message = self.loss_finding_message.as_ref().map_or_else(
            || "Computo Metrico evidence was preserved as a loss finding".to_owned(),
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

fn matches_computo_item(text: &str, item: &PrezzarioComputoItem) -> bool {
    text.to_lowercase()
        .contains(&item.match_text.to_lowercase())
}

fn has_prezzario_reference(annotations: &MultiStandardAnnotations, code: &str) -> bool {
    annotations
        .price_catalog_references
        .iter()
        .any(|reference| reference.system == CatalogSystem::Prezzario && reference.code == code)
}

fn has_quantity_reference(annotations: &MultiStandardAnnotations, reference: &str) -> bool {
    annotations
        .quantity_references
        .iter()
        .any(|quantity| quantity.reference == reference)
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

fn is_valid_prezzario_code(code: &str) -> bool {
    let Some(number) = code.strip_prefix("PREZZARIO-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_valid_computo_reference(reference: &str) -> bool {
    let Some(number) = reference.strip_prefix("COMPUTO-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_valid_currency(currency: &str) -> bool {
    currency.len() == 3 && currency.bytes().all(|byte| byte.is_ascii_uppercase())
}
