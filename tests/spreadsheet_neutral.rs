#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, GaebDocument, GaebDocumentSummary, GaebFormat, RichText,
    SourceProvenance,
};
use boq_core::spreadsheet::{apply_neutral_csv_updates, export_neutral_csv};
use boq_core::support::{SupportCapabilities, SupportStatus};
use rust_decimal::Decimal;

#[test]
fn neutral_csv_export_carries_provenance_support_and_loss_without_support_promotion() {
    let mut document = supported_document();
    document
        .findings
        .push(boq_core::error::ValidationFinding::warning(
            "synthetic_warning",
            "synthetic warning",
        ));
    let original_status = document.support_status;

    let sheet = export_neutral_csv(&document);

    assert_eq!(document.support_status, original_status);
    assert_eq!(
        sheet.provenance.source_uri.as_deref(),
        Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81")
    );
    assert!(sheet.csv.starts_with("oz,short_text,quantity,unit"));
    assert!(sheet.csv.contains("001.0010,Concrete slab,12.5,m2"));
    assert!(sheet.csv.contains("support_status=supported"));
    assert!(sheet.csv.contains("synthetic_warning"));
    assert_eq!(sheet.findings.len(), 1);
    assert_eq!(
        sheet
            .metadata
            .get("roundtrip_contract")
            .and_then(|value| value.as_str()),
        Some("oz_matched_csv_neutral")
    );
}

#[test]
fn neutral_csv_update_matches_by_oz_with_reordered_and_inserted_columns() {
    let mut document = supported_document();
    let original_status = document.support_status;
    let csv =
        "Kommentar,unit,quantity,oz,extra,total_price\nmanual,m2,12.50,001.0010,ignore,150.00\n";

    let findings = apply_neutral_csv_updates(&mut document, csv).expect("OZ-matched update works");

    assert_eq!(document.support_status, original_status);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "spreadsheet_neutral_update_applied")
    );
    let item = first_item(&document);
    assert_eq!(item.quantity, Decimal::new(1250, 2));
    assert_eq!(item.unit, "m2");
    assert_eq!(item.total_price, Some(Decimal::new(15000, 2)));
    assert_eq!(item.short_text, "Concrete slab");
    assert!(
        document
            .findings
            .iter()
            .any(|finding| finding.code == "spreadsheet_neutral_ignored_column")
    );
}

#[test]
fn neutral_csv_update_rejects_missing_or_empty_oz_without_row_order_guessing() {
    let mut document = supported_document();
    let missing = apply_neutral_csv_updates(&mut document, "quantity,unit\n12.5,m3\n")
        .expect_err("missing OZ column must reject");
    assert_eq!(missing.code, "spreadsheet_neutral_missing_oz_column");

    let empty = apply_neutral_csv_updates(&mut document, "oz,quantity,unit\n ,12.5,m3\n")
        .expect_err("empty OZ value must reject");
    assert_eq!(empty.code, "spreadsheet_neutral_missing_oz_value");

    assert_eq!(first_item(&document).quantity, Decimal::new(1250, 2));
}

#[test]
fn neutral_csv_update_records_unmatched_or_invalid_rows_as_findings() {
    let mut document = supported_document();
    let csv = "oz,quantity,unit_price,total_price,short_text\nmissing,abc,def,ghi,No item\n";

    let findings =
        apply_neutral_csv_updates(&mut document, csv).expect("row findings are nonfatal");

    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "spreadsheet_neutral_unmatched_oz")
    );
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "spreadsheet_neutral_invalid_decimal")
    );
    assert_eq!(first_item(&document).short_text, "Concrete slab");
}

#[test]
fn neutral_csv_roundtrip_preserves_quoted_text_and_metadata() {
    let mut document = document_with_item(BoqItem {
        short_text: "Concrete, quoted \"work\"".to_owned(),
        long_text: Some(RichText::Plain("Long text".to_owned())),
        quantity: Decimal::new(250, 2),
        unit: "m3".to_owned(),
        unit_price: Some(Decimal::new(1000, 2)),
        total_price: Some(Decimal::new(2500, 2)),
        notes: None,
        metadata: BTreeMap::default(),
    });

    let sheet = export_neutral_csv(&document);
    assert!(sheet.csv.contains("\"Concrete, quoted \"\"work\"\"\""));

    let updates = "oz,short_text,quantity,unit\n1.1,\"Updated, quoted \"\"text\"\"\",3.00,m2\n";
    let findings = apply_neutral_csv_updates(&mut document, updates).expect("quoted CSV parses");
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "spreadsheet_neutral_update_applied")
    );
    assert_eq!(
        nested_item(&document).short_text,
        "Updated, quoted \"text\""
    );
    assert_eq!(nested_item(&document).quantity, Decimal::new(300, 2));
    assert_eq!(nested_item(&document).unit, "m2");
}

#[test]
fn parse_only_document_is_not_promoted_by_neutral_csv_export_or_update() {
    let mut document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal D81 should parse");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);

    let sheet = export_neutral_csv(&document);
    assert!(sheet.csv.contains("support_status=supported_parse_only"));
    let _ = apply_neutral_csv_updates(&mut document, "oz,quantity\n01,2.00\n");

    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    let error = ObraImportDocument::try_from_gaeb(&document)
        .expect_err("neutral spreadsheet helper must not grant Obra adapter support");
    assert_eq!(error.code, "obra_adapter_not_supported");
}

#[test]
fn neutral_csv_rejects_malformed_rows_and_duplicate_headers() {
    let mut document = supported_document();
    let duplicate = apply_neutral_csv_updates(&mut document, "oz,quantity,oz\n1,2,3\n")
        .expect_err("duplicate headers reject deterministic matching");
    assert_eq!(duplicate.code, "spreadsheet_neutral_duplicate_header");

    let malformed = apply_neutral_csv_updates(&mut document, "oz,quantity\n\"unterminated,1\n")
        .expect_err("malformed CSV rejects");
    assert_eq!(malformed.code, "spreadsheet_neutral_invalid_csv");
}

fn supported_document() -> GaebDocument {
    boq_core::gaeb_xml::parse_str(
        include_str!("fixtures/synthetic/minimal_ava.x81"),
        Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
    )
    .expect("supported AVA fixture should parse")
}

fn document_with_item(item: BoqItem) -> GaebDocument {
    GaebDocument {
        source: SourceProvenance {
            source_uri: Some("synthetic/spreadsheet-neutral.x81".to_owned()),
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
            title: Some("Synthetic spreadsheet neutral".to_owned()),
            project_name: None,
        },
        boq: Boq {
            title: "Synthetic spreadsheet neutral".to_owned(),
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

fn nested_item(document: &GaebDocument) -> &boq_core::model::BoqItem {
    document.boq.nodes[0].children[0]
        .item
        .as_ref()
        .expect("synthetic nested document should have an item")
}
