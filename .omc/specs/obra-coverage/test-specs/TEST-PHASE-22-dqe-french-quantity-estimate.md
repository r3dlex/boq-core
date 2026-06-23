---
phase: PHASE-22
slug: dqe-french-quantity-estimate
title: DQE French quantity estimate model
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-21"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:dqe"]
status: planning-ready
---

# Test Spec PHASE-22: DQE French quantity estimate model

## Validation strategy
Use the smallest proof that validates this phase while preserving support honesty.

## Required checks
- Unit or contract tests for newly introduced behavior.
- Fixture-based regression tests when a parser/adapter/support-status claim changes.
- Negative tests proving unsupported/parse-only/future/reference-only paths are not import-supported.
- Local CI command recorded in PR evidence.
- GitHub CI verified green before merge.

## Suggested test data
- Existing supported fixtures first.
- Synthetic/minimal fixtures for standards where external data acquisition is excluded.
- Golden outputs only when stable enough to avoid masking support drift.

## Exit evidence
- Command output from owner repo local validation.
- GH checks URL or `gh pr checks` output.
- Reviewer-loop resolution evidence.
