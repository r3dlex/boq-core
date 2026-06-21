# Spec issue #31: X31/X86 baseline linking

## Scope

Implement deterministic X31 measurement-to-X86 contract baseline linking by BoQ ordinal inside the `boq_core::x31` domain boundary. The result is an audit/progress-link report for billing readiness only.

## Non-goals

- No paid BVBS certification action.
- No live/source download during unit tests.
- No invoice, XRechnung, payment, adapter export, or roundtrip generation.
- No parser support promotion beyond existing honest support statuses.
- No duplicate issue creation; this issue remains the owning lane for the rows listed below.
- No direct Obra backend imports.

## API

- `link_x31_to_x86_baseline(measurements, baseline) -> X31X86ProgressReport`
- `X31X86ProgressReport` contains baseline relation metadata, deterministic rows, findings, `invoice_generated = false`, and metadata.
- `X31X86ProgressRow` contains X31 row id/ordinal/quantity/unit, X86 baseline quantity/unit/unit price, computed progress value, and link status.
- `X31X86LinkStatus` values: `matched`, `missing_measurement_ordinal`, `missing_baseline_item`, `mismatched`.

## Matching rules

1. Traverse X86 BoQ item nodes recursively and index item nodes by ordinal.
2. Process X31 rows in source order.
3. Missing X31 ordinal -> nonfatal finding `x31_x86_missing_measurement_ordinal`.
4. Ordinal absent from X86 -> nonfatal finding `x31_x86_unmatched_measurement`.
5. Unit mismatch -> nonfatal finding `x31_x86_unit_mismatch` and row status `mismatched`.
6. Measured quantity greater than baseline quantity -> nonfatal finding `x31_x86_quantity_exceeds_baseline` and row status `mismatched`.
7. If measured quantity and unit price exist, compute `progress_value = measured_quantity * unit_price` with checked decimal arithmetic.
8. Never generate invoices; report field `invoice_generated` must remain false.

## Ranked roadmap source audit

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-04 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x31 | supported_parse_only | Parser-readiness fixture from #29; linking tests use synthetic local X31 rows. |
| R1-05 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x86 | future_track | Contract-baseline concept is exercised with synthetic local X86 domain fixtures; no support promotion. |
| R1-06 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for calculations PDF | reference_only | Reference-only certification visual output; not executable as parser fixture. |

## Acceptance criteria

- Ordinal matching is covered by tests.
- Missing and mismatched references produce structured audit findings.
- No invoice/XRechnung generation is claimed or performed.
- Integration-style tests cover successful and failed matching.
