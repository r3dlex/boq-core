//! X89 Rechnung invoice domain model.
//!
//! This module models GAEB Rechnung/X89 invoice data separately from the BoQ
//! parser and separately from any XRechnung envelope generator. It is a typed
//! design boundary for future parser work: constructing these structs does not
//! promote any `gaeb/manifest.toml` entry, does not parse a fixture, and does
//! not create an XRechnung payload.
//!
//! ```
//! use rust_decimal::Decimal;
//! use boq_core::model::{GaebFormat, GaebPhase, SourceProvenance};
//! use boq_core::x89::{InvoiceDocument, InvoiceHeader, InvoiceLine};
//!
//! let source = SourceProvenance {
//!     source_uri: Some("rechnung.X89".to_owned()),
//!     source_format: GaebFormat::GaebXml,
//!     gaeb_version: Some("3.3".to_owned()),
//!     phase: Some(GaebPhase { code: "89".to_owned(), label: Some("Rechnung".to_owned()) }),
//!     checksum: None,
//!     parser_version: boq_core::version().to_owned(),
//! };
//! let mut invoice = InvoiceDocument::new(source, InvoiceHeader::new("INV-1", "EUR"));
//! invoice.add_line(InvoiceLine::new("1", "001.0010", "m", Decimal::new(2, 0), Decimal::new(50, 0)));
//! invoice.recalculate_totals();
//!
//! assert_eq!(invoice.totals.net_amount, Decimal::new(100, 0));
//! assert!(!invoice.xrechnung_generated);
//! ```

use std::collections::BTreeMap;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::checksum::sha256_hex;
use crate::error::{ParseError, ValidationFinding};
use crate::format::detect_path;
use crate::model::{GaebFormat, GaebPhase, Metadata, SourceProvenance};

/// Parses a GAEB XML X89/Rechnung payload into the invoice-domain model.
///
/// This parser is an MVP for fixture-backed invoice-domain extraction. It does
/// not promote manifest support status and does not create XRechnung output.
///
/// # Errors
///
/// Returns a parse error when XML cannot be decoded or a numeric invoice field
/// cannot be parsed.
pub fn parse_str(source: &str, source_uri: Option<String>) -> Result<InvoiceDocument, ParseError> {
    X89Parser::new(source).parse(source_uri, Some(sha256_hex(source.as_bytes())))
}

/// Parses an X89 XML file from disk.
///
/// # Errors
///
/// Returns a parse error when the file cannot be read or parsed.
pub fn parse_file(path: impl AsRef<std::path::Path>) -> Result<InvoiceDocument, ParseError> {
    let path_ref = path.as_ref();
    let source = std::fs::read_to_string(path_ref).map_err(|error| ParseError {
        code: "x89_read_failed".to_owned(),
        message: error.to_string(),
        location: Some(path_ref.display().to_string()),
    })?;
    parse_str(&source, Some(path_ref.display().to_string()))
}

struct X89Parser<'a> {
    reader: Reader<&'a [u8]>,
    buffer: Vec<u8>,
    version: Option<String>,
}

impl<'a> X89Parser<'a> {
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
    ) -> Result<InvoiceDocument, ParseError> {
        let detected = source_uri.as_deref().map(detect_path);
        let source = SourceProvenance {
            source_uri,
            source_format: GaebFormat::GaebXml,
            gaeb_version: None,
            phase: detected.and_then(|format| format.phase).or_else(|| {
                Some(GaebPhase {
                    code: "89".to_owned(),
                    label: Some("Rechnung".to_owned()),
                })
            }),
            checksum,
            parser_version: crate::VERSION.to_owned(),
        };
        let mut document = InvoiceDocument::new(source, InvoiceHeader::new("x89-unknown", "EUR"));

        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(start)) => {
                    let owned = start.into_owned();
                    self.handle_document_start(&owned, &mut document)?;
                }
                Ok(Event::Empty(start)) => {
                    let owned = start.into_owned();
                    handle_document_empty(&owned, &mut document)?;
                }
                Ok(Event::Eof) => break,
                Err(error) => {
                    return Err(ParseError {
                        code: "x89_xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: None,
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }

        document.source.gaeb_version.clone_from(&self.version);
        document.recalculate_totals();
        Ok(document)
    }

    fn handle_document_start(
        &mut self,
        start: &BytesStart<'_>,
        document: &mut InvoiceDocument,
    ) -> Result<(), ParseError> {
        let local = local_name(start.name());
        match local.as_str() {
            "Version" => self.version = Some(self.read_text_for(start.name())?),
            "Invoice" => document.header = invoice_header_from_attrs(start),
            "Line" => document.lines.push(self.parse_line(start)?),
            other if is_unsupported_tax(other) => document.findings.push(unsupported_finding(
                "x89_unsupported_tax_field",
                other,
                &document.header.invoice_id,
            )),
            other if is_unsupported_payment(other) => document.findings.push(unsupported_finding(
                "x89_unsupported_payment_field",
                other,
                &document.header.invoice_id,
            )),
            _ => {}
        }
        Ok(())
    }

    fn parse_line(&mut self, start: &BytesStart<'_>) -> Result<InvoiceLine, ParseError> {
        let mut line = line_from_attrs(start)?;
        let line_id = line.line_id.clone();
        loop {
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(child)) => {
                    let owned = child.into_owned();
                    self.handle_line_start(&owned, &mut line)?;
                }
                Ok(Event::Empty(child)) => {
                    let owned = child.into_owned();
                    handle_line_empty(&owned, &mut line)?;
                }
                Ok(Event::End(end)) if matches!(local_name(end.name()).as_str(), "Line") => {
                    break;
                }
                Ok(Event::Eof) => {
                    return Err(ParseError {
                        code: "x89_unclosed_invoice_line".to_owned(),
                        message: "X89 invoice line ended before its closing tag".to_owned(),
                        location: Some(line_id),
                    });
                }
                Err(error) => {
                    return Err(ParseError {
                        code: "x89_xml_parse_failed".to_owned(),
                        message: error.to_string(),
                        location: Some(line_id),
                    });
                }
                _ => {}
            }
            self.buffer.clear();
        }
        Ok(line)
    }

    fn handle_line_start(
        &mut self,
        child: &BytesStart<'_>,
        line: &mut InvoiceLine,
    ) -> Result<(), ParseError> {
        let local = local_name(child.name());
        match local.as_str() {
            "Description" => line.description = Some(self.read_text_for(child.name())?),
            "Qty" => line.quantity = parse_decimal(&self.read_text_for(child.name())?, &local)?,
            "QU" => line.unit = self.read_text_for(child.name())?,
            "UnitPrice" => {
                line.unit_price = parse_decimal(&self.read_text_for(child.name())?, &local)?;
            }
            "NetAmount" => {
                line.net_amount = Some(parse_decimal(&self.read_text_for(child.name())?, &local)?);
            }
            other if is_unsupported_tax(other) => line.findings.push(unsupported_finding(
                "x89_unsupported_tax_field",
                other,
                &line.line_id,
            )),
            other if is_unsupported_payment(other) => line.findings.push(unsupported_finding(
                "x89_unsupported_payment_field",
                other,
                &line.line_id,
            )),
            other => line.findings.push(unsupported_finding(
                "x89_unsupported_invoice_line_field",
                other,
                &line.line_id,
            )),
        }
        Ok(())
    }

    fn read_text_for(&mut self, end: QName<'_>) -> Result<String, ParseError> {
        let text = self.reader.read_text(end).map_err(|error| ParseError {
            code: "x89_xml_text_read_failed".to_owned(),
            message: error.to_string(),
            location: Some(String::from_utf8_lossy(end.as_ref()).to_string()),
        })?;
        text.decode()
            .map(std::borrow::Cow::into_owned)
            .map_err(|error| ParseError {
                code: "x89_xml_text_decode_failed".to_owned(),
                message: error.to_string(),
                location: Some(String::from_utf8_lossy(end.as_ref()).to_string()),
            })
    }
}

fn handle_document_empty(
    start: &BytesStart<'_>,
    document: &mut InvoiceDocument,
) -> Result<(), ParseError> {
    let local = local_name(start.name());
    match local.as_str() {
        "Invoice" => document.header = invoice_header_from_attrs(start),
        "Party" => document.parties.push(party_from_attrs(start)),
        "Line" => document.lines.push(line_from_attrs(start)?),
        "ContractRef" => document.contract_links.push(contract_from_attrs(start)),
        "QuantityEvidence" => document
            .quantity_evidence
            .push(quantity_evidence_from_attrs(start)),
        "Payment" => document.payment = payment_from_attrs(start),
        other if is_unsupported_tax(other) => document.findings.push(unsupported_finding(
            "x89_unsupported_tax_field",
            other,
            &document.header.invoice_id,
        )),
        other if is_unsupported_payment(other) => document.findings.push(unsupported_finding(
            "x89_unsupported_payment_field",
            other,
            &document.header.invoice_id,
        )),
        _ => {}
    }
    Ok(())
}

fn line_from_attrs(start: &BytesStart<'_>) -> Result<InvoiceLine, ParseError> {
    let line_id = attr_value(start, b"ID").unwrap_or_else(|| "x89-line".to_owned());
    let ordinal = attr_value(start, b"RNo").unwrap_or_else(|| line_id.clone());
    let unit = attr_value(start, b"Unit").unwrap_or_default();
    let quantity = parse_optional_decimal(start, &[b"Qty"], "Qty")?.unwrap_or(Decimal::ZERO);
    let unit_price =
        parse_optional_decimal(start, &[b"UnitPrice"], "UnitPrice")?.unwrap_or(Decimal::ZERO);
    let mut line = InvoiceLine::new(line_id, ordinal, unit, quantity, unit_price);
    line.description = attr_value(start, b"Description");
    if let Some(net_amount) = parse_optional_decimal(start, &[b"NetAmount"], "NetAmount")? {
        line.net_amount = Some(net_amount);
    }
    Ok(line)
}

fn handle_line_empty(start: &BytesStart<'_>, line: &mut InvoiceLine) -> Result<(), ParseError> {
    let local = local_name(start.name());
    match local.as_str() {
        "Tax" => line.tax = Some(tax_from_attrs(start)?),
        "ContractRef" => line.contract = Some(contract_from_attrs(start)),
        "QuantityEvidence" => {
            line.quantity_evidence
                .push(quantity_evidence_from_attrs(start));
        }
        other if is_unsupported_tax(other) => line.findings.push(unsupported_finding(
            "x89_unsupported_tax_field",
            other,
            &line.line_id,
        )),
        other if is_unsupported_payment(other) => line.findings.push(unsupported_finding(
            "x89_unsupported_payment_field",
            other,
            &line.line_id,
        )),
        other => line.findings.push(unsupported_finding(
            "x89_unsupported_invoice_line_field",
            other,
            &line.line_id,
        )),
    }
    Ok(())
}

fn invoice_header_from_attrs(start: &BytesStart<'_>) -> InvoiceHeader {
    let mut header = InvoiceHeader::new(
        attr_value(start, b"ID").unwrap_or_else(|| "x89-unknown".to_owned()),
        attr_value(start, b"Currency").unwrap_or_else(|| "EUR".to_owned()),
    );
    header.invoice_date = attr_value(start, b"Date");
    header.project_id = attr_value(start, b"ProjectID");
    header.invoice_type = invoice_type(attr_value(start, b"Type").as_deref());
    header
}

fn party_from_attrs(start: &BytesStart<'_>) -> InvoiceParty {
    let role = party_role(attr_value(start, b"Role").as_deref().unwrap_or_default());
    let mut party = InvoiceParty::new(
        role,
        attr_value(start, b"Name").unwrap_or_else(|| "unknown party".to_owned()),
    );
    party.endpoint_id = attr_value(start, b"EndpointID");
    party.tax_id = attr_value(start, b"TaxID");
    party
}

fn tax_from_attrs(start: &BytesStart<'_>) -> Result<TaxBreakdown, ParseError> {
    let rate = parse_optional_decimal(start, &[b"Rate"], "Rate")?.unwrap_or(Decimal::ZERO);
    let taxable_amount = parse_optional_decimal(start, &[b"TaxableAmount"], "TaxableAmount")?;
    let tax_amount = parse_optional_decimal(start, &[b"Amount"], "TaxAmount")?;
    Ok(TaxBreakdown {
        category: TaxCategory::Vat,
        rate_percent: rate,
        taxable_amount,
        tax_amount,
    })
}

fn contract_from_attrs(start: &BytesStart<'_>) -> ContractReference {
    ContractReference {
        document_id: attr_value(start, b"DocumentID").unwrap_or_else(|| "x86-contract".to_owned()),
        kind: contract_kind(attr_value(start, b"Kind").as_deref()),
        relation: attr_value(start, b"Relation")
            .unwrap_or_else(|| "X89 invoice references X86 contract award baseline".to_owned()),
        ordinal: attr_value(start, b"RNo"),
    }
}

fn quantity_evidence_from_attrs(start: &BytesStart<'_>) -> QuantityEvidenceReference {
    QuantityEvidenceReference {
        document_id: attr_value(start, b"DocumentID")
            .unwrap_or_else(|| "x31-measurement".to_owned()),
        kind: quantity_kind(attr_value(start, b"Kind").as_deref()),
        relation: attr_value(start, b"Relation")
            .unwrap_or_else(|| "X89 invoice references X31 measured quantity evidence".to_owned()),
        ordinal: attr_value(start, b"RNo"),
    }
}

fn payment_from_attrs(start: &BytesStart<'_>) -> PaymentApplication {
    PaymentApplication {
        terms: attr_value(start, b"Terms"),
        due_date: attr_value(start, b"DueDate"),
        payment_reference: attr_value(start, b"PaymentReference"),
        buyer_reference: attr_value(start, b"BuyerReference"),
        metadata: BTreeMap::new(),
    }
}

fn parse_optional_decimal(
    start: &BytesStart<'_>,
    keys: &[&[u8]],
    field: &str,
) -> Result<Option<Decimal>, ParseError> {
    keys.iter()
        .find_map(|key| attr_value(start, key))
        .map(|value| parse_decimal(&value, field))
        .transpose()
}

fn parse_decimal(value: &str, field: &str) -> Result<Decimal, ParseError> {
    value
        .trim()
        .replace(',', ".")
        .parse::<Decimal>()
        .map_err(|error| ParseError {
            code: "x89_decimal_parse_failed".to_owned(),
            message: format!("invalid decimal in {field}: {error}"),
            location: Some(field.to_owned()),
        })
}

fn invoice_type(value: Option<&str>) -> InvoiceType {
    if value.unwrap_or_default().eq_ignore_ascii_case("progress") {
        InvoiceType::ProgressInvoice
    } else {
        InvoiceType::Invoice
    }
}

fn party_role(value: &str) -> InvoicePartyRole {
    if value.eq_ignore_ascii_case("seller") {
        InvoicePartyRole::Seller
    } else if value.eq_ignore_ascii_case("buyer") {
        InvoicePartyRole::Buyer
    } else {
        InvoicePartyRole::Unknown
    }
}

fn contract_kind(value: Option<&str>) -> ContractBaselineKind {
    if value.unwrap_or_default().eq_ignore_ascii_case("x86") {
        ContractBaselineKind::X86Contract
    } else {
        ContractBaselineKind::Unknown
    }
}

fn quantity_kind(value: Option<&str>) -> QuantityEvidenceKind {
    if value.unwrap_or_default().eq_ignore_ascii_case("x31") {
        QuantityEvidenceKind::X31Measurement
    } else {
        QuantityEvidenceKind::Unknown
    }
}

fn is_unsupported_tax(local: &str) -> bool {
    local == "UnsupportedTax"
}

fn is_unsupported_payment(local: &str) -> bool {
    local == "UnsupportedPayment" || local == "DirectDebitMandate"
}

fn unsupported_finding(code: &str, field: &str, location: &str) -> ValidationFinding {
    ValidationFinding::warning(
        code,
        format!("unsupported X89 field {field} was preserved as a finding"),
    )
    .at(location.to_owned())
}

fn local_name(name: QName<'_>) -> String {
    String::from_utf8_lossy(name.as_ref())
        .rsplit(':')
        .next()
        .unwrap_or_default()
        .to_owned()
}

fn attr_value(start: &BytesStart<'_>, key: &[u8]) -> Option<String> {
    start
        .attributes()
        .flatten()
        .find(|attr| attr.key.as_ref() == key)
        .map(|attr| String::from_utf8_lossy(attr.value.as_ref()).to_string())
}

/// A complete GAEB X89 invoice-domain document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceDocument {
    /// Source provenance for future X89 parser output or design fixtures.
    pub source: SourceProvenance,
    /// Invoice header and currency metadata.
    pub header: InvoiceHeader,
    /// Parties participating in the invoice, such as supplier and buyer.
    pub parties: Vec<InvoiceParty>,
    /// Invoice lines in deterministic source order.
    pub lines: Vec<InvoiceLine>,
    /// Document-level contract/baseline relationships.
    pub contract_links: Vec<ContractReference>,
    /// Document-level measurement/progress evidence relationships.
    pub quantity_evidence: Vec<QuantityEvidenceReference>,
    /// Totals represented by or derived from the invoice data.
    pub totals: InvoiceTotals,
    /// Payment terms and payment-application metadata.
    pub payment: PaymentApplication,
    /// Recoverable findings for audit, validation, and public-sector billing gaps.
    pub findings: Vec<ValidationFinding>,
    /// Explicit boundary marker: this model alone never emits XRechnung.
    pub xrechnung_generated: bool,
    /// Document-level metadata for unmapped GAEB fields.
    pub metadata: Metadata,
}

impl InvoiceDocument {
    /// Creates an empty X89 invoice document for a known source.
    #[must_use]
    pub fn new(source: SourceProvenance, header: InvoiceHeader) -> Self {
        Self {
            source,
            header,
            parties: Vec::new(),
            lines: Vec::new(),
            contract_links: Vec::new(),
            quantity_evidence: Vec::new(),
            totals: InvoiceTotals::default(),
            payment: PaymentApplication::default(),
            findings: Vec::new(),
            xrechnung_generated: false,
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a party to the invoice document.
    pub fn add_party(&mut self, party: InvoiceParty) {
        self.parties.push(party);
    }

    /// Adds a line to the invoice document.
    pub fn add_line(&mut self, line: InvoiceLine) {
        self.lines.push(line);
    }

    /// Adds a contract baseline link at document scope.
    pub fn add_contract_link(&mut self, link: ContractReference) {
        self.contract_links.push(link);
    }

    /// Adds quantity or progress evidence at document scope.
    pub fn add_quantity_evidence(&mut self, evidence: QuantityEvidenceReference) {
        self.quantity_evidence.push(evidence);
    }

    /// Recomputes deterministic monetary totals from invoice lines.
    ///
    /// Missing line tax amounts are treated as zero. This is a checksum-style
    /// consistency helper, not a tax law engine.
    pub fn recalculate_totals(&mut self) {
        let net_amount = self
            .lines
            .iter()
            .filter_map(|line| line.net_amount)
            .fold(Decimal::ZERO, |total, amount| total + amount);
        let tax_amount = self
            .lines
            .iter()
            .filter_map(|line| line.tax.as_ref().and_then(|tax| tax.tax_amount))
            .fold(Decimal::ZERO, |total, amount| total + amount);
        self.totals.net_amount = net_amount;
        self.totals.tax_amount = tax_amount;
        self.totals.gross_amount = net_amount + tax_amount;
    }

    /// Returns all invoice lines linked to a BoQ ordinal.
    #[must_use]
    pub fn lines_for_ordinal(&self, ordinal: &str) -> Vec<&InvoiceLine> {
        self.lines
            .iter()
            .filter(|line| line.ordinal.as_deref() == Some(ordinal))
            .collect()
    }

    /// Adds deterministic audit findings required before public-sector billing.
    ///
    /// The findings identify missing contract, quantity, tax, and payment data.
    /// They are intentionally non-fatal so future parser work can preserve
    /// partial X89 files without overclaiming billing readiness.
    pub fn record_public_sector_audit_findings(&mut self) {
        if self.contract_links.is_empty() && self.lines.iter().all(|line| line.contract.is_none()) {
            self.findings.push(
                ValidationFinding::warning(
                    "x89_missing_contract_baseline",
                    "X89 invoice has no X86 contract baseline reference",
                )
                .at(self.header.invoice_id.clone()),
            );
        }
        if self.quantity_evidence.is_empty()
            && self
                .lines
                .iter()
                .all(|line| line.quantity_evidence.is_empty())
        {
            self.findings.push(
                ValidationFinding::warning(
                    "x89_missing_x31_quantity_evidence",
                    "X89 invoice has no X31 measurement or progress evidence reference",
                )
                .at(self.header.invoice_id.clone()),
            );
        }
        if self.lines.iter().any(|line| line.tax.is_none()) {
            self.findings.push(
                ValidationFinding::warning(
                    "x89_missing_tax_breakdown",
                    "At least one X89 invoice line has no tax breakdown",
                )
                .at(self.header.invoice_id.clone()),
            );
        }
        if self.payment.terms.is_none() && self.payment.due_date.is_none() {
            self.findings.push(
                ValidationFinding::warning(
                    "x89_missing_payment_terms",
                    "X89 invoice has no payment terms or due date",
                )
                .at(self.header.invoice_id.clone()),
            );
        }
    }

    /// Returns a stable boundary statement for downstream bridges.
    #[must_use]
    pub fn xrechnung_boundary(&self) -> XRechnungBoundary {
        XRechnungBoundary {
            generated: self.xrechnung_generated,
            reason: "GAEB X89 invoice model is source-domain data; XRechnung envelope generation is a separate bridge"
                .to_owned(),
            required_bridge: "xrechnung-bridge".to_owned(),
        }
    }
}

/// Invoice header fields that identify the X89 invoice independently of lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceHeader {
    /// Stable invoice identifier from the X89 source.
    pub invoice_id: String,
    /// Optional invoice issue date as source text.
    pub invoice_date: Option<String>,
    /// Invoice type classification.
    pub invoice_type: InvoiceType,
    /// ISO-like currency code used by monetary values.
    pub currency: String,
    /// Optional project or contract identifier.
    pub project_id: Option<String>,
    /// Header metadata for unmapped GAEB fields.
    pub metadata: Metadata,
}

impl InvoiceHeader {
    /// Creates a standard invoice header with a known id and currency.
    #[must_use]
    pub fn new(invoice_id: impl Into<String>, currency: impl Into<String>) -> Self {
        Self {
            invoice_id: invoice_id.into(),
            invoice_date: None,
            invoice_type: InvoiceType::Invoice,
            currency: currency.into(),
            project_id: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Adds the source invoice date.
    #[must_use]
    pub fn with_invoice_date(mut self, invoice_date: impl Into<String>) -> Self {
        self.invoice_date = Some(invoice_date.into());
        self
    }

    /// Adds a project or contract identifier.
    #[must_use]
    pub fn with_project_id(mut self, project_id: impl Into<String>) -> Self {
        self.project_id = Some(project_id.into());
        self
    }
}

/// Supported invoice type markers for the Rechnung model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceType {
    /// Standard invoice.
    Invoice,
    /// Partial or progress invoice.
    ProgressInvoice,
    /// Final invoice.
    FinalInvoice,
    /// Credit note or correction.
    CreditNote,
    /// Unknown or deferred invoice classification.
    Unknown,
}

/// A party participating in the invoice exchange.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceParty {
    /// Party role in the X89 document.
    pub role: InvoicePartyRole,
    /// Human-readable party name.
    pub name: String,
    /// Optional electronic endpoint identifier.
    pub endpoint_id: Option<String>,
    /// Optional tax identifier.
    pub tax_id: Option<String>,
    /// Party metadata for unmapped GAEB fields.
    pub metadata: Metadata,
}

impl InvoiceParty {
    /// Creates a party with a role and name.
    #[must_use]
    pub fn new(role: InvoicePartyRole, name: impl Into<String>) -> Self {
        Self {
            role,
            name: name.into(),
            endpoint_id: None,
            tax_id: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Adds an electronic endpoint identifier.
    #[must_use]
    pub fn with_endpoint_id(mut self, endpoint_id: impl Into<String>) -> Self {
        self.endpoint_id = Some(endpoint_id.into());
        self
    }

    /// Adds a tax identifier.
    #[must_use]
    pub fn with_tax_id(mut self, tax_id: impl Into<String>) -> Self {
        self.tax_id = Some(tax_id.into());
        self
    }
}

/// Known invoice party roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePartyRole {
    /// Supplier, contractor, or biller.
    Seller,
    /// Buyer, client, or public-sector recipient.
    Buyer,
    /// Payee account holder.
    Payee,
    /// Tax representative.
    TaxRepresentative,
    /// Unknown or deferred party role.
    Unknown,
}

/// One X89 invoice line, usually linked to a BoQ ordinal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceLine {
    /// Stable line identifier from the X89 source.
    pub line_id: String,
    /// Linked BoQ ordinal, when present.
    pub ordinal: Option<String>,
    /// Optional line description.
    pub description: Option<String>,
    /// Invoiced quantity.
    pub quantity: Decimal,
    /// Quantity unit.
    pub unit: String,
    /// Unit price for this invoice line.
    pub unit_price: Decimal,
    /// Net line amount, normally quantity times unit price.
    pub net_amount: Option<Decimal>,
    /// Tax breakdown for this line.
    pub tax: Option<TaxBreakdown>,
    /// Contract baseline relation for this line.
    pub contract: Option<ContractReference>,
    /// Measurement/progress evidence used for this invoice line.
    pub quantity_evidence: Vec<QuantityEvidenceReference>,
    /// Line-level validation or audit findings.
    pub findings: Vec<ValidationFinding>,
    /// Line metadata for unmapped GAEB fields.
    pub metadata: Metadata,
}

impl InvoiceLine {
    /// Creates an invoice line and calculates the net amount when multiplication fits.
    #[must_use]
    pub fn new(
        line_id: impl Into<String>,
        ordinal: impl Into<String>,
        unit: impl Into<String>,
        quantity: Decimal,
        unit_price: Decimal,
    ) -> Self {
        let net_amount = quantity.checked_mul(unit_price);
        Self {
            line_id: line_id.into(),
            ordinal: Some(ordinal.into()),
            description: None,
            quantity,
            unit: unit.into(),
            unit_price,
            net_amount,
            tax: None,
            contract: None,
            quantity_evidence: Vec::new(),
            findings: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a human-readable line description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds tax details to this invoice line.
    #[must_use]
    pub const fn with_tax(mut self, tax: TaxBreakdown) -> Self {
        self.tax = Some(tax);
        self
    }

    /// Adds a contract baseline relation to this invoice line.
    #[must_use]
    pub fn with_contract(mut self, contract: ContractReference) -> Self {
        self.contract = Some(contract);
        self
    }

    /// Adds a quantity or progress evidence relation to this invoice line.
    #[must_use]
    pub fn with_quantity_evidence(mut self, evidence: QuantityEvidenceReference) -> Self {
        self.quantity_evidence.push(evidence);
        self
    }
}

/// Tax details associated with an invoice line or total.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TaxBreakdown {
    /// Tax category, for example VAT standard rate.
    pub category: TaxCategory,
    /// Tax percentage rate.
    pub rate_percent: Decimal,
    /// Taxable base amount.
    pub taxable_amount: Option<Decimal>,
    /// Tax amount.
    pub tax_amount: Option<Decimal>,
}

impl TaxBreakdown {
    /// Creates a tax breakdown from rate and taxable amount.
    #[must_use]
    pub fn vat(rate_percent: Decimal, taxable_amount: Decimal) -> Self {
        Self {
            category: TaxCategory::Vat,
            rate_percent,
            taxable_amount: Some(taxable_amount),
            tax_amount: taxable_amount
                .checked_mul(rate_percent)
                .and_then(|amount| amount.checked_div(Decimal::new(100, 0))),
        }
    }
}

/// Known tax categories for invoice planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxCategory {
    /// Value-added tax.
    Vat,
    /// Tax-exempt amount.
    Exempt,
    /// Reverse-charge handling.
    ReverseCharge,
    /// Unknown or deferred tax classification.
    Unknown,
}

/// Document totals for X89 invoice data.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceTotals {
    /// Net amount before tax.
    pub net_amount: Decimal,
    /// Total tax amount.
    pub tax_amount: Decimal,
    /// Gross amount including tax.
    pub gross_amount: Decimal,
    /// Paid or applied amount when known.
    pub paid_amount: Option<Decimal>,
    /// Remaining amount due when known.
    pub due_amount: Option<Decimal>,
}

/// Payment terms and public-sector payment application metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PaymentApplication {
    /// Payment terms as source text.
    pub terms: Option<String>,
    /// Due date as source text.
    pub due_date: Option<String>,
    /// Payment reference or remittance information.
    pub payment_reference: Option<String>,
    /// Public-sector buyer reference, when supplied.
    pub buyer_reference: Option<String>,
    /// Payment metadata for unmapped GAEB fields.
    pub metadata: Metadata,
}

impl PaymentApplication {
    /// Creates a payment application with source payment terms.
    #[must_use]
    pub fn with_terms(terms: impl Into<String>) -> Self {
        Self {
            terms: Some(terms.into()),
            due_date: None,
            payment_reference: None,
            buyer_reference: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a due date.
    #[must_use]
    pub fn with_due_date(mut self, due_date: impl Into<String>) -> Self {
        self.due_date = Some(due_date.into());
        self
    }

    /// Adds a payment reference.
    #[must_use]
    pub fn with_payment_reference(mut self, payment_reference: impl Into<String>) -> Self {
        self.payment_reference = Some(payment_reference.into());
        self
    }

    /// Adds a public-sector buyer reference.
    #[must_use]
    pub fn with_buyer_reference(mut self, buyer_reference: impl Into<String>) -> Self {
        self.buyer_reference = Some(buyer_reference.into());
        self
    }
}

/// Reference from X89 invoice data back to a contract or tender baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractReference {
    /// Baseline document identifier, source path, or checksum.
    pub document_id: String,
    /// Baseline kind.
    pub kind: ContractBaselineKind,
    /// Relation from invoice to baseline.
    pub relation: String,
    /// Optional BoQ ordinal covered by the reference.
    pub ordinal: Option<String>,
}

impl ContractReference {
    /// Creates an X86 contract baseline relation.
    #[must_use]
    pub fn x86_contract(document_id: impl Into<String>, ordinal: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
            kind: ContractBaselineKind::X86Contract,
            relation: "X89 invoice line references X86 contract award baseline".to_owned(),
            ordinal: Some(ordinal.into()),
        }
    }
}

/// Supported contract baseline kinds for X89 planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractBaselineKind {
    /// X86 contract award baseline.
    X86Contract,
    /// X83 request/tender baseline.
    X83Tender,
    /// Unknown or deferred baseline kind.
    Unknown,
}

/// Reference from X89 invoice data to quantity/progress evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuantityEvidenceReference {
    /// Evidence document identifier, source path, or checksum.
    pub document_id: String,
    /// Evidence kind.
    pub kind: QuantityEvidenceKind,
    /// Relation from invoice to quantity evidence.
    pub relation: String,
    /// Optional BoQ ordinal covered by the evidence.
    pub ordinal: Option<String>,
}

impl QuantityEvidenceReference {
    /// Creates an X31 measurement evidence relation.
    #[must_use]
    pub fn x31_measurement(document_id: impl Into<String>, ordinal: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
            kind: QuantityEvidenceKind::X31Measurement,
            relation: "X89 invoice line references X31 measured quantity evidence".to_owned(),
            ordinal: Some(ordinal.into()),
        }
    }
}

/// Supported quantity evidence kinds for X89 planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuantityEvidenceKind {
    /// X31 quantity takeoff or measurement evidence.
    X31Measurement,
    /// Manual or external progress statement.
    ProgressStatement,
    /// Unknown or deferred quantity evidence kind.
    Unknown,
}

/// Explicit XRechnung bridge boundary metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XRechnungBoundary {
    /// Whether an XRechnung payload has been generated by this model.
    pub generated: bool,
    /// Human-readable boundary explanation.
    pub reason: String,
    /// Name of the separate bridge required for envelope generation.
    pub required_bridge: String,
}
