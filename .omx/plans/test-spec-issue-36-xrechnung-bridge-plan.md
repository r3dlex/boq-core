# Test spec issue #36: XRechnung bridge plan

## Test intent

Define safe fixture-readiness and regression-test expectations for issue #36 without introducing live downloads, paid certification work, or unsupported parser claims.

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-04 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

## Executable local fixtures

- None yet.

## Reference-only / planned gates

- `R2-04` -> artifact-only/reference / Reference-only planning artifact; not executable as parser fixture.
- `R2-05` -> artifact-only/reference / Tooling or guidance reference for roundtrip planning; not vendored or executed.

## Verification expectations

- Unit/integration tests may read only local files already declared in `boq-core/gaeb/manifest.toml`.
- Planned fixture rows require license-safe acquisition, checksum recording, and manifest updates before any parser test consumes them.
- Documentation/schema/PDF rows can support review checklists but must not be asserted as parser executable fixtures.
- A no-overclaim grep must reject wording that implies BVBS certification completion or supported parser status where the ledger says `planned-support` or `reference_only`.

## Delivery notes

- No production XRechnung emission is implemented or claimed.
- `docs/fixtures/xrechnung-bridge-plan.md` records mapping assumptions, legal/compliance boundaries, required X31/X86/X89 data, and separate standards/dependency evaluation gates.
- Regression tests in `tests/xrechnung_bridge_plan.rs` keep the bridge plan, PRD, spec, and test spec synchronized.

## Automated regression tests

- `test_xrechnung_bridge_plan_requires_verified_quantities` checks that X31, X86, and X89 prerequisites are listed as blocking bridge inputs.
- `test_xrechnung_mapping_matrix_has_required_invoice_fields` checks the required-field matrix rows.
- `test_xrechnung_export_is_feature_gated_or_absent` checks that no `xrechnung` module/export support appears and the X89 boundary stays false.
- `test_docs_do_not_claim_xrechnung_generation` prevents documentation overclaims.
- `test_issue_36_artifacts_stay_in_sync` checks R2-04/R2-05 and No production XRechnung emission language across artifacts.
