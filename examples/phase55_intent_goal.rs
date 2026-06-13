use rugc::{
    compute_cognitive_flow_field, compute_cognitive_potential_field, compute_cognitive_topology,
    compute_intent_field, select_goal_directed_trajectory, MultiFrameCognition,
    MultiFrameConfig, SemanticConstraint,
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
    (topo, report.consolidated_memory.anchor_basis_ids.clone())
}

fn print_step(label: &str, snapshots: &[rugc::CognitiveTopology], anchor_ids: &[String]) {
    let flow = compute_cognitive_flow_field(snapshots, anchor_ids)
        .expect("flow should compute");
    let potential = compute_cognitive_potential_field(&flow)
        .expect("potential should compute");
    let intent = compute_intent_field(&potential, anchor_ids)
        .expect("intent field should compute");

    let current = potential
        .stability_energies
        .iter()
        .max_by_key(|e| e.potential)
        .map(|e| e.region_id.clone())
        .unwrap_or_default();

    let trajectory = select_goal_directed_trajectory(&potential, &intent, &current)
        .expect("intent trajectory should compute");

    println!("{label}");
    println!(
        "  goals={} preferred={} avoided={}",
        intent.goal_attractors.len(),
        intent.preferred_regions.len(),
        intent.avoidance_regions.len()
    );
    println!("  current region={}", current);
    println!(
        "  selected path={:?} projected_energy={}",
        trajectory.selected_path, trajectory.projected_energy
    );
    println!(
        "  stability: alignment={} efficiency={} projection={} confidence={}",
        trajectory.goal_stability.goal_alignment,
        trajectory.goal_stability.trajectory_efficiency,
        trajectory.goal_stability.stability_projection,
        trajectory.goal_stability.intent_confidence
    );
    println!("  intent hash={}...", &intent.canonical_hash[..16]);
    println!("  trajectory hash={}...", &trajectory.canonical_hash[..16]);
    println!();
}

fn main() {
    let (stable_topo, anchor_ids) = run_topo(false);
    let (perturbed_topo, _) = run_topo(true);
    let (recovery_topo, _) = run_topo(false);

    println!("=== Phase 5.5: Cognitive Intent & Goal Formation ===");
    println!();

    print_step(
        "Iteration 1 (stable):",
        &[stable_topo.clone(), stable_topo.clone()],
        &anchor_ids,
    );
    print_step(
        "Iteration 2 (stable replay):",
        &[stable_topo.clone(), stable_topo.clone()],
        &anchor_ids,
    );
    print_step(
        "Iteration 3 (perturbed):",
        &[stable_topo.clone(), perturbed_topo.clone()],
        &anchor_ids,
    );
    print_step(
        "Iteration 4 (recovery):",
        &[stable_topo, perturbed_topo, recovery_topo],
        &anchor_ids,
    );
}
