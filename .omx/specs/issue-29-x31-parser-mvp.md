# Spec: X31 parser MVP

## Issue
- GitHub issue: #29
- Branch: `issue-29-x31-parser-mvp`
- PRD: `.omx/plans/prd-issue-29-x31-parser-mvp.md`

## Scope
Implement a conservative GAEB XML X31 parser MVP that maps quantity-takeoff measurement groups and formula records into the `x31` domain model introduced by #28.

## In scope
- `x31::parse_str` and `x31::parse_file` entrypoints for XML X31 payloads.
- Measurement group id preservation in row metadata.
- Formula source text, result quantity, unit, linked BoQ ordinal, and row id extraction.
- Attachment detection as domain attachment assets and row attachment ids.
- Unsupported row/container constructs reported as recoverable findings.
- BVBS X31 manifest promotion to `supported_parse_only` for parser-readiness evidence only.

## Out of scope
- REB-VB 23.003 formula evaluation.
- X31/X86 result linking beyond ordinal/baseline-ready fields.
- X31 export/roundtrip/schema validation/Obra adapter support.
- Live BVBS downloads or paid certification/checker actions.

## Functional requirements
1. X31 XML parser entrypoints return a `QuantityTakeoffDocument` with provenance, version, rows, attachments, findings, and metadata.
2. Formula records preserve source expression text without evaluation.
3. Result quantities parse with decimal-comma and decimal-point tolerance.
4. Unsupported constructs become `x31_unsupported_feature` findings rather than panics.
5. The `bvbs_xml33_qty_x31` fixture manifest row is `supported_parse_only` with evidence test mappings.

## Source inventory constraints
Rows R1-02 through R1-06 remain governed by `.omx/specs/gaeb-ranked-source-status-ledger.md`. X86 and reference-only rows are not promoted by this issue.
