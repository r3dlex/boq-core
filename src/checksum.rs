//! Source checksum helpers for deterministic provenance.

use sha2::{Digest, Sha256};

/// Computes a lowercase SHA-256 hex digest for source bytes.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(output, "{byte:02x}");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_stable_sha256_hex() {
        assert_eq!(
            sha256_hex(b"boq-core"),
            "7899f1ca1ab7748fe072dddd5bac26c56629e160004bcbc2f6e4ec6c8a65613c"
        );
    }
}
