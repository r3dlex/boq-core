# PRD: Deepen AVA XML rich text and schema-version handling

## Issue
- GitHub issue: #19
- Milestone: v0.2 AVA certification readiness

## Product outcome
AVA XML parsing preserves rich text/tables, extracts schema-version metadata, and emits structured findings for unsupported nodes.

## Source/status anchors
- GAEB XML 3.3 LV schema: `reference_only` schema.
- BVBS AVA fixtures: `supported`.

## Requirements
- [x] Define rich-text normalization contract before element expansion.
- [x] Preserve plain text plus structured markup/table findings.
- [x] Detect XML version/namespace without support overclaiming.

## Planned tests
- [x] `test_ava_rich_text_preserves_plain_text_and_markup_findings`
- [x] `test_ava_xhtml_table_is_structured_or_reported`
- [x] `test_xml_version_metadata_is_extracted`
- [x] `test_unknown_ava_nodes_emit_structured_findings`

## Delivery evidence
- Parser change: AVA source URIs use the rich-description preservation path in `src/gaeb_xml/mod.rs`.
- Contract note: `docs/fixtures/ava-rich-text-schema-version.md`.
- Focused tests: `tests/bvbs_ava_integration.rs`.
- Golden reports refreshed because AVA long-text output shape changed from plain-only to preserved rich text.
