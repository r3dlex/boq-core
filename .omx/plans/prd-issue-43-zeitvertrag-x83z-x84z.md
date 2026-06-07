# PRD: Zeitvertrag X83Z/X84Z framework-contract support planning

## Issue
- GitHub issue: #43
- Milestone: v0.9 Non-certification exchange tracks

## Problem
The current roadmap identifies this source family, but future implementers need a source-by-source support matrix and concrete test obligations before safe parser/model work can begin.

## Product outcome
A future implementation lane can start from this PRD without re-discovering source boundaries, support status, fixture policies, or first-step architecture decisions.

## First architecture decision required
Create a boundary ADR for Z-phase framework-contract handling before changing ordinary X83/X84 behavior.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_zeitvertrag_pkg | official_gaeb | zeitvertrag | 3.3 X83Z/X84Z package | future_track | manifest download with checksum/license note | official schema/sample package | future_zeitvertrag_33_cataloged |
| gaeb32_zeitvertrag_pkg | official_gaeb | zeitvertrag | 3.2 package | future_track | manifest download with checksum/license note | official schema/sample package | future_zeitvertrag_32_cataloged |
| gaeb32_zeitvertrag_examples | official_gaeb | zeitvertrag | 3.2 examples | future_track | manifest download with checksum/license note | official examples; license note required | future_zeitvertrag_32_examples_cataloged |
| schema_x83z_33_chart | interactive_schema | zeitvertrag | 3.3 X83Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x83z_33_schema_chart |
| schema_x84z_33_chart | interactive_schema | zeitvertrag | 3.3 X84Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x84z_33_schema_chart |
| schema_x83z_32_chart | interactive_schema | zeitvertrag | 3.2 X83Z | reference_only | no CI dependency on external HTML | schema chart only | reference_x83z_32_schema_chart |

## Functional requirements
- [ ] Maintain or add fixture manifest entries for each non-documentation source with support_status from the matrix.
- [ ] Add/keep reference-only gates for documentation, beta, executable, commercial, or interactive-only sources.
- [ ] Create failing tests named in the test spec before changing parser/model behavior.
- [ ] Promote support_status only in the same PR as passing implementation tests and review evidence.

## Non-goals and boundaries
- [ ] No paid actions or external certification/payment/submission.
- [ ] No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- [ ] No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Follow-up issue policy
Follow-ups should update issue #43 and its PRD/test-spec. Create a new GitHub issue only when a genuinely new source family or independent implementation track is discovered.

## Acceptance criteria
- [ ] Per-source matrix is preserved in future implementation PRs.
- [ ] The first architecture decision is completed before parser/model code changes.
- [ ] Test-spec concrete tests are created or consciously deferred with rationale.
- [ ] Protected-main gates remain green: fmt, clippy, tests, 95% coverage thresholds, fixture verify, Archgate, prek.

## Goal-mode handoff
Default follow-up: `$ultragoal` for sequential implementation. Use `$team` only if fixture, parser, model, and docs lanes need parallel ownership. `$ralph` remains an explicit fallback only.

## RALPLAN-DR consensus summary

### Principles
- Preserve support-status honesty: `reference_only` and `future_track` are not parser support.
- Prefer source-family boundaries over one overloaded parser/model abstraction.
- Require failing tests and fixture manifest evidence before implementation/promotion.
- Avoid paid, executable, browser, commercial, or license-unclear side effects.
- Keep follow-up issue scope consolidated unless a genuinely independent source family appears.

### Decision drivers
1. **Traceability:** every source must map to a support status, policy, and planned test.
2. **Safety:** no paid/executable/commercial action or unsupported certification claim may occur implicitly.
3. **Implementability:** future `$ultragoal`/`$team` execution needs concrete red/green tests and first architecture decisions.

### Viable options considered
| Option | Pros | Cons | Decision |
|---|---|---|---|
| Per-issue PRDs | Maximum traceability; aligns with user decision; each issue can execute independently. | More artifacts to maintain. | Chosen. |
| Milestone-only PRDs | Lower artifact count; simpler navigation. | Hides source-family differences and weakens per-issue execution readiness. | Rejected. |
| Create many new issues | Highly granular backlog. | Violates no-duplicate issue boundary and fragments existing roadmap. | Rejected unless genuinely new source family appears. |
| Update existing issues #37-#44 | Keeps roadmap cohesive and avoids duplicates. | Requires richer issue bodies. | Chosen. |
| Promote future/reference sources now | Faster apparent progress. | Overclaims support and weakens certification honesty. | Rejected. |

### Architecture alternatives / invalidation rationale
- **Single unified parser expansion:** rejected because GAEB XML compatibility, GAEB 2000 keyword syntax, GAEB 90 fixed-width records, costing, trade, Zeitvertrag, and spreadsheet helpers have distinct data models and risks.
- **Core module for everything:** rejected for #41-#44 until boundary ADRs decide whether companion crate/module/examples-only is safer.
- **Reference-only forever:** rejected as a blanket rule because some future_track sources may be promoted after tests and implementation evidence.

### Risks and mitigations
| Risk | Impact | Mitigation |
|---|---|---|
| Fixture licensing/checksum drift | CI or redistribution risk. | Manifest checksums, license notes, and gated downloads. |
| Accidental support promotion | Users trust unsupported formats. | support_status tests and explicit promotion gates. |
| Parser-family coupling | Brittle architecture and category mistakes. | Boundary ADR before implementation for distinct source families. |
| Duplicate/ambiguous tests | Executors may implement the wrong track. | Namespace test names by source family/module. |
| ADR deferral risk | Implementation starts before boundaries are settled. | First architecture decision is an acceptance gate in every PRD. |
