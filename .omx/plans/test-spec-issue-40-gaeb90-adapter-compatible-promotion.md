# Test Spec: GAEB 90 adapter-compatible promotion planning

## Issue
- GitHub issue: #40

## Red/green order
1. Add fixture manifest/source-status tests for the per-source matrix.
2. Add detector/model/parser red tests named below.
3. Implement only the smallest behavior required to turn the tests green.
4. Run full protected-main gate stack.
5. Promote support_status only with green tests and review evidence.

## Concrete planned tests
- [ ] `test_gaeb90_adapter_gap_matrix_lists_required_fields` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_gaeb90_d81_d83_hierarchy_extraction_red_tests` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_gaeb90_windows1252_umlaut_decode_cases` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_gaeb90_malformed_fixed_width_recovery_findings` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_mwm_rialto_is_reference_only_non_executed` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.

## Per-source fixture/status checks
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| dangl_ava_gaeb90_examples | developer_repo | gaeb90 | D81/D83 examples | future_track | select fixture download gated by checksum | developer examples; license note required | future_dangl_ava_gaeb90_cataloged |
| mwm_rialto_gaeb90_demo | commercial_demo | spreadsheet_middleware | GAEB 90/2000/XML conversion demo | reference_only | do not execute/download in CI | commercial demo; compatibility reference only | reference_mwm_rialto_gaeb90_roundtrip |

## Boundary and negative tests
- [ ] Reference-only sources cannot be used as support evidence.
- [ ] Future-track sources cannot be parsed as supported until promotion tests pass.
- [ ] Paid, executable, commercial, browser, or interactive-only sources are never executed in CI.
- [ ] Unsupported fields produce structured findings rather than silent data loss.
- [ ] No duplicate issue is created for this source family unless a new independent track is discovered.

## Required verification commands for implementation PRs
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- `cargo llvm-cov --all-features --summary-only --ignore-filename-regex 'src/bin/xtask.rs' --fail-under-lines 95 --fail-under-functions 95 --fail-under-regions 95`
- `archgate check --ci`
- `cargo run --bin xtask -- fixtures verify`
- `uvx prek run --all-files`

## Ranked roadmap fixture/test mapping

This section records how issue #40 may use the linked source rows as local fixtures, planned fixture gates, or reference-only evidence.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R4-04 | #40 GAEB90 adapter-compatible promotion | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| R4-05 | #40 GAEB90 adapter-compatible promotion | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
