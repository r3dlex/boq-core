# Zeitvertrag X83Z/X84Z boundary matrix

Issue: #43
Architecture decision: ARCH-012
Status: reference_only planning; not parser support.

## Source matrix

| Source | Manifest/artifact id | Kind | Version/phase | Status | Execution policy |
| --- | --- | --- | --- | --- | --- |
| Official GAEB XML 3.3 Zeitvertrag package | `official_gaeb_xml33_zeitvertrag` | manifest row | 3.3 X83Z-X84Z package | `reference_only` | No CI download; future local vendoring/checksum/license gate required before promotion. |
| Official GAEB XML 3.2 Zeitvertrag package | `official_gaeb_xml32_zeitvertrag` | manifest row | 3.2 X83Z-X84Z package | `reference_only` | No CI download; future local vendoring/checksum/license gate required before promotion. |
| Official GAEB XML 3.2 Zeitvertrag examples | `official_gaeb_xml32_zeitvertrag_examples` | manifest row | 3.2 examples | `reference_only` | No CI download; future local vendoring/checksum/license gate required before promotion. |
| Interactive X83Z Zeitvertrag chart | `schema_x83z_33_chart` | artifact-only/reference | 3.3 X83Z | `reference_only` | No CI dependency on external HTML. |
| Interactive X84Z Zeitvertrag chart | `schema_x84z_33_chart` | artifact-only/reference | 3.3 X84Z | `reference_only` | No CI dependency on external HTML. |
| Interactive X83Z Zeitvertrag chart | `schema_x83z_32_chart` | artifact-only/reference | 3.2 X83Z | `reference_only` | No CI dependency on external HTML. |

## Future model obligations

A future promotion PR must introduce red/green tests for these boundaries before
claiming parser or adapter support:

- `test_x83z_x84z_are_not_misclassified_as_standard_x83_x84`: Z-suffixed phases
  must not silently fall back to ordinary Bauausführung X83/X84 behavior.
- `test_framework_discount_premium_red_tests`: framework discounts, premiums,
  call-off conditions, and contract catalog metadata need explicit model fields
  or structured unsupported-field findings.
- `test_zeitvertrag_sources_are_cataloged_by_z_phase`: manifested and
  artifact-only sources must remain explicit by version and Z-phase family.

## Current negative contract

- Reference-only sources cannot be used as runtime support evidence.
- Interactive schema charts are documentation references only.
- No Obra backend framework-contract changes are part of #43.
- No parser module is added for Zeitvertrag in this issue.
- Ordinary X83/X84 behavior must not be changed by Z-phase planning.
- Support status promotion requires a later PR with fixture verification,
  implementation, local gates, GitHub CI, and resolved review feedback.
