# PRD: Kosten und Kalkulation X50-X52 support planning

## Issue
- GitHub issue: #41
- Milestone: v0.9 Non-certification exchange tracks

## Problem
The current roadmap identifies this source family, but future implementers need a source-by-source support matrix and concrete test obligations before safe parser/model work can begin.

## Product outcome
A future implementation lane can start from this PRD without re-discovering source boundaries, support status, fixture policies, or pre-implementation candidate architecture decisions.

## Candidate architecture decision before implementation
ARCH-010 records the boundary decision: X50-X52 remains reference-only planning in this issue; future parser/model promotion may add a dedicated boq-core module only after source-backed fixtures and red/green tests, while estimating workflow or Obra-specific behavior belongs in a companion crate or Obra layer.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb33_kosten_kalkulation_pkg | official_gaeb | kosten_kalkulation | 3.3 X50-X52 package | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | official schema/sample package reference | future_promotion:kosten_kalkulation_33 |
| gaeb32_kalkulation_pkg | official_gaeb | kosten_kalkulation | 3.2 X50-X52 package | reference_only | no CI download; future local vendoring/checksum/license gate required before fixture promotion | official schema/sample package reference | future_promotion:kalkulation_32 |
| schema_x50_33_chart | interactive_schema | kosten_kalkulation | 3.3 X50 | reference_only | no CI dependency on external HTML | schema chart only | reference_x50_33_schema_chart |
| schema_x52_33_chart | interactive_schema | kosten_kalkulation | 3.3 X52 | reference_only | no CI dependency on external HTML | schema chart only | reference_x52_33_schema_chart |
| schema_x52_32_chart | interactive_schema | kosten_kalkulation | 3.2 X52 | reference_only | no CI dependency on external HTML | schema chart only | reference_x52_32_schema_chart |

## Functional requirements
- [x] Maintain or add fixture manifest entries for each non-documentation source with support_status from the matrix.
- [x] Add/keep reference-only gates for documentation, beta, executable, commercial, or interactive-only sources.
- [x] Create boundary/negative tests named in the test spec before changing parser/model behavior; no parser/model behavior is promoted in this issue.
- [x] Preserve reference_only support_status; promotion remains reserved for a future PR with passing implementation tests and review evidence.

## Non-goals and boundaries
- [x] No paid actions or external certification/payment/submission.
- [x] No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- [x] No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Follow-up issue policy
Follow-ups should update issue #41 and its PRD/test-spec. Create a new GitHub issue only when a genuinely new source family or independent implementation track is discovered.

## Acceptance criteria
- [ ] Per-source matrix is preserved in future implementation PRs.
- [ ] Any implementation follow-up records or links the candidate architecture decision before parser/model code changes.
- [ ] Test-spec concrete tests are created or consciously deferred with rationale.
- [ ] Protected-main gates remain green: fmt, clippy, tests, 95% coverage thresholds, fixture verify, Archgate, prek.

## Goal-mode handoff
Default follow-up: `$ultragoal` for sequential implementation. Use `$team` only if fixture, parser, model, and docs lanes need parallel ownership. `$ralph` remains an explicit fallback only.

## RALPLAN-DR consensus summary

> The following decisions were reached collectively for issues #37-#44 and are reproduced here for standalone executability. Cross-issue scope: see `ralplan-consensus-gaeb-next-step-specs.md` for the canonical source and the recommended implementation sequencing appendix.

### Principles
- Preserve support-status honesty: `reference_only` and `future_track` are not parser support.
- Prefer source-family boundaries over one overloaded parser/model abstraction.
- Require failing tests and fixture manifest evidence before implementation/promotion.
- Avoid paid, executable, browser, commercial, or license-unclear side effects.
- Keep follow-up issue scope consolidated unless a genuinely independent source family appears.

### Decision drivers
1. **Traceability:** every source must map to a support status, policy, and planned test.
2. **Safety:** no paid/executable/commercial action or unsupported certification claim may occur implicitly.
3. **Implementability:** future `$ultragoal`/`$team` execution needs concrete red/green tests and pre-implementation candidate architecture decisions.

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
| ADR deferral risk | Implementation starts before boundaries are settled. | Candidate architecture decision is a pre-implementation gate in every PRD. |

## Ranked roadmap source inventory binding

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #41 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R6-01 | #41 Kosten/Kalkulation X50-X52 | manifested | official_gaeb_xml33_kosten_und_kalkulation | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R6-02 | #41 Kosten/Kalkulation X50-X52 | manifested | official_gaeb_xml32_kalkulation | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R6-03 | #41 Kosten/Kalkulation X50-X52 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R6-04 | #41 Kosten/Kalkulation X50-X52 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R6-05 | #41 Kosten/Kalkulation X50-X52 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.


## Issue #41 delivery notes
- Boundary ADR: `.archgate/adrs/ARCH-010-kosten-kalkulation-boundary.md`.
- Boundary matrix: `docs/fixtures/kosten-kalkulation-x50-x52-boundary.md`.
- Tests: `tests/kosten_kalkulation_boundary.rs` locks official source rows, interactive chart reference-only policy, future cost-component obligations, X52 item-reference obligations, and support-policy `ReferenceOnly` behavior.
- Support status: no X50/X51/X52 parser or adapter support is promoted; official rows remain `reference_only`.
