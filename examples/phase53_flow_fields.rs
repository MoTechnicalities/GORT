use gort::{
    compute_cognitive_flow_field, compute_cognitive_topology, MultiFrameCognition,
    MultiFrameConfig, SemanticConstraint,
};

fn cfg() -> MultiFrameConfig {
    MultiFrameConfig {
        iterations: 12,
        worker_count: 4,
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

fn snap(external: bool) -> (gort::CognitiveTopology, Vec<String>) {
    let report = build(external).run(cfg()).expect("run should succeed");
    let topo = compute_cognitive_topology(&report.consolidated_memory, 500)
        .expect("topology should compute");
    let anchors = report.consolidated_memory.anchor_basis_ids.clone();
    (topo, anchors)
}

fn main() {
    println!("=== GORT Phase 5.3 Cognitive Flow Fields Demo ===\n");

    // Simulate: stable → stable → perturbed → stable → stable
    let sequence = [false, false, true, false, false];
    let mut snapshots = Vec::new();
    let mut anchor_ids = Vec::new();

    for &ext in &sequence {
        let (t, a) = snap(ext);
        if anchor_ids.is_empty() {
            anchor_ids = a;
        }
        snapshots.push(t);
    }

    let flow = compute_cognitive_flow_field(&snapshots, &anchor_ids)
        .expect("flow field should compute");

    println!("--- Concept Flow Vectors ---");
    for cv in &flow.concept_vectors {
        println!(
            "  concept={:<30} flux={:+4} anchor_pull={} net_dir={:+}",
            cv.concept, cv.region_flux, cv.anchor_pull, cv.net_direction
        );
    }

    println!("\n--- Region Flow Vectors ---");
    for rv in &flow.region_vectors {
        println!(
            "  region={} cohesion_trend={:+} size_trend={:+} persistence={}% attractor={}",
            rv.region_id,
            rv.cohesion_trend,
            rv.size_trend,
            rv.persistence_score / 10,
            rv.is_attractor
        );
    }

    println!("\n--- Flow Prediction ---");
    println!("  convergent:     {}", flow.prediction.convergent);
    println!("  momentum:       {}", flow.prediction.momentum);
    println!("  stable_regions: {:?}", flow.prediction.predicted_stable_region_ids);
    println!("  transient:      {:?}", flow.prediction.predicted_transient_region_ids);
    println!("\ncanonicalhash: {}", &flow.canonical_hash[..32]);
}
