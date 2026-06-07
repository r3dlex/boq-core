---
id: ARCH-001
domain: architecture
title: Loss-aware GAEB model before adapters
status: accepted
date: 2026-06-06
rules: true
files: ["**/*"]
tags: [gaeb, domain-model, adapters]
---

# ARCH-001: Loss-aware GAEB model before adapters

## Context

GAEB files contain phase-specific metadata and structures that may not fit Obra's current WBS/BOQ/line-item fields.

## Decision

`boq-core` parses into a loss-aware GAEB domain model first. Obra compatibility is provided through a separate adapter DTO with source provenance and loss reports.

## Consequences

- Parser modules must not map directly into Obra DTOs.
- Unsupported GAEB fields are preserved in metadata or loss reports.
- Adapter tests must snapshot loss/provenance data.
