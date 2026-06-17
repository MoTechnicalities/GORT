use serde::{Deserialize, Serialize};
use std::env;

const PHASE13_STATE_TELEMETRY: &str = "GORT_PHASE13_STATE_TELEMETRY";
const PHASE13_MEASUREMENT_TELEMETRY: &str = "GORT_PHASE13_MEASUREMENT_TELEMETRY";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase13QubitUnaryOp {
    PauliX,
    PauliZ,
    FixedHadamard,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase13QubitState {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub norm_error: i8,
    pub state_signature: String,
    pub state_well_formed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase13MeasurementOutcome {
    pub bit: u8,
    pub post_state: Phase13QubitState,
    pub policy: String,
    pub measurement_signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase13UnitarySequenceResult {
    pub initial_state_signature: String,
    pub final_state: Phase13QubitState,
    pub op_sequence_signature: String,
    pub evolution_signature: String,
    pub step_count: usize,
}

pub fn phase13_build_qubit_state(x: i8, y: i8, z: i8) -> Result<Phase13QubitState, String> {
    let norm_squared = (x as i16) * (x as i16) + (y as i16) * (y as i16) + (z as i16) * (z as i16);
    let norm_error = (norm_squared - 1) as i8;
    if norm_squared != 1 {
        return Err(format!(
            "phase13_invalid_state_norm: expected 1, observed {}",
            norm_squared,
        ));
    }

    let signature_input = format!("x={}|y={}|z={}", x, y, z);
    let state_signature = phase13_hash(&signature_input);
    Ok(Phase13QubitState {
        x,
        y,
        z,
        norm_error,
        state_signature,
        state_well_formed: true,
    })
}

pub fn phase13_validate_qubit_state_invariants(state: &Phase13QubitState) -> Result<(), String> {
    let expected = phase13_build_qubit_state(state.x, state.y, state.z)?;
    if expected.state_signature != state.state_signature {
        return Err("phase13_state_signature_mismatch".to_string());
    }
    if state.norm_error != 0 {
        return Err("phase13_state_norm_error_nonzero".to_string());
    }
    if !state.state_well_formed {
        return Err("phase13_state_marked_malformed".to_string());
    }
    Ok(())
}

pub fn phase13_unitary_signature(op: Phase13QubitUnaryOp) -> String {
    match op {
        Phase13QubitUnaryOp::PauliX => "[[0,1],[1,0]]".to_string(),
        Phase13QubitUnaryOp::PauliZ => "[[1,0],[0,-1]]".to_string(),
        Phase13QubitUnaryOp::FixedHadamard => "(1/sqrt(2))*[[1,1],[1,-1]]".to_string(),
    }
}

pub fn phase13_validate_unitary_invariants(op: Phase13QubitUnaryOp) -> Result<(), String> {
    if phase13_unitary_signature(op).is_empty() {
        return Err("phase13_empty_unitary_signature".to_string());
    }
    Ok(())
}

pub fn phase13_ops_commute(
    lhs: Phase13QubitUnaryOp,
    rhs: Phase13QubitUnaryOp,
) -> Result<bool, String> {
    let lhs_rhs = phase13_compose_integer_unitary_matrix(lhs, rhs)?;
    let rhs_lhs = phase13_compose_integer_unitary_matrix(rhs, lhs)?;
    Ok(lhs_rhs == rhs_lhs)
}

pub fn phase13_apply_unitary(
    state: &Phase13QubitState,
    op: Phase13QubitUnaryOp,
) -> Result<Phase13QubitState, String> {
    phase13_validate_qubit_state_invariants(state)?;
    phase13_validate_unitary_invariants(op)?;

    let (x, y, z) = match op {
        // 180 degree rotation around X axis.
        Phase13QubitUnaryOp::PauliX => (state.x, -state.y, -state.z),
        // 180 degree rotation around Z axis.
        Phase13QubitUnaryOp::PauliZ => (-state.x, -state.y, state.z),
        // H X H = Z and H Z H = X in Bloch-space form.
        Phase13QubitUnaryOp::FixedHadamard => (state.z, -state.y, state.x),
    };

    phase13_build_qubit_state(x, y, z)
}

pub fn phase13_measure_z(state: &Phase13QubitState) -> Result<Phase13MeasurementOutcome, String> {
    phase13_validate_qubit_state_invariants(state)?;

    let bit = if state.z > 0 {
        0
    } else if state.z < 0 {
        1
    } else if state.x > 0 || (state.x == 0 && state.y >= 0) {
        0
    } else {
        1
    };

    let post_state = if bit == 0 {
        phase13_build_qubit_state(0, 0, 1)?
    } else {
        phase13_build_qubit_state(0, 0, -1)?
    };

    let policy = "fixture_threshold_v1".to_string();
    let measurement_signature = phase13_hash(&format!(
        "policy={}|source={}|bit={}|post={}",
        policy, state.state_signature, bit, post_state.state_signature,
    ));

    Ok(Phase13MeasurementOutcome {
        bit,
        post_state,
        policy,
        measurement_signature,
    })
}

pub fn phase13_apply_unitary_sequence(
    initial_state: &Phase13QubitState,
    ops: &[Phase13QubitUnaryOp],
) -> Result<Phase13UnitarySequenceResult, String> {
    phase13_validate_qubit_state_invariants(initial_state)?;

    let mut current = initial_state.clone();
    let mut step_signatures = Vec::with_capacity(ops.len());

    for (index, op) in ops.iter().enumerate() {
        phase13_validate_unitary_invariants(*op)?;
        current = phase13_apply_unitary(&current, *op)?;
        step_signatures.push(format!(
            "step={}|op={}|state={}",
            index + 1,
            phase13_unitary_signature(*op),
            current.state_signature,
        ));
    }

    let op_sequence_signature = phase13_hash(&step_signatures.join("||"));
    let evolution_signature = phase13_hash(&format!(
        "initial={}|ops={}|final={}|steps={}",
        initial_state.state_signature,
        op_sequence_signature,
        current.state_signature,
        ops.len(),
    ));

    Ok(Phase13UnitarySequenceResult {
        initial_state_signature: initial_state.state_signature.clone(),
        final_state: current,
        op_sequence_signature,
        evolution_signature,
        step_count: ops.len(),
    })
}

pub fn phase13_emit_state_telemetry(state: &Phase13QubitState) -> String {
    let line = format!(
        "x={}:y={}:z={}:norm_error={}:state_signature={}:well_formed={}",
        state.x,
        state.y,
        state.z,
        state.norm_error,
        state.state_signature,
        state.state_well_formed,
    );
    env::set_var(PHASE13_STATE_TELEMETRY, &line);
    line
}

pub fn phase13_emit_measurement_telemetry(outcome: &Phase13MeasurementOutcome) -> String {
    let line = format!(
        "policy={}:bit={}:post_signature={}:measurement_signature={}",
        outcome.policy,
        outcome.bit,
        outcome.post_state.state_signature,
        outcome.measurement_signature,
    );
    env::set_var(PHASE13_MEASUREMENT_TELEMETRY, &line);
    line
}

fn phase13_hash(input: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn phase13_compose_integer_unitary_matrix(
    lhs: Phase13QubitUnaryOp,
    rhs: Phase13QubitUnaryOp,
) -> Result<[[i8; 2]; 2], String> {
    let a = phase13_integer_unitary_matrix(lhs)?;
    let b = phase13_integer_unitary_matrix(rhs)?;

    let out = [
        [
            ((a[0][0] as i16 * b[0][0] as i16) + (a[0][1] as i16 * b[1][0] as i16)) as i8,
            ((a[0][0] as i16 * b[0][1] as i16) + (a[0][1] as i16 * b[1][1] as i16)) as i8,
        ],
        [
            ((a[1][0] as i16 * b[0][0] as i16) + (a[1][1] as i16 * b[1][0] as i16)) as i8,
            ((a[1][0] as i16 * b[0][1] as i16) + (a[1][1] as i16 * b[1][1] as i16)) as i8,
        ],
    ];

    Ok(out)
}

fn phase13_integer_unitary_matrix(op: Phase13QubitUnaryOp) -> Result<[[i8; 2]; 2], String> {
    match op {
        Phase13QubitUnaryOp::PauliX => Ok([[0, 1], [1, 0]]),
        Phase13QubitUnaryOp::PauliZ => Ok([[1, 0], [0, -1]]),
        Phase13QubitUnaryOp::FixedHadamard => Err(
            "phase13_integer_matrix_unsupported_for_fixed_hadamard".to_string(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase13_build_state_rejects_non_unit_bloch_vector() {
        let err = phase13_build_qubit_state(1, 1, 0).expect_err("norm must fail");
        assert_eq!(err, "phase13_invalid_state_norm: expected 1, observed 2");
    }

    #[test]
    fn phase13_apply_unitary_is_replay_stable() {
        let state = phase13_build_qubit_state(0, 0, 1).expect("state");
        let a = phase13_apply_unitary(&state, Phase13QubitUnaryOp::FixedHadamard).expect("a");
        let b = phase13_apply_unitary(&state, Phase13QubitUnaryOp::FixedHadamard).expect("b");
        assert_eq!(a, b);
    }

    #[test]
    fn phase13_measurement_fixture_contract_is_deterministic() {
        let plus_x = phase13_build_qubit_state(1, 0, 0).expect("state");
        let out_a = phase13_measure_z(&plus_x).expect("measurement a");
        let out_b = phase13_measure_z(&plus_x).expect("measurement b");
        assert_eq!(out_a, out_b);
        assert_eq!(out_a.bit, 0);
        assert_eq!(out_a.post_state, phase13_build_qubit_state(0, 0, 1).expect("zero"));
    }

    #[test]
    fn phase13_x_and_z_are_non_commuting_ops() {
        let commute = phase13_ops_commute(Phase13QubitUnaryOp::PauliX, Phase13QubitUnaryOp::PauliZ)
            .expect("x/z composition should be computable");
        assert!(!commute);
    }

    #[test]
    fn phase13_unitary_sequence_helper_is_replay_stable() {
        let initial = phase13_build_qubit_state(0, 0, 1).expect("initial");
        let ops = [
            Phase13QubitUnaryOp::FixedHadamard,
            Phase13QubitUnaryOp::PauliZ,
            Phase13QubitUnaryOp::PauliX,
            Phase13QubitUnaryOp::FixedHadamard,
        ];

        let a = phase13_apply_unitary_sequence(&initial, &ops).expect("sequence a");
        let b = phase13_apply_unitary_sequence(&initial, &ops).expect("sequence b");

        assert_eq!(a, b);
        assert_eq!(a.step_count, 4);
        assert!(!a.evolution_signature.is_empty());
    }
}