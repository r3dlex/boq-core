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
//! - GAEB 90 D81/D83 is parse-only unless tests and the fixture manifest
//!   promote a more specific capability.
//! - X31, X89, GAEB XML 3.4 beta, GAEB 2000, Handel, Kosten/Kalkulation,
//!   and Zeitvertrag are future or reference tracks until `gaeb/manifest.toml`
//!   and tests say otherwise. The [`x31`] and [`x89`] modules are serializable
//!   domain models for future quantity-takeoff and Rechnung work, not
//!   support-status promotions.
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
//!   source encoding. These paths are currently parse-only unless
//!   [`support::SupportCapabilities`] says otherwise.
//! - GAEB DA XML X81 and other XML phases are parsed through
//!   [`gaeb_xml::parse_str`] or [`gaeb_xml::parse_file`]. GAEB DA XML X81,
//!   X84, and X86 AVA fixture paths are the current adapter-ready focus.
//! - GAEB DA XML X83/X84 Bauausführung paths are recognized as parse-only
//!   fixture-backed tracks when the manifest says so; adapter/export capability
//!   remains disabled unless [`support::SupportCapabilities`] says otherwise.
//! - X31 quantity-takeoff concepts are represented by [`x31`] without overloading
//!   BoQ item parser semantics.
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
//! Parse GAEB DA XML X83 as a loss-aware future-track document without
//! implying adapter support:
//!
//! ```
//! let source = r#"<GAEB><GAEBInfo><Version>3.3</Version></GAEBInfo><Project><Name>Bau X83</Name><BoQ><BoQBody><BoQCtgy ID="001" RNoPart="001"><Item ID="001.0010" RNoPart="10"><Qty>1.000</Qty><QU>m</QU><Description><CompleteText><DetailTxt><Text><p>Trench text</p></Text></DetailTxt></CompleteText></Description></Item></BoQCtgy></BoQBody></BoQ></Project></GAEB>"#;
//! let document = boq_core::gaeb_xml::parse_str(
//!     source,
//!     Some("gaeb/bvbs/gaeb_xml_3_3/construction_execution/x83/example.X83".to_owned()),
//! )?;
//!
//! assert_eq!(document.summary.phase.as_ref().map(|phase| phase.code.as_str()), Some("83"));
//! assert!(!document.capabilities.adapt_to_obra);
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
pub mod checksum;
pub mod error;
pub mod format;
pub mod gaeb90;
pub mod gaeb_xml;
pub mod model;
pub mod support;
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
