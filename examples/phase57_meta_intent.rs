use gort::{
    arbitrate_intent_field, build_meta_intent_field, compute_cognitive_flow_field,
    compute_cognitive_potential_field, compute_cognitive_topology, compute_intent_field,
    resolve_meta_intent_trajectory, MultiFrameCognition, MultiFrameConfig, SemanticConstraint,
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

fn run(external: bool) -> (gort::CognitivePotentialField, gort::ArbitratedIntentField) {
    let report = build(external).run(cfg(4)).expect("run should succeed");
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

    (potential, arbitrated)
}

fn show_step(label: &str, external: bool, recent_hashes: &[String]) -> String {
    let (potential, arbitrated) = run(external);
    let meta = build_meta_intent_field(&arbitrated, recent_hashes)
        .expect("meta should compute");

    let current = potential
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .unwrap_or_default();

    let trajectory = resolve_meta_intent_trajectory(&arbitrated, &potential, &current, recent_hashes)
        .expect("meta trajectory should compute");

    println!("{label}");
    println!(
        "  hierarchy layers={} revision_candidates={}",
        meta.hierarchy.layers.len(),
        meta.revision_candidates.len()
    );
    println!(
        "  coherence: hierarchy={} conflict_load={} revision_pressure={} temporal={} self_consistency={}",
        meta.self_coherence.hierarchy_coherence,
        meta.self_coherence.conflict_load,
        meta.self_coherence.revision_pressure,
        meta.self_coherence.temporal_stability,
        meta.self_coherence.self_consistency
    );
    println!("  selected path={:?}", trajectory.selected_path.join(" → "));
    println!("  revised goals={}", trajectory.revised_goals.len());
    println!("  meta hash={}...", &meta.canonical_hash[..16]);
    println!("  trajectory hash={}...", &trajectory.canonical_hash[..16]);
    println!();

    meta.canonical_hash
}

fn main() {
    println!("=== Phase 5.7: Self-Consistent Cognitive Dynamics (Meta-Intent) ===");
    println!();

    let mut history: Vec<String> = Vec::new();

    let h1 = show_step("Iteration 1 (stable):", false, &history);
    history.push(h1);

    let h2 = show_step("Iteration 2 (stable replay):", false, &history);
    history.push(h2);

    let h3 = show_step("Iteration 3 (perturbed):", true, &history);
    history.push(h3);

    let _h4 = show_step("Iteration 4 (recovery):", false, &history);
}
