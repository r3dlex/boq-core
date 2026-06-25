# `boq-core.obra-import.v1` service contract

`boq-core.obra-import.v1` is the service-facing JSON boundary for converting a
parsed GAEB document into Obra import DTOs. It is intentionally stricter than a
best-effort adapter call:

- `status: "ok"` includes a complete `import_document`.
- `status: "blocked"` means parsing succeeded, but the manifest-backed support
  status or capability flags do not allow Obra adapter conversion.
- `status: "error"` means parsing failed before adapter conversion could be
  evaluated.

## Stable adapter rejection codes

Blocked conversions use stable rejection codes:

- `obra_adapter_supported_parse_only`
- `obra_adapter_future_track`
- `obra_adapter_reference_only`
- `obra_adapter_not_supported`

The contract preserves the exact support vocabulary from the support manifest:
`supported`, `supported_parse_only`, `future_track`, and `reference_only`.

## DTO semantics

The `import_document` payload uses the public `adapter::obra` DTO:

- `boq.deterministic_key`, every `wbs_nodes[*].deterministic_key`, and every
  `line_items[*].deterministic_key` are deterministic import keys.
- `wbs_nodes` are Obra WBS candidates with `parent_key`, `code`, `title`,
  `level`, `path`, `sort_order`, and `node_type`.
- `line_items` carry item text, quantity, unit, unit price, total price, notes,
  and source metadata.
- `classifications` always include GAEB ordinal evidence for line items and may
  include classification/catalog/quantity overlays only when the parsed item has
  explicit annotation evidence.
- `loss_report.warnings` carries parser diagnostics and adapter annotation
  diagnostics; `unsupported_fields` and `lossy_mappings` remain explicit arrays.

## Support-honesty invariants

- `production_ready`: always `false`.
- `certification_claims`: always empty.
- No blocked conversion emits a partial-success `import_document`.
- No paid/external standards data, proprietary catalog download, credentials,
  production deployment, certification, complete-market-coverage, export, or
  roundtrip claim is made by this contract.

## Artifacts

- Schema: `docs/service-contract/obra-import-v1.schema.json`
- Supported golden: `tests/fixtures/service_contract/bvbs_ava_x81.obra_import.json`
- Blocked golden: `tests/fixtures/service_contract/minimal_d81.obra_import.json`
