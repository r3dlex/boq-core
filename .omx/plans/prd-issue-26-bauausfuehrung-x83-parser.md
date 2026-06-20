# PRD: Implement Bauausführung X83 request-for-quotation parser support

## Issue
- GitHub issue: #26
- Milestone: v0.4 BVBS Bauausführung support

## Product outcome
Construction-execution X83 request-for-quotation data parses into the common hierarchical BoQ model while preserving tender-specific metadata/findings.

## Source/status anchors
- BVBS Bauausführung X83: `supported_parse_only` after #25 fixture promotion.
- GAEB XML 3.3 LV schema: `reference_only` schema.

## Blocking dependencies
- #25 fixture promotion/source-status decision is complete; this issue builds on the supported parse-only X83 fixture path.

## Requirements
- [x] Decide shared AVA/Bauausführung parser boundary: extend the existing GAEB XML parser boundary without adding an Obra backend dependency.
- [x] Extract project/BoQ metadata, sections, items, long texts, and tender-specific fields.
- [x] Emit findings for unsupported nodes.

## Specification artifacts
- Spec: `.omx/specs/issue-26-bauausfuehrung-x83-parser.md`
- Test spec: `.omx/plans/test-spec-issue-26-bauausfuehrung-x83-parser.md`

## Planned tests
- [x] `test_bau_x83_extracts_project_and_boq_metadata`
- [x] `test_bau_x83_sections_and_items_match_hierarchy`
- [x] `test_bau_x83_tender_specific_fields_are_preserved`
- [x] `test_bau_x83_unknown_nodes_emit_findings`
- [x] `test_bau_x83_adapter_compatibility_remains_capability_gated`
