# GAEB ranked roadmap manifest/test gap audit

Planning-only cross-cutting audit generated from `.omx/specs/gaeb-ranked-source-status-ledger.md`.

## Constraints

- No paid BVBS certification actions.
- No live/source downloads during tests.
- No duplicate issue creation; source rows map to existing issue lanes #28-#44.
- Only rows with Manifest disposition `manifested` and Parser support status `fixture` may be used as executable local parser fixtures without further acquisition work.

## Complete ledger mapping

| Source ID | Rank / track | Source | URL / locator | Manifest disposition | Manifest ID / planned ID | Parser support status | Test mapping / gap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R1-01 | 1 | #28-#31 X31/Mengenermittlung roadmap | GAEB DA XML 3.3 technical documentation 2023-01 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R1-02 | 1 | #28-#31 X31/Mengenermittlung roadmap | Official quantity takeoff schema package | manifested | official_gaeb_xml33_mengenermittlung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-03 | 1 | #28-#31 X31/Mengenermittlung roadmap | Interactive X31 schema | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R1-04 | 1 | #28-#31 X31/Mengenermittlung roadmap | BVBS X31 quantity takeoff test file | manifested | bvbs_xml33_qty_x31 | future_track | ['future_quantity_takeoff_x31_cataloged'] |
| R1-05 | 1 | #28-#31 X31/Mengenermittlung roadmap | BVBS X86 quantity contract baseline | manifested | bvbs_xml33_qty_x86 | future_track | ['future_quantity_takeoff_x86_cataloged'] |
| R1-06 | 1 | #28-#31 X31/Mengenermittlung roadmap | Muster takeoff visual PDF calculations | gap | gap: manifest entry not present for calculations PDF | reference_only | Reference-only certification visual output; not executable as parser fixture. |
| R1-07 | 1 | #28-#31 X31/Mengenermittlung roadmap | Muster takeoff visual PDF results | manifested | bvbs_xml33_qty_results_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R1-08 | 1 | #28-#31 X31/Mengenermittlung roadmap | Reference layout drawing | gap | gap: manifest entry not present for reference drawing | reference_only | Reference-only visual/layout aid; not executable as parser fixture. |
| R1-09 | 1 | #28-#31 X31/Mengenermittlung roadmap | BVBS takeoff audit criteria | manifested | bvbs_xml33_mengenermittlung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R2-01 | 2 | #34-#36 Rechnung/XRechnung bridge | Official invoicing schema package | manifested | official_gaeb_xml33_rechnung | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R2-02 | 2 | #34-#36 Rechnung/XRechnung bridge | Interactive X89 schema | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-03 | 2 | #34-#36 Rechnung/XRechnung bridge | Interactive X89B schema | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R2-04 | 2 | #34-#36 Rechnung/XRechnung bridge | BVBS E-Rechnung implementation recommendation | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R2-05 | 2 | #34-#36 Rechnung/XRechnung bridge | MWM-Mengen-Viewer utility | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| R3-01 | 3 | #37 GAEB XML 3.1/3.2 compatibility | GAEB DA XML 3.2 technical documentation | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R3-02 | 3 | #37 GAEB XML 3.1/3.2 compatibility | GAEB DA XML 3.2 LV core package | manifested | official_gaeb_xml32_leistungsverzeichnis | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R3-03 | 3 | #37 GAEB XML 3.1/3.2 compatibility | Interactive GAEB XML 3.2 X83 schema | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R3-04 | 3 | #37 GAEB XML 3.1/3.2 compatibility | BVBS XML 3.2 AVA X81 | manifested | bvbs_xml32_ava_x81 | future_track | ['future_legacy_xml32_ava_x81_cataloged'] |
| R3-05 | 3 | #37 GAEB XML 3.1/3.2 compatibility | BVBS XML 3.2 AVA X84 | manifested | bvbs_xml32_ava_x84 | future_track | ['future_legacy_xml32_ava_x84_cataloged'] |
| R3-06 | 3 | #37 GAEB XML 3.1/3.2 compatibility | BVBS XML 3.2 AVA X86 | manifested | bvbs_xml32_ava_x86 | future_track | ['future_legacy_xml32_ava_x86_cataloged'] |
| R3-07 | 3 | #37 GAEB XML 3.1/3.2 compatibility | Muster-LV visual PDF AVA 3.2 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only visual PDF; not executable as parser fixture. |
| R3-08 | 3 | #37 GAEB XML 3.1/3.2 compatibility | BVBS AVA 3.2 criteria PDF | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only certification criteria PDF; not executable as parser fixture. |
| R3-09 | 3 | #37 GAEB XML 3.1/3.2 compatibility | GAEB DA XML 3.1 technical documentation | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R3-10 | 3 | #37 GAEB XML 3.1/3.2 compatibility | GAEB 3.1 sample datasets | manifested | official_gaeb_xml31_muster_2009_12 | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R3-11 | 3 | #37 GAEB XML 3.1/3.2 compatibility | Interactive GAEB XML 3.1 schema directory | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R3-12 | 3 | #37 GAEB XML 3.1/3.2 compatibility | BVBS 3.1 Bauausführung X83 | manifested | bvbs_xml31_bau_x83 | future_track | ['test_xml31_bau_sources_are_cataloged_before_parser_promotion'] |
| R3-13 | 3 | #37 GAEB XML 3.1/3.2 compatibility | BVBS 3.1 Bauausführung X84 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R3-14 | 3 | #37 GAEB XML 3.1/3.2 compatibility | Muster-LV visual PDF 3.1 Bauausführung | manifested | bvbs_xml31_bau_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R3-15 | 3 | #37 GAEB XML 3.1/3.2 compatibility | BVBS 3.1 Bauausführung criteria | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R4-01 | 4 | #39 GAEB 2000/Pxx compatibility | Dangl.AVA examples C# repo | manifested | dangl_ava_examples | future_track | ['future_dangl_ava_examples_cataloged'] |
| R4-02 | 4 | #39 GAEB 2000/Pxx compatibility | Dangl.AVA examples C++ repo | manifested | dangl_ava_examples_cpp | future_track | ['future_dangl_cpp_examples_cataloged'] |
| R4-03 | 4 | #39 GAEB 2000/Pxx compatibility | GAEB 2000 Sportheim demo gist | manifested | dangl_gaeb2000_sportheim_gist | future_track | ['future_gaeb2000_sportheim_cataloged'] |
| R4-04 | 4 | #40 GAEB90 adapter-compatible promotion | GAEB-Tools syntax/reference page | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| R4-05 | 4 | #40 GAEB90 adapter-compatible promotion | Sander-Doll GAEB 90/D83 overview | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| R5-01 | 5 | #32-#33 Texterstellung roadmap | BVBS Texterstellung X81 | manifested | bvbs_xml33_text_x81 | future_track | ['future_texterstellung_x81_cataloged'] |
| R5-02 | 5 | #32-#33 Texterstellung roadmap | BVBS Texterstellung X82 | manifested | bvbs_xml33_text_x82 | future_track | ['future_texterstellung_x82_cataloged'] |
| R5-03 | 5 | #32-#33 Texterstellung roadmap | Muster authoring verification guideline | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Reference-only planning artifact; not executable as parser fixture. |
| R5-04 | 5 | #32-#33 Texterstellung roadmap | BVBS Texterstellung audit criteria | manifested | bvbs_xml33_texterstellung_criteria_pdf | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R6-01 | 6 | #41 Kosten/Kalkulation X50-X52 | GAEB XML 3.3 Kosten/Kalkulation schema package | manifested | official_gaeb_xml33_kosten_und_kalkulation | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R6-02 | 6 | #41 Kosten/Kalkulation X50-X52 | GAEB XML 3.2 Kalkulation schema package | manifested | official_gaeb_xml32_kalkulation | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R6-03 | 6 | #41 Kosten/Kalkulation X50-X52 | Interactive X50 Baukostenkatalog schema | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R6-04 | 6 | #41 Kosten/Kalkulation X50-X52 | Interactive X52 Kalkulationsdaten schema 3.3 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R6-05 | 6 | #41 Kosten/Kalkulation X50-X52 | Interactive X52 Kalkulationsdaten schema 3.2 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R7-01 | 7 | #43 Zeitvertrag X83Z/X84Z | GAEB XML 3.3 Zeitvertrag schema package | manifested | official_gaeb_xml33_zeitvertrag | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-02 | 7 | #43 Zeitvertrag X83Z/X84Z | GAEB XML 3.2 Zeitvertrag schema package | manifested | official_gaeb_xml32_zeitvertrag | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-03 | 7 | #43 Zeitvertrag X83Z/X84Z | GAEB XML 3.2 Zeitvertrag examples | manifested | official_gaeb_xml32_zeitvertrag_examples | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R7-04 | 7 | #43 Zeitvertrag X83Z/X84Z | Interactive X83Z schema 3.3 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R7-05 | 7 | #43 Zeitvertrag X83Z/X84Z | Interactive X84Z schema 3.3 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R7-06 | 7 | #43 Zeitvertrag X83Z/X84Z | Interactive X83Z schema 3.2 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-01 | 8 | #42 Handel X93-X97 | GAEB XML 3.3 Handel schema package | manifested | official_gaeb_xml33_handel | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| R8-02 | 8 | #42 Handel X93-X97 | GAEB XML 3.2 Handel schema package | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-03 | 8 | #42 Handel X93-X97 | Interactive X93 schema 3.3 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-04 | 8 | #42 Handel X93-X97 | Interactive X94 schema 3.3 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| R8-05 | 8 | #42 Handel X93-X97 | Interactive X93 schema 3.2 | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Schema/documentation reference for validation planning; not a parser fixture. |
| A1-01 | A1 | #38 GAEB XML 3.4 beta tracking | GAEB XML 3.4 beta schema files | manifested | official_gaeb_xml34_beta_schema | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A1-02 | A1 | #38 GAEB XML 3.4 beta tracking | GAEB XML 3.4 beta changelog | manifested | official_gaeb_xml34_beta_changelog | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-01 | A2 | #44 Spreadsheet roundtrip | GAEB-Online Excel import blank sheet | manifested | gaeb_online_import_template | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-02 | A2 | #44 Spreadsheet roundtrip | GAEB-Online import sheet layout tutorial | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| A2-03 | A2 | #44 Spreadsheet roundtrip | Muster Excel-LV generator executable | manifested | gaeb_online_generator_exe | reference_only | Reference-only manifest artifact; not executable as parser fixture. |
| A2-04 | A2 | #44 Spreadsheet roundtrip | MWM-Rialto | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |
| A2-05 | A2 | #44 Spreadsheet roundtrip | EasyGAEB browser platform | artifact-only/reference | artifact-only/reference: documentation/schema/tooling | reference_only | Tooling or guidance reference for roundtrip planning; not vendored or executed. |

## Gap summary
- `artifact-only/reference`: 30 rows (R1-01, R1-03, R2-02, R2-03, R2-04, R2-05, R3-01, R3-03, R3-07, R3-08, R3-09, R3-11, R3-13, R3-15, R4-04, R4-05, R5-03, R6-03, R6-04, R6-05, R7-04, R7-05, R7-06, R8-02, R8-03, R8-04, R8-05, A2-02, A2-04, A2-05)
- `gap`: 2 rows (R1-06, R1-08)
- `manifested`: 29 rows (R1-02, R1-04, R1-05, R1-07, R1-09, R2-01, R3-02, R3-04, R3-05, R3-06, R3-10, R3-12, R3-14, R4-01, R4-02, R4-03, R5-01, R5-02, R5-04, R6-01, R6-02, R7-01, R7-02, R7-03, R8-01, A1-01, A1-02, A2-01, A2-03)

## Parser support summary
- `future_track`: 11 rows (R1-04, R1-05, R3-04, R3-05, R3-06, R3-12, R4-01, R4-02, R4-03, R5-01, R5-02)
- `reference_only`: 50 rows (R1-01, R1-02, R1-03, R1-06, R1-07, R1-08, R1-09, R2-01, R2-02, R2-03, R2-04, R2-05, R3-01, R3-02, R3-03, R3-07, R3-08, R3-09, R3-10, R3-11, R3-13, R3-14, R3-15, R4-04, R4-05, R5-03, R5-04, R6-01, R6-02, R6-03, R6-04, R6-05, R7-01, R7-02, R7-03, R7-04, R7-05, R7-06, R8-01, R8-02, R8-03, R8-04, R8-05, A1-01, A1-02, A2-01, A2-02, A2-03, A2-04, A2-05)

## Follow-up rule

Promote a planned row only after documenting license-safe acquisition, storing a local fixture, adding a checksum/manifest entry, and adding a targeted regression test.
