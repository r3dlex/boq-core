//! Service-consumable export/roundtrip boundary report.
//!
//! This module exposes the current export/update roadmap for services without
//! promoting spreadsheet, GAEB export, roundtrip, production, or certification
//! support. The only implemented evidence row is dependency-free neutral CSV,
//! matched by OZ/item ordinal and fail-closed when keys are missing.

use serde::{Deserialize, Serialize};

use crate::VERSION;

/// Schema version for [`ExportBoundaryReport`].
pub const EXPORT_BOUNDARY_SCHEMA_VERSION: &str = "boq-core.export-boundary.v1";

/// Exports the service-facing export/roundtrip boundary report.
#[must_use]
pub fn export_boundary_report() -> ExportBoundaryReport {
    ExportBoundaryReport {
        schema_version: EXPORT_BOUNDARY_SCHEMA_VERSION,
        crate_version: VERSION,
        neutral_csv: NeutralCsvBoundary {
            contract_key: "oz_matched_csv_neutral",
            implemented: true,
            export_supported: true,
            update_supported: true,
            match_key: "oz",
            fail_closed_error_codes: vec![
                "spreadsheet_neutral_missing_oz_column",
                "spreadsheet_neutral_duplicate_header",
                "spreadsheet_neutral_missing_oz_value",
                "spreadsheet_neutral_invalid_csv",
            ],
            service_contracts: vec!["boq-core.service-analyze.v1"],
            support_boundary: "dependency-free neutral CSV evidence only; no XLSX/ODS/binary spreadsheet parser and no production spreadsheet roundtrip claim",
        },
        blocked_formats: vec![
            BlockedExportFormat {
                format: "xlsx",
                reason: "reference-only spreadsheet sources are not runtime support evidence",
                implemented: false,
                production_supported: false,
            },
            BlockedExportFormat {
                format: "ods",
                reason: "no ODS parser/writer, fixtures, or production workflow are implemented",
                implemented: false,
                production_supported: false,
            },
            BlockedExportFormat {
                format: "gaeb-export-roundtrip",
                reason: "GAEB export/roundtrip remains capability-gated outside the neutral CSV helper",
                implemented: false,
                production_supported: false,
            },
        ],
        capability_flags: ExportCapabilityFlags {
            neutral_csv_export: true,
            neutral_csv_update: true,
            xlsx_export: false,
            ods_export: false,
            gaeb_export_roundtrip: false,
            production_spreadsheet_roundtrip: false,
            certification: false,
        },
        production_ready: false,
        certification_claims: Vec::new(),
        external_spreadsheet_dependency: false,
        support_boundary: "service consumers must treat export/roundtrip as bounded capability flags, not blanket support",
    }
}

/// Service-facing export/roundtrip boundary report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportBoundaryReport {
    /// Contract schema version.
    pub schema_version: &'static str,
    /// `boq-core` crate version that produced the report.
    pub crate_version: &'static str,
    /// Implemented neutral CSV evidence boundary.
    pub neutral_csv: NeutralCsvBoundary,
    /// Explicitly blocked/non-implemented export formats.
    pub blocked_formats: Vec<BlockedExportFormat>,
    /// Bounded service-facing capability flags.
    pub capability_flags: ExportCapabilityFlags,
    /// This report never claims production readiness.
    pub production_ready: bool,
    /// This report never claims certification.
    pub certification_claims: Vec<String>,
    /// No external spreadsheet dependency is required.
    pub external_spreadsheet_dependency: bool,
    /// Global boundary statement for service/Obra consumers.
    pub support_boundary: &'static str,
}

impl ExportBoundaryReport {
    /// Returns true when the report exposes only the bounded neutral CSV exchange lane.
    #[must_use]
    pub const fn supports_neutral_csv_exchange(&self) -> bool {
        self.neutral_csv.implemented
            && self.capability_flags.neutral_csv_export
            && self.capability_flags.neutral_csv_update
            && !self.capability_flags.has_binary_spreadsheet_support()
    }

    /// Looks up a blocked export format by stable key.
    #[must_use]
    pub fn blocked_format(&self, format: &str) -> Option<&BlockedExportFormat> {
        self.blocked_formats
            .iter()
            .find(|blocked| blocked.format == format)
    }

    /// Returns true when no production or certification claim is present.
    #[must_use]
    pub fn has_no_production_or_certification_claim(&self) -> bool {
        !self.production_ready
            && self.certification_claims.is_empty()
            && !self
                .capability_flags
                .has_production_or_certification_claim()
    }
}

/// Implemented neutral CSV boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeutralCsvBoundary {
    /// Stable roundtrip contract key embedded in neutral CSV metadata.
    pub contract_key: &'static str,
    /// Whether the neutral CSV helper exists.
    pub implemented: bool,
    /// Whether neutral CSV export exists.
    pub export_supported: bool,
    /// Whether neutral CSV update exists.
    pub update_supported: bool,
    /// Required row-matching key.
    pub match_key: &'static str,
    /// Stable fail-closed error codes for missing/invalid keys.
    pub fail_closed_error_codes: Vec<&'static str>,
    /// Service contracts that may surface neutral CSV metadata.
    pub service_contracts: Vec<&'static str>,
    /// Neutral CSV support boundary.
    pub support_boundary: &'static str,
}

impl NeutralCsvBoundary {
    /// Returns true when the required match key is the GAEB OZ/item ordinal.
    #[must_use]
    pub fn requires_oz_key(&self) -> bool {
        self.match_key == "oz"
    }

    /// Returns true when a stable error code is part of the fail-closed contract.
    #[must_use]
    pub fn fails_closed_for(&self, code: &str) -> bool {
        self.fail_closed_error_codes.contains(&code)
    }
}

/// Explicitly blocked/non-implemented export format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedExportFormat {
    /// Format or roadmap lane.
    pub format: &'static str,
    /// Why the format is not a runtime support claim.
    pub reason: &'static str,
    /// Whether the format is implemented today.
    pub implemented: bool,
    /// Whether production support is claimed today.
    pub production_supported: bool,
}

impl BlockedExportFormat {
    /// Returns true when the format has neither implementation nor production support.
    #[must_use]
    pub const fn is_blocked(&self) -> bool {
        !self.implemented && !self.production_supported
    }
}

/// Bounded service-facing capability flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportCapabilityFlags {
    /// Dependency-free neutral CSV export.
    pub neutral_csv_export: bool,
    /// OZ-keyed neutral CSV update.
    pub neutral_csv_update: bool,
    /// XLSX export/write support.
    pub xlsx_export: bool,
    /// ODS export/write support.
    pub ods_export: bool,
    /// GAEB export/roundtrip support.
    pub gaeb_export_roundtrip: bool,
    /// Production spreadsheet roundtrip support.
    pub production_spreadsheet_roundtrip: bool,
    /// Certification support.
    pub certification: bool,
}

impl ExportCapabilityFlags {
    /// Returns true when any binary spreadsheet export lane is claimed.
    #[must_use]
    pub const fn has_binary_spreadsheet_support(self) -> bool {
        self.xlsx_export || self.ods_export
    }

    /// Returns true when any production or certification flag is claimed.
    #[must_use]
    pub const fn has_production_or_certification_claim(self) -> bool {
        self.production_spreadsheet_roundtrip || self.certification
    }
}
