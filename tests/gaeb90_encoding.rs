#![allow(missing_docs, clippy::expect_used)]

use boq_core::checksum::sha256_hex;
use boq_core::gaeb90::{self, Gaeb90Encoding};

const WINDOWS_1252_UMLAUT_BYTES: &[u8] = b"2101010020  10000m\r\n25M\xFCller Stra\xDFe\r\n";
const WINDOWS_1252_UNDEFINED_BYTE: &[u8] = b"2101010020  10000m\r\n25Bad\x81Byte\r\n";

#[test]
fn test_gaeb90_windows_1252_umlauts_decode() {
    let document = gaeb90::parse_bytes_with_encoding(
        WINDOWS_1252_UMLAUT_BYTES,
        Some("umlaut.d83".to_owned()),
        Gaeb90Encoding::Windows1252,
    )
    .expect("explicit Windows-1252 GAEB 90 should parse");

    assert_eq!(
        document.metadata.get("gaeb90.encoding"),
        Some(&serde_json::json!("windows-1252"))
    );
    assert!(document.boq.nodes[0].title.contains("Müller Straße"));
    assert!(
        document
            .findings
            .iter()
            .all(|finding| finding.code != "gaeb90_encoding_fallback"),
        "caller-specified Windows-1252 should not be reported as auto fallback"
    );
}

#[test]
fn test_gaeb90_invalid_bytes_emit_findings() {
    let document = gaeb90::parse_bytes_with_encoding(
        WINDOWS_1252_UNDEFINED_BYTE,
        Some("invalid-byte.d83".to_owned()),
        Gaeb90Encoding::Utf8,
    )
    .expect("UTF-8 replacement decoding should remain recoverable");

    assert_eq!(
        document.metadata.get("gaeb90.encoding"),
        Some(&serde_json::json!("utf-8"))
    );
    assert!(
        document
            .findings
            .iter()
            .any(|finding| finding.code == "gaeb90_decode_replacement"),
        "invalid UTF-8 byte sequences must produce decode findings"
    );
}

#[test]
fn test_gaeb90_original_bytes_checksum_preserved() {
    let document = gaeb90::parse_bytes_with_encoding(
        WINDOWS_1252_UMLAUT_BYTES,
        Some("checksum.d83".to_owned()),
        Gaeb90Encoding::Windows1252,
    )
    .expect("explicit Windows-1252 GAEB 90 should parse");

    assert_eq!(
        document.source.checksum.as_deref(),
        Some(sha256_hex(WINDOWS_1252_UMLAUT_BYTES).as_str())
    );
}

#[test]
fn test_gaeb90_encoding_detection_is_explicit() {
    let auto = gaeb90::parse_bytes(WINDOWS_1252_UMLAUT_BYTES, Some("auto.d83".to_owned()))
        .expect("auto decoding should fall back to Windows-1252");
    let explicit = gaeb90::parse_bytes_with_encoding(
        WINDOWS_1252_UMLAUT_BYTES,
        Some("explicit.d83".to_owned()),
        Gaeb90Encoding::Windows1252,
    )
    .expect("explicit Windows-1252 decoding should parse");

    assert_eq!(
        auto.metadata.get("gaeb90.encoding"),
        Some(&serde_json::json!("windows-1252"))
    );
    assert!(
        auto.findings
            .iter()
            .any(|finding| finding.code == "gaeb90_encoding_fallback"),
        "auto mode should report non-UTF-8 fallback explicitly"
    );
    assert!(
        explicit
            .findings
            .iter()
            .all(|finding| finding.code != "gaeb90_encoding_fallback"),
        "caller-specified decoding should be explicit, not detection fallback"
    );
}
