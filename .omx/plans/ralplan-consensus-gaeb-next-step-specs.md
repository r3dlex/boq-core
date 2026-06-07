# Ralplan Consensus Handoff: GAEB next-step specs and PRDs

## Planning artifacts
- Deep-interview spec: `.omx/specs/deep-interview-gaeb-next-step-specs.md`
- Affected issues: #37-#44
- Per-issue specs: `.omx/specs/issue-37-*.md` through `.omx/specs/issue-44-*.md`
- Per-issue PRDs: `.omx/plans/prd-issue-37-*.md` through `.omx/plans/prd-issue-44-*.md`
- Per-issue test specs: `.omx/plans/test-spec-issue-37-*.md` through `.omx/plans/test-spec-issue-44-*.md`

## RALPLAN-DR summary

### Principles
- Preserve support-status honesty.
- Keep source-family parser/model boundaries explicit.
- Require failing tests and fixture evidence before support promotion.
- Avoid paid, executable, browser, commercial, or license-unclear side effects.
- Avoid duplicate issue explosion.

### Decision drivers
1. Per-issue traceability.
2. Safety and non-overclaiming.
3. Future implementability through concrete red/green tests.

### Options and decisions
- Chosen: PRDs/test-specs per affected issue #37-#44.
- Chosen: update existing issues rather than create duplicates.
- Chosen: future_track/reference_only source statuses until promotion evidence exists.
- Rejected: milestone-only PRDs because user selected per-issue PRDs.
- Rejected: one unified parser/model expansion because source families differ materially.
- Rejected: executing/downloading commercial/executable/browser tools in CI.

## Consensus gate state
- Architect review #1: ITERATE.
- Architect review #2: APPROVE.
- Critic review #1: ITERATE.
- Critic review #2: APPROVE.
- ralplan_consensus_gate.complete: true.
- Consensus order: Architect ITERATE -> Architect APPROVE -> Critic ITERATE -> Architect APPROVE -> Critic APPROVE.


## Final Architect review
APPROVE: RALPLAN-DR summaries, source matrices, concrete test plans, unique namespaced tests, and consensus handoff are present.

## Final Critic review
APPROVE: per-issue artifacts complete; alternatives, risks, support honesty, verification, and no-duplicate policy are sufficient for execution handoff.

---

## Recommended implementation sequencing (post-consensus addendum, 2026-06-07)

> **Source:** Final quality gate architect WATCH finding (architectural sequencing of the 8 boundary ADRs to avoid cross-PRD ADR conflicts and dependency pollution).

The 8 issues are not strictly independent. The boundary ADRs gate on a sequencing order that the original ralplan consensus did not state. Recommended execution order:

1. **#37** — GAEB XML 3.1/3.2 compatibility (XML version parser boundary)
2. **#38** — GAEB XML 3.4 beta tracking (tracking-only, no new ADR)
3. **#40** — GAEB 90 adapter-compatible promotion (anchors the companion-crate vs core decision pattern)
4. **#39** — GAEB 2000 / Pxx parser compatibility (distinct keyword-syntax boundary, informed by #40)
5. **#41, #42, #43** — Kosten/Kalkulation, Handel, Zeitvertrag (companion-crate cluster, ADRs aligned with #40)
6. **#44** — Spreadsheet roundtrip (last, highest boundary risk; module TBD)

#37 and #38 can run in parallel safely. #40 must precede #41-#43. #44 must run last.
