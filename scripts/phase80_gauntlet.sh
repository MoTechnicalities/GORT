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
  "phase14_operator_family_invariants"
  "phase14_commutator_gate"
  "phase14_commutation_table_replay_gate"
  "phase14_clifford_family_gate"
  "phase15_two_qubit_state_invariants"
  "phase15_binding_classification_gate"
  "phase15_entangling_op_replay_gate"
  "phase15_swap_involutory_gate"
  "phase16_thought_path_invariants"
  "phase16_trajectory_classification_gate"
  "phase16_thought_path_replay_gate"
  "phase16_step_signature_chain_gate"
  "phase17_semantic_surface_invariants"
  "phase17_semantic_measurement_gate"
  "phase17_semantic_replay_gate"
  "phase18_resonance_field_invariants"
  "phase18_inference_gate"
  "phase18_replay_gate"
  "phase19_arbitration_field_invariants"
  "phase19_operator_selection_gate"
  "phase19_conflict_resolution_replay_gate"
  "phase20_drift_correction_invariants"
  "phase20_repair_gate"
  "phase20_stabilization_replay_gate"
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
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase14_operator_family_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase14_commutator_non_commutation_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase14_commutation_table_replay_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase14_clifford_family_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase15_two_qubit_state_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase15_binding_classification_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase15_entangling_op_replay_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase15_swap_involutory_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase16_thought_path_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase16_trajectory_classification_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase16_thought_path_replay_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase16_step_signatures_are_chained_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase17_semantic_surface_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase17_semantic_measurement_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase17_semantic_replay_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase18_resonance_field_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase18_inference_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase18_replay_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase19_arbitration_field_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase19_operator_selection_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase19_conflict_resolution_replay_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase20_drift_correction_invariants_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase20_repair_gate_ -- --nocapture"
  "cargo test --test phase80_runtime_gauntlet gauntlet_phase20_stabilization_replay_gate_ -- --nocapture"
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
# (Phase16 extraction inserted before JSON generation block)
phase13_top_level_label="phase13_top_level_summary"
phase13_top_level_result="FAIL"
phase14_top_level_label="phase14_top_level_summary"
phase14_top_level_result="FAIL"
phase15_top_level_label="phase15_top_level_summary"
phase15_top_level_result="FAIL"
phase16_top_level_label="phase16_top_level_summary"
phase16_top_level_result="FAIL"
phase17_top_level_label="phase17_top_level_summary"
phase17_top_level_result="FAIL"
phase18_top_level_label="phase18_top_level_summary"
phase18_top_level_result="FAIL"
phase19_top_level_label="phase19_top_level_summary"
phase19_top_level_result="FAIL"
phase20_top_level_label="phase20_top_level_summary"
phase20_top_level_result="FAIL"
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

# Extract Phase 14 telemetry from test log
phase14_verdict="unknown"
phase14_family_signature="unknown"
phase14_table_signature="unknown"
phase14_telemetry_digest="unknown"
phase14_replay_loops="0"
_p14_log="$OUT_DIR/phase14_commutation_table_replay_gate.log"
if [[ -f "$_p14_log" ]]; then
  _p14_line="$(grep -o 'PHASE14_SUMMARY:[^ ]*' "$_p14_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p14_line" ]]; then
    _p14_data="${_p14_line#PHASE14_SUMMARY:}"
    phase14_verdict="$(printf '%s' "$_p14_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase14_family_signature="$(printf '%s' "$_p14_data" | grep -o 'family_signature=[^|]*' | cut -d= -f2)"
    phase14_table_signature="$(printf '%s' "$_p14_data" | grep -o 'table_signature=[^|]*' | cut -d= -f2)"
    phase14_telemetry_digest="$(printf '%s' "$_p14_data" | grep -o 'telemetry_digest=[^|]*' | cut -d= -f2)"
    phase14_replay_loops="$(printf '%s' "$_p14_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase14_replay_loops" =~ ^[0-9]+$ ]]; then
  phase14_replay_loops="0"
fi

phase14_family_idx=-1
phase14_commutator_idx=-1
phase14_table_idx=-1
phase14_clifford_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase14_operator_family_invariants" ]]; then
    phase14_family_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase14_commutator_gate" ]]; then
    phase14_commutator_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase14_commutation_table_replay_gate" ]]; then
    phase14_table_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase14_clifford_family_gate" ]]; then
    phase14_clifford_idx="$i"
  fi
done

phase14_family_result="FAIL"
phase14_commutator_result="FAIL"
phase14_table_result="FAIL"
phase14_clifford_result="FAIL"
if [[ "$phase14_family_idx" -ge 0 ]]; then
  phase14_family_result="${results[$phase14_family_idx]}"
fi
if [[ "$phase14_commutator_idx" -ge 0 ]]; then
  phase14_commutator_result="${results[$phase14_commutator_idx]}"
fi
if [[ "$phase14_table_idx" -ge 0 ]]; then
  phase14_table_result="${results[$phase14_table_idx]}"
fi
if [[ "$phase14_clifford_idx" -ge 0 ]]; then
  phase14_clifford_result="${results[$phase14_clifford_idx]}"
fi

if [[ "$phase14_family_result" == "PASS" ]] \
  && [[ "$phase14_commutator_result" == "PASS" ]] \
  && [[ "$phase14_table_result" == "PASS" ]] \
  && [[ "$phase14_clifford_result" == "PASS" ]] \
  && [[ "$phase14_verdict" == "true" ]]; then
  phase14_top_level_result="PASS"
fi

# Extract Phase 15 telemetry from test log
phase15_verdict="unknown"
phase15_binding_signature="unknown"
phase15_sequence_signature="unknown"
phase15_telemetry_digest="unknown"
phase15_replay_loops="0"
_p15_log="$OUT_DIR/phase15_entangling_op_replay_gate.log"
if [[ -f "$_p15_log" ]]; then
  _p15_line="$(grep -o 'PHASE15_SUMMARY:[^ ]*' "$_p15_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p15_line" ]]; then
    _p15_data="${_p15_line#PHASE15_SUMMARY:}"
    phase15_verdict="$(printf '%s' "$_p15_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase15_binding_signature="$(printf '%s' "$_p15_data" | grep -o 'binding_signature=[^|]*' | cut -d= -f2)"
    phase15_sequence_signature="$(printf '%s' "$_p15_data" | grep -o 'sequence_signature=[^|]*' | cut -d= -f2)"
    phase15_telemetry_digest="$(printf '%s' "$_p15_data" | grep -o 'telemetry_digest=[^|]*' | cut -d= -f2)"
    phase15_replay_loops="$(printf '%s' "$_p15_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase15_replay_loops" =~ ^[0-9]+$ ]]; then
  phase15_replay_loops="0"
fi

phase15_state_idx=-1
phase15_classify_idx=-1
phase15_replay_idx=-1
phase15_swap_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase15_two_qubit_state_invariants" ]]; then
    phase15_state_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase15_binding_classification_gate" ]]; then
    phase15_classify_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase15_entangling_op_replay_gate" ]]; then
    phase15_replay_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase15_swap_involutory_gate" ]]; then
    phase15_swap_idx="$i"
  fi
done

phase15_state_result="FAIL"
phase15_classify_result="FAIL"
phase15_replay_result="FAIL"
phase15_swap_result="FAIL"
if [[ "$phase15_state_idx" -ge 0 ]]; then
  phase15_state_result="${results[$phase15_state_idx]}"
fi
if [[ "$phase15_classify_idx" -ge 0 ]]; then
  phase15_classify_result="${results[$phase15_classify_idx]}"
fi
if [[ "$phase15_replay_idx" -ge 0 ]]; then
  phase15_replay_result="${results[$phase15_replay_idx]}"
fi
if [[ "$phase15_swap_idx" -ge 0 ]]; then
  phase15_swap_result="${results[$phase15_swap_idx]}"
fi

if [[ "$phase15_state_result" == "PASS" ]] \
  && [[ "$phase15_classify_result" == "PASS" ]] \
  && [[ "$phase15_replay_result" == "PASS" ]] \
  && [[ "$phase15_swap_result" == "PASS" ]] \
  && [[ "$phase15_verdict" == "true" ]]; then
  phase15_top_level_result="PASS"
fi

# Extract Phase 16 telemetry from test log
phase16_verdict="unknown"
phase16_trajectory_signature="unknown"
phase16_telemetry_digest="unknown"
phase16_step_count="0"
phase16_replay_loops="0"
_p16_log="$OUT_DIR/phase16_thought_path_replay_gate.log"
if [[ -f "$_p16_log" ]]; then
  _p16_line="$(grep -o 'PHASE16_SUMMARY:[^ ]*' "$_p16_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p16_line" ]]; then
    _p16_data="${_p16_line#PHASE16_SUMMARY:}"
    phase16_verdict="$(printf '%s' "$_p16_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase16_trajectory_signature="$(printf '%s' "$_p16_data" | grep -o 'trajectory_signature=[^|]*' | cut -d= -f2)"
    phase16_telemetry_digest="$(printf '%s' "$_p16_data" | grep -o 'telemetry_digest=[^|]*' | cut -d= -f2)"
    phase16_step_count="$(printf '%s' "$_p16_data" | grep -o 'step_count=[^|]*' | cut -d= -f2)"
    phase16_replay_loops="$(printf '%s' "$_p16_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase16_step_count" =~ ^[0-9]+$ ]]; then
  phase16_step_count="0"
fi
if ! [[ "$phase16_replay_loops" =~ ^[0-9]+$ ]]; then
  phase16_replay_loops="0"
fi

phase16_state_idx=-1
phase16_classify_idx=-1
phase16_replay_idx=-1
phase16_chain_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase16_thought_path_invariants" ]]; then
    phase16_state_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase16_trajectory_classification_gate" ]]; then
    phase16_classify_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase16_thought_path_replay_gate" ]]; then
    phase16_replay_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase16_step_signature_chain_gate" ]]; then
    phase16_chain_idx="$i"
  fi
done

phase16_state_result="FAIL"
phase16_classify_result="FAIL"
phase16_replay_result="FAIL"
phase16_chain_result="FAIL"
if [[ "$phase16_state_idx" -ge 0 ]]; then
  phase16_state_result="${results[$phase16_state_idx]}"
fi
if [[ "$phase16_classify_idx" -ge 0 ]]; then
  phase16_classify_result="${results[$phase16_classify_idx]}"
fi
if [[ "$phase16_replay_idx" -ge 0 ]]; then
  phase16_replay_result="${results[$phase16_replay_idx]}"
fi
if [[ "$phase16_chain_idx" -ge 0 ]]; then
  phase16_chain_result="${results[$phase16_chain_idx]}"
fi

if [[ "$phase16_state_result" == "PASS" ]] \
  && [[ "$phase16_classify_result" == "PASS" ]] \
  && [[ "$phase16_replay_result" == "PASS" ]] \
  && [[ "$phase16_chain_result" == "PASS" ]] \
  && [[ "$phase16_verdict" == "true" ]]; then
  phase16_top_level_result="PASS"
fi

# Extract Phase 17 telemetry from test log
phase17_verdict="unknown"
phase17_semantic_signature="unknown"
phase17_semantic_digest="unknown"
phase17_replay_loops="0"
_p17_log="$OUT_DIR/phase17_semantic_replay_gate.log"
if [[ -f "$_p17_log" ]]; then
  _p17_line="$(grep -o 'PHASE17_SUMMARY:[^ ]*' "$_p17_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p17_line" ]]; then
    _p17_data="${_p17_line#PHASE17_SUMMARY:}"
    phase17_verdict="$(printf '%s' "$_p17_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase17_semantic_signature="$(printf '%s' "$_p17_data" | grep -o 'semantic_signature=[^|]*' | cut -d= -f2)"
    phase17_semantic_digest="$(printf '%s' "$_p17_data" | grep -o 'semantic_digest=[^|]*' | cut -d= -f2)"
    phase17_replay_loops="$(printf '%s' "$_p17_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase17_replay_loops" =~ ^[0-9]+$ ]]; then
  phase17_replay_loops="0"
fi

phase17_surface_idx=-1
phase17_measurement_idx=-1
phase17_replay_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase17_semantic_surface_invariants" ]]; then
    phase17_surface_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase17_semantic_measurement_gate" ]]; then
    phase17_measurement_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase17_semantic_replay_gate" ]]; then
    phase17_replay_idx="$i"
  fi
done

phase17_surface_result="FAIL"
phase17_measurement_result="FAIL"
phase17_replay_result="FAIL"
if [[ "$phase17_surface_idx" -ge 0 ]]; then
  phase17_surface_result="${results[$phase17_surface_idx]}"
fi
if [[ "$phase17_measurement_idx" -ge 0 ]]; then
  phase17_measurement_result="${results[$phase17_measurement_idx]}"
fi
if [[ "$phase17_replay_idx" -ge 0 ]]; then
  phase17_replay_result="${results[$phase17_replay_idx]}"
fi

if [[ "$phase17_surface_result" == "PASS" ]] \
  && [[ "$phase17_measurement_result" == "PASS" ]] \
  && [[ "$phase17_replay_result" == "PASS" ]] \
  && [[ "$phase17_verdict" == "true" ]]; then
  phase17_top_level_result="PASS"
fi

# Extract Phase 18 telemetry from test log
phase18_verdict="unknown"
phase18_resonance_signature="unknown"
phase18_inference_signature="unknown"
phase18_telemetry_digest="unknown"
phase18_replay_loops="0"
_p18_log="$OUT_DIR/phase18_replay_gate.log"
if [[ -f "$_p18_log" ]]; then
  _p18_line="$(grep -o 'PHASE18_SUMMARY:[^ ]*' "$_p18_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p18_line" ]]; then
    _p18_data="${_p18_line#PHASE18_SUMMARY:}"
    phase18_verdict="$(printf '%s' "$_p18_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase18_resonance_signature="$(printf '%s' "$_p18_data" | grep -o 'resonance_signature=[^|]*' | cut -d= -f2)"
    phase18_inference_signature="$(printf '%s' "$_p18_data" | grep -o 'inference_signature=[^|]*' | cut -d= -f2)"
    phase18_telemetry_digest="$(printf '%s' "$_p18_data" | grep -o 'telemetry_digest=[^|]*' | cut -d= -f2)"
    phase18_replay_loops="$(printf '%s' "$_p18_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase18_replay_loops" =~ ^[0-9]+$ ]]; then
  phase18_replay_loops="0"
fi

phase18_field_idx=-1
phase18_inference_idx=-1
phase18_replay_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase18_resonance_field_invariants" ]]; then
    phase18_field_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase18_inference_gate" ]]; then
    phase18_inference_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase18_replay_gate" ]]; then
    phase18_replay_idx="$i"
  fi
done

phase18_field_result="FAIL"
phase18_inference_result="FAIL"
phase18_replay_result="FAIL"
if [[ "$phase18_field_idx" -ge 0 ]]; then
  phase18_field_result="${results[$phase18_field_idx]}"
fi
if [[ "$phase18_inference_idx" -ge 0 ]]; then
  phase18_inference_result="${results[$phase18_inference_idx]}"
fi
if [[ "$phase18_replay_idx" -ge 0 ]]; then
  phase18_replay_result="${results[$phase18_replay_idx]}"
fi

if [[ "$phase18_field_result" == "PASS" ]] \
  && [[ "$phase18_inference_result" == "PASS" ]] \
  && [[ "$phase18_replay_result" == "PASS" ]] \
  && [[ "$phase18_verdict" == "true" ]]; then
  phase18_top_level_result="PASS"
fi

# Extract Phase 19 telemetry from test log
phase19_verdict="unknown"
phase19_arbitration_signature="unknown"
phase19_decision_signature="unknown"
phase19_telemetry_digest="unknown"
phase19_replay_loops="0"
_p19_log="$OUT_DIR/phase19_conflict_resolution_replay_gate.log"
if [[ -f "$_p19_log" ]]; then
  _p19_line="$(grep -o 'PHASE19_SUMMARY:[^ ]*' "$_p19_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p19_line" ]]; then
    _p19_data="${_p19_line#PHASE19_SUMMARY:}"
    phase19_verdict="$(printf '%s' "$_p19_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase19_arbitration_signature="$(printf '%s' "$_p19_data" | grep -o 'arbitration_signature=[^|]*' | cut -d= -f2)"
    phase19_decision_signature="$(printf '%s' "$_p19_data" | grep -o 'decision_signature=[^|]*' | cut -d= -f2)"
    phase19_telemetry_digest="$(printf '%s' "$_p19_data" | grep -o 'telemetry_digest=[^|]*' | cut -d= -f2)"
    phase19_replay_loops="$(printf '%s' "$_p19_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase19_replay_loops" =~ ^[0-9]+$ ]]; then
  phase19_replay_loops="0"
fi

phase19_field_idx=-1
phase19_selection_idx=-1
phase19_replay_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase19_arbitration_field_invariants" ]]; then
    phase19_field_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase19_operator_selection_gate" ]]; then
    phase19_selection_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase19_conflict_resolution_replay_gate" ]]; then
    phase19_replay_idx="$i"
  fi
done

phase19_field_result="FAIL"
phase19_selection_result="FAIL"
phase19_replay_result="FAIL"
if [[ "$phase19_field_idx" -ge 0 ]]; then
  phase19_field_result="${results[$phase19_field_idx]}"
fi
if [[ "$phase19_selection_idx" -ge 0 ]]; then
  phase19_selection_result="${results[$phase19_selection_idx]}"
fi
if [[ "$phase19_replay_idx" -ge 0 ]]; then
  phase19_replay_result="${results[$phase19_replay_idx]}"
fi

if [[ "$phase19_field_result" == "PASS" ]] \
  && [[ "$phase19_selection_result" == "PASS" ]] \
  && [[ "$phase19_replay_result" == "PASS" ]] \
  && [[ "$phase19_verdict" == "true" ]]; then
  phase19_top_level_result="PASS"
fi

# Extract Phase 20 telemetry from test log
phase20_verdict="unknown"
phase20_correction_signature="unknown"
phase20_stabilization_signature="unknown"
phase20_telemetry_digest="unknown"
phase20_replay_loops="0"
_p20_log="$OUT_DIR/phase20_stabilization_replay_gate.log"
if [[ -f "$_p20_log" ]]; then
  _p20_line="$(grep -o 'PHASE20_SUMMARY:[^ ]*' "$_p20_log" 2>/dev/null | head -n 1 || true)"
  if [[ -n "$_p20_line" ]]; then
    _p20_data="${_p20_line#PHASE20_SUMMARY:}"
    phase20_verdict="$(printf '%s' "$_p20_data" | grep -o 'verdict=[^|]*' | cut -d= -f2)"
    phase20_correction_signature="$(printf '%s' "$_p20_data" | grep -o 'correction_signature=[^|]*' | cut -d= -f2)"
    phase20_stabilization_signature="$(printf '%s' "$_p20_data" | grep -o 'stabilization_signature=[^|]*' | cut -d= -f2)"
    phase20_telemetry_digest="$(printf '%s' "$_p20_data" | grep -o 'telemetry_digest=[^|]*' | cut -d= -f2)"
    phase20_replay_loops="$(printf '%s' "$_p20_data" | grep -o 'replay_loops=[^|]*' | cut -d= -f2)"
  fi
fi

if ! [[ "$phase20_replay_loops" =~ ^[0-9]+$ ]]; then
  phase20_replay_loops="0"
fi

phase20_drift_idx=-1
phase20_repair_idx=-1
phase20_replay_idx=-1
for i in "${!labels[@]}"; do
  if [[ "${labels[$i]}" == "phase20_drift_correction_invariants" ]]; then
    phase20_drift_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase20_repair_gate" ]]; then
    phase20_repair_idx="$i"
  fi
  if [[ "${labels[$i]}" == "phase20_stabilization_replay_gate" ]]; then
    phase20_replay_idx="$i"
  fi
done

phase20_drift_result="FAIL"
phase20_repair_result="FAIL"
phase20_replay_result="FAIL"
if [[ "$phase20_drift_idx" -ge 0 ]]; then
  phase20_drift_result="${results[$phase20_drift_idx]}"
fi
if [[ "$phase20_repair_idx" -ge 0 ]]; then
  phase20_repair_result="${results[$phase20_repair_idx]}"
fi
if [[ "$phase20_replay_idx" -ge 0 ]]; then
  phase20_replay_result="${results[$phase20_replay_idx]}"
fi

if [[ "$phase20_drift_result" == "PASS" ]] \
  && [[ "$phase20_repair_result" == "PASS" ]] \
  && [[ "$phase20_replay_result" == "PASS" ]] \
  && [[ "$phase20_verdict" == "true" ]]; then
  phase20_top_level_result="PASS"
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

# Patch phase14 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase14_verdict" "$phase14_family_signature" "$phase14_table_signature" "$phase14_telemetry_digest" "$phase14_replay_loops" <<'PHASE14_PATCH_PY'
import json, sys

json_path, verdict, family_sig, table_sig, tel_digest, replay_loops = sys.argv[1:7]

try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['phase14'] = {
        "phase14_verdict": verdict,
        "phase14_family_signature": family_sig,
        "phase14_table_signature": table_sig,
        "phase14_telemetry_digest": tel_digest,
        "phase14_replay_loops": int(replay_loops),
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching phase14: {e}", file=sys.stderr)
    sys.exit(1)
PHASE14_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase14 JSON patch failed" >&2
  exit 1
fi

# Patch phase15 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase15_verdict" "$phase15_binding_signature" "$phase15_sequence_signature" "$phase15_telemetry_digest" "$phase15_replay_loops" <<'PHASE15_PATCH_PY'
import json, sys

json_path, verdict, binding_sig, seq_sig, tel_digest, replay_loops = sys.argv[1:7]

try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['phase15'] = {
        "phase15_verdict": verdict,
        "phase15_binding_signature": binding_sig,
        "phase15_sequence_signature": seq_sig,
        "phase15_telemetry_digest": tel_digest,
        "phase15_replay_loops": int(replay_loops),
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching phase15: {e}", file=sys.stderr)
    sys.exit(1)
PHASE15_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase15 JSON patch failed" >&2
  exit 1
fi

# Patch phase16 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase16_verdict" "$phase16_trajectory_signature" "$phase16_telemetry_digest" "$phase16_step_count" "$phase16_replay_loops" <<'PHASE16_PATCH_PY'
import json, sys
json_path, verdict, traj_sig, tel_digest, step_count, replay_loops = sys.argv[1:7]
try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['phase16'] = {
        "phase16_verdict": verdict,
        "phase16_trajectory_signature": traj_sig,
        "phase16_telemetry_digest": tel_digest,
        "phase16_step_count": int(step_count),
        "phase16_replay_loops": int(replay_loops),
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching phase16: {e}", file=sys.stderr)
    sys.exit(1)
PHASE16_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase16 JSON patch failed" >&2
  exit 1
fi

# Patch phase17 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase17_verdict" "$phase17_semantic_signature" "$phase17_semantic_digest" "$phase17_replay_loops" <<'PHASE17_PATCH_PY'
import json, sys

json_path, verdict, semantic_signature, semantic_digest, replay_loops = sys.argv[1:6]

try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['phase17'] = {
        "phase17_verdict": verdict,
        "semantic_signature": semantic_signature,
        "semantic_digest": semantic_digest,
        "replay_loops": int(replay_loops),
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching phase17: {e}", file=sys.stderr)
    sys.exit(1)
PHASE17_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase17 JSON patch failed" >&2
  exit 1
fi

# Patch phase18 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase18_verdict" "$phase18_resonance_signature" "$phase18_inference_signature" "$phase18_telemetry_digest" "$phase18_replay_loops" <<'PHASE18_PATCH_PY'
import json, sys

json_path, verdict, resonance_signature, inference_signature, telemetry_digest, replay_loops = sys.argv[1:7]

try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['phase18'] = {
        "phase18_verdict": verdict,
        "resonance_signature": resonance_signature,
        "inference_signature": inference_signature,
        "telemetry_digest": telemetry_digest,
        "replay_loops": int(replay_loops),
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching phase18: {e}", file=sys.stderr)
    sys.exit(1)
PHASE18_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase18 JSON patch failed" >&2
  exit 1
fi

# Patch phase19 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase19_verdict" "$phase19_arbitration_signature" "$phase19_decision_signature" "$phase19_telemetry_digest" "$phase19_replay_loops" <<'PHASE19_PATCH_PY'
import json, sys

json_path, verdict, arbitration_signature, decision_signature, telemetry_digest, replay_loops = sys.argv[1:7]

try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['phase19'] = {
        "phase19_verdict": verdict,
        "arbitration_signature": arbitration_signature,
        "decision_signature": decision_signature,
        "telemetry_digest": telemetry_digest,
        "replay_loops": int(replay_loops),
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching phase19: {e}", file=sys.stderr)
    sys.exit(1)
PHASE19_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase19 JSON patch failed" >&2
  exit 1
fi

# Patch phase20 fields into JSON summary
python3 - "$JSON_SUMMARY_PATH" "$phase20_verdict" "$phase20_correction_signature" "$phase20_stabilization_signature" "$phase20_telemetry_digest" "$phase20_replay_loops" <<'PHASE20_PATCH_PY'
import json, sys

json_path, verdict, correction_signature, stabilization_signature, telemetry_digest, replay_loops = sys.argv[1:7]

try:
    with open(json_path, 'r') as f:
        doc = json.load(f)
    doc['phase20'] = {
        "phase20_verdict": verdict,
        "correction_signature": correction_signature,
        "stabilization_signature": stabilization_signature,
        "telemetry_digest": telemetry_digest,
        "replay_loops": int(replay_loops),
    }
    with open(json_path, 'w') as f:
        json.dump(doc, f, indent=2)
except Exception as e:
    print(f"error patching phase20: {e}", file=sys.stderr)
    sys.exit(1)
PHASE20_PATCH_PY
if [[ $? -ne 0 ]]; then
  echo "error: phase20 JSON patch failed" >&2
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
printf "%-32s | %s\n" "$phase14_top_level_label" "$phase14_top_level_result"
printf "%-32s | %s\n" "$phase15_top_level_label" "$phase15_top_level_result"
printf "%-32s | %s\n" "$phase16_top_level_label" "$phase16_top_level_result"
printf "%-32s | %s\n" "$phase17_top_level_label" "$phase17_top_level_result"
printf "%-32s | %s\n" "$phase18_top_level_label" "$phase18_top_level_result"
printf "%-32s | %s\n" "$phase19_top_level_label" "$phase19_top_level_result"
printf "%-32s | %s\n" "$phase20_top_level_label" "$phase20_top_level_result"
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
