# PRD: Implement Texterstellung X81/X82 rich text and table support

## Issue
- GitHub issue: #32
- Milestone: v0.6 BVBS Texterstellung support

## Product outcome
Texterstellung X81/X82 rich text and table structures parse while preserving public BoQ hierarchy output and structured findings.

## Source/status anchors
- BVBS Texterstellung X81/X82: `future_track`.
- BVBS text criteria PDF: `reference_only`.

## Blocking dependencies
- Depends on #19 rich-text/table normalization contract being finalized.

## Requirements
- [ ] Define rich text/table representation shared with #19.
- [ ] Preserve rich text blocks, tables, layout-relevant findings, and estimate metadata.
- [ ] Avoid unsupported layout claims.

## Planned tests
- [ ] `test_text_x81_rich_text_blocks_preserved`
- [ ] `test_text_x82_cost_estimate_metadata_preserved`
- [ ] `test_text_tables_normalize_to_document_blocks`
- [ ] `test_text_unsupported_layout_emits_findings`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #32 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R5-01 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x81 | future_track | ['future_texterstellung_x81_cataloged'] |
| R5-02 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x82 | future_track | ['future_texterstellung_x82_cataloged'] |
| R5-03 | #32-#33 Texterstellung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
