// .rules.ts — boq-core subrepo Archgate domain rules
// Each rule: name, severity ("error" | "warn" | "info"), match pattern, and example.
// Validated by scripts/validate-rules.sh (structural check only).
// Per-ADR .rules.ts files in .archgate/adrs/ remain the binding architectural rules.

export interface Rule {
  name: string;
  severity: "error" | "warn" | "info";
  match: string;
  violation?: string;
  correction?: string;
}

// ─── backend ─────────────────────────────────────────────────────────────────

export const backend: Rule[] = [
  {
    name: "loss-aware-model-first",
    severity: "error",
    match: "Parser modules must produce a rich GAEB model, not an Obra DTO (ARCH-001).",
    violation: "fn parse_x81(...) -> ObraDto { ... }",
    correction: "fn parse_x81(...) -> GaebModel { ... } // Obra adapter is a separate function",
  },
  {
    name: "no-obra-backend-mvp",
    severity: "error",
    match: "MVP must not modify the sibling ERP backend (ARCH-003).",
    violation: "PR changes files under the sibling ERP backend/",
    correction: "Obra integration is a separate story. Do not modify the obra submodule from a boq-core PR.",
  },
  {
    name: "deterministic-decimals",
    severity: "error",
    match: "Quantities and prices use rust_decimal with serde-with-str, not f64.",
    violation: "let qty: f64 = parse_quantity(s);",
    correction: "let qty: rust_decimal::Decimal = parse_quantity(s)?;",
  },
  {
    name: "no-unwrap-in-library",
    severity: "error",
    match: "Library code must use ?, not unwrap/expect, except in tests and xtask (clippy::unwrap_used).",
    violation: "let v = some_option.unwrap();",
    correction: "let v = some_option?; // or: some_option.ok_or(Error::Missing)?",
  },
];

// ─── frontend (n/a for boq-core) ─────────────────────────────────────────────

export const frontend: Rule[] = [];

// ─── data ────────────────────────────────────────────────────────────────────

export const data: Rule[] = [
  {
    name: "fixture-checksum-required",
    severity: "error",
    match: "Every fixture in gaeb/manifest.toml must have a SHA-256 checksum and a test mapping (ARCH-002).",
    violation: "{ id = \"x\", url = \"...\", support_status = \"supported\" }",
    correction: "{ id = \"x\", url = \"...\", checksum = \"sha256:...\", license_note = \"...\", support_status = \"supported\", test_mapping = [\"test_x_parse\"] }",
  },
  {
    name: "no-executable-in-ci",
    severity: "error",
    match: "Reference-only executable fixtures (.exe) must never be invoked in CI (ARCH-002).",
    violation: "- run: ./fixtures/bvbs/GAEBXmlChecker.exe ...",
    correction: "Catalog the executable in gaeb/manifest.toml as reference_only; do not invoke it.",
  },
  {
    name: "support-status-honest",
    severity: "error",
    match: "support_status is encoded in manifest data and tested (ARCH-002).",
    violation: "support_status = \"supported\" with empty test_mapping",
    correction: "Promote support_status only in the same PR as passing implementation tests and review evidence.",
  },
  {
    name: "no-paid-bvbs-auto-submit",
    severity: "error",
    match: "Paid BVBS submission and official certification claims are user-confirmation-gated (ARCH-004).",
    violation: "CI step calls BVBS submission API",
    correction: "Certify-path readiness only; official submission requires explicit user confirmation.",
  },
];

// ─── architecture ────────────────────────────────────────────────────────────

export const architecture: Rule[] = [
  {
    name: "adr-before-impl",
    severity: "error",
    match: "Architectural decisions must be recorded in .archgate/adrs/ARCH-XXX.md before implementation.",
    violation: "PR introduces a new architectural pattern with no ADR.",
    correction: "Add an ADR with id, title, status, date, context, decision, and consequences.",
  },
  {
    name: "boundary-adr-for-issue-tracks",
    severity: "error",
    match: "Each issue-level PRD (issues #37–#44) must be preceded by a boundary ADR (gap-analysis, schema-delta, etc.).",
    violation: "PR for issue #40 (GAEB 90 adapter promotion) without a gap-analysis ADR",
    correction: "Add a boundary ADR naming the exact data required for the promotion.",
  },
];

// ─── general ─────────────────────────────────────────────────────────────────

export const general: Rule[] = [
  {
    name: "marker-blocks-preserved",
    severity: "warn",
    match: "AI SDLC marker blocks must not be removed when AGENTS.md, CLAUDE.md, or README.md is updated.",
    violation: "AGENTS.md updated with the AI SDLC section removed.",
    correction: "Preserve the marker block; update content inside it.",
  },
  {
    name: "no-new-deps-without-request",
    severity: "info",
    match: "ai-sdlc-init does not add new dependencies. New crates need explicit user request.",
    violation: "PR adds a new dep without justification",
    correction: "Document the new dep in the PRD; require explicit user approval.",
  },
];

// ─── workspace (binding to obra-ws) ──────────────────────────────────────────

export const workspace: Rule[] = [
  {
    name: "workspace-binding-required",
    severity: "warn",
    match: "boq-core subrepo changes that affect the workspace binding must be paired with a workspace ADR (WS-XXX).",
    violation: "PR changes gaeb/manifest.toml but no WS-XXX ADR is added.",
    correction: "Add docs/adr/WS-XXX.md in the obra-ws workspace (or reference an existing one).",
  },
];
