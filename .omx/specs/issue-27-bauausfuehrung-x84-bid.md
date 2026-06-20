# Spec: Bauausführung X84 bid submission support

## Issue
- GitHub issue: #27
- Branch: `issue-27-bau-x84-bid`
- PRD: `.omx/plans/prd-issue-27-bauausfuehrung-x84-bid.md`

## Scope
Implement fixture-backed, parse-only Bauausführung X84 bid submission behavior on top of the X83 baseline parser from #26.

## In scope
- Treat X84 as an offer/bid payload, not as the authoritative tender baseline.
- Promote the BVBS GAEB XML 3.3 Bau X84 fixture to `supported_parse_only` only for parser/readiness evidence.
- Overlay X84 prices by ordinal onto an X83 baseline with deterministic missing/extra/duplicate findings.
- Preserve bidder remarks separately from X83 tender descriptions.
- Flag X84 description text that attempts to mutate tender baseline wording.
- Keep Obra adapter/export/roundtrip/schema-validation support disabled for cataloged X84 fixtures.

## Out of scope
- Paid BVBS certification/checker submission.
- X84 export fidelity certification.
- Direct Obra backend integration.
- Legacy XML 3.1/3.2 Bau X84 promotion.

## Functional requirements
1. X84 fixture paths resolve to `supported_parse_only` with parse-only capabilities.
2. `merge_x84_offer_into_x83_baseline` copies unit price and total price from matching X84 offer items to X83 baseline items by ordinal.
3. X83 baseline descriptions remain authoritative when X84 carries sparse or changed description text.
4. X84 bidder remarks are parsed into `BoqItem.notes` and `gaeb.bau_x84.bidder_remark` metadata, then preserved on the merged baseline item.
5. X84 offer items without a baseline match emit `gaeb_xml_bau_x84_extra_ordinal`; baseline items without an offer match emit `gaeb_xml_bau_x84_missing_ordinal`.
6. X84 description text that differs from the X83 baseline emits `gaeb_xml_bau_x84_mutable_tender_description`.

## Non-functional requirements
- Keep changes focused on Bau-specific parser/merge behavior, fixture manifest evidence, tests, and issue artifacts.
- Preserve ARCH-002 manifest vocabulary and ARCH-003/ARCH-004 boundaries.
- Local and GitHub quality gates must be green before merge.
