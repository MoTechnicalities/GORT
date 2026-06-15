use gort::{
    ArithmeticMode, AuditLogger, CognitiveFrame, ConstraintEvalEngine, DeterminismVerifier,
    GeometricState, SemanticConstraint, TaskScheduler,
};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Serialize)]
struct PipelineSnapshot {
    frame_id: String,
    closure_status: String,
    resolved_constraint_count: usize,
    selected_senses: Vec<(String, String, bool, i64)>,
    canonical_audit: Vec<String>,
    canonical_trace_hash: String,
}

fn run_full_pipeline(worker_count: usize) -> PipelineSnapshot {
    let constraints = vec![
        SemanticConstraint::assertion("light", "wave", true, 92),
        SemanticConstraint::assertion("light", "wave", false, 30),
        SemanticConstraint::assertion("light", "particle", true, 88),
        SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
        SemanticConstraint::assertion("vacuum", "has_medium", true, 20),
    ];

    let scheduler = TaskScheduler::new();
    let engine = ConstraintEvalEngine::new();
    let logger = AuditLogger::new();

    logger.record("pipeline:start");

    let mut evaluator_audit = Vec::new();
    let (resolved, summary) = engine
        .resolve_contradictions_parallel_deterministic(
            &constraints,
            &scheduler,
            worker_count,
            &mut evaluator_audit,
        )
        .expect("parallel contradiction resolution should succeed");

    logger.record(format!(
        "resolver:groups={} conflicts_resolved={}",
        summary.groups_processed, summary.conflicts_resolved
    ));
    for line in evaluator_audit {
        logger.record(format!("resolver:{}", line));
    }

    let nodes = engine.constraints_to_nodes(&resolved);
    logger.record(format!("nodes:{}", nodes.len()));

    let mut field = engine.project_nodes_to_field(&nodes);
    engine.apply_resonance_transform_with_mode(&mut field, &nodes, ArithmeticMode::Exact);
    logger.record(format!("field:concepts={}", field.concept_count()));

    let mut selected_senses = Vec::new();
    let mut frame = CognitiveFrame::new("phase3: contradiction to closure");
    for node in nodes {
        frame.add_node(node);
    }

    let subjects: BTreeSet<String> = resolved.iter().map(|c| c.subject.clone()).collect();
    for subject in subjects {
        if let Some(decision) = engine.disambiguate_subject_senses_with_margin(&field, &subject, 5000) {
            logger.record(format!(
                "sense:{} selected={} unresolved={} gap={}",
                subject, decision.selected_concept, decision.unresolved, decision.score_gap
            ));

            if decision.unresolved {
                frame.mark_unresolved_subject(subject.clone());
            }

            selected_senses.push((
                subject,
                decision.selected_concept,
                decision.unresolved,
                decision.score_gap,
            ));
        }
    }

    selected_senses.sort();

    let (closed, transition) = frame.attempt_closure();
    if let Some(t) = transition {
        logger.record(format!("closure:{}", t.reasoning_summary));
    }
    closed.validate().expect("frame validation should pass");

    for step in closed.audit_trail() {
        logger.record(format!("frame:{}", step));
    }

    PipelineSnapshot {
        frame_id: closed.frame_id(),
        closure_status: closed.closure_status().to_string(),
        resolved_constraint_count: resolved.len(),
        selected_senses,
        canonical_audit: logger.canonical_snapshot(),
        canonical_trace_hash: logger.canonical_trace_hash(),
    }
}

#[test]
fn phase3_pipeline_is_replay_stable_across_worker_counts() {
    let verifier = DeterminismVerifier::new();
    let baseline = run_full_pipeline(1);
    let baseline_hash = verifier
        .hash_state(&baseline)
        .expect("baseline hash should serialize");

    for workers in 2..=8 {
        let candidate = run_full_pipeline(workers);
        let candidate_hash = verifier
            .hash_state(&candidate)
            .expect("candidate hash should serialize");
        assert_eq!(candidate_hash, baseline_hash);
    }
}

#[test]
fn phase3_pipeline_is_replay_stable_across_repeated_runs() {
    let verifier = DeterminismVerifier::new();
    let baseline = run_full_pipeline(4);
    let baseline_hash = verifier
        .hash_state(&baseline)
        .expect("baseline hash should serialize");

    for _ in 0..10 {
        let candidate = run_full_pipeline(4);
        let candidate_hash = verifier
            .hash_state(&candidate)
            .expect("candidate hash should serialize");
        assert_eq!(candidate_hash, baseline_hash);
    }
}

#[test]
fn audit_canonicalization_is_byte_stable_for_replay_traces() {
    let unsorted = vec![
        "00000003: closure  attempted : unresolved".to_string(),
        "00000001: added   node a".to_string(),
        "00000002: added node b".to_string(),
    ];
    let normalized = vec![
        "00000001:added node a".to_string(),
        "00000002:added node b".to_string(),
        "00000003:closure attempted : unresolved".to_string(),
    ];

    let canonical_a = AuditLogger::canonicalize(&unsorted);
    let canonical_b = AuditLogger::canonicalize(&normalized);
    assert_eq!(canonical_a, canonical_b);
}
