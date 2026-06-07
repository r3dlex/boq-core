# Ralplan Draft: boq-core issues #15-#36 and documentation MVP

Date: 2026-06-07
Input: `.omx/specs/deep-interview-boq-core-missing-plan-specs-docs.md`
Mode: `$ralplan` planning-only; no implementation, GitHub writes, paid actions, or docs generation in this phase.

## RALPLAN-DR summary

### Principles
- Preserve Obra-compatible hierarchical BoQ output.
- Treat certification artifacts as evidence, not proof of paid/official certification.
- Keep support status aligned with `gaeb/manifest.toml`; do not invent statuses without an ARCH-002 manifest-vocabulary amendment.
- Use TDD with red tests and preserve 95% line/function/region coverage gates.
- Generate integrated documentation: rustdoc API/reference plus mdBook manuals/guides.

### Decision drivers
1. Certification-grade traceability for BVBS/GAEB sources and evidence.
2. Safe sequencing across parser, tooling, docs, and release tracks.
3. Public-consumer clarity before crates.io release or broader format claims.

### Viable options considered
| Option | Pros | Cons | Decision |
|---|---|---|---|
| Per-issue PRDs/test-specs for #15-#36 | Maximum traceability; matches user request. | More artifacts to maintain. | Chosen. |
| One PRD per milestone | Smaller artifact set. | Too coarse for TDD and issue execution. | Rejected. |
| Fold docs into #20 | Avoids new docs issues. | Hides mdBook/manual/publishing work. | Rejected. |
| Small docs milestone | Clear ownership for API, user, developer, certification, release docs. | Requires later GitHub milestone/issues. | Chosen. |
| Implement docs in ralplan | Faster. | Violates planning-only boundary. | Rejected until execution handoff. |

## Artifact target

Create individual specs, PRDs, and test specs for:
- Existing issues #15-#36.
- Planned docs milestone items DOC-1 through DOC-5.

## Implementation sequencing recommendation

1. Evidence/tooling foundation: #15, #16, #17, #18.
2. Public API/parser robustness: #19, #20, #21, #22, #23, #24.
3. Bauausführung: #25, #26, #27.
4. Mengenermittlung/X31: #28, #29, #30, #31.
5. Texterstellung: #32, #33.
6. Rechnung/XRechnung planning: #34, #35, #36.
7. Documentation MVP sequencing: DOC-4 after #15-#18; DOC-1/DOC-2/DOC-5 after #20/#21 in v0.3; DOC-3 after architecture is stable enough to document.

## Execution handoff guidance

Default: `$ultragoal .omx/plans/ralplan-consensus-boq-core-issues-15-36-docs.md`

Use `$team` for parallel lanes:
- Lane A: GitHub docs milestone/issues and issue-body links.
- Lane B: docs MVP implementation, sequenced early for DOC-1/DOC-2/DOC-4/DOC-5 rather than after v0.7.
- Lane C: selected evidence/tooling foundation (#15-#18).

Do not use `$ralph` unless explicitly choosing a single-owner fallback.

## ADR

### Decision
Use per-issue PRDs/test-specs for #15-#36 and a small documentation milestone with five focused planned docs issues. Implementation waits for `$ultragoal`/`$team` after consensus.

### Drivers
- User asked for all individual PRDs from specs.
- Certification-grade MVP needs traceability from source to test to support status.
- Documentation MVP spans API reference, manuals, certification guidance, and release guidance.

### Alternatives considered
- Milestone-only PRDs: rejected as too coarse.
- Docs folded into #20: rejected by selected docs milestone path.
- Direct implementation during ralplan: rejected by planning-safety boundary.

## Pre-execution validation gate

- Every PRD source/status anchor must be cross-checked against `gaeb/manifest.toml` before `$ultragoal`/`$team` execution.
- Any missing fixture must be added to the manifest before a PRD may call it `future_track`.
- GAEBXmlChecker remains `reference_only` unless a governed manifest-vocabulary change introduces a tooling-specific status.
