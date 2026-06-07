---
id: BOQ-PRD-009
title: Zeitvertrag X83Z/X84Z framework-contract support (issue #43)
status: pending
date: 2026-06-07
tags: [boq-core, gaeb, zeitvertrag, x83z, x84z, framework-contract]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
issue_source: .omx/plans/prd-issue-43-zeitvertrag-x83z-x84z.md
spec_source: .omx/specs/issue-43-zeitvertrag-x83z-x84z.md
test_spec_source: .omx/plans/test-spec-issue-43-zeitvertrag-x83z-x84z.md
milestone: v0.9 Non-certification exchange tracks
tracked_by: r3dlex/boq-core#43 (planning)
subrepo_binding:
  boq-core: boq-core/gaeb/manifest.toml, boq-core/src/model.rs (Z-phase framework-contract handling)
  workspace: docs/adr/WS-008-support-status-honesty-and-certification-boundary.md
acceptance_criteria:
  - First architecture decision: boundary ADR for Z-phase framework-contract handling before changing ordinary X83/X84 behavior
  - Per-source matrix: gaeb33_zeitvertrag_pkg (future_track), gaeb32_zeitvertrag_pkg (future_track), gaeb32_zeitvertrag_examples (future_track), schema_x83z_33/32 charts (reference_only)
---

# BOQ-PRD-009: Zeitvertrag X83Z/X84Z (issue #43)

Planning-only track. Z-phase framework-contract handling is a separate concern from ordinary X83/X84; the boundary ADR is the prerequisite.
