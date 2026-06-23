#![allow(missing_docs, clippy::expect_used)]

use std::fs;
use std::path::Path;

#[test]
fn mdbook_documentation_mvp_scaffold_exists() {
    for path in [
        "book.toml",
        "docs/book/SUMMARY.md",
        "docs/book/index.md",
        "docs/book/user-guide.md",
        "docs/book/developer-guide.md",
        "docs/book/certification-evidence-guide.md",
        "docs/book/bvbs-certification-runbook.md",
        "docs/book/release-guide.md",
    ] {
        assert!(
            Path::new(path).exists(),
            "missing documentation artifact: {path}"
        );
    }
}

#[test]
fn documentation_guides_cover_required_mvp_topics() {
    let user_guide = fs::read_to_string("docs/book/user-guide.md").expect("user guide exists");
    for topic in [
        "GAEB 90",
        "GAEB DA XML",
        "D81",
        "D83",
        "X81",
        "X83",
        "Texterstellung",
        "X89 Rechnung billing draft boundary",
        "ObraBillingDraft",
        "supported_parse_only",
        "rich-text/table parser-readiness",
        "Obra",
    ] {
        assert!(
            user_guide.contains(topic),
            "user guide missing topic: {topic}"
        );
    }

    let developer_guide =
        fs::read_to_string("docs/book/developer-guide.md").expect("developer guide exists");
    for topic in ["TDD", "95%", "manifest", "SupportStatus", "Obra adapter"] {
        assert!(
            developer_guide.contains(topic),
            "developer guide missing topic: {topic}"
        );
    }

    let certification_guide = fs::read_to_string("docs/book/certification-evidence-guide.md")
        .expect("certification guide exists");
    for topic in [
        "BVBS",
        "GAEBXmlChecker",
        "reference_only",
        "paid certification",
    ] {
        assert!(
            certification_guide.contains(topic),
            "certification guide missing topic: {topic}"
        );
    }

    let release_guide =
        fs::read_to_string("docs/book/release-guide.md").expect("release guide exists");
    for topic in ["semver", "crates.io", "protected main", "full-green PR"] {
        assert!(
            release_guide.contains(topic),
            "release guide missing topic: {topic}"
        );
    }
}

#[test]
fn test_developer_guide_links_fixture_manifest() {
    let guide = fs::read_to_string("docs/book/developer-guide.md").expect("developer guide exists");
    for required in [
        "gaeb/manifest.toml",
        "fixture manifest",
        "checksums",
        "support_status",
        "supported",
        "supported_parse_only",
        "future_track",
        "reference_only",
        "ci_policy",
        "test_mapping",
        "ARCH-002",
        "ARCH-004",
    ] {
        assert!(
            guide.contains(required),
            "developer guide missing fixture governance anchor: {required}"
        );
    }
}

#[test]
fn test_developer_guide_states_95_coverage_policy() {
    let guide = fs::read_to_string("docs/book/developer-guide.md").expect("developer guide exists");
    for required in [
        "red-green-refactor",
        "95% line/function/region coverage",
        "cargo llvm-cov --all-features --summary-only",
        "--fail-under-lines 95",
        "--fail-under-functions 95",
        "--fail-under-regions 95",
        "local and GitHub CI",
    ] {
        assert!(
            guide.contains(required),
            "developer guide missing TDD/coverage anchor: {required}"
        );
    }
}

#[test]
fn test_certification_guide_mentions_paid_gate() {
    let guide = fs::read_to_string("docs/book/certification-evidence-guide.md")
        .expect("certification guide exists");
    for required in [
        "readiness evidence only",
        "explicit human authorization",
        "No paid submission",
        "No credential entry",
        "ARCH-004",
        "Issue #18",
        "gaeb/manifest.toml",
        "reference_only",
        "supported_parse_only",
    ] {
        assert!(
            guide.contains(required),
            "certification guide missing paid-gate anchor: {required}"
        );
    }
}

#[test]
fn test_certification_guide_uses_no_certified_claim() {
    let guide = fs::read_to_string("docs/book/certification-evidence-guide.md")
        .expect("certification guide exists");

    let forbidden = [
        "officially certified".to_owned(),
        "BVBS certified".to_owned(),
        "certified by BVBS".to_owned(),
        "paid certification completed".to_owned(),
        "certification achieved".to_owned(),
        "tooling_only".to_owned(),
        "certification_fixture".to_owned(),
        ["obra", "backend"].join("/"),
    ];
    for forbidden in forbidden {
        assert!(
            !guide.contains(&forbidden),
            "certification guide contains forbidden claim or vocabulary: {forbidden}"
        );
    }

    for required in [
        "AVA",
        "Bauausführung",
        "Mengenermittlung",
        "Texterstellung",
        "docs/fixtures/gaebxmlchecker-ava-evidence.md",
        "docs/fixtures/bvbs-ava-criteria-readiness.md",
        "docs/fixtures/bvbs-ava-golden-reports.md",
        "docs/fixtures/bvbs-bau-x83-readiness.md",
        "docs/fixtures/bvbs-texterstellung-criteria-readiness.md",
        "docs/fixtures/xrechnung-bridge-plan.md",
        "docs/fixtures/gaeb-xml34-beta-impact.md",
        "docs/fixtures/kosten-kalkulation-x50-x52-boundary.md",
        "docs/fixtures/handel-x93-x97-boundary.md",
        "docs/fixtures/zeitvertrag-x83z-x84z-boundary.md",
        "docs/fixtures/spreadsheet-roundtrip-boundary.md",
    ] {
        assert!(
            guide.contains(required),
            "certification guide missing evidence-track anchor: {required}"
        );
    }
}

#[test]
fn docs_do_not_overclaim_certification_or_future_formats() {
    let mut combined = String::new();
    for path in [
        "README.md",
        "src/lib.rs",
        "docs/book/index.md",
        "docs/book/user-guide.md",
        "docs/book/developer-guide.md",
        "docs/book/certification-evidence-guide.md",
        "docs/book/bvbs-certification-runbook.md",
        "docs/book/release-guide.md",
    ] {
        combined.push_str(&fs::read_to_string(path).expect("guide exists"));
        combined.push('\n');
    }

    // Exact sentinels intentionally cover the highest-risk support and certification
    // phrases; human review covers broader paraphrases in the certification checklist.
    for forbidden in [
        "officially BVBS certified",
        "paid certification completed",
        "X31 is supported",
        "X31 support is included",
        "X83 is supported",
        "X83 support is included",
        "Texterstellung is supported",
        "Texterstellung support is included",
        "X89 is supported",
        "X89 support is included",
        "GAEB XML 3.4 is supported",
        "GAEB XML 3.4 production support",
    ] {
        assert!(
            !combined.contains(forbidden),
            "documentation overclaims unsupported status: {forbidden}"
        );
    }
}

#[test]
fn test_paid_cert_runbook_requires_human_authorization() {
    let runbook = fs::read_to_string("docs/book/bvbs-certification-runbook.md")
        .expect("BVBS certification runbook exists");
    for required in [
        "Issue #7",
        "explicit human authorization",
        "No paid submission",
        "No credential entry",
        "No external contact",
        "budget owner",
        "account owner",
        "submission owner",
        "ARCH-004",
        "stop condition",
    ] {
        assert!(
            runbook.contains(required),
            "runbook missing human-authorization anchor: {required}"
        );
    }
}

#[test]
fn test_cert_readiness_checklist_references_green_pr_gates() {
    let runbook = fs::read_to_string("docs/book/bvbs-certification-runbook.md")
        .expect("BVBS certification runbook exists");
    for required in [
        "Evidence bundle",
        "PR #91",
        "PR #92",
        "PR #93",
        "PR #94",
        "PR #96",
        "Rust quality gates",
        "review threads",
        "local docs gate",
        "GH CI",
        "mergeStateStatus CLEAN",
    ] {
        assert!(
            runbook.contains(required),
            "runbook missing green-gate anchor: {required}"
        );
    }
}

#[test]
fn test_runbook_distinguishes_readiness_from_certified_status() {
    let runbook = fs::read_to_string("docs/book/bvbs-certification-runbook.md")
        .expect("BVBS certification runbook exists");
    for required in [
        "readiness status",
        "official-result status",
        "certified wording becomes allowed only after",
        "official result artifact",
        "do not update support_status solely because the runbook exists",
        "reference_only",
        "supported_parse_only",
    ] {
        assert!(
            runbook.contains(required),
            "runbook missing readiness-vs-official-status anchor: {required}"
        );
    }

    for forbidden in [
        "officially BVBS certified",
        "paid certification completed",
        "certification achieved",
        "tooling_only",
        "certification_fixture",
    ] {
        assert!(
            !runbook.contains(forbidden),
            "runbook contains forbidden claim or vocabulary: {forbidden}"
        );
    }
}

#[test]
fn test_user_guide_links_supported_formats() {
    let guide = fs::read_to_string("docs/book/user-guide.md").expect("user guide exists");
    for required in [
        "Quickstart: parse without network or paid dependencies",
        "GAEB DA XML 3.3 AVA",
        "GAEB 90",
        "supported",
        "supported_parse_only",
        "future_track",
        "reference_only",
        "parse_bytes",
        "parse_str",
    ] {
        assert!(
            guide.contains(required),
            "user guide missing supported-format anchor: {required}"
        );
    }
}

#[test]
fn test_user_guide_x89_status_uses_manifest_vocabulary() {
    let guide = fs::read_to_string("docs/book/user-guide.md").expect("user guide exists");
    let x89_row = guide
        .lines()
        .find(|line| line.starts_with("| GAEB DA XML X89 Rechnung |"))
        .expect("X89 user-guide support row exists");

    assert!(
        x89_row.contains("`reference_only`"),
        "X89 status row must anchor on manifest support vocabulary: {x89_row}"
    );
    assert!(
        x89_row.contains("not manifest support promotion"),
        "X89 status row must distinguish contract evidence from manifest support: {x89_row}"
    );
    assert!(
        !x89_row.contains("parser/billing-draft evidence; no manifest support promotion"),
        "X89 status row must not use ad-hoc support status wording: {x89_row}"
    );
}

#[test]
fn test_user_guide_warns_reference_only_sources() {
    let guide = fs::read_to_string("docs/book/user-guide.md").expect("user guide exists");
    for required in [
        "reference_only",
        "does not grant paid or official certification",
        "paid tools",
        "network access",
        "Obra backend modules",
    ] {
        assert!(
            guide.contains(required),
            "user guide missing reference-only warning: {required}"
        );
    }
}

#[test]
fn test_user_guide_explains_boq_output_fields() {
    let guide = fs::read_to_string("docs/book/user-guide.md").expect("user guide exists");
    for required in [
        "hierarchy roots",
        "BoqNode",
        "quantity",
        "unit",
        "long text",
        "findings",
        "gaeb90_line_length",
        "gaeb_xml_description_plain_text_only",
    ] {
        assert!(
            guide.contains(required),
            "user guide missing output interpretation anchor: {required}"
        );
    }
}
