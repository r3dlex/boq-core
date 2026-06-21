# Handel X93-X97 boundary matrix

Issue: #42
Architecture decision: ARCH-011
Status: reference_only planning; not parser support.

## Source matrix

| Source | Manifest/artifact id | Kind | Version/phase | Status | Execution policy |
| --- | --- | --- | --- | --- | --- |
| Official GAEB XML 3.3 Handel package | `official_gaeb_xml33_handel` | manifest row | 3.3 X93-X94-X96-X97 package | `reference_only` | No CI download; future local vendoring/checksum/license gate required before promotion. |
| Official GAEB XML 3.2 Handel package | `gaeb32_handel_pkg` | artifact-only/reference | 3.2 X93-X97 package | `reference_only` | No CI download and no invented manifest URL; add a verified source row before fixture promotion. |
| Interactive X93 Handel chart | `schema_x93_33_chart` | artifact-only/reference | 3.3 X93 | `reference_only` | No CI dependency on external HTML. |
| Interactive X94 Handel chart | `schema_x94_33_chart` | artifact-only/reference | 3.3 X94 | `reference_only` | No CI dependency on external HTML. |
| Interactive X93 Handel chart | `schema_x93_32_chart` | artifact-only/reference | 3.2 X93 | `reference_only` | No CI dependency on external HTML. |

## Future model obligations

A future promotion PR must introduce red/green tests for these boundaries before
claiming parser or adapter support:

- `test_trade_document_is_not_classified_as_boq`: trade/procurement payloads
  must not be silently mapped into LV/BOQ item import semantics.
- `test_x93_x94_phase_detector_red_tests`: extension detection for X93/X94/X96/X97
  is advisory GAEB XML phase detection only, not support-status promotion.
- `test_handel_sources_are_cataloged_by_phase_x93_x94_x96_x97`: source rows and
  artifact-only references must remain explicit by version and phase family.

## Current negative contract

- Reference-only sources cannot be used as runtime support evidence.
- Interactive schema charts are documentation references only.
- No Obra backend procurement changes are part of #42.
- No parser module is added for Handel in this issue.
- Support status promotion requires a later PR with fixture verification,
  implementation, local gates, GitHub CI, and resolved review feedback.
