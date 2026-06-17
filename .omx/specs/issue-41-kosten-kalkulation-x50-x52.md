# Spec: Kosten und Kalkulation X50-X52 support planning

## GitHub issue
- Issue: #41
- Milestone: v0.9 Non-certification exchange tracks

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## First architecture decision
Create a boundary ADR deciding whether costing belongs in boq-core modules or a companion crate before implementation.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_kosten_kalkulation_pkg | official_gaeb | kosten_kalkulation | 3.3 X50-X52 package | future_track | manifest download with checksum/license note | official schema/sample package | future_kosten_kalkulation_33_cataloged |
| gaeb32_kalkulation_pkg | official_gaeb | kosten_kalkulation | 3.2 X50-X52 package | future_track | manifest download with checksum/license note | official schema/sample package | future_kalkulation_32_cataloged |
| schema_x50_33_chart | interactive_schema | kosten_kalkulation | 3.3 X50 | reference_only | no CI dependency on external HTML | schema chart only | reference_x50_33_schema_chart |
| schema_x52_33_chart | interactive_schema | kosten_kalkulation | 3.3 X52 | reference_only | no CI dependency on external HTML | schema chart only | reference_x52_33_schema_chart |
| schema_x52_32_chart | interactive_schema | kosten_kalkulation | 3.2 X52 | reference_only | no CI dependency on external HTML | schema chart only | reference_x52_32_schema_chart |

## Constraints / non-goals
- No paid actions or external certification/payment/submission.
- No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Concrete test names to plan
- test_costing_sources_are_cataloged_by_phase_x50_x51_x52
- test_costing_boundary_adr_exists_before_parser_modules
- test_cost_component_model_red_tests
- test_x52_item_reference_mapping_red_tests
- test_kosten_interactive_schema_charts_are_reference_only

## Acceptance criteria
- [ ] Issue body links this spec, PRD, and test-spec.
- [ ] PRD contains a per-source support matrix.
- [ ] Test-spec contains concrete red/green test names and promotion gates.
- [ ] Any implementation follow-up starts with the first architecture decision above.

## Ranked roadmap source audit

This section binds issue #41 to the canonical source/status ledger. It does not promote parser support beyond the statuses below.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R6-01 | #41 Kosten/Kalkulation X50-X52 | manifested | official_gaeb_xml33_kosten_und_kalkulation | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R6-02 | #41 Kosten/Kalkulation X50-X52 | manifested | official_gaeb_xml32_kalkulation | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R6-03 | #41 Kosten/Kalkulation X50-X52 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R6-04 | #41 Kosten/Kalkulation X50-X52 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R6-05 | #41 Kosten/Kalkulation X50-X52 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
