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
