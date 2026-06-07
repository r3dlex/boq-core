#!/usr/bin/env bash
# scripts/archgate.sh — Archgate runner wrapper (structural + optional semantic/drift).
#
# Usage:
#   bash scripts/archgate.sh --mode structural --rules .rules.ts --format json
#   ARCHGATE_SEMANTIC=1 bash scripts/archgate.sh --mode drift --base origin/main --head HEAD --format json
#
# Emits one JSON object on stdout. Exit code matches status.

set -euo pipefail

mode="structural"
rules_file=".rules.ts"
base=""
head=""
format="text"

while [ $# -gt 0 ]; do
  case "$1" in
    --mode)    mode="$2"; shift 2 ;;
    --rules)   rules_file="$2"; shift 2 ;;
    --base)    base="$2"; shift 2 ;;
    --head)    head="$2"; shift 2 ;;
    --format)  format="$2"; shift 2 ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

emit() {
  local status="$1" msg="$2"
  if [ "$format" = "json" ]; then
    printf '{"status":"%s","mode":"%s","rulesFile":"%s","base":"%s","head":"%s","checks":[{"id":"archgate-%s","status":"%s","message":"%s"}],"exitCode":%s}\n' \
      "$status" "$mode" "$rules_file" "$base" "$head" "$mode" "$status" "$msg" "$([ "$status" = "pass" ] && echo 0 || echo 1)"
  else
    echo "[$mode] $status: $msg"
  fi
}

case "$mode" in
  structural)
    if bash "$(dirname "$0")/validate-rules.sh" "$rules_file" --format json >/dev/null 2>&1; then
      emit "pass" "rules file exports the 5 required domains with valid severity values"
      exit 0
    else
      emit "fail" "rules file failed structural validation; run scripts/validate-rules.sh $rules_file for details"
      exit 1
    fi
    ;;
  semantic|drift)
    if [ -z "${ARCHGATE_SEMANTIC:-}" ]; then
      emit "skipped" "ARCHGATE_SEMANTIC=1 not set; semantic/drift modes are opt-in and return skipped without it"
      exit 0
    fi
    # Project-specific semantic/drift checker not yet implemented; until then, fail closed
    # when explicitly requested.
    emit "fail" "no project-specific semantic/drift checker wired for $mode; implement scripts/archgate-${mode}.sh and re-run with ARCHGATE_SEMANTIC=1"
    exit 1
    ;;
  *)
    emit "fail" "unknown mode: $mode (use structural|semantic|drift)"
    exit 2
    ;;
esac
