//! Service-consumable support/capability manifest export.
//!
//! This module projects the embedded fixture manifest into a deterministic JSON
//! report for services. It is evidence and policy metadata only: exporting a row
//! never downloads external data, never promotes support status, and never claims
//! certification or production readiness.

use serde::{Deserialize, Serialize};

use crate::VERSION;
use crate::support::SupportCapabilities;
use crate::support::manifest::{self, FixtureEntry, FixtureManifest, ManifestSupportStatus};

/// Schema version for [`ServiceSupportManifestReport`].
pub const SERVICE_SUPPORT_MANIFEST_SCHEMA_VERSION: &str = "boq-core.support-manifest.v1";

/// Canonical support vocabulary exported to services.
pub const SUPPORT_VOCABULARY: [&str; 4] = [
    "supported",
    "supported_parse_only",
    "future_track",
    "reference_only",
];

/// Service-facing support manifest report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceSupportManifestReport {
    /// Contract schema version.
    pub schema_version: &'static str,
    /// `boq-core` crate version that produced the report.
    pub crate_version: &'static str,
    /// Canonical support-status vocabulary in stable order.
    pub support_vocabulary: Vec<&'static str>,
    /// Fixture/source policy rows.
    pub entries: Vec<ServiceSupportManifestEntry>,
    /// This export never claims production readiness.
    pub production_ready: bool,
    /// This export never claims certification.
    pub certification_claims: Vec<String>,
}

/// A single service-facing fixture/source policy row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceSupportManifestEntry {
    /// Stable fixture/source id.
    pub fixture_id: String,
    /// Source family grouping.
    pub source_family: String,
    /// Process domain grouping.
    pub process_domain: String,
    /// GAEB version key from the manifest.
    pub gaeb_version: String,
    /// GAEB phase key from the manifest.
    pub phase: String,
    /// Fixture target directory under `gaeb/`.
    pub target_dir: String,
    /// Exact manifest support vocabulary.
    pub support_status: ManifestSupportStatus,
    /// Direction-aware capabilities for service gating.
    pub capabilities: ServiceSupportCapabilities,
    /// Source/CI/license policy that services can display without fetching data.
    pub source_policy: ServiceSourcePolicy,
    /// Evidence tests mapped to supported claims.
    pub test_mapping: Vec<String>,
}

/// Service-facing capability flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceSupportCapabilities {
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
    /// Official certification support.
    pub certification: bool,
    /// Reference-only marker.
    pub reference_only: bool,
}

impl From<SupportCapabilities> for ServiceSupportCapabilities {
    fn from(capabilities: SupportCapabilities) -> Self {
        Self {
            detect: capabilities.detect,
            parse: capabilities.parse,
            validate: capabilities.validate,
            adapt_to_obra: capabilities.adapt_to_obra,
            export: capabilities.export,
            roundtrip: capabilities.roundtrip,
            certification: false,
            reference_only: capabilities.reference_only,
        }
    }
}

/// Source policy exported without requiring services to download fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceSourcePolicy {
    /// CI/acquisition policy keyword from the manifest.
    pub ci_policy: String,
    /// License/redistribution note from the manifest.
    pub license_note: String,
    /// Whether the manifest row has a pinned archive checksum.
    pub has_archive_sha256: bool,
    /// Services can consume this export without fetching external data.
    pub service_export_requires_external_download: bool,
}

/// Exports the embedded manifest as a service-facing support report.
///
/// # Errors
///
/// Returns a manifest parse error if the embedded manifest cannot be parsed.
pub fn export_embedded_support_manifest()
-> Result<ServiceSupportManifestReport, manifest::ManifestError> {
    let manifest = manifest::parse(manifest::EMBEDDED_TOML)?;
    Ok(export_support_manifest(&manifest))
}

/// Exports a parsed manifest as a service-facing support report.
#[must_use]
pub fn export_support_manifest(manifest: &FixtureManifest) -> ServiceSupportManifestReport {
    let mut entries: Vec<_> = manifest.fixtures.iter().map(export_entry).collect();
    entries.sort_by(|left, right| left.fixture_id.cmp(&right.fixture_id));
    ServiceSupportManifestReport {
        schema_version: SERVICE_SUPPORT_MANIFEST_SCHEMA_VERSION,
        crate_version: VERSION,
        support_vocabulary: SUPPORT_VOCABULARY.to_vec(),
        entries,
        production_ready: false,
        certification_claims: Vec::new(),
    }
}

fn export_entry(entry: &FixtureEntry) -> ServiceSupportManifestEntry {
    ServiceSupportManifestEntry {
        fixture_id: entry.id.clone(),
        source_family: entry.source_family.clone(),
        process_domain: entry.process_domain.clone(),
        gaeb_version: entry.gaeb_version.clone(),
        phase: entry.phase.clone(),
        target_dir: entry.target_dir.clone(),
        support_status: entry.support_status,
        capabilities: capabilities_for_entry(entry),
        source_policy: ServiceSourcePolicy {
            ci_policy: entry.ci_policy.clone(),
            license_note: entry.license_note.clone(),
            has_archive_sha256: entry.archive_sha256.is_some(),
            service_export_requires_external_download: false,
        },
        test_mapping: entry.test_mapping.clone(),
    }
}

fn capabilities_for_entry(entry: &FixtureEntry) -> ServiceSupportCapabilities {
    let capabilities = match entry.support_status {
        ManifestSupportStatus::Supported if entry.process_domain == "ava" => {
            SupportCapabilities::supported_import()
        }
        ManifestSupportStatus::Supported if entry.process_domain == "gaeb90_examples" => {
            SupportCapabilities::supported_import()
        }
        ManifestSupportStatus::SupportedParseOnly
            if entry.process_domain == "construction_execution" =>
        {
            SupportCapabilities::parse_with_obra_adapter()
        }
        ManifestSupportStatus::Supported | ManifestSupportStatus::SupportedParseOnly => {
            SupportCapabilities::parse_only()
        }
        ManifestSupportStatus::FutureTrack | ManifestSupportStatus::ReferenceOnly => {
            SupportCapabilities::reference_only()
        }
    };
    capabilities.into()
}
