#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::format;
use boq_core::model::GaebFormat;
use boq_core::support::manifest::{self, FixtureEntry};
use boq_core::support::{
    DecisionSource, ManifestPolicy, SupportPolicy, SupportQuery, SupportStatus,
};

#[test]
fn test_zeitvertrag_sources_are_cataloged_by_z_phase() {
    let fixtures = fixture_map();
    for (id, version, phase) in [
        (
            "official_gaeb_xml33_zeitvertrag",
            "gaeb_xml_3_3",
            "schema_package",
        ),
        (
            "official_gaeb_xml32_zeitvertrag",
            "gaeb_xml_3_2",
            "schema_package",
        ),
        (
            "official_gaeb_xml32_zeitvertrag_examples",
            "gaeb_xml_3_2",
            "sample_files",
        ),
    ] {
        let fixture = &fixtures[id];
        assert_eq!(fixture.source_family, "official_gaeb");
        assert_eq!(fixture.process_domain, "zeitvertrag");
        assert_eq!(fixture.gaeb_version, version);
        assert_eq!(fixture.phase, phase);
        assert_eq!(fixture.support_status, "reference_only");
        assert!(fixture.license_note.contains("time-contract"));
        assert!(fixture.test_mapping.is_empty());
    }

    let matrix = include_str!("../docs/fixtures/zeitvertrag-x83z-x84z-boundary.md");
    for phase in ["X83Z", "X84Z"] {
        assert!(matrix.contains(phase), "matrix missing {phase}");
    }
}

#[test]
fn test_z_phase_boundary_adr_exists_before_parser_changes() {
    let adr = include_str!("../.archgate/adrs/ARCH-012-zeitvertrag-z-phase-boundary.md");
    for expected in [
        "Issue #43",
        "reference-only Z-phase framework-contract",
        "must not degrade into ordinary `.X83`/`.X84` phase claims",
        "No parser-support claim",
        "companion module/crate",
    ] {
        assert!(adr.contains(expected), "ADR missing {expected}");
    }

    let decision = ManifestPolicy::embedded().decide(SupportQuery {
        format: GaebFormat::GaebXml,
        version: Some("3.3"),
        phase: None,
        source_uri: Some("gaeb/official_gaeb/gaeb_xml_3_3/zeitvertrag/schema.x83z"),
    });

    assert_eq!(decision.status, SupportStatus::ReferenceOnly);
    assert!(decision.capabilities.reference_only);
    assert!(!decision.capabilities.parse);
    assert!(
        matches!(decision.source, DecisionSource::ManifestEntry { ref id } if id == "official_gaeb_xml33_zeitvertrag")
    );
}

#[test]
fn test_x83z_x84z_are_not_misclassified_as_standard_x83_x84() {
    for path in ["framework.X83Z", "calloff.X84Z"] {
        let detected = format::detect_path(path);
        assert_eq!(detected.format, GaebFormat::GaebXml);
        assert_eq!(
            detected.phase, None,
            "{path} must not become ordinary X83/X84"
        );
    }

    let matrix = include_str!("../docs/fixtures/zeitvertrag-x83z-x84z-boundary.md");
    assert!(
        matrix.contains("must not silently fall back to ordinary Bauausführung X83/X84 behavior")
    );
}

#[test]
fn test_framework_discount_premium_red_tests() {
    let matrix = include_str!("../docs/fixtures/zeitvertrag-x83z-x84z-boundary.md");
    for expected in [
        "framework discounts",
        "premiums",
        "call-off conditions",
        "contract catalog metadata",
        "structured unsupported-field findings",
    ] {
        assert!(
            matrix.contains(expected),
            "framework obligation missing {expected}"
        );
    }
}

#[test]
fn test_zeitvertrag_interactive_schema_charts_are_reference_only() {
    let matrix = include_str!("../docs/fixtures/zeitvertrag-x83z-x84z-boundary.md");
    for expected in [
        "schema_x83z_33_chart",
        "schema_x84z_33_chart",
        "schema_x83z_32_chart",
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
fn test_issue_43_artifacts_bind_zeitvertrag_boundary() {
    let artifacts = [
        include_str!("../.omx/plans/prd-issue-43-zeitvertrag-x83z-x84z.md"),
        include_str!("../.omx/specs/issue-43-zeitvertrag-x83z-x84z.md"),
        include_str!("../.omx/plans/test-spec-issue-43-zeitvertrag-x83z-x84z.md"),
        include_str!("../.archgate/adrs/ARCH-012-zeitvertrag-z-phase-boundary.md"),
    ];
    for artifact in artifacts {
        for expected in ["#43", "X83Z", "X84Z", "reference_only"] {
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
