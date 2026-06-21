#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::model::{GaebFormat, GaebPhase};
use boq_core::support::manifest::{self, FixtureEntry};
use boq_core::support::{
    DecisionSource, ManifestPolicy, SupportPolicy, SupportQuery, SupportStatus,
};

#[test]
fn test_costing_sources_are_cataloged_by_phase_x50_x51_x52() {
    let fixtures = fixture_map();

    let xml33 = &fixtures["official_gaeb_xml33_kosten_und_kalkulation"];
    assert_eq!(xml33.source_family, "official_gaeb");
    assert_eq!(xml33.process_domain, "kosten_und_kalkulation");
    assert_eq!(xml33.gaeb_version, "gaeb_xml_3_3");
    assert_eq!(xml33.support_status, "reference_only");
    assert!(xml33.license_note.contains("X50-X52"));
    assert!(xml33.test_mapping.is_empty());

    let xml32 = &fixtures["official_gaeb_xml32_kalkulation"];
    assert_eq!(xml32.source_family, "official_gaeb");
    assert_eq!(xml32.process_domain, "kalkulation");
    assert_eq!(xml32.gaeb_version, "gaeb_xml_3_2");
    assert_eq!(xml32.support_status, "reference_only");
    assert!(xml32.license_note.contains("legacy reference"));
    assert!(xml32.test_mapping.is_empty());

    let matrix = include_str!("../docs/fixtures/kosten-kalkulation-x50-x52-boundary.md");
    for phase in ["X50", "X51", "X52"] {
        assert!(matrix.contains(phase), "matrix missing {phase}");
    }
}

#[test]
fn test_costing_boundary_adr_exists_before_parser_modules() {
    let adr = include_str!("../.archgate/adrs/ARCH-010-kosten-kalkulation-boundary.md");
    for expected in [
        "Issue #41",
        "reference-only planning boundary",
        "No parser-support claim",
        "No Obra backend",
        "companion crate",
    ] {
        assert!(adr.contains(expected), "ADR missing {expected}");
    }

    let manifest_policy = ManifestPolicy::embedded();
    let phase = GaebPhase {
        code: "50".to_owned(),
        label: Some("Kosten/Kalkulation X50".to_owned()),
    };
    let decision = manifest_policy.decide(SupportQuery {
        format: GaebFormat::GaebXml,
        version: Some("3.3"),
        phase: Some(&phase),
        source_uri: Some("gaeb/official_gaeb/gaeb_xml_3_3/kosten_und_kalkulation/schema.x50"),
    });

    assert_eq!(decision.status, SupportStatus::ReferenceOnly);
    assert!(decision.capabilities.reference_only);
    assert!(!decision.capabilities.parse);
    assert!(
        matches!(decision.source, DecisionSource::ManifestEntry { ref id } if id == "official_gaeb_xml33_kosten_und_kalkulation")
    );
}

#[test]
fn test_cost_component_model_red_tests() {
    let matrix = include_str!("../docs/fixtures/kosten-kalkulation-x50-x52-boundary.md");
    for expected in [
        "cost-component identity",
        "amount basis",
        "surcharge/discount semantics",
        "unsupported-field findings",
        "Support status promotion requires a later PR",
    ] {
        assert!(
            matrix.contains(expected),
            "model obligation missing {expected}"
        );
    }
}

#[test]
fn test_x52_item_reference_mapping_red_tests() {
    let matrix = include_str!("../docs/fixtures/kosten-kalkulation-x50-x52-boundary.md");
    for expected in [
        "X52 calculation records",
        "BOQ item references",
        "without silently creating or mutating item text/quantity support",
    ] {
        assert!(
            matrix.contains(expected),
            "X52 mapping obligation missing {expected}"
        );
    }
}

#[test]
fn test_kosten_interactive_schema_charts_are_reference_only() {
    let matrix = include_str!("../docs/fixtures/kosten-kalkulation-x50-x52-boundary.md");
    for expected in [
        "schema_x50_33_chart",
        "schema_x52_33_chart",
        "schema_x52_32_chart",
        "artifact-only/reference",
        "No CI dependency on external HTML",
    ] {
        assert!(
            matrix.contains(expected),
            "interactive chart policy missing {expected}"
        );
    }
}

#[test]
fn test_issue_41_artifacts_bind_kosten_boundary() {
    let artifacts = [
        include_str!("../.omx/plans/prd-issue-41-kosten-kalkulation-x50-x52.md"),
        include_str!("../.omx/specs/issue-41-kosten-kalkulation-x50-x52.md"),
        include_str!("../.omx/plans/test-spec-issue-41-kosten-kalkulation-x50-x52.md"),
        include_str!("../.archgate/adrs/ARCH-010-kosten-kalkulation-boundary.md"),
    ];
    for artifact in artifacts {
        for expected in ["#41", "X50-X52", "reference_only", "kosten"] {
            assert!(artifact.contains(expected), "artifact missing {expected}");
        }
    }
}

fn fixture_map() -> BTreeMap<String, FixtureEntry> {
    let manifest = manifest::parse(manifest::EMBEDDED_TOML).expect("embedded manifest parses");
    manifest
        .fixtures
        .into_iter()
        .map(|fixture| (fixture.id.clone(), fixture))
        .collect()
}
