# Test spec issue #34: X89 Rechnung model

## Test intent

Define safe fixture-readiness and regression-test expectations for issue #34 without introducing live downloads, paid certification work, or unsupported parser claims.

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R2-01 | #34-#36 Rechnung/XRechnung bridge | manifested | official_gaeb_xml33_rechnung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R2-02 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-03 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-04 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R2-05 | #34-#36 Rechnung/XRechnung bridge | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

## Executable local fixtures

- None yet.

## Reference-only / planned gates

- `R2-01` -> manifested / Reference-only manifest artifact; not executable as parser fixture.
- `R2-02` -> artifact-only/reference / Schema/documentation reference for validation planning; not a parser fixture.
- `R2-03` -> artifact-only/reference / Schema/documentation reference for validation planning; not a parser fixture.
- `R2-04` -> artifact-only/reference / Reference-only planning artifact; not executable as parser fixture.
- `R2-05` -> artifact-only/reference / Tooling or guidance reference for roundtrip planning; not vendored or executed.

## Verification expectations

- Unit/integration tests may read only local files already declared in `boq-core/gaeb/manifest.toml`.
- Planned fixture rows require license-safe acquisition, checksum recording, and manifest updates before any parser test consumes them.
- Documentation/schema/PDF rows can support review checklists but must not be asserted as parser executable fixtures.
- A no-overclaim grep must reject wording that implies BVBS certification completion or supported parser status where the ledger says `planned-support` or `reference_only`.
