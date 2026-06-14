use rugc::{
    apply_phase62_structural_experiment, Phase62ExperimentKind, Phase62StructuralConfig,
    SemanticConstraint,
};

fn sample_constraints() -> Vec<SemanticConstraint> {
    vec![
        SemanticConstraint::assertion("balance", "stable", true, 60),
        SemanticConstraint::assertion("balance", "tilt", false, 30),
        SemanticConstraint::assertion("grasp", "secure", true, 55),
    ]
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
#[ignore = "Phase 6.2 placeholder: replace with full harness acceptance once integrated"]
fn gate_ba_phase62_structural_speed_improves_without_memory_regression() {
    // Placeholder acceptance target for Phase 6.2 integration.
    // Future expectation:
    // - learning_curve_iterations <= 10
    // - memory_improved_after_recovery == true
    // - strict anti-shortcut quality checks remain satisfied
    assert!(true);
}
