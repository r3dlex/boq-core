# ARCH-012: Zeitvertrag X83Z/X84Z boundary

## Status
Accepted

## Context
Issue #43 covers Zeitvertrag framework-contract packages and examples. The
Z-suffixed phases X83Z and X84Z overlap with ordinary construction-execution
X83/X84 data, but they represent framework/time-contract semantics such as
contract catalogs, framework quantities, discounts, premiums, and call-off
conditions. Treating them as ordinary X83/X84 would overclaim support and risk
silent data loss.

The manifest already catalogs official GAEB XML 3.3 Zeitvertrag, GAEB XML 3.2
Zeitvertrag, and GAEB XML 3.2 Zeitvertrag examples as reference-only rows. The
interactive X83Z/X84Z charts remain artifact-only reference evidence.

## Decision
Keep Zeitvertrag X83Z/X84Z as a **reference-only Z-phase framework-contract
boundary** until a future promotion PR introduces safe fixtures, a Z-phase domain
model, and red/green parser tests. Extension detection for `.X83Z` and `.X84Z`
must not degrade into ordinary `.X83`/`.X84` phase claims; until explicit
Z-phase detection exists, they should remain advisory GAEB XML inputs without a
standard phase classification.

Future implementation may add a dedicated Zeitvertrag/Z-phase module in boq-core
if it stays limited to GAEB exchange representation and structured findings.
Framework-contract business workflows, call-off processing, or Obra-specific
contract administration belong in a companion module/crate or Obra application
layer.

## Scope locked by this ADR
- `official_gaeb_xml33_zeitvertrag`, `official_gaeb_xml32_zeitvertrag`, and
  `official_gaeb_xml32_zeitvertrag_examples` remain `reference_only`.
- X83Z/X84Z must not be misclassified as standard X83/X84 parser support.
- Framework discount/premium/call-off semantics are future model obligations,
  not current BOQ import behavior.
- Interactive X83Z/X84Z schema charts remain documentation references only.

## Non-goals
- No paid actions, certification actions, browser automation, or external
  downloads in CI for this track.
- No parser-support claim for X83Z/X84Z.
- No change to ordinary X83/X84 construction-execution support.
- No Obra backend framework-contract workflow change.
- No duplicate issue creation for the same Zeitvertrag source family.

## Consequences
Future Zeitvertrag work starts from a clear Z-phase boundary: support promotion
requires fixture-safe inputs, a framework-contract model, structured findings for
unsupported fields, local protected-main gates, GitHub CI, and resolved review
feedback before any support status changes.
