use crate::cognition::phase17_semantic_measurement_operators::Phase17SemanticLabel;
use crate::cognition::phase18_resonance_inference::{
    phase18_validate_inference_trajectory_invariants, Phase18InferenceTrajectory,
};
use crate::cognition::phase19_arbitration_operators::{
    phase19_validate_arbitration_decision_invariants, Phase19ArbitrationDecision,
};
use serde::{Deserialize, Serialize};
use std::env;

const PHASE20_CORRECTION_TELEMETRY: &str = "GORT_PHASE20_CORRECTION_TELEMETRY";
const PHASE20_STABILIZATION_TELEMETRY: &str = "GORT_PHASE20_STABILIZATION_TELEMETRY";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase20CorrectionOperator {
    RestoreInvariant,
    RepairSemanticTrajectory,
    StabilizeInferenceField,
    DampDrift,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase20DriftReport {
    pub drift_detected: bool,
    pub invariant_violation_count: i32,
    pub semantic_divergence_score: i32,
    pub inference_instability_score: i32,
    pub drift_signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase20CorrectionPlan {
    pub correction_operator: Phase20CorrectionOperator,
    pub target_candidate_signature: String,
    pub target_inference_signature: String,
    pub target_resonance_signature: String,
    pub correction_weight: i32,
    pub correction_signature: String,
    pub plan_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase20StabilizedInference {
    pub original_inference_signature: String,
    pub stabilized_inference_signature: String,
    pub stabilized_semantic_label: Phase17SemanticLabel,
    pub stabilization_delta: i32,
    pub stabilization_signature: String,
    pub telemetry_digest: String,
    pub stabilized_well_formed: bool,
}

pub fn phase20_detect_drift(
    inference: &Phase18InferenceTrajectory,
) -> Result<Phase20DriftReport, String> {
    phase18_validate_inference_trajectory_invariants(inference)?;

    let instability_seed = phase20_hash_u64(&format!(
        "inference={}|telemetry={}|label={:?}",
        inference.inference_signature, inference.telemetry_digest, inference.final_semantic_label,
    ));
    let inference_instability_score = (instability_seed % 7) as i32;
    let semantic_divergence_score = match inference.final_semantic_label {
        Phase17SemanticLabel::Conflict => 3,
        Phase17SemanticLabel::Contrast => 2,
        Phase17SemanticLabel::EntanglingInfluence => 2,
        Phase17SemanticLabel::Alignment => 0,
        Phase17SemanticLabel::Reinforcement => 0,
    };
    let invariant_violation_count = if !inference.trajectory_well_formed { 1 } else { 0 };
    let drift_detected = (invariant_violation_count + semantic_divergence_score + inference_instability_score) > 4;

    let drift_signature = phase20_hash(&format!(
        "drift={}|inv={}|sem={}|inst={}|inf={}",
        drift_detected,
        invariant_violation_count,
        semantic_divergence_score,
        inference_instability_score,
        inference.inference_signature,
    ));

    Ok(Phase20DriftReport {
        drift_detected,
        invariant_violation_count,
        semantic_divergence_score,
        inference_instability_score,
        drift_signature,
    })
}

pub fn phase20_build_correction_plan(
    inference: &Phase18InferenceTrajectory,
    decision: &Phase19ArbitrationDecision,
) -> Result<Phase20CorrectionPlan, String> {
    phase18_validate_inference_trajectory_invariants(inference)?;
    phase19_validate_arbitration_decision_invariants(decision)?;

    let drift = phase20_detect_drift(inference)?;
    let correction_operator = if drift.invariant_violation_count > 0 {
        Phase20CorrectionOperator::RestoreInvariant
    } else if drift.semantic_divergence_score > 1 {
        Phase20CorrectionOperator::RepairSemanticTrajectory
    } else if drift.inference_instability_score > 2 {
        Phase20CorrectionOperator::StabilizeInferenceField
    } else {
        Phase20CorrectionOperator::DampDrift
    };

    let correction_weight = drift.invariant_violation_count * 5
        + drift.semantic_divergence_score * 3
        + drift.inference_instability_score
        + if drift.drift_detected { 5 } else { 1 };

    let correction_signature = phase20_hash(&format!(
        "op={:?}|cand={}|inf={}|res={}|w={}|drift={}",
        correction_operator,
        decision.selected_candidate_signature,
        inference.inference_signature,
        inference.resonance_signature,
        correction_weight,
        drift.drift_signature,
    ));

    Ok(Phase20CorrectionPlan {
        correction_operator,
        target_candidate_signature: decision.selected_candidate_signature.clone(),
        target_inference_signature: inference.inference_signature.clone(),
        target_resonance_signature: inference.resonance_signature.clone(),
        correction_weight,
        correction_signature,
        plan_well_formed: true,
    })
}

pub fn phase20_apply_self_correction(
    inference: &Phase18InferenceTrajectory,
    plan: &Phase20CorrectionPlan,
) -> Result<Phase20StabilizedInference, String> {
    phase18_validate_inference_trajectory_invariants(inference)?;
    phase20_validate_correction_plan_invariants(plan)?;

    let stabilization_delta = plan.correction_weight + (inference.inference_steps.len() as i32);
    let stabilized_semantic_label = match plan.correction_operator {
        Phase20CorrectionOperator::RestoreInvariant => Phase17SemanticLabel::Alignment,
        Phase20CorrectionOperator::RepairSemanticTrajectory => Phase17SemanticLabel::Reinforcement,
        Phase20CorrectionOperator::StabilizeInferenceField => inference.final_semantic_label,
        Phase20CorrectionOperator::DampDrift => inference.final_semantic_label,
    };

    let stabilized_inference_signature = phase20_hash(&format!(
        "orig={}|plan={}|delta={}|label={:?}",
        inference.inference_signature,
        plan.correction_signature,
        stabilization_delta,
        stabilized_semantic_label,
    ));
    let stabilization_signature = phase20_hash(&format!(
        "res={}|orig={}|stab={}|op={:?}",
        inference.resonance_signature,
        inference.inference_signature,
        stabilized_inference_signature,
        plan.correction_operator,
    ));
    let telemetry_digest = phase20_hash(&format!(
        "stabilization_signature={}|label={:?}|delta={}",
        stabilization_signature, stabilized_semantic_label, stabilization_delta,
    ));

    Ok(Phase20StabilizedInference {
        original_inference_signature: inference.inference_signature.clone(),
        stabilized_inference_signature,
        stabilized_semantic_label,
        stabilization_delta,
        stabilization_signature,
        telemetry_digest,
        stabilized_well_formed: true,
    })
}

pub fn phase20_validate_correction_plan_invariants(
    plan: &Phase20CorrectionPlan,
) -> Result<(), String> {
    if !plan.plan_well_formed {
        return Err("phase20_plan_marked_malformed".to_string());
    }
    if plan.target_candidate_signature.is_empty() {
        return Err("phase20_target_candidate_signature_empty".to_string());
    }
    if plan.target_inference_signature.is_empty() {
        return Err("phase20_target_inference_signature_empty".to_string());
    }
    if plan.target_resonance_signature.is_empty() {
        return Err("phase20_target_resonance_signature_empty".to_string());
    }
    if plan.correction_signature.is_empty() {
        return Err("phase20_correction_signature_empty".to_string());
    }
    if plan.correction_weight < 0 {
        return Err("phase20_correction_weight_negative".to_string());
    }
    Ok(())
}

pub fn phase20_validate_stabilized_inference_invariants(
    stabilized: &Phase20StabilizedInference,
) -> Result<(), String> {
    if !stabilized.stabilized_well_formed {
        return Err("phase20_stabilized_marked_malformed".to_string());
    }
    if stabilized.original_inference_signature.is_empty() {
        return Err("phase20_original_inference_signature_empty".to_string());
    }
    if stabilized.stabilized_inference_signature.is_empty() {
        return Err("phase20_stabilized_inference_signature_empty".to_string());
    }
    if stabilized.stabilization_signature.is_empty() {
        return Err("phase20_stabilization_signature_empty".to_string());
    }
    if stabilized.telemetry_digest.is_empty() {
        return Err("phase20_telemetry_digest_empty".to_string());
    }
    Ok(())
}

pub fn phase20_emit_correction_telemetry(plan: &Phase20CorrectionPlan) -> String {
    let line = format!(
        "op={:?}:candidate={}:inference={}:resonance={}:weight={}:signature={}:well_formed={}",
        plan.correction_operator,
        plan.target_candidate_signature,
        plan.target_inference_signature,
        plan.target_resonance_signature,
        plan.correction_weight,
        plan.correction_signature,
        plan.plan_well_formed,
    );
    env::set_var(PHASE20_CORRECTION_TELEMETRY, &line);
    line
}

pub fn phase20_emit_stabilization_telemetry(stabilized: &Phase20StabilizedInference) -> String {
    let line = format!(
        "orig={}:stabilized={}:label={:?}:delta={}:sig={}:digest={}:well_formed={}",
        stabilized.original_inference_signature,
        stabilized.stabilized_inference_signature,
        stabilized.stabilized_semantic_label,
        stabilized.stabilization_delta,
        stabilized.stabilization_signature,
        stabilized.telemetry_digest,
        stabilized.stabilized_well_formed,
    );
    env::set_var(PHASE20_STABILIZATION_TELEMETRY, &line);
    line
}

fn phase20_hash_u64(input: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn phase20_hash(input: &str) -> String {
    format!("{:016x}", phase20_hash_u64(input))
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
    use crate::cognition::phase19_arbitration_operators::{
        phase19_resolve_inference_conflict, Phase19MetaOperator,
    };

    fn fixture() -> (Phase18InferenceTrajectory, Phase19ArbitrationDecision) {
        let q0 = phase13_build_qubit_state(0, 0, 1).expect("q0");
        let initial = phase15_build_two_qubit_state(q0.clone(), q0.clone()).expect("init");
        let trajectory = phase16_trace_thought_path(
            &initial,
            &[
                phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledX),
                phase16_build_single_qubit_op(Phase16QubitTarget::Q2, Phase13QubitUnaryOp::PauliZ),
            ],
        )
        .expect("trajectory");
        let field18 = phase18_build_resonance_field(
            &trajectory.steps[0].op_signature,
            &trajectory.final_binding_signature,
            &trajectory.trajectory_signature,
        )
        .expect("field18");
        let inference = phase18_infer_trajectory(&trajectory, &field18).expect("inference");
        let (_, decision) = phase19_resolve_inference_conflict(
            &[inference.clone()],
            Phase19MetaOperator::ConflictResolver,
        )
        .expect("resolve");
        (inference, decision)
    }

    #[test]
    fn phase20_correction_plan_is_deterministic() {
        let (inference, decision) = fixture();
        let a = phase20_build_correction_plan(&inference, &decision).expect("a");
        let b = phase20_build_correction_plan(&inference, &decision).expect("b");
        assert_eq!(a, b);
        phase20_validate_correction_plan_invariants(&a).expect("invariants");
    }

    #[test]
    fn phase20_apply_self_correction_is_replay_stable() {
        let (inference, decision) = fixture();
        let plan = phase20_build_correction_plan(&inference, &decision).expect("plan");
        let base = phase20_apply_self_correction(&inference, &plan).expect("base");
        for _ in 0..50 {
            let current = phase20_apply_self_correction(&inference, &plan).expect("current");
            assert_eq!(current, base);
        }
        phase20_validate_stabilized_inference_invariants(&base).expect("stabilized invariants");
    }

    #[test]
    fn phase20_telemetry_is_stable() {
        let (inference, decision) = fixture();
        let plan = phase20_build_correction_plan(&inference, &decision).expect("plan");
        let stabilized = phase20_apply_self_correction(&inference, &plan).expect("stabilized");

        let p1 = phase20_emit_correction_telemetry(&plan);
        let p2 = phase20_emit_correction_telemetry(&plan);
        let s1 = phase20_emit_stabilization_telemetry(&stabilized);
        let s2 = phase20_emit_stabilization_telemetry(&stabilized);

        assert_eq!(p1, p2);
        assert_eq!(s1, s2);
    }
}
