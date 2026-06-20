# Spec issue #25: Bauausführung X83 fixture promotion

## Issue
- GitHub issue: #25
- Milestone: v0.4 BVBS Bauausführung support

## Scope
Promote the manifest row `bvbs_xml33_bau_x83` from cataloged `future_track` to parser-backed `supported_parse_only` once local synthetic parser evidence, readiness matrix evidence, and a golden readiness report are present.

## Support contract
- XML 3.3 Bauausführung X83 may be promoted to `supported_parse_only` only.
- X83 promotion enables detect+parse evidence but does not enable adapter, export, roundtrip, schema validation, paid certification, or production certification claims.
- XML 3.3 Bauausführung X84 remains `future_track` until issue #27.
- XML 3.1/3.2 Bauausführung X83/X84 remain `future_track` until their compatibility issues.
- Criteria PDFs and visual outputs remain `reference_only` / manual evidence.

## Implementation plan
1. Add tests that fail while `bvbs_xml33_bau_x83` remains `future_track`.
2. Promote only the X83 manifest row to `supported_parse_only` and map it to the parser/golden evidence tests.
3. Update the Bau criteria matrix so X83 import readiness is automated/readiness-covered, while X84/schema/visual entries remain gap/manual.
4. Add a golden readiness report documenting the parser evidence and non-certification boundary.
5. Keep PRD and workspace mirrors synced.

## Non-goals
- No X84 support promotion.
- No paid BVBS submission or certification claim.
- No Obra backend coupling.
- No real external downloads in CI.
