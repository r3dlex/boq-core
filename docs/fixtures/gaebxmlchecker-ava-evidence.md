# GAEBXmlChecker AVA evidence workflow

This workflow records optional, local GAEBXmlChecker readiness evidence for the
BVBS AVA X81, X84, and X86 fixtures. It is a readiness aid only: it is not a
BVBS submission, it is not official certification, and it must not be described
as certified support.

## Source and support status

`gaeb/manifest.toml` keeps `bvbs_gaeb_xml_checker` as `reference_only`. The
checker zip is cataloged so a human can checksum-pin and review a local run, but
agents and CI must not download or execute the checker binary. Supported AVA
fixtures remain parser-supported readiness targets:

| fixture id | phase | expected status | evidence key |
| --- | --- | --- | --- |
| `bvbs_xml33_ava_x81` | X81 | `supported` | `x81` |
| `bvbs_xml33_ava_x84` | X84 | `supported` | `x84` |
| `bvbs_xml33_ava_x86` | X86 | `supported` | `x86` |

## Missing-tool behavior

If the checker is unavailable, the result is `skipped_missing_checker`. A skip is
neither pass nor fail and must not be used to promote or demote parser support.
CI and agents use this gated state by default.

Allowed local-only inputs:

- a human-provided checker path outside the repository;
- a checksum-pinned checker archive or executable recorded in evidence;
- generated checker output reviewed in the same PR as the evidence artifact.

Disallowed automation:

- downloading the checker in CI;
- committing checker binaries or executable payloads;
- treating missing checker as a green certification result;
- using checker evidence as an official BVBS certification claim.

## Evidence artifact schema

Human-local evidence, when authorized, is stored under
`gaeb/evidence/gaebxmlchecker/ava/` as JSON. Evidence files are reviewed artifacts
and are not generated automatically by CI.

Required top-level fields:

```json
{
  "schema_version": 1,
  "tool": "GAEBXmlChecker",
  "tool_support_status": "reference_only",
  "tool_checksum_sha256": "<sha256 from local approved checker payload>",
  "certification_claim": false,
  "generated_by": "human-local-run",
  "results": [
    {
      "fixture_id": "bvbs_xml33_ava_x81",
      "phase": "x81",
      "fixture_support_status": "supported",
      "result": "passed | failed | skipped_missing_checker",
      "checker_output_sha256": "<sha256 of captured checker output or null>"
    },
    {
      "fixture_id": "bvbs_xml33_ava_x84",
      "phase": "x84",
      "fixture_support_status": "supported",
      "result": "passed | failed | skipped_missing_checker",
      "checker_output_sha256": "<sha256 of captured checker output or null>"
    },
    {
      "fixture_id": "bvbs_xml33_ava_x86",
      "phase": "x86",
      "fixture_support_status": "supported",
      "result": "passed | failed | skipped_missing_checker",
      "checker_output_sha256": "<sha256 of captured checker output or null>"
    }
  ]
}
```

Evidence labels may say "readiness" or "checker comparison". They must not use
positive certification language or equivalent wording.

## PR review checklist

- The checker remains `reference_only` in `gaeb/manifest.toml`.
- X81, X84, and X86 are the only AVA fixture phases covered by this evidence
  workflow.
- Missing checker is represented as `skipped_missing_checker` and is neither a
  pass nor a fail.
- Evidence JSON, if present, has `certification_claim: false`.
- No `.exe`, `.dll`, `.msi`, `.zip`, or other executable checker payload is
  committed under `gaeb/evidence/gaebxmlchecker/ava/`.
