#![allow(missing_docs, clippy::expect_used)]

use std::collections::BTreeMap;

use boq_core::adapter::obra::ObraImportDocument;
use boq_core::catalogo::{CatalogoCuadroTable, apply_catalogo_overlay};
use boq_core::model::{
    Boq, BoqItem, BoqNode, BoqNodeKind, CatalogSystem, GaebDocument, GaebDocumentSummary,
    GaebFormat, MultiStandardAnnotations, PriceCatalogReference, RichText, RichTextFragment,
    SourceProvenance,
};
use boq_core::support::{SupportCapabilities, SupportStatus};
use rust_decimal::Decimal;
use serde_json::json;

#[test]
fn catalogo_fixture_attaches_concept_and_cuadro_without_support_promotion() {
    let mut document = supported_ava_document();
    let original_status = document.support_status;
    let table = catalogo_table();

    let findings = apply_catalogo_overlay(&mut document, &table);

    assert_eq!(document.support_status, original_status);
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "catalogo_cuadro_price_factor_preserved_not_applied")
    );
    assert!(
        document
            .findings
            .iter()
            .any(|finding| finding.code == "catalogo_cuadro_price_factor_preserved_not_applied")
    );

    let annotations = first_item(&document)
        .try_multi_standard()
        .expect("Catálogo/Cuadro annotations should decode");
    assert_eq!(annotations.price_catalog_references.len(), 2);

    let concept = annotations
        .price_catalog_references
        .iter()
        .find(|reference| reference.system == CatalogSystem::CatalogoConceptos)
        .expect("concept catalog reference should exist");
    assert_eq!(concept.code, "CONCEPTO-SYN-001");
    assert_eq!(
        concept.label.as_deref(),
        Some("Synthetic Spanish concrete concept")
    );
    assert_eq!(concept.metadata.get("market_scope"), Some(&json!("ES/MX")));

    let price = annotations
        .price_catalog_references
        .iter()
        .find(|reference| reference.system == CatalogSystem::CuadroPrecios)
        .expect("cuadro de precios reference should exist");
    assert_eq!(price.code, "CUADRO-SYN-001");
    assert_eq!(price.unit_price, Some(Decimal::new(12645, 2)));
    assert_eq!(price.currency.as_deref(), Some("EUR"));
    assert_eq!(
        price.metadata.get("concept_code"),
        Some(&json!("CONCEPTO-SYN-001"))
    );
    assert_eq!(annotations.loss_findings.len(), 1);
    assert_eq!(annotations.provenance.len(), 1);
}

#[test]
fn obra_adapter_exports_catalogo_and_cuadro_when_adapter_is_already_supported() {
    let mut document = supported_ava_document();
    let findings = apply_catalogo_overlay(&mut document, &catalogo_table());
    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "catalogo_cuadro_price_factor_preserved_not_applied")
    );

    let import = ObraImportDocument::try_from_gaeb(&document)
        .expect("supported AVA fixture should still adapt");

    assert!(import.classifications.iter().any(|classification| {
        classification.system_code == "catalogo_conceptos"
            && classification.external_code == "CONCEPTO-SYN-001"
            && classification.external_title.as_deref()
                == Some("Synthetic Spanish concrete concept")
    }));
    assert!(import.classifications.iter().any(|classification| {
        classification.system_code == "cuadro_precios"
            && classification.external_code == "CUADRO-SYN-001"
            && classification.reference_price == Some(Decimal::new(12645, 2))
    }));
    assert!(
        import
            .loss_report
            .warnings
            .iter()
            .any(|finding| finding.code == "catalogo_cuadro_price_factor_preserved_not_applied")
    );
}

#[test]
fn parse_only_document_remains_not_adapter_supported_after_catalogo_overlay() {
    let mut document = boq_core::gaeb90::parse_bytes(
        include_bytes!("fixtures/synthetic/minimal.d81"),
        Some("minimal.d81".to_owned()),
    )
    .expect("minimal D81 should parse");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);

    let _findings = apply_catalogo_overlay(&mut document, &catalogo_table());

    let error = ObraImportDocument::try_from_gaeb(&document)
        .expect_err("Catálogo/Cuadro overlay must not promote parse-only adapter support");
    assert_eq!(error.code, "obra_adapter_not_supported");
    assert_eq!(document.support_status, SupportStatus::SupportedParseOnly);
}

#[test]
fn invalid_catalogo_fixture_variants_are_reported_explicitly() {
    let cases = [
        ("not json", "catalogo_cuadro_invalid_json"),
        (
            r#"{"source_uri":"fixture","items":[]}"#,
            "catalogo_cuadro_empty",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":" ","concept_code":"CONCEPTO-SYN-001","concept_label":"Concept","cuadro_code":"CUADRO-SYN-001","price_label":"Price","unit_price":"126.45","currency":"EUR","unit":"m3","market_scope":"ES/MX"}]}"#,
            "catalogo_cuadro_empty_match",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","concept_code":"BAD-001","concept_label":"Concept","cuadro_code":"CUADRO-SYN-001","price_label":"Price","unit_price":"126.45","currency":"EUR","unit":"m3","market_scope":"ES/MX"}]}"#,
            "catalogo_cuadro_invalid_concept_code",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","concept_code":"CONCEPTO-SYN-001","concept_label":" ","cuadro_code":"CUADRO-SYN-001","price_label":"Price","unit_price":"126.45","currency":"EUR","unit":"m3","market_scope":"ES/MX"}]}"#,
            "catalogo_cuadro_empty_concept_label",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","concept_code":"CONCEPTO-SYN-001","concept_label":"Concept","cuadro_code":"BAD-001","price_label":"Price","unit_price":"126.45","currency":"EUR","unit":"m3","market_scope":"ES/MX"}]}"#,
            "catalogo_cuadro_invalid_cuadro_code",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","concept_code":"CONCEPTO-SYN-001","concept_label":"Concept","cuadro_code":"CUADRO-SYN-001","price_label":" ","unit_price":"126.45","currency":"EUR","unit":"m3","market_scope":"ES/MX"}]}"#,
            "catalogo_cuadro_empty_price_label",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","concept_code":"CONCEPTO-SYN-001","concept_label":"Concept","cuadro_code":"CUADRO-SYN-001","price_label":"Price","unit_price":"abc","currency":"EUR","unit":"m3","market_scope":"ES/MX"}]}"#,
            "catalogo_cuadro_invalid_unit_price",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","concept_code":"CONCEPTO-SYN-001","concept_label":"Concept","cuadro_code":"CUADRO-SYN-001","price_label":"Price","unit_price":"126.45","currency":"eur","unit":"m3","market_scope":"ES/MX"}]}"#,
            "catalogo_cuadro_invalid_currency",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","concept_code":"CONCEPTO-SYN-001","concept_label":"Concept","cuadro_code":"CUADRO-SYN-001","price_label":"Price","unit_price":"126.45","currency":"EUR","unit":" ","market_scope":"ES/MX"}]}"#,
            "catalogo_cuadro_empty_unit",
        ),
        (
            r#"{"source_uri":"fixture","items":[{"match_text":"Concrete","concept_code":"CONCEPTO-SYN-001","concept_label":"Concept","cuadro_code":"CUADRO-SYN-001","price_label":"Price","unit_price":"126.45","currency":"EUR","unit":"m3","market_scope":" "}]}"#,
            "catalogo_cuadro_empty_market_scope",
        ),
    ];

    for (input, expected_code) in cases {
        let finding = CatalogoCuadroTable::from_json_str(input)
            .expect_err("fixture should reject invalid Catálogo/Cuadro data");
        assert_eq!(finding.code, expected_code);
    }
}

#[test]
fn overlay_completes_missing_cuadro_when_concept_reference_already_exists() {
    let mut document = supported_ava_document();
    let mut annotations = MultiStandardAnnotations::default();
    annotations
        .price_catalog_references
        .push(PriceCatalogReference {
            system: CatalogSystem::CatalogoConceptos,
            code: "CONCEPTO-SYN-001".to_owned(),
            label: Some("Pre-existing concept".to_owned()),
            unit_price: None,
            currency: None,
            source: None,
            metadata: BTreeMap::default(),
        });
    first_item_mut(&mut document)
        .set_multi_standard(annotations)
        .expect("pre-existing concept annotation should encode");

    let findings = apply_catalogo_overlay(&mut document, &catalogo_table());

    assert_eq!(
        findings
            .iter()
            .filter(|finding| finding.code == "catalogo_cuadro_price_factor_preserved_not_applied")
            .count(),
        1
    );
    let annotations = first_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations
            .price_catalog_references
            .iter()
            .filter(|reference| reference.system == CatalogSystem::CatalogoConceptos)
            .count(),
        1
    );
    assert!(
        annotations
            .price_catalog_references
            .iter()
            .any(|reference| {
                reference.system == CatalogSystem::CuadroPrecios
                    && reference.code == "CUADRO-SYN-001"
            })
    );
    assert_eq!(annotations.loss_findings.len(), 1);
    assert_eq!(annotations.provenance.len(), 1);
}

#[test]
fn overlay_completes_missing_concept_when_cuadro_reference_already_exists() {
    let mut document = supported_ava_document();
    let mut annotations = MultiStandardAnnotations::default();
    annotations
        .price_catalog_references
        .push(PriceCatalogReference {
            system: CatalogSystem::CuadroPrecios,
            code: "CUADRO-SYN-001".to_owned(),
            label: Some("Pre-existing price table".to_owned()),
            unit_price: Some(Decimal::new(12645, 2)),
            currency: Some("EUR".to_owned()),
            source: None,
            metadata: BTreeMap::default(),
        });
    first_item_mut(&mut document)
        .set_multi_standard(annotations)
        .expect("pre-existing price annotation should encode");

    let findings = apply_catalogo_overlay(&mut document, &catalogo_table());

    assert_eq!(
        findings
            .iter()
            .filter(|finding| finding.code == "catalogo_cuadro_price_factor_preserved_not_applied")
            .count(),
        1
    );
    let annotations = first_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert!(
        annotations
            .price_catalog_references
            .iter()
            .any(|reference| {
                reference.system == CatalogSystem::CatalogoConceptos
                    && reference.code == "CONCEPTO-SYN-001"
            })
    );
    assert_eq!(
        annotations
            .price_catalog_references
            .iter()
            .filter(|reference| reference.system == CatalogSystem::CuadroPrecios)
            .count(),
        1
    );
    assert_eq!(annotations.loss_findings.len(), 1);
    assert_eq!(annotations.provenance.len(), 1);
}

#[test]
fn malformed_existing_annotations_are_reported_not_hidden() {
    let mut document = supported_ava_document();
    first_item_mut(&mut document)
        .metadata
        .insert("boq_core.multi_standard".to_owned(), json!("malformed"));

    let findings = apply_catalogo_overlay(&mut document, &catalogo_table());

    assert!(
        findings
            .iter()
            .any(|finding| finding.code == "multi_standard_metadata_malformed")
    );
}

#[test]
fn overlay_matches_nested_rich_text_without_duplicate_references() {
    let mut document = document_with_item(BoqItem {
        short_text: "No match in short text".to_owned(),
        long_text: Some(RichText::Mixed(vec![
            RichTextFragment::Text("Intro".to_owned()),
            RichTextFragment::Table("Concrete table cell".to_owned()),
            RichTextFragment::Image {
                id: "img-1".to_owned(),
                description: Some("Concrete image description".to_owned()),
            },
            RichTextFragment::Unknown("Concrete unknown fragment".to_owned()),
        ])),
        quantity: Decimal::ONE,
        unit: "m3".to_owned(),
        unit_price: None,
        total_price: None,
        notes: None,
        metadata: BTreeMap::default(),
    });

    let _ = apply_catalogo_overlay(&mut document, &catalogo_table());
    let _ = apply_catalogo_overlay(&mut document, &catalogo_table());

    let annotations = nested_item(&document)
        .try_multi_standard()
        .expect("annotation metadata should decode");
    assert_eq!(
        annotations
            .price_catalog_references
            .iter()
            .filter(|reference| reference.system == CatalogSystem::CatalogoConceptos)
            .count(),
        1
    );
    assert_eq!(
        annotations
            .price_catalog_references
            .iter()
            .filter(|reference| reference.system == CatalogSystem::CuadroPrecios)
            .count(),
        1
    );
    assert_eq!(annotations.provenance.len(), 1);
}

fn catalogo_table() -> CatalogoCuadroTable {
    CatalogoCuadroTable::from_json_str(include_str!("fixtures/synthetic/catalogo_cuadro.json"))
        .expect("synthetic Catálogo/Cuadro fixture should load")
}

fn supported_ava_document() -> GaebDocument {
    boq_core::gaeb_xml::parse_str(
        include_str!("fixtures/synthetic/minimal_ava.x81"),
        Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
    )
    .expect("supported AVA fixture should parse")
}

fn document_with_item(item: BoqItem) -> GaebDocument {
    GaebDocument {
        source: SourceProvenance {
            source_uri: Some("synthetic/catalogo-cuadro-rich-text.x81".to_owned()),
            source_format: GaebFormat::GaebXml,
            gaeb_version: Some("3.3".to_owned()),
            phase: None,
            checksum: None,
            parser_version: boq_core::VERSION.to_owned(),
        },
        summary: GaebDocumentSummary {
            format: GaebFormat::GaebXml,
            version: Some("3.3".to_owned()),
            phase: None,
            title: Some("Synthetic Catálogo/Cuadro".to_owned()),
            project_name: None,
        },
        boq: Boq {
            title: "Synthetic Catálogo/Cuadro".to_owned(),
            nodes: vec![BoqNode {
                ordinal: "1".to_owned(),
                title: "Chapter".to_owned(),
                kind: BoqNodeKind::Chapter,
                children: vec![BoqNode {
                    ordinal: "1.1".to_owned(),
                    title: "Nested item".to_owned(),
                    kind: BoqNodeKind::Item,
                    children: Vec::new(),
                    item: Some(item),
                    sort_order: 0,
                    metadata: BTreeMap::default(),
                }],
                item: None,
                sort_order: 0,
                metadata: BTreeMap::default(),
            }],
            currency: Some("EUR".to_owned()),
            metadata: BTreeMap::default(),
        },
        capabilities: SupportCapabilities::supported_import(),
        support_status: SupportStatus::Supported,
        findings: Vec::new(),
        metadata: BTreeMap::default(),
    }
}

fn first_item(document: &GaebDocument) -> &boq_core::model::BoqItem {
    document.boq.nodes[0]
        .item
        .as_ref()
        .expect("minimal fixture should have an item")
}

fn first_item_mut(document: &mut GaebDocument) -> &mut boq_core::model::BoqItem {
    document.boq.nodes[0]
        .item
        .as_mut()
        .expect("minimal fixture should have an item")
}

fn nested_item(document: &GaebDocument) -> &boq_core::model::BoqItem {
    document.boq.nodes[0].children[0]
        .item
        .as_ref()
        .expect("synthetic nested document should have an item")
}
