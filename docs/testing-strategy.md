# Testing Strategy

`boq-core` follows TDD: add or update failing tests first for each parser/model/fixture behavior, then implement the smallest production code that satisfies them.

## Test types

- **Unit tests:** parser slices, model invariants, support-status logic, adapter mapping, fixture manifest validation, archive safety helpers.
- **Integration tests:** real GAEB fixtures from BVBS, official GAEB packages, Dangl/developer examples, and reference templates.
- **Conformance tests:** BVBS criteria-matrix checks for certification-path readiness.
- **Snapshot/golden tests:** rich model and Obra adapter outputs for stable fixtures.
- **Reference/future tests:** assert future/reference fixtures are cataloged but not overclaimed.

## BVBS certification coverage target

The public BVBS material supplied for this project is tracked as five certification fixture areas plus a checker/tooling track:

1. AVA (X81/X84/X86) — first supported path.
2. Bauausführung / construction execution (X83/X84) — follow-on.
3. Mengenermittlung / quantity takeoff (X31/X86) — X31 parser/canonical quantity evidence is `supported_parse_only` for selected synthetic-evidence paths; BVBS conformance remains download-on-demand/follow-on.
4. Texterstellung / specification authoring (X81/X82) — follow-on.
5. Legacy GAEB XML 3.2 AVA/Bauausführung fixtures — follow-on compatibility.
6. GAEB XML Checker — reference-only tooling; catalog checksum/version, do not execute in CI by default.

Only AVA XML 3.3, explicit GAEB 90 parse-only fixtures, Bauausführung parser/adapter-readiness slices, and the selected X31 parser-backed canonical quantity slice may be promoted in the MVP. Other BVBS fixtures stay `future_track` or `reference_only` until a failing-test-first implementation and criteria matrix promote them. X31 promotion here is synthetic parser evidence and does not claim BVBS fixture conformance.

## Non-certification fixture coverage

Non-certification sources must still have integration/reference tests:

- Official GAEB schema/sample packages for XML 3.3 (Leistungsverzeichnis, Mengenermittlung, Rechnung, Kosten/Kalkulation, Zeitvertrag, Handel), XML 3.2, XML 3.1, and XML 3.4 beta schema/changelog reference inputs.
- Dangl.AVA, Dangl C++, Dangl ÖNORM, AVACloud, and Sportheim GAEB 2000 gist examples.
- GAEB-Online spreadsheet template and generator executable reference.

Reference-only executable files must never run in CI.
