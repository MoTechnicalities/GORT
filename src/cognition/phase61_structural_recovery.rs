use serde::{Deserialize, Serialize};

/// Phase 6.1 structural recovery redesign config.
///
/// This subsystem is intentionally narrow: it provides a bounded snap-back policy
/// for known-stable regimes after perturbation drift, without changing acceptance
/// gates or canonical hash semantics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Phase61StructuralRecoveryConfig {
    pub enabled: bool,
    pub activation_anchor_overlap_floor: usize,
    pub activation_anchor_coherence_floor: i64,
    pub activation_energy_delta_multiplier: i64,
    pub activation_anchor_drift_floor: i64,
    pub active_window_iterations: usize,
    pub anchor_min_persistence_relaxation: usize,
    pub anchor_pull_boost: i64,
    pub ambiguity_margin_divisor: i64,
    pub extra_disambiguation_sweeps: usize,
}

impl Default for Phase61StructuralRecoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            activation_anchor_overlap_floor: 780,
            activation_anchor_coherence_floor: 780,
            activation_energy_delta_multiplier: 2,
            activation_anchor_drift_floor: 1,
            active_window_iterations: 3,
            anchor_min_persistence_relaxation: 1,
            anchor_pull_boost: 2,
            ambiguity_margin_divisor: 2,
            extra_disambiguation_sweeps: 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Phase61SignalSnapshot {
    pub has_previous_iteration: bool,
    pub contradiction_count: usize,
    pub unresolved_subjects: usize,
    pub anchor_overlap: usize,
    pub anchor_field_coherence: i64,
    pub anchor_drift: i64,
    pub energy_delta: i64,
    pub energy_delta_threshold: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Phase61RuntimePolicy {
    pub active: bool,
    pub effective_anchor_min_persistence: usize,
    pub effective_anchor_pull_strength: i64,
    pub sweep_ambiguity_margin: i64,
    pub extra_disambiguation_sweeps: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Phase61StructuralRecoveryState {
    active_until_iteration: Option<usize>,
}

impl Phase61StructuralRecoveryState {
    pub fn build_runtime_policy(
        &mut self,
        iter: usize,
        base_anchor_min_persistence: usize,
        base_anchor_pull_strength: i64,
        base_ambiguity_margin: i64,
        signal: Phase61SignalSnapshot,
        config: &Phase61StructuralRecoveryConfig,
    ) -> Phase61RuntimePolicy {
        let active_now = self.advance_activation(iter, signal, config);

        let effective_anchor_min_persistence = if active_now {
            base_anchor_min_persistence
                .saturating_sub(config.anchor_min_persistence_relaxation)
                .max(1)
        } else {
            base_anchor_min_persistence.max(1)
        };

        let effective_anchor_pull_strength = if active_now {
            (base_anchor_pull_strength + config.anchor_pull_boost).min(8)
        } else {
            base_anchor_pull_strength
        };

        let sweep_ambiguity_margin = if active_now {
            (base_ambiguity_margin / config.ambiguity_margin_divisor.max(1)).max(1)
        } else {
            base_ambiguity_margin
        };

        let extra_disambiguation_sweeps = if active_now {
            config.extra_disambiguation_sweeps
        } else {
            0
        };

        Phase61RuntimePolicy {
            active: active_now,
            effective_anchor_min_persistence,
            effective_anchor_pull_strength,
            sweep_ambiguity_margin,
            extra_disambiguation_sweeps,
        }
    }

    fn advance_activation(
        &mut self,
        iter: usize,
        signal: Phase61SignalSnapshot,
        config: &Phase61StructuralRecoveryConfig,
    ) -> bool {
        if !config.enabled {
            self.active_until_iteration = None;
            return false;
        }

        let currently_active = self
            .active_until_iteration
            .map(|until| iter <= until)
            .unwrap_or(false);

        let known_stable_regime = signal.has_previous_iteration
            && signal.contradiction_count == 0
            && signal.unresolved_subjects == 0
            && signal.anchor_overlap >= config.activation_anchor_overlap_floor
            && signal.anchor_field_coherence >= config.activation_anchor_coherence_floor;

        let drift_trigger = signal.anchor_drift >= config.activation_anchor_drift_floor
            || signal.energy_delta
                > signal
                    .energy_delta_threshold
                    .max(0)
                    .saturating_mul(config.activation_energy_delta_multiplier.max(1));

        if !currently_active && known_stable_regime && drift_trigger {
            self.active_until_iteration = Some(iter + config.active_window_iterations.saturating_sub(1));
        }

        self.active_until_iteration
            .map(|until| iter <= until)
            .unwrap_or(false)
    }
}
