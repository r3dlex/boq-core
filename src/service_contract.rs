//! Service-consumable parse/analyze JSON contract.
//!
//! This module is the stable boundary intended for `boq-service`. It wraps the
//! existing parser APIs into a deterministic, serde-serializable report without
//! changing support status, granting Obra adapter support, or claiming
//! production/certification readiness.

use serde::{Deserialize, Serialize};

use crate::VERSION;
use crate::error::{ParseError, ValidationFinding};
use crate::model::{GaebDocumentSummary, SourceProvenance};
use crate::support::{SupportCapabilities, SupportStatus};

/// Schema version for [`ServiceAnalyzeReport`].
pub const SERVICE_ANALYZE_SCHEMA_VERSION: &str = "boq-core.service-analyze.v1";

/// Caller-provided source format hint for service analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnalyzeFormatHint {
    /// GAEB DA XML input.
    GaebXml,
    /// GAEB 90 fixed-width byte input.
    Gaeb90,
}

impl AnalyzeFormatHint {
    /// Parses a CLI/API format label.
    #[must_use]
    pub fn parse(label: &str) -> Option<Self> {
        match label {
            "gaeb-xml" | "gaeb_xml" | "xml" => Some(Self::GaebXml),
            "gaeb90" | "gaeb-90" | "d81" | "d83" => Some(Self::Gaeb90),
            _ => None,
        }
    }
}

/// Input to the service analyze boundary.
#[derive(Debug, Clone)]
pub struct AnalyzeInput<'a> {
    /// Raw source bytes.
    pub bytes: &'a [u8],
    /// Optional source URI/path for provenance and format detection.
    pub source_uri: Option<String>,
    /// Optional explicit format hint.
    pub format_hint: Option<AnalyzeFormatHint>,
}

/// Top-level status for the service analyze report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalyzeStatus {
    /// Parse/analyze produced a document.
    Ok,
    /// Parse/analyze failed before a usable document could be produced.
    Error,
}

/// Document details exposed to services.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDocumentReport {
    /// Parsed document summary.
    pub summary: GaebDocumentSummary,
    /// Source provenance.
    pub provenance: SourceProvenance,
    /// Fixture/phase support status.
    pub support_status: SupportStatus,
    /// Direction-aware support capabilities.
    pub capabilities: SupportCapabilities,
    /// Number of top-level BoQ nodes.
    pub top_level_node_count: usize,
}

/// Deterministic service-facing analyze report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceAnalyzeReport {
    /// Contract schema version.
    pub schema_version: &'static str,
    /// `boq-core` crate version that produced the report.
    pub crate_version: &'static str,
    /// Analyze status.
    pub status: AnalyzeStatus,
    /// Parsed document report when status is `ok`.
    pub document: Option<ServiceDocumentReport>,
    /// Recoverable diagnostics/loss findings.
    pub diagnostics: Vec<ValidationFinding>,
    /// Unrecoverable parser error when status is `error`.
    pub error: Option<ParseError>,
    /// This contract never claims production readiness.
    pub production_ready: bool,
    /// This contract never claims certification by itself.
    pub certification_claims: Vec<String>,
}

/// Analyze bytes through the service-facing JSON contract.
#[must_use]
pub fn analyze_bytes(input: &AnalyzeInput<'_>) -> ServiceAnalyzeReport {
    let parsed = match resolve_format(input.format_hint, input.source_uri.as_deref()) {
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
    };

    match parsed {
        Ok(document) => ServiceAnalyzeReport {
            schema_version: SERVICE_ANALYZE_SCHEMA_VERSION,
            crate_version: VERSION,
            status: AnalyzeStatus::Ok,
            document: Some(ServiceDocumentReport {
                summary: document.summary,
                provenance: document.source,
                support_status: document.support_status,
                capabilities: document.capabilities,
                top_level_node_count: document.boq.nodes.len(),
            }),
            diagnostics: document.findings,
            error: None,
            production_ready: false,
            certification_claims: Vec::new(),
        },
        Err(error) => ServiceAnalyzeReport {
            schema_version: SERVICE_ANALYZE_SCHEMA_VERSION,
            crate_version: VERSION,
            status: AnalyzeStatus::Error,
            document: None,
            diagnostics: Vec::new(),
            error: Some(error),
            production_ready: false,
            certification_claims: Vec::new(),
        },
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
