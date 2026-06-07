# boq-core handoffs

This directory holds cross-team handoffs and reviewer briefs for the
boq-core subrepo. It is a tracked mirror of the same artifacts that
live under `.omx/artifacts/` (where reviewer outputs land first) and
that the workspace mirrors to `.omc/handoffs/boq-core/`.

## Conventions

- One handoff per topic (architect brief, critic brief, team plan, etc.)
- Filenames: `<role>-<topic>.md` (e.g. `architect-brief.md`, `critic-brief.md`).
- Each handoff must include: date, scope, decision summary, follow-ups.
- Do not store secrets, paid certification data, or non-public AVA test
  material here.

## Current contents

(none yet — populated on demand)

## Cross-repo mirrors

- Workspace: `.omc/handoffs/boq-core/` (via `setup.sh plans`)
- Source of truth: `.omx/artifacts/` (transient reviewer outputs that
  are durable enough to track are promoted here)
