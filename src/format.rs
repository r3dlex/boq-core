//! Format and phase detection utilities.
//!
//! Format detection is path/extension based and intentionally advisory: callers
//! still need support-status/capability checks after parsing.
//!
//! ```
//! let detected = boq_core::format::detect_path("example.D83");
//! assert_eq!(detected.format, boq_core::model::GaebFormat::Gaeb90);
//! assert_eq!(detected.phase.as_ref().map(|phase| phase.code.as_str()), Some("83"));
//! ```

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::model::{GaebFormat, GaebPhase};

/// Detected GAEB source information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedFormat {
    /// Format family.
    pub format: GaebFormat,
    /// Optional phase.
    pub phase: Option<GaebPhase>,
    /// Lowercase source extension without a leading dot.
    pub extension: Option<String>,
}

/// Detects GAEB format and phase from a file path.
#[must_use]
pub fn detect_path(path: impl AsRef<Path>) -> DetectedFormat {
    let extension = path
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase);

    let (format, phase) = extension
        .as_deref()
        .map_or((GaebFormat::GaebXml, None), detect_extension);

    DetectedFormat {
        format,
        phase,
        extension,
    }
}

fn detect_extension(extension: &str) -> (GaebFormat, Option<GaebPhase>) {
    let mut chars = extension.chars();
    let Some(prefix) = chars.next() else {
        return (GaebFormat::GaebXml, None);
    };
    let phase_code = chars.collect::<String>();
    let phase = if phase_code.chars().all(|ch| ch.is_ascii_digit()) && !phase_code.is_empty() {
        Some(GaebPhase {
            code: phase_code,
            label: None,
        })
    } else {
        None
    };
    let format = match prefix {
        'd' => GaebFormat::Gaeb90,
        'p' => GaebFormat::Gaeb2000,
        _ => GaebFormat::GaebXml,
    };
    (format, phase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_gaeb_xml_phase_case_insensitively() {
        let detected = detect_path("sample.X81");
        assert_eq!(detected.format, GaebFormat::GaebXml);
        assert_eq!(
            detected.phase.map(|phase| phase.code),
            Some("81".to_owned())
        );
    }

    #[test]
    fn detects_gaeb90_phase() {
        let detected = detect_path("sample.D83");
        assert_eq!(detected.format, GaebFormat::Gaeb90);
        assert_eq!(
            detected.phase.map(|phase| phase.code),
            Some("83".to_owned())
        );
    }

    #[test]
    fn detects_gaeb2000_phase() {
        let detected = detect_path("sample.P86");
        assert_eq!(detected.format, GaebFormat::Gaeb2000);
        assert_eq!(
            detected.phase.map(|phase| phase.code),
            Some("86".to_owned())
        );
    }

    #[test]
    fn handles_paths_without_parseable_phase() {
        let detected = detect_path("README");
        assert_eq!(detected.format, GaebFormat::GaebXml);
        assert!(detected.phase.is_none());

        let detected = detect_path("sample.xml");
        assert_eq!(detected.format, GaebFormat::GaebXml);
        assert!(detected.phase.is_none());
    }

    #[test]
    fn empty_extensions_default_to_gaeb_xml_without_phase() {
        let (format, phase) = detect_extension("");
        assert_eq!(format, GaebFormat::GaebXml);
        assert!(phase.is_none());
    }
}
