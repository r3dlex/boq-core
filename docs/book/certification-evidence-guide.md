# Certification Evidence Guide

This guide explains how `boq-core` handles BVBS, GAEBXmlChecker, and official GAEB artifacts as evidence.

## Evidence is not certification

BVBS fixture parsing, GAEBXmlChecker references, and golden reports are evidence
for certification readiness. They are not paid certification, official approval,
or a substitute for the external BVBS process.

The project must not claim paid certification unless an authorized human completes
the paid external submission and records the official result.

## BVBS areas

The fixture manifest tracks four BVBS certification areas, one legacy compatibility track, and checker/tooling references:

1. AVA / tendering, awarding, billing.
2. Bauausführung / construction execution.
3. Mengenermittlung / quantity takeoff.
4. Texterstellung / specification authoring.
5. Legacy GAEB XML 3.2 compatibility.
6. GAEBXmlChecker as a tooling reference.

Current MVP priority is AVA certification-path readiness. Other areas stay
`future_track` or `reference_only` until implementation and tests promote them.

## GAEBXmlChecker policy

GAEBXmlChecker is cataloged as `reference_only` in `gaeb/manifest.toml`. It may
be used as an external comparison signal only after the invocation policy,
platform constraints, checksums, and licensing are governed.

Do not change GAEBXmlChecker to a supported parser dependency without a manifest vocabulary decision and regression tests.

## Evidence workflow

For each certification-path PR:

1. Confirm fixture manifest entry, URL, support status, and license note.
2. Download or resolve fixtures according to CI policy.
3. Record checksums and fixture provenance when payloads are used.
4. Run parser or adapter tests against synthetic and real examples.
5. Produce a criteria matrix or golden report when the PRD requires it.
6. Document findings without claiming official approval.

## No-overclaiming checklist

Before merging certification-related docs or code, verify that the PR does not say:

- language that implies completed external BVBS approval;
- paid certification is complete;
- `reference_only` tools are supported parser functionality;
- future-track phases such as X31 or X89 are currently supported;
- GAEB XML 3.4 beta is production supported.
