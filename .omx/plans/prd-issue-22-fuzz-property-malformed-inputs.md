# PRD: Add fuzz and property tests for malformed GAEB inputs

## Issue
- GitHub issue: #22
- Milestone: v0.3 Public API and parser robustness

## Product outcome
GAEB 90 and XML parser recovery is protected by fuzz/property tests that prevent panics and silent loss.

## Source/status anchors
- GAEB 90/XML malformed fixtures: synthetic test evidence.
- Fuzz corpus: generated regression evidence.

## Requirements
- [ ] Define loss-aware recovery invariants.
- [ ] Add corpus/minimization strategy.
- [ ] Ensure unsupported data emits findings.

## Planned tests
- [ ] `test_gaeb90_random_line_lengths_never_panic`
- [ ] `test_xml_unknown_elements_never_silent_drop`
- [ ] `test_malformed_ordinal_numbers_emit_findings`
- [ ] `test_fuzz_corpus_minimizes_regressions`
