# PRD: Release and publishing guide

## Planned issue
- GitHub issue: #54
- Planned issue key: DOC-5
- Planned milestone: boq-core documentation MVP

## Product outcome
The release guide documents semver, crates.io readiness, docs publishing checks, protected-main/full-green PR workflow, and the manual publish gate.

## Requirements
- [x] Publishing is dry-run/planned unless explicitly authorized.
- [x] Branch protection and full-green PR policy are documented.
- [x] Docs build checks are included in release readiness.

## Implemented checks
- [x] `cargo doc --all-features --no-deps`
- [x] `mdbook build docs/book`
- [x] `test_release_guide_mentions_manual_publish_gate`

## Verification
- [x] `cargo test --test release_automation`
- [x] `cargo doc --all-features --no-deps`
- [x] `mdbook build`
