/// Audit and operation logging.
/// Records all derivations for reproducibility and debugging.

use parking_lot::Mutex;
use sha2::{Digest, Sha256};
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

    pub fn canonical_snapshot(&self) -> Vec<String> {
        Self::canonicalize(&self.snapshot())
    }

    pub fn canonical_trace_hash(&self) -> String {
        let canonical = self.canonical_snapshot().join("\n");
        let mut h = Sha256::new();
        h.update(canonical.as_bytes());
        format!("{:x}", h.finalize())
    }

    pub fn canonicalize(entries: &[String]) -> Vec<String> {
        let mut parsed: Vec<(u64, String)> = entries
            .iter()
            .map(|entry| {
                let mut parts = entry.splitn(2, ':');
                let seq = parts
                    .next()
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(u64::MAX);
                let event = parts.next().unwrap_or(entry.as_str());
                let normalized = event.split_whitespace().collect::<Vec<_>>().join(" ");
                (seq, normalized)
            })
            .collect();

        parsed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        parsed
            .into_iter()
            .map(|(seq, event)| format!("{:08}:{}", seq, event))
            .collect()
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

    #[test]
    fn canonicalization_is_byte_stable() {
        let a = vec![
            "00000002:  closure   attempted".to_string(),
            "00000000:frame initialized".to_string(),
            "00000001:added   node x".to_string(),
        ];
        let b = vec![
            "00000001:added node x".to_string(),
            "00000002:closure attempted".to_string(),
            "00000000:frame initialized".to_string(),
        ];

        let ca = AuditLogger::canonicalize(&a);
        let cb = AuditLogger::canonicalize(&b);
        assert_eq!(ca, cb);
    }
}
