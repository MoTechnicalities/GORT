use gort::{MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

fn main() {
    println!("=== GORT Phase 3 Multi-Frame Demo ===\n");

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

    let config = MultiFrameConfig {
        iterations: 3,
        worker_count: 4,
        ambiguity_margin: 5000,
        target_energy: 500,
        compression_threshold: 1,
        convergence_window: 2,
        energy_delta_threshold: 2,
        anchor_energy_max: 500,
        anchor_pull_strength: 4,
        anchor_min_persistence: 2,
        anchor_alignment_window: 25,
        anchor_contradiction_highlight: 6,
        anchor_fusion_bias: 8,
        emergent_min_cluster_size: 2,
        emergent_min_anchor_support: 1,
        emergent_resonance_threshold: 40,
        emergent_min_persistence: 2,
        emergent_constraint_weight: 36,
    };

    let report = mfc.run(config).expect("multi-frame run should succeed");

    for iter in &report.iterations {
        println!(
            "Iteration {}: shared_field_concepts={}, propagated_constraints={}",
            iter.iteration_index, iter.shared_field_concepts, iter.propagated_constraints
        );

        for frame in &iter.frame_results {
            println!(
                "  Frame {} -> status={} concepts={} frame_id={}",
                frame.topic, frame.closure_status, frame.field_concepts, frame.frame_id
            );
            for (subject, selected, unresolved, gap) in &frame.selected_senses {
                println!(
                    "    subject={} selected={} unresolved={} gap={}",
                    subject, selected, unresolved, gap
                );
            }
        }
    }

    println!("\nFinal canonical trace hash: {}", report.final_trace_hash);
}
