# GAEB ranked source status ledger

Status: planning support for ranks 1-8 and appendix tracks.
Scope: source-readiness ledger only; this PR does not download, vendor, execute, or certify any external source.

## Constraints

- No paid BVBS certification/support actions are required or implied.
- No duplicate issue explosion: every source row maps to the existing roadmap lanes #28-#44.
- CI and parser tests must not perform live network downloads.
- `reference_only` rows may support documentation, schemas, or planning, but must not become parser fixtures by themselves.
- `future_track` rows become executable test fixtures only after license-safe acquisition, local vendoring/fixture storage, checksum/manifest entry, and targeted regression test coverage.
- Executables, browser platforms, commercial utilities, and interactive-only pages are not executed by this PR. They are eligible only through a future safe fixture workflow, and paid/certification actions remain out of scope.

## Status vocabulary

| Status | Meaning |
| --- | --- |
| `manifested` | Source has a planned catalog/checksum manifest identity already named by the roadmap/testing specs. |
| `artifact-only/reference` | Source is useful for docs/planning but is not yet a local test artifact. |
| `gap` | Source is known from the roadmap but still needs a safe acquisition/storage decision. |
| `reference_only` | May be cited or consulted; not executable parser test input. |
| `future_track:<name>` | Candidate future regression lane after safe local fixture promotion. |

`manifested` records a roadmap/testing manifest identity only; it does not imply executable parser input unless the row is also promoted through the safe `future_track` workflow.

## Ranked source ledger

| ID | Rank | Issue lane | Source | Readiness | Manifest/test ID | Test posture |
| --- | --- | --- | --- | --- | --- | --- |
| R1-01 | Rank 1 | #28-#31 | GAEB DA XML 3.3 technical documentation 2023-01 | artifact-only/reference | — | reference_only |
| R1-02 | Rank 1 | #28-#31 | Official quantity takeoff schema package | manifested | official_gaeb_xml33_mengenermittlung | reference_only |
| R1-03 | Rank 1 | #28-#31 | Interactive X31 schema | artifact-only/reference | — | reference_only |
| R1-04 | Rank 1 | #28-#31 | BVBS X31 quantity takeoff test file | manifested | bvbs_xml33_qty_x31 | future_track:`future_quantity_takeoff_x31_cataloged` |
| R1-05 | Rank 1 | #28-#31 | BVBS X86 quantity contract baseline | manifested | bvbs_xml33_qty_x86 | future_track:`future_quantity_takeoff_x86_cataloged` |
| R1-06 | Rank 1 | #28-#31 | Muster takeoff visual PDF calculations | gap | — | reference_only |
| R1-07 | Rank 1 | #28-#31 | Muster takeoff visual PDF results | manifested | bvbs_xml33_qty_results_pdf | reference_only |
| R1-08 | Rank 1 | #28-#31 | Reference layout drawing | gap | — | reference_only |
| R1-09 | Rank 1 | #28-#31 | BVBS takeoff audit criteria | manifested | bvbs_xml33_mengenermittlung_criteria_pdf | reference_only |
| R2-01 | Rank 2 | #34-#36 | Official invoicing schema package | manifested | official_gaeb_xml33_rechnung | reference_only |
| R2-02 | Rank 2 | #34-#36 | Interactive X89 schema | artifact-only/reference | — | reference_only |
| R2-03 | Rank 2 | #34-#36 | Interactive X89B schema | artifact-only/reference | — | reference_only |
| R2-04 | Rank 2 | #34-#36 | BVBS E-Rechnung implementation recommendation | artifact-only/reference | — | reference_only |
| R2-05 | Rank 2 | #34-#36 | MWM-Mengen-Viewer utility | artifact-only/reference | — | reference_only |
| R3-01 | Rank 3 | #37 | GAEB DA XML 3.2 technical documentation | artifact-only/reference | — | reference_only |
| R3-02 | Rank 3 | #37 | GAEB DA XML 3.2 LV core package | manifested | official_gaeb_xml32_leistungsverzeichnis | reference_only |
| R3-03 | Rank 3 | #37 | Interactive GAEB XML 3.2 X83 schema | artifact-only/reference | — | reference_only |
| R3-04 | Rank 3 | #37 | BVBS XML 3.2 AVA X81 | manifested | bvbs_xml32_ava_x81 | future_track:`future_legacy_xml32_ava_x81_cataloged` |
| R3-05 | Rank 3 | #37 | BVBS XML 3.2 AVA X84 | manifested | bvbs_xml32_ava_x84 | future_track:`future_legacy_xml32_ava_x84_cataloged` |
| R3-06 | Rank 3 | #37 | BVBS XML 3.2 AVA X86 | manifested | bvbs_xml32_ava_x86 | future_track:`future_legacy_xml32_ava_x86_cataloged` |
| R3-07 | Rank 3 | #37 | Muster-LV visual PDF AVA 3.2 | artifact-only/reference | — | reference_only |
| R3-08 | Rank 3 | #37 | BVBS AVA 3.2 criteria PDF | artifact-only/reference | — | reference_only |
| R3-09 | Rank 3 | #37 | GAEB DA XML 3.1 technical documentation | artifact-only/reference | — | reference_only |
| R3-10 | Rank 3 | #37 | GAEB 3.1 sample datasets | manifested | official_gaeb_xml31_muster_2009_12 | reference_only |
| R3-11 | Rank 3 | #37 | Interactive GAEB XML 3.1 schema directory | artifact-only/reference | — | reference_only |
| R3-12 | Rank 3 | #37 | BVBS 3.1 Bauausführung X83 | manifested | bvbs_xml31_bau_x83 | future_track:`test_xml31_bau_sources_are_cataloged_before_parser_promotion` |
| R3-13 | Rank 3 | #37 | BVBS 3.1 Bauausführung X84 | artifact-only/reference | — | reference_only |
| R3-14 | Rank 3 | #37 | Muster-LV visual PDF 3.1 Bauausführung | manifested | bvbs_xml31_bau_pdf | reference_only |
| R3-15 | Rank 3 | #37 | BVBS 3.1 Bauausführung criteria | artifact-only/reference | — | reference_only |
| R4-01 | Rank 4 | #39/#40 | Dangl.AVA examples C# repo | manifested | dangl_ava_examples | future_track:`future_dangl_ava_examples_cataloged` |
| R4-02 | Rank 4 | #39/#40 | Dangl.AVA examples C++ repo | manifested | dangl_ava_examples_cpp | future_track:`future_dangl_cpp_examples_cataloged` |
| R4-03 | Rank 4 | #39 | GAEB 2000 Sportheim demo gist | manifested | dangl_gaeb2000_sportheim_gist | future_track:`future_gaeb2000_sportheim_cataloged` |
| R4-04 | Rank 4 | #40 | GAEB-Tools syntax/reference page | artifact-only/reference | — | reference_only |
| R4-05 | Rank 4 | #40 | Sander-Doll GAEB 90/D83 overview | artifact-only/reference | — | reference_only |
| R5-01 | Rank 5 | #32-#33 | BVBS Texterstellung X81 | manifested | bvbs_xml33_text_x81 | supported_parse_only:`test_text_x81_rich_text_blocks_preserved`, `test_text_tables_normalize_to_document_blocks`, `test_text_unsupported_layout_emits_findings`, `test_texterstellung_support_promotion_requires_rich_text_evidence` |
| R5-02 | Rank 5 | #32-#33 | BVBS Texterstellung X82 | manifested | bvbs_xml33_text_x82 | supported_parse_only:`test_text_x82_cost_estimate_metadata_preserved`, `test_text_unsupported_layout_emits_findings`, `test_text_tables_normalize_to_document_blocks`, `test_texterstellung_support_promotion_requires_rich_text_evidence` |
| R5-03 | Rank 5 | #32-#33 | Muster authoring verification guideline | artifact-only/reference | — | reference_only |
| R5-04 | Rank 5 | #32-#33 | BVBS Texterstellung audit criteria | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only |
| R6-01 | Rank 6 | #41 | GAEB XML 3.3 Kosten/Kalkulation schema package | manifested | official_gaeb_xml33_kosten_und_kalkulation | reference_only |
| R6-02 | Rank 6 | #41 | GAEB XML 3.2 Kalkulation schema package | manifested | official_gaeb_xml32_kalkulation | reference_only |
| R6-03 | Rank 6 | #41 | Interactive X50 Baukostenkatalog schema | artifact-only/reference | — | reference_only |
| R6-04 | Rank 6 | #41 | Interactive X52 Kalkulationsdaten schema 3.3 | artifact-only/reference | — | reference_only |
| R6-05 | Rank 6 | #41 | Interactive X52 Kalkulationsdaten schema 3.2 | artifact-only/reference | — | reference_only |
| R7-01 | Rank 7 | #43 | GAEB XML 3.3 Zeitvertrag schema package | manifested | official_gaeb_xml33_zeitvertrag | reference_only |
| R7-02 | Rank 7 | #43 | GAEB XML 3.2 Zeitvertrag schema package | manifested | official_gaeb_xml32_zeitvertrag | reference_only |
| R7-03 | Rank 7 | #43 | GAEB XML 3.2 Zeitvertrag examples | manifested | official_gaeb_xml32_zeitvertrag_examples | reference_only |
| R7-04 | Rank 7 | #43 | Interactive X83Z schema 3.3 | artifact-only/reference | — | reference_only |
| R7-05 | Rank 7 | #43 | Interactive X84Z schema 3.3 | artifact-only/reference | — | reference_only |
| R7-06 | Rank 7 | #43 | Interactive X83Z schema 3.2 | artifact-only/reference | — | reference_only |
| R8-01 | Rank 8 | #42 | GAEB XML 3.3 Handel schema package | manifested | official_gaeb_xml33_handel | reference_only |
| R8-02 | Rank 8 | #42 | GAEB XML 3.2 Handel schema package | artifact-only/reference | — | reference_only |
| R8-03 | Rank 8 | #42 | Interactive X93 schema 3.3 | artifact-only/reference | — | reference_only |
| R8-04 | Rank 8 | #42 | Interactive X94 schema 3.3 | artifact-only/reference | — | reference_only |
| R8-05 | Rank 8 | #42 | Interactive X93 schema 3.2 | artifact-only/reference | — | reference_only |
| A1-01 | Appendix | #38 | GAEB XML 3.4 beta schema files | manifested | official_gaeb_xml34_beta_schema | reference_only |
| A1-02 | Appendix | #38 | GAEB XML 3.4 beta changelog | manifested | official_gaeb_xml34_beta_changelog | reference_only |
| A2-01 | Appendix | #44 | GAEB-Online Excel import blank sheet | manifested | gaeb_online_import_template | reference_only |
| A2-02 | Appendix | #44 | GAEB-Online import sheet layout tutorial | artifact-only/reference | — | reference_only |
| A2-03 | Appendix | #44 | Muster Excel-LV generator executable | manifested | gaeb_online_generator_exe | reference_only |
| A2-04 | Appendix | #44 | MWM-Rialto | artifact-only/reference | — | reference_only |
| A2-05 | Appendix | #44 | EasyGAEB browser platform | artifact-only/reference | — | reference_only |
