#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::gaeb2000::{Gaeb2000Token, detect_pxx_phase, tokenize};
use boq_core::support::manifest::{self, FixtureEntry};

#[test]
fn test_gaeb2000_manifest_sources_are_future_or_reference_only() {
    let fixtures = fixture_map();
    for (id, expected_status) in [
        ("gaeb2000_priced_gist", "future_track"),
        ("dangl_ava_gaeb2000_examples", "future_track"),
        ("dangl_gaeb2000_sportheim_gist", "future_track"),
        ("gaeb2000_xml_mapping_chart", "reference_only"),
    ] {
        assert!(
            fixtures.contains_key(id),
            "missing GAEB 2000 source row {id}"
        );
        let fixture = &fixtures[id];
        assert_eq!(fixture.gaeb_version, "gaeb_2000");
        assert_eq!(fixture.support_status, expected_status);
        assert!(
            fixture.license_note.contains("future")
                || fixture.license_note.contains("reference")
                || fixture.license_note.contains("Reference")
        );
    }
}

#[test]
fn test_gaeb2000_tokenizer_handles_begin_end_nesting() {
    let source = "#begin[BoQ]\n#begin[Item]\n#qty[1.00]\n#end[Item]\n#end[BoQ]\n";
    let result = tokenize(source);

    assert!(result.findings.is_empty());
    assert_eq!(result.tokens.len(), 5);
    assert!(matches!(
        &result.tokens[0],
        Gaeb2000Token::Begin { keyword, line } if keyword == "BoQ" && *line == 1
    ));
    assert!(matches!(
        &result.tokens[2],
        Gaeb2000Token::Keyword { keyword, line, .. } if keyword == "qty" && *line == 3
    ));
    assert!(matches!(
        &result.tokens[4],
        Gaeb2000Token::End { keyword, line } if keyword == "BoQ" && *line == 5
    ));
}

#[test]
fn test_gaeb2000_tokenizer_reports_unclosed_begin_blocks() {
    let result = tokenize("#begin[BoQ]\n#begin[Item]\n#end[BoQ]\n");

    assert!(result.findings.iter().any(|finding| {
        finding.code == "gaeb2000_mismatched_end_block" && finding.location.as_deref() == Some("3")
    }));
    assert!(result.findings.iter().any(|finding| {
        finding.code == "gaeb2000_unclosed_begin_block"
            && finding.message.contains("BoQ")
            && finding.location.as_deref() == Some("1")
    }));
}

#[test]
fn test_gaeb2000_phase_detector_maps_p81_to_p86() {
    for phase in ["81", "82", "83", "84", "85", "86"] {
        let path = format!("sample.P{phase}");
        assert_eq!(detect_pxx_phase(&path).as_deref(), Some(phase));
    }
    assert_eq!(detect_pxx_phase("sample.X86"), None);
    assert_eq!(detect_pxx_phase("sample.D86"), None);
}

#[test]
fn test_gaeb2000_mapping_chart_is_not_used_as_runtime_support_evidence() {
    let fixtures = fixture_map();
    let mapping = &fixtures["gaeb2000_xml_mapping_chart"];
    assert_eq!(mapping.source_family, "interactive_schema");
    assert_eq!(mapping.support_status, "reference_only");
    assert_eq!(mapping.ci_policy, "reference_only");
    assert!(mapping.test_mapping.is_empty());

    let plan = include_str!("../docs/fixtures/gaeb2000-pxx-compatibility-plan.md");
    for expected in [
        "ARCH-008",
        "GAEB 90",
        "GAEB DA XML",
        "reference_only",
        "not runtime support evidence",
    ] {
        assert!(plan.contains(expected), "plan missing {expected}");
    }
}

#[test]
fn test_issue_39_artifacts_bind_gaeb2000_boundary() {
    let artifacts = [
        include_str!("../.omx/plans/prd-issue-39-gaeb-2000-pxx-compatibility.md"),
        include_str!("../.omx/specs/issue-39-gaeb-2000-pxx-compatibility.md"),
        include_str!("../.omx/plans/test-spec-issue-39-gaeb-2000-pxx-compatibility.md"),
        include_str!("../.archgate/adrs/ARCH-008-gaeb-2000-pxx-parser-boundary.md"),
    ];
    for artifact in artifacts {
        for expected in [
            "#39",
            "ARCH-008",
            "gaeb2000",
            "future_track",
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
