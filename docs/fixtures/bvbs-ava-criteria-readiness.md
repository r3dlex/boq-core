# BVBS AVA criteria readiness matrix

`gaeb/criteria/bvbs_ava_matrix.toml` maps the current BVBS AVA readiness
criteria to supported fixture phases, parser behavior, tests, planned golden
reports, and manual evidence gates.

The matrix is evidence for the paid-certification runbook, not paid
certification itself. All entries keep `certification_claim = false`; paid or
official BVBS submission remains blocked by the explicit authorization gate in
#7 and the runbook work in #18.

## Evidence status values

- `automated` — covered by an existing automated unit/integration/golden test.
- `planned_golden` — parser behavior is covered by tests today and deterministic
  golden report material is scheduled in #17.
- `manual_required` — a reviewed human-local evidence artifact is required
  before paid submission.
- `gap` — intentionally unsupported or gated behavior that must not be
  overclaimed.

## Current AVA coverage

- `ava_x81_import` maps `bvbs_xml33_ava_x81` to
  `ava_x81_imports_to_rich_model_and_obra_snapshot`.
- `ava_x84_import` maps `bvbs_xml33_ava_x84` to
  `ava_x84_imports_priced_bid_snapshot`.
- `ava_x86_import` maps `bvbs_xml33_ava_x86` to
  `ava_x86_imports_contract_award_snapshot`.
- `ava_checker_comparison_if_required` points to the optional local-only
  GAEBXmlChecker readiness workflow.
- `ava_export_roundtrip_if_required` is an explicit gap until export/roundtrip
  support has tests and evidence.
