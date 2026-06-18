# Test Spec: Spreadsheet roundtrip helper planning

## Issue
- GitHub issue: #44

## Red/green order
1. Add fixture manifest/source-status tests for the per-source matrix.
2. Add detector/model/parser red tests named below.
3. Implement only the smallest behavior required to turn the tests green.
4. Run full protected-main gate stack.
5. Promote support_status only with green tests and review evidence.

## Concrete planned tests
- [ ] `test_spreadsheet_sources_are_reference_only_non_executed` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_roundtrip_boundary_adr_exists_before_dependencies` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_oz_matching_reordered_columns_red_tests` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_inserted_columns_do_not_break_oz_matching_red_tests` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_missing_oz_rejects_roundtrip_red_tests` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.

## Per-source fixture/status checks
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb_online_import_template | spreadsheet_template | spreadsheet_roundtrip | Excel import template | reference_only | download manually only; checksum/license note; no parser support claim | spreadsheet template reference | reference_gaeb_online_import_template |
| gaeb_online_excel_generator | executable_tool | spreadsheet_roundtrip | Excel generator .exe | reference_only | do not download/execute in CI | executable; reference only | reference_gaeb_online_excel_generator |
| mwm_rialto_demo | commercial_demo | spreadsheet_roundtrip | Excel conversion demo | reference_only | do not download/execute in CI | commercial/demo utility; reference only | reference_mwm_rialto_demo |
| easy_gaeb_browser | browser_utility | spreadsheet_roundtrip | Browser utility | reference_only | no CI dependency; no scraping/execution | external web utility; reference only | reference_easy_gaeb_browser |

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

This section records how issue #44 may use the linked source rows as local fixtures, planned fixture gates, or reference-only evidence.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| A2-01 | #44 Spreadsheet roundtrip | manifested | gaeb_online_import_template | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-02 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| A2-03 | #44 Spreadsheet roundtrip | manifested | gaeb_online_generator_exe | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-04 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| A2-05 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
