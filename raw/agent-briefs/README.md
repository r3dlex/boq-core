# Workspace Agent Briefs Index

> **AI SDLC Methodology** — workspace-level agent briefs.
> Source-of-truth markers: `<!-- OMX:AI-SDLC:START -->` … `<!-- OMX:AI-SDLC:END -->`.
> Cross-references: `raw/brd/`, `raw/prd/`, `raw/tickets/`, `raw/drift-reports/`, `docs/adr/`.

An agent brief is a short, single-task handoff to a subagent (architect,
reviewer, executor, debugger, etc.). It must reference the ticket, PRD,
relevant ADRs, Archgate rules, and verification commands.

## Template

```markdown
# BR-XXX — <role> brief for <ticket>

- **Role:** architect | reviewer | executor | debugger
- **Subrepo:** obra | boq-core | workspace
- **Ticket:** <org>/<repo>#N
- **PRD:** WS-PRD-XXX | OBR-PRD-XXX | BOQ-PRD-XXX
- **ADRs:** WS-XXX, ARCH-XXX
- **Time-box:** ~5 min (review) / ½ day (executor) / per-architect
- **Date:** YYYY-MM-DD

## Read first
- PRD path
- ADR paths
- .rules.ts domains

## For each item
- <concrete instruction>

## Output format
- <PASS | CONCERN | REJECT> for architect/reviewer
- <DONE | BLOCKED | IN-FLIGHT> for executor
- 1-paragraph summary

## Constraints
- <scope limits>
- <verification commands>
```

## Index

| ID | Role | Subrepo | Ticket | Source |
| --- | --- | --- | --- | --- |
| `BR-001` | team-plan (multi-role) | obra | rolled-up Q3 PRs | `.omc/handoffs/team-plan.md` |
| `BR-002` | code-reviewer (opus) | obra | #59–#69 | `.omc/handoffs/code-reviewer-brief.md` |
| `BR-003` | team-exec (obra-q3-followups) | obra | #70, #71, #72 | `obra/.omc/handoffs/obra-q3-followups-2026.md` |
| `BR-004` | ultragoal (boq-core) | boq-core | #1–#7 | `.omx/plans/ultragoal-brief-boq-core-gaeb-parser-20260606.md` |
| `BR-005` | ralplan-handoff (boq-core) | boq-core | #1–#7 | `.omx/plans/ralplan-handoff-boq-core-gaeb-parser-20260606.json` |

## Source provenance

- `BR-001..003` are extracted from obra's existing handoffs in `.omc/handoffs/` and `obra/.omc/handoffs/`.
- `BR-004..005` are extracted from boq-core's ultragoal and ralplan handoff at `.omx/plans/`.
