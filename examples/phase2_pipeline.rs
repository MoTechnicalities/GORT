use rugc::{
    ArithmeticMode, CognitiveFrame, ConstraintEvalEngine, GeometricState, SemanticConstraint,
    TaskScheduler, ScheduledTask,
};

fn main() {
    println!("=== RUGC Phase 2 Pipeline ===\n");

    // 1) Constraints
    let constraints = vec![
        SemanticConstraint::assertion("light", "wave", true, 92),
        SemanticConstraint::assertion("light", "wave", false, 30),
        SemanticConstraint::assertion("light", "particle", true, 88),
        SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
        SemanticConstraint::assertion("vacuum", "has_medium", true, 20),
    ];
    println!("1. Constraints loaded: {}", constraints.len());

    // Deterministic work-stealing for constraint pre-processing.
    let scheduler = TaskScheduler::new();
    let scheduled: Vec<ScheduledTask<SemanticConstraint>> = constraints
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, c)| ScheduledTask {
            id: (idx as u64) + 1,
            payload: c,
        })
        .collect();

    let scheduled_ids = scheduler.run_work_stealing_deterministic(scheduled, 3, |task| task.id);
    println!("   Work-stealing order: {:?}", scheduled_ids);

    let engine = ConstraintEvalEngine::new();
    let mut contradiction_audit = Vec::new();
    let (resolved_constraints, summary) = engine
        .resolve_contradictions_parallel_deterministic(&constraints, &scheduler, 3, &mut contradiction_audit)
        .expect("phase2 contradiction resolution should succeed");
    println!(
        "   Parallel contradiction resolution: groups={}, conflicts_resolved={}",
        summary.groups_processed, summary.conflicts_resolved
    );
    for line in contradiction_audit {
        println!("   audit: {}", line);
    }

    // 2) Constraints -> Nodes
    let nodes = engine.constraints_to_nodes(&resolved_constraints);
    println!("2. Nodes generated: {}", nodes.len());

    // 3) Nodes -> Field
    let mut exact_field = engine.project_nodes_to_field(&nodes);
    let mut bounded_field = exact_field.clone();
    println!("3. Field projected: {} concepts", exact_field.concept_count());

    // 4) Resonance transform with arithmetic mode switch
    engine.apply_resonance_transform_with_mode(&mut exact_field, &nodes, ArithmeticMode::Exact);
    engine.apply_resonance_transform_with_mode(
        &mut bounded_field,
        &nodes,
        ArithmeticMode::BoundedApproximate { max_error: 2 },
    );

    println!("4. Resonance applied in two modes:");
    for (concept, point) in exact_field.ordered_concepts() {
        let bounded = bounded_field
            .concept(concept)
            .map(|p| p.intensity)
            .unwrap_or_default();
        println!(
            "   {} -> exact: {}, bounded: {}, |error|: {}",
            concept,
            point.intensity,
            bounded,
            (point.intensity - bounded).abs()
        );
    }

    // 4b) Multi-sense disambiguation by field interference
    let ambiguity_margin = 5000;
    let disambiguation = engine.disambiguate_subject_senses_with_margin(
        &exact_field,
        "light",
        ambiguity_margin,
    );

    if let Some(decision) = &disambiguation {
        println!("4b. Disambiguation for '{}':", decision.subject);
        println!("   selected: {}", decision.selected_concept);
        println!(
            "   gap: {}, unresolved: {} (margin <= {})",
            decision.score_gap, decision.unresolved, ambiguity_margin
        );
        for candidate in &decision.candidates {
            println!(
                "   {} -> support: {}, interference: {}, score: {}",
                candidate.concept, candidate.support, candidate.interference, candidate.score
            );
        }
    }

    // 5) Nodes -> Closure
    let mut frame = CognitiveFrame::new("phase2: light duality");
    for node in nodes {
        frame.add_node(node);
    }

    if let Some(decision) = disambiguation {
        if decision.unresolved {
            frame.mark_unresolved_subject(decision.subject);
        }
    }

    let (closed, transition) = frame.attempt_closure();

    println!("5. Closure status: {} -> {}", frame.closure_status(), closed.closure_status());
    if let Some(t) = transition {
        println!("   Transition reason: {}", t.reasoning_summary);
    }

    match closed.validate() {
        Ok(()) => println!("\nPipeline complete: invariants validated."),
        Err(e) => println!("\nPipeline complete with invariant issue: {}", e),
    }
}
