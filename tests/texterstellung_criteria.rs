#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeSet;
use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
struct CriteriaMatrix {
    criteria: Vec<Criterion>,
}

#[derive(Deserialize)]
struct Criterion {
    id: String,
    #[serde(rename = "criterion_section")]
    section: String,
    #[serde(rename = "criterion_type")]
    kind: String,
    fixture: String,
    evidence_kind: String,
    automated_test: String,
    status: String,
    support_claim: String,
    rendering_component_required: bool,
    certification_claim: String,
}

fn read_matrix() -> CriteriaMatrix {
    let text = fs::read_to_string("gaeb/criteria/bvbs_texterstellung_matrix.toml")
        .expect("Texterstellung criteria matrix exists");
    toml::from_str(&text).expect("Texterstellung criteria matrix parses")
}

fn read_manifest() -> toml::Value {
    let text = fs::read_to_string("gaeb/manifest.toml").expect("manifest exists");
    toml::from_str(&text).expect("manifest parses")
}

fn fixture_status(manifest: &toml::Value, id: &str) -> String {
    manifest
        .get("fixtures")
        .and_then(toml::Value::as_array)
        .and_then(|fixtures| {
            fixtures
                .iter()
                .filter_map(toml::Value::as_table)
                .find_map(|fixture| {
                    (fixture.get("id").and_then(toml::Value::as_str) == Some(id)).then(|| {
                        fixture
                            .get("support_status")
                            .and_then(toml::Value::as_str)
                            .expect("fixture support_status")
                            .to_owned()
                    })
                })
        })
        .expect("missing manifest fixture")
}

#[test]
fn test_text_criteria_matrix_covers_all_known_sections() {
    let matrix = read_matrix();
    let sections = matrix
        .criteria
        .iter()
        .map(|criterion| criterion.section.as_str())
        .collect::<BTreeSet<_>>();

    for expected in [
        "rich_text_markup",
        "table_structure",
        "text_complements",
        "cost_estimate_metadata",
        "visual_page_layout",
        "font_exact_rendering",
        "pdf_checker_result",
        "certification_submission",
    ] {
        assert!(
            sections.contains(expected),
            "missing criteria section: {expected}"
        );
    }

    assert_eq!(
        matrix.criteria.len(),
        sections.len(),
        "one row per known section"
    );
    assert!(
        matrix
            .criteria
            .iter()
            .all(|criterion| criterion.certification_claim == "none"),
        "Texterstellung matrix must not claim certification"
    );
}

#[test]
fn test_text_layout_manual_evidence_is_marked_manual() {
    let matrix = read_matrix();
    for criterion in matrix
        .criteria
        .iter()
        .filter(|criterion| criterion.kind == "rendering_only")
    {
        assert!(
            matches!(criterion.evidence_kind.as_str(), "manual" | "out_of_scope"),
            "rendering criterion must not be automated: {}",
            criterion.id
        );
        assert_eq!(criterion.automated_test, "");
        assert!(criterion.rendering_component_required);
        assert_ne!(criterion.status, "parser_readiness_covered");
        assert_eq!(criterion.support_claim, "reference_only");
    }
}

#[test]
fn test_text_fixture_golden_reports_link_criteria() {
    let matrix = read_matrix();
    let report = fs::read_to_string("docs/fixtures/bvbs-texterstellung-criteria-readiness.md")
        .expect("Texterstellung readiness report exists");

    for criterion in matrix
        .criteria
        .iter()
        .filter(|criterion| criterion.evidence_kind == "automated")
    {
        assert!(
            report.contains(&criterion.id),
            "missing report criterion: {}",
            criterion.id
        );
        assert!(
            report.contains(&criterion.automated_test),
            "missing report test mapping: {}",
            criterion.automated_test
        );
        assert!(
            report.contains(&criterion.fixture),
            "missing report fixture: {}",
            criterion.fixture
        );
    }

    for manual_id in [
        "text_visual_page_layout",
        "text_font_exact_rendering",
        "text_checker_result_pdf",
        "text_paid_certification_submission",
    ] {
        assert!(
            report.contains(manual_id),
            "missing manual criterion: {manual_id}"
        );
    }
    assert!(!report.contains("certification completed"));
}

#[test]
fn test_text_support_claims_require_matrix_status() {
    let matrix = read_matrix();
    let manifest = read_manifest();

    for criterion in &matrix.criteria {
        let manifest_status = fixture_status(&manifest, &criterion.fixture);
        if criterion.support_claim == "supported_parse_only" {
            assert_eq!(criterion.evidence_kind, "automated");
            assert_eq!(criterion.status, "parser_readiness_covered");
            assert!(!criterion.automated_test.is_empty());
            assert!(!criterion.rendering_component_required);
            assert_eq!(manifest_status, "supported_parse_only");
        } else {
            assert_eq!(criterion.support_claim, "reference_only");
            assert_eq!(manifest_status, "reference_only");
            assert_ne!(criterion.status, "parser_readiness_covered");
        }
    }
}
