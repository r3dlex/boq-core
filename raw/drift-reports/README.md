# Workspace Drift Reports Index

> **AI SDLC Methodology** — workspace-level drift reporting.
> Source-of-truth markers: `<!-- OMX:AI-SDLC:START -->` … `<!-- OMX:AI-SDLC:END -->`.
> Cross-references: `raw/brd/`, `raw/prd/`, `raw/tickets/`, `raw/agent-briefs/`, `docs/adr/`.

A drift report checks a PR diff against the BRD, PRD, acceptance criteria,
relevant ADRs, and `.rules.ts`. Workspace-level drift reports are produced
when a PR touches the workspace binding (setup.sh, .gitmodules, docs/adr/,
raw/, .rules.ts, prek.toml, scripts/, AGENTS.md, CLAUDE.md, README.md,
.github/workflows/).

## Template

```markdown
# DR-XXX — <short title>

- **PR:** <org>/<repo>#N
- **Subrepo:** obra | boq-core | workspace
- **PRD(s):** WS-PRD-XXX, OBR-PRD-XXX, BOQ-PRD-XXX
- **ADRs touched:** WS-XXX, ARCH-XXX
- **Drift category:** BRD gap | PRD gap | ADR gap | rule gap | support-status | certification-boundary
- **Severity:** blocker | major | minor | nit
- **Date:** YYYY-MM-DD

## BRD alignment
<does the PR still match the BRD?>

## PRD alignment
<does the PR still match the PRD's acceptance criteria?>

## ADR alignment
<does the PR still match the ADRs it cites?>

## .rules.ts alignment
<does the PR trigger any rule in .rules.ts?>

## Support-status / certification-boundary check
<does the PR claim support or certification that is not yet earned?>

## Decision
align | iterate | reject
```

## Index

| ID | Subrepo | PR | Drift category | Severity | Decision | Source |
| --- | --- | --- | --- | --- | --- | --- |
| `DR-001` | obra | #60 | ADR gap (archgate YAML not consumed) | blocker | iterate → US-008B | `team-plan.md` |
| `DR-002` | obra | #61 | rule gap (CODEOWNERS auto-assignment) | major | iterate → DISPATCHED | `team-plan.md` |
| `DR-003` | obra | #62 | rule gap (continue-on-error masking) | major | iterate → soft-fail pattern | `team-plan.md` |
| `DR-004` | obra | #63 | rule gap (thresholds not enforced) | blocker | iterate → DISPATCHED | `team-plan.md` |
| `DR-005` | obra | #64 | rule gap (PrimeVue deprecated, execSync brittle) | major | iterate → DRAFT defer | `team-plan.md` |
| `DR-006` | obra | #69 | AC gap (smoke-tests AC PARTIAL) | major | iterate → HOLD OPEN | `team-plan.md` |

## Source provenance

- Drift reports `DR-001..006` are derived from `.omc/handoffs/code-reviewer-brief.md` and `.omc/handoffs/team-plan.md`. Each entry in those documents becomes a drift report when the underlying PR is opened in a workspace context.
- Future drift reports for `boq-core` will be derived from the boq-core review and quality-gate artifacts at `.omx/ultragoal/quality-gate-g007.json` and `boq-core/docs/ultraqa-report.md`.
