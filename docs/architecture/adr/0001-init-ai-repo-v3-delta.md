# ADR 0001: Adopt init-ai-repo v3 delta scaffold

Date: 2026-06-30

## Status

Accepted

## Context

boq-core already had brownfield AI-SDLC governance surfaces. The v3 scaffold adds `.ai/`, `.memory/`, workflow, traceability, eval, MCP/A2A, and review checklist surfaces without deleting legacy artifacts.

## Decision

Adopt the v3 delta scaffold in checklist-only hosted mode. Preserve existing branch policy, CI, ADRs, and repo-specific operating instructions.

## Consequences

Future work can rely on deterministic local validation and traceability artifacts. Hosted policy changes still require explicit confirmation.
