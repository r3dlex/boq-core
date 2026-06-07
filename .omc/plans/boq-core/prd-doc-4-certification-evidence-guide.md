# PRD: Certification evidence guide

## Planned issue
- Planned issue key: DOC-4
- Planned milestone: boq-core documentation MVP

## Product outcome
The certification guide documents BVBS/GAEBXmlChecker evidence workflow, paid-certification gates, source matrix, and no-overclaiming rules.

## Requirements
- [ ] State that checker/fixture evidence is readiness evidence only.
- [ ] Document paid-action gates and required human authorization.
- [ ] Link AVA, Bauausführung, Mengenermittlung, and Texterstellung evidence tracks.

## Planned checks
- [ ] `mdbook build docs/book`
- [ ] `test_certification_guide_mentions_paid_gate`
- [ ] `test_certification_guide_uses_no_certified_claim`
