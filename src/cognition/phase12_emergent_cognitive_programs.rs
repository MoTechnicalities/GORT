use crate::cognition::phase90_cognitive_manifolds::Phase90CognitiveManifold;
use crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentPlan;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE12_PROGRAM_TELEMETRY: &str = "GORT_PHASE12_EMERGENT_PROGRAM_TELEMETRY";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase12EmergentCognitiveProgramStep {
    pub step_index: usize,
    pub source_operator_id: String,
    pub source_shape_index: usize,
    pub target_shape_index: usize,
    pub operator_kind: String,
    pub step_kind: String,
    pub operator_pressure_percent: u8,
    pub continuity_guard_percent: u8,
    pub manifold_alignment_percent: u8,
    pub step_signature: String,
    pub step_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase12EmergentCognitiveProgram {
    pub manifold_signature: String,
    pub operator_plan_hash: String,
    pub program_kind: String,
    pub step_count: usize,
    pub resonance_gate_percent: u8,
    pub program_steps: Vec<Phase12EmergentCognitiveProgramStep>,
    pub program_signature: String,
    pub program_profile_hash: String,
    pub program_well_formed: bool,
}

pub fn phase12_synthesize_emergent_cognitive_program(
    manifold: &Phase90CognitiveManifold,
    plan: &Phase90GeometryDrivenAdjustmentPlan,
) -> Result<Phase12EmergentCognitiveProgram, String> {
    if !manifold.manifold_well_formed {
        return Err("cannot synthesize program from malformed manifold".to_string());
    }
    if !plan.operator_plan_well_formed {
        return Err("cannot synthesize program from malformed adjustment plan".to_string());
    }
    if manifold.manifold_signature != plan.manifold_signature {
        return Err("manifold/plan signature mismatch".to_string());
    }
    if plan.operators.is_empty() {
        return Err("cannot synthesize program without adjustment operators".to_string());
    }

    let program_kind = phase12_program_kind(
        manifold.manifold_archetype.as_str(),
        plan.dominant_operator_kind.as_str(),
    );
    let resonance_gate_percent = ((manifold.manifold_embedding_weight_percent as u16
        + plan.aggregate_adjustment_pressure_percent as u16)
        / 2)
        .clamp(1, 100) as u8;

    let program_steps = plan
        .operators
        .iter()
        .enumerate()
        .map(|(step_index, operator)| {
            let step_kind = phase12_step_kind(&operator.operator_kind, step_index);
            let continuity_guard_percent = ((operator.continuity_bias_percent as u16
                + manifold.manifold_embedding_weight_percent as u16)
                / 2)
                .clamp(0, 100) as u8;
            let manifold_alignment_percent = ((operator.manifold_alignment_percent as u16
                + manifold.manifold_embedding_weight_percent as u16
                + plan.aggregate_adjustment_pressure_percent as u16)
                / 3)
                .clamp(0, 100) as u8;

            let step_signature = format!(
                "step={}|operator_id={}|source_shape={}|target_shape={}|kind={}|pressure={}|guard={}|alignment={}",
                step_index,
                operator.operator_id,
                operator.source_shape_index,
                operator.target_shape_index,
                step_kind,
                operator.adjustment_pressure_percent,
                continuity_guard_percent,
                manifold_alignment_percent,
            );

            let step_well_formed = !step_kind.is_empty()
                && continuity_guard_percent > 0
                && manifold_alignment_percent > 0;

            Phase12EmergentCognitiveProgramStep {
                step_index,
                source_operator_id: operator.operator_id.clone(),
                source_shape_index: operator.source_shape_index,
                target_shape_index: operator.target_shape_index,
                operator_kind: operator.operator_kind.clone(),
                step_kind,
                operator_pressure_percent: operator.adjustment_pressure_percent,
                continuity_guard_percent,
                manifold_alignment_percent,
                step_signature,
                step_well_formed,
            }
        })
        .collect::<Vec<_>>();

    let step_count = program_steps.len();
    let program_signature = format!(
        "manifold={}|plan={}|kind={}|gate={}|steps={}",
        manifold.manifold_signature,
        plan.operator_plan_hash,
        program_kind,
        resonance_gate_percent,
        program_steps
            .iter()
            .map(|step| step.step_signature.clone())
            .collect::<Vec<_>>()
            .join("||"),
    );

    let program_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        program_signature.hash(&mut hasher);
        resonance_gate_percent.hash(&mut hasher);
        step_count.hash(&mut hasher);
        hasher.finish()
    });

    let program_well_formed = step_count > 0
        && resonance_gate_percent > 0
        && !program_kind.is_empty()
        && program_steps.iter().all(|step| step.step_well_formed);

    Ok(Phase12EmergentCognitiveProgram {
        manifold_signature: manifold.manifold_signature.clone(),
        operator_plan_hash: plan.operator_plan_hash.clone(),
        program_kind,
        step_count,
        resonance_gate_percent,
        program_steps,
        program_signature,
        program_profile_hash,
        program_well_formed,
    })
}

pub fn phase12_emit_program_telemetry(program: &Phase12EmergentCognitiveProgram) -> String {
    let line = format!(
        "manifold_signature={}:operator_plan_hash={}:program_kind={}:step_count={}:resonance_gate={}:program_signature={}:profile_hash={}:well_formed={}",
        program.manifold_signature,
        program.operator_plan_hash,
        program.program_kind,
        program.step_count,
        program.resonance_gate_percent,
        program.program_signature,
        program.program_profile_hash,
        program.program_well_formed,
    );
    env::set_var(PHASE12_PROGRAM_TELEMETRY, &line);
    line
}

fn phase12_program_kind(manifold_archetype: &str, dominant_operator_kind: &str) -> String {
    match (manifold_archetype, dominant_operator_kind) {
        ("local_patch", _) => "local_patch_program".to_string(),
        (_, "resonance_reweight") => "resonant_program".to_string(),
        (_, "gradient_smooth") => "gradient_program".to_string(),
        (_, "continuity_lift") => "continuity_program".to_string(),
        (_, "boundary_probe") => "exploratory_program".to_string(),
        _ => format!("{}::{}", manifold_archetype, dominant_operator_kind),
    }
}

fn phase12_step_kind(operator_kind: &str, step_index: usize) -> String {
    match operator_kind {
        "resonance_reweight" => format!("stabilize_resonance_{}", step_index + 1),
        "gradient_smooth" => format!("smooth_gradient_{}", step_index + 1),
        "continuity_lift" => format!("lift_continuity_{}", step_index + 1),
        "boundary_probe" => format!("probe_boundary_{}", step_index + 1),
        other => format!("{}_{:02}", other, step_index + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognition::phase62_structural_experiment::{
        phase80_build_phase9_integration_hook, phase80_integrate_cross_frame_structural_deltas,
        phase80_run_multiframe_episode, phase80_summarize_episode_structural_integration,
        Phase70AdjustmentLog, Phase70AdjustmentLogEntry, Phase70StructuralParameterRegistry,
    };
    use crate::cognition::phase90_cognitive_manifolds::phase90_form_cognitive_manifold;
    use crate::cognition::phase90_continuity_weighted_fields::phase90_form_continuity_weighted_field_from_seed;
    use crate::cognition::phase90_emergent_cognitive_shapes::phase90_compose_emergent_cognitive_shape;
    use crate::cognition::phase90_geometry_driven_adjustment_operators::phase90_build_geometry_driven_adjustment_plan;
    use crate::cognition::phase90_geometric_cognitive_seed::phase90_form_geometry_seed_from_integration_hook;
    use crate::cognition::phase90_multishape_interaction_dynamics::phase90_compute_multishape_interaction_dynamics;

    const PARAM: &str = "continuity_pressure_boost";

    fn entry(
        sequence: u64,
        holdout_id: &str,
        semantic_context_used: &str,
        adjustment_applied: bool,
        pre_value: i32,
        post_value: i32,
        delta: i32,
    ) -> Phase70AdjustmentLogEntry {
        Phase70AdjustmentLogEntry {
            sequence,
            holdout_id: holdout_id.to_string(),
            parameter_name: PARAM.to_string(),
            semantic_context_used: semantic_context_used.to_string(),
            adjustment_applied,
            pre_value,
            post_value,
            delta,
            inverse_delta: if adjustment_applied { -delta } else { 0 },
        }
    }

    fn program_fixture() -> (Phase90CognitiveManifold, Phase90GeometryDrivenAdjustmentPlan) {
        let registry = Phase70StructuralParameterRegistry::canonical();

        let build_shape = |episode_prefix: &str, logs: Vec<Phase70AdjustmentLog>| {
            let mut fields = Vec::new();
            for (index, log) in logs.iter().enumerate() {
                let episode_id = format!("{}_{}", episode_prefix, index + 1);
                let trace = phase80_run_multiframe_episode(&episode_id, log, &registry).expect("trace");
                let deltas = phase80_integrate_cross_frame_structural_deltas(&trace, log, &registry)
                    .expect("deltas");
                let summary = phase80_summarize_episode_structural_integration(&trace, log, &registry)
                    .expect("summary");
                let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
                let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
                fields.push(phase90_form_continuity_weighted_field_from_seed(&seed));
            }
            phase90_compose_emergent_cognitive_shape(&fields).expect("shape")
        };

        let shape_a = build_shape(
            "program_shape_a",
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "program_a1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "program_a2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "program_a3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "program_a4", "none", false, 1, 1, 0),
                        entry(3, "program_a5", "none", true, 1, 2, 1),
                    ],
                },
            ],
        );

        let shape_b = build_shape(
            "program_shape_b",
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "program_b1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "program_b2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "program_b3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "program_b4", "none", true, 1, 2, 1),
                        entry(3, "program_b5", "none", false, 2, 2, 0),
                    ],
                },
            ],
        );

        let manifold = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold");
        let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");
        let plan = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("plan");
        (manifold, plan)
    }

    #[test]
    fn phase12_synthesizes_stable_program_from_converged_geometry() {
        let (manifold, plan) = program_fixture();
        let program = phase12_synthesize_emergent_cognitive_program(&manifold, &plan).expect("program");

        assert!(program.program_well_formed);
        assert_eq!(program.step_count, plan.operator_count);
        assert!(!program.program_signature.is_empty());
        assert!(!program.program_profile_hash.is_empty());
        assert_eq!(program.manifold_signature, manifold.manifold_signature);
    }

    #[test]
    fn phase12_program_synthesis_is_deterministic() {
        let (manifold, plan) = program_fixture();
        let program_a = phase12_synthesize_emergent_cognitive_program(&manifold, &plan).expect("a");
        let program_b = phase12_synthesize_emergent_cognitive_program(&manifold, &plan).expect("b");

        assert_eq!(program_a, program_b);
    }

    #[test]
    fn phase12_program_synthesis_rejects_signature_mismatch() {
        let (manifold, mut plan) = program_fixture();
        plan.manifold_signature = "tampered-manifold-signature".to_string();

        let err = phase12_synthesize_emergent_cognitive_program(&manifold, &plan)
            .expect_err("mismatch should be rejected");

        assert_eq!(err, "manifold/plan signature mismatch");
    }
}