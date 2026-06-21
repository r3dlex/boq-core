# Test spec issue #32: Texterstellung rich text and tables

## Test intent

Verify parse-only Texterstellung X81/X82 readiness without live downloads, paid certification actions, unsupported parser claims, rendering claims, or Obra adapter/export promotion.

## Executable local tests

| Test | Purpose |
| --- | --- |
| `test_text_x81_rich_text_blocks_preserved` | Parses a local X81 specification-authoring XML with styled paragraphs; verifies `supported_parse_only`, parse-only capabilities, preserved `XhtmlFragment`, extracted short text, and layout finding. |
| `test_text_tables_normalize_to_document_blocks` | Parses a table-heavy X81 description; verifies mixed rich text contains extracted `Text` plus preserved `Table` markup. |
| `test_text_x82_cost_estimate_metadata_preserved` | Parses a local X82 estimate item; verifies quantity, unit, unit price, total price, and text-complement XML fragment retention. |
| `test_text_unsupported_layout_emits_findings` | Verifies layout/style and text-complement preservation warnings are structured and stable. |
| `test_texterstellung_support_promotion_requires_rich_text_evidence` | Verifies X81/X82 manifest rows are `supported_parse_only` and list the #32 evidence tests. |
| `support_statuses_prevent_overclaiming_follow_on_tracks` | Ensures only explicitly evidence-backed rows are promoted while other future/reference rows remain non-supported. |

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R5-01 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x81 | supported_parse_only | `test_text_x81_rich_text_blocks_preserved`, `test_text_tables_normalize_to_document_blocks`, `test_text_unsupported_layout_emits_findings`, `test_texterstellung_support_promotion_requires_rich_text_evidence` |
| R5-02 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x82 | supported_parse_only | `test_text_x82_cost_estimate_metadata_preserved`, `test_text_unsupported_layout_emits_findings`, `test_text_tables_normalize_to_document_blocks`, `test_texterstellung_support_promotion_requires_rich_text_evidence` |
| R5-03 | #32-#33 Texterstellung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

## Verification expectations

- Unit/integration tests read only local synthetic XML embedded in test code or local files already declared in `boq-core/gaeb/manifest.toml`.
- Reference PDFs support review checklists but must not be asserted as parser executable fixtures.
- No-overclaim checks must reject wording that implies BVBS certification, adapter support, export, roundtrip, or rendered layout equivalence for Texterstellung rows.
- Coverage thresholds remain at >=95% regions/functions/lines for code changes.
