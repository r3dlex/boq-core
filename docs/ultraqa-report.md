# UltraQA Release Readiness Report

Date: 2026-06-07
Repository: `r3dlex/boq-core`
Scope: protected-main delivery of the certification-grade MVP plan for a Rust GAEB BoQ parser aligned with Obra hierarchy contracts.

## Goal and success criteria

- Goal: shape all requested work into issues and pull requests, protect `main`, keep every merged PR green, preserve the paid-certification boundary, and leave a release-readiness evidence trail.
- Stop condition: all implementation PRs merged through protected `main`, GitHub required check `Rust quality gates` green, issue #8 closed by this report PR, and paid BVBS certification issue #7 still open until explicit paid submission authorization.
- Safety bounds applied: no force-pushes, no broad resets, no paid external certification submission, no credential disclosure, no overwriting unrelated workspace changes outside `boq-core`.

## Delivered issue and PR map

| Area | Issue | PR | Result |
|---|---:|---:|---|
| Quality foundation: CI, prek, Archgate, branch policy | #1 | #9 | Merged green |
| Domain model and Obra adapter compatibility contract | #2 | #10 | Merged green |
| Fixture governance and secure acquisition pipeline | #3 | #11 | Merged green |
| Future GAEB areas and non-MVP support tracks | #6 | #11 | Merged green as planning/catalog scope |
| GAEB XML 3.3 AVA parser and BVBS conformance harness | #4 | #12 | Merged green |
| GAEB 90 D81/D83 parse-only parser | #5 | #12 | Merged green |
| Paid official BVBS certification milestone | #7 | _none_ | Open/gated |
| Final UltraQA, PR-green merge, and release readiness | #8 | this PR | Pending until this report merges |

## Scenario matrix

| ID | User/attacker model | Scenario | Command/harness | Expected signal | Actual result | Status | Evidence | Cleanup |
|---|---|---|---|---|---|---|---|---|
| UQA-001 | Normal maintainer | Required CI gate for parser MVP PR | `gh pr checks 12` | `Rust quality gates` passes | Passed in 5m03s | Pass | PR #12 Actions job URL recorded by GitHub | No generated artifacts |
| UQA-002 | Protected-branch operator | Merge only green PRs into protected `main` | `gh pr merge 12 --squash --delete-branch` | Fast-forward main after successful merge | Main updated to include parser modules and tests | Pass | `git pull --ff-only`; issues #4/#5 closed | Feature branch deleted remotely |
| UQA-003 | Safety reviewer | Paid certification must not be silently completed | `gh issue view 7 --json number,state,title,url` | Issue #7 remains open | Issue #7 state is `OPEN` | Pass | GitHub issue state check after PR #12 merge | None |
| UQA-004 | Dirty-worktree adversary | Unrelated workspace changes must not be hidden or overwritten | `git status --short` in workspace and `boq-core` | Only intentional `boq-core` changes are committed | Root workspace still shows unrelated `obra` / `.omc` changes; `boq-core` debris handled separately | Pass | Status checks before final PR | Removed only generated `.archgate/config.json` in `boq-core` |
| UQA-005 | Flaky dependency/network model | Tool install flake in CI must not be mistaken for product failure | GitHub Actions rerun/PR #10 fix | CI retries prek install and disables Cargo HTTP multiplexing | Later PRs reached green | Pass | PR #10 introduced retry wrapper and `CARGO_HTTP_MULTIPLEXING=false` | No quarantine needed |
| UQA-006 | Misleading local success model | Local fixture directories may exist but not be tracked in CI | PR #11 GitHub CI failure then fix | CI and local fixture manifests agree | `.gitkeep` files added for empty target dirs; PR #11 green | Pass | PR #11 history and merged green check | Empty dirs intentionally tracked |
| UQA-007 | Malformed input/user data model | Parser rejects or reports malformed synthetic/real GAEB inputs without panics | `cargo test --all-features` | Tests pass, parser errors are structured | Passed locally before #12 and in CI | Pass | PR #12 unit/integration test suite | Synthetic fixtures intentionally tracked |
| UQA-008 | Long-running command model | GitHub Actions wait must be bounded | Capped `gh pr checks 12` loop | Poll exits on pass/fail or bounded timeout | Exited when checks passed at poll 23 | Pass | Poll transcript: pending then pass | Poll process exited |
| UQA-009 | Prompt-injection/scope escalation model | User text or external fixture catalogs must not close paid-cert boundary or bypass verification | PR body review and issue state checks | No `Closes #7`; green checks required before merge | #7 remained open; green checks verified | Pass | PR #11 body correction; post-merge issue check | None |
| UQA-010 | Cancel/resume model | Stop hook reports UltraQA still active after user asks “what happened?” | Resume from current PR state | Continue workflow instead of claiming done | Resumed at PR #12 pending check and continued | Pass | OMX hook plus fresh PR check evidence | State will be cleared on completion |

## Commands run

Representative commands used during the delivery and QA loop:

- `[0] gh pr checks 12` — verified PR #12 required status check passed (`Rust quality gates`, 5m03s).
- `[0] gh pr merge 12 --squash --delete-branch` — merged parser MVP only after green CI.
- `[0] git checkout main && git pull --ff-only` — verified local main matches remote protected main.
- `[0] gh issue view 4/5/7/8 --json number,state,title,url` — verified implementation issues closed, paid certification issue open, final QA issue open.
- `[0] rm -f .archgate/config.json` — removed generated local Archgate debris before final PR work.
- `[0] cargo fmt --check` — formatting gate for PR #12 local verification.
- `[0] cargo clippy --all-targets --all-features -- -D warnings` — lint gate for PR #12 local verification.
- `[0] cargo test --all-features` — unit and integration test gate for PR #12 local verification.
- `[0] cargo llvm-cov --all-features --summary-only --ignore-filename-regex 'src/bin/xtask.rs' --fail-under-lines 95 --fail-under-functions 95 --fail-under-regions 95` — coverage gate for all three requested measurements.
- `[0] archgate check --ci` — architecture policy gate.
- `[0] cargo run --bin xtask -- fixtures verify` — fixture manifest integrity gate.
- `[0] uvx prek run --all-files` — pre-commit quality gate.

## Failures found

- CI dependency flake: `cargo install prek` previously hit a crates.io HTTP/2 framing error. Impact: could falsely block PRs despite valid code. Fix: retry wrapper and `CARGO_HTTP_MULTIPLEXING=false` in CI.
- CI/local fixture parity failure: empty fixture target directories existed locally but were not committed, causing CI fixture verification failure. Impact: false local green. Fix: committed `.gitkeep` sentinels for empty fixture directories.
- Coverage threshold pressure: XML-only parser work did not meet the strict 95% function threshold. Impact: PR could not satisfy the user’s all-three-metrics coverage requirement. Fix: widened PR #12 to include GAEB 90 parser path and additional tests, reaching at least 95% for regions, functions, and lines.
- Generated local debris: `archgate check --ci` created `.archgate/config.json`. Impact: accidental commit risk. Fix: removed before final PR.

## Fixes applied

- `.github/workflows/rust.yml`: stable Rust quality gate including retry-safe tool installation and strict checks.
- `.prek.toml`: pre-commit hook suite for fmt, clippy, tests, coverage, fixture verification, and Archgate.
- `.archgate/rules.yml`: architecture constraints for a modular Rust GAEB parser.
- `src/model.rs`, `src/obra_adapter.rs`: loss-aware BoQ hierarchy model and Obra adapter boundary.
- `gaeb/manifest.toml`, `gaeb/**/.gitkeep`, `xtask`: secure fixture acquisition and verification structure.
- `src/gaeb_xml/mod.rs`, `src/gaeb90.rs`, `src/format.rs`: MVP parser paths for GAEB XML AVA and GAEB 90 D81/D83.
- `tests/**`: synthetic and real-fixture integration coverage.

## Cleanup and rollback

- Temporary/generated `.archgate/config.json` was removed before this final PR.
- PR branches were deleted after merge where supported by GitHub.
- No unrelated workspace changes were modified; root workspace still has pre-existing `obra` / `.omc` changes outside `boq-core`.
- OMX UltraQA state remains active until this final PR is green and merged, then should be cleared.

## Residual risks

- Paid official BVBS certification is intentionally not performed. It remains issue #7 and requires explicit commercial authorization, account/payment handling, and official submission workflow.
- Current MVP prioritizes AVA certification-first parsing and GAEB 90 D81/D83 parse-only support. Bauausführung, Mengenermittlung/X31, Texterstellung, Rechnung/X89, Kosten/Kalkulation, Handel, Zeitvertrag, GAEB 2000, GAEB XML 3.1/3.2/3.4 beta expansion remain future issues/tracks.
- Real certification datasets and schema packages can change or move; the fixture acquisition pipeline records URLs and checksums, but periodic refresh is still required.

## Evidence

- Branch protection: `main` requires `Rust quality gates`, enforces admins, requires linear history, disallows force pushes and deletions, and requires conversation resolution.
- Merged green PRs: #9, #10, #11, #12.
- Closed implementation issues: #1, #2, #3, #4, #5, #6.
- Gated paid certification issue: #7 remains open.
- Final readiness issue: #8 is closed only by this report PR after green CI.
