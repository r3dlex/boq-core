# PRD: Rustdoc API reference

## Planned issue
- GitHub issue: #50
- Planned issue key: DOC-1
- Planned milestone: boq-core documentation MVP

## Product outcome
Crate/module rustdoc documents public parse, support-status, provenance/checksum, and Obra-adapter APIs with executable examples and support-status warnings.

## Requirements
- [x] Crate-level docs explain supported MVP and future/reference tracks.
- [x] Public modules include examples or explicit doc rationale.
- [x] Doctests compile where examples are executable.

## Implemented checks
- [x] `cargo doc --all-features --no-deps`
- [x] `cargo test --doc`
- [x] `test_rustdoc_api_reference_no_support_overclaiming`
- [x] `test_rustdoc_examples_parse_minimal_fixture`
- [x] `public_modules_have_examples_or_doc_rationale`

## Verification
- [x] `cargo test --test public_api_docs`
- [x] `cargo test --doc`
