use gort::{MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

fn main() {
    println!("=== GORT Phase 4.7 Anchor-Driven Emergent Concept Formation Demo ===\n");

    let mut mfc = MultiFrameCognition::new();

    mfc.register_frame(
        "physics",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 92),
            SemanticConstraint::assertion("light", "particle", true, 88),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
            SemanticConstraint::assertion("photon", "is_quantized", true, 64),
        ],
    );

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

    let cfg = MultiFrameConfig {
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
    };

    let report = mfc.run(cfg).expect("phase47 run should succeed");

    for iter in &report.iterations {
        println!(
            "iter={} emergent_candidates={} emergent_active={} anchors={} coherence={}",
            iter.iteration_index,
            iter.metrics.emergent_candidates,
            iter.metrics.emergent_concepts_active,
            iter.metrics.active_anchors,
            iter.metrics.anchor_field_coherence
        );
    }

    println!("\nEmergent concepts:");
    for concept in &report.consolidated_memory.emergent_concepts {
        println!(
            "  id={} subject={} hits={} resonance={} anchors={:?} members={:?}",
            concept.id,
            concept.subject,
            concept.persistence_hits,
            concept.resonance_score,
            concept.basis_anchors,
            concept.members
        );
    }

    println!(
        "\nOntology expansion score: {}",
        report.consolidated_memory.ontology_expansion_score
    );
    println!("Anchor basis hash: {}", report.consolidated_memory.anchor_basis_hash);
    println!("Artifact hash: {}", report.consolidated_memory.artifact_hash);
}
