//! X31 quantity takeoff domain model.
//!
//! This module intentionally models measurement/progress data separately from
//! [`crate::model::BoqItem`]. It is a domain boundary for future X31 parsing and
//! X31/X86 baseline linking; it does not claim parser support or BVBS
//! certification readiness by itself.
//!
//! ```
//! use rust_decimal::Decimal;
//! use boq_core::x31::{MeasurementFormula, MeasurementRow, RebFormulaSystem};
//!
//! let row = MeasurementRow::formula(
//!     "row-1",
//!     "001.0010",
//!     "m",
//!     MeasurementFormula::reb_vb_23003("2.5 * 4.0"),
//! )
//! .with_result(Decimal::new(100, 1));
//!
//! assert_eq!(row.ordinal.as_deref(), Some("001.0010"));
//! assert_eq!(row.formula.system, RebFormulaSystem::RebVb23003);
//! ```

use std::collections::BTreeMap;
use std::str::Chars;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::checksum::sha256_hex;
use crate::error::{ParseError, ValidationFinding};
use crate::format::detect_path;
use crate::model::{GaebFormat, Metadata, SourceProvenance};

/// Parses a GAEB XML X31 quantity takeoff payload into the X31 domain model.
///
/// This MVP parser preserves formula source text and basic result metadata. It
/// reports unsupported row-level constructs as findings instead of panicking or
/// pretending to evaluate formulas.
///
/// # Errors
///
/// Returns a parse error when XML cannot be decoded or a numeric result cannot
/// be parsed.
pub fn parse_str(
    source: &str,
    source_uri: Option<String>,
) -> Result<QuantityTakeoffDocument, ParseError> {
    X31Parser::new(source).parse(source_uri, Some(sha256_hex(source.as_bytes())))
}

/// Parses an X31 XML file from disk.
///
/// # Errors
///
/// Returns a parse error when the file cannot be read or parsed.
pub fn parse_file(
    path: impl AsRef<std::path::Path>,
) -> Result<QuantityTakeoffDocument, ParseError> {
    let path_ref = path.as_ref();
    let source = std::fs::read_to_string(path_ref).map_err(|error| ParseError {
        code: "x31_read_failed".to_owned(),
        message: error.to_string(),
        location: Some(path_ref.display().to_string()),
    })?;
    parse_str(&source, Some(path_ref.display().to_string()))
}

/// Explicit MVP subset supported by [`evaluate_reb_vb_23003`].
///
/// The evaluator intentionally supports only deterministic arithmetic needed by
/// local X31 proof fixtures. It does not call a scripting engine and it does not
/// implement the full REB-VB 23.003 grammar.
pub const SUPPORTED_REB_VB_23003_SUBSET: &[&str] = &[
    "decimal numbers with dot or comma separators",
    "parentheses",
    "unary plus and minus",
    "addition",
    "subtraction",
    "multiplication",
    "division",
];

/// Result of safely evaluating a REB-VB 23.003 formula subset.
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaEvaluation {
    /// Deterministic quantity when the expression is fully supported.
    pub quantity: Option<Decimal>,
    /// Structured findings explaining unsupported or invalid expressions.
    pub findings: Vec<ValidationFinding>,
}

impl FormulaEvaluation {
    const fn quantity(quantity: Decimal) -> Self {
        Self {
            quantity: Some(quantity),
            findings: Vec::new(),
        }
    }

    fn unevaluated(code: &str, message: impl Into<String>, location: impl Into<String>) -> Self {
        Self {
            quantity: None,
            findings: vec![ValidationFinding::warning(code, message).at(location.into())],
        }
    }
}

/// Evaluates the safe arithmetic subset of REB-VB 23.003 formulas.
///
/// Unsupported tokens, syntax errors, overflow, and division by zero return a
/// structured finding with no quantity. This function never uses unsafe or
/// dynamic expression execution.
#[must_use]
pub fn evaluate_reb_vb_23003(expression: &str) -> FormulaEvaluation {
    let trimmed = expression.trim();
    if trimmed.is_empty() {
        return FormulaEvaluation::unevaluated(
            "reb_formula_empty",
            "REB-VB formula expression is empty",
            "formula",
        );
    }

    let mut parser = FormulaParser::new(trimmed);
    match parser.parse_expression() {
        Ok(quantity) => {
            parser.skip_ws();
            if parser.peek().is_some() {
                FormulaEvaluation::unevaluated(
                    "reb_formula_unsupported_token",
                    format!(
                        "unsupported REB-VB token near '{}'; supported subset: {}",
                        parser.remaining(),
                        SUPPORTED_REB_VB_23003_SUBSET.join(", ")
                    ),
                    "formula",
                )
            } else {
                FormulaEvaluation::quantity(quantity)
            }
        }
        Err(finding) => FormulaEvaluation {
            quantity: None,
            findings: vec![finding],
        },
    }
}

struct FormulaParser<'a> {
    input: &'a str,
    chars: Chars<'a>,
    position: usize,
}

impl<'a> FormulaParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
            position: 0,
        }
    }

    fn parse_expression(&mut self) -> Result<Decimal, ValidationFinding> {
        let mut value = self.parse_term()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some('+') => {
                    self.bump();
                    value = checked_decimal(value.checked_add(self.parse_term()?), "+")?;
                }
                Some('-') => {
                    self.bump();
                    value = checked_decimal(value.checked_sub(self.parse_term()?), "-")?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_term(&mut self) -> Result<Decimal, ValidationFinding> {
        let mut value = self.parse_factor()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some('*') => {
                    self.bump();
                    value = checked_decimal(value.checked_mul(self.parse_factor()?), "*")?;
                }
                Some('/') => {
                    self.bump();
                    let divisor = self.parse_factor()?;
                    if divisor.is_zero() {
                        return Err(ValidationFinding::warning(
                            "reb_formula_division_by_zero",
                            "REB-VB formula division by zero was left unevaluated",
                        )
                        .at("formula"));
                    }
                    value = checked_decimal(value.checked_div(divisor), "/")?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_factor(&mut self) -> Result<Decimal, ValidationFinding> {
        self.skip_ws();
        match self.peek() {
            Some('+') => {
                self.bump();
                self.parse_factor()
            }
            Some('-') => {
                self.bump();
                checked_decimal(Decimal::ZERO.checked_sub(self.parse_factor()?), "unary -")
            }
            Some('(') => {
                self.bump();
                let value = self.parse_expression()?;
                self.skip_ws();
                if self.peek() == Some(')') {
                    self.bump();
                    Ok(value)
                } else {
                    Err(ValidationFinding::warning(
                        "reb_formula_syntax_error",
                        "REB-VB formula has an unclosed parenthesis",
                    )
                    .at("formula"))
                }
            }
            Some(ch) if ch.is_ascii_digit() || ch == ',' || ch == '.' => self.parse_number(),
            Some(ch) if ch.is_alphabetic() => Err(ValidationFinding::warning(
                "reb_formula_unsupported_token",
                format!(
                    "unsupported REB-VB identifier starting with '{ch}'; supported subset: {}",
                    SUPPORTED_REB_VB_23003_SUBSET.join(", ")
                ),
            )
            .at("formula")),
            Some(ch) => Err(ValidationFinding::warning(
                "reb_formula_syntax_error",
                format!("unexpected REB-VB formula token '{ch}'"),
            )
            .at("formula")),
            None => Err(ValidationFinding::warning(
                "reb_formula_syntax_error",
                "REB-VB formula ended unexpectedly",
            )
            .at("formula")),
        }
    }

    fn parse_number(&mut self) -> Result<Decimal, ValidationFinding> {
        let start = self.position;
        while matches!(self.peek(), Some(ch) if ch.is_ascii_digit() || ch == ',' || ch == '.') {
            self.bump();
        }
        let raw = &self.input[start..self.position];
        Decimal::from_str_exact(&raw.replace(',', ".")).map_err(|error| {
            ValidationFinding::warning(
                "reb_formula_number_parse_failed",
                format!("REB-VB numeric literal '{raw}' could not be parsed: {error}"),
            )
            .at("formula")
        })
    }

    fn skip_ws(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.bump();
        }
    }

    fn remaining(&self) -> &str {
        &self.input[self.position..]
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.position += ch.len_utf8();
        Some(ch)
    }
}

fn checked_decimal(value: Option<Decimal>, operator: &str) -> Result<Decimal, ValidationFinding> {
    value.ok_or_else(|| {
        ValidationFinding::warning(
            "reb_formula_decimal_overflow",
            format!("REB-VB decimal operation '{operator}' overflowed and was left unevaluated"),
        )
        .at("formula")
    })
}

struct X31Parser<'a> {
    reader: Reader<&'a [u8]>,
    buffer: Vec<u8>,
    version: Option<String>,
}

impl<'a> X31Parser<'a> {
    fn new(source: &'a str) -> Self {
        let mut reader = Reader::from_str(source);
        reader.config_mut().trim_text(true);
        Self {
            reader,
            buffer: Vec::new(),
            version: None,
        }
    }

    fn parse(
        &mut self,
        source_uri: Option<String>,
        checksum: Option<String>,
    ) -> Result<QuantityTakeoffDocument, ParseError> {
        let detected = source_uri.as_deref().map(detect_path);
        let mut document = QuantityTakeoffDocument::new(SourceProvenance {
            source_uri,
            source_format: GaebFormat::GaebXml,
            gaeb_version: None,
            phase: detected.and_then(|format| format.phase),
            checksum,
            parser_version: crate::VERSION.to_owned(),
        });
        let mut group_id: Option<String> = None;

        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(start)) => {
                    let owned = start.into_owned();
                    let local = local_name(owned.name());
                    match local.as_str() {
                        "Version" => self.version = self.read_text_for(owned.name())?,
                        "MeasurementGroup" | "MeasGrp" | "QtyGroup" => {
                            group_id =
                                attr_value(&owned, b"ID").or_else(|| attr_value(&owned, b"Id"));
                        }
                        "FormulaRecord" | "FormulaRow" | "Measurement" => {
                            let parsed = self.parse_row(&owned, group_id.as_deref())?;
                            document.attachments.extend(parsed.attachments);
                            document.findings.extend(parsed.findings);
                            document.rows.push(parsed.row);
                        }
                        other if is_unsupported_x31_container(other) => document.findings.push(
                            ValidationFinding::warning(
                                "x31_unsupported_feature",
                                format!("unsupported X31 container {other} was skipped"),
                            )
                            .at(other.to_owned()),
                        ),
                        _ => {}
                    }
                }
                Ok(Event::Empty(start)) => {
                    let local = local_name(start.name());
                    if matches!(local.as_str(), "MeasurementGroup" | "MeasGrp" | "QtyGroup") {
                        group_id = None;
                    } else if is_unsupported_x31_container(&local) {
                        document.findings.push(
                            ValidationFinding::warning(
                                "x31_unsupported_feature",
                                format!("unsupported X31 container {local} was skipped"),
                            )
                            .at(local),
                        );
                    }
                }
                Ok(Event::End(end)) => {
                    let local = local_name(end.name());
                    if matches!(local.as_str(), "MeasurementGroup" | "MeasGrp" | "QtyGroup") {
                        group_id = None;
                    }
                }
                Ok(Event::Eof) => break,
                Err(error) => {
                    return Err(ParseError {
                        code: "x31_xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: None,
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }
        document.source.gaeb_version.clone_from(&self.version);
        Ok(document)
    }

    fn parse_row(
        &mut self,
        start: &BytesStart<'_>,
        group_id: Option<&str>,
    ) -> Result<ParsedRow, ParseError> {
        let row_id = attr_value(start, b"ID")
            .or_else(|| attr_value(start, b"Id"))
            .unwrap_or_else(|| format!("row-{}", self.reader.buffer_position()));
        let ordinal = attr_value(start, b"RNo")
            .or_else(|| attr_value(start, b"Ordinal"))
            .or_else(|| attr_value(start, b"OZ"));
        let unit = attr_value(start, b"Unit").unwrap_or_default();
        let mut row = MeasurementRow::formula(
            row_id.clone(),
            ordinal.clone().unwrap_or_else(|| row_id.clone()),
            unit,
            MeasurementFormula::reb_vb_23003(""),
        );
        row.ordinal = ordinal;
        if let Some(group_id) = group_id {
            row.metadata.insert(
                "x31.measurement_group_id".to_owned(),
                serde_json::json!(group_id),
            );
        }
        let mut parsed = ParsedRow {
            row,
            attachments: Vec::new(),
            findings: Vec::new(),
        };
        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(child)) => {
                    let owned = child.into_owned();
                    self.handle_row_start(&owned, &mut parsed)?;
                }
                Ok(Event::Empty(child)) => {
                    let owned = child.into_owned();
                    Self::handle_row_empty(&owned, &mut parsed);
                }
                Ok(Event::End(end))
                    if matches!(
                        local_name(end.name()).as_str(),
                        "FormulaRecord" | "FormulaRow" | "Measurement"
                    ) =>
                {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(ParseError {
                        code: "x31_unclosed_measurement_row".to_owned(),
                        message: "X31 measurement row ended before its closing tag".to_owned(),
                        location: Some(row_id),
                    });
                }
                Err(error) => {
                    return Err(ParseError {
                        code: "x31_xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: Some(row_id),
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }
        Ok(parsed)
    }

    fn handle_row_start(
        &mut self,
        child: &BytesStart<'_>,
        parsed: &mut ParsedRow,
    ) -> Result<(), ParseError> {
        let local = local_name(child.name());
        match local.as_str() {
            "Formula" | "Expression" => {
                parsed.row.formula.expression =
                    self.read_text_for(child.name())?.unwrap_or_default();
            }
            "Result" | "Quantity" | "Qty" => {
                if let Some(value) = self.read_text_for(child.name())? {
                    parsed.row.result_quantity = Some(parse_decimal(&value, &local)?);
                }
            }
            "Unit" | "QU" => {
                parsed.row.unit = self.read_text_for(child.name())?.unwrap_or_default();
            }
            "Attachment" | "Drawing" | "Asset" => Self::capture_attachment(child, parsed),
            other => parsed
                .findings
                .push(unsupported_row_finding(other, &parsed.row.row_id)),
        }
        Ok(())
    }

    fn handle_row_empty(child: &BytesStart<'_>, parsed: &mut ParsedRow) {
        let local = local_name(child.name());
        match local.as_str() {
            "Attachment" | "Drawing" | "Asset" => Self::capture_attachment(child, parsed),
            other => parsed
                .findings
                .push(unsupported_row_finding(other, &parsed.row.row_id)),
        }
    }

    fn capture_attachment(child: &BytesStart<'_>, parsed: &mut ParsedRow) {
        let id = attr_value(child, b"ID")
            .or_else(|| attr_value(child, b"Id"))
            .unwrap_or_else(|| format!("attachment-{}", parsed.attachments.len()));
        let source_uri = attr_value(child, b"HRef")
            .or_else(|| attr_value(child, b"href"))
            .or_else(|| attr_value(child, b"Path"));
        parsed.row.attachment_ids.push(id.clone());
        parsed.attachments.push(MeasurementAttachment {
            id,
            kind: attachment_kind(attr_value(child, b"Type").as_deref()),
            source_uri,
            checksum: attr_value(child, b"Checksum"),
            metadata: BTreeMap::new(),
        });
    }

    fn read_text_for(&mut self, end: QName<'_>) -> Result<Option<String>, ParseError> {
        self.reader
            .read_text(end)
            .map_err(|error| ParseError {
                code: "x31_xml_text_read_failed".to_owned(),
                message: error.to_string(),
                location: Some(String::from_utf8_lossy(end.as_ref()).to_string()),
            })?
            .decode()
            .map(|decoded| Some(decoded.into_owned()))
            .map_err(|error| ParseError {
                code: "x31_xml_text_decode_failed".to_owned(),
                message: error.to_string(),
                location: Some(String::from_utf8_lossy(end.as_ref()).to_string()),
            })
    }
}

struct ParsedRow {
    row: MeasurementRow,
    attachments: Vec<MeasurementAttachment>,
    findings: Vec<ValidationFinding>,
}

fn unsupported_row_finding(field: &str, row_id: &str) -> ValidationFinding {
    ValidationFinding::warning(
        "x31_unsupported_feature",
        format!("unsupported X31 row field {field} was preserved as a finding"),
    )
    .at(format!("{row_id}/{field}"))
}

fn attachment_kind(raw: Option<&str>) -> MeasurementAttachmentKind {
    match raw.map(str::to_ascii_lowercase).as_deref() {
        Some("drawing" | "plan") => MeasurementAttachmentKind::Drawing,
        Some("photo") => MeasurementAttachmentKind::Photo,
        Some("calculation" | "calculation_sheet" | "sheet") => {
            MeasurementAttachmentKind::CalculationSheet
        }
        _ => MeasurementAttachmentKind::Unknown,
    }
}

fn is_unsupported_x31_container(local: &str) -> bool {
    matches!(local, "Sketch" | "FreeMeasurement" | "UnsupportedFeature")
}

fn parse_decimal(text: &str, location: &str) -> Result<Decimal, ParseError> {
    Decimal::from_str_exact(&text.trim().replace(',', ".")).map_err(|error| ParseError {
        code: "x31_decimal_parse_failed".to_owned(),
        message: error.to_string(),
        location: Some(location.to_owned()),
    })
}

fn local_name(name: QName<'_>) -> String {
    let raw = name.as_ref();
    let after_prefix = raw.rsplit(|byte| *byte == b':').next().unwrap_or(raw);
    String::from_utf8_lossy(after_prefix).to_string()
}

fn attr_value(start: &BytesStart<'_>, key: &[u8]) -> Option<String> {
    start
        .attributes()
        .flatten()
        .find(|attr| attr.key.as_ref() == key)
        .map(|attr| String::from_utf8_lossy(attr.value.as_ref()).to_string())
}

/// A complete X31 quantity takeoff document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantityTakeoffDocument {
    /// Source provenance for the X31 payload or future parser output.
    pub source: SourceProvenance,
    /// Optional baseline link to an X86 or BoQ document.
    pub baseline: Option<MeasurementBaselineLink>,
    /// Formula or measurement rows in deterministic source order.
    pub rows: Vec<MeasurementRow>,
    /// Attachment assets referenced by measurement rows.
    pub attachments: Vec<MeasurementAttachment>,
    /// Recoverable findings for unsupported or deferred X31 constructs.
    pub findings: Vec<ValidationFinding>,
    /// Document-level metadata.
    pub metadata: Metadata,
}

impl QuantityTakeoffDocument {
    /// Creates an empty X31 domain document for a known source.
    #[must_use]
    pub const fn new(source: SourceProvenance) -> Self {
        Self {
            source,
            baseline: None,
            rows: Vec::new(),
            attachments: Vec::new(),
            findings: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Returns all measurement rows linked to a BoQ ordinal.
    #[must_use]
    pub fn rows_for_ordinal(&self, ordinal: &str) -> Vec<&MeasurementRow> {
        self.rows
            .iter()
            .filter(|row| row.ordinal.as_deref() == Some(ordinal))
            .collect()
    }

    /// Records an attachment reference that cannot yet be materialized locally.
    pub fn record_attachment_gap(
        &mut self,
        attachment_id: impl Into<String>,
        reason: impl Into<String>,
    ) {
        let attachment_id = attachment_id.into();
        self.findings.push(
            ValidationFinding::warning(
                "x31_attachment_reference_only",
                format!(
                    "X31 attachment {attachment_id} is reference-only: {}",
                    reason.into()
                ),
            )
            .at(attachment_id),
        );
    }
}

/// Link from an X31 measurement set back to its baseline tender/contract data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasurementBaselineLink {
    /// Baseline document identifier or checksum.
    pub document_id: String,
    /// Baseline kind, for example X86 contract or X83 tender.
    pub kind: BaselineKind,
    /// Human-readable relation note.
    pub relation: String,
}

/// Supported baseline link kinds for X31 planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineKind {
    /// X86 contract award baseline.
    X86Contract,
    /// X83 tender/request baseline.
    X83Tender,
    /// Unknown or deferred baseline type.
    Unknown,
}

/// One X31 measurement/formula row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementRow {
    /// Stable row identifier from the X31 source or parser.
    pub row_id: String,
    /// Linked BoQ ordinal, when present.
    pub ordinal: Option<String>,
    /// Formula representation.
    pub formula: MeasurementFormula,
    /// Calculated/result quantity when known.
    pub result_quantity: Option<Decimal>,
    /// Quantity unit.
    pub unit: String,
    /// Optional physical progress data.
    pub progress: Option<PhysicalProgress>,
    /// Additional row references, such as drawings or REB line ids.
    pub references: Vec<MeasurementReference>,
    /// Attachment ids referenced by this row.
    pub attachment_ids: Vec<String>,
    /// Row-level metadata.
    pub metadata: Metadata,
}

impl MeasurementRow {
    /// Creates a formula row linked to a BoQ ordinal.
    #[must_use]
    pub fn formula(
        row_id: impl Into<String>,
        ordinal: impl Into<String>,
        unit: impl Into<String>,
        formula: MeasurementFormula,
    ) -> Self {
        Self {
            row_id: row_id.into(),
            ordinal: Some(ordinal.into()),
            formula,
            result_quantity: None,
            unit: unit.into(),
            progress: None,
            references: Vec::new(),
            attachment_ids: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a calculated/result quantity.
    #[must_use]
    pub const fn with_result(mut self, quantity: Decimal) -> Self {
        self.result_quantity = Some(quantity);
        self
    }

    /// Adds physical progress state.
    #[must_use]
    pub const fn with_progress(mut self, progress: PhysicalProgress) -> Self {
        self.progress = Some(progress);
        self
    }
}

/// Formula payload for X31 measurement rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementFormula {
    /// Formula system.
    pub system: RebFormulaSystem,
    /// Formula expression as source text; evaluation is a later issue.
    pub expression: String,
    /// Deterministic variables known at parse time.
    pub variables: BTreeMap<String, Decimal>,
}

impl MeasurementFormula {
    /// Creates a REB-VB 23.003 expression without evaluating it.
    #[must_use]
    pub fn reb_vb_23003(expression: impl Into<String>) -> Self {
        Self {
            system: RebFormulaSystem::RebVb23003,
            expression: expression.into(),
            variables: BTreeMap::new(),
        }
    }
}

/// Formula system marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebFormulaSystem {
    /// REB-VB 23.003 formula concept, represented but not evaluated here.
    RebVb23003,
    /// Unknown/deferred formula syntax.
    Unknown,
}

/// Physical-progress measurement data.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalProgress {
    /// Completed/progress quantity.
    pub completed_quantity: Decimal,
    /// Optional percent complete, 0-100 by convention.
    pub percent_complete: Option<Decimal>,
}

/// Reference from a measurement row to a source concept.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasurementReference {
    /// Reference kind.
    pub kind: MeasurementReferenceKind,
    /// Reference value.
    pub value: String,
}

/// Measurement reference kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementReferenceKind {
    /// BoQ ordinal reference.
    BoqOrdinal,
    /// Drawing or plan reference.
    Drawing,
    /// REB line/reference id.
    RebLine,
    /// Unknown/deferred reference.
    Unknown,
}

/// Attachment or asset referenced by X31 measurement data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementAttachment {
    /// Stable attachment id.
    pub id: String,
    /// Attachment kind.
    pub kind: MeasurementAttachmentKind,
    /// Optional local or external source URI.
    pub source_uri: Option<String>,
    /// Optional checksum when a local asset is available.
    pub checksum: Option<String>,
    /// Attachment metadata.
    pub metadata: Metadata,
}

/// Attachment classes relevant to quantity takeoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementAttachmentKind {
    /// Drawing, plan, or sketch.
    Drawing,
    /// Photo evidence.
    Photo,
    /// Calculation sheet or REB sidecar.
    CalculationSheet,
    /// Unknown/deferred attachment kind.
    Unknown,
}
