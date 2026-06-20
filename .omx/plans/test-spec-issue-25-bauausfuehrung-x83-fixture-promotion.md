# Test spec issue #25: Bauausführung X83 fixture promotion

## Required tests
- `test_bvbs_bau_x83_manifest_status_is_future_until_green`
  - Locks the promotion boundary: XML 3.3 Bau X83 is `supported_parse_only` only after local evidence exists; X84 remains `future_track`.
- `test_bau_x83_fixture_parses_to_boq_tree`
  - Parses the Bau X83 fixture path into hierarchy/item/quantity/text evidence.
- `test_bau_x83_support_promotion_requires_evidence`
  - Ensures manifest test mappings and criteria matrix evidence point to automated readiness tests and do not claim certification.
- `test_bau_x83_golden_report_matches`
  - Verifies a deterministic golden readiness report exists and uses readiness/support vocabulary without certification overclaiming.

## Required gates
- `cargo test --test bau_roundtrip`
- `cargo test --test testing_strategy`
- `cargo test manifest_keeps_supported_parse_only_bau_x83_and_future_x84`
- Full local CI before PR and GH CI before merge.
