use crate::cognition::phase13_qubit_kernel::{
    phase13_apply_unitary, phase13_validate_qubit_state_invariants, Phase13QubitState,
    Phase13QubitUnaryOp,
};
use serde::{Deserialize, Serialize};
use std::env;

const PHASE15_BINDING_TELEMETRY: &str = "GORT_PHASE15_BINDING_TELEMETRY";
const PHASE15_OP_TELEMETRY: &str = "GORT_PHASE15_OP_TELEMETRY";

// ── semantic binding classification ──────────────────────────────────────────

/// How the two qubits are related.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase15BindingKind {
    /// No Z-axis correlation and no entangling op applied.
    Separable,
    /// Both qubits share the same Z-axis orientation (both |0⟩-like or both |1⟩-like).
    Correlated,
    /// Qubits have opposite Z-axis orientations.
    AntiCorrelated,
    /// A two-qubit entangling gate was applied; binding is history-dependent.
    EntanglingOpApplied,
}

// ── core types ────────────────────────────────────────────────────────────────

/// A governed two-qubit semantic state binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase15TwoQubitState {
    pub q1: Phase13QubitState,
    pub q2: Phase13QubitState,
    pub binding_kind: Phase15BindingKind,
    /// Canonical hash of (q1_sig, q2_sig, binding_kind, op_history).
    pub binding_signature: String,
    /// Hash of the sequence of two-qubit ops applied so far.
    pub op_history_signature: String,
    pub state_well_formed: bool,
}

/// Two-qubit operators supported by the Phase15 kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase15TwoQubitOp {
    /// Exchange q1 and q2.
    Swap,
    /// Apply PauliZ to q2 when q1.z > 0 (control in |0⟩ hemisphere). Entangling.
    ControlledZ,
    /// Apply PauliX to q2 when q1.z > 0 (control in |0⟩ hemisphere). Entangling.
    ControlledX,
}

/// Result of applying a sequence of two-qubit ops to an initial binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase15TwoQubitSequenceResult {
    pub initial_binding_signature: String,
    pub final_state: Phase15TwoQubitState,
    pub op_count: usize,
    /// Stable canonical hash covering initial state, op list, and final state.
    pub sequence_signature: String,
}

// ── construction ──────────────────────────────────────────────────────────────

/// Build the initial two-qubit product state from two independent single-qubit states.
pub fn phase15_build_two_qubit_state(
    q1: Phase13QubitState,
    q2: Phase13QubitState,
) -> Result<Phase15TwoQubitState, String> {
    phase13_validate_qubit_state_invariants(&q1)?;
    phase13_validate_qubit_state_invariants(&q2)?;
    let binding_kind = phase15_classify_binding(&q1, &q2, false);
    let op_history = phase15_hash("initial");
    phase15_assemble(q1, q2, binding_kind, op_history)
}

// ── single-op evolution ───────────────────────────────────────────────────────

/// Apply one two-qubit operator and return the updated binding.
pub fn phase15_apply_two_qubit_op(
    state: &Phase15TwoQubitState,
    op: Phase15TwoQubitOp,
) -> Result<Phase15TwoQubitState, String> {
    phase15_validate_two_qubit_invariants(state)?;

    let (new_q1, new_q2, entangling) = match op {
        Phase15TwoQubitOp::Swap => (state.q2.clone(), state.q1.clone(), false),

        Phase15TwoQubitOp::ControlledZ => {
            // CZ: if q1.z > 0 apply Z to q2, else identity.
            let new_q2 = if state.q1.z > 0 {
                phase13_apply_unitary(&state.q2, Phase13QubitUnaryOp::PauliZ)?
            } else {
                state.q2.clone()
            };
            (state.q1.clone(), new_q2, true)
        }

        Phase15TwoQubitOp::ControlledX => {
            // CX/CNOT: if q1.z > 0 apply X to q2, else identity.
            let new_q2 = if state.q1.z > 0 {
                phase13_apply_unitary(&state.q2, Phase13QubitUnaryOp::PauliX)?
            } else {
                state.q2.clone()
            };
            (state.q1.clone(), new_q2, true)
        }
    };

    let binding_kind = phase15_classify_binding(&new_q1, &new_q2, entangling);
    let new_history = phase15_hash(&format!(
        "prev={}|op={}",
        state.op_history_signature,
        phase15_op_name(op),
    ));
    phase15_assemble(new_q1, new_q2, binding_kind, new_history)
}

// ── sequence evolution ────────────────────────────────────────────────────────

/// Apply a sequence of two-qubit operators and return a stable result artifact.
pub fn phase15_apply_two_qubit_sequence(
    initial: &Phase15TwoQubitState,
    ops: &[Phase15TwoQubitOp],
) -> Result<Phase15TwoQubitSequenceResult, String> {
    phase15_validate_two_qubit_invariants(initial)?;

    let mut current = initial.clone();
    for op in ops {
        current = phase15_apply_two_qubit_op(&current, *op)?;
    }

    let sequence_signature = phase15_hash(&format!(
        "initial={}|ops=[{}]|final={}|count={}",
        initial.binding_signature,
        ops.iter().map(|o| phase15_op_name(*o)).collect::<Vec<_>>().join(","),
        current.binding_signature,
        ops.len(),
    ));

    Ok(Phase15TwoQubitSequenceResult {
        initial_binding_signature: initial.binding_signature.clone(),
        final_state: current,
        op_count: ops.len(),
        sequence_signature,
    })
}

// ── validation ────────────────────────────────────────────────────────────────

pub fn phase15_validate_two_qubit_invariants(
    state: &Phase15TwoQubitState,
) -> Result<(), String> {
    if !state.state_well_formed {
        return Err("phase15_state_marked_malformed".to_string());
    }
    phase13_validate_qubit_state_invariants(&state.q1)?;
    phase13_validate_qubit_state_invariants(&state.q2)?;
    if state.binding_signature.is_empty() {
        return Err("phase15_binding_signature_empty".to_string());
    }
    if state.op_history_signature.is_empty() {
        return Err("phase15_op_history_signature_empty".to_string());
    }
    let expected = phase15_compute_binding_signature(
        &state.q1,
        &state.q2,
        state.binding_kind,
        &state.op_history_signature,
    );
    if expected != state.binding_signature {
        return Err("phase15_binding_signature_mismatch".to_string());
    }
    Ok(())
}

// ── telemetry ─────────────────────────────────────────────────────────────────

pub fn phase15_emit_binding_telemetry(state: &Phase15TwoQubitState) -> String {
    let line = format!(
        "q1_sig={}:q2_sig={}:binding={:?}:binding_sig={}:op_history={}:well_formed={}",
        state.q1.state_signature,
        state.q2.state_signature,
        state.binding_kind,
        state.binding_signature,
        state.op_history_signature,
        state.state_well_formed,
    );
    env::set_var(PHASE15_BINDING_TELEMETRY, &line);
    line
}

pub fn phase15_emit_op_telemetry(op: Phase15TwoQubitOp) -> String {
    let line = format!(
        "op={}:signature={}",
        phase15_op_name(op),
        phase15_op_signature(op),
    );
    env::set_var(PHASE15_OP_TELEMETRY, &line);
    line
}

// ── private helpers ───────────────────────────────────────────────────────────

fn phase15_classify_binding(
    q1: &Phase13QubitState,
    q2: &Phase13QubitState,
    entangling: bool,
) -> Phase15BindingKind {
    if entangling {
        return Phase15BindingKind::EntanglingOpApplied;
    }
    let z_product = (q1.z as i16) * (q2.z as i16);
    match z_product.cmp(&0) {
        std::cmp::Ordering::Greater => Phase15BindingKind::Correlated,
        std::cmp::Ordering::Less => Phase15BindingKind::AntiCorrelated,
        std::cmp::Ordering::Equal => Phase15BindingKind::Separable,
    }
}

fn phase15_assemble(
    q1: Phase13QubitState,
    q2: Phase13QubitState,
    binding_kind: Phase15BindingKind,
    op_history_signature: String,
) -> Result<Phase15TwoQubitState, String> {
    let binding_signature =
        phase15_compute_binding_signature(&q1, &q2, binding_kind, &op_history_signature);
    Ok(Phase15TwoQubitState {
        q1,
        q2,
        binding_kind,
        binding_signature,
        op_history_signature,
        state_well_formed: true,
    })
}

fn phase15_compute_binding_signature(
    q1: &Phase13QubitState,
    q2: &Phase13QubitState,
    binding_kind: Phase15BindingKind,
    op_history: &str,
) -> String {
    phase15_hash(&format!(
        "q1={}|q2={}|kind={:?}|history={}",
        q1.state_signature, q2.state_signature, binding_kind, op_history,
    ))
}

fn phase15_op_name(op: Phase15TwoQubitOp) -> &'static str {
    match op {
        Phase15TwoQubitOp::Swap => "Swap",
        Phase15TwoQubitOp::ControlledZ => "ControlledZ",
        Phase15TwoQubitOp::ControlledX => "ControlledX",
    }
}

fn phase15_op_signature(op: Phase15TwoQubitOp) -> &'static str {
    match op {
        Phase15TwoQubitOp::Swap => "SWAP:[q1<->q2]",
        Phase15TwoQubitOp::ControlledZ => "CZ:[q1.z>0?Z(q2):I]",
        Phase15TwoQubitOp::ControlledX => "CX:[q1.z>0?X(q2):I]",
    }
}

fn phase15_hash(input: &str) -> String {
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

    fn z0() -> Phase13QubitState { phase13_build_qubit_state(0, 0, 1).unwrap() }   // |0⟩
    fn z1() -> Phase13QubitState { phase13_build_qubit_state(0, 0, -1).unwrap() }  // |1⟩
    fn xp() -> Phase13QubitState { phase13_build_qubit_state(1, 0, 0).unwrap() }   // |+x⟩

    #[test]
    fn phase15_binding_classification_is_correct() {
        let corr = phase15_build_two_qubit_state(z0(), z0()).unwrap();
        assert_eq!(corr.binding_kind, Phase15BindingKind::Correlated);

        let anti = phase15_build_two_qubit_state(z0(), z1()).unwrap();
        assert_eq!(anti.binding_kind, Phase15BindingKind::AntiCorrelated);

        let sep = phase15_build_two_qubit_state(z0(), xp()).unwrap();
        assert_eq!(sep.binding_kind, Phase15BindingKind::Separable);
    }

    #[test]
    fn phase15_invariants_hold_for_product_state() {
        let s = phase15_build_two_qubit_state(z0(), z0()).unwrap();
        phase15_validate_two_qubit_invariants(&s).expect("invariants");
        assert!(s.state_well_formed);
    }

    #[test]
    fn phase15_swap_exchanges_qubits_deterministically() {
        let s = phase15_build_two_qubit_state(z0(), z1()).unwrap();
        let swapped = phase15_apply_two_qubit_op(&s, Phase15TwoQubitOp::Swap).unwrap();
        assert_eq!(swapped.q1, z1());
        assert_eq!(swapped.q2, z0());
        // Verify replay stability.
        let swapped2 = phase15_apply_two_qubit_op(&s, Phase15TwoQubitOp::Swap).unwrap();
        assert_eq!(swapped, swapped2);
    }

    #[test]
    fn phase15_controlled_x_marks_entangling_op_applied() {
        // q1 = |0⟩ (z=1 > 0) → CX applies X to q2.
        let s = phase15_build_two_qubit_state(z0(), z0()).unwrap();
        let after = phase15_apply_two_qubit_op(&s, Phase15TwoQubitOp::ControlledX).unwrap();
        assert_eq!(after.binding_kind, Phase15BindingKind::EntanglingOpApplied);
        assert_eq!(after.q2, z1()); // X|0⟩ = |1⟩
    }

    #[test]
    fn phase15_sequence_is_replay_stable() {
        let initial = phase15_build_two_qubit_state(z0(), z0()).unwrap();
        let ops = [
            Phase15TwoQubitOp::ControlledX,
            Phase15TwoQubitOp::Swap,
            Phase15TwoQubitOp::ControlledZ,
        ];
        let a = phase15_apply_two_qubit_sequence(&initial, &ops).unwrap();
        let b = phase15_apply_two_qubit_sequence(&initial, &ops).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.op_count, 3);
        assert!(!a.sequence_signature.is_empty());
    }
}
