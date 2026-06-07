---
id: BOQ-PRD-002
title: Test spec for the GAEB parser MVP
status: accepted
date: 2026-06-07
tags: [boq-core, testing, coverage, conformance]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
prd_link: BOQ-PRD-001-gaeb-parser.md
test_spec_source: .omx/plans/test-spec-boq-core-gaeb-parser-20260606.md
tracked_by: r3dlex/boq-core issues #1–#6 (all closed via the MVP PRs)
subrepo_binding:
  boq-core: boq-core/src/model.rs (unit tests), boq-core/src/adapter/obra.rs (adapter tests), boq-core/src/gaeb_xml/mod.rs (integration tests), boq-core/src/gaeb90.rs (fixed-width tests), boq-core/tests/ (integration + conformance + property tests)
  workspace: docs/adr/WS-003-language-pack-matrix.md
acceptance_criteria:
  - 95% unit-test coverage across lines, functions, regions (cargo llvm-cov)
  - Unit tests cover format detection, fixed-width parser slicing, XML model extraction, rich model invariants, adapter mapping, error handling
  - Fixture/golden tests cover BVBS X81/X84/X86 (supported), Dangl GAEB 90 D81/D83 (supported), GAEB XML official examples (supported_parse_only or supported)
  - Schema/conformance tests validate against XSD where Rust-compatible
  - Fixture tooling tests cover xtask download/unpack/manifest/verify
  - No reference-only executable (.exe) is run in CI
  - Integration tests assert fixture → rich model → Obra adapter DTO → snapshot
  - Property/fuzz tests assert no-panic on malformed input
---

# BOQ-PRD-002: Test spec for the GAEB parser MVP

Test spec at `.omx/plans/test-spec-boq-core-gaeb-parser-20260606.md`. Four claims proven: (1) supported GAEB fixtures parse into the rich model; (2) the Obra adapter produces WBS/BOQ/line-item compatible DTOs without backend changes; (3) fixture acquisition is reproducible, checksummed, and license-aware; (4) support status claims are truthful and enforced.

Coverage tool: `cargo llvm-cov` with `--ignore-filename-regex 'src/bin/xtask.rs'` and `--fail-under-lines 95 --fail-under-functions 95 --fail-under-regions 95`. `xtask` is excluded because it is an external fixture-acquisition CLI; it has its own unit tests.
