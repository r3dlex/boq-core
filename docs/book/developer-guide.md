# Developer Guide

This guide describes how to extend `boq-core` without weakening the support-status contract, the fixture manifest, or Obra hierarchy compatibility. It is developer-facing operational guidance; it does not create parser support or official certification claims by itself.

## Architecture map

| Area | Primary module or artifact | Responsibility | Extension notes |
| --- | --- | --- | --- |
| Loss-aware domain model | `boq_core::model` (`src/model.rs`) | `GaebDocument`, `Boq`, `BoqNode`, `BoqItem`, rich text, metadata, provenance, and findings. | Add model fields only when tests show the source data cannot be represented as metadata or findings. |
| Format detection | `boq_core::format` (`src/format.rs`) | Detects GAEB family, phase, and extension from source paths before parsing. | Add red tests for new extensions/phases before changing detection. |
| GAEB 90 parser | `boq_core::gaeb90` (`src/gaeb90.rs`) | Fixed-width GAEB 90 decoding and parse-only document construction. | Preserve original bytes/checksums and emit findings for malformed records instead of panicking. |
| GAEB DA XML parser | `boq_core::gaeb_xml` (`src/gaeb_xml/`) | XML 3.x parser foundation, rich text handling, support-policy tagging, and XML writer boundaries. | Keep unsupported elements visible through metadata/findings until promoted by tests and manifest evidence. |
| X31 quantities | `boq_core::x31` (`src/x31.rs`) | Quantity-takeoff domain model/parser with formula and attachment findings. | Do not treat X31 quantity evidence as BoQ adapter support without an explicit bridge plan. |
| X89 invoices | `boq_core::x89` (`src/x89.rs`) | Invoice-domain planning model separate from BoQ parsing and XRechnung generation. | `InvoiceDocument` is not an XRechnung payload and must stay boundary-gated. |
| Obra adapter boundary | `boq_core::adapter::obra` (`src/adapter/obra.rs`) | Deterministic DTOs compatible with Obra import concepts. | Keep sibling ERP server changes out of scope per `ARCH-003`; the Rust crate emits DTOs, not migrations. |
| Support policy | `boq_core::support` (`src/support.rs`) and `gaeb/manifest.toml` | `SupportStatus`, `SupportCapabilities`, fixture policies, and manifest-backed promotion rules. | Support claims must match the fixture manifest and automated tests. |
| Checksums/provenance | `boq_core::checksum` (`src/checksum.rs`) | SHA-256 helpers for source provenance and reproducible fixture evidence. | Checksum drift should be treated as evidence drift until reviewed. |

Relevant architecture decisions:

- `ARCH-001` keeps the loss-aware model before adapters.
- `ARCH-002` requires honest fixture support statuses from the manifest vocabulary.
- `ARCH-003` keeps Obra backend integration out of the MVP.
- `ARCH-004` gates paid BVBS submission and official certification claims.
- `ARCH-005` centralizes support policy in one seat instead of scattered claims.

## Fixture governance

The fixture manifest at `gaeb/manifest.toml` is the source of truth for cataloged inputs. Treat it as product evidence, not as a convenience list. Each fixture entry must make the following fields reviewable:

- `id` — stable identifier used by tests, reports, and documentation.
- `source_url` / `normalized_url` — external source location; network access remains explicit and reviewable.
- `source_family`, `process_domain`, `gaeb_version`, and `phase` — classification used for support-policy decisions.
- `target_dir` — where the fixture or reference artifact belongs when downloaded.
- `support_status` — one of the manifest vocabulary values below.
- `ci_policy` — whether CI may use the fixture directly, download on demand, or only catalog it.
- `license_note` — redistribution and usage caveats.
- `test_mapping` — tests that justify the support status.
- `checksum` fields, when present — locked checksums used to detect source drift.

Manifest support-status vocabulary must stay verbatim:

- `supported` — the named flow has parser evidence and all advertised capabilities for that scope.
- `supported_parse_only` — parsing has evidence, but validation, adapter, export, roundtrip, or certification capability remains limited.
- `future_track` — planned follow-on work with catalog/boundary tests, not production support.
- `reference_only` — reference material such as criteria PDFs, schemas, GAEBXmlChecker packages, beta packages, or external tools.

Governance rules:

1. Do not promote a fixture by editing prose alone; promote via tests plus `gaeb/manifest.toml`.
2. Do not run `reference_only` executables in CI unless a later ADR explicitly allows it.
3. Do not commit downloaded payloads unless the license note and checksum policy allow it.
4. Keep fixture report files deterministic. If parser output intentionally changes, refresh the golden report with the documented opt-in environment variable and explain the diff in the PR.
5. For BVBS-related sources, say readiness/evidence unless paid submission and official certification have actually been authorized and completed under `ARCH-004`.

## TDD and coverage workflow

Use red-green-refactor for new parser, adapter, or support-policy behavior:

1. Start with a PRD/plan or issue acceptance criteria that names the exact phase and support boundary.
2. Add a failing unit, integration, docs, or fixture-manifest test for the smallest behavior slice.
3. Implement the minimal code or documentation change to make that test pass.
4. Run the focused test first, then the full local quality gate.
5. Refactor only after the behavior is locked by tests.
6. Merge only after local and GitHub CI are green and review comments are resolved.

Implementation PRs must preserve the 95% line/function/region coverage policy. The local coverage gate is:

```bash
LLVM_COV="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-cov" \
LLVM_PROFDATA="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-profdata" \
cargo llvm-cov --all-features --summary-only \
  --ignore-filename-regex 'src/bin/xtask.rs' \
  --fail-under-lines 95 \
  --fail-under-functions 95 \
  --fail-under-regions 95
```

Docs-only PRs should still run the documentation-focused tests and mdBook build; they may also run the full gate when they change PRDs, architecture links, or wording that could trigger governance tests.

## Local quality gate

Before opening or updating a PR, choose the smallest useful focused checks, then run the broader gate expected by the issue. The standard gate for implementation work is:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo run --bin xtask -- fixtures verify
cargo doc --all-features --no-deps
mdbook build
archgate check --ci
uvx prek run --all-files
```

For docs-MVP changes, run at least:

```bash
cargo test --test docs_mvp
mdbook build
archgate check --ci
uvx prek run --all-files
```

## Obra adapter contract

The Obra adapter preserves hierarchical intent without modifying the Obra backend:

- chapter/item structure maps to WBS node candidates;
- ordinal numbers drive deterministic keys and path candidates;
- item quantities, units, prices, and long texts map to line-item DTOs;
- findings and lossy mappings remain visible in the loss report;
- parse-only, future-track, and reference-only inputs are rejected instead of silently producing unsupported imports.

Per `ARCH-003`, do not instruct contributors to patch the sibling ERP server for `boq-core` MVP work. Cross-repo integration belongs in a later workspace ADR/issue.

## Extension workflow

For a new GAEB phase such as X83, X31, X89, GAEB XML 3.4 beta, GAEB 2000, or a non-certification exchange track:

1. Register or confirm fixture manifest entries and source policy.
2. Add boundary tests that prevent overclaiming before implementation tests.
3. Add parser/domain tests using synthetic data first, then real fixture integration tests when legally and operationally safe.
4. Capture support-policy decisions in tests before adding broader adapter behavior.
5. Update rustdoc and mdBook only after behavior exists.
6. Keep paid submission, credentialed external tools, external publishing, and official certification claims gated by explicit human authorization.

### X89/Rechnung model boundary

`boq_core::x89` is a GAEB invoice-domain model for X89/Rechnung planning. It keeps invoice headers, parties, line amounts, tax/payment data, X86 contract baseline links, X31 quantity evidence links, totals, and audit findings separate from both the BoQ parser and any XRechnung envelope generator. A populated `InvoiceDocument` is not parser support, not an Obra adapter DTO, and not an XRechnung payload; use `InvoiceDocument::xrechnung_boundary()` to expose that boundary explicitly. Issue #36 records the planning-only bridge contract in `docs/fixtures/xrechnung-bridge-plan.md`: production XRechnung emission remains blocked until verified X31 quantities, X86 contract baselines, X89 invoice data, and a separate standards/dependency decision are available.
