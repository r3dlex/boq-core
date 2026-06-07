# PRD: Stabilize public Rust API and crate documentation

## Issue
- GitHub issue: #20
- Milestone: v0.3 Public API and parser robustness

## Product outcome
The public Rust API is stable enough for Obra adapter consumers and generated rustdoc, with explicit supported/unstable/reference-only boundaries.

## Source/status anchors
- `src/lib.rs` and public modules: API contract.
- README/rustdoc: public reference surface.

## Requirements
- [ ] Mark supported, unstable, and reference-only APIs before release automation.
- [ ] Provide parse/support/adapter examples in rustdoc.
- [ ] Keep Obra adapter DTO compatibility explicit.

## Planned tests/checks
- [ ] `test_public_parse_entrypoints_are_documented`
- [ ] `test_support_status_types_are_public_and_stable`
- [ ] `test_obra_adapter_dto_contract_has_examples`
- [ ] `test_rustdoc_examples_compile`
