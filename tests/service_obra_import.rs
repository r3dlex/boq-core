#![allow(missing_docs, clippy::expect_used)]

use std::{fs, process::Command};

use boq_core::service_contract::AnalyzeFormatHint;
use boq_core::service_obra_import::{
    ObraAdapterRejectionCode, ObraImportInput, ObraImportStatus, convert_bytes_to_obra_import,
    report_from_document,
};
use boq_core::support::{SupportCapabilities, SupportStatus};

const SUPPORTED_AVA_PATH: &str =
    "gaeb/bvbs/gaeb_xml_3_3/ava/x81/BVBS_Pruefdatei GAEB DA XML 3.3 - AVA - V 11 06 2021.X81";

#[test]
fn obra_import_supported_fixture_returns_stable_dto_contract() {
    let supported_bytes = fs::read(SUPPORTED_AVA_PATH).expect("supported fixture readable");
    let report = convert_bytes_to_obra_import(&ObraImportInput {
        bytes: &supported_bytes,
        source_uri: Some(SUPPORTED_AVA_PATH.to_owned()),
        format_hint: Some(AnalyzeFormatHint::GaebXml),
    });

    assert_eq!(report.schema_version, "boq-core.obra-import.v1");
    assert_eq!(report.status, ObraImportStatus::Ok);
    assert_eq!(report.support_status, Some(SupportStatus::Supported));
    assert!(report.capabilities.expect("capabilities").adapt_to_obra);
    assert!(report.rejection.is_none());
    assert!(report.error.is_none());
    assert!(!report.production_ready);
    assert!(report.certification_claims.is_empty());

    let import = report.import_document.expect("import document");
    assert!(!import.boq.deterministic_key.is_empty());
    assert!(
        import
            .wbs_nodes
            .iter()
            .all(|node| !node.deterministic_key.is_empty())
    );
    assert!(
        import
            .line_items
            .iter()
            .all(|line| !line.deterministic_key.is_empty())
    );
    assert!(import.classifications.iter().any(|classification| {
        classification.system_code == "gaeb" && !classification.external_code.is_empty()
    }));
    assert!(import.loss_report.unsupported_fields.is_empty());
}

#[test]
fn obra_import_parse_only_fixture_is_blocked_with_stable_code() {
    let report = convert_bytes_to_obra_import(&ObraImportInput {
        bytes: include_bytes!("fixtures/synthetic/minimal.d81"),
        source_uri: Some("tests/fixtures/synthetic/minimal.d81".to_owned()),
        format_hint: Some(AnalyzeFormatHint::Gaeb90),
    });

    assert_eq!(report.status, ObraImportStatus::Blocked);
    assert_eq!(
        report.support_status,
        Some(SupportStatus::SupportedParseOnly)
    );
    assert!(!report.capabilities.expect("capabilities").adapt_to_obra);
    assert!(report.import_document.is_none());
    let rejection = report.rejection.expect("rejection");
    assert_eq!(
        rejection.code,
        ObraAdapterRejectionCode::ObraAdapterSupportedParseOnly
    );
    assert_eq!(rejection.support_status, SupportStatus::SupportedParseOnly);
    assert!(
        report
            .diagnostics
            .iter()
            .any(|finding| finding.code == "obra_adapter_not_supported")
    );
    assert!(!report.production_ready);
    assert!(report.certification_claims.is_empty());
}

#[test]
fn obra_import_path_detection_and_parse_errors_are_stable() {
    let detected = convert_bytes_to_obra_import(&ObraImportInput {
        bytes: include_bytes!("fixtures/synthetic/minimal.d81"),
        source_uri: Some("tests/fixtures/synthetic/minimal.d81".to_owned()),
        format_hint: None,
    });
    assert_eq!(detected.status, ObraImportStatus::Blocked);
    assert_eq!(
        detected.rejection.expect("rejection").code,
        ObraAdapterRejectionCode::ObraAdapterSupportedParseOnly
    );

    let error = convert_bytes_to_obra_import(&ObraImportInput {
        bytes: b"<GAEB><",
        source_uri: Some("broken.x81".to_owned()),
        format_hint: Some(AnalyzeFormatHint::GaebXml),
    });
    assert_eq!(error.status, ObraImportStatus::Error);
    assert!(error.import_document.is_none());
    assert!(error.rejection.is_none());
    assert_eq!(error.error.expect("parse error").code, "xml_parse_failed");
    assert!(!error.production_ready);
    assert!(error.certification_claims.is_empty());

    let decode_error = convert_bytes_to_obra_import(&ObraImportInput {
        bytes: &[0xff, 0xfe],
        source_uri: Some("invalid-utf8.x81".to_owned()),
        format_hint: Some(AnalyzeFormatHint::GaebXml),
    });
    assert_eq!(decode_error.status, ObraImportStatus::Error);
    assert_eq!(
        decode_error.error.expect("decode error").code,
        "xml_decode_failed"
    );
}

#[test]
fn obra_import_blocked_status_codes_cover_future_reference_and_supported_without_adapter() {
    let mut document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("tests/fixtures/synthetic/minimal.d81".to_owned()),
    )
    .expect("minimal d81 parses");

    let cases = [
        (
            SupportStatus::SupportedParseOnly,
            SupportCapabilities::parse_with_obra_adapter(),
            ObraAdapterRejectionCode::ObraAdapterSupportedParseOnly,
        ),
        (
            SupportStatus::FutureTrack,
            SupportCapabilities::parse_only(),
            ObraAdapterRejectionCode::ObraAdapterFutureTrack,
        ),
        (
            SupportStatus::ReferenceOnly,
            SupportCapabilities::reference_only(),
            ObraAdapterRejectionCode::ObraAdapterReferenceOnly,
        ),
        (
            SupportStatus::Supported,
            SupportCapabilities::parse_only(),
            ObraAdapterRejectionCode::ObraAdapterNotSupported,
        ),
    ];

    for (status, capabilities, expected_code) in cases {
        document.support_status = status;
        document.capabilities = capabilities;
        let report = report_from_document(&document);
        assert_eq!(report.status, ObraImportStatus::Blocked);
        assert!(report.import_document.is_none());
        let rejection = report.rejection.expect("rejection");
        assert_eq!(rejection.code, expected_code);
        assert_eq!(rejection.support_status, status);
        assert_eq!(rejection.capabilities, capabilities);
    }
}

#[test]
fn obra_import_schema_docs_and_golden_fixtures_are_checked_in() {
    for path in [
        "docs/service-contract/obra-import-v1.md",
        "docs/service-contract/obra-import-v1.schema.json",
        "tests/fixtures/service_contract/bvbs_ava_x81.obra_import.json",
        "tests/fixtures/service_contract/minimal_d81.obra_import.json",
    ] {
        assert!(
            fs::metadata(path).is_ok(),
            "missing Obra import artifact: {path}"
        );
    }

    let docs = fs::read_to_string("docs/service-contract/obra-import-v1.md").expect("docs");
    for required in [
        "boq-core.obra-import.v1",
        "obra_adapter_supported_parse_only",
        "deterministic_key",
        "wbs_nodes",
        "line_items",
        "classifications",
        "loss_report",
        "production_ready`: always `false`",
        "certification_claims`: always empty",
    ] {
        assert!(docs.contains(required), "docs missing: {required}");
    }

    let schema: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("docs/service-contract/obra-import-v1.schema.json").expect("schema"),
    )
    .expect("schema is valid JSON");
    assert_eq!(
        schema["properties"]["schema_version"]["const"],
        "boq-core.obra-import.v1"
    );
    assert_eq!(schema["properties"]["production_ready"]["const"], false);
    assert_eq!(schema["properties"]["certification_claims"]["maxItems"], 0);
    assert_eq!(
        schema["properties"]["import_document"]["oneOf"][0]["$ref"],
        "#/$defs/obraImportDocument"
    );
    for def in [
        "sourceProvenance",
        "supportCapabilities",
        "obraImportDocument",
        "obraBoqDocument",
        "obraWbsNodeCandidate",
        "obraLineItem",
        "obraClassification",
        "lossReport",
    ] {
        assert!(
            schema["$defs"].get(def).is_some(),
            "missing schema $defs.{def}"
        );
    }
    for required in ["deterministic_key", "title", "status", "metadata"] {
        assert!(
            schema["$defs"]["obraBoqDocument"]["required"]
                .as_array()
                .expect("required array")
                .iter()
                .any(|value| value == required),
            "boq schema missing required {required}"
        );
    }
}

#[test]
fn obra_import_golden_fixtures_match_schema_required_shapes() {
    let supported: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("tests/fixtures/service_contract/bvbs_ava_x81.obra_import.json")
            .expect("supported golden"),
    )
    .expect("supported golden is valid JSON");
    let blocked: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("tests/fixtures/service_contract/minimal_d81.obra_import.json")
            .expect("blocked golden"),
    )
    .expect("blocked golden is valid JSON");

    assert_eq!(supported["schema_version"], "boq-core.obra-import.v1");
    assert_eq!(supported["status"], "ok");
    assert_eq!(supported["support_status"], "supported");
    assert_eq!(supported["production_ready"], false);
    assert!(
        supported["certification_claims"]
            .as_array()
            .expect("claims")
            .is_empty()
    );
    let import = &supported["import_document"];
    for key in [
        "source",
        "boq",
        "wbs_nodes",
        "line_items",
        "classifications",
        "loss_report",
    ] {
        assert!(!import[key].is_null(), "supported import missing {key}");
    }
    assert!(
        import["boq"]["deterministic_key"]
            .as_str()
            .is_some_and(|key| !key.is_empty())
    );
    assert!(
        import["wbs_nodes"]
            .as_array()
            .expect("wbs nodes")
            .iter()
            .all(|node| node["deterministic_key"]
                .as_str()
                .is_some_and(|key| !key.is_empty())
                && node["path"].as_str().is_some_and(|path| !path.is_empty()))
    );
    assert!(
        import["line_items"]
            .as_array()
            .expect("line items")
            .iter()
            .all(|line| line["deterministic_key"]
                .as_str()
                .is_some_and(|key| !key.is_empty())
                && line["wbs_node_key"]
                    .as_str()
                    .is_some_and(|key| !key.is_empty()))
    );
    assert!(
        import["classifications"]
            .as_array()
            .expect("classifications")
            .iter()
            .all(|classification| classification["system_code"]
                .as_str()
                .is_some_and(|code| !code.is_empty()))
    );
    assert!(import["loss_report"]["warnings"].as_array().is_some());
    assert!(
        import["loss_report"]["unsupported_fields"]
            .as_array()
            .is_some()
    );
    assert!(import["loss_report"]["lossy_mappings"].as_array().is_some());

    assert_eq!(blocked["schema_version"], "boq-core.obra-import.v1");
    assert_eq!(blocked["status"], "blocked");
    assert_eq!(blocked["support_status"], "supported_parse_only");
    assert!(blocked["import_document"].is_null());
    assert_eq!(
        blocked["rejection"]["code"],
        "obra_adapter_supported_parse_only"
    );
    assert_eq!(blocked["production_ready"], false);
    assert!(
        blocked["certification_claims"]
            .as_array()
            .expect("claims")
            .is_empty()
    );
}

#[test]
fn library_reports_match_checked_in_obra_import_golden_fixtures() {
    let supported_bytes = fs::read(SUPPORTED_AVA_PATH).expect("supported fixture readable");
    let cases: Vec<(&[u8], &str, AnalyzeFormatHint, &str)> = vec![
        (
            supported_bytes.as_slice(),
            SUPPORTED_AVA_PATH,
            AnalyzeFormatHint::GaebXml,
            "tests/fixtures/service_contract/bvbs_ava_x81.obra_import.json",
        ),
        (
            include_bytes!("fixtures/synthetic/minimal.d81").as_slice(),
            "tests/fixtures/synthetic/minimal.d81",
            AnalyzeFormatHint::Gaeb90,
            "tests/fixtures/service_contract/minimal_d81.obra_import.json",
        ),
    ];

    for (bytes, source_uri, format_hint, golden_path) in cases {
        let report = convert_bytes_to_obra_import(&ObraImportInput {
            bytes,
            source_uri: Some(source_uri.to_owned()),
            format_hint: Some(format_hint),
        });
        let actual = serde_json::to_value(&report).expect("report serializes");
        let expected: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(golden_path).expect("golden fixture readable"),
        )
        .expect("golden fixture is valid JSON");
        assert_eq!(actual, expected, "golden mismatch for {golden_path}");
    }
}

#[test]
fn service_cli_obra_import_output_matches_checked_in_golden_fixtures() {
    let exe = std::env::var("CARGO_BIN_EXE_boq-core-service").expect("service binary path");
    let cases = [
        (
            ["obra-import", SUPPORTED_AVA_PATH, "--format", "gaeb-xml"],
            "tests/fixtures/service_contract/bvbs_ava_x81.obra_import.json",
        ),
        (
            [
                "obra-import",
                "tests/fixtures/synthetic/minimal.d81",
                "--format",
                "gaeb90",
            ],
            "tests/fixtures/service_contract/minimal_d81.obra_import.json",
        ),
    ];

    for (args, golden_path) in cases {
        let output = Command::new(&exe)
            .args(args)
            .output()
            .expect("service cli runs");
        assert!(
            output.status.success(),
            "stderr={}",
            String::from_utf8_lossy(&output.stderr)
        );
        let actual: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("valid cli json");
        let expected: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(golden_path).expect("golden fixture readable"),
        )
        .expect("golden fixture is valid JSON");
        assert_eq!(actual, expected, "cli golden mismatch for {golden_path}");
    }
}
