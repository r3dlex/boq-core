//! Error and finding types shared by parsers, validators, adapters, and fixture tools.
//!
//! Public API callers should treat [`ParseError`] as an unrecoverable failure:
//! the requested source could not be read, decoded, or parsed into a
//! [`crate::model::GaebDocument`]. The parser did not produce a usable document
//! in that case.
//!
//! [`ValidationFinding`] is recoverable diagnostic evidence attached to a parsed
//! document or returned by an adapter gate. A finding has a stable finding code,
//! human-readable message, severity, and optional source location. Findings are
//! how `boq-core` reports malformed lines, unsupported adapter conversion,
//! lossy mappings, and certification/readiness gaps without pretending the
//! whole parse failed.

use serde::{Deserialize, Serialize};

/// A severity for validation, adapter, or certification findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Informational finding.
    Info,
    /// Recoverable warning.
    Warning,
    /// Error finding that may still be represented in reports.
    Error,
}

/// A recoverable finding with optional source location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Severity of the finding.
    pub severity: Severity,
    /// Stable finding code for tests and reports.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Optional source path, line, byte span, or XPath-like location.
    pub location: Option<String>,
}

impl ValidationFinding {
    /// Creates a warning finding.
    #[must_use]
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code: code.into(),
            message: message.into(),
            location: None,
        }
    }

    /// Adds source-location metadata to the finding.
    #[must_use]
    pub fn at(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }
}

/// Unrecoverable parser error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseError {
    /// Stable error code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Optional source location.
    pub location: Option<String>,
}

/// Fixture acquisition or verification error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureError {
    /// Stable error code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
}

/// Certification criterion mapping result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationFinding {
    /// Criterion identifier.
    pub criterion: String,
    /// Finding severity.
    pub severity: Severity,
    /// Evidence or reason.
    pub evidence: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_builder_sets_severity_and_location() {
        let finding = ValidationFinding::warning("line_length", "bad line").at("42");
        assert_eq!(finding.severity, Severity::Warning);
        assert_eq!(finding.code, "line_length");
        assert_eq!(finding.message, "bad line");
        assert_eq!(finding.location.as_deref(), Some("42"));
    }
}
