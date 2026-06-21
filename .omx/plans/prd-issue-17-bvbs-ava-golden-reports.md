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
- [x] Define stable golden report schema before snapshots.
- [x] Capture hierarchy, items, findings, support status, and checksums.
- [x] Fail tests on unreviewed golden drift.

## Planned tests
- [x] `test_bvbs_ava_x81_golden_report_matches`
- [x] `test_bvbs_ava_x84_golden_report_matches`
- [x] `test_bvbs_ava_x86_golden_report_matches`
- [x] `test_golden_reports_capture_support_status_and_findings`

## Delivery evidence
- Golden reports: `gaeb/golden/bvbs_ava/x81-report.json`, `x84-report.json`, `x86-report.json`.
- Review workflow: `docs/fixtures/bvbs-ava-golden-reports.md`.
- Regression tests: `tests/bvbs_ava_integration.rs`.
- Criteria matrix links concrete report paths for X81/X84/X86.
