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
