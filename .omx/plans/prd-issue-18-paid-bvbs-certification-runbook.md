# PRD: Prepare paid BVBS certification submission runbook

## Issue
- GitHub issue: #18
- Milestone: v0.2 AVA certification readiness

## Product outcome
A no-paid-actions runbook defines preflight evidence, human authorization gates, submission steps, and post-certification status updates.

## Source/status anchors
- BVBS certification page/criteria: `reference_only`.
- GAEBXmlChecker evidence: `reference_only` (matches gaeb/manifest.toml id `bvbs_gaeb_xml_checker`; no `tooling_only` vocabulary term exists in the manifest; see brief's pre-execution validation gate for any future vocabulary amendment via ARCH-002).
- Paid submission: gated manual action.

## Requirements
- [ ] Separate internal readiness from external paid submission authority.
- [ ] Require explicit human approval before payment/submission/contact.
- [ ] State exactly when `certified` wording becomes allowed.

## Planned tests/checks
- [ ] `test_paid_cert_runbook_requires_human_authorization`
- [ ] `test_cert_readiness_checklist_references_green_pr_gates`
- [ ] `test_runbook_distinguishes_readiness_from_certified_status`
