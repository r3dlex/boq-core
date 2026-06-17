# Test spec issue #32: Texterstellung rich text and tables

## Test intent

Define safe fixture-readiness and regression-test expectations for issue #32 without introducing live downloads, paid certification work, or unsupported parser claims.

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R5-01 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x81 | future_track | ['future_texterstellung_x81_cataloged'] |
| R5-02 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_text_x82 | future_track | ['future_texterstellung_x82_cataloged'] |
| R5-03 | #32-#33 Texterstellung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

## Executable local fixtures

- None yet.

## Reference-only / planned gates

- `R5-01` -> manifested / ['future_texterstellung_x81_cataloged']
- `R5-02` -> manifested / ['future_texterstellung_x82_cataloged']
- `R5-03` -> artifact-only/reference / Reference-only planning artifact; not executable as parser fixture.
- `R5-04` -> manifested / Reference-only manifest artifact; not executable as parser fixture.

## Verification expectations

- Unit/integration tests may read only local files already declared in `boq-core/gaeb/manifest.toml`.
- Planned fixture rows require license-safe acquisition, checksum recording, and manifest updates before any parser test consumes them.
- Documentation/schema/PDF rows can support review checklists but must not be asserted as parser executable fixtures.
- A no-overclaim grep must reject wording that implies BVBS certification completion or supported parser status where the ledger says `planned-support` or `reference_only`.
