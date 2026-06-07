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
