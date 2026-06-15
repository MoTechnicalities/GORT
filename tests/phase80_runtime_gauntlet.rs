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
