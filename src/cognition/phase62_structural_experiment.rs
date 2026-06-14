use crate::cognition::constraint::SemanticConstraint;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase62ExperimentKind {
    AnchorClosureSpineV1,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Phase62StructuralConfig {
    pub enabled: bool,
    pub kind: Phase62ExperimentKind,
    pub max_bridge_constraints_per_subject: usize,
    pub bridge_weight: u8,
}

impl Default for Phase62StructuralConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            kind: Phase62ExperimentKind::AnchorClosureSpineV1,
            max_bridge_constraints_per_subject: 1,
            bridge_weight: 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase62StructuralReport {
    pub applied: bool,
    pub generated_constraints: usize,
    pub note: String,
}

/// Phase 6.2 structural scaffold.
///
/// This is intentionally opt-in and isolated from the default runtime path.
/// It provides a deterministic hook for future convergence/anchor structural work
/// without touching Phase 6.1 thresholds or gate definitions.
pub fn apply_phase62_structural_experiment(
    input_constraints: &[SemanticConstraint],
    config: Phase62StructuralConfig,
) -> (Vec<SemanticConstraint>, Phase62StructuralReport) {
    if !config.enabled {
        return (
            input_constraints.to_vec(),
            Phase62StructuralReport {
                applied: false,
                generated_constraints: 0,
                note: "phase62 scaffold disabled".to_string(),
            },
        );
    }

    match config.kind {
        Phase62ExperimentKind::AnchorClosureSpineV1 => {
            scaffold_anchor_closure_spine_v1(input_constraints, config)
        }
    }
}

fn scaffold_anchor_closure_spine_v1(
    input_constraints: &[SemanticConstraint],
    config: Phase62StructuralConfig,
) -> (Vec<SemanticConstraint>, Phase62StructuralReport) {
    let mut output = input_constraints.to_vec();
    let mut generated = 0usize;

    let subjects: BTreeSet<String> = input_constraints
        .iter()
        .map(|c| c.subject.clone())
        .collect();

    for subject in subjects {
        if generated >= config.max_bridge_constraints_per_subject.max(1) {
            break;
        }

        output.push(SemanticConstraint::assertion(
            &subject,
            "phase62/anchor_closure_spine_candidate",
            true,
            config.bridge_weight,
        ));
        generated += 1;
    }

    (
        output,
        Phase62StructuralReport {
            applied: true,
            generated_constraints: generated,
            note: "phase62 scaffold generated deterministic structural candidates".to_string(),
        },
    )
}
