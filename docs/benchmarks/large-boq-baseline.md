# Large BoQ benchmark baseline

Issue: #23

## Purpose

This benchmark note records advisory, reproducible smoke baselines for large bill-of-quantities parsing. It is not a certification claim and it does not promote any fixture support status.

## Machine context

- CPU: Apple Silicon local development machine
- Memory: developer workstation memory, not normalized for CI capacity
- OS: macOS local run and GitHub-hosted Linux CI smoke gate
- Rust toolchain: stable toolchain from repository CI
- Command shape: `cargo test --test large_boq_benchmarks`

## Fixture scope

- Synthetic large BoQ: generated deterministically by `tests/large_boq_benchmarks.rs` with 250 AVA X81 items in CI smoke mode.
- License-safe fixture path: `tests/fixtures/synthetic/minimal_ava.x81` remains in the harness as a known safe real repository fixture path.

## Smoke budgets

These budgets are intentionally generous enough for normal CI and narrow enough to catch accidental severe regressions before release:

| Scenario | Input | Advisory budget |
| --- | --- | --- |
| Parse large GAEB XML | 250 generated AVA X81 items | <= 2 seconds |
| Convert to Obra DTO | Parsed 250-item AVA X81 document | <= 2 seconds |

## Baseline observations

Initial local smoke runs completed both parse and adapter conversion under the 2 second advisory budgets. Treat these as regression smoke checks, not absolute throughput claims; collect dedicated hardware-normalized numbers before publishing performance claims.
