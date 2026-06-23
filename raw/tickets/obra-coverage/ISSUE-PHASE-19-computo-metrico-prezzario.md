---
phase: PHASE-19
slug: computo-metrico-prezzario
title: Computo Metrico and Prezzario model
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-18"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:prezzario"]
status: planning-ready
---

# [Obra coverage PHASE-19] Computo Metrico and Prezzario model

## Goal
Add Italian quantity/price-list model seams and loss reporting without external regional data acquisition.

## Artifact packet
- Spec: `.omc/specs/obra-coverage/PHASE-19-computo-metrico-prezzario.md`
- PRD: `raw/prd/obra-coverage/PRD-PHASE-19-computo-metrico-prezzario.md`
- Test spec: `.omc/specs/obra-coverage/test-specs/TEST-PHASE-19-computo-metrico-prezzario.md`
- Plan: `.omc/plans/obra-coverage/PLAN-PHASE-19-computo-metrico-prezzario.md`

## Dependencies
PHASE-13, PHASE-18

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
track:obra-coverage, status:planning-ready, support-honesty, needs-local-ci, needs-gh-ci, needs-review-loop, area:boq-core, track:multi-standard, standard:prezzario
