# PRD: GAEB XML 3.1 and 3.2 compatibility track

## Issue
- GitHub issue: #37
- Milestone: v0.8 Format compatibility expansion

## Problem
The current roadmap identifies this source family, but future implementers need a source-by-source support matrix and concrete test obligations before safe parser/model work can begin.

## Product outcome
A future implementation lane can start from this PRD without re-discovering source boundaries, support status, fixture policies, or first-step architecture decisions.

## First architecture decision required
Add a compatibility ADR documenting XML 3.1/3.2 namespace/schema differences versus the current XML 3.3 AVA parser before parser changes.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb_xml32_doc | official_gaeb | compatibility | 3.2 docs | reference_only | local/manual only | PDF docs; do not assert runtime support | docs_reference_gaeb_xml32 |
| gaeb_xml32_lv_schema | official_gaeb | leistungsverzeichnis | 3.2 X81-X87 schema package | reference_only | manifest download gated by checksum | official schema package; no payload without license check | schema_reference_gaeb_xml32_lv |
| gaeb_xml32_x83_interactive | interactive_schema | bauausfuehrung | 3.2 X83 | reference_only | no CI dependency on external HTML | public interactive chart; documentation only | schema_reference_gaeb_xml32_x83 |
| bvbs_xml32_ava_x81 | bvbs | ava | 3.2 X81 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x81_cataloged |
| bvbs_xml32_ava_x84 | bvbs | ava | 3.2 X84 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x84_cataloged |
| bvbs_xml32_ava_x86 | bvbs | ava | 3.2 X86 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_ava_x86_cataloged |
| bvbs_xml32_bau_x83 | bvbs | bauausfuehrung | 3.2 X83 | future_track | fixture download only with checksum/license note | BVBS certification fixture; no support until tests pass | future_legacy_xml32_bau_x83_cataloged |
| gaeb_xml31_doc | official_gaeb | compatibility | 3.1 docs | reference_only | local/manual only | PDF docs; no runtime support claim | docs_reference_gaeb_xml31 |
| gaeb_xml31_muster | official_gaeb | compatibility | 3.1 2009-12 Musterdateien | future_track | fixture download only with checksum/license note | official examples; no support until tests pass | future_xml31_musterdateien_cataloged |
| gaeb_xml31_x81_x87_schema | official_gaeb | compatibility | 3.1 X81-X83/X85-X87 schemas | reference_only | manifest download gated by checksum | schema package only | schema_reference_gaeb_xml31_x81_x87 |

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
Follow-ups should update issue #37 and its PRD/test-spec. Create a new GitHub issue only when a genuinely new source family or independent implementation track is discovered.

## Acceptance criteria
- [ ] Per-source matrix is preserved in future implementation PRs.
- [ ] The first architecture decision is completed before parser/model code changes.
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
