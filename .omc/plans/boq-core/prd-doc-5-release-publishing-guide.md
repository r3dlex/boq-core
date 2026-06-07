# PRD: Release and publishing guide

## Planned issue
- Planned issue key: DOC-5
- Planned milestone: boq-core documentation MVP

## Product outcome
The release guide documents semver, crates.io readiness, docs publishing checks, and protected-main/full-green PR workflow.

## Requirements
- [ ] Publishing is dry-run/planned unless explicitly authorized.
- [ ] Branch protection and full-green PR policy are documented.
- [ ] Docs build checks are included in release readiness.

## Planned checks
- [ ] `cargo doc --all-features --no-deps`
- [ ] `mdbook build docs/book`
- [ ] `test_release_guide_mentions_manual_publish_gate`
