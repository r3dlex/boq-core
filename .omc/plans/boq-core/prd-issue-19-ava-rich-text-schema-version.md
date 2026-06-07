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
- [ ] Define rich-text normalization contract before element expansion.
- [ ] Preserve plain text plus structured markup/table findings.
- [ ] Detect XML version/namespace without support overclaiming.

## Planned tests
- [ ] `test_ava_rich_text_preserves_plain_text_and_markup_findings`
- [ ] `test_ava_xhtml_table_is_structured_or_reported`
- [ ] `test_xml_version_metadata_is_extracted`
- [ ] `test_unknown_ava_nodes_emit_structured_findings`
