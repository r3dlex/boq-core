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
- [ ] Decide pre-1.0 compatibility promise and release channels.
- [ ] Add dry-run release checks and changelog expectations.
- [ ] Require manual authorization for actual publish.

## Planned tests/checks
- [ ] `test_cargo_metadata_contains_release_fields`
- [ ] `test_release_workflow_is_dry_run_safe`
- [ ] `test_publish_requires_manual_authorization`
- [ ] `test_changelog_mentions_support_status_changes`
