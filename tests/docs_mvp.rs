#![allow(missing_docs, clippy::expect_used)]

use std::fs;
use std::path::Path;

#[test]
fn mdbook_documentation_mvp_scaffold_exists() {
    for path in [
        "book.toml",
        "docs/book/SUMMARY.md",
        "docs/book/index.md",
        "docs/book/user-guide.md",
        "docs/book/developer-guide.md",
        "docs/book/certification-evidence-guide.md",
        "docs/book/release-guide.md",
    ] {
        assert!(
            Path::new(path).exists(),
            "missing documentation artifact: {path}"
        );
    }
}

#[test]
fn documentation_guides_cover_required_mvp_topics() {
    let user_guide = fs::read_to_string("docs/book/user-guide.md").expect("user guide exists");
    for topic in ["GAEB 90", "GAEB DA XML", "D81", "D83", "X81", "X83", "Obra"] {
        assert!(
            user_guide.contains(topic),
            "user guide missing topic: {topic}"
        );
    }

    let developer_guide =
        fs::read_to_string("docs/book/developer-guide.md").expect("developer guide exists");
    for topic in ["TDD", "95%", "manifest", "SupportStatus", "Obra adapter"] {
        assert!(
            developer_guide.contains(topic),
            "developer guide missing topic: {topic}"
        );
    }

    let certification_guide = fs::read_to_string("docs/book/certification-evidence-guide.md")
        .expect("certification guide exists");
    for topic in [
        "BVBS",
        "GAEBXmlChecker",
        "reference_only",
        "paid certification",
    ] {
        assert!(
            certification_guide.contains(topic),
            "certification guide missing topic: {topic}"
        );
    }

    let release_guide =
        fs::read_to_string("docs/book/release-guide.md").expect("release guide exists");
    for topic in ["semver", "crates.io", "protected main", "full-green PR"] {
        assert!(
            release_guide.contains(topic),
            "release guide missing topic: {topic}"
        );
    }
}

#[test]
fn docs_do_not_overclaim_certification_or_future_formats() {
    let mut combined = String::new();
    for path in [
        "README.md",
        "src/lib.rs",
        "docs/book/index.md",
        "docs/book/user-guide.md",
        "docs/book/developer-guide.md",
        "docs/book/certification-evidence-guide.md",
        "docs/book/release-guide.md",
    ] {
        combined.push_str(&fs::read_to_string(path).expect("guide exists"));
        combined.push('\n');
    }

    // Exact sentinels intentionally cover the highest-risk support and certification
    // phrases; human review covers broader paraphrases in the certification checklist.
    for forbidden in [
        "officially BVBS certified",
        "paid certification completed",
        "X31 is supported",
        "X31 support is included",
        "X83 is supported",
        "X83 support is included",
        "X89 is supported",
        "X89 support is included",
        "GAEB XML 3.4 is supported",
        "GAEB XML 3.4 production support",
    ] {
        assert!(
            !combined.contains(forbidden),
            "documentation overclaims unsupported status: {forbidden}"
        );
    }
}
