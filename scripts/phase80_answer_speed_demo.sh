#!/usr/bin/env bash
set -u -o pipefail

# Lightweight cognition-cycle speed demonstration.
#
# This benchmark is intentionally small: it measures a single deterministic
# cognition pipeline path (phase2_pipeline example) and summarizes per-cycle
# wall time. This is the right comparison point for "answer a question"
# behavior, not the full stress gauntlet.
#
# Usage:
#   scripts/phase80_answer_speed_demo.sh [runs]
#
# Optional:
#   SPEED_DEMO_WITH_GAUNTLET=1 scripts/phase80_answer_speed_demo.sh [runs]
#   (Adds one timed gauntlet run as a heavy-reference comparison.)

RUNS="${1:-10}"
if ! [[ "$RUNS" =~ ^[0-9]+$ ]] || [[ "$RUNS" -lt 1 ]]; then
  echo "error: runs must be a positive integer" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="/tmp/gort_phase80_answer_speed_demo"
mkdir -p "$OUT_DIR"

TIMINGS_FILE="$OUT_DIR/cycle_ms.txt"
: > "$TIMINGS_FILE"

pushd "$ROOT_DIR" >/dev/null

# Build once so loop timings focus on runtime, not compilation.
echo "[speed-demo] building example binary"
if ! cargo build --example phase2_pipeline >"$OUT_DIR/build.log" 2>&1; then
  echo "error: failed to build example; see $OUT_DIR/build.log" >&2
  popd >/dev/null
  exit 1
fi

BIN="target/debug/examples/phase2_pipeline"
if [[ ! -x "$BIN" ]]; then
  echo "error: expected binary not found: $BIN" >&2
  popd >/dev/null
  exit 1
fi

# Warm-up pass (excluded from summary) to reduce one-off startup effects.
echo "[speed-demo] warm-up run"
"$BIN" >"$OUT_DIR/warmup.log" 2>&1

echo "[speed-demo] measuring $RUNS cognition cycles"
for i in $(seq 1 "$RUNS"); do
  start_ns="$(date +%s%N)"
  "$BIN" >"$OUT_DIR/run_${i}.log" 2>&1
  end_ns="$(date +%s%N)"

  elapsed_ms="$(awk -v s="$start_ns" -v e="$end_ns" 'BEGIN { printf "%.3f", (e - s) / 1000000.0 }')"
  echo "$elapsed_ms" >> "$TIMINGS_FILE"
  printf '[speed-demo] run %d/%d: %s ms\n' "$i" "$RUNS" "$elapsed_ms"
done

sorted_file="$OUT_DIR/cycle_ms.sorted.txt"
sort -n "$TIMINGS_FILE" > "$sorted_file"

stats="$(awk '
  {
    vals[NR] = $1
    sum += $1
  }
  END {
    if (NR == 0) {
      print "count=0 mean=0 min=0 max=0 median=0 p95=0"
      exit
    }

    count = NR
    min = vals[1]
    max = vals[count]
    mean = sum / count

    if (count % 2 == 1) {
      median = vals[(count + 1) / 2]
    } else {
      median = (vals[count / 2] + vals[(count / 2) + 1]) / 2
    }

    p95_index = int((count * 95 + 99) / 100)
    if (p95_index < 1) p95_index = 1
    if (p95_index > count) p95_index = count
    p95 = vals[p95_index]

    printf "count=%d mean=%.3f min=%.3f max=%.3f median=%.3f p95=%.3f", count, mean, min, max, median, p95
  }
' "$sorted_file")"

eval "$stats"
cycles_per_sec="$(awk -v m="$median" 'BEGIN { if (m > 0) printf "%.2f", 1000.0 / m; else printf "0.00" }')"

gauntlet_seconds="N/A"
ratio="N/A"
if [[ "${SPEED_DEMO_WITH_GAUNTLET:-0}" == "1" ]]; then
  echo "[speed-demo] timing heavy-reference gauntlet run"
  g_start="$(date +%s%N)"
  if ./scripts/phase80_gauntlet.sh >"$OUT_DIR/gauntlet.log" 2>&1; then
    g_end="$(date +%s%N)"
    gauntlet_seconds="$(awk -v s="$g_start" -v e="$g_end" 'BEGIN { printf "%.3f", (e - s) / 1000000000.0 }')"
    ratio="$(awk -v gs="$gauntlet_seconds" -v mm="$median" 'BEGIN { if (mm > 0) printf "%.1f", (gs * 1000.0) / mm; else printf "N/A" }')"
  else
    gauntlet_seconds="FAIL"
    ratio="N/A"
  fi
fi

echo
echo "=== GORT ANSWER-SPEED DEMO SUMMARY ==="
printf "%-34s | %s\n" "runs_requested" "$RUNS"
printf "%-34s | %s\n" "runs_measured" "$count"
printf "%-34s | %s\n" "cycle_min_ms" "$min"
printf "%-34s | %s\n" "cycle_median_ms" "$median"
printf "%-34s | %s\n" "cycle_p95_ms" "$p95"
printf "%-34s | %s\n" "cycle_mean_ms" "$mean"
printf "%-34s | %s\n" "cycle_max_ms" "$max"
printf "%-34s | %s\n" "est_cycles_per_second" "$cycles_per_sec"
printf "%-34s | %s\n" "gauntlet_seconds(optional)" "$gauntlet_seconds"
printf "%-34s | %s\n" "gauntlet_to_cycle_ratio(optional)" "$ratio"
echo "artifacts_dir=$OUT_DIR"
echo "=== END SUMMARY ==="

popd >/dev/null
