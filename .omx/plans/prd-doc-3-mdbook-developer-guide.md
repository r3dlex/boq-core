# PRD: mdBook developer guide

## Planned issue
- Planned issue key: DOC-3
- Planned milestone: boq-core documentation MVP

## Product outcome
The developer guide documents architecture, extension points, fixture governance, TDD/coverage workflow, and Obra compatibility contract.

## Requirements
- [x] Architecture overview links model/parser/adapter/support modules.
- [x] Fixture governance explains checksums, support statuses, and source policies.
- [x] TDD guide states red-green-refactor and 95% coverage expectations.

## Planned checks
- [x] `mdbook build`
- [x] `test_developer_guide_links_fixture_manifest`
- [x] `test_developer_guide_states_95_coverage_policy`

## Delivery evidence
- Developer guide expanded in `docs/book/developer-guide.md`.
- Regression tests added in `tests/docs_mvp.rs`.
- Local docs gate: `cargo test --test docs_mvp`, `mdbook build`, `archgate check --ci`, `uvx prek run --all-files`.
