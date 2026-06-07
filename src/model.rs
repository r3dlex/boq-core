//! Loss-aware GAEB domain model.

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
