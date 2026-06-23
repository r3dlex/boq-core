#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::error::ValidationFinding;
use boq_core::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, GaebDocument, GaebDocumentSummary, GaebFormat, GaebPhase,
    RichText, SourceProvenance,
};
use boq_core::support::{SupportCapabilities, SupportStatus};
use boq_core::x31::{
    BaselineKind, MeasurementAttachment, MeasurementAttachmentKind, MeasurementBaselineLink,
    MeasurementFormula, MeasurementReference, MeasurementReferenceKind, MeasurementRow,
    PhysicalProgress, QuantityTakeoffDocument, RebFormulaSystem, X31CanonicalQuantitySource,
    X31X86LinkStatus,
};
use rust_decimal::Decimal;

#[test]
fn test_x31_domain_represents_formula_rows() {
    let row = MeasurementRow::formula(
        "row-001",
        "001.0010",
        "m3",
        MeasurementFormula::reb_vb_23003("L * B * H"),
    )
    .with_result(Decimal::new(1250, 2))
    .with_progress(PhysicalProgress {
        completed_quantity: Decimal::new(625, 2),
        percent_complete: Some(Decimal::new(5000, 2)),
    });

    assert_eq!(row.formula.system, RebFormulaSystem::RebVb23003);
    assert_eq!(row.formula.expression, "L * B * H");
    assert_eq!(row.result_quantity, Some(Decimal::new(1250, 2)));
    assert_eq!(
        row.progress.map(|progress| progress.completed_quantity),
        Some(Decimal::new(625, 2))
    );
}

#[test]
fn test_x31_domain_links_measurements_to_ordinal() {
    let mut document = QuantityTakeoffDocument::new(source());
    document.baseline = Some(MeasurementBaselineLink {
        document_id: "x86-contract-sha256".to_owned(),
        kind: BaselineKind::X86Contract,
        relation: "contract baseline for measured progress".to_owned(),
    });
    document.rows.push(MeasurementRow::formula(
        "row-001",
        "001.0010",
        "m",
        MeasurementFormula::reb_vb_23003("A + B"),
    ));
    document.rows[0].references.push(MeasurementReference {
        kind: MeasurementReferenceKind::BoqOrdinal,
        value: "001.0010".to_owned(),
    });

    assert_eq!(document.rows_for_ordinal("001.0010").len(), 1);
    assert_eq!(
        document.baseline.as_ref().map(|baseline| baseline.kind),
        Some(BaselineKind::X86Contract)
    );
}

#[test]
fn test_x31_domain_represents_attachments_as_findings_or_assets() {
    let mut document = QuantityTakeoffDocument::new(source());
    document.attachments.push(MeasurementAttachment {
        id: "drawing-A".to_owned(),
        kind: MeasurementAttachmentKind::Drawing,
        source_uri: Some("drawings/A.pdf".to_owned()),
        checksum: Some("sha256:abc".to_owned()),
        metadata: BTreeMap::default(),
    });
    document.record_attachment_gap(
        "photo-1",
        "external photo was not acquired as a local fixture",
    );

    assert_eq!(
        document.attachments[0].kind,
        MeasurementAttachmentKind::Drawing
    );
    assert!(document.findings.iter().any(|finding| {
        finding.code == "x31_attachment_reference_only"
            && finding.location.as_deref() == Some("photo-1")
    }));
}

#[test]
fn test_x31_domain_is_serializable() {
    let mut document = QuantityTakeoffDocument::new(source());
    document.rows.push(MeasurementRow::formula(
        "row-001",
        "001.0010",
        "m2",
        MeasurementFormula::reb_vb_23003("2.0 * 3.0"),
    ));
    document.findings.push(ValidationFinding::warning(
        "x31_formula_not_evaluated",
        "formula evaluation is handled by a later issue",
    ));

    let first = serde_json::to_string(&document).expect("document serializes");
    let second = serde_json::to_string(&document).expect("document serializes deterministically");
    assert_eq!(first, second);

    let reparsed: QuantityTakeoffDocument =
        serde_json::from_str(&first).expect("document deserializes");
    assert_eq!(reparsed.rows[0].ordinal.as_deref(), Some("001.0010"));
}

fn source() -> SourceProvenance {
    SourceProvenance {
        source_uri: Some("gaeb/bvbs/gaeb_xml_3_3/quantity_takeoff/x31/synthetic.X31".to_owned()),
        source_format: GaebFormat::GaebXml,
        gaeb_version: Some("3.3".to_owned()),
        phase: None,
        checksum: Some("sha256:test".to_owned()),
        parser_version: boq_core::VERSION.to_owned(),
    }
}

const X31_XML: &str = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>X31</Name><QtyTakeoff><MeasurementGroup ID="G-1"><FormulaRecord ID="FR-1" RNo="001.0010" Unit="m3"><Formula>L * B * H</Formula><Result>12.50</Result><Attachment ID="D-1" Type="drawing" HRef="drawings/detail-a.pdf" Checksum="sha256:abc"/><UnsupportedFeature>needs future parser</UnsupportedFeature></FormulaRecord></MeasurementGroup></QtyTakeoff></Project></GAEB>"#;
const X31_URI: &str = "gaeb/bvbs/gaeb_xml_3_3/quantity_takeoff/x31/synthetic.X31";

#[test]
fn test_bvbs_x31_parses_measurement_groups() {
    let document =
        boq_core::x31::parse_str(X31_XML, Some(X31_URI.to_owned())).expect("x31 fixture parses");
    let row = &document.rows[0];

    assert_eq!(document.source.gaeb_version.as_deref(), Some("3.3"));
    assert_eq!(row.row_id, "FR-1");
    assert_eq!(row.ordinal.as_deref(), Some("001.0010"));
    assert_eq!(
        row.metadata
            .get("x31.measurement_group_id")
            .and_then(serde_json::Value::as_str),
        Some("G-1")
    );
}

#[test]
fn test_bvbs_x31_formula_records_preserve_source() {
    let document =
        boq_core::x31::parse_str(X31_XML, Some(X31_URI.to_owned())).expect("x31 fixture parses");
    let row = &document.rows[0];

    assert_eq!(row.formula.system, RebFormulaSystem::RebVb23003);
    assert_eq!(row.formula.expression, "L * B * H");
    assert_eq!(row.result_quantity, Some(Decimal::new(1250, 2)));
    assert_eq!(row.unit, "m3");
}

#[test]
fn test_bvbs_x31_attachments_are_detected() {
    let document =
        boq_core::x31::parse_str(X31_XML, Some(X31_URI.to_owned())).expect("x31 fixture parses");

    assert_eq!(document.rows[0].attachment_ids, ["D-1"]);
    assert_eq!(
        document.attachments[0].kind,
        MeasurementAttachmentKind::Drawing
    );
    assert_eq!(
        document.attachments[0].source_uri.as_deref(),
        Some("drawings/detail-a.pdf")
    );
}

#[test]
fn test_x31_parser_reports_unsupported_features() {
    let document =
        boq_core::x31::parse_str(X31_XML, Some(X31_URI.to_owned())).expect("x31 fixture parses");

    assert!(document.findings.iter().any(|finding| {
        finding.code == "x31_unsupported_feature"
            && finding.location.as_deref() == Some("FR-1/UnsupportedFeature")
    }));
}

#[test]
fn test_bvbs_x31_support_promotion_requires_parser_evidence() {
    let manifest = std::fs::read_to_string("gaeb/manifest.toml").expect("manifest exists");
    let parsed: toml::Value = toml::from_str(&manifest).expect("manifest parses");
    let fixtures = parsed
        .get("fixtures")
        .and_then(toml::Value::as_array)
        .expect("fixtures array");
    let x31 = fixtures
        .iter()
        .filter_map(toml::Value::as_table)
        .find(|fixture| {
            fixture.get("id").and_then(toml::Value::as_str) == Some("bvbs_xml33_qty_x31")
        })
        .expect("x31 fixture exists");

    assert_eq!(
        x31.get("support_status").and_then(toml::Value::as_str),
        Some("supported_parse_only")
    );
    assert!(
        x31.get("license_note")
            .and_then(toml::Value::as_str)
            .expect("license note")
            .contains("synthetic")
    );
    let mappings = x31
        .get("test_mapping")
        .and_then(toml::Value::as_array)
        .expect("x31 test mappings");
    for expected in [
        "test_bvbs_x31_parses_measurement_groups",
        "test_bvbs_x31_formula_records_preserve_source",
        "test_bvbs_x31_attachments_are_detected",
        "test_x31_parser_reports_unsupported_features",
        "test_x31_canonical_quantity_integration_preserves_provenance_and_loss_findings",
        "test_x31_canonical_quantity_missing_result_is_loss_finding",
        "test_bvbs_x31_support_promotion_requires_parser_evidence",
    ] {
        assert!(
            mappings
                .iter()
                .any(|mapping| mapping.as_str() == Some(expected)),
            "missing x31 parser evidence: {expected}"
        );
    }
}

#[test]
fn test_x31_parser_accepts_aliases_child_units_and_decimal_comma() {
    let xml = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><QtyTakeoff><MeasGrp Id="G-A"><FormulaRow Id="ROW-A" Ordinal="002.0010"><Expression>1,25 + 2</Expression><Quantity>3,25</Quantity><QU>m2</QU><Drawing Id="PLAN-1" Type="plan" href="plan.pdf"/><Asset Id="PHOTO-1" Type="photo" Path="photo.jpg"/><Asset Id="CALC-1" Type="calculation_sheet" Path="calc.xlsx"/><Asset Id="UNK-1" Type="other"/></FormulaRow></MeasGrp><QtyGroup ID="G-B"><Measurement ID="ROW-B" OZ="003.0010" Unit="kg"><Formula>5</Formula><Qty>5</Qty></Measurement></QtyGroup></QtyTakeoff></GAEB>"#;

    let document = boq_core::x31::parse_str(xml, Some(X31_URI.to_owned())).expect("aliases parse");

    assert_eq!(document.rows.len(), 2);
    assert_eq!(document.rows[0].row_id, "ROW-A");
    assert_eq!(document.rows[0].ordinal.as_deref(), Some("002.0010"));
    assert_eq!(document.rows[0].formula.expression, "1,25 + 2");
    assert_eq!(document.rows[0].result_quantity, Some(Decimal::new(325, 2)));
    assert_eq!(document.rows[0].unit, "m2");
    assert_eq!(
        document.rows[0]
            .metadata
            .get("x31.measurement_group_id")
            .and_then(serde_json::Value::as_str),
        Some("G-A")
    );
    assert_eq!(document.rows[1].row_id, "ROW-B");
    assert_eq!(document.rows[1].ordinal.as_deref(), Some("003.0010"));
    assert_eq!(document.rows[1].unit, "kg");
    assert_eq!(document.attachments.len(), 4);
    assert_eq!(
        document.attachments[0].kind,
        MeasurementAttachmentKind::Drawing
    );
    assert_eq!(
        document.attachments[1].kind,
        MeasurementAttachmentKind::Photo
    );
    assert_eq!(
        document.attachments[2].kind,
        MeasurementAttachmentKind::CalculationSheet
    );
    assert_eq!(
        document.attachments[3].kind,
        MeasurementAttachmentKind::Unknown
    );
    assert_eq!(
        document.attachments[0].source_uri.as_deref(),
        Some("plan.pdf")
    );
    assert_eq!(
        document.attachments[1].source_uri.as_deref(),
        Some("photo.jpg")
    );
}

#[test]
fn test_x31_parse_file_reads_document_from_disk() {
    let temp_dir = std::path::Path::new("target").join("x31-parser-tests");
    std::fs::create_dir_all(&temp_dir).expect("temp dir");
    let path = temp_dir.join("parse-file.X31");
    std::fs::write(&path, X31_XML).expect("write x31 temp file");

    let document = boq_core::x31::parse_file(&path).expect("parse file succeeds");

    assert_eq!(document.rows.len(), 1);
    assert_eq!(
        document.source.source_uri.as_deref(),
        Some(path.to_str().expect("utf8 path"))
    );
    assert!(
        document
            .source
            .checksum
            .as_deref()
            .is_some_and(|checksum| checksum.len() == 64)
    );
}

#[test]
fn test_x31_parse_file_reports_missing_file() {
    let error = boq_core::x31::parse_file("target/x31-parser-tests/missing.X31")
        .expect_err("missing file fails");

    assert_eq!(error.code, "x31_read_failed");
    assert_eq!(
        error.location.as_deref(),
        Some("target/x31-parser-tests/missing.X31")
    );
}

#[test]
fn test_x31_parser_reports_document_level_unsupported_containers() {
    let xml = r"<GAEB><Sketch/><FreeMeasurement/><UnsupportedFeature/></GAEB>";

    let document = boq_core::x31::parse_str(xml, Some(X31_URI.to_owned())).expect("x31 parses");

    for expected in ["Sketch", "FreeMeasurement", "UnsupportedFeature"] {
        assert!(
            document.findings.iter().any(|finding| {
                finding.code == "x31_unsupported_feature"
                    && finding.location.as_deref() == Some(expected)
            }),
            "missing finding for {expected}"
        );
    }
}

#[test]
fn test_x31_parser_reports_decimal_parse_errors() {
    let xml = r#"<GAEB><FormulaRecord ID="BAD" RNo="001"><Result>not-a-number</Result></FormulaRecord></GAEB>"#;

    let error =
        boq_core::x31::parse_str(xml, Some(X31_URI.to_owned())).expect_err("invalid decimal fails");

    assert_eq!(error.code, "x31_decimal_parse_failed");
    assert_eq!(error.location.as_deref(), Some("Result"));
}

#[test]
fn test_x31_parser_reports_unclosed_rows() {
    let xml = r#"<GAEB><FormulaRecord ID="OPEN" RNo="001"><Formula>1</Formula>"#;

    let error =
        boq_core::x31::parse_str(xml, Some(X31_URI.to_owned())).expect_err("unclosed row fails");

    assert_eq!(error.code, "x31_unclosed_measurement_row");
    assert_eq!(error.location.as_deref(), Some("OPEN"));
}

#[test]
fn test_x31_parser_reports_malformed_xml() {
    let xml = r"<GAEB><Broken></GAEB>";

    let error =
        boq_core::x31::parse_str(xml, Some(X31_URI.to_owned())).expect_err("malformed xml fails");

    assert_eq!(error.code, "x31_xml_parse_failed");
}

#[test]
fn test_x31_parser_reports_started_unsupported_containers_and_empty_groups() {
    let xml = r"<GAEB><MeasurementGroup/><Sketch>reference-only</Sketch></GAEB>";

    let document = boq_core::x31::parse_str(xml, Some(X31_URI.to_owned())).expect("x31 parses");

    assert!(document.rows.is_empty());
    assert!(document.findings.iter().any(|finding| {
        finding.code == "x31_unsupported_feature" && finding.location.as_deref() == Some("Sketch")
    }));
}

#[test]
fn test_x31_parser_reports_empty_row_unknown_fields() {
    let xml = r#"<GAEB><FormulaRecord ID="ROW-UNSUPPORTED" RNo="001"><UnsupportedEmpty/></FormulaRecord></GAEB>"#;

    let document = boq_core::x31::parse_str(xml, Some(X31_URI.to_owned())).expect("x31 parses");

    assert!(document.findings.iter().any(|finding| {
        finding.code == "x31_unsupported_feature"
            && finding.location.as_deref() == Some("ROW-UNSUPPORTED/UnsupportedEmpty")
    }));
}

#[test]
fn test_x31_parser_preserves_nested_formula_markup_as_source_text() {
    let xml = r#"<GAEB><FormulaRecord ID="NESTED" RNo="001"><Formula><Nested/></Formula></FormulaRecord></GAEB>"#;

    let document = boq_core::x31::parse_str(xml, Some(X31_URI.to_owned()))
        .expect("nested formula markup is preserved as source text");

    assert_eq!(document.rows[0].formula.expression, "<Nested/>");
}

#[test]
fn test_x31_parser_reports_formula_text_read_errors() {
    let xml =
        r#"<GAEB><FormulaRecord ID="TEXTERR" RNo="001"><Formula>1</Other></FormulaRecord></GAEB>"#;

    let error = boq_core::x31::parse_str(xml, Some(X31_URI.to_owned()))
        .expect_err("mismatched formula text fails");

    assert_eq!(error.code, "x31_xml_text_read_failed");
    assert_eq!(error.location.as_deref(), Some("Formula"));
}

#[test]
fn test_reb_formula_simple_arithmetic() {
    let evaluation = boq_core::x31::evaluate_reb_vb_23003("2 + 3 * (4 - 1)");

    assert_eq!(evaluation.quantity, Some(Decimal::new(11, 0)));
    assert!(evaluation.findings.is_empty());
    assert!(boq_core::x31::SUPPORTED_REB_VB_23003_SUBSET.contains(&"multiplication"));
}

#[test]
fn test_reb_formula_quantity_result_precision() {
    let evaluation = boq_core::x31::evaluate_reb_vb_23003("1,25 * 2 + 0.005");

    assert_eq!(evaluation.quantity, Some(Decimal::new(2505, 3)));
    assert!(evaluation.findings.is_empty());
}

#[test]
fn test_reb_formula_unsupported_expression_yields_finding() {
    let evaluation = boq_core::x31::evaluate_reb_vb_23003("SIN(30) + 1");

    assert_eq!(evaluation.quantity, None);
    assert!(evaluation.findings.iter().any(|finding| {
        finding.code == "reb_formula_unsupported_token"
            && finding.message.contains("supported subset")
    }));
}

#[test]
fn test_formula_evaluator_never_panics_on_bad_input() {
    for bad in ["", "1 / 0", "(1 + 2", "1 + * 2", "1.2.3"] {
        let evaluation = boq_core::x31::evaluate_reb_vb_23003(bad);
        assert_eq!(
            evaluation.quantity, None,
            "bad expression unexpectedly evaluated: {bad}"
        );
        assert!(!evaluation.findings.is_empty(), "missing finding for {bad}");
    }
}

#[test]
fn test_reb_formula_unary_and_division_are_deterministic() {
    let evaluation = boq_core::x31::evaluate_reb_vb_23003(" +10 / -2 ");

    assert_eq!(evaluation.quantity, Some(Decimal::new(-5, 0)));
    assert!(evaluation.findings.is_empty());
}

#[test]
fn test_reb_formula_trailing_tokens_are_unevaluated() {
    let evaluation = boq_core::x31::evaluate_reb_vb_23003("1 2");

    assert_eq!(evaluation.quantity, None);
    assert!(evaluation.findings.iter().any(|finding| {
        finding.code == "reb_formula_unsupported_token" && finding.message.contains("near '2'")
    }));
}

#[test]
fn test_reb_formula_decimal_overflow_yields_finding() {
    let evaluation = boq_core::x31::evaluate_reb_vb_23003("79228162514264337593543950335 * 10");

    assert_eq!(evaluation.quantity, None);
    assert!(
        evaluation
            .findings
            .iter()
            .any(|finding| finding.code == "reb_formula_decimal_overflow")
    );
}

#[test]
fn test_x31_links_to_x86_by_ordinal() {
    let mut measurements = QuantityTakeoffDocument::new(source());
    measurements.rows.push(
        MeasurementRow::formula(
            "M-1",
            "001.0010",
            "m3",
            MeasurementFormula::reb_vb_23003("2 * 3"),
        )
        .with_result(Decimal::new(6, 0)),
    );
    let baseline = x86_baseline_document(vec![x86_item(
        "001.0010",
        Decimal::new(10, 0),
        "m3",
        Some(Decimal::new(25, 0)),
    )]);

    let report = boq_core::x31::link_x31_to_x86_baseline(&measurements, &baseline);

    assert_eq!(report.baseline.kind, BaselineKind::X86Contract);
    assert_eq!(report.rows[0].status, X31X86LinkStatus::Matched);
    assert_eq!(report.rows[0].canonical_quantity, Some(Decimal::new(6, 0)));
    assert_eq!(report.rows[0].canonical_unit, "m3");
    assert_eq!(
        report.rows[0].canonical_quantity_source,
        X31CanonicalQuantitySource::MeasurementResult
    );
    assert_eq!(report.rows[0].baseline_quantity, Some(Decimal::new(10, 0)));
    assert_eq!(report.rows[0].unit_price, Some(Decimal::new(25, 0)));
    assert_eq!(report.rows[0].progress_value, Some(Decimal::new(150, 0)));
    assert!(report.findings.is_empty());
    assert_eq!(
        report.measurement_source.source_uri,
        measurements.source.source_uri
    );
    assert!(!report.invoice_generated);
    assert!(!report.obra_import_supported);
}

#[test]
fn test_x31_canonical_quantity_integration_preserves_provenance_and_loss_findings() {
    let measurements =
        boq_core::x31::parse_str(X31_XML, Some(X31_URI.to_owned())).expect("x31 fixture parses");
    let baseline = x86_baseline_document(vec![x86_item(
        "001.0010",
        Decimal::new(10, 0),
        "m3",
        Some(Decimal::new(2, 0)),
    )]);

    let report = boq_core::x31::link_x31_to_x86_baseline(&measurements, &baseline);

    assert_eq!(
        report.measurement_source.source_uri.as_deref(),
        Some(X31_URI)
    );
    assert_eq!(
        report.rows[0].canonical_quantity,
        Some(Decimal::new(1250, 2))
    );
    assert_eq!(report.rows[0].canonical_unit, "m3");
    assert_eq!(report.rows[0].attachment_ids, ["D-1"]);
    assert_eq!(
        report.rows[0].canonical_quantity_source,
        X31CanonicalQuantitySource::MeasurementResult
    );
    assert!(report.findings.iter().any(|finding| {
        finding.code == "x31_unsupported_feature"
            && finding.location.as_deref() == Some("FR-1/UnsupportedFeature")
    }));
    assert_eq!(
        report
            .metadata
            .get("x31.integration")
            .and_then(serde_json::Value::as_str),
        Some("canonical_quantity_evidence")
    );
    assert!(!report.invoice_generated);
    assert!(!report.obra_import_supported);
}

#[test]
fn test_x31_canonical_quantity_missing_result_is_loss_finding() {
    let mut measurements = QuantityTakeoffDocument::new(source());
    measurements.rows.push(MeasurementRow::formula(
        "M-FORMULA-ONLY",
        "001.0010",
        "m",
        MeasurementFormula::reb_vb_23003("L * B"),
    ));
    let baseline = x86_baseline_document(vec![x86_item("001.0010", Decimal::new(1, 0), "m", None)]);

    let report = boq_core::x31::link_x31_to_x86_baseline(&measurements, &baseline);

    assert_eq!(
        report.rows[0].status,
        X31X86LinkStatus::MissingCanonicalQuantity
    );
    assert_eq!(report.rows[0].canonical_quantity, None);
    assert_eq!(
        report.rows[0].canonical_quantity_source,
        X31CanonicalQuantitySource::FormulaSourceOnly
    );
    assert!(report.findings.iter().any(|finding| {
        finding.code == "x31_canonical_quantity_missing_result"
            && finding.location.as_deref() == Some("M-FORMULA-ONLY")
    }));
    assert!(!report.obra_import_supported);
}

#[test]
fn test_x31_x86_quantity_mismatch_reports_finding() {
    let mut measurements = QuantityTakeoffDocument::new(source());
    measurements.rows.push(
        MeasurementRow::formula(
            "M-OVER",
            "001.0010",
            "m3",
            MeasurementFormula::reb_vb_23003("12"),
        )
        .with_result(Decimal::new(12, 0)),
    );
    let baseline =
        x86_baseline_document(vec![x86_item("001.0010", Decimal::new(10, 0), "m3", None)]);

    let report = boq_core::x31::link_x31_to_x86_baseline(&measurements, &baseline);

    assert_eq!(report.rows[0].status, X31X86LinkStatus::Mismatched);
    assert!(report.findings.iter().any(|finding| {
        finding.code == "x31_x86_quantity_exceeds_baseline"
            && finding.location.as_deref() == Some("001.0010")
    }));
}

#[test]
fn test_x31_unmatched_measurement_is_nonfatal() {
    let mut measurements = QuantityTakeoffDocument::new(source());
    measurements.rows.push(
        MeasurementRow::formula(
            "M-MISSING",
            "009.9999",
            "m",
            MeasurementFormula::reb_vb_23003("1"),
        )
        .with_result(Decimal::new(1, 0)),
    );
    measurements.rows.push(MeasurementRow::formula(
        "M-NO-ORDINAL",
        "placeholder",
        "m",
        MeasurementFormula::reb_vb_23003("1"),
    ));
    measurements.rows[1].ordinal = None;
    let baseline = x86_baseline_document(vec![x86_item("001.0010", Decimal::new(1, 0), "m", None)]);

    let report = boq_core::x31::link_x31_to_x86_baseline(&measurements, &baseline);

    assert_eq!(report.rows.len(), 2);
    assert_eq!(report.rows[0].status, X31X86LinkStatus::MissingBaselineItem);
    assert_eq!(
        report.rows[1].status,
        X31X86LinkStatus::MissingMeasurementOrdinal
    );
    assert_eq!(report.findings.len(), 3);
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.code == "x31_canonical_quantity_missing_result")
    );
}

#[test]
fn test_linked_progress_report_is_deterministic() {
    let mut measurements = QuantityTakeoffDocument::new(source());
    measurements.rows.push(
        MeasurementRow::formula(
            "M-1",
            "001.0010",
            "m",
            MeasurementFormula::reb_vb_23003("1"),
        )
        .with_result(Decimal::new(1, 0)),
    );
    measurements.rows.push(
        MeasurementRow::formula(
            "M-2",
            "001.0020",
            "kg",
            MeasurementFormula::reb_vb_23003("2"),
        )
        .with_result(Decimal::new(2, 0)),
    );
    let baseline = x86_baseline_document(vec![
        x86_item(
            "001.0010",
            Decimal::new(1, 0),
            "m",
            Some(Decimal::new(5, 0)),
        ),
        x86_item(
            "001.0020",
            Decimal::new(2, 0),
            "t",
            Some(Decimal::new(7, 0)),
        ),
    ]);

    let first = boq_core::x31::link_x31_to_x86_baseline(&measurements, &baseline);
    let second = boq_core::x31::link_x31_to_x86_baseline(&measurements, &baseline);

    assert_eq!(first, second);
    assert_eq!(first.rows[0].ordinal.as_deref(), Some("001.0010"));
    assert_eq!(first.rows[1].status, X31X86LinkStatus::Mismatched);
    assert!(
        first
            .findings
            .iter()
            .any(|finding| finding.code == "x31_x86_unit_mismatch")
    );
}

fn x86_baseline_document(items: Vec<BoqNode>) -> GaebDocument {
    GaebDocument {
        source: SourceProvenance {
            source_uri: Some("gaeb/bvbs/gaeb_xml_3_3/ava/x86/synthetic.X86".to_owned()),
            source_format: GaebFormat::GaebXml,
            gaeb_version: Some("3.3".to_owned()),
            phase: Some(GaebPhase {
                code: "86".to_owned(),
                label: Some("Auftragserteilung".to_owned()),
            }),
            checksum: Some("sha256:x86-baseline".to_owned()),
            parser_version: boq_core::VERSION.to_owned(),
        },
        summary: GaebDocumentSummary {
            format: GaebFormat::GaebXml,
            version: Some("3.3".to_owned()),
            phase: Some(GaebPhase {
                code: "86".to_owned(),
                label: Some("Auftragserteilung".to_owned()),
            }),
            title: Some("X86 baseline".to_owned()),
            project_name: Some("X31/X86".to_owned()),
        },
        boq: Boq {
            title: "X86 baseline".to_owned(),
            nodes: vec![BoqNode {
                ordinal: "001".to_owned(),
                title: "section".to_owned(),
                kind: BoqNodeKind::Chapter,
                children: items,
                item: None,
                sort_order: 0,
                metadata: BTreeMap::new(),
            }],
            currency: Some("EUR".to_owned()),
            metadata: BTreeMap::new(),
        },
        capabilities: SupportCapabilities::parse_only(),
        support_status: SupportStatus::SupportedParseOnly,
        findings: Vec::new(),
        metadata: BTreeMap::new(),
    }
}

fn x86_item(ordinal: &str, quantity: Decimal, unit: &str, unit_price: Option<Decimal>) -> BoqNode {
    BoqNode {
        ordinal: ordinal.to_owned(),
        title: format!("Item {ordinal}"),
        kind: BoqNodeKind::Item,
        children: Vec::new(),
        item: Some(BoqItem {
            short_text: format!("Baseline {ordinal}"),
            long_text: Some(RichText::Plain("baseline".to_owned())),
            quantity,
            unit: unit.to_owned(),
            unit_price,
            total_price: unit_price.and_then(|price| price.checked_mul(quantity)),
            notes: None,
            metadata: BTreeMap::new(),
        }),
        sort_order: 0,
        metadata: BTreeMap::new(),
    }
}
