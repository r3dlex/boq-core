# Kosten/Kalkulation X50-X52 boundary matrix

Issue: #41
Architecture decision: ARCH-010
Status: reference_only planning; not parser support.

## Source matrix

| Source | Manifest/artifact id | Kind | Version/phase | Status | Execution policy |
| --- | --- | --- | --- | --- | --- |
| Official GAEB XML 3.3 Kosten und Kalkulation package | `official_gaeb_xml33_kosten_und_kalkulation` | manifest row | 3.3 X50-X51-X52 package | `reference_only` | No CI download; future local vendoring/checksum/license gate required before promotion. |
| Official GAEB XML 3.2 Kalkulation package | `official_gaeb_xml32_kalkulation` | manifest row | 3.2 X50-X51-X52 package | `reference_only` | No CI download; future local vendoring/checksum/license gate required before promotion. |
| Interactive X50 Baukostenkatalog chart | `schema_x50_33_chart` | artifact-only/reference | 3.3 X50 | `reference_only` | No CI dependency on external HTML. |
| Interactive X52 Kalkulationsdaten chart | `schema_x52_33_chart` | artifact-only/reference | 3.3 X52 | `reference_only` | No CI dependency on external HTML. |
| Interactive X52 Kalkulationsdaten chart | `schema_x52_32_chart` | artifact-only/reference | 3.2 X52 | `reference_only` | No CI dependency on external HTML. |

## Future model obligations

A future promotion PR must introduce red/green tests for these boundaries before
claiming parser or adapter support:

- `test_cost_component_model_red_tests`: cost-component identity, amount basis,
  surcharge/discount semantics, and unsupported-field findings.
- `test_x52_item_reference_mapping_red_tests`: X52 calculation records must map
to BOQ item references without silently creating or mutating item text/quantity support.
- `test_costing_sources_are_cataloged_by_phase_x50_x51_x52`: source rows and
  artifact-only references must stay explicit by version and phase family.

## Current negative contract

- Reference-only sources cannot be used as runtime support evidence.
- Interactive schema charts are documentation references only.
- No Obra backend changes are part of #41.
- No parser module is added for Kosten/Kalkulation in this issue.
- Support status promotion requires a later PR with fixture verification,
  implementation, local gates, GitHub CI, and resolved review feedback.
