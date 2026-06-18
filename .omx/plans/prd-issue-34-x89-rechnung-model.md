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

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #34 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-01 | #34-#36 Rechnung/XRechnung bridge | manifested | official_gaeb_xml33_rechnung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R2-02 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-03 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-04 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
