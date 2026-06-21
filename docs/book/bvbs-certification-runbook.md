# BVBS Certification Submission Runbook

This runbook prepares the manual path for paid official BVBS certification. It is a no-paid-actions artifact: it defines prerequisites, evidence, owner approvals, placeholders, and stop conditions, but it does not execute payment, credential entry, external contact, or submission.

## Authority and stop condition

`ARCH-004` is the governing gate. Issue #7 is the explicit paid external action gate. This issue (#18) only prepares the runbook; it does not authorize the external action.

Stop condition before any external step:

- No paid submission without explicit human authorization.
- No credential entry without explicit human authorization.
- No external contact without explicit human authorization.
- No portal upload, email submission, or payment without named approvers.
- No official-result status update without an official result artifact.

Required named approvers before Issue #7 may proceed:

| Role | Required decision | Placeholder |
| --- | --- | --- |
| budget owner | Approves fee, payment method, and cost center. | `TBD: budget owner` |
| account owner | Approves BVBS account, credentials, and tool/portal access. | `TBD: account owner` |
| submission owner | Approves evidence bundle and submits externally. | `TBD: submission owner` |
| technical reviewer | Confirms readiness evidence and support-status wording. | `TBD: reviewer` |
| release owner | Confirms post-result release/status update plan. | `TBD: release owner` |

## Prerequisites

Before opening the paid-action gate, confirm:

1. All relevant readiness PRs are merged with `Rust quality gates` green.
2. Local checks for the evidence package are green.
3. GH CI is green for the final runbook/evidence PR.
4. `mergeStateStatus CLEAN` was recorded before each merge.
5. GitHub review threads are empty or resolved.
6. `gaeb/manifest.toml` support statuses still use only `supported`, `supported_parse_only`, `future_track`, and `reference_only`.
7. The DOC-4 certification evidence guide is current.
8. No PR claims an official result before the external process returns one.

## Evidence bundle

The Evidence bundle should be assembled as a single reviewed folder or archive before Issue #7 is authorized. It should include:

| Bundle item | Source | Required status |
| --- | --- | --- |
| AVA checker evidence | `docs/fixtures/gaebxmlchecker-ava-evidence.md` / PR #91 | Merged, review threads resolved, GH CI green. |
| AVA criteria matrix | `docs/fixtures/bvbs-ava-criteria-readiness.md`, `gaeb/criteria/bvbs_ava_matrix.toml` / PR #92 | Merged, criteria statuses reviewed. |
| AVA golden reports | `docs/fixtures/bvbs-ava-golden-reports.md`, `gaeb/golden/bvbs_ava/` / PR #93 | Merged, deterministic reports current. |
| AVA rich text/schema handling | `docs/fixtures/ava-rich-text-schema-version.md` / PR #94 | Merged, parser behavior reflected in golden reports. |
| Certification evidence guide | `docs/book/certification-evidence-guide.md` / PR #96 | Merged, no-overclaiming tests green. |
| local docs gate | `cargo test --test docs_mvp`, `mdbook build`, `archgate check --ci`, `uvx prek run --all-files` | Green immediately before bundle handoff. |
| CI gate | GitHub `Rust quality gates` | Green on the runbook/evidence PR. |
| Review evidence | PR review threads | Empty/resolved before merge. |

Recommended bundle structure:

```text
bvbs-certification-evidence/
├── README.md                         # scope, date, commit SHAs, contact placeholders
├── manifest-summary.md               # selected gaeb/manifest.toml rows and support statuses
├── local-gates.md                     # command outputs or links to logs
├── github-ci.md                       # PR numbers, run URLs, merge commits
├── criteria/
│   └── bvbs_ava_matrix.toml
├── golden-reports/
│   ├── x81-report.json
│   ├── x84-report.json
│   └── x86-report.json
└── checker-evidence/
    └── gaebxmlchecker-ava-evidence.md
```

## Manual submission steps after authorization

Only after Issue #7 has explicit human authorization:

1. Confirm budget owner approval.
2. Confirm account owner approval and credential handling rules.
3. Confirm the external BVBS contact or portal path.
4. Confirm the evidence bundle checksum and commit SHA.
5. Submit or upload the evidence bundle manually.
6. Record the external submission timestamp, submitter, contact/channel, fee reference, and expected turnaround.
7. Store any official result artifact in the location approved by the release owner.
8. Open a follow-up PR for status wording or manifest changes only after the official result artifact exists.

Expected turnaround is external-provider dependent. Use `TBD: BVBS turnaround from quote/contact` until a current provider estimate is received by the submission owner.

## Contacts and placeholders

Do not fill these with credentials in git:

- BVBS contact or portal URL: `TBD by submission owner`.
- BVBS account holder: `TBD by account owner`.
- Payment method reference: `TBD by budget owner`.
- Evidence bundle storage location: `TBD by release owner`.
- Official result artifact location: `TBD after external result`.

## readiness status vs official-result status

Readiness status means local evidence, parser behavior, criteria matrices, docs, and CI are prepared for a possible external submission.

Official-result status means the external paid process returned an official result artifact and the release owner approved any public wording changes.

certified wording becomes allowed only after all of the following are true:

1. Issue #7 was explicitly authorized by the required humans.
2. The external paid submission was completed by the submission owner.
3. An official result artifact was received and stored.
4. A follow-up PR updates documentation, release notes, and any manifest wording with reviewer approval.
5. That PR keeps support-status vocabulary honest and passes local/GH gates.

do not update support_status solely because the runbook exists. `reference_only` and `supported_parse_only` entries remain unchanged unless separate parser/support evidence and review justify a manifest update.
