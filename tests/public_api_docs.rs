#![allow(missing_docs, clippy::expect_used)]

use std::fs;

#[test]
fn public_parse_entrypoints_are_documented_for_required_phases() {
    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in [
        "GAEB 90 D81",
        "GAEB 90 D83",
        "GAEB DA XML X81",
        "GAEB DA XML X83",
        "Texterstellung X81/X82",
        "`supported_parse_only` rich-text/table parser-readiness evidence",
        "gaeb90::parse_bytes",
        "gaeb_xml::parse_str",
    ] {
        assert!(
            lib.contains(required),
            "crate-level rustdoc missing public parse entrypoint anchor: {required}"
        );
    }
}

#[test]
fn support_status_types_are_public_and_stable() {
    let support = fs::read_to_string("src/support.rs").expect("support docs exist");
    for required in [
        "Supported",
        "SupportedParseOnly",
        "FutureTrack",
        "ReferenceOnly",
        "supported vs parse-only vs future-track vs reference-only",
        "SupportCapabilities",
        "adapt_to_obra",
        "roundtrip",
    ] {
        assert!(
            support.contains(required),
            "support rustdoc missing public support-boundary anchor: {required}"
        );
    }
}

#[test]
fn error_and_finding_semantics_are_documented() {
    let error = fs::read_to_string("src/error.rs").expect("error docs exist");
    for required in [
        "ParseError",
        "ValidationFinding",
        "unrecoverable",
        "recoverable",
        "finding code",
        "source location",
    ] {
        assert!(
            error.contains(required),
            "error rustdoc missing public semantics anchor: {required}"
        );
    }
}

#[test]
fn x89_billing_draft_contract_is_documented() {
    let x89 = fs::read_to_string("src/x89.rs").expect("x89 docs exist");
    for required in [
        "ObraBillingDraft",
        "BillingReadiness",
        "XRechnung envelope",
        "does not promote any official",
    ] {
        assert!(
            x89.contains(required),
            "X89 rustdoc missing billing draft boundary anchor: {required}"
        );
    }
}

#[test]
fn multi_standard_model_contract_is_documented_without_support_promotion() {
    let model = fs::read_to_string("src/model.rs").expect("model docs exist");
    for required in [
        "MultiStandardAnnotations",
        "ClassificationReference",
        "PriceCatalogReference",
        "QuantityReference",
        "ProgressReference",
        "does not promote support",
    ] {
        assert!(
            model.contains(required),
            "model rustdoc missing multi-standard contract anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in [
        "model::MultiStandardAnnotations",
        "price/catalog",
        "do not",
        "promote support",
    ] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing multi-standard support-honesty anchor: {required}"
        );
    }
}

#[test]
fn din276_overlay_contract_is_documented_without_support_promotion() {
    let din276 = fs::read_to_string("src/din276.rs").expect("DIN 276 docs exist");
    for required in [
        "DIN 276 classification overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            din276.contains(required),
            "DIN 276 rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`din276`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing DIN 276 boundary anchor: {required}"
        );
    }
}

#[test]
fn masterformat_overlay_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/csi_masterformat.rs").expect("MasterFormat docs exist");
    for required in [
        "CSI MasterFormat classification overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "MasterFormat rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`csi_masterformat`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing MasterFormat boundary anchor: {required}"
        );
    }
}

#[test]
fn uniclass_overlay_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/uniclass.rs").expect("Uniclass docs exist");
    for required in [
        "Uniclass classification overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "Uniclass rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`uniclass`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing Uniclass boundary anchor: {required}"
        );
    }
}

#[test]
fn nlsfb_overlay_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/nlsfb.rs").expect("NL-SfB docs exist");
    for required in [
        "NL-SfB classification overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "NL-SfB rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`nlsfb`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing NL-SfB boundary anchor: {required}"
        );
    }
}

#[test]
fn prezzario_computo_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/prezzario.rs").expect("Prezzario docs exist");
    for required in [
        "Computo Metrico and Prezzario overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "Prezzario rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`prezzario`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing Prezzario boundary anchor: {required}"
        );
    }
}

#[test]
fn catalogo_cuadro_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/catalogo.rs").expect("Catálogo/Cuadro docs exist");
    for required in [
        "Catálogo de Conceptos and Cuadro de Precios overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "Catálogo/Cuadro rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`catalogo`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing Catálogo/Cuadro boundary anchor: {required}"
        );
    }
}

#[test]
fn spreadsheet_neutral_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/spreadsheet.rs").expect("spreadsheet docs exist");
    for required in [
        "Dependency-free spreadsheet-neutral CSV exchange helpers",
        "matched by GAEB OZ/item ordinal only",
        "do not promote support status",
        "do not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "spreadsheet rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`spreadsheet`] helpers", "do not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing spreadsheet boundary anchor: {required}"
        );
    }

    for artifact_path in [
        ".omc/specs/obra-coverage/PHASE-23-spreadsheet-neutral-roundtrip.md",
        "raw/prd/obra-coverage/PRD-PHASE-23-spreadsheet-neutral-roundtrip.md",
        "raw/tickets/obra-coverage/ISSUE-PHASE-23-spreadsheet-neutral-roundtrip.md",
    ] {
        let artifact = fs::read_to_string(artifact_path).expect("PHASE-23 artifact exists");
        assert!(
            artifact.contains("dependency-free neutral CSV exchange"),
            "{artifact_path} must describe PHASE-23 as CSV-neutral, not XLSX support"
        );
        assert!(
            artifact.contains("XLSX/ODS readers, writers, binary fixtures, and spreadsheet dependencies remain out of scope"),
            "{artifact_path} must keep XLSX/ODS support explicitly out of scope"
        );
        assert!(
            !artifact.contains("XLSX/CSV neutral exchange"),
            "{artifact_path} must not overclaim XLSX/CSV neutral exchange support"
        );
    }
}

#[test]
fn dqe_quantity_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/dqe.rs").expect("DQE docs exist");
    for required in [
        "DQE French quantity estimate overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "DQE rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`dqe`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing DQE boundary anchor: {required}"
        );
    }
}

#[test]
fn stabu_raw_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/stabu.rs").expect("STABU/RAW docs exist");
    for required in [
        "STABU / RAW exchange overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "STABU/RAW rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`stabu`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing STABU/RAW boundary anchor: {required}"
        );
    }
}

#[test]
fn sinapi_catalog_contract_is_documented_without_support_promotion() {
    let module = fs::read_to_string("src/sinapi.rs").expect("SINAPI docs exist");
    for required in [
        "SINAPI catalog and BDI overlay",
        "fixture-backed",
        "does not promote",
        "does not grant Obra adapter support",
    ] {
        assert!(
            module.contains(required),
            "SINAPI rustdoc missing support-honesty anchor: {required}"
        );
    }

    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in ["[`sinapi`] overlay", "does not promote support"] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing SINAPI boundary anchor: {required}"
        );
    }
}

#[test]
fn obra_adapter_dto_contract_has_examples() {
    let adapter = fs::read_to_string("src/adapter/obra.rs").expect("adapter docs exist");
    for required in [
        "ObraImportDocument::try_from_gaeb",
        "Obra adapter DTO compatibility",
        "deterministic_key",
        "loss_report",
        "adapter support",
        "obra_adapter_not_supported",
    ] {
        assert!(
            adapter.contains(required),
            "Obra adapter rustdoc missing DTO contract anchor: {required}"
        );
    }
}

#[test]
fn test_rustdoc_api_reference_no_support_overclaiming() {
    let mut combined = String::new();
    for path in [
        "src/lib.rs",
        "src/gaeb90.rs",
        "src/gaeb_xml/mod.rs",
        "src/adapter/obra.rs",
        "src/support.rs",
        "src/error.rs",
        "src/model.rs",
        "src/format.rs",
        "src/checksum.rs",
    ] {
        combined.push_str(&fs::read_to_string(path).expect("rustdoc source exists"));
        combined.push('\n');
    }

    for forbidden in [
        "officially BVBS certified",
        "BVBS certified",
        "paid certification completed",
        "X31 is supported",
        "X83 is supported",
        "X89 is supported",
        "GAEB XML 3.4 is supported",
        "tooling_only",
        "certification_fixture",
    ] {
        assert!(
            !combined.contains(forbidden),
            "rustdoc overclaims unsupported or prohibited status: {forbidden}"
        );
    }
}

#[test]
fn test_rustdoc_examples_parse_minimal_fixture() {
    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in [
        "include_str!(\"../tests/fixtures/synthetic/minimal_ava.x81\")",
        "include_bytes!(\"../tests/fixtures/synthetic/minimal.d81\")",
        "Ok::<(), boq_core::error::ParseError>(())",
    ] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing doctest anchor: {required}"
        );
    }

    let xml = include_str!("fixtures/synthetic/minimal_ava.x81");
    let xml_doc = boq_core::gaeb_xml::parse_str(xml, Some("minimal_ava.x81".to_owned()))
        .expect("rustdoc XML fixture example should parse");
    assert_eq!(xml_doc.summary.format, boq_core::model::GaebFormat::GaebXml);

    let gaeb90 = include_bytes!("fixtures/synthetic/minimal.d81");
    let gaeb90_doc = boq_core::gaeb90::parse_bytes(gaeb90, Some("minimal.d81".to_owned()))
        .expect("rustdoc GAEB 90 fixture example should parse");
    assert_eq!(
        gaeb90_doc.summary.format,
        boq_core::model::GaebFormat::Gaeb90
    );
}

#[test]
fn public_modules_have_examples_or_doc_rationale() {
    for (path, required) in [
        ("src/lib.rs", "# Public parse entrypoints"),
        ("src/gaeb90.rs", "parse_bytes_with_encoding"),
        ("src/gaeb_xml/mod.rs", "parse_str"),
        ("src/adapter/obra.rs", "Obra adapter DTO compatibility"),
        ("src/support.rs", "# Public support boundary"),
        ("src/error.rs", "Public API callers"),
        ("src/model.rs", "Loss-aware GAEB domain model"),
        ("src/format.rs", "advisory"),
        ("src/checksum.rs", "original source bytes"),
    ] {
        let source = fs::read_to_string(path).expect("rustdoc source exists");
        assert!(
            source.contains(required),
            "public module {path} missing rustdoc example/rationale anchor: {required}"
        );
    }
}

#[test]
fn service_contract_json_boundary_is_documented_without_support_promotion() {
    let lib = fs::read_to_string("src/lib.rs").expect("crate docs exist");
    for required in [
        "service_contract",
        "boq-core-service",
        "deterministic JSON parse/analyze reports",
        "without claiming production or certification readiness",
    ] {
        assert!(
            lib.contains(required),
            "crate rustdoc missing service contract boundary anchor: {required}"
        );
    }
}
