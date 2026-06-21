#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::gaeb90::{self, Gaeb90Encoding};
use boq_core::support::SupportStatus;
use boq_core::support::manifest::{self, FixtureEntry};

#[test]
fn test_gaeb90_adapter_gap_matrix_lists_required_fields() {
    let matrix = include_str!("../docs/fixtures/gaeb90-adapter-gap-matrix.md");
    for expected in [
        "ARCH-009",
        "document title",
        "phase/provenance",
        "item ordinal",
        "short text",
        "long text",
        "quantity/unit",
        "legacy irregularities",
        "No Obra backend changes",
    ] {
        assert!(matrix.contains(expected), "gap matrix missing {expected}");
    }

    let adr = include_str!("../.archgate/adrs/ARCH-009-gaeb90-adapter-promotion-gap.md");
    assert!(adr.contains("No blanket GAEB 90 support promotion"));
    assert!(adr.contains("checked Dangl GAEB 90 D83 fixture"));
}

#[test]
fn test_gaeb90_d81_d83_hierarchy_extraction_red_tests() {
    let d81 = gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("synthetic D81 remains parseable");
    assert_eq!(d81.support_status, SupportStatus::SupportedParseOnly);
    assert_eq!(d81.boq.nodes[0].ordinal, "0101001");
    assert_eq!(
        d81.boq.nodes[0].item.as_ref().expect("item").short_text,
        "Synthetic item"
    );
    assert!(ObraImportDocument::try_from_gaeb(&d81).is_err());

    let d83 =
        gaeb90::parse_file("gaeb/developer_examples/dangl_ava_examples/gaeb90/d83/gaeb90.d83")
            .expect("Dangl D83 fixture parses");
    assert_eq!(d83.support_status, SupportStatus::Supported);
    assert!(d83.capabilities.adapt_to_obra);
    assert!(d83.boq.nodes.len() >= 4);

    let import = ObraImportDocument::try_from_gaeb(&d83)
        .expect("fixture-backed GAEB 90 D83 is adapter compatible");
    assert_eq!(
        import.source.source_format,
        boq_core::model::GaebFormat::Gaeb90
    );
    assert!(
        import
            .line_items
            .iter()
            .any(|line| line.description == "Site Preparation")
    );
    assert!(import.wbs_nodes.iter().any(|node| node.code == "0101001"));
}

#[test]
fn test_gaeb90_windows1252_umlaut_decode_cases() {
    let bytes = b"210102002  NNN         00000600000m\xB2                                      000030\n25F\xFCllung                                                                 000031";
    let document = gaeb90::parse_bytes_with_encoding(
        bytes,
        Some("umlaut.d83".to_owned()),
        Gaeb90Encoding::Windows1252,
    )
    .expect("Windows-1252 D83 parses");

    assert_eq!(
        document.metadata["gaeb90.encoding"],
        serde_json::json!("windows-1252")
    );
    assert_eq!(
        document.boq.nodes[0]
            .item
            .as_ref()
            .expect("item")
            .short_text,
        "Füllung"
    );
    assert!(
        document
            .findings
            .iter()
            .all(|finding| finding.code != "gaeb90_encoding_fallback")
    );
}

#[test]
fn test_gaeb90_malformed_fixed_width_recovery_findings() {
    let document = gaeb90::parse_bytes(b"21\n25short text\n", Some("malformed.d83".to_owned()))
        .expect("malformed GAEB 90 remains recoverable");

    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    assert!(
        document
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb90_line_length")
    );
    assert!(
        document
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb90_malformed_ordinal")
    );
    assert_eq!(
        ObraImportDocument::try_from_gaeb(&document)
            .expect_err("malformed unmanifested GAEB 90 remains adapter-gated")
            .code,
        "obra_adapter_not_supported"
    );
}

#[test]
fn test_mwm_rialto_is_reference_only_non_executed() {
    let fixtures = fixture_map();
    let fixture = &fixtures["mwm_rialto_gaeb90_demo"];
    assert_eq!(fixture.source_family, "commercial_demo");
    assert_eq!(fixture.support_status, "reference_only");
    assert_eq!(fixture.ci_policy, "reference_only");
    assert!(fixture.test_mapping.is_empty());
    assert!(
        fixture
            .license_note
            .contains("never executed or downloaded in CI")
    );
}

#[test]
fn test_issue_40_artifacts_bind_gap_analysis() {
    let artifacts = [
        include_str!("../.omx/plans/prd-issue-40-gaeb90-adapter-compatible-promotion.md"),
        include_str!("../.omx/specs/issue-40-gaeb90-adapter-compatible-promotion.md"),
        include_str!("../.omx/plans/test-spec-issue-40-gaeb90-adapter-compatible-promotion.md"),
    ];
    for artifact in artifacts {
        for expected in [
            "#40",
            "ARCH-009",
            "adapter-compatible",
            "mwm_rialto_gaeb90_demo",
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
