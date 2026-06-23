#![allow(clippy::expect_used)]

//! Regression tests for the X89 Rechnung parser MVP.

use boq_core::x89::{ContractBaselineKind, InvoiceType, QuantityEvidenceKind};
use rust_decimal::Decimal;

const X89_FIXTURE: &str = include_str!("fixtures/synthetic/x89_invoice.X89");
const X89_URI: &str = "tests/fixtures/synthetic/x89_invoice.X89";

#[test]
fn test_x89_fixture_parses_invoice_header() {
    let invoice = boq_core::x89::parse_str(X89_FIXTURE, Some(X89_URI.to_owned()))
        .expect("synthetic X89 fixture parses");

    assert_eq!(invoice.source.gaeb_version.as_deref(), Some("3.3"));
    assert_eq!(
        invoice
            .source
            .phase
            .as_ref()
            .map(|phase| phase.code.as_str()),
        Some("89")
    );
    assert_eq!(invoice.header.invoice_id, "RE-2026-0042");
    assert_eq!(invoice.header.invoice_date.as_deref(), Some("2026-06-21"));
    assert_eq!(invoice.header.invoice_type, InvoiceType::ProgressInvoice);
    assert_eq!(invoice.header.currency, "EUR");
    assert_eq!(invoice.header.project_id.as_deref(), Some("P-X89"));
    assert_eq!(invoice.parties.len(), 2);
    assert_eq!(invoice.parties[0].name, "Bau GmbH");
    assert_eq!(
        invoice.payment.buyer_reference.as_deref(),
        Some("04011000-12345-34")
    );
}

#[test]
fn test_x89_fixture_parses_line_items() {
    let invoice = boq_core::x89::parse_str(X89_FIXTURE, Some(X89_URI.to_owned()))
        .expect("synthetic X89 fixture parses");

    assert_eq!(invoice.lines.len(), 2);
    assert_eq!(invoice.lines[0].line_id, "L-1");
    assert_eq!(invoice.lines[0].ordinal.as_deref(), Some("001.0010"));
    assert_eq!(invoice.lines[0].quantity, Decimal::new(1250, 2));
    assert_eq!(invoice.lines[0].unit, "m3");
    assert_eq!(invoice.lines[0].unit_price, Decimal::new(8000, 2));
    assert_eq!(invoice.lines[0].net_amount, Some(Decimal::new(100_000, 2)));
    assert_eq!(
        invoice.lines[0].tax.as_ref().and_then(|tax| tax.tax_amount),
        Some(Decimal::new(19000, 2))
    );
    assert_eq!(invoice.lines[1].description.as_deref(), Some("Site setup"));
    assert_eq!(invoice.lines[1].unit, "psch");
    assert_eq!(invoice.totals.net_amount, Decimal::new(130_000, 2));
    assert_eq!(invoice.totals.tax_amount, Decimal::new(19000, 2));
    assert_eq!(invoice.totals.gross_amount, Decimal::new(149_000, 2));
}

#[test]
fn test_x89_links_to_ordinal_or_contract_item() {
    let invoice = boq_core::x89::parse_str(X89_FIXTURE, Some(X89_URI.to_owned()))
        .expect("synthetic X89 fixture parses");

    assert_eq!(invoice.lines_for_ordinal("001.0010").len(), 1);
    assert_eq!(
        invoice.contract_links[0].kind,
        ContractBaselineKind::X86Contract
    );
    assert_eq!(invoice.contract_links[0].document_id, "sha256:x86-contract");
    assert_eq!(
        invoice.quantity_evidence[0].kind,
        QuantityEvidenceKind::X31Measurement
    );
    assert_eq!(
        invoice.quantity_evidence[0].document_id,
        "sha256:x31-measurement"
    );
    assert_eq!(
        invoice.lines[0]
            .contract
            .as_ref()
            .map(|link| link.ordinal.as_deref()),
        Some(Some("001.0010"))
    );
    assert_eq!(
        invoice.lines[0].quantity_evidence[0].ordinal.as_deref(),
        Some("001.0010")
    );
}

#[test]
fn test_x89_unsupported_tax_or_payment_fields_emit_findings() {
    let invoice = boq_core::x89::parse_str(X89_FIXTURE, Some(X89_URI.to_owned()))
        .expect("synthetic X89 fixture parses");

    assert!(
        invoice
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_payment_field")
    );
    assert!(
        invoice.lines[1]
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_tax_field")
    );
    assert!(
        invoice.lines[1]
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_payment_field")
    );
    assert!(!invoice.xrechnung_boundary().generated);
}

#[test]
fn test_x89_fixture_adapts_to_obra_billing_draft_without_xrechnung() {
    let invoice = boq_core::x89::parse_str(X89_FIXTURE, Some(X89_URI.to_owned()))
        .expect("synthetic X89 fixture parses");

    let draft = boq_core::x89::ObraBillingDraft::from_x89(&invoice);

    assert_eq!(draft.invoice_id, "RE-2026-0042");
    assert_eq!(draft.currency, "EUR");
    assert_eq!(draft.lines.len(), 2);
    assert_eq!(draft.lines[0].boq_ordinal.as_deref(), Some("001.0010"));
    assert_eq!(draft.lines[0].net_amount, Some(Decimal::new(100_000, 2)));
    assert_eq!(draft.totals.gross_amount, Decimal::new(149_000, 2));
    assert!(draft.deterministic_key.starts_with("x89-billing:"));
    assert!(draft.lines[0].deterministic_key.contains("L-1"));
    assert_eq!(draft.source.source_uri.as_deref(), Some(X89_URI));
    assert_eq!(draft.xrechnung_boundary.required_bridge, "xrechnung-bridge");
    assert!(!draft.xrechnung_boundary.generated);
    assert!(!draft.readiness.ready_for_public_sector_billing);
    assert!(
        draft
            .readiness
            .blocking_findings
            .iter()
            .any(|code| code == "x89_missing_tax_breakdown")
    );
    assert!(
        draft
            .loss_report
            .warnings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_payment_field")
    );
    assert!(
        draft
            .loss_report
            .unsupported_fields
            .iter()
            .any(|field| field.ends_with("UnsupportedPayment")
                || field.ends_with("DirectDebitMandate")),
        "unsupported fields should preserve actual X89 field paths: {:?}",
        draft.loss_report.unsupported_fields
    );
    assert!(
        draft
            .loss_report
            .unsupported_fields
            .iter()
            .any(|field| field.ends_with("UnsupportedTax")),
        "unsupported fields should preserve unsupported tax field paths: {:?}",
        draft.loss_report.unsupported_fields
    );
}

#[test]
fn test_x89_parse_file_reads_invoice_fixture() {
    let invoice = boq_core::x89::parse_file(X89_URI).expect("synthetic X89 fixture file parses");

    assert_eq!(invoice.header.invoice_id, "RE-2026-0042");
    assert!(invoice.source.checksum.is_some());
}

#[test]
fn test_x89_parser_reports_decimal_parse_errors() {
    let xml = r#"<GAEB><Invoice ID="RE-BAD"><Line ID="L-BAD" RNo="001" Qty="not-a-number" Unit="m" UnitPrice="1" /></Invoice></GAEB>"#;
    let error = boq_core::x89::parse_str(xml, Some("bad.X89".to_owned()))
        .expect_err("invalid decimal fails");

    assert_eq!(error.code, "x89_decimal_parse_failed");
    assert_eq!(error.location.as_deref(), Some("Qty"));
}

#[test]
fn test_x89_parser_reports_malformed_lines() {
    let xml = r#"<GAEB><Invoice ID="RE-BAD"><Line ID="L-BAD" RNo="001" Qty="1" Unit="m" UnitPrice="1"></Invoice></GAEB>"#;
    let error = boq_core::x89::parse_str(xml, Some("bad.X89".to_owned()))
        .expect_err("malformed line fails");

    assert_eq!(error.code, "x89_xml_parse_failed");
    assert!(!error.message.is_empty());
}

#[test]
fn test_x89_parser_covers_defaults_and_unknown_kinds() {
    let xml = r#"<GAEB>
        <Invoice />
        <Party Role="Inspector" />
        <ContractRef Kind="other" />
        <QuantityEvidence Kind="manual" />
        <UnsupportedTax />
        <Line>
            <UnknownLineField>kept as finding</UnknownLineField>
            <OtherEmpty />
            <Tax />
        </Line>
    </GAEB>"#;
    let invoice = boq_core::x89::parse_str(xml, None).expect("defaulted X89 XML parses");

    assert_eq!(invoice.header.invoice_id, "x89-unknown");
    assert_eq!(invoice.header.currency, "EUR");
    assert_eq!(invoice.source.source_uri, None);
    assert_eq!(
        invoice
            .source
            .phase
            .as_ref()
            .map(|phase| phase.code.as_str()),
        Some("89")
    );
    assert_eq!(invoice.parties[0].name, "unknown party");
    assert_eq!(
        invoice.contract_links[0].kind,
        ContractBaselineKind::Unknown
    );
    assert_eq!(invoice.contract_links[0].document_id, "x86-contract");
    assert_eq!(
        invoice.quantity_evidence[0].kind,
        QuantityEvidenceKind::Unknown
    );
    assert_eq!(invoice.quantity_evidence[0].document_id, "x31-measurement");
    assert!(
        invoice
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_tax_field")
    );
    assert_eq!(invoice.lines[0].line_id, "x89-line");
    assert_eq!(invoice.lines[0].ordinal.as_deref(), Some("x89-line"));
    assert_eq!(invoice.lines[0].unit_price, Decimal::ZERO);
    assert_eq!(
        invoice.lines[0].tax.as_ref().map(|tax| tax.rate_percent),
        Some(Decimal::ZERO)
    );
    assert!(
        invoice.lines[0]
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_invoice_line_field")
    );
}

#[test]
fn test_x89_parse_file_reports_missing_file() {
    let error = boq_core::x89::parse_file("tests/fixtures/synthetic/missing-invoice.X89")
        .expect_err("missing X89 file reports read failure");

    assert_eq!(error.code, "x89_read_failed");
    assert!(
        error
            .location
            .as_deref()
            .is_some_and(|path| path.ends_with("missing-invoice.X89"))
    );
}

#[test]
fn test_x89_parser_reports_unclosed_invoice_line() {
    let xml =
        r#"<GAEB><Invoice ID="RE-BAD"><Line ID="L-BAD" RNo="001" Qty="1" Unit="m" UnitPrice="1">"#;
    let error =
        boq_core::x89::parse_str(xml, Some("bad.X89".to_owned())).expect_err("unclosed line fails");

    assert_eq!(error.code, "x89_unclosed_invoice_line");
    assert_eq!(error.location.as_deref(), Some("L-BAD"));
}

#[test]
fn test_x89_parser_reports_document_level_start_unsupported_fields() {
    let xml = r#"<GAEB>
        <Invoice ID="RE-START" />
        <UnsupportedTax>reverse-charge</UnsupportedTax>
        <DirectDebitMandate>iban</DirectDebitMandate>
        <IgnoredEmpty />
    </GAEB>"#;
    let invoice = boq_core::x89::parse_str(xml, Some("start.X89".to_owned()))
        .expect("document unsupported starts parse as findings");

    assert!(
        invoice
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_tax_field")
    );
    assert!(
        invoice
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_payment_field")
    );
}

#[test]
fn test_x89_parser_reports_line_level_start_unsupported_fields() {
    let xml = r#"<GAEB><Invoice ID="RE-LINE"><Line ID="L-UNSUPPORTED" RNo="001" Qty="1" Unit="m" UnitPrice="2">
        <UnsupportedTax>construction reverse charge</UnsupportedTax>
        <DirectDebitMandate>mandate unavailable</DirectDebitMandate>
    </Line></Invoice></GAEB>"#;
    let invoice = boq_core::x89::parse_str(xml, Some("line-start.X89".to_owned()))
        .expect("line unsupported starts parse as findings");

    assert!(
        invoice.lines[0]
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_tax_field")
    );
    assert!(
        invoice.lines[0]
            .findings
            .iter()
            .any(|finding| finding.code == "x89_unsupported_payment_field")
    );
}

#[test]
fn test_x89_parser_reports_top_level_malformed_xml() {
    let xml = r#"<GAEB><Invoice ID="RE-BAD"></GAEB>"#;
    let error = boq_core::x89::parse_str(xml, Some("bad-top.X89".to_owned()))
        .expect_err("top-level malformed XML fails");

    assert_eq!(error.code, "x89_xml_parse_failed");
    assert_eq!(error.location, None);
}
