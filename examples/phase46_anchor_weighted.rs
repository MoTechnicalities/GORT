use rugc::{MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

fn main() {
    println!("=== RUGC Phase 4.6 Anchor-Weighted Interpretation Demo ===\n");

    let mut mfc = MultiFrameCognition::new();

    mfc.register_frame(
        "physics",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 92),
            SemanticConstraint::assertion("light", "particle", true, 88),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
        ],
    );

    mfc.register_frame(
        "ontology",
        vec![
            SemanticConstraint::assertion("light", "wave", false, 30),
            SemanticConstraint::assertion("light", "particle", true, 65),
            SemanticConstraint::assertion("vacuum", "has_medium", true, 20),
        ],
    );

    mfc.register_frame(
        "observation",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 50),
            SemanticConstraint::assertion("light", "particle", true, 52),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 40),
        ],
    );

    let cfg = MultiFrameConfig {
        iterations: 10,
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
    };

    let report = mfc.run(cfg).expect("phase46 run should succeed");

    for iter in &report.iterations {
        println!(
            "iter={} converged={} overlap={} drift={} stability={} coherence={} highlighted={} active_anchors={}",
            iter.iteration_index,
            iter.converged,
            iter.metrics.anchor_overlap,
            iter.metrics.anchor_drift,
            iter.metrics.anchor_stability,
            iter.metrics.anchor_field_coherence,
            iter.metrics.anchor_contradictions_highlighted,
            iter.metrics.active_anchors
        );
    }

    println!("\nAnchor registry:");
    for anchor in &report.anchor_registry.anchors {
        println!(
            "  id={} hits={} energy={} hash={}",
            anchor.id, anchor.persistence_hits, anchor.energy, anchor.canonical_hash
        );
    }

    println!("\nAnchor basis hash: {}", report.consolidated_memory.anchor_basis_hash);
    println!(
        "Self continuity score: {}",
        report.consolidated_memory.self_continuity_score
    );
    println!(
        "External change score: {}",
        report.consolidated_memory.external_change_score
    );
    println!("Artifact hash: {}", report.consolidated_memory.artifact_hash);
}
