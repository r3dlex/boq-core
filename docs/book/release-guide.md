# Release Guide

This guide defines release-readiness expectations for `boq-core`.

## Versioning

Use semver for public API and support-status changes:

- Patch releases fix bugs without changing public contracts or support claims.
- Minor releases may add parser support, adapter capabilities, or new documented extension points.
- Major releases may change public DTOs, support semantics, or Obra adapter contracts.

## Protected main and PR policy

All release-bound changes must go through protected main and a full-green PR. A full-green PR includes:

- formatting and lint checks;
- targeted unit/integration tests;
- docs checks when docs changed;
- architecture/reviewer/executor agreement for non-trivial changes;
- no paid certification or publishing side effects unless explicitly authorized.

## Documentation checks

Before a release candidate:

```bash
cargo doc --all-features --no-deps
mdbook build
```

The generated rustdoc API reference and mdBook manuals must agree on current support boundaries.

## crates.io readiness

Before publishing to crates.io:

1. Confirm `Cargo.toml` metadata, license, repository, and README.
2. Confirm semver version and changelog/release notes.
3. Run the full local quality gate.
4. Confirm docs do not overclaim certification or future-track support.
5. Confirm branch protection and CI are green on the release PR.

Publishing remains gated until explicitly authorized.
