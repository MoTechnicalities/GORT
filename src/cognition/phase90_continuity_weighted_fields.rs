use crate::cognition::phase90_geometric_cognitive_seed::Phase90GeometricCognitiveSeed;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE90_FIELD_TELEMETRY: &str = "GORT_PHASE90_CONTINUITY_WEIGHTED_FIELD_TELEMETRY";

/// Phase 9 Slice 2: Continuity-weighted influence field formed from a geometric seed.
///
/// An influence field encodes how a seed's geometric meaning propagates through space,
/// weighted by its continuity preservation. Higher continuity weight yields stronger influence.
///
/// Fields capture:
/// - **Field strength** — derived from continuity_weight_percent (0-100 → 0.0-1.0)
/// - **Influence decay** — how influence attenuates with distance (deterministic profile)
/// - **Geometric radius** — spatial extent of field influence
/// - **Field signature** — deterministic encoding for replay verification
/// - **Well-formedness** — validation flag (all constraints satisfied)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90ContinuityWeightedGeometryField {
    /// Episode ID from source seed
    pub episode_id: String,

    /// Continuity weight percent normalized to field strength (0.0 → 1.0)
    pub field_strength: u8,

    /// Influence decay rate (percent per unit distance)
    pub influence_decay_rate: u8,

    /// Maximum geometric radius of field influence
    pub field_radius: u16,

    /// Deterministic field signature encoding geometry and decay
    pub field_signature: String,

    /// Deterministic hash of field profile
    pub field_profile_hash: String,

    /// Number of semantic anchor contexts from source seed
    pub source_context_count: usize,

    /// Total delta magnitude from source seed
    pub source_delta_magnitude: i32,

    /// Whether field satisfies all well-formedness constraints
    pub field_well_formed: bool,
}

/// Phase 9 Slice 2: Form a continuity-weighted influence field from a geometric seed.
///
/// This function:
/// 1. Normalizes the seed's continuity weight to field strength (0-100 → 0-100)
/// 2. Computes deterministic influence decay rate from field strength
/// 3. Determines geometric radius based on source delta magnitude
/// 4. Constructs deterministic field signature
/// 5. Derives field profile hash for replay verification
/// 6. Validates field well-formedness (all constraints)
///
/// Returns a fully deterministic Phase90ContinuityWeightedGeometryField.
pub fn phase90_form_continuity_weighted_field_from_seed(
    seed: &Phase90GeometricCognitiveSeed,
) -> Phase90ContinuityWeightedGeometryField {
    // Field strength directly from seed continuity weight
    let field_strength = seed.continuity_weight_percent;

    // Compute influence decay rate from field strength
    // Higher strength → lower decay (influence persists further)
    // decay = 100 - (strength * 0.5), clamped to [10, 90]
    let decay_base = 100i32 - ((field_strength as i32) * 50 / 100);
    let influence_decay_rate = (decay_base.clamp(10, 90)) as u8;

    // Determine field radius from source delta magnitude
    // Each delta unit contributes ~10 units of radius, clamped to [50, 500]
    let radius_from_delta = ((seed.total_delta_magnitude.abs() as u32) * 10).clamp(50, 500) as u16;
    let field_radius = radius_from_delta;

    // Construct deterministic field signature
    let field_signature = format!(
        "episode={}|strength={}|decay_rate={}|radius={}|contexts={}|delta_magnitude={}",
        seed.episode_id,
        field_strength,
        influence_decay_rate,
        field_radius,
        seed.semantic_anchor_contexts.len(),
        seed.total_delta_magnitude
    );

    // Compute deterministic field profile hash
    let field_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        field_signature.hash(&mut hasher);
        seed.seed_content_hash.hash(&mut hasher);
        field_strength.hash(&mut hasher);
        hasher.finish()
    });

    // Determine well-formedness:
    // - Seed must be geometrically well-formed
    // - Field strength must be non-zero if seed has deltas
    // - Decay rate must be in valid range [10, 90]
    // - Radius must be in valid range [50, 500]
    let has_semantic_anchors = !seed.semantic_anchor_contexts.is_empty();
    let field_well_formed = seed.geometry_well_formed
        && has_semantic_anchors
        && field_strength > 0
        && (10..=90).contains(&influence_decay_rate)
        && (50..=500).contains(&field_radius);

    Phase90ContinuityWeightedGeometryField {
        episode_id: seed.episode_id.clone(),
        field_strength,
        influence_decay_rate,
        field_radius,
        field_signature,
        field_profile_hash,
        source_context_count: seed.semantic_anchor_contexts.len(),
        source_delta_magnitude: seed.total_delta_magnitude,
        field_well_formed,
    }
}

/// Emit Phase 9 Slice 2 field formation telemetry to environment variable.
///
/// Canonical format: colon-delimited, no floating-point values, deterministic field order.
pub fn phase90_emit_field_telemetry(
    field: &Phase90ContinuityWeightedGeometryField,
) -> String {
    let line = format!(
        "episode_id={}:field_strength={}:decay_rate={}:radius={}:contexts={}:delta_magnitude={}:field_signature={}:profile_hash={}:well_formed={}",
        field.episode_id,
        field.field_strength,
        field.influence_decay_rate,
        field.field_radius,
        field.source_context_count,
        field.source_delta_magnitude,
        field.field_signature,
        field.field_profile_hash,
        field.field_well_formed,
    );
    env::set_var(PHASE90_FIELD_TELEMETRY, &line);
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

    #[test]
    fn phase90_slice2_forms_field_from_seed() {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let log = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_p9s2_a", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_p9s2_b", "none", true, 1, 2, 1),
                entry(3, "holdout_p9s2_c", "none", true, 2, 2, 0),
            ],
        };

        let trace = phase80_run_multiframe_episode("episode_p9s2_field", &log, &registry)
            .expect("trace");
        let deltas =
            phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
        let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
            .expect("summary");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
        let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);

        // Phase 9 Slice 2: form field
        let field = phase90_form_continuity_weighted_field_from_seed(&seed);

        assert_eq!(field.episode_id, "episode_p9s2_field");
        assert_eq!(field.field_strength, seed.continuity_weight_percent);
        assert!((10..=90).contains(&field.influence_decay_rate));
        assert!((50..=500).contains(&field.field_radius));
        assert!(!field.field_signature.is_empty());
        assert!(!field.field_profile_hash.is_empty());
        assert_eq!(field.source_context_count, seed.semantic_anchor_contexts.len());
    }

    #[test]
    fn phase90_slice2_field_strength_correlates_with_continuity_weight() {
        let registry = Phase70StructuralParameterRegistry::canonical();

        // Test case 1: high continuity
        let log_high = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_high_a", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_high_b", "none", true, 1, 2, 1),
                entry(3, "holdout_high_c", "none", true, 2, 2, 0),
            ],
        };

        let trace_high = phase80_run_multiframe_episode("episode_high_continuity", &log_high, &registry)
            .expect("trace");
        let deltas_high = phase80_integrate_cross_frame_structural_deltas(&trace_high, &log_high, &registry)
            .expect("deltas");
        let summary_high = phase80_summarize_episode_structural_integration(&trace_high, &log_high, &registry)
            .expect("summary");
        let hook_high = phase80_build_phase9_integration_hook(&summary_high, &deltas_high);
        let seed_high = phase90_form_geometry_seed_from_integration_hook(&hook_high, &summary_high, &deltas_high);
        let field_high = phase90_form_continuity_weighted_field_from_seed(&seed_high);

        // Test case 2: lower continuity
        let log_low = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_low_a", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_low_b", "none", false, 1, 1, 0),
                entry(3, "holdout_low_c", "none", true, 1, 2, 1),
            ],
        };

        let trace_low = phase80_run_multiframe_episode("episode_low_continuity", &log_low, &registry)
            .expect("trace");
        let deltas_low = phase80_integrate_cross_frame_structural_deltas(&trace_low, &log_low, &registry)
            .expect("deltas");
        let summary_low = phase80_summarize_episode_structural_integration(&trace_low, &log_low, &registry)
            .expect("summary");
        let hook_low = phase80_build_phase9_integration_hook(&summary_low, &deltas_low);
        let seed_low = phase90_form_geometry_seed_from_integration_hook(&hook_low, &summary_low, &deltas_low);
        let field_low = phase90_form_continuity_weighted_field_from_seed(&seed_low);

        // Field strength should correlate with continuity
        assert_eq!(field_high.field_strength, seed_high.continuity_weight_percent);
        assert_eq!(field_low.field_strength, seed_low.continuity_weight_percent);
    }

    #[test]
    fn phase90_slice2_field_decay_rate_is_deterministic() {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let log = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_p9s2_decay_a", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_p9s2_decay_b", "none", true, 1, 2, 1),
            ],
        };

        let mut decay_rates = Vec::new();

        for _ in 0..10 {
            let trace = phase80_run_multiframe_episode("episode_p9s2_decay", &log, &registry)
                .expect("trace");
            let deltas = phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry)
                .expect("deltas");
            let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
                .expect("summary");
            let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
            let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
            let field = phase90_form_continuity_weighted_field_from_seed(&seed);

            decay_rates.push(field.influence_decay_rate);
        }

        let first_rate = decay_rates[0];
        for rate in decay_rates.iter() {
            assert_eq!(
                *rate, first_rate,
                "decay rate must be deterministic across replays"
            );
        }
    }

    #[test]
    fn phase90_slice2_field_profile_hash_is_deterministic() {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let log = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_p9s2_hash_a", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_p9s2_hash_b", "none", true, 1, 2, 1),
                entry(3, "holdout_p9s2_hash_c", "none", false, 2, 2, 0),
                entry(4, "holdout_p9s2_hash_d", "continuity_insensitive", true, 2, 2, 0),
            ],
        };

        let trace_a = phase80_run_multiframe_episode("episode_p9s2_hash", &log, &registry)
            .expect("trace_a");
        let deltas_a = phase80_integrate_cross_frame_structural_deltas(&trace_a, &log, &registry)
            .expect("deltas_a");
        let summary_a = phase80_summarize_episode_structural_integration(&trace_a, &log, &registry)
            .expect("summary_a");
        let hook_a = phase80_build_phase9_integration_hook(&summary_a, &deltas_a);
        let seed_a = phase90_form_geometry_seed_from_integration_hook(&hook_a, &summary_a, &deltas_a);
        let field_a = phase90_form_continuity_weighted_field_from_seed(&seed_a);

        // Replay
        let trace_b = phase80_run_multiframe_episode("episode_p9s2_hash", &log, &registry)
            .expect("trace_b");
        let deltas_b = phase80_integrate_cross_frame_structural_deltas(&trace_b, &log, &registry)
            .expect("deltas_b");
        let summary_b = phase80_summarize_episode_structural_integration(&trace_b, &log, &registry)
            .expect("summary_b");
        let hook_b = phase80_build_phase9_integration_hook(&summary_b, &deltas_b);
        let seed_b = phase90_form_geometry_seed_from_integration_hook(&hook_b, &summary_b, &deltas_b);
        let field_b = phase90_form_continuity_weighted_field_from_seed(&seed_b);

        assert_eq!(field_a, field_b);
        assert_eq!(
            field_a.field_profile_hash, field_b.field_profile_hash,
            "profile hash must be identical"
        );
    }

    #[test]
    fn phase90_slice2_field_telemetry_is_canonical() {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let log = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_p9s2_telem_a", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_p9s2_telem_b", "none", true, 1, 2, 1),
            ],
        };

        let trace = phase80_run_multiframe_episode("episode_p9s2_telemetry", &log, &registry)
            .expect("trace");
        let deltas = phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry)
            .expect("deltas");
        let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
            .expect("summary");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
        let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
        let field = phase90_form_continuity_weighted_field_from_seed(&seed);

        let telemetry_1 = phase90_emit_field_telemetry(&field);
        let telemetry_2 = phase90_emit_field_telemetry(&field);

        assert_eq!(telemetry_1, telemetry_2);
        assert!(telemetry_1.contains("episode_id="));
        assert!(telemetry_1.contains("field_strength="));
        assert!(telemetry_1.contains("decay_rate="));
        assert!(telemetry_1.contains("radius="));
        assert!(telemetry_1.contains("well_formed="));
    }
}
