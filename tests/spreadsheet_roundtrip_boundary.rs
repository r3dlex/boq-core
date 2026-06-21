#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::support::manifest::{self, FixtureEntry};

#[test]
fn test_spreadsheet_sources_are_reference_only_non_executed() {
    let fixtures = fixture_map();
    for id in ["gaeb_online_import_template", "gaeb_online_generator_exe"] {
        let fixture = &fixtures[id];
        assert_eq!(fixture.source_family, "productivity_templates");
        assert_eq!(fixture.process_domain, "spreadsheet_reference");
        assert_eq!(fixture.support_status, "reference_only");
        assert!(fixture.test_mapping.is_empty());
    }

    let exe = &fixtures["gaeb_online_generator_exe"];
    assert_eq!(exe.phase, "exe");
    assert!(exe.license_note.contains("must never be executed in CI"));

    let mwm = &fixtures["mwm_rialto_gaeb90_demo"];
    assert_eq!(mwm.source_family, "commercial_demo");
    assert_eq!(mwm.support_status, "reference_only");
    assert_eq!(mwm.ci_policy, "reference_only");
    assert!(
        mwm.license_note
            .contains("never executed or downloaded in CI")
    );
}

#[test]
fn test_roundtrip_boundary_adr_exists_before_dependencies() {
    let adr = include_str!("../.archgate/adrs/ARCH-013-spreadsheet-roundtrip-boundary.md");
    for expected in [
        "Issue #44",
        "reference-only examples/companion-crate",
        "Do not add spreadsheet dependencies",
        "No new spreadsheet/binary dependency",
        "OZ/item ordinal",
    ] {
        assert!(adr.contains(expected), "ADR missing {expected}");
    }

    let cargo_toml = include_str!("../Cargo.toml");
    for forbidden_dependency in [
        "calamine",
        "umya-spreadsheet",
        "rust_xlsxwriter",
        "xlsxwriter",
    ] {
        assert!(
            !cargo_toml.contains(forbidden_dependency),
            "spreadsheet dependency {forbidden_dependency} requires a future ADR"
        );
    }
}

#[test]
fn test_oz_matching_reordered_columns_red_tests() {
    let headers = ["Kurztext", "Menge", "OZ", "Einheit"];
    let row = ["Concrete", "12.5", "01.02.0030", "m3"];
    let matched = extract_oz(&headers, &row).expect("OZ header exists");
    assert_eq!(matched, "01.02.0030");

    let matrix = include_str!("../docs/fixtures/spreadsheet-roundtrip-boundary.md");
    assert!(matrix.contains("reordered columns do not change matching"));
}

#[test]
fn test_inserted_columns_do_not_break_oz_matching_red_tests() {
    let headers = ["Kommentar", "OZ", "User helper", "Menge"];
    let row = ["manual note", "02.01.0005", "ignore me", "7"];
    let matched = extract_oz(&headers, &row).expect("OZ header exists despite helper columns");
    assert_eq!(matched, "02.01.0005");

    let matrix = include_str!("../docs/fixtures/spreadsheet-roundtrip-boundary.md");
    assert!(matrix.contains("ignore inserted non-GAEB helper columns"));
}

#[test]
fn test_missing_oz_rejects_roundtrip_red_tests() {
    let headers = ["Kurztext", "Menge", "Einheit"];
    let row = ["Concrete", "12.5", "m3"];
    let error = extract_oz(&headers, &row).expect_err("missing OZ must reject roundtrip");
    assert_eq!(error, "missing_oz_column");

    let matrix = include_str!("../docs/fixtures/spreadsheet-roundtrip-boundary.md");
    assert!(matrix.contains("reject updates that lack an"));
    assert!(matrix.contains("instead of guessing by row order"));
}

#[test]
fn test_issue_44_artifacts_bind_spreadsheet_boundary() {
    let artifacts = [
        include_str!("../.omx/plans/prd-issue-44-spreadsheet-roundtrip.md"),
        include_str!("../.omx/specs/issue-44-spreadsheet-roundtrip.md"),
        include_str!("../.omx/plans/test-spec-issue-44-spreadsheet-roundtrip.md"),
        include_str!("../.archgate/adrs/ARCH-013-spreadsheet-roundtrip-boundary.md"),
    ];
    for artifact in artifacts {
        for expected in ["#44", "spreadsheet", "reference_only", "OZ"] {
            assert!(artifact.contains(expected), "artifact missing {expected}");
        }
    }
}

fn extract_oz<'a>(headers: &[&str], row: &'a [&str]) -> Result<&'a str, &'static str> {
    let oz_index = headers
        .iter()
        .position(|header| {
            matches!(
                header.trim().to_ascii_lowercase().as_str(),
                "oz" | "item ordinal" | "ordnungszahl"
            )
        })
        .ok_or("missing_oz_column")?;
    row.get(oz_index)
        .copied()
        .filter(|value| !value.trim().is_empty())
        .ok_or("missing_oz_value")
}

fn fixture_map() -> BTreeMap<String, FixtureEntry> {
    let manifest = manifest::parse(manifest::EMBEDDED_TOML).expect("embedded manifest parses");
    manifest
        .fixtures
        .into_iter()
        .map(|fixture| (fixture.id.clone(), fixture))
        .collect()
}
