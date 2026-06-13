use rugc::{DeterminismVerifier, MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

fn cfg(workers: usize) -> MultiFrameConfig {
    MultiFrameConfig {
        iterations: 10,
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
    }
}

fn build() -> MultiFrameCognition {
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

#[test]
fn anchor_weighted_metrics_improve_during_loop() {
    let mut mfc = build();
    let report = mfc.run(cfg(4)).expect("run should succeed");
    assert!(report.iterations.len() >= 2);

    let first = &report.iterations[0].metrics;
    let last = &report.iterations[report.iterations.len() - 1].metrics;

    assert!(last.anchor_field_coherence >= first.anchor_field_coherence);
    assert!(last.anchor_drift <= first.anchor_drift || last.anchor_drift <= 2);
}

#[test]
fn anchor_weighted_interpretation_is_worker_invariant() {
    let verifier = DeterminismVerifier::new();
    let mut a = build();
    let mut b = build();

    let ra = a.run(cfg(1)).expect("run A should succeed");
    let rb = b.run(cfg(8)).expect("run B should succeed");

    assert_eq!(ra.consolidated_memory.anchor_basis_hash, rb.consolidated_memory.anchor_basis_hash);
    assert!(verifier.is_replay_stable(&ra.consolidated_memory, &rb.consolidated_memory).unwrap_or(false));
}

#[test]
fn anchor_guided_fusion_reduces_highlighted_contradictions() {
    let mut mfc = build();
    let report = mfc.run(cfg(4)).expect("run should succeed");

    let first = &report.iterations[0].metrics;
    let last = &report.iterations[report.iterations.len() - 1].metrics;

    assert!(last.anchor_contradictions_highlighted <= first.anchor_contradictions_highlighted);
}
