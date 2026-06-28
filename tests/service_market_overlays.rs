#![allow(missing_docs, clippy::expect_used)]

use std::{collections::BTreeSet, fs, process::Command};

use boq_core::catalogo::{CatalogoCuadroTable, apply_catalogo_overlay};
use boq_core::dqe::{DqeEstimateTable, apply_dqe_overlay};
use boq_core::prezzario::{PrezzarioComputoTable, apply_computo_overlay};
use boq_core::service_market_overlays::{
    MARKET_OVERLAY_READINESS_SCHEMA_VERSION, export_market_overlay_readiness,
};
use boq_core::service_obra_import::{
    ObraAdapterRejectionCode, ObraImportStatus, report_from_document,
};
use boq_core::sinapi::{SinapiCatalogTable, apply_catalog_overlay};
use boq_core::stabu::{StabuRawTable, apply_stabu_raw_overlay};
use boq_core::support::SupportStatus;

#[test]
fn market_overlay_readiness_report_is_stable_and_support_honest() {
    let report = export_market_overlay_readiness();

    assert_eq!(
        report.schema_version,
        MARKET_OVERLAY_READINESS_SCHEMA_VERSION
    );
    assert_eq!(report.overlays.len(), 9);
    assert!(!report.production_ready);
    assert!(report.certification_claims.is_empty());
    assert!(!report.external_catalog_download_required);
    assert!(report.support_boundary.contains("never promotes"));

    let keys: Vec<_> = report.overlays.iter().map(|row| row.overlay_key).collect();
    assert_eq!(
        keys,
        [
            "sinapi-bdi",
            "prezzario-computo",
            "catalogo-cuadro",
            "din276-classification",
            "csi-masterformat-classification",
            "uniclass-classification",
            "nlsfb-classification",
            "stabu-raw",
            "dqe-quantity"
        ]
    );

    for row in &report.overlays {
        assert!(
            fs::metadata(row.evidence_fixture).is_ok(),
            "missing fixture {}",
            row.evidence_fixture
        );
        assert!(row.module.starts_with("boq_core::"));
        assert!(
            row.service_contracts
                .contains(&"boq-core.service-analyze.v1")
        );
        assert!(row.service_contracts.contains(&"boq-core.obra-import.v1"));
        assert!(
            row.supported_metadata.contains(&"source_provenance"),
            "{} must carry provenance",
            row.overlay_key
        );
        assert!(!row.promotes_support_status, "{}", row.overlay_key);
        assert!(
            !row.grants_adapter_support_to_parse_only,
            "{}",
            row.overlay_key
        );
        assert!(!row.complete_market_coverage_claimed, "{}", row.overlay_key);
        let boundary = row.current_support_boundary.to_ascii_lowercase();
        assert!(boundary.contains("synthetic"), "{}", row.overlay_key);
        assert!(
            boundary.contains("no ") && boundary.contains("complete"),
            "{}",
            row.overlay_key
        );
    }
}

#[test]
fn market_overlay_schema_and_docs_are_checked_in() {
    for path in [
        "docs/service-contract/market-overlays-v1.md",
        "docs/service-contract/market-overlays-v1.schema.json",
    ] {
        assert!(fs::metadata(path).is_ok(), "missing artifact: {path}");
    }

    let docs = fs::read_to_string("docs/service-contract/market-overlays-v1.md").expect("docs");
    for required in [
        "boq-core.market-overlays.v1",
        "sinapi-bdi",
        "prezzario-computo",
        "catalogo-cuadro",
        "din276-classification",
        "csi-masterformat-classification",
        "uniclass-classification",
        "nlsfb-classification",
        "stabu-raw",
        "dqe-quantity",
        "production_ready`: always `false`",
        "external_catalog_download_required`: always `false`",
        "grants_adapter_support_to_parse_only`: always `false`",
    ] {
        assert!(docs.contains(required), "docs missing: {required}");
    }

    let forbidden = [
        "production ready",
        "certified",
        "complete SINAPI coverage",
        "complete Prezzario coverage",
        "complete STABU coverage",
        "complete DQE coverage",
    ];
    let lower_docs = docs.to_ascii_lowercase();
    for phrase in forbidden {
        assert!(
            !lower_docs.contains(&phrase.to_ascii_lowercase()),
            "docs must not overclaim: {phrase}"
        );
    }

    let schema: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("docs/service-contract/market-overlays-v1.schema.json")
            .expect("schema"),
    )
    .expect("schema JSON");
    assert_eq!(
        schema["properties"]["schema_version"]["const"],
        MARKET_OVERLAY_READINESS_SCHEMA_VERSION
    );
    assert_eq!(schema["properties"]["production_ready"]["const"], false);
    assert_eq!(
        schema["properties"]["external_catalog_download_required"]["const"],
        false
    );
}

#[test]
fn market_overlay_cli_output_matches_library_contract() {
    let exe = std::env::var("CARGO_BIN_EXE_boq-core-service").expect("service binary path");
    let output = Command::new(exe)
        .arg("market-overlays")
        .output()
        .expect("service cli runs");
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let actual: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let expected =
        serde_json::to_value(export_market_overlay_readiness()).expect("library report JSON");
    assert_eq!(actual, expected);
}

#[test]
fn market_overlays_do_not_promote_parse_only_inputs_or_adapter_support() {
    let mut document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal D81 should parse");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
    let original_capabilities = document.capabilities;
    let original_status = document.support_status;

    let mut finding_codes = BTreeSet::new();
    for finding in apply_catalog_overlay(&mut document, &sinapi_table()) {
        finding_codes.insert(finding.code);
    }
    for finding in apply_computo_overlay(&mut document, &prezzario_table()) {
        finding_codes.insert(finding.code);
    }
    for finding in apply_catalogo_overlay(&mut document, &catalogo_table()) {
        finding_codes.insert(finding.code);
    }
    for finding in apply_stabu_raw_overlay(&mut document, &stabu_table()) {
        finding_codes.insert(finding.code);
    }
    for finding in apply_dqe_overlay(&mut document, &dqe_table()) {
        finding_codes.insert(finding.code);
    }

    assert_eq!(document.support_status, original_status);
    assert_eq!(document.capabilities, original_capabilities);

    let report = report_from_document(&document);
    assert_eq!(report.status, ObraImportStatus::Blocked);
    let rejection = report
        .rejection
        .expect("parse-only document should explain adapter block");
    assert_eq!(
        rejection.code,
        ObraAdapterRejectionCode::ObraAdapterSupportedParseOnly
    );
    assert_eq!(rejection.support_status, SupportStatus::SupportedParseOnly);

    assert!(
        finding_codes.is_empty()
            || finding_codes
                .iter()
                .all(|code| code.ends_with("_preserved_not_evaluated")),
        "unexpected overlay findings: {finding_codes:?}"
    );
}

fn sinapi_table() -> SinapiCatalogTable {
    SinapiCatalogTable::from_json_str(include_str!("fixtures/synthetic/sinapi_catalog.json"))
        .expect("synthetic SINAPI fixture should load")
}

fn prezzario_table() -> PrezzarioComputoTable {
    PrezzarioComputoTable::from_json_str(include_str!("fixtures/synthetic/prezzario_computo.json"))
        .expect("synthetic Prezzario fixture should load")
}

fn catalogo_table() -> CatalogoCuadroTable {
    CatalogoCuadroTable::from_json_str(include_str!("fixtures/synthetic/catalogo_cuadro.json"))
        .expect("synthetic Catálogo/Cuadro fixture should load")
}

fn stabu_table() -> StabuRawTable {
    StabuRawTable::from_json_str(include_str!("fixtures/synthetic/stabu_raw.json"))
        .expect("synthetic STABU/RAW fixture should load")
}

fn dqe_table() -> DqeEstimateTable {
    DqeEstimateTable::from_json_str(include_str!("fixtures/synthetic/dqe_quantity.json"))
        .expect("synthetic DQE fixture should load")
}
