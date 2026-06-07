# PRD: Add golden reports for BVBS AVA X81/X84/X86 fixtures

## Issue
- GitHub issue: #17
- Milestone: v0.2 AVA certification readiness

## Product outcome
Deterministic golden reports exist for AVA certification-path fixtures and are compared by integration tests.

## Source/status anchors
- BVBS AVA X81/X84/X86: `supported`.
- Golden report schema: internal reproducibility evidence.

## Requirements
- [ ] Define stable golden report schema before snapshots.
- [ ] Capture hierarchy, items, findings, support status, and checksums.
- [ ] Fail tests on unreviewed golden drift.

## Planned tests
- [ ] `test_bvbs_ava_x81_golden_report_matches`
- [ ] `test_bvbs_ava_x84_golden_report_matches`
- [ ] `test_bvbs_ava_x86_golden_report_matches`
- [ ] `test_golden_reports_capture_support_status_and_findings`
