---
id: BOQ-PRD-004
title: GAEB XML 3.4 beta schema and changelog impact tracking (issue #38)
status: pending
date: 2026-06-07
tags: [boq-core, gaeb, xml-3-4, beta, tracking]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
issue_source: .omx/plans/prd-issue-38-gaeb-xml-34-beta-tracking.md
spec_source: .omx/specs/issue-38-gaeb-xml-34-beta-tracking.md
test_spec_source: .omx/plans/test-spec-issue-38-gaeb-xml-34-beta-tracking.md
milestone: v0.8 Format compatibility expansion
tracked_by: r3dlex/boq-core#38 (planning)
subrepo_binding:
  boq-core: boq-core/gaeb/manifest.toml (entries to add), boq-core/src/model.rs (extension points for sustainability/lifecycle/carbon)
  workspace: docs/adr/WS-008-support-status-honesty-and-certification-boundary.md
acceptance_criteria:
  - First architecture decision: beta-impact ADR records 3.4 as reference-only and identifies sustainability/lifecycle/carbon descriptor model extension points
  - Per-source matrix is preserved: gaeb_xml34_beta_schema (reference_only), gaeb_xml34_beta_changelog (reference_only)
  - No BVBS fixtures for 3.4 beta
  - Diff/compatibility tests optional until stable
---

# BOQ-PRD-004: GAEB XML 3.4 beta tracking (issue #38)

Reference-only forward-compatibility analysis. Sustainability/lifecycle/carbon descriptors are extension points to be wired when 3.4 stabilizes.
