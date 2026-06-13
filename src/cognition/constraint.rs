/// Semantic constraint definitions.
/// Defines what constraints can be applied to reasoning operations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintKind {
    Assertion,
    Exclusion,
    Link,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticConstraint {
    pub subject: String,
    pub predicate: String,
    pub object: Option<String>,
    pub affirmed: bool,
    pub kind: ConstraintKind,
    pub weight: u8,
}

impl SemanticConstraint {
    pub fn assertion(subject: impl Into<String>, predicate: impl Into<String>, affirmed: bool, weight: u8) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: None,
            affirmed,
            kind: ConstraintKind::Assertion,
            weight,
        }
    }

    pub fn key(&self) -> (String, String, Option<String>) {
        (
            self.subject.clone(),
            self.predicate.clone(),
            self.object.clone(),
        )
    }
}
