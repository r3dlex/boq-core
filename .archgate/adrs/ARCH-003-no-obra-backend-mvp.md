---
id: ARCH-003
domain: architecture
title: No Obra backend integration in MVP
status: accepted
date: 2026-06-06
rules: true
files: ["**/*"]
tags: [obra, integration, boundary]
---

# ARCH-003: No Obra backend integration in MVP

## Context

The MVP establishes a Rust parser core and adapter DTO. Obra backend integration is valuable but explicitly out of scope.

## Decision

The MVP must not modify the Obra Elixir/Phoenix backend. It may emit DTOs compatible with Obra concepts.

## Consequences

- No changes under `../obra/backend` are required for MVP completion.
- Integration work must be planned as a later story.
