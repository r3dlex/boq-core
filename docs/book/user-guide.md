# User Guide

This guide explains how to consume `boq-core` as a parser library and how to interpret the loss-aware BoQ output. It uses only local files or in-memory strings; no paid tools, BVBS submissions, network access, or Obra backend imports are required.

## Quickstart: parse without network or paid dependencies

Use the family-specific parser while the public parse facade is still stabilizing. File parsing works with local paths:

```rust,no_run
let gaeb90 = boq_core::gaeb90::parse_file("gaeb/example.D81")?;
let xml = boq_core::gaeb_xml::parse_file("gaeb/example.X81")?;
# Ok::<(), boq_core::error::ParseError>(())
```

For examples and tests, prefer in-memory fixtures so the workflow is deterministic:

```rust
let source = include_str!("../../tests/fixtures/synthetic/minimal_ava.x81");
let document = boq_core::gaeb_xml::parse_str(
    source,
    Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
)?;

assert_eq!(document.boq.title, "Minimal AVA");
assert_eq!(document.summary.format, boq_core::model::GaebFormat::GaebXml);
# Ok::<(), boq_core::error::ParseError>(())
```

GAEB 90 bytes can be parsed directly. Legacy ANSI files can use the explicit Windows-1252 decoder when the caller knows the encoding:

```rust
let bytes = include_bytes!("../../tests/fixtures/synthetic/minimal.d81");
let document = boq_core::gaeb90::parse_bytes(bytes, Some("minimal.d81".to_owned()))?;
assert_eq!(document.summary.format, boq_core::model::GaebFormat::Gaeb90);
# Ok::<(), boq_core::error::ParseError>(())
```

## Supported formats and support vocabulary

`boq-core` uses manifest-backed support vocabulary. The table below uses the exact status words accepted by the architecture rules: `supported`, `supported_parse_only`, `future_track`, and `reference_only`.

| Family | Examples | Current status | User guidance |
| --- | --- | --- | --- |
| GAEB DA XML 3.3 AVA | X81, X84, X86 | `supported` for selected AVA fixture-backed paths | Parse, inspect support capabilities, and use the Obra adapter only when `adapt_to_obra` is true. |
| GAEB 90 | D81, D83 | `supported` only for the selected Dangl GAEB 90 D83 fixture path; `supported_parse_only` for D81 and unmanifested parser MVP paths | Parse and inspect hierarchy/items; use the Obra adapter only when `adapt_to_obra` is true. Export and roundtrip are not implied. |
| GAEB DA XML Bauausführung | X83, X84 | `supported_parse_only` for selected construction-execution fixture-backed paths | Parse and use the Obra adapter only when `adapt_to_obra` is true; schema validation, export, roundtrip, production support, and certification remain unclaimed. |
| GAEB DA XML X31 Mengenermittlung | Selected quantity-takeoff paths with synthetic parser evidence | `supported_parse_only` | Parse formula/result rows into canonical quantity evidence with provenance and findings; no BVBS fixture conformance, Obra adapter DTO, export, billing, full REB formula conformance, roundtrip, production support, or certification is implied. |
| GAEB DA XML Texterstellung | X81, X82 rich-text and table specification-authoring paths | `supported_parse_only` for selected rich-text/table parser-readiness paths | Parse and inspect rich text/table/text-complement preservation with loss findings; no Obra adapter DTO, visual rendering fidelity, export, roundtrip, production support, or certification is implied. |
| GAEB DA XML X89 Rechnung | Synthetic invoice-domain parser and Obra billing draft boundary | official Rechnung manifest entries remain `reference_only`; synthetic parser/billing-draft contract evidence only, not manifest support promotion | Parse invoice-domain data, derive `ObraBillingDraft`, and inspect readiness/loss findings; no XRechnung generation, production billing readiness, export, roundtrip, or certification is implied. |
| GAEB XML 3.4 beta, Handel, Kosten/Kalkulation, Zeitvertrag | Follow-on domains | `future_track` or `reference_only` | Catalog/reference evidence only until implementation, fixtures, and tests promote support. |
| External BVBS/GAEBXmlChecker evidence | Checker reports, certification notes | `reference_only` unless mirrored by tested parser behavior | Evidence helps readiness reviews but does not grant paid or official certification. |

D81 and X81 represent service-description/design-stage BoQs. D83 and X83 represent request-for-quotation flows at the GAEB phase level. In this crate, selected Bauausführung X83/X84 fixture paths are parser-backed and Obra-adapter-ready only when `adapt_to_obra` is true; export, roundtrip, schema validation, production support, and certification remain unclaimed.

## Reading BoQ output

A parsed document contains:

- `summary` — detected GAEB format, version, phase, title, and project name.
- `source` — source URI, parser version, checksum, and provenance.
- `boq` — hierarchy roots with nested `BoqNode` chapters/items.
- `support_status` and `capabilities` — explicit status flags that prevent accidental overclaiming.
- `findings` — recoverable parser or validation findings.

`BoqNode` values represent chapters, items, resources, or assemblies. Item payloads carry short text, optional long text, quantity, unit, optional prices, and metadata. A typical inspection flow is:

```rust
let source = include_str!("../../tests/fixtures/synthetic/minimal_ava.x81");
let document = boq_core::gaeb_xml::parse_str(source, Some("minimal_ava.x81".to_owned()))?;
let first = &document.boq.nodes[0];
let item = first.item.as_ref().expect("minimal fixture has an item payload");

assert_eq!(first.ordinal, "001.0010");
assert_eq!(item.quantity.to_string(), "12.50");
assert_eq!(item.unit, "m2");
assert!(item.short_text.contains("Concrete"));
# Ok::<(), boq_core::error::ParseError>(())
```

When input is malformed but recoverable, `findings` explain what was preserved, normalized, or not yet supported. For example, GAEB 90 short lines emit `gaeb90_line_length`, blank item ordinals emit `gaeb90_malformed_ordinal`, and generic rich XML descriptions outside supported fixture paths can emit `gaeb_xml_description_plain_text_only`.

## Canonical multi-standard annotations

Every `BoqItem` can carry typed `MultiStandardAnnotations` through the
source-compatible `try_multi_standard()` and `set_multi_standard()` accessors.
`try_multi_standard()` returns a finding when reserved metadata is malformed, so
corrupted provenance or loss evidence is not silently hidden. Parsers leave this empty unless tested evidence explicitly attaches
cross-standard context. The annotation model can carry classification
references, price/catalog references, quantity references, progress references,
source provenance, and loss findings for later Obra import design.

These annotations are not support status. A populated annotation set does not
promote a GAEB phase, catalog, billing flow, adapter conversion, export,
roundtrip, production path, or certification claim. Callers must continue to
read `support_status`, `capabilities`, and manifest-backed evidence before
enabling any downstream behavior.

## DIN 276 classification overlay

The `boq_core::din276` module can apply a deterministic, fixture-backed DIN 276
classification overlay to parsed BoQ items. It writes DIN 276
`ClassificationReference` values into `MultiStandardAnnotations` and preserves
the mapping source as provenance.

The overlay is evidence only. It does not change `support_status`, does not
grant Obra adapter support to parse-only inputs, and does not claim complete DIN
276 coverage. When a document is already adapter-capable, the Obra adapter can
carry DIN 276 classifications as DTO evidence alongside the GAEB ordinal
classification.

## CSI MasterFormat classification overlay

The `boq_core::csi_masterformat` module can apply a deterministic, fixture-backed CSI MasterFormat classification overlay to parsed BoQ items. It writes MasterFormat `ClassificationReference` values into `MultiStandardAnnotations` and preserves the mapping source as provenance.

The overlay is evidence only. It does not change `support_status`, does not grant Obra adapter support to parse-only inputs, and does not claim complete MasterFormat coverage. When a document is already adapter-capable, the Obra adapter can carry MasterFormat classifications as DTO evidence alongside the GAEB ordinal classification.

## Uniclass classification overlay

The `boq_core::uniclass` module can apply a deterministic, fixture-backed Uniclass classification overlay to parsed BoQ items. It writes Uniclass `ClassificationReference` values into `MultiStandardAnnotations` and preserves the mapping source as provenance.

The overlay is evidence only. It does not change `support_status`, does not grant Obra adapter support to parse-only inputs, does not acquire external Uniclass catalog data, and does not claim complete Uniclass coverage. When a document is already adapter-capable, the Obra adapter can carry Uniclass classifications as DTO evidence alongside the GAEB ordinal classification.

## NL-SfB classification overlay

The `boq_core::nlsfb` module can apply a deterministic, fixture-backed NL-SfB classification overlay to parsed BoQ items. It writes NL-SfB `ClassificationReference` values into `MultiStandardAnnotations` and preserves the mapping source as provenance.

The overlay is evidence only. It does not change `support_status`, does not grant Obra adapter support to parse-only inputs, does not acquire external NL-SfB catalog data, and does not claim complete NL-SfB coverage. When a document is already adapter-capable, the Obra adapter can carry NL-SfB classifications as DTO evidence alongside the GAEB ordinal classification.

## SINAPI catalog and BDI overlay

The `boq_core::sinapi` module can apply a deterministic, fixture-backed SINAPI catalog and BDI overlay to parsed BoQ items. It writes synthetic SINAPI `PriceCatalogReference` values into `MultiStandardAnnotations` and preserves the mapping source as provenance.

The overlay is evidence only. It does not change `support_status`, does not grant Obra adapter support to parse-only inputs, does not acquire external SINAPI catalog data, and does not claim complete SINAPI coverage. When a document is already adapter-capable, the Obra adapter can carry SINAPI catalog code and reference price evidence alongside the GAEB ordinal classification.

## Computo Metrico and Prezzario overlay

The `boq_core::prezzario` module can apply a deterministic, fixture-backed Computo Metrico and Prezzario overlay to parsed BoQ items. It writes synthetic Prezzario `PriceCatalogReference` values and Computo Metrico `QuantityReference` values into `MultiStandardAnnotations`, preserves the mapping source as provenance, and records regional formula handling as explicit loss findings.

The overlay is evidence only. It does not change `support_status`, does not grant Obra adapter support to parse-only inputs, does not acquire external Prezzario or Computo Metrico data, and does not claim complete Italian regional coverage. When a document is already adapter-capable, the Obra adapter can carry Prezzario catalog code and reference price evidence alongside the GAEB ordinal classification.

## GAEB 90 adapter-compatible boundary

The selected Dangl GAEB 90 D83 fixture path is the PHASE-10 adapter-compatible promotion. It is manifest-backed and test-backed, so callers may convert that parsed document to an Obra import DTO when `document.capabilities.adapt_to_obra` is true. The adapter output still carries source provenance, deterministic keys, parser findings, and loss-report fields.

D81 remains parse-only, and malformed or unmanifested GAEB 90 remains adapter-gated with `obra_adapter_not_supported`. The commercial `mwm_rialto_gaeb90_demo` entry remains `reference_only` and is never executed or downloaded in CI. This is no blanket GAEB 90 promotion: validation, export, roundtrip, production support, and certification remain unclaimed unless a later manifest row and tests explicitly promote them.

## Texterstellung rich-text evidence

Texterstellung X81/X82 paths are parser-readiness evidence only. The XML parser preserves rich descriptions as `RichText::XhtmlFragment` or mixed text/table fragments, keeps X82 cost-estimate item quantities/prices as parser-visible metadata, and emits findings such as `gaeb_xml_texterstellung_layout_preserved_not_rendered` and `gaeb_xml_texterstellung_text_complement_preserved_as_markup` when markup is preserved without rendering or semantic completion.

Callers must treat those findings as loss/provenance evidence. Texterstellung parser output is not an Obra adapter DTO, does not render page layout or exact fonts, does not export or roundtrip documents, and is not a BVBS certification claim.

## X31 quantity-takeoff evidence

X31 Mengenermittlung inputs are parser-backed with synthetic canonical-quantity evidence for selected paths; the cataloged BVBS X31 source remains download-on-demand evidence, not vendored conformance proof. The `x31` module preserves row provenance, formula source, explicit result quantities, units, attachment ids, parser findings, and X31-to-X86 ordinal link findings. Its progress report exposes canonical quantity evidence for later Obra import design, but `obra_import_supported` and `invoice_generated` stay `false`.

Callers must treat missing result quantities, unmatched ordinals, unit mismatches, and unsupported X31 fields as loss findings. These reports are not Obra adapter DTOs and do not generate XRechnung, payment claims, exports, roundtrips, production support, or certification evidence.

## X89 Rechnung billing draft boundary

X89 Rechnung inputs can be parsed into `boq_core::x89::InvoiceDocument` and adapted into `boq_core::x89::ObraBillingDraft` for Obra billing design. The draft carries source provenance, deterministic invoice and line keys, parties, totals, payment metadata, line-level contract/X31 quantity evidence links, parser findings, and a billing readiness gate.

The billing draft is not an XRechnung envelope and does not claim public-sector billing readiness by default. Missing X86 contract baselines, missing X31 quantity evidence, missing tax/payment data, or unsupported payment/tax fields remain blocking findings in `BillingReadiness`/`BillingLossReport`. Official Rechnung packages in the fixture manifest remain `reference_only` until separate manifest and fixture evidence promotes them.

## Obra adapter boundary

The Obra adapter converts only documents whose `SupportCapabilities` allow `adapt_to_obra`. Parse-only GAEB 90 inputs can be read and inspected, but adapter conversion is intentionally rejected unless support is promoted by tests and manifest status. User-facing examples should stay on the public `boq_core::gaeb90`, `boq_core::gaeb_xml`, and `boq_core::adapter::obra` APIs; they must not import Obra backend modules.
