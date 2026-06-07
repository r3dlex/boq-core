# Spec: GAEB XML 3.1 and 3.2 compatibility track

## GitHub issue
- Issue: #37
- Milestone: v0.8 Format compatibility expansion

## Intent
Plan this source family as an execution-ready future track while preserving support-status honesty.

## First architecture decision
Add a compatibility ADR documenting XML 3.1/3.2 namespace/schema differences versus the current XML 3.3 AVA parser before parser changes.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb_xml32_doc | official_gaeb | compatibility | 3.2 docs | reference_only | local/manual only | PDF docs; do not assert runtime support | docs_reference_gaeb_xml32 |
| gaeb_xml32_lv_schema | official_gaeb | leistungsverzeichnis | 3.2 X81-X87 schema package | reference_only | manifest download gated by checksum | official schema package; no payload without license check | schema_reference_gaeb_xml32_lv |
| gaeb_xml32_x83_interactive | interactive_schema | bauausfuehrung | 3.2 X83 | reference_only | no CI dependency on external HTML | public interactive chart; documentation only | schema_reference_gaeb_xml32_x83 |
| bvbs_xml32_ava_x81 | bvbs | ava | 3.2 X81 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x81_cataloged |
| bvbs_xml32_ava_x84 | bvbs | ava | 3.2 X84 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x84_cataloged |
| bvbs_xml32_ava_x86 | bvbs | ava | 3.2 X86 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x86_cataloged |
| bvbs_xml32_bau_x83 | bvbs | bauausfuehrung | 3.2 X83 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_bau_x83_cataloged |
| gaeb_xml31_doc | official_gaeb | compatibility | 3.1 docs | reference_only | local/manual only | PDF docs; no runtime support claim | docs_reference_gaeb_xml31 |
| gaeb_xml31_muster | official_gaeb | compatibility | 3.1 2009-12 Musterdateien | future_track | fixture download only with checksum/license note | official examples; no support until tests pass | future_xml31_musterdateien_cataloged |
| gaeb_xml31_x81_x87_schema | official_gaeb | compatibility | 3.1 X81-X83/X85-X87 schemas | reference_only | manifest download gated by checksum | schema package only | schema_reference_gaeb_xml31_x81_x87 |

## Constraints / non-goals
- No paid actions or external certification/payment/submission.
- No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Concrete test names to plan
- test_manifest_catalogs_gaeb_xml31_and_xml32_sources
- test_xml_version_detector_distinguishes_31_32_33_namespaces
- test_xml32_ava_fixtures_remain_future_track_until_parser_promotion
- test_xml31_schema_sources_remain_reference_only
- test_unsupported_legacy_xml_features_emit_structured_findings

## Acceptance criteria
- [ ] Issue body links this spec, PRD, and test-spec.
- [ ] PRD contains a per-source support matrix.
- [ ] Test-spec contains concrete red/green test names and promotion gates.
- [ ] Any implementation follow-up starts with the first architecture decision above.
