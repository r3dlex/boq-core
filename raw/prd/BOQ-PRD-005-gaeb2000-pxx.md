---
id: BOQ-PRD-005
title: GAEB 2000 PXX compatibility (issue #39)
status: pending
date: 2026-06-07
tags: [boq-core, gaeb, gaeb-2000, pxx, compatibility]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
issue_source: .omx/plans/prd-issue-39-gaeb-2000-pxx-compatibility.md
spec_source: .omx/specs/issue-39-gaeb-2000-pxx-compatibility.md
test_spec_source: .omx/plans/test-spec-issue-39-gaeb-2000-pxx-compatibility.md
milestone: v0.8 Format compatibility expansion
tracked_by: r3dlex/boq-core#39 (planning)
subrepo_binding:
  boq-core: boq-core/gaeb/manifest.toml, boq-core/src/gaeb90.rs (parser extension)
  workspace: docs/adr/WS-008-support-status-honesty-and-certification-boundary.md
acceptance_criteria:
  - Per-source matrix is preserved
  - First architecture decision completed before parser changes
  - Test-spec concrete tests created
  - Protected-main gates remain green
---

# BOQ-PRD-005: GAEB 2000 PXX compatibility (issue #39)

Planning-only track. GAEB 2000 PXX is a future_track until the MVP-delivered D81/D83 path can be extended to PXX.
