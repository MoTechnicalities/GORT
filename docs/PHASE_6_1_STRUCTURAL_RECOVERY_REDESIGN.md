# Phase 6.1: Structural Recovery Redesign

## Intent

Open a dedicated post-Phase-6 redesign track focused on deterministic structural recovery speed,
with strict gates unchanged.

This workstream exists because flow/energy descent improvements are passing while structural
recovery still misses strict continuity/convergence closure targets on the failing path.

## Problem Statement

Current bottleneck profile indicates:

- energetic descent is sufficient,
- deterministic invariants are preserved,
- but topology/anchor recovery is not fast enough under perturbation replay.

The redesign must improve structural re-formation speed without loosening any acceptance criteria.

## Non-Negotiables

- No gate relaxation in harness or tests.
- Preserve deterministic behavior and canonical hash stability.
- Preserve worker-invariant replay outcomes.
- Keep changes auditable and explicitly scoped.

## Redesign Boundary

Phase 6.1 introduces an explicit structural policy subsystem, separate from ad hoc tuning:

- Module: src/cognition/phase61_structural_recovery.rs
- Integration point: MultiFrame run loop policy construction/execution
- Exports: lib.rs cognition re-exports for traceable API surface

The subsystem currently provides:

- activation signal model (known-stable + drift trigger),
- bounded active window policy,
- deterministic runtime parameter projection for recovery actions.

## Activation Model

A recovery window may activate only when all are true:

- previous iteration exists,
- zero contradictions,
- zero unresolved subjects,
- strong continuity baseline (anchor overlap/coherence floors),
- drift trigger from anchor drift and/or energy delta excursion.

When active, policy applies bounded deterministic projections:

- lower anchor persistence threshold (bounded by >=1),
- increase anchor pull strength (bounded ceiling),
- reduce ambiguity margin for extra deterministic disambiguation sweep(s),
- auto-expire after a fixed iteration window.

## Acceptance Criteria

Phase 6.1 is considered successful only if all hold:

- strict gate suite remains unchanged and passing behavior is not regressed,
- failing structural path improves to target recovery summary threshold,
- determinism tests remain stable across repeated runs,
- no new invariant violations.

## Execution Plan

1. Land structural policy scaffolding and explicit integration (this changeset).
2. Validate no-regression baseline:
   - cargo test
   - curriculum harness
   - plot generation
3. Use harness diagnostics to tune only Phase 6.1 policy constants and decision thresholds.
4. If needed, extend Phase 6.1 with additional deterministic structural operators in the same module.

## Out of Scope

- Relaxing gates, closure criteria, or test assertions.
- Non-deterministic exploration.
- Hidden coupling to unrelated flow/energy modules.
