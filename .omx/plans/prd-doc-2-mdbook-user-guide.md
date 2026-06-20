# PRD: mdBook user guide

## Planned issue
- GitHub issue: #51
- Planned issue key: DOC-2
- Planned milestone: boq-core documentation MVP

## Product outcome
An mdBook user guide explains quickstart parsing, GAEB phases, supported formats, fixture usage, and BoQ output interpretation without paid/network dependencies or support overclaiming.

## Requirements
- [x] Quickstart shows parse workflow without paid/network dependencies.
- [x] GAEB phase/support table distinguishes supported/future/reference tracks.
- [x] Output examples explain hierarchy, items, quantities, long text, and findings.

## Implemented checks
- [x] `mdbook build docs/book`
- [x] `test_user_guide_links_supported_formats`
- [x] `test_user_guide_warns_reference_only_sources`
- [x] `test_user_guide_explains_boq_output_fields`

## Verification
- [x] `cargo test --test docs_mvp`
- [x] `mdbook build`
