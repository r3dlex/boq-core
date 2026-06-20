//! Release automation policy tests.

use std::{error::Error, fs};

#[test]
fn cargo_metadata_contains_release_fields() -> Result<(), Box<dyn Error>> {
    let cargo_toml = fs::read_to_string("Cargo.toml")?;

    for required in [
        "name = \"boq-core\"",
        "version = ",
        "edition = \"2024\"",
        "rust-version = \"1.85\"",
        "description = ",
        "license = ",
        "repository = ",
        "readme = ",
    ] {
        assert!(
            cargo_toml.contains(required),
            "Cargo.toml is missing release metadata snippet: {required}"
        );
    }

    Ok(())
}

#[test]
fn release_workflow_is_dry_run_safe() -> Result<(), Box<dyn Error>> {
    let workflow = fs::read_to_string(".github/workflows/release-dry-run.yml")?;

    for required in [
        "workflow_dispatch:",
        "pull_request:",
        "Cargo.toml",
        "CHANGELOG.md",
        "docs/book/release-guide.md",
        "tests/release_automation.rs",
        "cargo package --locked",
        "cargo publish --dry-run --locked",
    ] {
        assert!(
            workflow.contains(required),
            "release workflow is missing required dry-run snippet: {required}"
        );
    }

    for line in workflow.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || !trimmed.contains("cargo publish") {
            continue;
        }
        assert!(
            trimmed.contains("--dry-run"),
            "release workflow must never publish without --dry-run: {trimmed}"
        );
    }

    Ok(())
}

#[test]
fn publish_requires_manual_authorization() -> Result<(), Box<dyn Error>> {
    let guide = fs::read_to_string("docs/book/release-guide.md")?.to_lowercase();

    for required in [
        "pre-1.0",
        "manual",
        "authorization",
        "crates.io",
        "dry-run",
        "support-status",
    ] {
        assert!(
            guide.contains(required),
            "release guide is missing required policy term: {required}"
        );
    }

    assert!(
        guide.contains("must not publish by default")
            || guide.contains("publishing remains gated until explicitly authorized"),
        "release guide must state that automation does not publish by default"
    );

    Ok(())
}

#[test]
fn changelog_mentions_support_status_changes() -> Result<(), Box<dyn Error>> {
    let changelog = fs::read_to_string("CHANGELOG.md")?.to_lowercase();

    for required in [
        "unreleased",
        "support status changes",
        "fixture",
        "tests or gates",
        "certification",
    ] {
        assert!(
            changelog.contains(required),
            "CHANGELOG.md is missing required release-planning term: {required}"
        );
    }

    Ok(())
}
