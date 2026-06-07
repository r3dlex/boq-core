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
