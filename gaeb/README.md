# GAEB Fixture Catalog

This directory is managed by `cargo run --bin xtask -- fixtures ...`.

The default CI path is offline: fixture archives are cataloged in `manifest.toml` and resolved artifacts are locked in `fixtures.lock` when downloaded. Large or license-uncertain files should be restored from cache or downloaded explicitly, not silently fetched during normal tests.

Support statuses:

- `supported`: parse + validation/snapshot coverage required.
- `supported_parse_only`: parser coverage required, no adapter/export claim.
- `future_track`: cataloged for sequenced follow-on work.
- `reference_only`: documented only; must not be executed or parsed as supported.

Windows executables and other runnable artifacts are reference-only and quarantined.
