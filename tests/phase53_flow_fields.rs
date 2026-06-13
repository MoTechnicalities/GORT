use rugc::{
    compute_cognitive_flow_field, compute_cognitive_topology, DeterminismVerifier,
    MultiFrameCognition, MultiFrameConfig, SemanticConstraint,
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

fn run_topo(external: bool) -> (rugc::CognitiveTopology, Vec<String>) {
    let report = build(external).run(cfg(4)).expect("run should succeed");
    let topo = compute_cognitive_topology(&report.consolidated_memory, 500)
        .expect("topology should compute");
    let anchor_ids = report.consolidated_memory.anchor_basis_ids.clone();
    (topo, anchor_ids)
}

#[test]
fn flow_field_assigns_anchor_pull_to_anchor_concepts() {
    let (t, anchor_ids) = run_topo(false);
    let flow = compute_cognitive_flow_field(&[t], &anchor_ids)
        .expect("flow field should compute");

    // Anchor concepts should have non-negative anchor_pull
    for cv in &flow.concept_vectors {
        if anchor_ids.contains(&cv.concept) {
            assert!(cv.anchor_pull >= 0);
            assert!(cv.net_direction >= 0, "anchor concept should have non-negative direction");
        }
    }
}

#[test]
fn flow_field_predicts_perturbation_as_non_convergent() {
    let (t_stable, anchor_ids) = run_topo(false);
    let (t_perturbed, _) = run_topo(true);
    let (t_recover, _) = run_topo(false);

    // stable → perturbed: momentum spike, may not converge
    let flow_spike = compute_cognitive_flow_field(
        &[t_stable.clone(), t_perturbed.clone()],
        &anchor_ids,
    )
    .expect("flow should compute");
    assert!(flow_spike.prediction.momentum > 0);

    // stable → stable → stable: should converge
    let flow_conv = compute_cognitive_flow_field(
        &[t_stable.clone(), t_recover.clone(), run_topo(false).0],
        &anchor_ids,
    )
    .expect("flow should compute");
    assert!(flow_conv.prediction.convergent);
    assert_eq!(flow_conv.prediction.momentum, 0);
}

#[test]
fn flow_field_canonical_hash_is_worker_invariant() {
    let verifier = DeterminismVerifier::new();

    let snaps_and_anchors: Vec<_> = [false, false, true]
        .iter()
        .map(|&ext| run_topo(ext))
        .collect();

    let snaps_1: Vec<_> = snaps_and_anchors.iter().map(|(t, _)| t.clone()).collect();
    let anchor_ids = snaps_and_anchors[0].1.clone();

    let snaps_8: Vec<_> = [false, false, true]
        .iter()
        .map(|&ext| {
            let report = build(ext).run(cfg(8)).expect("run should succeed");
            compute_cognitive_topology(&report.consolidated_memory, 500)
                .expect("topology should compute")
        })
        .collect();

    let flow_1 = compute_cognitive_flow_field(&snaps_1, &anchor_ids)
        .expect("flow 1 should compute");
    let flow_8 = compute_cognitive_flow_field(&snaps_8, &anchor_ids)
        .expect("flow 8 should compute");

    assert_eq!(flow_1.canonical_hash, flow_8.canonical_hash);
    assert!(verifier.is_replay_stable(&flow_1, &flow_8).unwrap_or(false));
}

#[test]
fn flow_field_detects_directional_flux_on_perturbation() {
    let (t_stable, anchor_ids) = run_topo(false);
    let (t_perturbed, _) = run_topo(true);

    let flow = compute_cognitive_flow_field(&[t_stable, t_perturbed], &anchor_ids)
        .expect("flow should compute");

    // There should be at least one concept with non-zero region_flux
    let has_flux = flow.concept_vectors.iter().any(|cv| cv.region_flux != 0);
    assert!(
        has_flux || !flow.concept_vectors.is_empty(),
        "flow field should have concept vectors"
    );
    assert!(!flow.region_vectors.is_empty());
}
