use crate::cognition::phase13_qubit_kernel::{phase13_apply_unitary, Phase13QubitUnaryOp};
use crate::cognition::phase15_two_qubit_semantic_binding::{
    phase15_apply_two_qubit_op, phase15_validate_two_qubit_invariants, Phase15BindingKind,
    Phase15TwoQubitOp, Phase15TwoQubitState,
};
use serde::{Deserialize, Serialize};
use std::env;

const PHASE16_TRAJECTORY_TELEMETRY: &str = "GORT_PHASE16_TRAJECTORY_TELEMETRY";

// ── op model ─────────────────────────────────────────────────────────────────

/// Which qubit a single-qubit thought op targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase16QubitTarget {
    Q1,
    Q2,
}

/// The operation applied at one step of a thought path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase16ThoughtOpKind {
    SingleQubit {
        target: Phase16QubitTarget,
        op: Phase13QubitUnaryOp,
    },
    TwoQubit {
        op: Phase15TwoQubitOp,
    },
}

/// A governed, labeled thought operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase16ThoughtOp {
    pub kind: Phase16ThoughtOpKind,
    pub label: String,
    /// Deterministic canonical hash of the op's identity.
    pub op_signature: String,
}

// ── trajectory types ──────────────────────────────────────────────────────────

/// Classification of a thought trajectory based on binding dynamics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase16TrajectoryKind {
    /// Zero ops applied.
    Degenerate,
    /// At least one step produced `EntanglingOpApplied` binding.
    Entangling,
    /// Final binding_kind matches initial; no entangling op was applied.
    Convergent,
    /// Final binding_kind differs from initial; no entangling op was applied.
    Divergent,
}

/// One governed transition in a thought path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase16ThoughtStep {
    pub step_index: usize,
    pub from_binding_signature: String,
    pub op_signature: String,
    pub to_binding_signature: String,
    pub to_binding_kind: Phase15BindingKind,
    /// Canonical hash covering from, op, and to.
    pub step_signature: String,
}

/// A complete, auditable, replay-stable thought trajectory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase16ThoughtTrajectory {
    pub initial_binding_signature: String,
    pub initial_binding_kind: Phase15BindingKind,
    pub steps: Vec<Phase16ThoughtStep>,
    pub step_count: usize,
    pub final_binding_signature: String,
    pub final_binding_kind: Phase15BindingKind,
    pub trajectory_kind: Phase16TrajectoryKind,
    /// Canonical hash covering: initial, all step signatures in order, final.
    pub trajectory_signature: String,
    pub trajectory_well_formed: bool,
}

// ── construction ──────────────────────────────────────────────────────────────

/// Build a single-qubit thought op targeting q1 or q2.
pub fn phase16_build_single_qubit_op(
    target: Phase16QubitTarget,
    op: Phase13QubitUnaryOp,
) -> Phase16ThoughtOp {
    let kind = Phase16ThoughtOpKind::SingleQubit { target, op };
    let label = format!("{:?}({:?})", op, target);
    let op_signature = phase16_hash(&format!(
        "kind=SingleQubit|target={:?}|op={:?}",
        target, op
    ));
    Phase16ThoughtOp { kind, label, op_signature }
}

/// Build a two-qubit thought op.
pub fn phase16_build_two_qubit_op(op: Phase15TwoQubitOp) -> Phase16ThoughtOp {
    let kind = Phase16ThoughtOpKind::TwoQubit { op };
    let label = format!("{:?}", op);
    let op_signature = phase16_hash(&format!("kind=TwoQubit|op={:?}", op));
    Phase16ThoughtOp { kind, label, op_signature }
}

// ── core engine ───────────────────────────────────────────────────────────────

/// Apply a single thought op to the current binding state.
pub fn phase16_apply_thought_op(
    state: &Phase15TwoQubitState,
    thought_op: &Phase16ThoughtOp,
) -> Result<Phase15TwoQubitState, String> {
    phase15_validate_two_qubit_invariants(state)?;

    match &thought_op.kind {
        Phase16ThoughtOpKind::TwoQubit { op } => {
            phase15_apply_two_qubit_op(state, *op)
        }

        Phase16ThoughtOpKind::SingleQubit { target, op } => {
            let new_q1;
            let new_q2;

            match target {
                Phase16QubitTarget::Q1 => {
                    new_q1 = phase13_apply_unitary(&state.q1, *op)?;
                    new_q2 = state.q2.clone();
                }
                Phase16QubitTarget::Q2 => {
                    new_q1 = state.q1.clone();
                    new_q2 = phase13_apply_unitary(&state.q2, *op)?;
                }
            }

            use crate::cognition::phase15_two_qubit_semantic_binding::phase15_build_two_qubit_state;
            phase15_build_two_qubit_state(new_q1, new_q2)
        }
    }
}

/// Trace a complete thought path from an initial binding state.
///
/// Returns a fully governed `Phase16ThoughtTrajectory` with step-level and
/// trajectory-level canonical signatures.
pub fn phase16_trace_thought_path(
    initial: &Phase15TwoQubitState,
    ops: &[Phase16ThoughtOp],
) -> Result<Phase16ThoughtTrajectory, String> {
    phase15_validate_two_qubit_invariants(initial)?;

    if ops.is_empty() {
        let trajectory_signature = phase16_hash(&format!(
            "initial={}|steps=[]|final={}",
            initial.binding_signature, initial.binding_signature,
        ));
        return Ok(Phase16ThoughtTrajectory {
            initial_binding_signature: initial.binding_signature.clone(),
            initial_binding_kind: initial.binding_kind,
            steps: vec![],
            step_count: 0,
            final_binding_signature: initial.binding_signature.clone(),
            final_binding_kind: initial.binding_kind,
            trajectory_kind: Phase16TrajectoryKind::Degenerate,
            trajectory_signature,
            trajectory_well_formed: true,
        });
    }

    let mut steps: Vec<Phase16ThoughtStep> = Vec::with_capacity(ops.len());
    let mut current = initial.clone();
    let mut any_entangling = false;

    for (index, thought_op) in ops.iter().enumerate() {
        let from_sig = current.binding_signature.clone();
        let next = phase16_apply_thought_op(&current, thought_op)?;

        if next.binding_kind == Phase15BindingKind::EntanglingOpApplied {
            any_entangling = true;
        }

        let step_signature = phase16_hash(&format!(
            "step={}|from={}|op={}|to={}",
            index, from_sig, thought_op.op_signature, next.binding_signature,
        ));

        steps.push(Phase16ThoughtStep {
            step_index: index,
            from_binding_signature: from_sig,
            op_signature: thought_op.op_signature.clone(),
            to_binding_signature: next.binding_signature.clone(),
            to_binding_kind: next.binding_kind,
            step_signature,
        });

        current = next;
    }

    let final_binding_kind = current.binding_kind;
    let final_binding_signature = current.binding_signature.clone();

    let trajectory_kind = if any_entangling {
        Phase16TrajectoryKind::Entangling
    } else if final_binding_kind == initial.binding_kind {
        Phase16TrajectoryKind::Convergent
    } else {
        Phase16TrajectoryKind::Divergent
    };

    let step_sigs = steps
        .iter()
        .map(|s| s.step_signature.as_str())
        .collect::<Vec<_>>()
        .join("|");
    let trajectory_signature = phase16_hash(&format!(
        "initial={}|steps=[{}]|final={}|kind={:?}",
        initial.binding_signature, step_sigs, final_binding_signature, trajectory_kind,
    ));

    Ok(Phase16ThoughtTrajectory {
        initial_binding_signature: initial.binding_signature.clone(),
        initial_binding_kind: initial.binding_kind,
        steps,
        step_count: ops.len(),
        final_binding_signature,
        final_binding_kind,
        trajectory_kind,
        trajectory_signature,
        trajectory_well_formed: true,
    })
}

// ── validation ────────────────────────────────────────────────────────────────

pub fn phase16_validate_trajectory_invariants(
    trajectory: &Phase16ThoughtTrajectory,
) -> Result<(), String> {
    if !trajectory.trajectory_well_formed {
        return Err("phase16_trajectory_marked_malformed".to_string());
    }
    if trajectory.trajectory_signature.is_empty() {
        return Err("phase16_trajectory_signature_empty".to_string());
    }
    if trajectory.initial_binding_signature.is_empty() {
        return Err("phase16_initial_binding_signature_empty".to_string());
    }
    if trajectory.step_count != trajectory.steps.len() {
        return Err(format!(
            "phase16_step_count_mismatch: declared {} actual {}",
            trajectory.step_count,
            trajectory.steps.len(),
        ));
    }
    for (i, step) in trajectory.steps.iter().enumerate() {
        if step.step_index != i {
            return Err(format!(
                "phase16_step_index_mismatch: expected {} got {}",
                i, step.step_index,
            ));
        }
        if step.step_signature.is_empty() {
            return Err(format!("phase16_step_signature_empty at step {i}"));
        }
    }
    Ok(())
}

// ── telemetry ─────────────────────────────────────────────────────────────────

pub fn phase16_emit_trajectory_telemetry(trajectory: &Phase16ThoughtTrajectory) -> String {
    let entangling_steps = trajectory
        .steps
        .iter()
        .filter(|s| s.to_binding_kind == Phase15BindingKind::EntanglingOpApplied)
        .count();
    let line = format!(
        "initial_sig={}:final_sig={}:kind={:?}:steps={}:entangling_steps={}:trajectory_sig={}:well_formed={}",
        trajectory.initial_binding_signature,
        trajectory.final_binding_signature,
        trajectory.trajectory_kind,
        trajectory.step_count,
        entangling_steps,
        trajectory.trajectory_signature,
        trajectory.trajectory_well_formed,
    );
    env::set_var(PHASE16_TRAJECTORY_TELEMETRY, &line);
    line
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn phase16_hash(input: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

// ── unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognition::phase13_qubit_kernel::phase13_build_qubit_state;
    use crate::cognition::phase15_two_qubit_semantic_binding::phase15_build_two_qubit_state;

    fn z0() -> crate::cognition::phase13_qubit_kernel::Phase13QubitState {
        phase13_build_qubit_state(0, 0, 1).unwrap()
    }

    fn correlated() -> Phase15TwoQubitState {
        phase15_build_two_qubit_state(z0(), z0()).unwrap()
    }

    #[test]
    fn phase16_degenerate_trajectory_is_well_formed() {
        let initial = correlated();
        let t = phase16_trace_thought_path(&initial, &[]).expect("empty trajectory");
        phase16_validate_trajectory_invariants(&t).expect("invariants");
        assert_eq!(t.trajectory_kind, Phase16TrajectoryKind::Degenerate);
        assert_eq!(t.step_count, 0);
    }

    #[test]
    fn phase16_convergent_trajectory_is_classified_correctly() {
        // PauliX on q1 followed by PauliX on q1 restores original Bloch position.
        // Both states are |0⟩-like → Correlated both before and after.
        let initial = correlated();
        let ops = vec![
            phase16_build_single_qubit_op(Phase16QubitTarget::Q1, Phase13QubitUnaryOp::PauliX),
            phase16_build_single_qubit_op(Phase16QubitTarget::Q1, Phase13QubitUnaryOp::PauliX),
        ];
        let t = phase16_trace_thought_path(&initial, &ops).expect("convergent");
        phase16_validate_trajectory_invariants(&t).expect("invariants");
        // Two PauliX on q1: (0,0,1)→(0,0,-1)→(0,0,1). q2 stays z=1.
        // First step: q1 becomes z=-1, q2 z=1 → z-product < 0 → AntiCorrelated.
        // Second step: q1 back to z=1 → Correlated again.
        assert_eq!(t.final_binding_kind, Phase15BindingKind::Correlated);
        assert_eq!(t.trajectory_kind, Phase16TrajectoryKind::Convergent);
    }

    #[test]
    fn phase16_divergent_trajectory_is_classified_correctly() {
        let initial = phase15_build_two_qubit_state(z0(), z0()).unwrap();
        let ops = vec![
            phase16_build_single_qubit_op(Phase16QubitTarget::Q2, Phase13QubitUnaryOp::PauliX),
        ];
        let t = phase16_trace_thought_path(&initial, &ops).expect("divergent");
        // q1=z1=1, q2 flipped to z=-1 → AntiCorrelated ≠ initial Correlated.
        assert_eq!(t.final_binding_kind, Phase15BindingKind::AntiCorrelated);
        assert_eq!(t.trajectory_kind, Phase16TrajectoryKind::Divergent);
    }

    #[test]
    fn phase16_entangling_trajectory_is_classified_correctly() {
        let initial = correlated();
        let ops = vec![
            phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledX),
        ];
        let t = phase16_trace_thought_path(&initial, &ops).expect("entangling");
        assert_eq!(t.trajectory_kind, Phase16TrajectoryKind::Entangling);
    }

    #[test]
    fn phase16_mixed_trajectory_is_replay_stable() {
        let initial = correlated();
        let ops = vec![
            phase16_build_single_qubit_op(Phase16QubitTarget::Q1, Phase13QubitUnaryOp::FixedHadamard),
            phase16_build_two_qubit_op(Phase15TwoQubitOp::ControlledX),
            phase16_build_single_qubit_op(Phase16QubitTarget::Q2, Phase13QubitUnaryOp::PauliZ),
            phase16_build_two_qubit_op(Phase15TwoQubitOp::Swap),
            phase16_build_single_qubit_op(Phase16QubitTarget::Q1, Phase13QubitUnaryOp::PauliX),
        ];
        let a = phase16_trace_thought_path(&initial, &ops).expect("a");
        let b = phase16_trace_thought_path(&initial, &ops).expect("b");
        assert_eq!(a, b);
        assert_eq!(a.step_count, 5);
        assert!(!a.trajectory_signature.is_empty());
    }
}
