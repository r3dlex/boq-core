# ARCH-009: GAEB 90 adapter-compatible promotion gap analysis

## Status
Accepted

## Context
Issue #40 evaluates whether selected GAEB 90 D81/D83 inputs can move from parse-only toward Obra-adapter-compatible support. The Obra adapter DTO requires a loss-aware BoQ title, deterministic source provenance, WBS/item ordinals, item descriptions, quantity/unit fields, and recoverable findings for malformed legacy records.

## Decision
Promote only the checked Dangl GAEB 90 D83 fixture path to adapter-compatible import capability. Unknown GAEB 90 inputs and malformed/synthetic examples remain parse-only unless a manifest row, fixture evidence, and tests explicitly promote them.

The required gap matrix for promotion is:

| Required field | GAEB 90 source evidence | Adapter usage | Gate |
| --- | --- | --- | --- |
| document title | record `02` payload | `ObraBoqDocument.title` | fixture test |
| phase/provenance | `.D81` / `.D83` extension and checksum | deterministic keys and import trace | parser test |
| item ordinal | record `21` payload positions 1-9 | WBS code/path/classification | malformed ordinal finding if blank |
| short text | record `25` payload | line item description | fixture test |
| long text | record `26` payload sequence | line item rich text | fixture test |
| quantity/unit | record `21` payload fields | line item quantity/unit | known gaps remain findings/metadata until exact GAEB 90 semantics are implemented |
| legacy irregularities | short lines, encoding fallback, bad ordinals | loss report warnings | recoverable finding tests |

## Boundaries
- No Obra backend changes; adapter-compatible means boq-core DTO compatibility only.
- No blanket GAEB 90 support promotion.
- No export/roundtrip claim.
- Commercial MWM/Rialto material is `reference_only` and not downloaded/executed in CI.
- Future D81/D83 promotion requires fixture rows, checksums/license notes, and tests in the same PR.

## Consequences
Support policy must consult manifest rows for GAEB 90 as well as GAEB XML, but only manifest-backed supported rows may expose `adapt_to_obra`. Tests must prove adapter conversion works for the promoted D83 fixture and remains rejected for malformed/unknown parse-only GAEB 90 inputs.
