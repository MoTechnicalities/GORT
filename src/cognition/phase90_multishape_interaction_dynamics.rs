use crate::cognition::phase90_cognitive_manifolds::Phase90CognitiveManifold;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE90_DYNAMICS_TELEMETRY: &str = "GORT_PHASE90_MULTISHAPE_DYNAMICS_TELEMETRY";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90ShapeInteractionEvent {
    pub from_shape_index: usize,
    pub to_shape_index: usize,
    pub adjacency_kind: String,
    pub interaction_mode: String,
    pub resonance_score_percent: u8,
    pub continuity_flow_percent: u8,
    pub operator_pressure_percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90MultiShapeInteractionDynamics {
    pub manifold_signature: String,
    pub interaction_count: usize,
    pub dominant_interaction_mode: String,
    pub aggregate_resonance_percent: u8,
    pub aggregate_operator_pressure_percent: u8,
    pub interaction_events: Vec<Phase90ShapeInteractionEvent>,
    pub dynamics_signature: String,
    pub dynamics_profile_hash: String,
    pub dynamics_well_formed: bool,
}

pub fn phase90_compute_multishape_interaction_dynamics(
    manifold: &Phase90CognitiveManifold,
) -> Result<Phase90MultiShapeInteractionDynamics, String> {
    if !manifold.manifold_well_formed {
        return Err("cannot compute interaction dynamics from malformed manifold".to_string());
    }

    if manifold.adjacency_edges.is_empty() {
        return Err("cannot compute interaction dynamics without manifold edges".to_string());
    }

    let interaction_events = manifold
        .adjacency_edges
        .iter()
        .map(|edge| {
            let interaction_mode = match edge.adjacency_kind.as_str() {
                "resonant_archetype" => "resonant_exchange",
                "proximal_overlap" => "gradient_blend",
                "continuity_bridge" => "continuity_transfer",
                _ => "boundary_probe",
            }
            .to_string();

            let resonance_score_percent = ((edge.continuity_bridge_percent as u16
                + (if edge.shared_archetype { 20 } else { 0 })
                + (100u16.saturating_sub(edge.radius_gap.min(100))))
                / 3)
                .clamp(0, 100) as u8;

            let continuity_flow_percent = edge.continuity_bridge_percent;
            let operator_pressure_percent = ((resonance_score_percent as u16
                + continuity_flow_percent as u16
                + manifold.manifold_embedding_weight_percent as u16)
                / 3)
                .clamp(0, 100) as u8;

            Phase90ShapeInteractionEvent {
                from_shape_index: edge.from_shape_index,
                to_shape_index: edge.to_shape_index,
                adjacency_kind: edge.adjacency_kind.clone(),
                interaction_mode,
                resonance_score_percent,
                continuity_flow_percent,
                operator_pressure_percent,
            }
        })
        .collect::<Vec<_>>();

    let interaction_count = interaction_events.len();
    let aggregate_resonance_percent = (interaction_events
        .iter()
        .map(|event| event.resonance_score_percent as u32)
        .sum::<u32>()
        / interaction_count as u32) as u8;
    let aggregate_operator_pressure_percent = (interaction_events
        .iter()
        .map(|event| event.operator_pressure_percent as u32)
        .sum::<u32>()
        / interaction_count as u32) as u8;

    let dominant_interaction_mode = interaction_events
        .iter()
        .map(|event| event.interaction_mode.clone())
        .max_by_key(|mode| {
            interaction_events
                .iter()
                .filter(|event| event.interaction_mode == *mode)
                .count()
        })
        .unwrap_or_else(|| "none".to_string());

    let dynamics_signature = format!(
        "manifold={}|count={}|dominant={}|resonance={}|pressure={}|events={}",
        manifold.manifold_signature,
        interaction_count,
        dominant_interaction_mode,
        aggregate_resonance_percent,
        aggregate_operator_pressure_percent,
        interaction_events
            .iter()
            .map(|event| format!(
                "{}>{}:{}:{}:{}",
                event.from_shape_index,
                event.to_shape_index,
                event.interaction_mode,
                event.resonance_score_percent,
                event.operator_pressure_percent,
            ))
            .collect::<Vec<_>>()
            .join("|"),
    );

    let dynamics_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        dynamics_signature.hash(&mut hasher);
        aggregate_resonance_percent.hash(&mut hasher);
        aggregate_operator_pressure_percent.hash(&mut hasher);
        hasher.finish()
    });

    let dynamics_well_formed = interaction_count > 0
        && aggregate_operator_pressure_percent > 0
        && aggregate_resonance_percent > 0
        && dominant_interaction_mode != "none";

    Ok(Phase90MultiShapeInteractionDynamics {
        manifold_signature: manifold.manifold_signature.clone(),
        interaction_count,
        dominant_interaction_mode,
        aggregate_resonance_percent,
        aggregate_operator_pressure_percent,
        interaction_events,
        dynamics_signature,
        dynamics_profile_hash,
        dynamics_well_formed,
    })
}

pub fn phase90_emit_interaction_dynamics_telemetry(
    dynamics: &Phase90MultiShapeInteractionDynamics,
) -> String {
    let line = format!(
        "manifold_signature={}:interaction_count={}:dominant_mode={}:aggregate_resonance={}:aggregate_pressure={}:dynamics_signature={}:profile_hash={}:well_formed={}",
        dynamics.manifold_signature,
        dynamics.interaction_count,
        dynamics.dominant_interaction_mode,
        dynamics.aggregate_resonance_percent,
        dynamics.aggregate_operator_pressure_percent,
        dynamics.dynamics_signature,
        dynamics.dynamics_profile_hash,
        dynamics.dynamics_well_formed,
    );
    env::set_var(PHASE90_DYNAMICS_TELEMETRY, &line);
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

    fn manifold_fixture() -> Phase90CognitiveManifold {
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
            "dynamics_shape_a",
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "dyn_a1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "dyn_a2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "dyn_a3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "dyn_a4", "none", false, 1, 1, 0),
                        entry(3, "dyn_a5", "none", true, 1, 2, 1),
                    ],
                },
            ],
        );

        let shape_b = build_shape(
            "dynamics_shape_b",
            vec![
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "dyn_b1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "dyn_b2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "dyn_b3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "dyn_b4", "none", true, 1, 2, 1),
                    ],
                },
            ],
        );

        phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold")
    }

    #[test]
    fn phase90_slice5_computes_interaction_dynamics_from_manifold() {
        let manifold = manifold_fixture();
        let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");

        assert!(dynamics.dynamics_well_formed);
        assert!(dynamics.interaction_count > 0);
        assert!(!dynamics.dynamics_signature.is_empty());
        assert!(!dynamics.dynamics_profile_hash.is_empty());
    }

    #[test]
    fn phase90_slice5_interaction_modes_are_deterministic() {
        let manifold = manifold_fixture();
        let dynamics_a = phase90_compute_multishape_interaction_dynamics(&manifold).expect("a");
        let dynamics_b = phase90_compute_multishape_interaction_dynamics(&manifold).expect("b");

        assert_eq!(dynamics_a.interaction_events, dynamics_b.interaction_events);
        assert_eq!(dynamics_a.dominant_interaction_mode, dynamics_b.dominant_interaction_mode);
    }

    #[test]
    fn phase90_slice5_dynamics_profile_hash_is_deterministic() {
        let manifold = manifold_fixture();
        let dynamics_a = phase90_compute_multishape_interaction_dynamics(&manifold).expect("a");
        let dynamics_b = phase90_compute_multishape_interaction_dynamics(&manifold).expect("b");

        assert_eq!(dynamics_a, dynamics_b);
        assert_eq!(dynamics_a.dynamics_profile_hash, dynamics_b.dynamics_profile_hash);
    }

    #[test]
    fn phase90_slice5_dynamics_telemetry_is_canonical() {
        let manifold = manifold_fixture();
        let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");
        let telemetry_a = phase90_emit_interaction_dynamics_telemetry(&dynamics);
        let telemetry_b = phase90_emit_interaction_dynamics_telemetry(&dynamics);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("manifold_signature="));
        assert!(telemetry_a.contains("interaction_count="));
        assert!(telemetry_a.contains("dominant_mode="));
        assert!(telemetry_a.contains("aggregate_pressure="));
        assert!(telemetry_a.contains("well_formed="));
    }
}