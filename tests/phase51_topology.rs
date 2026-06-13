use rugc::{
    compute_cognitive_topology, DeterminismVerifier, MultiFrameCognition, MultiFrameConfig,
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

#[test]
fn topology_forms_distinct_regions_from_anchors() {
    let mut mfc = build(false);
    let report = mfc.run(cfg(4)).expect("run should succeed");
    let topo = compute_cognitive_topology(&report.consolidated_memory, 500)
        .expect("topology should compute");

    assert!(topo.metrics.total_concepts > 0);
    assert!(topo.metrics.region_count >= 1);
    assert!(!topo.canonical_hash.is_empty());
    assert!(topo
        .regions
        .iter()
        .all(|r| !r.id.is_empty() && !r.representative.is_empty()));
}

#[test]
fn topology_canonical_hash_is_worker_invariant() {
    let verifier = DeterminismVerifier::new();
    let mut a = build(false);
    let mut b = build(false);

    let ra = a.run(cfg(1)).expect("run A should succeed");
    let rb = b.run(cfg(8)).expect("run B should succeed");

    assert!(verifier
        .is_replay_stable(&ra.consolidated_memory, &rb.consolidated_memory)
        .unwrap_or(false));

    let ta = compute_cognitive_topology(&ra.consolidated_memory, 500)
        .expect("topology A should compute");
    let tb = compute_cognitive_topology(&rb.consolidated_memory, 500)
        .expect("topology B should compute");

    assert_eq!(ta.canonical_hash, tb.canonical_hash);
    assert_eq!(ta.metrics.region_count, tb.metrics.region_count);
}

#[test]
fn topology_changes_under_external_perturbation() {
    let mut baseline = build(false);
    let mut perturbed = build(true);

    let rb = baseline.run(cfg(4)).expect("baseline should run");
    let rp = perturbed.run(cfg(4)).expect("perturbed should run");

    let tb = compute_cognitive_topology(&rb.consolidated_memory, 500)
        .expect("baseline topology should compute");
    let tp = compute_cognitive_topology(&rp.consolidated_memory, 500)
        .expect("perturbed topology should compute");

    assert_ne!(
        tb.canonical_hash, tp.canonical_hash,
        "external perturbation should alter topology"
    );
    assert!(tp.metrics.total_concepts >= tb.metrics.total_concepts);
}
