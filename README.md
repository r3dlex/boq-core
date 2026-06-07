# boq-core

`boq-core` is the planned Rust parser core for GAEB Bill of Quantities data used by Obra.

## Scope

- Loss-aware GAEB domain model first.
- Obra adapter DTO second; no Obra backend changes in MVP.
- GAEB DA XML 3.3 AVA certification-path readiness first.
- Bauausführung and Texterstellung follow after AVA.
- GAEB XML 3.4 beta is reference-only until explicitly promoted.

## Documentation

`boq-core` has two documentation layers:

- Rust API reference generated from crate/module rustdoc.
- mdBook manuals under `docs/book/` for users, developers, certification evidence, and releases.

Build them locally with:

```bash
cargo doc --all-features --no-deps
# Install once if mdBook is not available: cargo install mdbook --locked
mdbook build
```

The docs intentionally distinguish supported, parse-only, future-track, and reference-only formats.
BVBS and GAEBXmlChecker evidence is certification-readiness evidence, not paid or
official certification.

## Quality gates

Local/CI checks are expected to run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
archgate check --ci
prek run --all-files
```

Coverage target: 95% across the selected Rust coverage measurements once implementation code exists.

## Planning source of truth

See the workspace planning artifacts:

- `../.omx/plans/prd-boq-core-gaeb-parser-20260606.md`
- `../.omx/plans/test-spec-boq-core-gaeb-parser-20260606.md`
- `../.omx/plans/ralplan-handoff-boq-core-gaeb-parser-20260606.json`

<!-- OMX:AI-SDLC:START -->

## AI SDLC Methodology

This repository is part of the [obra-ws workspace](https://github.com/r3dlex/obra-workspace) monorepo. The AI SDLC layer is configured locally and at the workspace:

- **Karpathy guidelines:** `.agents/skills/karpathy-guidelines/SKILL.md` (entry point) and `REFERENCE.md` (stack-specific checklists).
- **Top-level rules:** `.rules.ts` (5-domain Archgate rules; complements the per-ADR rules in `.archgate/adrs/*`).
- **Prek:** `prek.toml` (companion to `.pre-commit-config.yaml`).
- **Subrepo ADRs:** `.archgate/adrs/ARCH-001..004.md`.
- **Cross-repo ADRs:** `../docs/adr/WS-XXX.md` (workspace).
- **BRD/PRD traceability:** `raw/brd/`, `raw/prd/`, `raw/tickets/`, `raw/drift-reports/`, `raw/agent-briefs/`. The workspace is the source of truth; this directory is a mirror synced via `../setup.sh plans`.

## PR merge gate (inherited from `ai-sdlc-init`)

A PR may be merged only when **all** are true: architect agrees the change matches ADRs, module boundaries, branch policy, and acceptance criteria; reviewer agrees code quality, safety, documentation, and drift checks have no blocking findings; executor agrees the requested change is implemented, cleanup is done, and required checks are green; the architect, reviewer, executor loop reaches explicit agreement.

<!-- OMX:AI-SDLC:END -->
