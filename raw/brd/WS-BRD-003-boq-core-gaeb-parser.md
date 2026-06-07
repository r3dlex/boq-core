---
id: WS-BRD-003
title: boq-core GAEB parser and certification harness
status: accepted
date: 2026-06-07
tags: [workspace, boq-core, gaeb, certification, parser]
brd_source: .omx/plans/prd-boq-core-gaeb-parser-20260606.md (PRD §1–§5)
test_spec_source: .omx/plans/test-spec-boq-core-gaeb-parser-20260606.md
ralplan_source: .omx/plans/ralplan-handoff-boq-core-gaeb-parser-20260606.json (architect APPROVE + critic APPROVE)
ultragoal_source: .omx/plans/ultragoal-brief-boq-core-gaeb-parser-20260606.md
ultraqa_source: boq-core/docs/ultraqa-report.md
target_deadline: 2026-09-30 (inherited; primary MVP is delivered)
subrepo_binding:
  boq-core: boq-core/.archgate/adrs/ARCH-001..004.md
tracked_by: r3dlex/boq-core issues #1–#8 (closed) and #7 (open, gated)
---

# WS-BRD-003: boq-core GAEB parser and certification harness

## Business problem

Obra needs a canonical, production-grade GAEB BoQ parsing core that can ingest
German construction tender data without embedding ad-hoc parsing logic into
the Elixir/Phoenix app. GAEB data has multiple format generations, phase-
specific semantics, certification expectations, legacy encoding hazards, and
rich XML structures that can exceed Obra's current WBS/BOQ/line-item fields.

## Target users / stakeholders

- Obra engineering team — integrates GAEB import into the ERP later.
- Rust library consumers — need typed GAEB parsing and certification-oriented fixtures.
- Certification/test maintainers — validate GAEB conformance behavior.
- BVBS (external) — receives certification submission only after explicit user confirmation; the MVP is certification-path readiness, not paid submission.

## Desired outcomes / metrics

- A Rust library named `boq-core` with a loss-aware GAEB domain model.
- An Obra adapter DTO that maps parsed GAEB data to Obra WBS nodes, BOQ documents, and line items.
- GAEB DA XML 3.3 AVA certification readiness (BVBS X81/X84/X86).
- 95% unit-test coverage across the three user-requested measurements (lines, functions, regions).
- `archgate check --ci` passes; `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`, `cargo run --bin xtask -- fixtures verify` all pass.
- `prek run --all-files` passes.
- All 5 ADRs (ARCH-001..004 + repo-specific) are respected; no overclaim.

## Constraints and non-goals

- Do not modify the Obra backend in the MVP.
- Do not perform paid BVBS submissions or represent the library as officially certified without explicit confirmation.
- Do not make GAEB XML 3.4 beta the stable v1 target.
- Do not execute downloaded Windows executables in CI.
- Do not claim support for X31, X89, XRechnung, Handel, Rechnung, Zeitvertrag, GAEB XML 3.1/3.2, or GAEB 2000 unless the corresponding support status is explicitly promoted from `future_track` / `reference_only`.

## Risks and open questions

- **Tool install flake in CI** (cargo, prek, archgate npm). Mitigation: retry wrappers and `CARGO_HTTP_MULTIPLEXING=false`.
- **Misleading local success** — local fixture directories may exist but not be tracked in CI. Mitigation: `.gitkeep` for empty dirs; CI manifest verify.
- **Prompt-injection / scope escalation** — user text or external fixture catalogs must not close the paid-cert boundary. Mitigation: PR body review and issue state checks.
- **Malformed input** — parser must reject or report malformed synthetic/real GAEB inputs without panics. Mitigation: structured errors + property/fuzz tests.
- **BVBS GAEBXmlChecker** is a Windows `.exe` reference; it must never run in CI by default.

## Acceptance criteria

- [ ] Per-source support matrix preserved in every future implementation PRD (issues #37–#44).
- [ ] First architecture decision is completed before parser/model code changes (e.g., gap-analysis ADR for issue #40).
- [ ] Test-spec concrete tests are created or consciously deferred with rationale.
- [ ] Protected-main gates remain green: fmt, clippy, tests, 95% coverage thresholds, fixture verify, Archgate, prek.
- [ ] All BVBS certification/checker areas and non-certification sources cataloged; future tracks have catalog-test mappings; reference-only fixtures do not claim support; requested phases are represented.
- [ ] Obr a adapter DTO is deterministic and provenance-aware.
- [ ] Final quality gate (G007) reports coverage ≥ 95% across the three measurements.

## Subrepo binding

- obra: n/a for MVP (ARCH-001..005 do not constrain boq-core directly); future integration is a separate PRD.
- boq-core: `boq-core/.archgate/adrs/ARCH-001-loss-aware-model-before-adapters.md`, `ARCH-002-fixture-support-status-honesty.md`, `ARCH-003-no-obra-backend-mvp.md`, `ARCH-004-external-certification-gated.md`.

## Verification

- `cargo fmt --check` exits 0.
- `cargo clippy --all-targets --all-features -- -D warnings` exits 0.
- `cargo test --all-features` exits 0 with all unit + integration tests passing.
- `cargo llvm-cov --all-features --summary-only --ignore-filename-regex 'src/bin/xtask.rs' --fail-under-lines 95 --fail-under-functions 95 --fail-under-regions 95` reports ≥ 95% in each measurement.
- `cargo run --bin xtask -- fixtures verify` exits 0.
- `archgate check --ci` exits 0.
- `prek run --all-files` exits 0.

## Rollback

- The MVP is delivered; rollback is to a previous tagged release. If a future issue-level PRD (#37–#44) is rejected, the corresponding fixture entries are marked `reference_only` (not deleted) and the test mapping is updated.
- Paid BVBS submission (issue #7) is user-confirmation-gated; it has no auto-path and no rollback needed.
