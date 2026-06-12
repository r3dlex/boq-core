---
id: ARCH-005
domain: architecture
title: Support policy and fixture manifest have a single seat
status: accepted
date: 2026-06-12
rules: true
files: ["**/*"]
tags: [support-policy, fixtures, ports-and-adapters]
---

# ARCH-005: Support policy and fixture manifest have a single seat

## Context

Before this decision, "what does this crate honestly claim to support?"
(ARCH-002) was decided in four places: `src/gaeb_xml/mod.rs` (private
`SupportPolicy` struct, embedded TOML, registry, mapping helpers),
`src/gaeb90.rs` (hardcoded parse-only), `src/bin/xtask.rs` (duplicate
`Manifest`/`Fixture` types and bespoke `verify_manifest` rules), and
`tests/testing_strategy.rs` (a third manifest struct duplicate). Adding a
new GAEB format or overlay would have required updating each site
independently.

## Decision

Support claims have a single producer: the `boq_core::support::SupportPolicy`
trait. Two day-one adapters live behind it — `ManifestPolicy` (embedded
manifest-backed, conservative-default on miss) and `LayeredPolicy` (a base +
downgrade-only overlay). `boq_core::support::default_policy()` returns the
embedded policy, optionally layered with an overlay loaded from
`BOQ_CORE_SUPPORT_OVERLAY`; load failure degrades to the embedded policy and
never panics.

The Fixture Manifest schema has one typed owner: `boq_core::support::manifest`.
The library, xtask, and tests consume `FixtureManifest`/`FixtureEntry` from
that module via `manifest::parse` and `manifest::validate`. xtask retains
filesystem, lockfile, and checksum bookkeeping but no longer carries its own
copy of the schema or its parser-relevant string-level checks.

Overlays may only downgrade (ARCH-002 honesty). Status ordering, most to
least conservative, is `ReferenceOnly < FutureTrack < SupportedParseOnly <
Supported`; capability fields are merged AND-wise except `reference_only`
which is merged OR-wise. An overlay can never upgrade the base decision.

## Consequences

- Parser modules import `SupportPolicy`/`SupportQuery` and call
  `default_policy().decide(...)` instead of constructing
  `SupportStatus::Supported` / `SupportCapabilities::supported_import` inline.
- `include_str!("../../gaeb/manifest.toml")` may appear only inside
  `src/support/`. xtask and tests load the manifest through
  `boq_core::support::manifest::parse`.
- Manifest behaviour is preserved field-for-field: the
  `supported`+`process_domain == "ava"` rule still promotes only AVA XML
  rows. `ManifestPolicy::decide` guards on `GaebFormat::GaebXml` so no
  manifest row can promote a non-XML format (behaviour preservation for
  GAEB 90).
- Overlay support is opt-in via `BOQ_CORE_SUPPORT_OVERLAY`. Failure to load
  the overlay degrades silently to the embedded policy.
