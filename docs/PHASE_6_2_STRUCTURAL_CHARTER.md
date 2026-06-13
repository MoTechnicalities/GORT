# Phase 6.2 Structural Charter

## Intent

Phase 6.2 targets faster structural recovery without reducing geometric quality or memory retention.

## Scope

Structural work only:

- convergence regime design
- anchor re-formation policy
- closure-path architecture

Out of scope:

- threshold-only tuning loops
- gate relaxation
- weakening anti-shortcut quality checks

## Success Criteria

All must hold:

- learning_curve_iterations <= 10 on canonical full_stack battery
- memory_improved_after_recovery remains true
- memory_improvement_score does not regress relative to Phase 6.1 baseline
- strict Phase 6 gate sequence passes without changing gate definitions
- deterministic replay invariants remain intact

## Design Constraints

- No non-deterministic operators.
- No hidden shortcuts that reduce continuity/region/anchor quality.
- Preserve existing auditability and canonical hash behavior.

## Execution Discipline

1. Propose one structural change at a time.
2. Validate with cargo test + curriculum harness + plot script.
3. Compare against Phase 6.1 state snapshot before promoting.
4. Reject changes that improve speed but degrade memory or structure.
