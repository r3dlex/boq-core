//! Fixture-backed SINAPI catalog and BDI overlay.
//!
//! This module carries Brazilian SINAPI price-catalog and BDI evidence into the
//! canonical multi-standard annotation model. It is an overlay only: applying a
//! catalog table does not promote parser support status, does not grant Obra adapter support,
//! does not claim complete SINAPI coverage, and does not acquire external SINAPI catalog data.
//! Synthetic fixture codes use the `SINAPI-SYN-NNN` namespace to avoid presenting
//! minimal test data as official catalog content.

use std::collections::BTreeMap;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::VERSION;
use crate::error::ValidationFinding;
use crate::model::{
    BoqItem, BoqNode, CatalogSystem, GaebDocument, GaebFormat, MultiStandardAnnotations,
    PriceCatalogReference, RichText, SourceProvenance,
};

/// A fixture-backed SINAPI catalog table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinapiCatalogTable {
    /// Catalog source provenance.
    pub source: SourceProvenance,
    items: Vec<SinapiCatalogItem>,
}

/// A single deterministic text-to-SINAPI catalog mapping item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinapiCatalogItem {
    /// Case-insensitive text fragment that triggers the item.
    pub match_text: String,
    /// Synthetic SINAPI fixture code such as `SINAPI-SYN-001`.
    pub code: String,
    /// Human-readable service label.
    pub label: String,
    /// Unit price evidence from the synthetic fixture.
    pub unit_price: Decimal,
    /// Three-letter currency code for the price evidence.
    pub currency: String,
    /// BDI percentage evidence represented as a decimal percent, for example `27.50`.
    pub bdi_percent: Decimal,
    /// Optional BDI source label carried as metadata.
    pub bdi_source: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SinapiCatalogFixture {
    source_uri: Option<String>,
    items: Vec<SinapiCatalogItemFixture>,
}

#[derive(Debug, Deserialize)]
struct SinapiCatalogItemFixture {
    match_text: String,
    code: String,
    label: String,
    unit_price: String,
    currency: String,
    bdi_percent: String,
    bdi_source: Option<String>,
}

impl SinapiCatalogTable {
    /// Loads a SINAPI catalog table from a deterministic JSON fixture.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the JSON is malformed or any item is
    /// empty/invalid. Only synthetic fixture codes are accepted.
    pub fn from_json_str(input: &str) -> Result<Self, ValidationFinding> {
        let fixture: SinapiCatalogFixture = serde_json::from_str(input).map_err(|error| {
            ValidationFinding::warning(
                "sinapi_catalog_invalid_json",
                format!("SINAPI catalog fixture could not be decoded: {error}"),
            )
        })?;

        if fixture.items.is_empty() {
            return Err(ValidationFinding::warning(
                "sinapi_catalog_empty",
                "SINAPI catalog fixture must contain at least one item",
            ));
        }

        let mut items = Vec::with_capacity(fixture.items.len());
        for (index, item) in fixture.items.into_iter().enumerate() {
            let location = format!("items[{index}]");
            let match_text = item.match_text.trim();
            if match_text.is_empty() {
                return Err(ValidationFinding::warning(
                    "sinapi_catalog_empty_match",
                    "SINAPI catalog item match_text must not be empty",
                )
                .at(location));
            }
            let code = item.code.trim();
            if !is_valid_synthetic_sinapi_code(code) {
                return Err(ValidationFinding::warning(
                    "sinapi_catalog_invalid_code",
                    "SINAPI catalog item code must use the synthetic SINAPI-SYN-NNN fixture namespace",
                )
                .at(format!("{location}.code")));
            }
            let label = item.label.trim();
            if label.is_empty() {
                return Err(ValidationFinding::warning(
                    "sinapi_catalog_empty_label",
                    "SINAPI catalog item label must not be empty",
                )
                .at(format!("{location}.label")));
            }
            let unit_price = parse_decimal(
                &item.unit_price,
                "sinapi_catalog_invalid_unit_price",
                "SINAPI catalog item unit_price must be a non-negative decimal",
                &format!("{location}.unit_price"),
            )?;
            let currency = item.currency.trim();
            if !is_valid_currency(currency) {
                return Err(ValidationFinding::warning(
                    "sinapi_catalog_invalid_currency",
                    "SINAPI catalog item currency must use a three-letter uppercase code",
                )
                .at(format!("{location}.currency")));
            }
            let bdi_percent = parse_decimal(
                &item.bdi_percent,
                "sinapi_catalog_invalid_bdi_percent",
                "SINAPI catalog item bdi_percent must be a decimal percent between 0 and 100",
                &format!("{location}.bdi_percent"),
            )?;
            if bdi_percent > Decimal::new(100, 0) {
                return Err(ValidationFinding::warning(
                    "sinapi_catalog_invalid_bdi_percent",
                    "SINAPI catalog item bdi_percent must be a decimal percent between 0 and 100",
                )
                .at(format!("{location}.bdi_percent")));
            }
            let bdi_source = item
                .bdi_source
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty());

            items.push(SinapiCatalogItem {
                match_text: match_text.to_owned(),
                code: code.to_owned(),
                label: label.to_owned(),
                unit_price,
                currency: currency.to_owned(),
                bdi_percent,
                bdi_source,
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
            items,
        })
    }

    /// Returns the deterministic catalog items.
    #[must_use]
    pub fn items(&self) -> &[SinapiCatalogItem] {
        &self.items
    }
}

/// Applies a fixture-backed SINAPI catalog and BDI overlay to a parsed document.
///
/// The document support status and capabilities are left unchanged. Any loss
/// findings are appended to the document and also returned to the caller.
#[must_use]
pub fn apply_catalog_overlay(
    document: &mut GaebDocument,
    table: &SinapiCatalogTable,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    apply_to_nodes(&mut document.boq.nodes, table, &mut findings);
    document.findings.extend(findings.clone());
    findings
}

fn apply_to_nodes(
    nodes: &mut [BoqNode],
    table: &SinapiCatalogTable,
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
    table: &SinapiCatalogTable,
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
    for (index, catalog_item) in table.items().iter().enumerate() {
        if !matches_catalog_item(&text, catalog_item)
            || has_catalog_reference(&annotations, &catalog_item.code)
        {
            continue;
        }

        let mut metadata = BTreeMap::new();
        metadata.insert(
            "match_text".to_owned(),
            serde_json::json!(catalog_item.match_text),
        );
        metadata.insert("rule_index".to_owned(), serde_json::json!(index));
        metadata.insert(
            "bdi_percent".to_owned(),
            serde_json::json!(catalog_item.bdi_percent.to_string()),
        );
        if let Some(source) = &catalog_item.bdi_source {
            metadata.insert("bdi_source".to_owned(), serde_json::json!(source));
        }

        annotations
            .price_catalog_references
            .push(PriceCatalogReference {
                system: CatalogSystem::Sinapi,
                code: catalog_item.code.clone(),
                label: Some(catalog_item.label.clone()),
                unit_price: Some(catalog_item.unit_price),
                currency: Some(catalog_item.currency.clone()),
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

fn matches_catalog_item(text: &str, item: &SinapiCatalogItem) -> bool {
    text.to_lowercase()
        .contains(&item.match_text.to_lowercase())
}

fn has_catalog_reference(annotations: &MultiStandardAnnotations, code: &str) -> bool {
    annotations
        .price_catalog_references
        .iter()
        .any(|reference| reference.system == CatalogSystem::Sinapi && reference.code == code)
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

fn is_valid_synthetic_sinapi_code(code: &str) -> bool {
    let Some(number) = code.strip_prefix("SINAPI-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_valid_currency(currency: &str) -> bool {
    currency.len() == 3 && currency.bytes().all(|byte| byte.is_ascii_uppercase())
}
