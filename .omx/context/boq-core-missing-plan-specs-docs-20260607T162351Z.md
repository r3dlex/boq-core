# Context Snapshot: boq-core missing plan specs and documentation generation

Created: 20260607T162351Z

## Task statement
User wants to spec remaining original-plan gaps: (1) per-issue PRDs for #15-#36, (2) actual implementation planning, (4) official/source verification refresh, (5) crates.io/release setup, (6) GAEBXmlChecker integration, (7) public documentation generation for boq-core including guides/manuals via rustdoc or mdBook.

## Current evidence
- #37-#44 already have per-issue specs, PRDs, test-specs, and ralplan consensus artifacts.
- #15-#36 are open roadmap issues but do not yet have the same per-issue planning artifact set.
- #7 paid BVBS certification remains open/gated.

## Open issues #15-#36
- #36 [Plan XRechnung bridge from verified GAEB billing data](https://github.com/r3dlex/boq-core/issues/36) — v0.7 Rechnung and XRechnung bridge
- #35 [Implement X89 parser MVP with Rechnung fixtures](https://github.com/r3dlex/boq-core/issues/35) — v0.7 Rechnung and XRechnung bridge
- #34 [Design X89 Rechnung invoice data model](https://github.com/r3dlex/boq-core/issues/34) — v0.7 Rechnung and XRechnung bridge
- #33 [Add Texterstellung layout criteria matrix](https://github.com/r3dlex/boq-core/issues/33) — v0.6 BVBS Texterstellung support
- #32 [Implement Texterstellung X81/X82 rich text and table support](https://github.com/r3dlex/boq-core/issues/32) — v0.6 BVBS Texterstellung support
- #31 [Link X31 results against X86 contract baseline](https://github.com/r3dlex/boq-core/issues/31) — v0.5 BVBS Mengenermittlung X31 support
- #30 [Implement REB-VB 23.003 formula evaluation MVP](https://github.com/r3dlex/boq-core/issues/30) — v0.5 BVBS Mengenermittlung X31 support
- #29 [Implement X31 parser MVP for BVBS Mengenermittlung fixtures](https://github.com/r3dlex/boq-core/issues/29) — v0.5 BVBS Mengenermittlung X31 support
- #28 [Design X31 quantity takeoff domain model](https://github.com/r3dlex/boq-core/issues/28) — v0.5 BVBS Mengenermittlung X31 support
- #27 [Plan and implement Bauausführung X84 bid submission support](https://github.com/r3dlex/boq-core/issues/27) — v0.4 BVBS Bauausführung support
- #26 [Implement Bauausführung X83 request-for-quotation parser support](https://github.com/r3dlex/boq-core/issues/26) — v0.4 BVBS Bauausführung support
- #25 [Promote BVBS Bauausführung X83 fixture to supported test target](https://github.com/r3dlex/boq-core/issues/25) — v0.4 BVBS Bauausführung support
- #24 [Support legacy ANSI/Windows-1252 encoding paths for GAEB 90](https://github.com/r3dlex/boq-core/issues/24) — v0.3 Public API and parser robustness
- #23 [Add performance benchmarks for large BoQs](https://github.com/r3dlex/boq-core/issues/23) — v0.3 Public API and parser robustness
- #22 [Add fuzz and property tests for malformed GAEB inputs](https://github.com/r3dlex/boq-core/issues/22) — v0.3 Public API and parser robustness
- #21 [Add semantic versioning and release automation](https://github.com/r3dlex/boq-core/issues/21) — v0.3 Public API and parser robustness
- #20 [Stabilize public Rust API and crate documentation](https://github.com/r3dlex/boq-core/issues/20) — v0.3 Public API and parser robustness
- #19 [Deepen AVA XML rich text and schema-version handling](https://github.com/r3dlex/boq-core/issues/19) — v0.2 AVA certification readiness
- #18 [Prepare paid BVBS certification submission runbook](https://github.com/r3dlex/boq-core/issues/18) — v0.2 AVA certification readiness
- #17 [Add golden reports for BVBS AVA X81/X84/X86 fixtures](https://github.com/r3dlex/boq-core/issues/17) — v0.2 AVA certification readiness
- #16 [Build BVBS AVA criteria evidence matrix](https://github.com/r3dlex/boq-core/issues/16) — v0.2 AVA certification readiness
- #15 [Integrate GAEBXmlChecker into AVA certification workflow](https://github.com/r3dlex/boq-core/issues/15) — v0.2 AVA certification readiness

## Existing docs/artifacts inspected
- ./README.md
- ./docs/branch-protection.md
- ./docs/coverage-policy.md
- ./docs/testing-strategy.md
- ./docs/ultraqa-report.md
- ./gaeb/README.md

## Decision gaps
- Documentation generation approach: rustdoc only, mdBook manual, or both.
- Whether docs generation should be a planning-only spec now or an implementation issue/PRD with CI checks.
- Whether #15-#36 should receive PRDs/test-specs only, or issue body updates plus committed artifacts like #37-#44.
- Whether actual implementation planning should be one implementation sequencing spec or per-issue implementation PRDs.
