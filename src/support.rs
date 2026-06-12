//! Support-status metadata and policy seam.
//!
//! The [`SupportPolicy`] trait is the single producer of support claims for
//! this crate (ARCH-005). The [`SupportStatus`] enum and
//! [`SupportCapabilities`] struct are pure data types reused by every
//! adapter; [`SupportDecision`] bundles the policy verdict and the reason
//! behind it; and [`default_policy`] returns the embedded manifest-backed
//! adapter (optionally layered with a downgrade-only overlay loaded from
//! `BOQ_CORE_SUPPORT_OVERLAY`).

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

pub mod manifest;

use crate::model::{GaebFormat, GaebPhase};

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

/// Outcome of a [`SupportPolicy::decide`] call.
#[derive(Debug, Clone)]
pub struct SupportDecision {
    /// Status the policy concluded for the query.
    pub status: SupportStatus,
    /// Direction-aware capabilities the policy advertises.
    pub capabilities: SupportCapabilities,
    /// Human-readable reason for the decision.
    pub reason: String,
    /// Origin of the decision.
    pub source: DecisionSource,
}

/// Where a [`SupportDecision`] came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionSource {
    /// The decision was driven by a specific manifest entry.
    ManifestEntry {
        /// Fixture id that matched the query.
        id: String,
    },
    /// The query did not match any manifest entry; the policy returned the
    /// conservative default.
    ConservativeDefault,
    /// An overlay policy downgraded the base decision.
    OverlayDowngrade,
}

/// Inputs the support policy uses to decide a [`SupportDecision`].
#[derive(Debug, Clone, Copy)]
pub struct SupportQuery<'a> {
    /// Source format family of the parsed document.
    pub format: GaebFormat,
    /// Optional GAEB version string (e.g. `"3.3"`, `"GAEB 90"`).
    pub version: Option<&'a str>,
    /// Optional GAEB phase descriptor.
    pub phase: Option<&'a GaebPhase>,
    /// Optional source URI (path or URL).
    pub source_uri: Option<&'a str>,
}

/// Producer of support claims for a parsed document.
///
/// All adapters in this crate consult an implementation of this trait
/// instead of constructing [`SupportStatus`]/[`SupportCapabilities`] inline
/// (ARCH-005).
pub trait SupportPolicy: Send + Sync {
    /// Returns the policy's decision for a given query.
    fn decide(&self, query: SupportQuery<'_>) -> SupportDecision;
}

impl<T: SupportPolicy + ?Sized> SupportPolicy for &T {
    fn decide(&self, query: SupportQuery<'_>) -> SupportDecision {
        (*self).decide(query)
    }
}

/// A manifest-backed policy that uses `gaeb/manifest.toml` rows as the
/// source of truth for fixture support claims.
pub struct ManifestPolicy {
    state: ManifestState,
}

enum ManifestState {
    Indexed(Vec<IndexedEntry>),
    Failed(String),
}

#[derive(Debug, Clone)]
struct IndexedEntry {
    id: String,
    process_domain: String,
    gaeb_version: String,
    phase_code: String,
    target_dir: String,
    support_status: String,
}

impl IndexedEntry {
    fn from_entry(entry: &manifest::FixtureEntry) -> Option<Self> {
        // Only manifest rows whose version/phase keys map to XML-style codes
        // are eligible to participate in the policy decision. Non-XML rows
        // (e.g. `gaeb_90`) are filtered out; the conservative default still
        // covers them.
        let gaeb_version = manifest::gaeb_xml_version(&entry.gaeb_version)?;
        let phase_code = manifest::phase_code(&entry.phase)?;
        Some(Self {
            id: entry.id.clone(),
            process_domain: entry.process_domain.clone(),
            gaeb_version,
            phase_code,
            target_dir: manifest::normalize_source_path(&entry.target_dir),
            support_status: entry.support_status.clone(),
        })
    }
}

impl ManifestPolicy {
    /// Returns the embedded manifest-backed policy.
    ///
    /// A failure to parse the embedded manifest is captured in the policy
    /// and surfaced through [`SupportPolicy::decide`] as a conservative
    /// parse-only decision; it never panics.
    #[must_use]
    pub fn embedded() -> &'static Self {
        static EMBEDDED: OnceLock<ManifestPolicy> = OnceLock::new();
        EMBEDDED.get_or_init(|| match Self::from_toml(manifest::EMBEDDED_TOML) {
            Ok(policy) => policy,
            Err(error) => Self {
                state: ManifestState::Failed(error.to_string()),
            },
        })
    }

    /// Builds a policy from an already-parsed manifest.
    #[must_use]
    pub fn from_manifest(manifest: &manifest::FixtureManifest) -> Self {
        let indexed = manifest
            .fixtures
            .iter()
            .filter_map(IndexedEntry::from_entry)
            .collect();
        Self {
            state: ManifestState::Indexed(indexed),
        }
    }

    /// Builds a policy from a TOML manifest string.
    ///
    /// # Errors
    ///
    /// Returns a [`manifest::ManifestError`] when the TOML cannot be parsed.
    pub fn from_toml(toml_text: &str) -> Result<Self, manifest::ManifestError> {
        manifest::parse(toml_text).map(|manifest| Self::from_manifest(&manifest))
    }

    fn find_entry(&self, query: SupportQuery<'_>) -> Option<&IndexedEntry> {
        let source_uri = query.source_uri?;
        let entries = match &self.state {
            ManifestState::Indexed(entries) => entries,
            ManifestState::Failed(_) => return None,
        };
        let normalized = manifest::normalize_source_path(source_uri);
        entries
            .iter()
            .find(|entry| manifest::source_path_matches_target_dir(&normalized, &entry.target_dir))
    }
}

impl SupportPolicy for ManifestPolicy {
    fn decide(&self, query: SupportQuery<'_>) -> SupportDecision {
        if let ManifestState::Failed(error) = &self.state {
            return SupportDecision {
                status: SupportStatus::SupportedParseOnly,
                capabilities: SupportCapabilities::parse_only(),
                reason: format!(
                    "embedded GAEB fixture manifest failed to parse; support registry disabled: {error}"
                ),
                source: DecisionSource::ConservativeDefault,
            };
        }

        // Behaviour preservation: pre-refactor `gaeb_xml::support_policy`
        // was the only consumer of the registry, so GAEB 90 documents
        // never reached manifest-driven promotion. Keep that invariant
        // explicit here so future manifest rows cannot accidentally
        // promote a non-XML format.
        if !matches!(query.format, GaebFormat::GaebXml) {
            return conservative_default();
        }

        let Some(entry) = self.find_entry(query) else {
            return conservative_default();
        };

        let phase_code = query.phase.map(|phase| phase.code.as_str());
        if query.version != Some(entry.gaeb_version.as_str())
            || phase_code != Some(entry.phase_code.as_str())
        {
            return conservative_default();
        }

        let (status, capabilities, summary) = match entry.support_status.as_str() {
            "supported" if entry.process_domain == "ava" => (
                SupportStatus::Supported,
                SupportCapabilities::supported_import(),
                "supported AVA import fixture",
            ),
            "supported_parse_only" => (
                SupportStatus::SupportedParseOnly,
                SupportCapabilities::parse_only(),
                "supported parse-only fixture",
            ),
            "future_track" => (
                SupportStatus::SupportedParseOnly,
                SupportCapabilities::parse_only(),
                "future-track fixture parsed without adapter/export promotion",
            ),
            "reference_only" => (
                SupportStatus::SupportedParseOnly,
                SupportCapabilities::parse_only(),
                "reference-only fixture parsed without support promotion",
            ),
            _ => (
                SupportStatus::SupportedParseOnly,
                SupportCapabilities::parse_only(),
                "manifest fixture parsed without support promotion",
            ),
        };

        SupportDecision {
            status,
            capabilities,
            reason: format!("manifest fixture {}: {summary}", entry.id),
            source: DecisionSource::ManifestEntry {
                id: entry.id.clone(),
            },
        }
    }
}

fn conservative_default() -> SupportDecision {
    SupportDecision {
        status: SupportStatus::SupportedParseOnly,
        capabilities: SupportCapabilities::parse_only(),
        reason: "GAEB XML parsed outside manifest-backed support registry".to_owned(),
        source: DecisionSource::ConservativeDefault,
    }
}

/// A layered policy that lets an overlay downgrade (never upgrade) a base
/// policy's decision.
pub struct LayeredPolicy<B, O> {
    /// Base policy whose decision provides the upper bound.
    pub base: B,
    /// Overlay policy that may only make the decision more conservative.
    pub overlay: O,
}

impl<B: SupportPolicy, O: SupportPolicy> SupportPolicy for LayeredPolicy<B, O> {
    fn decide(&self, query: SupportQuery<'_>) -> SupportDecision {
        let base = self.base.decide(query);
        let overlay = self.overlay.decide(query);
        let base_rank = status_rank(base.status);
        let overlay_rank = status_rank(overlay.status);
        let merged_status = if overlay_rank < base_rank {
            overlay.status
        } else {
            base.status
        };
        let merged_capabilities = merge_capabilities(base.capabilities, overlay.capabilities);
        let changed = merged_status != base.status || merged_capabilities != base.capabilities;
        if changed {
            SupportDecision {
                status: merged_status,
                capabilities: merged_capabilities,
                reason: format!(
                    "overlay downgraded base decision ({}); base reason: {}; overlay reason: {}",
                    describe_status(base.status, merged_status),
                    base.reason,
                    overlay.reason,
                ),
                source: DecisionSource::OverlayDowngrade,
            }
        } else {
            base
        }
    }
}

const fn merge_capabilities(
    base: SupportCapabilities,
    overlay: SupportCapabilities,
) -> SupportCapabilities {
    SupportCapabilities {
        detect: base.detect && overlay.detect,
        parse: base.parse && overlay.parse,
        validate: base.validate && overlay.validate,
        adapt_to_obra: base.adapt_to_obra && overlay.adapt_to_obra,
        export: base.export && overlay.export,
        roundtrip: base.roundtrip && overlay.roundtrip,
        reference_only: base.reference_only || overlay.reference_only,
    }
}

const fn status_rank(status: SupportStatus) -> u8 {
    match status {
        SupportStatus::ReferenceOnly => 0,
        SupportStatus::FutureTrack => 1,
        SupportStatus::SupportedParseOnly => 2,
        SupportStatus::Supported => 3,
    }
}

fn describe_status(base: SupportStatus, merged: SupportStatus) -> String {
    if base == merged {
        format!("{base:?} preserved with capability tightening")
    } else {
        format!("{base:?} -> {merged:?}")
    }
}

/// Returns the crate's default support policy.
///
/// Reads `BOQ_CORE_SUPPORT_OVERLAY` exactly once. If it is set and points to
/// a readable TOML manifest, the result is a [`LayeredPolicy`] with the
/// embedded manifest policy as base and the overlay's [`ManifestPolicy`] as
/// downgrade-only overlay. If the variable is unset or the overlay fails to
/// load, the embedded policy is returned alone (never panics).
#[must_use]
pub fn default_policy() -> &'static dyn SupportPolicy {
    static DEFAULT: OnceLock<&'static dyn SupportPolicy> = OnceLock::new();
    *DEFAULT.get_or_init(|| {
        let embedded: &'static ManifestPolicy = ManifestPolicy::embedded();
        match std::env::var("BOQ_CORE_SUPPORT_OVERLAY") {
            Ok(path) if !path.is_empty() => {
                let overlay = std::fs::read_to_string(&path)
                    .ok()
                    .and_then(|text| ManifestPolicy::from_toml(&text).ok());
                overlay.map_or(embedded as &'static dyn SupportPolicy, |overlay| {
                    let layered: &'static LayeredPolicy<&'static ManifestPolicy, ManifestPolicy> =
                        Box::leak(Box::new(LayeredPolicy {
                            base: embedded,
                            overlay,
                        }));
                    layered as &'static dyn SupportPolicy
                })
            }
            _ => embedded as &'static dyn SupportPolicy,
        }
    })
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic, clippy::as_ptr_cast_mut)]
mod tests {
    use super::*;

    fn xml_query<'a>(
        version: Option<&'a str>,
        phase_code_str: Option<&'a str>,
        source_uri: Option<&'a str>,
    ) -> (SupportQuery<'a>, Option<GaebPhase>) {
        let phase = phase_code_str.map(|code| GaebPhase {
            code: code.to_owned(),
            label: None,
        });
        let query = SupportQuery {
            format: GaebFormat::GaebXml,
            version,
            phase: None,
            source_uri,
        };
        (query, phase)
    }

    fn run_decide<'a>(
        policy: &dyn SupportPolicy,
        format: GaebFormat,
        version: Option<&'a str>,
        phase: Option<&'a GaebPhase>,
        source_uri: Option<&'a str>,
    ) -> SupportDecision {
        policy.decide(SupportQuery {
            format,
            version,
            phase,
            source_uri,
        })
    }

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

    #[test]
    fn embedded_policy_is_a_single_instance() {
        let a = std::ptr::from_ref(ManifestPolicy::embedded());
        let b = std::ptr::from_ref(ManifestPolicy::embedded());
        assert_eq!(a, b);
    }

    #[test]
    fn conservative_default_applies_when_no_source_uri() {
        let policy = ManifestPolicy::embedded();
        let decision = policy.decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: Some("3.3"),
            phase: None,
            source_uri: None,
        });
        assert_eq!(decision.status, SupportStatus::SupportedParseOnly);
        assert!(matches!(
            decision.source,
            DecisionSource::ConservativeDefault
        ));
        assert!(
            decision
                .reason
                .contains("outside manifest-backed support registry")
        );
    }

    #[test]
    fn manifest_promotes_supported_ava_xml_x81() {
        let policy = ManifestPolicy::embedded();
        let (mut query, phase) = xml_query(
            Some("3.3"),
            Some("81"),
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/file.x81"),
        );
        query.phase = phase.as_ref();
        let decision = policy.decide(query);
        assert_eq!(decision.status, SupportStatus::Supported);
        assert!(decision.capabilities.adapt_to_obra);
        assert!(
            matches!(decision.source, DecisionSource::ManifestEntry { ref id } if id == "bvbs_xml33_ava_x81")
        );
    }

    #[test]
    fn manifest_keeps_future_track_bau_parse_only() {
        let policy = ManifestPolicy::embedded();
        let (mut query, phase) = xml_query(
            Some("3.3"),
            Some("83"),
            Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/file.x83"),
        );
        query.phase = phase.as_ref();
        let decision = policy.decide(query);
        assert_eq!(decision.status, SupportStatus::SupportedParseOnly);
        assert!(!decision.capabilities.adapt_to_obra);
        assert!(decision.reason.contains("future-track"));
    }

    #[test]
    fn manifest_keeps_non_ava_supported_status_parse_only() {
        // The manifest's `supported` -> `Supported` promotion is restricted
        // to process_domain == "ava"; a hypothetical "supported" row in
        // another domain still degrades to parse-only.
        let toml_text = r#"
[[fixtures]]
id = "synthetic_non_ava_supported"
source_url = "https://www.bvbs.de/x.zip"
normalized_url = "https://www.bvbs.de/x.zip"
source_family = "bvbs"
process_domain = "construction_execution"
gaeb_version = "gaeb_xml_3_3"
phase = "x83"
target_dir = "gaeb/synthetic/non_ava"
support_status = "supported"
ci_policy = "download_on_demand"
license_note = "synthetic"
test_mapping = ["t"]
"#;
        let policy = ManifestPolicy::from_toml(toml_text).expect("synthetic manifest parses");
        let phase = GaebPhase {
            code: "83".to_owned(),
            label: None,
        };
        let decision = policy.decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: Some("3.3"),
            phase: Some(&phase),
            source_uri: Some("gaeb/synthetic/non_ava/file.x83"),
        });
        assert_eq!(decision.status, SupportStatus::SupportedParseOnly);
    }

    #[test]
    fn gaeb90_queries_are_never_manifest_promoted() {
        let policy = ManifestPolicy::embedded();
        let phase = GaebPhase {
            code: "83".to_owned(),
            label: None,
        };
        let decision = run_decide(
            policy,
            GaebFormat::Gaeb90,
            Some("GAEB 90"),
            Some(&phase),
            Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/imposter.d83"),
        );
        assert_eq!(decision.status, SupportStatus::SupportedParseOnly);
        assert_eq!(decision.capabilities, SupportCapabilities::parse_only());
        assert!(matches!(
            decision.source,
            DecisionSource::ConservativeDefault
        ));
    }

    #[test]
    fn manifest_parse_failure_falls_back_to_parse_only() {
        let policy = ManifestPolicy {
            state: ManifestState::Failed("synthetic parse failure".to_owned()),
        };
        let decision = policy.decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: Some("3.3"),
            phase: None,
            source_uri: Some("gaeb/synthetic/x.x81"),
        });
        assert_eq!(decision.status, SupportStatus::SupportedParseOnly);
        assert!(decision.reason.contains("synthetic parse failure"));
        assert!(matches!(
            decision.source,
            DecisionSource::ConservativeDefault
        ));
    }

    #[test]
    fn layered_policy_downgrades_only() {
        struct OverridePolicy(SupportDecision);
        impl SupportPolicy for OverridePolicy {
            fn decide(&self, _query: SupportQuery<'_>) -> SupportDecision {
                self.0.clone()
            }
        }

        let base = OverridePolicy(SupportDecision {
            status: SupportStatus::Supported,
            capabilities: SupportCapabilities::supported_import(),
            reason: "base supported".to_owned(),
            source: DecisionSource::ManifestEntry {
                id: "base".to_owned(),
            },
        });
        let overlay_more_conservative = OverridePolicy(SupportDecision {
            status: SupportStatus::SupportedParseOnly,
            capabilities: SupportCapabilities::parse_only(),
            reason: "overlay parse-only".to_owned(),
            source: DecisionSource::ManifestEntry {
                id: "overlay".to_owned(),
            },
        });
        let layered = LayeredPolicy {
            base,
            overlay: overlay_more_conservative,
        };
        let phase = GaebPhase {
            code: "81".to_owned(),
            label: None,
        };
        let decision = layered.decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: Some("3.3"),
            phase: Some(&phase),
            source_uri: Some("any"),
        });
        assert_eq!(decision.status, SupportStatus::SupportedParseOnly);
        assert!(matches!(decision.source, DecisionSource::OverlayDowngrade));
        assert!(decision.reason.contains("base reason"));
    }

    #[test]
    fn layered_policy_never_upgrades() {
        struct OverridePolicy(SupportDecision);
        impl SupportPolicy for OverridePolicy {
            fn decide(&self, _query: SupportQuery<'_>) -> SupportDecision {
                self.0.clone()
            }
        }
        let base = OverridePolicy(SupportDecision {
            status: SupportStatus::SupportedParseOnly,
            capabilities: SupportCapabilities::parse_only(),
            reason: "base parse-only".to_owned(),
            source: DecisionSource::ConservativeDefault,
        });
        let overlay_more_optimistic = OverridePolicy(SupportDecision {
            status: SupportStatus::Supported,
            capabilities: SupportCapabilities::supported_import(),
            reason: "overlay supported".to_owned(),
            source: DecisionSource::ManifestEntry {
                id: "overlay".to_owned(),
            },
        });
        let layered = LayeredPolicy {
            base,
            overlay: overlay_more_optimistic,
        };
        let decision = layered.decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: Some("3.3"),
            phase: None,
            source_uri: None,
        });
        assert_eq!(decision.status, SupportStatus::SupportedParseOnly);
        // No upgrade and no overlay-driven change, so the source must come
        // straight from the base.
        assert!(matches!(
            decision.source,
            DecisionSource::ConservativeDefault
        ));
    }

    #[test]
    fn layered_policy_tightens_capabilities_field_wise() {
        struct OverridePolicy(SupportDecision);
        impl SupportPolicy for OverridePolicy {
            fn decide(&self, _query: SupportQuery<'_>) -> SupportDecision {
                self.0.clone()
            }
        }
        let base = OverridePolicy(SupportDecision {
            status: SupportStatus::Supported,
            capabilities: SupportCapabilities::supported_import(),
            reason: "base".to_owned(),
            source: DecisionSource::ManifestEntry {
                id: "base".to_owned(),
            },
        });
        let overlay = OverridePolicy(SupportDecision {
            status: SupportStatus::Supported,
            capabilities: SupportCapabilities {
                detect: true,
                parse: true,
                validate: false,
                adapt_to_obra: false,
                export: false,
                roundtrip: false,
                reference_only: false,
            },
            reason: "overlay tightens".to_owned(),
            source: DecisionSource::ManifestEntry {
                id: "overlay".to_owned(),
            },
        });
        let layered = LayeredPolicy { base, overlay };
        let decision = layered.decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: None,
            phase: None,
            source_uri: None,
        });
        assert_eq!(decision.status, SupportStatus::Supported);
        assert!(!decision.capabilities.adapt_to_obra);
        assert!(!decision.capabilities.validate);
        assert!(matches!(decision.source, DecisionSource::OverlayDowngrade));
    }

    #[test]
    fn layered_policy_reference_only_capability_is_or() {
        struct OverridePolicy(SupportDecision);
        impl SupportPolicy for OverridePolicy {
            fn decide(&self, _query: SupportQuery<'_>) -> SupportDecision {
                self.0.clone()
            }
        }
        let base = OverridePolicy(SupportDecision {
            status: SupportStatus::Supported,
            capabilities: SupportCapabilities::supported_import(),
            reason: "base".to_owned(),
            source: DecisionSource::ManifestEntry {
                id: "base".to_owned(),
            },
        });
        let overlay = OverridePolicy(SupportDecision {
            status: SupportStatus::ReferenceOnly,
            capabilities: SupportCapabilities::reference_only(),
            reason: "overlay reference".to_owned(),
            source: DecisionSource::ManifestEntry {
                id: "overlay".to_owned(),
            },
        });
        let layered = LayeredPolicy { base, overlay };
        let decision = layered.decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: None,
            phase: None,
            source_uri: None,
        });
        assert_eq!(decision.status, SupportStatus::ReferenceOnly);
        assert!(decision.capabilities.reference_only);
    }

    #[test]
    fn default_policy_returns_a_usable_instance() {
        let phase = GaebPhase {
            code: "81".to_owned(),
            label: None,
        };
        let decision = default_policy().decide(SupportQuery {
            format: GaebFormat::GaebXml,
            version: Some("3.3"),
            phase: Some(&phase),
            source_uri: Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/file.x81"),
        });
        assert!(matches!(
            decision.status,
            SupportStatus::Supported | SupportStatus::SupportedParseOnly
        ));
    }
}
