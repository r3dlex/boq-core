# ARCH-008: GAEB 2000 / Pxx parser boundary

## Status
Accepted

## Context
Issue #39 tracks GAEB 2000/Pxx compatibility. GAEB 2000 is neither GAEB 90 fixed-width records nor GAEB DA XML. It uses bracketed keyword/tag blocks such as `#begin[...]` and `#end[...]` with phase-specific Pxx/Dxx file extensions.

## Decision
Create a separate `gaeb2000` module for GAEB 2000 tokenization and future parsing. The module owns keyword/block tokenization, begin/end nesting diagnostics, and Pxx/Dxx phase detection helpers. It must not reuse the GAEB XML tree parser or GAEB 90 fixed-width parser as an implementation shortcut.

Current scope is tokenizer/planning readiness only:

- `P81`-`P86` and `D81`-`D86` extensions map to numeric phase codes through the existing format detector.
- GAEB 2000 manifest rows remain `future_track` or `reference_only`.
- Mapping charts remain reference evidence only and must not be used as runtime support evidence.

## Boundaries
- No production support claim for GAEB 2000.
- No adapter/export/roundtrip promotion.
- No external clone/download/browser action in CI.
- No unified parser shared with GAEB XML or GAEB 90 until a future ADR supersedes this boundary.
- Future implementation requires checked-in fixtures or checksums, failing tests, support-policy updates, and review evidence.

## Consequences
Tests must cover source catalog status, basic begin/end nesting tokenization, unclosed begin diagnostics, P81-P86 phase detection, and the mapping-chart reference boundary.
