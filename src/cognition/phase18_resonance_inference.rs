use crate::cognition::phase15_two_qubit_semantic_binding::Phase15BindingKind;
use crate::cognition::phase16_thought_path_trajectories::{
    phase16_validate_trajectory_invariants, Phase16ThoughtStep, Phase16ThoughtTrajectory,
};
use crate::cognition::phase17_semantic_measurement_operators::Phase17SemanticLabel;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE18_RESONANCE_TELEMETRY: &str = "GORT_PHASE18_RESONANCE_TELEMETRY";
const PHASE18_INFERENCE_TELEMETRY: &str = "GORT_PHASE18_INFERENCE_TELEMETRY";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase18InfluenceOperator {
    FavorStability,
    FavorEntangling,
    FavorContrast,
    DampenConflict,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase18ResonanceField {
    pub operator_signature: String,
    pub binding_signature: String,
    pub trajectory_signature: String,
    pub operator_weight: i32,
    pub binding_weight: i32,
    pub trajectory_weight: i32,
    pub total_weight: i32,
    pub resonance_signature: String,
    pub field_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase18InferenceStep {
    pub step_index: usize,
    pub input_step_signature: String,
    pub influence_operator: Phase18InfluenceOperator,
    pub resonance_weight: i32,
    pub output_step: Phase16ThoughtStep,
    pub inference_step_signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase18InferenceTrajectory {
    pub initial_trajectory_signature: String,
    pub resonance_signature: String,
    pub influence_operator: Phase18InfluenceOperator,
    pub inference_steps: Vec<Phase18InferenceStep>,
    pub final_step_signature: String,
    pub final_semantic_label: Phase17SemanticLabel,
    pub inference_signature: String,
    pub telemetry_digest: String,
    pub trajectory_well_formed: bool,
}

pub fn phase18_build_resonance_field(
    operator_signature: &str,
    binding_signature: &str,
    trajectory_signature: &str,
) -> Result<Phase18ResonanceField, String> {
    if operator_signature.is_empty() {
        return Err("phase18_operator_signature_empty".to_string());
    }
    if binding_signature.is_empty() {
        return Err("phase18_binding_signature_empty".to_string());
    }
    if trajectory_signature.is_empty() {
        return Err("phase18_trajectory_signature_empty".to_string());
    }

    let operator_weight = phase18_weight(operator_signature, "op");
    let binding_weight = phase18_weight(binding_signature, "binding");
    let trajectory_weight = phase18_weight(trajectory_signature, "trajectory");
    let total_weight = operator_weight + binding_weight + trajectory_weight;
    let resonance_signature = phase18_hash(&format!(
        "op={}|binding={}|trajectory={}|ow={}|bw={}|tw={}|sum={}",
        operator_signature,
        binding_signature,
        trajectory_signature,
        operator_weight,
        binding_weight,
        trajectory_weight,
        total_weight,
    ));

    Ok(Phase18ResonanceField {
        operator_signature: operator_signature.to_string(),
        binding_signature: binding_signature.to_string(),
        trajectory_signature: trajectory_signature.to_string(),
        operator_weight,
        binding_weight,
        trajectory_weight,
        total_weight,
        resonance_signature,
        field_well_formed: true,
    })
}

pub fn phase18_select_influence_operator(
    field: &Phase18ResonanceField,
) -> Result<Phase18InfluenceOperator, String> {
    phase18_validate_resonance_field_invariants(field)?;

    let op = match field.total_weight.rem_euclid(4) {
        0 => Phase18InfluenceOperator::FavorStability,
        1 => Phase18InfluenceOperator::FavorEntangling,
        2 => Phase18InfluenceOperator::FavorContrast,
        _ => Phase18InfluenceOperator::DampenConflict,
    };
    Ok(op)
}

pub fn phase18_infer_step(
    input_step: &Phase16ThoughtStep,
    field: &Phase18ResonanceField,
    influence_operator: Phase18InfluenceOperator,
) -> Result<Phase18InferenceStep, String> {
    phase18_validate_resonance_field_invariants(field)?;
    if input_step.input_step_signature().is_empty() {
        return Err("phase18_input_step_signature_empty".to_string());
    }

    let resonance_weight = field.total_weight + (input_step.step_index as i32);
    let output_binding_kind = phase18_modulate_binding_kind(
        input_step.to_binding_kind,
        influence_operator,
        resonance_weight,
    );

    let output_to_binding_signature = phase18_hash(&format!(
        "from={}|op={}|influence={:?}|resonance={}|old_to={}|res_sig={}",
        input_step.from_binding_signature,
        input_step.op_signature,
        influence_operator,
        resonance_weight,
        input_step.to_binding_signature,
        field.resonance_signature,
    ));

    let output_step_signature = phase18_hash(&format!(
        "idx={}|from={}|op={}|to={}|kind={:?}",
        input_step.step_index,
        input_step.from_binding_signature,
        input_step.op_signature,
        output_to_binding_signature,
        output_binding_kind,
    ));

    let output_step = Phase16ThoughtStep {
        step_index: input_step.step_index,
        from_binding_signature: input_step.from_binding_signature.clone(),
        op_signature: input_step.op_signature.clone(),
        to_binding_signature: output_to_binding_signature,
        to_binding_kind: output_binding_kind,
        step_signature: output_step_signature,
    };

    let inference_step_signature = phase18_hash(&format!(
        "input={}|output={}|op={:?}|w={}|res={}",
        input_step.input_step_signature(),
        output_step.step_signature,
        influence_operator,
        resonance_weight,
        field.resonance_signature,
    ));

    Ok(Phase18InferenceStep {
        step_index: input_step.step_index,
        input_step_signature: input_step.input_step_signature().to_string(),
        influence_operator,
        resonance_weight,
        output_step,
        inference_step_signature,
    })
}

pub fn phase18_infer_trajectory(
    trajectory: &Phase16ThoughtTrajectory,
    field: &Phase18ResonanceField,
) -> Result<Phase18InferenceTrajectory, String> {
    phase16_validate_trajectory_invariants(trajectory)?;
    phase18_validate_resonance_field_invariants(field)?;

    let influence_operator = phase18_select_influence_operator(field)?;
    let mut inference_steps = Vec::with_capacity(trajectory.steps.len());

    for step in &trajectory.steps {
        let inferred = phase18_infer_step(step, field, influence_operator)?;
        inference_steps.push(inferred);
    }

    let final_step_signature = if let Some(last) = inference_steps.last() {
        last.output_step.step_signature.clone()
    } else {
        trajectory.trajectory_signature.clone()
    };

    let final_semantic_label = if let Some(last) = inference_steps.last() {
        phase18_binding_kind_to_semantic_label(last.output_step.to_binding_kind)
    } else {
        Phase17SemanticLabel::Alignment
    };

    let step_signatures = inference_steps
        .iter()
        .map(|s| s.inference_step_signature.as_str())
        .collect::<Vec<_>>()
        .join("|");
    let inference_signature = phase18_hash(&format!(
        "initial={}|resonance={}|operator={:?}|steps=[{}]|final_step={}|label={:?}",
        trajectory.trajectory_signature,
        field.resonance_signature,
        influence_operator,
        step_signatures,
        final_step_signature,
        final_semantic_label,
    ));

    let telemetry_digest = phase18_hash(&format!(
        "resonance={}|inference={}|label={:?}|count={}",
        field.resonance_signature,
        inference_signature,
        final_semantic_label,
        inference_steps.len(),
    ));

    Ok(Phase18InferenceTrajectory {
        initial_trajectory_signature: trajectory.trajectory_signature.clone(),
        resonance_signature: field.resonance_signature.clone(),
        influence_operator,
        inference_steps,
        final_step_signature,
        final_semantic_label,
        inference_signature,
        telemetry_digest,
        trajectory_well_formed: true,
    })
}

pub fn phase18_validate_resonance_field_invariants(
    field: &Phase18ResonanceField,
) -> Result<(), String> {
    if !field.field_well_formed {
        return Err("phase18_resonance_field_marked_malformed".to_string());
    }
    if field.operator_signature.is_empty() {
        return Err("phase18_operator_signature_empty".to_string());
    }
    if field.binding_signature.is_empty() {
        return Err("phase18_binding_signature_empty".to_string());
    }
    if field.trajectory_signature.is_empty() {
        return Err("phase18_trajectory_signature_empty".to_string());
    }
    if field.resonance_signature.is_empty() {
        return Err("phase18_resonance_signature_empty".to_string());
    }
    if field.total_weight != (field.operator_weight + field.binding_weight + field.trajectory_weight)
    {
        return Err("phase18_total_weight_mismatch".to_string());
    }
    Ok(())
}

pub fn phase18_validate_inference_trajectory_invariants(
    trajectory: &Phase18InferenceTrajectory,
) -> Result<(), String> {
    if !trajectory.trajectory_well_formed {
        return Err("phase18_inference_trajectory_marked_malformed".to_string());
    }
    if trajectory.initial_trajectory_signature.is_empty() {
        return Err("phase18_initial_trajectory_signature_empty".to_string());
    }
    if trajectory.resonance_signature.is_empty() {
        return Err("phase18_resonance_signature_empty".to_string());
    }
    if trajectory.final_step_signature.is_empty() {
        return Err("phase18_final_step_signature_empty".to_string());
    }
    if trajectory.inference_signature.is_empty() {
        return Err("phase18_inference_signature_empty".to_string());
    }
    if trajectory.telemetry_digest.is_empty() {
        return Err("phase18_telemetry_digest_empty".to_string());
    }
    for (expected_index, step) in trajectory.inference_steps.iter().enumerate() {
        if step.step_index != expected_index {
            return Err(format!(
                "phase18_step_index_mismatch: expected {} got {}",
                expected_index, step.step_index,
            ));
        }
        if step.inference_step_signature.is_empty() {
            return Err(format!(
                "phase18_inference_step_signature_empty at step {}",
                expected_index,
            ));
        }
    }
    Ok(())
}

pub fn phase18_emit_resonance_telemetry(field: &Phase18ResonanceField) -> String {
    let line = format!(
        "op_sig={}:binding_sig={}:trajectory_sig={}:ow={}:bw={}:tw={}:sum={}:res_sig={}:well_formed={}",
        field.operator_signature,
        field.binding_signature,
        field.trajectory_signature,
        field.operator_weight,
        field.binding_weight,
        field.trajectory_weight,
        field.total_weight,
        field.resonance_signature,
        field.field_well_formed,
    );
    env::set_var(PHASE18_RESONANCE_TELEMETRY, &line);
    line
}

pub fn phase18_emit_inference_telemetry(trajectory: &Phase18InferenceTrajectory) -> String {
    let line = format!(
        "initial={}:res={}:op={:?}:steps={}:final_step={}:label={:?}:inference_sig={}:telemetry_digest={}:well_formed={}",
        trajectory.initial_trajectory_signature,
        trajectory.resonance_signature,
        trajectory.influence_operator,
        trajectory.inference_steps.len(),
        trajectory.final_step_signature,
        trajectory.final_semantic_label,
        trajectory.inference_signature,
        trajectory.telemetry_digest,
        trajectory.trajectory_well_formed,
    );
    env::set_var(PHASE18_INFERENCE_TELEMETRY, &line);
    line
}

fn phase18_weight(signature: &str, salt: &str) -> i32 {
    let h = phase18_hash_u64(&format!("{}|{}", salt, signature));
    ((h % 97) as i32) + 1
}

fn phase18_modulate_binding_kind(
    original: Phase15BindingKind,
    influence_operator: Phase18InfluenceOperator,
    resonance_weight: i32,
) -> Phase15BindingKind {
    match influence_operator {
        Phase18InfluenceOperator::FavorStability => {
            if resonance_weight.rem_euclid(2) == 0 {
                Phase15BindingKind::Correlated
            } else {
                original
            }
        }
        Phase18InfluenceOperator::FavorEntangling => Phase15BindingKind::EntanglingOpApplied,
        Phase18InfluenceOperator::FavorContrast => {
            if original == Phase15BindingKind::Correlated {
                Phase15BindingKind::AntiCorrelated
            } else {
                Phase15BindingKind::AntiCorrelated
            }
        }
        Phase18InfluenceOperator::DampenConflict => {
            if original == Phase15BindingKind::Separable {
                Phase15BindingKind::Correlated
            } else {
                original
            }
        }
    }
}

fn phase18_binding_kind_to_semantic_label(kind: Phase15BindingKind) -> Phase17SemanticLabel {
    match kind {
        Phase15BindingKind::Correlated => Phase17SemanticLabel::Reinforcement,
        Phase15BindingKind::AntiCorrelated => Phase17SemanticLabel::Contrast,
        Phase15BindingKind::Separable => Phase17SemanticLabel::Conflict,
        Phase15BindingKind::EntanglingOpApplied => Phase17SemanticLabel::EntanglingInfluence,
    }
}

fn phase18_hash_u64(input: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn phase18_hash(input: &str) -> String {
    format!("{:016x}", phase18_hash_u64(input))
}

trait Phase18StepSignatureExt {
    fn input_step_signature(&self) -> &str;
}

impl Phase18StepSignatureExt for Phase16ThoughtStep {
    fn input_step_signature(&self) -> &str {
        &self.step_signature
    }
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

    fn fixture_trajectory() -> Phase16ThoughtTrajectory {
        let q0 = phase13_build_qubit_state(0, 0, 1).expect("q0");
        let initial = phase15_build_two_qubit_state(q0.clone(), q0.clone()).expect("initial");
        let ops = vec![
            phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledX),
            phase16_build_single_qubit_op(Phase16QubitTarget::Q2, Phase13QubitUnaryOp::PauliZ),
        ];
        phase16_trace_thought_path(&initial, &ops).expect("trajectory")
    }

    #[test]
    fn phase18_resonance_field_is_replay_stable() {
        let t = fixture_trajectory();
        let a = phase18_build_resonance_field(
            &t.steps[0].op_signature,
            &t.final_binding_signature,
            &t.trajectory_signature,
        )
        .expect("field a");
        let b = phase18_build_resonance_field(
            &t.steps[0].op_signature,
            &t.final_binding_signature,
            &t.trajectory_signature,
        )
        .expect("field b");
        assert_eq!(a, b);
        phase18_validate_resonance_field_invariants(&a).expect("invariants");
    }

    #[test]
    fn phase18_inference_step_is_deterministic() {
        let t = fixture_trajectory();
        let field = phase18_build_resonance_field(
            &t.steps[0].op_signature,
            &t.final_binding_signature,
            &t.trajectory_signature,
        )
        .expect("field");
        let op = phase18_select_influence_operator(&field).expect("operator");
        let a = phase18_infer_step(&t.steps[0], &field, op).expect("a");
        let b = phase18_infer_step(&t.steps[0], &field, op).expect("b");
        assert_eq!(a, b);
    }

    #[test]
    fn phase18_inference_trajectory_is_replay_stable() {
        let t = fixture_trajectory();
        let field = phase18_build_resonance_field(
            &t.steps[0].op_signature,
            &t.final_binding_signature,
            &t.trajectory_signature,
        )
        .expect("field");

        let baseline = phase18_infer_trajectory(&t, &field).expect("baseline");
        phase18_validate_inference_trajectory_invariants(&baseline).expect("invariants");

        for _ in 0..50 {
            let current = phase18_infer_trajectory(&t, &field).expect("current");
            assert_eq!(current, baseline);
        }
    }

    #[test]
    fn phase18_empty_trajectory_has_alignment_label() {
        let q0 = phase13_build_qubit_state(0, 0, 1).expect("q0");
        let initial = phase15_build_two_qubit_state(q0.clone(), q0.clone()).expect("initial");
        let t = phase16_trace_thought_path(&initial, &[]).expect("trajectory");
        let field = phase18_build_resonance_field(
            "empty-op",
            &t.final_binding_signature,
            &t.trajectory_signature,
        )
        .expect("field");

        let inferred = phase18_infer_trajectory(&t, &field).expect("inferred");
        assert_eq!(inferred.final_semantic_label, Phase17SemanticLabel::Alignment);
    }
}
