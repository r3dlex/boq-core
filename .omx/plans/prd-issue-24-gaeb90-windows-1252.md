# PRD: Support legacy ANSI/Windows-1252 encoding paths for GAEB 90

## Issue
- GitHub issue: #24
- Milestone: v0.3 Public API and parser robustness

## Product outcome
GAEB 90 parsing supports explicit ANSI/Windows-1252 decoding while preserving byte checksums and loss-aware findings.

## Source/status anchors
- GAEB 90 D81/D83: supported MVP path.
- Windows-1252 samples: encoding fixture evidence.

## Requirements
- [ ] Define byte-preservation and decoded-text policy.
- [ ] Decode umlauts/special characters without data loss.
- [ ] Report invalid bytes as structured findings.

## Planned tests
- [ ] `test_gaeb90_windows_1252_umlauts_decode`
- [ ] `test_gaeb90_invalid_bytes_emit_findings`
- [ ] `test_gaeb90_original_bytes_checksum_preserved`
- [ ] `test_gaeb90_encoding_detection_is_explicit`
