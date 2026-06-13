use rugc::{MultiFrameCognition, MultiFrameConfig, SemanticConstraint};
use std::collections::BTreeSet;

fn build(add_external_perturbation: bool) -> MultiFrameCognition {
    let mut mfc = MultiFrameCognition::new();

    let mut physics = vec![
        SemanticConstraint::assertion("light", "wave", true, 92),
        SemanticConstraint::assertion("light", "particle", true, 88),
        SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
    ];
    if add_external_perturbation {
        physics.push(SemanticConstraint::assertion("observer", "injects_noise", true, 70));
    }

    mfc.register_frame("physics", physics);
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

    mfc
}

fn config() -> MultiFrameConfig {
    MultiFrameConfig {
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
    }
}

fn main() {
    println!("=== RUGC Phase 4.5 Concept Anchor Demo ===\n");

    let mut baseline = build(false);
    let mut perturbed = build(true);

    let r0 = baseline.run(config()).expect("baseline should run");
    let r1 = perturbed.run(config()).expect("perturbed should run");

    println!("Baseline converged at: {:?}", r0.converged_iteration);
    println!("Perturbed converged at: {:?}", r1.converged_iteration);

    println!("\nBaseline anchors:");
    for anchor in &r0.anchor_registry.anchors {
        println!(
            "  id={} hits={} frames={} energy={} hash={}",
            anchor.id, anchor.persistence_hits, anchor.frame_count, anchor.energy, anchor.canonical_hash
        );
    }

    println!("\nPerturbed anchors:");
    for anchor in &r1.anchor_registry.anchors {
        println!(
            "  id={} hits={} frames={} energy={} hash={}",
            anchor.id, anchor.persistence_hits, anchor.frame_count, anchor.energy, anchor.canonical_hash
        );
    }

    println!("\nConsolidated artifact baseline: {}", r0.consolidated_memory.artifact_hash);
    println!("Consolidated artifact perturbed: {}", r1.consolidated_memory.artifact_hash);

    let baseline_ids: BTreeSet<String> = r0.anchor_registry.anchors.iter().map(|a| a.id.clone()).collect();
    let perturbed_ids: BTreeSet<String> = r1.anchor_registry.anchors.iter().map(|a| a.id.clone()).collect();
    let shared: Vec<String> = baseline_ids.intersection(&perturbed_ids).cloned().collect();

    println!("\nShared anchors (self basis): {:?}", shared);
    println!(
        "Self-vs-not-self signal: artifact changed = {}",
        r0.consolidated_memory.artifact_hash != r1.consolidated_memory.artifact_hash
    );
}
