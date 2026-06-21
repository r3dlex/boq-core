# PRD: Link X31 results against X86 contract baseline

## Issue
- GitHub issue: #31
- Milestone: v0.5 BVBS Mengenermittlung X31 support

## Product outcome
Measured X31 quantities link to X86 contract baseline items by ordinal for billing-readiness evidence, while explicitly avoiding invoice or XRechnung generation claims.

## Source/status anchors
- BVBS X31: `supported_parse_only` measurement fixture support from #29.
- BVBS X86: contract baseline data used as parser/domain input only.
- This issue produces an audit/progress-link report, not a billing document.

## Blocking dependencies
- Depends on #29 X31 parser MVP and existing X86 baseline parser/domain model.
- Builds on #30 formula evaluation MVP but does not require formula re-evaluation for linking.

## Requirements
- [x] Define mismatch/unmatched behavior before integration logic.
- [x] Link quantities to contract items by ordinal.
- [x] Produce deterministic progress report/findings.
- [x] Keep invoice/XRechnung generation out of scope and represented as `invoice_generated = false`.

## Delivered behavior
- `boq_core::x31::link_x31_to_x86_baseline` links each X31 measurement row to X86 baseline items by BoQ ordinal.
- `X31X86ProgressReport` includes deterministic rows, baseline relation metadata, findings, and `invoice_generated = false`.
- `X31X86LinkStatus` distinguishes matched rows, missing measurement ordinals, missing baseline items, and mismatched rows.
- Findings cover missing X31 ordinals, unmatched X86 baseline items, unit mismatches, and measured quantities exceeding baseline quantities.

## Executable tests
- [x] `test_x31_links_to_x86_by_ordinal`
- [x] `test_x31_x86_quantity_mismatch_reports_finding`
- [x] `test_x31_unmatched_measurement_is_nonfatal`
- [x] `test_linked_progress_report_is_deterministic`

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #31 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-04 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x31 | supported_parse_only | Parser-readiness fixture from #29; linking tests use synthetic local X31 rows. |
| R1-05 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x86 | future_track | Contract-baseline concept is exercised with synthetic local X86 domain fixtures; no support promotion. |
| R1-06 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for calculations PDF | reference_only | Reference-only certification visual output; not executable as parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.
