---
phase: PHASE-16
slug: uniclass-overlay
title: Uniclass overlay
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-15"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:uniclass"]
status: planning-ready
---

# [Obra coverage PHASE-16] Uniclass overlay

## Goal
Add Uniclass overlay for UK/international classification without external catalog acquisition.

## Artifact packet
- Spec: `.omc/specs/obra-coverage/PHASE-16-uniclass-overlay.md`
- PRD: `raw/prd/obra-coverage/PRD-PHASE-16-uniclass-overlay.md`
- Test spec: `.omc/specs/obra-coverage/test-specs/TEST-PHASE-16-uniclass-overlay.md`
- Plan: `.omc/plans/obra-coverage/PLAN-PHASE-16-uniclass-overlay.md`

## Dependencies
PHASE-13, PHASE-15

## Acceptance criteria
- Artifacts above stay in sync with this issue.
- One implementation PR is opened for this issue unless a split is explicitly documented.
- Local CI is green and evidence is posted before merge.
- GH CI is green before merge.
- Reviewer loop has no unresolved actionable comments before merge.
- Support status and loss/provenance contracts remain honest.

## Non-goals
- Paid certification execution
- Service deployment
- External data acquisition
- Unsupported standards promotion without evidence

## Labels
track:obra-coverage, status:planning-ready, support-honesty, needs-local-ci, needs-gh-ci, needs-review-loop, area:boq-core, track:multi-standard, standard:uniclass
