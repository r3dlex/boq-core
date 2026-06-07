# PRD: Implement Bauausführung X83 request-for-quotation parser support

## Issue
- GitHub issue: #26
- Milestone: v0.4 BVBS Bauausführung support

## Product outcome
Construction-execution X83 request-for-quotation data parses into the common hierarchical BoQ model while preserving tender-specific metadata/findings.

## Source/status anchors
- BVBS Bauausführung X83: `future_track`.
- GAEB XML 3.3 LV schema: `reference_only` schema.

## Blocking dependencies
- Depends on #25 fixture promotion/source-status decision.

## Requirements
- [ ] Decide shared AVA/Bauausführung parser boundary.
- [ ] Extract project/BoQ metadata, sections, items, long texts, and tender-specific fields.
- [ ] Emit findings for unsupported nodes.

## Planned tests
- [ ] `test_bau_x83_extracts_project_and_boq_metadata`
- [ ] `test_bau_x83_sections_and_items_match_hierarchy`
- [ ] `test_bau_x83_tender_specific_fields_are_preserved`
- [ ] `test_bau_x83_unknown_nodes_emit_findings`
