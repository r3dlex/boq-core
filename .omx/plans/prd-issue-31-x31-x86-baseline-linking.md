# PRD: Link X31 results against X86 contract baseline

## Issue
- GitHub issue: #31
- Milestone: v0.5 BVBS Mengenermittlung X31 support

## Product outcome
Measured X31 quantities link to X86 contract baseline items by ordinal/baseline metadata for billing-readiness evidence.

## Source/status anchors
- BVBS X31: measurement fixture.
- BVBS X86 Mengenermittlung: contract baseline fixture.

## Blocking dependencies
- Depends on #29 X31 parser MVP and X86 baseline fixture promotion/registration.

## Requirements
- [ ] Define mismatch/unmatched behavior before integration logic.
- [ ] Link quantities to contract items by ordinal.
- [ ] Produce deterministic progress report/findings.

## Planned tests
- [ ] `test_x31_links_to_x86_by_ordinal`
- [ ] `test_x31_x86_quantity_mismatch_reports_finding`
- [ ] `test_x31_unmatched_measurement_is_nonfatal`
- [ ] `test_linked_progress_report_is_deterministic`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #31 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-04 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x31 | future_track | ['future_quantity_takeoff_x31_cataloged'] |
| R1-05 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x86 | future_track | ['future_quantity_takeoff_x86_cataloged'] |
| R1-06 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for calculations PDF | reference_only | Reference-only certification visual output; not executable as parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
