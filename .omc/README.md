# boq-core .omc/ — local mirror of OMX planning layer

This directory mirrors the canonical planning layer that lives under
`.omx/` in this repo. It exists so that the boq-core subrepo carries
the same `.omc/` structure as the obra subrepo, the workspace, and
the cross-repo sync script (`setup.sh plans`).

## Layout

```
.omc/
├── plans/boq-core/         # mirror of .omx/plans/   (PRDs, ralplans, test specs)
├── specs/boq-core/         # mirror of .omx/specs/   (deep-interview handoffs, issue specs)
├── context/boq-core/       # mirror of .omx/context/ (consensus context snapshots)
├── interviews/boq-core/    # mirror of .omx/interviews/ (multi-round transcripts)
├── handoffs/               # cross-team handoffs (architect / critic / team plans)
├── drafts/                 # WIP drafts not yet ready for plans/ or specs/
├── state/                  # runtime state (gitignored)
├── sessions/               # session metadata (gitignored)
└── project-memory.json     # session memory (gitignored)
```

## Source of truth

Canonical content lives in `.omx/`. Mirroring into `.omc/` is done by
the `sync_subdir` helper in `setup.sh` at the workspace level; for
local symmetry, a one-shot `cp -r` is also acceptable as long as both
copies stay byte-identical.

## Gitignore

The repo `.gitignore` excludes the runtime state paths
(`.omc/state/`, `.omc/sessions/`, `.omc/project-memory.json`) and the
OMX runtime artifacts (`.omx/{artifacts,cache,logs,runtime,state}/`).
All planning content under `.omc/{plans,specs,context,interviews,handoffs,drafts}/`
is intentionally tracked.
