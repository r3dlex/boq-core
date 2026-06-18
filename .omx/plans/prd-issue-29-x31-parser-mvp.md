# PRD: Implement X31 parser MVP for BVBS Mengenermittlung fixtures

## Issue
- GitHub issue: #29
- Milestone: v0.5 BVBS Mengenermittlung X31 support

## Product outcome
BVBS X31 fixtures parse into the X31 measurement domain with formula source preservation and loss-aware findings.

## Source/status anchors
- BVBS X31: `future_track`.
- GAEB XML 3.3 X31 schema: `reference_only` schema.

## Blocking dependencies
- Depends on #28 X31 domain model.

## Requirements
- [ ] Complete #28 model before parser behavior.
- [ ] Parse measurement groups and formula records.
- [ ] Detect attachments and unsupported features.

## Planned tests
- [ ] `test_bvbs_x31_parses_measurement_groups`
- [ ] `test_bvbs_x31_formula_records_preserve_source`
- [ ] `test_bvbs_x31_attachments_are_detected`
- [ ] `test_x31_parser_reports_unsupported_features`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #29 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-02 | #28-#31 X31/Mengenermittlung roadmap | manifested | official_gaeb_xml33_mengenermittlung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-03 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R1-04 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x31 | future_track | ['future_quantity_takeoff_x31_cataloged'] |
| R1-05 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x86 | future_track | ['future_quantity_takeoff_x86_cataloged'] |
| R1-06 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for calculations PDF | reference_only | Reference-only certification visual output; not executable as parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
