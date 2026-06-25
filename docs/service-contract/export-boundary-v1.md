# Export and roundtrip boundary contract v1

Schema version: `boq-core.export-boundary.v1`

The export boundary report tells services and Obra which exchange capabilities
exist today and which lanes remain blocked. The current implemented evidence is
dependency-free neutral CSV only.

## Command

```sh
boq-core-service export-boundaries
```

## Neutral CSV boundary

Neutral CSV uses the stable `oz_matched_csv_neutral` contract:

- rows are matched by `oz` / item ordinal;
- helper columns may be ignored with explicit findings;
- missing, duplicated, or empty `oz` keys fail closed;
- malformed CSV fails closed;
- `support_status` and provenance are carried as evidence only.

Stable fail-closed codes:

- `spreadsheet_neutral_missing_oz_column`
- `spreadsheet_neutral_duplicate_header`
- `spreadsheet_neutral_missing_oz_value`
- `spreadsheet_neutral_invalid_csv`

## Blocked lanes

- `xlsx`: not implemented; reference-only spreadsheet sources are not runtime
  support evidence.
- `ods`: not implemented; no parser/writer, fixtures, or product workflow.
- `gaeb-export-roundtrip`: not implemented here; GAEB export and roundtrip
  remain separate capability-gated work.

## Support honesty

- `neutral_csv_export`: `true`
- `neutral_csv_update`: `true`
- `xlsx_export`: always `false`
- `ods_export`: always `false`
- `gaeb_export_roundtrip`: always `false`
- `production_spreadsheet_roundtrip`: always `false`
- `certification`: always `false`
- `production_ready`: always `false`
- `certification_claims`: always empty
- `external_spreadsheet_dependency`: always `false`

Services must treat export and roundtrip as bounded capability flags, not
blanket support. This contract does not claim production spreadsheet support,
XLSX/ODS support, GAEB roundtrip support, certification, or full export support.
