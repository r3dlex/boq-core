//! Loss-aware GAEB parser core and Obra adapter foundation.
//!
//! The implementation is driven by the approved PRD in
//! `../.omx/plans/prd-boq-core-gaeb-parser-20260606.md`.

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
