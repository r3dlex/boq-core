# AI SDLC workflow for boq-core

This repository follows the `init-ai-repo` v3 workflow in four phases:

1. Discover and decide repo topology, host, CI, tracker, and language packs.
2. Govern and plan with ADR/spec/plan/ticket traceability before implementation.
3. Configure and generate local command, policy, workflow, and validation surfaces.
4. Validate and hand off with local CI plus host CI evidence before merge.

## Merge gate

Merge only after architect, reviewer, and executor agree; all actionable comments
are resolved; local CI and GitHub CI are green; and branch policy permits merge.
Hosted branch/ruleset mutations are checklist-only unless explicitly confirmed.

## Boundary

- `obra` remains the only git submodule in `obra-workspace`.
- `boq-core` remains a sibling git repository.
- `boq-service` remains setup-managed and gitignored, not a submodule.
