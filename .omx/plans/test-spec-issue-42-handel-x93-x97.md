# Test Spec: Handel X93-X97 trade and commerce support planning

## Issue
- GitHub issue: #42

## Red/green order
1. Add fixture manifest/source-status tests for the per-source matrix.
2. Add detector/model/parser red tests named below.
3. Implement only the smallest behavior required to turn the tests green.
4. Run full protected-main gate stack.
5. Promote support_status only with green tests and review evidence.

## Concrete planned tests
- [ ] `test_handel_sources_are_cataloged_by_phase_x93_x94_x96_x97` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_handel_boundary_adr_exists_before_parser_modules` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_trade_document_is_not_classified_as_boq` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_x93_x94_phase_detector_red_tests` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_handel_interactive_schema_charts_are_reference_only` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.

## Per-source fixture/status checks
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_handel_pkg | official_gaeb | handel | 3.3 X93-X97 package | future_track | manifest download with checksum/license note | official schema/sample package | future_handel_33_cataloged |
| gaeb32_handel_pkg | official_gaeb | handel | 3.2 X93-X97 package | future_track | manifest download with checksum/license note | official schema/sample package | future_handel_32_cataloged |
| schema_x93_33_chart | interactive_schema | handel | 3.3 X93 | reference_only | no CI dependency on external HTML | schema chart only | reference_x93_33_schema_chart |
| schema_x94_33_chart | interactive_schema | handel | 3.3 X94 | reference_only | no CI dependency on external HTML | schema chart only | reference_x94_33_schema_chart |
| schema_x93_32_chart | interactive_schema | handel | 3.2 X93 | reference_only | no CI dependency on external HTML | schema chart only | reference_x93_32_schema_chart |

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

This section records how issue #42 may use the linked source rows as local fixtures, planned fixture gates, or reference-only evidence.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R8-01 | #42 Handel X93-X97 | manifested | official_gaeb_xml33_handel | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R8-02 | #42 Handel X93-X97 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-03 | #42 Handel X93-X97 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-04 | #42 Handel X93-X97 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-05 | #42 Handel X93-X97 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
