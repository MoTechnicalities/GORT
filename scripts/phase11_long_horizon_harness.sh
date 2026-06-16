#!/usr/bin/env bash
set -u -o pipefail

# Phase11 long-horizon convergence harness.
#
# Executes the long-horizon convergence runner over a loop range and emits
# a deterministic summary block for deep-time geometry observation.
#
# Usage:
#   scripts/phase11_long_horizon_harness.sh [min_loop_count] [max_loop_count] [loop_step] [cycle_count_per_loop]
#
# Defaults:
#   min_loop_count=50
#   max_loop_count=500
#   loop_step=50
#   cycle_count_per_loop=4

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="/tmp/gort_phase11_long_horizon"
mkdir -p "$OUT_DIR"

MIN_LOOP_COUNT="${1:-50}"
MAX_LOOP_COUNT="${2:-500}"
LOOP_STEP="${3:-50}"
CYCLE_COUNT_PER_LOOP="${4:-4}"

for value in "$MIN_LOOP_COUNT" "$MAX_LOOP_COUNT" "$LOOP_STEP" "$CYCLE_COUNT_PER_LOOP"; do
  if ! [[ "$value" =~ ^[0-9]+$ ]]; then
    echo "error: all arguments must be non-negative integers" >&2
    exit 2
  fi
done

LOG_PATH="$OUT_DIR/phase11_long_horizon.log"

if (
  cd "$ROOT_DIR"
  cargo run --example phase11_long_horizon -- \
    "$MIN_LOOP_COUNT" "$MAX_LOOP_COUNT" "$LOOP_STEP" "$CYCLE_COUNT_PER_LOOP"
) | tee "$LOG_PATH"; then
  echo "status=PASS"
  echo "log=$LOG_PATH"
  exit 0
fi

echo "status=FAIL"
echo "log=$LOG_PATH"
exit 1
