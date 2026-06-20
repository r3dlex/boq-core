# PRD: Add performance benchmarks for large BoQs

## Issue
- GitHub issue: #23
- Milestone: v0.3 Public API and parser robustness

## Product outcome
Reproducible, bounded smoke benchmarks measure large BoQ parsing and adapter conversion latency with documented machine context, while staying separated from heavyweight release benchmarking.

## Source/status anchors
- Synthetic large BoQ: deterministic generated AVA X81 fixture in `tests/large_boq_benchmarks.rs`.
- Bench harness: advisory performance evidence, not certification or support-promotion evidence.
- Baseline docs: `docs/benchmarks/large-boq-baseline.md`.

## Requirements
- [x] Create deterministic large-BoQ fixture generator.
  - `generate_large_gaeb_xml` produces a stable 250-item synthetic AVA X81 smoke fixture.
- [x] Establish advisory budgets after baseline measurement.
  - Parse and adapter-conversion smoke tests have 2 second CI budgets.
- [x] Keep performance claims tied to machine/context.
  - Baseline documentation records machine context, command shape, fixture scope, smoke budgets, and advisory-only caveat.

## Implemented tests/benches
- [x] `test_large_boq_fixture_generator_is_deterministic`
- [x] `bench_parse_large_gaeb_xml_under_budget`
- [x] `bench_adapter_conversion_under_budget`
- [x] `test_benchmark_docs_include_machine_context`
- [x] `benchmark_harness_includes_license_safe_fixture_path`

## Verification
- [x] `cargo test --test large_boq_benchmarks -- --nocapture`
