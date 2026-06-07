# PRD: Rustdoc API reference

## Planned issue
- Planned issue key: DOC-1
- Planned milestone: boq-core documentation MVP

## Product outcome
Crate/module rustdoc documents public parse, support-status, and Obra-adapter APIs with examples and support-status warnings.

## Requirements
- [ ] Crate-level docs explain supported MVP and future/reference tracks.
- [ ] Public modules include examples or explicit doc rationale.
- [ ] Doctests compile where examples are executable.

## Planned checks
- [ ] `cargo doc --all-features --no-deps`
- [ ] `test_rustdoc_api_reference_no_support_overclaiming`
- [ ] `test_rustdoc_examples_parse_minimal_fixture`
