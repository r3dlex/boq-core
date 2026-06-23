#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, CatalogSystem, GaebDocument, GaebDocumentSummary,
    GaebFormat, QuantityReferenceKind, RichText, RichTextFragment, SourceProvenance,
};
use boq_core::prezzario::{PrezzarioComputoTable, apply_computo_overlay};
use boq_core::support::{SupportCapabilities, SupportStatus};
use rust_decimal::Decimal;
use serde_json::json;

#[test]
fn prezzario_fixture_attaches_price_quantity_and_loss_without_support_promotion() {
    let mut document = supported_ava_document();
    let original_status = document.support_status;
    let table = computo_table();

    let findings = apply_computo_overlay(&mut document, &table);

    assert_eq!(document.support_status, original_status);
    assert!(
        findings
            .iter()
            .any(|finding| { finding.code == "prezzario_computo_formula_preserved_not_evaluated" })
    );
    assert!(
        document
            .findings
            .iter()
            .any(|finding| { finding.code == "prezzario_computo_formula_preserved_not_evaluated" })
    );

    let annotations = first_item(&document)
        .try_multi_standard()
        .expect("Prezzario annotations should decode");
    assert_eq!(annotations.price_catalog_references.len(), 1);
    let price = &annotations.price_catalog_references[0];
    assert_eq!(price.system, CatalogSystem::Prezzario);
    assert_eq!(price.code, "PREZZARIO-SYN-001");
    assert_eq!(
        price.label.as_deref(),
        Some("Synthetic Italian concrete service")
    );
    assert_eq!(price.unit_price, Some(Decimal::new(14567, 2)));
    assert_eq!(price.currency.as_deref(), Some("EUR"));
    assert_eq!(price.metadata.get("match_text"), Some(&json!("Concrete")));

    assert_eq!(annotations.quantity_references.len(), 1);
    let quantity = &annotations.quantity_references[0];
    assert_eq!(quantity.kind, QuantityReferenceKind::Measurement);
    assert_eq!(quantity.reference, "COMPUTO-SYN-001");
    assert_eq!(quantity.quantity, Some(Decimal::new(250, 2)));
    assert_eq!(quantity.unit.as_deref(), Some("m2"));
    assert_eq!(
        quantity.metadata.get("computo_formula"),
        Some(&json!("1.25 * 2"))
    );
    assert!(
        quantity
            .findings
            .iter()
            .any(|finding| { finding.code == "prezzario_computo_formula_preserved_not_evaluated" })
    );
    assert_eq!(annotations.loss_findings.len(), 1);
    assert_eq!(annotations.provenance.len(), 1);
}

#[test]
fn obra_adapter_exports_prezzario_catalog_when_adapter_is_already_supported() {
    let mut document = supported_ava_document();
    let findings = apply_computo_overlay(&mut document, &computo_table());
    assert!(
        findings
            .iter()
            .any(|finding| { finding.code == "prezzario_computo_formula_preserved_not_evaluated" })
    );

    let import = ObraImportDocument::try_from_gaeb(&document)
        .expect("supported AVA fixture should still adapt");

    assert!(import.classifications.iter().any(|classification| {
        classification.system_code == "prezzario"
            && classification.external_code == "PREZZARIO-SYN-001"
            && classification.external_title.as_deref()
                == Some("Synthetic Italian concrete service")
            && classification.unit.as_deref() == Some("m2")
            && classification.reference_price == Some(Decimal::new(14567, 2))
    }));
    assert!(
        import
            .loss_report
            .warnings
            .iter()
            .any(|finding| { finding.code == "prezzario_computo_formula_preserved_not_evaluated" })
    );
}

#[test]
fn parse_only_document_remains_not_adapter_supported_after_prezzario_overlay() {
    let mut document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal D81 should parse");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);

    let _findings = apply_computo_overlay(&mut document, &computo_table());

    let error = ObraImportDocument::try_from_gaeb(&document)
        .expect_err("Prezzario overlay must not promote parse-only adapter support");
    assert_eq!(error.code, "obra_adapter_not_supported");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
}

#[test]
fn invalid_prezzario_fixture_variants_are_reported_explicitly() {
    let cases = [
        ("not json", "prezzario_computo_invalid_json"),
        (
            r#"{"source_uri":"fixture","items":[]}"#,
            "prezzario_computo_empty",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":" ","prezzario_code":"PREZZARIO-SYN-001","label":"Service","unit_price":"145.67","currency":"EUR","quantity_reference":"COMPUTO-SYN-001","quantity":"2.50","unit":"m2"}]}"#,
            "prezzario_computo_empty_match",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","prezzario_code":"P-001","label":"Service","unit_price":"145.67","currency":"EUR","quantity_reference":"COMPUTO-SYN-001","quantity":"2.50","unit":"m2"}]}"#,
            "prezzario_computo_invalid_prezzario_code",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","prezzario_code":"PREZZARIO-SYN-001","label":" ","unit_price":"145.67","currency":"EUR","quantity_reference":"COMPUTO-SYN-001","quantity":"2.50","unit":"m2"}]}"#,
            "prezzario_computo_empty_label",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","prezzario_code":"PREZZARIO-SYN-001","label":"Service","unit_price":"abc","currency":"EUR","quantity_reference":"COMPUTO-SYN-001","quantity":"2.50","unit":"m2"}]}"#,
            "prezzario_computo_invalid_unit_price",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","prezzario_code":"PREZZARIO-SYN-001","label":"Service","unit_price":"145.67","currency":"eur","quantity_reference":"COMPUTO-SYN-001","quantity":"2.50","unit":"m2"}]}"#,
            "prezzario_computo_invalid_currency",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","prezzario_code":"PREZZARIO-SYN-001","label":"Service","unit_price":"145.67","currency":"EUR","quantity_reference":"BAD-001","quantity":"2.50","unit":"m2"}]}"#,
            "prezzario_computo_invalid_quantity_reference",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","prezzario_code":"PREZZARIO-SYN-001","label":"Service","unit_price":"145.67","currency":"EUR","quantity_reference":"COMPUTO-SYN-001","quantity":"abc","unit":"m2"}]}"#,
            "prezzario_computo_invalid_quantity",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","prezzario_code":"PREZZARIO-SYN-001","label":"Service","unit_price":"145.67","currency":"EUR","quantity_reference":"COMPUTO-SYN-001","quantity":"2.50","unit":" "}]}"#,
            "prezzario_computo_empty_unit",
        ),
    ];

    for (input, expected_code) in cases {
        let finding = PrezzarioComputoTable::from_json_str(input)
            .expect_err("fixture should reject invalid computo/prezzario data");
        assert_eq!(finding.code, expected_code);
    }
}

#[test]
fn malformed_existing_annotations_are_reported_not_hidden() {
    let mut document = supported_ava_document();
    first_item_mut(&mut document)
        .metadata
        .insert("boq_core.multi_standard".to_owned(), json!("malformed"));

    let findings = apply_computo_overlay(&mut document, &computo_table());

    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "multi_standard_metadata_malformed")
    );
}

#[test]
fn overlay_matches_nested_rich_text_without_duplicate_references() {
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

    let _ = apply_computo_overlay(&mut document, &computo_table());
    let _ = apply_computo_overlay(&mut document, &computo_table());

    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations
            .price_catalog_references
            .iter()
            .filter(|reference| reference.system == CatalogSystem::Prezzario)
            .count(),
        1
    );
    assert_eq!(annotations.quantity_references.len(), 1);
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

    let findings = apply_computo_overlay(&mut document, &computo_table());

    assert!(
        findings
            .iter()
            .any(|finding| { finding.code == "prezzario_computo_formula_preserved_not_evaluated" })
    );
    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations.price_catalog_references[0].code,
        "PREZZARIO-SYN-001"
    );
}

fn computo_table() -> PrezzarioComputoTable {
    PrezzarioComputoTable::from_json_str(include_str!("fixtures/synthetic/prezzario_computo.json"))
        .expect("synthetic Prezzario/Computo fixture should load")
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
            source_uri: Some("synthetic/prezzario-computo-rich-text.x81".to_owned()),
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
            title: Some("Synthetic Computo Metrico".to_owned()),
            project_name: None,
        },
        boq: Boq {
            title: "Synthetic Computo Metrico".to_owned(),
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
            currency: Some("EUR".to_owned()),
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
