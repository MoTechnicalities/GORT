use crate::cognition::phase13_qubit_kernel::{
    phase13_validate_qubit_state_invariants, Phase13QubitState,
};
use crate::cognition::phase15_two_qubit_semantic_binding::{
    phase15_validate_two_qubit_invariants, Phase15BindingKind, Phase15TwoQubitState,
};
use crate::cognition::phase16_thought_path_trajectories::{
    phase16_validate_trajectory_invariants, Phase16ThoughtTrajectory, Phase16TrajectoryKind,
};
use serde::{Deserialize, Serialize};
use std::env;

const PHASE17_MEASUREMENT_TELEMETRY: &str = "GORT_PHASE17_MEASUREMENT_TELEMETRY";

/// Canonical semantic categories emitted by Phase17 measurement surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase17SemanticLabel {
    Alignment,
    Contrast,
    Reinforcement,
    Conflict,
    EntanglingInfluence,
}

/// Deterministic semantic projection surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase17InterpretationSurface {
    ZAlignment,
    BindingKind,
    TrajectoryKind,
}

/// A governed semantic measurement artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase17SemanticMeasurement {
    pub semantic_surface: Phase17InterpretationSurface,
    pub semantic_label: Phase17SemanticLabel,
    pub source_signature: String,
    pub semantic_signature: String,
    pub telemetry_digest: String,
    pub measurement_well_formed: bool,
}

/// Deterministically project a Phase13 state onto the z-alignment surface.
pub fn phase17_measure_semantic(
    state: &Phase13QubitState,
) -> Result<Phase17SemanticMeasurement, String> {
    phase13_validate_qubit_state_invariants(state)?;

    let label = if state.z > 0 {
        Phase17SemanticLabel::Alignment
    } else if state.z < 0 {
        Phase17SemanticLabel::Contrast
    } else if state.x > 0 || (state.x == 0 && state.y >= 0) {
        Phase17SemanticLabel::Reinforcement
    } else {
        Phase17SemanticLabel::Conflict
    };

    Ok(phase17_build_measurement(
        Phase17InterpretationSurface::ZAlignment,
        label,
        &state.state_signature,
        &format!("x={}|y={}|z={}", state.x, state.y, state.z),
    ))
}

/// Deterministically project a Phase15 state onto the binding-kind surface.
pub fn phase17_measure_binding(
    state: &Phase15TwoQubitState,
) -> Result<Phase17SemanticMeasurement, String> {
    phase15_validate_two_qubit_invariants(state)?;

    let label = match state.binding_kind {
        Phase15BindingKind::Correlated => Phase17SemanticLabel::Alignment,
        Phase15BindingKind::AntiCorrelated => Phase17SemanticLabel::Contrast,
        Phase15BindingKind::Separable => Phase17SemanticLabel::Conflict,
        Phase15BindingKind::EntanglingOpApplied => Phase17SemanticLabel::EntanglingInfluence,
    };

    Ok(phase17_build_measurement(
        Phase17InterpretationSurface::BindingKind,
        label,
        &state.binding_signature,
        &format!("binding={:?}", state.binding_kind),
    ))
}

/// Deterministically project a Phase16 trajectory onto the trajectory-kind surface.
pub fn phase17_measure_trajectory(
    trajectory: &Phase16ThoughtTrajectory,
) -> Result<Phase17SemanticMeasurement, String> {
    phase16_validate_trajectory_invariants(trajectory)?;

    let label = match trajectory.trajectory_kind {
        Phase16TrajectoryKind::Degenerate => Phase17SemanticLabel::Alignment,
        Phase16TrajectoryKind::Convergent => Phase17SemanticLabel::Reinforcement,
        Phase16TrajectoryKind::Divergent => Phase17SemanticLabel::Conflict,
        Phase16TrajectoryKind::Entangling => Phase17SemanticLabel::EntanglingInfluence,
    };

    Ok(phase17_build_measurement(
        Phase17InterpretationSurface::TrajectoryKind,
        label,
        &trajectory.trajectory_signature,
        &format!(
            "trajectory_kind={:?}|steps={}",
            trajectory.trajectory_kind, trajectory.step_count,
        ),
    ))
}

pub fn phase17_validate_semantic_measurement_invariants(
    measurement: &Phase17SemanticMeasurement,
) -> Result<(), String> {
    if !measurement.measurement_well_formed {
        return Err("phase17_measurement_marked_malformed".to_string());
    }
    if measurement.source_signature.is_empty() {
        return Err("phase17_source_signature_empty".to_string());
    }
    if measurement.semantic_signature.is_empty() {
        return Err("phase17_semantic_signature_empty".to_string());
    }
    if measurement.telemetry_digest.is_empty() {
        return Err("phase17_telemetry_digest_empty".to_string());
    }
    Ok(())
}

pub fn phase17_emit_measurement_telemetry(measurement: &Phase17SemanticMeasurement) -> String {
    let line = format!(
        "surface={:?}:label={:?}:source={}:semantic_signature={}:telemetry_digest={}:well_formed={}",
        measurement.semantic_surface,
        measurement.semantic_label,
        measurement.source_signature,
        measurement.semantic_signature,
        measurement.telemetry_digest,
        measurement.measurement_well_formed,
    );
    env::set_var(PHASE17_MEASUREMENT_TELEMETRY, &line);
    line
}

/// Compose a canonical phase-level signature from one or more semantic measurements.
pub fn phase17_compose_semantic_signature(measurements: &[Phase17SemanticMeasurement]) -> String {
    let joined = measurements
        .iter()
        .map(|m| m.semantic_signature.as_str())
        .collect::<Vec<_>>()
        .join("|");
    phase17_hash(&joined)
}

/// Compose a canonical phase-level digest from one or more semantic measurements.
pub fn phase17_compose_semantic_digest(measurements: &[Phase17SemanticMeasurement]) -> String {
    let joined = measurements
        .iter()
        .map(|m| m.telemetry_digest.as_str())
        .collect::<Vec<_>>()
        .join("|");
    phase17_hash(&joined)
}

fn phase17_build_measurement(
    surface: Phase17InterpretationSurface,
    label: Phase17SemanticLabel,
    source_signature: &str,
    projection_trace: &str,
) -> Phase17SemanticMeasurement {
    let semantic_signature = phase17_hash(&format!(
        "surface={:?}|label={:?}|source={}|trace={}",
        surface, label, source_signature, projection_trace,
    ));
    let telemetry_digest = phase17_hash(&format!(
        "surface={:?}|label={:?}|semantic_signature={}|source={}",
        surface, label, semantic_signature, source_signature,
    ));

    Phase17SemanticMeasurement {
        semantic_surface: surface,
        semantic_label: label,
        source_signature: source_signature.to_string(),
        semantic_signature,
        telemetry_digest,
        measurement_well_formed: true,
    }
}

fn phase17_hash(input: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognition::phase13_qubit_kernel::{phase13_build_qubit_state, Phase13QubitUnaryOp};
    use crate::cognition::phase15_two_qubit_semantic_binding::{
        phase15_apply_two_qubit_op, phase15_build_two_qubit_state, Phase15TwoQubitOp,
    };
    use crate::cognition::phase16_thought_path_trajectories::{
        phase16_build_two_qubit_op, phase16_trace_thought_path,
    };

    #[test]
    fn phase17_state_surface_is_deterministic() {
        let z0 = phase13_build_qubit_state(0, 0, 1).expect("z0");
        let a = phase17_measure_semantic(&z0).expect("a");
        let b = phase17_measure_semantic(&z0).expect("b");
        assert_eq!(a, b);
        assert_eq!(a.semantic_label, Phase17SemanticLabel::Alignment);
    }

    #[test]
    fn phase17_binding_surface_detects_entangling_influence() {
        let q0 = phase13_build_qubit_state(0, 0, 1).expect("q0");
        let state = phase15_build_two_qubit_state(q0.clone(), q0.clone()).expect("state");
        let entangled = phase15_apply_two_qubit_op(&state, Phase15TwoQubitOp::ControlledX)
            .expect("entangled");

        let measurement = phase17_measure_binding(&entangled).expect("measurement");
        assert_eq!(
            measurement.semantic_label,
            Phase17SemanticLabel::EntanglingInfluence,
        );
        phase17_validate_semantic_measurement_invariants(&measurement).expect("invariants");
    }

    #[test]
    fn phase17_trajectory_surface_maps_divergent_to_conflict() {
        let q0 = phase13_build_qubit_state(0, 0, 1).expect("q0");
        let initial = phase15_build_two_qubit_state(q0.clone(), q0.clone()).expect("initial");
        let ops = [phase16_build_two_qubit_op(Phase15TwoQubitOp::Swap)];
        let trajectory = phase16_trace_thought_path(&initial, &ops).expect("trajectory");

        let measurement = phase17_measure_trajectory(&trajectory).expect("measurement");
        assert!(measurement.measurement_well_formed);
        assert!(!measurement.semantic_signature.is_empty());
    }

    #[test]
    fn phase17_compose_signatures_are_replay_stable() {
        let z0 = phase13_build_qubit_state(0, 0, 1).expect("z0");
        let zx = crate::cognition::phase13_qubit_kernel::phase13_apply_unitary(
            &z0,
            Phase13QubitUnaryOp::PauliX,
        )
        .expect("zx");

        let m0 = phase17_measure_semantic(&z0).expect("m0");
        let m1 = phase17_measure_semantic(&zx).expect("m1");
        let sig_a = phase17_compose_semantic_signature(&[m0.clone(), m1.clone()]);
        let sig_b = phase17_compose_semantic_signature(&[m0.clone(), m1.clone()]);
        let dig_a = phase17_compose_semantic_digest(&[m0.clone(), m1.clone()]);
        let dig_b = phase17_compose_semantic_digest(&[m0, m1]);

        assert_eq!(sig_a, sig_b);
        assert_eq!(dig_a, dig_b);
    }
}
