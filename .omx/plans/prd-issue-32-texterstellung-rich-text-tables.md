# PRD: Implement Texterstellung X81/X82 rich text and table support

## Issue
- GitHub issue: #32
- Milestone: v0.6 BVBS Texterstellung support

## Product outcome
Texterstellung X81/X82 fixtures can be parsed in a loss-aware way for specification-authoring review: rich text markup, XHTML tables, text-complement markup, quantities, units, and estimate prices are preserved in the public rich model with structured findings for layout semantics that are stored but not rendered/certified.

## Delivered scope
- Promote BVBS Texterstellung X81/X82 manifest rows from `future_track` to `supported_parse_only` only.
- Preserve Texterstellung description markup as `RichText::XhtmlFragment` for rich text blocks without tables.
- Preserve table-heavy descriptions as `RichText::Mixed` with:
  - `Unknown(full_inner_xml)` for complete source-fragment retention,
  - `Text(plain_text)` for searchable/extracted text,
  - one `Table(markup)` fragment per XHTML table.
- Preserve X82 cost-estimate item fields already represented by the BoQ model (`Qty`, `QU`, `UP`, `IT`).
- Emit loss-aware findings when layout/style or text-complement semantics are preserved as markup but not rendered or interpreted.

## Source/status anchors
- BVBS Texterstellung X81: `supported_parse_only` via `bvbs_xml33_text_x81`.
- BVBS Texterstellung X82: `supported_parse_only` via `bvbs_xml33_text_x82`.
- BVBS text criteria PDF: `reference_only`.

## Non-goals / guardrails
- No paid BVBS submission or certification claim.
- No Obra adapter promotion for Texterstellung rows.
- No export, roundtrip, rendering, visual-layout equivalence, or text-complement completion semantics.
- No live downloads in tests; synthetic local XML fixtures exercise the parser contract.

## Requirements
- [x] Preserve rich text blocks without flattening them to plain text on Texterstellung paths.
- [x] Preserve XHTML tables as table fragments and retain searchable text.
- [x] Preserve X82 cost-estimate item metadata in the existing BoQ item fields.
- [x] Report unsupported layout/text-complement semantics with structured findings.
- [x] Promote parser support status only to `supported_parse_only` and only with test evidence.

## Verification
- `test_text_x81_rich_text_blocks_preserved`
- `test_text_x82_cost_estimate_metadata_preserved`
- `test_text_tables_normalize_to_document_blocks`
- `test_text_unsupported_layout_emits_findings`
- `test_texterstellung_support_promotion_requires_rich_text_evidence`
- `support_statuses_prevent_overclaiming_follow_on_tracks`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #32 owns the following source rows for parser-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R5-01 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x81 | supported_parse_only | `test_text_x81_rich_text_blocks_preserved`, `test_text_tables_normalize_to_document_blocks`, `test_text_unsupported_layout_emits_findings`, `test_texterstellung_support_promotion_requires_rich_text_evidence` |
| R5-02 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x82 | supported_parse_only | `test_text_x82_cost_estimate_metadata_preserved`, `test_text_unsupported_layout_emits_findings`, `test_text_tables_normalize_to_document_blocks`, `test_texterstellung_support_promotion_requires_rich_text_evidence` |
| R5-03 | #32-#33 Texterstellung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat reference-only rows as review/planning evidence only until explicitly promoted in the manifest and test plan.
