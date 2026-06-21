# Spec issue #30: REB-VB 23.003 formula evaluator MVP

## Scope

Implement a deterministic evaluator for the safe arithmetic subset of REB-VB 23.003 expressions needed by local X31 proof scenarios. The evaluator lives in the `boq_core::x31` boundary and returns either a `Decimal` quantity or structured findings.

## Non-goals

- No paid BVBS certification action.
- No live/source download during unit tests.
- No full REB-VB grammar or function library.
- No formula evaluation through scripting engines, `eval`, shell execution, or unsafe code.
- No parser support promotion beyond #29's `supported_parse_only` X31 parser readiness.
- No adapter/export/roundtrip promotion.
- No direct Obra backend imports.

## API

- `SUPPORTED_REB_VB_23003_SUBSET: &[&str]` lists supported constructs.
- `FormulaEvaluation { quantity: Option<Decimal>, findings: Vec<ValidationFinding> }` reports evaluated or unevaluated formulas.
- `evaluate_reb_vb_23003(expression: &str) -> FormulaEvaluation` evaluates the subset.

## Supported subset

- Decimal literals with dot or comma separators.
- Parentheses.
- Unary `+` and `-`.
- Binary `+`, `-`, `*`, `/` with standard precedence.
- Checked `rust_decimal::Decimal` arithmetic.

## Unevaluated findings

The evaluator must return `quantity = None` plus a finding for:
- empty expressions (`reb_formula_empty`)
- unsupported identifiers/functions (`reb_formula_unsupported_token`)
- syntax errors (`reb_formula_syntax_error`)
- malformed numbers (`reb_formula_number_parse_failed`)
- division by zero (`reb_formula_division_by_zero`)
- decimal overflow (`reb_formula_decimal_overflow`)

## Ranked roadmap source audit

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-01 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; informs evaluator subset only. |
| R1-03 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R1-07 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_results_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-08 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for reference drawing | reference_only | Reference-only visual/layout aid; not executable as parser fixture. |
| R1-09 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_mengenermittlung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

## Acceptance criteria

- The supported subset is explicit in the public API and tests.
- Evaluation is deterministic and uses checked decimal arithmetic.
- Unsupported or invalid formulas return structured unevaluated findings.
- The PRD, spec, and test spec for issue #30 all reference the same canonical source IDs.
