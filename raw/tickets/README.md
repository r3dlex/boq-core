# Workspace Tickets Index

> **AI SDLC Methodology** — workspace-level ticket index.
> Source-of-truth markers: `<!-- OMX:AI-SDLC:START -->` … `<!-- OMX:AI-SDLC:END -->`.
> Cross-references: `raw/brd/`, `raw/prd/`, `raw/drift-reports/`, `raw/agent-briefs/`, `docs/adr/`.

This is the workspace roll-up of GitHub issues and PRs across both subrepos.
Tickets live in the upstream issue tracker; this index is the join key.

## Format

```
Issue #N — <title> — subrepo:<obra|boq-core> — PRD:<id> — state:<open|closed>
  PR #M — <title> — branch:<branch> — state:<open|merged|closed> — verdict:<PASS|CONCERN|REJECT>
```

## obra

### Modernization 2026 Q3 (rolled-up from `ralplan-obra-modernization-2026-q3.md`)

| Issue/PR | Title | PRD | State | Verdict | Source |
| --- | --- | --- | --- | --- | --- |
| #59 | US-001 precommit-hooks | `OBR-PRD-001` | MERGEABLE GREEN | PASS (0 blocker, 1 major, 3 minor, 2 nit) | `team-plan.md` |
| #60 | US-002 archgate-config | `OBR-PRD-002` | MERGEABLE UNSTABLE | CONCERN (1 blocker, 2 major, 1 minor, 1 nit; YAML not consumed — US-008B scope) | `team-plan.md` |
| #61 | US-003 process-governance | `OBR-PRD-003` | MERGEABLE GREEN | PASS (0 blocker, 2 major, 3 minor, 2 nit; MAJOR DISPATCHED) | `team-plan.md` |
| #62 | US-004 ci-matrix | `OBR-PRD-004` | MERGEABLE UNSTABLE | CONCERN (0 blocker, 2 major, 3 minor, 1 nit; macos+1.17 docker-not-found masked) | `team-plan.md` |
| #63 | US-005 coverage-step-a | `OBR-PRD-005` | MERGEABLE UNSTABLE | CONCERN (1 blocker, 3 major, 1 minor, 1 nit; thresholds DISPATCHED) | `team-plan.md` |
| #64 | US-006 frontend-config | `OBR-PRD-006` | MERGEABLE GREEN (DRAFT) | PASS (0 blocker, 2 major, 4 minor, 2 nit; defer) | `team-plan.md` |
| #65 | US-007 branch-protection | `OBR-PRD-007` | MERGEABLE GREEN | CONCERN (0 blocker, 0 major, 4 minor, 1 nit) | `team-plan.md` |
| #69 | US-008 smoke-tests | `OBR-PRD-008` | MERGEABLE GREEN (HOLD OPEN) | CONCERN (0 blocker, 2 major, 1 minor, 1 nit; AC PARTIAL → INCONCLUSIVE) | `team-plan.md` |
| #70 | US-008B archgate-yaml-loader | `OBR-PRD-008B` | pending | n/a | `ralplan-obra-q3-followups-2026.md` |
| #71 | US-010 coverage-flip | `OBR-PRD-010` | pending | n/a | `ralplan-obra-q3-followups-2026.md` |
| #72 | US-011 frontend-primevue-migration | `OBR-PRD-011` | pending | n/a | `ralplan-obra-q3-followups-2026.md` |

### Docker memory (from `ralplan-docker-memory-2gb.md`)

(infra change, no GitHub issue tracked at the workspace level; PR description references the plan)

## boq-core

### GAEB parser MVP (rolled-up from `prd-boq-core-gaeb-parser-20260606.md` and the ultragoal ledger)

| Issue/PR | Title | PRD | State | Verdict | Source |
| --- | --- | --- | --- | --- | --- |
| #1 / #9 | Quality foundation | `BOQ-PRD-001` §5.0 | MERGED GREEN | Pass | `ultraqa-report.md` |
| #2 / #10 | Domain model + Obra adapter | `BOQ-PRD-001` §5.1 | MERGED GREEN | Pass | `ultraqa-report.md` |
| #3 / #11 | Fixture governance + non-MVP tracks | `BOQ-PRD-001` §5.2 / `BOQ-PRD-003..010` planning | MERGED GREEN | Pass | `ultraqa-report.md` |
| #4 / #12 | GAEB XML 3.3 AVA parser + BVBS conformance | `BOQ-PRD-001` §5.3 | MERGED GREEN | Pass | `ultraqa-report.md` |
| #5 / #12 | GAEB 90 D81/D83 parse-only | `BOQ-PRD-001` §5.3 | MERGED GREEN | Pass | `ultraqa-report.md` |
| #6 / #11 | Future GAEB areas (planning) | `BOQ-PRD-003..010` | MERGED GREEN | Pass | `ultraqa-report.md` |
| #7 | Paid official BVBS certification (gated) | n/a | OPEN (gated) | Hold | `ultraqa-report.md` |
| #8 / TBD | Final UltraQA + release readiness | n/a | pending this report | n/a | `ultraqa-report.md` |

## Source provenance

- obra: `.omc/handoffs/team-plan.md`, `.omc/handoffs/code-reviewer-brief.md`, `.omc/plans/ralplan-obra-modernization-2026-q3.md`, `.omc/plans/ralplan-obra-q3-followups-2026.md`, `.omc/plans/ralplan-docker-memory-2gb.md`, `.omc/handoffs/obra-q3-followups-2026.md`.
- boq-core: `.omx/plans/ultragoal-brief-boq-core-gaeb-parser-20260606.md`, `.omx/ultragoal/goals.json`, `.omx/ultragoal/quality-gate-g007.json`, `.omx/plans/ralplan-handoff-boq-core-gaeb-parser-20260606.json`, `boq-core/docs/ultraqa-report.md`.
