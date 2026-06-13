/// Deterministic parallel execution runtime.
/// Enables safe, reproducible parallelism without data races or non-determinism.

use rayon::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct DeterministicRuntime;

impl DeterministicRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Executes work in parallel while preserving deterministic output ordering.
    pub fn execute_indexed<T, R, F>(&self, items: &[T], op: F) -> Vec<R>
    where
        T: Send + Sync,
        R: Send,
        F: Fn(&T) -> R + Send + Sync,
    {
        items.par_iter().map(op).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_execution_preserves_order() {
        let runtime = DeterministicRuntime::new();
        let items = vec![3, 1, 2];
        let out = runtime.execute_indexed(&items, |v| v * 10);
        assert_eq!(out, vec![30, 10, 20]);
    }
}
