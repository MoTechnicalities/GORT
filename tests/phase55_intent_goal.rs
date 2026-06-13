use rugc::{
    compute_cognitive_flow_field, compute_cognitive_potential_field, compute_cognitive_topology,
    compute_intent_field, compute_preference_gradients, select_goal_directed_trajectory,
    DeterminismVerifier, MultiFrameCognition, MultiFrameConfig, SemanticConstraint,
};
use std::collections::BTreeSet;

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

fn run_topo(external: bool, workers: usize) -> (rugc::CognitiveTopology, Vec<String>) {
    let report = build(external).run(cfg(workers)).expect("run should succeed");
    let topo = compute_cognitive_topology(&report.consolidated_memory, 500)
        .expect("topology should compute");
    (topo, report.consolidated_memory.anchor_basis_ids.clone())
}

#[test]
fn gate_aj_intent_field_forms_goal_attractors() {
    let (stable_topo, anchor_ids) = run_topo(false, 4);
    let flow = compute_cognitive_flow_field(&[stable_topo.clone(), stable_topo], &anchor_ids)
        .expect("flow should compute");
    let potential = compute_cognitive_potential_field(&flow)
        .expect("potential should compute");

    let intent = compute_intent_field(&potential, &anchor_ids)
        .expect("intent field should compute");

    assert!(!intent.goal_attractors.is_empty());
    assert!(!intent.preferred_regions.is_empty());
    assert!(!intent.canonical_hash.is_empty());
}

#[test]
fn gate_ak_preference_gradients_reward_goal_targets() {
    let (stable_topo, anchor_ids) = run_topo(false, 4);
    let (perturbed_topo, _) = run_topo(true, 4);

    let flow = compute_cognitive_flow_field(&[stable_topo, perturbed_topo], &anchor_ids)
        .expect("flow should compute");
    let potential = compute_cognitive_potential_field(&flow)
        .expect("potential should compute");
    let intent = compute_intent_field(&potential, &anchor_ids)
        .expect("intent field should compute");

    let gradients = compute_preference_gradients(&potential, &intent)
        .expect("preference gradients should compute");
    assert!(!gradients.is_empty());

    let preferred: BTreeSet<String> = intent.preferred_regions.iter().cloned().collect();
    let preferred_with_pull = gradients
        .iter()
        .any(|g| preferred.contains(&g.target_region) && g.goal_pull > 0);
    assert!(preferred_with_pull, "expected positive goal pull into preferred regions");
}

#[test]
fn gate_al_goal_directed_trajectory_moves_toward_intent() {
    let (stable_topo, anchor_ids) = run_topo(false, 4);
    let (perturbed_topo, _) = run_topo(true, 4);

    let flow = compute_cognitive_flow_field(&[stable_topo, perturbed_topo], &anchor_ids)
        .expect("flow should compute");
    let potential = compute_cognitive_potential_field(&flow)
        .expect("potential should compute");
    let intent = compute_intent_field(&potential, &anchor_ids)
        .expect("intent field should compute");

    let start = potential
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("must have at least one region");

    let traj = select_goal_directed_trajectory(&potential, &intent, &start)
        .expect("goal trajectory should compute");

    assert!(!traj.selected_path.is_empty());
    assert!(traj.goal_stability.intent_confidence >= 0);

    if traj.selected_path.len() > 1 {
        let destination = traj.selected_path.last().expect("path has destination");
        let preferred: BTreeSet<String> = intent.preferred_regions.iter().cloned().collect();
        assert!(
            preferred.contains(destination) || !intent.avoidance_regions.contains(destination),
            "destination should be preferred or at least not avoided"
        );
    }
}

#[test]
fn gate_am_external_perturbation_reduces_goal_stability() {
    let (stable_topo, anchor_ids) = run_topo(false, 4);
    let (perturbed_topo, _) = run_topo(true, 4);

    let flow_stable = compute_cognitive_flow_field(
        &[stable_topo.clone(), stable_topo.clone()],
        &anchor_ids,
    )
    .expect("stable flow should compute");
    let flow_perturbed = compute_cognitive_flow_field(
        &[stable_topo, perturbed_topo],
        &anchor_ids,
    )
    .expect("perturbed flow should compute");

    let potential_stable = compute_cognitive_potential_field(&flow_stable)
        .expect("stable potential should compute");
    let potential_perturbed = compute_cognitive_potential_field(&flow_perturbed)
        .expect("perturbed potential should compute");

    let intent_stable = compute_intent_field(&potential_stable, &anchor_ids)
        .expect("stable intent should compute");
    let intent_perturbed = compute_intent_field(&potential_perturbed, &anchor_ids)
        .expect("perturbed intent should compute");

    let start_stable = potential_stable
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("stable start region");
    let start_perturbed = potential_perturbed
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("perturbed start region");

    let traj_stable = select_goal_directed_trajectory(&potential_stable, &intent_stable, &start_stable)
        .expect("stable trajectory should compute");
    let traj_perturbed = select_goal_directed_trajectory(
        &potential_perturbed,
        &intent_perturbed,
        &start_perturbed,
    )
    .expect("perturbed trajectory should compute");

    assert!(
        traj_perturbed.projected_energy >= traj_stable.projected_energy,
        "perturbation should not reduce projected energy"
    );
    assert!(
        traj_perturbed.goal_stability.stability_projection
            <= traj_stable.goal_stability.stability_projection,
        "perturbation should reduce stability projection"
    );
}

#[test]
fn gate_an_intent_trajectory_hash_worker_invariant() {
    let verifier = DeterminismVerifier::new();
    let (t1, a1) = run_topo(false, 1);
    let (t8, a8) = run_topo(false, 8);

    let flow1 = compute_cognitive_flow_field(&[t1.clone(), t1.clone()], &a1)
        .expect("flow1 should compute");
    let flow8 = compute_cognitive_flow_field(&[t8.clone(), t8.clone()], &a8)
        .expect("flow8 should compute");

    let p1 = compute_cognitive_potential_field(&flow1).expect("p1 should compute");
    let p8 = compute_cognitive_potential_field(&flow8).expect("p8 should compute");

    let i1 = compute_intent_field(&p1, &a1).expect("i1 should compute");
    let i8 = compute_intent_field(&p8, &a8).expect("i8 should compute");

    assert!(verifier.is_replay_stable(&i1, &i8).unwrap_or(false));

    let s1 = p1
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("start 1");
    let s8 = p8
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("start 8");

    let tr1 = select_goal_directed_trajectory(&p1, &i1, &s1).expect("tr1 should compute");
    let tr8 = select_goal_directed_trajectory(&p8, &i8, &s8).expect("tr8 should compute");

    assert_eq!(tr1.canonical_hash, tr8.canonical_hash);
}
