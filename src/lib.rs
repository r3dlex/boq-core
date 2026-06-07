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
//!   and tests say otherwise.
//! - BVBS and GAEBXmlChecker evidence must not be described as paid or official
//!   certification.
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
