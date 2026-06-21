# ARCH-007: GAEB XML 3.4 beta impact boundary

## Status
Accepted

## Context
Issue #38 tracks GAEB XML 3.4 beta schema and changelog impact. The beta package is useful for forward-looking model analysis, but it is not production support evidence and it is not a BVBS certification fixture.

The current manifest catalogs two official GAEB reference sources:

- `official_gaeb_xml34_beta_schema`
- `official_gaeb_xml34_beta_changelog`

Both are `reference_only` and `download_on_demand`.

## Decision
`boq-core` may detect a GAEB XML 3.4 namespace/version and preserve the version as `3.4`, but parser support remains `ReferenceOnly` for manifest-backed beta sources. The parser must emit a structured beta-reference finding when a 3.4 document is parsed so callers can distinguish beta impact tracking from production support.

The beta impact model is documented in `docs/fixtures/gaeb-xml34-beta-impact.md`. The tracked extension points are:

- sustainability descriptors,
- lifecycle and maintenance descriptors,
- carbon / CO2 descriptor metadata,
- schema/changelog deltas that may affect future BoQ item metadata.

## Boundaries
- No production support claim for GAEB XML 3.4 beta.
- No BVBS certification readiness claim for GAEB XML 3.4 beta.
- No automatic support promotion from a beta schema/changelog row.
- No external download, browser, paid action, or executable action in CI.
- Future support requires a follow-up issue or updated #38 scope with stable schema evidence, failing tests, fixture checksums, implementation, and review evidence.

## Consequences
Tests must assert the manifest rows stay `reference_only`, parser policy returns `SupportStatus::ReferenceOnly` for 3.4 beta manifest paths, and docs preserve the beta-impact boundary without claiming GAEB XML 3.4 production support.
