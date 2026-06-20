# PRD: Support legacy ANSI/Windows-1252 encoding paths for GAEB 90

## Issue
- GitHub issue: #24
- Milestone: v0.3 Public API and parser robustness

## Product outcome
GAEB 90 parsing supports automatic UTF-8-to-Windows-1252 fallback and caller-specified ANSI/Windows-1252 decoding while preserving byte checksums and loss-aware findings.

## Source/status anchors
- GAEB 90 D81/D83: supported parse-only MVP path.
- Windows-1252 samples: `tests/gaeb90_encoding.rs`.

## Requirements
- [x] Define byte-preservation and decoded-text policy.
  - `source.checksum` always hashes original bytes.
  - `document.metadata["gaeb90.encoding"]` records the effective decoder.
  - `parse_bytes` remains automatic UTF-8 first, Windows-1252 fallback; `parse_bytes_with_encoding` lets callers choose `Utf8` or `Windows1252` explicitly.
- [x] Decode umlauts/special characters without data loss.
  - Windows-1252 `Müller Straße` bytes decode into title text through explicit Windows-1252 parsing.
- [x] Report invalid bytes as structured findings.
  - Invalid explicit UTF-8 bytes emit `gaeb90_decode_replacement`; automatic fallback emits `gaeb90_encoding_fallback`.

## Implemented tests
- [x] `test_gaeb90_windows_1252_umlauts_decode`
- [x] `test_gaeb90_invalid_bytes_emit_findings`
- [x] `test_gaeb90_original_bytes_checksum_preserved`
- [x] `test_gaeb90_encoding_detection_is_explicit`

## Verification
- [x] `cargo test --test gaeb90_encoding`
