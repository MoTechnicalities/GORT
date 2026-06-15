use crate::cognition::phase62_structural_experiment::{
    Phase80CrossFrameStructuralDelta, Phase80EpisodeStructuralSummary, Phase80Phase9IntegrationHook,
};
use serde::{Deserialize, Serialize};
use std::env;

const PHASE90_SEED_FORMATION_TELEMETRY: &str = "GORT_PHASE90_SEED_FORMATION_TELEMETRY";

/// Phase 9 Slice 1: Geometric cognitive seed formed from episode integration data.
///
/// A seed encodes the geometric structure of meaning propagated across an episode.
/// It captures:
/// - **Geometry signature**: deterministic hash of structural deltas and continuity
/// - **Semantic anchor points**: contexts that propagated across frames
/// - **Continuity weight**: influence of continuity preservation on geometry
/// - **Delta magnitude**: total structural change encoded in the seed
/// - **Replay stability**: deterministic seed signature across infinite replays
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase90GeometricCognitiveSeed {
    /// Episode that generated this seed (from Phase 8 integration)
    pub episode_id: String,

    /// Deterministic geometry signature (encodes full structural shape)
    pub geometry_signature: String,

    /// Semantic anchor points carried through episode
    pub semantic_anchor_contexts: Vec<String>,

    /// Continuity weight (0-100 percent) applied to geometry formation
    pub continuity_weight_percent: u8,

    /// Total magnitude of structural change in this seed
    pub total_delta_magnitude: i32,

    /// Number of cross-frame transitions encoded in seed
    pub transition_count: usize,

    /// Whether seed is geometrically well-formed (ready for Phase 9 operations)
    pub geometry_well_formed: bool,

    /// Deterministic hash of seed content (for replay verification)
    pub seed_content_hash: String,

    /// Frame count from originating episode
    pub source_frame_count: usize,
}

/// Phase 9 Slice 1: Form a geometric cognitive seed from Phase 8 integration hook + deltas.
///
/// This function:
/// 1. Validates hook readiness and delta cardinality
/// 2. Constructs semantic anchor point set from propagated contexts
/// 3. Computes delta magnitude and geometry signature
/// 4. Derives seed content hash for replay stability verification
/// 5. Determines geometric well-formedness (all constraints satisfied)
///
/// Returns a fully deterministic Phase90GeometricCognitiveSeed ready for Phase 9 operations.
pub fn phase90_form_geometry_seed_from_integration_hook(
    hook: &Phase80Phase9IntegrationHook,
    summary: &Phase80EpisodeStructuralSummary,
    deltas: &[Phase80CrossFrameStructuralDelta],
) -> Phase90GeometricCognitiveSeed {
    // Validate input cardinality
    let expected_delta_count = summary.transition_count;
    let actual_delta_count = deltas.len();
    let cardinality_valid = actual_delta_count == expected_delta_count;

    // Collect unique semantic anchor contexts (deterministic order via BTreeSet)
    let unique_contexts: std::collections::BTreeSet<_> =
        summary.propagated_semantic_contexts.iter().cloned().collect();
    let semantic_anchor_contexts: Vec<String> = unique_contexts.iter().cloned().collect();

    // Compute total delta magnitude
    let total_delta_magnitude: i32 = deltas
        .iter()
        .map(|d| d.continuity_adjusted_delta.abs())
        .sum();

    // Construct geometry signature from deltas (deterministic ordering)
    let delta_lines: Vec<String> = deltas
        .iter()
        .enumerate()
        .map(|(idx, delta)| {
            format!(
                "{}:{}->{}:{}:{}:{}",
                idx,
                delta.from_frame_id,
                delta.to_frame_id,
                delta.parameter_name,
                delta.raw_structural_delta,
                delta.continuity_adjusted_delta
            )
        })
        .collect();

    let geometry_signature = format!(
        "episode={}|contexts={}|weight={}|total_delta={}|deltas={}",
        hook.episode_id,
        semantic_anchor_contexts.join(","),
        hook.continuity_weight_percent,
        total_delta_magnitude,
        delta_lines.join("|")
    );

    // Compute seed content hash (deterministic)
    let seed_content_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        geometry_signature.hash(&mut hasher);
        summary.episode_id.hash(&mut hasher);
        hook.continuity_weight_percent.hash(&mut hasher);
        hasher.finish()
    });

    // Determine geometric well-formedness:
    // - Integration must be ready (from hook)
    // - Deltas must be non-empty
    // - Cardinality must match
    // - Must have at least one semantic anchor context
    let geometry_well_formed = hook.integration_ready
        && !deltas.is_empty()
        && cardinality_valid
        && !semantic_anchor_contexts.is_empty();

    Phase90GeometricCognitiveSeed {
        episode_id: hook.episode_id.clone(),
        geometry_signature,
        semantic_anchor_contexts,
        continuity_weight_percent: hook.continuity_weight_percent,
        total_delta_magnitude,
        transition_count: deltas.len(),
        geometry_well_formed,
        seed_content_hash,
        source_frame_count: summary.frame_count,
    }
}

/// Emit Phase 9 Slice 1 seed formation telemetry to environment variable.
///
/// Canonical format: pipe-delimited, no floating-point values, deterministic field order.
pub fn phase90_emit_seed_formation_telemetry(
    seed: &Phase90GeometricCognitiveSeed,
) -> String {
    let line = format!(
        "episode_id={}:geometry_signature={}:semantic_anchors={}:continuity_weight={}:total_delta={}:transition_count={}:well_formed={}:content_hash={}:frame_count={}",
        seed.episode_id,
        seed.geometry_signature,
        seed.semantic_anchor_contexts.join(","),
        seed.continuity_weight_percent,
        seed.total_delta_magnitude,
        seed.transition_count,
        seed.geometry_well_formed,
        seed.seed_content_hash,
        seed.source_frame_count,
    );
    env::set_var(PHASE90_SEED_FORMATION_TELEMETRY, &line);
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
    fn phase90_slice1_forms_seed_from_hook_and_deltas() {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let log = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_p9s1_a", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_p9s1_b", "none", false, 1, 1, 0),
                entry(3, "holdout_p9s1_c", "none", true, 1, 2, 1),
            ],
        };

        let trace = phase80_run_multiframe_episode("episode_p9s1", &log, &registry).expect("trace");
        let deltas =
            phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
        let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
            .expect("summary");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);

        let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);

        assert_eq!(seed.episode_id, "episode_p9s1");
        assert!(seed.geometry_well_formed);
        assert!(!seed.seed_content_hash.is_empty());
        assert_eq!(seed.transition_count, deltas.len());
        assert!(!seed.semantic_anchor_contexts.is_empty());
        assert_eq!(seed.source_frame_count, 3);
    }

    #[test]
    fn phase90_slice1_seed_has_well_formed_geometry_when_hook_ready() {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let log = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_p9s1_d", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_p9s1_e", "none", false, 1, 1, 0),
            ],
        };

        let trace = phase80_run_multiframe_episode("episode_p9s1_b", &log, &registry).expect("trace");
        let deltas =
            phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
        let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
            .expect("summary");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);

        let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);

        assert!(hook.integration_ready, "hook must be ready");
        assert!(!deltas.is_empty(), "deltas must be non-empty");
        assert!(!seed.semantic_anchor_contexts.is_empty(), "anchors must exist");
        assert!(seed.geometry_well_formed, "seed must be well-formed");
    }

    #[test]
    fn phase90_slice1_seed_content_hash_is_deterministic() {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let log = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_p9s1_f", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_p9s1_g", "none", false, 1, 1, 0),
                entry(3, "holdout_p9s1_h", "none", true, 1, 2, 1),
                entry(4, "holdout_p9s1_i", "continuity_insensitive", true, 2, 2, 0),
            ],
        };

        let trace_a =
            phase80_run_multiframe_episode("episode_p9s1_hash", &log, &registry).expect("trace_a");
        let deltas_a = phase80_integrate_cross_frame_structural_deltas(&trace_a, &log, &registry)
            .expect("deltas_a");
        let summary_a = phase80_summarize_episode_structural_integration(&trace_a, &log, &registry)
            .expect("summary_a");
        let hook_a = phase80_build_phase9_integration_hook(&summary_a, &deltas_a);
        let seed_a = phase90_form_geometry_seed_from_integration_hook(&hook_a, &summary_a, &deltas_a);

        // Replay the same pipeline
        let trace_b =
            phase80_run_multiframe_episode("episode_p9s1_hash", &log, &registry).expect("trace_b");
        let deltas_b = phase80_integrate_cross_frame_structural_deltas(&trace_b, &log, &registry)
            .expect("deltas_b");
        let summary_b = phase80_summarize_episode_structural_integration(&trace_b, &log, &registry)
            .expect("summary_b");
        let hook_b = phase80_build_phase9_integration_hook(&summary_b, &deltas_b);
        let seed_b = phase90_form_geometry_seed_from_integration_hook(&hook_b, &summary_b, &deltas_b);

        assert_eq!(seed_a, seed_b, "seeds must be identical");
        assert_eq!(
            seed_a.seed_content_hash, seed_b.seed_content_hash,
            "content hashes must be identical"
        );
    }

    #[test]
    fn phase90_slice1_telemetry_is_canonical_and_replay_stable() {
        let registry = Phase70StructuralParameterRegistry::canonical();
        let log = Phase70AdjustmentLog {
            entries: vec![
                entry(1, "holdout_p9s1_j", "continuity_insensitive", true, 0, 1, 1),
                entry(2, "holdout_p9s1_k", "none", false, 1, 1, 0),
                entry(3, "holdout_p9s1_l", "none", true, 1, 2, 1),
            ],
        };

        let trace = phase80_run_multiframe_episode("episode_p9s1_telemetry", &log, &registry)
            .expect("trace");
        let deltas =
            phase80_integrate_cross_frame_structural_deltas(&trace, &log, &registry).expect("deltas");
        let summary = phase80_summarize_episode_structural_integration(&trace, &log, &registry)
            .expect("summary");
        let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
        let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);

        let telemetry_1 = phase90_emit_seed_formation_telemetry(&seed);
        let telemetry_2 = phase90_emit_seed_formation_telemetry(&seed);

        assert_eq!(telemetry_1, telemetry_2, "telemetry must be identical on replay");
        assert!(
            telemetry_1.contains("episode_id="),
            "telemetry must contain episode_id"
        );
        assert!(
            telemetry_1.contains("geometry_signature="),
            "telemetry must contain geometry_signature"
        );
        assert!(
            telemetry_1.contains("semantic_anchors="),
            "telemetry must contain semantic_anchors"
        );
        assert!(
            telemetry_1.contains("well_formed="),
            "telemetry must contain well_formed"
        );
    }
}
