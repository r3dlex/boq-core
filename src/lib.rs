//! Loss-aware GAEB parser core and Obra adapter foundation.
//!
//! `boq-core` parses selected GAEB bill-of-quantities inputs into a
//! loss-aware [`model::GaebDocument`] and, when the parsed source declares
//! adapter support, can map the hierarchy into Obra-compatible import DTOs.
//!
//! # Current support boundary
//!
//! The MVP is intentionally conservative:
//!
//! - GAEB DA XML 3.3 AVA paths are the certification-readiness focus.
//! - GAEB 90 has no blanket promotion: the selected Dangl GAEB 90 D83 fixture path is adapter-compatible when the manifest and tests say so, while D81 and unmanifested D83 inputs remain `supported_parse_only`.
//! - X31 has parser-backed `supported_parse_only` canonical quantity evidence
//!   for selected paths; it still does not imply Obra adapter DTO,
//!   export, billing, full REB formula conformance, roundtrip, or certification
//!   support. GAEB XML 3.4 beta, GAEB 2000, Handel, Kosten/Kalkulation,
//!   and Zeitvertrag remain future or reference tracks until `gaeb/manifest.toml`
//!   and tests say otherwise.
//! - BVBS and GAEBXmlChecker evidence must not be described as paid or official
//!   certification.
//!
//! # Public parse entrypoints
//!
//! The stable public parsing surface is intentionally small:
//!
//! - GAEB 90 D81 and GAEB 90 D83 fixed-width bytes are parsed through
//!   [`gaeb90::parse_bytes`] or [`gaeb90::parse_file`]. Legacy ANSI inputs can
//!   use [`gaeb90::parse_bytes_with_encoding`] with
//!   [`gaeb90::Gaeb90Encoding::Windows1252`] when the caller already knows the
//!   source encoding. The selected Dangl GAEB 90 D83 fixture path is adapter-compatible when [`support::SupportCapabilities::adapt_to_obra`] is true; D81 and unmanifested D83 inputs remain `supported_parse_only`, so there is no blanket GAEB 90 promotion.
//! - GAEB DA XML X81 and other XML phases are parsed through
//!   [`gaeb_xml::parse_str`] or [`gaeb_xml::parse_file`]. GAEB DA XML X81,
//!   X84, and X86 AVA fixture paths are the current adapter-ready focus.
//! - Texterstellung X81/X82 paths have `supported_parse_only` rich-text/table parser-readiness evidence
//!   for parser structure preservation; they do not imply Obra adapter DTO,
//!   visual rendering, export, roundtrip, production, or certification support.
//! - GAEB DA XML X83/X84 Bauausführung paths are recognized as `supported_parse_only`
//!   fixture-backed tracks when the manifest says so; Obra adapter DTO readiness
//!   is capability-gated by [`support::SupportCapabilities::adapt_to_obra`],
//!   while schema validation, export, and roundtrip remain disabled unless
//!   capabilities say otherwise.
//! - X31 quantity-takeoff concepts are represented by [`x31`] as parse-only
//!   canonical quantity evidence without overloading BoQ item parser semantics.
//! - Canonical BoQ items expose
//!   [`model::MultiStandardAnnotations`] for future classification,
//!   price/catalog, quantity, progress, provenance, and loss evidence. Empty
//!   annotations are the parser default, and populated annotations do not
//!   promote support or imply Obra adapter, export, billing, production, or
//!   certification readiness.
//! - DIN 276 classification evidence is available through the fixture-backed
//!   [`din276`] overlay. It adds canonical classification annotations only and
//!   does not promote support or grant adapter support to parse-only inputs.
//! - CSI MasterFormat classification evidence is available through the fixture-backed
//!   [`csi_masterformat`] overlay. It adds canonical classification annotations
//!   only and does not promote support or grant adapter support to parse-only inputs.
//! - Uniclass classification evidence is available through the fixture-backed
//!   [`uniclass`] overlay. It adds canonical classification annotations only and
//!   does not promote support or grant adapter support to parse-only inputs.
//! - NL-SfB classification evidence is available through the fixture-backed
//!   [`nlsfb`] overlay. It adds canonical classification annotations only and
//!   does not promote support or grant adapter support to parse-only inputs.
//! - SINAPI catalog and BDI evidence is available through the fixture-backed
//!   [`sinapi`] overlay. It adds synthetic price-catalog annotations only and
//!   does not promote support or grant adapter support to parse-only inputs.
//! - Computo Metrico and Prezzario evidence is available through the fixture-backed
//!   [`prezzario`] overlay. It adds synthetic price-list and quantity annotations
//!   only and does not promote support or grant adapter support to parse-only inputs.
//! - Catálogo de Conceptos and Cuadro de Precios evidence is available through the fixture-backed
//!   [`catalogo`] overlay. It adds synthetic concept-catalog and price-table annotations
//!   only and does not promote support or grant adapter support to parse-only inputs.
//! - STABU and RAW exchange evidence is available through the fixture-backed
//!   [`stabu`] overlay. It adds synthetic Dutch classification and catalog annotations
//!   only and does not promote support or grant adapter support to parse-only inputs.
//! - DQE quantity-estimate evidence is available through the fixture-backed
//!   [`dqe`] overlay. It adds synthetic French classification and quantity annotations
//!   only and does not promote support or grant adapter support to parse-only inputs.
//! - Spreadsheet-neutral CSV exchange is available through dependency-free
//!   [`spreadsheet`] helpers. Rows are matched by OZ/item ordinal only and the helpers
//!   do not promote support or grant adapter support to parse-only inputs.
//! - X89 Rechnung paths have synthetic invoice-domain parser evidence and an
//!   [`x89::ObraBillingDraft`] boundary for Obra billing design. This does not
//!   imply XRechnung generation, public-sector billing readiness, manifest
//!   support promotion, production support, export, roundtrip, or certification.
//! - X89 invoice concepts are represented by [`x89`] without generating
//!   XRechnung envelopes or promoting parser support.
//!
//! Public callers should inspect [`model::GaebDocument::support_status`] and
//! [`model::GaebDocument::capabilities`] before assuming validation, Obra
//! adapter, export, or roundtrip behavior.
//!
//! # Examples
//!
//! Parse GAEB DA XML from memory:
//!
//! ```
//! let source = include_str!("../tests/fixtures/synthetic/minimal_ava.x81");
//! let document = boq_core::gaeb_xml::parse_str(
//!     source,
//!     Some("minimal_ava.x81".to_owned()),
//! )?;
//!
//! assert_eq!(document.boq.title, "Minimal AVA");
//! assert_eq!(document.summary.format, boq_core::model::GaebFormat::GaebXml);
//! # Ok::<(), boq_core::error::ParseError>(())
//! ```
//!
//! Parse GAEB 90 bytes as parse-only data:
//!
//! ```
//! let bytes = include_bytes!("../tests/fixtures/synthetic/minimal.d81");
//! let document = boq_core::gaeb90::parse_bytes(
//!     bytes,
//!     Some("minimal.d81".to_owned()),
//! )?;
//!
//! assert_eq!(document.summary.format, boq_core::model::GaebFormat::Gaeb90);
//! assert_eq!(
//!     document.support_status,
//!     boq_core::support::SupportStatus::SupportedParseOnly,
//! );
//! # Ok::<(), boq_core::error::ParseError>(())
//! ```
//!
//! Parse GAEB 90 D83 bytes. The source extension drives phase detection; the
//! checksum and decoded findings still describe the original bytes:
//!
//! ```
//! let bytes = include_bytes!("../tests/fixtures/synthetic/minimal.d81");
//! let document = boq_core::gaeb90::parse_bytes(
//!     bytes,
//!     Some("example.D83".to_owned()),
//! )?;
//!
//! assert_eq!(document.summary.format, boq_core::model::GaebFormat::Gaeb90);
//! assert_eq!(document.summary.phase.as_ref().map(|phase| phase.code.as_str()), Some("83"));
//! assert_eq!(
//!     document.support_status,
//!     boq_core::support::SupportStatus::SupportedParseOnly,
//! );
//! # Ok::<(), boq_core::error::ParseError>(())
//! ```
//!
//! Parse GAEB DA XML X83 as a loss-aware Bau document whose Obra adapter DTO
//! readiness remains explicit in capabilities:
//!
//! ```
//! let source = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Bau X83</Name><BoQ><BoQBody><BoQCtgy ID="001" RNoPart="001"><Item ID="001.0010" RNoPart="10"><Qty>1.000</Qty><QU>m</QU><Description><CompleteText><DetailTxt><Text><p>Trench text</p></Text></DetailTxt></CompleteText></Description></Item></BoQCtgy></BoQBody></BoQ></Project></GAEB>"#;
//! let document = boq_core::gaeb_xml::parse_str(
//!     source,
//!     Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/example.X83".to_owned()),
//! )?;
//!
//! assert_eq!(document.summary.phase.as_ref().map(|phase| phase.code.as_str()), Some("83"));
//! assert!(document.capabilities.adapt_to_obra);
//! assert!(!document.capabilities.export);
//! assert!(!document.capabilities.roundtrip);
//! # Ok::<(), boq_core::error::ParseError>(())
//! ```
//!
//! See the mdBook under `docs/book/` for user, developer, certification
//! evidence, and release guides.

#![allow(
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::use_self
)]

pub mod adapter;
pub mod catalogo;
pub mod checksum;
pub mod csi_masterformat;
pub mod din276;
pub mod dqe;
pub mod error;
pub mod format;
pub mod gaeb2000;
pub mod gaeb90;
pub mod gaeb_xml;
pub mod model;
pub mod nlsfb;
pub mod prezzario;
pub mod sinapi;
pub mod spreadsheet;
pub mod stabu;
pub mod support;
pub mod uniclass;
pub mod x31;
pub mod x89;

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the crate version.
#[must_use]
pub const fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_package_version() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    }
}
