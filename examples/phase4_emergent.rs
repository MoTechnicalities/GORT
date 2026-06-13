use rugc::{MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

fn main() {
    println!("=== RUGC Phase 4 Emergent Structure Demo ===\n");

    let mut mfc = MultiFrameCognition::new();

    // Frame A: physical interpretation
    mfc.register_frame(
        "physics",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 92),
            SemanticConstraint::assertion("light", "particle", true, 88),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
        ],
    );

    // Frame B: ontological interpretation with conflicting priors
    mfc.register_frame(
        "ontology",
        vec![
            SemanticConstraint::assertion("light", "wave", false, 30),
            SemanticConstraint::assertion("light", "particle", true, 65),
            SemanticConstraint::assertion("vacuum", "has_medium", true, 20),
        ],
    );

    // Frame C: observational evidence
    mfc.register_frame(
        "observation",
        vec![
            SemanticConstraint::assertion("light", "wave", true, 50),
            SemanticConstraint::assertion("light", "particle", true, 52),
            SemanticConstraint::assertion("vacuum", "has_medium", false, 40),
        ],
    );

    let config = MultiFrameConfig {
        iterations: 10,
        worker_count: 4,
        ambiguity_margin: 5000,
        target_energy: 500,
        compression_threshold: 1,
        convergence_window: 2,
        energy_delta_threshold: 2,
    };

    let report = mfc.run(config).expect("phase4 emergent run should succeed");

    for iter in &report.iterations {
        println!(
            "Iteration {} -> shared_concepts={} propagated={} converged={} energy_delta={} contradictions={} unresolved={}",
            iter.iteration_index,
            iter.shared_field_concepts,
            iter.propagated_constraints,
            iter.converged,
            iter.metrics.energy_delta,
            iter.metrics.contradiction_count,
            iter.metrics.unresolved_subjects
        );

        for frame in &iter.frame_results {
            println!(
                "  frame={} status={} concepts={} id={}",
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

    println!("\nConverged at iteration: {:?}", report.converged_iteration);
    println!(
        "Consolidated artifact hash: {}",
        report.consolidated_memory.artifact_hash
    );

    println!("Stable senses:");
    for sense in &report.consolidated_memory.stable_senses {
        println!(
            "  subject={} concept={} support_frames={}",
            sense.subject, sense.selected_concept, sense.support_frames
        );
    }

    println!("Clusters:");
    for cluster in &report.consolidated_memory.clusters {
        println!(
            "  anchor={} members={:?} total_intensity={}",
            cluster.anchor, cluster.members, cluster.total_intensity
        );
    }

    println!("\nTrace hash: {}", report.final_trace_hash);
}
