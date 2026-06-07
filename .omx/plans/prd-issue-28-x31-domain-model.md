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
- [ ] Separate measurement model from BoQ item model.
- [ ] Represent formula rows, quantities, ordinals, attachments, and findings.
- [ ] Keep serialization deterministic.

## Planned tests
- [ ] `test_x31_domain_represents_formula_rows`
- [ ] `test_x31_domain_links_measurements_to_ordinal`
- [ ] `test_x31_domain_represents_attachments_as_findings_or_assets`
- [ ] `test_x31_domain_is_serializable`
