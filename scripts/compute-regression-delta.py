#!/usr/bin/env python3
"""
Regression detection for phase80-gauntlet summaries.
Compares current summary against baseline to detect convergence regressions.
Respects schema version for field comparison.
"""
import json
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple, Optional

def load_summary(path: str) -> Optional[Dict[str, Any]]:
    """Load a JSON summary file; return None if missing or invalid."""
    try:
        with open(path) as f:
            return json.load(f)
    except (FileNotFoundError, json.JSONDecodeError):
        return None

def compare_check_arrays(
    current_checks: List[Dict],
    baseline_checks: List[Dict]
) -> Tuple[bool, List[str]]:
    """
    Compare check arrays and detect regressions.
    Return (has_regression, regression_messages).
    Regressions: PASS -> FAIL, or check missing in current.
    """
    has_regression = False
    messages = []
    
    baseline_by_name = {c["check"]: c["result"] for c in baseline_checks}
    current_by_name = {c["check"]: c["result"] for c in current_checks}
    
    # Detect PASS->FAIL transitions
    for check_name, baseline_result in baseline_by_name.items():
        current_result = current_by_name.get(check_name, "MISSING")
        if baseline_result == "PASS" and current_result != "PASS":
            messages.append(f"{check_name}: {baseline_result} → {current_result}")
            has_regression = True
    
    # Detect missing checks
    for check_name in current_by_name:
        if check_name not in baseline_by_name:
            messages.append(f"{check_name}: (new check)")
    
    return has_regression, messages

def compare_drift_verdicts(
    current_deep_time: Dict,
    baseline_deep_time: Dict
) -> Tuple[bool, List[str]]:
    """
    Compare drift verdicts and detect regressions.
    Regression: drift_verdict PASS -> FAIL, or first_drift_loop changed.
    """
    has_regression = False
    messages = []
    
    # Both disabled is OK
    if not current_deep_time.get("enabled") and not baseline_deep_time.get("enabled"):
        return False, []
    
    # Compare drift verdicts
    current_verdict = current_deep_time.get("drift_verdict", "UNKNOWN")
    baseline_verdict = baseline_deep_time.get("drift_verdict", "UNKNOWN")
    
    if baseline_verdict == "PASS" and current_verdict != "PASS":
        messages.append(f"drift_verdict: {baseline_verdict} → {current_verdict}")
        has_regression = True
    
    # If baseline had no drift but current does, flag regression
    if baseline_verdict == "PASS" and current_verdict == "FAIL":
        current_first = current_deep_time.get("first_drift_loop", "?")
        messages.append(f"convergence regressed: first drift at loop {current_first}")
        has_regression = True
    
    return has_regression, messages

def compare_phase12(
    current_p12: Dict[str, Any],
    baseline_p12: Dict[str, Any],
) -> Tuple[bool, List[str]]:
    """
    Compare Phase 12 emergent-program fields for drift.
    Regressions: verdict true→non-true, signature_hash changed,
    telemetry_digest changed.
    Informational (reported but not a regression): operator_plan_size,
    resonance_gate changes.
    """
    if not baseline_p12 or not current_p12:
        return False, []

    has_regression = False
    messages: List[str] = []

    # Verdict: true -> anything else is a hard failure.
    baseline_verdict = baseline_p12.get("phase12_verdict", "")
    current_verdict = current_p12.get("phase12_verdict", "")
    if baseline_verdict == "true" and current_verdict != "true":
        messages.append(
            f"phase12_verdict: {baseline_verdict} → {current_verdict}"
        )
        has_regression = True

    # Signature hash drift: program structure changed.
    baseline_sig = baseline_p12.get("phase12_signature_hash", "")
    current_sig = current_p12.get("phase12_signature_hash", "")
    if baseline_sig and current_sig and baseline_sig != current_sig:
        messages.append(
            f"phase12_signature_hash: {baseline_sig} → {current_sig}"
        )
        has_regression = True

    # Telemetry digest drift: telemetry output changed.
    baseline_tel = baseline_p12.get("phase12_telemetry_digest", "")
    current_tel = current_p12.get("phase12_telemetry_digest", "")
    if baseline_tel and current_tel and baseline_tel != current_tel:
        messages.append(
            f"phase12_telemetry_digest: {baseline_tel} → {current_tel}"
        )
        has_regression = True

    # Informational: operator plan size change.
    baseline_ops = baseline_p12.get("phase12_operator_plan_size", "")
    current_ops = current_p12.get("phase12_operator_plan_size", "")
    if baseline_ops and current_ops and baseline_ops != current_ops:
        messages.append(
            f"phase12_operator_plan_size (informational): {baseline_ops} → {current_ops}"
        )

    # Informational: resonance gate change.
    baseline_gate = baseline_p12.get("phase12_resonance_gate", "")
    current_gate = current_p12.get("phase12_resonance_gate", "")
    if baseline_gate and current_gate and baseline_gate != current_gate:
        messages.append(
            f"phase12_resonance_gate (informational): {baseline_gate} → {current_gate}"
        )

    return has_regression, messages


def compute_regression_delta(
    current_summary: Dict[str, Any],
    baseline_summary: Dict[str, Any]
) -> Dict[str, Any]:
    """
    Compute regression delta between current and baseline summaries.
    Return structured regression result with verdict, messages, and fields.
    """
    result = {
        "regression_detected": False,
        "regression_messages": [],
        "regression_fields": [],
        "baseline_schema_version": baseline_summary.get("schema_version", "unknown"),
        "current_schema_version": current_summary.get("schema_version", "unknown"),
    }
    
    # Schema compatibility check
    baseline_version = baseline_summary.get("schema_version", "")
    current_version = current_summary.get("schema_version", "")
    if baseline_version != current_version:
        result["regression_messages"].append(
            f"schema version mismatch: baseline={baseline_version}, current={current_version}"
        )
        result["regression_fields"].append("schema_version")
    
    # Compare overall result
    baseline_overall = baseline_summary.get("overall", "UNKNOWN")
    current_overall = current_summary.get("overall", "UNKNOWN")
    if baseline_overall == "PASS" and current_overall != "PASS":
        result["regression_detected"] = True
        result["regression_messages"].append(f"overall: {baseline_overall} → {current_overall}")
        result["regression_fields"].append("overall")
    
    # Compare checks array
    baseline_checks = baseline_summary.get("checks", [])
    current_checks = current_summary.get("checks", [])
    if baseline_checks and current_checks:
        checks_regressed, check_messages = compare_check_arrays(current_checks, baseline_checks)
        if checks_regressed:
            result["regression_detected"] = True
            result["regression_messages"].extend(check_messages)
            result["regression_fields"].append("checks")
    
    # Compare deep_time (phase11 convergence)
    baseline_deep_time = baseline_summary.get("deep_time", {})
    current_deep_time = current_summary.get("deep_time", {})
    if baseline_deep_time or current_deep_time:
        deep_time_regressed, deep_time_messages = compare_drift_verdicts(
            current_deep_time, baseline_deep_time
        )
        if deep_time_regressed:
            result["regression_detected"] = True
            result["regression_messages"].extend(deep_time_messages)
            result["regression_fields"].append("deep_time")

    # Compare phase12 emergent-program fields (long-horizon drift detection)
    baseline_p12 = baseline_summary.get("phase12", {})
    current_p12 = current_summary.get("phase12", {})
    if baseline_p12 or current_p12:
        p12_regressed, p12_messages = compare_phase12(current_p12, baseline_p12)
        if p12_messages:
            result["regression_messages"].extend(p12_messages)
        if p12_regressed:
            result["regression_detected"] = True
            result["regression_fields"].append("phase12")

    return result

def main():
    """
    Usage: compute-regression-delta.py <current.json> <baseline.json>
    Outputs regression delta as JSON to stdout.
    Exit 0 if no regression, 1 if regression detected.
    """
    if len(sys.argv) < 3:
        print("Usage: compute-regression-delta.py <current.json> <baseline.json>", file=sys.stderr)
        sys.exit(2)
    
    current_path = sys.argv[1]
    baseline_path = sys.argv[2]
    
    current_summary = load_summary(current_path)
    baseline_summary = load_summary(baseline_path)
    
    if not baseline_summary:
        # No baseline yet; not a regression, just initialization
        result = {
            "regression_detected": False,
            "regression_messages": ["baseline not found"],
            "regression_fields": [],
            "baseline_schema_version": "none",
            "current_schema_version": current_summary.get("schema_version", "unknown") if current_summary else "unknown",
        }
        print(json.dumps(result))
        sys.exit(0)
    
    if not current_summary:
        # Current invalid; this is an error state
        result = {
            "regression_detected": True,
            "regression_messages": ["current summary invalid or not found"],
            "regression_fields": [],
            "baseline_schema_version": baseline_summary.get("schema_version", "unknown"),
            "current_schema_version": "invalid",
        }
        print(json.dumps(result))
        sys.exit(1)
    
    delta = compute_regression_delta(current_summary, baseline_summary)
    print(json.dumps(delta))
    
    sys.exit(1 if delta["regression_detected"] else 0)

if __name__ == "__main__":
    main()
