# PRD: Build BVBS AVA criteria evidence matrix

## Issue
- GitHub issue: #16
- Milestone: v0.2 AVA certification readiness

## Product outcome
BVBS AVA criteria are mapped to fixtures, parser assertions, golden reports, manual evidence, gaps, and support-status claims.

## Source/status anchors
- BVBS AVA PDF criteria: `reference_only`.
- BVBS AVA X81/X84/X86: `supported`.

## Requirements
- [x] Normalize criteria IDs and evidence statuses.
- [x] Link each criterion to fixture/parser/golden/manual evidence or an explicit gap.
- [x] Keep readiness distinct from official certification.

## Planned tests
- [x] `test_bvbs_ava_criteria_matrix_has_fixture_mapping`
- [x] `test_bvbs_ava_criteria_matrix_flags_manual_evidence`
- [x] `test_bvbs_ava_criteria_matrix_rejects_empty_status`
- [x] `test_bvbs_ava_criteria_matrix_links_golden_reports`

## Delivery evidence
- Matrix: `gaeb/criteria/bvbs_ava_matrix.toml`.
- Readiness note: `docs/fixtures/bvbs-ava-criteria-readiness.md`.
- Focused tests: `tests/ava_criteria.rs`.
- Golden report links remain explicit `planned:#17` entries until issue #17 lands.
