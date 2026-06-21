# Test Spec: X31 parser MVP

## Issue
- GitHub issue: #29
- Spec: `.omx/specs/issue-29-x31-parser-mvp.md`

## Required automated tests
- `test_bvbs_x31_parses_measurement_groups`
  - Verifies X31 version, row id, ordinal, and measurement group metadata.
- `test_bvbs_x31_formula_records_preserve_source`
  - Verifies REB-VB formula source preservation, result quantity, and unit extraction.
- `test_bvbs_x31_attachments_are_detected`
  - Verifies attachment ids, kinds, source URI, and row links.
- `test_x31_parser_reports_unsupported_features`
  - Verifies unsupported X31 constructs emit recoverable findings.
- `test_bvbs_x31_support_promotion_requires_parser_evidence`
  - Verifies manifest status, readiness language, and test mappings for X31 parser support.

## Local gate sequence
1. `cargo fmt --check`
2. `cargo test --test x31_domain`
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
- GitHub PR must include `Closes #29`.
- All actionable review comments must be resolved.
- GH `Rust quality gates` must be successful and merge state must be clean.
- Self-approval is attempted; GitHub may reject own-PR approval even for admin users, in which case the rejection is recorded as evidence.
