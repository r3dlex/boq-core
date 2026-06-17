# PRD: Implement X89 parser MVP with Rechnung fixtures

## Issue
- GitHub issue: #35
- Milestone: v0.7 Rechnung and XRechnung bridge

## Product outcome
X89 Rechnung fixtures parse into the invoice model and link to BoQ/baseline identifiers where present.

## Source/status anchors
- GAEB Rechnung schema: `reference_only`.
- official GAEB XML 3.3 Rechnung package: `reference_only`; parser MVP requires a planned manifest entry for any concrete X89 fixture before `future_track` promotion.

## Requirements
- [ ] Complete #34 invoice domain and fixture manifest registration before parser behavior.
- [ ] Parse invoice headers, line items, and ordinal/contract links.
- [ ] Emit findings for unsupported tax/payment fields.

## Planned tests
- [ ] `test_x89_fixture_parses_invoice_header`
- [ ] `test_x89_fixture_parses_line_items`
- [ ] `test_x89_links_to_ordinal_or_contract_item`
- [ ] `test_x89_unsupported_tax_or_payment_fields_emit_findings`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #35 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-01 | #34-#36 Rechnung/XRechnung bridge | manifested | official_gaeb_xml33_rechnung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R2-02 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-03 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
