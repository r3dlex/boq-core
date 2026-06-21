#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::support::SupportStatus;
use boq_core::support::manifest::{self, FixtureEntry};

#[test]
fn test_gaeb_xml34_sources_are_reference_only() {
    let fixtures = fixture_map();
    for id in [
        "official_gaeb_xml34_beta_schema",
        "official_gaeb_xml34_beta_changelog",
    ] {
        assert!(
            fixtures.contains_key(id),
            "missing XML 3.4 beta fixture row {id}"
        );
        let fixture = &fixtures[id];
        assert_eq!(fixture.source_family, "official_gaeb");
        assert_eq!(fixture.gaeb_version, "gaeb_xml_3_4_beta");
        assert_eq!(fixture.support_status, "reference_only");
        assert_eq!(fixture.ci_policy, "download_on_demand");
        assert!(fixture.test_mapping.is_empty());
        assert!(fixture.license_note.contains("reference only"));
    }
}

#[test]
fn test_gaeb_xml34_does_not_promote_supported_versions() {
    let document =
        parse_xml34_beta_fixture("gaeb/official_gaeb/gaeb_xml_3_4_beta/schemata/sample.X81");

    assert_eq!(document.summary.version.as_deref(), Some("3.4"));
    assert_eq!(document.support_status, SupportStatus::ReferenceOnly);
    assert!(document.capabilities.reference_only);
    assert!(!document.capabilities.parse);
    assert!(document.findings.iter().any(|finding| {
        finding.code == "gaeb_xml_beta_version_reference_only"
            && finding.message.contains("not production parser support")
    }));
    assert_eq!(
        document.boq.metadata["gaeb.xml_version_inferred_from_namespace"],
        serde_json::json!(true)
    );
}

#[test]
fn test_beta_sustainability_fields_are_recorded_as_model_impact_notes() {
    let impact = include_str!("../docs/fixtures/gaeb-xml34-beta-impact.md");
    for expected in [
        "ARCH-007",
        "Sustainability descriptors",
        "Lifecycle descriptors",
        "Carbon / CO2 descriptors",
        "Changelog schema deltas",
        "reference_only",
    ] {
        assert!(impact.contains(expected), "impact notes missing {expected}");
    }

    let adr = include_str!("../.archgate/adrs/ARCH-007-gaeb-xml-34-beta-impact-boundary.md");
    for expected in [
        "sustainability",
        "lifecycle",
        "carbon",
        "No production support claim",
    ] {
        assert!(adr.contains(expected), "ADR missing {expected}");
    }
}

#[test]
fn test_no_bvbs_certification_claim_for_xml34_beta() {
    let docs = [
        include_str!("../docs/fixtures/gaeb-xml34-beta-impact.md"),
        include_str!("../.omx/plans/prd-issue-38-gaeb-xml-34-beta-tracking.md"),
        include_str!("../.omx/specs/issue-38-gaeb-xml-34-beta-tracking.md"),
        include_str!("../.omx/plans/test-spec-issue-38-gaeb-xml-34-beta-tracking.md"),
    ];
    for doc in docs {
        assert!(doc.contains("reference_only"));
        assert!(!doc.contains("GAEB XML 3.4 is supported"));
        assert!(!doc.contains("GAEB XML 3.4 production support"));
        assert!(!doc.contains("BVBS certification fixture for GAEB XML 3.4"));
    }
}

#[test]
fn test_issue_38_artifacts_bind_beta_adr_and_impact_doc() {
    let artifacts = [
        include_str!("../.omx/plans/prd-issue-38-gaeb-xml-34-beta-tracking.md"),
        include_str!("../.omx/specs/issue-38-gaeb-xml-34-beta-tracking.md"),
        include_str!("../.omx/plans/test-spec-issue-38-gaeb-xml-34-beta-tracking.md"),
    ];
    for artifact in artifacts {
        for expected in [
            "#38",
            "ARCH-007",
            "gaeb_xml34_beta_schema",
            "reference_only",
        ] {
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

fn parse_xml34_beta_fixture(source_uri: &str) -> boq_core::model::GaebDocument {
    let xml = r#"<GAEB xmlns="http://www.gaeb.de/GAEB_DA_XML/3.4"><BoQ><Item ID="001"><Description>Beta item</Description></Item></BoQ></GAEB>"#;
    boq_core::gaeb_xml::parse_str(xml, Some(source_uri.to_owned()))
        .expect("XML 3.4 beta fixture parses as reference-only impact data")
}
