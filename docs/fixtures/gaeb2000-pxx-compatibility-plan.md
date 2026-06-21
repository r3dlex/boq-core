# GAEB 2000 / Pxx compatibility plan

Issue: #39
ADR: ARCH-008
Status: future_track / reference_only planning; not production parser support.

## Syntax boundary

GAEB 2000/Pxx sources are bracketed keyword/tag files. They are distinct from:

- GAEB 90 fixed-width record lines (`Dxx` legacy fixed-width parser), and
- GAEB DA XML namespace/tree parsing (`Xxx` XML parser).

The `gaeb2000` module may tokenize `#begin[...]`, `#end[...]`, and scalar keyword rows as future parser preparation, but it must not claim BoQ import support until fixture-backed tests promote the track.

## Phase mapping

| Extension family | Example | Format | Phase |
| --- | --- | --- | --- |
| `P81`-`P86` | `sample.P86` | `Gaeb2000` | `81`-`86` |
| `D81`-`D86` as GAEB 2000 examples | Sportheim gist references may include D86/P86 naming | Future GAEB 2000 review required | `86` |
| XML mapping chart | web reference | reference_only | none |

## Source status

| Manifest id | Status | Boundary |
| --- | --- | --- |
| `gaeb2000_priced_gist` | `future_track` | priced D86/P86 sample; license/checksum review before vendoring |
| `dangl_ava_gaeb2000_examples` | `future_track` | select fixtures only after repository license/checksum review |
| `dangl_gaeb2000_sportheim_gist` | `future_track` | existing Sportheim gist catalog row |
| `gaeb2000_xml_mapping_chart` | `reference_only` | mapping reference, not runtime support evidence |

## Follow-up implementation issue

A future implementation issue should add checked-in synthetic fixtures first, then safe downloaded/reference fixtures with checksums, then parser support. Support promotion requires green tests and review evidence in the same PR.
