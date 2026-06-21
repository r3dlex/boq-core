# Spec: GAEB XML 3.4 beta schema and changelog impact tracking

## GitHub issue
- Issue: #38
- Milestone: v0.8 Format compatibility expansion

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## Candidate architecture decision before implementation
ARCH-007 records 3.4 as reference-only and identifies sustainability/lifecycle/carbon descriptor model extension points.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb_xml34_beta_schema | official_gaeb | beta_compatibility | 3.4 beta schemas | reference_only | manual/manifest gated; no CI dependency until stable | beta schema package; no BVBS fixtures | reference_gaeb_xml34_beta_schema |
| gaeb_xml34_beta_changelog | official_gaeb | beta_compatibility | 3.4 beta changelog | reference_only | manual/manifest gated; no CI dependency until stable | beta changelog; documentation only | reference_gaeb_xml34_beta_changelog |

## Constraints / non-goals
- No paid actions or external certification/payment/submission.
- No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Concrete test names to plan
- test_gaeb_xml34_sources_are_reference_only
- test_gaeb_xml34_does_not_promote_supported_versions
- test_beta_sustainability_fields_are_recorded_as_model_impact_notes
- test_no_bvbs_certification_claim_for_xml34_beta

## Acceptance criteria
- [x] Issue body links this spec, PRD, and test-spec.
- [x] PRD contains a per-source support matrix.
- [x] Test-spec contains concrete red/green test names and promotion gates.
- [x] Any implementation follow-up starts with the candidate architecture decision above.

## Ranked roadmap source audit

This section binds issue #38 to the canonical source/status ledger. It does not promote parser support beyond the statuses below.

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| A1-01 | #38 GAEB XML 3.4 beta tracking | manifested | official_gaeb_xml34_beta_schema | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A1-02 | #38 GAEB XML 3.4 beta tracking | manifested | official_gaeb_xml34_beta_changelog | reference_only | Reference-only manifest artifact; not executable as parser fixture. |


## Delivery notes
- ARCH-007 records the beta support boundary.
- `docs/fixtures/gaeb-xml34-beta-impact.md` lists sustainability, lifecycle, and carbon / CO2 impact extension points.
- `tests/gaeb_xml34_beta.rs` keeps manifest rows, parser policy, documentation, and no-certification claims in sync.
