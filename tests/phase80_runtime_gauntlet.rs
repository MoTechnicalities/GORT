use gort::{
    Phase70AdjustmentLog, Phase70AdjustmentLogEntry, Phase70StructuralParameterRegistry,
    Phase70StructuralParameterSpec,
    Phase80FrameLocalParameterRegistry, Phase80FrameParameterSnapshot, Phase80FrameParameterValue,
    Phase80FrameTransitionEvent, phase80_build_frame_local_parameter_registry,
    phase80_emit_episode_telemetry, phase80_emit_frame_transition_telemetry,
    phase80_emit_integration_telemetry, phase80_integrate_cross_frame_structural_deltas,
    phase80_build_phase9_integration_hook, phase80_summarize_episode_structural_integration,
    phase80_run_multiframe_episode, phase80_scaffold_frame_parameter_snapshots,
    phase80_scaffold_frame_transitions, phase80_sequence_frame_transitions,
    phase80_validate_frame_continuity_invariants,
    phase90_form_geometry_seed_from_integration_hook, phase90_emit_seed_formation_telemetry,
    phase90_form_continuity_weighted_field_from_seed, phase90_emit_field_telemetry,
    phase90_compose_emergent_cognitive_shape, phase90_emit_shape_telemetry,
    phase90_form_cognitive_manifold, phase90_emit_manifold_telemetry,
    phase90_compute_multishape_interaction_dynamics, phase90_emit_interaction_dynamics_telemetry,
    phase90_build_geometry_driven_adjustment_plan, phase90_emit_geometry_operator_telemetry,
    Phase90GeometryDrivenAdjustmentOperator, Phase90GeometryDrivenAdjustmentPlan,
    phase12_emit_program_telemetry, phase12_synthesize_emergent_cognitive_program,
    Phase13QubitUnaryOp,
    phase13_apply_unitary_sequence, phase13_build_qubit_state,
    phase13_emit_measurement_telemetry, phase13_emit_state_telemetry,
    phase13_measure_z, phase13_ops_commute, phase13_unitary_signature,
    phase13_validate_qubit_state_invariants, phase13_validate_unitary_invariants,
    Phase14OperatorFamilyKind,
    phase14_build_clifford_family, phase14_build_commutation_table, phase14_build_pauli_family,
    phase14_compute_commutator, phase14_emit_algebra_telemetry, phase14_emit_family_telemetry,
    phase14_validate_family_invariants, phase14_validate_table_invariants,
    phase10_build_runtime_adaptation_bridge, phase10_run_runtime_adaptation_episode,
    phase10_emit_runtime_adaptation_telemetry,
    Phase10Slice2RoutingAcceptancePolicy, phase10_validate_slice2_routing_acceptance_gate,
    Phase10Slice4RuntimeContinuityPolicy, phase10_validate_runtime_continuity_preservation,
    phase10_integrate_operator_plan_into_runtime_transitions,
    phase10_execute_closed_loop_integrity_stage,
    phase10_emit_operator_runtime_integration_telemetry,
    phase10_regenerate_phase9_seed_from_adapted_episode,
    phase10_emit_runtime_feedback_telemetry,
    phase10_run_top_level_acceptance_stage,
    phase10_emit_top_level_acceptance_telemetry,
    phase10_run_slice7_multicycle_replay_acceptance_stage,
    phase10_emit_slice7_multicycle_telemetry,
    Phase11ConvergenceAcceptancePolicy,
    phase11_run_multi_loop_convergence_stage,
    phase11_emit_multi_loop_convergence_telemetry,
};

const PARAM: &str = "continuity_pressure_boost";

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
        parameter_name: PARAM.to_string(),
        semantic_context_used: semantic_context_used.to_string(),
        adjustment_applied,
        pre_value,
        post_value,
        delta,
        inverse_delta: if adjustment_applied { -delta } else { 0 },
    }
}

fn phase90_field_fixture(
    episode_id: &str,
    log: &Phase70AdjustmentLog,
    registry: &Phase70StructuralParameterRegistry,
) -> gort::Phase90ContinuityWeightedGeometryField {
    let trace = phase80_run_multiframe_episode(episode_id, log, registry).expect("trace");
    let deltas =
        phase80_integrate_cross_frame_structural_deltas(&trace, log, registry).expect("deltas");
    let summary =
        phase80_summarize_episode_structural_integration(&trace, log, registry).expect("summary");
    let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
    let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
    phase90_form_continuity_weighted_field_from_seed(&seed)
}

fn phase90_shape_fixture(
    shape_prefix: &str,
    logs: &[Phase70AdjustmentLog],
    registry: &Phase70StructuralParameterRegistry,
) -> gort::Phase90EmergentCognitiveShape {
    let fields = logs
        .iter()
        .enumerate()
        .map(|(index, log)| phase90_field_fixture(&format!("{}_{}", shape_prefix, index + 1), log, registry))
        .collect::<Vec<_>>();
    phase90_compose_emergent_cognitive_shape(&fields).expect("shape")
}

fn phase90_manifold_fixture(
    manifold_prefix: &str,
    shape_log_groups: &[Vec<Phase70AdjustmentLog>],
    registry: &Phase70StructuralParameterRegistry,
) -> gort::Phase90CognitiveManifold {
    let shapes = shape_log_groups
        .iter()
        .enumerate()
        .map(|(index, logs)| phase90_shape_fixture(&format!("{}_shape{}", manifold_prefix, index + 1), logs, registry))
        .collect::<Vec<_>>();
    phase90_form_cognitive_manifold(&shapes).expect("manifold")
}

fn phase90_plan_fixture(
    manifold_prefix: &str,
    shape_log_groups: &[Vec<Phase70AdjustmentLog>],
    registry: &Phase70StructuralParameterRegistry,
) -> (
    gort::Phase90CognitiveManifold,
    gort::Phase90MultiShapeInteractionDynamics,
    gort::Phase90GeometryDrivenAdjustmentPlan,
) {
    let manifold = phase90_manifold_fixture(manifold_prefix, shape_log_groups, registry);
    let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");
    let plan = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("plan");
    (manifold, dynamics, plan)
}

#[test]
fn gauntlet_replay_stability_100_runs_identical_episode_trace_and_telemetry() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_01_recovery", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_02_recovery", "none", false, 1, 1, 0),
            entry(3, "holdout_03_recovery", "continuity_insensitive", true, 1, 2, 1),
        ],
    };

    let baseline_trace =
        phase80_run_multiframe_episode("gauntlet_replay", &log, &registry).expect("baseline episode");
    let baseline_telemetry = phase80_emit_episode_telemetry(&baseline_trace);
    let baseline_transition_telemetry = phase80_emit_frame_transition_telemetry(
        &phase80_scaffold_frame_transitions(
            &phase80_build_frame_local_parameter_registry(&log, &registry).expect("frame registry"),
            &registry,
        )
        .expect("transitions"),
    );

    for _ in 0..100 {
        let trace =
            phase80_run_multiframe_episode("gauntlet_replay", &log, &registry).expect("episode run");
        let telemetry = phase80_emit_episode_telemetry(&trace);
        let transition_telemetry = phase80_emit_frame_transition_telemetry(
            &phase80_scaffold_frame_transitions(
                &phase80_build_frame_local_parameter_registry(&log, &registry)
                    .expect("frame registry"),
                &registry,
            )
            .expect("transitions"),
        );

        assert_eq!(trace, baseline_trace);
        assert_eq!(telemetry, baseline_telemetry);
        assert_eq!(transition_telemetry, baseline_transition_telemetry);
    }
}

#[test]
fn gauntlet_dead_system_100_episodes_no_drift_or_mutation() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let inert_log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_dead_01", "none", false, 0, 0, 0),
            entry(2, "holdout_dead_02", "none", false, 0, 0, 0),
            entry(3, "holdout_dead_03", "none", false, 0, 0, 0),
        ],
    };

    let baseline_snapshots =
        phase80_scaffold_frame_parameter_snapshots(&inert_log, &registry).expect("snapshots");
    let baseline_trace =
        phase80_run_multiframe_episode("dead_system", &inert_log, &registry).expect("episode");

    for _ in 0..100 {
        let snapshots = phase80_scaffold_frame_parameter_snapshots(&inert_log, &registry)
            .expect("snapshots remain deterministic");
        let trace =
            phase80_run_multiframe_episode("dead_system", &inert_log, &registry).expect("episode");

        assert_eq!(snapshots, baseline_snapshots);
        assert_eq!(trace, baseline_trace);
        assert!(trace.boundary_continuity_preserved);
    }
}

#[test]
fn gauntlet_continuity_stress_deterministic_rejection_reason_codes() {
    let registry = Phase70StructuralParameterRegistry::canonical();

    let repeated_frame_registry = Phase80FrameLocalParameterRegistry {
        frames: vec![
            Phase80FrameParameterSnapshot {
                frame_id: "phase80_frame_0001".to_string(),
                holdout_id: "holdout_a".to_string(),
                parameters: vec![Phase80FrameParameterValue {
                    parameter_name: PARAM.to_string(),
                    effective_value: 0,
                }],
            },
            Phase80FrameParameterSnapshot {
                frame_id: "phase80_frame_0001".to_string(),
                holdout_id: "holdout_b".to_string(),
                parameters: vec![Phase80FrameParameterValue {
                    parameter_name: PARAM.to_string(),
                    effective_value: 1,
                }],
            },
        ],
    };

    let err_a = phase80_validate_frame_continuity_invariants(&repeated_frame_registry, &registry)
        .expect_err("repeated frame IDs must be rejected");
    let err_b = phase80_validate_frame_continuity_invariants(&repeated_frame_registry, &registry)
        .expect_err("repeated frame IDs must be rejected deterministically");

    assert_eq!(err_a, err_b);
    assert!(err_a.contains("non-canonical frame order"));
}

#[test]
fn gauntlet_semantic_isolation_and_structural_divergence_are_explicit() {
    let registry = Phase70StructuralParameterRegistry::canonical();

    let episode_a_log = Phase70AdjustmentLog {
        entries: vec![entry(
            1,
            "holdout_semantic_a",
            "continuity_insensitive",
            true,
            0,
            1,
            1,
        )],
    };
    let episode_b_log = Phase70AdjustmentLog {
        entries: vec![entry(1, "holdout_semantic_b", "none", false, 0, 0, 0)],
    };

    let trace_a =
        phase80_run_multiframe_episode("semantic_a", &episode_a_log, &registry).expect("episode A");
    let trace_b =
        phase80_run_multiframe_episode("semantic_b", &episode_b_log, &registry).expect("episode B");

    let snapshots_a =
        phase80_scaffold_frame_parameter_snapshots(&episode_a_log, &registry).expect("snapshots A");
    let snapshots_b =
        phase80_scaffold_frame_parameter_snapshots(&episode_b_log, &registry).expect("snapshots B");

    assert_ne!(snapshots_a, snapshots_b);
    assert_ne!(trace_a.steps, trace_b.steps);
    assert_eq!(Phase70StructuralParameterRegistry::canonical(), registry);
}

#[test]
fn gauntlet_merge_consistency_is_order_invariant_for_registry_permutations() {
    let reg_a = Phase70StructuralParameterRegistry {
        parameters: vec![
            gort::Phase70StructuralParameterSpec {
                name: "zeta_pressure".to_string(),
                env_key: "GORT_PHASE70_ZETA_PRESSURE".to_string(),
                min_value: 0,
                max_value: 3,
                delta: 1,
                inverse_delta: -1,
            },
            gort::Phase70StructuralParameterSpec {
                name: "alpha_pressure".to_string(),
                env_key: "GORT_PHASE70_ALPHA_PRESSURE".to_string(),
                min_value: 0,
                max_value: 3,
                delta: 1,
                inverse_delta: -1,
            },
        ],
    };

    let reg_b = Phase70StructuralParameterRegistry {
        parameters: vec![
            gort::Phase70StructuralParameterSpec {
                name: "alpha_pressure".to_string(),
                env_key: "GORT_PHASE70_ALPHA_PRESSURE".to_string(),
                min_value: 0,
                max_value: 3,
                delta: 1,
                inverse_delta: -1,
            },
            gort::Phase70StructuralParameterSpec {
                name: "zeta_pressure".to_string(),
                env_key: "GORT_PHASE70_ZETA_PRESSURE".to_string(),
                min_value: 0,
                max_value: 3,
                delta: 1,
                inverse_delta: -1,
            },
        ],
    };

    let log = Phase70AdjustmentLog {
        entries: vec![Phase70AdjustmentLogEntry {
            sequence: 1,
            holdout_id: "holdout_merge".to_string(),
            parameter_name: "alpha_pressure".to_string(),
            semantic_context_used: "continuity_insensitive".to_string(),
            adjustment_applied: true,
            pre_value: 0,
            post_value: 1,
            delta: 1,
            inverse_delta: -1,
        }],
    };

    let snap_a = phase80_scaffold_frame_parameter_snapshots(&log, &reg_a).expect("snap A");
    let snap_b = phase80_scaffold_frame_parameter_snapshots(&log, &reg_b).expect("snap B");

    assert_eq!(snap_a, snap_b);
}

#[test]
fn gauntlet_episode_boundary_accept_reject_and_identity_preservation() {
    let registry = Phase70StructuralParameterRegistry::canonical();

    let clean_log = Phase70AdjustmentLog {
        entries: vec![entry(1, "holdout_clean", "none", false, 0, 0, 0)],
    };
    let clean_before =
        phase80_run_multiframe_episode("identity_clean", &clean_log, &registry).expect("clean before");

    let bad_frame_registry = Phase80FrameLocalParameterRegistry {
        frames: vec![
            Phase80FrameParameterSnapshot {
                frame_id: "phase80_frame_0001".to_string(),
                holdout_id: "holdout_x".to_string(),
                parameters: vec![Phase80FrameParameterValue {
                    parameter_name: PARAM.to_string(),
                    effective_value: 0,
                }],
            },
            Phase80FrameParameterSnapshot {
                frame_id: "phase80_frame_0002".to_string(),
                holdout_id: "holdout_y".to_string(),
                parameters: vec![Phase80FrameParameterValue {
                    parameter_name: PARAM.to_string(),
                    effective_value: 1,
                }],
            },
        ],
    };
    let bad_transitions = vec![
        Phase80FrameTransitionEvent {
            from_frame_id: "phase80_origin".to_string(),
            to_frame_id: "phase80_frame_0001".to_string(),
            entry_allowed: true,
            exit_allowed: true,
            continuity_preserved: true,
            transition_reason: "bounded_continuity".to_string(),
        },
        Phase80FrameTransitionEvent {
            from_frame_id: "phase80_origin".to_string(),
            to_frame_id: "phase80_frame_0002".to_string(),
            entry_allowed: true,
            exit_allowed: true,
            continuity_preserved: true,
            transition_reason: "bounded_continuity".to_string(),
        },
    ];

    let err = phase80_sequence_frame_transitions(&bad_frame_registry, &bad_transitions)
        .expect_err("non-chain source must be rejected");
    assert!(err.contains("transition source mismatch"));

    let clean_after =
        phase80_run_multiframe_episode("identity_clean", &clean_log, &registry).expect("clean after");
    assert_eq!(clean_before, clean_after);
    assert_eq!(
        phase80_emit_episode_telemetry(&clean_before),
        phase80_emit_episode_telemetry(&clean_after)
    );
}

#[test]
fn gauntlet_slice5_integration_summary_and_phase9_hook_are_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_slice5_a", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_slice5_b", "none", false, 1, 1, 0),
            entry(3, "holdout_slice5_c", "none", true, 1, 2, 1),
        ],
    };

    let trace_a =
        phase80_run_multiframe_episode("gauntlet_slice5", &log, &registry).expect("episode A");
    let trace_b =
        phase80_run_multiframe_episode("gauntlet_slice5", &log, &registry).expect("episode B");

    let summary_a = phase80_summarize_episode_structural_integration(&trace_a, &log, &registry)
        .expect("summary A");
    let summary_b = phase80_summarize_episode_structural_integration(&trace_b, &log, &registry)
        .expect("summary B");

    let deltas_a = phase80_integrate_cross_frame_structural_deltas(&trace_a, &log, &registry)
        .expect("deltas A");
    let deltas_b = phase80_integrate_cross_frame_structural_deltas(&trace_b, &log, &registry)
        .expect("deltas B");

    let hook_a = phase80_build_phase9_integration_hook(&summary_a, &deltas_a);
    let hook_b = phase80_build_phase9_integration_hook(&summary_b, &deltas_b);

    let telemetry_a = phase80_emit_integration_telemetry(&summary_a, &hook_a);
    let telemetry_b = phase80_emit_integration_telemetry(&summary_b, &hook_b);

    assert_eq!(summary_a, summary_b);
    assert_eq!(deltas_a, deltas_b);
    assert_eq!(hook_a, hook_b);
    assert_eq!(telemetry_a, telemetry_b);
    assert_eq!(summary_a.total_raw_structural_delta, 2);
    assert_eq!(summary_a.total_continuity_adjusted_delta, 2);
    assert!(hook_a.integration_ready);
}

#[test]
fn gauntlet_slice6_gate_a_semantic_propagation_is_canonical_across_none_gaps() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_slice6_a1", "none", false, 0, 0, 0),
            entry(2, "holdout_slice6_a2", "continuity_insensitive", true, 0, 1, 1),
            entry(3, "holdout_slice6_a3", "none", true, 1, 2, 1),
        ],
    };

    let trace =
        phase80_run_multiframe_episode("slice6_gate_a", &log, &registry).expect("episode run");
    let deltas =
        phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");

    assert_eq!(deltas.len(), 3);
    assert_eq!(deltas[0].propagated_semantic_context, "none");
    assert_eq!(deltas[1].propagated_semantic_context, "continuity_insensitive");
    assert_eq!(deltas[2].propagated_semantic_context, "continuity_insensitive");
}

#[test]
fn gauntlet_slice6_gate_b_continuity_adjustment_degrades_deltas_and_weight_on_break() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_slice6_b1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_slice6_b2", "none", true, 1, 2, 1),
        ],
    };

    let mut trace =
        phase80_run_multiframe_episode("slice6_gate_b", &log, &registry).expect("episode run");
    // Simulate one continuity break to verify deterministic continuity-aware damping.
    trace.steps[1].continuity_preserved = false;

    let deltas =
        phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
    let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
        .expect("summary");
    let hook = phase80_build_phase9_integration_hook(&summary, &deltas);

    assert_eq!(summary.total_raw_structural_delta, 2);
    assert_eq!(summary.total_continuity_adjusted_delta, 1);
    assert_eq!(hook.continuity_weight_percent, 50);
    assert!(!summary.continuity_preserved);
}

#[test]
fn gauntlet_slice6_gate_c_phase9_hook_readiness_requires_continuity_and_nonempty_delta_chain() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![entry(1, "holdout_slice6_c1", "none", false, 0, 0, 0)],
    };

    let mut trace =
        phase80_run_multiframe_episode("slice6_gate_c", &log, &registry).expect("episode run");
    trace.boundary_continuity_preserved = false;

    let deltas =
        phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
    let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
        .expect("summary");
    let hook = phase80_build_phase9_integration_hook(&summary, &deltas);

    assert_eq!(deltas.len(), 1);
    assert!(!summary.continuity_preserved);
    assert!(!hook.integration_ready);
}

#[test]
fn gauntlet_slice6_gate_d_integration_telemetry_is_replay_stable_over_50_runs() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_slice6_d1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_slice6_d2", "none", false, 1, 1, 0),
            entry(3, "holdout_slice6_d3", "none", true, 1, 2, 1),
        ],
    };

    let baseline_trace =
        phase80_run_multiframe_episode("slice6_gate_d", &log, &registry).expect("episode");
    let baseline_summary =
        phase80_summarize_episode_structural_integration(&baseline_trace, &log, &registry)
            .expect("summary");
    let baseline_deltas =
        phase80_integrate_cross_frame_structural_deltas(&baseline_trace, &log, &registry)
            .expect("deltas");
    let baseline_hook = phase80_build_phase9_integration_hook(&baseline_summary, &baseline_deltas);
    let baseline_telemetry = phase80_emit_integration_telemetry(&baseline_summary, &baseline_hook);

    for _ in 0..50 {
        let trace =
            phase80_run_multiframe_episode("slice6_gate_d", &log, &registry).expect("episode");
        let summary =
            phase80_summarize_episode_structural_integration(&trace, &log, &registry)
                .expect("summary");
        let deltas =
            phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry)
                .expect("deltas");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
        let telemetry = phase80_emit_integration_telemetry(&summary, &hook);

        assert_eq!(summary, baseline_summary);
        assert_eq!(deltas, baseline_deltas);
        assert_eq!(hook, baseline_hook);
        assert_eq!(telemetry, baseline_telemetry);
    }
}

// ============ Phase 9 Slice 1: Geometric Cognitive Seed Formation ============

#[test]
fn gauntlet_phase9_slice1_seed_formation_from_integration_hook() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9_a", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9_b", "none", false, 1, 1, 0),
            entry(3, "holdout_p9_c", "none", true, 1, 2, 1),
        ],
    };

    let trace = phase80_run_multiframe_episode("gauntlet_p9s1", &log, &registry).expect("trace");
    let deltas =
        phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
    let summary =
        phase80_summarize_episode_structural_integration(&trace, &log, &registry).expect("summary");
    let hook = phase80_build_phase9_integration_hook(&summary, &deltas);

    // Phase 9 Slice 1: form seed
    let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);

    assert_eq!(seed.episode_id, "gauntlet_p9s1");
    assert!(seed.geometry_well_formed);
    assert!(!seed.seed_content_hash.is_empty());
    assert_eq!(seed.transition_count, deltas.len());
    assert!(!seed.semantic_anchor_contexts.is_empty());
    assert_eq!(seed.source_frame_count, 3);
    assert_eq!(seed.continuity_weight_percent, hook.continuity_weight_percent);
}

#[test]
fn gauntlet_phase9_slice1_seed_geometry_is_deterministic_over_100_replays() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9_d", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9_e", "none", true, 1, 2, 1),
            entry(3, "holdout_p9_f", "none", false, 2, 2, 0),
        ],
    };

    let mut seed_signatures = Vec::new();

    for _ in 0..100 {
        let trace = phase80_run_multiframe_episode("gauntlet_p9s1_replay", &log, &registry)
            .expect("trace");
        let deltas = phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry)
            .expect("deltas");
        let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
            .expect("summary");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
        let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);

        seed_signatures.push(seed.geometry_signature.clone());
    }

    let first_sig = &seed_signatures[0];
    for (idx, sig) in seed_signatures.iter().enumerate() {
        assert_eq!(
            sig, first_sig,
            "seed signature must be identical at replay {}: {} vs {}",
            idx + 1,
            sig,
            first_sig
        );
    }
}

#[test]
fn gauntlet_phase9_slice1_seed_telemetry_is_canonical() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9_g", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9_h", "none", false, 1, 1, 0),
        ],
    };

    let trace = phase80_run_multiframe_episode("gauntlet_p9s1_telemetry", &log, &registry)
        .expect("trace");
    let deltas =
        phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
    let summary =
        phase80_summarize_episode_structural_integration(&trace, &log, &registry).expect("summary");
    let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
    let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);

    let telemetry_1 = phase90_emit_seed_formation_telemetry(&seed);
    let telemetry_2 = phase90_emit_seed_formation_telemetry(&seed);

    assert_eq!(telemetry_1, telemetry_2);
    assert!(telemetry_1.contains("episode_id=gauntlet_p9s1_telemetry"));
    assert!(telemetry_1.contains("geometry_signature="));
    assert!(telemetry_1.contains("semantic_anchors="));
    assert!(telemetry_1.contains("continuity_weight="));
    assert!(telemetry_1.contains("transition_count="));
    assert!(telemetry_1.contains("well_formed=true"));
}

// ============ Phase 9 Slice 2: Continuity-Weighted Geometry Fields ============

#[test]
fn gauntlet_phase9_slice2_field_formation_from_seed() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s2_a", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s2_b", "none", true, 1, 2, 1),
            entry(3, "holdout_p9s2_c", "none", false, 2, 2, 0),
        ],
    };

    let trace = phase80_run_multiframe_episode("gauntlet_p9s2_field", &log, &registry)
        .expect("trace");
    let deltas =
        phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
    let summary =
        phase80_summarize_episode_structural_integration(&trace, &log, &registry).expect("summary");
    let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
    let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);

    // Phase 9 Slice 2: form field
    let field = phase90_form_continuity_weighted_field_from_seed(&seed);

    assert_eq!(field.episode_id, "gauntlet_p9s2_field");
    assert_eq!(field.field_strength, seed.continuity_weight_percent);
    assert!((10..=90).contains(&field.influence_decay_rate));
    assert!((50..=500).contains(&field.field_radius));
    assert!(!field.field_signature.is_empty());
    assert!(!field.field_profile_hash.is_empty());
    assert!(field.field_well_formed);
}

#[test]
fn gauntlet_phase9_slice2_field_is_deterministic_over_100_replays() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s2_d", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s2_e", "none", true, 1, 2, 1),
            entry(3, "holdout_p9s2_f", "none", true, 2, 2, 0),
        ],
    };

    let mut field_hashes = Vec::new();

    for _ in 0..100 {
        let trace = phase80_run_multiframe_episode("gauntlet_p9s2_determinism", &log, &registry)
            .expect("trace");
        let deltas = phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry)
            .expect("deltas");
        let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
            .expect("summary");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
        let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
        let field = phase90_form_continuity_weighted_field_from_seed(&seed);

        field_hashes.push(field.field_profile_hash.clone());
    }

    let first_hash = &field_hashes[0];
    for (idx, hash) in field_hashes.iter().enumerate() {
        assert_eq!(
            hash, first_hash,
            "field profile hash must be identical at replay {}: {} vs {}",
            idx + 1,
            hash,
            first_hash
        );
    }
}

#[test]
fn gauntlet_phase9_slice2_field_telemetry_is_canonical_and_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s2_g", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s2_h", "none", false, 1, 1, 0),
            entry(3, "holdout_p9s2_i", "none", true, 1, 2, 1),
        ],
    };

    let trace = phase80_run_multiframe_episode("gauntlet_p9s2_telemetry", &log, &registry)
        .expect("trace");
    let deltas =
        phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
    let summary =
        phase80_summarize_episode_structural_integration(&trace, &log, &registry).expect("summary");
    let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
    let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
    let field = phase90_form_continuity_weighted_field_from_seed(&seed);

    let telemetry_1 = phase90_emit_field_telemetry(&field);
    let telemetry_2 = phase90_emit_field_telemetry(&field);

    assert_eq!(telemetry_1, telemetry_2);
    assert!(telemetry_1.contains("episode_id=gauntlet_p9s2_telemetry"));
    assert!(telemetry_1.contains("field_strength="));
    assert!(telemetry_1.contains("decay_rate="));
    assert!(telemetry_1.contains("radius="));
    assert!(telemetry_1.contains("contexts="));
    assert!(telemetry_1.contains("well_formed=true"));
}

// ============ Phase 9 Slice 3: Emergent Cognitive Shapes ============

#[test]
fn gauntlet_phase9_slice3_shape_composition_from_fields() {
    let registry = Phase70StructuralParameterRegistry::canonical();

    let log_a = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s3_a1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s3_a2", "none", true, 1, 2, 1),
        ],
    };
    let log_b = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s3_b1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s3_b2", "none", false, 1, 1, 0),
            entry(3, "holdout_p9s3_b3", "none", true, 1, 2, 1),
        ],
    };

    let trace_a = phase80_run_multiframe_episode("gauntlet_p9s3_a", &log_a, &registry)
        .expect("trace a");
    let deltas_a = phase80_integrate_cross_frame_structural_deltas(&trace_a, &log_a, &registry)
        .expect("deltas a");
    let summary_a = phase80_summarize_episode_structural_integration(&trace_a, &log_a, &registry)
        .expect("summary a");
    let hook_a = phase80_build_phase9_integration_hook(&summary_a, &deltas_a);
    let seed_a = phase90_form_geometry_seed_from_integration_hook(&hook_a, &summary_a, &deltas_a);
    let field_a = phase90_form_continuity_weighted_field_from_seed(&seed_a);

    let trace_b = phase80_run_multiframe_episode("gauntlet_p9s3_b", &log_b, &registry)
        .expect("trace b");
    let deltas_b = phase80_integrate_cross_frame_structural_deltas(&trace_b, &log_b, &registry)
        .expect("deltas b");
    let summary_b = phase80_summarize_episode_structural_integration(&trace_b, &log_b, &registry)
        .expect("summary b");
    let hook_b = phase80_build_phase9_integration_hook(&summary_b, &deltas_b);
    let seed_b = phase90_form_geometry_seed_from_integration_hook(&hook_b, &summary_b, &deltas_b);
    let field_b = phase90_form_continuity_weighted_field_from_seed(&seed_b);

    let shape = phase90_compose_emergent_cognitive_shape(&[field_a, field_b]).expect("shape");

    assert_eq!(shape.constituent_field_count, 2);
    assert!(shape.shape_well_formed);
    assert!(!shape.emergent_shape_signature.is_empty());
    assert!(!shape.shape_profile_hash.is_empty());
}

#[test]
fn gauntlet_phase9_slice3_shape_is_deterministic_over_100_replays() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log_a = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s3_c1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s3_c2", "none", true, 1, 2, 1),
        ],
    };
    let log_b = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s3_d1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s3_d2", "none", false, 1, 1, 0),
            entry(3, "holdout_p9s3_d3", "none", true, 1, 2, 1),
        ],
    };

    let mut shape_hashes = Vec::new();

    for _ in 0..100 {
        let trace_a = phase80_run_multiframe_episode("gauntlet_p9s3_det_a", &log_a, &registry)
            .expect("trace a");
        let deltas_a = phase80_integrate_cross_frame_structural_deltas(&trace_a, &log_a, &registry)
            .expect("deltas a");
        let summary_a = phase80_summarize_episode_structural_integration(&trace_a, &log_a, &registry)
            .expect("summary a");
        let hook_a = phase80_build_phase9_integration_hook(&summary_a, &deltas_a);
        let seed_a = phase90_form_geometry_seed_from_integration_hook(&hook_a, &summary_a, &deltas_a);
        let field_a = phase90_form_continuity_weighted_field_from_seed(&seed_a);

        let trace_b = phase80_run_multiframe_episode("gauntlet_p9s3_det_b", &log_b, &registry)
            .expect("trace b");
        let deltas_b = phase80_integrate_cross_frame_structural_deltas(&trace_b, &log_b, &registry)
            .expect("deltas b");
        let summary_b = phase80_summarize_episode_structural_integration(&trace_b, &log_b, &registry)
            .expect("summary b");
        let hook_b = phase80_build_phase9_integration_hook(&summary_b, &deltas_b);
        let seed_b = phase90_form_geometry_seed_from_integration_hook(&hook_b, &summary_b, &deltas_b);
        let field_b = phase90_form_continuity_weighted_field_from_seed(&seed_b);

        let shape = phase90_compose_emergent_cognitive_shape(&[field_a, field_b]).expect("shape");
        shape_hashes.push(shape.shape_profile_hash);
    }

    let first_hash = &shape_hashes[0];
    for (index, hash) in shape_hashes.iter().enumerate() {
        assert_eq!(
            hash, first_hash,
            "shape profile hash must be identical at replay {}: {} vs {}",
            index + 1,
            hash,
            first_hash
        );
    }
}

#[test]
fn gauntlet_phase9_slice3_shape_telemetry_is_canonical_and_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log_a = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s3_e1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s3_e2", "none", true, 1, 2, 1),
        ],
    };
    let log_b = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s3_f1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s3_f2", "none", false, 1, 1, 0),
            entry(3, "holdout_p9s3_f3", "none", true, 1, 2, 1),
        ],
    };

    let trace_a = phase80_run_multiframe_episode("gauntlet_p9s3_telem_a", &log_a, &registry)
        .expect("trace a");
    let deltas_a = phase80_integrate_cross_frame_structural_deltas(&trace_a, &log_a, &registry)
        .expect("deltas a");
    let summary_a = phase80_summarize_episode_structural_integration(&trace_a, &log_a, &registry)
        .expect("summary a");
    let hook_a = phase80_build_phase9_integration_hook(&summary_a, &deltas_a);
    let seed_a = phase90_form_geometry_seed_from_integration_hook(&hook_a, &summary_a, &deltas_a);
    let field_a = phase90_form_continuity_weighted_field_from_seed(&seed_a);

    let trace_b = phase80_run_multiframe_episode("gauntlet_p9s3_telem_b", &log_b, &registry)
        .expect("trace b");
    let deltas_b = phase80_integrate_cross_frame_structural_deltas(&trace_b, &log_b, &registry)
        .expect("deltas b");
    let summary_b = phase80_summarize_episode_structural_integration(&trace_b, &log_b, &registry)
        .expect("summary b");
    let hook_b = phase80_build_phase9_integration_hook(&summary_b, &deltas_b);
    let seed_b = phase90_form_geometry_seed_from_integration_hook(&hook_b, &summary_b, &deltas_b);
    let field_b = phase90_form_continuity_weighted_field_from_seed(&seed_b);

    let shape = phase90_compose_emergent_cognitive_shape(&[field_a, field_b]).expect("shape");
    let telemetry_a = phase90_emit_shape_telemetry(&shape);
    let telemetry_b = phase90_emit_shape_telemetry(&shape);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("episode_span="));
    assert!(telemetry_a.contains("field_count=2"));
    assert!(telemetry_a.contains("archetype="));
    assert!(telemetry_a.contains("aggregate_strength="));
    assert!(telemetry_a.contains("continuity_envelope="));
    assert!(telemetry_a.contains("well_formed=true"));
}

// ============ Phase 9 Slice 4: Cognitive Manifolds / Topology Graphs ============

#[test]
fn gauntlet_phase9_slice4_manifold_forms_from_shapes() {
    let registry = Phase70StructuralParameterRegistry::canonical();

    let log_a1 = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s4_a1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s4_a2", "none", true, 1, 2, 1),
        ],
    };
    let log_a2 = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s4_a3", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s4_a4", "none", false, 1, 1, 0),
            entry(3, "holdout_p9s4_a5", "none", true, 1, 2, 1),
        ],
    };
    let log_b1 = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s4_b1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s4_b2", "none", true, 1, 2, 1),
        ],
    };
    let log_b2 = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s4_b3", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s4_b4", "none", true, 1, 2, 1),
        ],
    };

    let shape_a = {
        let trace_a1 = phase80_run_multiframe_episode("gauntlet_p9s4_shape_a1", &log_a1, &registry)
            .expect("trace a1");
        let deltas_a1 = phase80_integrate_cross_frame_structural_deltas(&trace_a1, &log_a1, &registry)
            .expect("deltas a1");
        let summary_a1 = phase80_summarize_episode_structural_integration(&trace_a1, &log_a1, &registry)
            .expect("summary a1");
        let hook_a1 = phase80_build_phase9_integration_hook(&summary_a1, &deltas_a1);
        let seed_a1 = phase90_form_geometry_seed_from_integration_hook(&hook_a1, &summary_a1, &deltas_a1);
        let field_a1 = phase90_form_continuity_weighted_field_from_seed(&seed_a1);

        let trace_a2 = phase80_run_multiframe_episode("gauntlet_p9s4_shape_a2", &log_a2, &registry)
            .expect("trace a2");
        let deltas_a2 = phase80_integrate_cross_frame_structural_deltas(&trace_a2, &log_a2, &registry)
            .expect("deltas a2");
        let summary_a2 = phase80_summarize_episode_structural_integration(&trace_a2, &log_a2, &registry)
            .expect("summary a2");
        let hook_a2 = phase80_build_phase9_integration_hook(&summary_a2, &deltas_a2);
        let seed_a2 = phase90_form_geometry_seed_from_integration_hook(&hook_a2, &summary_a2, &deltas_a2);
        let field_a2 = phase90_form_continuity_weighted_field_from_seed(&seed_a2);

        phase90_compose_emergent_cognitive_shape(&[field_a1, field_a2]).expect("shape a")
    };

    let shape_b = {
        let trace_b1 = phase80_run_multiframe_episode("gauntlet_p9s4_shape_b1", &log_b1, &registry)
            .expect("trace b1");
        let deltas_b1 = phase80_integrate_cross_frame_structural_deltas(&trace_b1, &log_b1, &registry)
            .expect("deltas b1");
        let summary_b1 = phase80_summarize_episode_structural_integration(&trace_b1, &log_b1, &registry)
            .expect("summary b1");
        let hook_b1 = phase80_build_phase9_integration_hook(&summary_b1, &deltas_b1);
        let seed_b1 = phase90_form_geometry_seed_from_integration_hook(&hook_b1, &summary_b1, &deltas_b1);
        let field_b1 = phase90_form_continuity_weighted_field_from_seed(&seed_b1);

        let trace_b2 = phase80_run_multiframe_episode("gauntlet_p9s4_shape_b2", &log_b2, &registry)
            .expect("trace b2");
        let deltas_b2 = phase80_integrate_cross_frame_structural_deltas(&trace_b2, &log_b2, &registry)
            .expect("deltas b2");
        let summary_b2 = phase80_summarize_episode_structural_integration(&trace_b2, &log_b2, &registry)
            .expect("summary b2");
        let hook_b2 = phase80_build_phase9_integration_hook(&summary_b2, &deltas_b2);
        let seed_b2 = phase90_form_geometry_seed_from_integration_hook(&hook_b2, &summary_b2, &deltas_b2);
        let field_b2 = phase90_form_continuity_weighted_field_from_seed(&seed_b2);

        phase90_compose_emergent_cognitive_shape(&[field_b1, field_b2]).expect("shape b")
    };

    let manifold = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold");

    assert_eq!(manifold.shape_count, 2);
    assert!(manifold.manifold_well_formed);
    assert!(!manifold.manifold_signature.is_empty());
    assert!(!manifold.adjacency_edges.is_empty());
}

#[test]
fn gauntlet_phase9_slice4_manifold_is_deterministic_over_100_replays() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log_left = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s4_c1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s4_c2", "none", true, 1, 2, 1),
        ],
    };
    let log_right = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s4_d1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s4_d2", "none", false, 1, 1, 0),
            entry(3, "holdout_p9s4_d3", "none", true, 1, 2, 1),
        ],
    };

    let mut manifold_hashes = Vec::new();

    for _ in 0..100 {
        let trace_left = phase80_run_multiframe_episode("gauntlet_p9s4_det_left", &log_left, &registry)
            .expect("trace left");
        let deltas_left = phase80_integrate_cross_frame_structural_deltas(&trace_left, &log_left, &registry)
            .expect("deltas left");
        let summary_left = phase80_summarize_episode_structural_integration(&trace_left, &log_left, &registry)
            .expect("summary left");
        let hook_left = phase80_build_phase9_integration_hook(&summary_left, &deltas_left);
        let seed_left = phase90_form_geometry_seed_from_integration_hook(&hook_left, &summary_left, &deltas_left);
        let field_left = phase90_form_continuity_weighted_field_from_seed(&seed_left);
        let shape_left = phase90_compose_emergent_cognitive_shape(&[field_left]).expect("shape left");

        let trace_right = phase80_run_multiframe_episode("gauntlet_p9s4_det_right", &log_right, &registry)
            .expect("trace right");
        let deltas_right = phase80_integrate_cross_frame_structural_deltas(&trace_right, &log_right, &registry)
            .expect("deltas right");
        let summary_right = phase80_summarize_episode_structural_integration(&trace_right, &log_right, &registry)
            .expect("summary right");
        let hook_right = phase80_build_phase9_integration_hook(&summary_right, &deltas_right);
        let seed_right = phase90_form_geometry_seed_from_integration_hook(&hook_right, &summary_right, &deltas_right);
        let field_right = phase90_form_continuity_weighted_field_from_seed(&seed_right);
        let shape_right = phase90_compose_emergent_cognitive_shape(&[field_right]).expect("shape right");

        let manifold = phase90_form_cognitive_manifold(&[shape_left, shape_right]).expect("manifold");
        manifold_hashes.push(manifold.manifold_profile_hash);
    }

    let first_hash = &manifold_hashes[0];
    for (index, hash) in manifold_hashes.iter().enumerate() {
        assert_eq!(
            hash, first_hash,
            "manifold profile hash must be identical at replay {}: {} vs {}",
            index + 1,
            hash,
            first_hash
        );
    }
}

#[test]
fn gauntlet_phase9_slice4_manifold_telemetry_is_canonical_and_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let log_left = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s4_e1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s4_e2", "none", true, 1, 2, 1),
        ],
    };
    let log_right = Phase70AdjustmentLog {
        entries: vec![
            entry(1, "holdout_p9s4_f1", "continuity_insensitive", true, 0, 1, 1),
            entry(2, "holdout_p9s4_f2", "none", true, 1, 2, 1),
        ],
    };

    let trace_left = phase80_run_multiframe_episode("gauntlet_p9s4_telem_left", &log_left, &registry)
        .expect("trace left");
    let deltas_left = phase80_integrate_cross_frame_structural_deltas(&trace_left, &log_left, &registry)
        .expect("deltas left");
    let summary_left = phase80_summarize_episode_structural_integration(&trace_left, &log_left, &registry)
        .expect("summary left");
    let hook_left = phase80_build_phase9_integration_hook(&summary_left, &deltas_left);
    let seed_left = phase90_form_geometry_seed_from_integration_hook(&hook_left, &summary_left, &deltas_left);
    let field_left = phase90_form_continuity_weighted_field_from_seed(&seed_left);
    let shape_left = phase90_compose_emergent_cognitive_shape(&[field_left]).expect("shape left");

    let trace_right = phase80_run_multiframe_episode("gauntlet_p9s4_telem_right", &log_right, &registry)
        .expect("trace right");
    let deltas_right = phase80_integrate_cross_frame_structural_deltas(&trace_right, &log_right, &registry)
        .expect("deltas right");
    let summary_right = phase80_summarize_episode_structural_integration(&trace_right, &log_right, &registry)
        .expect("summary right");
    let hook_right = phase80_build_phase9_integration_hook(&summary_right, &deltas_right);
    let seed_right = phase90_form_geometry_seed_from_integration_hook(&hook_right, &summary_right, &deltas_right);
    let field_right = phase90_form_continuity_weighted_field_from_seed(&seed_right);
    let shape_right = phase90_compose_emergent_cognitive_shape(&[field_right]).expect("shape right");

    let manifold = phase90_form_cognitive_manifold(&[shape_left, shape_right]).expect("manifold");
    let telemetry_a = phase90_emit_manifold_telemetry(&manifold);
    let telemetry_b = phase90_emit_manifold_telemetry(&manifold);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("shape_span="));
    assert!(telemetry_a.contains("shape_count=2"));
    assert!(telemetry_a.contains("edge_count="));
    assert!(telemetry_a.contains("embedding="));
    assert!(telemetry_a.contains("well_formed=true"));
}

// ============ Phase 9 Slice 5: Multi-Shape Interaction Dynamics ============

#[test]
fn gauntlet_phase9_slice5_dynamics_form_from_manifold() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let manifold = phase90_manifold_fixture(
        "gauntlet_p9s5",
        &[
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s5_a1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s5_a2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s5_a3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s5_a4", "none", false, 1, 1, 0),
                        entry(3, "p9s5_a5", "none", true, 1, 2, 1),
                    ],
                },
            ],
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s5_b1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s5_b2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s5_b3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s5_b4", "none", true, 1, 2, 1),
                    ],
                },
            ],
        ],
        &registry,
    );

    let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");

    assert!(dynamics.dynamics_well_formed);
    assert!(dynamics.interaction_count > 0);
    assert!(!dynamics.dynamics_signature.is_empty());
}

#[test]
fn gauntlet_phase9_slice5_dynamics_are_deterministic_over_100_replays() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let mut hashes = Vec::new();

    for _ in 0..100 {
        let manifold = phase90_manifold_fixture(
            "gauntlet_p9s5_replay",
            &[
                vec![Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s5_r1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s5_r2", "none", true, 1, 2, 1),
                    ],
                }],
                vec![Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s5_r3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s5_r4", "none", true, 1, 2, 1),
                    ],
                }],
            ],
            &registry,
        );
        let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");
        hashes.push(dynamics.dynamics_profile_hash);
    }

    let first_hash = &hashes[0];
    for (index, hash) in hashes.iter().enumerate() {
        assert_eq!(
            hash, first_hash,
            "dynamics profile hash must be identical at replay {}: {} vs {}",
            index + 1,
            hash,
            first_hash
        );
    }
}

#[test]
fn gauntlet_phase9_slice5_dynamics_telemetry_is_canonical_and_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let manifold = phase90_manifold_fixture(
        "gauntlet_p9s5_telem",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p9s5_t1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p9s5_t2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p9s5_t3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p9s5_t4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");
    let telemetry_a = phase90_emit_interaction_dynamics_telemetry(&dynamics);
    let telemetry_b = phase90_emit_interaction_dynamics_telemetry(&dynamics);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("interaction_count="));
    assert!(telemetry_a.contains("dominant_mode="));
    assert!(telemetry_a.contains("aggregate_pressure="));
    assert!(telemetry_a.contains("well_formed=true"));
}

// ============ Phase 9 Slice 6: Geometry-Driven Adjustment Operators ============

#[test]
fn gauntlet_phase9_slice6_operator_plan_forms_from_dynamics() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p9s6",
        &[
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s6_a1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s6_a2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s6_a3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s6_a4", "none", false, 1, 1, 0),
                        entry(3, "p9s6_a5", "none", true, 1, 2, 1),
                    ],
                },
            ],
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s6_b1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s6_b2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s6_b3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s6_b4", "none", true, 1, 2, 1),
                    ],
                },
            ],
        ],
        &registry,
    );

    assert!(plan.operator_plan_well_formed);
    assert!(plan.operator_count > 0);
    assert!(!plan.operator_plan_signature.is_empty());
}

#[test]
fn gauntlet_phase9_slice6_operator_plan_is_deterministic_over_100_replays() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let mut hashes = Vec::new();

    for _ in 0..100 {
        let (_manifold, _dynamics, plan) = phase90_plan_fixture(
            "gauntlet_p9s6_replay",
            &[
                vec![Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s6_r1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s6_r2", "none", true, 1, 2, 1),
                    ],
                }],
                vec![Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9s6_r3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9s6_r4", "none", true, 1, 2, 1),
                    ],
                }],
            ],
            &registry,
        );
        hashes.push(plan.operator_plan_hash);
    }

    let first_hash = &hashes[0];
    for (index, hash) in hashes.iter().enumerate() {
        assert_eq!(
            hash, first_hash,
            "operator plan hash must be identical at replay {}: {} vs {}",
            index + 1,
            hash,
            first_hash
        );
    }
}

#[test]
fn gauntlet_phase9_slice6_operator_telemetry_is_canonical_and_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p9s6_telem",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p9s6_t1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p9s6_t2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p9s6_t3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p9s6_t4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let telemetry_a = phase90_emit_geometry_operator_telemetry(&plan);
    let telemetry_b = phase90_emit_geometry_operator_telemetry(&plan);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("operator_count="));
    assert!(telemetry_a.contains("dominant_kind="));
    assert!(telemetry_a.contains("aggregate_pressure="));
    assert!(telemetry_a.contains("well_formed=true"));
}

// ============ Phase 9 Slice 7: Acceptance Gates ============

#[test]
fn gauntlet_phase9_gate_a_full_geometry_stack_is_well_formed_end_to_end() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (manifold, dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p9_gate_a",
        &[
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9ga1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9ga2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9ga3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9ga4", "none", false, 1, 1, 0),
                        entry(3, "p9ga5", "none", true, 1, 2, 1),
                    ],
                },
            ],
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9ga6", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9ga7", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9ga8", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9ga9", "none", true, 1, 2, 1),
                    ],
                },
            ],
        ],
        &registry,
    );

    assert!(manifold.manifold_well_formed);
    assert!(dynamics.dynamics_well_formed);
    assert!(plan.operator_plan_well_formed);
}

#[test]
fn gauntlet_phase9_gate_b_signature_mismatch_is_rejected_for_operator_synthesis() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (manifold, mut dynamics, _plan) = phase90_plan_fixture(
        "gauntlet_p9_gate_b",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p9gb1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p9gb2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p9gb3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p9gb4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    dynamics.manifold_signature.push_str("::mismatch");
    let err = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics)
        .expect_err("signature mismatch must be rejected");
    assert!(err.contains("signature mismatch"));
}

#[test]
fn gauntlet_phase9_gate_c_low_edge_manifold_reduces_interaction_and_operator_pressure() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_strong_manifold, strong_dynamics, strong_plan) = phase90_plan_fixture(
        "gauntlet_p9_gate_c_strong",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p9gc1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p9gc2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p9gc3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p9gc4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let weak_manifold = phase90_manifold_fixture(
        "gauntlet_p9_gate_c_weak",
        &[vec![Phase70AdjustmentLog {
            entries: vec![entry(1, "p9gc5", "none", false, 0, 0, 0)],
        }]],
        &registry,
    );

    let weak_dynamics = phase90_compute_multishape_interaction_dynamics(&weak_manifold)
        .expect_err("weak manifold without edges must be rejected");

    assert!(strong_dynamics.aggregate_operator_pressure_percent > 0);
    assert!(strong_plan.aggregate_adjustment_pressure_percent > 0);
    assert!(weak_dynamics.contains("without manifold edges"));
}

#[test]
fn gauntlet_phase9_gate_d_full_phase9_telemetry_chain_is_replay_stable_over_50_runs() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let baseline = phase90_plan_fixture(
        "gauntlet_p9_gate_d",
        &[
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9gd1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9gd2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9gd3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9gd4", "none", false, 1, 1, 0),
                        entry(3, "p9gd5", "none", true, 1, 2, 1),
                    ],
                },
            ],
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9gd6", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9gd7", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p9gd8", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p9gd9", "none", true, 1, 2, 1),
                    ],
                },
            ],
        ],
        &registry,
    );
    let baseline_manifold_telemetry = phase90_emit_manifold_telemetry(&baseline.0);
    let baseline_dynamics_telemetry = phase90_emit_interaction_dynamics_telemetry(&baseline.1);
    let baseline_operator_telemetry = phase90_emit_geometry_operator_telemetry(&baseline.2);

    for _ in 0..50 {
        let current = phase90_plan_fixture(
            "gauntlet_p9_gate_d",
            &[
                vec![
                    Phase70AdjustmentLog {
                        entries: vec![
                            entry(1, "p9gd1", "continuity_insensitive", true, 0, 1, 1),
                            entry(2, "p9gd2", "none", true, 1, 2, 1),
                        ],
                    },
                    Phase70AdjustmentLog {
                        entries: vec![
                            entry(1, "p9gd3", "continuity_insensitive", true, 0, 1, 1),
                            entry(2, "p9gd4", "none", false, 1, 1, 0),
                            entry(3, "p9gd5", "none", true, 1, 2, 1),
                        ],
                    },
                ],
                vec![
                    Phase70AdjustmentLog {
                        entries: vec![
                            entry(1, "p9gd6", "continuity_insensitive", true, 0, 1, 1),
                            entry(2, "p9gd7", "none", true, 1, 2, 1),
                        ],
                    },
                    Phase70AdjustmentLog {
                        entries: vec![
                            entry(1, "p9gd8", "continuity_insensitive", true, 0, 1, 1),
                            entry(2, "p9gd9", "none", true, 1, 2, 1),
                        ],
                    },
                ],
            ],
            &registry,
        );

        assert_eq!(phase90_emit_manifold_telemetry(&current.0), baseline_manifold_telemetry);
        assert_eq!(phase90_emit_interaction_dynamics_telemetry(&current.1), baseline_dynamics_telemetry);
        assert_eq!(phase90_emit_geometry_operator_telemetry(&current.2), baseline_operator_telemetry);
    }
}

// ============ Phase 10 Slice 1: Runtime Adaptation Feedback ============

#[test]
fn gauntlet_phase10_runtime_adaptation_bridge_feeds_back_into_episode_execution() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_feedback",
        &[
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p10f1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p10f2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p10f3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p10f4", "none", false, 1, 1, 0),
                        entry(3, "p10f5", "none", true, 1, 2, 1),
                    ],
                },
            ],
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p10f6", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p10f7", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p10f8", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p10f9", "none", true, 1, 2, 1),
                    ],
                },
            ],
        ],
        &registry,
    );

    assert!(dynamics.dynamics_well_formed);
    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let trace = phase10_run_runtime_adaptation_episode("gauntlet_p10_adapted", &bridge, &registry)
        .expect("adapted trace");

    assert!(bridge.adaptation_well_formed);
    assert!(bridge.adapted_entry_count > 0);
    assert!(!trace.steps.is_empty());
}

#[test]
fn gauntlet_phase10_runtime_adaptation_is_deterministic_over_100_replays() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_replay",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10r1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10r2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10r3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10r4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let mut hashes = Vec::new();
    for _ in 0..100 {
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
        let trace =
            phase10_run_runtime_adaptation_episode("gauntlet_p10_replay_episode", &bridge, &registry)
                .expect("trace");
        hashes.push(format!(
            "{}:{}:{}",
            bridge.adaptation_profile_hash,
            bridge.adapted_entry_count,
            trace.steps.len(),
        ));
    }

    let first_hash = &hashes[0];
    for (index, hash) in hashes.iter().enumerate() {
        assert_eq!(
            hash, first_hash,
            "phase10 adaptation replay hash mismatch at replay {}: {} vs {}",
            index + 1,
            hash,
            first_hash
        );
    }
}

#[test]
fn gauntlet_phase10_runtime_adaptation_telemetry_is_canonical_and_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_telem",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10t1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10t2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10t3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10t4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let telemetry_a = phase10_emit_runtime_adaptation_telemetry(&bridge);
    let telemetry_b = phase10_emit_runtime_adaptation_telemetry(&bridge);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("manifold_signature="));
    assert!(telemetry_a.contains("operator_plan_hash="));
    assert!(telemetry_a.contains("entry_count="));
    assert!(telemetry_a.contains("aggregate_delta="));
    assert!(telemetry_a.contains("well_formed=true"));
}

#[test]
fn gauntlet_phase10_slice2_multi_parameter_routing_is_deterministic_and_valid() {
    let mut registry = Phase70StructuralParameterRegistry::canonical();
    registry.parameters = vec![
        Phase70StructuralParameterSpec {
            name: "continuity_pressure_boost".to_string(),
            env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
            min_value: 0,
            max_value: 6,
            delta: 1,
            inverse_delta: -1,
        },
        Phase70StructuralParameterSpec {
            name: "coherence_gradient_trim".to_string(),
            env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
            min_value: 0,
            max_value: 4,
            delta: 1,
            inverse_delta: -1,
        },
        Phase70StructuralParameterSpec {
            name: "boundary_tension_relief".to_string(),
            env_key: "GORT_PHASE70_BOUNDARY_TENSION_RELIEF".to_string(),
            min_value: 0,
            max_value: 5,
            delta: 1,
            inverse_delta: -1,
        },
    ];

    let (manifold, _dynamics, _plan_fixture) = phase90_plan_fixture(
        "gauntlet_p10_slice2",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s2a1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s2a2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s2b1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s2b2", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let plan = Phase90GeometryDrivenAdjustmentPlan {
        manifold_signature: manifold.manifold_signature,
        operator_count: 4,
        dominant_operator_kind: "continuity_lift".to_string(),
        aggregate_adjustment_pressure_percent: 72,
        operators: vec![
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_op_1".to_string(),
                target_shape_index: 0,
                source_shape_index: 1,
                operator_kind: "continuity_lift".to_string(),
                adjustment_pressure_percent: 80,
                continuity_bias_percent: 70,
                manifold_alignment_percent: 75,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_op_2".to_string(),
                target_shape_index: 2,
                source_shape_index: 0,
                operator_kind: "gradient_smooth".to_string(),
                adjustment_pressure_percent: 78,
                continuity_bias_percent: 74,
                manifold_alignment_percent: 71,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_op_3".to_string(),
                target_shape_index: 1,
                source_shape_index: 2,
                operator_kind: "resonance_reweight".to_string(),
                adjustment_pressure_percent: 82,
                continuity_bias_percent: 68,
                manifold_alignment_percent: 73,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_op_4".to_string(),
                target_shape_index: 3,
                source_shape_index: 1,
                operator_kind: "continuity_lift".to_string(),
                adjustment_pressure_percent: 75,
                continuity_bias_percent: 72,
                manifold_alignment_percent: 70,
            },
        ],
        operator_plan_signature: "gauntlet_slice2_multi_parameter_fixture".to_string(),
        operator_plan_hash: "gauntlet_slice2_multi_parameter_fixture_hash".to_string(),
        operator_plan_well_formed: true,
    };

    let bridge_a = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge a");
    let bridge_b = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge b");

    assert!(bridge_a.adaptation_well_formed);
    assert!(bridge_a.routed_parameter_count >= 2);
    assert_eq!(bridge_a, bridge_b);
}

#[test]
fn gauntlet_phase10_slice2_acceptance_gate_passes_with_diverse_routing_at_threshold() {
    let mut registry = Phase70StructuralParameterRegistry::canonical();
    registry.parameters = vec![
        Phase70StructuralParameterSpec {
            name: "continuity_pressure_boost".to_string(),
            env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
            min_value: 0,
            max_value: 6,
            delta: 1,
            inverse_delta: -1,
        },
        Phase70StructuralParameterSpec {
            name: "coherence_gradient_trim".to_string(),
            env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
            min_value: 0,
            max_value: 4,
            delta: 1,
            inverse_delta: -1,
        },
    ];

    let plan = Phase90GeometryDrivenAdjustmentPlan {
        manifold_signature: "gauntlet_slice2_acceptance_pass".to_string(),
        operator_count: 3,
        dominant_operator_kind: "continuity_lift".to_string(),
        aggregate_adjustment_pressure_percent: 72,
        operators: vec![
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_acceptance_pass_1".to_string(),
                target_shape_index: 0,
                source_shape_index: 0,
                operator_kind: "continuity_lift".to_string(),
                adjustment_pressure_percent: 80,
                continuity_bias_percent: 70,
                manifold_alignment_percent: 75,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_acceptance_pass_2".to_string(),
                target_shape_index: 1,
                source_shape_index: 0,
                operator_kind: "gradient_smooth".to_string(),
                adjustment_pressure_percent: 78,
                continuity_bias_percent: 74,
                manifold_alignment_percent: 71,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_acceptance_pass_3".to_string(),
                target_shape_index: 0,
                source_shape_index: 1,
                operator_kind: "resonance_reweight".to_string(),
                adjustment_pressure_percent: 82,
                continuity_bias_percent: 68,
                manifold_alignment_percent: 73,
            },
        ],
        operator_plan_signature: "gauntlet_slice2_acceptance_pass_sig".to_string(),
        operator_plan_hash: "gauntlet_slice2_acceptance_pass_hash".to_string(),
        operator_plan_well_formed: true,
    };

    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let policy = Phase10Slice2RoutingAcceptancePolicy::canonical();

    phase10_validate_slice2_routing_acceptance_gate(&plan, &bridge, &policy)
        .expect("acceptance should pass");
}

#[test]
fn gauntlet_phase10_slice2_acceptance_gate_rejects_low_diversity_when_threshold_met() {
    let mut registry = Phase70StructuralParameterRegistry::canonical();
    registry.parameters = vec![Phase70StructuralParameterSpec {
        name: "continuity_pressure_boost".to_string(),
        env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
        min_value: 0,
        max_value: 6,
        delta: 1,
        inverse_delta: -1,
    }];

    let plan = Phase90GeometryDrivenAdjustmentPlan {
        manifold_signature: "gauntlet_slice2_acceptance_reject".to_string(),
        operator_count: 3,
        dominant_operator_kind: "continuity_lift".to_string(),
        aggregate_adjustment_pressure_percent: 72,
        operators: vec![
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_acceptance_reject_1".to_string(),
                target_shape_index: 0,
                source_shape_index: 0,
                operator_kind: "continuity_lift".to_string(),
                adjustment_pressure_percent: 80,
                continuity_bias_percent: 70,
                manifold_alignment_percent: 75,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_acceptance_reject_2".to_string(),
                target_shape_index: 1,
                source_shape_index: 0,
                operator_kind: "gradient_smooth".to_string(),
                adjustment_pressure_percent: 78,
                continuity_bias_percent: 74,
                manifold_alignment_percent: 71,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice2_acceptance_reject_3".to_string(),
                target_shape_index: 0,
                source_shape_index: 1,
                operator_kind: "resonance_reweight".to_string(),
                adjustment_pressure_percent: 82,
                continuity_bias_percent: 68,
                manifold_alignment_percent: 73,
            },
        ],
        operator_plan_signature: "gauntlet_slice2_acceptance_reject_sig".to_string(),
        operator_plan_hash: "gauntlet_slice2_acceptance_reject_hash".to_string(),
        operator_plan_well_formed: true,
    };

    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let policy = Phase10Slice2RoutingAcceptancePolicy::canonical();

    let err = phase10_validate_slice2_routing_acceptance_gate(&plan, &bridge, &policy)
        .expect_err("acceptance should reject low diversity");
    assert!(err.contains("routed parameter diversity"));
}

#[test]
fn gauntlet_phase10_slice3_operator_runtime_integration_is_deterministic() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice3",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s3a1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s3a2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s3b1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s3b2", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let integration_a = phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
        .expect("integration a");
    let integration_b = phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
        .expect("integration b");

    assert!(integration_a.integration_well_formed);
    assert_eq!(integration_a, integration_b);
}

#[test]
fn gauntlet_phase10_slice3_operator_runtime_integration_telemetry_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice3_telem",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s3t1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s3t2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s3t3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s3t4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let integration =
        phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
            .expect("integration");

    let telemetry_a = phase10_emit_operator_runtime_integration_telemetry(&integration);
    let telemetry_b = phase10_emit_operator_runtime_integration_telemetry(&integration);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("operator_plan_hash="));
    assert!(telemetry_a.contains("weighted_transition_count="));
    assert!(telemetry_a.contains("well_formed=true"));
}

#[test]
fn gauntlet_phase10_slice4_runtime_continuity_preservation_accepts_canonical_integration() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice4",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s4a1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s4a2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s4b1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s4b2", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let integration =
        phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
            .expect("integration");
    let policy = Phase10Slice4RuntimeContinuityPolicy::canonical();

    phase10_validate_runtime_continuity_preservation(&plan, &bridge, &integration, &policy)
        .expect("canonical integration should preserve continuity");
}

#[test]
fn gauntlet_phase10_slice4_runtime_continuity_rejects_with_canonical_reason_code() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice4_reject",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s4r1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s4r2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s4r3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s4r4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let mut integration =
        phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
            .expect("integration");
    integration.weighted_transitions[0].continuity_preserved = false;
    integration.weighted_transitions[0].transition_reason =
        "phase10_forced_continuity_break".to_string();

    let policy = Phase10Slice4RuntimeContinuityPolicy::canonical();
    let err = phase10_validate_runtime_continuity_preservation(&plan, &bridge, &integration, &policy)
        .expect_err("continuity violation must be rejected");

    assert_eq!(err, "phase10_slice4_reject_continuity_violation");
}

#[test]
fn gauntlet_phase10_slice4_runtime_continuity_validation_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice4_replay",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s4v1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s4v2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s4v3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s4v4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
    let integration =
        phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
            .expect("integration");
    let policy = Phase10Slice4RuntimeContinuityPolicy::canonical();

    let result_a = phase10_validate_runtime_continuity_preservation(&plan, &bridge, &integration, &policy);
    let result_b = phase10_validate_runtime_continuity_preservation(&plan, &bridge, &integration, &policy);

    assert_eq!(result_a, result_b);
    assert!(result_a.is_ok());
}

#[test]
fn gauntlet_phase10_slice5_closed_loop_integrity_stage_is_well_formed() {
    let mut registry = Phase70StructuralParameterRegistry::canonical();
    registry.parameters = vec![
        Phase70StructuralParameterSpec {
            name: "continuity_pressure_boost".to_string(),
            env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
            min_value: 0,
            max_value: 6,
            delta: 1,
            inverse_delta: -1,
        },
        Phase70StructuralParameterSpec {
            name: "coherence_gradient_trim".to_string(),
            env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
            min_value: 0,
            max_value: 4,
            delta: 1,
            inverse_delta: -1,
        },
    ];

    let plan = Phase90GeometryDrivenAdjustmentPlan {
        manifold_signature: "gauntlet_slice5_closed_loop".to_string(),
        operator_count: 4,
        dominant_operator_kind: "continuity_lift".to_string(),
        aggregate_adjustment_pressure_percent: 72,
        operators: vec![
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_op_1".to_string(),
                target_shape_index: 0,
                source_shape_index: 0,
                operator_kind: "continuity_lift".to_string(),
                adjustment_pressure_percent: 80,
                continuity_bias_percent: 70,
                manifold_alignment_percent: 75,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_op_2".to_string(),
                target_shape_index: 1,
                source_shape_index: 0,
                operator_kind: "gradient_smooth".to_string(),
                adjustment_pressure_percent: 78,
                continuity_bias_percent: 74,
                manifold_alignment_percent: 71,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_op_3".to_string(),
                target_shape_index: 0,
                source_shape_index: 1,
                operator_kind: "resonance_reweight".to_string(),
                adjustment_pressure_percent: 82,
                continuity_bias_percent: 68,
                manifold_alignment_percent: 73,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_op_4".to_string(),
                target_shape_index: 1,
                source_shape_index: 1,
                operator_kind: "continuity_lift".to_string(),
                adjustment_pressure_percent: 75,
                continuity_bias_percent: 72,
                manifold_alignment_percent: 70,
            },
        ],
        operator_plan_signature: "gauntlet_slice5_closed_loop_sig".to_string(),
        operator_plan_hash: "gauntlet_slice5_closed_loop_hash".to_string(),
        operator_plan_well_formed: true,
    };

    let stage = phase10_execute_closed_loop_integrity_stage(
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
    )
    .expect("closed loop stage");

    assert!(stage.integrity_well_formed);
    assert!(stage.continuity_gate_passed);
    assert!(stage.routed_parameter_count >= 2);
}

#[test]
fn gauntlet_phase10_slice5_closed_loop_integrity_stage_rejects_slice2_failure() {
    let mut registry = Phase70StructuralParameterRegistry::canonical();
    registry.parameters = vec![Phase70StructuralParameterSpec {
        name: "continuity_pressure_boost".to_string(),
        env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
        min_value: 0,
        max_value: 6,
        delta: 1,
        inverse_delta: -1,
    }];

    let plan = Phase90GeometryDrivenAdjustmentPlan {
        manifold_signature: "gauntlet_slice5_reject".to_string(),
        operator_count: 3,
        dominant_operator_kind: "continuity_lift".to_string(),
        aggregate_adjustment_pressure_percent: 72,
        operators: vec![
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_reject_1".to_string(),
                target_shape_index: 0,
                source_shape_index: 0,
                operator_kind: "continuity_lift".to_string(),
                adjustment_pressure_percent: 80,
                continuity_bias_percent: 70,
                manifold_alignment_percent: 75,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_reject_2".to_string(),
                target_shape_index: 1,
                source_shape_index: 0,
                operator_kind: "gradient_smooth".to_string(),
                adjustment_pressure_percent: 78,
                continuity_bias_percent: 74,
                manifold_alignment_percent: 71,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_reject_3".to_string(),
                target_shape_index: 0,
                source_shape_index: 1,
                operator_kind: "resonance_reweight".to_string(),
                adjustment_pressure_percent: 82,
                continuity_bias_percent: 68,
                manifold_alignment_percent: 73,
            },
        ],
        operator_plan_signature: "gauntlet_slice5_reject_sig".to_string(),
        operator_plan_hash: "gauntlet_slice5_reject_hash".to_string(),
        operator_plan_well_formed: true,
    };

    let err = phase10_execute_closed_loop_integrity_stage(
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
    )
    .expect_err("slice2 failure should fail closed-loop stage");

    assert!(err.contains("routed parameter diversity"));
}

#[test]
fn gauntlet_phase10_slice5_closed_loop_integrity_stage_is_replay_stable() {
    let mut registry = Phase70StructuralParameterRegistry::canonical();
    registry.parameters = vec![
        Phase70StructuralParameterSpec {
            name: "continuity_pressure_boost".to_string(),
            env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
            min_value: 0,
            max_value: 6,
            delta: 1,
            inverse_delta: -1,
        },
        Phase70StructuralParameterSpec {
            name: "coherence_gradient_trim".to_string(),
            env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
            min_value: 0,
            max_value: 4,
            delta: 1,
            inverse_delta: -1,
        },
    ];

    let plan = Phase90GeometryDrivenAdjustmentPlan {
        manifold_signature: "gauntlet_slice5_replay".to_string(),
        operator_count: 3,
        dominant_operator_kind: "continuity_lift".to_string(),
        aggregate_adjustment_pressure_percent: 72,
        operators: vec![
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_replay_1".to_string(),
                target_shape_index: 0,
                source_shape_index: 0,
                operator_kind: "continuity_lift".to_string(),
                adjustment_pressure_percent: 80,
                continuity_bias_percent: 70,
                manifold_alignment_percent: 75,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_replay_2".to_string(),
                target_shape_index: 1,
                source_shape_index: 0,
                operator_kind: "gradient_smooth".to_string(),
                adjustment_pressure_percent: 78,
                continuity_bias_percent: 74,
                manifold_alignment_percent: 71,
            },
            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: "gauntlet_slice5_replay_3".to_string(),
                target_shape_index: 0,
                source_shape_index: 1,
                operator_kind: "resonance_reweight".to_string(),
                adjustment_pressure_percent: 82,
                continuity_bias_percent: 68,
                manifold_alignment_percent: 73,
            },
        ],
        operator_plan_signature: "gauntlet_slice5_replay_sig".to_string(),
        operator_plan_hash: "gauntlet_slice5_replay_hash".to_string(),
        operator_plan_well_formed: true,
    };

    let stage_a = phase10_execute_closed_loop_integrity_stage(
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
    )
    .expect("stage a");
    let stage_b = phase10_execute_closed_loop_integrity_stage(
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
    )
    .expect("stage b");

    assert_eq!(stage_a, stage_b);
}

#[test]
fn gauntlet_phase10_slice6_feedback_regenerates_phase9_seed_material() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice6",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s6a1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s6a2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s6b1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s6b2", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let feedback = phase10_regenerate_phase9_seed_from_adapted_episode(
        "gauntlet_phase10_slice6_feedback",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
    )
    .expect("feedback");

    assert!(feedback.feedback_well_formed);
    assert!(feedback.regenerated_hook.integration_ready);
    assert!(feedback.regenerated_seed.geometry_well_formed);
}

#[test]
fn gauntlet_phase10_slice6_feedback_seed_regeneration_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice6_replay",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s6r1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s6r2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s6r3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s6r4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let feedback_a = phase10_regenerate_phase9_seed_from_adapted_episode(
        "gauntlet_phase10_slice6_feedback_replay",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
    )
    .expect("feedback a");
    let feedback_b = phase10_regenerate_phase9_seed_from_adapted_episode(
        "gauntlet_phase10_slice6_feedback_replay",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
    )
    .expect("feedback b");

    assert_eq!(feedback_a, feedback_b);
}

#[test]
fn gauntlet_phase10_slice6_feedback_telemetry_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice6_telem",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s6t1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s6t2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s6t3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s6t4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let feedback = phase10_regenerate_phase9_seed_from_adapted_episode(
        "gauntlet_phase10_slice6_feedback_telemetry",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
    )
    .expect("feedback");

    let telemetry_a = phase10_emit_runtime_feedback_telemetry(&feedback);
    let telemetry_b = phase10_emit_runtime_feedback_telemetry(&feedback);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("seed_hash="));
    assert!(telemetry_a.contains("hook_ready=true"));
    assert!(telemetry_a.contains("well_formed=true"));
}

#[test]
fn gauntlet_phase10_top_level_acceptance_stage_is_replay_stable_over_repeated_regenerations() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_top_level",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10ta1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10ta2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10tb1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10tb2", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let acceptance = phase10_run_top_level_acceptance_stage(
        "gauntlet_phase10_top_level_acceptance",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        50,
    )
    .expect("acceptance");

    assert!(acceptance.acceptance_well_formed);
    assert_eq!(acceptance.replay_count, 50);
}

#[test]
fn gauntlet_phase10_top_level_acceptance_stage_rejects_low_replay_count_with_canonical_code() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_top_level_reject",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10tr1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10tr2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10tr3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10tr4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let err = phase10_run_top_level_acceptance_stage(
        "gauntlet_phase10_top_level_acceptance_reject",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        1,
    )
    .expect_err("low replay count should fail acceptance");

    assert_eq!(err, "phase10_top_level_reject_replay_count_too_low");
}

#[test]
fn gauntlet_phase10_top_level_acceptance_telemetry_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_top_level_telem",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10tt1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10tt2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10tt3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10tt4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let acceptance = phase10_run_top_level_acceptance_stage(
        "gauntlet_phase10_top_level_acceptance_telemetry",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        20,
    )
    .expect("acceptance");

    let telemetry_a = phase10_emit_top_level_acceptance_telemetry(&acceptance);
    let telemetry_b = phase10_emit_top_level_acceptance_telemetry(&acceptance);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("replay_count=20"));
    assert!(telemetry_a.contains("baseline_feedback_hash="));
    assert!(telemetry_a.contains("baseline_seed_hash="));
    assert!(telemetry_a.contains("well_formed=true"));
}

#[test]
fn gauntlet_phase10_slice7_multicycle_replay_acceptance_is_well_formed() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice7",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s71", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s72", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s73", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s74", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let acceptance = phase10_run_slice7_multicycle_replay_acceptance_stage(
        "gauntlet_phase10_slice7_multicycle",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        7,
    )
    .expect("slice7 acceptance");

    assert!(acceptance.acceptance_well_formed);
    assert_eq!(acceptance.cycle_count, 7);
    assert_eq!(acceptance.cycle_plan_hashes.len(), 7);
}

#[test]
fn gauntlet_phase10_slice7_multicycle_replay_acceptance_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice7_replay",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s7r1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s7r2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s7r3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s7r4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let acceptance_a = phase10_run_slice7_multicycle_replay_acceptance_stage(
        "gauntlet_phase10_slice7_multicycle_replay",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        6,
    )
    .expect("acceptance a");
    let acceptance_b = phase10_run_slice7_multicycle_replay_acceptance_stage(
        "gauntlet_phase10_slice7_multicycle_replay",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        6,
    )
    .expect("acceptance b");

    assert_eq!(acceptance_a, acceptance_b);
}

#[test]
fn gauntlet_phase10_slice7_multicycle_replay_rejects_low_cycle_count_with_canonical_code() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice7_reject",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s7x1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s7x2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s7x3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s7x4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let err = phase10_run_slice7_multicycle_replay_acceptance_stage(
        "gauntlet_phase10_slice7_multicycle_reject",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        1,
    )
    .expect_err("low cycle count should fail");

    assert_eq!(err, "phase10_slice7_reject_cycle_count_too_low");
}

#[test]
fn gauntlet_phase10_slice7_multicycle_telemetry_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p10_slice7_telem",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s7t1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s7t2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p10s7t3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p10s7t4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let acceptance = phase10_run_slice7_multicycle_replay_acceptance_stage(
        "gauntlet_phase10_slice7_multicycle_telemetry",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        5,
    )
    .expect("slice7 acceptance");

    let telemetry_a = phase10_emit_slice7_multicycle_telemetry(&acceptance);
    let telemetry_b = phase10_emit_slice7_multicycle_telemetry(&acceptance);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("baseline_plan_hash="));
    assert!(telemetry_a.contains("cycle_count=5"));
    assert!(telemetry_a.contains("acceptance_hash="));
    assert!(telemetry_a.contains("well_formed=true"));
}

#[test]
fn gauntlet_phase11_multi_loop_convergence_is_well_formed() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p11_loop",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11l1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11l2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11l3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11l4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let convergence = phase11_run_multi_loop_convergence_stage(
        "gauntlet_phase11_multi_loop",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        &Phase11ConvergenceAcceptancePolicy::canonical(),
        5,
        4,
    )
    .expect("convergence");

    assert!(convergence.convergence_well_formed);
    assert_eq!(convergence.loop_count, 5);
    assert_eq!(convergence.cycle_count_per_loop, 4);
    assert_eq!(convergence.loop_acceptance_hashes.len(), 5);
    assert_eq!(convergence.convergence_metrics.acceptance_hash_stability_percent, 100);
    assert_eq!(convergence.convergence_metrics.terminal_plan_stability_percent, 100);
    assert_eq!(convergence.convergence_metrics.terminal_seed_stability_percent, 100);
    assert_eq!(convergence.convergence_metrics.convergence_radius, 0);
}

#[test]
fn gauntlet_phase11_multi_loop_convergence_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p11_loop_replay",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11r1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11r2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11r3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11r4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let stage_a = phase11_run_multi_loop_convergence_stage(
        "gauntlet_phase11_multi_loop_replay",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        &Phase11ConvergenceAcceptancePolicy::canonical(),
        4,
        4,
    )
    .expect("stage a");
    let stage_b = phase11_run_multi_loop_convergence_stage(
        "gauntlet_phase11_multi_loop_replay",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        &Phase11ConvergenceAcceptancePolicy::canonical(),
        4,
        4,
    )
    .expect("stage b");

    assert_eq!(stage_a, stage_b);
}

#[test]
fn gauntlet_phase11_multi_loop_convergence_rejects_low_loop_count_with_canonical_code() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p11_loop_reject",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11x1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11x2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11x3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11x4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let err = phase11_run_multi_loop_convergence_stage(
        "gauntlet_phase11_multi_loop_reject",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        &Phase11ConvergenceAcceptancePolicy::canonical(),
        1,
        4,
    )
    .expect_err("low loop count should fail");

    assert_eq!(err, "phase11_reject_loop_count_too_low");
}

#[test]
fn gauntlet_phase11_multi_loop_convergence_telemetry_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p11_loop_telem",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11t1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11t2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11t3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11t4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let convergence = phase11_run_multi_loop_convergence_stage(
        "gauntlet_phase11_multi_loop_telemetry",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        &Phase11ConvergenceAcceptancePolicy::canonical(),
        3,
        3,
    )
    .expect("convergence");

    let telemetry_a = phase11_emit_multi_loop_convergence_telemetry(&convergence);
    let telemetry_b = phase11_emit_multi_loop_convergence_telemetry(&convergence);

    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("baseline_plan_hash="));
    assert!(telemetry_a.contains("loop_count=3"));
    assert!(telemetry_a.contains("cycle_count_per_loop=3"));
    assert!(telemetry_a.contains("acceptance_hash_stability=100"));
    assert!(telemetry_a.contains("terminal_plan_stability=100"));
    assert!(telemetry_a.contains("terminal_seed_stability=100"));
    assert!(telemetry_a.contains("convergence_radius=0"));
    assert!(telemetry_a.contains("convergence_hash="));
    assert!(telemetry_a.contains("well_formed=true"));
}

#[test]
fn gauntlet_phase11_multi_loop_convergence_rejects_threshold_breach() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_p11_threshold_reject",
        &[
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11z1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11z2", "none", true, 1, 2, 1),
                ],
            }],
            vec![Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "p11z3", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "p11z4", "none", true, 1, 2, 1),
                ],
            }],
        ],
        &registry,
    );

    let strict = Phase11ConvergenceAcceptancePolicy {
        min_acceptance_hash_stability_percent: 101,
        min_terminal_plan_stability_percent: 100,
        min_terminal_seed_stability_percent: 100,
        max_convergence_radius: 0,
    };

    let err = phase11_run_multi_loop_convergence_stage(
        "gauntlet_phase11_multi_loop_threshold_reject",
        &plan,
        &registry,
        &Phase10Slice2RoutingAcceptancePolicy::canonical(),
        &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        &strict,
        3,
        3,
    )
    .expect_err("strict policy should reject below-threshold stability");

    assert_eq!(err, "phase11_reject_acceptance_hash_stability_below_min");
}

#[test]
fn gauntlet_phase12_emergent_programs_gate_is_replay_stable() {
    let registry = Phase70StructuralParameterRegistry::canonical();
    let (_manifold, _dynamics, plan) = phase90_plan_fixture(
        "gauntlet_phase12_programs",
        &[
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p12a1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p12a2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p12a3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p12a4", "none", false, 1, 1, 0),
                        entry(3, "p12a5", "none", true, 1, 2, 1),
                    ],
                },
            ],
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p12b1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p12b2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "p12b3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "p12b4", "none", true, 1, 2, 1),
                        entry(3, "p12b5", "none", false, 2, 2, 0),
                    ],
                },
            ],
        ],
        &registry,
    );

    let baseline_program = phase12_synthesize_emergent_cognitive_program(&_manifold, &plan)
        .expect("baseline program");
    let baseline_telemetry = phase12_emit_program_telemetry(&baseline_program);

    assert!(baseline_program.program_well_formed);
    assert!(!baseline_program.program_signature.is_empty());
    assert!(!baseline_program.program_profile_hash.is_empty());
    assert!(baseline_telemetry.contains("program_kind="));
    assert!(baseline_telemetry.contains("step_count="));
    assert!(baseline_telemetry.contains("well_formed=true"));

    const PHASE12_REPLAY_LOOPS: usize = 100;
    for _ in 0..PHASE12_REPLAY_LOOPS {
        let current_program = phase12_synthesize_emergent_cognitive_program(&_manifold, &plan)
            .expect("current program");
        let current_telemetry = phase12_emit_program_telemetry(&current_program);

        assert_eq!(current_program, baseline_program);
        assert_eq!(current_program.program_signature, baseline_program.program_signature);
        assert_eq!(current_program.program_profile_hash, baseline_program.program_profile_hash);
        assert_eq!(current_telemetry, baseline_telemetry);
    }

    // Emit parseable Phase 12 summary for gauntlet script extraction.
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        baseline_telemetry.hash(&mut h);
        let telemetry_digest = format!("{:016x}", h.finish());
        println!(
            "PHASE12_SUMMARY:verdict={}|signature_hash={}|operator_plan_size={}|resonance_gate={}|telemetry_digest={}|replay_loops={}",
            baseline_program.program_well_formed,
            baseline_program.program_profile_hash,
            baseline_program.step_count,
            baseline_program.resonance_gate_percent,
            telemetry_digest,
            PHASE12_REPLAY_LOOPS,
        );
    }
}

#[test]
fn gauntlet_phase13_qubit_state_invariants_are_enforced() {
    let stable = phase13_build_qubit_state(0, 0, 1).expect("state");
    phase13_validate_qubit_state_invariants(&stable).expect("valid state invariants");

    let telemetry = phase13_emit_state_telemetry(&stable);
    assert!(telemetry.contains("norm_error=0"));
    assert!(telemetry.contains("well_formed=true"));

    let err = phase13_build_qubit_state(1, 1, 0).expect_err("non-unit Bloch vector must fail");
    assert_eq!(err, "phase13_invalid_state_norm: expected 1, observed 2");
}

#[test]
fn gauntlet_phase13_unitary_invariants_are_enforced() {
    let ops = [
        Phase13QubitUnaryOp::PauliX,
        Phase13QubitUnaryOp::PauliZ,
        Phase13QubitUnaryOp::FixedHadamard,
    ];

    for op in ops {
        phase13_validate_unitary_invariants(op).expect("unitary invariants");
        assert!(!phase13_unitary_signature(op).is_empty());
    }

    let commute = phase13_ops_commute(Phase13QubitUnaryOp::PauliX, Phase13QubitUnaryOp::PauliZ)
        .expect("x/z composition should be computable");
    assert!(!commute, "expected PauliX and PauliZ to be non-commuting");
}

#[test]
fn gauntlet_phase13_evolution_replay_gate_is_byte_stable() {
    let baseline_initial = phase13_build_qubit_state(0, 0, 1).expect("initial state");
    let op_plan = [
        Phase13QubitUnaryOp::FixedHadamard,
        Phase13QubitUnaryOp::PauliZ,
        Phase13QubitUnaryOp::PauliX,
        Phase13QubitUnaryOp::FixedHadamard,
    ];

    let baseline_sequence = phase13_apply_unitary_sequence(&baseline_initial, &op_plan)
        .expect("baseline evolution");
    let baseline_final = baseline_sequence.final_state.clone();
    phase13_validate_qubit_state_invariants(&baseline_final).expect("post-state invariants");
    let baseline_state_telemetry = phase13_emit_state_telemetry(&baseline_final);
    let baseline_measurement = phase13_measure_z(&baseline_final).expect("baseline measurement");
    let baseline_measurement_telemetry = phase13_emit_measurement_telemetry(&baseline_measurement);

    const PHASE13_REPLAY_LOOPS: usize = 100;
    for _ in 0..PHASE13_REPLAY_LOOPS {
        let current_sequence = phase13_apply_unitary_sequence(&baseline_initial, &op_plan)
            .expect("current evolution");
        let current_final = current_sequence.final_state;
        let current_state_telemetry = phase13_emit_state_telemetry(&current_final);
        let current_measurement = phase13_measure_z(&current_final).expect("current measurement");
        let current_measurement_telemetry = phase13_emit_measurement_telemetry(&current_measurement);

        assert_eq!(current_final, baseline_final);
        assert_eq!(current_sequence.op_sequence_signature, baseline_sequence.op_sequence_signature);
        assert_eq!(current_sequence.evolution_signature, baseline_sequence.evolution_signature);
        assert_eq!(current_state_telemetry, baseline_state_telemetry);
        assert_eq!(current_measurement, baseline_measurement);
        assert_eq!(current_measurement_telemetry, baseline_measurement_telemetry);
    }

    {
        println!(
            "PHASE13_SUMMARY:verdict={}|state_signature={}|evolution_signature={}|measurement_signature={}|replay_loops={}",
            baseline_final.state_well_formed,
            baseline_final.state_signature,
            baseline_sequence.evolution_signature,
            baseline_measurement.measurement_signature,
            PHASE13_REPLAY_LOOPS,
        );
    }
}

#[test]
fn gauntlet_phase13_measurement_contract_gate_is_fixture_deterministic() {
    let fixtures = [
        (phase13_build_qubit_state(0, 0, 1).expect("|0>"), 0u8),
        (phase13_build_qubit_state(0, 0, -1).expect("|1>"), 1u8),
        (phase13_build_qubit_state(1, 0, 0).expect("|+x>"), 0u8),
        (phase13_build_qubit_state(-1, 0, 0).expect("|-x>"), 1u8),
    ];

    for (state, expected_bit) in fixtures {
        let a = phase13_measure_z(&state).expect("measurement a");
        let b = phase13_measure_z(&state).expect("measurement b");

        assert_eq!(a, b);
        assert_eq!(a.bit, expected_bit);
        if expected_bit == 0 {
            assert_eq!(a.post_state, phase13_build_qubit_state(0, 0, 1).expect("|0>"));
        } else {
            assert_eq!(a.post_state, phase13_build_qubit_state(0, 0, -1).expect("|1>"));
        }
    }
}

// ── Phase 14 gauntlet gates ───────────────────────────────────────────────────

#[test]
fn gauntlet_phase14_operator_family_invariants_are_enforced() {
    let pauli = phase14_build_pauli_family();
    phase14_validate_family_invariants(&pauli).expect("pauli family invariants");

    assert_eq!(pauli.family_kind, Phase14OperatorFamilyKind::PauliGroup);
    assert!(pauli.family_well_formed);
    assert_eq!(pauli.ops.len(), 2);

    let telemetry = phase14_emit_family_telemetry(&pauli);
    assert!(telemetry.contains("family=pauli_xz"));
    assert!(telemetry.contains("well_formed=true"));
}

#[test]
fn gauntlet_phase14_commutator_non_commutation_is_enforced() {
    // [X, Z] must be non-zero — this is the foundational non-commutative invariant.
    let xz = phase14_compute_commutator(
        Phase13QubitUnaryOp::PauliX,
        Phase13QubitUnaryOp::PauliZ,
    )
    .expect("commutator xz");
    assert!(!xz.is_zero, "X and Z must not commute");

    // [X, X] must be zero — same operator commutes with itself.
    let xx = phase14_compute_commutator(
        Phase13QubitUnaryOp::PauliX,
        Phase13QubitUnaryOp::PauliX,
    )
    .expect("commutator xx");
    assert!(xx.is_zero, "[X, X] must be zero");

    // Anti-symmetry: [X, Z] = -[Z, X] (off-diagonal signs flip).
    let zx = phase14_compute_commutator(
        Phase13QubitUnaryOp::PauliZ,
        Phase13QubitUnaryOp::PauliX,
    )
    .expect("commutator zx");
    for r in 0..2 {
        for c in 0..2 {
            assert_eq!(
                xz.matrix[r][c],
                -zx.matrix[r][c],
                "anti-symmetry violation at [{r},{c}]"
            );
        }
    }
}

#[test]
fn gauntlet_phase14_commutation_table_replay_gate_is_byte_stable() {
    let family = phase14_build_pauli_family();
    let table_a = phase14_build_commutation_table(&family);
    let table_b = phase14_build_commutation_table(&family);

    assert_eq!(table_a, table_b);
    phase14_validate_table_invariants(&table_a, &family).expect("table invariants");

    let telemetry_a = phase14_emit_algebra_telemetry(&family, &table_a);
    let telemetry_b = phase14_emit_algebra_telemetry(&family, &table_b);
    assert_eq!(telemetry_a, telemetry_b);
    assert!(telemetry_a.contains("non_commuting=1"));
    assert!(telemetry_a.contains("well_formed=true"));

    const PHASE14_REPLAY_LOOPS: usize = 100;
    for _ in 0..PHASE14_REPLAY_LOOPS {
        let t = phase14_build_commutation_table(&family);
        assert_eq!(t, table_a);
    }

    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        telemetry_a.hash(&mut h);
        let digest = format!("{:016x}", h.finish());

        println!(
            "PHASE14_SUMMARY:verdict={}|family_signature={}|table_signature={}|telemetry_digest={}|replay_loops={}",
            family.family_well_formed,
            family.family_signature,
            table_a.table_signature,
            digest,
            PHASE14_REPLAY_LOOPS,
        );
    }
}

#[test]
fn gauntlet_phase14_clifford_family_gate_is_well_formed() {
    let clifford = phase14_build_clifford_family();
    phase14_validate_family_invariants(&clifford).expect("clifford invariants");

    assert_eq!(clifford.ops.len(), 3);
    assert_eq!(clifford.family_kind, Phase14OperatorFamilyKind::CliffordFamily);

    let table = phase14_build_commutation_table(&clifford);
    phase14_validate_table_invariants(&table, &clifford).expect("clifford table invariants");

    // Pairs involving FixedHadamard carry the irrational sentinel.
    let h_pairs: Vec<_> = table.pairs.iter()
        .filter(|p| p.lhs_label == "FixedHadamard" || p.rhs_label == "FixedHadamard")
        .collect();
    assert!(!h_pairs.is_empty());
    assert!(h_pairs.iter().all(|p| p.commutator.commutator_signature == "irrational_pair_sentinel"));
}
