# PRD: Design X89 Rechnung invoice data model

## Issue
- GitHub issue: #34
- Milestone: v0.7 Rechnung and XRechnung bridge

## Product outcome
A GAEB X89 invoice domain model exists without conflating GAEB invoice data with XRechnung envelope generation.

## Source/status anchors
- GAEB Rechnung schema: `reference_only`.
- official GAEB XML 3.3 Rechnung package: `reference_only`; any BVBS/vendor X89 fixture must be added to the manifest before parser-promotion claims.
- Delivered model module: `src/x89.rs`.
- Boundary documentation: `docs/fixtures/x89-rechnung-model-design.md` and `docs/book/developer-guide.md`.

## Requirements
- [x] Separate GAEB invoice model from XRechnung envelope.
- [x] Represent invoice header, parties, line amounts, tax/payment data, totals, and contract links without parser-promotion claims.
- [x] Represent the relationship to X31 quantity evidence and X86 contract baselines.
- [x] Document that XRechnung generation is not supported by this model alone.
- [x] Identify validation/audit findings needed for public-sector billing readiness.

## Planned tests
- [x] `test_x89_domain_represents_invoice_header`
- [x] `test_x89_domain_represents_line_amounts`
- [x] `test_x89_domain_links_contract_baseline`
- [x] `test_x89_domain_does_not_claim_xrechnung_support`
- [x] `test_x89_audit_findings_identify_public_sector_billing_gaps`
- [x] `test_x89_payment_application_keeps_public_sector_references`

## Delivered behavior
- `boq_core::x89::InvoiceDocument` is a serializable Rechnung/X89 source-domain model.
- `InvoiceHeader`, `InvoiceParty`, `InvoiceLine`, `TaxBreakdown`, `PaymentApplication`, `InvoiceTotals`, `ContractReference`, and `QuantityEvidenceReference` model the issue #34 invoice concepts.
- `InvoiceDocument::recalculate_totals()` deterministically derives net/tax/gross totals from supplied line data.
- `InvoiceDocument::record_public_sector_audit_findings()` records missing X86 baseline, missing X31 evidence, missing tax breakdown, and missing payment terms findings.
- `InvoiceDocument::xrechnung_boundary()` and `xrechnung_generated = false` keep XRechnung output generation out of scope.
- No manifest support status was promoted; Rechnung fixtures remain reference-only.

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
