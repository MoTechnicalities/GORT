use rugc::{DeterminismVerifier, MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

fn build_conflicting_mfc() -> MultiFrameCognition {
    let mut mfc = MultiFrameCognition::new();

    mfc.register_frame(
        "physics",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 92),
            SemanticConstraint::assertion("light", "particle", true, 88),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
        ],
    );

    mfc.register_frame(
        "ontology",
        vec![
            SemanticConstraint::assertion("light", "wave", false, 30),
            SemanticConstraint::assertion("light", "particle", true, 65),
            SemanticConstraint::assertion("vacuum", "has_medium", true, 20),
        ],
    );

    mfc.register_frame(
        "observation",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 50),
            SemanticConstraint::assertion("light", "particle", true, 52),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 40),
        ],
    );

    mfc
}

fn convergence_config(worker_count: usize) -> MultiFrameConfig {
    MultiFrameConfig {
        iterations: 10,
        worker_count,
        ambiguity_margin: 5000,
        target_energy: 500,
        compression_threshold: 1,
        convergence_window: 2,
        energy_delta_threshold: 2,
        anchor_energy_max: 500,
        anchor_pull_strength: 4,
        anchor_min_persistence: 2,
    }
}

#[test]
fn converges_within_configured_iterations() {
    let mut mfc = build_conflicting_mfc();
    let cfg = convergence_config(4);
    let report = mfc.run(cfg).expect("run should succeed");

    let k = report.converged_iteration.expect("expected convergence");
    assert!(k < cfg.iterations);
    assert_eq!(report.consolidated_memory.converged_iteration, Some(k));
    assert!(!report.consolidated_memory.artifact_hash.is_empty());
}

#[test]
fn convergence_artifact_hash_is_worker_invariant() {
    let verifier = DeterminismVerifier::new();
    let mut a = build_conflicting_mfc();
    let mut b = build_conflicting_mfc();

    let ra = a
        .run(convergence_config(1))
        .expect("single-worker run should succeed");
    let rb = b
        .run(convergence_config(8))
        .expect("multi-worker run should succeed");

    assert_eq!(ra.consolidated_memory.artifact_hash, rb.consolidated_memory.artifact_hash);
    assert!(
        verifier
            .is_replay_stable(&ra.consolidated_memory, &rb.consolidated_memory)
            .unwrap_or(false)
    );
}

#[test]
fn convergence_artifact_hash_is_stable_across_replays() {
    let verifier = DeterminismVerifier::new();
    let mut baseline_mfc = build_conflicting_mfc();
    let baseline = baseline_mfc
        .run(convergence_config(4))
        .expect("baseline run should succeed")
        .consolidated_memory;

    for _ in 0..8 {
        let mut next_mfc = build_conflicting_mfc();
        let next = next_mfc
            .run(convergence_config(4))
            .expect("replay run should succeed")
            .consolidated_memory;

        assert_eq!(baseline.artifact_hash, next.artifact_hash);
        assert!(verifier.is_replay_stable(&baseline, &next).unwrap_or(false));
    }
}
