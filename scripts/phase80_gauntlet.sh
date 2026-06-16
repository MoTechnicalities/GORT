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

# Hard timeout for the longest integration stage to prevent indefinite hangs.
PHASE62_INTEGRATION_TIMEOUT_SECONDS="${PHASE62_INTEGRATION_TIMEOUT_SECONDS:-900}"

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

results=()
overall="PASS"

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

echo
echo "=== GORT PHASE62/PHASE80 GAUNTLET SUMMARY ==="
printf "%-32s | %s\n" "check" "result"
printf "%-32s-+-%s\n" "--------------------------------" "------"
for i in "${!labels[@]}"; do
  printf "%-32s | %s\n" "${labels[$i]}" "${results[$i]}"
done
printf "%-32s | %s\n" "overall" "$overall"
echo "logs_dir=$OUT_DIR"
echo "=== END SUMMARY ==="

if [[ "$overall" != "PASS" ]]; then
  exit 1
fi
