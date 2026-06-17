# Test spec issue #28: X31/X86 quantity takeoff domain model

## Test intent

Define safe fixture-readiness and regression-test expectations for issue #28 without introducing live downloads, paid certification work, or unsupported parser claims.

## Ranked roadmap fixture/test mapping

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R1-01 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R1-02 | #28-#31 X31/Mengenermittlung roadmap | manifested | official_gaeb_xml33_mengenermittlung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-03 | #28-#31 X31/Mengenermittlung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R1-04 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x31 | future_track | ['future_quantity_takeoff_x31_cataloged'] |
| R1-05 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_x86 | future_track | ['future_quantity_takeoff_x86_cataloged'] |
| R1-06 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for calculations PDF | reference_only | Reference-only certification visual output; not executable as parser fixture. |
| R1-07 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_qty_results_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-08 | #28-#31 X31/Mengenermittlung roadmap | gap | gap: manifest entry not present for reference drawing | reference_only | Reference-only visual/layout aid; not executable as parser fixture. |
| R1-09 | #28-#31 X31/Mengenermittlung roadmap | manifested | bvbs_xml33_mengenermittlung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

## Executable local fixtures

- None yet.

## Reference-only / planned gates

- `R1-01` -> artifact-only/reference / Reference-only planning artifact; not executable as parser fixture.
- `R1-02` -> manifested / Reference-only manifest artifact; not executable as parser fixture.
- `R1-03` -> artifact-only/reference / Schema/documentation reference for validation planning; not a parser fixture.
- `R1-04` -> manifested / ['future_quantity_takeoff_x31_cataloged']
- `R1-05` -> manifested / ['future_quantity_takeoff_x86_cataloged']
- `R1-06` -> gap / Reference-only certification visual output; not executable as parser fixture.
- `R1-07` -> manifested / Reference-only manifest artifact; not executable as parser fixture.
- `R1-08` -> gap / Reference-only visual/layout aid; not executable as parser fixture.
- `R1-09` -> manifested / Reference-only manifest artifact; not executable as parser fixture.

## Verification expectations

- Unit/integration tests may read only local files already declared in `boq-core/gaeb/manifest.toml`.
- Planned fixture rows require license-safe acquisition, checksum recording, and manifest updates before any parser test consumes them.
- Documentation/schema/PDF rows can support review checklists but must not be asserted as parser executable fixtures.
- A no-overclaim grep must reject wording that implies BVBS certification completion or supported parser status where the ledger says `planned-support` or `reference_only`.
