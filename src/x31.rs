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
use crate::model::{BoqNode, GaebDocument, GaebFormat, Metadata, SourceProvenance};

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

/// Deterministic X31-to-X86 progress-link report.
///
/// This report is billing-readiness evidence only. It does not create invoices,
/// XRechnung payloads, adapter exports, or payment claims.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct X31X86ProgressReport {
    /// Baseline relation used for the link attempt.
    pub baseline: MeasurementBaselineLink,
    /// One row per X31 measurement row in deterministic source order.
    pub rows: Vec<X31X86ProgressRow>,
    /// Audit findings for missing ordinals, unmatched baseline items, or mismatches.
    pub findings: Vec<ValidationFinding>,
    /// Explicit non-goal marker for downstream consumers.
    pub invoice_generated: bool,
    /// Report-level metadata.
    pub metadata: Metadata,
}

/// One linked or unlinked X31/X86 measurement row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct X31X86ProgressRow {
    /// X31 measurement row identifier.
    pub row_id: String,
    /// BoQ ordinal when present.
    pub ordinal: Option<String>,
    /// Link status.
    pub status: X31X86LinkStatus,
    /// X31 measured/result quantity.
    pub measured_quantity: Option<Decimal>,
    /// X31 unit.
    pub measured_unit: String,
    /// X86 baseline item quantity.
    pub baseline_quantity: Option<Decimal>,
    /// X86 baseline unit.
    pub baseline_unit: Option<String>,
    /// X86 baseline unit price.
    pub unit_price: Option<Decimal>,
    /// Measured progress value, computed as measured quantity times unit price.
    pub progress_value: Option<Decimal>,
}

/// Status of an X31 measurement row against an X86 baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum X31X86LinkStatus {
    /// Measurement row linked to a baseline item by ordinal without audit findings.
    Matched,
    /// The X31 row has no ordinal and cannot be linked.
    MissingMeasurementOrdinal,
    /// The X31 ordinal is absent from the X86 baseline.
    MissingBaselineItem,
    /// The row linked by ordinal but has quantity or unit mismatch findings.
    Mismatched,
}

/// Links X31 measurement rows to X86 contract baseline items by ordinal.
///
/// Findings are non-fatal and sorted in X31 source order. No invoice or
/// XRechnung generation is performed or implied.
#[must_use]
pub fn link_x31_to_x86_baseline(
    measurements: &QuantityTakeoffDocument,
    baseline: &GaebDocument,
) -> X31X86ProgressReport {
    let baseline_index = baseline_item_index(baseline);
    let mut rows = Vec::with_capacity(measurements.rows.len());
    let mut findings = Vec::new();

    for measurement in &measurements.rows {
        let measured_quantity = measurement.result_quantity;
        let mut row = X31X86ProgressRow {
            row_id: measurement.row_id.clone(),
            ordinal: measurement.ordinal.clone(),
            status: X31X86LinkStatus::Matched,
            measured_quantity,
            measured_unit: measurement.unit.clone(),
            baseline_quantity: None,
            baseline_unit: None,
            unit_price: None,
            progress_value: None,
        };

        let Some(ordinal) = measurement.ordinal.as_deref() else {
            row.status = X31X86LinkStatus::MissingMeasurementOrdinal;
            findings.push(
                ValidationFinding::warning(
                    "x31_x86_missing_measurement_ordinal",
                    "X31 measurement row has no BoQ ordinal for X86 baseline linking",
                )
                .at(measurement.row_id.clone()),
            );
            rows.push(row);
            continue;
        };

        let Some(baseline_node) = baseline_index.get(ordinal).copied() else {
            row.status = X31X86LinkStatus::MissingBaselineItem;
            findings.push(
                ValidationFinding::warning(
                    "x31_x86_unmatched_measurement",
                    format!("X31 measurement ordinal {ordinal} has no X86 baseline item"),
                )
                .at(ordinal.to_owned()),
            );
            rows.push(row);
            continue;
        };

        if let Some(item) = &baseline_node.item {
            row.baseline_quantity = Some(item.quantity);
            row.baseline_unit = Some(item.unit.clone());
            row.unit_price = item.unit_price;
            row.progress_value = measured_quantity
                .zip(item.unit_price)
                .and_then(|(quantity, unit_price)| quantity.checked_mul(unit_price));

            if !measurement.unit.is_empty() && measurement.unit != item.unit {
                row.status = X31X86LinkStatus::Mismatched;
                findings.push(
                    ValidationFinding::warning(
                        "x31_x86_unit_mismatch",
                        format!(
                            "X31 unit '{}' differs from X86 baseline unit '{}' for ordinal {ordinal}",
                            measurement.unit, item.unit
                        ),
                    )
                    .at(ordinal.to_owned()),
                );
            }

            if measured_quantity.is_some_and(|quantity| quantity > item.quantity) {
                row.status = X31X86LinkStatus::Mismatched;
                findings.push(
                    ValidationFinding::warning(
                        "x31_x86_quantity_exceeds_baseline",
                        format!("X31 quantity exceeds X86 baseline quantity for ordinal {ordinal}"),
                    )
                    .at(ordinal.to_owned()),
                );
            }
        }

        rows.push(row);
    }

    X31X86ProgressReport {
        baseline: MeasurementBaselineLink {
            document_id: baseline
                .source
                .checksum
                .clone()
                .or_else(|| baseline.source.source_uri.clone())
                .unwrap_or_else(|| "x86-baseline".to_owned()),
            kind: if baseline
                .source
                .phase
                .as_ref()
                .is_some_and(|phase| phase.code == "86")
            {
                BaselineKind::X86Contract
            } else {
                BaselineKind::Unknown
            },
            relation: "X31 measured quantities linked to X86 contract baseline by ordinal"
                .to_owned(),
        },
        rows,
        findings,
        invoice_generated: false,
        metadata: BTreeMap::new(),
    }
}

fn baseline_item_index(document: &GaebDocument) -> BTreeMap<String, &BoqNode> {
    let mut index = BTreeMap::new();
    for node in &document.boq.nodes {
        collect_item_nodes(node, &mut index);
    }
    index
}

fn collect_item_nodes<'a>(node: &'a BoqNode, index: &mut BTreeMap<String, &'a BoqNode>) {
    if node.item.is_some() {
        index.insert(node.ordinal.clone(), node);
    }
    for child in &node.children {
        collect_item_nodes(child, index);
    }
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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::model::{GaebFormat, GaebPhase};

    fn source() -> SourceProvenance {
        SourceProvenance {
            source_uri: Some("measurement.x31".to_owned()),
            source_format: GaebFormat::GaebXml,
            gaeb_version: Some("3.3".to_owned()),
            phase: Some(GaebPhase {
                code: "31".to_owned(),
                label: None,
            }),
            checksum: None,
            parser_version: "test".to_owned(),
        }
    }

    const X31_XML: &str = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>X31</Name><QtyTakeoff><MeasurementGroup ID="G-1"><FormulaRecord ID="FR-1" RNo="001.0010" Unit="m3"><Formula>2 * 6</Formula><Result>12,50</Result><Attachment ID="D-1" Type="drawing" HRef="drawings/detail-a.pdf" Checksum="sha256:abc"/><UnsupportedFeature>needs future parser</UnsupportedFeature></FormulaRecord></MeasurementGroup></QtyTakeoff></Project></GAEB>"#;

    #[test]
    fn parser_smoke_covers_private_x31_read_paths() {
        let document = parse_str(
            X31_XML,
            Some("gaeb/bvbs/gaeb_xml_3_3/quantity_takeoff/x31/internal.X31".to_owned()),
        )
        .expect("internal parser smoke parses");

        assert_eq!(document.rows.len(), 1);
        assert_eq!(
            document.rows[0].result_quantity,
            Some(Decimal::new(1250, 2))
        );
        assert_eq!(document.rows[0].attachment_ids, ["D-1"]);
        assert_eq!(document.attachments.len(), 1);
        assert!(document.findings.iter().any(|finding| {
            finding.code == "x31_unsupported_feature"
                && finding.location.as_deref() == Some("FR-1/UnsupportedFeature")
        }));
    }

    #[test]
    fn quantity_takeoff_helpers_filter_rows_and_record_attachment_gaps() {
        let mut document = QuantityTakeoffDocument::new(source());
        document.rows.push(MeasurementRow::formula(
            "r1",
            "001.0010",
            "m",
            MeasurementFormula::reb_vb_23003("1+2"),
        ));
        document.rows.push(MeasurementRow::formula(
            "r2",
            "001.0020",
            "m",
            MeasurementFormula::reb_vb_23003("3+4"),
        ));

        assert_eq!(document.rows_for_ordinal("001.0010").len(), 1);
        assert!(document.rows_for_ordinal("missing").is_empty());

        document.record_attachment_gap("plan-1", "not vendored");
        assert_eq!(document.findings.len(), 1);
        assert_eq!(document.findings[0].code, "x31_attachment_reference_only");
    }

    #[test]
    fn measurement_row_and_formula_evaluation_helpers_are_stable() {
        let with_result: fn(MeasurementRow, Decimal) -> MeasurementRow =
            MeasurementRow::with_result;
        let with_progress: fn(MeasurementRow, PhysicalProgress) -> MeasurementRow =
            MeasurementRow::with_progress;
        let quantity: fn(Decimal) -> FormulaEvaluation = FormulaEvaluation::quantity;
        let progress = PhysicalProgress {
            completed_quantity: Decimal::new(5, 0),
            percent_complete: Some(Decimal::new(50, 0)),
        };
        let row = with_progress(
            with_result(
                MeasurementRow::formula(
                    "r1",
                    "001.0010",
                    "m",
                    MeasurementFormula::reb_vb_23003("2*3"),
                ),
                Decimal::new(6, 0),
            ),
            progress,
        );

        assert_eq!(row.result_quantity, Some(Decimal::new(6, 0)));
        assert_eq!(row.progress, Some(progress));
        assert_eq!(row.formula.system, RebFormulaSystem::RebVb23003);

        let evaluated_quantity = quantity(Decimal::new(7, 0));
        assert_eq!(evaluated_quantity.quantity, Some(Decimal::new(7, 0)));
        assert!(evaluated_quantity.findings.is_empty());

        let unevaluated = FormulaEvaluation::unevaluated("x31_test", "not supported", "row-1");
        assert_eq!(unevaluated.quantity, None);
        assert_eq!(unevaluated.findings[0].code, "x31_test");
    }

    #[test]
    fn const_method_chaining_remains_source_compatible() {
        let progress = PhysicalProgress {
            completed_quantity: Decimal::new(5, 0),
            percent_complete: Some(Decimal::new(50, 0)),
        };
        let row = MeasurementRow::formula(
            "r1",
            "001.0010",
            "m",
            MeasurementFormula::reb_vb_23003("2*3"),
        )
        .with_result(Decimal::new(6, 0))
        .with_progress(progress);

        assert_eq!(row.result_quantity, Some(Decimal::new(6, 0)));
        assert_eq!(row.progress, Some(progress));
        assert_eq!(row.formula.system, RebFormulaSystem::RebVb23003);
    }
}
