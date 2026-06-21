# XRechnung bridge plan from verified GAEB billing data

Issue: #36  
Status: planning/reference only  
Non-goal: this repository does not emit production XRechnung, EN 16931, UBL, or CII payloads in this lane.

## Boundary statement

`boq-core` may prepare verified GAEB billing evidence for a future bridge, but
XRechnung envelope generation remains a separately approved component. The bridge
must not be implemented or advertised until a follow-up ADR/PRD chooses standards,
validator tooling, dependency ownership, and legal/compliance responsibilities.

The current source-domain boundary is:

- X31 supplies measured quantity and progress evidence.
- X86 supplies contract-award baseline references.
- X89 supplies invoice headers, parties, invoice lines, taxes, payment data, and
  unsupported billing findings.
- XRechnung generation is absent and represented by `InvoiceDocument::xrechnung_boundary()`.

## Required verified upstream data

| Bridge input | Required fields | Upstream source | Verification gate |
| --- | --- | --- | --- |
| Seller party | Name, endpoint or tax identifier, postal/legal identity if available | X89 `InvoicePartyRole::Seller` | Parsed from license-safe fixture or marked missing in audit findings. |
| Buyer party | Name, buyer reference, endpoint if available | X89 `InvoicePartyRole::Buyer` | Parsed from license-safe fixture and reviewed for public-sector billing requirements. |
| Contract baseline | X86 document id, ordinal relation, contract/bid baseline kind | X86 + X89 `ContractReference` | Matched by deterministic document id and ordinal before bridge use. |
| Quantity evidence | X31 document id, ordinal relation, measured quantity/progress reference | X31 + X89 `QuantityEvidenceReference` | Matched by deterministic document id and ordinal; no bridge without measurement evidence. |
| Invoice line | Line id, BoQ ordinal, unit, quantity, unit price, net amount | X89 `InvoiceLine` | Parsed, recalculated, and checked against totals. |
| Tax breakdown | Tax category, rate, taxable amount, tax amount | X89 `TaxBreakdown` | Present for every bridged invoice line or explicitly blocked. |
| Payment terms | Terms, due date, payment reference, buyer reference | X89 `PaymentApplication` | Present or explicitly blocked before public-sector output. |
| Unsupported constructs | Any payment/tax/accounting fields not mapped by the source model | X89 findings | Preserved as blocking review findings for bridge readiness. |
| Source provenance | Source URI, GAEB version, phase, parser version, checksum | X31/X86/X89 source provenance | Required for every bridged input before any generated e-invoice artifact. |

## Mapping assumptions

- GAEB ordinals are correlation keys only; they are not legal invoice line identifiers
  unless a follow-up bridge explicitly maps them.
- Recalculated totals are consistency evidence, not tax-law validation.
- Missing X31, X86, tax, or payment data blocks production XRechnung generation.
- Reference-only XRechnung artifacts can guide requirements, but they are not parser
  fixtures and are not proof of conformance.
- The bridge must preserve source provenance and checksums for every GAEB input used.

## Standards and dependency evaluation gates

No external e-invoicing dependency is adopted by this issue. A future bridge must
complete a separate dependency/standards decision before implementation:

| Candidate area | Examples to evaluate later | Current decision |
| --- | --- | --- |
| Semantic invoice standard | EN 16931 and current German XRechnung profile | Reference only; no generated payload. |
| Syntax binding | UBL XML and/or UN/CEFACT CII XML | Not selected. |
| Validation tooling | KoSIT validator and Schematron/profile packages | Not vendored, not executed. |
| Transport/interoperability | Peppol BIS Billing and recipient-specific routing | Out of scope. |
| Rust dependencies | XML writer/schema/validator crates | No dependency added. |

## Production bridge entry criteria

A future implementation PR may start only when all are true:

1. A follow-up issue and PRD explicitly authorize XRechnung output work.
2. Standards and validation dependencies are selected in an ADR or equivalent
   dependency decision.
3. License-safe X31, X86, and X89 fixtures exist with checksums and local tests.
4. Required-field tests fail before implementation and pass after implementation.
5. Documentation continues to say `boq-core` source-domain models alone do not
   create XRechnung payloads.
