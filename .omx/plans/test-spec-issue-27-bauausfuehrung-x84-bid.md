# Test Spec: Bauausführung X84 bid submission support

## Issue
- GitHub issue: #27
- Spec: `.omx/specs/issue-27-bauausfuehrung-x84-bid.md`

## Required automated tests
- `test_bau_x84_prices_map_by_ordinal`
  - Verifies X84 unit and total prices overlay onto the matching X83 baseline item.
- `test_bau_x84_missing_descriptions_resolve_against_x83_baseline`
  - Verifies sparse or changed X84 descriptions do not replace the X83 tender description and mutable tender text is flagged.
- `test_bau_x84_bidder_remarks_preserved`
  - Verifies bidder remarks are parsed and preserved through the X83/X84 merge.
- `test_bau_x84_unmatched_ordinal_emits_finding`
  - Verifies missing and extra ordinals are non-fatal deterministic findings.
- `test_bau_x84_support_promotion_requires_bid_evidence`
  - Verifies manifest support status, test mappings, and readiness language for the X84 fixture.

## Local gate sequence
1. `cargo fmt --check`
2. `cargo test --test bau_roundtrip`
3. `cargo test --test testing_strategy`
4. `cargo clippy --all-targets --all-features -- -D warnings`
5. `cargo test --all-features`
6. `cargo run --bin xtask -- fixtures verify`
7. `cargo doc --all-features --no-deps`
8. `mdbook build`
9. `archgate check --ci`
10. `uvx prek run --all-files`
11. `cargo llvm-cov --all-features --summary-only --ignore-filename-regex 'src/bin/xtask.rs' --fail-under-lines 95 --fail-under-functions 95 --fail-under-regions 95`

## Merge gate
- GitHub PR must include `Closes #27`.
- All actionable review comments must be resolved.
- GH `Rust quality gates` must be successful and merge state must be clean.
- Self-approval is attempted; GitHub may reject own-PR approval even for admin users, in which case the rejection is recorded as evidence.
