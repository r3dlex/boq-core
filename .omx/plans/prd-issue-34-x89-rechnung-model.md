# PRD: Design X89 Rechnung invoice data model

## Issue
- GitHub issue: #34
- Milestone: v0.7 Rechnung and XRechnung bridge

## Product outcome
A GAEB X89 invoice domain model exists without conflating GAEB invoice data with XRechnung envelope generation.

## Source/status anchors
- GAEB Rechnung schema: `reference_only`.
- official GAEB XML 3.3 Rechnung package: `reference_only`; any BVBS/vendor X89 fixture must be added to the manifest before parser-promotion claims.

## Requirements
- [ ] Separate GAEB invoice model from XRechnung envelope.
- [ ] Represent invoice header, line amounts, taxes/payment findings, and contract links after manifest/source registration is settled.
- [ ] Document that XRechnung generation is not supported by this model alone.

## Planned tests
- [ ] `test_x89_domain_represents_invoice_header`
- [ ] `test_x89_domain_represents_line_amounts`
- [ ] `test_x89_domain_links_contract_baseline`
- [ ] `test_x89_domain_does_not_claim_xrechnung_support`
