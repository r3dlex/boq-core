# Spec: X31 quantity takeoff domain model

## Issue
- GitHub issue: #28
- Branch: `issue-28-x31-domain-model`
- PRD: `.omx/plans/prd-issue-28-x31-domain-model.md`

## Scope
Define a serializable X31 quantity takeoff domain model that represents measurement/progress formula data separately from BoQ hierarchy and parser-specific item payloads.

## In scope
- Public `x31` module with domain structs for measurement documents, rows, formula concepts, baseline links, references, attachments, progress, metadata, and findings.
- REB-VB 23.003 represented as a formula system marker and source expression without evaluation.
- X31/X86 linkage represented through explicit baseline links and ordinal row references.
- Deterministic serialization using struct field order and `BTreeMap` metadata/variables.

## Out of scope
- X31 parser implementation or fixture promotion.
- REB-VB formula evaluation.
- X31/X86 matching algorithms beyond domain link representation.
- Paid BVBS certification/checker action.
- Obra backend coupling or AVA parser boundary changes.

## Functional requirements
1. The model separates measurement rows from `BoqItem` and the GAEB XML/GAEB90 parser internals.
2. Measurement rows can carry row id, linked BoQ ordinal, unit, formula, result quantity, progress, references, attachment ids, and metadata.
3. REB-VB 23.003 formula rows preserve expression text and deterministic variable maps without evaluating formulas.
4. Documents can link to X86 contract or X83 tender baselines via `MeasurementBaselineLink`.
5. Attachments can be represented as local assets or as recoverable findings when a reference-only artifact cannot be materialized.
6. The model roundtrips through serde JSON deterministically.

## Source inventory constraints
This issue remains bound to `.omx/specs/gaeb-ranked-source-status-ledger.md` rows R1-01 through R1-09. Manifest-backed X31/X86 rows remain `future_track` or `reference_only` until later issues add parser evidence and fixture checksums.
