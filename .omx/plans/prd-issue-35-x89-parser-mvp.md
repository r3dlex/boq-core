# PRD: Implement X89 parser MVP with Rechnung fixtures

## Issue
- GitHub issue: #35
- Milestone: v0.7 Rechnung and XRechnung bridge

## Product outcome
X89 Rechnung fixtures parse into the invoice model and link to BoQ/baseline identifiers where present.

## Source/status anchors
- GAEB Rechnung schema: `reference_only`.
- official GAEB XML 3.3 Rechnung package: `reference_only`; no official package sample is vendored or executed in CI.
- Local parser fixture: `tests/fixtures/synthetic/x89_invoice.X89` (license-safe synthetic fixture modeled after the issue acceptance fields).
- Parser entrypoints: `boq_core::x89::parse_str` and `boq_core::x89::parse_file`.
- No `gaeb/manifest.toml` support-status promotion is included in this issue; official Rechnung remains reference-only.

## Requirements
- [x] Complete #34 invoice domain and fixture manifest registration before parser behavior.
- [x] Parse invoice headers, line items, and ordinal/contract links.
- [x] Extract quantities, prices, tax amounts, payment references, and totals where supported.
- [x] Emit findings for unsupported tax/payment fields.
- [x] Keep XRechnung generation out of scope.

## Planned tests
- [x] `test_x89_fixture_parses_invoice_header`
- [x] `test_x89_fixture_parses_line_items`
- [x] `test_x89_links_to_ordinal_or_contract_item`
- [x] `test_x89_unsupported_tax_or_payment_fields_emit_findings`
- [x] `test_x89_parse_file_reads_invoice_fixture`
- [x] `test_x89_parser_reports_decimal_parse_errors`
- [x] `test_x89_parser_reports_malformed_lines`

## Delivered behavior
- `boq_core::x89::parse_str` and `parse_file` parse GAEB XML X89/Rechnung payloads into `InvoiceDocument`.
- The parser maps invoice id/date/type/currency/project, parties, payment terms, line ordinals, quantities, units, unit prices, net amounts, tax amounts, X86 contract links, and X31 quantity evidence links.
- Unsupported tax/payment constructs become non-fatal `ValidationFinding` entries.
- Totals are recomputed from parsed line data.
- XRechnung output remains absent and `xrechnung_generated` remains false.

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #35 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-01 | #34-#36 Rechnung/XRechnung bridge | manifested | official_gaeb_xml33_rechnung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R2-02 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-03 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
