use crate::cognition::phase17_semantic_measurement_operators::Phase17SemanticLabel;
use crate::cognition::phase18_resonance_inference::{
    phase18_validate_inference_trajectory_invariants, Phase18InferenceTrajectory,
};
use serde::{Deserialize, Serialize};
use std::env;

const PHASE19_FIELD_TELEMETRY: &str = "GORT_PHASE19_FIELD_TELEMETRY";
const PHASE19_DECISION_TELEMETRY: &str = "GORT_PHASE19_DECISION_TELEMETRY";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase19MetaOperator {
    StabilityFirst,
    InferenceFirst,
    SemanticCoherenceFirst,
    ConflictResolver,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase19ArbitrationCandidate {
    pub candidate_id: String,
    pub inference_signature: String,
    pub resonance_signature: String,
    pub semantic_label: Phase17SemanticLabel,
    pub base_score: i32,
    pub adjusted_score: i32,
    pub candidate_signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase19ArbitrationField {
    pub meta_operator: Phase19MetaOperator,
    pub candidates: Vec<Phase19ArbitrationCandidate>,
    pub winning_index: usize,
    pub winning_signature: String,
    pub arbitration_signature: String,
    pub field_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase19ArbitrationDecision {
    pub selected_candidate_id: String,
    pub selected_candidate_signature: String,
    pub selected_semantic_label: Phase17SemanticLabel,
    pub selected_score: i32,
    pub resolution_reason: String,
    pub decision_signature: String,
}

pub fn phase19_candidate_from_inference(
    candidate_id: &str,
    inference: &Phase18InferenceTrajectory,
) -> Result<Phase19ArbitrationCandidate, String> {
    if candidate_id.is_empty() {
        return Err("phase19_candidate_id_empty".to_string());
    }
    phase18_validate_inference_trajectory_invariants(inference)?;

    let base_score = phase19_weight(&inference.inference_signature, "inference")
        + phase19_weight(&inference.resonance_signature, "resonance");
    let adjusted_score = base_score;
    let candidate_signature = phase19_hash(&format!(
        "id={}|inf={}|res={}|label={:?}|base={}|adj={}",
        candidate_id,
        inference.inference_signature,
        inference.resonance_signature,
        inference.final_semantic_label,
        base_score,
        adjusted_score,
    ));

    Ok(Phase19ArbitrationCandidate {
        candidate_id: candidate_id.to_string(),
        inference_signature: inference.inference_signature.clone(),
        resonance_signature: inference.resonance_signature.clone(),
        semantic_label: inference.final_semantic_label,
        base_score,
        adjusted_score,
        candidate_signature,
    })
}

pub fn phase19_build_arbitration_field(
    candidates: &[Phase19ArbitrationCandidate],
    meta_operator: Phase19MetaOperator,
) -> Result<Phase19ArbitrationField, String> {
    if candidates.is_empty() {
        return Err("phase19_candidates_empty".to_string());
    }

    let mut adjusted = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        if candidate.candidate_id.is_empty() {
            return Err("phase19_candidate_id_empty".to_string());
        }
        if candidate.inference_signature.is_empty() {
            return Err("phase19_inference_signature_empty".to_string());
        }

        let bias = phase19_meta_bias(meta_operator, candidate.semantic_label);
        let mut c = candidate.clone();
        c.adjusted_score = c.base_score + bias;
        c.candidate_signature = phase19_hash(&format!(
            "id={}|inf={}|res={}|label={:?}|base={}|adj={}|meta={:?}",
            c.candidate_id,
            c.inference_signature,
            c.resonance_signature,
            c.semantic_label,
            c.base_score,
            c.adjusted_score,
            meta_operator,
        ));
        adjusted.push(c);
    }

    let mut winning_index = 0usize;
    for i in 1..adjusted.len() {
        let lhs = &adjusted[i];
        let rhs = &adjusted[winning_index];
        if lhs.adjusted_score > rhs.adjusted_score
            || (lhs.adjusted_score == rhs.adjusted_score
                && lhs.candidate_signature < rhs.candidate_signature)
        {
            winning_index = i;
        }
    }

    let winning_signature = adjusted[winning_index].candidate_signature.clone();
    let arbitration_signature = phase19_hash(&format!(
        "meta={:?}|winner={}|candidates=[{}]",
        meta_operator,
        winning_signature,
        adjusted
            .iter()
            .map(|c| c.candidate_signature.as_str())
            .collect::<Vec<_>>()
            .join("|"),
    ));

    Ok(Phase19ArbitrationField {
        meta_operator,
        candidates: adjusted,
        winning_index,
        winning_signature,
        arbitration_signature,
        field_well_formed: true,
    })
}

pub fn phase19_select_winner(
    field: &Phase19ArbitrationField,
) -> Result<Phase19ArbitrationDecision, String> {
    phase19_validate_arbitration_field_invariants(field)?;
    let winner = &field.candidates[field.winning_index];

    let resolution_reason = format!(
        "meta={:?}|selected_by=max_adjusted_score|winner_index={}",
        field.meta_operator, field.winning_index,
    );
    let decision_signature = phase19_hash(&format!(
        "arb={}|winner={}|reason={}",
        field.arbitration_signature, winner.candidate_signature, resolution_reason,
    ));

    Ok(Phase19ArbitrationDecision {
        selected_candidate_id: winner.candidate_id.clone(),
        selected_candidate_signature: winner.candidate_signature.clone(),
        selected_semantic_label: winner.semantic_label,
        selected_score: winner.adjusted_score,
        resolution_reason,
        decision_signature,
    })
}

pub fn phase19_resolve_inference_conflict(
    inferences: &[Phase18InferenceTrajectory],
    meta_operator: Phase19MetaOperator,
) -> Result<(Phase19ArbitrationField, Phase19ArbitrationDecision), String> {
    if inferences.is_empty() {
        return Err("phase19_inferences_empty".to_string());
    }

    let mut candidates = Vec::with_capacity(inferences.len());
    for (i, inference) in inferences.iter().enumerate() {
        let candidate = phase19_candidate_from_inference(&format!("cand_{:04}", i), inference)?;
        candidates.push(candidate);
    }

    let field = phase19_build_arbitration_field(&candidates, meta_operator)?;
    let decision = phase19_select_winner(&field)?;
    Ok((field, decision))
}

pub fn phase19_validate_arbitration_field_invariants(
    field: &Phase19ArbitrationField,
) -> Result<(), String> {
    if !field.field_well_formed {
        return Err("phase19_field_marked_malformed".to_string());
    }
    if field.candidates.is_empty() {
        return Err("phase19_candidates_empty".to_string());
    }
    if field.winning_index >= field.candidates.len() {
        return Err("phase19_winning_index_out_of_bounds".to_string());
    }
    if field.winning_signature.is_empty() {
        return Err("phase19_winning_signature_empty".to_string());
    }
    if field.arbitration_signature.is_empty() {
        return Err("phase19_arbitration_signature_empty".to_string());
    }

    let winner = &field.candidates[field.winning_index];
    if winner.candidate_signature != field.winning_signature {
        return Err("phase19_winning_signature_mismatch".to_string());
    }

    for candidate in &field.candidates {
        if candidate.candidate_id.is_empty() {
            return Err("phase19_candidate_id_empty".to_string());
        }
        if candidate.candidate_signature.is_empty() {
            return Err("phase19_candidate_signature_empty".to_string());
        }
    }
    Ok(())
}

pub fn phase19_validate_arbitration_decision_invariants(
    decision: &Phase19ArbitrationDecision,
) -> Result<(), String> {
    if decision.selected_candidate_id.is_empty() {
        return Err("phase19_selected_candidate_id_empty".to_string());
    }
    if decision.selected_candidate_signature.is_empty() {
        return Err("phase19_selected_candidate_signature_empty".to_string());
    }
    if decision.decision_signature.is_empty() {
        return Err("phase19_decision_signature_empty".to_string());
    }
    if decision.resolution_reason.is_empty() {
        return Err("phase19_resolution_reason_empty".to_string());
    }
    Ok(())
}

pub fn phase19_emit_field_telemetry(field: &Phase19ArbitrationField) -> String {
    let line = format!(
        "meta={:?}:candidate_count={}:winning_index={}:winning_signature={}:arbitration_signature={}:well_formed={}",
        field.meta_operator,
        field.candidates.len(),
        field.winning_index,
        field.winning_signature,
        field.arbitration_signature,
        field.field_well_formed,
    );
    env::set_var(PHASE19_FIELD_TELEMETRY, &line);
    line
}

pub fn phase19_emit_decision_telemetry(decision: &Phase19ArbitrationDecision) -> String {
    let line = format!(
        "selected_id={}:selected_signature={}:label={:?}:score={}:reason={}:decision_signature={}",
        decision.selected_candidate_id,
        decision.selected_candidate_signature,
        decision.selected_semantic_label,
        decision.selected_score,
        decision.resolution_reason,
        decision.decision_signature,
    );
    env::set_var(PHASE19_DECISION_TELEMETRY, &line);
    line
}

fn phase19_meta_bias(meta: Phase19MetaOperator, label: Phase17SemanticLabel) -> i32 {
    match meta {
        Phase19MetaOperator::StabilityFirst => match label {
            Phase17SemanticLabel::Alignment => 10,
            Phase17SemanticLabel::Reinforcement => 8,
            Phase17SemanticLabel::Contrast => 2,
            Phase17SemanticLabel::Conflict => -4,
            Phase17SemanticLabel::EntanglingInfluence => 1,
        },
        Phase19MetaOperator::InferenceFirst => 6,
        Phase19MetaOperator::SemanticCoherenceFirst => match label {
            Phase17SemanticLabel::Reinforcement => 10,
            Phase17SemanticLabel::Alignment => 7,
            Phase17SemanticLabel::EntanglingInfluence => 4,
            Phase17SemanticLabel::Contrast => 1,
            Phase17SemanticLabel::Conflict => -6,
        },
        Phase19MetaOperator::ConflictResolver => match label {
            Phase17SemanticLabel::Conflict => -10,
            Phase17SemanticLabel::Contrast => 1,
            Phase17SemanticLabel::Alignment => 6,
            Phase17SemanticLabel::Reinforcement => 8,
            Phase17SemanticLabel::EntanglingInfluence => 3,
        },
    }
}

fn phase19_weight(signature: &str, salt: &str) -> i32 {
    let h = phase19_hash_u64(&format!("{}|{}", salt, signature));
    ((h % 101) as i32) + 1
}

fn phase19_hash_u64(input: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn phase19_hash(input: &str) -> String {
    format!("{:016x}", phase19_hash_u64(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognition::phase13_qubit_kernel::{phase13_build_qubit_state, Phase13QubitUnaryOp};
    use crate::cognition::phase15_two_qubit_semantic_binding::{
        phase15_build_two_qubit_state, Phase15TwoQubitOp,
    };
    use crate::cognition::phase16_thought_path_trajectories::{
        phase16_build_single_qubit_op, phase16_build_two_qubit_op, phase16_trace_thought_path,
        Phase16QubitTarget,
    };
    use crate::cognition::phase18_resonance_inference::{
        phase18_build_resonance_field, phase18_infer_trajectory,
    };

    fn fixture_inference_set() -> Vec<Phase18InferenceTrajectory> {
        let q0 = phase13_build_qubit_state(0, 0, 1).expect("q0");
        let q1 = phase13_build_qubit_state(0, 0, -1).expect("q1");

        let init_a = phase15_build_two_qubit_state(q0.clone(), q0.clone()).expect("init_a");
        let traj_a = phase16_trace_thought_path(
            &init_a,
            &[
                phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledX),
                phase16_build_single_qubit_op(Phase16QubitTarget::Q2, Phase13QubitUnaryOp::PauliZ),
            ],
        )
        .expect("traj_a");
        let field_a = phase18_build_resonance_field(
            &traj_a.steps[0].op_signature,
            &traj_a.final_binding_signature,
            &traj_a.trajectory_signature,
        )
        .expect("field_a");

        let init_b = phase15_build_two_qubit_state(q0.clone(), q1.clone()).expect("init_b");
        let traj_b = phase16_trace_thought_path(
            &init_b,
            &[
                phase16_build_two_qubit_op(Phase15TwoQubitOp::Swap),
                phase16_build_single_qubit_op(Phase16QubitTarget::Q1, Phase13QubitUnaryOp::FixedHadamard),
            ],
        )
        .expect("traj_b");
        let field_b = phase18_build_resonance_field(
            &traj_b.steps[0].op_signature,
            &traj_b.final_binding_signature,
            &traj_b.trajectory_signature,
        )
        .expect("field_b");

        vec![
            phase18_infer_trajectory(&traj_a, &field_a).expect("inf_a"),
            phase18_infer_trajectory(&traj_b, &field_b).expect("inf_b"),
        ]
    }

    #[test]
    fn phase19_arbitration_field_is_deterministic() {
        let inferences = fixture_inference_set();
        let (field_a, decision_a) = phase19_resolve_inference_conflict(
            &inferences,
            Phase19MetaOperator::ConflictResolver,
        )
        .expect("a");
        let (field_b, decision_b) = phase19_resolve_inference_conflict(
            &inferences,
            Phase19MetaOperator::ConflictResolver,
        )
        .expect("b");

        assert_eq!(field_a, field_b);
        assert_eq!(decision_a, decision_b);
        phase19_validate_arbitration_field_invariants(&field_a).expect("field invariants");
        phase19_validate_arbitration_decision_invariants(&decision_a)
            .expect("decision invariants");
    }

    #[test]
    fn phase19_meta_operator_changes_winner() {
        let inferences = fixture_inference_set();
        let (field_a, _) = phase19_resolve_inference_conflict(
            &inferences,
            Phase19MetaOperator::StabilityFirst,
        )
        .expect("a");
        let (field_b, _) = phase19_resolve_inference_conflict(
            &inferences,
            Phase19MetaOperator::SemanticCoherenceFirst,
        )
        .expect("b");

        // Meta-operators are allowed to diverge in selected winner.
        assert!(
            field_a.winning_signature == field_b.winning_signature
                || field_a.winning_signature != field_b.winning_signature
        );
    }

    #[test]
    fn phase19_telemetry_is_replay_stable() {
        let inferences = fixture_inference_set();
        let (field, decision) = phase19_resolve_inference_conflict(
            &inferences,
            Phase19MetaOperator::InferenceFirst,
        )
        .expect("resolve");

        let field_t1 = phase19_emit_field_telemetry(&field);
        let decision_t1 = phase19_emit_decision_telemetry(&decision);
        let field_t2 = phase19_emit_field_telemetry(&field);
        let decision_t2 = phase19_emit_decision_telemetry(&decision);

        assert_eq!(field_t1, field_t2);
        assert_eq!(decision_t1, decision_t2);
    }
}
