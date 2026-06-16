use gort::{
    phase12_emit_program_telemetry, phase12_synthesize_emergent_cognitive_program,
    phase80_build_phase9_integration_hook, phase80_integrate_cross_frame_structural_deltas,
    phase80_run_multiframe_episode, phase80_summarize_episode_structural_integration,
    phase90_build_geometry_driven_adjustment_plan, phase90_compute_multishape_interaction_dynamics,
    phase90_compose_emergent_cognitive_shape, phase90_form_cognitive_manifold,
    phase90_form_continuity_weighted_field_from_seed, phase90_form_geometry_seed_from_integration_hook,
    Phase70AdjustmentLog, Phase70AdjustmentLogEntry, Phase70StructuralParameterRegistry,
};

fn entry(
    sequence: u64,
    holdout_id: &str,
    semantic_context_used: &str,
    adjustment_applied: bool,
    pre_value: i32,
    post_value: i32,
    delta: i32,
) -> Phase70AdjustmentLogEntry {
    Phase70AdjustmentLogEntry {
        sequence,
        holdout_id: holdout_id.to_string(),
        parameter_name: "continuity_pressure_boost".to_string(),
        semantic_context_used: semantic_context_used.to_string(),
        adjustment_applied,
        pre_value,
        post_value,
        delta,
        inverse_delta: if adjustment_applied { -delta } else { 0 },
    }
}

fn shape_from_log(
    episode_id: &str,
    log: &Phase70AdjustmentLog,
    registry: &Phase70StructuralParameterRegistry,
) -> Result<gort::Phase90EmergentCognitiveShape, String> {
    let trace = phase80_run_multiframe_episode(episode_id, log, registry)?;
    let deltas = phase80_integrate_cross_frame_structural_deltas(&trace, log, registry)?;
    let summary = phase80_summarize_episode_structural_integration(&trace, log, registry)?;
    let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
    let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
    let field = phase90_form_continuity_weighted_field_from_seed(&seed);
    phase90_compose_emergent_cognitive_shape(&[field])
}

fn main() {
    println!("=== GORT Phase 12 Emergent Cognitive Programs ===\n");

    let registry = Phase70StructuralParameterRegistry::canonical();

    let log_a = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "phase12_a1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "phase12_a2", "none", true, 1, 2, 1),
        ],
    };
    let log_b = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "phase12_b1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "phase12_b2", "none", false, 1, 1, 0),
            entry(3, "phase12_b3", "none", true, 1, 2, 1),
        ],
    };

    let shape_a = shape_from_log("phase12_shape_a", &log_a, &registry).expect("shape_a");
    let shape_b = shape_from_log("phase12_shape_b", &log_b, &registry).expect("shape_b");
    let manifold = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold");
    let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");
    let plan = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("plan");
    let program = phase12_synthesize_emergent_cognitive_program(&manifold, &plan).expect("program");

    println!("manifold_kind={}", manifold.manifold_archetype);
    println!("operator_plan_kind={}", plan.dominant_operator_kind);
    println!("program_kind={}", program.program_kind);
    println!("step_count={}", program.step_count);
    println!("telemetry={}", phase12_emit_program_telemetry(&program));
}