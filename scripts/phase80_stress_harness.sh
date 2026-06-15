#!/usr/bin/env bash
set -u -o pipefail

# Phase80 stress harness runner.
#
# Repeats the Phase62/Phase80 gauntlet N times, verifies every run passes,
# and checks that canonicalized runtime-gauntlet signatures remain stable.
#
# Usage:
#   scripts/phase80_stress_harness.sh [runs]
#
# Defaults:
#   runs=20

RUNS="${1:-20}"
if ! [[ "$RUNS" =~ ^[0-9]+$ ]] || [[ "$RUNS" -lt 1 ]]; then
  echo "error: runs must be a positive integer" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="/tmp/gort_phase80_stress_harness"
mkdir -p "$OUT_DIR"

pass_count=0
fail_count=0
completed=0
baseline_signature_hash=""
signature_stable="PASS"

for i in $(seq 1 "$RUNS"); do
  run_dir="$OUT_DIR/run_${i}"
  mkdir -p "$run_dir"

  echo "[stress] run ${i}/${RUNS}: gauntlet"
  if (
    cd "$ROOT_DIR"
    ./scripts/phase80_gauntlet.sh
  ) >"$run_dir/gauntlet.log" 2>&1; then
    pass_count=$((pass_count + 1))
  else
    fail_count=$((fail_count + 1))
  fi

  completed=$((completed + 1))

  runtime_log="/tmp/gort_phase80_gauntlet/phase80_runtime_gauntlet.log"
  if [[ -f "$runtime_log" ]]; then
    awk '
      /^test gauntlet_/ { print }
      /^test result:/ {
        line = $0
        sub(/finished in .*/, "finished in <duration>", line)
        print line
      }
    ' "$runtime_log" > "$run_dir/runtime_signature.txt"

    sig_hash="$(sha256sum "$run_dir/runtime_signature.txt" | awk '{print $1}')"
    echo "$sig_hash" > "$run_dir/runtime_signature.sha256"

    if [[ -z "$baseline_signature_hash" ]]; then
      baseline_signature_hash="$sig_hash"
    elif [[ "$sig_hash" != "$baseline_signature_hash" ]]; then
      signature_stable="FAIL"
    fi
  else
    signature_stable="FAIL"
  fi

done

overall="PASS"
if [[ "$fail_count" -ne 0 ]] || [[ "$signature_stable" != "PASS" ]]; then
  overall="FAIL"
fi

echo
echo "=== GORT PHASE80 STRESS HARNESS SUMMARY ==="
printf "%-32s | %s\n" "runs_requested" "$RUNS"
printf "%-32s | %s\n" "runs_completed" "$completed"
printf "%-32s | %s\n" "gauntlet_pass_count" "$pass_count"
printf "%-32s | %s\n" "gauntlet_fail_count" "$fail_count"
printf "%-32s | %s\n" "signature_stable" "$signature_stable"
printf "%-32s | %s\n" "overall" "$overall"
echo "artifacts_dir=$OUT_DIR"
echo "=== END SUMMARY ==="

if [[ "$overall" != "PASS" ]]; then
  exit 1
fi
