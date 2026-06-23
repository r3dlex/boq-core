#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::error::ValidationFinding;
use boq_core::model::{
    BoqItem, CatalogSystem, ClassificationReference, ClassificationSystem, GaebFormat, GaebPhase,
    MultiStandardAnnotations, PriceCatalogReference, ProgressReference, QuantityReference,
    QuantityReferenceKind, ReferenceConfidence, SourceProvenance,
};
use boq_core::support::SupportStatus;
use rust_decimal::Decimal;
use serde_json::json;

#[test]
fn parsed_items_start_with_empty_multi_standard_annotations_without_support_promotion() {
    let document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal GAEB 90 D81 should parse");

    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);

    let items = collect_items(&document.boq.nodes);
    assert!(
        !items.is_empty(),
        "fixture should contain at least one item"
    );
    assert!(
        items.iter().all(|item| item
            .try_multi_standard()
            .expect("annotations should decode")
            .is_empty()),
        "parsed GAEB items must not gain implicit multi-standard support evidence"
    );

    let encoded = serde_json::to_value(items[0]).expect("item should serialize");
    assert!(
        encoded.get("boq_core.multi_standard").is_none() && encoded.get("multi_standard").is_none(),
        "empty annotations should not churn serialized BoQ item output"
    );
}

#[test]
fn annotations_host_classification_price_quantity_progress_provenance_and_loss() {
    let annotations = sample_annotations();

    assert!(!annotations.is_empty());

    let encoded = serde_json::to_string(&annotations).expect("annotations should serialize");
    let decoded: MultiStandardAnnotations =
        serde_json::from_str(&encoded).expect("annotations should deserialize");

    assert_eq!(decoded, annotations);
    assert_eq!(
        decoded.classifications[0].system,
        ClassificationSystem::Din276
    );
    assert_eq!(
        decoded.price_catalog_references[0].system,
        CatalogSystem::Sinapi
    );
    assert_eq!(
        decoded.quantity_references[0].kind,
        QuantityReferenceKind::GaebX31
    );
    assert_eq!(decoded.progress_references[0].reference, "X86:PROGRESS-7");
    assert_eq!(decoded.loss_findings[0].code, "classification_unverified");

    let mut item = BoqItem {
        short_text: "annotated item".to_owned(),
        long_text: None,
        quantity: Decimal::new(1, 0),
        unit: "m".to_owned(),
        unit_price: None,
        total_price: None,
        notes: None,
        metadata: BTreeMap::new(),
    };

    item.set_multi_standard(annotations.clone())
        .expect("annotations should encode");
    assert_eq!(
        item.try_multi_standard()
            .expect("annotations should decode"),
        annotations
    );
    assert!(item.metadata.contains_key("boq_core.multi_standard"));

    item.set_multi_standard(MultiStandardAnnotations::default())
        .expect("empty annotations should clear");
    assert!(
        item.try_multi_standard()
            .expect("annotations should decode")
            .is_empty()
    );
    assert!(!item.metadata.contains_key("boq_core.multi_standard"));
}

#[test]
fn legacy_boq_item_json_defaults_to_empty_annotations() {
    let legacy = json!({
        "short_text": "legacy item",
        "long_text": null,
        "quantity": "1.00",
        "unit": "m",
        "unit_price": null,
        "total_price": null,
        "notes": null,
        "metadata": {}
    });

    let item: BoqItem = serde_json::from_value(legacy).expect("legacy item should deserialize");

    assert!(
        item.try_multi_standard()
            .expect("annotations should decode")
            .is_empty()
    );
}

#[test]
fn source_compatible_boq_item_literal_does_not_require_annotations_field() {
    let item = BoqItem {
        short_text: "source compatible".to_owned(),
        long_text: None,
        quantity: Decimal::new(1, 0),
        unit: "m".to_owned(),
        unit_price: None,
        total_price: None,
        notes: None,
        metadata: BTreeMap::new(),
    };

    assert!(
        item.try_multi_standard()
            .expect("annotations should decode")
            .is_empty()
    );
}

#[test]
fn malformed_annotation_metadata_returns_loss_finding() {
    let mut metadata = BTreeMap::new();
    metadata.insert("boq_core.multi_standard".to_owned(), json!("not-an-object"));
    let item = BoqItem {
        short_text: "malformed annotation".to_owned(),
        long_text: None,
        quantity: Decimal::new(1, 0),
        unit: "m".to_owned(),
        unit_price: None,
        total_price: None,
        notes: None,
        metadata,
    };

    let finding = item
        .try_multi_standard()
        .expect_err("malformed annotation metadata must not be hidden as empty");

    assert_eq!(finding.code, "multi_standard_metadata_malformed");
    assert_eq!(finding.location.as_deref(), Some("boq_core.multi_standard"));
}

fn collect_items(nodes: &[boq_core::model::BoqNode]) -> Vec<&BoqItem> {
    let mut items = Vec::new();
    for node in nodes {
        if let Some(item) = &node.item {
            items.push(item);
        }
        items.extend(collect_items(&node.children));
    }
    items
}
fn sample_annotations() -> MultiStandardAnnotations {
    let provenance = SourceProvenance {
        source_uri: Some("fixtures/phase-13/multi-standard-source.xml".to_owned()),
        source_format: GaebFormat::GaebXml,
        gaeb_version: Some("3.3".to_owned()),
        phase: Some(GaebPhase {
            code: "86".to_owned(),
            label: Some("Bauabrechnung".to_owned()),
        }),
        checksum: Some("sha256:phase13".to_owned()),
        parser_version: boq_core::VERSION.to_owned(),
    };

    let mut classification_metadata = BTreeMap::new();
    classification_metadata.insert("mapping_basis".to_owned(), json!("fixture-derived"));

    MultiStandardAnnotations {
        classifications: vec![ClassificationReference {
            system: ClassificationSystem::Din276,
            code: "300".to_owned(),
            label: Some("Bauwerk - Baukonstruktionen".to_owned()),
            confidence: ReferenceConfidence::Derived,
            source: Some(provenance.clone()),
            metadata: classification_metadata,
        }],
        price_catalog_references: vec![PriceCatalogReference {
            system: CatalogSystem::Sinapi,
            code: "SINAPI-0001".to_owned(),
            label: Some("Concrete crew reference".to_owned()),
            unit_price: Some(Decimal::new(12550, 2)),
            currency: Some("BRL".to_owned()),
            source: Some(provenance.clone()),
            metadata: BTreeMap::new(),
        }],
        quantity_references: vec![QuantityReference {
            kind: QuantityReferenceKind::GaebX31,
            reference: "X31:ROW-42".to_owned(),
            quantity: Some(Decimal::new(1500, 2)),
            unit: Some("m3".to_owned()),
            source: Some(provenance.clone()),
            findings: vec![ValidationFinding::warning(
                "quantity_reference_rounded",
                "quantity reference was rounded by the source standard",
            )],
            metadata: BTreeMap::new(),
        }],
        progress_references: vec![ProgressReference {
            reference: "X86:PROGRESS-7".to_owned(),
            percent_complete: Some(Decimal::new(3750, 2)),
            quantity_complete: Some(Decimal::new(5625, 3)),
            unit: Some("m3".to_owned()),
            source: Some(provenance.clone()),
            findings: Vec::new(),
            metadata: BTreeMap::new(),
        }],
        provenance: vec![provenance],
        loss_findings: vec![ValidationFinding::warning(
            "classification_unverified",
            "classification is carried as evidence, not supported import behavior",
        )],
        metadata: BTreeMap::new(),
    }
}
