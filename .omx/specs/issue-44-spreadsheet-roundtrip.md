# Spec: Spreadsheet roundtrip helper planning

## GitHub issue
- Issue: #44
- Milestone: v0.9 Non-certification exchange tracks

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## Candidate architecture decision before implementation
ARCH-013 records the boundary decision: spreadsheet roundtrip helpers remain reference-only examples/companion-crate planning in this issue; no spreadsheet dependency, executable run, parser support, export support, or roundtrip capability is added.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb_online_import_template | spreadsheet_template | spreadsheet_roundtrip | Excel import template | reference_only | download manually only; checksum/license note; no parser support claim | spreadsheet template reference | reference_gaeb_online_import_template |
| gaeb_online_excel_generator | executable_tool | spreadsheet_roundtrip | Excel generator .exe | reference_only | do not download/execute in CI | executable; reference only | reference_gaeb_online_excel_generator |
| mwm_rialto_demo | commercial_demo | spreadsheet_roundtrip | Excel conversion demo | reference_only | do not download/execute in CI | commercial/demo utility; reference only | reference_mwm_rialto_demo |
| easy_gaeb_browser | browser_utility | spreadsheet_roundtrip | Browser utility | reference_only | no CI dependency; no scraping/execution | external web utility; reference only | reference_easy_gaeb_browser |

## Constraints / non-goals
- No paid actions or external certification/payment/submission.
- No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Concrete test names to plan
- test_spreadsheet_sources_are_reference_only_non_executed
- test_roundtrip_boundary_adr_exists_before_dependencies
- test_oz_matching_reordered_columns_red_tests
- test_inserted_columns_do_not_break_oz_matching_red_tests
- test_missing_oz_rejects_roundtrip_red_tests

## Acceptance criteria
- [x] Issue body links this spec, PRD, and test-spec.
- [x] PRD contains a per-source support matrix.
- [x] Test-spec contains concrete red/green test names and promotion gates.
- [x] ARCH-013 records the candidate architecture decision before parser/helper promotion.

## Ranked roadmap source audit

This section binds issue #44 to the canonical source/status ledger. It does not promote parser support beyond the statuses below.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| A2-01 | #44 Spreadsheet roundtrip | manifested | gaeb_online_import_template | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-02 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| A2-03 | #44 Spreadsheet roundtrip | manifested | gaeb_online_generator_exe | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-04 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| A2-05 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |


## Issue #44 delivery notes
- Boundary ADR: `.archgate/adrs/ARCH-013-spreadsheet-roundtrip-boundary.md`.
- Boundary matrix: `docs/fixtures/spreadsheet-roundtrip-boundary.md`.
- Tests: `tests/spreadsheet_roundtrip_boundary.rs` locks reference-only/non-executed spreadsheet sources, no spreadsheet dependency, OZ matching with reordered/inserted columns, missing-OZ rejection, and artifact sync.
- Support status: no spreadsheet parser, export, executable, browser, or roundtrip helper support is promoted; relevant rows remain `reference_only`.
