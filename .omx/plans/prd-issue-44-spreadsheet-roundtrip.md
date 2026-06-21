# PRD: Spreadsheet roundtrip helper planning

## Issue
- GitHub issue: #44
- Milestone: v0.9 Non-certification exchange tracks

## Problem
The current roadmap identifies this source family, but future implementers need a source-by-source support matrix and concrete test obligations before safe parser/model work can begin.

## Product outcome
A future implementation lane can start from this PRD without re-discovering source boundaries, support status, fixture policies, or pre-implementation candidate architecture decisions.

## Candidate architecture decision before implementation
ARCH-013 records the boundary decision: spreadsheet roundtrip helpers remain reference-only examples/companion-crate planning in this issue; no spreadsheet dependency, executable run, parser support, export support, or roundtrip capability is added.

## Per-source support matrix
| Fixture/source id | Source family | Domain | Version/phase | support_status | CI/download policy | License/executable policy | Test mapping |
|---|---|---|---|---|---|---|---|
| gaeb_online_import_template | spreadsheet_template | spreadsheet_roundtrip | Excel import template | reference_only | download manually only; checksum/license note; no parser support claim | spreadsheet template reference | reference_gaeb_online_import_template |
| gaeb_online_excel_generator | executable_tool | spreadsheet_roundtrip | Excel generator .exe | reference_only | do not download/execute in CI | executable; reference only | reference_gaeb_online_excel_generator |
| mwm_rialto_demo | commercial_demo | spreadsheet_roundtrip | Excel conversion demo | reference_only | do not download/execute in CI | commercial/demo utility; reference only | reference_mwm_rialto_demo |
| easy_gaeb_browser | browser_utility | spreadsheet_roundtrip | Browser utility | reference_only | no CI dependency; no scraping/execution | external web utility; reference only | reference_easy_gaeb_browser |

## Functional requirements
- [x] Maintain fixture manifest/reference entries for each non-documentation source with support_status from the matrix.
- [x] Add/keep reference-only gates for documentation, executable, commercial, browser, or interactive-only sources.
- [x] Create boundary/negative tests named in the test spec before changing parser/model behavior; no parser/model behavior is promoted in this issue.
- [x] Preserve reference_only support_status; promotion remains reserved for a future PR with passing implementation tests and review evidence.

## Non-goals and boundaries
- [x] No paid actions or external certification/payment/submission.
- [x] No support overclaiming: support_status promotion requires failing tests, implementation, fixture verification, and review evidence.
- [x] No duplicate issue explosion: update this issue unless a genuinely missing source family requires a new issue.

## Follow-up issue policy
Follow-ups should update issue #44 and its PRD/test-spec. Create a new GitHub issue only when a genuinely new source family or independent implementation track is discovered.

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

This PRD is bound to the canonical ranked roadmap ledger in `.omx/specs/gaeb-ranked-source-status-ledger.md`. Issue #44 owns the following source rows for planning and test-readiness purposes:

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| A2-01 | #44 Spreadsheet roundtrip | manifested | gaeb_online_import_template | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-02 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| A2-03 | #44 Spreadsheet roundtrip | manifested | gaeb_online_generator_exe | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-04 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| A2-05 | #44 Spreadsheet roundtrip | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

Constraints: preserve PRD intent, avoid duplicate issue creation, avoid paid certification actions, and treat non-manifested rows as future safe-fixture or reference-only gates until explicitly promoted in the manifest and test plan.


## Issue #44 delivery notes
- Boundary ADR: `.archgate/adrs/ARCH-013-spreadsheet-roundtrip-boundary.md`.
- Boundary matrix: `docs/fixtures/spreadsheet-roundtrip-boundary.md`.
- Tests: `tests/spreadsheet_roundtrip_boundary.rs` locks reference-only/non-executed spreadsheet sources, no spreadsheet dependency, OZ matching with reordered/inserted columns, missing-OZ rejection, and artifact sync.
- Support status: no spreadsheet parser, export, executable, browser, or roundtrip helper support is promoted; relevant rows remain `reference_only`.
