#![allow(missing_docs, clippy::expect_used)]

use std::{collections::BTreeSet, fs, process::Command};

use boq_core::service_export_boundary::{EXPORT_BOUNDARY_SCHEMA_VERSION, export_boundary_report};
use boq_core::spreadsheet::{apply_neutral_csv_updates, export_neutral_csv};
use boq_core::support::SupportStatus;

#[test]
fn export_boundary_report_separates_neutral_csv_from_blocked_formats() {
    let report = export_boundary_report();

    assert_eq!(report.schema_version, EXPORT_BOUNDARY_SCHEMA_VERSION);
    assert_eq!(report.neutral_csv.contract_key, "oz_matched_csv_neutral");
    assert!(report.neutral_csv.implemented);
    assert!(report.neutral_csv.export_supported);
    assert!(report.neutral_csv.update_supported);
    assert_eq!(report.neutral_csv.match_key, "oz");
    assert!(report.neutral_csv.requires_oz_key());
    assert!(
        report
            .neutral_csv
            .fails_closed_for("spreadsheet_neutral_missing_oz_column")
    );
    assert!(report.supports_neutral_csv_exchange());
    assert!(report.has_no_production_or_certification_claim());
    assert!(!report.production_ready);
    assert!(report.certification_claims.is_empty());
    assert!(!report.external_spreadsheet_dependency);

    let flags = report.capability_flags;
    assert!(flags.neutral_csv_export);
    assert!(flags.neutral_csv_update);
    assert!(!flags.xlsx_export);
    assert!(!flags.ods_export);
    assert!(!flags.has_binary_spreadsheet_support());
    assert!(!flags.gaeb_export_roundtrip);
    assert!(!flags.production_spreadsheet_roundtrip);
    assert!(!flags.certification);
    assert!(!flags.has_production_or_certification_claim());

    let blocked_formats: BTreeSet<_> = report
        .blocked_formats
        .iter()
        .map(|format| format.format)
        .collect();
    assert_eq!(
        blocked_formats,
        ["xlsx", "ods", "gaeb-export-roundtrip"]
            .into_iter()
            .collect()
    );
    assert_eq!(
        report
            .blocked_format("xlsx")
            .expect("xlsx blocked row")
            .reason,
        "reference-only spreadsheet sources are not runtime support evidence"
    );
    for blocked in &report.blocked_formats {
        assert!(!blocked.implemented, "{}", blocked.format);
        assert!(!blocked.production_supported, "{}", blocked.format);
        assert!(blocked.is_blocked(), "{}", blocked.format);
    }
}

#[test]
fn export_boundary_schema_docs_and_cli_are_stable() {
    for path in [
        "docs/service-contract/export-boundary-v1.md",
        "docs/service-contract/export-boundary-v1.schema.json",
    ] {
        assert!(fs::metadata(path).is_ok(), "missing artifact: {path}");
    }

    let docs = fs::read_to_string("docs/service-contract/export-boundary-v1.md").expect("docs");
    for required in [
        "boq-core.export-boundary.v1",
        "oz_matched_csv_neutral",
        "spreadsheet_neutral_missing_oz_column",
        "xlsx_export`: always `false`",
        "ods_export`: always `false`",
        "production_spreadsheet_roundtrip`: always `false`",
        "external_spreadsheet_dependency`: always `false`",
    ] {
        assert!(docs.contains(required), "docs missing: {required}");
    }

    let schema: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("docs/service-contract/export-boundary-v1.schema.json")
            .expect("schema"),
    )
    .expect("schema JSON");
    assert_eq!(
        schema["properties"]["schema_version"]["const"],
        EXPORT_BOUNDARY_SCHEMA_VERSION
    );
    assert_eq!(
        schema["$defs"]["capabilityFlags"]["properties"]["xlsx_export"]["const"],
        false
    );

    let exe = std::env::var("CARGO_BIN_EXE_boq-core-service").expect("service binary path");
    let output = Command::new(exe)
        .arg("export-boundaries")
        .output()
        .expect("service cli runs");
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let actual: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let expected = serde_json::to_value(export_boundary_report()).expect("library report JSON");
    assert_eq!(actual, expected);
}

#[test]
fn neutral_csv_boundary_is_oz_keyed_and_fail_closed() {
    let mut document = boq_core::gaeb_xml::parse_str(
        include_str!("fixtures/synthetic/minimal_ava.x81"),
        Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
    )
    .expect("supported AVA fixture should parse");
    let original_status = document.support_status;

    let export = export_neutral_csv(&document);
    assert!(export.csv.contains("roundtrip_contract"));
    assert!(export.csv.contains("oz_matched_csv_neutral"));
    assert_eq!(
        export
            .metadata
            .get("roundtrip_contract")
            .and_then(|value| value.as_str()),
        Some("oz_matched_csv_neutral")
    );

    let missing_oz = apply_neutral_csv_updates(&mut document, "quantity,unit\n2,m2\n")
        .expect_err("missing OZ must fail closed");
    assert_eq!(missing_oz.code, "spreadsheet_neutral_missing_oz_column");

    let duplicate_oz = apply_neutral_csv_updates(&mut document, "oz,quantity,oz\n1,2,3\n")
        .expect_err("duplicate OZ must fail closed");
    assert_eq!(duplicate_oz.code, "spreadsheet_neutral_duplicate_header");

    let empty_oz = apply_neutral_csv_updates(&mut document, "oz,quantity\n ,2\n")
        .expect_err("empty OZ must fail closed");
    assert_eq!(empty_oz.code, "spreadsheet_neutral_missing_oz_value");

    assert_eq!(document.support_status, original_status);
}

#[test]
fn parse_only_inputs_do_not_gain_export_or_roundtrip_support_claims() {
    let document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal D81 should parse");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    let original_capabilities = document.capabilities;

    let export = export_neutral_csv(&document);
    assert!(export.csv.contains("support_status=supported_parse_only"));
    assert_eq!(
        export
            .metadata
            .get("dependency_policy")
            .and_then(|value| value.as_str()),
        Some("no_xlsx_or_binary_dependency")
    );

    let report = export_boundary_report();
    assert!(!report.capability_flags.gaeb_export_roundtrip);
    assert!(!report.capability_flags.production_spreadsheet_roundtrip);
    assert!(!report.capability_flags.certification);
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert_eq!(document.capabilities, original_capabilities);
}
