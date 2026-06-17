# Test Spec: GAEB XML 3.1 and 3.2 compatibility track

## Issue
- GitHub issue: #37

## Red/green order
1. Add fixture manifest/source-status tests for the per-source matrix.
2. Add detector/model/parser red tests named below.
3. Implement only the smallest behavior required to turn the tests green.
4. Run full protected-main gate stack.
5. Promote support_status only with green tests and review evidence.

## Concrete planned tests
- [ ] `test_manifest_catalogs_gaeb_xml31_and_xml32_sources` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_xml_version_detector_distinguishes_31_32_33_namespaces` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_xml32_ava_fixtures_remain_future_track_until_parser_promotion` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_xml31_schema_sources_remain_reference_only` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_unsupported_legacy_xml_features_emit_structured_findings` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.

## Per-source fixture/status checks
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb_xml32_doc | official_gaeb | compatibility | 3.2 docs | reference_only | local/manual only | PDF docs; do not assert runtime support | docs_reference_gaeb_xml32 |
| gaeb_xml32_lv_schema | official_gaeb | leistungsverzeichnis | 3.2 X81-X87 schema package | reference_only | manifest download gated by checksum | official schema package; no payload without license check | schema_reference_gaeb_xml32_lv |
| gaeb_xml32_x83_interactive | interactive_schema | bauausfuehrung | 3.2 X83 | reference_only | no CI dependency on external HTML | public interactive chart; documentation only | schema_reference_gaeb_xml32_x83 |
| bvbs_xml32_ava_x81 | bvbs | ava | 3.2 X81 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x81_cataloged |
| bvbs_xml32_ava_x84 | bvbs | ava | 3.2 X84 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x84_cataloged |
| bvbs_xml32_ava_x86 | bvbs | ava | 3.2 X86 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x86_cataloged |
| bvbs_xml32_bau_x83 | bvbs | bauausfuehrung | 3.2 X83 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_bau_x83_cataloged |
| gaeb_xml31_doc | official_gaeb | compatibility | 3.1 docs | reference_only | local/manual only | PDF docs; no runtime support claim | docs_reference_gaeb_xml31 |
| gaeb_xml31_muster | official_gaeb | compatibility | 3.1 2009-12 Musterdateien | future_track | fixture download only with checksum/license note | official examples; no support until tests pass | future_xml31_musterdateien_cataloged |
| gaeb_xml31_x81_x87_schema | official_gaeb | compatibility | 3.1 X81-X83/X85-X87 schemas | reference_only | manifest download gated by checksum | schema package only | schema_reference_gaeb_xml31_x81_x87 |

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

This section records how issue #37 may use the linked source rows as local fixtures, planned fixture gates, or reference-only evidence.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R3-01 | #37 GAEB XML 3.1/3.2 compatibility | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R3-02 | #37 GAEB XML 3.1/3.2 compatibility | manifested | official_gaeb_xml32_leistungsverzeichnis | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R3-03 | #37 GAEB XML 3.1/3.2 compatibility | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R3-04 | #37 GAEB XML 3.1/3.2 compatibility | manifested | bvbs_xml32_ava_x81 | future_track | ['future_legacy_xml32_ava_x81_cataloged'] |
| R3-05 | #37 GAEB XML 3.1/3.2 compatibility | manifested | bvbs_xml32_ava_x84 | future_track | ['future_legacy_xml32_ava_x84_cataloged'] |
| R3-06 | #37 GAEB XML 3.1/3.2 compatibility | manifested | bvbs_xml32_ava_x86 | future_track | ['future_legacy_xml32_ava_x86_cataloged'] |
| R3-07 | #37 GAEB XML 3.1/3.2 compatibility | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only visual PDF; not executable as parser fixture. |
| R3-08 | #37 GAEB XML 3.1/3.2 compatibility | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only certification criteria PDF; not executable as parser fixture. |
| R3-09 | #37 GAEB XML 3.1/3.2 compatibility | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R3-10 | #37 GAEB XML 3.1/3.2 compatibility | manifested | official_gaeb_xml31_muster_2009_12 | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R3-11 | #37 GAEB XML 3.1/3.2 compatibility | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R3-12 | #37 GAEB XML 3.1/3.2 compatibility | manifested | bvbs_xml31_bau_x83 | future_track | ['test_xml31_bau_sources_are_cataloged_before_parser_promotion'] |
| R3-13 | #37 GAEB XML 3.1/3.2 compatibility | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R3-14 | #37 GAEB XML 3.1/3.2 compatibility | manifested | bvbs_xml31_bau_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
