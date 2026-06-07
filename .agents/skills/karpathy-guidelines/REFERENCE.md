# Karpathy Guidelines — Reference (boq-core)

> Companion to `SKILL.md`. Load only when you need stack-specific checklists or anti-pattern examples.

## Stack-specific checklist (Rust)

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features` (all unit + integration + conformance + property tests)
- [ ] `cargo llvm-cov --all-features --summary-only --ignore-filename-regex 'src/bin/xtask.rs' --fail-under-lines 95 --fail-under-functions 95 --fail-under-regions 95`
- [ ] `cargo run --bin xtask -- fixtures verify`
- [ ] `archgate check --ci`
- [ ] `prek run --all-files` (or `uvx prek run --all-files`)
- [ ] No new dependency unless explicitly requested (`ai-sdlc-init` rule)

## Support-status honesty (ARCH-002)

- `supported` — failing tests first, implementation, fixture verify, review evidence.
- `supported_parse_only` — parses but does not produce Obra adapter DTO.
- `future_track` — cataloged, not yet implemented.
- `reference_only` — documentation, executable, commercial, browser, or beta sources; must never run in CI.

## Certification boundary (ARCH-004)

- `boq-core` may prepare certification evidence, but paid BVBS submission and official certification representation require explicit user confirmation.
- CI and docs must say "certification-path readiness" unless official certification is actually obtained.
- No paid or credentialed external action occurs automatically.

## Loss-aware model first (ARCH-001)

- Parser modules must not map directly into Obra DTOs.
- Unsupported GAEB fields are preserved in metadata or loss reports.
- Adapter tests must snapshot loss/provenance data.

## Obra backend boundary (ARCH-003)

- The MVP must not modify the Obra Elixir/Phoenix backend.
- It may emit DTOs compatible with Obra concepts.

## Anti-pattern examples

### Overclaim
```rust
// BAD — claimed support without failing tests
support_status: "supported"
// GOOD — supported only with test_mapping
support_status: "supported"
test_mapping: ["test_bvbs_ava_x81_parse", "test_bvbs_ava_x84_parse"]
```

### Executed reference
```yaml
# BAD — runs the .exe in CI
- run: ./GAEBXmlChecker.exe fixtures/...
# GOOD — catalog only
- run: echo "GAEBXmlChecker is reference-only; do not run"
```

### Lossy parser
```rust
// BAD — parser maps directly into Obra DTO
fn parse(...) -> ObraDto { ... }
// GOOD — parser maps to rich model; adapter is separate
fn parse(...) -> GaebModel { ... }
fn adapt_to_obra(m: &GaebModel) -> Result<ObraDto, LossReport> { ... }
```

## PR merge gate (inherited from `ai-sdlc-init`)

A PR may be merged only when **all** are true:

1. **Architect** agrees the change matches ADRs, module boundaries, branch policy, and acceptance criteria.
2. **Reviewer** agrees code quality, safety, documentation, and drift checks have no blocking findings.
3. **Executor** agrees the requested change is implemented, cleanup is done, and required checks are green.
4. The architect, reviewer, executor loop reaches explicit agreement.
