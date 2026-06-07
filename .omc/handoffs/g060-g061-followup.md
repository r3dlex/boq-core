# G060 + G061 follow-up — docs MVP and PRD links on #15-#36

**Date:** 2026-06-07
**Branch:** `feat/g060-g061-prd-links-and-docs-issues`
**Aggregate ultragoal plan:** `boq-core-issues-15-36-docs`
**Related ultragoal goals:** G060, G061, G064 (resolution anchor)

## Scope

Closes the G060 (docs milestone + tracking issues) and G061 (PRD links
+ ARCH guardrails on existing issues) follow-up identified as
"deferred to execution lane" in the G064 final quality gate.

## What was done

### G060 — milestone + 5 docs issues

1. Created milestone **`boq-core documentation MVP`** (#9) in boq-core.
2. Created 5 issues in that milestone, each implementing the corresponding
   PRD under `.omx/plans/prd-doc-{1..5}-*.md`:

   | Issue | Title | PRD |
   |---|---|---|
   | #50 | DOC-1: Rustdoc API reference (docs MVP) | `prd-doc-1-rustdoc-api-reference.md` |
   | #51 | DOC-2: mdBook user guide (docs MVP) | `prd-doc-2-mdbook-user-guide.md` |
   | #52 | DOC-3: mdBook developer guide (docs MVP) | `prd-doc-3-mdbook-developer-guide.md` |
   | #53 | DOC-4: Certification evidence guide (docs MVP) | `prd-doc-4-certification-evidence-guide.md` |
   | #54 | DOC-5: Release and publishing guide (docs MVP) | `prd-doc-5-release-publishing-guide.md` |

   All 5 issues were assigned the `docs-mvp` label (newly created) plus
   `documentation` and a type label (`type:docs` / `type:api` /
   `type:certification` / `type:tooling`).

### G061 — PRD links + ARCH guardrails on #15-#36

For each open issue #15 through #36, appended a "Planning layer
(G061 follow-up)" section to the body containing:

- Direct link to the per-issue PRD in
  `boq-core/.omx/plans/prd-issue-NN-*.md` (with mirror at
  `.omc/plans/boq-core/prd-issue-NN-*.md`).
- ARCH-002 (manifest vocabulary) guardrails.
- ARCH-004 (paid-certification gate) guardrails.
- ARCH-003 (no Obra backend coupling) guardrails.
- A tracking line pointing back to the aggregate ultragoal plan.

22/22 issues updated; idempotent (the script skips issues that already
have the marker).

### Local mirror work (this commit)

Also mirrored the boq-core `.omx/` planning layer into `.omc/` so the
subrepo carries the same `.omc/{plans,specs,context,interviews,handoffs,drafts}/`
shape as obra and the workspace:

- `.omc/plans/boq-core/` — 46 files (PRDs, test specs, ralplan consensus/draft)
- `.omc/specs/boq-core/` — 10 files (deep-interview handoffs + issue specs)
- `.omc/context/boq-core/` — 2 files
- `.omc/interviews/boq-core/` — 2 files (1 md + 1 rounds.json)
- `.omc/handoffs/README.md`, `.omc/drafts/README.md`, `.omc/README.md` (new)

Total: 63 tracked files. Runtime state paths
(`.omc/state/`, `.omc/sessions/`, `.omc/project-memory.json`) remain
gitignored as before.

## Verification

- `gh issue list --repo r3dlex/boq-core --state all` shows
  - milestone #9 `boq-core documentation MVP` (0/0)
  - issues #50..#54 OPEN
  - issues #15..#36 OPEN, each with `G061 follow-up` marker (count=2)
- boq-core worktree is clean; mirror commit on
  `feat/g060-g061-prd-links-and-docs-issues` is the only commit
  ahead of `origin/main`.

## Out of scope (intentionally)

- Implementation of DOC-1..DOC-5 content (covered by future execution PRs).
- Implementation of #15-#36 content (covered by per-issue execution lanes).
- Mirror of the workspace `.omx/plans/ralplan-consensus-obra-ai-sdlc-regression-fixes.md`
  file (that artifact is obra-related and lives in the obra repo / workspace
  mirror, not boq-core's source-of-truth `.omx/plans/`).
- The user-approved G030-G063 display-only mutation (preserved as
  `displayMutation` metadata; not modified by this follow-up).
