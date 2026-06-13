use rugc::{
    compare_topologies, compute_cognitive_topology, detect_phase_transition,
    track_manifold_evolution, MultiFrameCognition, MultiFrameConfig, SemanticConstraint,
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

fn snap(external: bool) -> rugc::CognitiveTopology {
    let report = build(external).run(cfg(4)).expect("run should succeed");
    compute_cognitive_topology(&report.consolidated_memory, 500)
        .expect("topology should compute")
}

fn main() {
    println!("=== RUGC Phase 5.2 Cognitive Manifold Dynamics Demo ===\n");

    // Simulate an evolution: stable -> stable -> perturbed -> stable
    let sequence = vec![false, false, true, false];
    let snapshots: Vec<_> = sequence.iter().map(|&ext| snap(ext)).collect();

    println!("--- Topology Snapshots ---");
    for (i, t) in snapshots.iter().enumerate() {
        println!(
            "step={} external={} regions={} boundaries={} stability={} hash={}",
            i,
            sequence[i],
            t.metrics.region_count,
            t.metrics.boundary_count,
            t.metrics.manifold_stability,
            &t.canonical_hash[..16]
        );
    }

    println!("\n--- Manifold Drift (step-to-step) ---");
    for i in 1..snapshots.len() {
        let drift = compare_topologies(&snapshots[i - 1], &snapshots[i]);
        let transition = detect_phase_transition(&drift, 200);
        println!(
            "step {}->{}: score={} region_delta={} boundary_delta={} hash_changed={} PHASE_TRANSITION={}",
            i - 1,
            i,
            drift.drift_score,
            drift.region_delta,
            drift.boundary_delta,
            drift.hash_changed,
            transition
        );
        if !drift.added_regions.is_empty() {
            println!("  added_regions:   {:?}", drift.added_regions);
        }
        if !drift.removed_regions.is_empty() {
            println!("  removed_regions: {:?}", drift.removed_regions);
        }
    }

    println!("\n--- Manifold Evolution Trace ---");
    let trace = track_manifold_evolution(&snapshots, 200).expect("trace should compute");
    println!("overall_stability:     {}", trace.overall_stability);
    println!("phase_transitions at:  {:?}", trace.phase_transition_steps);
    println!("persistent_regions:    {} regions", trace.persistent_region_ids.len());
    println!("transient_regions:     {} regions", trace.transient_region_ids.len());
    println!("canonical_hash:        {}", &trace.canonical_hash[..32]);
}
