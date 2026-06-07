# PRD: Plan and implement Bauausführung X84 bid submission support

## Issue
- GitHub issue: #27
- Milestone: v0.4 BVBS Bauausführung support

## Product outcome
Bauausführung X84 bid submissions map prices and bidder remarks by ordinal against a known X83 baseline.

## Source/status anchors
- BVBS Bauausführung X84: `future_track` / `future_track` until green.
- X83 baseline: required matching evidence.

## Blocking dependencies
- Depends on #26 X83 parser baseline.

## Requirements
- [ ] Define X84 sparse-description/pricing contract.
- [ ] Resolve prices by ordinal against X83 baseline.
- [ ] Preserve bidder remarks and unmatched-ordinal findings.

## Planned tests
- [ ] `test_bau_x84_prices_map_by_ordinal`
- [ ] `test_bau_x84_missing_descriptions_resolve_against_x83_baseline`
- [ ] `test_bau_x84_bidder_remarks_preserved`
- [ ] `test_bau_x84_unmatched_ordinal_emits_finding`
