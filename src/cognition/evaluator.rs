/// Semantic constraint evaluation engine.
/// Implements deterministic constraint satisfaction and conflict detection.

use crate::cognition::constraint::SemanticConstraint;
use crate::cognition::node::SemanticNode;
use crate::geom::field::{FieldPoint, SemanticField};
use crate::geom::invariants::{ConstraintEvaluator, InvariantViolation};
use crate::geom::mode::{ArithmeticMode, ResonanceMode, ResonanceTransform};
use crate::geom::space::Coordinate3;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintStatus {
    Satisfied,
    Violated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SenseInterferenceScore {
    pub concept: String,
    pub support: i64,
    pub interference: i64,
    pub score: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisambiguationDecision {
    pub subject: String,
    pub selected_concept: String,
    pub score_gap: i64,
    pub unresolved: bool,
    pub candidates: Vec<SenseInterferenceScore>,
}

#[derive(Debug, Default)]
pub struct ConstraintEvalEngine;

impl ConstraintEvalEngine {
    pub fn new() -> Self {
        Self
    }

    /// Transform constraints into semantic nodes.
    pub fn constraints_to_nodes(&self, constraints: &[SemanticConstraint]) -> Vec<SemanticNode> {
        constraints
            .iter()
            .map(|c| {
                let concept = format!("{}:{}", c.subject, c.predicate);
                SemanticNode::new(concept, c.affirmed, c.weight)
            })
            .collect()
    }

    /// Transform nodes into a geometric semantic field with deterministic coordinate assignment.
    pub fn project_nodes_to_field(&self, nodes: &[SemanticNode]) -> SemanticField {
        let mut field = SemanticField::new();
        for (idx, node) in nodes.iter().enumerate() {
            let x = idx as i64;
            let y = if node.polarity { 1 } else { -1 };
            let z = node.confidence as i64;

            field.upsert_concept(
                node.concept.clone(),
                FieldPoint {
                    position: Coordinate3::new(x, y, z),
                    intensity: if node.polarity {
                        node.confidence as i64
                    } else {
                        -(node.confidence as i64)
                    },
                },
            );
        }
        field
    }

    /// First real cognitive transform: apply resonance by aggregate confidence.
    pub fn apply_resonance_transform(&self, field: &mut SemanticField, nodes: &[SemanticNode]) {
        self.apply_resonance_transform_with_mode(field, nodes, ArithmeticMode::Exact);
    }

    pub fn apply_resonance_transform_with_mode(
        &self,
        field: &mut SemanticField,
        nodes: &[SemanticNode],
        arithmetic: ArithmeticMode,
    ) {
        let signed_energy: i64 = nodes
            .iter()
            .map(|n| if n.polarity { n.confidence as i64 } else { -(n.confidence as i64) })
            .sum();

        let (mode, magnitude) = if signed_energy > 0 {
            (ResonanceMode::Amplify, (signed_energy / 20).max(1))
        } else if signed_energy < 0 {
            (ResonanceMode::Dampen, ((-signed_energy) / 20).max(1))
        } else {
            (ResonanceMode::Balance, 0)
        };

        ResonanceTransform::new(mode, magnitude, arithmetic).apply(field);
    }

    /// Phase 2 disambiguation: resolve competing senses for one subject by field interference.
    ///
    /// Scores are deterministic fixed-point integers:
    /// - `support = intensity * 1000`
    /// - `interference = Σ(((self.intensity - other.intensity) * 100) / (distance + 1))`
    /// - `score = support + interference`
    ///
    /// If scores tie, lexicographic concept order is used for stable selection.
    pub fn disambiguate_subject_senses(
        &self,
        field: &SemanticField,
        subject: &str,
    ) -> Option<DisambiguationDecision> {
        self.disambiguate_subject_senses_with_margin(field, subject, 0)
    }

    /// Deterministic thresholding for ambiguity handling.
    ///
    /// A decision is marked unresolved when the top-2 score gap is <= ambiguity_margin.
    pub fn disambiguate_subject_senses_with_margin(
        &self,
        field: &SemanticField,
        subject: &str,
        ambiguity_margin: i64,
    ) -> Option<DisambiguationDecision> {
        let prefix = format!("{}:", subject);
        let candidates: Vec<(String, FieldPoint)> = field
            .ordered_concepts()
            .filter(|(concept, _)| concept.starts_with(&prefix))
            .map(|(concept, point)| (concept.clone(), point.clone()))
            .collect();

        if candidates.len() < 2 {
            return None;
        }

        let mut scored: Vec<SenseInterferenceScore> = Vec::with_capacity(candidates.len());
        for (concept, point) in &candidates {
            let support = point.intensity * 1000;
            let mut interference = 0;

            for (other_concept, other_point) in &candidates {
                if concept == other_concept {
                    continue;
                }

                let distance = point.position.manhattan_distance(&other_point.position) + 1;
                interference += ((point.intensity - other_point.intensity) * 100) / distance;
            }

            scored.push(SenseInterferenceScore {
                concept: concept.clone(),
                support,
                interference,
                score: support + interference,
            });
        }

        scored.sort_by(|a, b| b.score.cmp(&a.score).then(a.concept.cmp(&b.concept)));

        let score_gap = if scored.len() >= 2 {
            scored[0].score - scored[1].score
        } else {
            i64::MAX
        };
        let unresolved = score_gap <= ambiguity_margin.max(0);

        Some(DisambiguationDecision {
            subject: subject.to_string(),
            selected_concept: scored[0].concept.clone(),
            score_gap,
            unresolved,
            candidates: scored,
        })
    }
}

impl ConstraintEvaluator for ConstraintEvalEngine {
    type Constraint = SemanticConstraint;
    type EvaluationResult = ConstraintStatus;

    fn evaluate(&self, constraint: &Self::Constraint) -> Result<Self::EvaluationResult, InvariantViolation> {
        if constraint.subject.trim().is_empty() || constraint.predicate.trim().is_empty() {
            return Err(InvariantViolation::Consistency {
                message: "subject/predicate cannot be empty".to_string(),
                contradicting_terms: vec![constraint.subject.clone(), constraint.predicate.clone()],
            });
        }

        if constraint.weight == 0 {
            return Ok(ConstraintStatus::Violated);
        }

        Ok(ConstraintStatus::Satisfied)
    }

    fn detect_conflicts(&self, constraints: &[Self::Constraint]) -> Vec<(usize, usize, String)> {
        let mut seen: BTreeMap<(String, String, Option<String>), (usize, bool)> = BTreeMap::new();
        let mut conflicts = Vec::new();

        for (idx, c) in constraints.iter().enumerate() {
            let key = c.key();
            if let Some((prior_idx, prior_polarity)) = seen.get(&key) {
                if *prior_polarity != c.affirmed {
                    conflicts.push((
                        *prior_idx,
                        idx,
                        format!(
                            "Conflict on {}:{} polarity mismatch",
                            c.subject, c.predicate
                        ),
                    ));
                }
            } else {
                seen.insert(key, (idx, c.affirmed));
            }
        }

        conflicts
    }

    fn resolve_contradictions(
        &self,
        constraints: &[Self::Constraint],
        audit_trail: &mut Vec<String>,
    ) -> Result<Vec<Self::Constraint>, InvariantViolation> {
        let conflicts = self.detect_conflicts(constraints);
        if !conflicts.is_empty() {
            audit_trail.push(format!("{} conflict(s) detected", conflicts.len()));
            return Err(InvariantViolation::Consistency {
                message: "unable to auto-resolve contradictory constraints".to_string(),
                contradicting_terms: conflicts.into_iter().map(|(_, _, msg)| msg).collect(),
            });
        }

        let mut resolved = constraints.to_vec();
        resolved.sort_by(|a, b| {
            a.subject
                .cmp(&b.subject)
                .then(a.predicate.cmp(&b.predicate))
                .then(a.object.cmp(&b.object))
                .then(a.affirmed.cmp(&b.affirmed))
                .then(a.weight.cmp(&b.weight))
        });
        audit_trail.push(format!("resolved {} constraints", resolved.len()));
        Ok(resolved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognition::constraint::SemanticConstraint;

    #[test]
    fn detects_polarity_conflicts() {
        let engine = ConstraintEvalEngine::new();
        let constraints = vec![
            SemanticConstraint::assertion("light", "wave", true, 100),
            SemanticConstraint::assertion("light", "wave", false, 90),
        ];
        let conflicts = engine.detect_conflicts(&constraints);
        assert_eq!(conflicts.len(), 1);
    }

    #[test]
    fn cognitive_transforms_generate_field() {
        let engine = ConstraintEvalEngine::new();
        let constraints = vec![SemanticConstraint::assertion("light", "wave", true, 90)];
        let nodes = engine.constraints_to_nodes(&constraints);
        let mut field = engine.project_nodes_to_field(&nodes);
        engine.apply_resonance_transform(&mut field, &nodes);
        assert!(field.concept_count() > 0);
    }

    #[test]
    fn bounded_mode_produces_quantized_field() {
        let engine = ConstraintEvalEngine::new();
        let constraints = vec![SemanticConstraint::assertion("light", "wave", true, 91)];
        let nodes = engine.constraints_to_nodes(&constraints);
        let mut field = engine.project_nodes_to_field(&nodes);

        engine.apply_resonance_transform_with_mode(
            &mut field,
            &nodes,
            ArithmeticMode::BoundedApproximate { max_error: 3 },
        );

        let intensity = field.concept("light:wave").map(|p| p.intensity).unwrap_or_default();
        assert_eq!(intensity % 4, 0);
    }

    #[test]
    fn disambiguates_multi_sense_subject_via_interference() {
        let engine = ConstraintEvalEngine::new();
        let constraints = vec![
            SemanticConstraint::assertion("light", "wave", true, 92),
            SemanticConstraint::assertion("light", "particle", true, 88),
            SemanticConstraint::assertion("light", "illusion", false, 60),
        ];

        let nodes = engine.constraints_to_nodes(&constraints);
        let mut field = engine.project_nodes_to_field(&nodes);
        engine.apply_resonance_transform(&mut field, &nodes);

        let decision = engine
            .disambiguate_subject_senses(&field, "light")
            .expect("expected multi-sense candidates");

        assert_eq!(decision.subject, "light");
        assert_eq!(decision.candidates.len(), 3);
        assert_eq!(decision.selected_concept, "light:wave");
        assert!(!decision.unresolved);
        assert!(decision.score_gap > 0);

        for pair in decision.candidates.windows(2) {
            assert!(pair[0].score >= pair[1].score);
        }
    }

    #[test]
    fn disambiguation_tie_breaker_is_lexicographic() {
        let engine = ConstraintEvalEngine::new();
        let mut field = SemanticField::new();
        field.upsert_concept(
            "light:alpha",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 10,
            },
        );
        field.upsert_concept(
            "light:beta",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 10,
            },
        );

        let decision = engine
            .disambiguate_subject_senses(&field, "light")
            .expect("expected tied candidates");

        assert_eq!(decision.selected_concept, "light:alpha");
        assert_eq!(decision.candidates[0].score, decision.candidates[1].score);
        assert_eq!(decision.score_gap, 0);
        assert!(decision.unresolved);
    }

    #[test]
    fn ambiguity_margin_marks_near_ties_unresolved() {
        let engine = ConstraintEvalEngine::new();
        let mut field = SemanticField::new();
        field.upsert_concept(
            "light:wave",
            FieldPoint {
                position: Coordinate3::new(0, 0, 0),
                intensity: 100,
            },
        );
        field.upsert_concept(
            "light:particle",
            FieldPoint {
                position: Coordinate3::new(0, 1, 0),
                intensity: 99,
            },
        );

        let strict = engine
            .disambiguate_subject_senses_with_margin(&field, "light", 0)
            .expect("expected candidates");
        assert!(!strict.unresolved);

        let tolerant = engine
            .disambiguate_subject_senses_with_margin(&field, "light", 5000)
            .expect("expected candidates");
        assert!(tolerant.unresolved);
    }
}
