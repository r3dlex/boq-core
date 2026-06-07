# boq-core

`boq-core` is the planned Rust parser core for GAEB Bill of Quantities data used by Obra.

## Scope

- Loss-aware GAEB domain model first.
- Obra adapter DTO second; no Obra backend changes in MVP.
- GAEB DA XML 3.3 AVA certification-path readiness first.
- Bauausführung and Texterstellung follow after AVA.
- GAEB XML 3.4 beta is reference-only until explicitly promoted.

## Quality gates

Local/CI checks are expected to run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
archgate check --ci
prek run --all-files
```

Coverage target: 95% across the selected Rust coverage measurements once implementation code exists.

## Planning source of truth

See the workspace planning artifacts:

- `../.omx/plans/prd-boq-core-gaeb-parser-20260606.md`
- `../.omx/plans/test-spec-boq-core-gaeb-parser-20260606.md`
- `../.omx/plans/ralplan-handoff-boq-core-gaeb-parser-20260606.json`
