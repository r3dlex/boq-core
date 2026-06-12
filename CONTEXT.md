# boq-core — GAEB Parsing Context

Parses GAEB Bill-of-Quantities exchange files into a loss-aware domain model.
Obra compatibility is a separate adapter concern (ARCH-001); support claims are
governed by the Fixture Manifest (ARCH-002).

## Language

**GAEB**:
The German data-exchange standard for construction Bills of Quantities. Comes
in format families (GAEB 90, GAEB 2000, GAEB DA XML) and phases.

**Phase**:
The exchange step a GAEB file represents, identified by a two-digit code with
a format prefix (D81/X81 request … D84/X84 bid, D86/X86 award).
_Avoid_: "file type", "variant"

**Bill of Quantities (BoQ)**:
The hierarchical document of chapters and line items being exchanged.
_Avoid_: WBS (that is the Obra-side concept the adapter maps into)

**Fixture**:
A cataloged GAEB sample file used for tests and certification evidence. Being
cataloged does not imply parser support (ARCH-002).

**Fixture Manifest**:
The TOML catalog (`gaeb/manifest.toml`) recording every Fixture's identity,
license, checksum, and Support Status. The single source of truth for support
claims; embedded into the library at compile time.
_Avoid_: "registry file", "fixture list"

**Support Status**:
The honest claim level for a Fixture or phase: `supported`,
`supported_parse_only`, `future_track`, or `reference_only`.
_Avoid_: "works", "compatible", "implemented"

**Support Capabilities**:
The direction-aware booleans (detect / parse / validate / adapt-to-Obra /
export / roundtrip) that spell out exactly what a Support Status means.

**Support Policy**:
The decision attached to every parsed document: Support Status + Support
Capabilities + a human-readable reason, derived from the Fixture Manifest and
conservative per-format defaults.

**Provenance**:
Where a parsed document came from: source URI, format, version, Phase,
checksum, parser version. Immutable once parsed.

**Loss Report**:
The adapter-side record of GAEB data that could not be mapped into Obra
concepts (ARCH-001). Owned by the Obra Adapter output, not threaded through
the parser.

**Obra Adapter**:
The module mapping a parsed GAEB document into Obra-compatible import DTOs,
carrying Provenance and a Loss Report.
_Avoid_: "exporter", "converter"

## Flagged ambiguities

- **`supported` (manifest string) vs `Supported` (status)**: a manifest row
  marked `supported` only promotes to `SupportStatus::Supported` when its
  `process_domain` is `ava`; all other domains stay parse-only. This promotion
  rule is intentional (certification focus is AVA) and must remain explicit in
  the Support Policy decision, not buried in a match arm.

## Example dialogue

> **Dev**: The X84 fixture parses fine — can I mark it `supported`?
> **Domain expert**: Parsing is not the bar. `supported` means the Fixture
> Manifest row has test coverage mapped, the Support Capabilities claim
> import including the Obra Adapter, and the process domain is AVA. If only
> parsing works, it is `supported_parse_only` — the Support Policy on the
> parsed document will say so, with the reason pointing at the manifest row.
