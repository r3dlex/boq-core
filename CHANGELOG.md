# Changelog

All notable changes to `boq-core` are recorded here.

## Unreleased

### Release automation

- Add release dry-run checks for package archive creation and publish safety.
- Document the pre-1.0 semver promise, release channels, changelog expectations, and manual publish gate.

### Support status changes

- Add X31 canonical quantity evidence for selected parser-backed paths as `supported_parse_only`; evidence is synthetic/parser-level and does not claim BVBS fixture conformance, Obra adapter DTO output, export, billing/XRechnung generation, full REB formula conformance, roundtrip, production support, or certification.
- Future support-status changes must name the affected exchange formats, fixture evidence, tests or gates, and certification limitations.
