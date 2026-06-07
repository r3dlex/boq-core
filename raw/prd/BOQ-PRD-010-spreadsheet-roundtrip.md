---
id: BOQ-PRD-010
title: Spreadsheet roundtrip helper planning (issue #44)
status: pending
date: 2026-06-07
tags: [boq-core, gaeb, spreadsheet, roundtrip, exchange]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
issue_source: .omx/plans/prd-issue-44-spreadsheet-roundtrip.md
spec_source: .omx/specs/issue-44-spreadsheet-roundtrip.md
test_spec_source: .omx/plans/test-spec-issue-44-spreadsheet-roundtrip.md
milestone: v0.9 Non-certification exchange tracks
tracked_by: r3dlex/boq-core#44 (planning)
subrepo_binding:
  boq-core: boq-core/gaeb/manifest.toml
  workspace: docs/adr/WS-008-support-status-honesty-and-certification-boundary.md
acceptance_criteria:
  - First architecture decision: boundary ADR deciding core vs companion crate/examples-only for spreadsheet roundtrip helpers before adding dependencies
  - Per-source matrix: gaeb_online_import_template (reference_only, spreadsheet template), gaeb_online_excel_generator (reference_only, executable .exe), mwm_rialto_demo (reference_only, commercial demo), easy_gaeb_browser (reference_only, browser utility)
  - Concrete test names from the test spec: test_spreadsheet_sources_are_reference_only_non_executed, test_roundtrip_boundary_adr_exists_before_dependencies, test_oz_matching_reordered_columns_red_tests, test_inserted_columns_do_not_break_oz_matching_red_tests, test_missing_oz_rejects_roundtrip_red_tests
---

# BOQ-PRD-010: Spreadsheet roundtrip helper planning (issue #44)

Planning-only track. All sources are reference_only; the .exe is never downloaded/executed in CI, the commercial demo is reference-only, the browser utility is documentation-only.
