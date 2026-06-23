---
phase: PHASE-23
slug: spreadsheet-neutral-roundtrip
title: Spreadsheet neutral roundtrip
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-18", "PHASE-19", "PHASE-20", "PHASE-21", "PHASE-22"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:spreadsheet"]
status: planning-ready
---

# [Obra coverage PHASE-23] Spreadsheet neutral roundtrip

## Goal
Provide governed, dependency-free neutral CSV exchange for spreadsheet workflows as a practical cross-market bridge with provenance and loss reports. XLSX/ODS readers, writers, binary fixtures, and spreadsheet dependencies remain out of scope.

## Artifact packet
- Spec: `.omc/specs/obra-coverage/PHASE-23-spreadsheet-neutral-roundtrip.md`
- PRD: `raw/prd/obra-coverage/PRD-PHASE-23-spreadsheet-neutral-roundtrip.md`
- Test spec: `.omc/specs/obra-coverage/test-specs/TEST-PHASE-23-spreadsheet-neutral-roundtrip.md`
- Plan: `.omc/plans/obra-coverage/PLAN-PHASE-23-spreadsheet-neutral-roundtrip.md`

## Dependencies
PHASE-13, PHASE-18, PHASE-19, PHASE-20, PHASE-21, PHASE-22

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
track:obra-coverage, status:planning-ready, support-honesty, needs-local-ci, needs-gh-ci, needs-review-loop, area:boq-core, track:multi-standard, standard:spreadsheet
