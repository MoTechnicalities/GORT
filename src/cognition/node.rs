/// Semantic reasoning nodes.
/// Represents individual reasoning entities within the geometric cognition space.

use crate::geom::invariants::{ClosureStatus, ClosureTransition, GeometricState, InvariantViolation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticNode {
    pub id: String,
    pub concept: String,
    pub polarity: bool,
    pub confidence: u8,
}

impl SemanticNode {
    pub fn new(concept: impl Into<String>, polarity: bool, confidence: u8) -> Self {
        let concept = concept.into();
        let id = deterministic_id(&concept, polarity, confidence);
        Self {
            id,
            concept,
            polarity,
            confidence,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveFrame {
    pub topic: String,
    pub nodes: Vec<SemanticNode>,
    pub unresolved_subjects: Vec<String>,
    pub status: ClosureStatus,
    pub audit: Vec<String>,
}

impl CognitiveFrame {
    pub fn new(topic: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            nodes: Vec::new(),
            unresolved_subjects: Vec::new(),
            status: ClosureStatus::Open,
            audit: vec!["frame initialized".to_string()],
        }
    }

    pub fn add_node(&mut self, node: SemanticNode) {
        self.audit.push(format!("added node {}", node.id));
        self.nodes.push(node);
    }

    pub fn mark_unresolved_subject(&mut self, subject: impl Into<String>) {
        let subject = subject.into();
        if self.unresolved_subjects.iter().any(|s| s == &subject) {
            return;
        }

        self.audit
            .push(format!("disambiguation unresolved for subject {}", subject));
        self.unresolved_subjects.push(subject);
        self.unresolved_subjects.sort();
    }
}

impl GeometricState for CognitiveFrame {
    fn frame_id(&self) -> String {
        let mut h = Sha256::new();
        h.update(self.topic.as_bytes());
        for node in &self.nodes {
            h.update(node.id.as_bytes());
        }
        for subject in &self.unresolved_subjects {
            h.update(subject.as_bytes());
        }
        format!("{:x}", h.finalize())
    }

    fn closure_status(&self) -> ClosureStatus {
        self.status
    }

    fn validate(&self) -> Result<(), InvariantViolation> {
        if self.audit.is_empty() {
            return Err(InvariantViolation::Auditability {
                message: "audit trail cannot be empty".to_string(),
                missing_audit_step: "frame initialized".to_string(),
            });
        }
        Ok(())
    }

    fn attempt_closure(&self) -> (Self, Option<ClosureTransition>) {
        let mut next = self.clone();
        if next.nodes.is_empty() {
            next.status = ClosureStatus::Partial;
            next.audit.push("closure attempted: no nodes present".to_string());
            let transition = ClosureTransition {
                from_status: self.status,
                to_status: ClosureStatus::Partial,
                resolved_by_last_user_turn: false,
                reasoning_summary: "missing semantic anchors".to_string(),
            };
            return (next, Some(transition));
        }

        if !next.unresolved_subjects.is_empty() {
            next.status = ClosureStatus::Partial;
            next.audit.push(format!(
                "closure attempted: unresolved senses for {}",
                next.unresolved_subjects.join(",")
            ));
            let transition = ClosureTransition {
                from_status: self.status,
                to_status: ClosureStatus::Partial,
                resolved_by_last_user_turn: false,
                reasoning_summary: format!(
                    "unresolved multi-sense disambiguation: {}",
                    next.unresolved_subjects.join(",")
                ),
            };
            return (next, Some(transition));
        }

        next.status = ClosureStatus::Closed;
        next.audit.push("closure attempted: closed".to_string());
        let transition = ClosureTransition {
            from_status: self.status,
            to_status: ClosureStatus::Closed,
            resolved_by_last_user_turn: true,
            reasoning_summary: "all nodes resolved".to_string(),
        };
        (next, Some(transition))
    }

    fn record_derivation(&mut self, step: String) {
        self.audit.push(step);
    }

    fn audit_trail(&self) -> Vec<String> {
        self.audit.clone()
    }
}

fn deterministic_id(concept: &str, polarity: bool, confidence: u8) -> String {
    let mut h = Sha256::new();
    h.update(concept.as_bytes());
    h.update([polarity as u8]);
    h.update([confidence]);
    format!("{:x}", h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_id_is_deterministic() {
        let a = SemanticNode::new("light", true, 90);
        let b = SemanticNode::new("light", true, 90);
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn frame_closure_without_nodes_is_partial() {
        let frame = CognitiveFrame::new("test");
        let (next, _) = frame.attempt_closure();
        assert_eq!(next.status, ClosureStatus::Partial);
    }

    #[test]
    fn frame_with_unresolved_subject_stays_partial() {
        let mut frame = CognitiveFrame::new("light");
        frame.add_node(SemanticNode::new("light:wave", true, 90));
        frame.mark_unresolved_subject("light");

        let (next, transition) = frame.attempt_closure();
        assert_eq!(next.status, ClosureStatus::Partial);

        let transition = transition.expect("expected transition");
        assert!(transition.reasoning_summary.contains("unresolved multi-sense disambiguation"));
    }
}
