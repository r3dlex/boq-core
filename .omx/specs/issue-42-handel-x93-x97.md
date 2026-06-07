# Spec: Handel X93-X97 trade and commerce support planning

## GitHub issue
- Issue: #42
- Milestone: v0.9 Non-certification exchange tracks

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## First architecture decision
Create a boundary ADR deciding if Handel procurement entities belong in boq-core or a companion trade crate/module.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_handel_pkg | official_gaeb | handel | 3.3 X93-X97 package | future_track | manifest download with checksum/license note | official schema/sample package | future_handel_33_cataloged |
| gaeb32_handel_pkg | official_gaeb | handel | 3.2 X93-X97 package | future_track | manifest download with checksum/license note | official schema/sample package | future_handel_32_cataloged |
| schema_x93_33_chart | interactive_schema | handel | 3.3 X93 | reference_only | no CI dependency on external HTML | schema chart only | reference_x93_33_schema_chart |
| schema_x94_33_chart | interactive_schema | handel | 3.3 X94 | reference_only | no CI dependency on external HTML | schema chart only | reference_x94_33_schema_chart |
| schema_x93_32_chart | interactive_schema | handel | 3.2 X93 | reference_only | no CI dependency on external HTML | schema chart only | reference_x93_32_schema_chart |

## Constraints / non-goals
- No paid actions or external certification/payment/submission.
- No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Concrete test names to plan
- test_handel_sources_are_cataloged_by_phase_x93_x94_x96_x97
- test_handel_boundary_adr_exists_before_parser_modules
- test_trade_document_is_not_classified_as_boq
- test_x93_x94_phase_detector_red_tests
- test_handel_interactive_schema_charts_are_reference_only

## Acceptance criteria
- [ ] Issue body links this spec, PRD, and test-spec.
- [ ] PRD contains a per-source support matrix.
- [ ] Test-spec contains concrete red/green test names and promotion gates.
- [ ] Any implementation follow-up starts with the first architecture decision above.
