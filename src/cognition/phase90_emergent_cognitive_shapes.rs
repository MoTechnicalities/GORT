use crate::cognition::phase90_continuity_weighted_fields::Phase90ContinuityWeightedGeometryField;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE90_SHAPE_TELEMETRY: &str = "GORT_PHASE90_EMERGENT_SHAPE_TELEMETRY";

/// Phase 9 Slice 3: Higher-order geometric structure composed from continuity-weighted fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90EmergentCognitiveShape {
    pub episode_span_signature: String,
    pub constituent_field_count: usize,
    pub shape_archetype: String,
    pub aggregate_field_strength: u8,
    pub continuity_envelope_percent: u8,
    pub emergent_radius: u16,
    pub structural_density_percent: u8,
    pub constituent_field_signatures: Vec<String>,
    pub emergent_shape_signature: String,
    pub shape_profile_hash: String,
    pub shape_well_formed: bool,
}

/// Compose a deterministic higher-order shape from ordered continuity-weighted fields.
pub fn phase90_compose_emergent_cognitive_shape(
    fields: &[Phase90ContinuityWeightedGeometryField],
) -> Result<Phase90EmergentCognitiveShape, String> {
    if fields.is_empty() {
        return Err("cannot compose emergent shape from empty field set".to_string());
    }

    let all_fields_well_formed = fields.iter().all(|field| field.field_well_formed);
    let total_strength: u32 = fields.iter().map(|field| field.field_strength as u32).sum();
    let total_radius: u32 = fields.iter().map(|field| field.field_radius as u32).sum();
    let total_contexts: u32 = fields
        .iter()
        .map(|field| field.source_context_count as u32)
        .sum();

    let aggregate_field_strength = (total_strength / fields.len() as u32) as u8;
    let continuity_envelope_percent = fields
        .iter()
        .map(|field| field.field_strength)
        .min()
        .unwrap_or(0);
    let max_radius = fields.iter().map(|field| field.field_radius).max().unwrap_or(0) as u32;
    let emergent_radius = (max_radius + (fields.len() as u32 * 10)).clamp(50, 1024) as u16;

    let density_base = if total_radius == 0 {
        0
    } else {
        ((total_strength * 100) / total_radius).clamp(0, 100)
    };
    let context_boost = total_contexts.min(20);
    let structural_density_percent = (density_base + context_boost).clamp(0, 100) as u8;

    let shape_archetype = phase90_shape_archetype(fields.len(), structural_density_percent);
    let constituent_field_signatures = fields
        .iter()
        .map(|field| field.field_signature.clone())
        .collect::<Vec<_>>();
    let episode_span_signature = fields
        .iter()
        .map(|field| field.episode_id.clone())
        .collect::<Vec<_>>()
        .join("|");

    let emergent_shape_signature = format!(
        "episodes={}|fields={}|archetype={}|strength={}|envelope={}|radius={}|density={}|signatures={}",
        episode_span_signature,
        fields.len(),
        shape_archetype,
        aggregate_field_strength,
        continuity_envelope_percent,
        emergent_radius,
        structural_density_percent,
        constituent_field_signatures.join("||")
    );

    let shape_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        emergent_shape_signature.hash(&mut hasher);
        aggregate_field_strength.hash(&mut hasher);
        continuity_envelope_percent.hash(&mut hasher);
        emergent_radius.hash(&mut hasher);
        hasher.finish()
    });

    let shape_well_formed = all_fields_well_formed
        && aggregate_field_strength > 0
        && continuity_envelope_percent > 0
        && emergent_radius >= 50
        && !shape_archetype.is_empty();

    Ok(Phase90EmergentCognitiveShape {
        episode_span_signature,
        constituent_field_count: fields.len(),
        shape_archetype,
        aggregate_field_strength,
        continuity_envelope_percent,
        emergent_radius,
        structural_density_percent,
        constituent_field_signatures,
        emergent_shape_signature,
        shape_profile_hash,
        shape_well_formed,
    })
}

fn phase90_shape_archetype(field_count: usize, density_percent: u8) -> String {
    match (field_count, density_percent) {
        (1, _) => "singleton".to_string(),
        (2..=3, 0..=34) => "bridge".to_string(),
        (2..=3, _) => "chain".to_string(),
        (4..=6, 0..=49) => "cluster".to_string(),
        (4..=6, _) => "lattice".to_string(),
        (_, 0..=49) => "constellation".to_string(),
        _ => "manifold".to_string(),
    }
}

/// Emit canonical telemetry for Slice 3 emergent shape composition.
pub fn phase90_emit_shape_telemetry(shape: &Phase90EmergentCognitiveShape) -> String {
    let line = format!(
        "episode_span={}:field_count={}:archetype={}:aggregate_strength={}:continuity_envelope={}:radius={}:density={}:shape_signature={}:profile_hash={}:well_formed={}",
        shape.episode_span_signature,
        shape.constituent_field_count,
        shape.shape_archetype,
        shape.aggregate_field_strength,
        shape.continuity_envelope_percent,
        shape.emergent_radius,
        shape.structural_density_percent,
        shape.emergent_shape_signature,
        shape.shape_profile_hash,
        shape.shape_well_formed,
    );
    env::set_var(PHASE90_SHAPE_TELEMETRY, &line);
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

    fn field_from_log(episode_id: &str, log: &Phase70AdjustmentLog) -> Phase90ContinuityWeightedGeometryField {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let trace = phase80_run_multiframe_episode(episode_id, log, &registry).expect("trace");
        let deltas = phase80_integrate_cross_frame_structural_deltas(&trace, log, &registry)
            .expect("deltas");
        let summary = phase80_summarize_episode_structural_integration(&trace, log, &registry)
            .expect("summary");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
        let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
        phase90_form_continuity_weighted_field_from_seed(&seed)
    }

    #[test]
    fn phase90_slice3_composes_shape_from_multiple_fields() {
        let field_a = field_from_log(
            "episode_shape_a",
            &Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "shape_a_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "shape_a_2", "none", true, 1, 2, 1),
                ],
            },
        );
        let field_b = field_from_log(
            "episode_shape_b",
            &Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "shape_b_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "shape_b_2", "none", false, 1, 1, 0),
                    entry(3, "shape_b_3", "none", true, 1, 2, 1),
                ],
            },
        );

        let shape = phase90_compose_emergent_cognitive_shape(&[field_a, field_b]).expect("shape");

        assert_eq!(shape.constituent_field_count, 2);
        assert!(shape.shape_well_formed);
        assert!(!shape.emergent_shape_signature.is_empty());
        assert!(!shape.shape_profile_hash.is_empty());
        assert!(shape.emergent_radius >= 50);
    }

    #[test]
    fn phase90_slice3_shape_archetype_changes_with_composition_scale() {
        let base_field = field_from_log(
            "episode_shape_scale",
            &Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "shape_scale_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "shape_scale_2", "none", true, 1, 2, 1),
                ],
            },
        );

        let bridge_shape = phase90_compose_emergent_cognitive_shape(&[
            base_field.clone(),
            base_field.clone(),
        ])
        .expect("bridge shape");
        let lattice_shape = phase90_compose_emergent_cognitive_shape(&[
            base_field.clone(),
            base_field.clone(),
            base_field.clone(),
            base_field.clone(),
        ])
        .expect("lattice shape");

        assert_ne!(bridge_shape.shape_archetype, lattice_shape.shape_archetype);
    }

    #[test]
    fn phase90_slice3_shape_profile_hash_is_deterministic() {
        let field_a = field_from_log(
            "episode_shape_hash_a",
            &Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "shape_hash_a_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "shape_hash_a_2", "none", false, 1, 1, 0),
                    entry(3, "shape_hash_a_3", "none", true, 1, 2, 1),
                ],
            },
        );
        let field_b = field_from_log(
            "episode_shape_hash_b",
            &Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "shape_hash_b_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "shape_hash_b_2", "none", true, 1, 2, 1),
                ],
            },
        );

        let shape_a = phase90_compose_emergent_cognitive_shape(&[field_a.clone(), field_b.clone()])
            .expect("shape a");
        let shape_b = phase90_compose_emergent_cognitive_shape(&[field_a, field_b]).expect("shape b");

        assert_eq!(shape_a, shape_b);
        assert_eq!(shape_a.shape_profile_hash, shape_b.shape_profile_hash);
    }

    #[test]
    fn phase90_slice3_shape_telemetry_is_canonical() {
        let field = field_from_log(
            "episode_shape_telem",
            &Phase70AdjustmentLog {
                entries: vec![
                    entry(1, "shape_telem_1", "continuity_insensitive", true, 0, 1, 1),
                    entry(2, "shape_telem_2", "none", true, 1, 2, 1),
                ],
            },
        );

        let shape = phase90_compose_emergent_cognitive_shape(&[field.clone(), field]).expect("shape");
        let telemetry_a = phase90_emit_shape_telemetry(&shape);
        let telemetry_b = phase90_emit_shape_telemetry(&shape);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("episode_span="));
        assert!(telemetry_a.contains("archetype="));
        assert!(telemetry_a.contains("aggregate_strength="));
        assert!(telemetry_a.contains("density="));
        assert!(telemetry_a.contains("well_formed="));
    }
}