# Test spec issue #31: X31/X86 baseline linking

## Test intent

Define deterministic local regression tests for linking X31 measurement rows to X86 contract baseline items without live downloads, paid certification work, unsupported parser claims, invoice generation, or XRechnung generation.

## Executable local tests

- `test_x31_links_to_x86_by_ordinal`
  - Synthetic X31 measurement row matches a synthetic nested X86 baseline item by ordinal.
  - Verifies baseline kind, baseline quantity, unit price, computed progress value, no findings, and `invoice_generated = false`.
- `test_x31_x86_quantity_mismatch_reports_finding`
  - X31 measured quantity exceeds X86 baseline quantity.
  - Verifies `mismatched` status and `x31_x86_quantity_exceeds_baseline` finding.
- `test_x31_unmatched_measurement_is_nonfatal`
  - Covers an X31 ordinal missing from X86 and an X31 row missing an ordinal.
  - Verifies nonfatal row statuses and two structured findings.
- `test_linked_progress_report_is_deterministic`
  - Runs the same input twice and verifies report equality and deterministic row ordering.
  - Verifies unit mismatch finding and no invoice side effect.

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-04 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x31 | supported_parse_only | Parser-readiness fixture from #29; linking tests use synthetic local X31 rows. |
| R1-05 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x86 | future_track | Contract-baseline concept is exercised with synthetic local X86 domain fixtures; no support promotion. |
| R1-06 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for calculations PDF | reference_only | Reference-only certification visual output; not executable as parser fixture. |

## Verification expectations

- Unit/integration tests may read only local files already declared in `boq-core/gaeb/manifest.toml` or use inline synthetic data.
- Planned fixture rows require license-safe acquisition, checksum recording, and manifest updates before any parser test consumes them.
- Documentation/schema/PDF rows can support review checklists but must not be asserted as parser executable fixtures.
- A no-overclaim grep must reject wording that implies BVBS certification completion, invoice generation, XRechnung generation, or supported parser status where the ledger says `future_track` or `reference_only`.
