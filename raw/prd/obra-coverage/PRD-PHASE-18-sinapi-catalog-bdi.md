---
phase: PHASE-18
slug: sinapi-catalog-bdi
title: SINAPI catalog and BDI model
owner_repo: r3dlex/boq-core
dependencies: ["PHASE-13"]
labels: ["track:obra-coverage", "status:planning-ready", "support-honesty", "needs-local-ci", "needs-gh-ci", "needs-review-loop", "area:boq-core", "track:multi-standard", "standard:sinapi"]
status: planning-ready
---

# PRD PHASE-18: SINAPI catalog and BDI model

## Problem
Obra needs full, honest BoQ coverage through boq-core and the boq-service boundary. This phase solves one independently deliverable slice: Add a SINAPI-compatible catalog/BDI data model and adapter seams using only synthetic/minimal fixtures.

## Users
- Obra estimator or construction user importing or consuming BoQ data.
- Obra backend/frontend developer relying on stable parser/service contracts.
- boq-core maintainer protecting standards support claims and fixture evidence.

## Product outcome
A user or downstream system can rely on this phase's output without hidden support inflation, silent data loss, or ambiguous provenance.

## Functional requirements
- Expose a deterministic, documented behavior or contract for this phase.
- Return or persist support status before import/consumption where applicable.
- Return or persist loss findings and provenance for every transformed document where applicable.
- Fail closed for unsupported inputs.

## Quality requirements
- No broad architectural rewrites outside the phase boundary.
- Backward compatibility for existing supported fixtures unless explicitly changed by the phase plan.
- Reproducible local validation command documented in the implementation PR.
- GitHub CI and reviewer loop green/resolved before merge.

## Out of scope
- Paid BVBS certification execution
- External catalog/standards acquisition
- Service deployment
- Merging multiple phase implementations into one PR unless the issue explicitly requires it
