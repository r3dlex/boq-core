# Spec: GAEB XML 3.4 beta schema and changelog impact tracking

## GitHub issue
- Issue: #38
- Milestone: v0.8 Format compatibility expansion

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## First architecture decision
Create a beta-impact ADR that records 3.4 as reference-only and identifies sustainability/lifecycle/carbon descriptor model extension points.

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
- [ ] Issue body links this spec, PRD, and test-spec.
- [ ] PRD contains a per-source support matrix.
- [ ] Test-spec contains concrete red/green test names and promotion gates.
- [ ] Any implementation follow-up starts with the first architecture decision above.
