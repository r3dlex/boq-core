# Certification Evidence Guide

This guide explains how `boq-core` handles BVBS fixtures, GAEBXmlChecker references, GAEB source material, and golden reports as evidence. All checker output, fixture parsing, criteria matrices, and generated reports are readiness evidence only. They help reviewers understand whether the crate is prepared for a later external process; they do not create official approval or paid certification status.

## Evidence is not certification

BVBS fixture parsing, GAEBXmlChecker references, criteria matrices, and golden reports are useful for certification-path readiness. They are not a paid certification result, official approval, or a substitute for the external BVBS process.

The project must not claim an official result unless an authorized human completes the paid external submission and records the official outcome. Until that happens, use these terms:

- certification-path readiness;
- readiness evidence;
- parser-readiness evidence;
- manual evidence gate;
- external submission pending human authorization.

Do not use wording that implies external approval has already happened.

## Paid-action gate

`ARCH-004` governs external certification actions. It creates a hard gate for paid, credentialed, or externally submitted workflows:

- No paid submission occurs automatically.
- No credential entry occurs automatically.
- No external BVBS portal/tool submission occurs automatically.
- No official-result claim is made from local fixtures, local CI, GH CI, or GAEBXmlChecker output alone.
- Explicit human authorization is required before any paid or credentialed external action.

Issue #18 is the runbook track for preparing that authorization path. It may describe the evidence package and manual decision points, but it is not itself execution authorization.

## Source-of-truth controls

Use these controls when preparing or reviewing certification-related work:

| Control | Source | Rule |
| --- | --- | --- |
| Fixture catalog | `gaeb/manifest.toml` | Every source has `support_status`, `ci_policy`, `license_note`, and `test_mapping` metadata. |
| Manifest vocabulary | `supported`, `supported_parse_only`, `future_track`, `reference_only` | Do not introduce ad-hoc statuses for tooling or certification fixtures. |
| Fixture honesty | `ARCH-002` | Cataloging a source does not imply parser support. |
| Paid gate | `ARCH-004` | Paid submission and official-result wording require explicit human authorization. |
| Obra boundary | `ARCH-003` | Certification evidence does not require sibling ERP server changes. |
| Local evidence | tests, mdBook, Archgate, Prek, coverage when code changes | Local and GH CI are readiness gates, not external approval. |

## Evidence workflow

For each certification-path PR:

1. Confirm the `gaeb/manifest.toml` fixture entry, source URL, support status, CI policy, and license note.
2. Confirm the PRD/plan names the exact evidence scope and the no-paid-action boundary.
3. Download or resolve fixtures only according to the manifest CI policy.
4. Record checksums and fixture provenance when payloads are used.
5. Run parser, adapter, docs, or manifest tests against synthetic and real examples as scoped.
6. Produce a criteria matrix, golden report, or external-checker evidence document when the PRD requires it.
7. Document findings and gaps without claiming official approval.
8. Require human authorization before any external paid action, credentialed run, or submission package upload.

## Evidence-track matrix

| Track | Current purpose | Evidence artifacts | Support boundary |
| --- | --- | --- | --- |
| AVA | Tendering/award/billing certification-path readiness. | `docs/fixtures/gaebxmlchecker-ava-evidence.md`, `docs/fixtures/bvbs-ava-criteria-readiness.md`, `docs/fixtures/bvbs-ava-golden-reports.md`, `docs/fixtures/ava-rich-text-schema-version.md`, `gaeb/criteria/bvbs_ava_matrix.toml`, `gaeb/golden/bvbs_ava/`. | AVA fixture entries may be `supported` only where tests justify the named flow; external certification remains gated. |
| Bauausführung | Construction-execution parser-readiness evidence. | `docs/fixtures/bvbs-bau-x83-readiness.md`, `gaeb/golden/bvbs_bau/`, Bau X83/X84 tests. | `supported_parse_only` readiness; no adapter/export/roundtrip promotion unless separately proven. |
| Mengenermittlung | X31 quantity-takeoff model/parser-readiness evidence. | `src/x31.rs`, `tests/x31_domain.rs`, X31 PRDs/test specs for #28-#31. | Quantity evidence is not BoQ adapter support and not XRechnung generation. |
| Texterstellung | Rich-text/table specification-authoring readiness. | `docs/fixtures/bvbs-texterstellung-criteria-readiness.md`, `tests/texterstellung.rs`, `tests/texterstellung_criteria.rs`. | `supported_parse_only` rich-text evidence; layout/rendering and certification remain manual/gated. |
| Rechnung / XRechnung | Invoice model and bridge planning. | `docs/fixtures/x89-rechnung-model-design.md`, `docs/fixtures/xrechnung-bridge-plan.md`, `tests/x89_domain.rs`, `tests/x89_parser.rs`, `tests/xrechnung_bridge_plan.rs`. | X89 model/parser evidence is not XRechnung envelope generation. |
| Compatibility expansion | Version and legacy compatibility boundaries. | `docs/fixtures/gaeb-xml34-beta-impact.md`, `docs/fixtures/gaeb2000-pxx-compatibility-plan.md`, `docs/fixtures/gaeb90-adapter-gap-matrix.md`. | Compatibility sources remain boundary-gated until manifest status and tests promote them. |
| Non-certification exchange tracks | Costing, trade, framework-contract, and spreadsheet exchange boundaries. | `docs/fixtures/kosten-kalkulation-x50-x52-boundary.md`, `docs/fixtures/handel-x93-x97-boundary.md`, `docs/fixtures/zeitvertrag-x83z-x84z-boundary.md`, `docs/fixtures/spreadsheet-roundtrip-boundary.md`. | These are not BVBS certification tracks and must not be represented as certification evidence. |
| GAEBXmlChecker | External tool comparison signal for AVA evidence packages. | `docs/fixtures/gaebxmlchecker-ava-evidence.md`, `tests/gaebxmlchecker_ava.rs`. | `reference_only` unless a later governed policy explicitly changes invocation and support semantics. |

## GAEBXmlChecker policy

GAEBXmlChecker is cataloged as `reference_only` in `gaeb/manifest.toml`. It may be used as an external comparison signal only after invocation policy, platform constraints, checksums, and licensing are governed.

Do not change GAEBXmlChecker into a parser dependency or support claim without a manifest vocabulary decision, regression tests, and the paid-action gate review above.

## No-overclaiming checklist

Before merging certification-related docs or code, verify that the PR does not say or imply:

- external BVBS approval is complete;
- paid certification has been completed;
- `reference_only` tools are supported parser functionality;
- `future_track` phases are production-supported;
- GAEB XML 3.4 beta is production-supported;
- non-certification exchange tracks are BVBS evidence;
- local CI, GH CI, or golden reports can replace the external paid process.

Use the positive wording from this guide instead: readiness evidence, certification-path readiness, manual gate, and explicit human authorization.
