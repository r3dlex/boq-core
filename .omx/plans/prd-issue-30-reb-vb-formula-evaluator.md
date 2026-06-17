# PRD: Implement REB-VB 23.003 formula evaluation MVP

## Issue
- GitHub issue: #30
- Milestone: v0.5 BVBS Mengenermittlung X31 support

## Product outcome
A safe MVP subset of REB-VB 23.003 formulas can be evaluated for certification-path X31 checks with explicit unsupported findings.

## Source/status anchors
- REB-VB 23.003: `reference_only` rules.
- BVBS X31: `future_track` inputs.

## Requirements
- [ ] Define supported formula subset and precision policy.
- [ ] Evaluate arithmetic/quantity formulas needed by fixtures.
- [ ] Never panic on bad input; return findings.

## Blocking dependencies
- Depends on #29 (X31 parser MVP) for fixture context and formula source data.

## Planned tests
- [ ] `test_reb_formula_simple_arithmetic`
- [ ] `test_reb_formula_quantity_result_precision`
- [ ] `test_reb_formula_unsupported_expression_yields_finding`
- [ ] `test_formula_evaluator_never_panics_on_bad_input`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #30 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-01 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R1-03 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R1-07 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_results_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-08 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for reference drawing | reference_only | Reference-only visual/layout aid; not executable as parser fixture. |
| R1-09 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_mengenermittlung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
