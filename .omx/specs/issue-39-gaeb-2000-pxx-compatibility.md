# Spec: GAEB 2000 / Pxx parser compatibility planning

## GitHub issue
- Issue: #39
- Milestone: v0.8 Format compatibility expansion

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## First architecture decision
Create an ADR deciding a separate `gaeb2000` parser boundary with tag/keyword tokenization, distinct from GAEB XML and GAEB 90.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb2000_priced_gist | developer_example | gaeb2000 | D86/P86 priced sample | future_track | download only as text fixture with checksum | developer gist; license note required | future_gaeb2000_priced_gist_cataloged |
| dangl_ava_gaeb2000_examples | developer_repo | gaeb2000 | GaebFiles Pxx/Dxx examples | future_track | clone/download gated; select fixtures only | developer-maintained examples; license note required | future_dangl_ava_gaeb2000_cataloged |
| gaeb2000_xml_mapping_chart | interactive_schema | mapping_reference | GAEB 2.1 to XML mapping | reference_only | no CI dependency on external HTML | mapping reference only; not GAEB 2000 support evidence | reference_gaeb2000_mapping_chart |

## Constraints / non-goals
- No paid actions or external certification/payment/submission.
- No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Concrete test names to plan
- test_gaeb2000_manifest_sources_are_future_or_reference_only
- test_gaeb2000_tokenizer_handles_begin_end_nesting
- test_gaeb2000_tokenizer_reports_unclosed_begin_blocks
- test_gaeb2000_phase_detector_maps_p81_to_p86
- test_gaeb2000_mapping_chart_is_not_used_as_runtime_support_evidence

## Acceptance criteria
- [ ] Issue body links this spec, PRD, and test-spec.
- [ ] PRD contains a per-source support matrix.
- [ ] Test-spec contains concrete red/green test names and promotion gates.
- [ ] Any implementation follow-up starts with the first architecture decision above.
