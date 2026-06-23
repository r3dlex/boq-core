---
phase: PHASE-18
slug: sinapi-catalog-bdi
title: SINAPI catalog and BDI model
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:sinapi"]
status: planning-ready
---

# PHASE-18: SINAPI catalog and BDI model

## Intent
Add a SINAPI-compatible catalog/BDI data model and adapter seams using only synthetic/minimal fixtures.

## Context
This phase belongs to the full Obra coverage roadmap. It is governed by the workspace and is intentionally split from the other phases so implementation can proceed one issue and one PR at a time.

## Scope
- Create or update only the implementation surface needed for this phase.
- Preserve support-status honesty: supported, supported_parse_only, future_track, and reference_only must not be conflated.
- Carry provenance, parser diagnostics, and loss-report findings through any adapter or API boundary touched by this phase.
- Keep the phase independently reviewable and mergeable through its own PR.

## Non-goals
- No paid certification execution.
- No service deployment or production operation.
- No external standards-data acquisition.
- No broad support promotion without fixture evidence and Obra-consumable adapter output.

## Dependencies
PHASE-13

## Required artifacts
- .omc/specs/obra-coverage/PHASE-18-sinapi-catalog-bdi.md
- raw/prd/obra-coverage/PRD-PHASE-18-sinapi-catalog-bdi.md
- .omc/specs/obra-coverage/test-specs/TEST-PHASE-18-sinapi-catalog-bdi.md
- .omc/plans/obra-coverage/PLAN-PHASE-18-sinapi-catalog-bdi.md
- raw/tickets/obra-coverage/ISSUE-PHASE-18-sinapi-catalog-bdi.md

## Acceptance criteria
- Phase artifacts remain in sync with the live GitHub issue.
- Implementation PR references this phase spec, PRD, test spec, plan, and issue.
- Local CI relevant to the owner repository is green before merge.
- GitHub CI for the PR is green before merge.
- Reviewer loop is resolved with architect, reviewer, and executor agreement before auto-approval/merge.
- Any unsupported, parse-only, future, or reference-only input is blocked from being represented as fully import-supported.

## Evidence requirements
- Fixture or contract evidence for all newly claimed behavior.
- Explicit loss/provenance assertions for adapter or service output.
- Updated roadmap/live issue mapping after issue creation or state change.
