# ARCH-010: Kosten/Kalkulation X50-X52 boundary

## Status
Accepted

## Context
Issue #41 covers GAEB XML Kosten und Kalkulation tracks X50-X52. The available
inputs are official schema/sample packages and interactive schema charts, not
checked parser fixtures with redistribution-safe payloads and expected semantic
outputs. Costing data has a different product shape from BOQ import: cost
components, calculation bases, supplements, and item references can inform an
estimate, but they are not themselves LV item text/quantity import support.

## Decision
Keep X50-X52 in boq-core as a **reference-only planning boundary** until a later
PR introduces local, checksum/license-verified fixtures and red/green semantic
model tests. Do not add parser modules, adapter DTO fields, Obra backend calls,
or support-status promotion in this issue.

A future implementation may add a dedicated `kosten_kalkulation` module in
boq-core only after it proves the minimal source-backed domain model. If costing
logic grows beyond GAEB exchange parsing into estimating workflows, pricing
rules, or Obra-specific cost-account behavior, that behavior belongs in a
companion crate or the Obra application layer, not in boq-core parser core.

## Scope locked by this ADR
- Catalog official GAEB XML 3.3 Kosten/Kalkulation and GAEB XML 3.2 Kalkulation
  rows as reference-only evidence.
- Treat interactive X50/X52 charts as documentation references, not CI fixtures.
- Record planned model obligations for cost components and X52 item references.
- Require future promotion PRs to include failing tests first, implementation,
  fixture verification, and review evidence before changing support status.

## Non-goals
- No paid actions, certification actions, browser automation, or external
  downloads in CI for this track.
- No parser-support claim for X50/X51/X52.
- No Obra backend schema or API change.
- No duplicate issue creation for the same Kosten/Kalkulation source family.

## Consequences
Future executors can start from a stable boundary: X50-X52 support requires a
source-backed costing model, fixture-safe inputs, and explicit support-policy
promotion. Until that happens, manifest and support-policy responses must remain
`reference_only` with `reference_only` capabilities.
