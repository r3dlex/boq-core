# PRD: Add fuzz and property tests for malformed GAEB inputs

## Issue
- GitHub issue: #22
- Milestone: v0.3 Public API and parser robustness

## Product outcome
GAEB 90 and XML parser recovery is protected by bounded fuzz/property-style tests that prevent panics and silent loss while keeping CI runtime deterministic.

## Source/status anchors
- GAEB 90/XML malformed fixtures: `tests/malformed_input_properties.rs`.
- Fuzz corpus: minimized in-test corpus with unique labels and <=512 byte inputs.

## Requirements
- [x] Define loss-aware recovery invariants.
  - GAEB 90 line length variation must parse or return structured `ParseError` without panicking.
  - XML unknown empty payloads must survive as loss-aware `gaeb.empty.*` metadata while known item fields remain available.
  - Blank malformed item ordinals must recover to stable fallback ordinals and emit findings.
- [x] Add corpus/minimization strategy.
  - `REGRESSION_CORPUS` is deterministic, label-unique, entrypoint-tagged, and bounded to small minimized inputs for CI.
- [x] Ensure unsupported data emits findings.
  - Added `gaeb90_malformed_ordinal` and `gaeb_xml_malformed_ordinal` recoverable warnings for blank malformed item ordinals.

## Implemented tests
- [x] `test_gaeb90_random_line_lengths_never_panic`
- [x] `test_xml_unknown_elements_never_silent_drop`
- [x] `test_malformed_ordinal_numbers_emit_findings`
- [x] `test_fuzz_corpus_minimizes_regressions`

## Verification
- [x] `cargo test --test malformed_input_properties`
- [x] `cargo test gaeb90::tests::malformed_lines_are_recoverable_findings`
