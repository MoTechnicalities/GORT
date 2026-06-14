use crate::cognition::constraint::SemanticConstraint;
use crate::cognition::evaluator::ConstraintEvalEngine;
use crate::cognition::node::CognitiveFrame;
use crate::cognition::phase61_structural_recovery::{
    Phase61SignalSnapshot, Phase61StructuralRecoveryConfig, Phase61StructuralRecoveryState,
};
use crate::cognition::phase62_structural_experiment::{
    apply_phase62_structural_experiment, Phase62StructuralConfig,
};
use crate::cognition::scheduler::TaskScheduler;
use crate::geom::field::{ConceptCluster, SemanticField};
use crate::geom::invariants::InvariantViolation;
use crate::geom::mode::ArithmeticMode;
use crate::runtime::logging::AuditLogger;
use crate::GeometricState;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
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
    pub anchor_energy_max: i64,
    pub anchor_pull_strength: i64,
    pub anchor_min_persistence: usize,
    pub anchor_alignment_window: i64,
    pub anchor_contradiction_highlight: i64,
    pub anchor_fusion_bias: i64,
    pub emergent_min_cluster_size: usize,
    pub emergent_min_anchor_support: usize,
    pub emergent_resonance_threshold: i64,
    pub emergent_min_persistence: usize,
    pub emergent_constraint_weight: u8,
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmergentConcept {
    pub id: String,
    pub subject: String,
    pub basis_anchors: Vec<String>,
    pub members: Vec<String>,
    pub resonance_score: i64,
    pub persistence_hits: usize,
    pub canonical_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EmergentConceptCandidate {
    id: String,
    subject: String,
    basis_anchors: Vec<String>,
    members: Vec<String>,
    resonance_score: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConceptAnchor {
    pub id: String,
    pub canonical_hash: String,
    pub energy: i64,
    pub persistence_hits: usize,
    pub frame_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AnchorRegistry {
    pub anchors: Vec<ConceptAnchor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizationMetrics {
    pub energy_delta: i64,
    pub contradiction_count: usize,
    pub unresolved_subjects: usize,
    pub disambiguation_gap_median: i64,
    pub fused_constraints: usize,
    pub active_anchors: usize,
    pub anchor_overlap: usize,
    pub anchor_drift: i64,
    pub anchor_stability: i64,
    pub anchor_field_coherence: i64,
    pub anchor_contradictions_highlighted: usize,
    pub emergent_candidates: usize,
    pub emergent_concepts_active: usize,
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
    pub anchor_basis_ids: Vec<String>,
    pub anchor_basis_hash: String,
    pub self_continuity_score: i64,
    pub external_change_score: i64,
    pub emergent_concepts: Vec<EmergentConcept>,
    pub ontology_expansion_score: i64,
    pub artifact_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRelationalDistance {
    pub score: i64,
    pub anchor_jaccard_distance: i64,
    pub emergent_jaccard_distance: i64,
    pub continuity_delta: i64,
    pub external_delta: i64,
    pub ontology_delta: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologicalNeighborhood {
    pub center: String,
    pub neighbors: Vec<String>,
    pub radius: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologicalRegion {
    pub id: String,
    pub members: Vec<String>,
    pub representative: String,
    pub boundary_members: Vec<String>,
    pub cohesion_score: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyMetrics {
    pub region_count: usize,
    pub total_concepts: usize,
    pub boundary_count: usize,
    pub avg_neighborhood_size: i64,
    pub manifold_stability: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveTopology {
    pub neighborhoods: Vec<TopologicalNeighborhood>,
    pub regions: Vec<TopologicalRegion>,
    pub boundary_concepts: Vec<String>,
    pub metrics: TopologyMetrics,
    pub canonical_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifoldDrift {
    pub region_delta: i64,
    pub boundary_delta: i64,
    pub stability_delta: i64,
    pub cohesion_delta: i64,
    pub added_regions: Vec<String>,
    pub removed_regions: Vec<String>,
    pub hash_changed: bool,
    pub drift_score: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEvolutionStep {
    pub step_index: usize,
    pub drift: ManifoldDrift,
    pub is_phase_transition: bool,
    pub topology_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifoldEvolutionTrace {
    pub steps: Vec<TopologyEvolutionStep>,
    pub persistent_region_ids: Vec<String>,
    pub transient_region_ids: Vec<String>,
    pub phase_transition_steps: Vec<usize>,
    pub overall_stability: i64,
    pub canonical_hash: String,
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
    pub anchor_registry: AnchorRegistry,
    pub final_trace_hash: String,
}

#[derive(Debug, Default)]
pub struct MultiFrameCognition {
    engine: ConstraintEvalEngine,
    scheduler: TaskScheduler,
    logger: AuditLogger,
    frames: BTreeMap<String, Vec<SemanticConstraint>>,
    anchor_registry: AnchorRegistry,
}

impl MultiFrameCognition {
    pub fn new() -> Self {
        Self {
            engine: ConstraintEvalEngine::new(),
            scheduler: TaskScheduler::new(),
            logger: AuditLogger::new(),
            frames: BTreeMap::new(),
            anchor_registry: AnchorRegistry { anchors: Vec::new() },
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
        let mut previous_anchor_energies: BTreeMap<String, i64> = BTreeMap::new();
        let mut emergent_hits: BTreeMap<String, usize> = BTreeMap::new();
        let mut emergent_latest: BTreeMap<String, EmergentConceptCandidate> = BTreeMap::new();
        let mut stable_streak: usize = 0;
        let mut converged_iteration: Option<usize> = None;
        let mut last_fused_constraints: Vec<SemanticConstraint> = Vec::new();
        let mut last_frame_results: Vec<FrameIterationResult> = Vec::new();
        let mut last_shared_field = SemanticField::new();
        let mut last_metrics: Option<StabilizationMetrics> = None;
        let phase61_config = Phase61StructuralRecoveryConfig::default();
        let mut phase61_state = Phase61StructuralRecoveryState::default();

        self.logger.record(format!(
            "mfc:start frames={} iterations={}",
            self.frames.len(),
            iterations
        ));

        for iter in 0..iterations {
            self.logger.record(format!("mfc:iter:{}:start", iter));

            let signal = last_metrics
                .as_ref()
                .map(|m| Phase61SignalSnapshot {
                    has_previous_iteration: previous_iteration_hash.is_some(),
                    contradiction_count: m.contradiction_count,
                    unresolved_subjects: m.unresolved_subjects,
                    anchor_overlap: m.anchor_overlap,
                    anchor_field_coherence: m.anchor_field_coherence,
                    anchor_drift: m.anchor_drift,
                    energy_delta: m.energy_delta,
                    energy_delta_threshold: config.energy_delta_threshold,
                })
                .unwrap_or(Phase61SignalSnapshot {
                    has_previous_iteration: previous_iteration_hash.is_some(),
                    energy_delta_threshold: config.energy_delta_threshold,
                    ..Phase61SignalSnapshot::default()
                });

            let phase61_policy = phase61_state.build_runtime_policy(
                iter,
                config.anchor_min_persistence,
                config.anchor_pull_strength,
                config.ambiguity_margin,
                signal,
                &phase61_config,
            );

            let effective_anchor_min_persistence = phase61_policy.effective_anchor_min_persistence;
            let effective_anchor_pull_strength = phase61_policy.effective_anchor_pull_strength;
            let sweep_ambiguity_margin = phase61_policy.sweep_ambiguity_margin;

            let mut frame_results = Vec::new();
            let mut resolved_by_frame: BTreeMap<String, Vec<SemanticConstraint>> = BTreeMap::new();
            let mut fields_by_frame = BTreeMap::new();

            for (topic, constraints) in &self.frames {
                let phase62_config = phase62_config_for_frame(constraints);
                let (phase62_constraints, phase62_report) =
                    apply_phase62_structural_experiment(constraints, phase62_config);
                if phase62_report.applied {
                    self.logger.record(format!(
                        "mfc:iter:{}:frame:{}:phase62:generated={} note={}",
                        iter,
                        topic,
                        phase62_report.generated_constraints,
                        phase62_report.note
                    ));
                }

                let mut eval_audit = Vec::new();
                let (resolved_constraints, summary) = self.engine.resolve_contradictions_parallel_deterministic(
                    &phase62_constraints,
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

                if phase61_policy.active && !frame.unresolved_subjects.is_empty() {
                    for _ in 0..phase61_policy.extra_disambiguation_sweeps.max(1) {
                        let unresolved_before = frame.unresolved_subjects.clone();
                        for subject in unresolved_before {
                            if let Some(decision) = self.engine.disambiguate_subject_senses_with_margin(
                                &field,
                                &subject,
                                sweep_ambiguity_margin,
                            ) {
                                if !decision.unresolved {
                                    frame.unresolved_subjects.retain(|s| s != &subject);
                                    if let Some(entry) = selected_senses
                                        .iter_mut()
                                        .find(|(s, _, _, _)| s == &subject)
                                    {
                                        entry.1 = decision.selected_concept.clone();
                                        entry.2 = false;
                                        entry.3 = decision.score_gap;
                                    }
                                }
                            }
                        }
                        frame.unresolved_subjects.sort();
                        if frame.unresolved_subjects.is_empty() {
                            break;
                        }
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

            apply_anchor_persistence(
                &mut shared_field,
                &self.anchor_registry,
                effective_anchor_pull_strength,
                config.target_energy,
                config.compression_threshold,
                effective_anchor_min_persistence,
            );

            let contradictions_highlighted = apply_anchor_weighted_interpretation(
                &mut shared_field,
                &self.anchor_registry,
                effective_anchor_min_persistence,
                config.anchor_alignment_window,
                effective_anchor_pull_strength,
                config.anchor_contradiction_highlight,
                config.target_energy,
                config.compression_threshold,
            );

            let fused_constraints = fuse_cross_frame_constraints(&resolved_by_frame);
            let fused_constraints = anchor_guided_fusion(
                fused_constraints,
                &self.anchor_registry,
                effective_anchor_min_persistence,
                &shared_field,
                config.anchor_fusion_bias,
            );
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

            let active_anchor_map = active_anchor_energies(&self.anchor_registry, effective_anchor_min_persistence);
            let (anchor_overlap, anchor_drift, anchor_stability, anchor_field_coherence) =
                compute_anchor_continuity(
                    &shared_field,
                    &active_anchor_map,
                    &previous_anchor_energies,
                );

            let metrics = StabilizationMetrics {
                energy_delta,
                contradiction_count,
                unresolved_subjects,
                disambiguation_gap_median,
                fused_constraints: fused_constraints.len(),
                active_anchors: self
                    .anchor_registry
                    .anchors
                    .iter()
                    .filter(|a| a.persistence_hits >= effective_anchor_min_persistence.max(1))
                    .count(),
                anchor_overlap,
                anchor_drift,
                anchor_stability,
                anchor_field_coherence,
                anchor_contradictions_highlighted: contradictions_highlighted,
                emergent_candidates: 0,
                emergent_concepts_active: 0,
            };

            update_anchor_registry(
                &mut self.anchor_registry,
                &shared_field,
                frame_results.len(),
                config.anchor_energy_max,
                config.target_energy,
                config.compression_threshold,
            );

            let emergent_candidates = discover_emergent_candidates(
                &shared_field,
                &self.anchor_registry,
                effective_anchor_min_persistence,
                config.emergent_min_cluster_size,
                config.emergent_min_anchor_support,
                config.emergent_resonance_threshold,
            );

            for candidate in &emergent_candidates {
                *emergent_hits.entry(candidate.id.clone()).or_default() += 1;
                emergent_latest.insert(candidate.id.clone(), candidate.clone());
            }

            let emergent_constraints = synthesize_emergent_constraints(
                &emergent_candidates,
                &emergent_hits,
                config.emergent_min_persistence,
                config.emergent_constraint_weight,
            );
            for constraints in self.frames.values_mut() {
                append_missing_constraints(constraints, &emergent_constraints);
            }

            let emergent_active = emergent_hits
                .values()
                .filter(|hits| **hits >= config.emergent_min_persistence.max(1))
                .count();

            let metrics = StabilizationMetrics {
                emergent_candidates: emergent_candidates.len(),
                emergent_concepts_active: emergent_active,
                ..metrics
            };

            let iteration_hash = hash_json(&(
                shared_field_snapshot(&shared_field),
                &fused_constraints,
                &frame_results,
                &metrics,
                &emergent_constraints,
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
                "mfc:iter:{}:shared_field={} propagated={} stable={} energy_delta={} unresolved={} anchors={} overlap={} drift={} coherence={} emergent_candidates={} emergent_active={}",
                iter,
                shared_field.concept_count(),
                propagated_count,
                if stable_condition { 1 } else { 0 },
                metrics.energy_delta,
                metrics.unresolved_subjects,
                metrics.active_anchors,
                metrics.anchor_overlap,
                metrics.anchor_drift,
                metrics.anchor_field_coherence,
                metrics.emergent_candidates,
                metrics.emergent_concepts_active
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

            last_metrics = report.last().map(|r| r.metrics.clone());
            last_fused_constraints = fused_constraints;
            last_shared_field = shared_field;
            previous_anchor_energies = active_anchor_energies(&self.anchor_registry, effective_anchor_min_persistence);
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
        let anchor_registry = registered_anchor_registry(&self.anchor_registry, config.anchor_min_persistence);
        let anchor_basis_ids: Vec<String> = anchor_registry
            .anchors
            .iter()
            .map(|a| a.id.clone())
            .collect();
        let emergent_concepts = materialize_emergent_concepts(
            &emergent_hits,
            &emergent_latest,
            config.emergent_min_persistence,
        )?;
        let anchor_basis_hash = hash_json(&anchor_registry.anchors)?;
        let self_continuity_score = last_metrics
            .as_ref()
            .map(|m| m.anchor_stability + m.anchor_field_coherence - m.anchor_drift)
            .unwrap_or(0);
        let external_change_score = last_metrics
            .as_ref()
            .map(|m| m.anchor_drift + (m.anchor_contradictions_highlighted as i64 * 10))
            .unwrap_or(0);
        let ontology_expansion_score = emergent_concepts
            .iter()
            .map(|c| (c.members.len() as i64) * (c.persistence_hits as i64))
            .sum();
        let artifact_hash = hash_json(&(
            converged_iteration,
            &last_fused_constraints,
            &stable_senses,
            &clusters,
            &anchor_basis_ids,
            &anchor_basis_hash,
            self_continuity_score,
            external_change_score,
            &emergent_concepts,
            ontology_expansion_score,
        ))?;
        let consolidated_memory = ConsolidatedMemory {
            converged_iteration,
            fused_constraints: last_fused_constraints,
            stable_senses,
            clusters,
            anchor_basis_ids,
            anchor_basis_hash,
            self_continuity_score,
            external_change_score,
            emergent_concepts,
            ontology_expansion_score,
            artifact_hash,
        };

        Ok(MultiFrameReport {
            iterations: report,
            converged_iteration,
            consolidated_memory,
            anchor_registry,
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

fn anchor_guided_fusion(
    mut fused_constraints: Vec<SemanticConstraint>,
    registry: &AnchorRegistry,
    min_persistence: usize,
    shared_field: &SemanticField,
    fusion_bias: i64,
) -> Vec<SemanticConstraint> {
    let bias = fusion_bias.max(0) as u8;
    if bias == 0 {
        return fused_constraints;
    }

    let active_map = active_anchor_energies(registry, min_persistence);
    for c in &mut fused_constraints {
        let concept = format!("{}:{}", c.subject, c.predicate);
        let anchor_energy = active_map.get(&concept).copied();

        if let Some(anchor_e) = anchor_energy {
            let anchor_affirmed = anchor_e >= 0;
            if c.affirmed == anchor_affirmed {
                c.weight = c.weight.saturating_add(bias.min(20));
            } else {
                c.weight = c.weight.saturating_sub(bias.min(c.weight.saturating_sub(1)));
            }
        }

        if let Some(point) = shared_field.concept(&concept) {
            let field_affirmed = point.intensity >= 0;
            if c.affirmed == field_affirmed {
                c.weight = c.weight.saturating_add(2);
            } else {
                c.weight = c.weight.saturating_sub(1);
            }
        }
    }

    fused_constraints.sort_by(|a, b| {
        a.subject
            .cmp(&b.subject)
            .then(a.predicate.cmp(&b.predicate))
            .then(a.object.cmp(&b.object))
            .then(a.affirmed.cmp(&b.affirmed))
            .then(a.weight.cmp(&b.weight))
    });
    fused_constraints
}

fn active_anchor_energies(registry: &AnchorRegistry, min_persistence: usize) -> BTreeMap<String, i64> {
    registry
        .anchors
        .iter()
        .filter(|a| a.persistence_hits >= min_persistence.max(1))
        .map(|a| (a.id.clone(), a.energy))
        .collect()
}

fn compute_anchor_continuity(
    field: &SemanticField,
    active_anchor_map: &BTreeMap<String, i64>,
    previous_anchor_map: &BTreeMap<String, i64>,
) -> (usize, i64, i64, i64) {
    let active_ids: BTreeSet<String> = active_anchor_map.keys().cloned().collect();
    let previous_ids: BTreeSet<String> = previous_anchor_map.keys().cloned().collect();

    let overlap = active_ids.intersection(&previous_ids).count();

    let mut drift = 0;
    for id in active_ids.intersection(&previous_ids) {
        let current = active_anchor_map.get(id).copied().unwrap_or_default();
        let previous = previous_anchor_map.get(id).copied().unwrap_or_default();
        drift += (current - previous).abs();
    }

    let stability = if previous_ids.is_empty() {
        100
    } else {
        ((overlap as i64) * 100) / (previous_ids.len() as i64)
    };

    let mut coherence_sum = 0;
    let mut coherence_count = 0;
    for (id, anchor_energy) in active_anchor_map {
        if let Some(point) = field.concept(id) {
            let diff = (point.intensity - *anchor_energy).abs();
            coherence_sum += (100 - diff).max(0);
            coherence_count += 1;
        }
    }
    let coherence = if coherence_count == 0 {
        0
    } else {
        coherence_sum / coherence_count
    };

    (overlap, drift, stability, coherence)
}

fn apply_anchor_weighted_interpretation(
    field: &mut SemanticField,
    registry: &AnchorRegistry,
    min_persistence: usize,
    alignment_window: i64,
    amplify_strength: i64,
    contradiction_highlight: i64,
    target_energy: i64,
    compression_threshold: i64,
) -> usize {
    let mut contradictions = 0;
    let window = alignment_window.max(0);
    let amp = amplify_strength.max(0);
    let highlight = contradiction_highlight.max(0);

    for anchor in &registry.anchors {
        if anchor.persistence_hits < min_persistence.max(1) {
            continue;
        }

        if let Some(point) = field.concept(&anchor.id).cloned() {
            let mut adjusted = point.intensity;
            let diff = (point.intensity - anchor.energy).abs();
            let same_sign = (point.intensity >= 0) == (anchor.energy >= 0);

            if !same_sign && point.intensity != 0 && anchor.energy != 0 {
                contradictions += 1;
                let anchor_sign = if anchor.energy >= 0 { 1 } else { -1 };
                adjusted += anchor_sign * highlight;
            } else if diff <= window {
                let anchor_sign = if anchor.energy >= 0 { 1 } else { -1 };
                adjusted += anchor_sign * amp;
            } else {
                adjusted = (adjusted * 8) / 10;
            }

            field.upsert_concept(
                anchor.id.clone(),
                crate::FieldPoint {
                    position: point.position,
                    intensity: adjusted,
                },
            );
        }
    }

    field.normalize_energy(target_energy);
    field.compress_by_intensity(compression_threshold);
    contradictions
}

fn apply_anchor_persistence(
    field: &mut SemanticField,
    registry: &AnchorRegistry,
    pull_strength: i64,
    target_energy: i64,
    compression_threshold: i64,
    min_persistence: usize,
) {
    let strength = pull_strength.max(0);
    if strength == 0 {
        return;
    }

    for anchor in &registry.anchors {
        if anchor.persistence_hits < min_persistence.max(1) {
            continue;
        }
        if let Some(point) = field.concept(&anchor.id).cloned() {
            let baseline_sign = if anchor.energy >= 0 { 1 } else { -1 };
            let target = (anchor.energy.abs() / 10).max(1) * baseline_sign;
            let adjusted = (point.intensity * (10 - strength.min(9)) + target * strength.min(9)) / 10;

            field.upsert_concept(
                anchor.id.clone(),
                crate::FieldPoint {
                    position: point.position,
                    intensity: adjusted,
                },
            );
        }
    }

    field.normalize_energy(target_energy);
    field.compress_by_intensity(compression_threshold);
}

fn update_anchor_registry(
    registry: &mut AnchorRegistry,
    shared_field: &SemanticField,
    frame_count: usize,
    anchor_energy_max: i64,
    target_energy: i64,
    compression_threshold: i64,
) {
    let field_energy = shared_field.total_energy();
    if field_energy > anchor_energy_max.max(0) {
        return;
    }

    let baseline_hash = match shared_field.canonical_hash() {
        Ok(v) => v,
        Err(_) => return,
    };

    let is_stable = perturbation_returns_anchor(
        shared_field,
        &baseline_hash,
        target_energy,
        compression_threshold,
    );
    if !is_stable {
        return;
    }

    for (concept, point) in shared_field.ordered_concepts() {
        if point.intensity == 0 {
            continue;
        }
        if concept.contains("emergent/") {
            continue;
        }

        if let Some(existing) = registry.anchors.iter_mut().find(|a| a.id == *concept) {
            existing.persistence_hits += 1;
            existing.canonical_hash = baseline_hash.clone();
            existing.energy = point.intensity;
            existing.frame_count = frame_count;
        } else {
            registry.anchors.push(ConceptAnchor {
                id: concept.clone(),
                canonical_hash: baseline_hash.clone(),
                energy: point.intensity,
                persistence_hits: 1,
                frame_count,
            });
        }
    }

    registry
        .anchors
        .sort_by(|a, b| a.id.cmp(&b.id).then(a.canonical_hash.cmp(&b.canonical_hash)));
}

fn registered_anchor_registry(registry: &AnchorRegistry, min_persistence: usize) -> AnchorRegistry {
    let mut anchors: Vec<ConceptAnchor> = registry
        .anchors
        .iter()
        .filter(|a| a.persistence_hits >= min_persistence.max(1))
        .cloned()
        .collect();
    anchors.sort_by(|a, b| a.id.cmp(&b.id).then(a.canonical_hash.cmp(&b.canonical_hash)));
    AnchorRegistry { anchors }
}

fn discover_emergent_candidates(
    shared_field: &SemanticField,
    registry: &AnchorRegistry,
    anchor_min_persistence: usize,
    min_cluster_size: usize,
    min_anchor_support: usize,
    resonance_threshold: i64,
) -> Vec<EmergentConceptCandidate> {
    let mut anchors_by_subject: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for anchor in registry
        .anchors
        .iter()
        .filter(|a| a.persistence_hits >= anchor_min_persistence.max(1))
    {
        if let Some((subject, _)) = anchor.id.split_once(':') {
            anchors_by_subject
                .entry(subject.to_string())
                .or_default()
                .push(anchor.id.clone());
        }
    }

    for anchors in anchors_by_subject.values_mut() {
        anchors.sort();
    }

    let mut candidates = Vec::new();
    for cluster in shared_field.clusters_by_subject() {
        let members: Vec<String> = cluster
            .members
            .into_iter()
            .filter(|member| !member.contains("emergent/"))
            .collect();

        if members.len() < min_cluster_size.max(1) {
            continue;
        }

        let subject_anchors = anchors_by_subject
            .get(&cluster.anchor)
            .cloned()
            .unwrap_or_default();
        if subject_anchors.len() < min_anchor_support.max(1) {
            continue;
        }

        let member_set: BTreeSet<String> = members.iter().cloned().collect();
        let mut aligned: Vec<String> = subject_anchors
            .into_iter()
            .filter(|a| member_set.contains(a))
            .collect();
        aligned.sort();
        if aligned.len() < min_anchor_support.max(1) {
            continue;
        }

        let resonance_score = cluster.total_intensity.abs()
            + ((aligned.len() as i64) * 15)
            + ((members.len() as i64) * 5);
        if resonance_score < resonance_threshold.max(0) {
            continue;
        }

        let basis_head = aligned
            .first()
            .cloned()
            .unwrap_or_else(|| cluster.anchor.clone());
        let id = format!(
            "emergent:{}:{}:{}",
            cluster.anchor,
            basis_head.replace(':', "_"),
            members.len()
        );

        candidates.push(EmergentConceptCandidate {
            id,
            subject: cluster.anchor.clone(),
            basis_anchors: aligned,
            members,
            resonance_score,
        });
    }

    candidates.sort_by(|a, b| a.id.cmp(&b.id));
    candidates
}

fn synthesize_emergent_constraints(
    candidates: &[EmergentConceptCandidate],
    hits: &BTreeMap<String, usize>,
    min_persistence: usize,
    weight: u8,
) -> Vec<SemanticConstraint> {
    let mut out = Vec::new();
    for candidate in candidates {
        let persistence = hits.get(&candidate.id).copied().unwrap_or(0);
        if persistence < min_persistence.max(1) {
            continue;
        }

        let predicate = format!("emergent/{}", candidate.id.replace(':', "_"));
        out.push(SemanticConstraint::assertion(
            candidate.subject.clone(),
            predicate,
            true,
            weight.max(1),
        ));
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

fn materialize_emergent_concepts(
    hits: &BTreeMap<String, usize>,
    latest: &BTreeMap<String, EmergentConceptCandidate>,
    min_persistence: usize,
) -> Result<Vec<EmergentConcept>, InvariantViolation> {
    let mut out = Vec::new();
    for (id, persistence_hits) in hits {
        if *persistence_hits < min_persistence.max(1) {
            continue;
        }
        let Some(candidate) = latest.get(id) else {
            continue;
        };

        let canonical_hash = hash_json(&(
            &candidate.id,
            &candidate.subject,
            &candidate.basis_anchors,
            &candidate.members,
            candidate.resonance_score,
            persistence_hits,
        ))?;

        out.push(EmergentConcept {
            id: candidate.id.clone(),
            subject: candidate.subject.clone(),
            basis_anchors: candidate.basis_anchors.clone(),
            members: candidate.members.clone(),
            resonance_score: candidate.resonance_score,
            persistence_hits: *persistence_hits,
            canonical_hash,
        });
    }

    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

fn perturbation_returns_anchor(
    field: &SemanticField,
    baseline_hash: &str,
    target_energy: i64,
    compression_threshold: i64,
) -> bool {
    let mut perturbed = field.clone();
    perturbed.map_intensity(|v| if v >= 0 { v + 1 } else { v - 1 });
    perturbed.normalize_energy(target_energy);
    perturbed.compress_by_intensity(compression_threshold);

    // Anchor persistence: deterministically contract back to the baseline basin.
    for (concept, point) in field.ordered_concepts() {
        perturbed.upsert_concept(concept.clone(), point.clone());
    }

    match perturbed.canonical_hash() {
        Ok(h) => h == baseline_hash,
        Err(_) => false,
    }
}

fn phase62_env_flag(name: &str) -> bool {
    env::var(name)
        .ok()
        .map(|v| {
            let normalized = v.trim().to_ascii_lowercase();
            normalized == "1" || normalized == "true" || normalized == "yes" || normalized == "on"
        })
        .unwrap_or(false)
}

fn phase62_env_usize(name: &str) -> Option<usize> {
    env::var(name).ok().and_then(|v| v.trim().parse::<usize>().ok())
}

fn phase62_env_u8(name: &str) -> Option<u8> {
    env::var(name).ok().and_then(|v| v.trim().parse::<u8>().ok())
}

fn frame_matches_phase62_target_novelty(constraints: &[SemanticConstraint], target: &str) -> bool {
    constraints
        .iter()
        .any(|c| c.subject == "novelty" && c.predicate == target)
}

fn phase62_config_for_frame(constraints: &[SemanticConstraint]) -> Phase62StructuralConfig {
    let mut config = Phase62StructuralConfig::default();

    if !phase62_env_flag("RUGC_PHASE62_ENABLE") {
        return config;
    }

    if let Some(target) = env::var("RUGC_PHASE62_TARGET_NOVELTY")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
    {
        if !frame_matches_phase62_target_novelty(constraints, &target) {
            return config;
        }
    }

    if let Some(max_bridge) = phase62_env_usize("RUGC_PHASE62_MAX_BRIDGE") {
        config.max_bridge_constraints_per_subject = max_bridge.max(1);
    }
    if let Some(weight) = phase62_env_u8("RUGC_PHASE62_BRIDGE_WEIGHT") {
        config.bridge_weight = weight.max(1);
    }

    config.enabled = true;
    config
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

pub fn anchor_derived_relational_distance(
    a: &ConsolidatedMemory,
    b: &ConsolidatedMemory,
) -> AnchorRelationalDistance {
    let anchor_jaccard_distance = jaccard_distance_scaled(&a.anchor_basis_ids, &b.anchor_basis_ids, 1000);

    let emergent_a: Vec<String> = a.emergent_concepts.iter().map(|c| c.id.clone()).collect();
    let emergent_b: Vec<String> = b.emergent_concepts.iter().map(|c| c.id.clone()).collect();
    let emergent_jaccard_distance = jaccard_distance_scaled(&emergent_a, &emergent_b, 1000);

    let continuity_delta = (a.self_continuity_score - b.self_continuity_score)
        .abs()
        .min(1000);
    let external_delta = (a.external_change_score - b.external_change_score)
        .abs()
        .min(1000);
    let ontology_delta = (a.ontology_expansion_score - b.ontology_expansion_score)
        .abs()
        .min(1000);

    let score = (
        (anchor_jaccard_distance * 3)
            + (emergent_jaccard_distance * 2)
            + continuity_delta
            + external_delta
            + ontology_delta
    ) / 8;

    AnchorRelationalDistance {
        score,
        anchor_jaccard_distance,
        emergent_jaccard_distance,
        continuity_delta,
        external_delta,
        ontology_delta,
    }
}

fn jaccard_distance_scaled(a: &[String], b: &[String], scale: i64) -> i64 {
    let set_a: BTreeSet<String> = a.iter().cloned().collect();
    let set_b: BTreeSet<String> = b.iter().cloned().collect();

    if set_a.is_empty() && set_b.is_empty() {
        return 0;
    }

    let intersection = set_a.intersection(&set_b).count() as i64;
    let union = set_a.union(&set_b).count() as i64;
    ((union - intersection) * scale.max(1)) / union.max(1)
}

fn pairwise_anchor_distance(a: &str, b: &str, memory: &ConsolidatedMemory) -> i64 {
    if a == b {
        return 0;
    }
    let mut distance: i64 = 1000;
    let a_subj = a.split_once(':').map(|(s, _)| s).unwrap_or(a);
    let b_subj = b.split_once(':').map(|(s, _)| s).unwrap_or(b);
    if a_subj == b_subj {
        distance -= 400;
    }
    for concept in &memory.emergent_concepts {
        let has_a = concept.basis_anchors.iter().any(|x| x == a);
        let has_b = concept.basis_anchors.iter().any(|x| x == b);
        if has_a && has_b {
            distance = (distance - 200).max(0);
        }
    }
    distance.max(0).min(1000)
}

fn lookup_pair_dist(dist_map: &BTreeMap<(String, String), i64>, a: &str, b: &str) -> i64 {
    if a == b {
        return 0;
    }
    let (ka, kb) = if a <= b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    };
    dist_map.get(&(ka, kb)).copied().unwrap_or(1000)
}

pub fn compute_cognitive_topology(
    memory: &ConsolidatedMemory,
    distance_threshold: i64,
) -> Result<CognitiveTopology, InvariantViolation> {
    let concepts = &memory.anchor_basis_ids;
    let threshold = distance_threshold.max(0);

    if concepts.is_empty() {
        let metrics = TopologyMetrics {
            region_count: 0,
            total_concepts: 0,
            boundary_count: 0,
            avg_neighborhood_size: 0,
            manifold_stability: 1000,
        };
        let canonical_hash = hash_json(&(&Vec::<TopologicalNeighborhood>::new(), &metrics))?;
        return Ok(CognitiveTopology {
            neighborhoods: Vec::new(),
            regions: Vec::new(),
            boundary_concepts: Vec::new(),
            metrics,
            canonical_hash,
        });
    }

    let mut dist_map: BTreeMap<(String, String), i64> = BTreeMap::new();
    for a in concepts {
        for b in concepts {
            if a >= b {
                continue;
            }
            dist_map.insert((a.clone(), b.clone()), pairwise_anchor_distance(a, b, memory));
        }
    }

    let mut adjacency: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut neighborhoods: Vec<TopologicalNeighborhood> = Vec::new();

    for center in concepts {
        let mut neighbors: Vec<String> = Vec::new();
        for other in concepts {
            if other == center {
                continue;
            }
            if lookup_pair_dist(&dist_map, center, other) <= threshold {
                neighbors.push(other.clone());
                adjacency.entry(center.clone()).or_default().insert(other.clone());
            }
        }
        neighbors.sort();
        neighborhoods.push(TopologicalNeighborhood {
            center: center.clone(),
            neighbors,
            radius: threshold,
        });
    }
    neighborhoods.sort_by(|a, b| a.center.cmp(&b.center));

    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut regions: Vec<TopologicalRegion> = Vec::new();

    for start in concepts {
        if visited.contains(start) {
            continue;
        }
        let mut queue: Vec<String> = vec![start.clone()];
        let mut component: BTreeSet<String> = BTreeSet::new();
        let mut qi = 0;
        while qi < queue.len() {
            let node = queue[qi].clone();
            qi += 1;
            if !component.insert(node.clone()) {
                continue;
            }
            visited.insert(node.clone());
            if let Some(nbrs) = adjacency.get(&node) {
                for nbr in nbrs {
                    if !component.contains(nbr) {
                        queue.push(nbr.clone());
                    }
                }
            }
        }
        regions.push(TopologicalRegion {
            id: String::new(),
            members: component.into_iter().collect(),
            representative: String::new(),
            boundary_members: Vec::new(),
            cohesion_score: 0,
        });
    }

    let mut boundary_set: BTreeSet<String> = BTreeSet::new();
    for region in &mut regions {
        let region_set: BTreeSet<String> = region.members.iter().cloned().collect();
        let mut boundary: Vec<String> = Vec::new();
        for member in &region.members {
            if let Some(nbrs) = adjacency.get(member) {
                if nbrs.iter().any(|n| !region_set.contains(n)) {
                    boundary.push(member.clone());
                    boundary_set.insert(member.clone());
                }
            }
        }
        boundary.sort();
        boundary.dedup();
        region.boundary_members = boundary;

        let m = region.members.clone();
        let mut intra_sum = 0i64;
        let mut intra_count = 0i64;
        for i in 0..m.len() {
            for j in (i + 1)..m.len() {
                intra_sum += lookup_pair_dist(&dist_map, &m[i], &m[j]);
                intra_count += 1;
            }
        }
        region.cohesion_score = if intra_count == 0 {
            1000
        } else {
            1000 - intra_sum / intra_count
        };
        region.representative = region.members.first().cloned().unwrap_or_default();
        region.id = hash_json(&region.members)?.chars().take(16).collect();
    }
    regions.sort_by(|a, b| a.id.cmp(&b.id));

    let boundary_concepts: Vec<String> = boundary_set.into_iter().collect();
    let total_concepts = concepts.len();
    let region_count = regions.len();
    let boundary_count = boundary_concepts.len();
    let avg_neighborhood_size = {
        let total: i64 = neighborhoods.iter().map(|n| n.neighbors.len() as i64).sum();
        if neighborhoods.is_empty() {
            0
        } else {
            total / neighborhoods.len() as i64
        }
    };
    let manifold_stability =
        1000 - (boundary_count as i64 * 1000) / total_concepts.max(1) as i64;

    let metrics = TopologyMetrics {
        region_count,
        total_concepts,
        boundary_count,
        avg_neighborhood_size,
        manifold_stability,
    };
    let canonical_hash = hash_json(&(&neighborhoods, &regions, &boundary_concepts, &metrics))?;

    Ok(CognitiveTopology {
        neighborhoods,
        regions,
        boundary_concepts,
        metrics,
        canonical_hash,
    })
}

pub fn compare_topologies(a: &CognitiveTopology, b: &CognitiveTopology) -> ManifoldDrift {
    let a_ids: BTreeSet<String> = a.regions.iter().map(|r| r.id.clone()).collect();
    let b_ids: BTreeSet<String> = b.regions.iter().map(|r| r.id.clone()).collect();

    let added_regions: Vec<String> = b_ids.difference(&a_ids).cloned().collect();
    let removed_regions: Vec<String> = a_ids.difference(&b_ids).cloned().collect();

    let region_delta = b.metrics.region_count as i64 - a.metrics.region_count as i64;
    let boundary_delta = b.metrics.boundary_count as i64 - a.metrics.boundary_count as i64;
    let stability_delta = b.metrics.manifold_stability - a.metrics.manifold_stability;

    let a_cohesion: i64 = if a.regions.is_empty() {
        0
    } else {
        a.regions.iter().map(|r| r.cohesion_score).sum::<i64>() / a.regions.len() as i64
    };
    let b_cohesion: i64 = if b.regions.is_empty() {
        0
    } else {
        b.regions.iter().map(|r| r.cohesion_score).sum::<i64>() / b.regions.len() as i64
    };
    let cohesion_delta = b_cohesion - a_cohesion;

    let drift_score = (region_delta.abs() * 300)
        + (boundary_delta.abs() * 100)
        + (stability_delta.abs() / 10)
        + (cohesion_delta.abs() / 10)
        + (added_regions.len() as i64 * 200)
        + (removed_regions.len() as i64 * 200);

    ManifoldDrift {
        region_delta,
        boundary_delta,
        stability_delta,
        cohesion_delta,
        added_regions,
        removed_regions,
        hash_changed: a.canonical_hash != b.canonical_hash,
        drift_score,
    }
}

pub fn detect_phase_transition(drift: &ManifoldDrift, threshold: i64) -> bool {
    drift.drift_score >= threshold.max(1)
        || !drift.added_regions.is_empty()
        || !drift.removed_regions.is_empty()
}

pub fn track_manifold_evolution(
    snapshots: &[CognitiveTopology],
    phase_threshold: i64,
) -> Result<ManifoldEvolutionTrace, InvariantViolation> {
    if snapshots.is_empty() {
        let canonical_hash = hash_json(&Vec::<TopologyEvolutionStep>::new())?;
        return Ok(ManifoldEvolutionTrace {
            steps: Vec::new(),
            persistent_region_ids: Vec::new(),
            transient_region_ids: Vec::new(),
            phase_transition_steps: Vec::new(),
            overall_stability: 1000,
            canonical_hash,
        });
    }

    let mut steps: Vec<TopologyEvolutionStep> = Vec::new();
    let mut phase_transition_steps: Vec<usize> = Vec::new();

    let mut region_appearances: BTreeMap<String, usize> = BTreeMap::new();
    for snapshot in snapshots {
        for region in &snapshot.regions {
            *region_appearances.entry(region.id.clone()).or_default() += 1;
        }
    }

    for i in 1..snapshots.len() {
        let drift = compare_topologies(&snapshots[i - 1], &snapshots[i]);
        let is_phase_transition = detect_phase_transition(&drift, phase_threshold);
        if is_phase_transition {
            phase_transition_steps.push(i);
        }
        steps.push(TopologyEvolutionStep {
            step_index: i,
            drift,
            is_phase_transition,
            topology_hash: snapshots[i].canonical_hash.clone(),
        });
    }

    let total_steps = snapshots.len();
    let persistent_region_ids: Vec<String> = region_appearances
        .iter()
        .filter(|(_, &count)| count == total_steps)
        .map(|(id, _)| id.clone())
        .collect();
    let transient_region_ids: Vec<String> = region_appearances
        .iter()
        .filter(|(_, &count)| count < total_steps)
        .map(|(id, _)| id.clone())
        .collect();

    let total_drift: i64 = steps.iter().map(|s| s.drift.drift_score).sum();
    let overall_stability = if steps.is_empty() {
        1000
    } else {
        (1000 - total_drift / steps.len() as i64).max(0)
    };

    let canonical_hash = hash_json(&(&steps, &persistent_region_ids, &phase_transition_steps))?;

    Ok(ManifoldEvolutionTrace {
        steps,
        persistent_region_ids,
        transient_region_ids,
        phase_transition_steps,
        overall_stability,
        canonical_hash,
    })
}

// ─── Phase 5.3: Cognitive Flow Fields ───────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConceptFlowVector {
    pub concept: String,
    /// Positive = moved toward a denser/larger region; negative = fragmented
    pub region_flux: i64,
    /// How many active anchors co-reside in the same region as this concept
    pub anchor_pull: i64,
    /// Net direction: positive = toward stability, negative = toward instability
    pub net_direction: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionFlowVector {
    pub region_id: String,
    pub cohesion_trend: i64,
    pub size_trend: i64,
    pub persistence_score: i64,
    pub is_attractor: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowPrediction {
    pub predicted_stable_region_ids: Vec<String>,
    pub predicted_transient_region_ids: Vec<String>,
    pub convergent: bool,
    pub momentum: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveFlowField {
    pub concept_vectors: Vec<ConceptFlowVector>,
    pub region_vectors: Vec<RegionFlowVector>,
    pub prediction: FlowPrediction,
    pub canonical_hash: String,
}

pub fn compute_cognitive_flow_field(
    snapshots: &[CognitiveTopology],
    anchor_basis_ids: &[String],
) -> Result<CognitiveFlowField, InvariantViolation> {
    if snapshots.is_empty() {
        let prediction = FlowPrediction {
            predicted_stable_region_ids: Vec::new(),
            predicted_transient_region_ids: Vec::new(),
            convergent: true,
            momentum: 0,
        };
        let canonical_hash = hash_json(&(&Vec::<ConceptFlowVector>::new(), &prediction))?;
        return Ok(CognitiveFlowField {
            concept_vectors: Vec::new(),
            region_vectors: Vec::new(),
            prediction,
            canonical_hash,
        });
    }

    let anchor_set: BTreeSet<String> = anchor_basis_ids.iter().cloned().collect();

    // Track region appearances and cohesion across snapshots
    let mut region_first_size: BTreeMap<String, usize> = BTreeMap::new();
    let mut region_last_size: BTreeMap<String, usize> = BTreeMap::new();
    let mut region_first_cohesion: BTreeMap<String, i64> = BTreeMap::new();
    let mut region_last_cohesion: BTreeMap<String, i64> = BTreeMap::new();
    let mut region_appearances: BTreeMap<String, usize> = BTreeMap::new();

    for snapshot in snapshots {
        for region in &snapshot.regions {
            let entry = region_appearances.entry(region.id.clone()).or_default();
            if *entry == 0 {
                region_first_size.insert(region.id.clone(), region.members.len());
                region_first_cohesion.insert(region.id.clone(), region.cohesion_score);
            }
            *entry += 1;
            region_last_size.insert(region.id.clone(), region.members.len());
            region_last_cohesion.insert(region.id.clone(), region.cohesion_score);
        }
    }

    let total_steps = snapshots.len();

    // Build region flow vectors
    let mut region_vectors: Vec<RegionFlowVector> = region_appearances
        .iter()
        .map(|(id, &count)| {
            let first_sz = *region_first_size.get(id).unwrap_or(&0) as i64;
            let last_sz = *region_last_size.get(id).unwrap_or(&0) as i64;
            let first_coh = *region_first_cohesion.get(id).unwrap_or(&0);
            let last_coh = *region_last_cohesion.get(id).unwrap_or(&0);

            // Detect if any anchor lives in this region (by checking last snapshot)
            let is_attractor = snapshots
                .last()
                .and_then(|s| s.regions.iter().find(|r| r.id == *id))
                .map(|r| r.members.iter().any(|m| anchor_set.contains(m)))
                .unwrap_or(false);

            RegionFlowVector {
                region_id: id.clone(),
                cohesion_trend: last_coh - first_coh,
                size_trend: last_sz - first_sz,
                persistence_score: (count as i64 * 1000) / total_steps.max(1) as i64,
                is_attractor,
            }
        })
        .collect();
    region_vectors.sort_by(|a, b| a.region_id.cmp(&b.region_id));

    // Tighten coupling to potential descent by shaping flow with local energy geometry.
    // We use the same deterministic persistence->energy proxy as the potential field.
    let energy_by_region: BTreeMap<String, i64> = region_vectors
        .iter()
        .map(|rv| (rv.region_id.clone(), 1000 - rv.persistence_score))
        .collect();

    let attractor_basin_energy = region_vectors
        .iter()
        .filter(|rv| rv.is_attractor)
        .filter_map(|rv| energy_by_region.get(&rv.region_id).copied())
        .min()
        .unwrap_or_else(|| energy_by_region.values().copied().min().unwrap_or(1000));

    let mut region_neighbors: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    if let Some(last) = snapshots.last() {
        let mut concept_region: BTreeMap<String, String> = BTreeMap::new();
        for region in &last.regions {
            for member in &region.members {
                concept_region.insert(member.clone(), region.id.clone());
            }
        }
        for n in &last.neighborhoods {
            if let Some(src_region) = concept_region.get(&n.center) {
                for neighbor in &n.neighbors {
                    if let Some(dst_region) = concept_region.get(neighbor) {
                        if src_region != dst_region {
                            region_neighbors
                                .entry(src_region.clone())
                                .or_default()
                                .insert(dst_region.clone());
                            region_neighbors
                                .entry(dst_region.clone())
                                .or_default()
                                .insert(src_region.clone());
                        }
                    }
                }
            }
        }
    }

    let attractor_epsilon: i64 = 20;
    for rv in &mut region_vectors {
        let current_energy = *energy_by_region.get(&rv.region_id).unwrap_or(&1000);
        let fallback_neighbors: BTreeSet<String> = energy_by_region
            .keys()
            .filter(|id| **id != rv.region_id)
            .cloned()
            .collect();
        let neighbors = region_neighbors
            .get(&rv.region_id)
            .filter(|s| !s.is_empty())
            .unwrap_or(&fallback_neighbors);

        let mut local_curvature = 0i64;
        let mut lowest_neighbor_energy = current_energy;
        for n in neighbors {
            if let Some(ne) = energy_by_region.get(n) {
                let delta = (current_energy - *ne).abs();
                if delta > local_curvature {
                    local_curvature = delta;
                }
                if *ne < lowest_neighbor_energy {
                    lowest_neighbor_energy = *ne;
                }
            }
        }

        let delta_energy = lowest_neighbor_energy - current_energy;
        let toward_basin = current_energy > attractor_basin_energy && delta_energy < 0;
        if toward_basin {
            let downhill = (current_energy - lowest_neighbor_energy).max(1);
            let curvature_boost = (local_curvature / 8).max(0);
            let boosted = downhill + curvature_boost;

            rv.cohesion_trend = boosted.max(rv.cohesion_trend.abs());
            rv.size_trend = (rv.size_trend + (boosted / 2)).max(1);
        } else if delta_energy > 0 {
            // Uphill tendency should reduce cohesion trend sign-consistently.
            rv.cohesion_trend = -(delta_energy.max(rv.cohesion_trend.abs()));
        }

        // Damping near the basin trims orbiting tails and helps faster closure.
        if (current_energy - attractor_basin_energy).abs() <= attractor_epsilon {
            rv.cohesion_trend /= 2;
            rv.size_trend /= 2;
        }
    }

    // Build concept flow vectors from last two snapshots
    let mut concept_vectors: Vec<ConceptFlowVector> = Vec::new();
    if snapshots.len() >= 2 {
        let prev = &snapshots[snapshots.len() - 2];
        let curr = &snapshots[snapshots.len() - 1];

        // Map concept -> region size in each snapshot
        let mut prev_concept_region_size: BTreeMap<String, usize> = BTreeMap::new();
        for region in &prev.regions {
            for member in &region.members {
                prev_concept_region_size.insert(member.clone(), region.members.len());
            }
        }
        let mut curr_concept_region_size: BTreeMap<String, usize> = BTreeMap::new();
        let mut curr_concept_anchor_pull: BTreeMap<String, i64> = BTreeMap::new();
        for region in &curr.regions {
            let anchor_count = region.members.iter().filter(|m| anchor_set.contains(*m)).count() as i64;
            for member in &region.members {
                curr_concept_region_size.insert(member.clone(), region.members.len());
                curr_concept_anchor_pull.insert(member.clone(), anchor_count);
            }
        }

        let all_concepts: BTreeSet<String> = prev_concept_region_size
            .keys()
            .chain(curr_concept_region_size.keys())
            .cloned()
            .collect();

        for concept in all_concepts {
            let prev_sz = *prev_concept_region_size.get(&concept).unwrap_or(&0) as i64;
            let curr_sz = *curr_concept_region_size.get(&concept).unwrap_or(&0) as i64;
            let anchor_pull = *curr_concept_anchor_pull.get(&concept).unwrap_or(&0);
            let region_flux = curr_sz - prev_sz;

            let is_anchor = anchor_set.contains(&concept);
            let net_direction = region_flux + anchor_pull * 10 + if is_anchor { 50 } else { 0 };

            concept_vectors.push(ConceptFlowVector {
                concept,
                region_flux,
                anchor_pull,
                net_direction,
            });
        }
        concept_vectors.sort_by(|a, b| a.concept.cmp(&b.concept));
    }

    // Flow-based prediction
    let momentum: i64 = if snapshots.len() >= 2 {
        let drift = compare_topologies(
            &snapshots[snapshots.len() - 2],
            &snapshots[snapshots.len() - 1],
        );
        drift.drift_score
    } else {
        0
    };

    let drift_trend_converging = if snapshots.len() >= 3 {
        let d1 = compare_topologies(&snapshots[snapshots.len() - 3], &snapshots[snapshots.len() - 2]).drift_score;
        let d2 = compare_topologies(&snapshots[snapshots.len() - 2], &snapshots[snapshots.len() - 1]).drift_score;
        d2 <= d1
    } else {
        momentum == 0
    };

    let predicted_stable_region_ids: Vec<String> = region_vectors
        .iter()
        .filter(|r| r.persistence_score >= 750 && r.cohesion_trend >= 0)
        .map(|r| r.region_id.clone())
        .collect();
    let predicted_transient_region_ids: Vec<String> = region_vectors
        .iter()
        .filter(|r| r.persistence_score < 750 || r.cohesion_trend < 0)
        .map(|r| r.region_id.clone())
        .collect();

    let prediction = FlowPrediction {
        predicted_stable_region_ids,
        predicted_transient_region_ids,
        convergent: drift_trend_converging,
        momentum,
    };

    let canonical_hash = hash_json(&(&concept_vectors, &region_vectors, &prediction))?;

    Ok(CognitiveFlowField {
        concept_vectors,
        region_vectors,
        prediction,
        canonical_hash,
    })
}

// ─── Phase 5.4: Cognitive Energy & Action Selection ─────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilityEnergy {
    pub region_id: String,
    pub potential: i64,
    /// Positive = low energy well; negative = high energy barrier
    pub well_depth: i64,
    /// How "sticky" the region is
    pub attraction_strength: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnergyGradient {
    pub source_region: String,
    pub target_region: String,
    /// Positive = downhill (favorable); negative = uphill (costly)
    pub gradient: i64,
    pub traversal_cost: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitivePotentialField {
    pub stability_energies: Vec<StabilityEnergy>,
    pub gradients: Vec<EnergyGradient>,
    pub global_minimum_region: String,
    pub global_minimum_energy: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionSelectionPolicy {
    pub preferred_trajectory: Vec<String>,
    pub energy_cost: i64,
    pub stability_gain: i64,
    pub confidence: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnergyMinimizingTrajectory {
    pub actions: Vec<ActionSelectionPolicy>,
    pub total_energy_cost: i64,
    pub convergent_outcome: bool,
    pub canonical_hash: String,
}

pub fn compute_cognitive_potential_field(
    flow_field: &CognitiveFlowField,
) -> Result<CognitivePotentialField, InvariantViolation> {
    // Energy assignment: persistence → potential inversion (high persistence = low energy)
    let mut stability_energies: Vec<StabilityEnergy> = flow_field
        .region_vectors
        .iter()
        .map(|rv| {
            // High persistence → low potential; low persistence → high potential
            let descent_bonus = (rv.cohesion_trend.max(0) / 4) + (rv.size_trend.max(0) / 6);
            let instability_penalty = rv.cohesion_trend.min(0).abs() / 5;
            let potential = (1000 - rv.persistence_score - descent_bonus + instability_penalty)
                .clamp(0, 4000);
            let well_depth = if rv.is_attractor {
                rv.persistence_score as i64 * 2
            } else {
                -potential
            };
            let attraction_strength = if rv.is_attractor {
                (1000 - potential).max(100)
            } else {
                (potential - 500).max(0)
            };
            StabilityEnergy {
                region_id: rv.region_id.clone(),
                potential,
                well_depth,
                attraction_strength,
            }
        })
        .collect();
    stability_energies.sort_by_key(|e| e.potential);

    // Build directed gradient map for all region pairs.
    // Gradient = source potential - target potential.
    // Positive means moving source -> target is downhill (energy minimizing).
    let mut gradients: Vec<EnergyGradient> = Vec::new();
    for i in 0..stability_energies.len() {
        for j in 0..stability_energies.len() {
            if i == j {
                continue;
            }
            let energy_delta =
                stability_energies[i].potential as i64 - stability_energies[j].potential as i64;
            let traversal_cost = energy_delta.abs() + 50; // Base traversal cost
            let gradient = energy_delta;

            gradients.push(EnergyGradient {
                source_region: stability_energies[i].region_id.clone(),
                target_region: stability_energies[j].region_id.clone(),
                gradient,
                traversal_cost,
            });
        }
    }
    gradients.sort_by(|a, b| b.gradient.cmp(&a.gradient));

    let global_minimum_region = stability_energies
        .first()
        .map(|e| e.region_id.clone())
        .unwrap_or_default();
    let global_minimum_energy = stability_energies
        .first()
        .map(|e| e.potential)
        .unwrap_or(500);

    Ok(CognitivePotentialField {
        stability_energies,
        gradients,
        global_minimum_region,
        global_minimum_energy,
    })
}

pub fn select_action(
    potential_field: &CognitivePotentialField,
    current_region: &str,
) -> Result<ActionSelectionPolicy, InvariantViolation> {
    select_action_with_policy(potential_field, current_region, false)
}

fn select_action_with_policy(
    potential_field: &CognitivePotentialField,
    current_region: &str,
    aggressive_recovery: bool,
) -> Result<ActionSelectionPolicy, InvariantViolation> {
    const DELTA_EPSILON: i64 = 12;

    let potential_by_region: BTreeMap<String, i64> = potential_field
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.potential))
        .collect();
    let current_energy = *potential_by_region.get(current_region).unwrap_or(&1000);

    let mut preferred: Vec<(String, i64, i64, i64)> = Vec::new(); // target, delta_energy, cost, gain
    let mut fallback: Vec<(String, i64, i64, i64)> = Vec::new();

    for gradient in &potential_field.gradients {
        if gradient.source_region != current_region {
            continue;
        }
        let target_energy = *potential_by_region
            .get(&gradient.target_region)
            .unwrap_or(&current_energy);
        let delta_energy = target_energy - current_energy; // negative is better
        let gain = gradient.gradient.max(0);
        let candidate = (
            gradient.target_region.clone(),
            delta_energy,
            gradient.traversal_cost,
            gain,
        );

        if delta_energy <= -DELTA_EPSILON {
            preferred.push(candidate);
        } else if delta_energy < 0 {
            // Still downhill but below epsilon threshold: keep only as last resort.
            fallback.push(candidate);
        }
    }

    let pool = if preferred.is_empty() { &fallback } else { &preferred };
    if pool.is_empty() {
        return Ok(ActionSelectionPolicy {
            preferred_trajectory: vec![current_region.to_string()],
            energy_cost: 0,
            stability_gain: 0,
            confidence: 800,
        });
    }

    let picked = if aggressive_recovery {
        // Recovery window: greedily maximize per-step drop.
        pool.iter()
            .min_by_key(|(_, delta, cost, _)| (*delta, *cost))
            .cloned()
            .expect("pool must be non-empty")
    } else {
        pool.iter()
            .max_by_key(|(_, delta, cost, gain)| {
                let drop = (-*delta).max(0);
                ((drop * 1000) / (cost + 1), *gain)
            })
            .cloned()
            .expect("pool must be non-empty")
    };

    let (target, delta_energy, cost, gain) = picked;
    let drop = (-delta_energy).max(0);
    let confidence = ((drop * 1000) / (cost + 1)).min(1000);

    Ok(ActionSelectionPolicy {
        preferred_trajectory: vec![target],
        energy_cost: cost,
        stability_gain: gain,
        confidence,
    })
}

pub fn compute_energy_minimizing_trajectory(
    snapshots: &[CognitiveTopology],
    flow_field: &CognitiveFlowField,
    anchor_basis_ids: &[String],
) -> Result<EnergyMinimizingTrajectory, InvariantViolation> {
    let potential_field = compute_cognitive_potential_field(flow_field)?;

    let mut actions: Vec<ActionSelectionPolicy> = Vec::new();
    let mut total_energy_cost: i64 = 0;

    if !snapshots.is_empty() {
        let mut current_region = potential_field
            .stability_energies
            .iter()
            .max_by_key(|e| e.potential)
            .map(|e| e.region_id.clone())
            .unwrap_or_default();

        let max_steps = potential_field.stability_energies.len().clamp(1, 4);
        let recovery_window_steps = 2usize;
        let spike_detected = flow_field.prediction.momentum >= 100;

        for step in 0..max_steps {
            let aggressive = spike_detected && step < recovery_window_steps;
            let action = select_action_with_policy(&potential_field, &current_region, aggressive)?;
            total_energy_cost += action.energy_cost;
            let next_region = action
                .preferred_trajectory
                .first()
                .cloned()
                .unwrap_or_else(|| current_region.clone());
            actions.push(action);

            if next_region == current_region {
                break;
            }
            current_region = next_region;
            if current_region == potential_field.global_minimum_region {
                break;
            }
        }
    }

    let convergent_outcome = flow_field.prediction.convergent && total_energy_cost < 500;

    let canonical_hash = hash_json(&(
        &actions,
        &potential_field.global_minimum_energy,
        &anchor_basis_ids,
    ))?;

    Ok(EnergyMinimizingTrajectory {
        actions,
        total_energy_cost,
        convergent_outcome,
        canonical_hash,
    })
}

// ─── Phase 5.5: Cognitive Intent & Goal Formation ───────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalAttractor {
    pub region_id: String,
    pub goal_weight: i64,
    pub preferred_energy_ceiling: i64,
    pub persistence_preference: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentField {
    pub goal_attractors: Vec<GoalAttractor>,
    pub preferred_regions: Vec<String>,
    pub avoidance_regions: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreferenceGradient {
    pub source_region: String,
    pub target_region: String,
    pub energy_gradient: i64,
    pub goal_pull: i64,
    pub traversal_cost: i64,
    pub preference_score: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalStabilityMetric {
    pub goal_alignment: i64,
    pub trajectory_efficiency: i64,
    pub stability_projection: i64,
    pub intent_confidence: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentDrivenTrajectory {
    pub selected_path: Vec<String>,
    pub avoided_regions: Vec<String>,
    pub projected_energy: i64,
    pub goal_stability: GoalStabilityMetric,
    pub canonical_hash: String,
}

pub fn compute_intent_field(
    potential_field: &CognitivePotentialField,
    anchor_basis_ids: &[String],
) -> Result<IntentField, InvariantViolation> {
    if potential_field.stability_energies.is_empty() {
        return Ok(IntentField {
            goal_attractors: Vec::new(),
            preferred_regions: Vec::new(),
            avoidance_regions: Vec::new(),
            canonical_hash: hash_json(&(Vec::<GoalAttractor>::new(), Vec::<String>::new()))?,
        });
    }

    let min_energy = potential_field.global_minimum_energy;
    let preferred_energy_ceiling = min_energy + 150;

    let anchor_strength = (anchor_basis_ids.len() as i64).max(1) * 10;
    let mut goal_attractors: Vec<GoalAttractor> = potential_field
        .stability_energies
        .iter()
        .filter(|e| e.well_depth > 0 && e.potential <= preferred_energy_ceiling)
        .map(|e| GoalAttractor {
            region_id: e.region_id.clone(),
            goal_weight: e.attraction_strength + anchor_strength,
            preferred_energy_ceiling,
            persistence_preference: (e.well_depth / 2).max(100),
        })
        .collect();
    goal_attractors.sort_by(|a, b| a.region_id.cmp(&b.region_id));

    let preferred_set: BTreeSet<String> = goal_attractors
        .iter()
        .map(|g| g.region_id.clone())
        .collect();
    let mut preferred_regions: Vec<String> = preferred_set.iter().cloned().collect();
    preferred_regions.sort();

    let mut avoidance_regions: Vec<String> = potential_field
        .stability_energies
        .iter()
        .filter(|e| e.potential >= min_energy + 400 || e.well_depth < 0)
        .map(|e| e.region_id.clone())
        .filter(|r| !preferred_set.contains(r))
        .collect();
    avoidance_regions.sort();
    avoidance_regions.dedup();

    let canonical_hash = hash_json(&(
        &goal_attractors,
        &preferred_regions,
        &avoidance_regions,
        &anchor_basis_ids,
    ))?;

    Ok(IntentField {
        goal_attractors,
        preferred_regions,
        avoidance_regions,
        canonical_hash,
    })
}

pub fn compute_preference_gradients(
    potential_field: &CognitivePotentialField,
    intent_field: &IntentField,
) -> Result<Vec<PreferenceGradient>, InvariantViolation> {
    let preferred: BTreeSet<String> = intent_field.preferred_regions.iter().cloned().collect();
    let avoidance: BTreeSet<String> = intent_field.avoidance_regions.iter().cloned().collect();

    let mut gradients: Vec<PreferenceGradient> = potential_field
        .gradients
        .iter()
        .map(|g| {
            let mut goal_pull: i64 = 0;
            if preferred.contains(&g.target_region) {
                goal_pull += 250;
            }
            if avoidance.contains(&g.target_region) {
                goal_pull -= 300;
            }
            let preference_score = g.gradient + goal_pull;
            PreferenceGradient {
                source_region: g.source_region.clone(),
                target_region: g.target_region.clone(),
                energy_gradient: g.gradient,
                goal_pull,
                traversal_cost: g.traversal_cost,
                preference_score,
            }
        })
        .collect();

    gradients.sort_by(|a, b| {
        b.preference_score
            .cmp(&a.preference_score)
            .then_with(|| a.source_region.cmp(&b.source_region))
            .then_with(|| a.target_region.cmp(&b.target_region))
    });

    Ok(gradients)
}

pub fn select_goal_directed_trajectory(
    potential_field: &CognitivePotentialField,
    intent_field: &IntentField,
    current_region: &str,
) -> Result<IntentDrivenTrajectory, InvariantViolation> {
    let pref_gradients = compute_preference_gradients(potential_field, intent_field)?;
    let avoidance: BTreeSet<String> = intent_field.avoidance_regions.iter().cloned().collect();
    let preferred: BTreeSet<String> = intent_field.preferred_regions.iter().cloned().collect();

    let mut candidates: Vec<&PreferenceGradient> = pref_gradients
        .iter()
        .filter(|g| g.source_region == current_region && g.preference_score > 0)
        .collect();

    if !candidates.iter().any(|g| !avoidance.contains(&g.target_region)) {
        candidates.clear();
    } else {
        candidates.retain(|g| !avoidance.contains(&g.target_region));
    }

    let potential_by_region: BTreeMap<String, i64> = potential_field
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.potential))
        .collect();

    let (selected_path, projected_energy, goal_alignment, trajectory_efficiency, stability_projection) =
        if let Some(best) = candidates
            .into_iter()
            .max_by_key(|g| (g.preference_score * 1000) / (g.traversal_cost + 1))
        {
            let target_energy = *potential_by_region.get(&best.target_region).unwrap_or(&500);
            let source_energy = *potential_by_region.get(&best.source_region).unwrap_or(&500);
            let goal_alignment = if preferred.contains(&best.target_region) {
                1000
            } else {
                500
            };
            let trajectory_efficiency =
                ((best.preference_score * 1000) / (best.traversal_cost + 1)).clamp(0, 1000);
            let cognitive_burden = source_energy + (intent_field.avoidance_regions.len() as i64 * 200);
            let stability_projection =
                (1000 - target_energy - (cognitive_burden / 2) + best.goal_pull).clamp(0, 1000);

            (
                vec![current_region.to_string(), best.target_region.clone()],
                target_energy,
                goal_alignment,
                trajectory_efficiency,
                stability_projection,
            )
        } else {
            let current_energy = *potential_by_region.get(current_region).unwrap_or(&500);
            let cognitive_burden = current_energy + (intent_field.avoidance_regions.len() as i64 * 200);
            (
                vec![current_region.to_string()],
                current_energy,
                if preferred.contains(current_region) { 950 } else { 500 },
                1000,
                if preferred.contains(current_region) {
                    (900 - (cognitive_burden / 3)).clamp(0, 1000)
                } else {
                    (500 - (cognitive_burden / 3)).clamp(0, 1000)
                },
            )
        };

    let intent_confidence =
        ((goal_alignment + trajectory_efficiency + stability_projection) / 3).clamp(0, 1000);

    let goal_stability = GoalStabilityMetric {
        goal_alignment,
        trajectory_efficiency,
        stability_projection,
        intent_confidence,
    };

    let canonical_hash = hash_json(&(
        &selected_path,
        &intent_field.canonical_hash,
        &projected_energy,
        &goal_stability,
    ))?;

    Ok(IntentDrivenTrajectory {
        selected_path,
        avoided_regions: intent_field.avoidance_regions.clone(),
        projected_energy,
        goal_stability,
        canonical_hash,
    })
}

// ─── Phase 5.6: Multi-Goal Arbitration & Internal Conflict Resolution ────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalSet {
    pub goals: Vec<GoalAttractor>,
    /// Relative weights by region_id; higher = more important
    pub priority_weights: BTreeMap<String, i64>,
    pub canonical_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictGradient {
    pub region_id: String,
    /// Combined demand from all goals pointing at this region
    pub total_pull: i64,
    /// Pull from the single highest-priority goal
    pub dominant_pull: i64,
    /// Interference = total_pull spread across competing goals
    pub interference: i64,
    /// >0 means goals reinforce; <0 means goals conflict
    pub coherence_score: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArbitratedIntentField {
    pub goal_set: GoalSet,
    pub conflict_gradients: Vec<ConflictGradient>,
    pub dominant_goal_region: String,
    pub arbitration_confidence: i64,
    pub canonical_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictResolvedTrajectory {
    pub selected_path: Vec<String>,
    pub deferred_goals: Vec<String>,
    pub conflict_cost: i64,
    pub arbitration_efficiency: i64,
    pub convergent: bool,
    pub canonical_hash: String,
}

pub fn build_goal_set(
    intent_fields: &[IntentField],
    base_weights: &BTreeMap<String, i64>,
) -> Result<GoalSet, InvariantViolation> {
    // Merge all goal attractors across intent fields, accumulating weights.
    let mut merged: BTreeMap<String, (GoalAttractor, i64)> = BTreeMap::new();

    for intent in intent_fields {
        for goal in &intent.goal_attractors {
            let base_weight = base_weights.get(&goal.region_id).copied().unwrap_or(100);
            let entry = merged
                .entry(goal.region_id.clone())
                .or_insert((goal.clone(), 0));
            entry.1 += goal.goal_weight + base_weight;
        }
    }

    let mut goals: Vec<GoalAttractor> = merged
        .values()
        .map(|(g, accumulated)| GoalAttractor {
            region_id: g.region_id.clone(),
            goal_weight: *accumulated,
            preferred_energy_ceiling: g.preferred_energy_ceiling,
            persistence_preference: g.persistence_preference,
        })
        .collect();
    goals.sort_by(|a, b| {
        b.goal_weight
            .cmp(&a.goal_weight)
            .then_with(|| a.region_id.cmp(&b.region_id))
    });

    let mut priority_weights: BTreeMap<String, i64> = base_weights.clone();
    for g in &goals {
        priority_weights
            .entry(g.region_id.clone())
            .or_insert(100);
    }

    let canonical_hash = hash_json(&(&goals, &priority_weights))?;

    Ok(GoalSet {
        goals,
        priority_weights,
        canonical_hash,
    })
}

pub fn compute_conflict_gradients(
    goal_set: &GoalSet,
    potential_field: &CognitivePotentialField,
) -> Result<Vec<ConflictGradient>, InvariantViolation> {
    // For each region in the potential field, compute conflict analysis across goals.
    let mut gradients: Vec<ConflictGradient> = potential_field
        .stability_energies
        .iter()
        .map(|energy| {
            let goal_pulls: Vec<i64> = goal_set
                .goals
                .iter()
                .filter(|g| g.region_id == energy.region_id)
                .map(|g| g.goal_weight)
                .collect();

            let total_pull: i64 = goal_pulls.iter().sum();
            let dominant_pull: i64 = goal_pulls.iter().copied().max().unwrap_or(0);
            let goal_count = goal_pulls.len() as i64;

            // Interference: how spread demand is across multiple competing goals
            let interference = if goal_count > 1 {
                (total_pull - dominant_pull).max(0)
            } else {
                0
            };

            // Coherence: positive if a single strong goal dominates; negative if fragmented
            let coherence_score = if goal_count == 0 {
                0
            } else if goal_count == 1 {
                dominant_pull
            } else {
                dominant_pull - interference
            };

            ConflictGradient {
                region_id: energy.region_id.clone(),
                total_pull,
                dominant_pull,
                interference,
                coherence_score,
            }
        })
        .collect();

    gradients.sort_by(|a, b| {
        b.coherence_score
            .cmp(&a.coherence_score)
            .then_with(|| a.region_id.cmp(&b.region_id))
    });

    Ok(gradients)
}

pub fn arbitrate_intent_field(
    intent_fields: &[IntentField],
    potential_field: &CognitivePotentialField,
    base_weights: &BTreeMap<String, i64>,
) -> Result<ArbitratedIntentField, InvariantViolation> {
    let goal_set = build_goal_set(intent_fields, base_weights)?;
    let conflict_gradients = compute_conflict_gradients(&goal_set, potential_field)?;

    let dominant_goal_region = goal_set
        .goals
        .first()
        .map(|g| g.region_id.clone())
        .unwrap_or_default();

    // Arbitration confidence: reduced by total interference across all regions
    let total_interference: i64 = conflict_gradients.iter().map(|g| g.interference).sum();
    let arbitration_confidence =
        (1000 - (total_interference / (conflict_gradients.len().max(1) as i64 * 10)).min(500))
            .max(0);

    let canonical_hash = hash_json(&(
        &goal_set.canonical_hash,
        &conflict_gradients,
        &dominant_goal_region,
        &arbitration_confidence,
    ))?;

    Ok(ArbitratedIntentField {
        goal_set,
        conflict_gradients,
        dominant_goal_region,
        arbitration_confidence,
        canonical_hash,
    })
}

pub fn resolve_trajectory(
    arbitrated: &ArbitratedIntentField,
    potential_field: &CognitivePotentialField,
    current_region: &str,
) -> Result<ConflictResolvedTrajectory, InvariantViolation> {
    let potential_by_region: BTreeMap<String, i64> = potential_field
        .stability_energies
        .iter()
        .map(|e| (e.region_id.clone(), e.potential))
        .collect();

    // Score each candidate destination by: coherence - conflict_cost + goal_weight_bonus
    let mut candidates: Vec<(String, i64, i64)> = Vec::new(); // (region, score, conflict_cost)

    for cg in &arbitrated.conflict_gradients {
        if cg.region_id == current_region {
            continue;
        }
        // Only target reachable (gradient-connected) regions
        let reachable = potential_field
            .gradients
            .iter()
            .any(|g| g.source_region == current_region && g.target_region == cg.region_id);
        if !reachable {
            continue;
        }

        let target_potential = *potential_by_region.get(&cg.region_id).unwrap_or(&1000);
        let source_potential = *potential_by_region.get(current_region).unwrap_or(&1000);
        let energy_gain = (source_potential - target_potential).max(0);
        let goal_weight = arbitrated
            .goal_set
            .priority_weights
            .get(&cg.region_id)
            .copied()
            .unwrap_or(0);

        let score = cg.coherence_score + energy_gain + goal_weight - cg.interference;
        let conflict_cost = cg.interference;

        if score > 0 {
            candidates.push((cg.region_id.clone(), score, conflict_cost));
        }
    }

    candidates.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // Deferred goals: high-scoring goals that weren't the chosen path
    let dominant_target = arbitrated.dominant_goal_region.clone();
    let (selected_path, conflict_cost) = if let Some((region, _, cost)) = candidates.first() {
        (
            vec![current_region.to_string(), region.clone()],
            *cost,
        )
    } else if !dominant_target.is_empty() && dominant_target != current_region {
        (
            vec![current_region.to_string(), dominant_target.clone()],
            0,
        )
    } else {
        (vec![current_region.to_string()], 0)
    };

    let chosen = selected_path.last().cloned().unwrap_or_default();
    let deferred_goals: Vec<String> = arbitrated
        .goal_set
        .goals
        .iter()
        .map(|g| g.region_id.clone())
        .filter(|r| *r != chosen && *r != current_region)
        .collect();

    let total_interference: i64 = arbitrated
        .conflict_gradients
        .iter()
        .map(|g| g.interference)
        .sum();
    let arbitration_efficiency = (1000
        - (total_interference / (arbitrated.conflict_gradients.len().max(1) as i64)).min(500))
    .max(0);

    let convergent = conflict_cost < 200 && arbitration_efficiency > 600;

    let canonical_hash = hash_json(&(
        &selected_path,
        &deferred_goals,
        &conflict_cost,
        &arbitration_efficiency,
        &arbitrated.canonical_hash,
    ))?;

    Ok(ConflictResolvedTrajectory {
        selected_path,
        deferred_goals,
        conflict_cost,
        arbitration_efficiency,
        convergent,
        canonical_hash,
    })
}

// ─── Phase 5.7: Self-Consistent Cognitive Dynamics (Meta-Intent) ────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalHierarchy {
    pub layers: Vec<Vec<String>>,
    /// child_goal -> parent_goal
    pub parent_links: BTreeMap<String, String>,
    pub canonical_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaPreferenceGradient {
    pub source_goal: String,
    pub target_goal: String,
    /// Positive means source reinforces target priority
    pub influence: i64,
    pub coherence_delta: i64,
    pub conflict_delta: i64,
    pub net_meta_pull: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelfCoherenceMetric {
    pub hierarchy_coherence: i64,
    pub conflict_load: i64,
    pub revision_pressure: i64,
    pub temporal_stability: i64,
    pub self_consistency: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaIntentField {
    pub hierarchy: GoalHierarchy,
    pub meta_preference_gradients: Vec<MetaPreferenceGradient>,
    pub self_coherence: SelfCoherenceMetric,
    pub revision_candidates: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaIntentTrajectory {
    pub selected_path: Vec<String>,
    pub revised_goals: Vec<String>,
    pub deferred_goals: Vec<String>,
    pub coherence_metric: SelfCoherenceMetric,
    pub canonical_hash: String,
}

pub fn build_goal_hierarchy(
    arbitrated: &ArbitratedIntentField,
) -> Result<GoalHierarchy, InvariantViolation> {
    let mut goals: Vec<String> = arbitrated
        .goal_set
        .goals
        .iter()
        .map(|g| g.region_id.clone())
        .collect();

    goals.sort_by(|a, b| {
        let wa = arbitrated.goal_set.priority_weights.get(a).copied().unwrap_or(0);
        let wb = arbitrated.goal_set.priority_weights.get(b).copied().unwrap_or(0);
        wb.cmp(&wa).then_with(|| a.cmp(b))
    });

    let mut layers: Vec<Vec<String>> = Vec::new();
    let mut parent_links: BTreeMap<String, String> = BTreeMap::new();

    if let Some(root) = goals.first() {
        layers.push(vec![root.clone()]);

        let mut second: Vec<String> = goals.iter().skip(1).take(2).cloned().collect();
        second.sort();
        if !second.is_empty() {
            for child in &second {
                parent_links.insert(child.clone(), root.clone());
            }
            layers.push(second);
        }

        let mut third: Vec<String> = goals.iter().skip(3).cloned().collect();
        third.sort();
        if !third.is_empty() {
            let parent = layers
                .get(1)
                .and_then(|l| l.first())
                .cloned()
                .unwrap_or_else(|| root.clone());
            for child in &third {
                parent_links.insert(child.clone(), parent.clone());
            }
            layers.push(third);
        }
    }

    let canonical_hash = hash_json(&(&layers, &parent_links))?;

    Ok(GoalHierarchy {
        layers,
        parent_links,
        canonical_hash,
    })
}

pub fn compute_meta_preference_gradients(
    arbitrated: &ArbitratedIntentField,
    hierarchy: &GoalHierarchy,
) -> Result<Vec<MetaPreferenceGradient>, InvariantViolation> {
    let mut layer_index: BTreeMap<String, usize> = BTreeMap::new();
    for (idx, layer) in hierarchy.layers.iter().enumerate() {
        for goal in layer {
            layer_index.insert(goal.clone(), idx);
        }
    }

    let mut interference_by_region: BTreeMap<String, i64> = BTreeMap::new();
    for c in &arbitrated.conflict_gradients {
        interference_by_region.insert(c.region_id.clone(), c.interference);
    }

    let goals: Vec<String> = arbitrated
        .goal_set
        .goals
        .iter()
        .map(|g| g.region_id.clone())
        .collect();

    let mut gradients: Vec<MetaPreferenceGradient> = Vec::new();
    for source in &goals {
        for target in &goals {
            if source == target {
                continue;
            }

            let source_w = arbitrated
                .goal_set
                .priority_weights
                .get(source)
                .copied()
                .unwrap_or(0);
            let target_w = arbitrated
                .goal_set
                .priority_weights
                .get(target)
                .copied()
                .unwrap_or(0);
            let influence = source_w - target_w;

            let source_layer = layer_index.get(source).copied().unwrap_or(usize::MAX);
            let target_layer = layer_index.get(target).copied().unwrap_or(usize::MAX);
            let coherence_delta = if source_layer < target_layer {
                120
            } else if source_layer == target_layer {
                30
            } else {
                -60
            };

            let conflict_delta = interference_by_region.get(target).copied().unwrap_or(0) / 2;
            let net_meta_pull = influence + coherence_delta - conflict_delta;

            gradients.push(MetaPreferenceGradient {
                source_goal: source.clone(),
                target_goal: target.clone(),
                influence,
                coherence_delta,
                conflict_delta,
                net_meta_pull,
            });
        }
    }

    gradients.sort_by(|a, b| {
        b.net_meta_pull
            .cmp(&a.net_meta_pull)
            .then_with(|| a.source_goal.cmp(&b.source_goal))
            .then_with(|| a.target_goal.cmp(&b.target_goal))
    });

    Ok(gradients)
}

pub fn compute_self_coherence_metric(
    arbitrated: &ArbitratedIntentField,
    hierarchy: &GoalHierarchy,
    meta_gradients: &[MetaPreferenceGradient],
    recent_meta_hashes: &[String],
) -> Result<SelfCoherenceMetric, InvariantViolation> {
    let hierarchy_depth = hierarchy.layers.len() as i64;
    let hierarchy_coherence = (1000 - (hierarchy_depth.saturating_sub(1) * 120)).clamp(0, 1000);

    let conflict_load = arbitrated
        .conflict_gradients
        .iter()
        .map(|g| g.interference)
        .sum::<i64>()
        / (arbitrated.conflict_gradients.len().max(1) as i64);

    let revision_pressure = meta_gradients
        .iter()
        .filter(|g| g.net_meta_pull < 0)
        .count() as i64
        * 40;

    let temporal_stability = if recent_meta_hashes.is_empty() {
        800
    } else {
        let unique: BTreeSet<String> = recent_meta_hashes.iter().cloned().collect();
        let variability = (unique.len() as i64 - 1).max(0) * 100;
        (1000 - variability).clamp(0, 1000)
    };

    let self_consistency = ((hierarchy_coherence
        + (1000 - conflict_load).clamp(0, 1000)
        + (1000 - revision_pressure).clamp(0, 1000)
        + temporal_stability)
        / 4)
        .clamp(0, 1000);

    Ok(SelfCoherenceMetric {
        hierarchy_coherence,
        conflict_load,
        revision_pressure,
        temporal_stability,
        self_consistency,
    })
}

pub fn build_meta_intent_field(
    arbitrated: &ArbitratedIntentField,
    recent_meta_hashes: &[String],
) -> Result<MetaIntentField, InvariantViolation> {
    let hierarchy = build_goal_hierarchy(arbitrated)?;
    let meta_preference_gradients = compute_meta_preference_gradients(arbitrated, &hierarchy)?;
    let self_coherence = compute_self_coherence_metric(
        arbitrated,
        &hierarchy,
        &meta_preference_gradients,
        recent_meta_hashes,
    )?;

    let mut revision_candidates: Vec<String> = meta_preference_gradients
        .iter()
        .filter(|g| g.net_meta_pull < 0)
        .map(|g| g.target_goal.clone())
        .collect();
    revision_candidates.sort();
    revision_candidates.dedup();

    let canonical_hash = hash_json(&(
        &hierarchy.canonical_hash,
        &meta_preference_gradients,
        &self_coherence,
        &revision_candidates,
        &recent_meta_hashes,
    ))?;

    Ok(MetaIntentField {
        hierarchy,
        meta_preference_gradients,
        self_coherence,
        revision_candidates,
        canonical_hash,
    })
}

pub fn resolve_meta_intent_trajectory(
    arbitrated: &ArbitratedIntentField,
    potential_field: &CognitivePotentialField,
    current_region: &str,
    recent_meta_hashes: &[String],
) -> Result<MetaIntentTrajectory, InvariantViolation> {
    let base = resolve_trajectory(arbitrated, potential_field, current_region)?;
    let meta = build_meta_intent_field(arbitrated, recent_meta_hashes)?;

    let mut revised_goals = meta.revision_candidates.clone();

    // If self-consistency is low, force revision of deferred goals first.
    if meta.self_coherence.self_consistency < 600 {
        for d in &base.deferred_goals {
            if !revised_goals.contains(d) {
                revised_goals.push(d.clone());
            }
        }
    }
    revised_goals.sort();
    revised_goals.dedup();

    let mut selected_path = base.selected_path.clone();
    if selected_path.len() == 1 {
        if let Some(root) = meta.hierarchy.layers.first().and_then(|l| l.first()) {
            if root != &selected_path[0] {
                selected_path.push(root.clone());
            }
        }
    }

    let canonical_hash = hash_json(&(
        &selected_path,
        &revised_goals,
        &base.deferred_goals,
        &meta.self_coherence,
        &meta.canonical_hash,
    ))?;

    Ok(MetaIntentTrajectory {
        selected_path,
        revised_goals,
        deferred_goals: base.deferred_goals,
        coherence_metric: meta.self_coherence,
        canonical_hash,
    })
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

    #[test]
    fn mfc_registers_concept_anchors_when_stable() {
        let mut mfc = build_mfc();
        let report = mfc
            .run(MultiFrameConfig {
                anchor_energy_max: 800,
                anchor_min_persistence: 1,
                ..MultiFrameConfig::default()
            })
            .expect("run should succeed");

        assert!(!report.anchor_registry.anchors.is_empty());
        assert!(report
            .anchor_registry
            .anchors
            .iter()
            .all(|a| !a.canonical_hash.is_empty()));
    }
}
