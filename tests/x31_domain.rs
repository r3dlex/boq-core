#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::error::ValidationFinding;
use boq_core::model::{GaebFormat, SourceProvenance};
use boq_core::x31::{
    BaselineKind, MeasurementAttachment, MeasurementAttachmentKind, MeasurementBaselineLink,
    MeasurementFormula, MeasurementReference, MeasurementReferenceKind, MeasurementRow,
    PhysicalProgress, QuantityTakeoffDocument, RebFormulaSystem,
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
