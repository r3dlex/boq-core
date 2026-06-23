---
phase: PHASE-23
slug: spreadsheet-neutral-roundtrip
title: Spreadsheet neutral roundtrip
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-18", "PHASE-19", "PHASE-20", "PHASE-21", "PHASE-22"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:spreadsheet"]
status: planning-ready
---

# PHASE-23: Spreadsheet neutral roundtrip

## Intent
Provide governed, dependency-free neutral CSV exchange for spreadsheet workflows as a practical cross-market bridge with provenance and loss reports. XLSX/ODS readers, writers, binary fixtures, and spreadsheet dependencies remain out of scope.

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
PHASE-13, PHASE-18, PHASE-19, PHASE-20, PHASE-21, PHASE-22

## Required artifacts
- .omc/specs/obra-coverage/PHASE-23-spreadsheet-neutral-roundtrip.md
- raw/prd/obra-coverage/PRD-PHASE-23-spreadsheet-neutral-roundtrip.md
- .omc/specs/obra-coverage/test-specs/TEST-PHASE-23-spreadsheet-neutral-roundtrip.md
- .omc/plans/obra-coverage/PLAN-PHASE-23-spreadsheet-neutral-roundtrip.md
- raw/tickets/obra-coverage/ISSUE-PHASE-23-spreadsheet-neutral-roundtrip.md

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
