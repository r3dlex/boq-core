# Workspace Product Requirements Documents (PRDs)

> **AI SDLC Methodology** — workspace-level PRD layer.
> Source-of-truth markers: `<!-- OMX:AI-SDLC:START -->` … `<!-- OMX:AI-SDLC:END -->`.
> Cross-references: `raw/brd/`, `raw/tickets/`, `raw/drift-reports/`, `raw/agent-briefs/`, `docs/adr/`.

This directory contains workspace-level PRDs. A PRD translates a BRD into
user stories, acceptance criteria, implementation decisions, testing
decisions, and out-of-scope items. Subrepo PRDs (`OBR-PRD-*` for obra,
`BOQ-PRD-*` for boq-core) live in the same directory; the workspace is the
single read-only view of all PRDs.

## Convention

- `WS-PRD-XXX-*.md` — workspace-level PRD, binding ≥ 2 subrepos.
- `OBR-PRD-XXX-*.md` — obra subrepo PRD, mirrors the Q3 plan and follow-ups.
- `BOQ-PRD-XXX-*.md` — boq-core subrepo PRD, mirrors the GAEB parser PRD and the issue-level PRDs.

Every PRD frontmatter must include:

- `id`, `title`, `status`, `date`
- `brd_link`: path to the BRD or "n/a — subrepo owns the BRD"
- `subrepo_binding`: list of subrepo paths and their governing ADRs
- `tracked_by`: a GitHub issue number or "pending"
- `acceptance_criteria`: a checklist with concrete commands

## Index

### Workspace-level PRDs

(none yet — workspace PRDs are produced by following `OBR-PRD-*` or `BOQ-PRD-*` linkages)

### obra subrepo PRDs (`OBR-PRD-*`)

- `OBR-PRD-001` — precommit hooks (lefthook)
- `OBR-PRD-002` — archgate config (YAML → rules)
- `OBR-PRD-003` — process governance (CODEOWNERS, PR template, Dependabot)
- `OBR-PRD-004` — CI matrix
- `OBR-PRD-005` — coverage 90% (Step A: soft-fail dry-run)
- `OBR-PRD-006` — frontend PrimeVue + Tailwind (DRAFT)
- `OBR-PRD-007` — branch protection
- `OBR-PRD-008` — smoke tests
- `OBR-PRD-008B` — archgate YAML loader (US-008B)
- `OBR-PRD-009` — Docker memory optimization (2 GB)
- `OBR-PRD-010` — coverage flip per stack (US-010)
- `OBR-PRD-011` — frontend PrimeVue migration rollout (US-011)

### boq-core subrepo PRDs (`BOQ-PRD-*`)

- `BOQ-PRD-001` — GAEB parser and certification harness (master PRD)
- `BOQ-PRD-002` — test spec for `BOQ-PRD-001`
- `BOQ-PRD-003` — GAEB XML 3.1/3.2 compatibility (issue #37)
- `BOQ-PRD-004` — GAEB XML 3.4 beta tracking (issue #38)
- `BOQ-PRD-005` — GAEB 2000 PXX compatibility (issue #39)
- `BOQ-PRD-006` — GAEB 90 adapter-compatible promotion (issue #40)
- `BOQ-PRD-007` — Kosten/Kalkulation X50/X52 (issue #41)
- `BOQ-PRD-008` — Handel X93/X97 (issue #42)
- `BOQ-PRD-009` — Zeitvertrag X83Z/X84Z (issue #43)
- `BOQ-PRD-010` — Spreadsheet roundtrip (issue #44)

## Source provenance

- `OBR-PRD-*` are extracted from `.omc/plans/ralplan-obra-modernization-2026-q3.md` (US-001..008), `.omc/plans/ralplan-obra-q3-followups-2026.md` (US-008B/010/011), and `.omc/plans/ralplan-docker-memory-2gb.md`.
- `BOQ-PRD-*` are extracted from `.omx/plans/prd-boq-core-gaeb-parser-20260606.md` (master), `.omx/plans/prd-issue-37..44.md` (issue-level), and `.omx/plans/test-spec-issue-37..44.md` (test specs).

## Subrepo binding

- obra: `obra/.archgate/adrs/ARCH-001..005.md`.
- boq-core: `boq-core/.archgate/adrs/ARCH-001..004.md`.
- Workspace: `docs/adr/WS-001..008.md`.
