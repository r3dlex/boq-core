//! X31 quantity takeoff domain model.
//!
//! This module intentionally models measurement/progress data separately from
//! [`crate::model::BoqItem`]. It is a domain boundary for future X31 parsing and
//! X31/X86 baseline linking; it does not claim parser support or BVBS
//! certification readiness by itself.
//!
//! ```
//! use rust_decimal::Decimal;
//! use boq_core::x31::{MeasurementFormula, MeasurementRow, RebFormulaSystem};
//!
//! let row = MeasurementRow::formula(
//!     "row-1",
//!     "001.0010",
//!     "m",
//!     MeasurementFormula::reb_vb_23003("2.5 * 4.0"),
//! )
//! .with_result(Decimal::new(100, 1));
//!
//! assert_eq!(row.ordinal.as_deref(), Some("001.0010"));
//! assert_eq!(row.formula.system, RebFormulaSystem::RebVb23003);
//! ```

use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::ValidationFinding;
use crate::model::{Metadata, SourceProvenance};

/// A complete X31 quantity takeoff document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantityTakeoffDocument {
    /// Source provenance for the X31 payload or future parser output.
    pub source: SourceProvenance,
    /// Optional baseline link to an X86 or BoQ document.
    pub baseline: Option<MeasurementBaselineLink>,
    /// Formula or measurement rows in deterministic source order.
    pub rows: Vec<MeasurementRow>,
    /// Attachment assets referenced by measurement rows.
    pub attachments: Vec<MeasurementAttachment>,
    /// Recoverable findings for unsupported or deferred X31 constructs.
    pub findings: Vec<ValidationFinding>,
    /// Document-level metadata.
    pub metadata: Metadata,
}

impl QuantityTakeoffDocument {
    /// Creates an empty X31 domain document for a known source.
    #[must_use]
    pub const fn new(source: SourceProvenance) -> Self {
        Self {
            source,
            baseline: None,
            rows: Vec::new(),
            attachments: Vec::new(),
            findings: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Returns all measurement rows linked to a BoQ ordinal.
    #[must_use]
    pub fn rows_for_ordinal(&self, ordinal: &str) -> Vec<&MeasurementRow> {
        self.rows
            .iter()
            .filter(|row| row.ordinal.as_deref() == Some(ordinal))
            .collect()
    }

    /// Records an attachment reference that cannot yet be materialized locally.
    pub fn record_attachment_gap(
        &mut self,
        attachment_id: impl Into<String>,
        reason: impl Into<String>,
    ) {
        let attachment_id = attachment_id.into();
        self.findings.push(
            ValidationFinding::warning(
                "x31_attachment_reference_only",
                format!(
                    "X31 attachment {attachment_id} is reference-only: {}",
                    reason.into()
                ),
            )
            .at(attachment_id),
        );
    }
}

/// Link from an X31 measurement set back to its baseline tender/contract data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasurementBaselineLink {
    /// Baseline document identifier or checksum.
    pub document_id: String,
    /// Baseline kind, for example X86 contract or X83 tender.
    pub kind: BaselineKind,
    /// Human-readable relation note.
    pub relation: String,
}

/// Supported baseline link kinds for X31 planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineKind {
    /// X86 contract award baseline.
    X86Contract,
    /// X83 tender/request baseline.
    X83Tender,
    /// Unknown or deferred baseline type.
    Unknown,
}

/// One X31 measurement/formula row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementRow {
    /// Stable row identifier from the X31 source or parser.
    pub row_id: String,
    /// Linked BoQ ordinal, when present.
    pub ordinal: Option<String>,
    /// Formula representation.
    pub formula: MeasurementFormula,
    /// Calculated/result quantity when known.
    pub result_quantity: Option<Decimal>,
    /// Quantity unit.
    pub unit: String,
    /// Optional physical progress data.
    pub progress: Option<PhysicalProgress>,
    /// Additional row references, such as drawings or REB line ids.
    pub references: Vec<MeasurementReference>,
    /// Attachment ids referenced by this row.
    pub attachment_ids: Vec<String>,
    /// Row-level metadata.
    pub metadata: Metadata,
}

impl MeasurementRow {
    /// Creates a formula row linked to a BoQ ordinal.
    #[must_use]
    pub fn formula(
        row_id: impl Into<String>,
        ordinal: impl Into<String>,
        unit: impl Into<String>,
        formula: MeasurementFormula,
    ) -> Self {
        Self {
            row_id: row_id.into(),
            ordinal: Some(ordinal.into()),
            formula,
            result_quantity: None,
            unit: unit.into(),
            progress: None,
            references: Vec::new(),
            attachment_ids: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a calculated/result quantity.
    #[must_use]
    pub const fn with_result(mut self, quantity: Decimal) -> Self {
        self.result_quantity = Some(quantity);
        self
    }

    /// Adds physical progress state.
    #[must_use]
    pub const fn with_progress(mut self, progress: PhysicalProgress) -> Self {
        self.progress = Some(progress);
        self
    }
}

/// Formula payload for X31 measurement rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementFormula {
    /// Formula system.
    pub system: RebFormulaSystem,
    /// Formula expression as source text; evaluation is a later issue.
    pub expression: String,
    /// Deterministic variables known at parse time.
    pub variables: BTreeMap<String, Decimal>,
}

impl MeasurementFormula {
    /// Creates a REB-VB 23.003 expression without evaluating it.
    #[must_use]
    pub fn reb_vb_23003(expression: impl Into<String>) -> Self {
        Self {
            system: RebFormulaSystem::RebVb23003,
            expression: expression.into(),
            variables: BTreeMap::new(),
        }
    }
}

/// Formula system marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebFormulaSystem {
    /// REB-VB 23.003 formula concept, represented but not evaluated here.
    RebVb23003,
    /// Unknown/deferred formula syntax.
    Unknown,
}

/// Physical-progress measurement data.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalProgress {
    /// Completed/progress quantity.
    pub completed_quantity: Decimal,
    /// Optional percent complete, 0-100 by convention.
    pub percent_complete: Option<Decimal>,
}

/// Reference from a measurement row to a source concept.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasurementReference {
    /// Reference kind.
    pub kind: MeasurementReferenceKind,
    /// Reference value.
    pub value: String,
}

/// Measurement reference kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementReferenceKind {
    /// BoQ ordinal reference.
    BoqOrdinal,
    /// Drawing or plan reference.
    Drawing,
    /// REB line/reference id.
    RebLine,
    /// Unknown/deferred reference.
    Unknown,
}

/// Attachment or asset referenced by X31 measurement data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementAttachment {
    /// Stable attachment id.
    pub id: String,
    /// Attachment kind.
    pub kind: MeasurementAttachmentKind,
    /// Optional local or external source URI.
    pub source_uri: Option<String>,
    /// Optional checksum when a local asset is available.
    pub checksum: Option<String>,
    /// Attachment metadata.
    pub metadata: Metadata,
}

/// Attachment classes relevant to quantity takeoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementAttachmentKind {
    /// Drawing, plan, or sketch.
    Drawing,
    /// Photo evidence.
    Photo,
    /// Calculation sheet or REB sidecar.
    CalculationSheet,
    /// Unknown/deferred attachment kind.
    Unknown,
}
