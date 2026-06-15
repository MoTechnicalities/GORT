use gort::{
    arbitrate_intent_field, build_meta_intent_field, compute_cognitive_flow_field,
    compute_cognitive_potential_field, compute_cognitive_topology, compute_intent_field,
    compute_meta_preference_gradients, resolve_meta_intent_trajectory, DeterminismVerifier,
    MultiFrameCognition, MultiFrameConfig, SemanticConstraint,
};
use std::collections::BTreeMap;

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

fn run_pipeline(
    external: bool,
    workers: usize,
) -> (gort::CognitivePotentialField, gort::ArbitratedIntentField, Vec<String>) {
    let report = build(external).run(cfg(workers)).expect("run should succeed");
    let mem = &report.consolidated_memory;
    let anchors = mem.anchor_basis_ids.clone();

    let topo1 = compute_cognitive_topology(mem, 500).expect("topo1 should compute");
    let topo2 = compute_cognitive_topology(mem, 500).expect("topo2 should compute");

    let flow = compute_cognitive_flow_field(&[topo1, topo2], &anchors)
        .expect("flow should compute");
    let potential = compute_cognitive_potential_field(&flow).expect("potential should compute");
    let intent = compute_intent_field(&potential, &anchors).expect("intent should compute");

    let base_weights: BTreeMap<String, i64> = potential
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.attraction_strength))
        .collect();

    let arbitrated = arbitrate_intent_field(&[intent.clone(), intent], &potential, &base_weights)
        .expect("arbitration should compute");

    (potential, arbitrated, anchors)
}

#[test]
fn gate_at_meta_intent_field_builds_hierarchy_and_coherence() {
    let (_potential, arbitrated, _) = run_pipeline(false, 4);

    let meta = build_meta_intent_field(&arbitrated, &[])
        .expect("meta intent should compute");

    assert!(!meta.hierarchy.layers.is_empty());
    assert!(!meta.canonical_hash.is_empty());
    assert!(meta.self_coherence.self_consistency >= 0);
}

#[test]
fn gate_au_meta_preference_gradients_capture_goal_modulation() {
    let (_potential, arbitrated, _) = run_pipeline(false, 4);
    let meta = build_meta_intent_field(&arbitrated, &[])
        .expect("meta intent should compute");

    let gradients = compute_meta_preference_gradients(&arbitrated, &meta.hierarchy)
        .expect("meta preference gradients should compute");

    assert!(!gradients.is_empty());
    assert!(
        gradients.iter().any(|g| g.net_meta_pull != 0),
        "expected non-zero meta pull among goal pairs"
    );
}

#[test]
fn gate_av_self_coherence_drops_under_perturbation() {
    let (_p_stable, arb_stable, _) = run_pipeline(false, 4);
    let (_p_perturbed, arb_perturbed, _) = run_pipeline(true, 4);

    let stable_meta = build_meta_intent_field(&arb_stable, &[])
        .expect("stable meta should compute");
    let perturbed_meta = build_meta_intent_field(
        &arb_perturbed,
        &[stable_meta.canonical_hash.clone()],
    )
    .expect("perturbed meta should compute");

    assert!(
        perturbed_meta.self_coherence.self_consistency
            <= stable_meta.self_coherence.self_consistency,
        "perturbation should not improve self consistency"
    );
}

#[test]
fn gate_aw_meta_trajectory_revises_goals_under_pressure() {
    let (potential, arbitrated, _) = run_pipeline(true, 4);

    let current = potential
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("must have a current region");

    let history = vec!["meta_hash_prev_1".to_string(), "meta_hash_prev_2".to_string()];
    let traj = resolve_meta_intent_trajectory(&arbitrated, &potential, &current, &history)
        .expect("meta trajectory should compute");

    assert!(!traj.selected_path.is_empty());
    assert!(!traj.canonical_hash.is_empty());

    if traj.coherence_metric.self_consistency < 600 {
        assert!(
            !traj.revised_goals.is_empty(),
            "low consistency should trigger goal revision"
        );
    }
}

#[test]
fn gate_ax_meta_hash_worker_invariant() {
    let verifier = DeterminismVerifier::new();

    let (p1, a1, _) = run_pipeline(false, 1);
    let (p8, a8, _) = run_pipeline(false, 8);

    let m1 = build_meta_intent_field(&a1, &[]).expect("m1 should compute");
    let m8 = build_meta_intent_field(&a8, &[]).expect("m8 should compute");

    assert!(verifier.is_replay_stable(&m1, &m8).unwrap_or(false));

    let c1 = p1
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("current 1");
    let c8 = p8
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("current 8");

    let t1 = resolve_meta_intent_trajectory(&a1, &p1, &c1, &[])
        .expect("t1 should compute");
    let t8 = resolve_meta_intent_trajectory(&a8, &p8, &c8, &[])
        .expect("t8 should compute");

    assert_eq!(t1.canonical_hash, t8.canonical_hash);
}
