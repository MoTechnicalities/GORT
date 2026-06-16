#!/usr/bin/env bash
set -u -o pipefail

# Phase80 deterministic runtime gauntlet runner.
#
# Runs Phase62/Phase80 verification suites and emits one PASS/FAIL summary block
# suitable for CI logs and manual architecture audits.
#
# Usage:
#   scripts/phase80_gauntlet.sh

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="/tmp/gort_phase80_gauntlet"
mkdir -p "$OUT_DIR"
JSON_SUMMARY_PATH="${GORT_GAUNTLET_JSON_SUMMARY_PATH:-$OUT_DIR/summary.json}"
GORT_GAUNTLET_JSON_SCHEMA_VERSION="${GORT_GAUNTLET_JSON_SCHEMA_VERSION:-v1}"
GORT_GAUNTLET_JSON_SCHEMA_URI="${GORT_GAUNTLET_JSON_SCHEMA_URI:-}"
JSON_SCHEMA_MANIFEST_PATH="${GORT_GAUNTLET_JSON_SCHEMA_MANIFEST_PATH:-docs/schemas/phase80-gauntlet-schema-manifest.json}"
manifest_signature="unknown"
if [[ -f "$JSON_SCHEMA_MANIFEST_PATH" ]]; then
  manifest_signature="$(sha256sum "$JSON_SCHEMA_MANIFEST_PATH" | awk '{print $1}')"
fi
if [[ -n "$GORT_GAUNTLET_JSON_SCHEMA_URI" ]]; then
  JSON_SCHEMA_URI="$GORT_GAUNTLET_JSON_SCHEMA_URI"
  JSON_EFFECTIVE_SCHEMA_SOURCE="override"
elif [[ "$GORT_GAUNTLET_JSON_SCHEMA_VERSION" == "v1" ]]; then
  JSON_SCHEMA_URI="docs/schemas/phase80-gauntlet-summary-v1.schema.json"
  JSON_EFFECTIVE_SCHEMA_SOURCE="version"
elif [[ "$GORT_GAUNTLET_JSON_SCHEMA_VERSION" == "v1-experimental" ]]; then
  JSON_SCHEMA_URI="docs/schemas/phase80-gauntlet-summary-v1-experimental.schema.json"
  JSON_EFFECTIVE_SCHEMA_SOURCE="version"
else
  JSON_SCHEMA_URI="docs/schemas/phase80-gauntlet-summary-${GORT_GAUNTLET_JSON_SCHEMA_VERSION}.schema.json"
  JSON_EFFECTIVE_SCHEMA_SOURCE="version-fallback"
fi

schema_deprecation_warning_label="schema_deprecation_warning"
schema_deprecation_warning_result=""
schema_version_deprecated="unknown"
if [[ -f "$JSON_SCHEMA_MANIFEST_PATH" ]]; then
  schema_version_deprecated="$(python3 - "$JSON_SCHEMA_MANIFEST_PATH" "$GORT_GAUNTLET_JSON_SCHEMA_VERSION" <<'PY'
import json
import sys

manifest_path = sys.argv[1]
schema_version = sys.argv[2]

try:
    with open(manifest_path, "r", encoding="utf-8") as handle:
        manifest = json.load(handle)
    entry = manifest.get("supported_versions", {}).get(schema_version)
    if entry is None:
        print("unknown")
    else:
        print("true" if bool(entry.get("deprecated", False)) else "false")
except Exception:
    print("unknown")
PY
  )"
fi

if [[ "$schema_version_deprecated" == "true" ]]; then
  schema_deprecation_warning_result="WARN (${GORT_GAUNTLET_JSON_SCHEMA_VERSION} is deprecated)"
  echo "[gauntlet][warning] selected schema version ${GORT_GAUNTLET_JSON_SCHEMA_VERSION} is deprecated" >&2
fi
GORT_GAUNTLET_JSON_SELFTEST="${GORT_GAUNTLET_JSON_SELFTEST:-0}"

# Hard timeout for the longest integration stage to prevent indefinite hangs.
PHASE62_INTEGRATION_TIMEOUT_SECONDS="${PHASE62_INTEGRATION_TIMEOUT_SECONDS:-900}"

# Optional deep-time Phase11 checkpoint.
# Set PHASE11_LONG_HORIZON_ENABLED=1 to include it in this gauntlet run.
PHASE11_LONG_HORIZON_ENABLED="${PHASE11_LONG_HORIZON_ENABLED:-0}"
PHASE11_LONG_HORIZON_MIN_LOOP_COUNT="${PHASE11_LONG_HORIZON_MIN_LOOP_COUNT:-50}"
PHASE11_LONG_HORIZON_MAX_LOOP_COUNT="${PHASE11_LONG_HORIZON_MAX_LOOP_COUNT:-500}"
PHASE11_LONG_HORIZON_LOOP_STEP="${PHASE11_LONG_HORIZON_LOOP_STEP:-50}"
PHASE11_LONG_HORIZON_CYCLES_PER_LOOP="${PHASE11_LONG_HORIZON_CYCLES_PER_LOOP:-4}"

# Schema-versioned regression detection.
# Set GORT_GAUNTLET_REGRESSION_BASELINE_DIR to enable diffing against a baseline.
GORT_GAUNTLET_REGRESSION_BASELINE_DIR="${GORT_GAUNTLET_REGRESSION_BASELINE_DIR:-}"
REGRESSION_BASELINE_PATH=""
REGRESSION_ENABLED="0"
if [[ -n "$GORT_GAUNTLET_REGRESSION_BASELINE_DIR" ]]; then
  mkdir -p "$GORT_GAUNTLET_REGRESSION_BASELINE_DIR"
  REGRESSION_BASELINE_PATH="$GORT_GAUNTLET_REGRESSION_BASELINE_DIR/baseline-latest.json"
  REGRESSION_ENABLED="1"
fi

# Schema evolution invariant checking.
# Set GORT_GAUNTLET_SCHEMA_EVOLUTION_CHECK=1 to validate backward compatibility rules.
GORT_GAUNTLET_SCHEMA_EVOLUTION_CHECK="${GORT_GAUNTLET_SCHEMA_EVOLUTION_CHECK:-0}"

labels=(
  "phase62_unit_core"
  "phase62_integration_suite"
  "phase80_unit_suite"
  "phase80_slice6_integration_quality"
  "phase90_geometry_integrity"
  "phase10_slice2_acceptance_gate"
  "phase10_closed_loop_integrity"
  "phase10_runtime_feedback_loop"
  "phase10_top_level_acceptance"
  "phase10_slice7_multicycle"
  "phase11_multi_loop_convergence"
  "phase10_runtime_adaptation"
  "phase80_runtime_gauntlet"
)

commands=(
  "cargo test phase62_structural_experiment -- --nocapture --test-threads=1"
  "timeout --signal=TERM --kill-after=30s ${PHASE62_INTEGRATION_TIMEOUT_SECONDS}s cargo test --test phase62_structural -- --nocapture"
  "cargo test phase80_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_slice6_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase9_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase10_slice2_acceptance_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase10_slice5_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase10_slice6_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase10_top_level_acceptance_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase10_slice7_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase11_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase10_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet -- --nocapture"
)

if [[ "$PHASE11_LONG_HORIZON_ENABLED" == "1" ]]; then
  labels+=("phase11_long_horizon_convergence")
  commands+=(
    "./scripts/phase11_long_horizon_harness.sh ${PHASE11_LONG_HORIZON_MIN_LOOP_COUNT} ${PHASE11_LONG_HORIZON_MAX_LOOP_COUNT} ${PHASE11_LONG_HORIZON_LOOP_STEP} ${PHASE11_LONG_HORIZON_CYCLES_PER_LOOP}"
  )
fi

if [[ "$GORT_GAUNTLET_SCHEMA_EVOLUTION_CHECK" == "1" ]]; then
  labels+=("schema_evolution_invariants")
  commands+=(
    "python3 \"$ROOT_DIR/scripts/validate-schema-evolution.py\" \"$ROOT_DIR/$JSON_SCHEMA_MANIFEST_PATH\""
  )
fi

results=()
overall="PASS"
drift_verdict_label="phase11_long_horizon_drift_verdict"
drift_verdict_result=""
drift_window_label="phase11_long_horizon_drift_window"
drift_window_result=""
regression_delta_result=""
regression_verdict_label="schema_regression_detection"
regression_verdict_result=""

for i in "${!labels[@]}"; do
  label="${labels[$i]}"
  cmd="${commands[$i]}"
  log_path="$OUT_DIR/${label}.log"

  echo "[gauntlet] running ${label}"
  if (
    cd "$ROOT_DIR"
    eval "$cmd"
  ) >"$log_path" 2>&1; then
    results+=("PASS")
  else
    results+=("FAIL")
    overall="FAIL"
  fi
done

if [[ "$PHASE11_LONG_HORIZON_ENABLED" == "1" ]]; then
  drift_log_path="$OUT_DIR/phase11_long_horizon_convergence.log"
  drift_detected=""
  first_drift_loop=""
  drift_loop_window=""

  if [[ -f "$drift_log_path" ]]; then
    drift_detected="$(grep -E '^drift_detected=' "$drift_log_path" | tail -n 1 | cut -d'=' -f2-)"
    first_drift_loop="$(grep -E '^first_drift_loop=' "$drift_log_path" | tail -n 1 | cut -d'=' -f2-)"
    drift_loop_window="$(grep -E '^loop_window=' "$drift_log_path" | tail -n 1 | cut -d'=' -f2-)"
  fi

  if [[ -n "$drift_loop_window" ]]; then
    drift_window_result="${drift_loop_window// step /@step}"
  else
    drift_window_result="unknown"
  fi

  if [[ "$drift_detected" == "false" ]]; then
    drift_verdict_result="PASS (no_drift)"
  elif [[ "$drift_detected" == "true" ]]; then
    if [[ -n "$first_drift_loop" ]] && [[ "$first_drift_loop" != "none" ]]; then
      drift_verdict_result="FAIL (first_loop=${first_drift_loop})"
    else
      drift_verdict_result="FAIL (drift_detected)"
    fi
    overall="FAIL"
  else
    drift_verdict_result="FAIL (drift_report_missing)"
    overall="FAIL"
  fi
fi

# Generate JSON summary first so regression detection can work
{
  echo "{";
  printf '  "$schema": "%s",\n' "$JSON_SCHEMA_URI";
  printf "  \"schema_version\": \"%s\",\n" "$GORT_GAUNTLET_JSON_SCHEMA_VERSION";
  printf "  \"effective_schema_uri\": \"%s\",\n" "$JSON_SCHEMA_URI";
  printf "  \"effective_schema_source\": \"%s\",\n" "$JSON_EFFECTIVE_SCHEMA_SOURCE";
  printf "  \"schema_manifest\": \"%s\",\n" "$JSON_SCHEMA_MANIFEST_PATH";
  printf "  \"manifest_signature\": \"%s\",\n" "$manifest_signature";
  echo "  \"checks\": [";
  for i in "${!labels[@]}"; do
    comma=",";
    if [[ "$i" -eq "$((${#labels[@]} - 1))" ]]; then
      comma="";
    fi
    printf "    {\"check\":\"%s\",\"result\":\"%s\"}%s\n" \
      "${labels[$i]}" "${results[$i]}" "$comma";
  done
  echo "  ],";
  printf "  \"overall\": \"%s\",\n" "$overall";
  printf "  \"phase62_integration_timeout_seconds\": %s,\n" "$PHASE62_INTEGRATION_TIMEOUT_SECONDS";
  if [[ "$PHASE11_LONG_HORIZON_ENABLED" == "1" ]]; then
    echo "  \"deep_time\": {";
    echo "    \"enabled\": true,";
    printf "    \"min_loop_count\": %s,\n" "$PHASE11_LONG_HORIZON_MIN_LOOP_COUNT";
    printf "    \"max_loop_count\": %s,\n" "$PHASE11_LONG_HORIZON_MAX_LOOP_COUNT";
    printf "    \"loop_step\": %s,\n" "$PHASE11_LONG_HORIZON_LOOP_STEP";
    printf "    \"cycles_per_loop\": %s,\n" "$PHASE11_LONG_HORIZON_CYCLES_PER_LOOP";
    printf "    \"drift_verdict\": \"%s\",\n" "$drift_verdict_result";
    printf "    \"drift_window\": \"%s\",\n" "$drift_window_result";
    echo "    \"drift_window_machine\": {";
    printf "      \"min_loop_count\": %s,\n" "$PHASE11_LONG_HORIZON_MIN_LOOP_COUNT";
    printf "      \"max_loop_count\": %s,\n" "$PHASE11_LONG_HORIZON_MAX_LOOP_COUNT";
    printf "      \"loop_step\": %s\n" "$PHASE11_LONG_HORIZON_LOOP_STEP";
    echo "    }";
    echo "  }";
  else
    echo "  \"deep_time\": {";
    echo "    \"enabled\": false";
    echo "  }";
  fi
  echo "}";
} >"$JSON_SUMMARY_PATH"

if [[ "$GORT_GAUNTLET_JSON_SELFTEST" == "1" ]]; then
  printf '\n{"json_selftest_corruption": ]\n' >>"$JSON_SUMMARY_PATH"
fi

[[ -s "$JSON_SUMMARY_PATH" ]] && python3 -m json.tool "$JSON_SUMMARY_PATH" >/dev/null || { echo "error: invalid or missing JSON summary at $JSON_SUMMARY_PATH" >&2; exit 1; }

# Schema-versioned regression detection (now with JSON available)
if [[ "$REGRESSION_ENABLED" == "1" ]]; then
  if [[ -f "$REGRESSION_BASELINE_PATH" ]]; then
    # Run regression detector and capture output to temp file
    # Note: script exits 1 if regression detected, 0 if no regression, so we don't check exit code
    regression_delta_file="$OUT_DIR/regression_delta.json"
    python3 "$ROOT_DIR/scripts/compute-regression-delta.py" "$JSON_SUMMARY_PATH" "$REGRESSION_BASELINE_PATH" >"$regression_delta_file" 2>/dev/null || true
    
    if [[ -f "$regression_delta_file" ]] && [[ -s "$regression_delta_file" ]]; then
      regression_detected="$(python3 -c "import json; d=json.load(open('$regression_delta_file')); print(d.get('regression_detected', False))" 2>/dev/null || echo 'unknown')"
      
      if [[ "$regression_detected" == "True" ]]; then
        regression_delta_result="FAIL (regression detected)"
        overall="FAIL"
      else
        regression_delta_result="PASS (no regression)"
      fi
      
      # Patch JSON with regression_delta using Python
      python3 - "$JSON_SUMMARY_PATH" "$regression_delta_file" <<'PATCH_PY'
import json
import sys

json_path = sys.argv[1]
delta_file = sys.argv[2]

try:
    with open(delta_file) as f:
        delta = json.load(f)
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['regression_delta'] = delta
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching regression_delta: {e}", file=sys.stderr)
    sys.exit(1)
PATCH_PY
    else
      regression_delta_result="SKIP (regression detection output not found)"
    fi
  else
    # First run with regression enabled; no baseline yet
    regression_delta_result="SKIP (baseline not found)"
    
    # Still patch with empty regression_delta for schema completeness
    python3 - "$JSON_SUMMARY_PATH" <<'INIT_REGR_PY'
import json
import sys

json_path = sys.argv[1]
try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['regression_delta'] = {
        "regression_detected": False,
        "regression_messages": ["baseline not found on first run"],
        "regression_fields": [],
        "baseline_schema_version": "none",
        "current_schema_version": doc.get("schema_version", "unknown")
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error initializing regression_delta: {e}", file=sys.stderr)
    sys.exit(1)
INIT_REGR_PY
  fi
fi

echo
echo "=== GORT PHASE62/PHASE80 GAUNTLET SUMMARY ==="
printf "%-32s | %s\n" "check" "result"
printf "%-32s-+-%s\n" "--------------------------------" "------"
for i in "${!labels[@]}"; do
  printf "%-32s | %s\n" "${labels[$i]}" "${results[$i]}"
done
if [[ -n "$drift_verdict_result" ]]; then
  printf "%-32s | %s\n" "$drift_verdict_label" "$drift_verdict_result"
  printf "%-32s | %s\n" "$drift_window_label" "$drift_window_result"
fi
if [[ -n "$schema_deprecation_warning_result" ]]; then
  printf "%-32s | %s\n" "$schema_deprecation_warning_label" "$schema_deprecation_warning_result"
fi
if [[ -n "$regression_delta_result" ]]; then
  printf "%-32s | %s\n" "$regression_verdict_label" "$regression_delta_result"
fi
printf "%-32s | %s\n" "overall" "$overall"

echo "logs_dir=$OUT_DIR"
echo "summary_json=$JSON_SUMMARY_PATH"
echo "=== END SUMMARY ==="

# Store baseline for next regression run
if [[ "$REGRESSION_ENABLED" == "1" ]] && [[ "$overall" == "PASS" ]]; then
  cp "$JSON_SUMMARY_PATH" "$REGRESSION_BASELINE_PATH"
fi

if [[ "$overall" != "PASS" ]]; then
  exit 1
fi
