# Release Guide

This guide defines release-readiness expectations for `boq-core`.

## Versioning

Use semver for public API and support-status changes:

- Patch releases fix bugs without changing public contracts or support claims.
- Minor releases may add parser support, adapter capabilities, or new documented extension points.
- Major releases may change public DTOs, support semantics, or Obra adapter contracts.

## Pre-1.0 compatibility promise

While `boq-core` is below `1.0.0`, every release still follows an explicit compatibility promise:

- Patch releases are limited to bug fixes, documentation fixes, fixture corrections, and non-breaking release automation.
- Minor releases are required for breaking public API changes, supported-format promotions, support-status changes, new parser capabilities, or Obra adapter contract changes.
- Support-status changes must be called out in `CHANGELOG.md` under `Support status changes` and must reference the fixture evidence, tests or gates, and certification limitations that justify the claim.
- No release may imply paid BVBS certification, future-track compatibility, or production support unless that evidence is already present in the repository and explicitly documented.

## Release channels

Release candidates move through protected `main` via a full-green PR. Crates.io is the only public package channel currently documented for Rust package publishing.

Automation may verify release readiness, but it must not publish by default. Actual crates.io publishing requires explicit manual maintainer authorization after local checks and GitHub CI are green. This manual publish gate is mandatory: no agent, workflow, or script may remove `--dry-run`, upload to crates.io, create a public release, or submit paid certification evidence unless a maintainer explicitly authorizes that external side effect for the specific release.

## Protected main and PR policy

All release-bound changes must go through protected main and a full-green PR. Release work is never pushed directly to `main`; it starts on a branch, opens a pull request, resolves review comments, waits for required GitHub CI checks, and merges only after the branch is clean. A full-green PR includes:

- formatting and lint checks;
- targeted unit/integration tests;
- docs checks when docs changed;
- architecture/reviewer/executor agreement for non-trivial changes;
- no unresolved review threads or requested changes;
- no paid certification or publishing side effects unless explicitly authorized.

## Dry-run release checks

Release PRs that touch package metadata, changelog content, release documentation, or release workflow files run dry-run release checks:

- `cargo package --locked` verifies the package archive can be built from the locked dependency graph.
- `cargo publish --dry-run --locked` verifies crates.io publish readiness without uploading a release.

These checks are safe verification gates only. Removing `--dry-run` from automation is a release-blocking policy violation.

## Changelog expectations

Every release-bound PR must update `CHANGELOG.md` when it changes release behavior, public API, parser support, adapter contracts, support status, fixture evidence, or documented certification limitations.

Support-status changes must include the affected exchange format or track, the fixture evidence used, the tests or gates that prove the claim, and any certification limitation.

## Documentation checks

Docs publishing readiness is part of every release gate. Run rustdoc and mdBook locally before opening or updating a release PR, and rely on GitHub CI to repeat the protected-main checks before merge:

```bash
cargo doc --all-features --no-deps
mdbook build
```

Release notes and documentation must use only support status vocabulary backed by `gaeb/manifest.toml`: `supported`, `supported_parse_only`, `future_track`, and `reference_only`. A release note that invents another status word or implies support beyond manifest evidence is not release-ready.

## crates.io readiness

Before publishing to crates.io, confirm the manual publish gate has been authorized for this specific release. Then verify:

1. Confirm `Cargo.toml` metadata, license, repository, and README.
2. Confirm semver version and changelog/release notes.
3. Run the full local quality gate.
4. Confirm docs do not overclaim certification or future-track support.
5. Confirm branch protection and CI are green on the release PR.
6. Confirm a maintainer has explicitly authorized the manual publish action.
7. Keep `cargo publish --dry-run --locked` as the final automated publishing check; the real publish command is a manual maintainer action only.

Publishing remains gated until explicitly authorized; without that authorization the outcome is dry-run readiness only.
