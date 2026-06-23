//! Dependency-free spreadsheet-neutral CSV exchange helpers.
//!
//! This module provides a governed neutral CSV bridge for spreadsheet workflows without
//! adding XLSX/ODS readers, writers, binary fixtures, browser automation, or external
//! spreadsheet dependencies. Rows are matched by GAEB OZ/item ordinal only; missing or
//! duplicated OZ columns reject updates instead of guessing by spreadsheet row order.
//! The helpers do not promote support status, do not grant Obra adapter support, and do
//! not claim production spreadsheet roundtrip support.

use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use rust_decimal::Decimal;

use crate::error::ValidationFinding;
use crate::model::{BoqItem, BoqNode, GaebDocument, Metadata, SourceProvenance};
use crate::support::SupportStatus;

const HEADER: [&str; 11] = [
    "oz",
    "short_text",
    "quantity",
    "unit",
    "unit_price",
    "total_price",
    "currency",
    "support_status",
    "source_uri",
    "loss_findings",
    "roundtrip_contract",
];

/// CSV-neutral spreadsheet export payload.
#[derive(Debug, Clone, PartialEq)]
pub struct NeutralSpreadsheet {
    /// UTF-8 CSV content with stable headers.
    pub csv: String,
    /// Source provenance of the parsed GAEB document.
    pub provenance: SourceProvenance,
    /// Findings present at export time.
    pub findings: Vec<ValidationFinding>,
    /// Export metadata, including the roundtrip contract.
    pub metadata: Metadata,
}

/// Exports a parsed document to a deterministic CSV-neutral spreadsheet payload.
///
/// The export is evidence-only. It preserves support status in CSV metadata but
/// does not promote parser, adapter, import, export, production, or certification support.
#[must_use]
pub fn export_neutral_csv(document: &GaebDocument) -> NeutralSpreadsheet {
    let mut csv = HEADER.join(",");
    csv.push('\n');
    let support_status = support_status_code(document.support_status);
    let source_uri = document.source.source_uri.as_deref().unwrap_or_default();
    let loss_codes = document
        .findings
        .iter()
        .map(|finding| finding.code.as_str())
        .collect::<Vec<_>>()
        .join("|");

    for node in flatten_item_nodes(&document.boq.nodes) {
        if let Some(item) = &node.item {
            let row = [
                node.ordinal.as_str(),
                item.short_text.as_str(),
                &decimal_to_csv(item.quantity),
                item.unit.as_str(),
                &optional_decimal_to_csv(item.unit_price),
                &optional_decimal_to_csv(item.total_price),
                document.boq.currency.as_deref().unwrap_or_default(),
                &format!("support_status={support_status}"),
                source_uri,
                loss_codes.as_str(),
                "oz_matched_csv_neutral",
            ];
            csv.push_str(&encode_csv_row(row));
            csv.push('\n');
        }
    }

    let mut metadata = BTreeMap::new();
    metadata.insert(
        "roundtrip_contract".to_owned(),
        serde_json::json!("oz_matched_csv_neutral"),
    );
    metadata.insert(
        "support_status".to_owned(),
        serde_json::json!(support_status),
    );
    metadata.insert(
        "dependency_policy".to_owned(),
        serde_json::json!("no_xlsx_or_binary_dependency"),
    );

    NeutralSpreadsheet {
        csv,
        provenance: document.source.clone(),
        findings: document.findings.clone(),
        metadata,
    }
}

/// Applies CSV-neutral spreadsheet updates to matching BoQ items by OZ/item ordinal.
///
/// Supported update headers are `short_text`, `quantity`, `unit`, `unit_price`,
/// and `total_price`. Unknown helper columns are ignored with explicit findings.
/// Missing, duplicated, or empty OZ/item ordinal values reject the update instead
/// of falling back to row order.
///
/// # Errors
///
/// Returns a validation finding when CSV parsing fails, the OZ column is missing
/// or duplicated, or a row has an empty OZ value.
pub fn apply_neutral_csv_updates(
    document: &mut GaebDocument,
    csv: &str,
) -> Result<Vec<ValidationFinding>, ValidationFinding> {
    let rows = parse_csv(csv)?;
    let Some(header_row) = rows.first() else {
        return Err(ValidationFinding::warning(
            "spreadsheet_neutral_empty_csv",
            "neutral spreadsheet CSV must include a header row",
        ));
    };
    let header = HeaderMap::new(header_row)?;
    let mut findings = header.ignored_column_findings();
    let mut item_index = BTreeMap::new();
    index_items(&mut document.boq.nodes, &mut item_index);

    for (row_number, row) in rows.iter().enumerate().skip(1) {
        if row.iter().all(|value| value.trim().is_empty()) {
            continue;
        }
        let oz = header.required_oz(row, row_number + 1)?;
        let Some(item) = item_index.get_mut(oz) else {
            findings.push(
                ValidationFinding::warning(
                    "spreadsheet_neutral_unmatched_oz",
                    format!("neutral spreadsheet row references unknown OZ '{oz}'"),
                )
                .at(format!("row[{row_number}].oz")),
            );
            collect_decimal_findings(row, &header, row_number + 1, &mut findings);
            continue;
        };

        let before = item.clone();
        apply_text_cell(row, &header, "short_text", &mut item.short_text);
        apply_text_cell(row, &header, "unit", &mut item.unit);
        apply_decimal_cell(
            row,
            &header,
            "quantity",
            &mut item.quantity,
            row_number + 1,
            &mut findings,
        );
        apply_optional_decimal_cell(
            row,
            &header,
            "unit_price",
            &mut item.unit_price,
            row_number + 1,
            &mut findings,
        );
        apply_optional_decimal_cell(
            row,
            &header,
            "total_price",
            &mut item.total_price,
            row_number + 1,
            &mut findings,
        );

        if **item != before {
            findings.push(
                ValidationFinding::warning(
                    "spreadsheet_neutral_update_applied",
                    format!("neutral spreadsheet update applied by OZ '{oz}'"),
                )
                .at(oz.to_owned()),
            );
        }
    }

    document.findings.extend(findings.clone());
    Ok(findings)
}

struct HeaderMap {
    indexes: BTreeMap<String, usize>,
    ignored_headers: Vec<String>,
}

impl HeaderMap {
    fn new(headers: &[String]) -> Result<Self, ValidationFinding> {
        let mut indexes = BTreeMap::new();
        let mut ignored_headers = Vec::new();
        let mut seen = BTreeSet::new();
        for (index, header) in headers.iter().enumerate() {
            let normalized = normalize_header(header);
            if normalized.is_empty() {
                ignored_headers.push(header.clone());
                continue;
            }
            if !seen.insert(normalized.clone()) {
                return Err(ValidationFinding::warning(
                    "spreadsheet_neutral_duplicate_header",
                    format!("neutral spreadsheet header '{header}' appears more than once"),
                )
                .at(format!("header[{index}]")));
            }
            if is_known_header(&normalized) {
                indexes.insert(normalized, index);
            } else {
                ignored_headers.push(header.clone());
            }
        }
        if !indexes.contains_key("oz") {
            return Err(ValidationFinding::warning(
                "spreadsheet_neutral_missing_oz_column",
                "neutral spreadsheet updates require an OZ/item ordinal column",
            ));
        }
        Ok(Self {
            indexes,
            ignored_headers,
        })
    }

    fn get<'a>(&self, row: &'a [String], header: &str) -> Option<&'a str> {
        self.indexes
            .get(header)
            .and_then(|index| row.get(*index))
            .map(String::as_str)
    }

    fn required_oz<'a>(
        &self,
        row: &'a [String],
        row_number: usize,
    ) -> Result<&'a str, ValidationFinding> {
        let value = self.get(row, "oz").unwrap_or_default().trim();
        if value.is_empty() {
            return Err(ValidationFinding::warning(
                "spreadsheet_neutral_missing_oz_value",
                "neutral spreadsheet row has an empty OZ/item ordinal value",
            )
            .at(format!("row[{row_number}].oz")));
        }
        Ok(value)
    }

    fn ignored_column_findings(&self) -> Vec<ValidationFinding> {
        self.ignored_headers
            .iter()
            .filter(|header| !header.trim().is_empty())
            .map(|header| {
                ValidationFinding::warning(
                    "spreadsheet_neutral_ignored_column",
                    format!("neutral spreadsheet helper column '{header}' was ignored"),
                )
                .at(header.clone())
            })
            .collect()
    }
}

fn flatten_item_nodes(nodes: &[BoqNode]) -> Vec<&BoqNode> {
    let mut flattened = Vec::new();
    for node in nodes {
        if node.item.is_some() {
            flattened.push(node);
        }
        flattened.extend(flatten_item_nodes(&node.children));
    }
    flattened
}

fn index_items<'a>(nodes: &'a mut [BoqNode], index: &mut BTreeMap<String, &'a mut BoqItem>) {
    for node in nodes {
        if let Some(item) = &mut node.item {
            index.insert(node.ordinal.clone(), item);
        }
        index_items(&mut node.children, index);
    }
}

fn apply_text_cell(row: &[String], header: &HeaderMap, name: &str, target: &mut String) {
    if let Some(value) = header.get(row, name) {
        let value = value.trim();
        if !value.is_empty() {
            value.clone_into(target);
        }
    }
}

fn apply_decimal_cell(
    row: &[String],
    header: &HeaderMap,
    name: &str,
    target: &mut Decimal,
    row_number: usize,
    findings: &mut Vec<ValidationFinding>,
) {
    if let Some(value) = parse_decimal_cell(row, header, name, row_number, findings) {
        *target = value;
    }
}

fn apply_optional_decimal_cell(
    row: &[String],
    header: &HeaderMap,
    name: &str,
    target: &mut Option<Decimal>,
    row_number: usize,
    findings: &mut Vec<ValidationFinding>,
) {
    if let Some(value) = parse_decimal_cell(row, header, name, row_number, findings) {
        *target = Some(value);
    }
}

fn collect_decimal_findings(
    row: &[String],
    header: &HeaderMap,
    row_number: usize,
    findings: &mut Vec<ValidationFinding>,
) {
    for name in ["quantity", "unit_price", "total_price"] {
        let _ = parse_decimal_cell(row, header, name, row_number, findings);
    }
}

fn parse_decimal_cell(
    row: &[String],
    header: &HeaderMap,
    name: &str,
    row_number: usize,
    findings: &mut Vec<ValidationFinding>,
) -> Option<Decimal> {
    let value = header.get(row, name)?.trim();
    if value.is_empty() {
        return None;
    }
    match Decimal::from_str(value) {
        Ok(value) if value >= Decimal::ZERO => Some(value),
        _ => {
            findings.push(
                ValidationFinding::warning(
                    "spreadsheet_neutral_invalid_decimal",
                    format!("neutral spreadsheet field '{name}' is not a non-negative decimal"),
                )
                .at(format!("row[{row_number}].{name}")),
            );
            None
        }
    }
}

fn parse_csv(input: &str) -> Result<Vec<Vec<String>>, ValidationFinding> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut cell = String::new();
    let mut chars = input.chars().peekable();
    let mut in_quotes = false;

    while let Some(ch) = chars.next() {
        match ch {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                cell.push('"');
                let _ = chars.next();
            }
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                row.push(std::mem::take(&mut cell));
            }
            '\n' if !in_quotes => {
                row.push(std::mem::take(&mut cell));
                rows.push(std::mem::take(&mut row));
            }
            '\r' if !in_quotes => {}
            _ => cell.push(ch),
        }
    }

    if in_quotes {
        return Err(ValidationFinding::warning(
            "spreadsheet_neutral_invalid_csv",
            "neutral spreadsheet CSV has an unterminated quoted field",
        ));
    }

    if !cell.is_empty() || !row.is_empty() {
        row.push(cell);
        rows.push(row);
    }

    Ok(rows)
}

fn encode_csv_row<'a>(cells: impl IntoIterator<Item = &'a str>) -> String {
    cells
        .into_iter()
        .map(escape_csv_cell)
        .collect::<Vec<_>>()
        .join(",")
}

fn escape_csv_cell(cell: &str) -> String {
    if cell.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", cell.replace('"', "\"\""))
    } else {
        cell.to_owned()
    }
}

fn normalize_header(header: &str) -> String {
    match header.trim().to_ascii_lowercase().as_str() {
        "oz" | "item ordinal" | "ordnungszahl" => "oz".to_owned(),
        "short_text" | "kurztext" | "description" => "short_text".to_owned(),
        "quantity" | "menge" => "quantity".to_owned(),
        "unit" | "einheit" => "unit".to_owned(),
        "unit_price" | "ep" | "einheitspreis" => "unit_price".to_owned(),
        "total_price" | "gp" | "gesamtpreis" => "total_price".to_owned(),
        "currency" | "währung" | "waehrung" => "currency".to_owned(),
        other => other.to_owned(),
    }
}

fn is_known_header(header: &str) -> bool {
    matches!(
        header,
        "oz" | "short_text"
            | "quantity"
            | "unit"
            | "unit_price"
            | "total_price"
            | "currency"
            | "support_status"
            | "source_uri"
            | "loss_findings"
            | "roundtrip_contract"
    )
}

fn decimal_to_csv(value: Decimal) -> String {
    value.normalize().to_string()
}

fn optional_decimal_to_csv(value: Option<Decimal>) -> String {
    value.map_or_else(String::new, decimal_to_csv)
}

const fn support_status_code(status: SupportStatus) -> &'static str {
    match status {
        SupportStatus::Supported => "supported",
        SupportStatus::SupportedParseOnly => "supported_parse_only",
        SupportStatus::FutureTrack => "future_track",
        SupportStatus::ReferenceOnly => "reference_only",
    }
}
