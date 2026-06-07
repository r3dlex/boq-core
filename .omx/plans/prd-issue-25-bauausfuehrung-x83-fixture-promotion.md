# PRD: Promote BVBS Bauausführung X83 fixture to supported test target

## Issue
- GitHub issue: #25
- Milestone: v0.4 BVBS Bauausführung support

## Product outcome
BVBS Bauausführung X83 is promoted from future/cataloged fixture to supported integration target only with passing parser evidence.

## Source/status anchors
- BVBS Bauausführung X83: `future_track` / `future_track` until green.
- BVBS Bauausführung criteria PDF: `reference_only`.

## Requirements
- [ ] Map Bauausführung X83 differences from AVA before promotion.
- [ ] Add golden report and support-status promotion checks.
- [ ] Document readiness vs certification status.

## Planned tests
- [ ] `test_bvbs_bau_x83_manifest_status_is_future_until_green`
- [ ] `test_bau_x83_fixture_parses_to_boq_tree`
- [ ] `test_bau_x83_support_promotion_requires_evidence`
- [ ] `test_bau_x83_golden_report_matches`
