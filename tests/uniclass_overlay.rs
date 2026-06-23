#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, ClassificationSystem, GaebDocument, GaebDocumentSummary,
    GaebFormat, RichText, RichTextFragment, SourceProvenance,
};
use boq_core::support::{SupportCapabilities, SupportStatus};
use boq_core::uniclass::{UniclassMappingTable, apply_classification_overlay};
use rust_decimal::Decimal;
use serde_json::json;

#[test]
fn uniclass_mapping_fixture_attaches_classification_without_support_promotion() {
    let mut document = supported_ava_document();
    let original_status = document.support_status;
    let table = mapping_table();

    let findings = apply_classification_overlay(&mut document, &table);

    assert!(findings.is_empty());
    assert_eq!(document.support_status, original_status);
    let annotations = first_item(&document)
        .try_multi_standard()
        .expect("Uniclass annotations should decode");
    assert_eq!(annotations.classifications.len(), 1);
    let classification = &annotations.classifications[0];
    assert_eq!(classification.system, ClassificationSystem::Uniclass);
    assert_eq!(classification.code, "Ss_25_10_30");
    assert_eq!(
        classification.label.as_deref(),
        Some("Synthetic concrete systems")
    );
    assert_eq!(
        classification
            .source
            .as_ref()
            .and_then(|source| source.source_uri.as_deref()),
        Some("tests/fixtures/synthetic/uniclass_mapping.json")
    );
}

#[test]
fn obra_adapter_exports_uniclass_when_adapter_is_already_supported() {
    let mut document = supported_ava_document();
    let findings = apply_classification_overlay(&mut document, &mapping_table());
    assert!(findings.is_empty());

    let import = ObraImportDocument::try_from_gaeb(&document)
        .expect("supported AVA fixture should still adapt");

    assert!(import.classifications.iter().any(|classification| {
        classification.system_code == "uniclass"
            && classification.external_code == "Ss_25_10_30"
            && classification.external_title.as_deref() == Some("Synthetic concrete systems")
    }));
}

#[test]
fn parse_only_document_remains_not_adapter_supported_after_uniclass_overlay() {
    let mut document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal D81 should parse");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);

    let findings = apply_classification_overlay(&mut document, &mapping_table());
    assert!(findings.is_empty());

    let error = ObraImportDocument::try_from_gaeb(&document)
        .expect_err("Uniclass overlay must not promote parse-only adapter support");
    assert_eq!(error.code, "obra_adapter_not_supported");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
}

#[test]
fn invalid_uniclass_fixture_variants_are_reported_explicitly() {
    let cases = [
        ("not json", "uniclass_mapping_invalid_json"),
        (
            r#"{"source_uri":"fixture","rules":[]}"#,
            "uniclass_mapping_empty",
        ),
        (
            r#"{"source_uri":"fixture","rules":[{"match_text":" ","code":"Ss_25_10_30","label":"Wall"}]}"#,
            "uniclass_mapping_empty_match",
        ),
        (
            r#"{"source_uri":"fixture","rules":[{"match_text":"Wall","code":"Ss251030","label":"Wall"}]}"#,
            "uniclass_mapping_invalid_code",
        ),
        (
            r#"{"source_uri":"fixture","rules":[{"match_text":"Wall","code":"Ss_25_10_30","label":" "}]}"#,
            "uniclass_mapping_empty_label",
        ),
    ];

    for (input, expected_code) in cases {
        let finding =
            UniclassMappingTable::from_json_str(input).expect_err("fixture should be rejected");
        assert_eq!(finding.code, expected_code);
    }
}

#[test]
fn malformed_existing_annotations_are_reported_not_hidden() {
    let mut document = supported_ava_document();
    first_item_mut(&mut document)
        .metadata
        .insert("boq_core.multi_standard".to_owned(), json!("malformed"));

    let findings = apply_classification_overlay(&mut document, &mapping_table());

    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "multi_standard_metadata_malformed")
    );
}

#[test]
fn overlay_matches_nested_rich_text_without_duplicate_classifications() {
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

    assert!(apply_classification_overlay(&mut document, &mapping_table()).is_empty());
    assert!(apply_classification_overlay(&mut document, &mapping_table()).is_empty());

    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations
            .classifications
            .iter()
            .filter(|classification| classification.system == ClassificationSystem::Uniclass)
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

    let findings = apply_classification_overlay(&mut document, &mapping_table());

    assert!(findings.is_empty());
    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(annotations.classifications[0].code, "Ss_25_10_30");
}

fn mapping_table() -> UniclassMappingTable {
    UniclassMappingTable::from_json_str(include_str!("fixtures/synthetic/uniclass_mapping.json"))
        .expect("synthetic Uniclass fixture should load")
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
            source_uri: Some("synthetic/uniclass-rich-text.x81".to_owned()),
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
            title: Some("Synthetic Uniclass".to_owned()),
            project_name: None,
        },
        boq: Boq {
            title: "Synthetic Uniclass".to_owned(),
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
            currency: Some("USD".to_owned()),
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
