//! Regression tests for the X89 Rechnung invoice-domain model.

use boq_core::model::{GaebFormat, GaebPhase, SourceProvenance};
use boq_core::x89::{
    ContractBaselineKind, ContractReference, InvoiceDocument, InvoiceHeader, InvoiceLine,
    InvoiceParty, InvoicePartyRole, InvoiceType, PaymentApplication, QuantityEvidenceKind,
    QuantityEvidenceReference, TaxBreakdown,
};
use rust_decimal::Decimal;

#[test]
fn test_x89_domain_represents_invoice_header() {
    let mut invoice = InvoiceDocument::new(
        source(),
        InvoiceHeader::new("RE-2026-0001", "EUR")
            .with_invoice_date("2026-06-21")
            .with_project_id("P-100"),
    );
    invoice.header.invoice_type = InvoiceType::ProgressInvoice;
    invoice.add_party(
        InvoiceParty::new(InvoicePartyRole::Seller, "Bau GmbH")
            .with_endpoint_id("0204:seller")
            .with_tax_id("DE123456789"),
    );
    invoice.add_party(InvoiceParty::new(InvoicePartyRole::Buyer, "Stadt Muster"));

    assert_eq!(invoice.header.invoice_id, "RE-2026-0001");
    assert_eq!(invoice.header.invoice_date.as_deref(), Some("2026-06-21"));
    assert_eq!(invoice.header.project_id.as_deref(), Some("P-100"));
    assert_eq!(invoice.header.currency, "EUR");
    assert_eq!(invoice.header.invoice_type, InvoiceType::ProgressInvoice);
    assert_eq!(invoice.parties[0].role, InvoicePartyRole::Seller);
    assert_eq!(
        invoice.parties[0].endpoint_id.as_deref(),
        Some("0204:seller")
    );
}

#[test]
fn test_x89_domain_represents_line_amounts() {
    let mut invoice = InvoiceDocument::new(source(), InvoiceHeader::new("RE-2", "EUR"));
    invoice.add_line(
        InvoiceLine::new(
            "L-1",
            "001.0010",
            "m3",
            Decimal::new(250, 1),
            Decimal::new(1200, 2),
        )
        .with_description("Concrete")
        .with_tax(TaxBreakdown::vat(
            Decimal::new(1900, 2),
            Decimal::new(30000, 2),
        )),
    );
    invoice.add_line(InvoiceLine::new(
        "L-2",
        "001.0020",
        "m",
        Decimal::new(10, 0),
        Decimal::new(5, 0),
    ));
    invoice.recalculate_totals();

    assert_eq!(invoice.lines[0].net_amount, Some(Decimal::new(30000, 2)));
    assert_eq!(invoice.lines[0].description.as_deref(), Some("Concrete"));
    assert_eq!(invoice.totals.net_amount, Decimal::new(35000, 2));
    assert_eq!(invoice.totals.tax_amount, Decimal::new(5700, 2));
    assert_eq!(invoice.totals.gross_amount, Decimal::new(40700, 2));
    assert_eq!(invoice.lines_for_ordinal("001.0010").len(), 1);
}

#[test]
fn test_x89_domain_links_contract_baseline() {
    let contract = ContractReference::x86_contract("sha256:x86-contract", "001.0010");
    let quantity = QuantityEvidenceReference::x31_measurement("sha256:x31-measurement", "001.0010");
    let line = InvoiceLine::new(
        "L-1",
        "001.0010",
        "m2",
        Decimal::new(42, 0),
        Decimal::new(3, 0),
    )
    .with_contract(contract.clone())
    .with_quantity_evidence(quantity.clone());
    let mut invoice = InvoiceDocument::new(source(), InvoiceHeader::new("RE-3", "EUR"));
    invoice.add_contract_link(contract);
    invoice.add_quantity_evidence(quantity);
    invoice.add_line(line);

    assert_eq!(
        invoice.contract_links[0].kind,
        ContractBaselineKind::X86Contract
    );
    assert_eq!(
        invoice.contract_links[0].ordinal.as_deref(),
        Some("001.0010")
    );
    assert_eq!(
        invoice.quantity_evidence[0].kind,
        QuantityEvidenceKind::X31Measurement
    );
    assert_eq!(
        invoice.lines[0]
            .contract
            .as_ref()
            .map(|c| c.document_id.as_str()),
        Some("sha256:x86-contract")
    );
    assert_eq!(
        invoice.lines[0].quantity_evidence[0].document_id,
        "sha256:x31-measurement"
    );
}

#[test]
fn test_x89_domain_does_not_claim_xrechnung_support() {
    let invoice = InvoiceDocument::new(source(), InvoiceHeader::new("RE-4", "EUR"));
    let boundary = invoice.xrechnung_boundary();

    assert!(!invoice.xrechnung_generated);
    assert!(!boundary.generated);
    assert_eq!(boundary.required_bridge, "xrechnung-bridge");
    assert!(boundary.reason.contains("separate bridge"));
}

#[test]
fn test_x89_audit_findings_identify_public_sector_billing_gaps() {
    let mut invoice = InvoiceDocument::new(source(), InvoiceHeader::new("RE-5", "EUR"));
    invoice.add_line(InvoiceLine::new(
        "L-1",
        "001.0010",
        "h",
        Decimal::new(1, 0),
        Decimal::new(80, 0),
    ));
    invoice.record_public_sector_audit_findings();

    let codes: Vec<&str> = invoice
        .findings
        .iter()
        .map(|finding| finding.code.as_str())
        .collect();
    assert!(codes.contains(&"x89_missing_contract_baseline"));
    assert!(codes.contains(&"x89_missing_x31_quantity_evidence"));
    assert!(codes.contains(&"x89_missing_tax_breakdown"));
    assert!(codes.contains(&"x89_missing_payment_terms"));
}

#[test]
fn test_x89_payment_application_keeps_public_sector_references() {
    let mut invoice = InvoiceDocument::new(source(), InvoiceHeader::new("RE-6", "EUR"));
    invoice.payment = PaymentApplication::with_terms("30 days")
        .with_due_date("2026-07-21")
        .with_payment_reference("RF18539007547034")
        .with_buyer_reference("04011000-12345-34");

    assert_eq!(invoice.payment.terms.as_deref(), Some("30 days"));
    assert_eq!(invoice.payment.due_date.as_deref(), Some("2026-07-21"));
    assert_eq!(
        invoice.payment.payment_reference.as_deref(),
        Some("RF18539007547034")
    );
    assert_eq!(
        invoice.payment.buyer_reference.as_deref(),
        Some("04011000-12345-34")
    );
}

fn source() -> SourceProvenance {
    SourceProvenance {
        source_uri: Some("gaeb/reference/rechnung/example.X89".to_owned()),
        source_format: GaebFormat::GaebXml,
        gaeb_version: Some("3.3".to_owned()),
        phase: Some(GaebPhase {
            code: "89".to_owned(),
            label: Some("Rechnung".to_owned()),
        }),
        checksum: Some("sha256:x89-design".to_owned()),
        parser_version: boq_core::version().to_owned(),
    }
}
