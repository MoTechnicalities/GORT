use crate::cognition::phase90_emergent_cognitive_shapes::Phase90EmergentCognitiveShape;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE90_MANIFOLD_TELEMETRY: &str = "GORT_PHASE90_COGNITIVE_MANIFOLD_TELEMETRY";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90ShapeAdjacencyEdge {
    pub from_shape_index: usize,
    pub to_shape_index: usize,
    pub adjacency_kind: String,
    pub continuity_bridge_percent: u8,
    pub radius_gap: u16,
    pub shared_archetype: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90CognitiveManifold {
    pub shape_span_signature: String,
    pub shape_count: usize,
    pub topology_edge_count: usize,
    pub manifold_archetype: String,
    pub manifold_embedding_weight_percent: u8,
    pub manifold_radius: u16,
    pub adjacency_edges: Vec<Phase90ShapeAdjacencyEdge>,
    pub topology_graph_signature: String,
    pub manifold_signature: String,
    pub manifold_profile_hash: String,
    pub manifold_well_formed: bool,
}

pub fn phase90_form_cognitive_manifold(
    shapes: &[Phase90EmergentCognitiveShape],
) -> Result<Phase90CognitiveManifold, String> {
    if shapes.is_empty() {
        return Err("cannot form cognitive manifold from empty shape set".to_string());
    }

    let all_shapes_well_formed = shapes.iter().all(|shape| shape.shape_well_formed);
    let shape_count = shapes.len();
    let shape_span_signature = shapes
        .iter()
        .map(|shape| shape.episode_span_signature.clone())
        .collect::<Vec<_>>()
        .join("||");

    let manifold_embedding_weight_percent = shapes
        .iter()
        .map(|shape| shape.continuity_envelope_percent)
        .min()
        .unwrap_or(0);
    let max_radius = shapes
        .iter()
        .map(|shape| shape.emergent_radius)
        .max()
        .unwrap_or(0) as u32;
    let manifold_radius = (max_radius + (shape_count as u32 * 25)).clamp(64, 2048) as u16;

    let structural_density_average = (shapes
        .iter()
        .map(|shape| shape.structural_density_percent as u32)
        .sum::<u32>()
        / shape_count as u32) as u8;
    let manifold_archetype = phase90_manifold_archetype(shape_count, structural_density_average);

    let adjacency_edges = phase90_build_shape_adjacency_edges(shapes);
    let topology_edge_count = adjacency_edges.len();
    let topology_graph_signature = adjacency_edges
        .iter()
        .map(|edge| {
            format!(
                "{}>{}:{}:{}:{}:{}",
                edge.from_shape_index,
                edge.to_shape_index,
                edge.adjacency_kind,
                edge.continuity_bridge_percent,
                edge.radius_gap,
                edge.shared_archetype,
            )
        })
        .collect::<Vec<_>>()
        .join("|");

    let manifold_signature = format!(
        "shapes={}|edges={}|archetype={}|embedding={}|radius={}|graph={}",
        shape_span_signature,
        topology_edge_count,
        manifold_archetype,
        manifold_embedding_weight_percent,
        manifold_radius,
        topology_graph_signature,
    );

    let manifold_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        manifold_signature.hash(&mut hasher);
        manifold_embedding_weight_percent.hash(&mut hasher);
        manifold_radius.hash(&mut hasher);
        topology_edge_count.hash(&mut hasher);
        hasher.finish()
    });

    let manifold_well_formed = all_shapes_well_formed
        && manifold_embedding_weight_percent > 0
        && manifold_radius >= 64
        && (!adjacency_edges.is_empty() || shape_count == 1)
        && !manifold_archetype.is_empty();

    Ok(Phase90CognitiveManifold {
        shape_span_signature,
        shape_count,
        topology_edge_count,
        manifold_archetype,
        manifold_embedding_weight_percent,
        manifold_radius,
        adjacency_edges,
        topology_graph_signature,
        manifold_signature,
        manifold_profile_hash,
        manifold_well_formed,
    })
}

fn phase90_build_shape_adjacency_edges(
    shapes: &[Phase90EmergentCognitiveShape],
) -> Vec<Phase90ShapeAdjacencyEdge> {
    let mut edges = Vec::new();

    for left_index in 0..shapes.len() {
        for right_index in (left_index + 1)..shapes.len() {
            let left = &shapes[left_index];
            let right = &shapes[right_index];
            let continuity_bridge_percent = left
                .continuity_envelope_percent
                .min(right.continuity_envelope_percent);
            let radius_gap = left.emergent_radius.abs_diff(right.emergent_radius);
            let shared_archetype = left.shape_archetype == right.shape_archetype;

            let adjacency_kind = if shared_archetype && continuity_bridge_percent >= 60 {
                Some("resonant_archetype")
            } else if radius_gap <= 64 && continuity_bridge_percent >= 40 {
                Some("proximal_overlap")
            } else if continuity_bridge_percent >= 75 {
                Some("continuity_bridge")
            } else {
                None
            };

            if let Some(adjacency_kind) = adjacency_kind {
                edges.push(Phase90ShapeAdjacencyEdge {
                    from_shape_index: left_index,
                    to_shape_index: right_index,
                    adjacency_kind: adjacency_kind.to_string(),
                    continuity_bridge_percent,
                    radius_gap,
                    shared_archetype,
                });
            }
        }
    }

    edges
}

fn phase90_manifold_archetype(shape_count: usize, density_average: u8) -> String {
    match (shape_count, density_average) {
        (1, _) => "local_patch".to_string(),
        (2..=3, 0..=49) => "corridor".to_string(),
        (2..=3, _) => "braid".to_string(),
        (4..=6, 0..=59) => "mesh".to_string(),
        (4..=6, _) => "lattice_graph".to_string(),
        (_, 0..=59) => "atlas".to_string(),
        _ => "topology_graph".to_string(),
    }
}

pub fn phase90_emit_manifold_telemetry(manifold: &Phase90CognitiveManifold) -> String {
    let line = format!(
        "shape_span={}:shape_count={}:edge_count={}:archetype={}:embedding={}:radius={}:graph_signature={}:manifold_signature={}:profile_hash={}:well_formed={}",
        manifold.shape_span_signature,
        manifold.shape_count,
        manifold.topology_edge_count,
        manifold.manifold_archetype,
        manifold.manifold_embedding_weight_percent,
        manifold.manifold_radius,
        manifold.topology_graph_signature,
        manifold.manifold_signature,
        manifold.manifold_profile_hash,
        manifold.manifold_well_formed,
    );
    env::set_var(PHASE90_MANIFOLD_TELEMETRY, &line);
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

    fn shape_from_log_sets(
        episode_prefix: &str,
        logs: &[Phase70AdjustmentLog],
    ) -> Phase90EmergentCognitiveShape {
        let registry = Phase70StructuralParameterRegistry::canonical();
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
    }

    #[test]
    fn phase90_slice4_forms_manifold_from_shapes() {
        let shape_a = shape_from_log_sets(
            "manifold_a",
            &[
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "m_a_1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "m_a_2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "m_a_3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "m_a_4", "none", false, 1, 1, 0),
                        entry(3, "m_a_5", "none", true, 1, 2, 1),
                    ],
                },
            ],
        );
        let shape_b = shape_from_log_sets(
            "manifold_b",
            &[
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "m_b_1", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "m_b_2", "none", true, 1, 2, 1),
                    ],
                },
                Phase70AdjustmentLog {
                    entries: vec![
                        entry(1, "m_b_3", "continuity_insensitive", true, 0, 1, 1),
                        entry(2, "m_b_4", "none", true, 1, 2, 1),
                    ],
                },
            ],
        );

        let manifold = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold");

        assert_eq!(manifold.shape_count, 2);
        assert!(manifold.manifold_well_formed);
        assert!(!manifold.manifold_signature.is_empty());
        assert!(!manifold.manifold_profile_hash.is_empty());
        assert!(manifold.manifold_radius >= 64);
    }

    #[test]
    fn phase90_slice4_adjacency_rules_are_deterministic() {
        let shape_a = shape_from_log_sets(
            "adj_a",
            &[Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "adj_a_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "adj_a_2", "none", true, 1, 2, 1),
                ],
            }],
        );
        let shape_b = shape_from_log_sets(
            "adj_b",
            &[Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "adj_b_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "adj_b_2", "none", true, 1, 2, 1),
                ],
            }],
        );

        let manifold_a = phase90_form_cognitive_manifold(&[shape_a.clone(), shape_b.clone()])
            .expect("manifold a");
        let manifold_b = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold b");

        assert_eq!(manifold_a.adjacency_edges, manifold_b.adjacency_edges);
    }

    #[test]
    fn phase90_slice4_manifold_profile_hash_is_deterministic() {
        let shape_a = shape_from_log_sets(
            "hash_a",
            &[Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "hash_a_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "hash_a_2", "none", false, 1, 1, 0),
                    entry(3, "hash_a_3", "none", true, 1, 2, 1),
                ],
            }],
        );
        let shape_b = shape_from_log_sets(
            "hash_b",
            &[Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "hash_b_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "hash_b_2", "none", true, 1, 2, 1),
                ],
            }],
        );

        let manifold_a = phase90_form_cognitive_manifold(&[shape_a.clone(), shape_b.clone()])
            .expect("manifold a");
        let manifold_b = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold b");

        assert_eq!(manifold_a, manifold_b);
        assert_eq!(manifold_a.manifold_profile_hash, manifold_b.manifold_profile_hash);
    }

    #[test]
    fn phase90_slice4_manifold_telemetry_is_canonical() {
        let shape_a = shape_from_log_sets(
            "telem_a",
            &[Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "telem_a_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "telem_a_2", "none", true, 1, 2, 1),
                ],
            }],
        );
        let shape_b = shape_from_log_sets(
            "telem_b",
            &[Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "telem_b_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "telem_b_2", "none", true, 1, 2, 1),
                ],
            }],
        );

        let manifold = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold");
        let telemetry_a = phase90_emit_manifold_telemetry(&manifold);
        let telemetry_b = phase90_emit_manifold_telemetry(&manifold);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("shape_span="));
        assert!(telemetry_a.contains("edge_count="));
        assert!(telemetry_a.contains("archetype="));
        assert!(telemetry_a.contains("embedding="));
        assert!(telemetry_a.contains("well_formed="));
    }
}