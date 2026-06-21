# X89 Rechnung model design

Issue #34 introduces a source-domain model for GAEB X89/Rechnung invoice data.
It is intentionally not an XRechnung generator and does not promote any
reference-only Rechnung fixture in `gaeb/manifest.toml`.

## Boundary

- `boq_core::x89::InvoiceDocument` stores GAEB invoice-domain data.
- `InvoiceDocument::xrechnung_generated` is always initialized as `false`.
- `InvoiceDocument::xrechnung_boundary()` names a future separate bridge as the
  required component for XRechnung envelope generation.

## Relationships

- X86 contract baseline links are represented by `ContractReference`.
- X31 measurement/progress evidence links are represented by
  `QuantityEvidenceReference`.
- Invoice lines keep BoQ ordinals so future parsers can reconcile X89 values
  against X31 measurements and X86 contract awards without coupling to the Obra
  adapter or an XRechnung exporter.

## Public-sector billing audit findings

`InvoiceDocument::record_public_sector_audit_findings()` records non-fatal
findings for missing contract baselines, missing X31 quantity evidence, missing
tax breakdowns, and missing payment terms. These findings are readiness evidence
only; they are not certification or submission claims.
