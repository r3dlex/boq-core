# GAEB 90 adapter-compatible gap matrix

No Obra backend changes are allowed; adapter-compatible means boq-core DTO compatibility only.

Issue: #40
ADR: ARCH-009
Status: selected fixture-backed D83 adapter-compatible promotion; no blanket GAEB 90 promotion.

| Required field | Source record | Current parser behavior | Adapter implication | Gate |
| --- | --- | --- | --- | --- |
| document title | `02` | preserved as BoQ title/project name | `ObraBoqDocument.title` | required |
| phase/provenance | `.D81`/`.D83` path and checksum | preserved in `SourceProvenance` | deterministic import keys | required |
| item ordinal | `21` payload positions 1-9 | item ordinal or fallback with finding | WBS code/path/classification | required |
| short text | `25` | item title/short text | line item description | required |
| long text | `26` sequence | plain rich text | line item long text | required |
| quantity/unit | `21` fixed-width fields | best-effort unit, zero quantity until exact semantics are implemented | line item quantity/unit; no pricing claim | known gap |
| legacy irregularities | line length, encoding fallback, blank ordinals | recoverable findings | adapter loss report warnings | required |

## Reference-only commercial source

`mwm_rialto_gaeb90_demo` is cataloged as `reference_only`. It may inform future interoperability planning, but it must not be downloaded, executed, or treated as runtime support evidence in CI.
