# Notice: boq-core subrepo BRD mirror

The `WS-BRD-003` BRD is stored in the workspace (`../raw/brd/`). The copy in
this directory is mirrored for subrepo-local review; updates must be made in
the workspace first, then re-synced via `../setup.sh plans` (workspace
command).

The workspace is the source of truth for the binding; this directory is a
read-only mirror of `WS-BRD-003`. The `WS-` prefix means "workspace-level"
— this BRD binds the workspace (boq-core plus its future obra integration),
not boq-core alone.
