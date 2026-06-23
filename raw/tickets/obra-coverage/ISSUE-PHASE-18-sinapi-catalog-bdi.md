---
phase: PHASE-18
slug: sinapi-catalog-bdi
title: SINAPI catalog and BDI model
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:sinapi"]
status: planning-ready
---

# [Obra coverage PHASE-18] SINAPI catalog and BDI model

## Goal
Add a SINAPI-compatible catalog/BDI data model and adapter seams using only synthetic/minimal fixtures.

## Artifact packet
- Spec: `.omc/specs/obra-coverage/PHASE-18-sinapi-catalog-bdi.md`
- PRD: `raw/prd/obra-coverage/PRD-PHASE-18-sinapi-catalog-bdi.md`
- Test spec: `.omc/specs/obra-coverage/test-specs/TEST-PHASE-18-sinapi-catalog-bdi.md`
- Plan: `.omc/plans/obra-coverage/PLAN-PHASE-18-sinapi-catalog-bdi.md`

## Dependencies
PHASE-13

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
track:obra-coverage, status:planning-ready, support-honesty, needs-local-ci, needs-gh-ci, needs-review-loop, area:boq-core, track:multi-standard, standard:sinapi
