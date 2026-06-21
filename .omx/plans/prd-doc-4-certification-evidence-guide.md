# PRD: Certification evidence guide

## Planned issue
- Planned issue key: DOC-4
- Planned milestone: boq-core documentation MVP

## Product outcome
The certification guide documents BVBS/GAEBXmlChecker evidence workflow, paid-certification gates, source matrix, and no-overclaiming rules.

## Requirements
- [x] State that checker/fixture evidence is readiness evidence only.
- [x] Document paid-action gates and required human authorization.
- [x] Link AVA, Bauausführung, Mengenermittlung, and Texterstellung evidence tracks.

## Planned checks
- [x] `mdbook build`
- [x] `test_certification_guide_mentions_paid_gate`
- [x] `test_certification_guide_uses_no_certified_claim`

## Delivery evidence
- Certification evidence guide expanded in `docs/book/certification-evidence-guide.md`.
- Regression tests added in `tests/docs_mvp.rs`.
- Local docs gate: `cargo test --test docs_mvp`, `mdbook build`, `archgate check --ci`, `uvx prek run --all-files`.
