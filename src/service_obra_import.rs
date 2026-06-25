//! Service-consumable Obra import DTO conversion contract.
//!
//! This module wraps the existing [`crate::adapter::obra`] DTO with a stable
//! JSON report for `boq-service` and Obra callers. It does not grant adapter
//! support, export, roundtrip, production readiness, or certification status;
//! it only serializes the current manifest-backed parser/adapter decision.

use serde::{Deserialize, Serialize};

use crate::VERSION;
use crate::adapter::obra::ObraImportDocument;
use crate::error::{ParseError, ValidationFinding};
use crate::model::{GaebDocument, SourceProvenance};
use crate::service_contract::AnalyzeFormatHint;
use crate::support::{SupportCapabilities, SupportStatus};

/// Schema version for [`ServiceObraImportReport`].
pub const SERVICE_OBRA_IMPORT_SCHEMA_VERSION: &str = "boq-core.obra-import.v1";

/// Input to the service Obra import boundary.
#[derive(Debug, Clone)]
pub struct ObraImportInput<'a> {
    /// Raw source bytes.
    pub bytes: &'a [u8],
    /// Optional source URI/path for provenance and format detection.
    pub source_uri: Option<String>,
    /// Optional explicit format hint.
    pub format_hint: Option<AnalyzeFormatHint>,
}

/// Top-level status for the service Obra import report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObraImportStatus {
    /// Parse and adapter conversion produced an Obra import document.
    Ok,
    /// Parse succeeded but support/capability policy blocked adapter conversion.
    Blocked,
    /// Parse failed before adapter conversion could be evaluated.
    Error,
}

/// Stable adapter rejection codes for service consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObraAdapterRejectionCode {
    /// The document is only supported for parse/import analysis, not Obra DTO conversion.
    ObraAdapterSupportedParseOnly,
    /// The source is planned future work and must fail closed today.
    ObraAdapterFutureTrack,
    /// The source is reference-only evidence and must not be parsed/adapted as support.
    ObraAdapterReferenceOnly,
    /// The status/capability combination does not allow Obra adapter conversion.
    ObraAdapterNotSupported,
}

/// Service-facing adapter rejection detail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObraAdapterRejection {
    /// Stable rejection code.
    pub code: ObraAdapterRejectionCode,
    /// Human-readable explanation.
    pub message: String,
    /// Source support status that caused or contributed to the block.
    pub support_status: SupportStatus,
    /// Capability flags at the time adapter conversion was evaluated.
    pub capabilities: SupportCapabilities,
}

/// Deterministic service-facing Obra import report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceObraImportReport {
    /// Contract schema version.
    pub schema_version: &'static str,
    /// `boq-core` crate version that produced the report.
    pub crate_version: &'static str,
    /// Conversion status.
    pub status: ObraImportStatus,
    /// Source provenance when parsing succeeded.
    pub provenance: Option<SourceProvenance>,
    /// Fixture/phase support status when parsing succeeded.
    pub support_status: Option<SupportStatus>,
    /// Direction-aware support capabilities when parsing succeeded.
    pub capabilities: Option<SupportCapabilities>,
    /// Obra import DTO when status is `ok`.
    pub import_document: Option<ObraImportDocument>,
    /// Recoverable parser/adapter diagnostics and loss findings.
    pub diagnostics: Vec<ValidationFinding>,
    /// Adapter rejection detail when status is `blocked`.
    pub rejection: Option<ObraAdapterRejection>,
    /// Unrecoverable parser error when status is `error`.
    pub error: Option<ParseError>,
    /// This contract never claims production readiness.
    pub production_ready: bool,
    /// This contract never claims certification by itself.
    pub certification_claims: Vec<String>,
}

/// Parse bytes and attempt service-facing Obra import DTO conversion.
#[must_use]
pub fn convert_bytes_to_obra_import(input: &ObraImportInput<'_>) -> ServiceObraImportReport {
    match parse_input(input) {
        Ok(document) => report_from_document(&document),
        Err(error) => ServiceObraImportReport {
            schema_version: SERVICE_OBRA_IMPORT_SCHEMA_VERSION,
            crate_version: VERSION,
            status: ObraImportStatus::Error,
            provenance: None,
            support_status: None,
            capabilities: None,
            import_document: None,
            diagnostics: Vec::new(),
            rejection: None,
            error: Some(error),
            production_ready: false,
            certification_claims: Vec::new(),
        },
    }
}

/// Attempt service-facing Obra import DTO conversion for an already parsed document.
#[must_use]
pub fn report_from_document(document: &GaebDocument) -> ServiceObraImportReport {
    if document.support_status != SupportStatus::Supported {
        return blocked_report(document);
    }
    match ObraImportDocument::try_from_gaeb(document) {
        Ok(import_document) => ServiceObraImportReport {
            schema_version: SERVICE_OBRA_IMPORT_SCHEMA_VERSION,
            crate_version: VERSION,
            status: ObraImportStatus::Ok,
            provenance: Some(document.source.clone()),
            support_status: Some(document.support_status),
            capabilities: Some(document.capabilities),
            import_document: Some(import_document),
            diagnostics: document.findings.clone(),
            rejection: None,
            error: None,
            production_ready: false,
            certification_claims: Vec::new(),
        },
        Err(adapter_error) => blocked_report_with_adapter_error(document, adapter_error),
    }
}

fn blocked_report(document: &GaebDocument) -> ServiceObraImportReport {
    blocked_report_with_adapter_error(
        document,
        ValidationFinding::warning(
            "obra_adapter_not_supported",
            "document support status does not allow service Obra import conversion",
        ),
    )
}

fn blocked_report_with_adapter_error(
    document: &GaebDocument,
    adapter_error: ValidationFinding,
) -> ServiceObraImportReport {
    let mut diagnostics = document.findings.clone();
    diagnostics.push(adapter_error);
    ServiceObraImportReport {
        schema_version: SERVICE_OBRA_IMPORT_SCHEMA_VERSION,
        crate_version: VERSION,
        status: ObraImportStatus::Blocked,
        provenance: Some(document.source.clone()),
        support_status: Some(document.support_status),
        capabilities: Some(document.capabilities),
        import_document: None,
        diagnostics,
        rejection: Some(adapter_rejection(document)),
        error: None,
        production_ready: false,
        certification_claims: Vec::new(),
    }
}

fn parse_input(input: &ObraImportInput<'_>) -> Result<GaebDocument, ParseError> {
    match resolve_format(input.format_hint, input.source_uri.as_deref()) {
        AnalyzeFormatHint::GaebXml => std::str::from_utf8(input.bytes)
            .map_err(|error| ParseError {
                code: "xml_decode_failed".to_owned(),
                message: error.to_string(),
                location: input.source_uri.clone(),
            })
            .and_then(|source| crate::gaeb_xml::parse_str(source, input.source_uri.clone())),
        AnalyzeFormatHint::Gaeb90 => {
            crate::gaeb90::parse_bytes(input.bytes, input.source_uri.clone())
        }
    }
}

fn resolve_format(hint: Option<AnalyzeFormatHint>, source_uri: Option<&str>) -> AnalyzeFormatHint {
    if let Some(hint) = hint {
        return hint;
    }
    let detected = source_uri.map(crate::format::detect_path);
    match detected.map(|d| d.format) {
        Some(crate::model::GaebFormat::Gaeb90) => AnalyzeFormatHint::Gaeb90,
        _ => AnalyzeFormatHint::GaebXml,
    }
}

fn adapter_rejection(document: &GaebDocument) -> ObraAdapterRejection {
    let code = match document.support_status {
        SupportStatus::SupportedParseOnly => {
            ObraAdapterRejectionCode::ObraAdapterSupportedParseOnly
        }
        SupportStatus::FutureTrack => ObraAdapterRejectionCode::ObraAdapterFutureTrack,
        SupportStatus::ReferenceOnly => ObraAdapterRejectionCode::ObraAdapterReferenceOnly,
        SupportStatus::Supported => ObraAdapterRejectionCode::ObraAdapterNotSupported,
    };
    ObraAdapterRejection {
        code,
        message: match code {
            ObraAdapterRejectionCode::ObraAdapterSupportedParseOnly => {
                "document is parse-only for this source; Obra adapter conversion is disabled"
            }
            ObraAdapterRejectionCode::ObraAdapterFutureTrack => {
                "document belongs to a future-track source; Obra adapter conversion is disabled"
            }
            ObraAdapterRejectionCode::ObraAdapterReferenceOnly => {
                "document belongs to a reference-only source; Obra adapter conversion is disabled"
            }
            ObraAdapterRejectionCode::ObraAdapterNotSupported => {
                "document capabilities do not allow Obra adapter conversion"
            }
        }
        .to_owned(),
        support_status: document.support_status,
        capabilities: document.capabilities,
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn unit_report_from_document_blocks_parse_only_with_stable_code() {
        let document = crate::gaeb90::parse_bytes(
            include_bytes!("../tests/fixtures/synthetic/minimal.d81"),
            Some("tests/fixtures/synthetic/minimal.d81".to_owned()),
        )
        .expect("minimal d81 parses");

        let report = report_from_document(&document);

        assert_eq!(report.status, ObraImportStatus::Blocked);
        assert_eq!(
            report.rejection.expect("rejection").code,
            ObraAdapterRejectionCode::ObraAdapterSupportedParseOnly
        );
        assert!(report.import_document.is_none());
        assert!(!report.production_ready);
        assert!(report.certification_claims.is_empty());
    }

    #[test]
    fn unit_report_from_document_blocks_parse_only_even_when_adapter_capability_is_true() {
        let mut document = crate::gaeb90::parse_bytes(
            include_bytes!("../tests/fixtures/synthetic/minimal.d81"),
            Some("tests/fixtures/synthetic/minimal.d81".to_owned()),
        )
        .expect("minimal d81 parses");
        document.support_status = SupportStatus::SupportedParseOnly;
        document.capabilities = SupportCapabilities::parse_with_obra_adapter();

        let report = report_from_document(&document);

        assert_eq!(report.status, ObraImportStatus::Blocked);
        assert_eq!(
            report.rejection.expect("rejection").code,
            ObraAdapterRejectionCode::ObraAdapterSupportedParseOnly
        );
        assert!(report.import_document.is_none());
    }

    #[test]
    fn unit_convert_bytes_reports_parse_errors_without_support_promotion() {
        let report = convert_bytes_to_obra_import(&ObraImportInput {
            bytes: b"<GAEB><",
            source_uri: Some("broken.x81".to_owned()),
            format_hint: Some(AnalyzeFormatHint::GaebXml),
        });

        assert_eq!(report.status, ObraImportStatus::Error);
        assert_eq!(report.error.expect("error").code, "xml_parse_failed");
        assert!(report.import_document.is_none());
        assert!(report.rejection.is_none());
        assert!(!report.production_ready);
        assert!(report.certification_claims.is_empty());
    }

    #[test]
    fn unit_convert_bytes_reports_utf8_decode_errors() {
        let report = convert_bytes_to_obra_import(&ObraImportInput {
            bytes: &[0xff, 0xfe],
            source_uri: Some("broken.x81".to_owned()),
            format_hint: Some(AnalyzeFormatHint::GaebXml),
        });

        assert_eq!(report.status, ObraImportStatus::Error);
        assert_eq!(report.error.expect("error").code, "xml_decode_failed");
        assert!(report.import_document.is_none());
        assert!(report.rejection.is_none());
    }
}
