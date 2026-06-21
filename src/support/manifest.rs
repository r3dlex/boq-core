//! Typed fixture manifest schema shared by the library, xtask, and tests.
//!
//! This module is the single owner of the `gaeb/manifest.toml` schema
//! (ARCH-005). The library consumes it through support-policy adapters;
//! xtask uses it for download/lockfile/checksum bookkeeping; tests assert
//! catalog coverage on it.

use std::fmt;

use serde::Deserialize;

/// Embedded canonical fixture manifest TOML.
///
/// The path is relative to this source file (`src/support/manifest.rs` ->
/// `gaeb/manifest.toml`).
pub const EMBEDDED_TOML: &str = include_str!("../../gaeb/manifest.toml");

/// Strongly typed deserialization of `gaeb/manifest.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct FixtureManifest {
    /// All fixture rows.
    pub fixtures: Vec<FixtureEntry>,
}

/// A single fixture row from `gaeb/manifest.toml`.
///
/// This mirrors the full superset of fields used by both the library (for
/// support-policy decisions) and xtask (for downloads and lockfile checks).
#[derive(Debug, Clone, Deserialize)]
pub struct FixtureEntry {
    /// Stable fixture identifier.
    pub id: String,
    /// Origin URL of the upstream fixture.
    pub source_url: String,
    /// Normalized, allowlisted HTTPS URL used by the downloader.
    pub normalized_url: String,
    /// Source family grouping (e.g. `bvbs`, `official_gaeb`).
    pub source_family: String,
    /// Process domain (e.g. `ava`, `construction_execution`).
    pub process_domain: String,
    /// GAEB version key (e.g. `gaeb_xml_3_3`, `gaeb_90`).
    pub gaeb_version: String,
    /// GAEB phase key (e.g. `x81`, `d83`, `pdf_reference`).
    pub phase: String,
    /// Target directory under `gaeb/` where the unpacked payload lives.
    pub target_dir: String,
    /// Support status (`supported` | `supported_parse_only` | `future_track` | `reference_only`).
    pub support_status: String,
    /// CI policy keyword.
    pub ci_policy: String,
    /// License/redistribution note.
    pub license_note: String,
    /// Tests that must exist for this fixture's support claim.
    pub test_mapping: Vec<String>,
    /// Optional pinned archive checksum (otherwise resolved via `gaeb/fixtures.lock`).
    pub archive_sha256: Option<String>,
}

/// Manifest parse error.
#[derive(Debug)]
pub struct ManifestError(String);

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ManifestError {}

impl From<toml::de::Error> for ManifestError {
    fn from(error: toml::de::Error) -> Self {
        Self(error.to_string())
    }
}

impl ManifestError {
    /// Returns the underlying error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.0
    }
}

/// A semantic manifest issue surfaced by [`validate`].
#[derive(Debug, Clone)]
pub struct ManifestIssue {
    /// Fixture id the issue is attached to, when applicable.
    pub fixture_id: Option<String>,
    /// Human-readable issue description.
    pub message: String,
}

impl fmt::Display for ManifestIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.fixture_id {
            Some(id) => write!(f, "{id}: {}", self.message),
            None => f.write_str(&self.message),
        }
    }
}

/// Parses a TOML manifest text into a [`FixtureManifest`].
///
/// # Errors
///
/// Returns a [`ManifestError`] when the TOML cannot be deserialized.
pub fn parse(toml_text: &str) -> Result<FixtureManifest, ManifestError> {
    toml::from_str(toml_text).map_err(ManifestError::from)
}

/// Runs pure-string-level validation on a parsed manifest.
///
/// This covers the parser-relevant rules absorbed from xtask's
/// `verify_manifest`: duplicate ids, google-search-wrapped URLs, required
/// non-empty fields, valid `support_status`, supported/parse-only rows must
/// have non-empty `test_mapping`, identity safety (id charset, target_dir
/// staying under `gaeb/`), and HTTPS-allowlist URL checks.
///
/// Filesystem, lockfile, and checksum checks remain in xtask.
///
/// # Errors
///
/// Returns the list of issues if any are found; returns `Ok(())` otherwise.
#[allow(clippy::result_large_err)]
pub fn validate(manifest: &FixtureManifest) -> Result<(), Vec<ManifestIssue>> {
    let mut issues = Vec::new();
    let mut seen_ids = std::collections::BTreeSet::new();

    for fixture in &manifest.fixtures {
        if !seen_ids.insert(fixture.id.as_str()) {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message: "duplicate fixture id".to_owned(),
            });
        }
        if fixture.normalized_url.contains("google.com/search") {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message: "google-search-wrapped URL".to_owned(),
            });
        }
        if let Err(message) = validate_identity(fixture) {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message,
            });
        }
        if let Err(message) = validate_url(&fixture.normalized_url) {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message,
            });
        }
        if fixture.source_url.is_empty()
            || fixture.normalized_url.is_empty()
            || fixture.source_family.is_empty()
            || fixture.process_domain.is_empty()
            || fixture.gaeb_version.is_empty()
            || fixture.phase.is_empty()
            || fixture.license_note.is_empty()
        {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message: "missing required metadata".to_owned(),
            });
        }
        if !matches!(
            fixture.support_status.as_str(),
            "supported" | "supported_parse_only" | "future_track" | "reference_only"
        ) {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message: format!("invalid support_status: {}", fixture.support_status),
            });
        }
        if matches!(
            fixture.support_status.as_str(),
            "supported" | "supported_parse_only"
        ) && fixture.test_mapping.is_empty()
        {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message: "supported fixture lacks test_mapping".to_owned(),
            });
        }
        if std::path::Path::new(&fixture.normalized_url)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
            && fixture.support_status != "reference_only"
        {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message: "executable must be reference_only".to_owned(),
            });
        }
        if fixture.ci_policy == "reference_only" && !fixture.test_mapping.is_empty() {
            issues.push(ManifestIssue {
                fixture_id: Some(fixture.id.clone()),
                message: "reference_only ci_policy must not map tests".to_owned(),
            });
        }
    }

    if issues.is_empty() {
        Ok(())
    } else {
        Err(issues)
    }
}

/// Validates the identity fields of a single fixture entry.
///
/// Checks that `id` uses only lowercase ASCII letters, digits, and underscores,
/// and that `target_dir` stays within `gaeb/` with no path-traversal components.
/// Returns an error string describing the first violation found.
///
/// # Errors
///
/// Returns a descriptive error string when the entry fails identity validation.
pub fn validate_identity(fixture: &FixtureEntry) -> Result<(), String> {
    if fixture.id.is_empty()
        || !fixture
            .id
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Err(format!("invalid fixture id: {}", fixture.id));
    }
    let target_dir = std::path::Path::new(&fixture.target_dir);
    if target_dir.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_)
        )
    }) {
        return Err(format!("unsafe target_dir: {}", fixture.target_dir));
    }
    if target_dir.is_absolute() || !target_dir.starts_with("gaeb") {
        return Err(format!(
            "target_dir must stay under gaeb/: {}",
            fixture.target_dir
        ));
    }
    Ok(())
}

fn validate_url(url: &str) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err(format!("fixture URL must use https: {url}"));
    }
    let host = url
        .strip_prefix("https://")
        .and_then(|rest| rest.split('/').next())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let allowed = [
        "www.bvbs.de",
        "www.gaeb.de",
        "github.com",
        "gist.github.com",
        "www.gaeb-online.de",
    ];
    if !allowed.contains(&host.as_str()) {
        return Err(format!("fixture host is not allowlisted: {host}"));
    }
    Ok(())
}

/// Maps the manifest `gaeb_version` key to the GAEB XML version string used
/// by the parser (e.g. `gaeb_xml_3_3` -> `3.3`).
///
/// Returns `None` for non-XML versions such as `gaeb_90`.
#[must_use]
pub fn gaeb_xml_version(value: &str) -> Option<String> {
    if value == "gaeb_xml_3_4_beta" {
        return Some("3.4".to_owned());
    }
    value
        .strip_prefix("gaeb_xml_3_")
        .map(|suffix| format!("3.{suffix}"))
}

/// Maps the manifest `phase` key to the phase code string used by the parser
/// (e.g. `x81` -> `81`).
///
/// Returns `None` for non-numeric phase keys such as `pdf_reference` or
/// `schema`.
#[must_use]
pub fn phase_code(value: &str) -> Option<String> {
    value
        .strip_prefix('x')
        .filter(|code| !code.is_empty() && code.chars().all(|ch| ch.is_ascii_digit()))
        .map(ToOwned::to_owned)
}

/// Normalizes a source URI for fixture path matching.
#[must_use]
pub fn normalize_source_path(value: &str) -> String {
    value
        .replace('\\', "/")
        .trim_start_matches("./")
        .trim_end_matches('/')
        .to_ascii_lowercase()
}

/// Returns `true` when a normalized source path matches a fixture's
/// `target_dir`.
#[must_use]
pub fn source_path_matches_target_dir(source_path: &str, target_dir: &str) -> bool {
    source_path == target_dir
        || source_path.starts_with(&format!("{target_dir}/"))
        || source_path.ends_with(&format!("/{target_dir}"))
        || source_path.contains(&format!("/{target_dir}/"))
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn embedded_manifest_parses() {
        let manifest = parse(EMBEDDED_TOML).expect("embedded manifest parses");
        assert!(
            manifest
                .fixtures
                .iter()
                .any(|f| f.id == "bvbs_xml33_ava_x81"),
            "expected canonical AVA X81 row"
        );
    }

    #[test]
    fn embedded_manifest_validates_cleanly() {
        let manifest = parse(EMBEDDED_TOML).expect("embedded manifest parses");
        if let Err(issues) = validate(&manifest) {
            panic!("embedded manifest validation issues: {issues:?}");
        }
    }

    #[test]
    fn validate_flags_duplicate_ids() {
        let toml_text = r#"
[[fixtures]]
id = "dup"
source_url = "https://www.bvbs.de/a.zip"
normalized_url = "https://www.bvbs.de/a.zip"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/a"
support_status = "supported"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = ["t"]

[[fixtures]]
id = "dup"
source_url = "https://www.bvbs.de/b.zip"
normalized_url = "https://www.bvbs.de/b.zip"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/b"
support_status = "supported"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = ["t"]
"#;
        let manifest = parse(toml_text).expect("parses");
        let issues = validate(&manifest).expect_err("duplicate id should fail");
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("duplicate"))
        );
    }

    #[test]
    fn validate_flags_invalid_support_status() {
        let toml_text = r#"
[[fixtures]]
id = "bad_status"
source_url = "https://www.bvbs.de/a.zip"
normalized_url = "https://www.bvbs.de/a.zip"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/a"
support_status = "totally_supported"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = []
"#;
        let manifest = parse(toml_text).expect("parses");
        let issues = validate(&manifest).expect_err("bad status should fail");
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("invalid support_status"))
        );
    }

    #[test]
    fn validate_flags_missing_test_mapping_for_supported() {
        let toml_text = r#"
[[fixtures]]
id = "no_tests"
source_url = "https://www.bvbs.de/a.zip"
normalized_url = "https://www.bvbs.de/a.zip"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/a"
support_status = "supported"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = []
"#;
        let manifest = parse(toml_text).expect("parses");
        let issues = validate(&manifest).expect_err("missing test_mapping should fail");
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("lacks test_mapping"))
        );
    }

    #[test]
    fn validate_flags_google_search_url_and_non_https() {
        let toml_text = r#"
[[fixtures]]
id = "bad_url"
source_url = "http://www.bvbs.de/a.zip"
normalized_url = "http://google.com/search?q=gaeb"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/a"
support_status = "supported"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = ["t"]
"#;
        let manifest = parse(toml_text).expect("parses");
        let issues = validate(&manifest).expect_err("bad url should fail");
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("google-search-wrapped"))
        );
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("must use https"))
        );
    }

    #[test]
    fn validate_flags_unsafe_target_dir_and_invalid_id() {
        let toml_text = r#"
[[fixtures]]
id = "Bad-ID"
source_url = "https://www.bvbs.de/a.zip"
normalized_url = "https://www.bvbs.de/a.zip"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/a"
support_status = "supported"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = ["t"]

[[fixtures]]
id = "outside_target"
source_url = "https://www.bvbs.de/b.zip"
normalized_url = "https://www.bvbs.de/b.zip"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "../outside"
support_status = "supported"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = ["t"]
"#;
        let manifest = parse(toml_text).expect("parses");
        let issues = validate(&manifest).expect_err("bad identity should fail");
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("invalid fixture id"))
        );
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("unsafe target_dir"))
        );
    }

    #[test]
    fn validate_flags_executable_outside_reference_only() {
        let toml_text = r#"
[[fixtures]]
id = "bad_exe"
source_url = "https://www.bvbs.de/a.exe"
normalized_url = "https://www.bvbs.de/a.exe"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/a"
support_status = "supported"
ci_policy = "download_on_demand"
license_note = "x"
test_mapping = ["t"]
"#;
        let manifest = parse(toml_text).expect("parses");
        let issues = validate(&manifest).expect_err("exe outside reference_only should fail");
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("executable must be reference_only"))
        );
    }

    #[test]
    fn validate_flags_reference_only_ci_with_mapped_tests() {
        let toml_text = r#"
[[fixtures]]
id = "ref_with_tests"
source_url = "https://www.bvbs.de/a.zip"
normalized_url = "https://www.bvbs.de/a.zip"
source_family = "bvbs"
process_domain = "ava"
gaeb_version = "gaeb_xml_3_3"
phase = "x81"
target_dir = "gaeb/test/a"
support_status = "reference_only"
ci_policy = "reference_only"
license_note = "x"
test_mapping = ["t"]
"#;
        let manifest = parse(toml_text).expect("parses");
        let issues = validate(&manifest).expect_err("reference_only ci with tests should fail");
        assert!(issues.iter().any(|issue| {
            issue
                .message
                .contains("reference_only ci_policy must not map tests")
        }));
    }

    #[test]
    fn gaeb_xml_version_maps_xml_only() {
        assert_eq!(gaeb_xml_version("gaeb_xml_3_3").as_deref(), Some("3.3"));
        assert_eq!(gaeb_xml_version("gaeb_90"), None);
    }

    #[test]
    fn phase_code_maps_numeric_x_phases_only() {
        assert_eq!(phase_code("x81").as_deref(), Some("81"));
        assert_eq!(phase_code("schema"), None);
        assert_eq!(phase_code("x"), None);
    }

    #[test]
    fn normalize_source_path_lowercases_and_trims() {
        assert_eq!(
            normalize_source_path("./Gaeb\\BVBS\\GAEB_XML_3_3/AVA/x81/"),
            "gaeb/bvbs/gaeb_xml_3_3/ava/x81"
        );
    }

    #[test]
    fn source_path_matches_target_dir_handles_substring_paths() {
        assert!(source_path_matches_target_dir(
            "gaeb/bvbs/gaeb_xml_3_3/ava/x81/file.x81",
            "gaeb/bvbs/gaeb_xml_3_3/ava/x81"
        ));
        assert!(!source_path_matches_target_dir(
            "gaeb/bvbs/gaeb_xml_3_3/not_ava/x81/file.x81",
            "gaeb/bvbs/gaeb_xml_3_3/ava/x81"
        ));
    }

    #[test]
    fn manifest_error_displays_inner_message() {
        let error = parse("not toml = = =").expect_err("parse fails");
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn manifest_issue_display_with_and_without_id() {
        let with_id = ManifestIssue {
            fixture_id: Some("foo".into()),
            message: "bad".into(),
        };
        assert_eq!(with_id.to_string(), "foo: bad");
        let without_id = ManifestIssue {
            fixture_id: None,
            message: "broken".into(),
        };
        assert_eq!(without_id.to_string(), "broken");
    }
}
