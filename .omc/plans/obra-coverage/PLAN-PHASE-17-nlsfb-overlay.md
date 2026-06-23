---
phase: PHASE-17
slug: nlsfb-overlay
title: NL-SfB overlay
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13", "PHASE-16"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:nlsfb"]
status: planning-ready
---

# Plan PHASE-17: NL-SfB overlay

## Sequence
1. Re-read this phase spec, PRD, test spec, and live GitHub issue.
2. Create a dedicated branch in `r3dlex/boq-core` for this phase only.
3. Implement the minimal phase scope.
4. Add or update tests before claiming support changes.
5. Run local CI and fix failures.
6. Open a PR linked to the phase issue.
7. Resolve reviewer comments until architect/reviewer/executor agreement is explicit.
8. Confirm GH CI green.
9. Auto-approve/merge only if branch policy and admin/self-approval rules permit it and all merge gates are satisfied.
10. Update roadmap/live issue mapping after merge.

## Merge gate
Do not merge until local CI, GH CI, and reviewer loop are all green/resolved.

## Rollback
If phase scope expands, stop and split a follow-up issue instead of broadening the PR.
