#![allow(missing_docs, clippy::expect_used, clippy::panic)]

use std::{collections::BTreeSet, fs, process::Command};

use boq_core::service_support_manifest::{
    SERVICE_SUPPORT_MANIFEST_SCHEMA_VERSION, SUPPORT_VOCABULARY, export_embedded_support_manifest,
};
use boq_core::support::manifest;

#[test]
fn support_manifest_export_preserves_exact_vocabulary_and_support_honesty() {
    let report = export_embedded_support_manifest().expect("embedded manifest exports");

    assert_eq!(
        report.schema_version,
        SERVICE_SUPPORT_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(report.support_vocabulary, SUPPORT_VOCABULARY);
    assert!(!report.production_ready);
    assert!(report.certification_claims.is_empty());
    assert!(!report.entries.is_empty());

    let statuses: BTreeSet<_> = report
        .entries
        .iter()
        .map(|entry| entry.support_status.as_str())
        .collect();
    assert_eq!(statuses, SUPPORT_VOCABULARY.into_iter().collect());

    for entry in &report.entries {
        assert!(SUPPORT_VOCABULARY.contains(&entry.support_status.as_str()));
        assert!(
            !entry.capabilities.certification,
            "{} must not claim certification",
            entry.fixture_id
        );
        assert!(
            !entry
                .source_policy
                .service_export_requires_external_download
        );
        assert!(entry.target_dir.starts_with("gaeb/"));
        match entry.support_status.as_str() {
            "supported" | "supported_parse_only" => {
                assert!(entry.capabilities.detect, "{}", entry.fixture_id);
                assert!(entry.capabilities.parse, "{}", entry.fixture_id);
                assert!(!entry.capabilities.export, "{}", entry.fixture_id);
                assert!(!entry.capabilities.roundtrip, "{}", entry.fixture_id);
                assert!(!entry.capabilities.reference_only, "{}", entry.fixture_id);
                assert!(!entry.test_mapping.is_empty(), "{}", entry.fixture_id);
            }
            "future_track" | "reference_only" => {
                assert!(entry.capabilities.detect, "{}", entry.fixture_id);
                assert!(!entry.capabilities.parse, "{}", entry.fixture_id);
                assert!(!entry.capabilities.validate, "{}", entry.fixture_id);
                assert!(!entry.capabilities.adapt_to_obra, "{}", entry.fixture_id);
                assert!(!entry.capabilities.export, "{}", entry.fixture_id);
                assert!(!entry.capabilities.roundtrip, "{}", entry.fixture_id);
                assert!(entry.capabilities.reference_only, "{}", entry.fixture_id);
            }
            other => panic!("unknown support vocabulary exported: {other}"),
        }
    }
}

#[test]
fn support_manifest_export_includes_expected_service_gating_rows() {
    let report = export_embedded_support_manifest().expect("embedded manifest exports");
    let supported_ava = report
        .entries
        .iter()
        .find(|entry| entry.fixture_id == "bvbs_xml33_ava_x81")
        .expect("AVA X81 row");
    assert_eq!(supported_ava.support_status.as_str(), "supported");
    assert!(supported_ava.capabilities.parse);
    assert!(supported_ava.capabilities.validate);
    assert!(supported_ava.capabilities.adapt_to_obra);
    assert!(!supported_ava.capabilities.certification);

    let parse_only_bau = report
        .entries
        .iter()
        .find(|entry| entry.fixture_id == "bvbs_xml33_bau_x83")
        .expect("Bau X83 row");
    assert_eq!(
        parse_only_bau.support_status.as_str(),
        "supported_parse_only"
    );
    assert!(parse_only_bau.capabilities.parse);
    assert!(parse_only_bau.capabilities.adapt_to_obra);
    assert!(!parse_only_bau.capabilities.validate);
    assert!(!parse_only_bau.capabilities.certification);

    let future = report
        .entries
        .iter()
        .find(|entry| entry.support_status.as_str() == "future_track")
        .expect("future-track row");
    assert!(!future.capabilities.parse);
    assert!(future.capabilities.reference_only);

    let reference = report
        .entries
        .iter()
        .find(|entry| entry.support_status.as_str() == "reference_only")
        .expect("reference-only row");
    assert!(!reference.capabilities.parse);
    assert!(reference.capabilities.reference_only);
}

#[test]
fn manifest_parser_rejects_unknown_support_vocabulary() {
    let toml_text = r#"
[[fixtures]]
id = "bad_status"
source_url = "https://www.bvbs.de/a.zip"
normalized_url = "https://www.bvbs.de/a.zip"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/a"
support_status = "certified"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = []
"#;
    let error = manifest::parse(toml_text).expect_err("unknown vocabulary must fail");
    assert!(
        error.message().contains("unknown variant `certified`"),
        "unexpected error: {error}"
    );
}

#[test]
fn support_manifest_schema_docs_and_golden_fixture_are_checked_in() {
    for path in [
        "docs/service-contract/support-manifest-v1.md",
        "docs/service-contract/support-manifest-v1.schema.json",
        "tests/fixtures/service_contract/support_manifest.capabilities.json",
    ] {
        assert!(
            fs::metadata(path).is_ok(),
            "missing support manifest artifact: {path}"
        );
    }

    let docs = fs::read_to_string("docs/service-contract/support-manifest-v1.md").expect("docs");
    for required in [
        "boq-core.support-manifest.v1",
        "supported_parse_only",
        "certification`: always `false`",
        "service_export_requires_external_download`: always `false`",
    ] {
        assert!(docs.contains(required), "docs missing: {required}");
    }

    let schema: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("docs/service-contract/support-manifest-v1.schema.json")
            .expect("schema"),
    )
    .expect("schema JSON");
    assert_eq!(
        schema["properties"]["schema_version"]["const"],
        SERVICE_SUPPORT_MANIFEST_SCHEMA_VERSION
    );
}

#[test]
fn support_manifest_cli_output_matches_golden_fixture() {
    let exe = std::env::var("CARGO_BIN_EXE_boq-core-service").expect("service binary path");
    let output = Command::new(exe)
        .arg("capabilities")
        .output()
        .expect("service cli runs");
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let actual: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    let expected: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("tests/fixtures/service_contract/support_manifest.capabilities.json")
            .expect("golden fixture"),
    )
    .expect("golden JSON");
    assert_eq!(actual, expected);
}
