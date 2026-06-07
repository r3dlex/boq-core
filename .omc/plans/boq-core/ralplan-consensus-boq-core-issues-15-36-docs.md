# Ralplan Consensus Handoff: boq-core issues #15-#36 and documentation MVP

Date: 2026-06-07
Mode: `$ralplan`
Scope: #15-#36 plus planned docs milestone DOC-1..DOC-5.
Out of scope for this batch: #37-#44, implementation code, GitHub issue edits, paid certification, external publishing.

## Planning artifacts

### Intake / draft
- Deep-interview handoff: `.omx/specs/deep-interview-boq-core-missing-plan-specs-docs.md`
- Ralplan draft: `.omx/plans/ralplan-draft-boq-core-issues-15-36-docs.md`

### Individual PRDs generated
- `.omx/plans/prd-issue-15-gaebxmlchecker-ava.md`
- `.omx/plans/prd-issue-16-bvbs-ava-criteria-matrix.md`
- `.omx/plans/prd-issue-17-bvbs-ava-golden-reports.md`
- `.omx/plans/prd-issue-18-paid-bvbs-certification-runbook.md`
- `.omx/plans/prd-issue-19-ava-rich-text-schema-version.md`
- `.omx/plans/prd-issue-20-public-api-crate-docs.md`
- `.omx/plans/prd-issue-21-semver-release-automation.md`
- `.omx/plans/prd-issue-22-fuzz-property-malformed-inputs.md`
- `.omx/plans/prd-issue-23-large-boq-benchmarks.md`
- `.omx/plans/prd-issue-24-gaeb90-windows-1252.md`
- `.omx/plans/prd-issue-25-bauausfuehrung-x83-fixture-promotion.md`
- `.omx/plans/prd-issue-26-bauausfuehrung-x83-parser.md`
- `.omx/plans/prd-issue-27-bauausfuehrung-x84-bid.md`
- `.omx/plans/prd-issue-28-x31-domain-model.md`
- `.omx/plans/prd-issue-29-x31-parser-mvp.md`
- `.omx/plans/prd-issue-30-reb-vb-formula-evaluator.md`
- `.omx/plans/prd-issue-31-x31-x86-baseline-linking.md`
- `.omx/plans/prd-issue-32-texterstellung-rich-text-tables.md`
- `.omx/plans/prd-issue-33-texterstellung-layout-criteria.md`
- `.omx/plans/prd-issue-34-x89-rechnung-model.md`
- `.omx/plans/prd-issue-35-x89-parser-mvp.md`
- `.omx/plans/prd-issue-36-xrechnung-bridge-plan.md`
- `.omx/plans/prd-doc-1-rustdoc-api-reference.md`
- `.omx/plans/prd-doc-2-mdbook-user-guide.md`
- `.omx/plans/prd-doc-3-mdbook-developer-guide.md`
- `.omx/plans/prd-doc-4-certification-evidence-guide.md`
- `.omx/plans/prd-doc-5-release-publishing-guide.md`

## RALPLAN-DR summary

### Principles
- Preserve Obra-compatible hierarchical BoQ output.
- Treat certification artifacts as evidence, not proof of paid/official certification.
- Align source/support statuses with `gaeb/manifest.toml`; do not invent statuses without a governed manifest-vocabulary change.
- Use TDD with red tests and preserve 95% line/function/region coverage gates.
- Generate integrated documentation: rustdoc API/reference plus mdBook manuals/guides.

### Decision drivers
1. Certification-grade traceability for BVBS/GAEB sources and evidence.
2. Safe sequencing across parser, tooling, docs, and release tracks.
3. Public-consumer clarity before crates.io release or broader format claims.

### Chosen options
- Per-issue PRDs for #15-#36.
- Small docs milestone with five focused planned docs issues.
- Execution deferred to `$ultragoal` or `$team` after this planning gate.

### Rejected options
- Milestone-only PRDs: too coarse for TDD and issue execution.
- Folding docs into #20: hides mdBook/manual/publishing scope.
- Implementing docs in ralplan: violates planning-only boundary.

## Required pre-execution validation gate

- Cross-check every PRD source/status anchor against `gaeb/manifest.toml` before `$ultragoal`/`$team` execution.
- Any missing fixture must be added to the manifest before a PRD may call it `future_track`.
- GAEBXmlChecker remains `reference_only` unless a governed manifest-vocabulary change introduces a tooling-specific status.
- Scope execution to #15-#36 + DOC-1..DOC-5; do not include #37-#44 unless explicitly requested.

## Sequencing

1. Evidence/tooling foundation: #15, #16, #17, #18, plus DOC-4 after those PRDs are ready.
2. Public API/parser robustness: #19, #20, #21, #22, #23, #24; DOC-1/DOC-2/DOC-5 after #20/#21 in v0.3.
3. Bauausführung: #25 → #26 → #27.
4. Mengenermittlung/X31: #28 → #29 → #30 and #31 after #29 plus X86 baseline registration.
5. Texterstellung: #32 after #19 rich-text/table contract; #33 criteria matrix can proceed with #32 evidence.
6. Rechnung/XRechnung: #34 → #35; #36 remains roadmap/reference until verified GAEB billing data exists.
7. DOC-3 developer guide after enough architecture is stable to document accurately.

## Consensus gate evidence

- Architect review #1: ITERATE.
  - Artifact: `.omx/artifacts/claude-review-omx-plans-ralplan-draft-boq-core-issues-15-36-docs-md-2026-06-07T16-48-46-318Z.md`
  - Required fixes: GAEBXmlChecker status, X89 status, docs sequencing, dependency annotations, manifest cross-check gate.
- Architect review #2: APPROVE / CLEAR.
  - Artifact: `.omx/artifacts/claude-re-review-omx-plans-ralplan-draft-boq-core-issues-15-36-docs-2026-06-07T16-51-34-827Z.md`
- Critic review #1: APPROVE.
  - Artifact: `.omx/artifacts/claude-critic-review-for-ralplan-consensus-inputs-omx-plans-ralplan-2026-06-07T16-53-33-204Z.md`
- `ralplan_consensus_gate.complete`: true.
- Review order: Architect ITERATE → Architect APPROVE → Critic APPROVE.

## Execution command

Default durable execution:

```bash
$ultragoal .omx/plans/ralplan-consensus-boq-core-issues-15-36-docs.md
```

Parallel execution option for docs + GitHub planning surface + evidence lanes:

```bash
$team .omx/plans/ralplan-consensus-boq-core-issues-15-36-docs.md
```

Recommended first execution outputs:
- Create GitHub docs milestone and DOC-1..DOC-5 issues.
- Update issue bodies #15-#36 with PRD links and guardrails.
- Implement docs MVP only after execution mode is active.
- Keep paid certification, publishing, and external submission gated.
