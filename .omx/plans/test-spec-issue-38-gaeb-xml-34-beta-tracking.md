# Test Spec: GAEB XML 3.4 beta schema and changelog impact tracking

## Issue
- GitHub issue: #38

## Red/green order
1. Add fixture manifest/source-status tests for the per-source matrix.
2. Add detector/model/parser red tests named below.
3. Implement only the smallest behavior required to turn the tests green.
4. Run full protected-main gate stack.
5. Promote support_status only with green tests and review evidence.

## Concrete planned tests
- [ ] `test_gaeb_xml34_sources_are_reference_only` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_gaeb_xml34_does_not_promote_supported_versions` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_beta_sustainability_fields_are_recorded_as_model_impact_notes` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.
- [ ] `test_no_bvbs_certification_claim_for_xml34_beta` — add as a failing/red test before implementation; turn green only with source-backed behavior and support-status review.

## Per-source fixture/status checks
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb_xml34_beta_schema | official_gaeb | beta_compatibility | 3.4 beta schemas | reference_only | manual/manifest gated; no CI dependency until stable | beta schema package; no BVBS fixtures | reference_gaeb_xml34_beta_schema |
| gaeb_xml34_beta_changelog | official_gaeb | beta_compatibility | 3.4 beta changelog | reference_only | manual/manifest gated; no CI dependency until stable | beta changelog; documentation only | reference_gaeb_xml34_beta_changelog |

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

This section records how issue #38 may use the linked source rows as local fixtures, planned fixture gates, or reference-only evidence.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| A1-01 | #38 GAEB XML 3.4 beta tracking | manifested | official_gaeb_xml34_beta_schema | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A1-02 | #38 GAEB XML 3.4 beta tracking | manifested | official_gaeb_xml34_beta_changelog | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
