---
phase: PHASE-22
slug: dqe-french-quantity-estimate
title: DQE French quantity estimate model
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-21"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:dqe"]
status: planning-ready
---

# [Obra coverage PHASE-22] DQE French quantity estimate model

## Goal
Define DQE quantity-estimate model and adapter contract with support honesty.

## Artifact packet
- Spec: `.omc/specs/obra-coverage/PHASE-22-dqe-french-quantity-estimate.md`
- PRD: `raw/prd/obra-coverage/PRD-PHASE-22-dqe-french-quantity-estimate.md`
- Test spec: `.omc/specs/obra-coverage/test-specs/TEST-PHASE-22-dqe-french-quantity-estimate.md`
- Plan: `.omc/plans/obra-coverage/PLAN-PHASE-22-dqe-french-quantity-estimate.md`

## Dependencies
PHASE-13, PHASE-21

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
track:obra-coverage, status:planning-ready, support-honesty, needs-local-ci, needs-gh-ci, needs-review-loop, area:boq-core, track:multi-standard, standard:dqe
