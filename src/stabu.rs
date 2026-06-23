//! Fixture-backed STABU / RAW exchange overlay.
//!
//! This module carries Dutch STABU classification and RAW exchange evidence into
//! the canonical multi-standard annotation model. It is an overlay only: applying
//! a table does not promote parser support status, does not grant Obra adapter support,
//! does not claim complete STABU or RAW coverage, and does not acquire external Dutch
//! catalog or exchange data. Synthetic fixture codes use `STABU-SYN-NNN` and
//! `RAW-SYN-NNN` namespaces to avoid presenting minimal test data as official content.

use std::collections::BTreeMap;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::VERSION;
use crate::error::ValidationFinding;
use crate::model::{
    BoqItem, BoqNode, CatalogSystem, ClassificationReference, ClassificationSystem, GaebDocument,
    GaebFormat, MultiStandardAnnotations, PriceCatalogReference, ReferenceConfidence, RichText,
    SourceProvenance,
};

/// A fixture-backed STABU / RAW exchange table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabuRawTable {
    /// Mapping source provenance.
    pub source: SourceProvenance,
    items: Vec<StabuRawItem>,
}

/// A deterministic text-to-STABU / RAW mapping item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabuRawItem {
    /// Case-insensitive text fragment that triggers the item.
    pub match_text: String,
    /// Synthetic STABU fixture code such as `STABU-SYN-001`.
    pub stabu_code: String,
    /// Human-readable STABU label.
    pub stabu_label: String,
    /// Synthetic RAW fixture code such as `RAW-SYN-001`.
    pub raw_code: String,
    /// Human-readable RAW exchange label.
    pub raw_label: String,
    /// Unit price evidence from the synthetic fixture.
    pub unit_price: Decimal,
    /// Three-letter currency code for the price evidence.
    pub currency: String,
    /// Unit label for the price evidence.
    pub unit: String,
    /// RAW exchange profile carried as metadata, for example `RAW2020`.
    pub exchange_profile: String,
    /// Optional loss-finding code to attach when the fixture intentionally records a gap.
    pub loss_finding_code: Option<String>,
    /// Optional loss-finding message paired with [`Self::loss_finding_code`].
    pub loss_finding_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StabuRawFixture {
    source_uri: Option<String>,
    items: Vec<StabuRawItemFixture>,
}

#[derive(Debug, Deserialize)]
struct StabuRawItemFixture {
    match_text: String,
    stabu_code: String,
    stabu_label: String,
    raw_code: String,
    raw_label: String,
    unit_price: String,
    currency: String,
    unit: String,
    exchange_profile: String,
    loss_finding_code: Option<String>,
    loss_finding_message: Option<String>,
}

impl StabuRawTable {
    /// Loads a STABU / RAW exchange table from a deterministic JSON fixture.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the JSON is malformed or any item is
    /// empty/invalid. Only synthetic fixture namespaces are accepted.
    pub fn from_json_str(input: &str) -> Result<Self, ValidationFinding> {
        let fixture: StabuRawFixture = serde_json::from_str(input).map_err(|error| {
            ValidationFinding::warning(
                "stabu_raw_invalid_json",
                format!("STABU/RAW fixture could not be decoded: {error}"),
            )
        })?;

        if fixture.items.is_empty() {
            return Err(ValidationFinding::warning(
                "stabu_raw_empty",
                "STABU/RAW fixture must contain at least one item",
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

    /// Returns the deterministic STABU / RAW items.
    #[must_use]
    pub fn items(&self) -> &[StabuRawItem] {
        &self.items
    }
}

fn parse_fixture_item(
    (index, item): (usize, StabuRawItemFixture),
) -> Result<StabuRawItem, ValidationFinding> {
    let location = format!("items[{index}]");
    let match_text = required_trimmed(
        &item.match_text,
        "stabu_raw_empty_match",
        "STABU/RAW item match_text must not be empty",
        &location,
    )?;
    let stabu_code = item.stabu_code.trim();
    if !is_valid_stabu_code(stabu_code) {
        return Err(ValidationFinding::warning(
            "stabu_raw_invalid_stabu_code",
            "STABU code must use the synthetic STABU-SYN-NNN fixture namespace",
        )
        .at(format!("{location}.stabu_code")));
    }
    let stabu_label = required_trimmed(
        &item.stabu_label,
        "stabu_raw_empty_stabu_label",
        "STABU label must not be empty",
        &format!("{location}.stabu_label"),
    )?;
    let raw_code = item.raw_code.trim();
    if !is_valid_raw_code(raw_code) {
        return Err(ValidationFinding::warning(
            "stabu_raw_invalid_raw_code",
            "RAW code must use the synthetic RAW-SYN-NNN fixture namespace",
        )
        .at(format!("{location}.raw_code")));
    }
    let raw_label = required_trimmed(
        &item.raw_label,
        "stabu_raw_empty_raw_label",
        "RAW label must not be empty",
        &format!("{location}.raw_label"),
    )?;
    let unit_price = parse_decimal(
        &item.unit_price,
        "stabu_raw_invalid_unit_price",
        "RAW unit_price must be a non-negative decimal",
        &format!("{location}.unit_price"),
    )?;
    let currency = item.currency.trim();
    if !is_valid_currency(currency) {
        return Err(ValidationFinding::warning(
            "stabu_raw_invalid_currency",
            "RAW currency must use a three-letter uppercase code",
        )
        .at(format!("{location}.currency")));
    }
    let unit = required_trimmed(
        &item.unit,
        "stabu_raw_empty_unit",
        "RAW unit must not be empty",
        &format!("{location}.unit"),
    )?;
    let exchange_profile = required_trimmed(
        &item.exchange_profile,
        "stabu_raw_empty_exchange_profile",
        "RAW exchange_profile must not be empty",
        &format!("{location}.exchange_profile"),
    )?;

    Ok(StabuRawItem {
        match_text,
        stabu_code: stabu_code.to_owned(),
        stabu_label,
        raw_code: raw_code.to_owned(),
        raw_label,
        unit_price,
        currency: currency.to_owned(),
        unit,
        exchange_profile,
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

/// Applies a fixture-backed STABU / RAW overlay to a parsed document.
///
/// The document support status and capabilities are left unchanged. Any loss
/// findings are appended to the document and also returned to the caller.
#[must_use]
pub fn apply_stabu_raw_overlay(
    document: &mut GaebDocument,
    table: &StabuRawTable,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    apply_to_nodes(&mut document.boq.nodes, table, &mut findings);
    document.findings.extend(findings.clone());
    findings
}

fn apply_to_nodes(
    nodes: &mut [BoqNode],
    table: &StabuRawTable,
    findings: &mut Vec<ValidationFinding>,
) {
    for node in nodes {
        if let Some(item) = &mut node.item {
            apply_to_item(item, table, findings);
        }
        apply_to_nodes(&mut node.children, table, findings);
    }
}

fn apply_to_item(item: &mut BoqItem, table: &StabuRawTable, findings: &mut Vec<ValidationFinding>) {
    let text = item_search_text(item);
    let mut annotations = match item.try_multi_standard() {
        Ok(value) => value,
        Err(finding) => {
            findings.push(finding);
            return;
        }
    };

    let mut changed = false;
    for (index, stabu_item) in table.items().iter().enumerate() {
        if !matches_stabu_item(&text, stabu_item) {
            continue;
        }

        let stabu_exists = has_classification(&annotations, &stabu_item.stabu_code);
        let raw_exists = has_stabu_catalog_reference(&annotations, &stabu_item.raw_code);
        if stabu_exists && raw_exists {
            continue;
        }

        if !stabu_exists {
            annotations.classifications.push(stabu_classification(
                stabu_item,
                &table.source,
                index,
            ));
            changed = true;
        }
        if !raw_exists {
            annotations
                .price_catalog_references
                .push(raw_catalog_reference(stabu_item, &table.source));
            changed = true;
        }
        if let Some(finding) = stabu_item.loss_finding()
            && !has_loss_finding(&annotations, &finding)
        {
            findings.push(finding.clone());
            annotations.loss_findings.push(finding);
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

fn stabu_classification(
    item: &StabuRawItem,
    source: &SourceProvenance,
    rule_index: usize,
) -> ClassificationReference {
    let mut metadata = BTreeMap::new();
    metadata.insert("match_text".to_owned(), serde_json::json!(item.match_text));
    metadata.insert("rule_index".to_owned(), serde_json::json!(rule_index));
    metadata.insert("raw_code".to_owned(), serde_json::json!(item.raw_code));
    metadata.insert(
        "exchange_profile".to_owned(),
        serde_json::json!(item.exchange_profile),
    );

    ClassificationReference {
        system: ClassificationSystem::Stabu,
        code: item.stabu_code.clone(),
        label: Some(item.stabu_label.clone()),
        confidence: ReferenceConfidence::Derived,
        source: Some(source.clone()),
        metadata,
    }
}

fn raw_catalog_reference(item: &StabuRawItem, source: &SourceProvenance) -> PriceCatalogReference {
    let mut metadata = BTreeMap::new();
    metadata.insert("match_text".to_owned(), serde_json::json!(item.match_text));
    metadata.insert("stabu_code".to_owned(), serde_json::json!(item.stabu_code));
    metadata.insert("unit".to_owned(), serde_json::json!(item.unit));
    metadata.insert(
        "exchange_profile".to_owned(),
        serde_json::json!(item.exchange_profile),
    );

    PriceCatalogReference {
        system: CatalogSystem::Stabu,
        code: item.raw_code.clone(),
        label: Some(item.raw_label.clone()),
        unit_price: Some(item.unit_price),
        currency: Some(item.currency.clone()),
        source: Some(source.clone()),
        metadata,
    }
}

impl StabuRawItem {
    fn loss_finding(&self) -> Option<ValidationFinding> {
        let code = self.loss_finding_code.as_ref()?;
        let message = self.loss_finding_message.as_ref().map_or_else(
            || "STABU/RAW exchange evidence was preserved as a loss finding".to_owned(),
            Clone::clone,
        );
        Some(ValidationFinding::warning(code.clone(), message).at(self.raw_code.clone()))
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

fn matches_stabu_item(text: &str, item: &StabuRawItem) -> bool {
    text.to_lowercase()
        .contains(&item.match_text.to_lowercase())
}

fn has_classification(annotations: &MultiStandardAnnotations, code: &str) -> bool {
    annotations
        .classifications
        .iter()
        .any(|reference| reference.system == ClassificationSystem::Stabu && reference.code == code)
}

fn has_stabu_catalog_reference(annotations: &MultiStandardAnnotations, code: &str) -> bool {
    annotations
        .price_catalog_references
        .iter()
        .any(|reference| reference.system == CatalogSystem::Stabu && reference.code == code)
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

fn is_valid_stabu_code(code: &str) -> bool {
    let Some(number) = code.strip_prefix("STABU-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_valid_raw_code(code: &str) -> bool {
    let Some(number) = code.strip_prefix("RAW-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_valid_currency(currency: &str) -> bool {
    currency.len() == 3 && currency.bytes().all(|byte| byte.is_ascii_uppercase())
}
