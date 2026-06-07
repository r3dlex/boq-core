# Spec: Zeitvertrag X83Z/X84Z framework-contract support planning

## GitHub issue
- Issue: #43
- Milestone: v0.9 Non-certification exchange tracks

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## First architecture decision
Create a boundary ADR for Z-phase framework-contract handling before changing ordinary X83/X84 behavior.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_zeitvertrag_pkg | official_gaeb | zeitvertrag | 3.3 X83Z/X84Z package | future_track | manifest download with checksum/license note | official schema/sample package | future_zeitvertrag_33_cataloged |
| gaeb32_zeitvertrag_pkg | official_gaeb | zeitvertrag | 3.2 package | future_track | manifest download with checksum/license note | official schema/sample package | future_zeitvertrag_32_cataloged |
| gaeb32_zeitvertrag_examples | official_gaeb | zeitvertrag | 3.2 examples | future_track | manifest download with checksum/license note | official examples; license note required | future_zeitvertrag_32_examples_cataloged |
| schema_x83z_33_chart | interactive_schema | zeitvertrag | 3.3 X83Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x83z_33_schema_chart |
| schema_x84z_33_chart | interactive_schema | zeitvertrag | 3.3 X84Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x84z_33_schema_chart |
| schema_x83z_32_chart | interactive_schema | zeitvertrag | 3.2 X83Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x83z_32_schema_chart |

## Constraints / non-goals
- No paid actions or external certification/payment/submission.
- No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Concrete test names to plan
- test_zeitvertrag_sources_are_cataloged_by_z_phase
- test_z_phase_boundary_adr_exists_before_parser_changes
- test_x83z_x84z_are_not_misclassified_as_standard_x83_x84
- test_framework_discount_premium_red_tests
- test_zeitvertrag_interactive_schema_charts_are_reference_only

## Acceptance criteria
- [ ] Issue body links this spec, PRD, and test-spec.
- [ ] PRD contains a per-source support matrix.
- [ ] Test-spec contains concrete red/green test names and promotion gates.
- [ ] Any implementation follow-up starts with the first architecture decision above.
