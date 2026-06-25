# boq-core service analyze contract v1

`boq-core.service-analyze.v1` is the stable JSON boundary for service consumers such as `boq-service`.
It wraps existing parser APIs and reports parser evidence without promoting support beyond the parsed document.

## Contract files

- Schema: `docs/service-contract/service-analyze-v1.schema.json`
- Golden GAEB XML report: `tests/fixtures/service_contract/minimal_ava.analyze.json`
- Golden GAEB 90 report: `tests/fixtures/service_contract/minimal_d81.analyze.json`

## Required fields

Every report contains:

- `schema_version`: always `boq-core.service-analyze.v1`
- `crate_version`: producing `boq-core` crate version
- `status`: `ok` or `error`
- `document`: parsed summary, provenance, support status, capabilities, and top-level node count when status is `ok`
- `diagnostics`: recoverable parser/loss findings
- `error`: unrecoverable parser error when status is `error`
- `production_ready`: always `false`
- `certification_claims`: always empty

## Support-honesty rules

The contract is not a certification, production-readiness, deployment, validation, export, roundtrip, or full-support claim.
Callers must inspect `document.support_status` and `document.capabilities` before enabling downstream behavior.
