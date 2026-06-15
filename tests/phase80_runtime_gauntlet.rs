use gort::{
    Phase70AdjustmentLog, Phase70AdjustmentLogEntry, Phase70StructuralParameterRegistry,
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
