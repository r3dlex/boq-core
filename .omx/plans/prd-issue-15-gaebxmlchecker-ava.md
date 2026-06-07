# PRD: Integrate GAEBXmlChecker into AVA certification workflow

## Issue
- GitHub issue: #15
- Milestone: v0.2 AVA certification readiness

## Product outcome
GAEBXmlChecker is available as an optional, checksum-pinned local evidence step for AVA X81/X84/X86 fixtures. It must never imply paid BVBS certification or require CI network access.

## Source/status anchors
- GAEBXmlChecker zip: `reference_only`, optional local evidence; do not introduce a new status without an ARCH-002 manifest-vocabulary amendment.
- BVBS AVA X81/X84/X86: `supported`, no official certification claim until paid process succeeds.

## Requirements
- [ ] Add a local-only checker invocation/runbook and evidence artifact schema.
- [ ] Capture pass/fail output per AVA fixture.
- [ ] Skip safely when checker is unavailable.
- [ ] Document that checker evidence is readiness evidence, not official certification.

## Planned tests
- [ ] `test_gaebxmlchecker_ava_source_status_matrix`
- [ ] `test_gaebxmlchecker_ava_optional_tool_skip`
- [ ] `test_gaebxmlchecker_ava_evidence_output_per_fixture`
- [ ] `test_gaebxmlchecker_ava_no_certification_claim`
