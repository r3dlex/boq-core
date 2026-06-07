# Deep Interview Handoff — boq-core missing specs and documentation MVP

Date: 2026-06-07
Mode: `$deep-interview` → `$ralplan`
Repo: `r3dlex/boq-core`

## User intent

Create execution-ready specifications for the remaining original boq-core roadmap areas and documentation work:

1. Per-issue PRDs/specs for open issues #15–#36.
2. Actual implementation path for the planned work.
4. Official/source verification refresh.
5. Crates.io/release setup.
6. GAEBXmlChecker integration.
7. Public documentation generation for guides/manuals/reference.

The result must preserve the existing certification-grade MVP direction, Obra compatibility contract, high quality gates, PR-only green merge workflow, and honest support status boundaries.

## Settled interview decisions

| Round | Decision | Result |
| --- | --- | --- |
| 1 | Documentation strategy | **Both integrated**: rustdoc for API/reference and mdBook for guides/manuals, cross-linked and CI-validated. |
| 2 | Documentation artifact scope | **Docs milestone**: create a small dedicated documentation milestone rather than hiding all work inside #20. |
| 3 | Execution boundary | **Generate docs MVP**: after planning, implementation may create initial mdBook + crate docs + starter guides + build checks. |
| 4 | Docs MVP content | Required: API reference, user guide, developer guide, certification guide, release guide. |

## Required documentation MVP

The docs MVP is complete only when all of these exist and build locally/CI:

- **API reference**: crate-level rustdoc, public module docs, examples, parse/adapter snippets, support-status warnings.
- **User guide**: mdBook quickstart/manual covering fixture parsing, GAEB phases, output interpretation, and supported vs reference-only formats.
- **Developer guide**: architecture, extension points, fixture governance, TDD/coverage workflow, and Obra compatibility contract.
- **Certification guide**: BVBS/GAEBXmlChecker evidence workflow, paid-certification gates, source matrix, and no-overclaiming rules.
- **Release guide**: semver, crates.io publishing readiness, docs publishing checks, protected-branch/full-green PR expectations.

## Non-goals and gates

- Do **not** perform paid BVBS certification submission, payment, or vendor contact without explicit authorization.
- Do **not** claim official support/certification merely because a fixture/schema/checker exists.
- Do **not** create duplicate issue explosion beyond the chosen small docs milestone.
- Do **not** merge directly to `main`; all source/doc changes go through PR and green checks.
- Do **not** weaken the existing 95% line/function/region coverage policy.

## Source refresh facts to preserve in plans

- GAEB downloads page identifies **GAEB DA XML 3.3 2023-01** as current and states 3.4 2026-03 files are beta, not currently introduced/valid.
- GAEB 3.3 2023-01 changed documentation and the X31 Mengenermittlung package; other thematic packages remain at 2021-05.
- GAEB product page warns older XML rules, including XML 3.2, are no longer fachlich supported and are not syntax-compatible with 3.3.
- BVBS certification distinguishes GAEB DA XML 3.3/3.2/3.1 certification areas including AVA, Bauausführung, Mengenermittlung, and Texterstellung.
- BVBS certification download page lists GAEB Checker plus official criteria/files for AVA, Bauausführung, Mengenermittlung, and Texterstellung.

## Ralplan task

Create a consensus plan that:

1. Generates issue specs/PRDs/test-specs for #15–#36.
2. Adds a small documentation milestone with focused docs issues and PRDs.
3. Defines implementation sequencing for docs MVP without bypassing ralplan/execution boundary.
4. Updates GitHub issues with plan links and support-status guardrails.
5. Includes Architect → Critic consensus evidence before implementation handoff.
6. Recommends the next execution command after planning, preferably `$ultragoal` for durable execution and `$team` where parallel docs/spec/issue lanes are useful.

