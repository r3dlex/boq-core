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
| GAEB 90 | D81, D83 | `supported_parse_only` for parser MVP paths | Parse and inspect hierarchy/items; adapter/export/roundtrip are not implied. |
| GAEB DA XML Bauausführung | X83, X84 | `supported_parse_only` for selected construction-execution fixture-backed paths | Parse and use the Obra adapter only when `adapt_to_obra` is true; schema validation, export, roundtrip, production support, and certification remain unclaimed. |
| GAEB XML 3.4 beta, X31, X89, Handel, Kosten/Kalkulation, Zeitvertrag | Follow-on domains | `future_track` or `reference_only` | Catalog/reference evidence only until implementation, fixtures, and tests promote support. |
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

When input is malformed but recoverable, `findings` explain what was preserved, normalized, or not yet supported. For example, GAEB 90 short lines emit `gaeb90_line_length`, blank item ordinals emit `gaeb90_malformed_ordinal`, and rich XML descriptions currently normalized to plain text emit `gaeb_xml_description_plain_text_only`.

## Obra adapter boundary

The Obra adapter converts only documents whose `SupportCapabilities` allow `adapt_to_obra`. Parse-only GAEB 90 inputs can be read and inspected, but adapter conversion is intentionally rejected until support is promoted by tests and manifest status. User-facing examples should stay on the public `boq_core::gaeb90`, `boq_core::gaeb_xml`, and `boq_core::adapter::obra` APIs; they must not import Obra backend modules.
