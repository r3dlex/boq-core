---
id: ARCH-004
domain: architecture
title: Official BVBS certification actions are gated
status: accepted
date: 2026-06-06
rules: true
files: ["**/*"]
tags: [certification, external-actions]
---

# ARCH-004: Official BVBS certification actions are gated

## Context

The user wants BVBS certification-path readiness, but paid submission and official claims are external actions.

## Decision

`boq-core` may prepare certification evidence, but paid BVBS submission and official certification representation require explicit user confirmation.

## Consequences

- CI and docs must say certification-path readiness unless official certification is actually obtained.
- No paid or credentialed external action occurs automatically.
