# Spec: Zeitvertrag X83Z/X84Z framework-contract support planning

## GitHub issue
- Issue: #43
- Milestone: v0.9 Non-certification exchange tracks

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## Candidate architecture decision before implementation
ARCH-012 records the boundary decision: Zeitvertrag X83Z/X84Z remains reference-only Z-phase framework-contract planning; future parser/model promotion requires safe fixtures and red/green tests, and ordinary X83/X84 behavior must not change in this issue.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_zeitvertrag_pkg | official_gaeb | zeitvertrag | 3.3 X83Z/X84Z package | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | reference schema/sample package; not executable parser fixture | reference_zeitvertrag_33_package_cataloged |
| gaeb32_zeitvertrag_pkg | official_gaeb | zeitvertrag | 3.2 package | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | reference schema/sample package; not executable parser fixture | reference_zeitvertrag_32_package_cataloged |
| gaeb32_zeitvertrag_examples | official_gaeb | zeitvertrag | 3.2 examples | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | reference examples; not executable parser fixture | reference_zeitvertrag_32_examples_cataloged |
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
- [x] Issue body links this spec, PRD, and test-spec.
- [x] PRD contains a per-source support matrix.
- [x] Test-spec contains concrete red/green test names and promotion gates.
- [x] ARCH-012 records the candidate architecture decision before parser/model promotion.

## Ranked roadmap source audit

This section binds issue #43 to the canonical source/status ledger. It does not promote parser support beyond the statuses below.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R7-01 | #43 Zeitvertrag X83Z/X84Z | manifested | official_gaeb_xml33_zeitvertrag | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-02 | #43 Zeitvertrag X83Z/X84Z | manifested | official_gaeb_xml32_zeitvertrag | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-03 | #43 Zeitvertrag X83Z/X84Z | manifested | official_gaeb_xml32_zeitvertrag_examples | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-04 | #43 Zeitvertrag X83Z/X84Z | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R7-05 | #43 Zeitvertrag X83Z/X84Z | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R7-06 | #43 Zeitvertrag X83Z/X84Z | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |


## Issue #43 delivery notes
- Boundary ADR: `.archgate/adrs/ARCH-012-zeitvertrag-z-phase-boundary.md`.
- Boundary matrix: `docs/fixtures/zeitvertrag-x83z-x84z-boundary.md`.
- Tests: `tests/zeitvertrag_boundary.rs` locks the manifested Zeitvertrag source rows, interactive chart reference-only policy, X83Z/X84Z not-misclassified-as-X83/X84 behavior, framework discount/premium obligations, and support-policy `ReferenceOnly` behavior.
- Support status: no X83Z/X84Z parser or adapter support is promoted; official rows remain `reference_only`.
