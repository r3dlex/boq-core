# Developer Guide

This guide describes how to extend `boq-core` without weakening the support-status contract or Obra hierarchy compatibility.

## Architecture map

| Module | Responsibility |
| --- | --- |
| `model` | Loss-aware GAEB domain model and BoQ hierarchy. |
| `format` | Format and phase detection from paths/extensions. |
| `gaeb90` | Fixed-width GAEB 90 record decoding and parse-only document construction. |
| `gaeb_xml` | GAEB DA XML parser foundation and support-policy tagging. |
| `adapter::obra` | Obra adapter DTOs and deterministic import-key mapping. |
| `support` | `SupportStatus` and `SupportCapabilities` definitions. |
| `checksum` | Deterministic SHA-256 provenance helpers. |

## TDD expectations

Use TDD for new support claims:

1. Add or promote a fixture entry in `gaeb/manifest.toml`.
2. Add a failing unit or integration test for the exact phase/format behavior.
3. Implement the smallest parser or adapter change that makes the test pass.
4. Run targeted tests, then the broader quality gate.
5. Keep the 95% line/function/region coverage target in view for implementation PRs.

Do not promote a fixture to supported status unless parser behavior, adapter boundaries, and no-overclaiming tests agree.

## Fixture governance

`gaeb/manifest.toml` is the source of truth for fixture URLs, process domains,
phases, test mappings, CI policy, and support status. The manifest vocabulary is
intentionally conservative:

- `supported` — full supported capability set for the named flow.
- `supported_parse_only` — parsing works, but validation/adapter/export may not.
- `future_track` — planned follow-on work with catalog tests only.
- `reference_only` — reference material such as PDFs, schemas, GAEBXmlChecker, or beta packages.

Any new source must include a license note, target directory, and test mapping policy.

## Obra adapter contract

The Obra adapter preserves hierarchical intent:

- chapter/item structure maps to WBS node candidates;
- ordinal numbers drive deterministic keys and path candidates;
- item quantities, units, prices, and long texts map to line-item DTOs;
- findings and lossy mappings remain visible in the loss report.

The adapter must reject parse-only or reference-only inputs rather than silently producing unsupported Obra imports.

## Extension workflow

### X89/Rechnung model boundary

`boq_core::x89` is a GAEB invoice-domain model for X89/Rechnung planning. It
keeps invoice headers, parties, line amounts, tax/payment data, X86 contract
baseline links, X31 quantity evidence links, totals, and audit findings separate
from both the BoQ parser and any XRechnung envelope generator. A populated
`InvoiceDocument` is not parser support, not an Obra adapter DTO, and not an
XRechnung payload; use `InvoiceDocument::xrechnung_boundary()` to expose that
boundary explicitly.

For a new GAEB phase such as X83, X31, or X89:

1. Register or confirm fixture manifest entries.
2. Add parser tests using synthetic data first, then real fixture integration tests.
3. Capture support-policy decisions in tests before adding broader adapter behavior.
4. Update rustdoc and this book only after behavior exists.
5. Keep paid submission, external publishing, and official certification claims gated by explicit human authorization.
