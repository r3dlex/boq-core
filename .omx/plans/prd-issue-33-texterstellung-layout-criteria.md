# PRD: Add Texterstellung layout criteria matrix

## Issue
- GitHub issue: #33
- Milestone: v0.6 BVBS Texterstellung support

## Product outcome
Texterstellung BVBS layout criteria map to parser evidence, golden reports, manual evidence, and explicit gaps.

## Source/status anchors
- BVBS Texterstellung criteria: `reference_only`.
- Texterstellung X81/X82 fixtures: `future_track`.

## Requirements
- [ ] Define parser evidence vs layout/manual evidence.
- [ ] Cover all known criteria sections.
- [ ] Require matrix status before support/certification claims.

## Planned tests
- [ ] `test_text_criteria_matrix_covers_all_known_sections`
- [ ] `test_text_layout_manual_evidence_is_marked_manual`
- [ ] `test_text_fixture_golden_reports_link_criteria`
- [ ] `test_text_support_claims_require_matrix_status`
