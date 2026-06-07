---
id: BOQ-PRD-001
title: GAEB parser and certification harness (master PRD)
status: accepted
date: 2026-06-07
tags: [boq-core, gaeb, parser, certification]
brd_link: ../../raw/brd/WS-BRD-003-boq-core-gaeb-parser.md
prd_source: .omx/plans/prd-boq-core-gaeb-parser-20260606.md (PRD §1–§5)
test_spec_source: .omx/plans/test-spec-boq-core-gaeb-parser-20260606.md
ralplan_source: .omx/plans/ralplan-handoff-boq-core-gaeb-parser-20260606.json
ultragoal_source: .omx/plans/ultragoal-brief-boq-core-gaeb-parser-20260606.md
ultraqa_source: boq-core/docs/ultraqa-report.md
tracked_by: r3dlex/boq-core issues #1 (closed), #2 (closed), #3 (closed), #4 (closed), #5 (closed), #6 (closed), #7 (open, gated)
subrepo_binding:
  boq-core: boq-core/Cargo.toml, boq-core/src/lib.rs, boq-core/src/model.rs, boq-core/src/adapter/obra.rs, boq-core/src/gaeb_xml/mod.rs, boq-core/src/gaeb90.rs, boq-core/src/format.rs, boq-core/src/support.rs, boq-core/src/error.rs, boq-core/gaeb/manifest.toml, boq-core/gaeb/fixtures.lock
  workspace: docs/adr/WS-008-support-status-honesty-and-certification-boundary.md
acceptance_criteria:
  - G001: Repository quality foundation — prek, fmt, clippy, tests, coverage threshold plan, archgate, CI, branch protection documentation, no Obra backend changes (issue #1 / PR #9)
  - G002: Domain model and Obra adapter contract — loss-aware GAEB model, deterministic Obra DTO mapper, provenance, rich text representation, decimal quantities/prices, loss reporting, support-status metadata (issue #2 / PR #10)
  - G003: Fixture manifest and acquisition pipeline — gaeb/ tree, manifest/lockfile schema, safe downloader/unpacker/verifier, checksum/license metadata, offline CI, executable quarantine, 3.4 beta references, GAEBXmlChecker catalog policy (issue #3 / PR #11)
  - G004: GAEB XML 3.3 AVA parser and conformance harness — AVA-first detection/import/validation/adaptation for BVBS X81/X84/X86 and X83, BVBS criteria matrix, golden snapshots, schema/checker policy (issue #4 / PR #12)
  - G005: GAEB 90 D81/D83 parse-only support — fixed-width Satzart parser, UTF-8/Windows-1252 decoding, D81/D83 phase detection, SupportedParseOnly capability (issue #5 / PR #12)
  - G006: Follow-on tracks and reference catalog — 42 manifest entries, testing-strategy assertions, requested phases represented, overclaim boundaries (issue #6 / PR #11)
  - G007: Verification and final quality — coverage 95% lines/functions/regions, fixture manifest verify, archgate, prek all PASS
---

# BOQ-PRD-001: GAEB parser and certification harness (master PRD)

Master PRD at `.omx/plans/prd-boq-core-gaeb-parser-20260606.md`. The full milestone breakdown is in §5 of the source PRD. PRD §20 (product acceptance criteria) supersedes the earlier meta criteria in §11.

Key boundary: certification-path readiness only. No paid BVBS submission, no official certification representation, no Obra backend integration in the MVP.

The PRD is approved by architect (APPROVE) and critic (APPROVE) at iter 1/1; see `ralplan-handoff-boq-core-gaeb-parser-20260606.json`. The ultragoal ledger at `.omx/ultragoal/goals.json` and the G007 quality gate at `.omx/ultragoal/quality-gate-g007.json` are the implementation record.

See `raw/prd/BOQ-PRD-002..010` for the issue-level PRDs (#37–#44) that extend this master PRD.
