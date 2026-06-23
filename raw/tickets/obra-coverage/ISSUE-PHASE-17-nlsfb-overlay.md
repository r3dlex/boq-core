---
phase: PHASE-17
slug: nlsfb-overlay
title: NL-SfB overlay
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-16"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:nlsfb"]
status: planning-ready
---

# [Obra coverage PHASE-17] NL-SfB overlay

## Goal
Add NL-SfB overlay as classification mapping foundation for Netherlands/Belgium.

## Artifact packet
- Spec: `.omc/specs/obra-coverage/PHASE-17-nlsfb-overlay.md`
- PRD: `raw/prd/obra-coverage/PRD-PHASE-17-nlsfb-overlay.md`
- Test spec: `.omc/specs/obra-coverage/test-specs/TEST-PHASE-17-nlsfb-overlay.md`
- Plan: `.omc/plans/obra-coverage/PLAN-PHASE-17-nlsfb-overlay.md`

## Dependencies
PHASE-13, PHASE-16

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
track:obra-coverage, status:planning-ready, support-honesty, needs-local-ci, needs-gh-ci, needs-review-loop, area:boq-core, track:multi-standard, standard:nlsfb
