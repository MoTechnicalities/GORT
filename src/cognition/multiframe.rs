use crate::cognition::constraint::SemanticConstraint;
use crate::cognition::evaluator::ConstraintEvalEngine;
use crate::cognition::node::CognitiveFrame;
use crate::cognition::scheduler::TaskScheduler;
use crate::geom::field::{ConceptCluster, SemanticField};
use crate::geom::invariants::InvariantViolation;
use crate::geom::mode::ArithmeticMode;
use crate::runtime::logging::AuditLogger;
use crate::GeometricState;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

type ConstraintKey = (String, String, Option<String>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MultiFrameConfig {
    pub iterations: usize,
    pub worker_count: usize,
    pub ambiguity_margin: i64,
    pub target_energy: i64,
    pub compression_threshold: i64,
    pub convergence_window: usize,
    pub energy_delta_threshold: i64,
}

impl Default for MultiFrameConfig {
    fn default() -> Self {
        Self {
            iterations: 6,
            worker_count: 4,
            ambiguity_margin: 5000,
            target_energy: 500,
            compression_threshold: 1,
            convergence_window: 2,
            energy_delta_threshold: 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizationMetrics {
    pub energy_delta: i64,
    pub contradiction_count: usize,
    pub unresolved_subjects: usize,
    pub disambiguation_gap_median: i64,
    pub fused_constraints: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSense {
    pub subject: String,
    pub selected_concept: String,
    pub support_frames: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsolidatedMemory {
    pub converged_iteration: Option<usize>,
    pub fused_constraints: Vec<SemanticConstraint>,
    pub stable_senses: Vec<StableSense>,
    pub clusters: Vec<ConceptCluster>,
    pub artifact_hash: String,
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
    pub metrics: StabilizationMetrics,
    pub converged: bool,
    pub iteration_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiFrameReport {
    pub iterations: Vec<MultiFrameIteration>,
    pub converged_iteration: Option<usize>,
    pub consolidated_memory: ConsolidatedMemory,
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
        let convergence_window = config.convergence_window.max(1);
        let mut report = Vec::with_capacity(iterations);
        let mut previous_shared_energy: Option<i64> = None;
        let mut previous_iteration_hash: Option<String> = None;
        let mut stable_streak: usize = 0;
        let mut converged_iteration: Option<usize> = None;
        let mut last_fused_constraints: Vec<SemanticConstraint> = Vec::new();
        let mut last_frame_results: Vec<FrameIterationResult> = Vec::new();
        let mut last_shared_field = SemanticField::new();

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

            let mut shared_field = SemanticField::new();
            for field in fields_by_frame.values() {
                shared_field.merge_from(field);
            }
            shared_field.normalize_energy(config.target_energy);
            shared_field.compress_by_intensity(config.compression_threshold);

            let fused_constraints = fuse_cross_frame_constraints(&resolved_by_frame);
            let propagated = propagate_resonance_constraints(&shared_field, &fused_constraints);
            let propagated_count = propagated.len();

            for constraints in self.frames.values_mut() {
                append_missing_constraints(constraints, &propagated);
            }

            let energy = shared_field.total_energy();
            let energy_delta = previous_shared_energy
                .map(|prev| (energy - prev).abs())
                .unwrap_or(0);
            previous_shared_energy = Some(energy);

            let contradiction_count = count_cross_frame_conflicts(&resolved_by_frame);
            let unresolved_subjects = frame_results
                .iter()
                .flat_map(|r| r.selected_senses.iter())
                .filter(|(_, _, unresolved, _)| *unresolved)
                .count();

            let mut gaps: Vec<i64> = frame_results
                .iter()
                .flat_map(|r| r.selected_senses.iter().map(|(_, _, _, gap)| *gap))
                .collect();
            gaps.sort_unstable();
            let disambiguation_gap_median = if gaps.is_empty() {
                0
            } else {
                gaps[gaps.len() / 2]
            };

            let metrics = StabilizationMetrics {
                energy_delta,
                contradiction_count,
                unresolved_subjects,
                disambiguation_gap_median,
                fused_constraints: fused_constraints.len(),
            };

            let iteration_hash = hash_json(&(
                shared_field_snapshot(&shared_field),
                &fused_constraints,
                &frame_results,
                &metrics,
            ))?;

            let stable_condition = previous_iteration_hash
                .as_ref()
                .map(|h| h == &iteration_hash)
                .unwrap_or(false)
                && metrics.energy_delta <= config.energy_delta_threshold.max(0)
                && metrics.contradiction_count == 0
                && metrics.unresolved_subjects == 0;

            if stable_condition {
                stable_streak += 1;
            } else {
                stable_streak = 0;
            }

            let converged = stable_streak >= convergence_window.saturating_sub(1) && previous_iteration_hash.is_some();
            previous_iteration_hash = Some(iteration_hash.clone());

            self.logger.record(format!(
                "mfc:iter:{}:shared_field={} propagated={} stable={} energy_delta={} unresolved={}",
                iter,
                shared_field.concept_count(),
                propagated_count,
                if stable_condition { 1 } else { 0 },
                metrics.energy_delta,
                metrics.unresolved_subjects
            ));

            report.push(MultiFrameIteration {
                iteration_index: iter,
                frame_results,
                shared_field_concepts: shared_field.concept_count(),
                propagated_constraints: propagated_count,
                metrics,
                converged,
                iteration_hash: iteration_hash.clone(),
            });

            last_fused_constraints = fused_constraints;
            last_shared_field = shared_field;
            last_frame_results = report
                .last()
                .map(|r| r.frame_results.clone())
                .unwrap_or_default();

            if converged {
                converged_iteration = Some(iter);
                self.logger.record(format!("mfc:converged_at={}", iter));
                break;
            }
        }

        let stable_senses = consolidate_stable_senses(&last_frame_results);
        let clusters = last_shared_field.clusters_by_subject();
        let artifact_hash = hash_json(&(converged_iteration, &last_fused_constraints, &stable_senses, &clusters))?;
        let consolidated_memory = ConsolidatedMemory {
            converged_iteration,
            fused_constraints: last_fused_constraints,
            stable_senses,
            clusters,
            artifact_hash,
        };

        Ok(MultiFrameReport {
            iterations: report,
            converged_iteration,
            consolidated_memory,
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

fn fuse_cross_frame_constraints(
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

fn propagate_resonance_constraints(
    shared_field: &SemanticField,
    fused_constraints: &[SemanticConstraint],
) -> Vec<SemanticConstraint> {
    let mut out = Vec::with_capacity(fused_constraints.len());
    for c in fused_constraints {
        let concept = format!("{}:{}", c.subject, c.predicate);
        let mut next = c.clone();

        if let Some(point) = shared_field.concept(&concept) {
            let bonus = (point.intensity.abs() / 25).min(20) as u8;
            if (point.intensity >= 0) == c.affirmed {
                next.weight = next.weight.saturating_add(bonus);
            } else {
                next.weight = next.weight.saturating_sub(bonus.min(next.weight.saturating_sub(1)));
            }
        }

        out.push(next);
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

fn count_cross_frame_conflicts(by_frame: &BTreeMap<String, Vec<SemanticConstraint>>) -> usize {
    let mut seen: BTreeMap<ConstraintKey, BTreeSet<bool>> = BTreeMap::new();
    for constraints in by_frame.values() {
        for c in constraints {
            seen.entry(c.key()).or_default().insert(c.affirmed);
        }
    }
    seen.values().filter(|polarities| polarities.len() > 1).count()
}

fn consolidate_stable_senses(frame_results: &[FrameIterationResult]) -> Vec<StableSense> {
    let mut votes: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    for frame in frame_results {
        for (subject, selected, unresolved, _gap) in &frame.selected_senses {
            if *unresolved {
                continue;
            }
            *votes
                .entry(subject.clone())
                .or_default()
                .entry(selected.clone())
                .or_default() += 1;
        }
    }

    let mut out = Vec::new();
    for (subject, by_concept) in votes {
        let mut ranked: Vec<(String, usize)> = by_concept.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        if let Some((selected_concept, support_frames)) = ranked.into_iter().next() {
            out.push(StableSense {
                subject,
                selected_concept,
                support_frames,
            });
        }
    }
    out.sort_by(|a, b| a.subject.cmp(&b.subject));
    out
}

fn shared_field_snapshot(field: &SemanticField) -> Vec<(String, i64, i64, i64, i64)> {
    field
        .ordered_concepts()
        .map(|(concept, point)| {
            (
                concept.clone(),
                point.intensity,
                point.position.x,
                point.position.y,
                point.position.z,
            )
        })
        .collect()
}

fn hash_json<T: Serialize>(value: &T) -> Result<String, InvariantViolation> {
    let bytes = serde_json::to_vec(value).map_err(|e| InvariantViolation::Determinism {
        message: format!("serialization failure during deterministic hash: {}", e),
    })?;
    let mut h = Sha256::new();
    h.update(bytes);
    Ok(format!("{:x}", h.finalize()))
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

    #[test]
    fn mfc_produces_consolidated_memory_artifact() {
        let mut mfc = build_mfc();
        let report = mfc.run(MultiFrameConfig::default()).expect("run should succeed");
        assert!(!report.consolidated_memory.artifact_hash.is_empty());
        assert!(!report.consolidated_memory.fused_constraints.is_empty());
    }
}
