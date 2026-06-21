---
id: ARCH-006
domain: architecture
title: GAEB XML 3.1 and 3.2 compatibility boundary
status: accepted
date: 2026-06-21
rules: false
files: ["src/gaeb_xml/**", "gaeb/manifest.toml", "tests/**"]
tags: [gaeb-xml, compatibility, support-policy]
---

# ARCH-006: GAEB XML 3.1 and 3.2 compatibility boundary

## Context

Issue #37 expands the roadmap from GAEB XML 3.3 toward GAEB XML 3.1/3.2
compatibility. Existing XML 3.1/3.2 sources are mixed: some are official schema
or documentation references, some are future BVBS certification fixtures, and
some are sample packages whose license/checksum state does not yet authorize CI
parser promotion. Treating those sources as if they were current XML 3.3 parser
support would violate ARCH-002 support-status honesty.

## Decision

The parser may detect XML 3.1/3.2 versions from explicit `<Version>` elements or
root namespaces and may emit structured compatibility findings for legacy XML.
That detection is metadata and review evidence only; it does not promote any XML
3.1/3.2 manifest row to parser support.

All XML 3.1/3.2 source rows remain `reference_only` or `future_track` until a
follow-up PR supplies license-safe local fixtures, checksums/lockfile entries,
failing tests, implementation, and review evidence in the same change.
Unsupported legacy constructs must be preserved as findings/metadata rather than
silently coerced into XML 3.3 semantics.

## Consequences

- GAEB XML 3.1/3.2 inputs can be identified and reported without claiming full
  parser compatibility.
- `gaeb/manifest.toml` remains the support-status source of truth.
- Parser changes for legacy versions must stay conservative and finding-backed
  until fixture-backed promotion is explicitly reviewed.
- Future XML 3.1/3.2 support work should extend issue #37 artifacts instead of
  creating duplicate issues unless a genuinely independent source family appears.
