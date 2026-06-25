#![allow(missing_docs, clippy::expect_used)]

use std::process::Command;

use boq_core::service_contract::{AnalyzeFormatHint, AnalyzeInput, AnalyzeStatus, analyze_bytes};
use boq_core::support::SupportStatus;

#[test]
fn analyze_gaeb_xml_returns_service_json_contract_with_support_evidence() {
    let source = include_str!("fixtures/synthetic/minimal_ava.x81");
    let report = analyze_bytes(&AnalyzeInput {
        bytes: source.as_bytes(),
        source_uri: Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
        format_hint: Some(AnalyzeFormatHint::GaebXml),
    });

    assert_eq!(report.schema_version, "boq-core.service-analyze.v1");
    assert_eq!(report.status, AnalyzeStatus::Ok);
    assert_eq!(
        report.document.as_ref().expect("document").support_status,
        SupportStatus::Supported
    );
    assert!(
        report
            .document
            .as_ref()
            .expect("document")
            .capabilities
            .adapt_to_obra
    );
    assert!(report.diagnostics.is_empty());
    assert!(report.error.is_none());
    assert!(!report.production_ready);
    assert!(report.certification_claims.is_empty());
}

#[test]
fn analyze_malformed_input_returns_error_contract_without_support_promotion() {
    let report = analyze_bytes(&AnalyzeInput {
        bytes: b"<GAEB><",
        source_uri: Some("broken.x81".to_owned()),
        format_hint: Some(AnalyzeFormatHint::GaebXml),
    });

    assert_eq!(report.status, AnalyzeStatus::Error);
    assert!(report.document.is_none());
    assert_eq!(
        report.error.as_ref().expect("error").code,
        "xml_parse_failed"
    );
    assert!(!report.production_ready);
    assert!(report.certification_claims.is_empty());
}

#[test]
fn service_cli_analyze_emits_json_contract() {
    let exe = std::env::var("CARGO_BIN_EXE_boq-core-service").expect("service binary path");
    let output = Command::new(exe)
        .args([
            "analyze",
            "tests/fixtures/synthetic/minimal_ava.x81",
            "--format",
            "gaeb-xml",
        ])
        .output()
        .expect("service cli runs");

    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(json["schema_version"], "boq-core.service-analyze.v1");
    assert_eq!(json["status"], "ok");
    assert_eq!(json["production_ready"], false);
    assert!(
        json["certification_claims"]
            .as_array()
            .expect("claims array")
            .is_empty()
    );
}

#[test]
fn analyze_gaeb90_hint_and_path_detection_return_service_contract() {
    let source = include_bytes!("fixtures/synthetic/minimal.d81");
    let hinted = analyze_bytes(&AnalyzeInput {
        bytes: source,
        source_uri: Some("tests/fixtures/synthetic/minimal.d81".to_owned()),
        format_hint: Some(AnalyzeFormatHint::Gaeb90),
    });
    let detected = analyze_bytes(&AnalyzeInput {
        bytes: source,
        source_uri: Some("tests/fixtures/synthetic/minimal.d81".to_owned()),
        format_hint: None,
    });

    for report in [hinted, detected] {
        assert_eq!(report.status, AnalyzeStatus::Ok);
        let document = report.document.as_ref().expect("document");
        assert_eq!(
            document
                .summary
                .phase
                .as_ref()
                .map(|phase| phase.code.as_str()),
            Some("81")
        );
        assert_eq!(document.top_level_node_count, 1);
        assert!(!report.production_ready);
        assert!(report.certification_claims.is_empty());
    }
}

#[test]
fn analyze_format_hint_accepts_aliases_and_rejects_unknown_values() {
    assert_eq!(
        AnalyzeFormatHint::parse("gaeb_xml"),
        Some(AnalyzeFormatHint::GaebXml)
    );
    assert_eq!(
        AnalyzeFormatHint::parse("xml"),
        Some(AnalyzeFormatHint::GaebXml)
    );
    assert_eq!(
        AnalyzeFormatHint::parse("gaeb-90"),
        Some(AnalyzeFormatHint::Gaeb90)
    );
    assert_eq!(
        AnalyzeFormatHint::parse("d83"),
        Some(AnalyzeFormatHint::Gaeb90)
    );
    assert_eq!(AnalyzeFormatHint::parse("ifc"), None);
}

#[test]
fn service_cli_rejects_invalid_invocations_without_json_success() {
    let exe = std::env::var("CARGO_BIN_EXE_boq-core-service").expect("service binary path");
    let cases: [(&[&str], &str); 4] = [
        (&[], "usage:"),
        (
            &["inspect", "tests/fixtures/synthetic/minimal_ava.x81"],
            "usage:",
        ),
        (
            &[
                "analyze",
                "tests/fixtures/synthetic/minimal_ava.x81",
                "--format",
                "ifc",
            ],
            "unsupported --format value",
        ),
        (
            &[
                "analyze",
                "tests/fixtures/synthetic/minimal_ava.x81",
                "--unexpected",
            ],
            "unknown argument",
        ),
    ];

    for (args, expected_stderr) in cases {
        let output = Command::new(&exe)
            .args(args)
            .output()
            .expect("service cli runs");
        assert!(!output.status.success(), "args={args:?}");
        assert_eq!(output.status.code(), Some(2), "args={args:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected_stderr),
            "args={args:?}, stderr={}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty(), "args={args:?}");
    }
}

#[test]
fn service_cli_reports_read_errors_without_success() {
    let exe = std::env::var("CARGO_BIN_EXE_boq-core-service").expect("service binary path");
    let output = Command::new(exe)
        .args(["analyze", "tests/fixtures/synthetic/does-not-exist.x81"])
        .output()
        .expect("service cli runs");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("failed to read"));
    assert!(output.stdout.is_empty());
}
