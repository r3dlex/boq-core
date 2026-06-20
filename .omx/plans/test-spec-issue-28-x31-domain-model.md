# Test Spec: X31 quantity takeoff domain model

## Issue
- GitHub issue: #28
- Spec: `.omx/specs/issue-28-x31-domain-model.md`

## Required automated tests
- `test_x31_domain_represents_formula_rows`
  - Verifies REB-VB 23.003 formula marker, expression preservation, result quantity, and physical progress fields.
- `test_x31_domain_links_measurements_to_ordinal`
  - Verifies BoQ ordinal links and X86 baseline link representation without touching BoQ parser models.
- `test_x31_domain_represents_attachments_as_findings_or_assets`
  - Verifies local attachment assets and reference-only attachment findings.
- `test_x31_domain_is_serializable`
  - Verifies deterministic serde JSON serialization and deserialization.

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
- GitHub PR must include `Closes #28`.
- All actionable review comments must be resolved.
- GH `Rust quality gates` must be successful and merge state must be clean.
- Self-approval is attempted; GitHub may reject own-PR approval even for admin users, in which case the rejection is recorded as evidence.
