//! Obra-compatible import DTOs and mapping helpers.
//!
//! # Obra adapter DTO compatibility
//!
//! [`ObraImportDocument`] is the public compatibility boundary for handing a
//! parsed GAEB document to Obra-facing import code. It intentionally mirrors
//! stable DTO concepts (`boq`, `wbs_nodes`, `line_items`, `classifications`,
//! and `loss_report`) instead of importing any Obra backend module.
//!
//! Adapter conversion is capability-gated: callers should use
//! [`ObraImportDocument::try_from_gaeb`] and handle the
//! `obra_adapter_not_supported` finding when a document is parse-only,
//! future-track, or reference-only. Successful DTOs contain deterministic_key
//! values so repeated imports from the same source produce stable Obra-side
//! candidates; lossy or unsupported details are reported in `loss_report`.
//!
//! ```
//! let source = include_str!("../../tests/fixtures/synthetic/minimal_ava.x81");
//! let document = boq_core::gaeb_xml::parse_str(
//!     source,
//!     Some("gaeb/bvbs/gaeb_xml_3_3/ava/x81/minimal_ava.x81".to_owned()),
//! )?;
//! let import = boq_core::adapter::obra::ObraImportDocument::try_from_gaeb(&document)
//!     .expect("AVA X81 fixture path has adapter support");
//!
//! assert!(!import.boq.deterministic_key.is_empty());
//! assert!(import.loss_report.unsupported_fields.is_empty());
//! # Ok::<(), boq_core::error::ParseError>(())
//! ```
//!
//! ```
//! let bytes = include_bytes!("../../tests/fixtures/synthetic/minimal.d81");
//! let document = boq_core::gaeb90::parse_bytes(bytes, Some("minimal.d81".to_owned()))?;
//! let finding = boq_core::adapter::obra::ObraImportDocument::try_from_gaeb(&document)
//!     .expect_err("GAEB 90 D81 remains parse-only for the Obra adapter");
//!
//! assert_eq!(finding.code, "obra_adapter_not_supported");
//! # Ok::<(), boq_core::error::ParseError>(())
//! ```

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::ValidationFinding;
use crate::model::{
    BoqNode, BoqNodeKind, CatalogSystem, ClassificationSystem, GaebDocument, Metadata, RichText,
    SourceProvenance,
};

/// Obra import DTO root.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObraImportDocument {
    /// Source provenance.
    pub source: SourceProvenance,
    /// BoQ document payload.
    pub boq: ObraBoqDocument,
    /// WBS node candidates.
    pub wbs_nodes: Vec<ObraWbsNodeCandidate>,
    /// Line items.
    pub line_items: Vec<ObraLineItem>,
    /// Classification mappings.
    pub classifications: Vec<ObraClassification>,
    /// Loss report.
    pub loss_report: LossReport,
}

/// Obra BOQ document payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObraBoqDocument {
    /// Deterministic import key.
    pub deterministic_key: String,
    /// BoQ title.
    pub title: String,
    /// Obra status, usually `draft` for imports.
    pub status: ObraBoqStatus,
    /// Total amount when known.
    pub total_amount: Option<Decimal>,
    /// Currency when known.
    pub currency: Option<String>,
    /// Generated/imported timestamp when known.
    pub generated_at: Option<String>,
    /// Notes.
    pub notes: Option<String>,
    /// Metadata namespace for GAEB-specific data.
    pub metadata: Metadata,
}

/// Obra BOQ status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObraBoqStatus {
    /// Draft import status.
    Draft,
    /// Approved status.
    Approved,
    /// Issued status.
    Issued,
}

/// Candidate WBS node for Obra import.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObraWbsNodeCandidate {
    /// Deterministic import key.
    pub deterministic_key: String,
    /// Parent deterministic key.
    pub parent_key: Option<String>,
    /// Obra code, usually GAEB OZ.
    pub code: String,
    /// Node title.
    pub title: String,
    /// Hierarchy level.
    pub level: u8,
    /// Ltree-compatible path candidate.
    pub path: String,
    /// Sort order.
    pub sort_order: i32,
    /// Node type.
    pub node_type: ObraNodeType,
    /// Metadata.
    pub metadata: Metadata,
}

/// Obra node type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObraNodeType {
    /// Chapter node.
    Chapter,
    /// Item node.
    Item,
    /// Resource node.
    Resource,
    /// Assembly node.
    Assembly,
}

/// Obra line item DTO.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObraLineItem {
    /// Deterministic import key.
    pub deterministic_key: String,
    /// Referenced WBS node key.
    pub wbs_node_key: String,
    /// Short description.
    pub description: String,
    /// Rich long text when present.
    pub long_text: Option<RichText>,
    /// Quantity.
    pub quantity: Decimal,
    /// Unit.
    pub unit: String,
    /// Unit price.
    pub unit_price: Option<Decimal>,
    /// Total price.
    pub total_price: Option<Decimal>,
    /// Notes.
    pub notes: Option<String>,
    /// Metadata.
    pub metadata: Metadata,
}

/// Obra classification mapping DTO.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObraClassification {
    /// Referenced WBS node key.
    pub wbs_node_key: String,
    /// Classification system code.
    pub system_code: String,
    /// External classification code.
    pub external_code: String,
    /// External classification title.
    pub external_title: Option<String>,
    /// Unit.
    pub unit: Option<String>,
    /// Reference price.
    pub reference_price: Option<Decimal>,
}

/// Loss and provenance report for the adapter.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LossReport {
    /// Recoverable warnings.
    pub warnings: Vec<ValidationFinding>,
    /// Unsupported field paths.
    pub unsupported_fields: Vec<String>,
    /// Known lossy mappings.
    pub lossy_mappings: Vec<String>,
}

impl ObraImportDocument {
    /// Creates an Obra import document only when the source declares adapter support.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the parsed document is parse-only or reference-only.
    pub fn try_from_gaeb(document: &GaebDocument) -> Result<Self, ValidationFinding> {
        Self::from_gaeb(document)
    }

    /// Creates an Obra import document only when the source declares adapter support.
    ///
    /// # Errors
    ///
    /// Returns a validation finding when the parsed document is parse-only or reference-only.
    pub fn from_gaeb(document: &GaebDocument) -> Result<Self, ValidationFinding> {
        if document.capabilities.adapt_to_obra {
            Ok(Self::from_supported_gaeb(document))
        } else {
            Err(ValidationFinding::warning(
                "obra_adapter_not_supported",
                "document support capabilities do not allow Obra adapter conversion",
            ))
        }
    }

    fn from_supported_gaeb(document: &GaebDocument) -> Self {
        let seed = provenance_seed(&document.source);
        let boq_key = deterministic_key(&seed, "boq", &document.boq.title);
        let mut wbs_nodes = Vec::new();
        let mut line_items = Vec::new();
        let mut classifications = Vec::new();
        let mut annotation_warnings = Vec::new();

        for (index, node) in document.boq.nodes.iter().enumerate() {
            collect_node(
                node,
                None,
                1,
                i32::try_from(index).unwrap_or(i32::MAX),
                &seed,
                &mut wbs_nodes,
                &mut line_items,
                &mut classifications,
                &mut annotation_warnings,
            );
        }

        Self {
            source: document.source.clone(),
            boq: ObraBoqDocument {
                deterministic_key: boq_key,
                title: document.boq.title.clone(),
                status: ObraBoqStatus::Draft,
                total_amount: sum_total(&line_items),
                currency: document.boq.currency.clone(),
                generated_at: None,
                notes: None,
                metadata: document.boq.metadata.clone(),
            },
            wbs_nodes,
            line_items,
            classifications,
            loss_report: LossReport {
                // Archgate invariant: warnings: document.findings.clone() must remain
                // the base parser-finding propagation before annotation warnings are appended.
                warnings: document
                    .findings
                    .clone()
                    .into_iter()
                    .chain(annotation_warnings)
                    .collect(),
                unsupported_fields: Vec::new(),
                lossy_mappings: Vec::new(),
            },
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_node(
    node: &BoqNode,
    parent_key: Option<String>,
    level: u8,
    sibling_index: i32,
    seed: &str,
    wbs_nodes: &mut Vec<ObraWbsNodeCandidate>,
    line_items: &mut Vec<ObraLineItem>,
    classifications: &mut Vec<ObraClassification>,
    warnings: &mut Vec<ValidationFinding>,
) {
    let key = deterministic_key(seed, "wbs", &node.ordinal);
    let path = ltree_path(&node.ordinal);
    let node_type = match node.kind {
        BoqNodeKind::Chapter => ObraNodeType::Chapter,
        BoqNodeKind::Item => ObraNodeType::Item,
        BoqNodeKind::Resource => ObraNodeType::Resource,
        BoqNodeKind::Assembly => ObraNodeType::Assembly,
    };

    wbs_nodes.push(ObraWbsNodeCandidate {
        deterministic_key: key.clone(),
        parent_key,
        code: node.ordinal.clone(),
        title: node.title.clone(),
        level,
        path,
        sort_order: node.sort_order.max(sibling_index),
        node_type,
        metadata: node.metadata.clone(),
    });

    if let Some(item) = &node.item {
        line_items.push(ObraLineItem {
            deterministic_key: deterministic_key(seed, "line_item", &node.ordinal),
            wbs_node_key: key.clone(),
            description: item.short_text.clone(),
            long_text: item.long_text.clone(),
            quantity: item.quantity,
            unit: item.unit.clone(),
            unit_price: item.unit_price,
            total_price: item.total_price,
            notes: item.notes.clone(),
            metadata: item.metadata.clone(),
        });

        classifications.push(ObraClassification {
            wbs_node_key: key.clone(),
            system_code: "gaeb".to_owned(),
            external_code: node.ordinal.clone(),
            external_title: Some(node.title.clone()),
            unit: Some(item.unit.clone()),
            reference_price: item.unit_price,
        });

        match item.try_multi_standard() {
            Ok(annotations) => {
                for classification in annotations.classifications {
                    classifications.push(ObraClassification {
                        wbs_node_key: key.clone(),
                        system_code: classification_system_code(&classification.system),
                        external_code: classification.code,
                        external_title: classification.label,
                        unit: Some(item.unit.clone()),
                        reference_price: item.unit_price,
                    });
                }
                for catalog_reference in annotations.price_catalog_references {
                    classifications.push(ObraClassification {
                        wbs_node_key: key.clone(),
                        system_code: catalog_system_code(&catalog_reference.system),
                        external_code: catalog_reference.code,
                        external_title: catalog_reference.label,
                        unit: Some(item.unit.clone()),
                        reference_price: catalog_reference.unit_price,
                    });
                }
            }
            Err(finding) => warnings.push(finding),
        }
    }

    for (index, child) in node.children.iter().enumerate() {
        collect_node(
            child,
            Some(key.clone()),
            level.saturating_add(1),
            i32::try_from(index).unwrap_or(i32::MAX),
            seed,
            wbs_nodes,
            line_items,
            classifications,
            warnings,
        );
    }
}

fn classification_system_code(system: &ClassificationSystem) -> String {
    match system {
        ClassificationSystem::Gaeb => "gaeb".to_owned(),
        ClassificationSystem::Din276 => "din276".to_owned(),
        ClassificationSystem::CsiMasterFormat => "csi_masterformat".to_owned(),
        ClassificationSystem::Uniclass => "uniclass".to_owned(),
        ClassificationSystem::NlSfb => "nlsfb".to_owned(),
        ClassificationSystem::Sinapi => "sinapi".to_owned(),
        ClassificationSystem::Stabu => "stabu".to_owned(),
        ClassificationSystem::Dqe => "dqe".to_owned(),
        ClassificationSystem::Custom(value) => value.clone(),
    }
}

fn catalog_system_code(system: &CatalogSystem) -> String {
    match system {
        CatalogSystem::Sinapi => "sinapi".to_owned(),
        CatalogSystem::Prezzario => "prezzario".to_owned(),
        CatalogSystem::CatalogoConceptos => "catalogo_conceptos".to_owned(),
        CatalogSystem::CuadroPrecios => "cuadro_precios".to_owned(),
        CatalogSystem::Stabu => "stabu".to_owned(),
        CatalogSystem::Custom(value) => value.clone(),
    }
}

fn provenance_seed(source: &SourceProvenance) -> String {
    source.checksum.clone().unwrap_or_else(|| {
        format!(
            "{:?}:{}:{}",
            source.source_format,
            source.gaeb_version.as_deref().unwrap_or("unknown"),
            source
                .phase
                .as_ref()
                .map_or("unknown", |phase| phase.code.as_str())
        )
    })
}

fn deterministic_key(seed: &str, kind: &str, value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher.update(b":");
    hasher.update(kind.as_bytes());
    hasher.update(b":");
    hasher.update(value.as_bytes());
    let digest = hasher.finalize();
    format!("{kind}_{digest:.12x}")
}

fn ltree_path(ordinal: &str) -> String {
    let mut parts = Vec::new();
    let mut current = String::new();

    for ch in ordinal.chars() {
        if ch.is_ascii_alphanumeric() {
            current.push(ch.to_ascii_lowercase());
        } else if !current.is_empty() {
            parts.push(std::mem::take(&mut current));
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    if parts.is_empty() {
        "unknown".to_owned()
    } else {
        parts.join(".")
    }
}

fn sum_total(items: &[ObraLineItem]) -> Option<Decimal> {
    items
        .iter()
        .map(|item| item.total_price)
        .try_fold(Decimal::ZERO, |acc, value| value.map(|amount| acc + amount))
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::collections::BTreeMap;

    use rust_decimal::Decimal;

    use super::*;
    use crate::model::{
        Boq, BoqItem, BoqNode, BoqNodeKind, GaebDocument, GaebDocumentSummary, GaebFormat,
        GaebPhase,
    };
    use crate::support::{SupportCapabilities, SupportStatus};

    #[test]
    fn maps_gaeb_tree_to_obra_import_document() {
        let document = sample_document();
        let adapted =
            ObraImportDocument::from_gaeb(&document).expect("supported document should adapt");

        assert_eq!(adapted.boq.title, "Sample AVA BoQ");
        assert_eq!(adapted.wbs_nodes.len(), 2);
        assert_eq!(adapted.line_items.len(), 1);
        assert_eq!(adapted.classifications.len(), 1);
        assert_eq!(adapted.wbs_nodes[0].path, "001");
        assert_eq!(adapted.wbs_nodes[1].path, "001.0010");
        assert_eq!(adapted.line_items[0].description, "Concrete works");
        assert_eq!(
            adapted.line_items[0].total_price,
            Some(Decimal::new(25000, 2))
        );
        assert_eq!(adapted.boq.total_amount, Some(Decimal::new(25000, 2)));
    }

    #[test]
    fn try_from_gaeb_rejects_parse_only_documents() {
        let mut document = sample_document();
        document.capabilities = SupportCapabilities::parse_only();
        document.support_status = SupportStatus::SupportedParseOnly;

        let error = ObraImportDocument::try_from_gaeb(&document)
            .expect_err("parse-only document must not adapt to Obra");
        assert_eq!(error.code, "obra_adapter_not_supported");
    }

    #[test]
    fn try_from_gaeb_accepts_supported_documents() {
        let document = sample_document();
        let adapted = ObraImportDocument::try_from_gaeb(&document)
            .expect("supported document should adapt to Obra");

        assert_eq!(adapted.boq.title, "Sample AVA BoQ");
        assert_eq!(adapted.line_items.len(), 1);
    }

    #[test]
    fn maps_non_item_node_kinds_and_fallback_seed() {
        let mut document = sample_document();
        document.source.checksum = None;
        document.source.gaeb_version = None;
        document.source.phase = None;
        document.boq.nodes.push(BoqNode {
            ordinal: "@@@".to_owned(),
            title: "Resource bucket".to_owned(),
            kind: BoqNodeKind::Resource,
            children: Vec::new(),
            item: None,
            sort_order: 1,
            metadata: BTreeMap::new(),
        });
        document.boq.nodes.push(BoqNode {
            ordinal: "ASM-01".to_owned(),
            title: "Assembly bucket".to_owned(),
            kind: BoqNodeKind::Assembly,
            children: Vec::new(),
            item: None,
            sort_order: 2,
            metadata: BTreeMap::new(),
        });

        let adapted =
            ObraImportDocument::from_gaeb(&document).expect("supported document should adapt");
        assert!(
            adapted
                .wbs_nodes
                .iter()
                .any(|node| node.node_type == ObraNodeType::Resource && node.path == "unknown")
        );
        assert!(
            adapted
                .wbs_nodes
                .iter()
                .any(|node| node.node_type == ObraNodeType::Assembly)
        );
        assert!(!adapted.boq.deterministic_key.is_empty());
    }

    #[test]
    fn deterministic_keys_are_stable_for_same_source() {
        let first =
            ObraImportDocument::from_gaeb(&sample_document()).expect("first document adapts");
        let second =
            ObraImportDocument::from_gaeb(&sample_document()).expect("second document adapts");

        assert_eq!(first.boq.deterministic_key, second.boq.deterministic_key);
        assert_eq!(
            first.wbs_nodes[1].deterministic_key,
            second.wbs_nodes[1].deterministic_key
        );
        assert_eq!(
            first.line_items[0].deterministic_key,
            second.line_items[0].deterministic_key
        );
    }

    fn sample_document() -> GaebDocument {
        let item = BoqItem {
            short_text: "Concrete works".to_owned(),
            long_text: Some(RichText::Plain("Pour concrete foundation".to_owned())),
            quantity: Decimal::new(100, 0),
            unit: "m3".to_owned(),
            unit_price: Some(Decimal::new(250, 2)),
            total_price: Some(Decimal::new(25000, 2)),
            notes: None,
            metadata: BTreeMap::new(),
        };
        let child = BoqNode {
            ordinal: "001.0010".to_owned(),
            title: "Concrete works".to_owned(),
            kind: BoqNodeKind::Item,
            children: Vec::new(),
            item: Some(item),
            sort_order: 0,
            metadata: BTreeMap::new(),
        };
        let root = BoqNode {
            ordinal: "001".to_owned(),
            title: "Foundations".to_owned(),
            kind: BoqNodeKind::Chapter,
            children: vec![child],
            item: None,
            sort_order: 0,
            metadata: BTreeMap::new(),
        };

        GaebDocument {
            source: SourceProvenance {
                source_uri: Some("fixture.x81".to_owned()),
                source_format: GaebFormat::GaebXml,
                gaeb_version: Some("3.3".to_owned()),
                phase: Some(GaebPhase {
                    code: "81".to_owned(),
                    label: Some("Leistungsbeschreibung".to_owned()),
                }),
                checksum: Some("abc123".to_owned()),
                parser_version: "0.1.0".to_owned(),
            },
            summary: GaebDocumentSummary {
                format: GaebFormat::GaebXml,
                version: Some("3.3".to_owned()),
                phase: None,
                title: Some("Sample AVA BoQ".to_owned()),
                project_name: Some("Sample".to_owned()),
            },
            boq: Boq {
                title: "Sample AVA BoQ".to_owned(),
                nodes: vec![root],
                currency: Some("EUR".to_owned()),
                metadata: BTreeMap::new(),
            },
            capabilities: SupportCapabilities::supported_import(),
            support_status: SupportStatus::Supported,
            findings: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }
}
