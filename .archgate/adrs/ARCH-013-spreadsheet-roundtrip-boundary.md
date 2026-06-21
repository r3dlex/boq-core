# ARCH-013: Spreadsheet roundtrip helper boundary

## Status
Accepted

## Context
Issue #44 covers spreadsheet import/export helper planning for contractor
productivity workflows. Spreadsheet templates, generator executables, commercial
demos, and browser utilities can inform requirements, but they are not GAEB
parser fixtures. Adding spreadsheet readers/writers or executing helper binaries
would expand boq-core beyond parser-core boundaries and introduce licensing,
security, and binary dependency risk.

## Decision
Keep spreadsheet roundtrip helpers as a **reference-only examples/companion-crate
boundary** in this issue. Do not add spreadsheet dependencies, binary fixtures,
executable actions, browser scraping, parser support claims, or Obra backend
changes.

A future implementation may add CSV/spreadsheet helper examples if they remain
pure, dependency-light, and explicitly capability-gated. XLSX/ODS parsing or
writer support should live in a companion crate unless a later ADR proves it is
small, safe, and parser-core relevant. Every future roundtrip helper must match
rows by OZ/item ordinal, not by spreadsheet row order.

## Scope locked by this ADR
- `gaeb_online_import_template` and `gaeb_online_generator_exe` remain
  `reference_only` manifest rows.
- `mwm_rialto_gaeb90_demo`, EasyGAEB/browser utilities, and other commercial or
  browser references remain artifact-only/reference evidence unless separately
  verified and approved.
- OZ matching must survive reordered columns and inserted non-GAEB columns.
- Missing OZ/item ordinal must reject roundtrip updates instead of guessing.

## Non-goals
- No paid actions, certification actions, browser automation, external downloads,
  or executable runs in CI.
- No new spreadsheet/binary dependency in this issue.
- No parser-support, export, or roundtrip capability promotion.
- No Obra backend spreadsheet workflow change.
- No duplicate issue creation for the same spreadsheet helper source family.

## Consequences
Future spreadsheet work starts from a safe boundary: use reference-only sources
for planning, require explicit approval before dependencies, and prove OZ-based
matching with red/green tests before any roundtrip helper can be promoted.
