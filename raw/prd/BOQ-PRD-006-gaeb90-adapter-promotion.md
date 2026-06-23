---
id: BOQ-PRD-006
title: GAEB 90 adapter-compatible promotion (PHASE-10 / issue #102)
status: pending
date: 2026-06-07
tags: [boq-core, gaeb, gaeb-90, adapter, promotion]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
issue_source: .omx/plans/prd-issue-40-gaeb90-adapter-compatible-promotion.md
spec_source: .omx/specs/issue-40-gaeb90-adapter-compatible-promotion.md
test_spec_source: .omx/plans/test-spec-issue-40-gaeb90-adapter-compatible-promotion.md
milestone: v0.8 Format compatibility expansion
tracked_by: r3dlex/boq-core#102 (PHASE-10 delivery; supersedes planning issue #40)
lineage_note: issue_source/spec_source/test_spec_source are historical issue #40 gap-analysis artifacts; PHASE-10 / #102 is the current delivery tracker and narrows promotion to the selected Dangl GAEB 90 D83 fixture path.
subrepo_binding:
  boq-core: boq-core/gaeb/manifest.toml, boq-core/src/gaeb90.rs, boq-core/src/adapter/obra.rs (extension)
  workspace: docs/adr/WS-008-support-status-honesty-and-certification-boundary.md
acceptance_criteria:
  - First architecture decision: gap-analysis ADR lists the data required to promote only the selected Dangl GAEB 90 D83 fixture path to Obra-adapter-compatible
  - Per-source matrix: selected Dangl GAEB 90 D83 fixture path (supported adapter-compatible), D81 and unmanifested/malformed GAEB 90 (parse-only/gated), mwm_rialto_gaeb90_demo (reference_only, commercial demo)
  - support_status promotion only with passing implementation tests, manifest evidence, and review evidence
---

# BOQ-PRD-006: GAEB 90 adapter-compatible promotion (PHASE-10 / issue #102)

Promote only fixture-backed, adapter-compatible GAEB 90 behavior for PHASE-10 / #102. The selected Dangl GAEB 90 D83 fixture path is adapter-compatible because the manifest, parser, and adapter tests preserve hierarchy/provenance and produce deterministic Obra DTO candidates. D81 remains parse-only, malformed or unmanifested GAEB 90 remains adapter-gated, and `mwm_rialto_gaeb90_demo` stays `reference_only`; there is no blanket GAEB 90 promotion.
