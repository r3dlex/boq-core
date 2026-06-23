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
const X83_WITH_TENDER_FIELD: &str = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Bau X83</Name><BoQ><BoQBody><BoQCtgy ID="001" RNoPart="001"><Item ID="001.0010" RNoPart="10"><Qty>2.500</Qty><QU>m</QU><Description><CompleteText><DetailTxt><Text><p>Baseline trench text</p></Text></DetailTxt></CompleteText></Description><ExecutionWindow><Start>2026-07-01</Start><End>2026-08-15</End></ExecutionWindow></Item></BoQCtgy></BoQBody></BoQ></Project></GAEB>"#;
const X84_WITH_REMARK: &str = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Bau X84</Name><BoQ><BoQBody><BoQCtgy ID="001" RNoPart="001"><Item ID="001.0010" RNoPart="10"><Qty>2.500</Qty><QU>m</QU><UP>3.200</UP><IT>8.00</IT><Description><CompleteText><DetailTxt><Text><p>Changed bidder text</p></Text></DetailTxt></CompleteText></Description><BidderRemark>Includes winter surcharge</BidderRemark></Item><Item ID="009.9999" RNoPart="999"><Qty>1</Qty><QU>psch</QU><UP>99.00</UP><IT>99.00</IT></Item></BoQCtgy></BoQBody></BoQ></Project></GAEB>"#;

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
    let document = x83_document();
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert!(document.capabilities.adapt_to_obra);
    assert!(!document.capabilities.validate);
    assert!(!document.capabilities.export);
    assert!(!document.capabilities.roundtrip);

    let adapted = ObraImportDocument::from_gaeb(&document).expect("x83 adapts");
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
    let document = x84_document();
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert!(document.capabilities.adapt_to_obra);
    assert!(!document.capabilities.validate);
    assert!(!document.capabilities.export);
    assert!(!document.capabilities.roundtrip);

    let adapted = ObraImportDocument::from_gaeb(&document).expect("x84 adapts");
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
fn test_bvbs_bau_x83_x84_manifest_statuses_have_readiness_evidence() {
    let manifest = fs::read_to_string("gaeb/manifest.toml").expect("manifest exists");
    let parsed: toml::Value = toml::from_str(&manifest).expect("manifest parses");
    let fixtures = parsed
        .get("fixtures")
        .and_then(toml::Value::as_array)
        .expect("fixtures array");

    let x83 = manifest_fixture(fixtures, "bvbs_xml33_bau_x83");
    assert_eq!(field(x83, "support_status"), "supported_parse_only");
    assert_eq!(field(x83, "phase"), "x83");
    assert!(field(x83, "license_note").contains("parser-readiness"));
    assert!(field(x83, "license_note").contains("Obra adapter DTO evidence"));
    assert!(field(x83, "license_note").contains("readiness-only"));
    assert!(!field(x83, "license_note").contains("certification completed"));

    let x84 = manifest_fixture(fixtures, "bvbs_xml33_bau_x84");
    assert_eq!(field(x84, "support_status"), "supported_parse_only");
    assert!(field(x84, "license_note").contains("Obra adapter DTO evidence"));
}

#[test]
fn test_bau_x84_prices_map_by_ordinal() {
    let merged = merge_x84_offer_into_x83_baseline(&x83_document(), &x84_document());
    let item = first_item(&merged);

    assert_eq!(item.unit_price, Some(Decimal::new(3200, 3)));
    assert_eq!(item.total_price, Some(Decimal::new(800, 2)));
    assert!(item.short_text.contains("Baseline trench text"));
}

#[test]
fn test_bau_x84_missing_descriptions_resolve_against_x83_baseline() {
    let offer = parse_str(X84_WITH_REMARK, Some(X84_URI.to_owned())).expect("x84 parses");
    let merged = merge_x84_offer_into_x83_baseline(&x83_document(), &offer);
    let item = first_item(&merged);

    assert!(item.short_text.contains("Baseline trench text"));
    assert!(!item.short_text.contains("Changed bidder text"));
    assert!(merged.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_bau_x84_mutable_tender_description"
            && finding.location.as_deref() == Some("001.0010")
    }));
}

#[test]
fn test_bau_x84_bidder_remarks_preserved() {
    let offer = parse_str(X84_WITH_REMARK, Some(X84_URI.to_owned())).expect("x84 parses");
    let merged = merge_x84_offer_into_x83_baseline(&x83_document(), &offer);
    let item = first_item(&merged);

    assert_eq!(item.notes.as_deref(), Some("Includes winter surcharge"));
    assert_eq!(
        item.metadata
            .get("gaeb.bau_x84.bidder_remark")
            .and_then(serde_json::Value::as_str),
        Some("Includes winter surcharge")
    );
}

#[test]
fn test_bau_x84_unmatched_ordinal_emits_finding() {
    let offer = parse_str(X84_WITH_REMARK, Some(X84_URI.to_owned())).expect("x84 parses");
    let merged = merge_x84_offer_into_x83_baseline(&x83_document(), &offer);

    assert!(merged.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_bau_x84_extra_ordinal"
            && finding.location.as_deref() == Some("009.9999")
    }));

    let missing_offer = parse_str(
        r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><BoQ><BoQBody><Item ID="009.9999"><Qty>1</Qty></Item></BoQBody></BoQ></Project></GAEB>"#,
        Some(X84_URI.to_owned()),
    )
    .expect("sparse x84 parses");
    let missing = merge_x84_offer_into_x83_baseline(&x83_document(), &missing_offer);
    assert!(missing.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_bau_x84_missing_ordinal"
            && finding.location.as_deref() == Some("001.0010")
    }));
}

#[test]
fn test_bau_x83_extracts_project_and_boq_metadata() {
    let document = x83_document();

    assert_eq!(document.summary.version.as_deref(), Some("3.3"));
    assert_eq!(document.summary.project_name.as_deref(), Some("Bau X83"));
    assert_eq!(document.summary.title.as_deref(), Some("Bau X83"));
    assert_eq!(
        document
            .source
            .phase
            .as_ref()
            .map(|phase| phase.code.as_str()),
        Some("83")
    );
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert_eq!(
        document.capabilities,
        SupportCapabilities::parse_with_obra_adapter()
    );
    assert!(document.source.checksum.is_some());
    assert_eq!(
        document
            .boq
            .metadata
            .get("gaeb.support_policy")
            .and_then(|policy| policy.get("status"))
            .and_then(serde_json::Value::as_str),
        Some("supported_parse_only")
    );
}

#[test]
fn test_bau_x83_sections_and_items_match_hierarchy() {
    let document = x83_document();
    let section = &document.boq.nodes[0];
    let item_node = first_item_boq_node(&document);
    let item = item_node.item.as_ref().expect("item payload");

    assert_eq!(section.ordinal, "001");
    assert_eq!(section.title, "001");
    assert_eq!(section.sort_order, 0);
    assert_eq!(item_node.ordinal, "001.0010");
    assert_eq!(
        item_node
            .metadata
            .get("gaeb.rno_part")
            .and_then(serde_json::Value::as_str),
        Some("10")
    );
    assert_eq!(item_node.sort_order, 0);
    assert_eq!(item.quantity, Decimal::new(2500, 3));
    assert_eq!(item.unit, "m");
    assert_eq!(
        item.long_text.as_ref(),
        Some(&RichText::Plain("Baseline trench text".to_owned()))
    );
}

#[test]
fn test_bau_x83_tender_specific_fields_are_preserved() {
    let document = parse_str(X83_WITH_TENDER_FIELD, Some(X83_URI.to_owned()))
        .expect("x83 tender field document parses");
    let item_node = first_item_boq_node(&document);

    assert_eq!(
        item_node
            .metadata
            .get("gaeb.unsupported.ExecutionWindow")
            .and_then(serde_json::Value::as_str),
        Some("2026-07-01 2026-08-15")
    );
}

#[test]
fn test_bau_x83_unknown_nodes_emit_findings() {
    let document = parse_str(X83_WITH_TENDER_FIELD, Some(X83_URI.to_owned()))
        .expect("x83 tender field document parses");

    assert!(document.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_unsupported_item_field"
            && finding.location.as_deref() == Some("001.0010/ExecutionWindow")
    }));
}

#[test]
fn test_bau_x83_adapter_dto_is_supported_without_export_or_roundtrip() {
    let document = x83_document();
    assert!(document.capabilities.adapt_to_obra);
    assert!(!document.capabilities.validate);
    assert!(!document.capabilities.export);
    assert!(!document.capabilities.roundtrip);

    let adapted = ObraImportDocument::from_gaeb(&document)
        .expect("cataloged X83 fixture has Obra adapter DTO readiness");
    assert_eq!(adapted.line_items.len(), 1);
    assert_eq!(adapted.loss_report.unsupported_fields, Vec::<String>::new());
}

#[test]
fn test_bau_x84_adapter_dto_is_supported_without_export_or_roundtrip() {
    let document = x84_document();
    assert!(document.capabilities.adapt_to_obra);
    assert!(!document.capabilities.validate);
    assert!(!document.capabilities.export);
    assert!(!document.capabilities.roundtrip);

    let adapted = ObraImportDocument::from_gaeb(&document)
        .expect("cataloged X84 fixture has Obra adapter DTO readiness");
    assert_eq!(adapted.line_items.len(), 1);
    assert_eq!(
        adapted.line_items[0].unit_price,
        Some(Decimal::new(3200, 3))
    );
}

#[test]
fn test_bau_x83_fixture_parses_to_boq_tree() {
    let document = x83_document();
    let item = first_item(&document);

    assert_eq!(document.summary.version.as_deref(), Some("3.3"));
    assert_eq!(
        document
            .source
            .phase
            .as_ref()
            .map(|phase| phase.code.as_str()),
        Some("83")
    );
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert_eq!(
        document.capabilities,
        SupportCapabilities::parse_with_obra_adapter()
    );
    assert_eq!(document.boq.nodes[0].ordinal, "001");
    assert_eq!(document.boq.nodes[0].children[0].ordinal, "001.0010");
    assert_eq!(item.quantity, Decimal::new(2500, 3));
    assert_eq!(item.unit, "m");
    assert!(item.short_text.contains("Baseline trench text"));
}

#[test]
fn test_bau_x83_support_promotion_requires_evidence() {
    let matrix = read_criteria_matrix();
    let x83 = matrix
        .criteria
        .iter()
        .find(|criterion| criterion.id == "bau_x83_import_lv")
        .expect("x83 import criterion");
    assert_eq!(x83.evidence_kind, "automated");
    assert_eq!(
        x83.automated_test,
        "test_bau_x83_fixture_parses_to_boq_tree"
    );
    assert_eq!(x83.status, "readiness_covered");

    let manifest = fs::read_to_string("gaeb/manifest.toml").expect("manifest exists");
    let parsed: toml::Value = toml::from_str(&manifest).expect("manifest parses");
    let fixtures = parsed
        .get("fixtures")
        .and_then(toml::Value::as_array)
        .expect("fixtures array");
    let x83_fixture = manifest_fixture(fixtures, "bvbs_xml33_bau_x83");
    let mappings = x83_fixture
        .get("test_mapping")
        .and_then(toml::Value::as_array)
        .expect("x83 test mappings");
    for expected in [
        "test_bau_x83_fixture_parses_to_boq_tree",
        "test_bau_x83_adapter_dto_is_supported_without_export_or_roundtrip",
        "test_bau_x83_support_promotion_requires_evidence",
        "test_bau_x83_golden_report_matches",
    ] {
        assert!(
            mappings
                .iter()
                .any(|mapping| mapping.as_str() == Some(expected)),
            "missing x83 evidence mapping: {expected}"
        );
    }

    let x84 = manifest_fixture(fixtures, "bvbs_xml33_bau_x84");
    assert_eq!(field(x84, "support_status"), "supported_parse_only");
}

#[test]
fn test_bau_x84_support_promotion_requires_bid_evidence() {
    let matrix = read_criteria_matrix();
    let x84 = matrix
        .criteria
        .iter()
        .find(|criterion| criterion.id == "bau_x84_export_prices")
        .expect("x84 criterion");
    assert_eq!(x84.evidence_kind, "automated");
    assert_eq!(x84.automated_test, "test_bau_x84_prices_map_by_ordinal");
    assert_eq!(x84.status, "readiness_covered");

    let manifest = fs::read_to_string("gaeb/manifest.toml").expect("manifest exists");
    let parsed: toml::Value = toml::from_str(&manifest).expect("manifest parses");
    let fixtures = parsed
        .get("fixtures")
        .and_then(toml::Value::as_array)
        .expect("fixtures array");
    let x84_fixture = manifest_fixture(fixtures, "bvbs_xml33_bau_x84");
    assert_eq!(field(x84_fixture, "support_status"), "supported_parse_only");
    assert!(field(x84_fixture, "license_note").contains("bid import coverage"));
    assert!(field(x84_fixture, "license_note").contains("Obra adapter DTO evidence"));
    assert!(field(x84_fixture, "license_note").contains("readiness-only"));
    assert!(!field(x84_fixture, "license_note").contains("certification completed"));
    let mappings = x84_fixture
        .get("test_mapping")
        .and_then(toml::Value::as_array)
        .expect("x84 test mappings");
    for expected in [
        "test_bau_x84_prices_map_by_ordinal",
        "test_bau_x84_adapter_dto_is_supported_without_export_or_roundtrip",
        "test_bau_x84_missing_descriptions_resolve_against_x83_baseline",
        "test_bau_x84_bidder_remarks_preserved",
        "test_bau_x84_unmatched_ordinal_emits_finding",
        "test_bau_x84_support_promotion_requires_bid_evidence",
    ] {
        assert!(
            mappings
                .iter()
                .any(|mapping| mapping.as_str() == Some(expected)),
            "missing x84 evidence mapping: {expected}"
        );
    }
}

#[test]
fn test_bau_x83_golden_report_matches() {
    let report = fs::read_to_string("docs/fixtures/bvbs-bau-x83-readiness.md")
        .expect("x83 readiness report exists");
    for expected in [
        "Status: `supported_parse_only` readiness",
        "Manifest fixture: `bvbs_xml33_bau_x83`",
        "test_bau_x83_fixture_parses_to_boq_tree",
        "test_bau_x83_adapter_dto_is_supported_without_export_or_roundtrip",
        "readiness-only evidence",
        "DTO-readiness evidence",
        "Schema validation, export, roundtrip, and certification capabilities are not promoted",
    ] {
        assert!(
            report.contains(expected),
            "missing report evidence: {expected}"
        );
    }
    assert!(!report.contains("certification completed"));
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
    // cataloged BVBS Bau X83/X84 fixtures remain adapter-only until locked.
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

fn first_item_boq_node(document: &GaebDocument) -> &BoqNode {
    first_boq_item_node(&document.boq.nodes).expect("first item node exists")
}

fn first_boq_item_node(nodes: &[BoqNode]) -> Option<&BoqNode> {
    for node in nodes {
        if node.item.is_some() {
            return Some(node);
        }
        if let Some(item) = first_boq_item_node(&node.children) {
            return Some(item);
        }
    }
    None
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

fn manifest_fixture<'a>(fixtures: &'a [toml::Value], id: &str) -> &'a toml::value::Table {
    let fixture = fixtures
        .iter()
        .filter_map(toml::Value::as_table)
        .find(|fixture| fixture.get("id").and_then(toml::Value::as_str) == Some(id));
    assert!(fixture.is_some(), "missing manifest fixture: {id}");
    fixture.expect("fixture presence asserted")
}

fn field<'a>(table: &'a toml::value::Table, key: &str) -> &'a str {
    let value = table.get(key).and_then(toml::Value::as_str);
    assert!(value.is_some(), "missing fixture field: {key}");
    value.expect("fixture field presence asserted")
}
