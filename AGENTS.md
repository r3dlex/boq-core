# boq-core — Agent Guide

> **AI SDLC Methodology** — subrepo-level entry point.
> Source-of-truth markers: `<!-- OMX:AI-SDLC:START -->` … `<!-- OMX:AI-SDLC:END -->`.

`boq-core` is a Rust library for parsing GAEB Bill of Quantities data. It is
designed to align with `obra`'s WBS/BOQ/line-item contracts without coupling
to the Obra backend in the MVP.

## Workspace binding

This subrepo is bound to the [obra-ws workspace](https://github.com/r3dlex/obra-workspace) as a sibling git repo (not a submodule). The binding contract is documented in `../docs/adr/WS-005-boq-core-as-subrepo-of-obra-ws.md`.

- **Workspace path:** `../` (obra-ws).
- **Sibling repo URL:** `https://github.com/r3dlex/boq-core.git`.
- **Plan sync command:** `../setup.sh plans`.
- **Subrepo ADRs:** `.archgate/adrs/ARCH-001..004.md` (existing).
- **Cross-repo ADRs:** `../docs/adr/WS-XXX.md` (workspace).
- **BRD/PRD traceability:** `raw/brd/`, `raw/prd/`, `raw/tickets/`, `raw/drift-reports/`, `raw/agent-briefs/`. The workspace is the source of truth; this directory is a mirror synced via `../setup.sh plans`.

## Quick context

- Stack: Rust (edition 2024, rust-version 1.85).
- Parser architecture: loss-aware rich model first, Obra adapter DTO second (ARCH-001).
- Fixture strategy: GAEB manifest with checksum/license/support_status (ARCH-002); no Obra backend coupling in MVP (ARCH-003); paid BVBS submission is user-confirmation-gated (ARCH-004).
- Quality gates: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`, `cargo llvm-cov` at 95% lines/functions/regions, `cargo run --bin xtask -- fixtures verify`, `archgate check --ci`, `prek run --all-files`.

## Agent skills (progressive disclosure)

- `.agents/skills/karpathy-guidelines/SKILL.md` (entry point) and `REFERENCE.md` (stack-specific checklists, anti-pattern examples).
- `../.agents/skills/karpathy-guidelines/SKILL.md` is the cross-repo source of truth.

## PR merge gate (inherited from `ai-sdlc-init`)

A PR may be merged only when **all** are true:

1. **Architect** agrees the change matches ADRs, module boundaries, branch policy, and acceptance criteria.
2. **Reviewer** agrees code quality, safety, documentation, and drift checks have no blocking findings.
3. **Executor** agrees the requested change is implemented, cleanup is done, and required checks are green.
4. The architect, reviewer, executor loop reaches explicit agreement. If any role disagrees or checks are not green, do not merge.

## Sandboxing (LLM agents)

- No internet access — cannot fetch URLs, install packages from registries, or call external APIs (CI's retry wrapper is an exception, scoped to the build).
- All fixture acquisition goes through `xtask` with checksum and license verification; reference-only `.exe` files must never be executed.
- No secrets in code; environment variables and config files only.
- ADRs are the source of truth; conflicts must be flagged, not silently resolved.

## AI SDLC marker

<!-- OMX:AI-SDLC:START -->

This file is the AI SDLC entry point. The marker block above is the contract
that `ai-sdlc-init` checks when re-running. The full set of artifacts:

- `.agents/skills/karpathy-guidelines/` — engineering habits.
- `.rules.ts` — 5-domain Archgate rules (complements the per-ADR rules in `.archgate/adrs/*`).
- `prek.toml` — pre-commit configuration (companion to `.pre-commit-config.yaml`).
- `raw/brd/`, `raw/prd/`, `raw/tickets/`, `raw/drift-reports/`, `raw/agent-briefs/` — BRD/PRD traceability chain.

<!-- OMX:AI-SDLC:END -->
