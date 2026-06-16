#!/usr/bin/env python3
"""Validate the Phase 12 manifest invariants used by the gauntlet runner."""

from __future__ import annotations

import json
import sys
from pathlib import Path


EXPECTED_GATE_NAME = "phase12_emergent_cognitive_program"
EXPECTED_ACCEPTANCE_RULES = [
    "manifold.manifold_well_formed must be true before synthesis",
    "plan.operator_plan_well_formed must be true before synthesis",
    "manifold.manifold_signature must match plan.manifold_signature",
    "plan.operators must be non-empty",
    "every synthesized step must have a non-empty step_kind and positive continuity_guard_percent and manifold_alignment_percent",
    "program.step_count must be greater than zero",
    "program.resonance_gate_percent must be between 1 and 100",
    "program.program_well_formed must be true",
    "telemetry must be derived from synthesized program fields only",
]
EXPECTED_DETERMINISM_RULES = [
    "identical manifold and plan inputs must synthesize the same program_signature and program_profile_hash",
    "telemetry emission must be replay-stable for identical program inputs",
]
EXPECTED_DRIFT_WINDOW_INVARIANTS = [
    "phase12_drift_window must be emitted in gauntlet summary when phase12 data is present",
    "phase12_drift_window.replay_loop_count must be positive",
    "phase12_drift_window must record baseline_available and drift_detected flags",
    "phase12_drift_window must include current signature hash and telemetry digest",
    "when baseline is available, phase12_drift_window must include baseline signature hash and telemetry digest",
]


def fail(message: str) -> int:
    print(f"error: {message}", file=sys.stderr)
    return 1


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: validate-phase12-invariants.py <manifest-path>")

    manifest_path = Path(sys.argv[1])
    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        return fail(f"manifest not found: {manifest_path}")
    except json.JSONDecodeError as exc:
        return fail(f"manifest is not valid JSON: {exc}")

    invariants = manifest.get("phase12_invariants")
    if not isinstance(invariants, dict):
        return fail("phase12_invariants block is missing or malformed")

    gate_name = invariants.get("gate_name")
    if gate_name != EXPECTED_GATE_NAME:
        return fail(
            f"gate_name must be {EXPECTED_GATE_NAME!r}, got {gate_name!r}"
        )

    acceptance_rules = invariants.get("acceptance_rules")
    if acceptance_rules != EXPECTED_ACCEPTANCE_RULES:
        return fail("acceptance_rules do not match the expected Phase 12 acceptance rules")

    determinism_rules = invariants.get("determinism_rules")
    if determinism_rules != EXPECTED_DETERMINISM_RULES:
        return fail("determinism_rules do not match the expected Phase 12 determinism rules")

    drift_window_invariants = invariants.get("drift_window_invariants")
    if drift_window_invariants != EXPECTED_DRIFT_WINDOW_INVARIANTS:
        return fail("drift_window_invariants do not match the expected Phase 12 drift-window invariants")

    print("phase12_invariants | PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())