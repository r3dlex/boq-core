#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeSet;
use std::fs;

use boq_core::support::manifest;
use serde::Deserialize;

#[derive(Deserialize)]
struct CriteriaFile {
    criteria: Vec<Criterion>,
}

#[derive(Deserialize)]
struct Criterion {
    id: String,
    source_reference: String,
    fixture: String,
    phase: String,
    expected_behavior: String,
    required_action: String,
    support_status: String,
    evidence_status: String,
    automated_test: String,
    golden_report: String,
    manual_evidence: String,
    certification_claim: bool,
    in_v1: bool,
    blocking: bool,
}

fn criteria_file() -> CriteriaFile {
    let text = fs::read_to_string("gaeb/criteria/bvbs_ava_matrix.toml").expect("criteria exists");
    toml::from_str(&text).expect("criteria parses")
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
fn in_v1_blocking_ava_criteria_have_automated_tests() {
    for criterion in criteria_file()
        .criteria
        .iter()
        .filter(|criterion| criterion.in_v1 && criterion.blocking)
    {
        assert!(!criterion.id.is_empty());
        assert!(
            !criterion.automated_test.is_empty(),
            "blocking criterion lacks automated test: {}",
            criterion.id
        );
    }
}

#[test]
fn test_bvbs_ava_criteria_matrix_has_fixture_mapping() {
    let file = criteria_file();
    let mapped_fixtures = file
        .criteria
        .iter()
        .map(|criterion| criterion.fixture.as_str())
        .collect::<BTreeSet<_>>();

    assert!(mapped_fixtures.contains("bvbs_xml33_ava_x81"));
    assert!(mapped_fixtures.contains("bvbs_xml33_ava_x84"));
    assert!(mapped_fixtures.contains("bvbs_xml33_ava_x86"));

    for criterion in &file.criteria {
        assert!(!criterion.source_reference.is_empty(), "{}", criterion.id);
        assert!(!criterion.expected_behavior.is_empty(), "{}", criterion.id);
        assert!(!criterion.required_action.is_empty(), "{}", criterion.id);
        let fixture = manifest_entry(&criterion.fixture);
        assert_eq!(fixture.process_domain, "ava", "{}", criterion.id);
        assert_eq!(fixture.phase, criterion.phase, "{}", criterion.id);
        assert_eq!(
            fixture.support_status, criterion.support_status,
            "{}",
            criterion.id
        );
    }
}

#[test]
fn test_bvbs_ava_criteria_matrix_flags_manual_evidence() {
    let file = criteria_file();
    let mut has_manual_gate = false;

    for criterion in &file.criteria {
        assert!(!criterion.manual_evidence.is_empty(), "{}", criterion.id);
        assert!(
            criterion.manual_evidence.contains("readiness")
                || criterion.manual_evidence.contains("none")
                || criterion.manual_evidence.contains("required"),
            "manual evidence must be explicit and readiness-scoped: {}",
            criterion.id
        );
        assert!(
            !criterion.certification_claim,
            "criteria matrix must not claim certification: {}",
            criterion.id
        );
        has_manual_gate |= criterion.manual_evidence.contains("required before paid");
    }

    assert!(
        has_manual_gate,
        "matrix must include paid-submission manual gate"
    );
}

#[test]
fn test_bvbs_ava_criteria_matrix_rejects_empty_status() {
    let allowed_statuses = [
        "supported",
        "supported_parse_only",
        "future_track",
        "reference_only",
    ];
    let allowed_evidence_statuses = ["automated", "manual_required", "planned_golden", "gap"];

    for criterion in &criteria_file().criteria {
        assert!(
            allowed_statuses.contains(&criterion.support_status.as_str()),
            "invalid support status for {}: {}",
            criterion.id,
            criterion.support_status
        );
        assert!(
            allowed_evidence_statuses.contains(&criterion.evidence_status.as_str()),
            "invalid evidence status for {}: {}",
            criterion.id,
            criterion.evidence_status
        );
    }
}

#[test]
fn test_bvbs_ava_criteria_matrix_links_golden_reports() {
    for criterion in &criteria_file().criteria {
        assert!(!criterion.golden_report.is_empty(), "{}", criterion.id);
        assert!(
            criterion.golden_report.starts_with("gaeb/golden/bvbs_ava/")
                || criterion.golden_report == "planned:#17",
            "criterion must link a deterministic golden report path or explicit #17 planned link: {}",
            criterion.id
        );
        if criterion.in_v1 && criterion.golden_report == "planned:#17" {
            assert_eq!(
                criterion.evidence_status, "planned_golden",
                "{}",
                criterion.id
            );
        }
    }
}
