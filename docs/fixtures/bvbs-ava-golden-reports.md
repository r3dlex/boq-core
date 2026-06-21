# BVBS AVA golden report workflow

`gaeb/golden/bvbs_ava/*.json` contains deterministic readiness snapshots for
the supported BVBS AVA X81, X84, and X86 fixtures.

The reports are regression evidence only. They do not claim paid BVBS
certification and each report carries `certification_claim: false`.

## Schema

Each report records:

- `schema_version` — current report schema version (`1`).
- `fixture_id` — stable manifest fixture id.
- `source_path` and `source_checksum_sha256` — fixture provenance.
- `parser_version` — crate version that generated the report.
- `support_status` and `capabilities` — support-policy claim at parse time.
- `summary` — GAEB format/version/phase/title/project/currency metadata.
- `hierarchy` — root count, total node count, total item count, and root ordinals.
- `item_samples` — deterministic first item samples with quantities, units,
  prices, and long-text kind.
- `findings` — recoverable parser findings serialized as JSON.
- `certification_claim` — always `false`.

## Refresh command

Refresh is explicit and must be reviewed in a PR:

```bash
UPDATE_BVBS_AVA_GOLDEN=1 cargo test --test bvbs_ava_integration
cargo test --test bvbs_ava_integration
```

Do not set `UPDATE_BVBS_AVA_GOLDEN` in CI. CI must compare committed reports and
fail on unreviewed drift.

## Review checklist

- Confirm parser changes explain every golden diff.
- Confirm fixture checksums remain stable or are reviewed with manifest/lockfile
  evidence.
- Confirm no report uses certification language beyond `certification_claim:
  false`.
- Confirm `gaeb/criteria/bvbs_ava_matrix.toml` links the concrete report paths.
