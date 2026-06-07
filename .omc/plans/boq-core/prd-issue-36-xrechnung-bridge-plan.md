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
