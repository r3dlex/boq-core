---
phase: PHASE-21
slug: stabu-raw-exchange
title: STABU / RAW exchange model
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-17"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:stabu-raw"]
status: planning-ready
---

# [Obra coverage PHASE-21] STABU / RAW exchange model

## Goal
Define Dutch STABU/RAW exchange model boundaries and synthetic adapter contracts.

## Artifact packet
- Spec: `.omc/specs/obra-coverage/PHASE-21-stabu-raw-exchange.md`
- PRD: `raw/prd/obra-coverage/PRD-PHASE-21-stabu-raw-exchange.md`
- Test spec: `.omc/specs/obra-coverage/test-specs/TEST-PHASE-21-stabu-raw-exchange.md`
- Plan: `.omc/plans/obra-coverage/PLAN-PHASE-21-stabu-raw-exchange.md`

## Dependencies
PHASE-13, PHASE-17

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
track:obra-coverage, status:planning-ready, support-honesty, needs-local-ci, needs-gh-ci, needs-review-loop, area:boq-core, track:multi-standard, standard:stabu-raw
