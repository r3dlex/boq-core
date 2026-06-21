# Spec: Handel X93-X97 trade and commerce support planning

## GitHub issue
- Issue: #42
- Milestone: v0.9 Non-certification exchange tracks

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## Candidate architecture decision before implementation
ARCH-011 records the boundary decision: Handel X93-X97 remains reference-only planning in this issue; future parser/model promotion may add a dedicated boq-core Handel module only after safe fixtures and red/green tests, while procurement workflows and Obra-specific trade behavior belong in a companion trade crate or Obra layer.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_handel_pkg | official_gaeb | handel | 3.3 X93-X97 package | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | reference schema/sample package; not executable parser fixture | reference_handel_33_package_cataloged |
| gaeb32_handel_pkg | official_gaeb | handel | 3.2 X93-X97 package | reference_only | artifact-only reference; no invented manifest URL until verified source is available | reference schema/sample package; not executable parser fixture | reference_handel_32_package_cataloged |
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
- [x] Issue body links this spec, PRD, and test-spec.
- [x] PRD contains a per-source support matrix.
- [x] Test-spec contains concrete red/green test names and promotion gates.
- [x] ARCH-011 records the candidate architecture decision before parser/model promotion.

## Ranked roadmap source audit

This section binds issue #42 to the canonical source/status ledger. It does not promote parser support beyond the statuses below.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R8-01 | #42 Handel X93-X97 | manifested | official_gaeb_xml33_handel | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R8-02 | #42 Handel X93-X97 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-03 | #42 Handel X93-X97 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-04 | #42 Handel X93-X97 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-05 | #42 Handel X93-X97 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |


## Issue #42 delivery notes
- Boundary ADR: `.archgate/adrs/ARCH-011-handel-boundary.md`.
- Boundary matrix: `docs/fixtures/handel-x93-x97-boundary.md`.
- Tests: `tests/handel_boundary.rs` locks the official 3.3 Handel source row, artifact-only 3.2/chart policies, advisory X93/X94/X96/X97 phase detection, trade-not-BOQ negative contract, and support-policy `ReferenceOnly` behavior.
- Support status: no X93/X94/X96/X97 parser or adapter support is promoted; official rows remain `reference_only`.
