# PRD: Promote BVBS Bauausführung X83 fixture to supported test target

## Issue
- GitHub issue: #25
- Milestone: v0.4 BVBS Bauausführung support

## Product outcome
BVBS Bauausführung X83 is promoted from future/cataloged fixture to `supported_parse_only` parser-readiness coverage with explicit evidence, while X84 and certification claims remain gated.

## Source/status anchors
- BVBS Bauausführung X83: promoted to `supported_parse_only` parser-readiness coverage.
- BVBS Bauausführung X84: remains `future_track` until issue #27.
- BVBS Bauausführung criteria PDF: remains `reference_only` / manual evidence.

## Requirements
- [x] Map Bauausführung X83 differences from AVA before promotion.
  - X83 import is parser-readiness only: detect+parse, no adapter/export/roundtrip/certification promotion.
- [x] Add golden report and support-status promotion checks.
  - Added manifest, criteria-matrix, and golden-readiness report checks.
- [x] Document readiness vs certification status.
  - Added `docs/fixtures/bvbs-bau-x83-readiness.md` with explicit non-certification boundaries.

## Implemented tests
- [x] `test_bvbs_bau_x83_manifest_status_is_future_until_green`
- [x] `test_bau_x83_fixture_parses_to_boq_tree`
- [x] `test_bau_x83_support_promotion_requires_evidence`
- [x] `test_bau_x83_golden_report_matches`

## Verification
- [x] `cargo test --test bau_roundtrip`
- [x] `cargo test --test testing_strategy`
- [x] `cargo test manifest_keeps_supported_parse_only_bau_x83_and_future_x84`
