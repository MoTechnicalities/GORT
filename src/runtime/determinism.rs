/// Determinism enforcement and verification.
/// Ensures all operations are deterministic and reproducible.

use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Default, Clone, Copy)]
pub struct DeterminismVerifier;

impl DeterminismVerifier {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_state<T: Serialize>(&self, state: &T) -> Result<String, serde_json::Error> {
        let bytes = serde_json::to_vec(state)?;
        let mut h = Sha256::new();
        h.update(bytes);
        Ok(format!("{:x}", h.finalize()))
    }

    pub fn is_replay_stable<T: Serialize>(&self, a: &T, b: &T) -> Result<bool, serde_json::Error> {
        Ok(self.hash_state(a)? == self.hash_state(b)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Snapshot {
        topic: &'static str,
        status: &'static str,
    }

    #[test]
    fn verifier_detects_stability() {
        let verifier = DeterminismVerifier::new();
        let a = Snapshot {
            topic: "light",
            status: "closed",
        };
        let b = Snapshot {
            topic: "light",
            status: "closed",
        };
        assert!(verifier.is_replay_stable(&a, &b).unwrap_or(false));
    }
}
