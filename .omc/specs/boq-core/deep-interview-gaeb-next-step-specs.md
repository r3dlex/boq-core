# Deep Interview Spec: GAEB next-step issue/spec/PRD update

## Metadata
- Source workflow: deep-interview
- Context type: brownfield
- Final ambiguity: 0.16
- Threshold: 0.20
- Context snapshot: `.omx/context/gaeb-next-step-specs-20260607T073634Z.md`
- Transcript: `.omx/interviews/gaeb-next-step-specs-<timestamp>.md`

## Intent
Turn the supplied documentation and example-file directory into execution-ready roadmap artifacts for `boq-core`, without prematurely implementing parser code or overclaiming unsupported GAEB areas.

## Desired outcome
- Existing roadmap issues are updated with the newly supplied source directories, schema packages, examples, and acceptance implications.
- Specs are created for each affected next-step issue.
- Per-issue PRD and test-spec artifacts are created via the ralplan planning handoff pattern.

## In scope
- Affected issues: GAEB XML 3.1/3.2 compatibility, GAEB XML 3.4 beta tracking, GAEB 2000/Pxx, GAEB 90 compatibility where relevant, X50-X52 costing, X93-X97 Handel, Zeitvertrag, and spreadsheet roundtrip helpers.
- Issue body updates and/or comments that add source references, fixture/test expectations, and planning constraints.
- `.omx/specs/` per-issue specs.
- `.omx/plans/` per-issue PRD and test-spec artifacts.

## Out of scope / non-goals
- No paid actions: do not submit to BVBS, pay fees, enter credentials, or claim official certification.
- No support overclaiming: future/reference fixtures remain future-track/reference-only until failing tests and implementation promote them.
- No duplicate issue explosion: update existing issues where possible; create new issues only if a supplied source family is genuinely missing.

## Decision boundaries
- OMX may update existing issue bodies/comments and create planning artifacts.
- OMX may normalize Google-search-wrapped URLs to direct source URLs when obvious.
- OMX may mark sources as reference-only if they are executable, beta, commercial/demo, license-unclear, or not yet covered by parser implementation.
- OMX must not perform external paid or production actions.

## Acceptance criteria
- Every affected issue has an updated source/reference section.
- Every affected issue has a per-issue spec under `.omx/specs/`.
- Every affected issue has a PRD and test-spec under `.omx/plans/`.
- PRDs preserve support-status honesty and explicitly list non-goals.
- Ralplan consensus evidence is recorded as a planning handoff summary.

## Pressure-pass finding
The initial “hybrid” answer was refined by a pressure pass into “PRDs per issue.” Final rule: organize by milestone for navigation, but create per-issue PRD/test-spec artifacts for affected issues.

## Residual risks
- Some source URLs are documentation/reference resources with uncertain redistribution terms; specs must treat them as acquisition/reference inputs, not committed payloads unless license-safe.
- GAEB XML 3.4 remains beta; no BVBS certification files exist for it.
- Spreadsheet helpers may belong in a companion crate rather than parser core.
