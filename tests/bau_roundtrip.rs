#![allow(missing_docs, clippy::expect_used)]

use std::fs;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::gaeb_xml::{
    bau::merge_x84_offer_into_x83_baseline, parse_str, schema_validation_findings, write_string,
};
use boq_core::model::{BoqItem, BoqNode, GaebDocument, RichText, RichTextFragment};
use boq_core::support::{SupportCapabilities, SupportStatus};
use rust_decimal::Decimal;
use serde::Deserialize;

const X83_URI: &str = "gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/synthetic.X83";
const X84_URI: &str = "gaeb/bvbs/gaeb_xml_3_3/construction_execution/x84/synthetic.X84";

const X83: &str = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Bau X83</Name><BoQ><BoQBody><BoQCtgy ID="001" RNoPart="001"><Item ID="001.0010" RNoPart="10"><Qty>2.500</Qty><QU>m</QU><Description><CompleteText><DetailTxt><Text><p>Baseline trench text</p></Text></DetailTxt></CompleteText></Description></Item></BoQCtgy></BoQBody></BoQ></Project></GAEB>"#;
const X84: &str = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Bau X84</Name><BoQ><BoQBody><BoQCtgy ID="001" RNoPart="001"><Item ID="001.0010" RNoPart="10"><Qty>2.500</Qty><QU>m</QU><UP>3.200</UP><IT>8.00</IT><Description><CompleteText><DetailTxt><Text><p>Offer trench text</p></Text></DetailTxt></CompleteText></Description></Item></BoQCtgy></BoQBody></BoQ></Project></GAEB>"#;

#[test]
fn test_xml33_bau_x83_imports_to_rich_model_snapshot() {
    let document = x83_document();
    let item = first_item(&document);

    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert!(!document.capabilities.roundtrip);
    assert_eq!(document.summary.version.as_deref(), Some("3.3"));
    assert_eq!(item.quantity, Decimal::new(2500, 3));
    assert_eq!(item.unit, "m");
    assert!(item.short_text.contains("Baseline trench text"));
}

#[test]
fn test_xml33_bau_x84_imports_to_rich_model_snapshot() {
    let document = x84_document();
    let item = first_item(&document);

    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert!(!document.capabilities.export);
    assert_eq!(item.unit_price, Some(Decimal::new(3200, 3)));
    assert_eq!(item.total_price, Some(Decimal::new(800, 2)));
    assert!(item.short_text.contains("Offer trench text"));
}

#[test]
fn test_xml33_bau_x84_matches_x83_baseline_by_ordinal() {
    let merged = merge_x84_offer_into_x83_baseline(&x83_document(), &x84_document());
    let item = first_item(&merged);

    assert_eq!(item.unit_price, Some(Decimal::new(3200, 3)));
    assert_eq!(item.total_price, Some(Decimal::new(800, 2)));
    assert!(item.short_text.contains("Baseline trench text"));
    assert!(
        !merged
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb_xml_bau_x84_missing_ordinal")
    );
}

#[test]
fn test_xml33_bau_x84_missing_ordinal_is_nonfatal_finding() {
    let offer = parse_str(
        r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="999.9999"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
        Some(X84_URI.to_owned()),
    )
    .expect("offer should parse");

    let merged = merge_x84_offer_into_x83_baseline(&x83_document(), &offer);
    assert!(
        merged
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb_xml_bau_x84_missing_ordinal")
    );
    assert!(
        merged
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb_xml_bau_x84_extra_ordinal")
    );
}

#[test]
fn test_xml33_bau_x83_adapts_to_obra_dto_snapshot_when_supported() {
    let adapted = ObraImportDocument::from_gaeb(&synthetic_roundtrip_ready(x83_document()))
        .expect("x83 adapts");
    assert_eq!(adapted.line_items.len(), 1);
    assert_eq!(adapted.line_items[0].quantity, Decimal::new(2500, 3));
    assert!(
        adapted.line_items[0]
            .description
            .contains("Baseline trench")
    );
}

#[test]
fn test_xml33_bau_x84_adapts_to_obra_dto_snapshot_when_supported() {
    let adapted = ObraImportDocument::from_gaeb(&synthetic_roundtrip_ready(x84_document()))
        .expect("x84 adapts");
    assert_eq!(adapted.line_items.len(), 1);
    assert_eq!(
        adapted.line_items[0].unit_price,
        Some(Decimal::new(3200, 3))
    );
    assert_eq!(
        adapted.line_items[0].total_price,
        Some(Decimal::new(800, 2))
    );
}

#[test]
fn test_xml33_bau_parse_only_document_rejects_obra_adapter() {
    let document = parse_str(
        X83,
        Some("gaeb/bvbs/gaeb_xml_3_3/not_construction_execution/x83/synthetic.X83".to_owned()),
    )
    .expect("parse-only document parses");

    assert!(ObraImportDocument::from_gaeb(&document).is_err());
}

#[test]
fn test_xml33_bau_x83_exports_gaeb_xml() {
    let exported = write_string(&synthetic_roundtrip_ready(x83_document())).expect("x83 exports");
    assert!(exported.contains("<DP>83</DP>"));
    assert!(exported.contains(r#"RNoPart="10""#));
    assert!(!exported.contains(r#"RNoPart="Baseline trench text""#));
    assert!(exported.contains("Baseline trench text"));
}

#[test]
fn test_xml33_bau_x84_exports_gaeb_xml() {
    let exported = write_string(&synthetic_roundtrip_ready(x84_document())).expect("x84 exports");
    assert!(exported.contains("<DP>84</DP>"));
    assert!(exported.contains(r#"RNoPart="10""#));
    assert!(!exported.contains(r#"RNoPart="Offer trench text""#));
    assert!(exported.contains("<UP>3.2</UP>"));
}

#[test]
fn test_xml33_bau_export_rejects_mixed_rich_text_without_loss_handling() {
    let mut document = synthetic_roundtrip_ready(x83_document());
    document.boq.nodes[0].children[0]
        .item
        .as_mut()
        .expect("item")
        .long_text = Some(RichText::Mixed(vec![RichTextFragment::Text(
        "mixed fragment".to_owned(),
    )]));

    let finding = write_string(&document).expect_err("mixed rich text should not be dropped");
    assert_eq!(
        finding.code,
        "gaeb_xml_mixed_rich_text_export_not_supported"
    );
}

#[test]
fn test_xml33_bau_schema_validation_gap_is_recorded_when_tooling_unavailable() {
    let document = synthetic_roundtrip_ready(x84_document());
    let findings = schema_validation_findings(&document);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "gaeb_xml_schema_validation_tooling_unavailable")
    );
}

#[test]
fn test_xml33_bau_x83_import_export_import_preserves_hierarchy_ordinals_quantities_texts() {
    let original = synthetic_roundtrip_ready(x83_document());
    let reparsed = reparsed(&original, X83_URI);

    assert_eq!(reparsed.boq.nodes[0].ordinal, original.boq.nodes[0].ordinal);
    assert_eq!(
        reparsed.boq.nodes[0].children[0].ordinal,
        original.boq.nodes[0].children[0].ordinal
    );
    assert_eq!(
        reparsed.boq.nodes[0].children[0]
            .metadata
            .get("gaeb.rno_part")
            .and_then(serde_json::Value::as_str),
        Some("10")
    );
    assert_eq!(
        first_item(&reparsed).quantity,
        first_item(&original).quantity
    );
    assert!(first_item(&reparsed).short_text.contains("Baseline trench"));
}

#[test]
fn test_xml33_bau_x84_import_export_import_preserves_prices_and_findings() {
    let original = synthetic_roundtrip_ready(x84_document());
    let reparsed = reparsed(&original, X84_URI);

    assert_eq!(
        reparsed.boq.nodes[0].children[0]
            .metadata
            .get("gaeb.rno_part")
            .and_then(serde_json::Value::as_str),
        Some("10")
    );
    assert_eq!(
        first_item(&reparsed).unit_price,
        first_item(&original).unit_price
    );
    assert_eq!(
        first_item(&reparsed).total_price,
        first_item(&original).total_price
    );
    assert!(
        reparsed
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb_xml_description_plain_text_only")
    );
}

#[test]
fn test_xml33_bau_roundtrip_does_not_claim_byte_identical_output() {
    let document = synthetic_roundtrip_ready(x84_document());
    let exported = write_string(&document).expect("x84 exports");
    assert_ne!(exported.trim(), X84.trim());
    assert!(document.capabilities.roundtrip);
    assert!(!document.capabilities.validate);
}

#[test]
fn test_bau_criteria_matrix_maps_each_known_bvbs_check() {
    let matrix = read_criteria_matrix();
    for expected in [
        "bau_x83_import_lv",
        "bau_x84_export_prices",
        "bau_schema_checker_validation",
        "bau_visual_pdf_layout",
    ] {
        assert!(
            matrix
                .criteria
                .iter()
                .any(|criterion| criterion.id == expected),
            "missing Bau criterion: {expected}"
        );
    }
}

#[test]
fn test_bau_visual_pdf_checks_are_manual_or_gap_not_automated_claims() {
    let matrix = read_criteria_matrix();
    let visual = matrix
        .criteria
        .iter()
        .find(|criterion| criterion.id == "bau_visual_pdf_layout")
        .expect("visual criterion");
    assert_eq!(visual.evidence_kind, "manual");
    assert_eq!(visual.automated_test, "");
    assert_ne!(visual.status, "covered");
}

#[test]
fn test_bau_docs_use_readiness_not_certification_language() {
    let combined = fs::read_to_string("gaeb/criteria/bvbs_bau_matrix.toml")
        .expect("Bau criteria matrix exists");
    assert!(combined.contains("readiness"));
    assert!(!combined.contains("certified"));
    assert!(!combined.contains("certification completed"));
}

fn x83_document() -> GaebDocument {
    parse_str(X83, Some(X83_URI.to_owned())).expect("x83 parses")
}

fn x84_document() -> GaebDocument {
    parse_str(X84, Some(X84_URI.to_owned())).expect("x84 parses")
}

const fn synthetic_roundtrip_ready(mut document: GaebDocument) -> GaebDocument {
    // Synthetic in-crate writer tests opt into export capability explicitly;
    // cataloged BVBS Bau X83/X84 fixtures stay future-track until locked.
    document.support_status = SupportStatus::Supported;
    document.capabilities = SupportCapabilities::roundtrip_without_schema_validation();
    document
}

fn reparsed(document: &GaebDocument, uri: &str) -> GaebDocument {
    let exported = write_string(document).expect("document exports");
    parse_str(&exported, Some(uri.to_owned())).expect("export parses")
}

fn first_item(document: &GaebDocument) -> &BoqItem {
    first_item_node(&document.boq.nodes).expect("first item exists")
}

fn first_item_node(nodes: &[BoqNode]) -> Option<&BoqItem> {
    for node in nodes {
        if let Some(item) = &node.item {
            return Some(item);
        }
        if let Some(item) = first_item_node(&node.children) {
            return Some(item);
        }
    }
    None
}

#[derive(Deserialize)]
struct CriteriaMatrix {
    criteria: Vec<Criterion>,
}

#[derive(Deserialize)]
struct Criterion {
    id: String,
    evidence_kind: String,
    automated_test: String,
    status: String,
}

fn read_criteria_matrix() -> CriteriaMatrix {
    let text = fs::read_to_string("gaeb/criteria/bvbs_bau_matrix.toml")
        .expect("Bau criteria matrix exists");
    toml::from_str(&text).expect("Bau criteria matrix parses")
}
