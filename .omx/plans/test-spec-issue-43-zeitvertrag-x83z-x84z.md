# Test Spec: Zeitvertrag X83Z/X84Z framework-contract support planning

## Issue
- GitHub issue: #43

## Red/green order
1. Add fixture manifest/source-status tests for the per-source matrix.
2. Add detector/model/parser red tests named below.
3. Implement only the smallest behavior required to turn the tests green.
4. Run full protected-main gate stack.
5. Promote support_status only with green tests and review evidence.

## Concrete planned tests
- [ ] `test_zeitvertrag_sources_are_cataloged_by_z_phase` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_z_phase_boundary_adr_exists_before_parser_changes` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_x83z_x84z_are_not_misclassified_as_standard_x83_x84` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_framework_discount_premium_red_tests` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_zeitvertrag_interactive_schema_charts_are_reference_only` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.

## Per-source fixture/status checks
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_zeitvertrag_pkg | official_gaeb | zeitvertrag | 3.3 X83Z/X84Z package | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | reference schema/sample package; not executable parser fixture | reference_zeitvertrag_33_package_cataloged |
| gaeb32_zeitvertrag_pkg | official_gaeb | zeitvertrag | 3.2 package | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | reference schema/sample package; not executable parser fixture | reference_zeitvertrag_32_package_cataloged |
| gaeb32_zeitvertrag_examples | official_gaeb | zeitvertrag | 3.2 examples | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | reference examples; not executable parser fixture | reference_zeitvertrag_32_examples_cataloged |
| schema_x83z_33_chart | interactive_schema | zeitvertrag | 3.3 X83Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x83z_33_schema_chart |
| schema_x84z_33_chart | interactive_schema | zeitvertrag | 3.3 X84Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x84z_33_schema_chart |
| schema_x83z_32_chart | interactive_schema | zeitvertrag | 3.2 X83Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x83z_32_schema_chart |

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

This section records how issue #43 may use the linked source rows as local fixtures, planned fixture gates, or reference-only evidence.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R7-01 | #43 Zeitvertrag X83Z/X84Z | manifested | official_gaeb_xml33_zeitvertrag | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-02 | #43 Zeitvertrag X83Z/X84Z | manifested | official_gaeb_xml32_zeitvertrag | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-03 | #43 Zeitvertrag X83Z/X84Z | manifested | official_gaeb_xml32_zeitvertrag_examples | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-04 | #43 Zeitvertrag X83Z/X84Z | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R7-05 | #43 Zeitvertrag X83Z/X84Z | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R7-06 | #43 Zeitvertrag X83Z/X84Z | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
