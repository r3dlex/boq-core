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
- [x] Mark supported, unstable, and reference-only APIs before release automation.
  - Evidence: `src/support.rs` documents supported vs parse-only vs future-track vs reference-only and independent `SupportCapabilities`.
- [x] Provide parse/support/adapter examples in rustdoc.
  - Evidence: `src/lib.rs` documents D81/D83/X81/X83 parse entrypoints; `src/adapter/obra.rs` includes compile-checked adapter examples.
- [x] Keep Obra adapter DTO compatibility explicit.
  - Evidence: `src/adapter/obra.rs` documents `ObraImportDocument` as the DTO boundary and `obra_adapter_not_supported` finding semantics.

## Planned tests/checks
- [x] `public_parse_entrypoints_are_documented_for_required_phases`
- [x] `support_status_types_are_public_and_stable`
- [x] `obra_adapter_dto_contract_has_examples`
- [x] `cargo test --doc`

## Verification
- `cargo fmt --check`
- `cargo test --test public_api_docs`
- `cargo test --doc`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- `cargo run --bin xtask -- fixtures verify`
- `cargo doc --all-features --no-deps`
- `mdbook build`
- `archgate check --ci`
- `uvx prek run --all-files`
