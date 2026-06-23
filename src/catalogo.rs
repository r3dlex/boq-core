//! Fixture-backed Catálogo de Conceptos and Cuadro de Precios overlay.
//!
//! This module carries Spain/Mexico concept-catalog and price-table evidence into
//! the canonical multi-standard annotation model. It is an overlay only: applying
//! a table does not promote parser support status, does not grant Obra adapter support, does not claim complete Catálogo de Conceptos or Cuadro de Precios
//! coverage, and does not acquire external Spain/Mexico catalog data. Synthetic
//! fixture codes use `CONCEPTO-SYN-NNN` and `CUADRO-SYN-NNN` namespaces to avoid
//! presenting minimal test data as official national or regional content.

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

/// A fixture-backed Catálogo de Conceptos / Cuadro de Precios table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogoCuadroTable {
    /// Mapping source provenance.
    pub source: SourceProvenance,
    items: Vec<CatalogoCuadroItem>,
}

/// A deterministic text-to-concept and price-table mapping item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogoCuadroItem {
    /// Case-insensitive text fragment that triggers the item.
    pub match_text: String,
    /// Synthetic Catálogo de Conceptos fixture code such as `CONCEPTO-SYN-001`.
    pub concept_code: String,
    /// Human-readable concept catalog label.
    pub concept_label: String,
    /// Synthetic Cuadro de Precios fixture code such as `CUADRO-SYN-001`.
    pub cuadro_code: String,
    /// Human-readable price-table label.
    pub price_label: String,
    /// Unit price evidence from the synthetic fixture.
    pub unit_price: Decimal,
    /// Three-letter currency code for the price evidence.
    pub currency: String,
    /// Unit label for the price evidence.
    pub unit: String,
    /// Market scope carried as metadata, for example `ES/MX`.
    pub market_scope: String,
    /// Optional price-table family carried as metadata.
    pub price_table_kind: Option<String>,
    /// Optional loss-finding code to attach when the fixture intentionally records a gap.
    pub loss_finding_code: Option<String>,
    /// Optional loss-finding message paired with [`Self::loss_finding_code`].
    pub loss_finding_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CatalogoCuadroFixture {
    source_uri: Option<String>,
    items: Vec<CatalogoCuadroItemFixture>,
}

#[derive(Debug, Deserialize)]
struct CatalogoCuadroItemFixture {
    match_text: String,
    concept_code: String,
    concept_label: String,
    cuadro_code: String,
    price_label: String,
    unit_price: String,
    currency: String,
    unit: String,
    market_scope: String,
    price_table_kind: Option<String>,
    loss_finding_code: Option<String>,
    loss_finding_message: Option<String>,
}

impl CatalogoCuadroTable {
    /// Loads a Catálogo de Conceptos / Cuadro de Precios table from a deterministic JSON fixture.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the JSON is malformed or any item is
    /// empty/invalid. Only synthetic fixture namespaces are accepted.
    pub fn from_json_str(input: &str) -> Result<Self, ValidationFinding> {
        let fixture: CatalogoCuadroFixture = serde_json::from_str(input).map_err(|error| {
            ValidationFinding::warning(
                "catalogo_cuadro_invalid_json",
                format!("Catálogo/Cuadro fixture could not be decoded: {error}"),
            )
        })?;

        if fixture.items.is_empty() {
            return Err(ValidationFinding::warning(
                "catalogo_cuadro_empty",
                "Catálogo/Cuadro fixture must contain at least one item",
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

    /// Returns the deterministic Catálogo/Cuadro items.
    #[must_use]
    pub fn items(&self) -> &[CatalogoCuadroItem] {
        &self.items
    }
}

fn parse_fixture_item(
    (index, item): (usize, CatalogoCuadroItemFixture),
) -> Result<CatalogoCuadroItem, ValidationFinding> {
    let location = format!("items[{index}]");
    let match_text = required_trimmed(
        &item.match_text,
        "catalogo_cuadro_empty_match",
        "Catálogo/Cuadro item match_text must not be empty",
        &location,
    )?;
    let concept_code = item.concept_code.trim();
    if !is_valid_concept_code(concept_code) {
        return Err(ValidationFinding::warning(
            "catalogo_cuadro_invalid_concept_code",
            "Catálogo item code must use the synthetic CONCEPTO-SYN-NNN fixture namespace",
        )
        .at(format!("{location}.concept_code")));
    }
    let concept_label = required_trimmed(
        &item.concept_label,
        "catalogo_cuadro_empty_concept_label",
        "Catálogo item concept_label must not be empty",
        &format!("{location}.concept_label"),
    )?;
    let cuadro_code = item.cuadro_code.trim();
    if !is_valid_cuadro_code(cuadro_code) {
        return Err(ValidationFinding::warning(
            "catalogo_cuadro_invalid_cuadro_code",
            "Cuadro de Precios item code must use the synthetic CUADRO-SYN-NNN fixture namespace",
        )
        .at(format!("{location}.cuadro_code")));
    }
    let price_label = required_trimmed(
        &item.price_label,
        "catalogo_cuadro_empty_price_label",
        "Cuadro de Precios item price_label must not be empty",
        &format!("{location}.price_label"),
    )?;
    let unit_price = parse_decimal(
        &item.unit_price,
        "catalogo_cuadro_invalid_unit_price",
        "Cuadro de Precios item unit_price must be a non-negative decimal",
        &format!("{location}.unit_price"),
    )?;
    let currency = item.currency.trim();
    if !is_valid_currency(currency) {
        return Err(ValidationFinding::warning(
            "catalogo_cuadro_invalid_currency",
            "Cuadro de Precios item currency must use a three-letter uppercase code",
        )
        .at(format!("{location}.currency")));
    }
    let unit = required_trimmed(
        &item.unit,
        "catalogo_cuadro_empty_unit",
        "Cuadro de Precios item unit must not be empty",
        &format!("{location}.unit"),
    )?;
    let market_scope = required_trimmed(
        &item.market_scope,
        "catalogo_cuadro_empty_market_scope",
        "Catálogo/Cuadro item market_scope must not be empty",
        &format!("{location}.market_scope"),
    )?;

    Ok(CatalogoCuadroItem {
        match_text,
        concept_code: concept_code.to_owned(),
        concept_label,
        cuadro_code: cuadro_code.to_owned(),
        price_label,
        unit_price,
        currency: currency.to_owned(),
        unit,
        market_scope,
        price_table_kind: optional_trimmed(item.price_table_kind),
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

/// Applies a fixture-backed Catálogo de Conceptos / Cuadro de Precios overlay to a parsed document.
///
/// The document support status and capabilities are left unchanged. Any loss
/// findings are appended to the document and also returned to the caller.
#[must_use]
pub fn apply_catalogo_overlay(
    document: &mut GaebDocument,
    table: &CatalogoCuadroTable,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    apply_to_nodes(&mut document.boq.nodes, table, &mut findings);
    document.findings.extend(findings.clone());
    findings
}

fn apply_to_nodes(
    nodes: &mut [BoqNode],
    table: &CatalogoCuadroTable,
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
    table: &CatalogoCuadroTable,
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
    for (index, catalogo_item) in table.items().iter().enumerate() {
        if !matches_catalogo_item(&text, catalogo_item) {
            continue;
        }

        let concept_exists = has_catalog_reference(
            &annotations,
            &CatalogSystem::CatalogoConceptos,
            &catalogo_item.concept_code,
        );
        let cuadro_exists = has_catalog_reference(
            &annotations,
            &CatalogSystem::CuadroPrecios,
            &catalogo_item.cuadro_code,
        );
        if concept_exists && cuadro_exists {
            continue;
        }

        if !concept_exists {
            annotations.price_catalog_references.push(concept_reference(
                catalogo_item,
                &table.source,
                index,
            ));
            changed = true;
        }

        if !cuadro_exists {
            annotations
                .price_catalog_references
                .push(cuadro_reference(catalogo_item, &table.source));
            changed = true;
        }

        if let Some(finding) = catalogo_item.loss_finding()
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

fn concept_reference(
    item: &CatalogoCuadroItem,
    source: &SourceProvenance,
    rule_index: usize,
) -> PriceCatalogReference {
    let mut metadata = BTreeMap::new();
    metadata.insert("match_text".to_owned(), serde_json::json!(item.match_text));
    metadata.insert("rule_index".to_owned(), serde_json::json!(rule_index));
    metadata.insert(
        "market_scope".to_owned(),
        serde_json::json!(item.market_scope),
    );
    metadata.insert(
        "cuadro_code".to_owned(),
        serde_json::json!(item.cuadro_code),
    );

    PriceCatalogReference {
        system: CatalogSystem::CatalogoConceptos,
        code: item.concept_code.clone(),
        label: Some(item.concept_label.clone()),
        unit_price: None,
        currency: None,
        source: Some(source.clone()),
        metadata,
    }
}

fn cuadro_reference(item: &CatalogoCuadroItem, source: &SourceProvenance) -> PriceCatalogReference {
    let mut metadata = BTreeMap::new();
    metadata.insert("match_text".to_owned(), serde_json::json!(item.match_text));
    metadata.insert(
        "concept_code".to_owned(),
        serde_json::json!(item.concept_code),
    );
    metadata.insert("unit".to_owned(), serde_json::json!(item.unit));
    metadata.insert(
        "market_scope".to_owned(),
        serde_json::json!(item.market_scope),
    );
    if let Some(kind) = &item.price_table_kind {
        metadata.insert("price_table_kind".to_owned(), serde_json::json!(kind));
    }

    PriceCatalogReference {
        system: CatalogSystem::CuadroPrecios,
        code: item.cuadro_code.clone(),
        label: Some(item.price_label.clone()),
        unit_price: Some(item.unit_price),
        currency: Some(item.currency.clone()),
        source: Some(source.clone()),
        metadata,
    }
}

impl CatalogoCuadroItem {
    fn loss_finding(&self) -> Option<ValidationFinding> {
        let code = self.loss_finding_code.as_ref()?;
        let message = self.loss_finding_message.as_ref().map_or_else(
            || "Catálogo/Cuadro price-table evidence was preserved as a loss finding".to_owned(),
            Clone::clone,
        );
        Some(ValidationFinding::warning(code.clone(), message).at(self.cuadro_code.clone()))
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

fn matches_catalogo_item(text: &str, item: &CatalogoCuadroItem) -> bool {
    text.to_lowercase()
        .contains(&item.match_text.to_lowercase())
}

fn has_catalog_reference(
    annotations: &MultiStandardAnnotations,
    system: &CatalogSystem,
    code: &str,
) -> bool {
    annotations
        .price_catalog_references
        .iter()
        .any(|reference| &reference.system == system && reference.code == code)
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

fn is_valid_concept_code(code: &str) -> bool {
    let Some(number) = code.strip_prefix("CONCEPTO-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_valid_cuadro_code(code: &str) -> bool {
    let Some(number) = code.strip_prefix("CUADRO-SYN-") else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_valid_currency(currency: &str) -> bool {
    currency.len() == 3 && currency.bytes().all(|byte| byte.is_ascii_uppercase())
}
