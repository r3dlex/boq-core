//! Service-consumable market overlay readiness export.
//!
//! This module reports fixture-backed market overlay readiness for downstream
//! services. It is policy/evidence metadata only: exporting this report never
//! downloads external catalog data, never promotes parser support status, and
//! never claims complete market coverage, production readiness, or
//! certification.

use serde::{Deserialize, Serialize};

use crate::VERSION;

/// Schema version for [`MarketOverlayReadinessReport`].
pub const MARKET_OVERLAY_READINESS_SCHEMA_VERSION: &str = "boq-core.market-overlays.v1";

/// Exports the fixture-backed market overlay readiness matrix.
#[must_use]
pub fn export_market_overlay_readiness() -> MarketOverlayReadinessReport {
    MarketOverlayReadinessReport {
        schema_version: MARKET_OVERLAY_READINESS_SCHEMA_VERSION,
        crate_version: VERSION,
        overlays: overlay_rows(),
        production_ready: false,
        certification_claims: Vec::new(),
        external_catalog_download_required: false,
        support_boundary: "fixture-backed overlays only; applying overlays never promotes support_status or grants adapter support to parse-only inputs",
    }
}

/// Service-facing market overlay readiness report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketOverlayReadinessReport {
    /// Contract schema version.
    pub schema_version: &'static str,
    /// `boq-core` crate version that produced the report.
    pub crate_version: &'static str,
    /// Stable overlay rows in service display order.
    pub overlays: Vec<MarketOverlayReadinessRow>,
    /// This report never claims production readiness.
    pub production_ready: bool,
    /// This report never claims certification.
    pub certification_claims: Vec<String>,
    /// Services can consume this report without fetching external market data.
    pub external_catalog_download_required: bool,
    /// Global support boundary for all rows.
    pub support_boundary: &'static str,
}

/// A single fixture-backed market overlay readiness row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketOverlayReadinessRow {
    /// Stable service-facing overlay key.
    pub overlay_key: &'static str,
    /// Human-readable market/system label.
    pub label: &'static str,
    /// Rust module that owns the overlay implementation.
    pub module: &'static str,
    /// Deterministic fixture used by the tests.
    pub evidence_fixture: &'static str,
    /// Output contract surfaces where overlay evidence can appear.
    pub service_contracts: Vec<&'static str>,
    /// Metadata carried by the overlay when present.
    pub supported_metadata: Vec<&'static str>,
    /// Loss-finding semantics carried by the overlay when present.
    pub loss_findings: Vec<&'static str>,
    /// Row-specific support boundary.
    pub current_support_boundary: &'static str,
    /// Whether applying the row may promote parser support status.
    pub promotes_support_status: bool,
    /// Whether applying the row may grant adapter support to parse-only inputs.
    pub grants_adapter_support_to_parse_only: bool,
    /// Whether the row claims complete market/catalog coverage.
    pub complete_market_coverage_claimed: bool,
}

fn overlay_rows() -> Vec<MarketOverlayReadinessRow> {
    vec![
        sinapi_row(),
        prezzario_row(),
        catalogo_row(),
        din276_row(),
        csi_masterformat_row(),
        uniclass_row(),
        nlsfb_row(),
        stabu_row(),
        dqe_row(),
    ]
}

fn service_contracts() -> Vec<&'static str> {
    vec!["boq-core.service-analyze.v1", "boq-core.obra-import.v1"]
}

fn sinapi_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "sinapi-bdi",
        label: "SINAPI catalog and BDI evidence",
        module: "boq_core::sinapi",
        evidence_fixture: "tests/fixtures/synthetic/sinapi_catalog.json",
        service_contracts: service_contracts(),
        supported_metadata: vec![
            "catalog_reference",
            "unit_price",
            "currency",
            "bdi_percent",
            "source_provenance",
        ],
        loss_findings: vec![],
        current_support_boundary: "synthetic fixture-backed price catalog/BDI annotation only; no official SINAPI catalog acquisition or complete Brazil market coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}

fn prezzario_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "prezzario-computo",
        label: "Prezzario and Computo Metrico evidence",
        module: "boq_core::prezzario",
        evidence_fixture: "tests/fixtures/synthetic/prezzario_computo.json",
        service_contracts: service_contracts(),
        supported_metadata: vec![
            "price_catalog_reference",
            "quantity_reference",
            "computo_formula",
            "unit_price",
            "currency",
            "source_provenance",
        ],
        loss_findings: vec!["prezzario_computo_formula_preserved_not_evaluated"],
        current_support_boundary: "synthetic fixture-backed price/quantity annotation only; no regional Prezzario acquisition or complete Italian market coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}

fn catalogo_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "catalogo-cuadro",
        label: "Catálogo de Conceptos and Cuadro de Precios evidence",
        module: "boq_core::catalogo",
        evidence_fixture: "tests/fixtures/synthetic/catalogo_cuadro.json",
        service_contracts: service_contracts(),
        supported_metadata: vec![
            "concept_code",
            "cuadro_code",
            "market_scope",
            "price_table_kind",
            "unit_price",
            "currency",
            "source_provenance",
        ],
        loss_findings: vec!["catalogo_cuadro_price_table_preserved_not_normalized"],
        current_support_boundary: "synthetic fixture-backed concept/price-table annotation only; no Spain/Mexico catalog acquisition or complete market coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}

fn din276_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "din276-classification",
        label: "DIN 276 cost-group classification evidence",
        module: "boq_core::din276",
        evidence_fixture: "tests/fixtures/synthetic/din276_mapping.json",
        service_contracts: service_contracts(),
        supported_metadata: vec!["match_text", "rule_index", "source_provenance"],
        loss_findings: vec!["din276_classification_preserved_not_certified"],
        current_support_boundary: "synthetic fixture-backed DIN 276 classification annotation only; no official DIN data acquisition, no certification, and no complete German cost-group coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}

fn csi_masterformat_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "csi-masterformat-classification",
        label: "CSI MasterFormat classification evidence",
        module: "boq_core::csi_masterformat",
        evidence_fixture: "tests/fixtures/synthetic/masterformat_mapping.json",
        service_contracts: service_contracts(),
        supported_metadata: vec!["match_text", "rule_index", "source_provenance"],
        loss_findings: vec!["masterformat_classification_preserved_not_certified"],
        current_support_boundary: "synthetic fixture-backed CSI MasterFormat classification annotation only; no paid/external CSI data acquisition, no certification, and no complete North American market coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}

fn uniclass_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "uniclass-classification",
        label: "Uniclass classification evidence",
        module: "boq_core::uniclass",
        evidence_fixture: "tests/fixtures/synthetic/uniclass_mapping.json",
        service_contracts: service_contracts(),
        supported_metadata: vec!["match_text", "rule_index", "source_provenance"],
        loss_findings: vec!["uniclass_classification_preserved_not_certified"],
        current_support_boundary: "synthetic fixture-backed Uniclass classification annotation only; no official Uniclass catalog acquisition, no certification, and no complete UK market coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}

fn nlsfb_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "nlsfb-classification",
        label: "NL-SfB classification evidence",
        module: "boq_core::nlsfb",
        evidence_fixture: "tests/fixtures/synthetic/nlsfb_mapping.json",
        service_contracts: service_contracts(),
        supported_metadata: vec!["match_text", "rule_index", "source_provenance"],
        loss_findings: vec!["nlsfb_classification_preserved_not_certified"],
        current_support_boundary: "synthetic fixture-backed NL-SfB classification annotation only; no external NL-SfB catalog acquisition, no certification, and no complete Dutch market coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}

fn stabu_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "stabu-raw",
        label: "STABU and RAW exchange evidence",
        module: "boq_core::stabu",
        evidence_fixture: "tests/fixtures/synthetic/stabu_raw.json",
        service_contracts: service_contracts(),
        supported_metadata: vec![
            "stabu_code",
            "raw_code",
            "exchange_profile",
            "unit_price",
            "currency",
            "source_provenance",
        ],
        loss_findings: vec!["stabu_raw_exchange_metadata_preserved_not_interpreted"],
        current_support_boundary: "synthetic fixture-backed STABU/RAW annotation only; no Dutch catalog/exchange acquisition or complete market coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}

fn dqe_row() -> MarketOverlayReadinessRow {
    MarketOverlayReadinessRow {
        overlay_key: "dqe-quantity",
        label: "DQE quantity-estimate evidence",
        module: "boq_core::dqe",
        evidence_fixture: "tests/fixtures/synthetic/dqe_quantity.json",
        service_contracts: service_contracts(),
        supported_metadata: vec![
            "dqe_code",
            "quantity_reference",
            "estimate_basis",
            "calculation_note",
            "source_provenance",
        ],
        loss_findings: vec!["dqe_quantity_method_preserved_not_evaluated"],
        current_support_boundary: "synthetic fixture-backed quantity annotation only; no French catalog/quantity-estimate acquisition or complete DQE coverage",
        promotes_support_status: false,
        grants_adapter_support_to_parse_only: false,
        complete_market_coverage_claimed: false,
    }
}
