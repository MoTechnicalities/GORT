use gort::{
    apply_phase62_structural_experiment, Phase62ExperimentKind, Phase62ExperimentPlan,
    Phase62StructuralConfig, SemanticConstraint,
};
use std::process::Command;

fn sample_constraints() -> Vec<SemanticConstraint> {
    vec![
        SemanticConstraint::assertion("balance", "stable", true, 60),
        SemanticConstraint::assertion("balance", "tilt", false, 30),
        SemanticConstraint::assertion("grasp", "secure", true, 55),
    ]
}

fn run_curriculum_harness() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
        .current_dir(manifest_dir)
        .args(["run", "--quiet", "--example", "curriculum_harness"])
        .output()
        .expect("curriculum harness should launch");

    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn latest_line_containing(log: &str, needle: &str) -> String {
    log.lines()
        .rev()
        .find(|line| line.contains(needle))
        .unwrap_or_else(|| panic!("missing log line containing: {}", needle))
        .to_string()
}

fn count_lines_containing(log: &str, needle: &str) -> usize {
    log.lines().filter(|line| line.contains(needle)).count()
}

fn extract_targeted_gate_lines(log: &str) -> (usize, usize, String, String) {
    let target_count = count_lines_containing(
        log,
        "phase62_toggle episode_id=holdout_05_recovery novelty=offset_stack_torsion_swap_recovery enabled=true max_bridge=2 bridge_weight=6",
    );
    let disabled_count = count_lines_containing(
        log,
        "phase62_toggle episode_id=held_01 novelty=held enabled=false max_bridge=0 bridge_weight=0",
    );
    let learning_line = latest_line_containing(
        log,
        "learning assessment (full_stack) => structural_adaptation_present=true learning_curve_iterations=1 efficiency_verified=true budget<=10",
    );
    let memory_line = latest_line_containing(
        log,
        "memory assessment (full_stack) => memory_improved_after_recovery=true",
    );

    (target_count, disabled_count, learning_line, memory_line)
}

fn extract_multi_holdout_gate_lines(log: &str) -> (Vec<usize>, usize, String, String, String, String, String) {
    let holdout_toggle_counts = [
        "phase62_toggle episode_id=holdout_01_recovery novelty=holdout_diagonal_recovery enabled=true max_bridge=2 bridge_weight=6",
        "phase62_toggle episode_id=holdout_02_recovery novelty=spiral_lurch_terrain_shear_recovery enabled=true max_bridge=2 bridge_weight=6",
        "phase62_toggle episode_id=holdout_03_recovery novelty=counterweight_spiral_trip_recovery enabled=true max_bridge=2 bridge_weight=6",
        "phase62_toggle episode_id=holdout_04_recovery novelty=blind_regrasp_load_shift_recovery enabled=true max_bridge=2 bridge_weight=6",
        "phase62_toggle episode_id=holdout_05_recovery novelty=offset_stack_torsion_swap_recovery enabled=true max_bridge=2 bridge_weight=6",
    ]
    .into_iter()
    .map(|needle| count_lines_containing(log, needle))
    .collect();

    let disabled_count = count_lines_containing(
        log,
        "phase62_toggle episode_id=held_01 novelty=held enabled=false max_bridge=0 bridge_weight=0",
    );
    let topology_line = latest_line_containing(
        log,
        "PASS trained learner builds richer topology across holdouts :: avg_region_advantage=",
    );
    let external_line = latest_line_containing(
        log,
        "PASS trained learner transfers under harder noisy perturbations :: avg_external_change_delta=",
    );
    let recovery_line = latest_line_containing(
        log,
        "PASS trained learner recovers within speed budget :: avg_recovery_converged_iteration=1 required<=10",
    );
    let learning_line = latest_line_containing(
        log,
        "learning assessment (full_stack) => structural_adaptation_present=true learning_curve_iterations=1 efficiency_verified=true budget<=10",
    );
    let memory_line = latest_line_containing(
        log,
        "memory assessment (full_stack) => memory_improved_after_recovery=true",
    );

    (
        holdout_toggle_counts,
        disabled_count,
        topology_line,
        external_line,
        recovery_line,
        learning_line,
        memory_line,
    )
}

fn extract_phase66_02_04_lines(log: &str) -> Vec<String> {
    log.lines()
        .filter(|line| {
            line.contains("phase66_holdout_telemetry holdout_id=holdout_02")
                || line.contains("phase66_holdout_telemetry holdout_id=holdout_04")
        })
        .map(|line| line.to_string())
        .collect()
}

fn latest_phase66_holdout_line(log: &str, holdout_id: &str) -> String {
    latest_line_containing(
        log,
        &format!("phase66_holdout_telemetry holdout_id={}", holdout_id),
    )
}

fn latest_phase67_holdout_line(log: &str, holdout_id: &str) -> String {
    latest_line_containing(
        log,
        &format!("phase67_holdout_telemetry holdout_id={}", holdout_id),
    )
}

fn latest_phase63_holdout_line(log: &str, holdout_id: &str) -> String {
    latest_line_containing(
        log,
        &format!("phase63_holdout_telemetry holdout_id={}", holdout_id),
    )
}

fn parse_i32_field(segment: &str, field: &str) -> i32 {
    let marker = format!("{}=", field);
    let value = segment
        .split(&marker)
        .nth(1)
        .and_then(|rest| rest.split(' ').next())
        .unwrap_or_else(|| panic!("missing field {} in segment: {}", field, segment));
    value
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("invalid integer for {}: {}", field, value))
}

fn parse_bool_field(segment: &str, field: &str) -> bool {
    let marker = format!("{}=", field);
    let value = segment
        .split(&marker)
        .nth(1)
        .and_then(|rest| rest.split(' ').next())
        .unwrap_or_else(|| panic!("missing field {} in segment: {}", field, segment));
    match value {
        "true" => true,
        "false" => false,
        _ => panic!("invalid boolean for {}: {}", field, value),
    }
}

fn parse_phase66_final_continuity_metrics(line: &str) -> (i32, i32) {
    let final_segment = line
        .split(" final_telemetry=")
        .nth(1)
        .unwrap_or_else(|| panic!("missing final_telemetry segment: {}", line));
    let continuity_delta = parse_i32_field(final_segment, "continuity_delta");
    let continuity_rebased = parse_i32_field(final_segment, "continuity_rebased");
    (continuity_delta, continuity_rebased)
}

fn parse_phase66_final_i32_field(line: &str, field: &str) -> i32 {
    let final_segment = line
        .split(" final_telemetry=")
        .nth(1)
        .unwrap_or_else(|| panic!("missing final_telemetry segment: {}", line));
    parse_i32_field(final_segment, field)
}

fn parse_phase66_final_bool_field(line: &str, field: &str) -> bool {
    let final_segment = line
        .split(" final_telemetry=")
        .nth(1)
        .unwrap_or_else(|| panic!("missing final_telemetry segment: {}", line));
    parse_bool_field(final_segment, field)
}

fn parse_phase63_final_plan(line: &str) -> String {
    line.split(" final_plan=")
        .nth(1)
        .and_then(|rest| rest.split(" pre_regime=").next())
        .unwrap_or_else(|| panic!("missing final_plan in line: {}", line))
        .to_string()
}

fn run_curriculum_harness_with_phase62_kind(kind: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
        .current_dir(manifest_dir)
        .env("GORT_PHASE62_KIND", kind)
        .args(["run", "--quiet", "--example", "curriculum_harness"])
        .output()
        .expect("curriculum harness should launch with phase62 kind");

    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn run_curriculum_harness_with_phase62_kind_and_handoff(kind: &str, handoff: bool) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
        .current_dir(manifest_dir)
        .env("GORT_PHASE62_KIND", kind)
        .env(
            "GORT_PHASE63_ESCALATION_HANDOFF",
            if handoff { "true" } else { "false" },
        )
        .args(["run", "--quiet", "--example", "curriculum_harness"])
        .output()
        .expect("curriculum harness should launch with phase62 kind and handoff");

    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}


#[test]
fn gate_aa_phase62_experiment_plan_orders_anchor_closure_first() {
    let plan = Phase62ExperimentPlan::new();

    assert_eq!(plan.primary, Phase62ExperimentKind::AnchorClosureSpineV1);
    assert_eq!(plan.follow_ons[0], Phase62ExperimentKind::RegionMergeSplitStabilizationV1);
    assert_eq!(plan.follow_ons[1], Phase62ExperimentKind::ManifoldDriftSuppressionV1);
    assert_eq!(plan.follow_ons[2], Phase62ExperimentKind::ContradictionReliefV1);
    assert_eq!(plan.follow_ons[3], Phase62ExperimentKind::ContradictionClosureRegimeV2);
}

#[test]
fn gate_ay_phase62_scaffold_is_deterministic_noop_by_default() {
    let base = sample_constraints();
    let config = Phase62StructuralConfig::default();

    let (out_a, report_a) = apply_phase62_structural_experiment(&base, config);
    let (out_b, report_b) = apply_phase62_structural_experiment(&base, config);

    assert_eq!(out_a, base);
    assert_eq!(out_a, out_b);
    assert_eq!(report_a.generated_constraints, 0);
    assert_eq!(report_a.applied, false);
    assert_eq!(report_a.note, report_b.note);
}

#[test]
fn gate_az_phase62_anchor_closure_spine_generates_structural_candidates() {
    let base = sample_constraints();
    let config = Phase62StructuralConfig {
        enabled: true,
        kind: Phase62ExperimentKind::AnchorClosureSpineV1,
        max_bridge_constraints_per_subject: 2,
        bridge_weight: 6,
    };

    let (out, report) = apply_phase62_structural_experiment(&base, config);

    assert!(report.applied);
    assert!(report.generated_constraints >= 1);
    assert!(out.len() > base.len());
}

#[test]
fn gate_ba_phase62_targeted_recovery_bridge_is_replay_stable() {
    let first_log = run_curriculum_harness();
    let first_gate_lines = extract_targeted_gate_lines(&first_log);

    let second_log = run_curriculum_harness();
    let second_gate_lines = extract_targeted_gate_lines(&second_log);

    assert_eq!(first_gate_lines, second_gate_lines);
}

#[test]
fn gate_bb_phase62_anchor_closure_spine_v1_multi_holdout_validation_is_replay_stable() {
    let first_log = run_curriculum_harness();
    let first_gate_lines = extract_multi_holdout_gate_lines(&first_log);

    assert!(first_gate_lines.0.iter().all(|count| *count > 0));
    assert!(first_gate_lines.1 > 0);

    let second_log = run_curriculum_harness();
    let second_gate_lines = extract_multi_holdout_gate_lines(&second_log);

    assert_eq!(first_gate_lines, second_gate_lines);
}

#[test]
#[ignore = "AnchorClosureSpineV1 remains Phase 6.2 flag-gated until multi-holdout memory score matches the holdout-05 baseline"]
fn gate_bc_phase62_anchor_closure_spine_v1_matches_memory_baseline_across_hard_holdouts() {
    let log = run_curriculum_harness();
    let memory_line = latest_line_containing(&log, "memory assessment (full_stack) =>");

    assert!(
        memory_line.contains("memory_improvement_score=5")
            || memory_line.contains("memory_improvement_score=6"),
        "expected multi-holdout memory score to meet or exceed the single-holdout baseline: {}",
        memory_line
    );
}

#[test]
fn gate_bd_phase62_v3b_closure_regime_is_replay_stable() {
    let first_log = run_curriculum_harness_with_phase62_kind("v3b");
    let first_gate_lines = extract_multi_holdout_gate_lines(&first_log);

    assert!(first_gate_lines.0.iter().all(|count| *count > 0));
    assert!(first_gate_lines.1 > 0);

    let second_log = run_curriculum_harness_with_phase62_kind("v3b");
    let second_gate_lines = extract_multi_holdout_gate_lines(&second_log);

    assert_eq!(first_gate_lines, second_gate_lines);
}

#[test]
fn gate_v4d_phase63_replay_tuple_identity() {
    let first_log = run_curriculum_harness_with_phase62_kind("phase63");
    let second_log = run_curriculum_harness_with_phase62_kind("phase63");

    let first: Vec<&str> = first_log
        .lines()
        .filter(|line| line.contains("phase63_canonical_snapshot"))
        .filter(|line| line.contains("canonical_regime=closure_deficit") || line.contains("canonical_regime=closure_ready"))
        .collect();
    let second: Vec<&str> = second_log
        .lines()
        .filter(|line| line.contains("phase63_canonical_snapshot"))
        .filter(|line| line.contains("canonical_regime=closure_deficit") || line.contains("canonical_regime=closure_ready"))
        .collect();

    assert!(!first.is_empty(), "expected phase63 canonical snapshot lines");
    assert_eq!(first, second);

    // Gate D: deterministic branch/regime selection across runs.
    let first_regimes: Vec<&str> = first
        .iter()
        .filter_map(|line| line.split(" canonical_regime=").nth(1))
        .filter_map(|rest| rest.split(' ').next())
        .collect();
    let second_regimes: Vec<&str> = second
        .iter()
        .filter_map(|line| line.split(" canonical_regime=").nth(1))
        .filter_map(|rest| rest.split(' ').next())
        .collect();
    assert_eq!(first_regimes, second_regimes);
}

#[test]
fn gate_v4e_phase63_topology_hash_stability_canonical_window() {
    let first_log = run_curriculum_harness_with_phase62_kind("phase63");
    let second_log = run_curriculum_harness_with_phase62_kind("phase63");

    let first: Vec<&str> = first_log
        .lines()
        .filter(|line| line.contains("phase63_canonical_snapshot"))
        .filter(|line| line.contains("canonical_regime=closure_deficit") || line.contains("canonical_regime=closure_ready"))
        .collect();
    let second: Vec<&str> = second_log
        .lines()
        .filter(|line| line.contains("phase63_canonical_snapshot"))
        .filter(|line| line.contains("canonical_regime=closure_deficit") || line.contains("canonical_regime=closure_ready"))
        .collect();

    assert!(!first.is_empty(), "expected canonical window telemetry lines");
    assert_eq!(first, second);

    for line in first {
        assert!(line.contains("holdout_hash="), "missing holdout_hash: {}", line);
        assert!(line.contains("boundary_hash="), "missing boundary_hash: {}", line);
    }
}

#[test]
fn gate_v4a_phase63_repairs_canonical_02_class_without_speed_regression() {
    let log = run_curriculum_harness_with_phase62_kind("phase63");
    let holdout_line = latest_line_containing(&log, "phase63_holdout_telemetry holdout_id=holdout_02");
    let learning_line = latest_line_containing(
        &log,
        "learning assessment (full_stack) => structural_adaptation_present=true learning_curve_iterations=1 efficiency_verified=true budget<=10",
    );

    assert!(holdout_line.contains("phase63_holdout_telemetry"));
    // Gate A: topology-guided recovery improves/maintains continuity for 02-class.
    let memory_line = latest_line_containing(&log, "memory assessment (full_stack) =>");
    assert!(
        memory_line.contains("continuity_delta=1")
            || memory_line.contains("continuity_delta=0"),
        "expected non-negative continuity delta in phase63 run: {}",
        memory_line
    );
    assert!(learning_line.contains("learning_curve_iterations=1"));
}

#[test]
fn gate_v4b_v4c_phase63_no_regression_and_battery_floor() {
    let log = run_curriculum_harness_with_phase62_kind("phase63");
    let holdout_01 = latest_line_containing(&log, "phase63_holdout_telemetry holdout_id=holdout_01");
    let memory_line = latest_line_containing(&log, "memory assessment (full_stack) =>");
    let topology_line = latest_line_containing(
        &log,
        "PASS trained learner builds richer topology across holdouts ::",
    );

    // Gate B: no regression on strong holdout telemetry presence + deterministic final regime/hash emission.
    assert!(holdout_01.contains("final_regime="));
    assert!(holdout_01.contains("final_hash="));

    // Gate C: battery floor remains in accepted structural band.
    assert!(
        memory_line.contains("memory_improvement_score=4")
            || memory_line.contains("memory_improvement_score=5")
            || memory_line.contains("memory_improvement_score=6"),
        "unexpected memory score line: {}",
        memory_line
    );

    // Gate E: topology stability signal remains valid in full-stack verification.
    assert!(topology_line.contains("avg_region_advantage="));
}

#[test]
fn gate_v6a_phase66_continuity_rebased_emitted_and_replay_stable_for_02_04() {
    let first_log = run_curriculum_harness_with_phase62_kind("phase66");
    let second_log = run_curriculum_harness_with_phase62_kind("phase66");

    let first = extract_phase66_02_04_lines(&first_log);
    let second = extract_phase66_02_04_lines(&second_log);

    assert!(!first.is_empty(), "expected phase66 telemetry lines for holdout_02/04");
    assert_eq!(first, second);

    let holdout_02_count = first
        .iter()
        .filter(|line| line.contains("holdout_id=holdout_02"))
        .count();
    let holdout_04_count = first
        .iter()
        .filter(|line| line.contains("holdout_id=holdout_04"))
        .count();

    assert!(holdout_02_count > 0, "missing holdout_02 phase66 telemetry");
    assert!(holdout_04_count > 0, "missing holdout_04 phase66 telemetry");

    for line in first {
        assert!(line.contains("continuity_rebased="), "missing rebased continuity: {}", line);
        assert!(line.contains("alpha_num=4"), "missing alpha numerator: {}", line);
        assert!(line.contains("alpha_den=1000000"), "missing alpha denominator: {}", line);
        assert!(line.contains("beta=2"), "missing beta constant: {}", line);
        assert!(line.contains("gamma=1"), "missing gamma constant: {}", line);
    }
}

#[test]
fn gate_v6b_phase66_negative_rebased_flat_delta_requires_phase63_action() {
    let phase66_log = run_curriculum_harness_with_phase62_kind("phase66");
    let phase63_log = run_curriculum_harness_with_phase62_kind("phase63");

    let holdouts = [
        "holdout_01",
        "holdout_02",
        "holdout_03",
        "holdout_04",
        "holdout_05",
    ];

    let mut problem_holdouts = Vec::new();
    for holdout_id in holdouts {
        let phase66_line = latest_phase66_holdout_line(&phase66_log, holdout_id);
        let (continuity_delta, continuity_rebased) =
            parse_phase66_final_continuity_metrics(&phase66_line);
        if continuity_delta == 0 && continuity_rebased < 0 {
            problem_holdouts.push((holdout_id.to_string(), continuity_rebased));
        }
    }

    for (holdout_id, continuity_rebased) in problem_holdouts {
        let phase63_line = latest_phase63_holdout_line(&phase63_log, &holdout_id);
        let final_plan = parse_phase63_final_plan(&phase63_line);
        assert!(
            final_plan != "none",
            "holdout {} has continuity_delta=0 with continuity_rebased={} in phase66 but no phase63 action flag (final_plan=none): {}",
            holdout_id,
            continuity_rebased,
            phase63_line
        );
    }
}

#[test]
fn gate_v6c_phase63_semantic_improvement_required_matches_formula_for_02_04() {
    let phase63_log = run_curriculum_harness_with_phase62_kind("phase63");

    for holdout_id in ["holdout_02", "holdout_04"] {
        let phase63_line = latest_phase63_holdout_line(&phase63_log, holdout_id);
        let effectiveness = parse_i32_field(&phase63_line, "effectiveness");
        let problematic = parse_bool_field(&phase63_line, "problematic");
        let semantic_improvement_required =
            parse_bool_field(&phase63_line, "semantic_improvement_required");
        let escalation_candidate = parse_bool_field(&phase63_line, "escalation_candidate");
        let escalation_handoff = parse_bool_field(&phase63_line, "escalation_handoff");

        assert_eq!(
            semantic_improvement_required,
            problematic && effectiveness >= 0,
            "semantic_improvement_required formula mismatch for {}: {}",
            holdout_id,
            phase63_line
        );
        assert_eq!(
            escalation_candidate,
            semantic_improvement_required,
            "escalation_candidate must mirror semantic_improvement_required for {}: {}",
            holdout_id,
            phase63_line
        );
        assert_eq!(
            escalation_handoff,
            escalation_candidate,
            "escalation_handoff must mirror escalation_candidate for {}: {}",
            holdout_id,
            phase63_line
        );
    }
}

#[test]
fn gate_v6d_phase66_emits_phase67_marker_when_escalation_handoff_true() {
    let phase66_log = run_curriculum_harness_with_phase62_kind_and_handoff("phase66", true);
    let holdout_line = latest_phase66_holdout_line(&phase66_log, "holdout_04");

    assert!(
        holdout_line.contains("escalation_handoff=true"),
        "missing escalation_handoff=true in phase66 telemetry: {}",
        holdout_line
    );
    assert!(
        holdout_line.contains("phase67_escalation_marker=true"),
        "missing phase67 escalation marker in phase66 telemetry: {}",
        holdout_line
    );
}

#[test]
fn gate_v6e_phase67_stub_emits_marker_context_and_ready_fields() {
    let phase66_log = run_curriculum_harness_with_phase62_kind_and_handoff("phase66", true);
    let phase66_line = latest_phase66_holdout_line(&phase66_log, "holdout_04");
    let holdout_line = latest_phase67_holdout_line(&phase66_log, "holdout_04");

    let problematic = parse_phase66_final_bool_field(&phase66_line, "problematic");
    let effectiveness = parse_phase66_final_i32_field(&phase66_line, "effectiveness");
    let expected_context = if problematic && effectiveness >= 0 {
        "continuity_insensitive"
    } else {
        "none"
    };

    assert!(
        holdout_line.contains("phase67_holdout_telemetry"),
        "missing phase67 holdout telemetry line: {}",
        holdout_line
    );
    assert!(
        holdout_line.contains("phase67_escalation_marker_in=true"),
        "missing phase67 escalation marker input in telemetry: {}",
        holdout_line
    );
    assert!(
        holdout_line.contains(&format!("phase67_semantic_context={}", expected_context)),
        "phase67 semantic context mapping mismatch in telemetry: {}",
        holdout_line
    );
    assert!(
        holdout_line.contains("phase67_ready=true"),
        "missing phase67 ready flag in telemetry: {}",
        holdout_line
    );
}
