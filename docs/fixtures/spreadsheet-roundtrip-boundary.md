# Spreadsheet roundtrip helper boundary matrix

Issue: #44
Architecture decision: ARCH-013
Status: reference_only planning; not parser/export/roundtrip support.

## Source matrix

| Source | Manifest/artifact id | Kind | Version/phase | Status | Execution policy |
| --- | --- | --- | --- | --- | --- |
| GAEB-Online Excel import template | `gaeb_online_import_template` | manifest row | XLSX import template | `reference_only` | Manual reference only; no parser support claim. |
| GAEB-Online Excel generator executable | `gaeb_online_generator_exe` | manifest row | Windows `.exe` | `reference_only` | Never downloaded or executed in CI. |
| MWM/Rialto spreadsheet conversion demo | `mwm_rialto_gaeb90_demo` / `mwm_rialto_demo` | manifest row/reference alias | commercial demo | `reference_only` | Never downloaded or executed in CI. |
| EasyGAEB browser utility | `easy_gaeb_browser` | artifact-only/reference | browser utility | `reference_only` | No CI dependency, scraping, or browser execution. |

## OZ matching requirements

A future promotion PR must introduce red/green tests for these spreadsheet
roundtrip rules before claiming support:

- `test_oz_matching_reordered_columns_red_tests`: locate OZ/item ordinal by
  header identity, so reordered columns do not change matching.
- `test_inserted_columns_do_not_break_oz_matching_red_tests`: ignore inserted non-GAEB helper columns while preserving known GAEB fields.
- `test_missing_oz_rejects_roundtrip_red_tests`: reject updates that lack an
  OZ/item ordinal column instead of guessing by row order.

## Current negative contract

- Reference-only sources cannot be used as runtime support evidence.
- Executables and browser utilities are documentation references only.
- No spreadsheet dependency is added in #44.
- No Obra backend spreadsheet workflow changes are part of #44.
- No parser module or roundtrip helper is promoted in this issue.
- Support status promotion requires a later PR with fixture verification,
  implementation, local gates, GitHub CI, and resolved review feedback.
