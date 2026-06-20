# Spec: Bauausführung X83 parser support

## Issue
- GitHub issue: #26
- Branch: `issue-26-bau-x83-parser`
- PRD: `.omx/plans/prd-issue-26-bauausfuehrung-x83-parser.md`

## Scope
Implement parser-level support for BVBS Bauausführung X83 request-for-quotation data in the existing GAEB XML parser boundary.

## In scope
- Parse X83 project metadata, GAEB version/phase, section hierarchy, item ordinals, quantities, units, and long text into the common `GaebDocument`/`Boq` model.
- Preserve unsupported X83 phase-specific item elements as deterministic node metadata.
- Emit non-fatal findings for unsupported X83 item elements so callers can audit loss without rejecting the document.
- Keep X83 capability-gated as `supported_parse_only`; do not claim Obra adapter, export, validation, or roundtrip support for cataloged BVBS X83 fixtures.

## Out of scope
- Paid BVBS checker execution or certification submission.
- Direct Obra backend integration.
- X84 offer/export support promotion.
- Schema-level validation of all X83 constructs.

## Functional requirements
1. `parse_str` with an X83 source URI must produce `summary.phase.code == "83"` and `support_status == supported_parse_only` for the BVBS Bau X83 fixture path.
2. The parser must retain X83 hierarchy: top-level `BoQCtgy` sections and child `Item` nodes retain ordinals/RNoPart metadata and sort order.
3. Item payloads must map `Qty`, `QU`, and `Description` into `BoqItem.quantity`, `BoqItem.unit`, `BoqItem.short_text`, and `BoqItem.long_text`.
4. Unknown non-empty item elements must be preserved under `BoqNode.metadata["gaeb.unsupported.<Element>"]` with text content when available, or `true` when the subtree has no text.
5. Unknown non-empty item elements must append a warning finding with code `gaeb_xml_unsupported_item_field` and an item-local location of `<ordinal>/<Element>`.
6. Empty unknown item elements keep the existing `gaeb.empty.<Element>` metadata behavior.
7. Obra adapter conversion remains denied for cataloged X83 parse-only documents.

## Non-functional requirements
- Keep the diff focused on parser behavior, tests, and issue artifacts.
- Maintain the existing manifest support vocabulary and ARCH-002/003/004 guardrails.
- Local quality gates and GitHub `Rust quality gates` must pass before merge.
