#![allow(missing_docs, clippy::expect_used)]

use std::fs;
use std::path::Path;

use boq_core::support::manifest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EvidenceFile {
    schema_version: u64,
    tool: String,
    tool_support_status: String,
    certification_claim: bool,
    results: Vec<EvidenceResult>,
}

#[derive(Debug, Deserialize)]
struct EvidenceResult {
    fixture_id: String,
    phase: String,
    fixture_support_status: String,
    result: String,
}

fn manifest_entry(id: &str) -> manifest::FixtureEntry {
    let parsed = manifest::parse(manifest::EMBEDDED_TOML).expect("manifest parses");
    parsed
        .fixtures
        .into_iter()
        .find(|fixture| fixture.id == id)
        .expect("manifest fixture exists")
}

#[test]
fn test_gaebxmlchecker_ava_source_status_matrix() {
    let checker = manifest_entry("bvbs_gaeb_xml_checker");
    assert_eq!(checker.source_family, "bvbs");
    assert_eq!(checker.process_domain, "tooling_reference");
    assert_eq!(checker.support_status, "reference_only");
    assert_eq!(checker.phase, "tool");
    assert!(checker.test_mapping.is_empty());

    for (fixture_id, phase) in [
        ("bvbs_xml33_ava_x81", "x81"),
        ("bvbs_xml33_ava_x84", "x84"),
        ("bvbs_xml33_ava_x86", "x86"),
    ] {
        let fixture = manifest_entry(fixture_id);
        assert_eq!(fixture.process_domain, "ava");
        assert_eq!(fixture.phase, phase);
        assert_eq!(fixture.support_status, "supported");
        assert!(
            !fixture.test_mapping.is_empty(),
            "supported AVA fixture must keep parser evidence mapping: {fixture_id}"
        );
    }
}

#[test]
fn test_gaebxmlchecker_ava_optional_tool_skip() {
    let docs = fs::read_to_string("docs/fixtures/gaebxmlchecker-ava-evidence.md")
        .expect("checker evidence docs exist");

    assert!(docs.contains("skipped_missing_checker"));
    assert!(docs.contains("neither pass nor fail"));
    assert!(docs.contains("CI must not download or execute"));
    assert!(docs.contains("agents and CI must not download or execute"));
    assert!(docs.contains("must not download or execute the checker binary"));
}

#[test]
fn test_gaebxmlchecker_ava_evidence_output_per_fixture() {
    let docs = fs::read_to_string("docs/fixtures/gaebxmlchecker-ava-evidence.md")
        .expect("checker evidence docs exist");

    for fixture_id in [
        "bvbs_xml33_ava_x81",
        "bvbs_xml33_ava_x84",
        "bvbs_xml33_ava_x86",
    ] {
        assert!(
            docs.contains(fixture_id),
            "evidence schema must name supported AVA fixture {fixture_id}"
        );
    }

    assert!(docs.contains("tool_support_status"));
    assert!(docs.contains("reference_only"));
    assert!(docs.contains("certification_claim"));
    assert!(Path::new("gaeb/evidence/gaebxmlchecker/ava/README.md").is_file());
}

#[test]
fn test_gaebxmlchecker_ava_no_certification_claim() {
    let docs = fs::read_to_string("docs/fixtures/gaebxmlchecker-ava-evidence.md")
        .expect("checker evidence docs exist");
    let readme = fs::read_to_string("gaeb/evidence/gaebxmlchecker/ava/README.md")
        .expect("checker evidence readme exists");
    let combined = format!("{docs}\n{readme}");

    assert!(combined.contains("not official BVBS certification"));
    assert!(combined.contains("certification_claim: false"));
    for forbidden_claim in [
        "is certified",
        "certification complete",
        "official BVBS certification complete",
    ] {
        assert!(
            !combined.contains(forbidden_claim),
            "policy text must not make positive certification claim: {forbidden_claim}"
        );
    }
}

#[test]
fn committed_checker_evidence_has_safe_schema_and_no_certification_claim() {
    let evidence_dir = Path::new("gaeb/evidence/gaebxmlchecker/ava");
    let mut json_files = Vec::new();

    for entry in fs::read_dir(evidence_dir).expect("evidence directory exists") {
        let entry = entry.expect("evidence dir entry readable");
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            json_files.push(path);
        }
    }

    for path in json_files {
        let text = fs::read_to_string(&path).expect("evidence json readable");
        let evidence: EvidenceFile =
            serde_json::from_str(&text).expect("committed evidence JSON matches schema");
        assert_eq!(evidence.schema_version, 1, "{}", path.display());
        assert_eq!(evidence.tool, "GAEBXmlChecker", "{}", path.display());
        assert_eq!(
            evidence.tool_support_status,
            "reference_only",
            "{}",
            path.display()
        );
        assert!(
            !evidence.certification_claim,
            "evidence must not claim certification: {}",
            path.display()
        );
        assert_eq!(evidence.results.len(), 3, "{}", path.display());
        for result in &evidence.results {
            assert!(
                matches!(
                    result.fixture_id.as_str(),
                    "bvbs_xml33_ava_x81" | "bvbs_xml33_ava_x84" | "bvbs_xml33_ava_x86"
                ),
                "unexpected fixture {} in {}",
                result.fixture_id,
                path.display()
            );
            assert!(matches!(result.phase.as_str(), "x81" | "x84" | "x86"));
            assert_eq!(result.fixture_support_status, "supported");
            assert!(matches!(
                result.result.as_str(),
                "passed" | "failed" | "skipped_missing_checker"
            ));
        }
    }
}

#[test]
fn gaebxmlchecker_ava_evidence_directory_has_no_executable_payloads() {
    let evidence_dir = Path::new("gaeb/evidence/gaebxmlchecker/ava");
    let forbidden_extensions = ["exe", "dll", "msi", "zip", "bat", "cmd", "ps1"];

    for entry in fs::read_dir(evidence_dir).expect("evidence directory exists") {
        let entry = entry.expect("evidence dir entry readable");
        let path = entry.path();
        if path.is_file() {
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            assert!(
                !forbidden_extensions.contains(&extension.as_str()),
                "checker payloads must not be committed as evidence: {}",
                path.display()
            );
        }
    }
}
