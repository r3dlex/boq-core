---
id: ARCH-002
domain: architecture
title: Fixture support status honesty
status: accepted
date: 2026-06-06
rules: true
files: ["**/*"]
tags: [fixtures, certification, testing]
---

# ARCH-002: Fixture support status honesty

## Context

The project uses official and third-party GAEB files across many phases. Cataloging a fixture must not imply parser support.

## Decision

Every fixture has an explicit support status: `supported`, `supported_parse_only`, `future_track`, or `reference_only`. Tests must fail if a `supported` fixture lacks test coverage or if a `reference_only` executable is run in CI.

## Consequences

- Support claims are encoded in manifest data and tests.
- GAEB XML 3.4 beta remains reference-only until explicitly promoted.
- GAEBXmlChecker is tooling/reference unless a policy allows invocation.
