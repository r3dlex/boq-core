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
- [ ] Treat XRechnung as roadmap/reference until verified GAEB billing data exists.
- [ ] Build required-field mapping matrix.
- [ ] Feature-gate or omit export until criteria are met.

## Planned tests/checks
- [ ] `test_xrechnung_bridge_plan_requires_verified_quantities`
- [ ] `test_xrechnung_mapping_matrix_has_required_invoice_fields`
- [ ] `test_xrechnung_export_is_feature_gated_or_absent`
- [ ] `test_docs_do_not_claim_xrechnung_generation`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #36 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-04 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
