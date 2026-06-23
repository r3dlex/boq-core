//! Loss-aware GAEB domain model.
//!
//! The canonical BoQ model can carry multi-standard annotations for
//! classifications, price/catalog references, quantity evidence, progress
//! evidence, provenance, and recoverable loss findings. These annotations are
//! evidence containers only: adding them to an item does not promote support
//! status or imply adapter, export, roundtrip, billing, production, or
//! certification readiness.

use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::ValidationFinding;
use crate::support::{SupportCapabilities, SupportStatus};

/// A JSON-like metadata map that keeps unmapped GAEB fields available.
pub type Metadata = BTreeMap<String, serde_json::Value>;

/// Supported GAEB format families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GaebFormat {
    /// GAEB 90 fixed-width line format.
    Gaeb90,
    /// GAEB 2000 text/tag format.
    Gaeb2000,
    /// GAEB DA XML format.
    GaebXml,
}

/// GAEB transaction phase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GaebPhase {
    /// Numeric phase code, for example `81`, `83`, `84`, or `86`.
    pub code: String,
    /// Human-readable phase label when known.
    pub label: Option<String>,
}

/// Source provenance for a parsed document or derived adapter DTO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceProvenance {
    /// Optional original source URI or local path.
    pub source_uri: Option<String>,
    /// Source format family.
    pub source_format: GaebFormat,
    /// Optional GAEB version, for example `3.3` or `GAEB 90`.
    pub gaeb_version: Option<String>,
    /// Optional GAEB phase.
    pub phase: Option<GaebPhase>,
    /// Optional source checksum.
    pub checksum: Option<String>,
    /// Parser crate version.
    pub parser_version: String,
}

/// Document-level GAEB summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GaebDocumentSummary {
    /// Source format.
    pub format: GaebFormat,
    /// Version label when present.
    pub version: Option<String>,
    /// Transaction phase.
    pub phase: Option<GaebPhase>,
    /// Document title.
    pub title: Option<String>,
    /// Project name.
    pub project_name: Option<String>,
}

/// A complete parsed GAEB document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GaebDocument {
    /// Source provenance.
    pub source: SourceProvenance,
    /// Document summary.
    pub summary: GaebDocumentSummary,
    /// Bill of quantities tree.
    pub boq: Boq,
    /// Direction-aware support capabilities used for honest reporting.
    pub capabilities: SupportCapabilities,
    /// Fixture or phase support status.
    pub support_status: SupportStatus,
    /// Recoverable findings.
    pub findings: Vec<ValidationFinding>,
    /// Unmapped document-level metadata.
    pub metadata: Metadata,
}

/// A bill of quantities tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Boq {
    /// BoQ title.
    pub title: String,
    /// Top-level hierarchy nodes.
    pub nodes: Vec<BoqNode>,
    /// Optional currency.
    pub currency: Option<String>,
    /// BoQ-level metadata.
    pub metadata: Metadata,
}

/// A hierarchy node or item node inside a BoQ.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoqNode {
    /// GAEB ordinal number or section code.
    pub ordinal: String,
    /// Display title.
    pub title: String,
    /// Node kind.
    pub kind: BoqNodeKind,
    /// Child hierarchy nodes.
    pub children: Vec<BoqNode>,
    /// Optional line item payload.
    pub item: Option<BoqItem>,
    /// Source order among siblings.
    pub sort_order: i32,
    /// Node metadata.
    pub metadata: Metadata,
}

/// BoQ node kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoqNodeKind {
    /// Chapter/section node.
    Chapter,
    /// Item/position node.
    Item,
    /// Resource node.
    Resource,
    /// Assembly node.
    Assembly,
}

/// Optional multi-standard evidence attached to a canonical BoQ item.
///
/// This structure is intentionally independent from [`SupportStatus`]. It can
/// preserve evidence from future or reference standards, but by itself it does
/// not promote support or declare Obra import compatibility.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MultiStandardAnnotations {
    /// Classification references such as DIN 276, CSI MasterFormat, or custom codes.
    pub classifications: Vec<ClassificationReference>,
    /// Price or catalog references such as SINAPI, STABU, or local catalogs.
    pub price_catalog_references: Vec<PriceCatalogReference>,
    /// Quantity takeoff or measurement references linked to the item.
    pub quantity_references: Vec<QuantityReference>,
    /// Progress or billing-progress references linked to the item.
    pub progress_references: Vec<ProgressReference>,
    /// Provenance records for external evidence sources used by annotations.
    pub provenance: Vec<SourceProvenance>,
    /// Recoverable loss or mapping findings for the annotation set.
    pub loss_findings: Vec<ValidationFinding>,
    /// Annotation-level metadata for standard-specific fields that are not yet modeled.
    pub metadata: Metadata,
}

const MULTI_STANDARD_METADATA_KEY: &str = "boq_core.multi_standard";

impl MultiStandardAnnotations {
    /// Returns true when the annotation set carries no multi-standard evidence.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.classifications.is_empty()
            && self.price_catalog_references.is_empty()
            && self.quantity_references.is_empty()
            && self.progress_references.is_empty()
            && self.provenance.is_empty()
            && self.loss_findings.is_empty()
            && self.metadata.is_empty()
    }
}

/// A classification reference for a canonical BoQ item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationReference {
    /// Classification system that defined the code.
    pub system: ClassificationSystem,
    /// Classification code as stated by the source or mapping evidence.
    pub code: String,
    /// Optional human-readable classification label.
    pub label: Option<String>,
    /// Confidence level for the mapping.
    pub confidence: ReferenceConfidence,
    /// Optional source provenance for this classification.
    pub source: Option<SourceProvenance>,
    /// Classification-specific metadata.
    pub metadata: Metadata,
}

/// Known classification systems that can be carried without implying support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationSystem {
    /// GAEB-native classification or hierarchy evidence.
    Gaeb,
    /// DIN 276 cost group.
    Din276,
    /// CSI MasterFormat classification.
    CsiMasterFormat,
    /// Uniclass classification.
    Uniclass,
    /// NL-SfB classification.
    NlSfb,
    /// Brazilian SINAPI classification evidence.
    Sinapi,
    /// Dutch STABU classification evidence.
    Stabu,
    /// Spain/LatAm DQE-style classification evidence.
    Dqe,
    /// A named project, customer, or national classification system.
    Custom(String),
}

/// A confidence marker for references carried by the canonical model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceConfidence {
    /// Observed directly in the parsed source.
    Observed,
    /// Derived by a deterministic mapping or parser rule.
    Derived,
    /// Candidate evidence that still requires downstream confirmation.
    Candidate,
}

/// A price or catalog reference for a canonical BoQ item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceCatalogReference {
    /// Price/catalog system that defined the code.
    pub system: CatalogSystem,
    /// Catalog code or price reference.
    pub code: String,
    /// Optional human-readable label.
    pub label: Option<String>,
    /// Optional unit price carried as evidence.
    pub unit_price: Option<Decimal>,
    /// Optional currency for the price evidence.
    pub currency: Option<String>,
    /// Optional source provenance for this catalog reference.
    pub source: Option<SourceProvenance>,
    /// Catalog-specific metadata.
    pub metadata: Metadata,
}

/// Known price/catalog systems that can be carried without implying support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogSystem {
    /// Brazilian SINAPI catalog evidence.
    Sinapi,
    /// Italian prezzario catalog evidence.
    Prezzario,
    /// Spanish or LatAm concept catalog evidence.
    CatalogoConceptos,
    /// Spanish cuadro de precios evidence.
    CuadroPrecios,
    /// Dutch STABU catalog evidence.
    Stabu,
    /// A named project, customer, or national catalog system.
    Custom(String),
}

/// Quantity evidence linked to a canonical BoQ item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantityReference {
    /// Quantity reference kind.
    pub kind: QuantityReferenceKind,
    /// Stable reference identifier from the source or derived mapping.
    pub reference: String,
    /// Optional quantity value.
    pub quantity: Option<Decimal>,
    /// Optional unit label for the quantity.
    pub unit: Option<String>,
    /// Optional source provenance for this quantity reference.
    pub source: Option<SourceProvenance>,
    /// Recoverable findings specific to this quantity reference.
    pub findings: Vec<ValidationFinding>,
    /// Quantity-reference metadata.
    pub metadata: Metadata,
}

/// Quantity evidence families that can be carried without implying support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuantityReferenceKind {
    /// GAEB X31 quantity takeoff evidence.
    GaebX31,
    /// Measurement-book or measurement-sheet evidence.
    Measurement,
    /// Progress-measurement evidence.
    Progress,
    /// External quantity evidence.
    External,
    /// A named project, customer, or national quantity reference family.
    Custom(String),
}

/// Progress evidence linked to a canonical BoQ item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressReference {
    /// Stable progress reference identifier from the source or derived mapping.
    pub reference: String,
    /// Optional percent-complete value.
    pub percent_complete: Option<Decimal>,
    /// Optional completed quantity value.
    pub quantity_complete: Option<Decimal>,
    /// Optional unit label for the completed quantity.
    pub unit: Option<String>,
    /// Optional source provenance for this progress reference.
    pub source: Option<SourceProvenance>,
    /// Recoverable findings specific to this progress reference.
    pub findings: Vec<ValidationFinding>,
    /// Progress-reference metadata.
    pub metadata: Metadata,
}

/// A BoQ line item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoqItem {
    /// Short description.
    pub short_text: String,
    /// Optional rich long text.
    pub long_text: Option<RichText>,
    /// Quantity.
    pub quantity: Decimal,
    /// Unit label.
    pub unit: String,
    /// Optional unit price.
    pub unit_price: Option<Decimal>,
    /// Optional total price.
    pub total_price: Option<Decimal>,
    /// Optional item notes.
    pub notes: Option<String>,
    /// Item metadata.
    pub metadata: Metadata,
}

impl BoqItem {
    /// Returns typed multi-standard evidence stored in item metadata.
    ///
    /// Missing metadata returns an empty annotation set. Malformed metadata is
    /// returned as a finding so corrupted provenance or loss evidence is not
    /// silently hidden. The accessor preserves source compatibility for callers
    /// that construct `BoqItem` with struct literals while still giving the
    /// canonical model a typed host for cross-standard evidence. Returned
    /// annotations do not promote support status.
    ///
    /// # Errors
    ///
    /// Returns a finding when the reserved metadata value exists but cannot be
    /// decoded as [`MultiStandardAnnotations`].
    pub fn try_multi_standard(&self) -> Result<MultiStandardAnnotations, ValidationFinding> {
        let Some(value) = self.metadata.get(MULTI_STANDARD_METADATA_KEY) else {
            return Ok(MultiStandardAnnotations::default());
        };

        serde_json::from_value(value.clone()).map_err(|error| {
            ValidationFinding::warning(
                "multi_standard_metadata_malformed",
                format!("multi-standard annotation metadata could not be decoded: {error}"),
            )
            .at(MULTI_STANDARD_METADATA_KEY)
        })
    }

    /// Stores typed multi-standard evidence in item metadata.
    ///
    /// Empty annotations remove the metadata key so serialized output remains
    /// stable for items without cross-standard evidence.
    ///
    /// # Errors
    ///
    /// Returns a finding if the annotation value cannot be serialized into
    /// metadata.
    pub fn set_multi_standard(
        &mut self,
        annotations: MultiStandardAnnotations,
    ) -> Result<(), ValidationFinding> {
        if annotations.is_empty() {
            self.metadata.remove(MULTI_STANDARD_METADATA_KEY);
            return Ok(());
        }

        let value = serde_json::to_value(annotations).map_err(|error| {
            ValidationFinding::warning(
                "multi_standard_metadata_serialize_failed",
                format!("multi-standard annotation metadata could not be encoded: {error}"),
            )
            .at(MULTI_STANDARD_METADATA_KEY)
        })?;
        self.metadata
            .insert(MULTI_STANDARD_METADATA_KEY.to_owned(), value);
        Ok(())
    }
}

/// Rich text preserving plain text and structured fragments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RichText {
    /// Plain text long text.
    Plain(String),
    /// XHTML or XML fragment captured without rendering.
    XhtmlFragment(String),
    /// Mixed text fragments.
    Mixed(Vec<RichTextFragment>),
}

/// A rich text fragment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RichTextFragment {
    /// Text fragment.
    Text(String),
    /// Table fragment preserved as markup.
    Table(String),
    /// Image reference or base64 payload metadata.
    Image {
        /// Image identifier or reference key.
        id: String,
        /// Optional image description.
        description: Option<String>,
    },
    /// Unknown markup fragment.
    Unknown(String),
}
