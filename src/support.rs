//! Support-status metadata used to prevent overclaiming fixture or phase support.

use serde::{Deserialize, Serialize};

/// Truthful support status for a fixture, phase, or format family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportStatus {
    /// Fully supported for the stated capability set.
    Supported,
    /// Parsing is supported, but adapter/export/roundtrip may not be.
    SupportedParseOnly,
    /// Known follow-on work item.
    FutureTrack,
    /// Cataloged only; normal parser tests must not claim support.
    ReferenceOnly,
}

/// Direction-aware capabilities for a format/phase.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCapabilities {
    /// Detection support.
    pub detect: bool,
    /// Parse/import support.
    pub parse: bool,
    /// Validation support.
    pub validate: bool,
    /// Obra adapter support.
    pub adapt_to_obra: bool,
    /// Export/write support.
    pub export: bool,
    /// Roundtrip support.
    pub roundtrip: bool,
    /// Reference-only marker.
    pub reference_only: bool,
}

impl SupportCapabilities {
    /// Creates a fully supported import capability set.
    #[must_use]
    pub const fn supported_import() -> Self {
        Self {
            detect: true,
            parse: true,
            validate: true,
            adapt_to_obra: true,
            export: false,
            roundtrip: false,
            reference_only: false,
        }
    }

    /// Creates a supported import/export/roundtrip capability set.
    #[must_use]
    pub const fn supported_roundtrip() -> Self {
        Self {
            detect: true,
            parse: true,
            validate: true,
            adapt_to_obra: true,
            export: true,
            roundtrip: true,
            reference_only: false,
        }
    }

    /// Creates an import/export/roundtrip capability set without schema validation.
    ///
    /// Use this for synthetic writer-readiness tests where semantic export and
    /// reparse are covered, but local checksummed GAEB schema validation is not
    /// configured yet.
    #[must_use]
    pub const fn roundtrip_without_schema_validation() -> Self {
        Self {
            detect: true,
            parse: true,
            validate: false,
            adapt_to_obra: true,
            export: true,
            roundtrip: true,
            reference_only: false,
        }
    }

    /// Creates a parse-only capability set.
    #[must_use]
    pub const fn parse_only() -> Self {
        Self {
            detect: true,
            parse: true,
            validate: false,
            adapt_to_obra: false,
            export: false,
            roundtrip: false,
            reference_only: false,
        }
    }

    /// Creates a reference-only capability set.
    #[must_use]
    pub const fn reference_only() -> Self {
        Self {
            detect: true,
            parse: false,
            validate: false,
            adapt_to_obra: false,
            export: false,
            roundtrip: false,
            reference_only: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_only_capabilities_do_not_claim_validation_or_adapter_support() {
        let capabilities = SupportCapabilities::parse_only();
        assert!(capabilities.detect);
        assert!(capabilities.parse);
        assert!(!capabilities.validate);
        assert!(!capabilities.adapt_to_obra);
        assert!(!capabilities.export);
        assert!(!capabilities.roundtrip);
        assert!(!capabilities.reference_only);
    }

    #[test]
    fn reference_only_capabilities_do_not_claim_parser_support() {
        let capabilities = SupportCapabilities::reference_only();
        assert!(capabilities.detect);
        assert!(capabilities.reference_only);
        assert!(!capabilities.parse);
        assert!(!capabilities.validate);
        assert!(!capabilities.adapt_to_obra);
        assert!(!capabilities.export);
        assert!(!capabilities.roundtrip);
    }

    #[test]
    fn roundtrip_capabilities_keep_export_explicit() {
        let capabilities = SupportCapabilities::supported_roundtrip();
        assert!(capabilities.detect);
        assert!(capabilities.parse);
        assert!(capabilities.validate);
        assert!(capabilities.adapt_to_obra);
        assert!(capabilities.export);
        assert!(capabilities.roundtrip);
        assert!(!capabilities.reference_only);
    }

    #[test]
    fn roundtrip_without_schema_validation_keeps_validation_explicitly_false() {
        let capabilities = SupportCapabilities::roundtrip_without_schema_validation();
        assert!(capabilities.detect);
        assert!(capabilities.parse);
        assert!(!capabilities.validate);
        assert!(capabilities.adapt_to_obra);
        assert!(capabilities.export);
        assert!(capabilities.roundtrip);
        assert!(!capabilities.reference_only);
    }
}
