---
id: BOQ-PRD-003
title: GAEB XML 3.1 and 3.2 compatibility track (issue #37)
status: pending
date: 2026-06-07
tags: [boq-core, gaeb, xml-3-1, xml-3-2, compatibility]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
issue_source: .omx/plans/prd-issue-37-gaeb-xml-31-32-compatibility.md
spec_source: .omx/specs/issue-37-gaeb-xml-31-32-compatibility.md
test_spec_source: .omx/plans/test-spec-issue-37-gaeb-xml-31-32-compatibility.md
milestone: v0.8 Format compatibility expansion
tracked_by: r3dlex/boq-core#37 (planning)
subrepo_binding:
  boq-core: boq-core/gaeb/manifest.toml (entries to add), boq-core/src/gaeb_xml/ (parser changes pending the ADR)
  workspace: docs/adr/WS-008-support-status-honesty-and-certification-boundary.md
acceptance_criteria:
  - Per-source matrix is preserved in future implementation PRs
  - The first architecture decision (compatibility ADR documenting XML 3.1/3.2 namespace/schema differences vs. the current XML 3.3 AVA parser) is completed before parser changes
  - Concrete test names from the test spec are created: test_manifest_catalogs_gaeb_xml31_and_xml32_sources, test_xml_version_detector_distinguishes_31_32_33_namespaces, test_xml32_ava_fixtures_remain_future_track_until_parser_promotion, test_xml31_schema_sources_remain_reference_only, test_unsupported_legacy_xml_features_emit_structured_findings
  - Protected-main gates remain green
  - support_status promotion only in the same PR as passing implementation tests and review evidence
---

# BOQ-PRD-003: GAEB XML 3.1 and 3.2 compatibility (issue #37)

Planning-only track. Per-source support matrix:

- `gaeb_xml32_doc` (reference_only, PDF)
- `gaeb_xml32_lv_schema` (reference_only, schema package)
- `bvbs_xml32_ava_x81/x84/x86` (future_track, BVBS fixtures)
- `bvbs_xml32_bau_x83` (future_track)
- `gaeb_xml31_doc` (reference_only)
- `gaeb_xml31_muster` (future_track, 2009-12 Musterdateien)
- `gaeb_xml31_x81_x87_schema` (reference_only)

Follow-up issue policy: update #37 unless a genuinely new source family appears.
