//! GORT: Geometric Operator-Regulated Thought
//!
//! A deterministic geometric reasoning engine built on Rust invariants,
//! grounded in mathematical rigor, and designed for formal verification.
//!
//! ## Architecture
//!
//! GORT forms the missing middle between the UGC geometric representational layer
//! and CPU hardware execution. It provides:
//!
//! - **Geometric Primitives** (`geom/`): Space, fields, and resonance modes
//! - **Semantic Reasoning** (`cognition/`): Nodes, constraints, and evaluators
//! - **Deterministic Runtime** (`runtime/`): Parallel execution with reproducibility
//!
//! ## Core Invariants
//!
//! All reasoning operations must satisfy four non-negotiable invariants:
//!
//! 1. **Determinism**: Same input + state produces identical output (byte-stable)
//! 2. **Consistency**: No contradictory conclusions in final state
//! 3. **Closure**: All frames reach deterministic final states
//! 4. **Auditability**: All derivations traceable and reproducible
//!
//! These invariants are enforced by the cognitive kernel in `geom::invariants`.

pub mod geom {
    pub mod invariants;
    pub mod space;
    pub mod field;
    pub mod mode;

    pub use invariants::{
        CoreInvariant, ClosureStatus, ClosureTransition, InvariantViolation,
        GeometricState, ConstraintEvaluator, ConstraintSystem,
    };
    pub use space::{Coordinate3, GeometricSpace, Metric, Scalar};
    pub use field::{ConceptCluster, FieldPoint, SemanticField};
    pub use mode::{ArithmeticMode, ResonanceMode, ResonanceTransform};
}

pub mod cognition {
    pub mod node;
    pub mod constraint;
    pub mod evaluator;
    pub mod scheduler;
    pub mod multiframe;
    pub mod phase61_structural_recovery;
    pub mod phase62_structural_experiment;

    pub use constraint::ConstraintKind;
    pub use constraint::SemanticConstraint;
    pub use evaluator::ConstraintStatus;
    pub use evaluator::DisambiguationDecision;
    pub use evaluator::ParallelResolutionSummary;
    pub use evaluator::ConstraintEvalEngine;
    pub use evaluator::SenseInterferenceScore;
    pub use multiframe::{
        anchor_derived_relational_distance, compare_topologies, compute_cognitive_flow_field,
        compute_intent_field, compute_preference_gradients,
        compute_cognitive_potential_field, compute_cognitive_topology,
        compute_energy_minimizing_trajectory, detect_phase_transition, select_action,
        select_goal_directed_trajectory, build_goal_set, compute_conflict_gradients,
        arbitrate_intent_field, resolve_trajectory, build_goal_hierarchy,
        compute_meta_preference_gradients, compute_self_coherence_metric,
        build_meta_intent_field, resolve_meta_intent_trajectory,
        track_manifold_evolution, ActionSelectionPolicy, AnchorRegistry, AnchorRelationalDistance,
        ArbitratedIntentField, CognitiveFlowField, CognitivePotentialField, CognitiveTopology,
        ConceptAnchor, ConceptFlowVector, ConflictGradient, ConflictResolvedTrajectory,
        ConsolidatedMemory, EnergyGradient, EnergyMinimizingTrajectory,
        EmergentConcept, FlowPrediction, FrameIterationResult, GoalAttractor, GoalSet,
        GoalHierarchy, GoalStabilityMetric, IntentDrivenTrajectory, IntentField,
        MetaIntentField, MetaIntentTrajectory, MetaPreferenceGradient, SelfCoherenceMetric,
        ManifoldDrift,
        ManifoldEvolutionTrace, MultiFrameCognition, MultiFrameConfig, MultiFrameIteration,
        MultiFrameReport, PreferenceGradient, RegionFlowVector, StabilizationMetrics,
        StableSense, StabilityEnergy, TopologicalNeighborhood, TopologicalRegion,
        TopologyEvolutionStep, TopologyMetrics,
    };
    pub use node::{CognitiveFrame, SemanticNode};
    pub use scheduler::{ScheduledTask, TaskScheduler};
    pub use phase61_structural_recovery::{
        Phase61RuntimePolicy, Phase61SignalSnapshot, Phase61StructuralRecoveryConfig,
        Phase61StructuralRecoveryState,
    };
    pub use phase62_structural_experiment::{
        apply_phase62_structural_experiment, Phase62ExperimentKind, Phase62ExperimentPlan,
        Phase62RuntimeSummary, Phase62StructuralConfig, Phase62StructuralReport,
        Phase62V3Branch, Phase63Kind, Phase63RegionRole, Phase63RepairOperator,
        Phase63RepairPlan, Phase63RepairTarget, Phase63RuntimeSummary, Phase63Telemetry,
        Phase70AdjustmentLog, Phase70AdjustmentLogEntry, Phase70AdjustmentParameter,
        Phase70SemanticMappingRule, Phase70SemanticMappingTable,
        Phase80CrossFrameStructuralDelta, Phase80EpisodeStructuralSummary,
        Phase80Phase9IntegrationHook,
        Phase80EpisodeStep, Phase80EpisodeTrace,
        Phase80FrameTransitionEvent,
        Phase80FrameLocalParameterRegistry,
        Phase80FrameParameterSnapshot, Phase80FrameParameterValue,
        Phase80FrameStructuralContext, Phase80MultiFrameStructuralContext,
        Phase70StructuralParameterRegistry,
        Phase70StructuralParameterSpec, Phase70Telemetry,
        phase80_build_frame_local_parameter_registry,
        phase80_emit_episode_telemetry,
        phase80_emit_frame_transition_telemetry,
        phase80_emit_integration_telemetry,
        phase80_integrate_cross_frame_structural_deltas,
        phase80_resolve_frame_parameter,
        phase80_run_multiframe_episode,
        phase80_summarize_episode_structural_integration,
        phase80_build_phase9_integration_hook,
        phase80_sequence_frame_transitions,
        phase80_scaffold_frame_transitions,
        phase80_scaffold_frame_parameter_snapshots,
        phase80_scaffold_multiframe_structural_context,
        phase80_validate_frame_continuity_invariants,
        phase70_validate_adjustment_log_invariants,
        scaffold_phase70_structural_adjustment,
    };
}

pub mod runtime {
    pub mod parallel;
    pub mod logging;
    pub mod determinism;

    pub use parallel::DeterministicRuntime;
    pub use logging::AuditLogger;
    pub use determinism::DeterminismVerifier;
}

pub use geom::{
    CoreInvariant, ClosureStatus, ClosureTransition, InvariantViolation,
    GeometricState, ConstraintEvaluator, ConstraintSystem,
};
pub use geom::{
    ArithmeticMode, ConceptCluster, Coordinate3, FieldPoint, GeometricSpace, ResonanceMode,
    ResonanceTransform, SemanticField,
};
pub use cognition::{
    anchor_derived_relational_distance,
    compare_topologies,
    compute_cognitive_flow_field,
    compute_intent_field,
    compute_preference_gradients,
    compute_cognitive_potential_field,
    compute_cognitive_topology,
    compute_energy_minimizing_trajectory,
    detect_phase_transition,
    select_action,
    select_goal_directed_trajectory,
    build_goal_set,
    compute_conflict_gradients,
    arbitrate_intent_field,
    resolve_trajectory,
    build_goal_hierarchy,
    compute_meta_preference_gradients,
    compute_self_coherence_metric,
    build_meta_intent_field,
    resolve_meta_intent_trajectory,
    track_manifold_evolution,
    ActionSelectionPolicy,
    AnchorRegistry,
    AnchorRelationalDistance,
    ArbitratedIntentField,
    CognitiveFlowField,
    CognitivePotentialField,
    CognitiveTopology,
    ConceptAnchor,
    ConceptFlowVector,
    ConflictGradient,
    ConflictResolvedTrajectory,
    ConsolidatedMemory,
    EnergyGradient,
    EnergyMinimizingTrajectory,
    EmergentConcept,
    FlowPrediction,
    Phase61RuntimePolicy,
    Phase61SignalSnapshot,
    Phase61StructuralRecoveryConfig,
    Phase61StructuralRecoveryState,
    Phase62ExperimentKind,
    Phase62ExperimentPlan,
    Phase62RuntimeSummary,
    Phase62StructuralConfig,
    Phase62StructuralReport,
    Phase62V3Branch,
    Phase63Kind,
    Phase63RegionRole,
    Phase63RepairOperator,
    Phase63RepairPlan,
    Phase63RepairTarget,
    Phase63RuntimeSummary,
    Phase63Telemetry,
    Phase70AdjustmentLog,
    Phase70AdjustmentLogEntry,
    Phase70AdjustmentParameter,
    Phase70SemanticMappingRule,
    Phase70SemanticMappingTable,
    Phase80CrossFrameStructuralDelta,
    Phase80EpisodeStructuralSummary,
    Phase80Phase9IntegrationHook,
    Phase80EpisodeStep,
    Phase80EpisodeTrace,
    Phase80FrameTransitionEvent,
    Phase80FrameLocalParameterRegistry,
    Phase80FrameParameterSnapshot,
    Phase80FrameParameterValue,
    Phase80FrameStructuralContext,
    Phase80MultiFrameStructuralContext,
    Phase70StructuralParameterRegistry,
    Phase70StructuralParameterSpec,
    Phase70Telemetry,
    phase80_build_frame_local_parameter_registry,
    phase80_emit_episode_telemetry,
    phase80_emit_frame_transition_telemetry,
    phase80_emit_integration_telemetry,
    phase80_integrate_cross_frame_structural_deltas,
    phase80_resolve_frame_parameter,
    phase80_run_multiframe_episode,
    phase80_summarize_episode_structural_integration,
    phase80_build_phase9_integration_hook,
    phase80_sequence_frame_transitions,
    phase80_scaffold_frame_transitions,
    phase80_scaffold_frame_parameter_snapshots,
    phase80_scaffold_multiframe_structural_context,
    phase80_validate_frame_continuity_invariants,
    phase70_validate_adjustment_log_invariants,
    scaffold_phase70_structural_adjustment,
    apply_phase62_structural_experiment,
    GoalAttractor,
    GoalHierarchy,
    GoalSet,
    GoalStabilityMetric,
    IntentDrivenTrajectory,
    IntentField,
    MetaIntentField,
    MetaIntentTrajectory,
    MetaPreferenceGradient,
    ManifoldDrift,
    ManifoldEvolutionTrace,
    PreferenceGradient,
    RegionFlowVector,
    SelfCoherenceMetric,
    StabilityEnergy,
    TopologicalNeighborhood,
    TopologicalRegion,
    TopologyEvolutionStep,
    TopologyMetrics,
    CognitiveFrame, ConstraintEvalEngine, ConstraintKind, ConstraintStatus, DisambiguationDecision,
    FrameIterationResult, MultiFrameCognition, MultiFrameConfig, MultiFrameIteration,
    MultiFrameReport, ParallelResolutionSummary, ScheduledTask, SemanticConstraint,
    SemanticNode, SenseInterferenceScore, StabilizationMetrics, StableSense, TaskScheduler,
};
pub use runtime::{AuditLogger, DeterminismVerifier, DeterministicRuntime};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn core_invariants_accessible() {
        let _determinism = CoreInvariant::Determinism;
        let _consistency = CoreInvariant::Consistency;
        let _closure = CoreInvariant::Closure;
        let _auditability = CoreInvariant::Auditability;
    }

    #[test]
    fn closure_status_transitions() {
        use ClosureStatus::*;
        assert!(!Open.is_final());
        assert!(Closed.is_final());
        assert!(Contradictory.is_final());
        assert!(!Partial.is_final());
    }

    #[test]
    fn phase2_pipeline_smoke_test() {
        let engine = ConstraintEvalEngine::new();
        let constraints = vec![SemanticConstraint::assertion("light", "wave", true, 90)];
        let nodes = engine.constraints_to_nodes(&constraints);
        let mut field = engine.project_nodes_to_field(&nodes);
        engine.apply_resonance_transform(&mut field, &nodes);
        assert_eq!(field.concept_count(), 1);
    }
}
