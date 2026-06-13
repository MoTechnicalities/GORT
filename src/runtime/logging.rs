/// Audit and operation logging.
/// Records all derivations for reproducibility and debugging.

use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct AuditLogger {
    sequence: AtomicU64,
    entries: Mutex<Vec<String>>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            sequence: AtomicU64::new(0),
            entries: Mutex::new(Vec::new()),
        }
    }

    pub fn record(&self, event: impl Into<String>) -> u64 {
        let id = self.sequence.fetch_add(1, Ordering::SeqCst);
        self.entries.lock().push(format!("{:08}:{}", id, event.into()));
        id
    }

    pub fn snapshot(&self) -> Vec<String> {
        self.entries.lock().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_is_monotonic() {
        let logger = AuditLogger::new();
        let a = logger.record("init");
        let b = logger.record("step");
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(logger.snapshot().len(), 2);
    }
}
