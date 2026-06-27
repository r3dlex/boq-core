# ISSUE-142: Non-paid GAEB fixture and service-contract expansion

GitHub: https://github.com/r3dlex/boq-core/issues/142

## Scope

Expand service-contract support evidence with repository-local non-paid fixtures
before any paid/official certification milestone.

## Acceptance mapping

- `gaeb/manifest.toml` classifies two non-paid synthetic rows:
  - `non_paid_synthetic_gaeb90_d81` as `supported_parse_only`
  - `non_paid_synthetic_gaeb_xml_x81` as `supported`
- Both rows point at exact repository-local fixture paths under `gaeb/non_paid/synthetic/**` and use `ci_policy = "repository_fixture"`.
- Neither row has archive checksums or external download requirements.
- `tests/service_support_manifest.rs` proves the rows are exported to services
  with stable capabilities, and that service analysis of the exact target fixture
  paths matches the exported capabilities with no production/certification claims.
- `tests/fixtures/service_contract/support_manifest.capabilities.json` is
  updated from the service CLI.
