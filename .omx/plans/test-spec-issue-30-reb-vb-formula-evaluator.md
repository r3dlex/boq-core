# Test spec issue #30: REB-VB 23.003 formula evaluator MVP

## Test intent

Lock the safe evaluator subset for issue #30 without introducing live downloads, paid certification work, unsupported parser claims, or dynamic expression execution.

## Executable local tests

- `test_reb_formula_simple_arithmetic`
  - Verifies precedence and parentheses: `2 + 3 * (4 - 1) = 11`.
  - Verifies the public supported-subset list names multiplication.
- `test_reb_formula_quantity_result_precision`
  - Verifies comma/dot decimals and deterministic Decimal precision: `1,25 * 2 + 0.005 = 2.505`.
- `test_reb_formula_unsupported_expression_yields_finding`
  - Verifies unsupported functions/identifiers do not evaluate and return `reb_formula_unsupported_token` with supported-subset guidance.
- `test_formula_evaluator_never_panics_on_bad_input`
  - Verifies empty input, division by zero, unclosed parentheses, bad operator sequences, and malformed numbers all return findings.

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-01 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; informs evaluator subset only. |
| R1-03 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R1-07 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_results_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-08 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for reference drawing | reference_only | Reference-only visual/layout aid; not executable as parser fixture. |
| R1-09 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_mengenermittlung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

## Verification expectations

- Unit tests consume only inline/local expressions.
- Planned fixture rows require license-safe acquisition, checksum recording, and manifest updates before any parser test consumes them.
- Documentation/schema/PDF rows can support review checklists but must not be asserted as parser executable fixtures.
- A no-overclaim grep must reject wording that implies BVBS certification completion or supported parser status where the ledger says `planned-support` or `reference_only`.
