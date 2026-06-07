# Deep Interview Transcript: GAEB next-step specs and PRDs

## Metadata
- Profile: standard
- Context type: brownfield
- Threshold: 0.20
- Final ambiguity: 0.16
- Context snapshot: `.omx/context/gaeb-next-step-specs-20260607T073634Z.md`

## Rounds

### Round 1 — Scope granularity
Question: For updating GitHub issues and creating specs/PRDs from the new GAEB source directory, what granularity should govern artifacts?
Answer: `hybrid`.
Interpretation: Use milestone-level grouping for orientation, but issue-level detail where implementation/test differences materially matter.

### Round 2 — Non-goals / decision boundaries
Question: Which boundaries must be preserved while updating issues, writing specs, and running ralplan?
Answer: `no-paid-actions`, `no-support-overclaiming`, `no-duplicate-issue-explosion`.
Interpretation: Keep BVBS paid certification gated, keep future/reference fixture honesty, update existing issues where possible.

### Round 3 — Pressure pass on PRD granularity
Question: Earlier answer said hybrid, latest text implied “PRDs per…”. Which rule governs PRD artifacts?
Answer: `prds-per-issue`.
Interpretation: Every affected issue gets its own PRD/test-spec artifact; milestone grouping is only navigational.
