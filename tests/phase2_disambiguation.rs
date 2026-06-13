use rugc::{
    ArithmeticMode, ConstraintEvalEngine, ScheduledTask, SemanticConstraint, TaskScheduler,
};
use std::collections::BTreeMap;

fn run_phase2_disambiguation(worker_count: usize) -> (String, bool) {
    let constraints = vec![
        SemanticConstraint::assertion("light", "wave", true, 92),
        SemanticConstraint::assertion("light", "particle", true, 88),
        SemanticConstraint::assertion("light", "illusion", false, 60),
        SemanticConstraint::assertion("vacuum", "has_medium", false, 74),
        SemanticConstraint::assertion("vacuum", "permits_wave", true, 80),
    ];

    let scheduler = TaskScheduler::new();
    let tasks: Vec<ScheduledTask<SemanticConstraint>> = constraints
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, c)| ScheduledTask {
            id: (idx as u64) + 1,
            payload: c,
        })
        .collect();

    let planned_ids =
        scheduler.run_work_stealing_deterministic(tasks.clone(), worker_count, |task| task.id);

    let by_id: BTreeMap<u64, SemanticConstraint> = tasks.into_iter().map(|t| (t.id, t.payload)).collect();
    let ordered_constraints: Vec<SemanticConstraint> = planned_ids
        .iter()
        .filter_map(|id| by_id.get(id).cloned())
        .collect();

    let engine = ConstraintEvalEngine::new();
    let nodes = engine.constraints_to_nodes(&ordered_constraints);
    let mut field = engine.project_nodes_to_field(&nodes);
    engine.apply_resonance_transform_with_mode(&mut field, &nodes, ArithmeticMode::Exact);

    let decision = engine
        .disambiguate_subject_senses_with_margin(&field, "light", 200)
        .expect("expected multi-sense subject");

    (decision.selected_concept, decision.unresolved)
}

#[test]
fn disambiguation_is_stable_across_repeated_runs() {
    let baseline = run_phase2_disambiguation(3);

    for _ in 0..10 {
        let current = run_phase2_disambiguation(3);
        assert_eq!(current, baseline);
    }
}

#[test]
fn disambiguation_is_stable_across_worker_counts() {
    let baseline = run_phase2_disambiguation(1);

    for workers in 2..=6 {
        let current = run_phase2_disambiguation(workers);
        assert_eq!(current, baseline);
    }
}
