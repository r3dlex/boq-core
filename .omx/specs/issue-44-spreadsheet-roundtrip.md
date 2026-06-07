# Spec: Spreadsheet roundtrip helper planning

## GitHub issue
- Issue: #44
- Milestone: v0.9 Non-certification exchange tracks

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## First architecture decision
Create a boundary ADR deciding core vs companion crate/examples-only for spreadsheet roundtrip helpers before adding dependencies.

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
- [ ] Issue body links this spec, PRD, and test-spec.
- [ ] PRD contains a per-source support matrix.
- [ ] Test-spec contains concrete red/green test names and promotion gates.
- [ ] Any implementation follow-up starts with the first architecture decision above.
