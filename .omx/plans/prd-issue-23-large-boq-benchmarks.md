# PRD: Add performance benchmarks for large BoQs

## Issue
- GitHub issue: #23
- Milestone: v0.3 Public API and parser robustness

## Product outcome
Reproducible benchmarks measure large BoQ parsing and adapter conversion latency/memory with documented machine context.

## Source/status anchors
- Synthetic large BoQ: generated fixture.
- Bench harness: advisory performance evidence.

## Requirements
- [ ] Create deterministic large-BoQ fixture generator.
- [ ] Establish advisory budgets after baseline measurement.
- [ ] Keep performance claims tied to machine/context.

## Planned tests/benches
- [ ] `test_large_boq_fixture_generator_is_deterministic`
- [ ] `bench_parse_large_gaeb_xml_under_budget`
- [ ] `bench_adapter_conversion_under_budget`
- [ ] `test_benchmark_docs_include_machine_context`
