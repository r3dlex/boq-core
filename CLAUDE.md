# boq-core — Claude / Codex Agent Instructions

> **Full guide:** [AGENTS.md](AGENTS.md) (entry point for AI agents).
> **AI SDLC Methodology:** [AGENTS.md](AGENTS.md) "AI SDLC marker" section.

## Quick context

`boq-core` is a Rust GAEB parser. Stack: Rust (edition 2024, rust-version 1.85). Module namespace: `boq_core::*`. Loss-aware rich model first, Obra adapter DTO second (ARCH-001).

## Critical rules

1. **Loss-aware first** (ARCH-001). Parser modules must not map directly into Obra DTOs.
2. **Support-status honesty** (ARCH-002). `supported` requires failing tests, implementation, fixture verify, and review evidence.
3. **No Obra backend coupling in MVP** (ARCH-003). Do not modify the sibling ERP backend (see binding ADR WS-005).
4. **Certification boundary** (ARCH-004). Paid BVBS submission is a user-confirmation-gated external action; never auto-claim or auto-submit.
5. **Deterministic artifacts.** Cargo.lock, gaeb/manifest.toml, gaeb/fixtures.lock are versioned and checksum-verified.
6. **No new dependencies without explicit request** (per `ai-sdlc-init`).
7. **ADRs are the source of truth**; conflicts must be flagged.

## Quality gates (every PR)

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo llvm-cov --all-features --summary-only \
  --ignore-filename-regex 'src/bin/xtask.rs' \
  --fail-under-lines 95 --fail-under-functions 95 --fail-under-regions 95
cargo run --bin xtask -- fixtures verify
archgate check --ci
prek run --all-files
```

## PR merge gate (inherited from `ai-sdlc-init`)

Architect, reviewer, and executor must all agree, and required checks must be green. If any role disagrees or checks are not green, do not merge.

## Workspace binding

- **Workspace:** `../` (obra-ws).
- **Subrepo binding ADR:** `../docs/adr/WS-005-boq-core-as-subrepo-of-obra-ws.md`.
- **Plan sync command:** `../setup.sh plans`.
- **Cross-repo ADRs:** `../docs/adr/WS-XXX.md`.
