# PRD: Plan and implement Bauausführung X84 bid submission support

## Issue
- GitHub issue: #27
- Milestone: v0.4 BVBS Bauausführung support

## Product outcome
Bauausführung X84 bid submissions map prices and bidder remarks by ordinal against a known X83 baseline.

## Source/status anchors
- BVBS Bauausführung X84: `supported_parse_only` after this issue; adapter/export/roundtrip/schema validation remain disabled.
- X83 baseline: #26 parser baseline required for authoritative tender descriptions.

## Blocking dependencies
- #26 X83 parser baseline is complete and is the authoritative tender-description source for X84 merging.

## Requirements
- [x] Define X84 sparse-description/pricing contract.
- [x] Resolve prices by ordinal against X83 baseline.
- [x] Preserve bidder remarks and unmatched-ordinal findings.

## Specification artifacts
- Spec: `.omx/specs/issue-27-bauausfuehrung-x84-bid.md`
- Test spec: `.omx/plans/test-spec-issue-27-bauausfuehrung-x84-bid.md`

## Planned tests
- [x] `test_bau_x84_prices_map_by_ordinal`
- [x] `test_bau_x84_missing_descriptions_resolve_against_x83_baseline`
- [x] `test_bau_x84_bidder_remarks_preserved`
- [x] `test_bau_x84_unmatched_ordinal_emits_finding`
- [x] `test_bau_x84_support_promotion_requires_bid_evidence`
