#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeSet;
use std::fs;

use boq_core::support::manifest::{self, FixtureManifest};

#[test]
fn bvbs_certification_and_checker_areas_are_cataloged() {
    let manifest = read_manifest();
    let bvbs_domains = manifest
        .fixtures
        .iter()
        .filter(|fixture| fixture.source_family == "bvbs")
        .map(|fixture| fixture.process_domain.as_str())
        .collect::<BTreeSet<_>>();

    for expected in [
        "ava",
        "construction_execution",
        "quantity_takeoff",
        "specification_authoring",
        "legacy_xml_3_2",
        "legacy_xml_3_1",
        "tooling_reference",
    ] {
        assert!(
            bvbs_domains.contains(expected),
            "missing BVBS certification/checker area: {expected}"
        );
    }
}

#[test]
fn non_certification_schema_and_example_sources_are_cataloged() {
    let manifest = read_manifest();
    for source_family in [
        "official_gaeb",
        "developer_examples",
        "productivity_templates",
    ] {
        assert!(
            manifest
                .fixtures
                .iter()
                .any(|fixture| fixture.source_family == source_family),
            "missing non-certification source family: {source_family}"
        );
    }

    for (version, domain) in [
        ("gaeb_xml_3_3", "leistungsverzeichnis"),
        ("gaeb_xml_3_3", "mengenermittlung"),
        ("gaeb_xml_3_3", "rechnung"),
        ("gaeb_xml_3_3", "kosten_und_kalkulation"),
        ("gaeb_xml_3_3", "zeitvertrag"),
        ("gaeb_xml_3_3", "handel"),
        ("gaeb_xml_3_4_beta", "schema_reference"),
        ("gaeb_xml_3_2", "leistungsverzeichnis"),
        ("gaeb_xml_3_2", "rechnung"),
        ("gaeb_xml_3_1", "musterdateien"),
    ] {
        assert!(
            manifest.fixtures.iter().any(|fixture| {
                fixture.source_family == "official_gaeb"
                    && fixture.gaeb_version == version
                    && fixture.process_domain == domain
            }),
            "missing official GAEB {version}/{domain} reference"
        );
    }

    for expected_id in [
        "dangl_ava_examples",
        "dangl_ava_gaeb90_d83",
        "dangl_ava_examples_cpp",
        "dangl_oenorm_examples",
        "avacloud_demo_node",
        "dangl_gaeb2000_sportheim_gist",
        "gaeb_online_import_template",
        "gaeb_online_generator_exe",
    ] {
        assert!(
            manifest
                .fixtures
                .iter()
                .any(|fixture| fixture.id == expected_id),
            "missing developer/productivity fixture: {expected_id}"
        );
    }
}

#[test]
fn support_statuses_prevent_overclaiming_follow_on_tracks() {
    let manifest = read_manifest();
    let supported_ids = manifest
        .fixtures
        .iter()
        .filter(|fixture| {
            matches!(
                fixture.support_status.as_str(),
                "supported" | "supported_parse_only"
            )
        })
        .map(|fixture| fixture.id.as_str())
        .collect::<BTreeSet<_>>();

    for expected_supported in [
        "bvbs_xml33_ava_x81",
        "bvbs_xml33_ava_x84",
        "bvbs_xml33_ava_x86",
        "dangl_ava_gaeb90_d83",
        "bvbs_xml33_bau_x83",
        "bvbs_xml33_bau_x84",
    ] {
        assert!(
            supported_ids.contains(expected_supported),
            "expected supported fixture missing: {expected_supported}"
        );
    }

    for future_or_reference in manifest.fixtures.iter().filter(|fixture| {
        !matches!(fixture.id.as_str(), "dangl_ava_examples")
            && !matches!(
                fixture.id.as_str(),
                "bvbs_xml33_ava_x81"
                    | "bvbs_xml33_ava_x84"
                    | "bvbs_xml33_ava_x86"
                    | "dangl_ava_gaeb90_d83"
                    | "bvbs_xml33_bau_x83"
                    | "bvbs_xml33_bau_x84"
            )
    }) {
        assert!(
            matches!(
                future_or_reference.support_status.as_str(),
                "future_track" | "reference_only"
            ),
            "follow-on/reference fixture is overclaimed: {} ({})",
            future_or_reference.id,
            future_or_reference.support_status
        );
    }

    assert!(manifest.fixtures.iter().any(|fixture| {
        fixture.id == "official_gaeb_xml34_beta_schema"
            && fixture.support_status == "reference_only"
    }));
    assert!(manifest.fixtures.iter().any(|fixture| {
        fixture.id == "gaeb_online_generator_exe" && fixture.support_status == "reference_only"
    }));
}

#[test]
fn future_track_fixtures_have_catalog_tests_and_reference_fixtures_do_not_claim_tests() {
    let manifest = read_manifest();
    for fixture in &manifest.fixtures {
        match fixture.support_status.as_str() {
            "future_track" => assert!(
                !fixture.test_mapping.is_empty(),
                "future fixture needs a catalog/overclaim-prevention test mapping: {}",
                fixture.id
            ),
            "reference_only" => assert!(
                fixture.test_mapping.is_empty(),
                "reference-only fixture must not claim automated support tests: {}",
                fixture.id
            ),
            _ => {}
        }
    }
}

#[test]
fn phase_catalog_includes_requested_current_and_future_gaeb_phases() {
    let manifest = read_manifest();
    let phases = manifest
        .fixtures
        .iter()
        .map(|fixture| fixture.phase.as_str())
        .collect::<BTreeSet<_>>();

    for expected in ["x31", "x81", "x82", "x83", "x84", "x86", "d86", "schema"] {
        assert!(
            phases.contains(expected),
            "missing phase catalog entry: {expected}"
        );
    }
}

#[test]
fn bau_source_matrix_lists_xml33_xml32_xml31_x83_x84_pdf_criteria() {
    let manifest = read_manifest();

    for (version, domain, statuses) in [
        (
            "gaeb_xml_3_3",
            "construction_execution",
            [
                ("x83", "supported_parse_only"),
                ("x84", "supported_parse_only"),
                ("pdf_reference", "reference_only"),
                ("criteria_pdf", "reference_only"),
            ],
        ),
        (
            "gaeb_xml_3_2",
            "legacy_xml_3_2",
            [
                ("x83", "future_track"),
                ("x84", "future_track"),
                ("pdf_reference", "reference_only"),
                ("criteria_pdf", "reference_only"),
            ],
        ),
        (
            "gaeb_xml_3_1",
            "legacy_xml_3_1",
            [
                ("x83", "future_track"),
                ("x84", "future_track"),
                ("pdf_reference", "reference_only"),
                ("criteria_pdf", "reference_only"),
            ],
        ),
    ] {
        for (phase, support_status) in statuses {
            assert!(
                manifest.fixtures.iter().any(|fixture| {
                    fixture.source_family == "bvbs"
                        && fixture.gaeb_version == version
                        && fixture.process_domain == domain
                        && fixture.phase == phase
                        && fixture.support_status == support_status
                }),
                "missing Bau source-matrix row: {version}/{domain}/{phase}/{support_status}"
            );
        }
    }

    assert!(manifest.fixtures.iter().any(|fixture| {
        fixture.id == "bvbs_xml31_bau_criteria_supplement"
            && fixture.support_status == "reference_only"
    }));
}

#[test]
fn bau_legacy_sources_remain_gated_until_version_tests_pass() {
    let manifest = read_manifest();
    for fixture in manifest.fixtures.iter().filter(|fixture| {
        fixture.source_family == "bvbs"
            && matches!(
                fixture.process_domain.as_str(),
                "legacy_xml_3_1" | "legacy_xml_3_2"
            )
            && matches!(fixture.phase.as_str(), "x83" | "x84")
    }) {
        assert_eq!(
            fixture.support_status, "future_track",
            "legacy Bau parser support is overclaimed: {}",
            fixture.id
        );
    }
}

fn read_manifest() -> FixtureManifest {
    let text = fs::read_to_string("gaeb/manifest.toml").expect("manifest exists");
    manifest::parse(&text).expect("manifest parses")
}
