//! Regression checks for the issue #36 `XRechnung` bridge planning boundary.

const PLAN: &str = include_str!("../docs/fixtures/xrechnung-bridge-plan.md");
const PRD: &str = include_str!("../.omx/plans/prd-issue-36-xrechnung-bridge-plan.md");
const SPEC: &str = include_str!("../.omx/specs/issue-36-xrechnung-bridge-plan.md");
const TEST_SPEC: &str = include_str!("../.omx/plans/test-spec-issue-36-xrechnung-bridge-plan.md");
const DEVELOPER_GUIDE: &str = include_str!("../docs/book/developer-guide.md");
const LIB_RS: &str = include_str!("../src/lib.rs");
const X89_RS: &str = include_str!("../src/x89.rs");

#[test]
fn test_xrechnung_bridge_plan_requires_verified_quantities() {
    for required in [
        "X31 supplies measured quantity and progress evidence",
        "X86 supplies contract-award baseline references",
        "X89 supplies invoice headers",
        "no bridge without measurement evidence",
        "Missing X31, X86, tax, or payment data blocks production XRechnung generation",
    ] {
        assert!(
            PLAN.contains(required),
            "missing bridge prerequisite: {required}"
        );
    }
}

#[test]
fn test_xrechnung_mapping_matrix_has_required_invoice_fields() {
    for required in [
        "Seller party",
        "Buyer party",
        "Contract baseline",
        "Quantity evidence",
        "Invoice line",
        "Tax breakdown",
        "Payment terms",
        "Unsupported constructs",
        "Source provenance",
    ] {
        assert!(
            PLAN.contains(required),
            "missing mapping row or requirement: {required}"
        );
    }
}

#[test]
fn test_xrechnung_export_is_feature_gated_or_absent() {
    assert!(PLAN.contains("Non-goal: this repository does not emit production XRechnung"));
    assert!(PLAN.contains("No external e-invoicing dependency is adopted by this issue"));
    assert!(PLAN.contains("Rust dependencies") && PLAN.contains("No dependency added"));
    assert!(X89_RS.contains("xrechnung_generated: false"));
    assert!(X89_RS.contains("required_bridge: \"xrechnung-bridge\""));
    assert!(!LIB_RS.contains("pub mod xrechnung"));
}

#[test]
fn test_docs_do_not_claim_xrechnung_generation() {
    for (name, text) in [
        ("bridge plan", PLAN),
        ("PRD", PRD),
        ("spec", SPEC),
        ("test spec", TEST_SPEC),
        ("developer guide", DEVELOPER_GUIDE),
    ] {
        let lower = text.to_lowercase();
        for forbidden in [
            "xrechnung generation is supported",
            "generates xrechnung",
            "production xrechnung emission is supported",
            "xrechnung payload support",
        ] {
            assert!(
                !lower.contains(forbidden),
                "{name} overclaims XRechnung support with phrase: {forbidden}"
            );
        }
    }
}

#[test]
fn test_issue_36_artifacts_stay_in_sync() {
    for text in [PRD, SPEC, TEST_SPEC] {
        assert!(text.contains("R2-04"));
        assert!(text.contains("R2-05"));
        assert!(text.contains("reference_only"));
        assert!(text.contains("No production XRechnung emission"));
    }
}
