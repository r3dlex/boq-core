# Test Spec: Bauausführung X83 parser support

## Issue
- GitHub issue: #26
- Spec: `.omx/specs/issue-26-bauausfuehrung-x83-parser.md`

## Required automated tests
- `test_bau_x83_extracts_project_and_boq_metadata`
  - Verifies X83 source URI detection, project name, version, title, support status, capabilities, and checksum.
- `test_bau_x83_sections_and_items_match_hierarchy`
  - Verifies section ordinal/title, item ordinal/RNoPart, quantity, unit, long text, and sort order.
- `test_bau_x83_tender_specific_fields_are_preserved`
  - Verifies unsupported X83 tender/execution fields are retained as node metadata with text content.
- `test_bau_x83_unknown_nodes_emit_findings`
  - Verifies unsupported X83 item fields emit `gaeb_xml_unsupported_item_field` warnings with deterministic locations.
- `test_bau_x83_adapter_compatibility_remains_capability_gated`
  - Verifies cataloged X83 parse-only documents still reject Obra adapter conversion.

## Local gate sequence
1. `cargo fmt --check`
2. `cargo test --test bau_roundtrip`
3. `cargo test --test testing_strategy`
4. `cargo clippy --all-targets --all-features -- -D warnings`
5. `cargo test --all-features`
6. `cargo run --bin xtask -- fixtures verify`
7. `cargo doc --all-features --no-deps`
8. `mdbook build`
9. `archgate check --ci`
10. `uvx prek run --all-files`
11. `cargo llvm-cov --all-features --summary-only --ignore-filename-regex 'src/bin/xtask.rs' --fail-under-lines 95 --fail-under-functions 95 --fail-under-regions 95`

## Merge gate
- GitHub PR must include `Closes #26`.
- All actionable review comments must be resolved.
- GH `Rust quality gates` must be successful and merge state must be clean.
- Self-approval is attempted; GitHub may reject own-PR approval even for admin users, in which case the rejection is recorded as evidence.
