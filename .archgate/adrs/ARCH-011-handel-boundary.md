# ARCH-011: Handel X93-X97 boundary

## Status
Accepted

## Context
Issue #42 covers GAEB XML Handel phases X93-X97 for material ordering,
procurement, product catalog, and logistics exchange. These documents are
adjacent to BOQ import but are not LV/BoQ documents: they can reference items,
products, suppliers, quantities, and ordering processes that should not be
silently folded into boq-core's BOQ tree or Obra import DTO.

Available evidence is a GAEB XML 3.3 official Handel schema package plus
reference-only 3.2 and interactive schema-chart entries in the ranked roadmap.
There is no local checksum/license-verified parser fixture with expected trade
semantics in this issue.

## Decision
Keep Handel X93-X97 as a **reference-only trade/procurement boundary** in
boq-core until a future promotion PR adds safe fixtures, a trade-domain model,
and red/green parser tests. The existing extension detector may identify X93,
X94, X96, and X97 as GAEB XML phases, but that detection is advisory only and
must not classify the payload as BOQ parser support.

A future implementation may introduce a dedicated `handel` module for exchange
parsing if it remains limited to GAEB source representation and structured
findings. Procurement workflows, product catalog normalization, supplier/order
state machines, or Obra-specific purchase processes belong in a companion trade crate or Obra application layer.

## Scope locked by this ADR
- `official_gaeb_xml33_handel` remains `reference_only`.
- GAEB XML 3.2 Handel and interactive X93/X94 charts remain artifact-only
  reference evidence until verified source rows or local fixtures are added.
- X93/X94/X96/X97 detection tests are advisory and do not imply parse support.
- Trade documents must not be treated as BOQ/LV adapter evidence.

## Non-goals
- No paid actions, certification actions, browser automation, or external
  downloads in CI for this track.
- No parser-support claim for X93/X94/X96/X97.
- No Obra backend purchase/procurement model change.
- No duplicate issue creation for the same Handel source family.

## Consequences
Future Handel work starts from a clear boundary: parser promotion requires
fixture-safe inputs, a separate trade-domain model decision, structured findings
for unsupported fields, local protected-main gates, GitHub CI, and resolved
review feedback before support status can change.
