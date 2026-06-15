use gort::{
    compare_topologies, compute_cognitive_topology, detect_phase_transition,
    track_manifold_evolution, DeterminismVerifier, MultiFrameCognition, MultiFrameConfig,
    SemanticConstraint,
};

fn cfg(workers: usize) -> MultiFrameConfig {
    MultiFrameConfig {
        iterations: 12,
        worker_count: workers,
        ambiguity_margin: 5000,
        target_energy: 500,
        compression_threshold: 1,
        convergence_window: 2,
        energy_delta_threshold: 2,
        anchor_energy_max: 2000,
        anchor_pull_strength: 4,
        anchor_min_persistence: 1,
        anchor_alignment_window: 25,
        anchor_contradiction_highlight: 6,
        anchor_fusion_bias: 8,
        emergent_min_cluster_size: 2,
        emergent_min_anchor_support: 1,
        emergent_resonance_threshold: 40,
        emergent_min_persistence: 2,
        emergent_constraint_weight: 36,
    }
}

fn build(external: bool) -> MultiFrameCognition {
    let mut mfc = MultiFrameCognition::new();
    let mut physics = vec![
        SemanticConstraint::assertion("light", "wave", true, 92),
        SemanticConstraint::assertion("light", "particle", true, 88),
        SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
        SemanticConstraint::assertion("photon", "is_quantized", true, 64),
    ];
    if external {
        physics.push(SemanticConstraint::assertion("observer", "injects_noise", true, 70));
    }
    mfc.register_frame("physics", physics);
    mfc.register_frame(
        "ontology",
        vec![
            SemanticConstraint::assertion("light", "wave", false, 30),
            SemanticConstraint::assertion("light", "particle", true, 65),
            SemanticConstraint::assertion("vacuum", "has_medium", true, 20),
            SemanticConstraint::assertion("photon", "is_quantized", true, 58),
        ],
    );
    mfc.register_frame(
        "observation",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 50),
            SemanticConstraint::assertion("light", "particle", true, 52),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 40),
            SemanticConstraint::assertion("photon", "is_quantized", true, 60),
        ],
    );
    mfc
}

fn topo(mfc: &mut MultiFrameCognition) -> gort::CognitiveTopology {
    let report = mfc.run(cfg(4)).expect("run should succeed");
    compute_cognitive_topology(&report.consolidated_memory, 500)
        .expect("topology should compute")
}

#[test]
fn manifold_drift_detects_topology_changes() {
    let t_stable = topo(&mut build(false));
    let t_perturbed = topo(&mut build(true));

    let drift = compare_topologies(&t_stable, &t_perturbed);
    assert!(drift.hash_changed);
    assert!(drift.drift_score > 0);
    assert!(drift.region_delta > 0 || !drift.added_regions.is_empty());
}

#[test]
fn manifold_drift_is_zero_for_identical_topologies() {
    let t1 = topo(&mut build(false));
    let t2 = topo(&mut build(false));

    let drift = compare_topologies(&t1, &t2);
    assert!(!drift.hash_changed);
    assert_eq!(drift.drift_score, 0);
    assert!(drift.added_regions.is_empty());
    assert!(drift.removed_regions.is_empty());
}

#[test]
fn phase_transition_detected_after_external_injection() {
    let t_stable = topo(&mut build(false));
    let t_perturbed = topo(&mut build(true));

    let drift = compare_topologies(&t_stable, &t_perturbed);
    assert!(detect_phase_transition(&drift, 1));
}

#[test]
fn evolution_trace_is_worker_invariant() {
    let verifier = DeterminismVerifier::new();

    let snapshots_1: Vec<_> = (0..3)
        .map(|i| {
            let mut mfc = build(i % 2 == 1);
            let report = mfc.run(cfg(1)).expect("run should succeed");
            compute_cognitive_topology(&report.consolidated_memory, 500)
                .expect("topology should compute")
        })
        .collect();

    let snapshots_8: Vec<_> = (0..3)
        .map(|i| {
            let mut mfc = build(i % 2 == 1);
            let report = mfc.run(cfg(8)).expect("run should succeed");
            compute_cognitive_topology(&report.consolidated_memory, 500)
                .expect("topology should compute")
        })
        .collect();

    let trace_1 = track_manifold_evolution(&snapshots_1, 200).expect("trace 1 should compute");
    let trace_8 = track_manifold_evolution(&snapshots_8, 200).expect("trace 8 should compute");

    assert_eq!(trace_1.canonical_hash, trace_8.canonical_hash);
    assert_eq!(
        trace_1.phase_transition_steps,
        trace_8.phase_transition_steps
    );
    assert!(verifier.is_replay_stable(&trace_1, &trace_8).unwrap_or(false));
}

#[test]
fn persistent_regions_survive_across_stable_snapshots() {
    let snapshots: Vec<_> = (0..4)
        .map(|_| topo(&mut build(false)))
        .collect();

    let trace = track_manifold_evolution(&snapshots, 200).expect("trace should compute");
    assert!(!trace.persistent_region_ids.is_empty());
    assert_eq!(trace.phase_transition_steps.len(), 0);
    assert_eq!(trace.overall_stability, 1000);
}
