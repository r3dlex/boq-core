# Test Spec: GAEB 2000 / Pxx parser compatibility planning

## Issue
- GitHub issue: #39

## Red/green order
1. Add fixture manifest/source-status tests for the per-source matrix.
2. Add detector/model/parser red tests named below.
3. Implement only the smallest behavior required to turn the tests green.
4. Run full protected-main gate stack.
5. Promote support_status only with green tests and review evidence.

## Concrete planned tests
- [ ] `test_gaeb2000_manifest_sources_are_future_or_reference_only` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_gaeb2000_tokenizer_handles_begin_end_nesting` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_gaeb2000_tokenizer_reports_unclosed_begin_blocks` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_gaeb2000_phase_detector_maps_p81_to_p86` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_gaeb2000_mapping_chart_is_not_used_as_runtime_support_evidence` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.

## Per-source fixture/status checks
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb2000_priced_gist | developer_example | gaeb2000 | D86/P86 priced sample | future_track | download only as text fixture with checksum | developer gist; license note required | future_gaeb2000_priced_gist_cataloged |
| dangl_ava_gaeb2000_examples | developer_repo | gaeb2000 | GaebFiles Pxx/Dxx examples | future_track | clone/download gated; select fixtures only | developer-maintained examples; license note required | future_dangl_ava_gaeb2000_cataloged |
| gaeb2000_xml_mapping_chart | interactive_schema | mapping_reference | GAEB 2.1 to XML mapping | reference_only | no CI dependency on external HTML | mapping reference only; not GAEB 2000 support evidence | reference_gaeb2000_mapping_chart |

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

This section records how issue #39 may use the linked source rows as local fixtures, planned fixture gates, or reference-only evidence.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R4-01 | #39 GAEB 2000/Pxx compatibility | manifested | dangl_ava_examples | future_track | ['future_dangl_ava_examples_cataloged'] |
| R4-02 | #39 GAEB 2000/Pxx compatibility | manifested | dangl_ava_examples_cpp | future_track | ['future_dangl_cpp_examples_cataloged'] |
| R4-03 | #39 GAEB 2000/Pxx compatibility | manifested | dangl_gaeb2000_sportheim_gist | future_track | ['future_gaeb2000_sportheim_cataloged'] |


## Delivery notes
- ARCH-008 records the separate GAEB 2000/Pxx tokenizer/parser boundary.
- `docs/fixtures/gaeb2000-pxx-compatibility-plan.md` documents GAEB 2000 syntax, phase mapping, source status, and follow-up implementation policy.
- `tests/gaeb2000_compatibility.rs` covers catalog status, begin/end nesting diagnostics, P81-P86 phase detection, and mapping-chart reference-only boundaries.
