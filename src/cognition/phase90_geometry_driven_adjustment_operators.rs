use crate::cognition::phase90_cognitive_manifolds::Phase90CognitiveManifold;
use crate::cognition::phase90_multishape_interaction_dynamics::Phase90MultiShapeInteractionDynamics;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE90_OPERATOR_TELEMETRY: &str = "GORT_PHASE90_GEOMETRY_OPERATOR_TELEMETRY";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90GeometryDrivenAdjustmentOperator {
    pub operator_id: String,
    pub target_shape_index: usize,
    pub source_shape_index: usize,
    pub operator_kind: String,
    pub adjustment_pressure_percent: u8,
    pub continuity_bias_percent: u8,
    pub manifold_alignment_percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90GeometryDrivenAdjustmentPlan {
    pub manifold_signature: String,
    pub operator_count: usize,
    pub dominant_operator_kind: String,
    pub aggregate_adjustment_pressure_percent: u8,
    pub operators: Vec<Phase90GeometryDrivenAdjustmentOperator>,
    pub operator_plan_signature: String,
    pub operator_plan_hash: String,
    pub operator_plan_well_formed: bool,
}

pub fn phase90_build_geometry_driven_adjustment_plan(
    manifold: &Phase90CognitiveManifold,
    dynamics: &Phase90MultiShapeInteractionDynamics,
) -> Result<Phase90GeometryDrivenAdjustmentPlan, String> {
    if !manifold.manifold_well_formed {
        return Err("cannot build adjustment plan from malformed manifold".to_string());
    }
    if !dynamics.dynamics_well_formed {
        return Err("cannot build adjustment plan from malformed interaction dynamics".to_string());
    }
    if manifold.manifold_signature != dynamics.manifold_signature {
        return Err("manifold/dynamics signature mismatch".to_string());
    }

    let operators = dynamics
        .interaction_events
        .iter()
        .enumerate()
        .map(|(index, event)| {
            let operator_kind = match event.interaction_mode.as_str() {
                "resonant_exchange" => "resonance_reweight",
                "gradient_blend" => "gradient_smooth",
                "continuity_transfer" => "continuity_lift",
                _ => "boundary_probe",
            }
            .to_string();

            let adjustment_pressure_percent = ((event.operator_pressure_percent as u16
                + dynamics.aggregate_operator_pressure_percent as u16)
                / 2)
                .clamp(0, 100) as u8;
            let continuity_bias_percent = ((event.continuity_flow_percent as u16
                + manifold.manifold_embedding_weight_percent as u16)
                / 2)
                .clamp(0, 100) as u8;
            let manifold_alignment_percent = ((event.resonance_score_percent as u16
                + manifold.manifold_embedding_weight_percent as u16
                + manifold.shape_count.min(100) as u16)
                / 3)
                .clamp(0, 100) as u8;

            Phase90GeometryDrivenAdjustmentOperator {
                operator_id: format!(
                    "{}:{}:{}:{}",
                    manifold.manifold_profile_hash,
                    index,
                    event.from_shape_index,
                    event.to_shape_index,
                ),
                target_shape_index: event.to_shape_index,
                source_shape_index: event.from_shape_index,
                operator_kind,
                adjustment_pressure_percent,
                continuity_bias_percent,
                manifold_alignment_percent,
            }
        })
        .collect::<Vec<_>>();

    let operator_count = operators.len();
    if operator_count == 0 {
        return Err("cannot build adjustment plan without interaction events".to_string());
    }

    let aggregate_adjustment_pressure_percent = (operators
        .iter()
        .map(|operator| operator.adjustment_pressure_percent as u32)
        .sum::<u32>()
        / operator_count as u32) as u8;

    let mut best_kind = String::new();
    let mut best_count = 0usize;
    for operator in &operators {
        let count = operators
            .iter()
            .filter(|candidate| candidate.operator_kind == operator.operator_kind)
            .count();
        if count > best_count || (count == best_count && operator.operator_kind < best_kind) {
            best_count = count;
            best_kind = operator.operator_kind.clone();
        }
    }
    let dominant_operator_kind = best_kind;

    let operator_plan_signature = format!(
        "manifold={}|count={}|dominant={}|aggregate_pressure={}|operators={}",
        manifold.manifold_signature,
        operator_count,
        dominant_operator_kind,
        aggregate_adjustment_pressure_percent,
        operators
            .iter()
            .map(|operator| format!(
                "{}:{}:{}:{}:{}",
                operator.source_shape_index,
                operator.target_shape_index,
                operator.operator_kind,
                operator.adjustment_pressure_percent,
                operator.continuity_bias_percent,
            ))
            .collect::<Vec<_>>()
            .join("|"),
    );

    let operator_plan_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        operator_plan_signature.hash(&mut hasher);
        aggregate_adjustment_pressure_percent.hash(&mut hasher);
        dominant_operator_kind.hash(&mut hasher);
        hasher.finish()
    });

    let operator_plan_well_formed = aggregate_adjustment_pressure_percent > 0
        && !dominant_operator_kind.is_empty()
        && operators.iter().all(|operator| operator.manifold_alignment_percent > 0);

    Ok(Phase90GeometryDrivenAdjustmentPlan {
        manifold_signature: manifold.manifold_signature.clone(),
        operator_count,
        dominant_operator_kind,
        aggregate_adjustment_pressure_percent,
        operators,
        operator_plan_signature,
        operator_plan_hash,
        operator_plan_well_formed,
    })
}

pub fn phase90_emit_geometry_operator_telemetry(
    plan: &Phase90GeometryDrivenAdjustmentPlan,
) -> String {
    let line = format!(
        "manifold_signature={}:operator_count={}:dominant_kind={}:aggregate_pressure={}:plan_signature={}:plan_hash={}:well_formed={}",
        plan.manifold_signature,
        plan.operator_count,
        plan.dominant_operator_kind,
        plan.aggregate_adjustment_pressure_percent,
        plan.operator_plan_signature,
        plan.operator_plan_hash,
        plan.operator_plan_well_formed,
    );
    env::set_var(PHASE90_OPERATOR_TELEMETRY, &line);
    line
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

    fn plan_fixture() -> (Phase90CognitiveManifold, Phase90MultiShapeInteractionDynamics) {
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
            "ops_shape_a",
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "ops_a1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "ops_a2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "ops_a3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "ops_a4", "none", false, 1, 1, 0),
                        entry(3, "ops_a5", "none", true, 1, 2, 1),
                    ],
                },
            ],
        );
        let shape_b = build_shape(
            "ops_shape_b",
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "ops_b1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "ops_b2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "ops_b3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "ops_b4", "none", true, 1, 2, 1),
                    ],
                },
            ],
        );

        let manifold = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold");
        let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");
        (manifold, dynamics)
    }

    #[test]
    fn phase90_slice6_builds_adjustment_plan_from_manifold_and_dynamics() {
        let (manifold, dynamics) = plan_fixture();
        let plan = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("plan");

        assert!(plan.operator_plan_well_formed);
        assert!(plan.operator_count > 0);
        assert!(!plan.operator_plan_signature.is_empty());
        assert!(!plan.operator_plan_hash.is_empty());
    }

    #[test]
    fn phase90_slice6_operator_kinds_are_deterministic() {
        let (manifold, dynamics) = plan_fixture();
        let plan_a = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("a");
        let plan_b = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("b");

        assert_eq!(plan_a.operators, plan_b.operators);
        assert_eq!(plan_a.dominant_operator_kind, plan_b.dominant_operator_kind);
    }

    #[test]
    fn phase90_slice6_operator_plan_hash_is_deterministic() {
        let (manifold, dynamics) = plan_fixture();
        let plan_a = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("a");
        let plan_b = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("b");

        assert_eq!(plan_a, plan_b);
        assert_eq!(plan_a.operator_plan_hash, plan_b.operator_plan_hash);
    }

    #[test]
    fn phase90_slice6_operator_telemetry_is_canonical() {
        let (manifold, dynamics) = plan_fixture();
        let plan = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("plan");
        let telemetry_a = phase90_emit_geometry_operator_telemetry(&plan);
        let telemetry_b = phase90_emit_geometry_operator_telemetry(&plan);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("manifold_signature="));
        assert!(telemetry_a.contains("operator_count="));
        assert!(telemetry_a.contains("dominant_kind="));
        assert!(telemetry_a.contains("aggregate_pressure="));
        assert!(telemetry_a.contains("well_formed="));
    }
}