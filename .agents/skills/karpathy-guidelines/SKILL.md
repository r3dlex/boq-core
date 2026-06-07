---
name: karpathy-guidelines
description: 'AI SDLC engineering habits at the boq-core subrepo. Tight feedback loops, diff minimalism, test-first, explicit rules, deterministic artifacts, reverse-first. Use when planning or reviewing any boq-core change.'
---

# Karpathy Guidelines (boq-core)

> **AI SDLC Methodology** — subrepo-level guidance.
> Source-of-truth markers: `<!-- OMX:AI-SDLC:START -->` … `<!-- OMX:AI-SDLC:END -->`.
> Binds to: `AGENTS.md`, `CLAUDE.md`, `README.md`, `.archgate/adrs/`, `.rules.ts`, `prek.toml`.
> Workspace-level: `../.agents/skills/karpathy-guidelines/SKILL.md`.

## Intent

Apply the Karpathy engineering habits in the boq-core Rust crate:

- **Tight feedback loops** — `cargo test --all-features`, `archgate check --ci`, `prek run --all-files` all run in seconds locally.
- **Diff minimalism** — one PR per US-XXX / issue-XXX change; one ADR per architectural decision; one fixture per test gap.
- **Test-first** — failing tests named in PRDs; every issue PRD has a "concrete test names" section.
- **Explicit over implicit** — 4 ADRs (ARCH-001..004) + `.archgate/rules.d.ts` + per-ADR `.rules.ts` + top-level `.rules.ts`.
- **Deterministic artifacts** — `Cargo.lock`, `gaeb/manifest.toml`, `gaeb/fixtures.lock` are versioned; xtask is excluded from coverage; checksum-verified.
- **Reverse first** — new ADRs supersede or extend; never duplicate.

## boq-core-specific application

| Habit | Concrete expression |
| --- | --- |
| Tight loop | `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features` |
| Diff minimalism | One issue-XXX PR (issues #37–#44); one ADR for the boundary decision |
| Test-first | Test spec names listed in every issue-level PRD |
| Explicit over implicit | support_status honesty (ARCH-002) + certification boundary (ARCH-004) |
| Deterministic artifacts | `gaeb/manifest.toml` with checksums; `fixtures verify` is a CI gate |
| Reverse first | `loss-aware` model before adapter (ARCH-001) |

## Anti-patterns to refuse

- **Overclaim** — a fixture is `supported` only when failing tests, implementation, fixture verify, and review evidence all agree.
- **Loose certification** — paid BVBS submission is a user-confirmation-gated external action (ARCH-004); never auto-claim or auto-submit.
- **Executed reference** — a reference-only `.exe` (e.g. `GAEBXmlChecker`) must never run in CI.
- **Lossy parsing** — parser maps into rich model first; Obra DTO is a separate adapter (ARCH-001).
- **Obra backend coupling** — do not modify `../obra/backend` in the MVP (ARCH-003).

## PR merge gate (inherited from `ai-sdlc-init`)

A PR may be merged only when **all** are true:

1. **Architect** agrees the change matches ADRs, module boundaries, branch policy, and acceptance criteria.
2. **Reviewer** agrees code quality, safety, documentation, and drift checks have no blocking findings.
3. **Executor** agrees the requested change is implemented, cleanup is done, and required checks are green.
4. The architect, reviewer, executor loop reaches explicit agreement.

## Progressive disclosure

This file is the entry point. Read `REFERENCE.md` only when you need the full
habit catalog or stack-specific checklists.
