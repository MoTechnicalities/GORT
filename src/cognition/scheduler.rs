/// Reasoning task scheduler.
/// Manages deterministic parallel execution of reasoning operations.

use crate::runtime::parallel::DeterministicRuntime;

#[derive(Debug, Clone)]
pub struct ScheduledTask<T> {
    pub id: u64,
    pub payload: T,
}

#[derive(Debug, Default)]
pub struct TaskScheduler {
    runtime: DeterministicRuntime,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            runtime: DeterministicRuntime::new(),
        }
    }

    pub fn run_deterministic<T, R, F>(&self, mut tasks: Vec<ScheduledTask<T>>, op: F) -> Vec<R>
    where
        T: Send + Sync,
        R: Send,
        F: Fn(&ScheduledTask<T>) -> R + Send + Sync,
    {
        tasks.sort_by_key(|t| t.id);
        self.runtime.execute_indexed(&tasks, op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_enforces_stable_order() {
        let scheduler = TaskScheduler::new();
        let tasks = vec![
            ScheduledTask { id: 20, payload: 2 },
            ScheduledTask { id: 10, payload: 1 },
        ];

        let out = scheduler.run_deterministic(tasks, |task| task.id);
        assert_eq!(out, vec![10, 20]);
    }
}
