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
- [ ] Normalize criteria IDs and evidence statuses.
- [ ] Link each criterion to fixture/parser/golden/manual evidence or an explicit gap.
- [ ] Keep readiness distinct from official certification.

## Planned tests
- [ ] `test_bvbs_ava_criteria_matrix_has_fixture_mapping`
- [ ] `test_bvbs_ava_criteria_matrix_flags_manual_evidence`
- [ ] `test_bvbs_ava_criteria_matrix_rejects_empty_status`
- [ ] `test_bvbs_ava_criteria_matrix_links_golden_reports`
