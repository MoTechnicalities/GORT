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
  "phase12_manifest_invariants"
  "phase12_emergent_programs_gate"
  "phase13_qubit_state_invariants"
  "phase13_unitary_invariants"
  "phase13_evolution_replay_gate"
  "phase13_measurement_contract_gate"
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
  "python3 \"$ROOT_DIR/scripts/validate-phase12-invariants.py\" \"$ROOT_DIR/$JSON_SCHEMA_MANIFEST_PATH\""
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase12_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase13_qubit_state_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase13_unitary_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase13_evolution_replay_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase13_measurement_contract_gate_ -- --nocapture"
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
phase12_drift_window_label="phase12_drift_window"
phase12_drift_window_result=""
phase12_top_level_label="phase12_top_level_summary"
phase12_top_level_result="FAIL"
phase13_top_level_label="phase13_top_level_summary"
phase13_top_level_result="FAIL"
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

# Extract Phase 12 telemetry from test log
phase12_verdict="unknown"
phase12_signature_hash="unknown"
phase12_operator_plan_size="unknown"
phase12_resonance_gate="unknown"
phase12_telemetry_digest="unknown"
phase12_replay_loops="0"
_p12_log="$OUT_DIR/phase12_emergent_programs_gate.log"
if [[ -f "$_p12_log" ]]; then
  _p12_line="$(grep -o 'PHASE12_SUMMARY:[^ ]*' "$_p12_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p12_line" ]]; then
    _p12_data="${_p12_line#PHASE12_SUMMARY:}"
    phase12_verdict="$(printf '%s' "$_p12_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase12_signature_hash="$(printf '%s' "$_p12_data" | grep -o 'signature_hash=[^|]*' | cut -d= -f2)"
    phase12_operator_plan_size="$(printf '%s' "$_p12_data" | grep -o 'operator_plan_size=[^|]*' | cut -d= -f2)"
    phase12_resonance_gate="$(printf '%s' "$_p12_data" | grep -o 'resonance_gate=[^|]*' | cut -d= -f2)"
    phase12_telemetry_digest="$(printf '%s' "$_p12_data" | grep -o 'telemetry_digest=[^|]*' | cut -d= -f2)"
    phase12_replay_loops="$(printf '%s' "$_p12_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase12_replay_loops" =~ ^[0-9]+$ ]]; then
  phase12_replay_loops="0"
fi

phase12_drift_window_enabled="false"
phase12_baseline_available="false"
phase12_signature_drift_detected="false"
phase12_telemetry_drift_detected="false"
phase12_drift_detected="false"
phase12_baseline_signature_hash="none"
phase12_baseline_telemetry_digest="none"
phase12_current_signature_hash="$phase12_signature_hash"
phase12_current_telemetry_digest="$phase12_telemetry_digest"
phase12_drift_verdict="SKIP (phase12 summary missing)"
phase12_drift_window="run@loops=${phase12_replay_loops}"

if [[ "$phase12_verdict" != "unknown" ]]; then
  phase12_drift_window_enabled="true"
  phase12_drift_verdict="SKIP (regression baseline disabled)"

  if [[ "$REGRESSION_ENABLED" == "1" ]]; then
    if [[ -f "$REGRESSION_BASELINE_PATH" ]]; then
      _baseline_phase12="$(python3 - "$REGRESSION_BASELINE_PATH" <<'P12_BASE_PY'
import json
import sys

path = sys.argv[1]
try:
    with open(path, "r", encoding="utf-8") as handle:
        doc = json.load(handle)
    phase12 = doc.get("phase12", {})
    print("|".join([
        "true",
        str(phase12.get("phase12_signature_hash", "none")),
        str(phase12.get("phase12_telemetry_digest", "none")),
    ]))
except Exception:
    print("false|none|none")
P12_BASE_PY
      )"
      phase12_baseline_available="$(printf '%s' "$_baseline_phase12" | cut -d'|' -f1)"
      phase12_baseline_signature_hash="$(printf '%s' "$_baseline_phase12" | cut -d'|' -f2)"
      phase12_baseline_telemetry_digest="$(printf '%s' "$_baseline_phase12" | cut -d'|' -f3)"

      if [[ "$phase12_baseline_available" == "true" ]]; then
        if [[ "$phase12_baseline_signature_hash" != "$phase12_signature_hash" ]]; then
          phase12_signature_drift_detected="true"
        fi
        if [[ "$phase12_baseline_telemetry_digest" != "$phase12_telemetry_digest" ]]; then
          phase12_telemetry_drift_detected="true"
        fi

        if [[ "$phase12_signature_drift_detected" == "true" ]] || [[ "$phase12_telemetry_drift_detected" == "true" ]] || [[ "$phase12_verdict" != "true" ]]; then
          phase12_drift_detected="true"
          phase12_drift_verdict="FAIL (phase12 drift detected)"
          phase12_drift_window_result="$phase12_drift_verdict"
          overall="FAIL"
        else
          phase12_drift_verdict="PASS (no_drift)"
        fi
      else
        phase12_drift_verdict="SKIP (baseline phase12 missing)"
      fi
    else
      phase12_drift_verdict="SKIP (baseline not found)"
    fi
  fi
fi

if [[ -z "$phase12_drift_window_result" ]]; then
  phase12_drift_window_result="$phase12_drift_verdict"
fi

phase12_gate_idx=-1
phase12_manifest_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase12_emergent_programs_gate" ]]; then
    phase12_gate_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase12_manifest_invariants" ]]; then
    phase12_manifest_idx="$i"
  fi
done

phase12_gate_result="FAIL"
phase12_manifest_result="FAIL"
if [[ "$phase12_gate_idx" -ge 0 ]]; then
  phase12_gate_result="${results[$phase12_gate_idx]}"
fi
if [[ "$phase12_manifest_idx" -ge 0 ]]; then
  phase12_manifest_result="${results[$phase12_manifest_idx]}"
fi

if [[ "$phase12_gate_result" == "PASS" ]] \
  && [[ "$phase12_manifest_result" == "PASS" ]] \
  && [[ "$phase12_verdict" == "true" ]] \
  && [[ "$phase12_drift_verdict" != FAIL* ]]; then
  phase12_top_level_result="PASS"
fi

# Extract Phase 13 telemetry from test log
phase13_verdict="unknown"
phase13_state_signature="unknown"
phase13_evolution_signature="unknown"
phase13_measurement_signature="unknown"
phase13_replay_loops="0"
_p13_log="$OUT_DIR/phase13_evolution_replay_gate.log"
if [[ -f "$_p13_log" ]]; then
  _p13_line="$(grep -o 'PHASE13_SUMMARY:[^ ]*' "$_p13_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p13_line" ]]; then
    _p13_data="${_p13_line#PHASE13_SUMMARY:}"
    phase13_verdict="$(printf '%s' "$_p13_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase13_state_signature="$(printf '%s' "$_p13_data" | grep -o 'state_signature=[^|]*' | cut -d= -f2)"
    phase13_evolution_signature="$(printf '%s' "$_p13_data" | grep -o 'evolution_signature=[^|]*' | cut -d= -f2)"
    phase13_measurement_signature="$(printf '%s' "$_p13_data" | grep -o 'measurement_signature=[^|]*' | cut -d= -f2)"
    phase13_replay_loops="$(printf '%s' "$_p13_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase13_replay_loops" =~ ^[0-9]+$ ]]; then
  phase13_replay_loops="0"
fi

phase13_state_idx=-1
phase13_unitary_idx=-1
phase13_evolution_idx=-1
phase13_measurement_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase13_qubit_state_invariants" ]]; then
    phase13_state_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase13_unitary_invariants" ]]; then
    phase13_unitary_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase13_evolution_replay_gate" ]]; then
    phase13_evolution_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase13_measurement_contract_gate" ]]; then
    phase13_measurement_idx="$i"
  fi
done

phase13_state_result="FAIL"
phase13_unitary_result="FAIL"
phase13_evolution_result="FAIL"
phase13_measurement_result="FAIL"
if [[ "$phase13_state_idx" -ge 0 ]]; then
  phase13_state_result="${results[$phase13_state_idx]}"
fi
if [[ "$phase13_unitary_idx" -ge 0 ]]; then
  phase13_unitary_result="${results[$phase13_unitary_idx]}"
fi
if [[ "$phase13_evolution_idx" -ge 0 ]]; then
  phase13_evolution_result="${results[$phase13_evolution_idx]}"
fi
if [[ "$phase13_measurement_idx" -ge 0 ]]; then
  phase13_measurement_result="${results[$phase13_measurement_idx]}"
fi

if [[ "$phase13_state_result" == "PASS" ]] \
  && [[ "$phase13_unitary_result" == "PASS" ]] \
  && [[ "$phase13_evolution_result" == "PASS" ]] \
  && [[ "$phase13_measurement_result" == "PASS" ]] \
  && [[ "$phase13_verdict" == "true" ]]; then
  phase13_top_level_result="PASS"
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

# Patch phase12 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase12_verdict" "$phase12_signature_hash" "$phase12_operator_plan_size" "$phase12_resonance_gate" "$phase12_telemetry_digest" "$phase12_drift_window_enabled" "$phase12_replay_loops" "$phase12_baseline_available" "$phase12_drift_detected" "$phase12_drift_verdict" "$phase12_drift_window" "$phase12_signature_drift_detected" "$phase12_telemetry_drift_detected" "$phase12_baseline_signature_hash" "$phase12_current_signature_hash" "$phase12_baseline_telemetry_digest" "$phase12_current_telemetry_digest" <<'PHASE12_PATCH_PY'
import json, sys
(
  json_path,
  verdict,
  sig_hash,
  plan_size,
  resonance_gate,
  tel_digest,
  drift_enabled,
  replay_loops,
  baseline_available,
  drift_detected,
  drift_verdict,
  drift_window,
  signature_drift_detected,
  telemetry_drift_detected,
  baseline_signature_hash,
  current_signature_hash,
  baseline_telemetry_digest,
  current_telemetry_digest,
) = sys.argv[1:19]

def as_bool(value):
  return str(value).lower() == "true"

try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['phase12'] = {
        "phase12_verdict": verdict,
        "phase12_signature_hash": sig_hash,
        "phase12_operator_plan_size": plan_size,
        "phase12_resonance_gate": resonance_gate,
        "phase12_telemetry_digest": tel_digest,
    }
    doc['phase12_drift_window'] = {
        "enabled": as_bool(drift_enabled),
        "replay_loop_count": int(replay_loops),
        "baseline_available": as_bool(baseline_available),
        "drift_detected": as_bool(drift_detected),
        "drift_verdict": drift_verdict,
        "drift_window": drift_window,
        "signature_drift_detected": as_bool(signature_drift_detected),
        "telemetry_drift_detected": as_bool(telemetry_drift_detected),
        "baseline_signature_hash": baseline_signature_hash,
        "current_signature_hash": current_signature_hash,
        "baseline_telemetry_digest": baseline_telemetry_digest,
        "current_telemetry_digest": current_telemetry_digest,
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching phase12: {e}", file=sys.stderr)
    sys.exit(1)
PHASE12_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase12 JSON patch failed" >&2
  exit 1
fi

# Patch phase13 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase13_verdict" "$phase13_state_signature" "$phase13_evolution_signature" "$phase13_measurement_signature" "$phase13_replay_loops" <<'PHASE13_PATCH_PY'
import json, sys

json_path, verdict, state_signature, evolution_signature, measurement_signature, replay_loops = sys.argv[1:7]

try:
  with open(json_path, 'r') as f:
    doc = json.load(f)
  doc['phase13'] = {
    "phase13_verdict": verdict,
    "phase13_state_signature": state_signature,
    "phase13_evolution_signature": evolution_signature,
    "phase13_measurement_signature": measurement_signature,
    "phase13_replay_loops": int(replay_loops),
  }
  with open(json_path, 'w') as f:
    json.dump(doc, f, indent=2)
except Exception as e:
  print(f"error patching phase13: {e}", file=sys.stderr)
  sys.exit(1)
PHASE13_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase13 JSON patch failed" >&2
  exit 1
fi

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
if [[ -n "$phase12_drift_window_result" ]]; then
  printf "%-32s | %s\n" "$phase12_drift_window_label" "$phase12_drift_window_result"
fi
printf "%-32s | %s\n" "$phase12_top_level_label" "$phase12_top_level_result"
printf "%-32s | %s\n" "$phase13_top_level_label" "$phase13_top_level_result"
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
