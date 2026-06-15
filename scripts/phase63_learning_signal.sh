#!/usr/bin/env bash
set -euo pipefail

# Deterministic Phase63 learning-signal extractor.
#
# Produces:
#  1) per-snapshot CSV with canonical + realized deltas
#  2) per-plan summary counts for success/neutral/harmful in closure-deficit regime
#
# Usage:
#   scripts/phase63_learning_signal.sh [runs]
#
# Defaults:
#   runs=5

RUNS="${1:-5}"
if ! [[ "$RUNS" =~ ^[0-9]+$ ]] || [[ "$RUNS" -lt 1 ]]; then
  echo "error: runs must be a positive integer" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="/tmp/gort_phase63_learning"
mkdir -p "$OUT_DIR"

CSV_PATH="$OUT_DIR/phase63_learning_signal.csv"
SUMMARY_PATH="$OUT_DIR/phase63_plan_training_summary.csv"

cat > "$CSV_PATH" <<'CSV'
run_index,holdout_id,canonical_regime,canonical_plan,continuity_delta,region_delta,anchor_delta,realized_continuity_delta,realized_region_delta,realized_anchor_delta,training_signal
CSV

for i in $(seq 1 "$RUNS"); do
  LOG_PATH="$OUT_DIR/phase63_run_${i}.log"
  run_exit=0
  (
    cd "$ROOT_DIR"
    GORT_PHASE62_KIND=phase63 cargo run --quiet --example curriculum_harness > "$LOG_PATH"
  ) || run_exit=$?

  if [[ "$run_exit" -ne 0 ]]; then
    echo "warning: run ${i} exited with code ${run_exit}; extracting snapshots from log anyway" >&2
  fi

  awk -v run_index="$i" '
    function process_record(rec,   n, fields, j, key, value, holdout, regime, plan, plan_csv, continuity, region, anchor, realized_continuity, realized_region, realized_anchor, signal, rc) {
      holdout=""; regime=""; plan="";
      continuity=""; region=""; anchor="";
      realized_continuity=""; realized_region=""; realized_anchor="";

      n = split(rec, fields, " ");
      for (j = 1; j <= n; j++) {
        split(fields[j], kv, "=");
        key = kv[1];
        value = substr(fields[j], length(key) + 2);
        if (key == "holdout_id") holdout = value;
        else if (key == "canonical_regime") regime = value;
        else if (key == "canonical_plan") plan = value;
        else if (key == "continuity_delta") continuity = value;
        else if (key == "region_delta") region = value;
        else if (key == "anchor_delta") anchor = value;
        else if (key == "realized_continuity_delta") realized_continuity = value;
        else if (key == "realized_region_delta") realized_region = value;
        else if (key == "realized_anchor_delta") realized_anchor = value;
      }

      signal = "non_closure_deficit";
      if (regime == "closure_deficit") {
        rc = realized_continuity + 0;
        if (rc > 0) signal = "success";
        else if (rc == 0) signal = "neutral";
        else signal = "harmful";
      }

      if (holdout != "" && regime != "" && plan != "") {
        plan_csv = plan;
        gsub(/,/, ";", plan_csv);
        printf "%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s\n", \
          run_index, holdout, regime, plan_csv, continuity, region, anchor, \
          realized_continuity, realized_region, realized_anchor, signal;
      }
    }

    {
      if ($0 ~ /phase63_canonical_realized_snapshot/) {
        rec = $0;
      } else if (rec != "") {
        rec = rec " " $0;
      }

      if (rec != "" && rec ~ /final_hash=/) {
        process_record(rec);
        rec = "";
      }
    }
  ' "$LOG_PATH" >> "$CSV_PATH"
done

awk -F',' '
  NR == 1 { next }
  $3 != "closure_deficit" { next }
  {
    plan = $4;
    signal = $11;
    key = plan;
    total[key]++;
    if (signal == "success") success[key]++;
    else if (signal == "neutral") neutral[key]++;
    else if (signal == "harmful") harmful[key]++;
  }
  END {
    print "canonical_plan,success_count,neutral_count,harmful_count,total_count,success_rate";
    for (k in total) {
      s = success[k] + 0;
      n = neutral[k] + 0;
      h = harmful[k] + 0;
      t = total[k] + 0;
      rate = (t > 0) ? s / t : 0;
      printf "%s,%d,%d,%d,%d,%.6f\n", k, s, n, h, t, rate;
    }
  }
' "$CSV_PATH" | sort > "$SUMMARY_PATH"

echo "wrote: $CSV_PATH"
echo "wrote: $SUMMARY_PATH"
echo
echo "top summary rows:"
sed -n '1,12p' "$SUMMARY_PATH"