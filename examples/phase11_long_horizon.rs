use gort::{
    phase10_run_runtime_adaptation_episode, phase10_run_top_level_acceptance_stage,
    phase11_emit_long_horizon_convergence_telemetry,
    phase11_render_long_horizon_drift_diagnostic_report,
    phase11_run_long_horizon_convergence_harness, phase80_build_phase9_integration_hook,
    phase80_integrate_cross_frame_structural_deltas, phase80_run_multiframe_episode,
    phase80_summarize_episode_structural_integration, phase90_build_geometry_driven_adjustment_plan,
    phase90_compute_multishape_interaction_dynamics, phase90_compose_emergent_cognitive_shape,
    phase90_form_cognitive_manifold, phase90_form_continuity_weighted_field_from_seed,
    phase90_form_geometry_seed_from_integration_hook, Phase10Slice2RoutingAcceptancePolicy,
    Phase10Slice4RuntimeContinuityPolicy, Phase11ConvergenceAcceptancePolicy,
    Phase70AdjustmentLog, Phase70AdjustmentLogEntry, Phase70StructuralParameterRegistry,
};

fn usage(program: &str) -> String {
    format!(
        "Usage: {program} [min_loop_count] [max_loop_count] [loop_step] [cycle_count_per_loop]\nDefaults: 50 500 50 4"
    )
}

fn parse_arg(args: &[String], index: usize, default: usize) -> Result<usize, String> {
    if let Some(raw) = args.get(index) {
        return raw
            .parse::<usize>()
            .map_err(|_| format!("invalid numeric argument at position {}: {}", index, raw));
    }
    Ok(default)
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

fn plan_fixture() -> Result<
    (
        gort::Phase90GeometryDrivenAdjustmentPlan,
        Phase70StructuralParameterRegistry,
    ),
    String,
> {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log_a = Phase70AdjustmentLog {
        entries: vec![
            Phase70AdjustmentLogEntry {
                sequence: 1,
                holdout_id: "phase11_long_horizon_a1".to_string(),
                parameter_name: "continuity_pressure_boost".to_string(),
                semantic_context_used: "continuity_insensitive".to_string(),
                adjustment_applied: true,
                pre_value: 0,
                post_value: 1,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70AdjustmentLogEntry {
                sequence: 2,
                holdout_id: "phase11_long_horizon_a2".to_string(),
                parameter_name: "continuity_pressure_boost".to_string(),
                semantic_context_used: "none".to_string(),
                adjustment_applied: true,
                pre_value: 1,
                post_value: 2,
                delta: 1,
                inverse_delta: -1,
            },
        ],
    };
    let log_b = Phase70AdjustmentLog {
        entries: vec![
            Phase70AdjustmentLogEntry {
                sequence: 1,
                holdout_id: "phase11_long_horizon_b1".to_string(),
                parameter_name: "continuity_pressure_boost".to_string(),
                semantic_context_used: "continuity_insensitive".to_string(),
                adjustment_applied: true,
                pre_value: 0,
                post_value: 1,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70AdjustmentLogEntry {
                sequence: 2,
                holdout_id: "phase11_long_horizon_b2".to_string(),
                parameter_name: "continuity_pressure_boost".to_string(),
                semantic_context_used: "none".to_string(),
                adjustment_applied: false,
                pre_value: 1,
                post_value: 1,
                delta: 0,
                inverse_delta: 0,
            },
        ],
    };

    let shape_a = shape_from_log("phase11_long_horizon_shape_a", &log_a, &registry)?;
    let shape_b = shape_from_log("phase11_long_horizon_shape_b", &log_b, &registry)?;
    let manifold = phase90_form_cognitive_manifold(&[shape_a, shape_b])?;
    let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold)?;
    let plan = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics)?;
    Ok((plan, registry))
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        println!("{}", usage(&args[0]));
        return;
    }

    let min_loop_count = match parse_arg(&args, 1, 50) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            eprintln!("{}", usage(&args[0]));
            std::process::exit(2);
        }
    };
    let max_loop_count = match parse_arg(&args, 2, 500) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            eprintln!("{}", usage(&args[0]));
            std::process::exit(2);
        }
    };
    let loop_step = match parse_arg(&args, 3, 50) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            eprintln!("{}", usage(&args[0]));
            std::process::exit(2);
        }
    };
    let cycle_count_per_loop = match parse_arg(&args, 4, 4) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            eprintln!("{}", usage(&args[0]));
            std::process::exit(2);
        }
    };

    let (plan, registry) = match plan_fixture() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("failed to build Phase11 fixture plan: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = phase10_run_top_level_acceptance_stage(
        "phase11_long_horizon_baseline",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        3,
    ) {
        eprintln!("baseline top-level acceptance failed: {}", err);
        std::process::exit(1);
    }

    // Warm the adaptation path to ensure this executable exercises the full stack once.
    if let Ok(bridge) = gort::phase10_build_runtime_adaptation_bridge(&plan, &registry) {
        if let Err(err) = phase10_run_runtime_adaptation_episode(
            "phase11_long_horizon_warm",
            &bridge,
            &registry,
        ) {
            eprintln!("runtime adaptation warmup failed: {}", err);
            std::process::exit(1);
        }
    }

    let harness = match phase11_run_long_horizon_convergence_harness(
        "phase11_long_horizon",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        &Phase11ConvergenceAcceptancePolicy::canonical(),
        min_loop_count,
        max_loop_count,
        loop_step,
        cycle_count_per_loop,
    ) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("long horizon harness failed: {}", err);
            std::process::exit(1);
        }
    };

    println!("=== GORT PHASE11 LONG-HORIZON CONVERGENCE SUMMARY ===");
    println!("check                            | result");
    println!("---------------------------------+-------");
    println!(
        "loop_range                        | {}..={} step {}",
        harness.min_loop_count, harness.max_loop_count, harness.loop_step
    );
    println!(
        "cycle_count_per_loop              | {}",
        harness.cycle_count_per_loop
    );
    println!(
        "observation_count                 | {}",
        harness.observations.len()
    );
    println!(
        "profile_hash_stability_percent    | {}",
        harness.profile_hash_stability_percent
    );
    println!(
        "max_convergence_radius            | {}",
        harness.max_convergence_radius
    );
    println!(
        "harness_well_formed               | {}",
        if harness.harness_well_formed { "PASS" } else { "FAIL" }
    );

    for obs in &harness.observations {
        println!(
            "loop_{:03}_radius                 | {}",
            obs.loop_count, obs.convergence_metrics.convergence_radius
        );
    }

    let telemetry = phase11_emit_long_horizon_convergence_telemetry(&harness);
    println!("telemetry={}", telemetry);
    println!(
        "{}",
        phase11_render_long_horizon_drift_diagnostic_report(&harness).trim_end()
    );
    println!("=== END SUMMARY ===");

    if !harness.harness_well_formed {
        std::process::exit(1);
    }
}
