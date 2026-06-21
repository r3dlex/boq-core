#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::support::SupportStatus;
use boq_core::support::manifest::{self, FixtureEntry};

#[test]
fn test_manifest_catalogs_gaeb_xml31_and_xml32_sources() {
    let fixtures = fixture_map();
    for expected in [
        "official_gaeb_xml32_leistungsverzeichnis",
        "bvbs_xml32_ava_x81",
        "bvbs_xml32_ava_x84",
        "bvbs_xml32_ava_x86",
        "bvbs_xml32_bau_x83",
        "official_gaeb_xml31_muster_2009_12",
        "bvbs_xml31_bau_x83",
        "bvbs_xml31_bau_pdf",
    ] {
        assert!(
            fixtures.contains_key(expected),
            "missing XML compatibility fixture {expected}"
        );
        let fixture = &fixtures[expected];
        assert!(
            matches!(
                fixture.support_status.as_str(),
                "reference_only" | "future_track"
            ),
            "{expected} must not be promoted in issue #37"
        );
        assert_eq!(fixture.ci_policy, "download_on_demand");
        assert!(fixture.license_note.contains("Legacy") || fixture.license_note.contains("legacy"));
    }
}

#[test]
fn test_xml_version_detector_distinguishes_31_32_33_namespaces() {
    let xml31 = parse_namespace_fixture("3.1");
    let xml32 = parse_namespace_fixture("3.2");
    let xml33 = parse_namespace_fixture("3.3");

    assert_eq!(xml31.summary.version.as_deref(), Some("3.1"));
    assert_eq!(xml32.summary.version.as_deref(), Some("3.2"));
    assert_eq!(xml33.summary.version.as_deref(), Some("3.3"));
    assert_ne!(xml31.summary.version, xml33.summary.version);
    assert_ne!(xml32.summary.version, xml33.summary.version);
    assert!(xml31.metadata_contains_namespace_inference());
    assert!(xml32.metadata_contains_namespace_inference());
    assert!(!has_legacy_finding(&xml33));
}

#[test]
fn test_xml32_ava_fixtures_remain_future_track_until_parser_promotion() {
    let fixtures = fixture_map();
    for id in [
        "bvbs_xml32_ava_x81",
        "bvbs_xml32_ava_x84",
        "bvbs_xml32_ava_x86",
    ] {
        let fixture = fixtures.get(id).expect("XML 3.2 AVA fixture is cataloged");
        assert_eq!(fixture.support_status, "future_track");
        assert!(!fixture.test_mapping.is_empty());
    }

    let document = parse_namespace_fixture_with_uri(
        "3.2",
        "gaeb/bvbs/gaeb_xml_3_2/legacy_xml_3_2/ava/x81/legacy.X81",
    );
    assert_eq!(document.support_status, SupportStatus::FutureTrack);
    assert!(has_legacy_finding(&document));
}

#[test]
fn test_xml31_schema_sources_remain_reference_only() {
    let fixtures = fixture_map();
    for id in ["official_gaeb_xml31_muster_2009_12", "bvbs_xml31_bau_pdf"] {
        let fixture = fixtures.get(id).expect("XML 3.1 source is cataloged");
        assert_eq!(fixture.support_status, "reference_only");
        assert!(fixture.test_mapping.is_empty());
    }

    let document = parse_namespace_fixture_with_uri(
        "3.1",
        "gaeb/official_gaeb/gaeb_xml_3_1/musterdateien_2009_12/legacy.X81",
    );
    assert_eq!(document.support_status, SupportStatus::ReferenceOnly);
    assert!(has_legacy_finding(&document));
}

#[test]
fn test_unsupported_legacy_xml_features_emit_structured_findings() {
    let xml = r#"<GAEB xmlns="http://www.gaeb.de/GAEB_DA_XML/3.2"><GAEBInfo><Version>3.2</Version></GAEBInfo><BoQ><Item ID="001"><Description>Legacy item</Description><LegacyDiscount>5</LegacyDiscount></Item></BoQ></GAEB>"#;
    let document = boq_core::gaeb_xml::parse_str(xml, Some("legacy.X81".to_owned()))
        .expect("legacy XML fixture parses conservatively");

    assert!(has_legacy_finding(&document));
    assert!(document.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_unsupported_item_field"
            && finding.location.as_deref() == Some("001/LegacyDiscount")
    }));
    assert_eq!(
        document.boq.nodes[0].metadata["gaeb.unsupported.LegacyDiscount"],
        serde_json::json!("5")
    );
}

fn fixture_map() -> BTreeMap<String, FixtureEntry> {
    let manifest = manifest::parse(manifest::EMBEDDED_TOML).expect("embedded manifest parses");
    manifest
        .fixtures
        .into_iter()
        .map(|fixture| (fixture.id.clone(), fixture))
        .collect()
}

fn parse_namespace_fixture(version: &str) -> boq_core::model::GaebDocument {
    parse_namespace_fixture_with_uri(version, &format!("version-{version}.X81"))
}

fn parse_namespace_fixture_with_uri(
    version: &str,
    source_uri: &str,
) -> boq_core::model::GaebDocument {
    let xml = format!(
        r#"<GAEB xmlns="http://www.gaeb.de/GAEB_DA_XML/{version}"><BoQ><Item ID="001"><Description>Versioned item</Description></Item></BoQ></GAEB>"#
    );
    boq_core::gaeb_xml::parse_str(&xml, Some(source_uri.to_owned()))
        .expect("version namespace fixture parses")
}

trait CompatibilityAssertions {
    fn metadata_contains_namespace_inference(&self) -> bool;
}

impl CompatibilityAssertions for boq_core::model::GaebDocument {
    fn metadata_contains_namespace_inference(&self) -> bool {
        self.boq
            .metadata
            .get("gaeb.xml_version_inferred_from_namespace")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    }
}

fn has_legacy_finding(document: &boq_core::model::GaebDocument) -> bool {
    document
        .findings
        .iter()
        .any(|finding| finding.code == "gaeb_xml_legacy_version_compatibility")
}

#[test]
fn test_issue_37_planning_artifacts_bind_boundary_adr() {
    let prd = include_str!("../.omx/plans/prd-issue-37-gaeb-xml-31-32-compatibility.md");
    let spec = include_str!("../.omx/specs/issue-37-gaeb-xml-31-32-compatibility.md");
    let test_spec =
        include_str!("../.omx/plans/test-spec-issue-37-gaeb-xml-31-32-compatibility.md");
    let adr = include_str!("../.archgate/adrs/ARCH-006-gaeb-xml-31-32-compatibility-boundary.md");

    for artifact in [prd, spec, test_spec, adr] {
        for expected in ["ARCH-006", "3.1", "3.2", "reference_only", "future_track"] {
            assert!(artifact.contains(expected), "artifact missing {expected}");
        }
    }
}
