# Workspace Business Requirements Documents (BRDs)

> **AI SDLC Methodology** — workspace-level BRD layer.
> Source-of-truth markers: `<!-- OMX:AI-SDLC:START -->` … `<!-- OMX:AI-SDLC:END -->`.
> Cross-references: `raw/prd/`, `raw/tickets/`, `raw/drift-reports/`, `raw/agent-briefs/`, `docs/adr/`.

This directory contains workspace-level BRDs. A BRD captures the business
problem, target users, desired outcomes, constraints, and risks. PRDs at
`raw/prd/` translate each BRD into user stories, acceptance criteria, and
implementation decisions. Tickets at `raw/tickets/` slice the PRDs into
tracer-bullet implementation tasks.

## Chain

```
BRD  (raw/brd/WS-BRD-XXX.md)
 └─ PRD  (raw/prd/WS-PRD-XXX.md  OR  subrepo raw/prd/OBR-PRD-XXX.md / BOQ-PRD-XXX.md)
      └─ Ticket  (raw/tickets/INDEX.md → GitHub issue #N)
            └─ Agent brief  (raw/agent-briefs/BR-XXX.md)
                  └─ Drift report  (raw/drift-reports/DR-XXX.md) ↔ PR
```

## Index

| BRD ID | Title | Linked PRD(s) | Subrepo binding | Tracked by |
| --- | --- | --- | --- | --- |
| `WS-BRD-001` | [obra Modernization 2026 Q3](./WS-BRD-001-obra-modernization-2026-q3.md) | `OBR-PRD-001..008`, `OBR-PRD-008B`, `OBR-PRD-010`, `OBR-PRD-011` | obra | rolled-up GitHub issue (pending) |
| `WS-BRD-002` | [obra Docker memory optimization (2 GB)](./WS-BRD-002-obra-docker-memory-2gb.md) | `OBR-PRD-009` | obra | (no GitHub issue — infra PR) |
| `WS-BRD-003` | [boq-core GAEB parser and certification harness](./WS-BRD-003-boq-core-gaeb-parser.md) | `BOQ-PRD-001..010` | boq-core | issues #1–#7 (closed), #8 (UltraQA) |

## Source provenance

- `WS-BRD-001` is derived from `.omc/plans/ralplan-obra-modernization-2026-q3.md` (v2 consensus) and `.omc/plans/ralplan-obra-q3-followups-2026.md` (iter 2/5 consensus).
- `WS-BRD-002` is derived from `.omc/plans/ralplan-docker-memory-2gb.md` (ralplan iter 2 consensus).
- `WS-BRD-003` is derived from `.omx/plans/prd-boq-core-gaeb-parser-20260606.md` and the issue-level PRDs at `.omx/plans/prd-issue-37..44.md`.

A workspace PR that re-extracts a BRD must reference the source plan; the
source of truth for engineering detail remains the subrepo plan/spec/PRD.

## Subrepo binding

- obra: `obra/.archgate/adrs/ARCH-001..005.md`.
- boq-core: `boq-core/.archgate/adrs/ARCH-001..004.md`.
- Workspace: `docs/adr/WS-001..008.md`.
