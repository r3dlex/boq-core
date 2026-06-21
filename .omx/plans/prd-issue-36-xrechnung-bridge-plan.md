# PRD: Plan XRechnung bridge from verified GAEB billing data

## Issue
- GitHub issue: #36
- Milestone: v0.7 Rechnung and XRechnung bridge

## Product outcome
A roadmap bridge maps verified GAEB X31/X86/X89 data toward XRechnung without prematurely implementing or claiming e-invoice generation.

## Source/status anchors
- XRechnung standard: `reference_only`.
- X31/X86/X89 models: prerequisite verified data.

## Requirements
- [x] Treat XRechnung as roadmap/reference until verified GAEB billing data exists.
- [x] Build required-field mapping matrix.
- [x] Feature-gate or omit export until criteria are met.

## Planned tests/checks
- [x] `test_xrechnung_bridge_plan_requires_verified_quantities`
- [x] `test_xrechnung_mapping_matrix_has_required_invoice_fields`
- [x] `test_xrechnung_export_is_feature_gated_or_absent`
- [x] `test_docs_do_not_claim_xrechnung_generation`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #36 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-04 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.

## Delivery notes

- No production XRechnung emission is implemented or claimed.
- `docs/fixtures/xrechnung-bridge-plan.md` records mapping assumptions, legal/compliance boundaries, required X31/X86/X89 data, and separate standards/dependency evaluation gates.
- Regression tests in `tests/xrechnung_bridge_plan.rs` keep the bridge plan, PRD, spec, and test spec synchronized.

## Acceptance evidence

- Mapping assumptions and legal/compliance boundaries live in `docs/fixtures/xrechnung-bridge-plan.md`.
- Required upstream X31/X86/X89 verification data is listed in the bridge-plan matrix.
- External e-invoicing standards and dependencies are explicitly deferred to a separate ADR/dependency decision.
- No production XRechnung emission is implemented or claimed in issue #36.
