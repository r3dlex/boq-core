#!/usr/bin/env bash
# scripts/validate-rules.sh — Structural validator for Archgate .rules.ts files.
#
# Usage: bash scripts/validate-rules.sh <rules-file>
# Exits 0 on pass, non-zero on fail. Emits a one-line summary to stdout
# and a JSON report to stdout when --format json is passed.

set -euo pipefail

rules_file="${1:-.rules.ts}"
format="text"

if [ "${2:-}" = "--format" ] && [ "${3:-}" = "json" ]; then
  format="json"
fi

if [ ! -f "$rules_file" ]; then
  if [ "$format" = "json" ]; then
    echo '{"status":"fail","message":"rules file not found","rulesFile":"'"$rules_file"'"}'
  else
    echo "FAIL: $rules_file not found"
  fi
  exit 2
fi

# Required exports (the 5 domain rule arrays the .rules.ts template expects).
required_exports=(workspace backend frontend data architecture general)

errors=()
for export in "${required_exports[@]}"; do
  if ! grep -qE "^export const ${export}[[:space:]]*:" "$rules_file"; then
    errors+=("missing required export: $export")
  fi
done

# Severity must be one of error | warn | info.
if grep -qE 'severity[[:space:]]*:[[:space:]]*"[^"]+"' "$rules_file"; then
  bad=$(grep -E 'severity[[:space:]]*:[[:space:]]*"[^"]+"' "$rules_file" \
    | sed -E 's/.*"([^"]+)".*/\1/' \
    | grep -vE '^(error|warn|info)$' || true)
  if [ -n "$bad" ]; then
    errors+=("invalid severity values: $bad (must be error|warn|info)")
  fi
fi

if [ "${#errors[@]}" -eq 0 ]; then
  if [ "$format" = "json" ]; then
    echo '{"status":"pass","rulesFile":"'"$rules_file"'","domains":["workspace","backend","frontend","data","architecture","general"]}'
  else
    echo "PASS: $rules_file — 5 domain exports present, severity values valid"
  fi
  exit 0
fi

if [ "$format" = "json" ]; then
  joined=$(printf '%s|' "${errors[@]}")
  echo '{"status":"fail","rulesFile":"'"$rules_file"'","errors":"'"${joined%|}"'"}'
else
  for e in "${errors[@]}"; do echo "FAIL: $e"; done
fi
exit 1
