# AI SDLC rules for boq-core

- Treat `main` as protected and deliver implementation through pull requests.
- Require an active spec or PRD, an implementation plan, and acceptance criteria before coding.
- Preserve architecture boundaries documented in `AGENTS.md`, `.rules.ts`, and ADRs.
- Do not mutate hosted branch protection, rulesets, project settings, or credentials without explicit confirmation.
- Do not store secrets in `.ai/`, `.memory/`, plans, tickets, drift reports, or generated logs.
- Prefer existing tools and repo-local commands; do not add dependencies unless explicitly requested.
- For AI-assisted code, work through `.ai/reviews/ai-failure-modes.md` before merge.
