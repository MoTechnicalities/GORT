use crate::cognition::phase13_qubit_kernel::Phase13QubitUnaryOp;
use serde::{Deserialize, Serialize};
use std::env;

const PHASE14_ALGEBRA_TELEMETRY: &str = "GORT_PHASE14_ALGEBRA_TELEMETRY";
const PHASE14_FAMILY_TELEMETRY: &str = "GORT_PHASE14_FAMILY_TELEMETRY";

// ── operator algebra types ────────────────────────────────────────────────────

/// Classification of an operator family by algebraic structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase14OperatorFamilyKind {
    /// Pauli group generators (X, Y, Z).
    PauliGroup,
    /// Clifford unitaries (H and combinations with Pauli).
    CliffordFamily,
    /// Mixed / not yet classified.
    Mixed,
}

/// The commutator [A, B] = AB - BA expressed as an integer 2x2 matrix.
/// Entries are signed 16-bit to avoid overflow when combining Pauli products.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase14Commutator {
    /// Entries row-major: [[m00, m01], [m10, m11]].
    pub matrix: [[i16; 2]; 2],
    /// True when [A, B] == 0 (operators commute).
    pub is_zero: bool,
    /// Deterministic canonical hash of the commutator matrix.
    pub commutator_signature: String,
}

/// A named, closed operator family with algebra invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase14OperatorFamily {
    pub family_name: String,
    pub family_kind: Phase14OperatorFamilyKind,
    pub ops: Vec<Phase14OperatorEntry>,
    /// Hash of the full operator set, order-stable for governance.
    pub family_signature: String,
    pub family_well_formed: bool,
}

/// One member of an operator family with its associated metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase14OperatorEntry {
    pub op: Phase13QubitUnaryOp,
    pub label: String,
    pub matrix_signature: String,
}

/// Pairwise commutation table for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase14CommutationTable {
    pub family_signature: String,
    /// One entry per ordered pair (i, j) where i < j.
    pub pairs: Vec<Phase14CommutationPair>,
    pub table_signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phase14CommutationPair {
    pub lhs_label: String,
    pub rhs_label: String,
    pub commutator: Phase14Commutator,
}

// ── construction ──────────────────────────────────────────────────────────────

/// Build the canonical Pauli operator family { X, Z }.
///
/// FixedHadamard is excluded because its matrix entries are irrational;
/// it lives in the Clifford family instead.
pub fn phase14_build_pauli_family() -> Phase14OperatorFamily {
    let ops = vec![
        phase14_entry(Phase13QubitUnaryOp::PauliX, "PauliX"),
        phase14_entry(Phase13QubitUnaryOp::PauliZ, "PauliZ"),
    ];
    phase14_assemble_family("pauli_xz", Phase14OperatorFamilyKind::PauliGroup, ops)
}

/// Build the minimal Clifford family { X, Z, H } — the generating set for
/// single-qubit Clifford circuits.
pub fn phase14_build_clifford_family() -> Phase14OperatorFamily {
    let ops = vec![
        phase14_entry(Phase13QubitUnaryOp::PauliX, "PauliX"),
        phase14_entry(Phase13QubitUnaryOp::PauliZ, "PauliZ"),
        phase14_entry(Phase13QubitUnaryOp::FixedHadamard, "FixedHadamard"),
    ];
    phase14_assemble_family(
        "clifford_xzh",
        Phase14OperatorFamilyKind::CliffordFamily,
        ops,
    )
}

/// Compute the commutator [lhs, rhs] = lhs*rhs - rhs*lhs.
///
/// Returns an error when either operator does not have an integer matrix
/// representation (i.e. FixedHadamard, which is irrational).
pub fn phase14_compute_commutator(
    lhs: Phase13QubitUnaryOp,
    rhs: Phase13QubitUnaryOp,
) -> Result<Phase14Commutator, String> {
    let a = phase14_integer_matrix(lhs)?;
    let b = phase14_integer_matrix(rhs)?;

    let ab = phase14_matmul_i16(&a, &b);
    let ba = phase14_matmul_i16(&b, &a);

    let matrix = [
        [ab[0][0] - ba[0][0], ab[0][1] - ba[0][1]],
        [ab[1][0] - ba[1][0], ab[1][1] - ba[1][1]],
    ];
    let is_zero = matrix.iter().all(|row| row.iter().all(|&e| e == 0));
    let commutator_signature = phase14_hash(&format!(
        "comm=[{},{},{},{}]",
        matrix[0][0], matrix[0][1], matrix[1][0], matrix[1][1],
    ));

    Ok(Phase14Commutator {
        matrix,
        is_zero,
        commutator_signature,
    })
}

/// Build the full pairwise commutation table for an operator family.
///
/// Pairs are ordered (i < j); families containing FixedHadamard will
/// skip pairs involving H because H has no integer matrix and instead
/// record a fixed sentinel signature.
pub fn phase14_build_commutation_table(
    family: &Phase14OperatorFamily,
) -> Phase14CommutationTable {
    let mut pairs = Vec::new();

    for i in 0..family.ops.len() {
        for j in (i + 1)..family.ops.len() {
            let lhs = &family.ops[i];
            let rhs = &family.ops[j];

            let commutator = match phase14_compute_commutator(lhs.op, rhs.op) {
                Ok(c) => c,
                Err(_) => Phase14Commutator {
                    matrix: [[0; 2]; 2],
                    is_zero: false,
                    commutator_signature: "irrational_pair_sentinel".to_string(),
                },
            };

            pairs.push(Phase14CommutationPair {
                lhs_label: lhs.label.clone(),
                rhs_label: rhs.label.clone(),
                commutator,
            });
        }
    }

    let pair_sigs = pairs
        .iter()
        .map(|p| p.commutator.commutator_signature.clone())
        .collect::<Vec<_>>()
        .join("|");
    let table_signature = phase14_hash(&format!(
        "family={}|pairs={}",
        family.family_signature, pair_sigs,
    ));

    Phase14CommutationTable {
        family_signature: family.family_signature.clone(),
        pairs,
        table_signature,
    }
}

/// Validate operator family governance invariants.
pub fn phase14_validate_family_invariants(
    family: &Phase14OperatorFamily,
) -> Result<(), String> {
    if !family.family_well_formed {
        return Err("phase14_family_marked_malformed".to_string());
    }
    if family.ops.is_empty() {
        return Err("phase14_family_has_no_operators".to_string());
    }
    if family.family_name.is_empty() {
        return Err("phase14_family_name_empty".to_string());
    }
    if family.family_signature.is_empty() {
        return Err("phase14_family_signature_empty".to_string());
    }
    // Labels must be unique within the family.
    let mut seen = std::collections::HashSet::new();
    for entry in &family.ops {
        if !seen.insert(&entry.label) {
            return Err(format!(
                "phase14_duplicate_operator_label: {}",
                entry.label
            ));
        }
    }
    Ok(())
}

/// Validate commutation table governance invariants.
pub fn phase14_validate_table_invariants(
    table: &Phase14CommutationTable,
    family: &Phase14OperatorFamily,
) -> Result<(), String> {
    let expected_pair_count = family.ops.len() * (family.ops.len().saturating_sub(1)) / 2;
    if table.pairs.len() != expected_pair_count {
        return Err(format!(
            "phase14_table_pair_count_mismatch: expected {} got {}",
            expected_pair_count,
            table.pairs.len(),
        ));
    }
    if table.family_signature != family.family_signature {
        return Err("phase14_table_family_signature_mismatch".to_string());
    }
    if table.table_signature.is_empty() {
        return Err("phase14_table_signature_empty".to_string());
    }
    Ok(())
}

// ── telemetry ─────────────────────────────────────────────────────────────────

pub fn phase14_emit_algebra_telemetry(
    family: &Phase14OperatorFamily,
    table: &Phase14CommutationTable,
) -> String {
    let non_commuting = table.pairs.iter().filter(|p| !p.commutator.is_zero).count();
    let commuting = table.pairs.len() - non_commuting;
    let line = format!(
        "family={}:kind={:?}:op_count={}:pairs={}:commuting={}:non_commuting={}:family_sig={}:table_sig={}:well_formed={}",
        family.family_name,
        family.family_kind,
        family.ops.len(),
        table.pairs.len(),
        commuting,
        non_commuting,
        family.family_signature,
        table.table_signature,
        family.family_well_formed,
    );
    env::set_var(PHASE14_ALGEBRA_TELEMETRY, &line);
    line
}

pub fn phase14_emit_family_telemetry(family: &Phase14OperatorFamily) -> String {
    let labels = family
        .ops
        .iter()
        .map(|e| e.label.as_str())
        .collect::<Vec<_>>()
        .join(",");
    let line = format!(
        "family={}:kind={:?}:ops=[{}]:signature={}:well_formed={}",
        family.family_name,
        family.family_kind,
        labels,
        family.family_signature,
        family.family_well_formed,
    );
    env::set_var(PHASE14_FAMILY_TELEMETRY, &line);
    line
}

// ── private helpers ───────────────────────────────────────────────────────────

fn phase14_entry(op: Phase13QubitUnaryOp, label: &str) -> Phase14OperatorEntry {
    let sig_input = format!("op={}|label={}", phase14_op_name(op), label);
    Phase14OperatorEntry {
        op,
        label: label.to_string(),
        matrix_signature: phase14_hash(&sig_input),
    }
}

fn phase14_assemble_family(
    name: &str,
    kind: Phase14OperatorFamilyKind,
    ops: Vec<Phase14OperatorEntry>,
) -> Phase14OperatorFamily {
    let sig_input = format!(
        "name={}|kind={:?}|ops=[{}]",
        name,
        kind,
        ops.iter().map(|e| e.label.as_str()).collect::<Vec<_>>().join(","),
    );
    let family_signature = phase14_hash(&sig_input);
    let family_well_formed = !ops.is_empty() && !name.is_empty();
    Phase14OperatorFamily {
        family_name: name.to_string(),
        family_kind: kind,
        ops,
        family_signature,
        family_well_formed,
    }
}

fn phase14_integer_matrix(op: Phase13QubitUnaryOp) -> Result<[[i16; 2]; 2], String> {
    match op {
        Phase13QubitUnaryOp::PauliX => Ok([[0, 1], [1, 0]]),
        Phase13QubitUnaryOp::PauliZ => Ok([[1, 0], [0, -1]]),
        Phase13QubitUnaryOp::FixedHadamard => Err(
            "phase14_integer_matrix_unsupported_for_fixed_hadamard".to_string(),
        ),
    }
}

fn phase14_matmul_i16(a: &[[i16; 2]; 2], b: &[[i16; 2]; 2]) -> [[i16; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ],
    ]
}

fn phase14_op_name(op: Phase13QubitUnaryOp) -> &'static str {
    match op {
        Phase13QubitUnaryOp::PauliX => "PauliX",
        Phase13QubitUnaryOp::PauliZ => "PauliZ",
        Phase13QubitUnaryOp::FixedHadamard => "FixedHadamard",
    }
}

fn phase14_hash(input: &str) -> String {
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

    #[test]
    fn phase14_pauli_family_is_well_formed() {
        let family = phase14_build_pauli_family();
        phase14_validate_family_invariants(&family).expect("pauli family valid");
        assert_eq!(family.ops.len(), 2);
        assert!(family.family_well_formed);
    }

    #[test]
    fn phase14_commutator_xz_is_non_zero_and_deterministic() {
        let a = phase14_compute_commutator(
            Phase13QubitUnaryOp::PauliX,
            Phase13QubitUnaryOp::PauliZ,
        )
        .expect("commutator");
        let b = phase14_compute_commutator(
            Phase13QubitUnaryOp::PauliX,
            Phase13QubitUnaryOp::PauliZ,
        )
        .expect("commutator replay");
        assert_eq!(a, b);
        assert!(!a.is_zero, "X and Z must not commute");
    }

    #[test]
    fn phase14_commutator_xx_is_zero() {
        let comm = phase14_compute_commutator(
            Phase13QubitUnaryOp::PauliX,
            Phase13QubitUnaryOp::PauliX,
        )
        .expect("commutator");
        assert!(comm.is_zero, "[X, X] must be zero");
    }

    #[test]
    fn phase14_commutation_table_is_replay_stable() {
        let family = phase14_build_pauli_family();
        let table_a = phase14_build_commutation_table(&family);
        let table_b = phase14_build_commutation_table(&family);
        assert_eq!(table_a, table_b);
        phase14_validate_table_invariants(&table_a, &family).expect("table valid");
        assert_eq!(table_a.pairs.len(), 1);
        assert!(!table_a.pairs[0].commutator.is_zero);
    }

    #[test]
    fn phase14_clifford_family_includes_hadamard_sentinel() {
        let family = phase14_build_clifford_family();
        phase14_validate_family_invariants(&family).expect("clifford family valid");
        assert_eq!(family.ops.len(), 3);
        let table = phase14_build_commutation_table(&family);
        let h_pairs: Vec<_> = table
            .pairs
            .iter()
            .filter(|p| p.lhs_label == "FixedHadamard" || p.rhs_label == "FixedHadamard")
            .collect();
        assert!(
            h_pairs.iter().all(|p| p.commutator.commutator_signature
                == "irrational_pair_sentinel"),
            "Hadamard pairs must carry the irrational sentinel"
        );
    }
}
