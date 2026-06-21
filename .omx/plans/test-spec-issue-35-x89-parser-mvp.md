# Test spec issue #35: X89 parser MVP

## Test intent

Define safe fixture-readiness and regression-test expectations for issue #35 without introducing live downloads, paid certification work, unsupported parser claims, or XRechnung output claims.

## Executable local tests

- `test_x89_fixture_parses_invoice_header` verifies version, phase, invoice id/date/type/currency/project, parties, and buyer reference.
- `test_x89_fixture_parses_line_items` verifies line ids, ordinals, descriptions, decimal comma parsing, units, unit prices, net/tax/gross totals, and child text fields.
- `test_x89_links_to_ordinal_or_contract_item` verifies document and line links to X86 contract baselines and X31 quantity evidence.
- `test_x89_unsupported_tax_or_payment_fields_emit_findings` verifies unsupported tax/payment constructs produce stable findings and no XRechnung generation.
- `test_x89_parse_file_reads_invoice_fixture` verifies the file entrypoint and checksum provenance.
- `test_x89_parser_reports_decimal_parse_errors` verifies invalid decimal inputs return `x89_decimal_parse_failed`.
- `test_x89_parser_reports_malformed_lines` verifies malformed XML returns `x89_xml_parse_failed`.

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-01 | #34-#36 Rechnung/XRechnung bridge | manifested | official_gaeb_xml33_rechnung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R2-02 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-03 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

## Executable local fixtures

- `tests/fixtures/synthetic/x89_invoice.X89` — license-safe local synthetic invoice fixture for parser MVP tests.

## Reference-only / planned gates

- `R2-01` -> manifested / Reference-only manifest artifact; not executable as parser fixture.
- `R2-02` -> artifact-only/reference / Schema/documentation reference for validation planning; not a parser fixture.
- `R2-03` -> artifact-only/reference / Schema/documentation reference for validation planning; not a parser fixture.
- `R2-05` -> artifact-only/reference / Tooling or guidance reference for roundtrip planning; not vendored or executed.

## Verification expectations

- Unit/integration tests may read only local files already declared in the repo or synthetic fixtures created for this issue.
- Planned fixture rows require license-safe acquisition, checksum recording, and manifest updates before any parser test consumes them.
- Documentation/schema/PDF rows can support review checklists but must not be asserted as parser executable fixtures.
- A no-overclaim grep must reject wording that implies BVBS certification completion, XRechnung output support, or supported parser status where the ledger says `planned-support` or `reference_only`.
