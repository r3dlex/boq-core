# Context Snapshot: GAEB next-step specs and PRDs

Created: 20260607T073634Z

## Task statement
User invoked deep-interview to update GitHub issues and create specs/PRDs for GAEB compatibility, GAEB 3.4 beta tracking, GAEB 2000/Pxx, X50-X52 costing, X93-X97 Handel, Zeitvertrag, and spreadsheet roundtrip helpers based on supplied documentation/test file directories.

## Desired outcome
- Clarified scope for issue updates and spec artifacts.
- Updated GitHub issues for next-step roadmap.
- Specs for each next step.
- Afterwards run ralplan to create PRDs.

## Known repository facts
- #44 [Plan spreadsheet roundtrip import/export helpers](https://github.com/r3dlex/boq-core/issues/44) — milestone: v0.9 Non-certification exchange tracks
- #43 [Plan Zeitvertrag time-contract support](https://github.com/r3dlex/boq-core/issues/43) — milestone: v0.9 Non-certification exchange tracks
- #42 [Plan Handel X93-X97 trade and commerce support](https://github.com/r3dlex/boq-core/issues/42) — milestone: v0.9 Non-certification exchange tracks
- #41 [Plan Kosten und Kalkulation X50-X52 support](https://github.com/r3dlex/boq-core/issues/41) — milestone: v0.9 Non-certification exchange tracks
- #40 [Promote GAEB 90 from parse-only toward adapter-compatible support](https://github.com/r3dlex/boq-core/issues/40) — milestone: v0.8 Format compatibility expansion
- #39 [Plan GAEB 2000 Pxx parser compatibility](https://github.com/r3dlex/boq-core/issues/39) — milestone: v0.8 Format compatibility expansion
- #38 [Track GAEB XML 3.4 beta schema and changelog impact](https://github.com/r3dlex/boq-core/issues/38) — milestone: v0.8 Format compatibility expansion
- #37 [Add GAEB XML 3.1 and 3.2 compatibility track](https://github.com/r3dlex/boq-core/issues/37) — milestone: v0.8 Format compatibility expansion
- #36 [Plan XRechnung bridge from verified GAEB billing data](https://github.com/r3dlex/boq-core/issues/36) — milestone: v0.7 Rechnung and XRechnung bridge
- #35 [Implement X89 parser MVP with Rechnung fixtures](https://github.com/r3dlex/boq-core/issues/35) — milestone: v0.7 Rechnung and XRechnung bridge
- #34 [Design X89 Rechnung invoice data model](https://github.com/r3dlex/boq-core/issues/34) — milestone: v0.7 Rechnung and XRechnung bridge
- #33 [Add Texterstellung layout criteria matrix](https://github.com/r3dlex/boq-core/issues/33) — milestone: v0.6 BVBS Texterstellung support
- #32 [Implement Texterstellung X81/X82 rich text and table support](https://github.com/r3dlex/boq-core/issues/32) — milestone: v0.6 BVBS Texterstellung support
- #31 [Link X31 results against X86 contract baseline](https://github.com/r3dlex/boq-core/issues/31) — milestone: v0.5 BVBS Mengenermittlung X31 support
- #30 [Implement REB-VB 23.003 formula evaluation MVP](https://github.com/r3dlex/boq-core/issues/30) — milestone: v0.5 BVBS Mengenermittlung X31 support
- #29 [Implement X31 parser MVP for BVBS Mengenermittlung fixtures](https://github.com/r3dlex/boq-core/issues/29) — milestone: v0.5 BVBS Mengenermittlung X31 support
- #28 [Design X31 quantity takeoff domain model](https://github.com/r3dlex/boq-core/issues/28) — milestone: v0.5 BVBS Mengenermittlung X31 support
- #27 [Plan and implement Bauausführung X84 bid submission support](https://github.com/r3dlex/boq-core/issues/27) — milestone: v0.4 BVBS Bauausführung support
- #26 [Implement Bauausführung X83 request-for-quotation parser support](https://github.com/r3dlex/boq-core/issues/26) — milestone: v0.4 BVBS Bauausführung support
- #25 [Promote BVBS Bauausführung X83 fixture to supported test target](https://github.com/r3dlex/boq-core/issues/25) — milestone: v0.4 BVBS Bauausführung support
- #24 [Support legacy ANSI/Windows-1252 encoding paths for GAEB 90](https://github.com/r3dlex/boq-core/issues/24) — milestone: v0.3 Public API and parser robustness
- #23 [Add performance benchmarks for large BoQs](https://github.com/r3dlex/boq-core/issues/23) — milestone: v0.3 Public API and parser robustness
- #22 [Add fuzz and property tests for malformed GAEB inputs](https://github.com/r3dlex/boq-core/issues/22) — milestone: v0.3 Public API and parser robustness
- #21 [Add semantic versioning and release automation](https://github.com/r3dlex/boq-core/issues/21) — milestone: v0.3 Public API and parser robustness
- #20 [Stabilize public Rust API and crate documentation](https://github.com/r3dlex/boq-core/issues/20) — milestone: v0.3 Public API and parser robustness
- #19 [Deepen AVA XML rich text and schema-version handling](https://github.com/r3dlex/boq-core/issues/19) — milestone: v0.2 AVA certification readiness
- #18 [Prepare paid BVBS certification submission runbook](https://github.com/r3dlex/boq-core/issues/18) — milestone: v0.2 AVA certification readiness
- #17 [Add golden reports for BVBS AVA X81/X84/X86 fixtures](https://github.com/r3dlex/boq-core/issues/17) — milestone: v0.2 AVA certification readiness
- #16 [Build BVBS AVA criteria evidence matrix](https://github.com/r3dlex/boq-core/issues/16) — milestone: v0.2 AVA certification readiness
- #15 [Integrate GAEBXmlChecker into AVA certification workflow](https://github.com/r3dlex/boq-core/issues/15) — milestone: v0.2 AVA certification readiness
- #7 [Paid official BVBS certification milestone](https://github.com/r3dlex/boq-core/issues/7) — milestone: v0.2 AVA certification readiness

## Current milestones
- #1 v0.2 AVA certification readiness: open 6, closed 0
- #2 v0.3 Public API and parser robustness: open 5, closed 0
- #3 v0.4 BVBS Bauausführung support: open 3, closed 0
- #4 v0.5 BVBS Mengenermittlung X31 support: open 4, closed 0
- #5 v0.6 BVBS Texterstellung support: open 2, closed 0
- #6 v0.7 Rechnung and XRechnung bridge: open 3, closed 0
- #7 v0.8 Format compatibility expansion: open 4, closed 0
- #8 v0.9 Non-certification exchange tracks: open 4, closed 0

## Relevant local docs inspected
- docs/testing-strategy.md
- docs/ultraqa-report.md
- gaeb/manifest.toml
- gaeb/README.md

## Constraints
- Deep-interview is requirements mode: no implementation until clarified and handed off.
- Paid official BVBS certification remains gated (#7) unless explicitly authorized.
- Existing protected-main workflow and 95% coverage constraints remain binding for code work.
- Specs/PRDs should avoid overclaiming support for future_track/reference_only fixtures.

## Unknowns / decision-boundary gaps
- Whether to create one spec/PRD per issue, per milestone, or per source family.
- Whether to update existing issues only or also create new issues for newly supplied source URLs not covered by #37-#44.
- Whether specs should include source acquisition/checksum tasks now or only acceptance criteria and architecture/test plans.
- Whether official-source verification should be performed before PRD generation or recorded as a PRD task.
