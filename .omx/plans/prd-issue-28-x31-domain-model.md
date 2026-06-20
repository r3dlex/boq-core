# PRD: Design X31 quantity takeoff domain model

## Issue
- GitHub issue: #28
- Milestone: v0.5 BVBS Mengenermittlung X31 support

## Product outcome
A distinct X31 measurement domain represents formula records, physical progress, attachments, and baseline links without overloading BoQ items.

## Source/status anchors
- GAEB X31 schema/docs: `reference_only`.
- BVBS X31: `future_track`.
- REB-VB 23.003: `reference_only` calculation rules.

## Requirements
- [x] Separate measurement model from BoQ item model.
- [x] Represent formula rows, quantities, ordinals, attachments, and findings.
- [x] Keep serialization deterministic.

## Specification artifacts
- Spec: `.omx/specs/issue-28-x31-domain-model.md`
- Test spec: `.omx/plans/test-spec-issue-28-x31-domain-model.md`

## Planned tests
- [x] `test_x31_domain_represents_formula_rows`
- [x] `test_x31_domain_links_measurements_to_ordinal`
- [x] `test_x31_domain_represents_attachments_as_findings_or_assets`
- [x] `test_x31_domain_is_serializable`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #28 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-01 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R1-02 | #28-#31 X31/Mengenermittlung roadmap | manifested | official_gaeb_xml33_mengenermittlung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-03 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R1-04 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x31 | future_track | ['future_quantity_takeoff_x31_cataloged'] |
| R1-05 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x86 | future_track | ['future_quantity_takeoff_x86_cataloged'] |
| R1-06 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for calculations PDF | reference_only | Reference-only certification visual output; not executable as parser fixture. |
| R1-07 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_results_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-08 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for reference drawing | reference_only | Reference-only visual/layout aid; not executable as parser fixture. |
| R1-09 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_mengenermittlung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
