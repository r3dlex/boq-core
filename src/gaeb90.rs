//! GAEB 90 fixed-width parser foundation.

use std::collections::BTreeMap;
use std::path::Path;

use encoding_rs::{UTF_8, WINDOWS_1252};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::VERSION;
use crate::checksum::sha256_hex;
use crate::error::{ParseError, ValidationFinding};
use crate::format::detect_path;
use crate::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, GaebDocument, GaebDocumentSummary, GaebFormat, RichText,
    SourceProvenance,
};
use crate::support::SupportQuery;

/// Character decoding policy for GAEB 90 byte input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gaeb90Encoding {
    /// Try UTF-8 first, then fall back to Windows-1252 with a finding.
    Auto,
    /// Decode as UTF-8 and report replacement characters as findings.
    Utf8,
    /// Decode as legacy ANSI/Windows-1252 and report replacement characters as findings.
    Windows1252,
}

impl Gaeb90Encoding {
    const fn label(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Utf8 => "utf-8",
            Self::Windows1252 => "windows-1252",
        }
    }
}

#[derive(Debug)]
struct DecodedText {
    text: String,
    effective_encoding: Gaeb90Encoding,
    findings: Vec<ValidationFinding>,
}

/// A decoded GAEB 90 record preserving fixed-width source fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gaeb90Record {
    /// One-based physical line number.
    pub physical_line: usize,
    /// Two-character Satzart / record type.
    pub satzart: String,
    /// Payload from positions 3 through 74.
    pub payload: String,
    /// Six-character source line id from positions 75 through 80.
    pub line_id: Option<String>,
    /// Raw decoded line.
    pub raw_line: String,
}

/// Parses a GAEB 90 file from disk.
///
/// # Errors
///
/// Returns a parse error when the file cannot be read or decoded.
pub fn parse_file(path: impl AsRef<Path>) -> Result<GaebDocument, ParseError> {
    let path_ref = path.as_ref();
    let bytes = std::fs::read(path_ref).map_err(|error| ParseError {
        code: "gaeb90_read_failed".to_owned(),
        message: error.to_string(),
        location: Some(path_ref.display().to_string()),
    })?;
    parse_bytes(&bytes, Some(path_ref.display().to_string()))
}

/// Parses GAEB 90 bytes with automatic UTF-8/Windows-1252 detection.
///
/// Use [`parse_bytes_with_encoding`] when a caller or upstream file catalog
/// already knows the legacy text encoding. Automatic mode preserves the
/// original byte checksum, reports UTF-8 fallback as a recoverable finding,
/// and records the effective decoder in document metadata.
///
/// # Errors
///
/// Returns a parse error when no decodable records can be produced.
pub fn parse_bytes(bytes: &[u8], source_uri: Option<String>) -> Result<GaebDocument, ParseError> {
    parse_bytes_with_encoding(bytes, source_uri, Gaeb90Encoding::Auto)
}

/// Parses GAEB 90 bytes with a caller-specified text encoding policy.
///
/// This entrypoint is intended for legacy GAEB 90 files whose catalog, file
/// source, or user-selected import setting identifies ANSI/Windows-1252 before
/// parsing. Invalid byte sequences are decoded with replacement characters and
/// reported as structured `gaeb90_decode_replacement` findings.
///
/// # Errors
///
/// Returns a parse error when no decodable records can be produced.
pub fn parse_bytes_with_encoding(
    bytes: &[u8],
    source_uri: Option<String>,
    encoding: Gaeb90Encoding,
) -> Result<GaebDocument, ParseError> {
    let decoded = decode_bytes(bytes, encoding);
    let phase = source_uri.as_deref().and_then(|uri| detect_path(uri).phase);
    let records = parse_records(&decoded.text);
    if records.is_empty() {
        return Err(ParseError {
            code: "gaeb90_no_records".to_owned(),
            message: "GAEB 90 source did not contain records".to_owned(),
            location: source_uri,
        });
    }

    let mut findings = decoded.findings;
    findings.extend(
        records
            .iter()
            .filter(|record| record.raw_line.chars().count() != 80)
            .map(|record| {
                ValidationFinding::warning(
                    "gaeb90_line_length",
                    format!(
                        "line {} has {} chars, expected 80",
                        record.physical_line,
                        record.raw_line.chars().count()
                    ),
                )
                .at(record.physical_line.to_string())
            }),
    );

    findings.extend(records.iter().filter_map(malformed_item_ordinal_finding));

    let nodes = records_to_nodes(&records);
    let title = records
        .iter()
        .find(|record| record.satzart == "02")
        .map(|record| record.payload.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "GAEB 90 BoQ".to_owned());

    let support = crate::support::default_policy().decide(SupportQuery {
        format: GaebFormat::Gaeb90,
        version: Some("GAEB 90"),
        phase: phase.as_ref(),
        source_uri: source_uri.as_deref(),
    });

    let mut boq_metadata = BTreeMap::new();
    boq_metadata.insert(
        "gaeb.support_policy".to_owned(),
        serde_json::json!({
            "status": support.status,
            "reason": support.reason,
        }),
    );

    Ok(GaebDocument {
        source: SourceProvenance {
            source_uri,
            source_format: GaebFormat::Gaeb90,
            gaeb_version: Some("GAEB 90".to_owned()),
            phase: phase.clone(),
            checksum: Some(sha256_hex(bytes)),
            parser_version: VERSION.to_owned(),
        },
        summary: GaebDocumentSummary {
            format: GaebFormat::Gaeb90,
            version: Some("GAEB 90".to_owned()),
            phase,
            title: Some(title.clone()),
            project_name: Some(title.clone()),
        },
        boq: Boq {
            title,
            nodes,
            currency: None,
            metadata: boq_metadata,
        },
        capabilities: support.capabilities,
        support_status: support.status,
        findings,
        metadata: BTreeMap::from([(
            "gaeb90.encoding".to_owned(),
            serde_json::json!(decoded.effective_encoding.label()),
        )]),
    })
}

/// Parses decoded GAEB 90 text into records.
#[must_use]
pub fn parse_records(source: &str) -> Vec<Gaeb90Record> {
    source
        .lines()
        .enumerate()
        .filter_map(|(index, line)| parse_record(index + 1, line))
        .collect()
}

fn parse_record(physical_line: usize, line: &str) -> Option<Gaeb90Record> {
    if line.trim().is_empty() {
        return None;
    }
    let chars = line.chars().collect::<Vec<_>>();
    let satzart = chars.iter().take(2).collect::<String>();
    let payload = chars.iter().skip(2).take(72).collect::<String>();
    let line_id = (chars.len() >= 80).then(|| chars.iter().skip(74).take(6).collect::<String>());
    Some(Gaeb90Record {
        physical_line,
        satzart,
        payload,
        line_id,
        raw_line: line.to_owned(),
    })
}

fn malformed_item_ordinal_finding(record: &Gaeb90Record) -> Option<ValidationFinding> {
    if record.satzart != "21" {
        return None;
    }

    let ordinal = record.payload.chars().take(9).collect::<String>();
    if ordinal.trim().is_empty() {
        Some(
            ValidationFinding::warning(
                "gaeb90_malformed_ordinal",
                "item ordinal field was blank; a stable fallback ordinal was used",
            )
            .at(record.physical_line.to_string()),
        )
    } else {
        None
    }
}

fn records_to_nodes(records: &[Gaeb90Record]) -> Vec<BoqNode> {
    let mut nodes = Vec::new();
    let mut current_item: Option<BoqNode> = None;

    for record in records {
        match record.satzart.as_str() {
            "21" => {
                if let Some(item) = current_item.take() {
                    nodes.push(item);
                }
                let ordinal = record
                    .payload
                    .chars()
                    .take(9)
                    .collect::<String>()
                    .trim()
                    .to_owned();
                current_item = Some(BoqNode {
                    ordinal: if ordinal.is_empty() {
                        format!("item_{}", record.physical_line)
                    } else {
                        ordinal
                    },
                    title: String::new(),
                    kind: BoqNodeKind::Item,
                    children: Vec::new(),
                    item: Some(BoqItem {
                        short_text: String::new(),
                        long_text: None,
                        quantity: Decimal::ZERO,
                        unit: record
                            .payload
                            .chars()
                            .skip(43)
                            .take(4)
                            .collect::<String>()
                            .trim()
                            .to_owned(),
                        unit_price: None,
                        total_price: None,
                        notes: None,
                        metadata: BTreeMap::new(),
                    }),
                    sort_order: i32::try_from(nodes.len()).unwrap_or(i32::MAX),
                    metadata: BTreeMap::new(),
                });
            }
            "25" => {
                if let Some(item) = &mut current_item {
                    let text = record.payload.trim().to_owned();
                    item.title.clone_from(&text);
                    if let Some(payload) = &mut item.item {
                        payload.short_text = text;
                    }
                }
            }
            "26" => {
                if let Some(item) = &mut current_item {
                    let text = record.payload.trim().to_owned();
                    if let Some(payload) = &mut item.item {
                        payload.long_text = Some(match payload.long_text.take() {
                            Some(RichText::Plain(existing)) => {
                                RichText::Plain(format!("{existing}\n{text}"))
                            }
                            _ => RichText::Plain(text),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(item) = current_item {
        nodes.push(item);
    }

    nodes
}

fn decode_bytes(bytes: &[u8], encoding: Gaeb90Encoding) -> DecodedText {
    match encoding {
        Gaeb90Encoding::Auto => decode_bytes_auto(bytes),
        Gaeb90Encoding::Utf8 => decode_with_encoding(bytes, Gaeb90Encoding::Utf8),
        Gaeb90Encoding::Windows1252 => decode_with_encoding(bytes, Gaeb90Encoding::Windows1252),
    }
}

fn decode_bytes_auto(bytes: &[u8]) -> DecodedText {
    let (utf8, _, had_errors) = UTF_8.decode(bytes);
    if had_errors {
        let mut decoded = decode_with_encoding(bytes, Gaeb90Encoding::Windows1252);
        decoded.findings.insert(
            0,
            ValidationFinding::warning(
                "gaeb90_encoding_fallback",
                "input was not valid UTF-8; decoded as Windows-1252",
            ),
        );
        decoded
    } else {
        DecodedText {
            text: utf8.into_owned(),
            effective_encoding: Gaeb90Encoding::Utf8,
            findings: Vec::new(),
        }
    }
}

fn decode_with_encoding(bytes: &[u8], encoding: Gaeb90Encoding) -> DecodedText {
    let (text, _, had_errors) = match encoding {
        Gaeb90Encoding::Utf8 => UTF_8.decode(bytes),
        Gaeb90Encoding::Windows1252 => WINDOWS_1252.decode(bytes),
        Gaeb90Encoding::Auto => {
            unreachable!("auto decoding is handled before fixed decoder dispatch")
        }
    };
    let mut findings = Vec::new();
    if had_errors {
        findings.push(ValidationFinding::warning(
            "gaeb90_decode_replacement",
            format!(
                "{} decoding replaced invalid byte sequences",
                encoding.label()
            ),
        ));
    }

    DecodedText {
        text: text.into_owned(),
        effective_encoding: encoding,
        findings,
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::support::SupportStatus;

    #[test]
    fn parses_fixed_width_record_fields() {
        let line =
            "2101010020  10000Psch                                                     000008";
        let record = parse_record(1, line).expect("record should parse");
        assert_eq!(record.satzart, "21");
        assert_eq!(record.line_id.as_deref(), Some("000008"));
        assert_eq!(record.raw_line.chars().count(), 80);
    }

    #[test]
    fn parses_synthetic_d81_as_parse_only() {
        let document = parse_bytes(
            include_bytes!("../tests/fixtures/synthetic/minimal.d81"),
            Some("minimal.d81".to_owned()),
        )
        .expect("synthetic D81 should parse");
        assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
        assert!(
            document.boq.metadata.contains_key("gaeb.support_policy"),
            "gaeb90 boq metadata must carry gaeb.support_policy provenance"
        );
        let policy_meta = &document.boq.metadata["gaeb.support_policy"];
        assert!(
            policy_meta.get("status").is_some(),
            "gaeb.support_policy must include status"
        );
        assert!(
            policy_meta.get("reason").is_some(),
            "gaeb.support_policy must include reason"
        );
        assert_eq!(
            document
                .source
                .phase
                .as_ref()
                .map(|phase| phase.code.as_str()),
            Some("81")
        );
        assert_eq!(document.boq.nodes.len(), 1);
        assert!(!document.capabilities.validate);
        assert!(document.source.checksum.is_some());
    }

    #[test]
    fn reports_empty_input_as_parse_error() {
        let error = parse_bytes(b"   \r\n", Some("empty.d83".to_owned()))
            .expect_err("empty source should fail");
        assert_eq!(error.code, "gaeb90_no_records");
        assert_eq!(error.location.as_deref(), Some("empty.d83"));
    }

    #[test]
    fn missing_file_reports_read_error() {
        let error = parse_file("tests/fixtures/synthetic/does-not-exist.d81")
            .expect_err("missing file should fail");
        assert_eq!(error.code, "gaeb90_read_failed");
    }

    #[test]
    fn malformed_lines_are_recoverable_findings() {
        let document = parse_bytes(
            b"21\r\n25Short text without active complete width\r\n",
            Some("broken.d83".to_owned()),
        )
        .expect("recoverable malformed lines should parse");
        assert!(
            document
                .findings
                .iter()
                .any(|finding| finding.code == "gaeb90_line_length")
        );
        assert!(
            document
                .findings
                .iter()
                .any(|finding| finding.code == "gaeb90_malformed_ordinal")
        );
        assert_eq!(document.boq.nodes[0].ordinal, "item_1");
    }

    #[test]
    fn ignores_blank_records() {
        let records = parse_records("\r\n21abc\r\n   \r\n");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].satzart, "21");
    }
}
