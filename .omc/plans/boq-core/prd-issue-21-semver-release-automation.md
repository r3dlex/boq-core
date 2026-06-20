# PRD: Add semantic versioning and release automation

## Issue
- GitHub issue: #21
- Milestone: v0.3 Public API and parser robustness

## Product outcome
Semver and release automation are ready for crates.io dry runs, but publishing remains gated by explicit authorization.

## Source/status anchors
- Cargo metadata: release reference.
- GitHub Actions: dry-run automation.
- crates.io publishing: gated manual action.

## Requirements
- [x] Decide pre-1.0 compatibility promise and release channels.
  - Evidence: `docs/book/release-guide.md` defines pre-1.0 patch/minor semantics, release channels, and the manual crates.io gate.
- [x] Add dry-run release checks and changelog expectations.
  - Evidence: `.github/workflows/release-dry-run.yml` runs package/publish dry-run checks; `CHANGELOG.md` records release automation and support-status expectations.
- [x] Require manual authorization for actual publish.
  - Evidence: release workflow keeps `--dry-run`; the release guide documents manual maintainer authorization; regression tests block non-dry-run publish automation.

## Planned tests/checks
- [x] `cargo_metadata_contains_release_fields`
- [x] `release_workflow_is_dry_run_safe`
- [x] `publish_requires_manual_authorization`
- [x] `changelog_mentions_support_status_changes`

## Verification
- `cargo fmt --check`
- `cargo test --test release_automation`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- `cargo run --bin xtask -- fixtures verify`
- `cargo doc --all-features --no-deps`
- `mdbook build`
- `archgate check --ci`
- `uvx prek run --all-files`

## Known local validation notes
- `mdbook build` was rerun from the repository root and passed after the earlier wrong-path invocation (`mdbook build docs/book`) was corrected.
- Coverage threshold check was not run because `cargo-llvm-cov` is unavailable locally; all required local non-coverage gates passed.
