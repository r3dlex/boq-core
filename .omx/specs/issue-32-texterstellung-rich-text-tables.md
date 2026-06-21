# Spec issue #32: Texterstellung rich text and tables

## Scope

Issue #32 adds parse-only Texterstellung support for GAEB DA XML 3.3 specification-authoring X81/X82 rows. The parser must retain rich description structures with explicit loss semantics and must not claim rendering, validation, adapter, export, roundtrip, or BVBS certification support.

## Source URI boundary

Rich-description preservation is enabled for specification-authoring/Texterstellung source URIs, including manifest-backed paths under:

- `gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x81`
- `gaeb/bvbs/gaeb_xml_3_3/specification_authoring/x82`

Existing AVA/Bau parser behavior remains plain-text normalized unless those paths are explicitly promoted in their own issues.

## Rich-description contract

For Texterstellung descriptions:

1. Plain searchable text remains available through `BoqItem.short_text` and the `RichTextFragment::Text` fragment when tables are present.
2. Descriptions with rich markup and no tables are preserved as `RichText::XhtmlFragment(inner_xml)`.
3. Descriptions with one or more XHTML tables are preserved as `RichText::Mixed` containing:
   - `RichTextFragment::Unknown(inner_xml)` for complete inner XML retention,
   - `RichTextFragment::Text(plain_text)` for extracted text,
   - `RichTextFragment::Table(table_markup)` for each table subtree.
4. X82 cost-estimate fields map to the existing `BoqItem` quantity/unit/price fields.
5. Layout/style and `TextComplement` semantics produce findings because they are preserved but not rendered/interpreted:
   - `gaeb_xml_texterstellung_layout_preserved_not_rendered`
   - `gaeb_xml_texterstellung_text_complement_preserved_as_markup`

## Support-status contract

- `bvbs_xml33_text_x81`: `supported_parse_only`.
- `bvbs_xml33_text_x82`: `supported_parse_only`.
- `bvbs_xml33_texterstellung_criteria_pdf`: `reference_only`.

Capabilities for X81/X82 remain parse-only: no Obra adapter, export, roundtrip, visual layout, or certification capability.

## Non-goals

- No paid BVBS certification action.
- No live/source download during unit tests.
- No parser support promotion beyond `supported_parse_only`.
- No duplicate issue creation; #32 remains the owning lane for rich-text/table parse readiness.
- No rendering or visual-layout equivalence claim.
- No text-complement business semantics beyond source markup retention.

## Ranked roadmap source audit

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R5-01 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x81 | supported_parse_only | `test_text_x81_rich_text_blocks_preserved`, `test_text_tables_normalize_to_document_blocks`, `test_text_unsupported_layout_emits_findings`, `test_texterstellung_support_promotion_requires_rich_text_evidence` |
| R5-02 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x82 | supported_parse_only | `test_text_x82_cost_estimate_metadata_preserved`, `test_text_unsupported_layout_emits_findings`, `test_text_tables_normalize_to_document_blocks`, `test_texterstellung_support_promotion_requires_rich_text_evidence` |
| R5-03 | #32-#33 Texterstellung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

## Acceptance criteria mapping

- BVBS Texterstellung X81/X82 fixtures have integration-style parser tests: covered by `tests/texterstellung.rs` synthetic local X81/X82 documents plus manifest row tests.
- Tables/lists/rich text are preserved or reported with loss-aware findings: covered by rich fragment/table tests and finding tests.
- Parser support status is promoted only for covered structures: covered by manifest test mappings and anti-overclaim support-status tests.
- Golden snapshots protect extracted text/table structure: covered by deterministic fragment assertions on full XML/table substrings and extracted text.
