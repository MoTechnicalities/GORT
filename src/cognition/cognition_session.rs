use crate::cognition::phase13_qubit_kernel::{
    phase13_build_qubit_state, Phase13QubitState, Phase13QubitUnaryOp,
};
use crate::cognition::phase14_operator_algebra::{
    phase14_build_commutation_table, phase14_build_pauli_family, phase14_compute_commutator,
    Phase14Commutator,
};
use crate::cognition::phase15_two_qubit_semantic_binding::{
    phase15_build_two_qubit_state, Phase15TwoQubitState, Phase15TwoQubitOp,
};
use crate::cognition::phase16_thought_path_trajectories::{
    phase16_apply_thought_op, phase16_build_single_qubit_op, phase16_build_two_qubit_op,
    phase16_trace_thought_path, Phase16QubitTarget, Phase16ThoughtOp, Phase16TrajectoryKind,
};
use crate::cognition::phase17_semantic_measurement_operators::{
    phase17_compose_semantic_digest, phase17_compose_semantic_signature, phase17_measure_binding,
    phase17_measure_semantic, phase17_measure_trajectory, Phase17SemanticLabel,
};
use crate::cognition::phase18_resonance_inference::{
    phase18_build_resonance_field, phase18_infer_trajectory, Phase18InferenceTrajectory,
};
use crate::cognition::phase19_arbitration_operators::{
    phase19_resolve_inference_conflict, Phase19ArbitrationDecision, Phase19ArbitrationField,
    Phase19MetaOperator,
};
use crate::cognition::phase20_self_correction_operators::{
    phase20_apply_self_correction, phase20_build_correction_plan, phase20_detect_drift,
    phase20_emit_correction_telemetry, phase20_emit_stabilization_telemetry,
    Phase20CorrectionPlan, Phase20DriftReport, Phase20StabilizedInference,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitionPlan {
    pub goal: String,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitionPlanTrace {
    pub plan_id: String,
    pub plan_signature: String,
    pub goal_signature: String,
    pub step_signature: String,
    pub initial_q1_signature: String,
    pub initial_q2_signature: String,
    pub phase14_commutation_signature: String,
    pub phase14_commutator: Phase14Commutator,
    pub phase15_initial_binding_signature: String,
    pub phase16_trajectory_signature: String,
    pub phase16_trajectory_kind: Phase16TrajectoryKind,
    pub phase17_semantic_signature: String,
    pub phase17_semantic_digest: String,
    pub phase17_trajectory_label: Phase17SemanticLabel,
    pub phase18_resonance_signature: String,
    pub phase18_inference_signature: String,
    pub phase18_final_label: Phase17SemanticLabel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitionSessionResult {
    pub winner_plan_id: String,
    pub winner_plan_signature: String,
    pub semantic_label: Phase17SemanticLabel,
    pub arbitration_signature: String,
    pub correction_signature: String,
    pub stabilization_signature: String,
    pub trajectory_kind: Phase16TrajectoryKind,
    pub telemetry_digest: String,
    pub arbitration_decision_signature: String,
    pub plan_traces: Vec<CognitionPlanTrace>,
    pub arbitration_field: Phase19ArbitrationField,
    pub arbitration_decision: Phase19ArbitrationDecision,
    pub drift_report: Phase20DriftReport,
    pub correction_plan: Phase20CorrectionPlan,
    pub stabilized_inference: Phase20StabilizedInference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitionTournamentResult {
    pub winner_plan_id: String,
    pub winner_plan_signature: String,
    pub semantic_label: Phase17SemanticLabel,
    pub arbitration_signature: String,
    pub correction_signature: String,
    pub stabilization_signature: String,
    pub trajectory_kind: Phase16TrajectoryKind,
    pub telemetry_digest: String,
    pub arbitration_decision_signature: String,
    pub candidate_count: usize,
    pub plan_traces: Vec<CognitionPlanTrace>,
    pub arbitration_field: Phase19ArbitrationField,
    pub arbitration_decision: Phase19ArbitrationDecision,
    pub drift_report: Phase20DriftReport,
    pub correction_plan: Phase20CorrectionPlan,
    pub stabilized_inference: Phase20StabilizedInference,
}

pub fn run_cognition_session(
    plan_a: &CognitionPlan,
    plan_b: &CognitionPlan,
) -> Result<CognitionSessionResult, String> {
    let outputs = vec![
        phase13_to_phase18_plan_pipeline("plan_a", plan_a)?,
        phase13_to_phase18_plan_pipeline("plan_b", plan_b)?,
    ];

    let inferences = outputs
        .iter()
        .map(|o| o.inference.clone())
        .collect::<Vec<_>>();
    let (field, decision) = phase19_resolve_inference_conflict(
        &inferences,
        Phase19MetaOperator::SemanticCoherenceFirst,
    )?;

    let finalized = finalize_cognitive_decision(outputs, field, decision)?;

    Ok(CognitionSessionResult {
        winner_plan_id: finalized.winner_plan_id,
        winner_plan_signature: finalized.winner_plan_signature,
        semantic_label: finalized.semantic_label,
        arbitration_signature: finalized.arbitration_signature,
        correction_signature: finalized.correction_signature,
        stabilization_signature: finalized.stabilization_signature,
        trajectory_kind: finalized.trajectory_kind,
        telemetry_digest: finalized.telemetry_digest,
        arbitration_decision_signature: finalized.arbitration_decision_signature,
        plan_traces: finalized.plan_traces,
        arbitration_field: finalized.arbitration_field,
        arbitration_decision: finalized.arbitration_decision,
        drift_report: finalized.drift_report,
        correction_plan: finalized.correction_plan,
        stabilized_inference: finalized.stabilized_inference,
    })
}

pub fn run_cognition_tournament(
    plans: &[CognitionPlan],
) -> Result<CognitionTournamentResult, String> {
    if plans.len() < 3 {
        return Err("cognition_tournament_requires_at_least_3_plans".to_string());
    }

    let mut outputs = Vec::with_capacity(plans.len());
    for (idx, plan) in plans.iter().enumerate() {
        outputs.push(phase13_to_phase18_plan_pipeline(
            &format!("plan_{:02}", idx + 1),
            plan,
        )?);
    }

    let inferences = outputs
        .iter()
        .map(|o| o.inference.clone())
        .collect::<Vec<_>>();
    let (field, decision) = phase19_resolve_inference_conflict(
        &inferences,
        Phase19MetaOperator::SemanticCoherenceFirst,
    )?;

    let finalized = finalize_cognitive_decision(outputs, field, decision)?;
    Ok(CognitionTournamentResult {
        winner_plan_id: finalized.winner_plan_id,
        winner_plan_signature: finalized.winner_plan_signature,
        semantic_label: finalized.semantic_label,
        arbitration_signature: finalized.arbitration_signature,
        correction_signature: finalized.correction_signature,
        stabilization_signature: finalized.stabilization_signature,
        trajectory_kind: finalized.trajectory_kind,
        telemetry_digest: finalized.telemetry_digest,
        arbitration_decision_signature: finalized.arbitration_decision_signature,
        candidate_count: finalized.plan_traces.len(),
        plan_traces: finalized.plan_traces,
        arbitration_field: finalized.arbitration_field,
        arbitration_decision: finalized.arbitration_decision,
        drift_report: finalized.drift_report,
        correction_plan: finalized.correction_plan,
        stabilized_inference: finalized.stabilized_inference,
    })
}

#[derive(Debug, Clone)]
struct PlanPipelineOutput {
    trace: CognitionPlanTrace,
    inference: Phase18InferenceTrajectory,
}

#[derive(Debug, Clone)]
struct FinalizedCognitiveDecision {
    winner_plan_id: String,
    winner_plan_signature: String,
    semantic_label: Phase17SemanticLabel,
    arbitration_signature: String,
    correction_signature: String,
    stabilization_signature: String,
    trajectory_kind: Phase16TrajectoryKind,
    telemetry_digest: String,
    arbitration_decision_signature: String,
    plan_traces: Vec<CognitionPlanTrace>,
    arbitration_field: Phase19ArbitrationField,
    arbitration_decision: Phase19ArbitrationDecision,
    drift_report: Phase20DriftReport,
    correction_plan: Phase20CorrectionPlan,
    stabilized_inference: Phase20StabilizedInference,
}

fn finalize_cognitive_decision(
    outputs: Vec<PlanPipelineOutput>,
    field: Phase19ArbitrationField,
    decision: Phase19ArbitrationDecision,
) -> Result<FinalizedCognitiveDecision, String> {
    let winner_index = parse_candidate_index(&decision.selected_candidate_id)?;
    let winner = outputs
        .get(winner_index)
        .ok_or_else(|| "cognition_candidate_index_out_of_bounds".to_string())?;

    let drift_report = phase20_detect_drift(&winner.inference)?;
    let correction_plan = phase20_build_correction_plan(&winner.inference, &decision)?;
    let stabilized_inference = phase20_apply_self_correction(&winner.inference, &correction_plan)?;

    let correction_telemetry = phase20_emit_correction_telemetry(&correction_plan);
    let stabilization_telemetry = phase20_emit_stabilization_telemetry(&stabilized_inference);
    let telemetry_digest = cognition_hash(&format!(
        "winner={}|field={}|decision={}|corr={}|stab={}",
        winner.trace.plan_signature,
        field.arbitration_signature,
        decision.decision_signature,
        correction_telemetry,
        stabilization_telemetry,
    ));

    Ok(FinalizedCognitiveDecision {
        winner_plan_id: winner.trace.plan_id.clone(),
        winner_plan_signature: winner.trace.plan_signature.clone(),
        semantic_label: stabilized_inference.stabilized_semantic_label,
        arbitration_signature: field.arbitration_signature.clone(),
        correction_signature: correction_plan.correction_signature.clone(),
        stabilization_signature: stabilized_inference.stabilization_signature.clone(),
        trajectory_kind: winner.trace.phase16_trajectory_kind,
        telemetry_digest,
        arbitration_decision_signature: decision.decision_signature.clone(),
        plan_traces: outputs.into_iter().map(|o| o.trace).collect::<Vec<_>>(),
        arbitration_field: field,
        arbitration_decision: decision,
        drift_report,
        correction_plan,
        stabilized_inference,
    })
}

fn phase13_to_phase18_plan_pipeline(
    plan_id: &str,
    plan: &CognitionPlan,
) -> Result<PlanPipelineOutput, String> {
    validate_plan(plan)?;

    let goal_signature = cognition_hash(&format!("goal={}", normalize_text(&plan.goal)));
    let step_signature = cognition_hash(&plan.steps.iter().map(|s| normalize_text(s)).collect::<Vec<_>>().join("|"));
    let plan_signature = cognition_hash(&format!("goal={}|steps={}", goal_signature, step_signature));

    let q1 = basis_state_from_seed(&goal_signature)?;
    let q2 = basis_state_from_seed(&step_signature)?;
    let initial_binding = phase15_build_two_qubit_state(q1.clone(), q2.clone())?;

    let thought_ops = plan
        .steps
        .iter()
        .map(|s| step_to_thought_op(s))
        .collect::<Vec<_>>();
    let trajectory = phase16_trace_thought_path(&initial_binding, &thought_ops)?;

    let pauli_family = phase14_build_pauli_family();
    let commutation_table = phase14_build_commutation_table(&pauli_family);
    let unary_pair = phase14_unary_pair(plan);
    let commutator = phase14_compute_commutator(unary_pair.0, unary_pair.1)?;

    let final_binding = replay_final_binding(&initial_binding, &thought_ops)?;
    let m_state = phase17_measure_semantic(&final_binding.q1)?;
    let m_binding = phase17_measure_binding(&final_binding)?;
    let m_trajectory = phase17_measure_trajectory(&trajectory)?;
    let phase17_semantic_signature =
        phase17_compose_semantic_signature(&[m_state.clone(), m_binding.clone(), m_trajectory.clone()]);
    let phase17_semantic_digest =
        phase17_compose_semantic_digest(&[m_state, m_binding, m_trajectory.clone()]);

    let resonance_anchor = trajectory
        .steps
        .first()
        .map(|s| s.op_signature.as_str())
        .unwrap_or(&trajectory.trajectory_signature);
    let resonance_field = phase18_build_resonance_field(
        resonance_anchor,
        &trajectory.final_binding_signature,
        &trajectory.trajectory_signature,
    )?;
    let inference = phase18_infer_trajectory(&trajectory, &resonance_field)?;

    Ok(PlanPipelineOutput {
        trace: CognitionPlanTrace {
            plan_id: plan_id.to_string(),
            plan_signature,
            goal_signature,
            step_signature,
            initial_q1_signature: q1.state_signature,
            initial_q2_signature: q2.state_signature,
            phase14_commutation_signature: commutation_table.table_signature,
            phase14_commutator: commutator.clone(),
            phase15_initial_binding_signature: initial_binding.binding_signature,
            phase16_trajectory_signature: trajectory.trajectory_signature,
            phase16_trajectory_kind: trajectory.trajectory_kind,
            phase17_semantic_signature,
            phase17_semantic_digest,
            phase17_trajectory_label: m_trajectory.semantic_label,
            phase18_resonance_signature: resonance_field.resonance_signature,
            phase18_inference_signature: inference.inference_signature.clone(),
            phase18_final_label: inference.final_semantic_label,
        },
        inference,
    })
}

fn validate_plan(plan: &CognitionPlan) -> Result<(), String> {
    if normalize_text(&plan.goal).is_empty() {
        return Err("cognition_plan_goal_empty".to_string());
    }
    if plan.steps.is_empty() {
        return Err("cognition_plan_steps_empty".to_string());
    }
    for (idx, step) in plan.steps.iter().enumerate() {
        if normalize_text(step).is_empty() {
            return Err(format!("cognition_plan_step_empty:{idx}"));
        }
    }
    Ok(())
}

fn normalize_text(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn basis_state_from_seed(seed: &str) -> Result<Phase13QubitState, String> {
    const BASIS: [(i8, i8, i8); 6] = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let idx = (cognition_hash_u64(seed) % BASIS.len() as u64) as usize;
    let (x, y, z) = BASIS[idx];
    phase13_build_qubit_state(x, y, z)
}

fn step_to_thought_op(step: &str) -> Phase16ThoughtOp {
    let normalized = normalize_text(step);

    if normalized.contains("turn") {
        return phase16_build_two_qubit_op(Phase15TwoQubitOp::Swap);
    }
    if normalized.contains("advance") || normalized.contains("forward") || normalized.contains("move") {
        return phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledX);
    }
    if normalized.contains("align") || normalized.contains("stabil") {
        return phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledZ);
    }

    match cognition_hash_u64(&normalized) % 5 {
        0 => phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledX),
        1 => phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledZ),
        2 => phase16_build_two_qubit_op(Phase15TwoQubitOp::Swap),
        3 => phase16_build_single_qubit_op(Phase16QubitTarget::Q1, Phase13QubitUnaryOp::PauliX),
        _ => phase16_build_single_qubit_op(Phase16QubitTarget::Q2, Phase13QubitUnaryOp::PauliZ),
    }
}

fn replay_final_binding(
    initial_binding: &Phase15TwoQubitState,
    thought_ops: &[Phase16ThoughtOp],
) -> Result<Phase15TwoQubitState, String> {
    let mut current = initial_binding.clone();
    for op in thought_ops {
        current = phase16_apply_thought_op(&current, op)?;
    }
    Ok(current)
}

fn phase14_unary_pair(plan: &CognitionPlan) -> (Phase13QubitUnaryOp, Phase13QubitUnaryOp) {
    let first = plan
        .steps
        .first()
        .map(|s| step_to_unary_hint(s))
        .unwrap_or(Phase13QubitUnaryOp::PauliX);
    let last = plan
        .steps
        .last()
        .map(|s| step_to_unary_hint(s))
        .unwrap_or(Phase13QubitUnaryOp::PauliZ);
    (first, last)
}

fn step_to_unary_hint(step: &str) -> Phase13QubitUnaryOp {
    let normalized = normalize_text(step);
    if normalized.contains("left") || normalized.contains("right") || normalized.contains("turn") {
        Phase13QubitUnaryOp::PauliZ
    } else if normalized.contains("move") || normalized.contains("advance") || normalized.contains("forward") {
        Phase13QubitUnaryOp::PauliX
    } else {
        // Keep hints in integer-commutator-safe space for domain-agnostic plan text.
        if cognition_hash_u64(&normalized) % 2 == 0 {
            Phase13QubitUnaryOp::PauliX
        } else {
            Phase13QubitUnaryOp::PauliZ
        }
    }
}

fn parse_candidate_index(candidate_id: &str) -> Result<usize, String> {
    let suffix = candidate_id
        .strip_prefix("cand_")
        .ok_or_else(|| "cognition_candidate_id_prefix_invalid".to_string())?;
    suffix
        .parse::<usize>()
        .map_err(|_| "cognition_candidate_id_parse_failed".to_string())
}

fn cognition_hash_u64(input: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn cognition_hash(input: &str) -> String {
    format!("{:016x}", cognition_hash_u64(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_plan_a() -> CognitionPlan {
        CognitionPlan {
            goal: "Reach point X".to_string(),
            steps: vec![
                "Move forward".to_string(),
                "Turn left".to_string(),
                "Advance 3 units".to_string(),
            ],
        }
    }

    fn fixture_plan_b() -> CognitionPlan {
        CognitionPlan {
            goal: "Reach point X".to_string(),
            steps: vec![
                "Turn left".to_string(),
                "Move forward".to_string(),
                "Advance 3 units".to_string(),
            ],
        }
    }

    fn fixture_plan_c() -> CognitionPlan {
        CognitionPlan {
            goal: "Reach point X".to_string(),
            steps: vec![
                "Move forward".to_string(),
                "Check fuel".to_string(),
                "Turn left".to_string(),
                "Advance 3 units".to_string(),
            ],
        }
    }

    #[test]
    fn cognition_session_is_deterministic_for_two_structured_plans() {
        let plan_a = fixture_plan_a();
        let plan_b = fixture_plan_b();

        let a = run_cognition_session(&plan_a, &plan_b).expect("run a");
        let b = run_cognition_session(&plan_a, &plan_b).expect("run b");

        assert_eq!(a, b);
        assert!(a.winner_plan_id == "plan_a" || a.winner_plan_id == "plan_b");
        assert_eq!(a.plan_traces.len(), 2);
        assert!(!a.arbitration_signature.is_empty());
        assert!(!a.correction_signature.is_empty());
        assert!(!a.stabilization_signature.is_empty());
        assert!(!a.telemetry_digest.is_empty());
    }

    #[test]
    fn cognition_session_rejects_empty_structured_inputs() {
        let bad_plan = CognitionPlan {
            goal: " ".to_string(),
            steps: vec!["ok".to_string()],
        };
        let good = fixture_plan_b();
        let err = run_cognition_session(&bad_plan, &good).expect_err("should fail");
        assert_eq!(err, "cognition_plan_goal_empty");
    }

    #[test]
    fn cognition_tournament_is_deterministic_for_three_plans() {
        let plans = vec![fixture_plan_a(), fixture_plan_b(), fixture_plan_c()];

        let a = run_cognition_tournament(&plans).expect("run a");
        let b = run_cognition_tournament(&plans).expect("run b");

        assert_eq!(a, b);
        assert_eq!(a.candidate_count, 3);
        assert_eq!(a.plan_traces.len(), 3);
        assert!(a.winner_plan_id.starts_with("plan_"));
        assert!(!a.arbitration_signature.is_empty());
        assert!(!a.correction_signature.is_empty());
        assert!(!a.stabilization_signature.is_empty());
        assert!(!a.telemetry_digest.is_empty());
    }

    #[test]
    fn cognition_tournament_requires_three_or_more_plans() {
        let plans = vec![fixture_plan_a(), fixture_plan_b()];
        let err = run_cognition_tournament(&plans).expect_err("should fail");
        assert_eq!(err, "cognition_tournament_requires_at_least_3_plans");
    }
}