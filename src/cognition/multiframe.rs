use crate::cognition::constraint::SemanticConstraint;
use crate::cognition::evaluator::ConstraintEvalEngine;
use crate::cognition::node::CognitiveFrame;
use crate::cognition::scheduler::TaskScheduler;
use crate::geom::invariants::InvariantViolation;
use crate::geom::mode::ArithmeticMode;
use crate::runtime::logging::AuditLogger;
use crate::GeometricState;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

type ConstraintKey = (String, String, Option<String>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MultiFrameConfig {
    pub iterations: usize,
    pub worker_count: usize,
    pub ambiguity_margin: i64,
    pub target_energy: i64,
    pub compression_threshold: i64,
}

impl Default for MultiFrameConfig {
    fn default() -> Self {
        Self {
            iterations: 2,
            worker_count: 4,
            ambiguity_margin: 5000,
            target_energy: 500,
            compression_threshold: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameIterationResult {
    pub topic: String,
    pub frame_id: String,
    pub closure_status: String,
    pub selected_senses: Vec<(String, String, bool, i64)>,
    pub field_concepts: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiFrameIteration {
    pub iteration_index: usize,
    pub frame_results: Vec<FrameIterationResult>,
    pub shared_field_concepts: usize,
    pub propagated_constraints: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiFrameReport {
    pub iterations: Vec<MultiFrameIteration>,
    pub final_trace_hash: String,
}

#[derive(Debug, Default)]
pub struct MultiFrameCognition {
    engine: ConstraintEvalEngine,
    scheduler: TaskScheduler,
    logger: AuditLogger,
    frames: BTreeMap<String, Vec<SemanticConstraint>>,
}

impl MultiFrameCognition {
    pub fn new() -> Self {
        Self {
            engine: ConstraintEvalEngine::new(),
            scheduler: TaskScheduler::new(),
            logger: AuditLogger::new(),
            frames: BTreeMap::new(),
        }
    }

    pub fn register_frame(&mut self, topic: impl Into<String>, constraints: Vec<SemanticConstraint>) {
        self.frames.insert(topic.into(), constraints);
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn run(&mut self, config: MultiFrameConfig) -> Result<MultiFrameReport, InvariantViolation> {
        let iterations = config.iterations.max(1);
        let workers = config.worker_count.max(1);
        let mut report = Vec::with_capacity(iterations);

        self.logger.record(format!(
            "mfc:start frames={} iterations={}",
            self.frames.len(),
            iterations
        ));

        for iter in 0..iterations {
            self.logger.record(format!("mfc:iter:{}:start", iter));
            let mut frame_results = Vec::new();
            let mut resolved_by_frame: BTreeMap<String, Vec<SemanticConstraint>> = BTreeMap::new();
            let mut fields_by_frame = BTreeMap::new();

            for (topic, constraints) in &self.frames {
                let mut eval_audit = Vec::new();
                let (resolved_constraints, summary) = self.engine.resolve_contradictions_parallel_deterministic(
                    constraints,
                    &self.scheduler,
                    workers,
                    &mut eval_audit,
                )?;

                self.logger.record(format!(
                    "mfc:iter:{}:frame:{}:groups={} conflicts={}",
                    iter, topic, summary.groups_processed, summary.conflicts_resolved
                ));
                for line in eval_audit {
                    self.logger.record(format!("mfc:iter:{}:frame:{}:audit:{}", iter, topic, line));
                }

                let nodes = self.engine.constraints_to_nodes(&resolved_constraints);
                let mut field = self.engine.project_nodes_to_field(&nodes);
                self.engine
                    .apply_resonance_transform_with_mode(&mut field, &nodes, ArithmeticMode::Exact);
                field.normalize_energy(config.target_energy);
                field.compress_by_intensity(config.compression_threshold);

                let subjects: BTreeSet<String> = resolved_constraints
                    .iter()
                    .map(|c| c.subject.clone())
                    .collect();

                let mut frame = CognitiveFrame::new(topic.clone());
                for node in nodes {
                    frame.add_node(node);
                }

                let mut selected_senses = Vec::new();
                for subject in subjects {
                    if let Some(decision) = self.engine.disambiguate_subject_senses_with_margin(
                        &field,
                        &subject,
                        config.ambiguity_margin,
                    ) {
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
                    self.logger.record(format!(
                        "mfc:iter:{}:frame:{}:closure:{}",
                        iter, topic, t.reasoning_summary
                    ));
                }
                for step in closed.audit_trail() {
                    self.logger
                        .record(format!("mfc:iter:{}:frame:{}:step:{}", iter, topic, step));
                }

                frame_results.push(FrameIterationResult {
                    topic: topic.clone(),
                    frame_id: closed.frame_id(),
                    closure_status: closed.closure_status().to_string(),
                    selected_senses,
                    field_concepts: field.concept_count(),
                });

                resolved_by_frame.insert(topic.clone(), resolved_constraints);
                fields_by_frame.insert(topic.clone(), field);
            }

            frame_results.sort_by(|a, b| a.topic.cmp(&b.topic));

            let mut shared_field = crate::SemanticField::new();
            for field in fields_by_frame.values() {
                shared_field.merge_from(field);
            }
            shared_field.normalize_energy(config.target_energy);
            shared_field.compress_by_intensity(config.compression_threshold);

            let propagated = resolve_cross_frame_constraints(&resolved_by_frame);
            let propagated_count = propagated.len();
            for constraints in self.frames.values_mut() {
                append_missing_constraints(constraints, &propagated);
            }

            self.logger.record(format!(
                "mfc:iter:{}:shared_field={} propagated={}",
                iter,
                shared_field.concept_count(),
                propagated_count
            ));

            report.push(MultiFrameIteration {
                iteration_index: iter,
                frame_results,
                shared_field_concepts: shared_field.concept_count(),
                propagated_constraints: propagated_count,
            });
        }

        Ok(MultiFrameReport {
            iterations: report,
            final_trace_hash: self.logger.canonical_trace_hash(),
        })
    }
}

fn append_missing_constraints(base: &mut Vec<SemanticConstraint>, additions: &[SemanticConstraint]) {
    let mut seen: BTreeSet<(String, String, Option<String>, bool)> = base
        .iter()
        .map(|c| (c.subject.clone(), c.predicate.clone(), c.object.clone(), c.affirmed))
        .collect();

    for c in additions {
        let key = (c.subject.clone(), c.predicate.clone(), c.object.clone(), c.affirmed);
        if seen.insert(key) {
            base.push(c.clone());
        }
    }

    base.sort_by(|a, b| {
        a.subject
            .cmp(&b.subject)
            .then(a.predicate.cmp(&b.predicate))
            .then(a.object.cmp(&b.object))
            .then(a.affirmed.cmp(&b.affirmed))
            .then(a.weight.cmp(&b.weight))
    });
}

fn resolve_cross_frame_constraints(
    by_frame: &BTreeMap<String, Vec<SemanticConstraint>>,
) -> Vec<SemanticConstraint> {
    let mut grouped: BTreeMap<ConstraintKey, Vec<SemanticConstraint>> = BTreeMap::new();
    for constraints in by_frame.values() {
        for c in constraints {
            grouped.entry(c.key()).or_default().push(c.clone());
        }
    }

    let mut out = Vec::new();
    for ((_subject, _predicate, _object), group) in grouped {
        let affirmed_weight: u16 = group
            .iter()
            .filter(|c| c.affirmed)
            .map(|c| c.weight as u16)
            .sum();
        let negated_weight: u16 = group
            .iter()
            .filter(|c| !c.affirmed)
            .map(|c| c.weight as u16)
            .sum();

        let choose_affirmed = match affirmed_weight.cmp(&negated_weight) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => true,
        };

        let mut selected: Vec<SemanticConstraint> = group
            .iter()
            .filter(|c| c.affirmed == choose_affirmed)
            .cloned()
            .collect();
        selected.sort_by(|a, b| {
            b.weight
                .cmp(&a.weight)
                .then(a.subject.cmp(&b.subject))
                .then(a.predicate.cmp(&b.predicate))
                .then(a.object.cmp(&b.object))
        });

        if let Some(mut best) = selected.into_iter().next() {
            let merged = if choose_affirmed {
                affirmed_weight
            } else {
                negated_weight
            };
            best.weight = merged.min(u8::MAX as u16) as u8;
            out.push(best);
        }
    }

    out.sort_by(|a, b| {
        a.subject
            .cmp(&b.subject)
            .then(a.predicate.cmp(&b.predicate))
            .then(a.object.cmp(&b.object))
            .then(a.affirmed.cmp(&b.affirmed))
            .then(a.weight.cmp(&b.weight))
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::determinism::DeterminismVerifier;

    fn build_mfc() -> MultiFrameCognition {
        let mut mfc = MultiFrameCognition::new();
        mfc.register_frame(
            "physics",
            vec![
                SemanticConstraint::assertion("light", "wave", true, 90),
                SemanticConstraint::assertion("light", "particle", true, 85),
                SemanticConstraint::assertion("vacuum", "has_medium", false, 80),
            ],
        );
        mfc.register_frame(
            "ontology",
            vec![
                SemanticConstraint::assertion("light", "wave", false, 20),
                SemanticConstraint::assertion("light", "particle", true, 60),
                SemanticConstraint::assertion("vacuum", "has_medium", true, 15),
            ],
        );
        mfc
    }

    #[test]
    fn mfc_is_replay_stable_across_worker_counts() {
        let verifier = DeterminismVerifier::new();

        let mut a = build_mfc();
        let mut b = build_mfc();
        let config_a = MultiFrameConfig {
            worker_count: 1,
            ..MultiFrameConfig::default()
        };
        let config_b = MultiFrameConfig {
            worker_count: 8,
            ..MultiFrameConfig::default()
        };

        let ra = a.run(config_a).expect("run should succeed");
        let rb = b.run(config_b).expect("run should succeed");

        assert!(verifier.is_replay_stable(&ra, &rb).unwrap_or(false));
    }

    #[test]
    fn mfc_propagates_cross_frame_constraints() {
        let mut mfc = build_mfc();
        let report = mfc.run(MultiFrameConfig::default()).expect("run should succeed");
        assert!(!report.iterations.is_empty());
        assert!(report.iterations[0].propagated_constraints > 0);
    }
}
