# Test spec issue #33: Texterstellung layout criteria

## Test intent

Define safe fixture-readiness and regression-test expectations for issue #33 without introducing live downloads, paid certification work, or unsupported parser claims.

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R5-03 | #32-#33 Texterstellung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

## Executable local fixtures

- `gaeb/criteria/bvbs_texterstellung_matrix.toml` is parsed by `tests/texterstellung_criteria.rs` as the executable readiness matrix.
- `docs/fixtures/bvbs-texterstellung-criteria-readiness.md` is checked as the golden readiness report linking matrix criteria to tests and manual gaps.

## Reference-only / planned gates

- `R5-03` -> artifact-only/reference / Reference-only planning artifact; not executable as parser fixture.
- `R5-04` -> manifested / Reference-only manifest artifact; not executable as parser fixture.

## Verification expectations

- Unit/integration tests may read only local files already declared in `boq-core/gaeb/manifest.toml`.
- Planned fixture rows require license-safe acquisition, checksum recording, and manifest updates before any parser test consumes them.
- Documentation/schema/PDF rows can support review checklists but must not be asserted as parser executable fixtures.
- `test_text_criteria_matrix_covers_all_known_sections` verifies all known sections and rejects certification claims.
- `test_text_layout_manual_evidence_is_marked_manual` keeps visual/layout criteria manual or out-of-scope.
- `test_text_fixture_golden_reports_link_criteria` ensures golden readiness docs link automated criteria, tests, and fixtures.
- `test_text_support_claims_require_matrix_status` requires parser support claims to be backed by automated matrix rows and manifest statuses.
- A no-overclaim grep must reject wording that implies BVBS certification completion or supported parser status where the ledger says `planned-support` or `reference_only`.
