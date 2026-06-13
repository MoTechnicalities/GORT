use rugc::{DeterminismVerifier, MultiFrameCognition, MultiFrameConfig, SemanticConstraint};

fn base_config(worker_count: usize) -> MultiFrameConfig {
    MultiFrameConfig {
        iterations: 10,
        worker_count,
        ambiguity_margin: 5000,
        target_energy: 500,
        compression_threshold: 1,
        convergence_window: 2,
        energy_delta_threshold: 2,
        anchor_energy_max: 2000,
        anchor_pull_strength: 4,
        anchor_min_persistence: 1,
    }
}

fn build_mfc(add_external_perturbation: bool) -> MultiFrameCognition {
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

#[test]
fn concept_anchors_register_after_persistent_stability() {
    let mut mfc = build_mfc(false);
    let report = mfc.run(base_config(4)).expect("run should succeed");

    assert!(!report.anchor_registry.anchors.is_empty());
    assert!(
        report
            .anchor_registry
            .anchors
            .iter()
            .all(|a| a.persistence_hits >= 2)
    );
}

#[test]
fn concept_anchor_registry_is_worker_invariant() {
    let verifier = DeterminismVerifier::new();
    let mut a = build_mfc(false);
    let mut b = build_mfc(false);

    let ra = a.run(base_config(1)).expect("run A should succeed");
    let rb = b.run(base_config(8)).expect("run B should succeed");

    let ha = verifier
        .hash_state(&ra.anchor_registry)
        .expect("anchor registry A should hash");
    let hb = verifier
        .hash_state(&rb.anchor_registry)
        .expect("anchor registry B should hash");
    assert_eq!(ha, hb);
}

#[test]
fn external_perturbation_changes_memory_but_preserves_anchor_basis() {
    let mut baseline = build_mfc(false);
    let mut perturbed = build_mfc(true);

    let r0 = baseline
        .run(base_config(4))
        .expect("baseline run should succeed");
    let r1 = perturbed
        .run(base_config(4))
        .expect("perturbed run should succeed");

    assert_ne!(
        r0.consolidated_memory.artifact_hash,
        r1.consolidated_memory.artifact_hash,
        "external perturbation should alter consolidated memory"
    );

    let a0: std::collections::BTreeSet<String> =
        r0.anchor_registry.anchors.iter().map(|a| a.id.clone()).collect();
    let a1: std::collections::BTreeSet<String> =
        r1.anchor_registry.anchors.iter().map(|a| a.id.clone()).collect();

    let intersection: Vec<String> = a0.intersection(&a1).cloned().collect();
    assert!(
        !intersection.is_empty(),
        "at least one anchor should persist across perturbation"
    );
}
