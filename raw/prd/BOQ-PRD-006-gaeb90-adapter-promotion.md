---
id: BOQ-PRD-006
title: GAEB 90 adapter-compatible promotion (issue #40)
status: pending
date: 2026-06-07
tags: [boq-core, gaeb, gaeb-90, adapter, promotion]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
issue_source: .omx/plans/prd-issue-40-gaeb90-adapter-compatible-promotion.md
spec_source: .omx/specs/issue-40-gaeb90-adapter-compatible-promotion.md
test_spec_source: .omx/plans/test-spec-issue-40-gaeb90-adapter-compatible-promotion.md
milestone: v0.8 Format compatibility expansion
tracked_by: r3dlex/boq-core#40 (planning)
subrepo_binding:
  boq-core: boq-core/gaeb/manifest.toml, boq-core/src/gaeb90.rs, boq-core/src/adapter/obra.rs (extension)
  workspace: docs/adr/WS-008-support-status-honesty-and-certification-boundary.md
acceptance_criteria:
  - First architecture decision: gap-analysis ADR listing exact data required to promote GAEB 90 D81/D83 from parse-only to Obra-adapter-compatible
  - Per-source matrix: dangl_ava_gaeb90_examples (future_track, developer examples), mwm_rialto_gaeb90_demo (reference_only, commercial demo)
  - support_status promotion only with passing implementation tests and review evidence
---

# BOQ-PRD-006: GAEB 90 adapter-compatible promotion (issue #40)

Promote the MVP-delivered parse-only D81/D83 support to Obra-adapter-compatible. The first architecture decision is a gap-analysis ADR naming the exact data required for promotion.
