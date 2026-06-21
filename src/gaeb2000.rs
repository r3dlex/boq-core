//! GAEB 2000 / Pxx tokenizer boundary.
//!
//! This module intentionally stops at tokenization and planning diagnostics. It
//! does not parse a [`crate::model::GaebDocument`] and must not be described as
//! production GAEB 2000 support until manifest-backed fixture tests promote the
//! track.

use crate::error::ValidationFinding;
use crate::format::detect_path;
use crate::model::GaebFormat;

/// A token emitted by the GAEB 2000 tokenizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gaeb2000Token {
    /// A `#begin[...]` block opener.
    Begin {
        /// Keyword inside the brackets.
        keyword: String,
        /// One-based source line.
        line: usize,
    },
    /// A `#end[...]` block closer.
    End {
        /// Keyword inside the brackets.
        keyword: String,
        /// One-based source line.
        line: usize,
    },
    /// Any other keyword row beginning with `#`.
    Keyword {
        /// Keyword name before the first `[` or whitespace.
        keyword: String,
        /// Raw source line.
        raw: String,
        /// One-based source line.
        line: usize,
    },
}

/// Result of tokenizing GAEB 2000 text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenizeResult {
    /// Tokens extracted from keyword lines.
    pub tokens: Vec<Gaeb2000Token>,
    /// Recoverable structural diagnostics.
    pub findings: Vec<ValidationFinding>,
}

/// Tokenizes GAEB 2000 keyword rows and reports nesting diagnostics.
#[must_use]
pub fn tokenize(source: &str) -> TokenizeResult {
    let mut tokens = Vec::new();
    let mut findings = Vec::new();
    let mut stack: Vec<(String, usize)> = Vec::new();

    for (idx, raw_line) in source.lines().enumerate() {
        let line = idx + 1;
        let trimmed = raw_line.trim();
        if !trimmed.starts_with('#') {
            continue;
        }

        if let Some(keyword) = bracket_keyword(trimmed, "#begin[") {
            stack.push((keyword.clone(), line));
            tokens.push(Gaeb2000Token::Begin { keyword, line });
        } else if let Some(keyword) = bracket_keyword(trimmed, "#end[") {
            match stack.pop() {
                Some((open, _)) if open == keyword => {}
                Some((open, open_line)) => {
                    findings.push(ValidationFinding::warning(
                        "gaeb2000_mismatched_end_block",
                        format!("GAEB 2000 block {open} opened on line {open_line} but closed as {keyword}"),
                    ).at(line.to_string()));
                }
                None => findings.push(
                    ValidationFinding::warning(
                        "gaeb2000_unmatched_end_block",
                        format!("GAEB 2000 end block {keyword} has no matching begin"),
                    )
                    .at(line.to_string()),
                ),
            }
            tokens.push(Gaeb2000Token::End { keyword, line });
        } else {
            tokens.push(Gaeb2000Token::Keyword {
                keyword: scalar_keyword(trimmed),
                raw: trimmed.to_owned(),
                line,
            });
        }
    }

    for (keyword, line) in stack {
        findings.push(
            ValidationFinding::warning(
                "gaeb2000_unclosed_begin_block",
                format!("GAEB 2000 begin block {keyword} was not closed"),
            )
            .at(line.to_string()),
        );
    }

    TokenizeResult { tokens, findings }
}

/// Detects a GAEB 2000 Pxx phase from a path when the extension is P-prefixed.
#[must_use]
pub fn detect_pxx_phase(path: &str) -> Option<String> {
    let detected = detect_path(path);
    if detected.format == GaebFormat::Gaeb2000 {
        detected.phase.map(|phase| phase.code)
    } else {
        None
    }
}

fn bracket_keyword(line: &str, prefix: &str) -> Option<String> {
    line.strip_prefix(prefix)
        .and_then(|rest| rest.split(']').next())
        .filter(|keyword| !keyword.is_empty())
        .map(ToOwned::to_owned)
}

fn scalar_keyword(line: &str) -> String {
    line.trim_start_matches('#')
        .split(['[', ' ', '\t'])
        .next()
        .unwrap_or_default()
        .to_owned()
}
