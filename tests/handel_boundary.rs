#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::format;
use boq_core::model::{GaebFormat, GaebPhase};
use boq_core::support::manifest::{self, FixtureEntry};
use boq_core::support::{
    DecisionSource, ManifestPolicy, SupportPolicy, SupportQuery, SupportStatus,
};

#[test]
fn test_handel_sources_are_cataloged_by_phase_x93_x94_x96_x97() {
    let fixtures = fixture_map();
    let fixture = &fixtures["official_gaeb_xml33_handel"];
    assert_eq!(fixture.source_family, "official_gaeb");
    assert_eq!(fixture.process_domain, "handel");
    assert_eq!(fixture.gaeb_version, "gaeb_xml_3_3");
    assert_eq!(fixture.support_status, "reference_only");
    assert!(fixture.license_note.contains("X93-X97"));
    assert!(fixture.test_mapping.is_empty());

    let matrix = include_str!("../docs/fixtures/handel-x93-x97-boundary.md");
    for phase in ["X93", "X94", "X96", "X97"] {
        assert!(matrix.contains(phase), "matrix missing {phase}");
    }
    assert!(matrix.contains("gaeb32_handel_pkg"));
    assert!(matrix.contains("no invented manifest URL"));
}

#[test]
fn test_handel_boundary_adr_exists_before_parser_modules() {
    let adr = include_str!("../.archgate/adrs/ARCH-011-handel-boundary.md");
    for expected in [
        "Issue #42",
        "reference-only trade/procurement boundary",
        "advisory only",
        "No parser-support claim",
        "companion trade crate",
    ] {
        assert!(adr.contains(expected), "ADR missing {expected}");
    }

    let phase = GaebPhase {
        code: "93".to_owned(),
        label: Some("Handel X93".to_owned()),
    };
    let decision = ManifestPolicy::embedded().decide(SupportQuery {
        format: GaebFormat::GaebXml,
        version: Some("3.3"),
        phase: Some(&phase),
        source_uri: Some("gaeb/official_gaeb/gaeb_xml_3_3/handel/schema.x93"),
    });

    assert_eq!(decision.status, SupportStatus::ReferenceOnly);
    assert!(decision.capabilities.reference_only);
    assert!(!decision.capabilities.parse);
    assert!(
        matches!(decision.source, DecisionSource::ManifestEntry { ref id } if id == "official_gaeb_xml33_handel")
    );
}

#[test]
fn test_trade_document_is_not_classified_as_boq() {
    let matrix = include_str!("../docs/fixtures/handel-x93-x97-boundary.md");
    for expected in [
        "trade/procurement payloads",
        "must not be silently mapped into LV/BOQ item import semantics",
        "No Obra backend procurement changes",
        "No parser module is added for Handel",
    ] {
        assert!(
            matrix.contains(expected),
            "trade boundary missing {expected}"
        );
    }
}

#[test]
fn test_x93_x94_phase_detector_red_tests() {
    for (path, phase) in [
        ("catalog.X93", "93"),
        ("order.X94", "94"),
        ("delivery.X96", "96"),
        ("invoice.X97", "97"),
    ] {
        let detected = format::detect_path(path);
        assert_eq!(detected.format, GaebFormat::GaebXml);
        assert_eq!(
            detected.phase.map(|phase| phase.code),
            Some(phase.to_owned())
        );
    }

    let matrix = include_str!("../docs/fixtures/handel-x93-x97-boundary.md");
    assert!(matrix.contains("advisory GAEB XML phase detection only"));
}

#[test]
fn test_handel_interactive_schema_charts_are_reference_only() {
    let matrix = include_str!("../docs/fixtures/handel-x93-x97-boundary.md");
    for expected in [
        "schema_x93_33_chart",
        "schema_x94_33_chart",
        "schema_x93_32_chart",
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
fn test_issue_42_artifacts_bind_handel_boundary() {
    let artifacts = [
        include_str!("../.omx/plans/prd-issue-42-handel-x93-x97.md"),
        include_str!("../.omx/specs/issue-42-handel-x93-x97.md"),
        include_str!("../.omx/plans/test-spec-issue-42-handel-x93-x97.md"),
        include_str!("../.archgate/adrs/ARCH-011-handel-boundary.md"),
    ];
    for artifact in artifacts {
        for expected in ["#42", "X93-X97", "reference_only", "handel"] {
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
