# PRD: mdBook user guide

## Planned issue
- Planned issue key: DOC-2
- Planned milestone: boq-core documentation MVP

## Product outcome
An mdBook user guide explains quickstart parsing, GAEB phases, supported formats, fixture usage, and BoQ output interpretation.

## Requirements
- [ ] Quickstart shows parse workflow without paid/network dependencies.
- [ ] GAEB phase/support table distinguishes supported/future/reference tracks.
- [ ] Output examples explain hierarchy, items, quantities, long text, and findings.

## Planned checks
- [ ] `mdbook build docs/book`
- [ ] `test_user_guide_links_supported_formats`
- [ ] `test_user_guide_warns_reference_only_sources`
