# Spec issue #33: Texterstellung layout criteria

## Scope

Planning-only specification for the GAEB ranked roadmap source slice backing issue #33. This artifact binds the issue to the canonical source inventory and records how each linked source may be used for safe fixture readiness.

## Non-goals

- No paid BVBS certification action.
- No live/source download during unit tests.
- No parser support promotion beyond the parser support status recorded in the canonical ledger.
- No duplicate issue creation; this issue remains the owning lane for the rows listed below.

## Ranked roadmap source audit

| Source ID | Source | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- |
| R5-03 | #32-#33 Texterstellung roadmap | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | #32-#33 Texterstellung roadmap | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |

## Acceptance criteria

- The PRD, spec, and test spec for issue #33 all reference the same canonical source IDs.
- Manifest-backed rows can only be used through local fixture manifest entries.
- Planned or artifact-only rows remain documented as research/reference gates until a license-safe local fixture is added and checksummed.
- Certification PDFs and visual reference PDFs are treated as reference-only evidence, not executable parser fixtures.

## Implementation handoff

Use `.omx/specs/gaeb-ranked-source-status-ledger.md` as the source of truth for dispositions. Update `boq-core/gaeb/manifest.toml` and a follow-up test spec before promoting any planned row to fixture-backed execution.
