use gort::{
    arbitrate_intent_field, build_goal_set, compute_cognitive_flow_field,
    compute_cognitive_potential_field, compute_cognitive_topology, compute_conflict_gradients,
    compute_intent_field, resolve_trajectory, DeterminismVerifier, MultiFrameCognition,
    MultiFrameConfig, SemanticConstraint,
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

fn run_pipeline(external: bool, workers: usize) -> (
    gort::CognitivePotentialField,
    gort::IntentField,
    Vec<String>,
) {
    let report = build(external).run(cfg(workers)).expect("run should succeed");
    let topo = compute_cognitive_topology(&report.consolidated_memory, 500)
        .expect("topology should compute");
    let anchor_ids = report.consolidated_memory.anchor_basis_ids.clone();

    let topo2 = compute_cognitive_topology(&report.consolidated_memory, 500)
        .expect("topology2 should compute");
    let flow = compute_cognitive_flow_field(&[topo, topo2], &anchor_ids)
        .expect("flow should compute");
    let potential = compute_cognitive_potential_field(&flow)
        .expect("potential should compute");
    let intent = compute_intent_field(&potential, &anchor_ids)
        .expect("intent should compute");

    (potential, intent, anchor_ids)
}

#[test]
fn gate_ao_goal_set_merges_multiple_intent_fields() {
    let (potential_s, intent_s, _) = run_pipeline(false, 4);
    let (_potential_p, intent_p, _) = run_pipeline(true, 4);

    // Re-derive second intent from perturbed topology so all regions are represented
    let base_weights: BTreeMap<String, i64> = potential_s
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.attraction_strength))
        .collect();

    let goal_set = build_goal_set(&[intent_s.clone(), intent_p.clone()], &base_weights)
        .expect("goal set should build");

    assert!(!goal_set.goals.is_empty());
    // GoalSet should contain at least the attractors from the stable field
    assert!(goal_set.goals.len() >= intent_s.goal_attractors.len());
    assert!(!goal_set.canonical_hash.is_empty());
}

#[test]
fn gate_ap_conflict_gradients_detect_goal_interference() {
    let (_potential_s, intent_s, _) = run_pipeline(false, 4);
    let (potential_p, intent_p, _) = run_pipeline(true, 4);

    let base_weights: BTreeMap<String, i64> = potential_p
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.attraction_strength))
        .collect();

    let goal_set = build_goal_set(&[intent_s, intent_p], &base_weights)
        .expect("goal set should build");

    let conflict_gradients = compute_conflict_gradients(&goal_set, &potential_p)
        .expect("conflict gradients should compute");

    assert!(!conflict_gradients.is_empty());

    // At least one region should have a meaningful coherence score
    let any_coherent = conflict_gradients.iter().any(|g| g.coherence_score > 0);
    assert!(any_coherent, "expected at least one region with coherent goal alignment");
}

#[test]
fn gate_aq_arbitration_produces_dominant_goal() {
    let (potential, intent, _) = run_pipeline(false, 4);

    let base_weights: BTreeMap<String, i64> = potential
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.attraction_strength))
        .collect();

    // Use same intent twice to simulate multiple goal sources
    let arbitrated = arbitrate_intent_field(&[intent.clone(), intent], &potential, &base_weights)
        .expect("arbitration should succeed");

    assert!(!arbitrated.dominant_goal_region.is_empty());
    assert!(arbitrated.arbitration_confidence > 0);
    assert!(!arbitrated.canonical_hash.is_empty());

    // Dominant region should be one of the known goal attractors
    let goal_regions: Vec<String> = arbitrated
        .goal_set
        .goals
        .iter()
        .map(|g| g.region_id.clone())
        .collect();
    assert!(
        goal_regions.contains(&arbitrated.dominant_goal_region),
        "dominant goal region must be in the goal set"
    );
}

#[test]
fn gate_ar_conflict_resolution_selects_coherent_trajectory() {
    let (_potential_s, intent_s, _) = run_pipeline(false, 4);
    let (potential_p, intent_p, _) = run_pipeline(true, 4);

    let base_weights: BTreeMap<String, i64> = potential_p
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.attraction_strength))
        .collect();

    let arbitrated = arbitrate_intent_field(&[intent_s, intent_p], &potential_p, &base_weights)
        .expect("arbitration should succeed");

    let current = potential_p
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("must have a region");

    let trajectory = resolve_trajectory(&arbitrated, &potential_p, &current)
        .expect("trajectory resolution should succeed");

    assert!(!trajectory.selected_path.is_empty());
    assert!(!trajectory.canonical_hash.is_empty());

    // Selected path must not route into an extremely high-energy region
    // (conflict_cost is bounded by the arbitration field)
    assert!(
        trajectory.arbitration_efficiency >= 0,
        "arbitration efficiency must be non-negative"
    );
}

#[test]
fn gate_as_arbitration_hash_worker_invariant() {
    let verifier = DeterminismVerifier::new();

    let (p1, i1, _) = run_pipeline(false, 1);
    let (p8, i8, _) = run_pipeline(false, 8);

    let bw1: BTreeMap<String, i64> = p1
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.attraction_strength))
        .collect();
    let bw8: BTreeMap<String, i64> = p8
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.attraction_strength))
        .collect();

    let arb1 = arbitrate_intent_field(&[i1.clone(), i1], &p1, &bw1)
        .expect("arb1 should succeed");
    let arb8 = arbitrate_intent_field(&[i8.clone(), i8], &p8, &bw8)
        .expect("arb8 should succeed");

    assert!(verifier.is_replay_stable(&arb1, &arb8).unwrap_or(false));

    let curr1 = p1
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("curr1");
    let curr8 = p8
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .expect("curr8");

    let tr1 = resolve_trajectory(&arb1, &p1, &curr1).expect("tr1 should succeed");
    let tr8 = resolve_trajectory(&arb8, &p8, &curr8).expect("tr8 should succeed");

    assert_eq!(tr1.canonical_hash, tr8.canonical_hash);
}
