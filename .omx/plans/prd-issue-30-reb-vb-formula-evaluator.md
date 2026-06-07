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
