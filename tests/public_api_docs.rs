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
