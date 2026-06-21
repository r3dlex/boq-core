# Spec issue #35: X89 parser MVP

## Scope

Implement a conservative GAEB XML X89/Rechnung parser MVP that targets the #34 invoice-domain model and uses local, license-safe fixtures only.

## Non-goals

- No paid BVBS certification action.
- No live/source download during unit tests.
- No parser support promotion beyond the parser support status recorded in the canonical ledger.
- No duplicate issue creation; this issue remains the owning lane for the rows listed below.
- No XRechnung envelope generation.
- No Obra adapter coupling.

## Parser requirements

- `parse_str` and `parse_file` return `InvoiceDocument`.
- Source provenance records GAEB XML, detected/default phase `89`, version text, checksum, and parser version.
- Invoice root attributes map to `InvoiceHeader`.
- Party elements map to `InvoiceParty`.
- Line elements map to `InvoiceLine`, including ordinal, description, quantity, unit, unit price, net amount, tax, line-level contract reference, and X31 quantity evidence.
- Document-level contract and quantity evidence references are preserved.
- Payment attributes preserve terms, due date, payment reference, and buyer reference.
- Unsupported tax/payment constructs are reported as findings rather than silently dropped.
- Invalid decimals and malformed XML return stable `ParseError` codes.

## Delivered files

- `src/x89.rs`
- `tests/x89_parser.rs`
- `tests/fixtures/synthetic/x89_invoice.X89`
- `.omx/plans/prd-issue-35-x89-parser-mvp.md`
- `.omx/plans/test-spec-issue-35-x89-parser-mvp.md`
- `.omx/specs/issue-35-x89-parser-mvp.md`

## Ranked roadmap source audit

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-01 | #34-#36 Rechnung/XRechnung bridge | manifested | official_gaeb_xml33_rechnung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R2-02 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-03 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

## Acceptance criteria

- The PRD, spec, and test spec for issue #35 all reference the same canonical source IDs.
- Manifest-backed rows can only be used through local fixture manifest entries.
- Planned or artifact-only rows remain documented as research/reference gates until a license-safe local fixture is added and checksummed.
- Certification PDFs and visual reference PDFs are treated as reference-only evidence, not executable parser fixtures.
- Parser MVP has local fixture coverage without changing official Rechnung support status.

## Implementation handoff

Use `.omx/specs/gaeb-ranked-source-status-ledger.md` as the source of truth for dispositions. Update `boq-core/gaeb/manifest.toml` and a follow-up test spec before promoting any planned row to broader fixture-backed execution.
