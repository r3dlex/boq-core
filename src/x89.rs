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

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::ValidationFinding;
use crate::model::{Metadata, SourceProvenance};

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
