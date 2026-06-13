use rugc::{DeterminismVerifier, MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

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

fn build() -> MultiFrameCognition {
    let mut mfc = MultiFrameCognition::new();
    mfc.register_frame(
        "physics",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 92),
            SemanticConstraint::assertion("light", "particle", true, 88),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
            SemanticConstraint::assertion("photon", "is_quantized", true, 64),
        ],
    );
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

#[test]
fn emergent_concepts_form_from_anchor_aligned_clusters() {
    let mut mfc = build();
    let report = mfc.run(cfg(4)).expect("run should succeed");

    assert!(!report.consolidated_memory.emergent_concepts.is_empty());
    assert!(report
        .consolidated_memory
        .emergent_concepts
        .iter()
        .all(|c| c.persistence_hits >= 2));
    assert!(report
        .consolidated_memory
        .ontology_expansion_score
        > 0);
}

#[test]
fn emergent_concept_registry_is_worker_invariant() {
    let verifier = DeterminismVerifier::new();
    let mut a = build();
    let mut b = build();

    let ra = a.run(cfg(1)).expect("run A should succeed");
    let rb = b.run(cfg(8)).expect("run B should succeed");

    assert_eq!(
        ra.consolidated_memory.emergent_concepts,
        rb.consolidated_memory.emergent_concepts
    );
    assert!(verifier
        .is_replay_stable(&ra.consolidated_memory, &rb.consolidated_memory)
        .unwrap_or(false));
}

#[test]
fn emergent_constraints_expand_internal_ontology() {
    let mut mfc = build();
    let report = mfc.run(cfg(4)).expect("run should succeed");
    let last = report
        .iterations
        .last()
        .expect("expected at least one iteration");

    assert!(last.metrics.emergent_concepts_active > 0);
    assert!(last.metrics.emergent_candidates > 0);
    assert!(report
        .consolidated_memory
        .fused_constraints
        .iter()
        .any(|c| c.predicate.starts_with("emergent/")));
}
