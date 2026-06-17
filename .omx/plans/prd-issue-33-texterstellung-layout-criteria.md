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

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #33 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R5-03 | #32-#33 Texterstellung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
