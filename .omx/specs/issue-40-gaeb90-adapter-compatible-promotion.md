# Spec: GAEB 90 adapter-compatible promotion planning

## GitHub issue
- Issue: #40
- Milestone: v0.8 Format compatibility expansion

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## Candidate architecture decision before implementation
Before implementation, record or link a candidate gap-analysis ADR listing exact data required to promote GAEB 90 D81/D83 from parse-only to Obra-adapter-compatible (ARCH-001 DTO layer; ARCH-003 backend-prohibited).

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| dangl_ava_gaeb90_examples | developer_repo | gaeb90 | D81/D83 examples | future_track | select fixture download gated by checksum | developer examples; license note required | future_dangl_ava_gaeb90_cataloged |
| mwm_rialto_gaeb90_demo | commercial_demo | spreadsheet_middleware | GAEB 90/2000/XML conversion demo | reference_only | do not execute/download in CI | commercial demo; compatibility reference only | reference_mwm_rialto_gaeb90_roundtrip |

## Constraints / non-goals
- No paid actions or external certification/payment/submission.
- No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Concrete test names to plan
- test_gaeb90_adapter_gap_matrix_lists_required_fields
- test_gaeb90_d81_d83_hierarchy_extraction_red_tests
- test_gaeb90_windows1252_umlaut_decode_cases
- test_gaeb90_malformed_fixed_width_recovery_findings
- test_mwm_rialto_is_reference_only_non_executed

## Acceptance criteria
- [ ] Issue body links this spec, PRD, and test-spec.
- [ ] PRD contains a per-source support matrix.
- [ ] Test-spec contains concrete red/green test names and promotion gates.
- [ ] Any implementation follow-up records or links the candidate architecture decision above before implementation work begins.

## Ranked roadmap source audit

This section binds issue #40 to the canonical source/status ledger. It does not promote parser support beyond the statuses below.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R4-04 | #40 GAEB90 adapter-compatible promotion | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| R4-05 | #40 GAEB90 adapter-compatible promotion | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
