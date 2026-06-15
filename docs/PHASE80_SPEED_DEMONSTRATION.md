# Phase80 Speed Demonstration

Date: 2026-06-15

## Purpose

Document a small, reproducible speed comparison between:

- Lightweight answer-like cognition cycle (single deterministic pipeline execution)
- Heavy verification path (Phase80 stress harness / gauntlet)

This is intended to clarify why stress audits can be slow while normal cognition cycles remain fast.

## Method

### Lightweight cycle benchmark

Command:

```bash
./scripts/phase80_answer_speed_demo.sh 10
```

Benchmark path:

- Builds and runs `target/debug/examples/phase2_pipeline`
- Uses one warm-up run (excluded)
- Measures wall-clock time for each cycle
- Reports min/median/p95/mean/max and estimated cycles per second

Observed summary:

```text
=== GORT ANSWER-SPEED DEMO SUMMARY ===
runs_requested                     | 10
runs_measured                      | 10
cycle_min_ms                       | 2.255
cycle_median_ms                    | 2.913
cycle_p95_ms                       | 3.499
cycle_mean_ms                      | 2.960
cycle_max_ms                       | 3.499
est_cycles_per_second              | 343.29
gauntlet_seconds(optional)         | N/A
gauntlet_to_cycle_ratio(optional)  | N/A
artifacts_dir=/tmp/gort_phase80_answer_speed_demo
=== END SUMMARY ===
```

### Heavy stress verification reference

Command:

```bash
./scripts/phase80_stress_harness.sh 2
```

Observed summary:

```text
=== GORT PHASE80 STRESS HARNESS SUMMARY ===
runs_requested                   | 2
runs_completed                   | 2
gauntlet_pass_count              | 2
gauntlet_fail_count              | 0
signature_stable                 | PASS
overall                          | PASS
artifacts_dir=/tmp/gort_phase80_stress_harness
=== END SUMMARY ===
STRESS_EXIT_CODE=0
```

## Comparative interpretation

- Lightweight answer-like cycles are low single-digit milliseconds (median ~2.9 ms in this run).
- Stress harness runs execute full Phase62/Phase80 verification, including long integration checks and replay-signature validation.
- The two paths are intentionally different workloads:
  - Lightweight path measures normal deterministic cognition-cycle execution.
  - Stress path measures deterministic correctness under heavy audit conditions.

## Reproduce

From repository root:

```bash
# 1) Lightweight answer-like cycle timing
./scripts/phase80_answer_speed_demo.sh 10

# 2) Stress verification timing and replay stability
./scripts/phase80_stress_harness.sh 2
```

Artifacts:

- `/tmp/gort_phase80_answer_speed_demo`
- `/tmp/gort_phase80_stress_harness`
