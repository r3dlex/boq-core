#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, CatalogSystem, GaebDocument, GaebDocumentSummary,
    GaebFormat, RichText, RichTextFragment, SourceProvenance,
};
use boq_core::sinapi::{SinapiCatalogTable, apply_catalog_overlay};
use boq_core::support::{SupportCapabilities, SupportStatus};
use rust_decimal::Decimal;
use serde_json::json;

#[test]
fn sinapi_fixture_attaches_catalog_and_bdi_without_support_promotion() {
    let mut document = supported_ava_document();
    let original_status = document.support_status;
    let table = catalog_table();

    let findings = apply_catalog_overlay(&mut document, &table);

    assert!(findings.is_empty());
    assert_eq!(document.support_status, original_status);
    let annotations = first_item(&document)
        .try_multi_standard()
        .expect("SINAPI annotations should decode");
    assert_eq!(annotations.price_catalog_references.len(), 1);
    let reference = &annotations.price_catalog_references[0];
    assert_eq!(reference.system, CatalogSystem::Sinapi);
    assert_eq!(reference.code, "SINAPI-SYN-001");
    assert_eq!(
        reference.label.as_deref(),
        Some("Synthetic concrete service")
    );
    assert_eq!(reference.unit_price, Some(Decimal::new(12345, 2)));
    assert_eq!(reference.currency.as_deref(), Some("BRL"));
    assert_eq!(reference.metadata.get("bdi_percent"), Some(&json!("27.50")));
    assert_eq!(
        reference.metadata.get("bdi_source"),
        Some(&json!("synthetic"))
    );
    assert_eq!(
        reference.metadata.get("match_text"),
        Some(&json!("Concrete"))
    );
    assert_eq!(annotations.provenance.len(), 1);
}

#[test]
fn obra_adapter_exports_sinapi_catalog_when_adapter_is_already_supported() {
    let mut document = supported_ava_document();
    assert!(apply_catalog_overlay(&mut document, &catalog_table()).is_empty());

    let import = ObraImportDocument::try_from_gaeb(&document)
        .expect("supported AVA fixture should still adapt");

    assert!(import.classifications.iter().any(|classification| {
        classification.system_code == "sinapi"
            && classification.external_code == "SINAPI-SYN-001"
            && classification.external_title.as_deref() == Some("Synthetic concrete service")
            && classification.unit.as_deref() == Some("m2")
            && classification.reference_price == Some(Decimal::new(12345, 2))
    }));
}

#[test]
fn parse_only_document_remains_not_adapter_supported_after_sinapi_overlay() {
    let mut document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal D81 should parse");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);

    let findings = apply_catalog_overlay(&mut document, &catalog_table());
    assert!(findings.is_empty());

    let error = ObraImportDocument::try_from_gaeb(&document)
        .expect_err("SINAPI overlay must not promote parse-only adapter support");
    assert_eq!(error.code, "obra_adapter_not_supported");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
}

#[test]
fn invalid_sinapi_fixture_variants_are_reported_explicitly() {
    let cases = [
        ("not json", "sinapi_catalog_invalid_json"),
        (
            r#"{"source_uri":"fixture","items":[]}"#,
            "sinapi_catalog_empty",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":" ","code":"SINAPI-SYN-001","label":"Service","unit_price":"123.45","currency":"BRL","bdi_percent":"27.50"}]}"#,
            "sinapi_catalog_empty_match",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","code":"SYN-001","label":"Service","unit_price":"123.45","currency":"BRL","bdi_percent":"27.50"}]}"#,
            "sinapi_catalog_invalid_code",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","code":"SINAPI-SYN-001","label":" ","unit_price":"123.45","currency":"BRL","bdi_percent":"27.50"}]}"#,
            "sinapi_catalog_empty_label",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","code":"SINAPI-SYN-001","label":"Service","unit_price":"abc","currency":"BRL","bdi_percent":"27.50"}]}"#,
            "sinapi_catalog_invalid_unit_price",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","code":"SINAPI-SYN-001","label":"Service","unit_price":"123.45","currency":"brl","bdi_percent":"27.50"}]}"#,
            "sinapi_catalog_invalid_currency",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","code":"SINAPI-SYN-001","label":"Service","unit_price":"123.45","currency":"BRL","bdi_percent":"abc"}]}"#,
            "sinapi_catalog_invalid_bdi_percent",
        ),
    ];

    for (input, expected_code) in cases {
        let finding = SinapiCatalogTable::from_json_str(input).expect_err("fixture should reject");
        assert_eq!(finding.code, expected_code);
    }
}

#[test]
fn malformed_existing_annotations_are_reported_not_hidden() {
    let mut document = supported_ava_document();
    first_item_mut(&mut document)
        .metadata
        .insert("boq_core.multi_standard".to_owned(), json!("malformed"));

    let findings = apply_catalog_overlay(&mut document, &catalog_table());

    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "multi_standard_metadata_malformed")
    );
}

#[test]
fn overlay_matches_nested_rich_text_without_duplicate_catalog_references() {
    let mut document = document_with_item(BoqItem {
        short_text: "No match in short text".to_owned(),
        long_text: Some(RichText::Mixed(vec![
            RichTextFragment::Text("Intro".to_owned()),
            RichTextFragment::Table("Concrete table cell".to_owned()),
            RichTextFragment::Image {
                id: "img-1".to_owned(),
                description: Some("Concrete image description".to_owned()),
            },
            RichTextFragment::Unknown("Concrete unknown fragment".to_owned()),
        ])),
        quantity: Decimal::ONE,
        unit: "m2".to_owned(),
        unit_price: None,
        total_price: None,
        notes: None,
        metadata: BTreeMap::default(),
    });

    assert!(apply_catalog_overlay(&mut document, &catalog_table()).is_empty());
    assert!(apply_catalog_overlay(&mut document, &catalog_table()).is_empty());

    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations
            .price_catalog_references
            .iter()
            .filter(|reference| reference.system == CatalogSystem::Sinapi)
            .count(),
        1
    );
    assert_eq!(annotations.provenance.len(), 1);
}

#[test]
fn overlay_matches_xhtml_long_text_case_insensitively() {
    let mut document = document_with_item(BoqItem {
        short_text: "No short match".to_owned(),
        long_text: Some(RichText::XhtmlFragment(
            "<p>CONCRETE XHTML evidence</p>".to_owned(),
        )),
        quantity: Decimal::ONE,
        unit: "m2".to_owned(),
        unit_price: None,
        total_price: None,
        notes: None,
        metadata: BTreeMap::default(),
    });

    let findings = apply_catalog_overlay(&mut document, &catalog_table());

    assert!(findings.is_empty());
    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations.price_catalog_references[0].code,
        "SINAPI-SYN-001"
    );
}

fn catalog_table() -> SinapiCatalogTable {
    SinapiCatalogTable::from_json_str(include_str!("fixtures/synthetic/sinapi_catalog.json"))
        .expect("synthetic SINAPI fixture should load")
}

fn supported_ava_document() -> GaebDocument {
    boq_core::gaeb_xml::parse_str(
        include_str!("fixtures/synthetic/minimal_ava.x81"),
        Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
    )
    .expect("supported AVA fixture should parse")
}

fn document_with_item(item: BoqItem) -> GaebDocument {
    GaebDocument {
        source: SourceProvenance {
            source_uri: Some("synthetic/sinapi-rich-text.x81".to_owned()),
            source_format: GaebFormat::GaebXml,
            gaeb_version: Some("3.3".to_owned()),
            phase: None,
            checksum: None,
            parser_version: boq_core::VERSION.to_owned(),
        },
        summary: GaebDocumentSummary {
            format: GaebFormat::GaebXml,
            version: Some("3.3".to_owned()),
            phase: None,
            title: Some("Synthetic SINAPI".to_owned()),
            project_name: None,
        },
        boq: Boq {
            title: "Synthetic SINAPI".to_owned(),
            nodes: vec![BoqNode {
                ordinal: "1".to_owned(),
                title: "Chapter".to_owned(),
                kind: BoqNodeKind::Chapter,
                children: vec![BoqNode {
                    ordinal: "1.1".to_owned(),
                    title: "Nested item".to_owned(),
                    kind: BoqNodeKind::Item,
                    children: Vec::new(),
                    item: Some(item),
                    sort_order: 0,
                    metadata: BTreeMap::default(),
                }],
                item: None,
                sort_order: 0,
                metadata: BTreeMap::default(),
            }],
            currency: Some("BRL".to_owned()),
            metadata: BTreeMap::default(),
        },
        capabilities: SupportCapabilities::supported_import(),
        support_status: SupportStatus::Supported,
        findings: Vec::new(),
        metadata: BTreeMap::default(),
    }
}

fn first_item(document: &GaebDocument) -> &boq_core::model::BoqItem {
    document.boq.nodes[0]
        .item
        .as_ref()
        .expect("minimal fixture should have an item")
}

fn first_item_mut(document: &mut GaebDocument) -> &mut boq_core::model::BoqItem {
    document.boq.nodes[0]
        .item
        .as_mut()
        .expect("minimal fixture should have an item")
}

fn nested_item(document: &GaebDocument) -> &boq_core::model::BoqItem {
    document.boq.nodes[0].children[0]
        .item
        .as_ref()
        .expect("synthetic nested document should have an item")
}
