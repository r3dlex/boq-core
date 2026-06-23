#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::dqe::{DqeEstimateTable, apply_dqe_overlay};
use boq_core::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, ClassificationSystem, GaebDocument, GaebDocumentSummary,
    GaebFormat, MultiStandardAnnotations, QuantityReferenceKind, RichText, RichTextFragment,
    SourceProvenance,
};
use boq_core::support::{SupportCapabilities, SupportStatus};
use rust_decimal::Decimal;
use serde_json::json;

#[test]
fn dqe_fixture_attaches_classification_quantity_and_loss_without_support_promotion() {
    let mut document = supported_ava_document();
    let original_status = document.support_status;

    let findings = apply_dqe_overlay(&mut document, &dqe_table());

    assert_eq!(document.support_status, original_status);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "dqe_quantity_method_preserved_not_evaluated")
    );
    assert!(
        document
            .findings
            .iter()
            .any(|finding| finding.code == "dqe_quantity_method_preserved_not_evaluated")
    );

    let annotations = first_item(&document)
        .try_multi_standard()
        .expect("DQE annotations should decode");
    let classification = annotations
        .classifications
        .iter()
        .find(|reference| reference.system == ClassificationSystem::Dqe)
        .expect("DQE classification should exist");
    assert_eq!(classification.code, "DQE-SYN-001");
    assert_eq!(
        classification.label.as_deref(),
        Some("Synthetic French concrete quantity estimate")
    );
    assert_eq!(
        classification.metadata.get("quantity_reference"),
        Some(&json!("DQE-QTY-SYN-001"))
    );
    assert_eq!(
        classification.metadata.get("estimate_basis"),
        Some(&json!("avant-metre"))
    );

    let quantity = annotations
        .quantity_references
        .iter()
        .find(|reference| reference.reference == "DQE-QTY-SYN-001")
        .expect("DQE quantity reference should exist");
    assert_eq!(quantity.kind, QuantityReferenceKind::External);
    assert_eq!(quantity.quantity, Some(Decimal::new(375, 2)));
    assert_eq!(quantity.unit.as_deref(), Some("m3"));
    assert_eq!(
        quantity.metadata.get("dqe_code"),
        Some(&json!("DQE-SYN-001"))
    );
    assert_eq!(
        quantity.metadata.get("calculation_note"),
        Some(&json!("longueur * largeur * hauteur"))
    );
    assert!(
        quantity
            .findings
            .iter()
            .any(|finding| finding.code == "dqe_quantity_method_preserved_not_evaluated")
    );
    assert_eq!(annotations.loss_findings.len(), 1);
    assert_eq!(annotations.provenance.len(), 1);
}

#[test]
fn obra_adapter_exports_dqe_classification_and_preserves_quantity_metadata() {
    let mut document = supported_ava_document();
    let findings = apply_dqe_overlay(&mut document, &dqe_table());
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "dqe_quantity_method_preserved_not_evaluated")
    );

    let import = ObraImportDocument::try_from_gaeb(&document)
        .expect("supported AVA fixture should still adapt");

    assert!(import.classifications.iter().any(|classification| {
        classification.system_code == "dqe"
            && classification.external_code == "DQE-SYN-001"
            && classification.external_title.as_deref()
                == Some("Synthetic French concrete quantity estimate")
    }));
    let metadata = &import.line_items[0].metadata["boq_core.multi_standard"];
    assert_eq!(
        metadata["quantity_references"][0]["reference"],
        json!("DQE-QTY-SYN-001")
    );
    assert_eq!(
        metadata["quantity_references"][0]["quantity"],
        json!("3.75")
    );
    assert!(
        import
            .loss_report
            .warnings
            .iter()
            .any(|finding| finding.code == "dqe_quantity_method_preserved_not_evaluated")
    );
}

#[test]
fn parse_only_document_remains_not_adapter_supported_after_dqe_overlay() {
    let mut document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal D81 should parse");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);

    let _findings = apply_dqe_overlay(&mut document, &dqe_table());

    let error = ObraImportDocument::try_from_gaeb(&document)
        .expect_err("DQE overlay must not promote parse-only adapter support");
    assert_eq!(error.code, "obra_adapter_not_supported");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
}

#[test]
fn invalid_dqe_fixture_variants_are_reported_explicitly() {
    let cases = [
        ("not json", "dqe_estimate_invalid_json"),
        (
            r#"{"source_uri":"fixture","items":[]}"#,
            "dqe_estimate_empty",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":" ","dqe_code":"DQE-SYN-001","label":"Estimate","quantity_reference":"DQE-QTY-SYN-001","quantity":"3.75","unit":"m3","estimate_basis":"avant-metre"}]}"#,
            "dqe_estimate_empty_match",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","dqe_code":"BAD-001","label":"Estimate","quantity_reference":"DQE-QTY-SYN-001","quantity":"3.75","unit":"m3","estimate_basis":"avant-metre"}]}"#,
            "dqe_estimate_invalid_dqe_code",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","dqe_code":"DQE-SYN-001","label":" ","quantity_reference":"DQE-QTY-SYN-001","quantity":"3.75","unit":"m3","estimate_basis":"avant-metre"}]}"#,
            "dqe_estimate_empty_label",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","dqe_code":"DQE-SYN-001","label":"Estimate","quantity_reference":"BAD-001","quantity":"3.75","unit":"m3","estimate_basis":"avant-metre"}]}"#,
            "dqe_estimate_invalid_quantity_reference",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","dqe_code":"DQE-SYN-001","label":"Estimate","quantity_reference":"DQE-QTY-SYN-001","quantity":"abc","unit":"m3","estimate_basis":"avant-metre"}]}"#,
            "dqe_estimate_invalid_quantity",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","dqe_code":"DQE-SYN-001","label":"Estimate","quantity_reference":"DQE-QTY-SYN-001","quantity":"3.75","unit":" ","estimate_basis":"avant-metre"}]}"#,
            "dqe_estimate_empty_unit",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","dqe_code":"DQE-SYN-001","label":"Estimate","quantity_reference":"DQE-QTY-SYN-001","quantity":"3.75","unit":"m3","estimate_basis":" "}]}"#,
            "dqe_estimate_empty_basis",
        ),
    ];

    for (input, expected_code) in cases {
        let finding = DqeEstimateTable::from_json_str(input)
            .expect_err("fixture should reject invalid DQE data");
        assert_eq!(finding.code, expected_code);
    }
}

#[test]
fn overlay_completes_missing_quantity_when_dqe_classification_already_exists() {
    let mut document = supported_ava_document();
    let mut annotations = MultiStandardAnnotations::default();
    annotations
        .classifications
        .push(boq_core::model::ClassificationReference {
            system: ClassificationSystem::Dqe,
            code: "DQE-SYN-001".to_owned(),
            label: Some("Pre-existing DQE".to_owned()),
            confidence: boq_core::model::ReferenceConfidence::Derived,
            source: None,
            metadata: BTreeMap::default(),
        });
    first_item_mut(&mut document)
        .set_multi_standard(annotations)
        .expect("pre-existing DQE annotation should encode");

    let findings = apply_dqe_overlay(&mut document, &dqe_table());
    assert_eq!(findings.len(), 1);

    let annotations = first_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations
            .classifications
            .iter()
            .filter(|reference| reference.system == ClassificationSystem::Dqe)
            .count(),
        1
    );
    assert!(
        annotations
            .quantity_references
            .iter()
            .any(|reference| reference.reference == "DQE-QTY-SYN-001")
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
        unit: "m3".to_owned(),
        unit_price: None,
        total_price: None,
        notes: None,
        metadata: BTreeMap::default(),
    });

    let _ = apply_dqe_overlay(&mut document, &dqe_table());
    let _ = apply_dqe_overlay(&mut document, &dqe_table());

    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations
            .classifications
            .iter()
            .filter(|reference| reference.system == ClassificationSystem::Dqe)
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
        unit: "m3".to_owned(),
        unit_price: None,
        total_price: None,
        notes: None,
        metadata: BTreeMap::default(),
    });

    let findings = apply_dqe_overlay(&mut document, &dqe_table());

    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "dqe_quantity_method_preserved_not_evaluated")
    );
    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(annotations.classifications[0].code, "DQE-SYN-001");
}

#[test]
fn malformed_existing_annotations_are_reported_not_hidden() {
    let mut document = supported_ava_document();
    first_item_mut(&mut document)
        .metadata
        .insert("boq_core.multi_standard".to_owned(), json!("malformed"));

    let findings = apply_dqe_overlay(&mut document, &dqe_table());

    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "multi_standard_metadata_malformed")
    );
}

fn dqe_table() -> DqeEstimateTable {
    DqeEstimateTable::from_json_str(include_str!("fixtures/synthetic/dqe_quantity.json"))
        .expect("synthetic DQE fixture should load")
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
            source_uri: Some("synthetic/dqe-rich-text.x81".to_owned()),
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
            title: Some("Synthetic DQE".to_owned()),
            project_name: None,
        },
        boq: Boq {
            title: "Synthetic DQE".to_owned(),
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
