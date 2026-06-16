use crate::cognition::phase62_structural_experiment::{
    phase70_validate_adjustment_log_invariants, phase80_build_frame_local_parameter_registry,
    phase80_build_phase9_integration_hook, phase80_integrate_cross_frame_structural_deltas,
    phase80_run_multiframe_episode, phase80_scaffold_frame_transitions,
    phase80_sequence_frame_transitions, phase80_summarize_episode_structural_integration,
    Phase70AdjustmentLog, Phase70AdjustmentLogEntry, Phase70StructuralParameterRegistry,
    Phase80CrossFrameStructuralDelta, Phase80EpisodeStructuralSummary, Phase80EpisodeTrace,
    Phase80FrameTransitionEvent, Phase80Phase9IntegrationHook,
};
use crate::cognition::phase90_cognitive_manifolds::phase90_form_cognitive_manifold;
use crate::cognition::phase90_continuity_weighted_fields::phase90_form_continuity_weighted_field_from_seed;
use crate::cognition::phase90_emergent_cognitive_shapes::phase90_compose_emergent_cognitive_shape;
use crate::cognition::phase90_geometric_cognitive_seed::{
    phase90_form_geometry_seed_from_integration_hook, Phase90GeometricCognitiveSeed,
};
use crate::cognition::phase90_geometry_driven_adjustment_operators::{
    phase90_build_geometry_driven_adjustment_plan, Phase90GeometryDrivenAdjustmentPlan,
};
use crate::cognition::phase90_multishape_interaction_dynamics::
    phase90_compute_multishape_interaction_dynamics;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE10_RUNTIME_ADAPTATION_TELEMETRY: &str = "GORT_PHASE10_RUNTIME_ADAPTATION_TELEMETRY";
const PHASE10_OPERATOR_RUNTIME_INTEGRATION_TELEMETRY: &str =
    "GORT_PHASE10_OPERATOR_RUNTIME_INTEGRATION_TELEMETRY";
const PHASE10_RUNTIME_FEEDBACK_TELEMETRY: &str =
    "GORT_PHASE10_RUNTIME_FEEDBACK_TELEMETRY";
const PHASE10_TOP_LEVEL_ACCEPTANCE_TELEMETRY: &str =
    "GORT_PHASE10_TOP_LEVEL_ACCEPTANCE_TELEMETRY";
const PHASE10_SLICE7_MULTICYCLE_TELEMETRY: &str =
    "GORT_PHASE10_SLICE7_MULTICYCLE_TELEMETRY";
const PHASE11_MULTI_LOOP_CONVERGENCE_TELEMETRY: &str =
    "GORT_PHASE11_MULTI_LOOP_CONVERGENCE_TELEMETRY";
const PHASE10_TOP_LEVEL_REJECT_REPLAY_COUNT_TOO_LOW: &str =
    "phase10_top_level_reject_replay_count_too_low";
const PHASE10_TOP_LEVEL_REJECT_FEEDBACK_NOT_WELL_FORMED: &str =
    "phase10_top_level_reject_feedback_not_well_formed";
const PHASE10_TOP_LEVEL_REJECT_FEEDBACK_MISMATCH: &str =
    "phase10_top_level_reject_feedback_mismatch";
const PHASE10_SLICE7_REJECT_CYCLE_COUNT_TOO_LOW: &str =
    "phase10_slice7_reject_cycle_count_too_low";
const PHASE10_SLICE7_REJECT_FEEDBACK_NOT_WELL_FORMED: &str =
    "phase10_slice7_reject_feedback_not_well_formed";
const PHASE10_SLICE7_REJECT_PLAN_NOT_WELL_FORMED: &str =
    "phase10_slice7_reject_plan_not_well_formed";
const PHASE10_SLICE7_REJECT_TRANSITION_COUNT_DRIFT: &str =
    "phase10_slice7_reject_transition_count_drift";
const PHASE10_SLICE7_REJECT_CONTINUITY_WEIGHT_DRIFT: &str =
    "phase10_slice7_reject_continuity_weight_drift";
const PHASE10_SLICE7_REJECT_OPERATOR_WEIGHT_DRIFT: &str =
    "phase10_slice7_reject_operator_weight_drift";
const PHASE10_SLICE7_REJECT_OPERATOR_COUNT_DRIFT: &str =
    "phase10_slice7_reject_operator_count_drift";
const PHASE11_REJECT_LOOP_COUNT_TOO_LOW: &str =
    "phase11_reject_loop_count_too_low";
const PHASE11_REJECT_CYCLE_COUNT_TOO_LOW: &str =
    "phase11_reject_cycle_count_too_low";
const PHASE11_REJECT_SLICE7_NOT_WELL_FORMED: &str =
    "phase11_reject_slice7_not_well_formed";
const PHASE11_REJECT_SLICE7_PROFILE_DRIFT: &str =
    "phase11_reject_slice7_profile_drift";
const PHASE11_REJECT_TRANSITION_COUNT_DRIFT: &str =
    "phase11_reject_transition_count_drift";
const PHASE11_REJECT_CONTINUITY_WEIGHT_DRIFT: &str =
    "phase11_reject_continuity_weight_drift";
const PHASE11_REJECT_OPERATOR_WEIGHT_DRIFT: &str =
    "phase11_reject_operator_weight_drift";
const PHASE10_SLICE4_REJECT_INTEGRATION_NOT_WELL_FORMED: &str =
    "phase10_slice4_reject_integration_not_well_formed";
const PHASE10_SLICE4_REJECT_HASH_MISMATCH: &str =
    "phase10_slice4_reject_hash_mismatch";
const PHASE10_SLICE4_REJECT_TRANSITION_COUNT_MISMATCH: &str =
    "phase10_slice4_reject_transition_count_mismatch";
const PHASE10_SLICE4_REJECT_OPERATOR_WEIGHT_OUT_OF_BOUNDS: &str =
    "phase10_slice4_reject_operator_weight_out_of_bounds";
const PHASE10_SLICE4_REJECT_ENVELOPE_DELTA_EXCEEDED: &str =
    "phase10_slice4_reject_envelope_delta_exceeded";
const PHASE10_SLICE4_REJECT_CONTINUITY_VIOLATION: &str =
    "phase10_slice4_reject_continuity_violation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10RuntimeAdaptationBridge {
    pub manifold_signature: String,
    pub operator_plan_hash: String,
    pub adapted_entry_count: usize,
    pub aggregate_applied_delta: i32,
    pub routed_parameter_count: usize,
    pub routed_parameter_names: Vec<String>,
    pub adaptation_log: Phase70AdjustmentLog,
    pub adaptation_signature: String,
    pub adaptation_profile_hash: String,
    pub adaptation_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10Slice2RoutingAcceptancePolicy {
    pub min_operator_count_threshold: usize,
    pub min_routed_parameter_diversity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10OperatorWeightedFrameTransition {
    pub from_frame_id: String,
    pub to_frame_id: String,
    pub entry_allowed: bool,
    pub exit_allowed: bool,
    pub continuity_preserved: bool,
    pub transition_reason: String,
    pub operator_kind: String,
    pub operator_weight_percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10OperatorRuntimeIntegration {
    pub operator_plan_hash: String,
    pub adaptation_profile_hash: String,
    pub weighted_transition_count: usize,
    pub aggregate_operator_weight_percent: u8,
    pub weighted_transitions: Vec<Phase10OperatorWeightedFrameTransition>,
    pub integration_signature: String,
    pub integration_profile_hash: String,
    pub integration_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10ClosedLoopIntegrityStage {
    pub operator_plan_hash: String,
    pub adaptation_profile_hash: String,
    pub integration_profile_hash: String,
    pub routed_parameter_count: usize,
    pub weighted_transition_count: usize,
    pub aggregate_operator_weight_percent: u8,
    pub continuity_gate_passed: bool,
    pub integrity_signature: String,
    pub integrity_profile_hash: String,
    pub integrity_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10RuntimeFeedbackSeedLoop {
    pub operator_plan_hash: String,
    pub integrity_profile_hash: String,
    pub episode_id: String,
    pub regenerated_trace: Phase80EpisodeTrace,
    pub regenerated_deltas: Vec<Phase80CrossFrameStructuralDelta>,
    pub regenerated_summary: Phase80EpisodeStructuralSummary,
    pub regenerated_hook: Phase80Phase9IntegrationHook,
    pub regenerated_seed: Phase90GeometricCognitiveSeed,
    pub feedback_signature: String,
    pub feedback_profile_hash: String,
    pub feedback_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10TopLevelAcceptanceStage {
    pub operator_plan_hash: String,
    pub replay_count: usize,
    pub baseline_feedback_hash: String,
    pub baseline_seed_hash: String,
    pub acceptance_signature: String,
    pub acceptance_profile_hash: String,
    pub acceptance_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10Slice7MultiCycleReplayAcceptanceStage {
    pub baseline_operator_plan_hash: String,
    pub cycle_count: usize,
    pub cycle_plan_hashes: Vec<String>,
    pub cycle_feedback_hashes: Vec<String>,
    pub cycle_seed_hashes: Vec<String>,
    pub baseline_transition_count: usize,
    pub baseline_continuity_weight_percent: u8,
    pub baseline_operator_weight_percent: u8,
    pub acceptance_signature: String,
    pub acceptance_profile_hash: String,
    pub acceptance_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase11MultiLoopConvergenceStage {
    pub baseline_operator_plan_hash: String,
    pub loop_count: usize,
    pub cycle_count_per_loop: usize,
    pub convergence_loop_index: usize,
    pub loop_acceptance_hashes: Vec<String>,
    pub loop_terminal_plan_hashes: Vec<String>,
    pub loop_terminal_seed_hashes: Vec<String>,
    pub baseline_transition_count: usize,
    pub baseline_continuity_weight_percent: u8,
    pub baseline_operator_weight_percent: u8,
    pub convergence_signature: String,
    pub convergence_profile_hash: String,
    pub convergence_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase10Slice4RuntimeContinuityPolicy {
    pub min_operator_weight_percent: u8,
    pub max_operator_weight_percent: u8,
    pub max_runtime_delta_per_transition: i32,
}

impl Phase10Slice2RoutingAcceptancePolicy {
    pub fn canonical() -> Self {
        Self {
            min_operator_count_threshold: 3,
            min_routed_parameter_diversity: 2,
        }
    }
}

impl Phase10Slice4RuntimeContinuityPolicy {
    pub fn canonical() -> Self {
        Self {
            min_operator_weight_percent: 40,
            max_operator_weight_percent: 100,
            max_runtime_delta_per_transition: 1,
        }
    }
}

pub fn phase10_validate_slice2_routing_acceptance_gate(
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    bridge: &Phase10RuntimeAdaptationBridge,
    policy: &Phase10Slice2RoutingAcceptancePolicy,
) -> Result<(), String> {
    if policy.min_routed_parameter_diversity == 0 {
        return Err("invalid slice2 policy: min_routed_parameter_diversity must be >= 1".to_string());
    }
    if !bridge.adaptation_well_formed {
        return Err("slice2 acceptance failed: adaptation bridge is not well formed".to_string());
    }
    if plan.operator_plan_hash != bridge.operator_plan_hash {
        return Err("slice2 acceptance failed: plan/bridge hash mismatch".to_string());
    }

    if plan.operator_count < policy.min_operator_count_threshold {
        return Ok(());
    }

    if bridge.routed_parameter_count < policy.min_routed_parameter_diversity {
        return Err(format!(
            "slice2 acceptance failed: routed parameter diversity {} below required {} when operator_count={} (threshold={})",
            bridge.routed_parameter_count,
            policy.min_routed_parameter_diversity,
            plan.operator_count,
            policy.min_operator_count_threshold
        ));
    }

    Ok(())
}

pub fn phase10_integrate_operator_plan_into_runtime_transitions(
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    bridge: &Phase10RuntimeAdaptationBridge,
    registry: &Phase70StructuralParameterRegistry,
) -> Result<Phase10OperatorRuntimeIntegration, String> {
    if !bridge.adaptation_well_formed {
        return Err("cannot integrate malformed adaptation bridge into runtime transitions".to_string());
    }
    if !plan.operator_plan_well_formed || plan.operators.is_empty() {
        return Err("cannot integrate malformed operator plan into runtime transitions".to_string());
    }
    if plan.operator_plan_hash != bridge.operator_plan_hash {
        return Err("cannot integrate plan/bridge with mismatched operator hashes".to_string());
    }

    let frame_registry = phase80_build_frame_local_parameter_registry(&bridge.adaptation_log, registry)?;
    let transitions = phase80_scaffold_frame_transitions(&frame_registry, registry)?;
    let sequenced = phase80_sequence_frame_transitions(&frame_registry, &transitions)?;

    let weighted_transitions = sequenced
        .iter()
        .enumerate()
        .map(|(index, transition)| {
            phase10_apply_operator_weight_to_transition(index, transition, plan)
        })
        .collect::<Vec<_>>();

    let weighted_transition_count = weighted_transitions.len();
    let aggregate_operator_weight_percent = if weighted_transition_count == 0 {
        0
    } else {
        (weighted_transitions
            .iter()
            .map(|transition| transition.operator_weight_percent as u32)
            .sum::<u32>()
            / weighted_transition_count as u32) as u8
    };

    let integration_signature = format!(
        "plan_hash={}|adaptation_hash={}|weighted_count={}|aggregate_weight={}|transitions={}",
        plan.operator_plan_hash,
        bridge.adaptation_profile_hash,
        weighted_transition_count,
        aggregate_operator_weight_percent,
        weighted_transitions
            .iter()
            .map(|transition| format!(
                "{}:{}:{}:{}:{}",
                transition.from_frame_id,
                transition.to_frame_id,
                transition.operator_kind,
                transition.operator_weight_percent,
                transition.continuity_preserved,
            ))
            .collect::<Vec<_>>()
            .join("|"),
    );

    let integration_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        integration_signature.hash(&mut hasher);
        weighted_transition_count.hash(&mut hasher);
        aggregate_operator_weight_percent.hash(&mut hasher);
        hasher.finish()
    });

    let integration_well_formed = weighted_transition_count > 0
        && weighted_transitions
            .iter()
            .all(|transition| transition.operator_weight_percent > 0);

    Ok(Phase10OperatorRuntimeIntegration {
        operator_plan_hash: plan.operator_plan_hash.clone(),
        adaptation_profile_hash: bridge.adaptation_profile_hash.clone(),
        weighted_transition_count,
        aggregate_operator_weight_percent,
        weighted_transitions,
        integration_signature,
        integration_profile_hash,
        integration_well_formed,
    })
}

pub fn phase10_emit_operator_runtime_integration_telemetry(
    integration: &Phase10OperatorRuntimeIntegration,
) -> String {
    let line = format!(
        "operator_plan_hash={}:adaptation_profile_hash={}:weighted_transition_count={}:aggregate_operator_weight={}:integration_signature={}:integration_hash={}:well_formed={}",
        integration.operator_plan_hash,
        integration.adaptation_profile_hash,
        integration.weighted_transition_count,
        integration.aggregate_operator_weight_percent,
        integration.integration_signature,
        integration.integration_profile_hash,
        integration.integration_well_formed,
    );
    env::set_var(PHASE10_OPERATOR_RUNTIME_INTEGRATION_TELEMETRY, &line);
    line
}

pub fn phase10_validate_runtime_continuity_preservation(
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    bridge: &Phase10RuntimeAdaptationBridge,
    integration: &Phase10OperatorRuntimeIntegration,
    policy: &Phase10Slice4RuntimeContinuityPolicy,
) -> Result<(), String> {
    if !integration.integration_well_formed || !bridge.adaptation_well_formed {
        return Err(PHASE10_SLICE4_REJECT_INTEGRATION_NOT_WELL_FORMED.to_string());
    }
    if plan.operator_plan_hash != bridge.operator_plan_hash
        || plan.operator_plan_hash != integration.operator_plan_hash
        || bridge.adaptation_profile_hash != integration.adaptation_profile_hash
    {
        return Err(PHASE10_SLICE4_REJECT_HASH_MISMATCH.to_string());
    }
    if integration.weighted_transition_count != bridge.adaptation_log.entries.len()
        || integration.weighted_transition_count != integration.weighted_transitions.len()
    {
        return Err(PHASE10_SLICE4_REJECT_TRANSITION_COUNT_MISMATCH.to_string());
    }

    for (index, transition) in integration.weighted_transitions.iter().enumerate() {
        if transition.operator_weight_percent < policy.min_operator_weight_percent
            || transition.operator_weight_percent > policy.max_operator_weight_percent
        {
            return Err(PHASE10_SLICE4_REJECT_OPERATOR_WEIGHT_OUT_OF_BOUNDS.to_string());
        }

        let Some(entry) = bridge.adaptation_log.entries.get(index) else {
            return Err(PHASE10_SLICE4_REJECT_TRANSITION_COUNT_MISMATCH.to_string());
        };
        if entry.delta.abs() > policy.max_runtime_delta_per_transition {
            return Err(PHASE10_SLICE4_REJECT_ENVELOPE_DELTA_EXCEEDED.to_string());
        }

        if !transition.continuity_preserved
            || !transition.entry_allowed
            || !transition.exit_allowed
            || !transition.transition_reason.contains("operator_kind=")
        {
            return Err(PHASE10_SLICE4_REJECT_CONTINUITY_VIOLATION.to_string());
        }
    }

    Ok(())
}

pub fn phase10_execute_closed_loop_integrity_stage(
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    registry: &Phase70StructuralParameterRegistry,
    slice2_policy: &Phase10Slice2RoutingAcceptancePolicy,
    slice4_policy: &Phase10Slice4RuntimeContinuityPolicy,
) -> Result<Phase10ClosedLoopIntegrityStage, String> {
    let bridge = phase10_build_runtime_adaptation_bridge(plan, registry)?;
    phase10_validate_slice2_routing_acceptance_gate(plan, &bridge, slice2_policy)?;

    let integration =
        phase10_integrate_operator_plan_into_runtime_transitions(plan, &bridge, registry)?;
    phase10_validate_runtime_continuity_preservation(plan, &bridge, &integration, slice4_policy)?;

    let continuity_gate_passed = true;
    let integrity_signature = format!(
        "plan_hash={}|adaptation_hash={}|integration_hash={}|routed_parameter_count={}|weighted_transition_count={}|aggregate_operator_weight={}|continuity_gate_passed={}",
        plan.operator_plan_hash,
        bridge.adaptation_profile_hash,
        integration.integration_profile_hash,
        bridge.routed_parameter_count,
        integration.weighted_transition_count,
        integration.aggregate_operator_weight_percent,
        continuity_gate_passed,
    );
    let integrity_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        integrity_signature.hash(&mut hasher);
        bridge.routed_parameter_count.hash(&mut hasher);
        integration.weighted_transition_count.hash(&mut hasher);
        continuity_gate_passed.hash(&mut hasher);
        hasher.finish()
    });
    let integrity_well_formed = continuity_gate_passed
        && bridge.adaptation_well_formed
        && integration.integration_well_formed;

    Ok(Phase10ClosedLoopIntegrityStage {
        operator_plan_hash: plan.operator_plan_hash.clone(),
        adaptation_profile_hash: bridge.adaptation_profile_hash,
        integration_profile_hash: integration.integration_profile_hash,
        routed_parameter_count: bridge.routed_parameter_count,
        weighted_transition_count: integration.weighted_transition_count,
        aggregate_operator_weight_percent: integration.aggregate_operator_weight_percent,
        continuity_gate_passed,
        integrity_signature,
        integrity_profile_hash,
        integrity_well_formed,
    })
}

pub fn phase10_regenerate_phase9_seed_from_adapted_episode(
    episode_id: &str,
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    registry: &Phase70StructuralParameterRegistry,
    slice2_policy: &Phase10Slice2RoutingAcceptancePolicy,
    slice4_policy: &Phase10Slice4RuntimeContinuityPolicy,
) -> Result<Phase10RuntimeFeedbackSeedLoop, String> {
    let stage = phase10_execute_closed_loop_integrity_stage(
        plan,
        registry,
        slice2_policy,
        slice4_policy,
    )?;
    let bridge = phase10_build_runtime_adaptation_bridge(plan, registry)?;
    let regenerated_trace = phase10_run_runtime_adaptation_episode(episode_id, &bridge, registry)?;
    let regenerated_deltas =
        phase80_integrate_cross_frame_structural_deltas(&regenerated_trace, &bridge.adaptation_log, registry)?;
    let regenerated_summary = phase80_summarize_episode_structural_integration(
        &regenerated_trace,
        &bridge.adaptation_log,
        registry,
    )?;
    let regenerated_hook =
        phase80_build_phase9_integration_hook(&regenerated_summary, &regenerated_deltas);
    let regenerated_seed = phase90_form_geometry_seed_from_integration_hook(
        &regenerated_hook,
        &regenerated_summary,
        &regenerated_deltas,
    );

    let feedback_signature = format!(
        "plan_hash={}|integrity_hash={}|episode_id={}|seed_hash={}|hook_ready={}|transition_count={}|continuity_weight={}",
        plan.operator_plan_hash,
        stage.integrity_profile_hash,
        episode_id,
        regenerated_seed.seed_content_hash,
        regenerated_hook.integration_ready,
        regenerated_seed.transition_count,
        regenerated_seed.continuity_weight_percent,
    );
    let feedback_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        feedback_signature.hash(&mut hasher);
        regenerated_seed.seed_content_hash.hash(&mut hasher);
        regenerated_hook.geometry_seed_signature.hash(&mut hasher);
        hasher.finish()
    });
    let feedback_well_formed = stage.integrity_well_formed
        && regenerated_hook.integration_ready
        && regenerated_seed.geometry_well_formed;

    Ok(Phase10RuntimeFeedbackSeedLoop {
        operator_plan_hash: plan.operator_plan_hash.clone(),
        integrity_profile_hash: stage.integrity_profile_hash,
        episode_id: episode_id.to_string(),
        regenerated_trace,
        regenerated_deltas,
        regenerated_summary,
        regenerated_hook,
        regenerated_seed,
        feedback_signature,
        feedback_profile_hash,
        feedback_well_formed,
    })
}

pub fn phase10_emit_runtime_feedback_telemetry(
    feedback: &Phase10RuntimeFeedbackSeedLoop,
) -> String {
    let line = format!(
        "operator_plan_hash={}:integrity_profile_hash={}:episode_id={}:seed_hash={}:hook_ready={}:transition_count={}:continuity_weight={}:feedback_signature={}:feedback_hash={}:well_formed={}",
        feedback.operator_plan_hash,
        feedback.integrity_profile_hash,
        feedback.episode_id,
        feedback.regenerated_seed.seed_content_hash,
        feedback.regenerated_hook.integration_ready,
        feedback.regenerated_seed.transition_count,
        feedback.regenerated_seed.continuity_weight_percent,
        feedback.feedback_signature,
        feedback.feedback_profile_hash,
        feedback.feedback_well_formed,
    );
    env::set_var(PHASE10_RUNTIME_FEEDBACK_TELEMETRY, &line);
    line
}

pub fn phase10_run_top_level_acceptance_stage(
    episode_id: &str,
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    registry: &Phase70StructuralParameterRegistry,
    slice2_policy: &Phase10Slice2RoutingAcceptancePolicy,
    slice4_policy: &Phase10Slice4RuntimeContinuityPolicy,
    replay_count: usize,
) -> Result<Phase10TopLevelAcceptanceStage, String> {
    if replay_count < 2 {
        return Err(PHASE10_TOP_LEVEL_REJECT_REPLAY_COUNT_TOO_LOW.to_string());
    }

    let baseline = phase10_regenerate_phase9_seed_from_adapted_episode(
        episode_id,
        plan,
        registry,
        slice2_policy,
        slice4_policy,
    )?;

    if !baseline.feedback_well_formed {
        return Err(PHASE10_TOP_LEVEL_REJECT_FEEDBACK_NOT_WELL_FORMED.to_string());
    }

    for _ in 1..replay_count {
        let current = phase10_regenerate_phase9_seed_from_adapted_episode(
            episode_id,
            plan,
            registry,
            slice2_policy,
            slice4_policy,
        )?;

        if !current.feedback_well_formed {
            return Err(PHASE10_TOP_LEVEL_REJECT_FEEDBACK_NOT_WELL_FORMED.to_string());
        }

        if current.feedback_profile_hash != baseline.feedback_profile_hash
            || current.regenerated_seed != baseline.regenerated_seed
            || current.regenerated_hook != baseline.regenerated_hook
            || current.regenerated_trace != baseline.regenerated_trace
        {
            return Err(PHASE10_TOP_LEVEL_REJECT_FEEDBACK_MISMATCH.to_string());
        }
    }

    let acceptance_signature = format!(
        "plan_hash={}|episode_id={}|replay_count={}|baseline_feedback_hash={}|baseline_seed_hash={}",
        plan.operator_plan_hash,
        episode_id,
        replay_count,
        baseline.feedback_profile_hash,
        baseline.regenerated_seed.seed_content_hash,
    );
    let acceptance_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        acceptance_signature.hash(&mut hasher);
        baseline.feedback_profile_hash.hash(&mut hasher);
        baseline.regenerated_seed.seed_content_hash.hash(&mut hasher);
        hasher.finish()
    });

    Ok(Phase10TopLevelAcceptanceStage {
        operator_plan_hash: plan.operator_plan_hash.clone(),
        replay_count,
        baseline_feedback_hash: baseline.feedback_profile_hash,
        baseline_seed_hash: baseline.regenerated_seed.seed_content_hash,
        acceptance_signature,
        acceptance_profile_hash,
        acceptance_well_formed: true,
    })
}

pub fn phase10_emit_top_level_acceptance_telemetry(
    acceptance: &Phase10TopLevelAcceptanceStage,
) -> String {
    let line = format!(
        "operator_plan_hash={}:replay_count={}:baseline_feedback_hash={}:baseline_seed_hash={}:acceptance_signature={}:acceptance_hash={}:well_formed={}",
        acceptance.operator_plan_hash,
        acceptance.replay_count,
        acceptance.baseline_feedback_hash,
        acceptance.baseline_seed_hash,
        acceptance.acceptance_signature,
        acceptance.acceptance_profile_hash,
        acceptance.acceptance_well_formed,
    );
    env::set_var(PHASE10_TOP_LEVEL_ACCEPTANCE_TELEMETRY, &line);
    line
}

pub fn phase10_run_slice7_multicycle_replay_acceptance_stage(
    episode_id: &str,
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    registry: &Phase70StructuralParameterRegistry,
    slice2_policy: &Phase10Slice2RoutingAcceptancePolicy,
    slice4_policy: &Phase10Slice4RuntimeContinuityPolicy,
    cycle_count: usize,
) -> Result<Phase10Slice7MultiCycleReplayAcceptanceStage, String> {
    if cycle_count < 2 {
        return Err(PHASE10_SLICE7_REJECT_CYCLE_COUNT_TOO_LOW.to_string());
    }
    if !plan.operator_plan_well_formed {
        return Err(PHASE10_SLICE7_REJECT_PLAN_NOT_WELL_FORMED.to_string());
    }

    let mut rolling_plan = plan.clone();
    let mut cycle_plan_hashes = Vec::with_capacity(cycle_count);
    let mut cycle_feedback_hashes = Vec::with_capacity(cycle_count);
    let mut cycle_seed_hashes = Vec::with_capacity(cycle_count);

    let baseline_stage = phase10_execute_closed_loop_integrity_stage(
        &rolling_plan,
        registry,
        slice2_policy,
        slice4_policy,
    )?;
    let baseline_feedback = phase10_regenerate_phase9_seed_from_adapted_episode(
        episode_id,
        &rolling_plan,
        registry,
        slice2_policy,
        slice4_policy,
    )?;
    if !baseline_feedback.feedback_well_formed {
        return Err(PHASE10_SLICE7_REJECT_FEEDBACK_NOT_WELL_FORMED.to_string());
    }

    let baseline_seed = baseline_feedback.regenerated_seed.clone();
    let baseline_transition_count = baseline_seed.transition_count;
    let baseline_continuity_weight_percent = baseline_seed.continuity_weight_percent;
    let baseline_operator_weight_percent = baseline_stage.aggregate_operator_weight_percent;

    cycle_plan_hashes.push(rolling_plan.operator_plan_hash.clone());
    cycle_feedback_hashes.push(baseline_feedback.feedback_profile_hash.clone());
    cycle_seed_hashes.push(baseline_seed.seed_content_hash.clone());

    for _ in 1..cycle_count {
        let stage = phase10_execute_closed_loop_integrity_stage(
            &rolling_plan,
            registry,
            slice2_policy,
            slice4_policy,
        )?;
        if stage.aggregate_operator_weight_percent != baseline_operator_weight_percent {
            return Err(PHASE10_SLICE7_REJECT_OPERATOR_WEIGHT_DRIFT.to_string());
        }

        let feedback = phase10_regenerate_phase9_seed_from_adapted_episode(
            episode_id,
            &rolling_plan,
            registry,
            slice2_policy,
            slice4_policy,
        )?;
        if !feedback.feedback_well_formed {
            return Err(PHASE10_SLICE7_REJECT_FEEDBACK_NOT_WELL_FORMED.to_string());
        }
        if feedback.regenerated_seed.transition_count != baseline_transition_count {
            return Err(PHASE10_SLICE7_REJECT_TRANSITION_COUNT_DRIFT.to_string());
        }
        if feedback.regenerated_seed.continuity_weight_percent != baseline_continuity_weight_percent {
            return Err(PHASE10_SLICE7_REJECT_CONTINUITY_WEIGHT_DRIFT.to_string());
        }

        cycle_plan_hashes.push(rolling_plan.operator_plan_hash.clone());
        cycle_feedback_hashes.push(feedback.feedback_profile_hash.clone());
        cycle_seed_hashes.push(feedback.regenerated_seed.seed_content_hash.clone());

        let next_plan = phase10_build_plan_from_seed_pair(&baseline_seed, &feedback.regenerated_seed)?;
        if !next_plan.operator_plan_well_formed {
            return Err(PHASE10_SLICE7_REJECT_PLAN_NOT_WELL_FORMED.to_string());
        }
        if next_plan.operator_count != plan.operator_count {
            return Err(PHASE10_SLICE7_REJECT_OPERATOR_COUNT_DRIFT.to_string());
        }
        rolling_plan = next_plan;
    }

    let acceptance_signature = format!(
        "baseline_plan_hash={}|cycle_count={}|baseline_transition_count={}|baseline_continuity_weight={}|baseline_operator_weight={}|cycle_plan_hashes={}|cycle_feedback_hashes={}|cycle_seed_hashes={}",
        plan.operator_plan_hash,
        cycle_count,
        baseline_transition_count,
        baseline_continuity_weight_percent,
        baseline_operator_weight_percent,
        cycle_plan_hashes.join("|"),
        cycle_feedback_hashes.join("|"),
        cycle_seed_hashes.join("|"),
    );
    let acceptance_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        acceptance_signature.hash(&mut hasher);
        cycle_plan_hashes.hash(&mut hasher);
        cycle_feedback_hashes.hash(&mut hasher);
        cycle_seed_hashes.hash(&mut hasher);
        hasher.finish()
    });
    let acceptance_well_formed = cycle_plan_hashes.len() == cycle_count
        && cycle_feedback_hashes.len() == cycle_count
        && cycle_seed_hashes.len() == cycle_count
        && baseline_transition_count > 0;

    Ok(Phase10Slice7MultiCycleReplayAcceptanceStage {
        baseline_operator_plan_hash: plan.operator_plan_hash.clone(),
        cycle_count,
        cycle_plan_hashes,
        cycle_feedback_hashes,
        cycle_seed_hashes,
        baseline_transition_count,
        baseline_continuity_weight_percent,
        baseline_operator_weight_percent,
        acceptance_signature,
        acceptance_profile_hash,
        acceptance_well_formed,
    })
}

pub fn phase10_emit_slice7_multicycle_telemetry(
    acceptance: &Phase10Slice7MultiCycleReplayAcceptanceStage,
) -> String {
    let line = format!(
        "baseline_plan_hash={}:cycle_count={}:baseline_transition_count={}:baseline_continuity_weight={}:baseline_operator_weight={}:acceptance_hash={}:well_formed={}",
        acceptance.baseline_operator_plan_hash,
        acceptance.cycle_count,
        acceptance.baseline_transition_count,
        acceptance.baseline_continuity_weight_percent,
        acceptance.baseline_operator_weight_percent,
        acceptance.acceptance_profile_hash,
        acceptance.acceptance_well_formed,
    );
    env::set_var(PHASE10_SLICE7_MULTICYCLE_TELEMETRY, &line);
    line
}

pub fn phase11_run_multi_loop_convergence_stage(
    episode_id: &str,
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    registry: &Phase70StructuralParameterRegistry,
    slice2_policy: &Phase10Slice2RoutingAcceptancePolicy,
    slice4_policy: &Phase10Slice4RuntimeContinuityPolicy,
    loop_count: usize,
    cycle_count_per_loop: usize,
) -> Result<Phase11MultiLoopConvergenceStage, String> {
    if loop_count < 2 {
        return Err(PHASE11_REJECT_LOOP_COUNT_TOO_LOW.to_string());
    }
    if cycle_count_per_loop < 2 {
        return Err(PHASE11_REJECT_CYCLE_COUNT_TOO_LOW.to_string());
    }

    let baseline = phase10_run_slice7_multicycle_replay_acceptance_stage(
        episode_id,
        plan,
        registry,
        slice2_policy,
        slice4_policy,
        cycle_count_per_loop,
    )?;
    if !baseline.acceptance_well_formed {
        return Err(PHASE11_REJECT_SLICE7_NOT_WELL_FORMED.to_string());
    }

    let baseline_transition_count = baseline.baseline_transition_count;
    let baseline_continuity_weight_percent = baseline.baseline_continuity_weight_percent;
    let baseline_operator_weight_percent = baseline.baseline_operator_weight_percent;

    let baseline_terminal_plan_hash = baseline
        .cycle_plan_hashes
        .last()
        .cloned()
        .unwrap_or_default();
    let baseline_terminal_seed_hash = baseline
        .cycle_seed_hashes
        .last()
        .cloned()
        .unwrap_or_default();

    let mut loop_acceptance_hashes = vec![baseline.acceptance_profile_hash.clone()];
    let mut loop_terminal_plan_hashes = vec![baseline_terminal_plan_hash.clone()];
    let mut loop_terminal_seed_hashes = vec![baseline_terminal_seed_hash.clone()];

    for _ in 1..loop_count {
        let stage = phase10_run_slice7_multicycle_replay_acceptance_stage(
            episode_id,
            plan,
            registry,
            slice2_policy,
            slice4_policy,
            cycle_count_per_loop,
        )?;
        if !stage.acceptance_well_formed {
            return Err(PHASE11_REJECT_SLICE7_NOT_WELL_FORMED.to_string());
        }

        if stage.acceptance_profile_hash != baseline.acceptance_profile_hash {
            return Err(PHASE11_REJECT_SLICE7_PROFILE_DRIFT.to_string());
        }
        if stage.baseline_transition_count != baseline_transition_count {
            return Err(PHASE11_REJECT_TRANSITION_COUNT_DRIFT.to_string());
        }
        if stage.baseline_continuity_weight_percent != baseline_continuity_weight_percent {
            return Err(PHASE11_REJECT_CONTINUITY_WEIGHT_DRIFT.to_string());
        }
        if stage.baseline_operator_weight_percent != baseline_operator_weight_percent {
            return Err(PHASE11_REJECT_OPERATOR_WEIGHT_DRIFT.to_string());
        }

        loop_acceptance_hashes.push(stage.acceptance_profile_hash.clone());
        loop_terminal_plan_hashes.push(
            stage
                .cycle_plan_hashes
                .last()
                .cloned()
                .unwrap_or_default(),
        );
        loop_terminal_seed_hashes.push(
            stage
                .cycle_seed_hashes
                .last()
                .cloned()
                .unwrap_or_default(),
        );
    }

    let convergence_loop_index = 1;
    let convergence_signature = format!(
        "baseline_plan_hash={}|loop_count={}|cycle_count_per_loop={}|convergence_loop_index={}|baseline_transition_count={}|baseline_continuity_weight={}|baseline_operator_weight={}|loop_acceptance_hashes={}|loop_terminal_plan_hashes={}|loop_terminal_seed_hashes={}",
        plan.operator_plan_hash,
        loop_count,
        cycle_count_per_loop,
        convergence_loop_index,
        baseline_transition_count,
        baseline_continuity_weight_percent,
        baseline_operator_weight_percent,
        loop_acceptance_hashes.join("|"),
        loop_terminal_plan_hashes.join("|"),
        loop_terminal_seed_hashes.join("|"),
    );
    let convergence_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        convergence_signature.hash(&mut hasher);
        loop_acceptance_hashes.hash(&mut hasher);
        loop_terminal_plan_hashes.hash(&mut hasher);
        loop_terminal_seed_hashes.hash(&mut hasher);
        hasher.finish()
    });
    let convergence_well_formed = loop_acceptance_hashes.len() == loop_count
        && loop_terminal_plan_hashes.len() == loop_count
        && loop_terminal_seed_hashes.len() == loop_count
        && baseline_transition_count > 0;

    Ok(Phase11MultiLoopConvergenceStage {
        baseline_operator_plan_hash: plan.operator_plan_hash.clone(),
        loop_count,
        cycle_count_per_loop,
        convergence_loop_index,
        loop_acceptance_hashes,
        loop_terminal_plan_hashes,
        loop_terminal_seed_hashes,
        baseline_transition_count,
        baseline_continuity_weight_percent,
        baseline_operator_weight_percent,
        convergence_signature,
        convergence_profile_hash,
        convergence_well_formed,
    })
}

pub fn phase11_emit_multi_loop_convergence_telemetry(
    convergence: &Phase11MultiLoopConvergenceStage,
) -> String {
    let line = format!(
        "baseline_plan_hash={}:loop_count={}:cycle_count_per_loop={}:convergence_loop_index={}:baseline_transition_count={}:baseline_continuity_weight={}:baseline_operator_weight={}:convergence_hash={}:well_formed={}",
        convergence.baseline_operator_plan_hash,
        convergence.loop_count,
        convergence.cycle_count_per_loop,
        convergence.convergence_loop_index,
        convergence.baseline_transition_count,
        convergence.baseline_continuity_weight_percent,
        convergence.baseline_operator_weight_percent,
        convergence.convergence_profile_hash,
        convergence.convergence_well_formed,
    );
    env::set_var(PHASE11_MULTI_LOOP_CONVERGENCE_TELEMETRY, &line);
    line
}

fn phase10_build_plan_from_seed_pair(
    anchor_seed: &Phase90GeometricCognitiveSeed,
    replay_seed: &Phase90GeometricCognitiveSeed,
) -> Result<Phase90GeometryDrivenAdjustmentPlan, String> {
    let anchor_field = phase90_form_continuity_weighted_field_from_seed(anchor_seed);
    let replay_field = phase90_form_continuity_weighted_field_from_seed(replay_seed);

    let anchor_shape = phase90_compose_emergent_cognitive_shape(&[anchor_field])?;
    let replay_shape = phase90_compose_emergent_cognitive_shape(&[replay_field])?;
    let manifold = phase90_form_cognitive_manifold(&[anchor_shape, replay_shape])?;
    let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold)?;
    phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics)
}

fn phase10_apply_operator_weight_to_transition(
    transition_index: usize,
    transition: &Phase80FrameTransitionEvent,
    plan: &Phase90GeometryDrivenAdjustmentPlan,
) -> Phase10OperatorWeightedFrameTransition {
    let operator = &plan.operators[transition_index % plan.operators.len()];
    let operator_weight_percent = ((operator.adjustment_pressure_percent as u16
        + operator.continuity_bias_percent as u16
        + operator.manifold_alignment_percent as u16)
        / 3)
        .clamp(0, 100) as u8;

    let continuity_preserved = transition.continuity_preserved && operator_weight_percent >= 40;
    let transition_reason = format!(
        "{}|operator_kind={}|operator_weight={}",
        transition.transition_reason, operator.operator_kind, operator_weight_percent
    );

    Phase10OperatorWeightedFrameTransition {
        from_frame_id: transition.from_frame_id.clone(),
        to_frame_id: transition.to_frame_id.clone(),
        entry_allowed: transition.entry_allowed,
        exit_allowed: transition.exit_allowed,
        continuity_preserved,
        transition_reason,
        operator_kind: operator.operator_kind.clone(),
        operator_weight_percent,
    }
}

pub fn phase10_build_runtime_adaptation_bridge(
    plan: &Phase90GeometryDrivenAdjustmentPlan,
    registry: &Phase70StructuralParameterRegistry,
) -> Result<Phase10RuntimeAdaptationBridge, String> {
    if !plan.operator_plan_well_formed {
        return Err("cannot build runtime adaptation from malformed operator plan".to_string());
    }

    let Some(_) = registry.parameters.first() else {
        return Err("cannot build runtime adaptation with empty parameter registry".to_string());
    };

    let ordered_parameters = registry
        .parameters
        .iter()
        .collect::<Vec<_>>();

    let mut entries = Vec::with_capacity(plan.operators.len());
    let mut parameter_values = ordered_parameters
        .iter()
        .map(|spec| (spec.name.clone(), spec.min_value))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut aggregate_applied_delta = 0;

    for (index, operator) in plan.operators.iter().enumerate() {
        let parameter_index = (operator.target_shape_index + operator.source_shape_index + index)
            % ordered_parameters.len();
        let spec = ordered_parameters[parameter_index];
        let sequence = (index + 1) as u64;
        let apply_delta = operator.adjustment_pressure_percent >= 50
            && operator.continuity_bias_percent >= 40
            && operator.manifold_alignment_percent >= 40;

        let current_value = *parameter_values
            .get(&spec.name)
            .ok_or_else(|| format!("missing routed parameter state for {}", spec.name))?;
        let base_delta = if apply_delta { spec.delta } else { 0 };
        let post_value = (current_value + base_delta).clamp(spec.min_value, spec.max_value);
        let applied_delta = post_value - current_value;
        let inverse_delta = if apply_delta { -applied_delta } else { 0 };

        entries.push(Phase70AdjustmentLogEntry {
            sequence,
            holdout_id: format!(
                "phase10_shape{}_to_{}",
                operator.source_shape_index, operator.target_shape_index,
            ),
            parameter_name: spec.name.clone(),
            semantic_context_used: format!(
                "{}|route_param={}|route_idx={}",
                operator.operator_kind, spec.name, parameter_index
            ),
            adjustment_applied: apply_delta,
            pre_value: current_value,
            post_value,
            delta: applied_delta,
            inverse_delta,
        });

        parameter_values.insert(spec.name.clone(), post_value);
        aggregate_applied_delta += applied_delta;
    }

    let adaptation_log = Phase70AdjustmentLog { entries };
    phase70_validate_adjustment_log_invariants(&adaptation_log, registry)?;

    let adaptation_signature = format!(
        "manifold={}|operators={}|aggregate_delta={}|entries={}",
        plan.manifold_signature,
        plan.operator_count,
        aggregate_applied_delta,
        adaptation_log
            .entries
            .iter()
            .map(|entry| format!(
                "{}:{}:{}:{}:{}",
                entry.sequence,
                entry.holdout_id,
                entry.semantic_context_used,
                entry.delta,
                entry.post_value,
            ))
            .collect::<Vec<_>>()
            .join("|"),
    );

    let routed_parameter_names = adaptation_log
        .entries
        .iter()
        .map(|entry| entry.parameter_name.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let routed_parameter_count = routed_parameter_names.len();

    let adaptation_profile_hash = format!("{:x}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        adaptation_signature.hash(&mut hasher);
        aggregate_applied_delta.hash(&mut hasher);
        plan.operator_plan_hash.hash(&mut hasher);
        routed_parameter_count.hash(&mut hasher);
        routed_parameter_names.hash(&mut hasher);
        hasher.finish()
    });

    let adaptation_well_formed = !adaptation_log.entries.is_empty() && routed_parameter_count > 0;

    Ok(Phase10RuntimeAdaptationBridge {
        manifold_signature: plan.manifold_signature.clone(),
        operator_plan_hash: plan.operator_plan_hash.clone(),
        adapted_entry_count: adaptation_log.entries.len(),
        aggregate_applied_delta,
        routed_parameter_count,
        routed_parameter_names,
        adaptation_log,
        adaptation_signature,
        adaptation_profile_hash,
        adaptation_well_formed,
    })
}

pub fn phase10_run_runtime_adaptation_episode(
    episode_id: &str,
    bridge: &Phase10RuntimeAdaptationBridge,
    registry: &Phase70StructuralParameterRegistry,
) -> Result<Phase80EpisodeTrace, String> {
    if !bridge.adaptation_well_formed {
        return Err("cannot run runtime adaptation episode with malformed bridge".to_string());
    }
    phase80_run_multiframe_episode(episode_id, &bridge.adaptation_log, registry)
}

pub fn phase10_emit_runtime_adaptation_telemetry(
    bridge: &Phase10RuntimeAdaptationBridge,
) -> String {
    let line = format!(
        "manifold_signature={}:operator_plan_hash={}:entry_count={}:aggregate_delta={}:routed_parameter_count={}:routed_parameters={}:adaptation_signature={}:adaptation_hash={}:well_formed={}",
        bridge.manifold_signature,
        bridge.operator_plan_hash,
        bridge.adapted_entry_count,
        bridge.aggregate_applied_delta,
        bridge.routed_parameter_count,
        bridge.routed_parameter_names.join(","),
        bridge.adaptation_signature,
        bridge.adaptation_profile_hash,
        bridge.adaptation_well_formed,
    );
    env::set_var(PHASE10_RUNTIME_ADAPTATION_TELEMETRY, &line);
    line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognition::phase62_structural_experiment::{
        phase80_build_phase9_integration_hook, phase80_integrate_cross_frame_structural_deltas,
        phase80_summarize_episode_structural_integration, Phase70StructuralParameterSpec,
    };
    use crate::cognition::phase90_cognitive_manifolds::phase90_form_cognitive_manifold;
    use crate::cognition::phase90_continuity_weighted_fields::phase90_form_continuity_weighted_field_from_seed;
    use crate::cognition::phase90_emergent_cognitive_shapes::phase90_compose_emergent_cognitive_shape;
    use crate::cognition::phase90_geometric_cognitive_seed::phase90_form_geometry_seed_from_integration_hook;
    use crate::cognition::phase90_geometry_driven_adjustment_operators::phase90_build_geometry_driven_adjustment_plan;
    use crate::cognition::phase90_multishape_interaction_dynamics::phase90_compute_multishape_interaction_dynamics;

    fn operator_plan_fixture() -> (
        Phase90GeometryDrivenAdjustmentPlan,
        Phase70StructuralParameterRegistry,
    ) {
        let registry = Phase70StructuralParameterRegistry::canonical();

        let shape_from_log = |episode_id: &str, log: &Phase70AdjustmentLog| {
            let trace = phase80_run_multiframe_episode(episode_id, log, &registry).expect("trace");
            let deltas = phase80_integrate_cross_frame_structural_deltas(&trace, log, &registry)
                .expect("deltas");
            let summary = phase80_summarize_episode_structural_integration(&trace, log, &registry)
                .expect("summary");
            let hook = phase80_build_phase9_integration_hook(&summary, &deltas);
            let seed = phase90_form_geometry_seed_from_integration_hook(&hook, &summary, &deltas);
            let field = phase90_form_continuity_weighted_field_from_seed(&seed);
            phase90_compose_emergent_cognitive_shape(&[field]).expect("shape")
        };

        let shape_a = shape_from_log(
            "phase10_fixture_a",
            &Phase70AdjustmentLog {
                entries: vec![
                    Phase70AdjustmentLogEntry {
                        sequence: 1,
                        holdout_id: "phase10_a1".to_string(),
                        parameter_name: "continuity_pressure_boost".to_string(),
                        semantic_context_used: "continuity_insensitive".to_string(),
                        adjustment_applied: true,
                        pre_value: 0,
                        post_value: 1,
                        delta: 1,
                        inverse_delta: -1,
                    },
                    Phase70AdjustmentLogEntry {
                        sequence: 2,
                        holdout_id: "phase10_a2".to_string(),
                        parameter_name: "continuity_pressure_boost".to_string(),
                        semantic_context_used: "none".to_string(),
                        adjustment_applied: true,
                        pre_value: 1,
                        post_value: 2,
                        delta: 1,
                        inverse_delta: -1,
                    },
                ],
            },
        );

        let shape_b = shape_from_log(
            "phase10_fixture_b",
            &Phase70AdjustmentLog {
                entries: vec![
                    Phase70AdjustmentLogEntry {
                        sequence: 1,
                        holdout_id: "phase10_b1".to_string(),
                        parameter_name: "continuity_pressure_boost".to_string(),
                        semantic_context_used: "continuity_insensitive".to_string(),
                        adjustment_applied: true,
                        pre_value: 0,
                        post_value: 1,
                        delta: 1,
                        inverse_delta: -1,
                    },
                    Phase70AdjustmentLogEntry {
                        sequence: 2,
                        holdout_id: "phase10_b2".to_string(),
                        parameter_name: "continuity_pressure_boost".to_string(),
                        semantic_context_used: "none".to_string(),
                        adjustment_applied: false,
                        pre_value: 1,
                        post_value: 1,
                        delta: 0,
                        inverse_delta: 0,
                    },
                ],
            },
        );

        let manifold = phase90_form_cognitive_manifold(&[shape_a, shape_b]).expect("manifold");
        let dynamics = phase90_compute_multishape_interaction_dynamics(&manifold).expect("dynamics");
        let plan = phase90_build_geometry_driven_adjustment_plan(&manifold, &dynamics).expect("plan");
        (plan, registry)
    }

    #[test]
    fn phase10_slice1_builds_runtime_adaptation_bridge_from_operator_plan() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");

        assert!(bridge.adaptation_well_formed);
        assert!(bridge.adapted_entry_count > 0);
        assert!(!bridge.adaptation_signature.is_empty());
        assert!(!bridge.adaptation_profile_hash.is_empty());
    }

    #[test]
    fn phase10_slice1_bridge_log_is_phase70_invariant_valid() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");

        let validation = phase70_validate_adjustment_log_invariants(&bridge.adaptation_log, &registry);
        assert!(validation.is_ok());
    }

    #[test]
    fn phase10_slice1_adapted_episode_is_replay_stable() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");

        let trace_a =
            phase10_run_runtime_adaptation_episode("phase10_adapted", &bridge, &registry)
                .expect("trace a");
        let trace_b =
            phase10_run_runtime_adaptation_episode("phase10_adapted", &bridge, &registry)
                .expect("trace b");

        assert_eq!(trace_a, trace_b);
    }

    #[test]
    fn phase10_slice1_runtime_adaptation_telemetry_is_canonical() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");

        let telemetry_a = phase10_emit_runtime_adaptation_telemetry(&bridge);
        let telemetry_b = phase10_emit_runtime_adaptation_telemetry(&bridge);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("manifold_signature="));
        assert!(telemetry_a.contains("operator_plan_hash="));
        assert!(telemetry_a.contains("entry_count="));
        assert!(telemetry_a.contains("aggregate_delta="));
        assert!(telemetry_a.contains("routed_parameter_count="));
        assert!(telemetry_a.contains("routed_parameters="));
        assert!(telemetry_a.contains("well_formed="));
    }

    #[test]
    fn phase10_slice2_routes_operators_across_multiple_parameters() {
        let (plan_from_fixture, mut registry) = operator_plan_fixture();
        let plan = Phase90GeometryDrivenAdjustmentPlan {
            manifold_signature: plan_from_fixture.manifold_signature,
            operator_count: 4,
            dominant_operator_kind: "continuity_lift".to_string(),
            aggregate_adjustment_pressure_percent: 72,
            operators: vec![
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_op_1".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 80,
                    continuity_bias_percent: 70,
                    manifold_alignment_percent: 75,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_op_2".to_string(),
                    target_shape_index: 2,
                    source_shape_index: 0,
                    operator_kind: "gradient_smooth".to_string(),
                    adjustment_pressure_percent: 78,
                    continuity_bias_percent: 74,
                    manifold_alignment_percent: 71,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_op_3".to_string(),
                    target_shape_index: 1,
                    source_shape_index: 2,
                    operator_kind: "resonance_reweight".to_string(),
                    adjustment_pressure_percent: 82,
                    continuity_bias_percent: 68,
                    manifold_alignment_percent: 73,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_op_4".to_string(),
                    target_shape_index: 3,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 75,
                    continuity_bias_percent: 72,
                    manifold_alignment_percent: 70,
                },
            ],
            operator_plan_signature: "slice2_multi_parameter_fixture".to_string(),
            operator_plan_hash: "slice2_multi_parameter_fixture_hash".to_string(),
            operator_plan_well_formed: true,
        };
        registry.parameters = vec![
            Phase70StructuralParameterSpec {
                name: "continuity_pressure_boost".to_string(),
                env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
                min_value: 0,
                max_value: 6,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70StructuralParameterSpec {
                name: "coherence_gradient_trim".to_string(),
                env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
                min_value: 0,
                max_value: 4,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70StructuralParameterSpec {
                name: "boundary_tension_relief".to_string(),
                env_key: "GORT_PHASE70_BOUNDARY_TENSION_RELIEF".to_string(),
                min_value: 0,
                max_value: 5,
                delta: 1,
                inverse_delta: -1,
            },
        ];

        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");

        assert!(bridge.adaptation_well_formed);
        assert!(bridge.routed_parameter_count >= 2);
        assert!(bridge
            .routed_parameter_names
            .contains(&"continuity_pressure_boost".to_string()));
        assert!(bridge
            .routed_parameter_names
            .iter()
            .any(|name| name == "coherence_gradient_trim" || name == "boundary_tension_relief"));
    }

    #[test]
    fn phase10_slice2_multi_parameter_routing_is_replay_stable() {
        let (plan_from_fixture, mut registry) = operator_plan_fixture();
        let plan = Phase90GeometryDrivenAdjustmentPlan {
            manifold_signature: plan_from_fixture.manifold_signature,
            operator_count: 3,
            dominant_operator_kind: "continuity_lift".to_string(),
            aggregate_adjustment_pressure_percent: 70,
            operators: vec![
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_replay_op_1".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 80,
                    continuity_bias_percent: 70,
                    manifold_alignment_percent: 75,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_replay_op_2".to_string(),
                    target_shape_index: 2,
                    source_shape_index: 0,
                    operator_kind: "gradient_smooth".to_string(),
                    adjustment_pressure_percent: 78,
                    continuity_bias_percent: 74,
                    manifold_alignment_percent: 71,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_replay_op_3".to_string(),
                    target_shape_index: 1,
                    source_shape_index: 2,
                    operator_kind: "resonance_reweight".to_string(),
                    adjustment_pressure_percent: 82,
                    continuity_bias_percent: 68,
                    manifold_alignment_percent: 73,
                },
            ],
            operator_plan_signature: "slice2_replay_fixture".to_string(),
            operator_plan_hash: "slice2_replay_fixture_hash".to_string(),
            operator_plan_well_formed: true,
        };
        registry.parameters = vec![
            Phase70StructuralParameterSpec {
                name: "continuity_pressure_boost".to_string(),
                env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
                min_value: 0,
                max_value: 6,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70StructuralParameterSpec {
                name: "coherence_gradient_trim".to_string(),
                env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
                min_value: 0,
                max_value: 4,
                delta: 1,
                inverse_delta: -1,
            },
        ];

        let bridge_a = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge a");
        let bridge_b = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge b");

        assert_eq!(bridge_a.adaptation_profile_hash, bridge_b.adaptation_profile_hash);
        assert_eq!(bridge_a.adaptation_log, bridge_b.adaptation_log);
        assert_eq!(bridge_a.routed_parameter_names, bridge_b.routed_parameter_names);
    }

    #[test]
    fn phase10_slice2_acceptance_gate_enforces_diversity_above_threshold() {
        let (plan_from_fixture, mut registry) = operator_plan_fixture();
        let plan = Phase90GeometryDrivenAdjustmentPlan {
            manifold_signature: plan_from_fixture.manifold_signature,
            operator_count: 4,
            dominant_operator_kind: "continuity_lift".to_string(),
            aggregate_adjustment_pressure_percent: 72,
            operators: vec![
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_gate_op_1".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 80,
                    continuity_bias_percent: 70,
                    manifold_alignment_percent: 75,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_gate_op_2".to_string(),
                    target_shape_index: 2,
                    source_shape_index: 0,
                    operator_kind: "gradient_smooth".to_string(),
                    adjustment_pressure_percent: 78,
                    continuity_bias_percent: 74,
                    manifold_alignment_percent: 71,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_gate_op_3".to_string(),
                    target_shape_index: 1,
                    source_shape_index: 2,
                    operator_kind: "resonance_reweight".to_string(),
                    adjustment_pressure_percent: 82,
                    continuity_bias_percent: 68,
                    manifold_alignment_percent: 73,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_gate_op_4".to_string(),
                    target_shape_index: 3,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 75,
                    continuity_bias_percent: 72,
                    manifold_alignment_percent: 70,
                },
            ],
            operator_plan_signature: "slice2_acceptance_fixture".to_string(),
            operator_plan_hash: "slice2_acceptance_fixture_hash".to_string(),
            operator_plan_well_formed: true,
        };
        registry.parameters = vec![
            Phase70StructuralParameterSpec {
                name: "continuity_pressure_boost".to_string(),
                env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
                min_value: 0,
                max_value: 6,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70StructuralParameterSpec {
                name: "coherence_gradient_trim".to_string(),
                env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
                min_value: 0,
                max_value: 4,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70StructuralParameterSpec {
                name: "boundary_tension_relief".to_string(),
                env_key: "GORT_PHASE70_BOUNDARY_TENSION_RELIEF".to_string(),
                min_value: 0,
                max_value: 5,
                delta: 1,
                inverse_delta: -1,
            },
        ];

        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
        let policy = Phase10Slice2RoutingAcceptancePolicy::canonical();

        phase10_validate_slice2_routing_acceptance_gate(&plan, &bridge, &policy)
            .expect("slice2 acceptance should pass with diverse routing");
    }

    #[test]
    fn phase10_slice2_acceptance_gate_rejects_low_diversity_when_threshold_met() {
        let (plan_from_fixture, mut registry) = operator_plan_fixture();
        let plan = Phase90GeometryDrivenAdjustmentPlan {
            manifold_signature: plan_from_fixture.manifold_signature,
            operator_count: 4,
            dominant_operator_kind: "continuity_lift".to_string(),
            aggregate_adjustment_pressure_percent: 72,
            operators: vec![
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_reject_op_1".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 80,
                    continuity_bias_percent: 70,
                    manifold_alignment_percent: 75,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_reject_op_2".to_string(),
                    target_shape_index: 2,
                    source_shape_index: 0,
                    operator_kind: "gradient_smooth".to_string(),
                    adjustment_pressure_percent: 78,
                    continuity_bias_percent: 74,
                    manifold_alignment_percent: 71,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_reject_op_3".to_string(),
                    target_shape_index: 1,
                    source_shape_index: 2,
                    operator_kind: "resonance_reweight".to_string(),
                    adjustment_pressure_percent: 82,
                    continuity_bias_percent: 68,
                    manifold_alignment_percent: 73,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_reject_op_4".to_string(),
                    target_shape_index: 3,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 75,
                    continuity_bias_percent: 72,
                    manifold_alignment_percent: 70,
                },
            ],
            operator_plan_signature: "slice2_reject_fixture".to_string(),
            operator_plan_hash: "slice2_reject_fixture_hash".to_string(),
            operator_plan_well_formed: true,
        };
        registry.parameters = vec![Phase70StructuralParameterSpec {
            name: "continuity_pressure_boost".to_string(),
            env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
            min_value: 0,
            max_value: 6,
            delta: 1,
            inverse_delta: -1,
        }];

        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
        let policy = Phase10Slice2RoutingAcceptancePolicy::canonical();

        let err = phase10_validate_slice2_routing_acceptance_gate(&plan, &bridge, &policy)
            .expect_err("slice2 acceptance should reject low diversity");
        assert!(err.contains("routed parameter diversity"));
    }

    #[test]
    fn phase10_slice2_acceptance_gate_bypasses_diversity_below_threshold() {
        let (plan_from_fixture, mut registry) = operator_plan_fixture();
        let plan = Phase90GeometryDrivenAdjustmentPlan {
            manifold_signature: plan_from_fixture.manifold_signature,
            operator_count: 2,
            dominant_operator_kind: "continuity_lift".to_string(),
            aggregate_adjustment_pressure_percent: 70,
            operators: vec![
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_bypass_op_1".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 80,
                    continuity_bias_percent: 70,
                    manifold_alignment_percent: 75,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice2_bypass_op_2".to_string(),
                    target_shape_index: 1,
                    source_shape_index: 0,
                    operator_kind: "gradient_smooth".to_string(),
                    adjustment_pressure_percent: 78,
                    continuity_bias_percent: 74,
                    manifold_alignment_percent: 71,
                },
            ],
            operator_plan_signature: "slice2_bypass_fixture".to_string(),
            operator_plan_hash: "slice2_bypass_fixture_hash".to_string(),
            operator_plan_well_formed: true,
        };
        registry.parameters = vec![Phase70StructuralParameterSpec {
            name: "continuity_pressure_boost".to_string(),
            env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
            min_value: 0,
            max_value: 6,
            delta: 1,
            inverse_delta: -1,
        }];

        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
        let policy = Phase10Slice2RoutingAcceptancePolicy::canonical();

        phase10_validate_slice2_routing_acceptance_gate(&plan, &bridge, &policy)
            .expect("slice2 acceptance should bypass diversity below threshold");
    }

    #[test]
    fn phase10_slice3_integrates_operator_weighted_runtime_transitions() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");

        let integration =
            phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
                .expect("integration");

        assert!(integration.integration_well_formed);
        assert!(integration.weighted_transition_count > 0);
        assert!(integration.aggregate_operator_weight_percent > 0);
        assert!(integration
            .weighted_transitions
            .iter()
            .all(|transition| transition.transition_reason.contains("operator_kind=")));
    }

    #[test]
    fn phase10_slice3_operator_runtime_integration_is_replay_stable() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");

        let integration_a =
            phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
                .expect("integration a");
        let integration_b =
            phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
                .expect("integration b");

        assert_eq!(integration_a, integration_b);
        assert_eq!(
            integration_a.integration_profile_hash,
            integration_b.integration_profile_hash
        );
    }

    #[test]
    fn phase10_slice3_operator_runtime_integration_telemetry_is_canonical() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
        let integration =
            phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
                .expect("integration");

        let telemetry_a = phase10_emit_operator_runtime_integration_telemetry(&integration);
        let telemetry_b = phase10_emit_operator_runtime_integration_telemetry(&integration);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("operator_plan_hash="));
        assert!(telemetry_a.contains("adaptation_profile_hash="));
        assert!(telemetry_a.contains("weighted_transition_count="));
        assert!(telemetry_a.contains("aggregate_operator_weight="));
        assert!(telemetry_a.contains("well_formed=true"));
    }

    #[test]
    fn phase10_slice4_runtime_continuity_preservation_accepts_canonical_integration() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
        let integration =
            phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
                .expect("integration");
        let policy = Phase10Slice4RuntimeContinuityPolicy::canonical();

        phase10_validate_runtime_continuity_preservation(&plan, &bridge, &integration, &policy)
            .expect("canonical integration should preserve runtime continuity");
    }

    #[test]
    fn phase10_slice4_runtime_continuity_rejects_violation_with_canonical_reason_code() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
        let mut integration =
            phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
                .expect("integration");
        integration.weighted_transitions[0].continuity_preserved = false;
        integration.weighted_transitions[0].transition_reason =
            "phase10_forced_continuity_break".to_string();

        let policy = Phase10Slice4RuntimeContinuityPolicy::canonical();
        let err = phase10_validate_runtime_continuity_preservation(
            &plan,
            &bridge,
            &integration,
            &policy,
        )
        .expect_err("continuity violation must be rejected");

        assert_eq!(err, PHASE10_SLICE4_REJECT_CONTINUITY_VIOLATION);
    }

    #[test]
    fn phase10_slice4_runtime_continuity_validation_is_replay_stable() {
        let (plan, registry) = operator_plan_fixture();
        let bridge = phase10_build_runtime_adaptation_bridge(&plan, &registry).expect("bridge");
        let integration =
            phase10_integrate_operator_plan_into_runtime_transitions(&plan, &bridge, &registry)
                .expect("integration");
        let policy = Phase10Slice4RuntimeContinuityPolicy::canonical();

        let result_a =
            phase10_validate_runtime_continuity_preservation(&plan, &bridge, &integration, &policy);
        let result_b =
            phase10_validate_runtime_continuity_preservation(&plan, &bridge, &integration, &policy);

        assert_eq!(result_a, result_b);
        assert!(result_a.is_ok());
    }

    #[test]
    fn phase10_slice5_closed_loop_integrity_stage_is_well_formed_end_to_end() {
        let (plan_from_fixture, mut registry) = operator_plan_fixture();
        let plan = Phase90GeometryDrivenAdjustmentPlan {
            manifold_signature: plan_from_fixture.manifold_signature,
            operator_count: 4,
            dominant_operator_kind: "continuity_lift".to_string(),
            aggregate_adjustment_pressure_percent: 72,
            operators: vec![
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice5_stage_op_1".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 0,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 80,
                    continuity_bias_percent: 70,
                    manifold_alignment_percent: 75,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice5_stage_op_2".to_string(),
                    target_shape_index: 1,
                    source_shape_index: 0,
                    operator_kind: "gradient_smooth".to_string(),
                    adjustment_pressure_percent: 78,
                    continuity_bias_percent: 74,
                    manifold_alignment_percent: 71,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice5_stage_op_3".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 1,
                    operator_kind: "resonance_reweight".to_string(),
                    adjustment_pressure_percent: 82,
                    continuity_bias_percent: 68,
                    manifold_alignment_percent: 73,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice5_stage_op_4".to_string(),
                    target_shape_index: 1,
                    source_shape_index: 1,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 75,
                    continuity_bias_percent: 72,
                    manifold_alignment_percent: 70,
                },
            ],
            operator_plan_signature: "slice5_closed_loop_fixture".to_string(),
            operator_plan_hash: "slice5_closed_loop_fixture_hash".to_string(),
            operator_plan_well_formed: true,
        };
        registry.parameters = vec![
            Phase70StructuralParameterSpec {
                name: "continuity_pressure_boost".to_string(),
                env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
                min_value: 0,
                max_value: 6,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70StructuralParameterSpec {
                name: "coherence_gradient_trim".to_string(),
                env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
                min_value: 0,
                max_value: 4,
                delta: 1,
                inverse_delta: -1,
            },
        ];

        let stage = phase10_execute_closed_loop_integrity_stage(
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        )
        .expect("stage");

        assert!(stage.integrity_well_formed);
        assert!(stage.continuity_gate_passed);
        assert!(stage.routed_parameter_count >= 2);
        assert!(stage.weighted_transition_count > 0);
    }

    #[test]
    fn phase10_slice5_closed_loop_integrity_stage_replay_is_stable() {
        let (plan_from_fixture, mut registry) = operator_plan_fixture();
        let plan = Phase90GeometryDrivenAdjustmentPlan {
            manifold_signature: plan_from_fixture.manifold_signature,
            operator_count: 3,
            dominant_operator_kind: "continuity_lift".to_string(),
            aggregate_adjustment_pressure_percent: 72,
            operators: vec![
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice5_replay_op_1".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 0,
                    operator_kind: "continuity_lift".to_string(),
                    adjustment_pressure_percent: 80,
                    continuity_bias_percent: 70,
                    manifold_alignment_percent: 75,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice5_replay_op_2".to_string(),
                    target_shape_index: 1,
                    source_shape_index: 0,
                    operator_kind: "gradient_smooth".to_string(),
                    adjustment_pressure_percent: 78,
                    continuity_bias_percent: 74,
                    manifold_alignment_percent: 71,
                },
                crate::cognition::phase90_geometry_driven_adjustment_operators::Phase90GeometryDrivenAdjustmentOperator {
                    operator_id: "slice5_replay_op_3".to_string(),
                    target_shape_index: 0,
                    source_shape_index: 1,
                    operator_kind: "resonance_reweight".to_string(),
                    adjustment_pressure_percent: 82,
                    continuity_bias_percent: 68,
                    manifold_alignment_percent: 73,
                },
            ],
            operator_plan_signature: "slice5_replay_fixture".to_string(),
            operator_plan_hash: "slice5_replay_fixture_hash".to_string(),
            operator_plan_well_formed: true,
        };
        registry.parameters = vec![
            Phase70StructuralParameterSpec {
                name: "continuity_pressure_boost".to_string(),
                env_key: "GORT_PHASE70_CONTINUITY_PRESSURE_BOOST".to_string(),
                min_value: 0,
                max_value: 6,
                delta: 1,
                inverse_delta: -1,
            },
            Phase70StructuralParameterSpec {
                name: "coherence_gradient_trim".to_string(),
                env_key: "GORT_PHASE70_COHERENCE_GRADIENT_TRIM".to_string(),
                min_value: 0,
                max_value: 4,
                delta: 1,
                inverse_delta: -1,
            },
        ];

        let stage_a = phase10_execute_closed_loop_integrity_stage(
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        )
        .expect("stage a");
        let stage_b = phase10_execute_closed_loop_integrity_stage(
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        )
        .expect("stage b");

        assert_eq!(stage_a, stage_b);
        assert_eq!(stage_a.integrity_profile_hash, stage_b.integrity_profile_hash);
    }

    #[test]
    fn phase10_slice6_regenerates_phase9_seed_from_adapted_episode() {
        let (plan, registry) = operator_plan_fixture();
        let feedback = phase10_regenerate_phase9_seed_from_adapted_episode(
            "phase10_slice6_feedback",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        )
        .expect("feedback");

        assert!(feedback.feedback_well_formed);
        assert!(feedback.regenerated_hook.integration_ready);
        assert!(feedback.regenerated_seed.geometry_well_formed);
        assert_eq!(feedback.regenerated_seed.episode_id, "phase10_slice6_feedback");
    }

    #[test]
    fn phase10_slice6_feedback_seed_regeneration_is_replay_stable() {
        let (plan, registry) = operator_plan_fixture();
        let feedback_a = phase10_regenerate_phase9_seed_from_adapted_episode(
            "phase10_slice6_feedback_replay",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        )
        .expect("feedback a");
        let feedback_b = phase10_regenerate_phase9_seed_from_adapted_episode(
            "phase10_slice6_feedback_replay",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        )
        .expect("feedback b");

        assert_eq!(feedback_a, feedback_b);
        assert_eq!(feedback_a.feedback_profile_hash, feedback_b.feedback_profile_hash);
        assert_eq!(feedback_a.regenerated_seed, feedback_b.regenerated_seed);
    }

    #[test]
    fn phase10_slice6_feedback_telemetry_is_canonical() {
        let (plan, registry) = operator_plan_fixture();
        let feedback = phase10_regenerate_phase9_seed_from_adapted_episode(
            "phase10_slice6_feedback_telemetry",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
        )
        .expect("feedback");

        let telemetry_a = phase10_emit_runtime_feedback_telemetry(&feedback);
        let telemetry_b = phase10_emit_runtime_feedback_telemetry(&feedback);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("operator_plan_hash="));
        assert!(telemetry_a.contains("integrity_profile_hash="));
        assert!(telemetry_a.contains("seed_hash="));
        assert!(telemetry_a.contains("hook_ready=true"));
        assert!(telemetry_a.contains("well_formed=true"));
    }

    #[test]
    fn phase10_top_level_acceptance_stage_is_replay_stable_over_repeated_regenerations() {
        let (plan, registry) = operator_plan_fixture();
        let acceptance = phase10_run_top_level_acceptance_stage(
            "phase10_top_level_acceptance",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            25,
        )
        .expect("acceptance");

        assert!(acceptance.acceptance_well_formed);
        assert_eq!(acceptance.replay_count, 25);
        assert!(!acceptance.baseline_feedback_hash.is_empty());
        assert!(!acceptance.baseline_seed_hash.is_empty());
    }

    #[test]
    fn phase10_top_level_acceptance_stage_rejects_low_replay_count_with_canonical_code() {
        let (plan, registry) = operator_plan_fixture();
        let err = phase10_run_top_level_acceptance_stage(
            "phase10_top_level_acceptance_reject",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            1,
        )
        .expect_err("replay count below threshold must be rejected");

        assert_eq!(err, PHASE10_TOP_LEVEL_REJECT_REPLAY_COUNT_TOO_LOW);
    }

    #[test]
    fn phase10_top_level_acceptance_telemetry_is_canonical() {
        let (plan, registry) = operator_plan_fixture();
        let acceptance = phase10_run_top_level_acceptance_stage(
            "phase10_top_level_acceptance_telemetry",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            10,
        )
        .expect("acceptance");

        let telemetry_a = phase10_emit_top_level_acceptance_telemetry(&acceptance);
        let telemetry_b = phase10_emit_top_level_acceptance_telemetry(&acceptance);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("operator_plan_hash="));
        assert!(telemetry_a.contains("replay_count="));
        assert!(telemetry_a.contains("baseline_feedback_hash="));
        assert!(telemetry_a.contains("baseline_seed_hash="));
        assert!(telemetry_a.contains("well_formed=true"));
    }

    #[test]
    fn phase10_slice7_multicycle_replay_acceptance_is_well_formed() {
        let (plan, registry) = operator_plan_fixture();
        let acceptance = phase10_run_slice7_multicycle_replay_acceptance_stage(
            "phase10_slice7_multicycle",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            8,
        )
        .expect("slice7 acceptance");

        assert!(acceptance.acceptance_well_formed);
        assert_eq!(acceptance.cycle_count, 8);
        assert_eq!(acceptance.cycle_plan_hashes.len(), 8);
        assert_eq!(acceptance.cycle_feedback_hashes.len(), 8);
        assert_eq!(acceptance.cycle_seed_hashes.len(), 8);
    }

    #[test]
    fn phase10_slice7_multicycle_replay_acceptance_is_replay_stable() {
        let (plan, registry) = operator_plan_fixture();
        let acceptance_a = phase10_run_slice7_multicycle_replay_acceptance_stage(
            "phase10_slice7_multicycle_replay",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            6,
        )
        .expect("acceptance a");
        let acceptance_b = phase10_run_slice7_multicycle_replay_acceptance_stage(
            "phase10_slice7_multicycle_replay",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            6,
        )
        .expect("acceptance b");

        assert_eq!(acceptance_a, acceptance_b);
        assert_eq!(
            acceptance_a.acceptance_profile_hash,
            acceptance_b.acceptance_profile_hash
        );
    }

    #[test]
    fn phase10_slice7_multicycle_replay_rejects_low_cycle_count_with_canonical_code() {
        let (plan, registry) = operator_plan_fixture();
        let err = phase10_run_slice7_multicycle_replay_acceptance_stage(
            "phase10_slice7_multicycle_reject",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            1,
        )
        .expect_err("cycle count below threshold must be rejected");

        assert_eq!(err, PHASE10_SLICE7_REJECT_CYCLE_COUNT_TOO_LOW);
    }

    #[test]
    fn phase10_slice7_multicycle_telemetry_is_canonical() {
        let (plan, registry) = operator_plan_fixture();
        let acceptance = phase10_run_slice7_multicycle_replay_acceptance_stage(
            "phase10_slice7_multicycle_telemetry",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            5,
        )
        .expect("slice7 acceptance");

        let telemetry_a = phase10_emit_slice7_multicycle_telemetry(&acceptance);
        let telemetry_b = phase10_emit_slice7_multicycle_telemetry(&acceptance);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("baseline_plan_hash="));
        assert!(telemetry_a.contains("cycle_count=5"));
        assert!(telemetry_a.contains("baseline_operator_weight="));
        assert!(telemetry_a.contains("acceptance_hash="));
        assert!(telemetry_a.contains("well_formed=true"));
    }

    #[test]
    fn phase11_multi_loop_convergence_is_well_formed() {
        let (plan, registry) = operator_plan_fixture();
        let convergence = phase11_run_multi_loop_convergence_stage(
            "phase11_multi_loop",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            6,
            5,
        )
        .expect("convergence");

        assert!(convergence.convergence_well_formed);
        assert_eq!(convergence.loop_count, 6);
        assert_eq!(convergence.cycle_count_per_loop, 5);
        assert_eq!(convergence.loop_acceptance_hashes.len(), 6);
        assert_eq!(convergence.convergence_loop_index, 1);
    }

    #[test]
    fn phase11_multi_loop_convergence_is_replay_stable() {
        let (plan, registry) = operator_plan_fixture();
        let stage_a = phase11_run_multi_loop_convergence_stage(
            "phase11_multi_loop_replay",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            4,
            4,
        )
        .expect("stage a");
        let stage_b = phase11_run_multi_loop_convergence_stage(
            "phase11_multi_loop_replay",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            4,
            4,
        )
        .expect("stage b");

        assert_eq!(stage_a, stage_b);
        assert_eq!(stage_a.convergence_profile_hash, stage_b.convergence_profile_hash);
    }

    #[test]
    fn phase11_multi_loop_convergence_rejects_low_loop_count_with_canonical_code() {
        let (plan, registry) = operator_plan_fixture();
        let err = phase11_run_multi_loop_convergence_stage(
            "phase11_multi_loop_reject",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            1,
            4,
        )
        .expect_err("low loop count should be rejected");

        assert_eq!(err, PHASE11_REJECT_LOOP_COUNT_TOO_LOW);
    }

    #[test]
    fn phase11_multi_loop_convergence_telemetry_is_canonical() {
        let (plan, registry) = operator_plan_fixture();
        let convergence = phase11_run_multi_loop_convergence_stage(
            "phase11_multi_loop_telemetry",
            &plan,
            &registry,
            &Phase10Slice2RoutingAcceptancePolicy::canonical(),
            &Phase10Slice4RuntimeContinuityPolicy::canonical(),
            3,
            3,
        )
        .expect("convergence");

        let telemetry_a = phase11_emit_multi_loop_convergence_telemetry(&convergence);
        let telemetry_b = phase11_emit_multi_loop_convergence_telemetry(&convergence);

        assert_eq!(telemetry_a, telemetry_b);
        assert!(telemetry_a.contains("baseline_plan_hash="));
        assert!(telemetry_a.contains("loop_count=3"));
        assert!(telemetry_a.contains("cycle_count_per_loop=3"));
        assert!(telemetry_a.contains("convergence_hash="));
        assert!(telemetry_a.contains("well_formed=true"));
    }
}
