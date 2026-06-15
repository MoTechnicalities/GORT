use gort::{
    arbitrate_intent_field, compute_cognitive_flow_field, compute_cognitive_potential_field,
    compute_cognitive_topology, compute_intent_field, resolve_trajectory, MultiFrameCognition,
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

fn run_pipeline(external: bool) -> (gort::CognitivePotentialField, gort::IntentField, Vec<String>) {
    let report = build(external).run(cfg(4)).expect("run should succeed");
    let mem = &report.consolidated_memory;
    let anchor_ids = mem.anchor_basis_ids.clone();

    let topo1 = compute_cognitive_topology(mem, 500).expect("topo1 should compute");
    let topo2 = compute_cognitive_topology(mem, 500).expect("topo2 should compute");
    let flow = compute_cognitive_flow_field(&[topo1, topo2], &anchor_ids)
        .expect("flow should compute");
    let potential = compute_cognitive_potential_field(&flow).expect("potential should compute");
    let intent = compute_intent_field(&potential, &anchor_ids).expect("intent should compute");

    (potential, intent, anchor_ids)
}

fn print_step(label: &str, external: bool) {
    let (_potential_s, intent_s, _anchor_ids) = run_pipeline(false);
    let (potential_p, intent_p, _) = if external {
        run_pipeline(true)
    } else {
        run_pipeline(false)
    };

    let base_weights: BTreeMap<String, i64> = potential_p
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.attraction_strength))
        .collect();

    let arbitrated =
        arbitrate_intent_field(&[intent_s, intent_p], &potential_p, &base_weights)
            .expect("arbitration should succeed");

    let current = potential_p
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .unwrap_or_default();

    let trajectory = resolve_trajectory(&arbitrated, &potential_p, &current)
        .expect("trajectory resolution should succeed");

    println!("{label}");
    println!(
        "  goals={}, dominant={}",
        arbitrated.goal_set.goals.len(),
        arbitrated.dominant_goal_region
    );
    println!(
        "  arbitration_confidence={}, conflict_gradients={}",
        arbitrated.arbitration_confidence,
        arbitrated.conflict_gradients.len()
    );
    for cg in &arbitrated.conflict_gradients {
        if cg.total_pull > 0 || cg.interference > 0 {
            println!(
                "    {} pull={} dominant={} interference={} coherence={}",
                cg.region_id, cg.total_pull, cg.dominant_pull, cg.interference, cg.coherence_score
            );
        }
    }
    println!(
        "  selected_path={:?}",
        trajectory.selected_path.join(" → ")
    );
    println!(
        "  deferred_goals={}, conflict_cost={}, efficiency={}, convergent={}",
        trajectory.deferred_goals.len(),
        trajectory.conflict_cost,
        trajectory.arbitration_efficiency,
        trajectory.convergent
    );
    println!("  arb hash={}...", &arbitrated.canonical_hash[..16]);
    println!("  traj hash={}...", &trajectory.canonical_hash[..16]);
    println!();
}

fn main() {
    println!("=== Phase 5.6: Multi-Goal Arbitration & Internal Conflict Resolution ===");
    println!();

    print_step("Iteration 1 (stable, dual-goal):", false);
    print_step("Iteration 2 (stable replay):", false);
    print_step("Iteration 3 (perturbed, conflicting goals):", true);
    print_step("Iteration 4 (recovery):", false);
}
