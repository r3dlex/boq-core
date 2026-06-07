# User Guide

This guide explains how to consume `boq-core` as a parser library and how to interpret the loss-aware BoQ output.

## Supported input families

`boq-core` currently distinguishes three GAEB families:

| Family | Examples | Current status |
| --- | --- | --- |
| GAEB 90 | D81, D83 | Parse-only MVP support for fixed-width records and selected fixtures. |
| GAEB DA XML | X81, X84, X86 | GAEB DA XML 3.3 AVA paths are the certification-readiness focus; X83 remains governed by manifest status until an AVA X83 fixture is promoted. |
| GAEB 2000 | P81..P86 | Cataloged for future implementation, not part of this docs MVP. |

D81 and X81 represent service-description/design-stage BoQs. D83 and X83
represent request-for-quotation flows at the GAEB phase level. In this crate,
current X83 fixture coverage remains `future_track`; the parser preserves phase
metadata when it can detect the file extension.

## Basic parsing flow

Use the family-specific parser while the public parse facade is still stabilizing:

```rust,no_run
let gaeb90 = boq_core::gaeb90::parse_file("gaeb/example.D81")?;
let xml = boq_core::gaeb_xml::parse_file("gaeb/example.X81")?;
# Ok::<(), boq_core::error::ParseError>(())
```

For in-memory data:

```rust,no_run
let source = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Minimal AVA</Name><BoQ><BoQBody /></BoQ></Project></GAEB>"#;
let document = boq_core::gaeb_xml::parse_str(source, Some("minimal_ava.x81".to_owned()))?;
assert_eq!(document.boq.title, "Minimal AVA");
# Ok::<(), boq_core::error::ParseError>(())
```

## Reading BoQ output

A parsed document contains:

- `summary` — detected GAEB format, version, phase, title, and project name.
- `source` — source URI, parser version, checksum, and provenance.
- `boq` — Obra-compatible hierarchy roots with nested nodes and line items.
- `support_status` and `capabilities` — explicit status flags that prevent accidental overclaiming.
- `findings` — recoverable parser or validation findings.

`BoqNode` values represent chapters, items, resources, or assemblies. Item
payloads carry short text, optional long text, quantity, unit, optional prices,
and metadata.

## Obra adapter boundary

The Obra adapter converts only documents whose `SupportCapabilities` allow
`adapt_to_obra`. Parse-only GAEB 90 inputs can be read and inspected, but adapter
conversion is intentionally rejected until support is promoted by tests and
manifest status.
