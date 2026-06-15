use gort::{compute_cognitive_topology, MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

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

fn main() {
    println!("=== GORT Phase 5.1 Emergent Cognitive Topology Demo ===\n");

    let mut baseline = build(false);
    let mut perturbed = build(true);

    let rb = baseline.run(cfg(4)).expect("baseline run should succeed");
    let rp = perturbed.run(cfg(4)).expect("perturbed run should succeed");

    let tb = compute_cognitive_topology(&rb.consolidated_memory, 500)
        .expect("baseline topology should compute");
    let tp = compute_cognitive_topology(&rp.consolidated_memory, 500)
        .expect("perturbed topology should compute");

    println!("--- Baseline Topology ---");
    println!(
        "concepts={} regions={} boundaries={} avg_neighborhood={} manifold_stability={}",
        tb.metrics.total_concepts,
        tb.metrics.region_count,
        tb.metrics.boundary_count,
        tb.metrics.avg_neighborhood_size,
        tb.metrics.manifold_stability
    );
    for region in &tb.regions {
        println!(
            "  region={} members={:?} boundaries={:?} cohesion={}",
            region.id, region.members, region.boundary_members, region.cohesion_score
        );
    }
    if !tb.boundary_concepts.is_empty() {
        println!("  boundary concepts: {:?}", tb.boundary_concepts);
    }
    println!("  canonical_hash: {}", tb.canonical_hash);

    println!("\n--- Perturbed Topology ---");
    println!(
        "concepts={} regions={} boundaries={} avg_neighborhood={} manifold_stability={}",
        tp.metrics.total_concepts,
        tp.metrics.region_count,
        tp.metrics.boundary_count,
        tp.metrics.avg_neighborhood_size,
        tp.metrics.manifold_stability
    );
    for region in &tp.regions {
        println!(
            "  region={} members={:?} boundaries={:?} cohesion={}",
            region.id, region.members, region.boundary_members, region.cohesion_score
        );
    }
    println!("  canonical_hash: {}", tp.canonical_hash);

    println!("\n--- Topological Change ---");
    println!(
        "topology hash changed: {}",
        tb.canonical_hash != tp.canonical_hash
    );
    println!(
        "region_count delta: baseline={} perturbed={}",
        tb.metrics.region_count, tp.metrics.region_count
    );
}
