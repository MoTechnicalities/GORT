# RUGC: Rust Unified Geometric Cognition

**The first deterministic geometric reasoning engine.**

RUGC is a formal, independent Rust implementation of a semantic compilation stack grounded in geometric invariants. It sits between the **UGC geometric representational layer** and **CPU hardware execution** — the missing middle of deterministic AI.

## Philosophy

RUGC rejects the statistical/probabilistic paradigm that dominates modern AI. Instead, it proposes:

- **Determinism**: Same input + state → identical output (byte-stable reasoning frames)
- **Rigor**: All operations derive from formal geometric and semantic invariants
- **Auditability**: Every conclusion is traceable; every derivation is reproducible
- **Exactness**: CPU-native arithmetic without approximation where it matters

This is not pattern-matching. This is cognition.

## Design Axiom: Geometry Emerges From Stability

RUGC adopts a developmental architecture where geometry is not assumed at initialization.

Naive paradigm:

Predefined geometry -> learning inside it -> reasoning on top of it.

RUGC paradigm:

Stable identity -> exploratory interaction -> relational regularities -> endogenous geometry -> understanding -> intelligence.

### Core Axiom

**GeometricState is a state capable of generating geometry.**

### Container vs Generative Geometry

- Container geometry: geometry is predefined and cognition is inserted into it
- Generative geometry: stability and continuity are established first, then geometry is discovered from repeated interaction

RUGC explicitly follows the generative model.

### Developmental Sequence in RUGC

- Invariant -> state -> attractor -> play -> relationship discovery -> geometry -> understanding -> intelligence
- `GeometricState` provides the persistent substrate and reference stability
- Concept anchors serve as attractors that preserve identity and continuity
- Multi-frame loops provide deterministic exploratory interaction across perspectives
- Emergent concepts encode newly stabilized relational structure

### Why the Current Order Is Intentional

RUGC is not missing geometry. RUGC is implementing the preconditions that make geometry meaningful and auditable.

- Without persistent state, there is no stable reference frame
- Without a stable reference frame, relational terms (near/far, same/different, before/after) are operationally undefined
- Therefore, geometry is a developmental consequence, not a bootstrap primitive

This is why geometric geometry is treated as a later phase milestone, not a starting data type.

For the formal architecture statement, see `ARCHITECTURE_AXIOMS.md`.

## Core Architecture

RUGC is organized into three conceptual layers:

### 1. **Geometric Primitives** (`src/geom/`)

Defines the mathematical foundations:

- `invariants.rs` — **The cognitive kernel**: Core invariants, `GeometricState` trait, `ConstraintEvaluator` trait
- `space.rs` — Coordinate systems, transformations, spatial relationships
- `field.rs` — Semantic field definitions encoding meaning and relationships
- `mode.rs` — Resonance modes for semantic propagation

### 2. **Semantic Reasoning** (`src/cognition/`)

Implements constraint-driven reasoning:

- `node.rs` — Individual reasoning entities
- `constraint.rs` — Semantic constraint definitions
- `evaluator.rs` — Deterministic constraint satisfaction engine
- `scheduler.rs` — Task orchestration and parallel reasoning

### 3. **Deterministic Runtime** (`src/runtime/`)

Ensures reproducible execution:

- `parallel.rs` — Safe, reproducible parallelism without data races
- `logging.rs` — Audit trail recording for full traceability
- `determinism.rs` — Verification that all operations are deterministic

## The Cognitive Kernel

The foundation of RUGC is a three-piece cognitive kernel:

### 1. Core Invariants

All reasoning must satisfy four non-negotiable invariants:

```rust
pub enum CoreInvariant {
    /// Same input + state → identical output
    Determinism,
    
    /// No contradictory conclusions in final state
    Consistency,
    
    /// All frames reach deterministic final states
    Closure,
    
    /// All derivations traceable and reproducible
    Auditability,
}
```

### 2. Geometric State Trait

Defines what a reasoning frame must support:

```rust
pub trait GeometricState: Send + Sync {
    fn frame_id(&self) -> String;              // Deterministic identifier
    fn closure_status(&self) -> ClosureStatus;  // Current state
    fn validate(&self) -> Result<(), InvariantViolation>;
    fn attempt_closure(&self) -> (Self, Option<ClosureTransition>);
    fn record_derivation(&mut self, step: String);
    fn audit_trail(&self) -> Vec<String>;
}
```

### 3. Constraint Evaluator Trait

Defines how constraints are evaluated deterministically:

```rust
pub trait ConstraintEvaluator: Send + Sync {
    type Constraint;
    type EvaluationResult;
    
    fn evaluate(&self, constraint: &Self::Constraint) -> Result<Self::EvaluationResult, InvariantViolation>;
    fn detect_conflicts(&self, constraints: &[Self::Constraint]) -> Vec<(usize, usize, String)>;
    fn resolve_contradictions(&self, constraints: &[Self::Constraint], audit_trail: &mut Vec<String>) -> Result<Vec<Self::Constraint>, InvariantViolation>;
}
```

These three pieces form the foundation. Everything else grows from them.

## Getting Started

### Build

```bash
cargo build --release
```

### Run Examples

Basic geometric reasoning with frame determinism:

```bash
cargo run --example basic_reasoning
```

Constraint satisfaction and conflict detection:

```bash
cargo run --example geometric_constraint
```

Phase 3 multi-frame deterministic cognition demo:

```bash
cargo run --example phase3_multiframe
```

Phase 4 emergent stable-structure demo:

```bash
cargo run --example phase4_emergent
```

Phase 4.5 concept-anchor attractor demo:

```bash
cargo run --example phase45_concept_anchor
```

Phase 4.6 anchor-weighted interpretation demo:

```bash
cargo run --example phase46_anchor_weighted
```

Phase 4.7 anchor-driven emergent concept formation demo:

```bash
cargo run --example phase47_emergent_concepts
```

Phase 5.0 anchor-derived relational distance demo:

```bash
cargo run --example phase50_anchor_distance
```

Phase 5.1 emergent cognitive topology demo:

```bash
cargo run --example phase51_topology
```

Phase 5.2 cognitive manifold dynamics demo:

```bash
cargo run --example phase52_manifold_dynamics
```

Phase 5.3 cognitive flow fields demo:

```bash
cargo run --example phase53_flow_fields
```

Phase 5.4 cognitive energy & action selection demo:

```bash
cargo run --example phase54_energy_action
```

Phase 5.5 cognitive intent & goal formation demo:

```bash
cargo run --example phase55_intent_goal
```

Phase 5.6 multi-goal arbitration demo:

```bash
cargo run --example phase56_multi_goal
```

Phase 5.7 self-consistent cognitive dynamics demo:

```bash
cargo run --example phase57_meta_intent
```

### Run Tests

```bash
cargo test
```

## Design Principles

### 1. Determinism by Default

- No floating-point unless necessary; prefer exact arithmetic
- No unordered collections; use sorted structures with deterministic ordering
- All state transitions produce audit trails for replay
- Frame IDs are SHA256 hashes of semantic content (stable across runs)

### 2. Invariants Enforced at the Type Level

- `GeometricState` trait enforces frame closure semantics
- `ConstraintEvaluator` trait enforces consistent resolution
- `CoreInvariant` enum documents the four non-negotiable properties

### 3. Auditability as First-Class

- Every semantic step is recorded in an audit trail
- Reasoning is reproducible: given same input + audit trail, replay produces same output
- Contradictions are explicit, not hidden

### 4. Mechanical Verification Ready

- JSON schema validation for semantic tensors
- Trait-based architecture enables mock implementations for testing
- Determinism enables property-based testing across reasoning chains

## Historical Context

RUGC represents a fundamental departure from the GPU-driven statistical era:

| Aspect | Statistical Era | RUGC |
|--------|-----------------|------|
| **Paradigm** | Probabilistic pattern-fitting | Deterministic geometric reasoning |
| **Output** | High-confidence approximations | Formally justified conclusions |
| **Auditability** | Black-box (millions of parameters) | Transparent (traced derivations) |
| **Reproducibility** | Non-deterministic (even with fixed seeds) | Byte-stable (exact replay) |
| **Hardware** | GPU massively parallel | CPU deterministic parallelism |

RUGC is the first system designed for **actual cognition**, not statistical fitting.

## Roadmap

### Phase 1: Cognitive Kernel (Current)
✅ Core invariants and traits defined  
✅ Basic geometric state implementation  
✅ Constraint evaluation framework  

### Phase 2: Semantic Field Implementation (In Progress)
✅ Encode semantic meaning as geometric fields
✅ Implement resonance modes for semantic propagation
✅ Multi-sense disambiguation through field interference

### Phase 3: Deterministic Parallelism (In Progress)
✅ Deterministic scheduler/evaluator contradiction-resolution handoff
✅ Reproducible task scheduling
- Lock-free constraint resolution
- Parallel reasoning without non-determinism

### Phase 3 Acceptance Gates (Enforced in Tests/CI)
- Gate A: Full constraint-to-closure pipeline hash is identical across worker counts
- Gate B: Full pipeline hash is identical across repeated runs
- Gate C: Canonicalized audit traces are byte-stable for replay
- Gate D: Parallel contradiction resolution outputs deterministic resolved constraints

These gates are asserted in `tests/phase3_acceptance.rs` and run in CI.

### Phase 4: Multi-Frame Cognition (In Progress)
✅ Multi-frame deterministic loop: evaluate -> transform -> resolve -> stabilize -> repeat
✅ Cross-frame field sharing and deterministic constraint propagation
✅ Resonance-driven inference in iterative frame updates
✅ Semantic field normalization, compression, and concept clustering primitives
✅ Convergence detection and memory consolidation artifact hashing
- Multi-frame contradiction negotiation policies (advanced)
- Long-horizon iterative reasoning with bounded convergence proofs

### Phase 4 Acceptance Gates (Enforced in Tests/CI)
- Gate E: Multi-frame loop converges in K iterations under configured thresholds
- Gate F: Consolidated memory artifact hash is identical across worker counts
- Gate G: Consolidated memory artifact hash is stable across repeated replays

These gates are asserted in `tests/phase4_convergence.rs` and run in CI.

### Phase 4.5: Concept Anchors (In Progress)
✅ Stable low-energy attractor detection in shared semantic fields
✅ Anchor persistence under deterministic perturbation tests
✅ Anchor registry integrated into multi-frame interpretation loop
✅ Anchor-guided stabilization and self-basis continuity checks
- Internal/external perturbation classifiers driven by anchor drift metrics

### Phase 4.5 Acceptance Gates (Enforced in Tests/CI)
- Gate H: Concept anchors are registered only after persistence threshold is reached
- Gate I: Anchor registry hash is invariant across worker counts
- Gate J: External perturbation changes consolidated memory while preserving at least one anchor basis

These gates are asserted in `tests/phase45_anchors.rs` and run in CI.

### Phase 4.6: Anchor-Weighted Interpretation (In Progress)
✅ Anchor-weighted resonance (amplify near-anchor, dampen far-anchor, highlight contradictions)
✅ Anchor-guided fusion with anchor/field alignment bias
✅ Self-continuity metrics (overlap, drift, stability, anchor-field coherence)
✅ Anchor-driven consolidation scores for internal continuity vs external change
- Anchor-driven adaptive policy learning for long-horizon interpretation

### Phase 4.6 Acceptance Gates (Enforced in Tests/CI)
- Gate K: Anchor-field coherence improves across iterative runs
- Gate L: Anchor-weighted consolidation outputs are invariant across worker counts
- Gate M: Anchor-guided interpretation reduces highlighted contradictions over iterations

These gates are asserted in `tests/phase46_anchor_weighted.rs` and run in CI.

### Phase 4.7: Anchor-Driven Emergent Concept Formation (In Progress)
✅ Emergent concept candidates detected from anchor-aligned stable clusters
✅ Persistent candidate promotion into deterministic emergent concept registry
✅ Emergent constraint synthesis expands internal ontology deterministically
✅ Consolidated memory captures emergent concepts and ontology expansion score
- Adaptive emergent concept pruning/merging over long-horizon memory windows

### Phase 4.7 Acceptance Gates (Enforced in Tests/CI)
- Gate N: Emergent concepts form only after anchor-aligned persistence thresholds
- Gate O: Emergent concept registry and consolidation outputs are worker-invariant
- Gate P: Emergent constraint synthesis expands ontology with replay-stable signatures

These gates are asserted in `tests/phase47_emergent_concepts.rs` and run in CI.

### Phase 4: Cross-Lingual Auditing (Planned)
- Canonical token normalization across languages
- Polarity conflict detection (affirm vs. negate)
- Contradiction counting and marking

### Phase 5: Formal Verification (Future)
- Mechanical proof of invariant satisfaction
- Safety properties formalized in Coq/Lean
- Determinism certified at proof level

### Phase 5.0: Anchor-Derived Relational Distance (In Progress)
✅ First endogenous relational distance derived from anchor basis overlap and emergent concept overlap
✅ Internal continuity and external change deltas integrated into a replay-stable distance score
✅ Deterministic near/far ordering across baseline replay vs perturbed runs
✅ Worker-invariant relational distance computation across parallel schedules
- Extend scalar relational distance into higher-order endogenous manifold construction

### Phase 5.0 Acceptance Gates (Enforced in Tests/CI)
- Gate Q: Relational distance orders near/far (baseline replay is nearer than external perturbation)
- Gate R: Relational distance is invariant across worker counts
- Gate S: Relational distance detects externally injected change signals

These gates are asserted in `tests/phase50_anchor_distance.rs` and run in CI.

### Phase 5.1: Emergent Cognitive Topology (In Progress)
✅ Neighborhood formation — concept neighborhoods at anchor-derived distance threshold
✅ Region detection — connected components of neighborhoods form stable manifolds
✅ Boundary detection — concepts bridging distinct regions are topological boundaries
✅ Cohesion scoring — intra-region compactness measured from endogenous distance
✅ Topology-aware consolidation — canonical topology hash is replay-stable and worker-invariant
- Topological invariant tracking across iterative memory updates

### Phase 5.1 Acceptance Gates (Enforced in Tests/CI)
- Gate T: Topology forms at least one region from anchor-aligned concept space
- Gate U: Topology canonical hash is invariant across worker counts
- Gate V: External perturbation produces a distinct topology signature

These gates are asserted in `tests/phase51_topology.rs` and run in CI.

### Phase 5.2: Cognitive Manifold Dynamics (In Progress)
✅ Manifold drift detection — compare sequential topology snapshots for structural change
✅ Phase transition detection — threshold-based detection of manifold regime shifts
✅ Manifold evolution tracing — track persistent vs transient regions across time
✅ Worker-invariant evolution trace with replay-stable canonical hash
- Higher-order manifold clustering (regions of regions)
- Long-horizon topological memory with drift budgeting

### Phase 5.2 Acceptance Gates (Enforced in Tests/CI)
- Gate W: Manifold drift detects topology changes between baseline and perturbed runs
- Gate X: Drift is zero for identical topology replays
- Gate Y: Phase transitions detected on external injection; absent on stable replays
- Gate Z: Evolution trace canonical hash is invariant across worker counts

These gates are asserted in `tests/phase52_manifold_dynamics.rs` and run in CI.

### Phase 5.3: Cognitive Flow Fields (In Progress)
✅ Concept flow vectors — directional drift per concept across topology snapshots
✅ Region flow vectors — cohesion trend, size trend, persistence score, attractor flag
✅ Anchor-pull field — attractor influence measured from co-resident anchor count
✅ Flow-based stability prediction — convergent/divergent classification from drift trend
✅ Cognitive momentum — scalar magnitude of last-step manifold drift
- Trajectory extrapolation for predicted next topology state
- Multi-step flow budgeting for anticipatory reasoning

### Phase 5.3 Acceptance Gates (Enforced in Tests/CI)
- Gate AA: Anchor concepts receive positive anchor-pull in flow vectors
- Gate AB: Perturbation produces non-zero momentum; stable replay produces zero momentum
- Gate AC: Flow field canonical hash is invariant across worker counts
- Gate AD: Concept flux is non-zero when topology changes between snapshots

These gates are asserted in `tests/phase53_flow_fields.rs` and run in CI.

### Phase 5.4: Cognitive Energy & Action Selection (In Progress)
✅ Cognitive potential field — energy landscape over topological manifold
✅ Energy gradient computation — directional descent scores per region pair
✅ Stability energy wells — low-energy attractors derived from region persistence
✅ Action selection policy — policy for selecting energy-minimizing trajectories
✅ Energy-minimizing trajectory — deterministic trajectory prediction based on energy descent
- Long-horizon cognitive forecasting with energy budgeting

### Phase 5.4 Acceptance Gates (Enforced in Tests/CI)
- Gate AE: Energy wells form at attractor regions in stable topology
- Gate AF: Gradient descent follows flow field predictions (gradients ordered by energy delta)
- Gate AG: Action selection produces valid trajectory minimizing energy
- Gate AH: External perturbation creates energy spike in potential field
- Gate AI: Trajectory canonical hash is invariant across replays

These gates are asserted in `tests/phase54_energy_action.rs` and run in CI.

### Phase 5.5: Cognitive Intent & Goal Formation (In Progress)
✅ Intent field formation — internal goal attractors derived from stable low-energy wells
✅ Goal attractor weighting — preference strength from persistence and anchor support
✅ Preference gradients — goal pull overlaid on energy gradients for intentional routing
✅ Intent-driven trajectory selection — choose paths toward preferred regions while avoiding unstable regions
✅ Goal stability metrics — alignment, efficiency, projected stability, and intent confidence
- Multi-goal arbitration across competing preference fields

### Phase 5.5 Acceptance Gates (Enforced in Tests/CI)
- Gate AJ: Intent field forms at least one goal attractor in stable topology
- Gate AK: Preference gradients apply positive goal pull toward preferred regions
- Gate AL: Goal-directed trajectory moves toward preferred/non-avoided regions
- Gate AM: External perturbation increases projected difficulty and reduces goal stability
- Gate AN: Intent-driven trajectory canonical hash is invariant across worker counts

These gates are asserted in `tests/phase55_intent_goal.rs` and run in CI.

### Phase 5.6: Multi-Goal Arbitration & Internal Conflict Resolution (In Progress)
✅ GoalSet — structured set of simultaneous goals merged from multiple intent fields
✅ GoalPriorityWeights — relative importance weights per goal region
✅ ConflictGradient — coherence/interference analysis per region across competing goals
✅ ArbitratedIntentField — merged intent field with dominant goal and arbitration confidence
✅ ConflictResolvedTrajectory — chosen path that respects goal hierarchy and minimizes conflict cost
- Multi-level goal hierarchies and meta-intent structures

### Phase 5.6 Acceptance Gates (Enforced in Tests/CI)
- Gate AO: GoalSet merges multiple intent fields and accumulates goal weights
- Gate AP: ConflictGradients detect goal interference across competing attractors
- Gate AQ: Arbitration produces a single dominant goal region from the goal hierarchy
- Gate AR: Conflict resolution selects a coherent trajectory respecting goal priorities
- Gate AS: Arbitration canonical hash is invariant across worker counts

These gates are asserted in `tests/phase56_multi_goal.rs` and run in CI.

### Phase 5.7: Self-Consistent Cognitive Dynamics (Meta-Intent) (In Progress)
✅ GoalHierarchy — layered goal structure with deterministic parent links
✅ MetaIntentField — intent-about-intent synthesis over arbitrated goal fields
✅ MetaPreferenceGradient — cross-goal modulation (influence, coherence delta, conflict delta)
✅ SelfCoherenceMetric — hierarchy coherence, revision pressure, temporal stability, self-consistency
✅ MetaIntentTrajectory — recursive trajectory selection with goal revision under coherence pressure
- Self-model formation from long-horizon meta-intent history

### Phase 5.7 Acceptance Gates (Enforced in Tests/CI)
- Gate AT: MetaIntentField forms hierarchy and coherence metrics from arbitrated goals
- Gate AU: MetaPreferenceGradients capture inter-goal modulation effects
- Gate AV: Self-coherence decreases under external perturbation pressure
- Gate AW: Meta-intent trajectory revises goals when self-consistency is low
- Gate AX: Meta-intent canonical hashes are invariant across worker counts

These gates are asserted in `tests/phase57_meta_intent.rs` and run in CI.

### Phase 6: Self-Directed Cognitive Dynamics (Planned)
- Intentional self-modeling and persistent identity priors
- Autonomous goal revision loops with bounded contradiction budgets
- Long-horizon self-directed learning under deterministic replay

### Phase 6.2: Anchor-Closure Spine V1 (Validated, Still Flag-Gated)
✅ Recovery-only bridge from anchor to region, one-shot, replay-stable across the hard-holdout battery (learning_curve_iterations 13 → 1 on canonical holdout)
✅ Phase 6.2 gates enforced in `tests/phase62_structural.rs`: canonical hardest-case replay → multi-holdout replay stability → promotion-bar memory consistency (ignored, declared future bar)
✅ Extended CI validation: two additional full passes, identical per-holdout tuples and toggle logs, no regressions

**Per-holdout memory pattern (validated, stable across runs):**
- holdout_01 — memory_score = 5–6 (strong; large anchor gain, continuity positive)
- holdout_02 — memory_score = 3–4 (weak; continuity_delta ≤ 0, external_change_delta ≥ 0 — noise not dampened)
- holdout_03 — memory_score = 4–5 (mid; region and anchor gains present, continuity borderline)
- holdout_04 — memory_score = 3–4 (weak; external_change_delta ≥ 0 despite anchor gain — structural change doesn't reduce external noise)
- holdout_05 — memory_score = 3–6 (variable; bridge fires but continuity and external signals are tuning-pass sensitive)

The distinguishing signal in weak holdouts (02, 04): recovery reduces neither external noise nor self-continuity deficit. The closure spine is necessary but not sufficient.

### Phase 6.2 V2: Region-Cohesion and External-Dampening Reinforcement (In Progress)
Target: structural reinforcement for holdouts with memory_score < 5, specifically holdout_02 and holdout_04
Basis: per-holdout recovery deltas extracted from Phase 6.2 V1 multi-holdout battery

Candidate interventions:
- Multi-anchor closure — bridge from the N highest-coherence anchors, not just one, to improve continuity recovery
- Region-cohesion stabilization — inject region-merge constraints where topology_regions is flat across recovery
- External-change dampening — add contradiction-damping constraints targeting subjects with positive external_change_delta after recovery
- Manifold drift suppression — suppress momentum in regions that show divergent drift post-bridge

Acceptance criteria:
- memory_score ≥ 5 for all 5 hard holdouts
- no regression on V1 strong cases (holdout_01 must stay ≥ 5)
- replay-stable (two consecutive harness runs produce identical per-holdout tuples)
- recovery_converged_iteration ≤ 10 (speed budget preserved)
- gate_bc un-ignored and passing

Measured V2a effect (region cohesion + external dampening, full hard-holdout battery, two replay-identical runs):
- Aggregate memory score: unchanged at 5/6 (V1: 5/6 -> V2: 5/6)
- Replay stability: per-holdout tuples identical across two consecutive V2 battery runs
- Recovery speed: unchanged and within budget on all holdouts (1 iteration on every hard holdout)
- Holdout_02: improved 3/6 -> 5/6 (continuity_delta -1 -> 1; external_change_delta remained 10)
- Holdout_04: partial improvement 4/6 -> 4/6 (external_change_delta 0 -> -10; continuity_delta remained 0)
- Holdout_01: stable at 5/6 (no regression)
- Holdout_05: improved 4/6 -> 6/6 (continuity_delta -1 -> 1; external_change_delta stayed negative)

Interpretation: V2a is a true structural improvement and the current promoted Phase 6.2 V2 path (still flag-gated). It repairs continuity for holdout_02 and holdout_05 and dampens external change for holdout_04, while preserving replay determinism and speed budget.

### Phase 6.2 V2b: Plateau Continuity Lift (Manifold Drift Suppression) (Experimental, Pattern-Specific)
Target: narrow plateau-pattern intervention for holdout_04-style failures; not a general battery uplift
Status: replay-stable, not eligible for promotion due to holdout_02 regression risk

Measured V2b effect (hard-exclusion branch + full hard-holdout battery, two replay-identical runs):
- Aggregate memory score: unchanged at 5/6 (V1: 5/6 -> V2a: 5/6 -> V2b: 5/6)
- Replay stability: per-holdout tuples identical across two consecutive V2b runs
- Recovery speed: unchanged and within budget on all holdouts (1 iteration on every hard holdout)
- Holdout_01: stable at 5/6 (no regression)
- Holdout_02: regresses in V2b 3/6 -> 3/6 vs V1 baseline profile (continuity_delta -1 -> -2 under V2b)
- Holdout_03: neutral at 4/6 (0 -> 0 continuity)
- Holdout_04: strong V2b uplift 4/6 -> 6/6 (continuity_delta 0 -> 1; external_change_delta 0 -> -10)
- Holdout_05: neutral at 4/6 under V2b (V2a remains better at 6/6)

Interpretation: V2b is currently a holdout_04-pattern research probe, not a battery-level structural layer. It should remain experimental and flag-gated until selector criteria prevent holdout_02-style contradictory continuity regressions. The next V2b refinement should use explicit numeric plateau signatures (for example, continuity_delta == 0 and external_change_delta >= 0) rather than broader predicate-only matching.

### Phase 6.2 V3: Contradiction-Relief and Continuity-Rebinding (Experimental, 02-Class Probe)
Target: contradiction-dominated recovery, especially holdout_02-style failures with high support-demand pressure
Status: implemented as a separate experiment kind (`RUGC_PHASE62_KIND=contradiction`), replay-stable, not yet effective on the primary target class

Measured V3 effect (first contradiction-relief probe, full hard-holdout battery, two replay-identical runs):
- Aggregate memory score: unchanged at 5/6 (V1: 5/6 -> V2a: 5/6 -> V2b: 5/6 -> V3: 5/6)
- Replay stability: per-holdout tuples identical across two consecutive V3 runs
- Recovery speed: unchanged and within budget on all holdouts (1 iteration on every hard holdout)
- Holdout_02: no improvement versus V1 profile under this first V3 probe (3/6 remains 3/6; continuity_delta -1 -> -2)
- Holdout_03: neutral at 4/6 (0 -> 0 continuity)
- Holdout_04: no uplift under V3 (remains 4/6, unlike V2b)
- Holdout_05: partial uplift 4/6 -> 5/6 (continuity_delta -1 -> 1 while external_change_delta stays negative)
- Holdout_01: stable at 5/6 (no regression)

Interpretation: V3 confirms that contradiction-dominated recovery needs a genuinely different structural layer, but this first probe does not yet solve the intended 02-class failure. The current contradiction-relief + continuity-rebinding injection is replay-stable and non-destructive on strong cases, but its measurable effect lands on holdout_05 rather than holdout_02. Next V3 work should move beyond novelty-family gating and incorporate deeper runtime/closure-regime signals so contradiction pressure is relieved where continuity is actually collapsing.

### Phase 6.2 V3b: Closure-Regime-First Contradiction Branching (Experimental)
Target: split contradiction recovery into closure-ready (05-class) vs closure-deficit (02-class) branches using runtime summary signals from the first trained recovery pre-pass
Status: implemented as a separate experiment kind (`RUGC_PHASE62_KIND=v3b`), replay-stable, structurally cleaner than novelty-family gating, not yet promoted

Measured V3b effect (closure-regime branch, full hard-holdout battery, two replay-identical runs):
- Aggregate memory score: 4/6 on this branch snapshot (V1: 4/6 -> V2a: 4/6 -> V2b: 4/6 -> V3: 5/6 -> V3b: 4/6)
- Replay stability: per-holdout full-stack tuples identical across two consecutive V3b runs
- Recovery speed: unchanged and within budget on all holdouts (1 iteration on every hard holdout)
- Holdout_02: closure-deficit branch selected; trained recovery tuple stayed at continuity/external `(200, 0)` with lower region/anchor growth than V3 in this snapshot (`regions +0`, `anchors +2` vs V3 `regions +2`, `anchors +4`)
- Holdout_04: closure-deficit branch selected; stronger absolute recovery structure than V3 in this snapshot (`trained_recovery regions/anchors 60/63` vs V3 `59/60`)
- Holdout_05: closure-ready branch selected; stronger absolute recovery structure than V3 in this snapshot (`trained_recovery regions/anchors 65/68` vs V3 `63/64`)
- Holdout_01 and holdout_03: no destructive regression; continuity/external remained at `(200, 0)` in trained recovery

Interpretation: V3b successfully introduces the right architectural split (runtime-summary-driven closure regime instead of novelty-family-only gating) and preserves determinism, but this first closure-aware branch still needs threshold/selector tuning to consistently outperform V3 on aggregate memory score. The design direction is now correct; the remaining work is numeric calibration of branch criteria and closure-deficit subject ranking so 02-class gains become dominant rather than incidental.

### Phase 6.2 Status Snapshot (Frozen)

Current Phase 6.2 status is intentionally frozen to preserve reproducible baselines for the next structural step.

- V1 (Anchor-Closure Spine): baseline structural backbone; deterministic and replay-stable.
- V2a (Region Cohesion + External Dampening): promoted Phase 6.2 path for battery-safe uplift; no observed regressions in the validated runs.
- V2b (Plateau Continuity Lift): narrow 04-pattern research probe; replay-stable and explicitly non-promoted.
- V3 (Contradiction-Relief + Continuity-Rebinding): strongest aggregate memory in this snapshot (5/6), but still structurally mismatched for primary 02-class closure-deficit failures.
- V3b (Closure-Regime-First Branching): runtime-summary-driven branch model with explicit branch telemetry and replay stability; architecturally correct direction, but not yet a net aggregate upgrade over V3.

What this freeze means:

- Promoted production-story path remains V2a.
- V2b and V3b are retained as instrumented experimental probes, not promotion candidates yet.
- V3 remains current best aggregate contradiction regime in this snapshot.
- Next meaningful progress should be a new structural layer (V4-style), designed from telemetry, rather than additional predicate-only micro-tuning.

### Phase 6.2 V4 Design Brief (Draft)

Objective: deliver a structural V4 layer that specifically repairs 02-class closure-deficit behavior while preserving deterministic replay, one-iteration recovery speed, and battery-level memory safety.

#### 1) Target Failure Geometry (02-Class)

02-class failures are defined by a closure-deficit regime where trained recovery converges quickly but lands in a structurally shallow basin:

- Continuity saturation without structural lift: continuity/external remains numerically acceptable (often `(200, 0)`) while region/anchor growth under-recovers versus stronger contradiction paths.
- External-noise stickiness: post-recovery external-change signal fails to decrease enough to indicate true dampening of contradictory pressure.
- Topology plateau under contradiction load: region structure does not thicken despite bridge activation, indicating that contradiction relief is not coupling into manifold reinforcement.
- Selector mismatch risk: contradiction handling improvements can land on 05-class closure-ready cases while 02-class closure-deficit cases remain unchanged.

V4 intent is to force recovery energy into deficit geometry first: continuity recovery must be coupled to measurable region/anchor and external-dampening improvement on 02-class holdouts.

#### 2) Required Runtime Signals

V4 should consume a compact, deterministic runtime signal set derived from the first trained-recovery pre-pass and final trained-recovery state.

Core per-holdout signals:

- `continuity_pre`, `continuity_post`
- `external_pre`, `external_post`
- `regions_pre`, `regions_post`
- `anchors_pre`, `anchors_post`
- `support_signal`, `contradiction_signal`
- `phase62_v3b_branch` (or V4 branch equivalent) for explicit path labeling

Derived regime features (computed deterministically from core signals):

- `continuity_delta = continuity_post - continuity_pre`
- `external_delta = external_post - external_pre`
- `region_delta = regions_post - regions_pre`
- `anchor_delta = anchors_post - anchors_pre`
- `closure_deficit_index` (example composite):
    - high when continuity is flat/negative, external is flat/positive, and region+anchor growth is low
    - low when continuity rises, external falls, and region+anchor growth is strong
- `contradiction_pressure_ratio = contradiction_signal / max(1, support_signal)`

Telemetry requirements:

- Emit branch decision and skip/apply reason per holdout.
- Emit pre/final signal tuple per holdout in replay-stable order.
- Preserve run-to-run tuple comparability for automatic diff checks.

#### 3) Minimal Acceptance Gates for Promotion Readiness

V4 promotion should use a minimal, strict gate set that measures target-class repair first, then safety and determinism.

- Gate V4-A (02-Class Repair): on canonical 02-class holdout(s), memory_score reaches at least 5 with positive `continuity_delta` and non-positive `external_delta`.
- Gate V4-B (No Regression on Strong Cases): holdout_01 remains at least 5, and existing strong paths do not lose continuity/external quality.
- Gate V4-C (Battery Floor): full hard-holdout battery aggregate memory is at least current promoted baseline and never below V3 snapshot aggregate.
- Gate V4-D (Replay Stability): two consecutive full battery runs produce identical per-holdout tuples, branch labels, and gate outcomes.
- Gate V4-E (Speed Budget): `recovery_converged_iteration` remains at most 10 on all hard holdouts.

Promotion rule:

- Promote V4 only if all V4-A through V4-E pass in the same validation window.
- If V4-A passes but any safety gate fails, keep V4 experimental and retain V2a as promoted path.

Execution note:

- V4 development should prioritize structural mechanisms driven by `closure_deficit_index` and contradiction-pressure coupling, not additional predicate-only threshold nudges.

#### One-Pass V4 Validation Commands

```bash
# Reproducible V4 gate pass: V4-A..V4-E
# Assumes V4 branch kind is wired to RUGC_PHASE62_KIND=v4.
set -euo pipefail

OUT1=/tmp/rugc_v4_pass1.txt
OUT2=/tmp/rugc_v4_pass2.txt

# 1) Static/test gate sanity (structural tests + phase62 integration gates)
cargo test phase62_structural_experiment -- --nocapture --test-threads=1
cargo test --test phase62_structural -- --nocapture

# 2) Full hard-holdout battery twice (replay stability check)
RUGC_PHASE62_KIND=v4 cargo run --quiet --example curriculum_harness > "$OUT1"
RUGC_PHASE62_KIND=v4 cargo run --quiet --example curriculum_harness > "$OUT2"

# 3) Replay identity for V4-D (same per-holdout telemetry tuples)
grep -E "phase62_v3b_holdout_telemetry|phase62_v4_holdout_telemetry|holdout_[0-9]{2}.*memory_score|learning_curve_iterations" "$OUT1" > /tmp/rugc_v4_tuples1.txt
grep -E "phase62_v3b_holdout_telemetry|phase62_v4_holdout_telemetry|holdout_[0-9]{2}.*memory_score|learning_curve_iterations" "$OUT2" > /tmp/rugc_v4_tuples2.txt
diff -u /tmp/rugc_v4_tuples1.txt /tmp/rugc_v4_tuples2.txt

# 4) Compact gate evidence extraction for V4-A..V4-E
echo "=== V4-A (02-class repair) ==="
grep -E "holdout_02|continuity_delta|external_change_delta|memory_score" "$OUT1" | tail -n 12

echo "=== V4-B (no regression on strong case holdout_01) ==="
grep -E "holdout_01|continuity_delta|external_change_delta|memory_score" "$OUT1" | tail -n 12

echo "=== V4-C (battery floor aggregate) ==="
grep -E "aggregate|memory.*score|hard-holdout|final summary" "$OUT1" | tail -n 20

echo "=== V4-D (replay stability) ==="
echo "PASS if diff output above is empty"

echo "=== V4-E (speed budget) ==="
grep -E "learning_curve_iterations|recovery_converged_iteration" "$OUT1" | tail -n 20
```

Pass criteria in this one-pass script:

- V4-A: `holdout_02` shows `memory_score >= 5`, `continuity_delta > 0`, and `external_change_delta <= 0`.
- V4-B: `holdout_01` remains `memory_score >= 5` with no continuity/external degradation.
- V4-C: aggregate memory is not below promoted baseline and not below V3 snapshot baseline.
- V4-D: `diff -u` between pass1 and pass2 tuple extracts is empty.
- V4-E: `recovery_converged_iteration <= 10` (or equivalent learning-curve recovery budget signal) on all hard holdouts.

Gate-to-evidence mapping (protocol quick-reference):

| Gate | Exact evidence slice from command block | Pass focus |
|------|-----------------------------------------|------------|
| V4-A | `=== V4-A (02-class repair) ===` grep tail for `holdout_02`, `continuity_delta`, `external_change_delta`, `memory_score` | 02-class closure-deficit repair signal (continuity/external plus score) |
| V4-B | `=== V4-B (no regression on strong case holdout_01) ===` grep tail for `holdout_01`, `continuity_delta`, `external_change_delta`, `memory_score` | Strong-case no-regression safety envelope |
| V4-C | `=== V4-C (battery floor aggregate) ===` grep tail for `aggregate`, `memory.*score`, `hard-holdout`, `final summary` | Battery-level floor and promoted-baseline parity |
| V4-D | `diff -u /tmp/rugc_v4_tuples1.txt /tmp/rugc_v4_tuples2.txt` after tuple extraction grep | Replay tuple identity across consecutive full runs |
| V4-E | `=== V4-E (speed budget) ===` grep tail for `learning_curve_iterations` and `recovery_converged_iteration` | Convergence-speed budget and cross-regime consistency |

### Phase 6.3 Direction: New Structural Mechanism (Not Another Probe)

The next phase should add a new capability layer, not another selector/probe variant. Three viable structural directions are defined:

1. Closure-energy reinforcement
- New capability: explicit reinforcement of deficit-closure trajectories so continuity lift is coupled to durable structure growth.
- Structural intent: add a closure-energy term that rewards region/anchor formation only when external contradiction pressure is falling.
- Best use: when recovery appears numerically stable but remains structurally shallow on 02-class cases.

2. Topology-guided contradiction repair
- New capability: contradiction repair routed through manifold topology rather than local predicate gates.
- Structural intent: target boundary/bridge regions with contradiction-aware repair operators and region-cohesion constraints.
- Best use: when contradiction pressure localizes to repeatable topology patterns (plateaus, weak bridges, unstable boundaries).

3. Multi-pass recovery with structural memory
- New capability: staged recovery passes that preserve and reuse structural lessons across passes.
- Structural intent: pass 1 detects deficit geometry, pass 2 applies topology/closure repair, pass 3 verifies stabilization against replay-safe memory anchors.
- Best use: when one-pass recovery is fast but inconsistent across holdouts or regimes.

Recommended sequencing:

- Primary Phase 6.3 candidate: topology-guided contradiction repair.
- Follow-on Phase 6.4 candidate: multi-pass recovery with structural memory.
- Integrate closure-energy reinforcement as a shared mechanism used by both phases, rather than as a standalone probe track.

Promotion posture:

- Any selected direction must be evaluated through the existing V4-A..V4-E protocol and must beat or match the promoted baseline without replay or speed regressions.

### Phase 6.3 Minimal Implementation Spec (Topology-Guided Contradiction Repair)

Scope: implement the smallest structural layer that adds topology-guided contradiction repair with replay-stable telemetry and no runtime-model breakage.

#### Data Structures (Exact Minimal Set)

Add to `src/cognition/phase62_structural_experiment.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase63Kind {
    TopologyGuidedContradictionRepair,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase63RegionRole {
    Boundary,
    Bridge,
    Core,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase63RepairOperator {
    BoundaryDampen,
    BridgeRebind,
    CoreStabilize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase63RuntimeSummary {
    pub holdout_id: String,
    pub continuity_pre: i32,
    pub continuity_post: i32,
    pub external_pre: i32,
    pub external_post: i32,
    pub regions_pre: usize,
    pub regions_post: usize,
    pub anchors_pre: usize,
    pub anchors_post: usize,
    pub support_signal: i32,
    pub contradiction_signal: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase63RepairTarget {
    pub region_id: String,
    pub role: Phase63RegionRole,
    pub contradiction_pressure: i32,
    pub closure_deficit_index: i32,
    pub operator: Phase63RepairOperator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase63RepairPlan {
    pub kind: Phase63Kind,
    pub targets: Vec<Phase63RepairTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase63Telemetry {
    pub holdout_id: String,
    pub selected_targets: usize,
    pub applied_targets: usize,
    pub skipped_reason: Option<String>,
    pub continuity_delta: i32,
    pub external_delta: i32,
    pub region_delta: i32,
    pub anchor_delta: i32,
}
```

Notes:

- Use only integer/scalar fields to preserve deterministic ordering and replay stability.
- Keep `Vec` insertion order canonical (sorted by `region_id` before apply).

#### Hook Points (Exact Integration Points)

1. Kind parsing and routing
- File: `src/cognition/multiframe.rs`
- Extend `phase62_env_kind` alias parser with `phase63` and `topology_repair` that map to new execution kind.

2. Structural planning/apply path
- File: `src/cognition/phase62_structural_experiment.rs`
- Add `scaffold_topology_guided_contradiction_repair_v63(...) -> Phase63RepairPlan`.
- Add `select_phase63_repair_targets(...) -> Vec<Phase63RepairTarget>`.
- Add `apply_phase63_repair_plan(...)` directly in the existing phase62 experiment apply flow after trained-recovery pre-pass summary is available.

3. Runtime summary and telemetry wiring
- File: `examples/curriculum_harness.rs`
- Extend episode metrics with `phase63_plan` and `phase63_telemetry` output fields.
- Emit a per-holdout line: `phase63_holdout_telemetry=...` in stable key order.
- Preserve/restore env flags in scope guard, matching existing phase62 pattern.

4. Public exports
- File: `src/lib.rs`
- Export `Phase63RuntimeSummary`, `Phase63RepairPlan`, `Phase63RepairTarget`, `Phase63Telemetry`.

#### Minimal Acceptance-Test Stubs

Add unit tests to `src/cognition/phase62_structural_experiment.rs` test module:

```rust
#[test]
fn phase63_selects_boundary_bridge_targets_for_closure_deficit_fixture() {
    // Arrange: 02-class-like fixture with high contradiction pressure and low structural lift.
    // Act: select_phase63_repair_targets(...).
    // Assert: includes Boundary or Bridge targets; deterministic ordering by region_id.
}

#[test]
fn phase63_repair_plan_is_replay_stable_for_identical_summary() {
    // Arrange: identical summary twice.
    // Act: scaffold_topology_guided_contradiction_repair_v63(...) twice.
    // Assert: plans are equal byte-for-byte on debug/serialized form.
}
```

Add integration test stubs to `tests/phase62_structural.rs`:

```rust
#[test]
fn gate_v4d_phase63_replay_tuple_identity() {
    // Run phase63 twice on hard-holdout battery fixture.
    // Compare extracted phase63_holdout_telemetry tuples; must be identical.
}

#[test]
fn gate_v4a_phase63_repairs_canonical_02_class_without_speed_regression() {
    // Assert holdout_02 meets memory/continuity/external criteria.
    // Assert recovery_converged_iteration <= 10.
}

#[test]
fn gate_v4b_v4c_phase63_no_regression_and_battery_floor() {
    // Assert holdout_01 safety and aggregate floor vs promoted baseline.
}
```

#### Implementation Order (Minimal)

1. Add enums/structs and parser alias plumbing.
2. Implement deterministic target selection and repair-plan scaffolding.
3. Wire apply path and telemetry emission.
4. Add replay and gate stubs; then convert stubs to full assertions once first Phase 6.3 run artifacts are captured.

Definition of done for this minimal spec:

- New `phase63` kind compiles and runs through `curriculum_harness`.
- Emits deterministic `phase63_holdout_telemetry` lines.
- Replay tuple identity test passes.
- V4-A..V4-E command protocol can evaluate phase63 outputs without script changes.

### Phase 6.4: Operator Expansion

Fresh Phase63 sweep note, focused on holdout_02 and holdout_04:

- The closure_deficit candidate set now expands before scoring, so the new operators are actually exercised.
- holdout_02 and holdout_04 still did not lift continuity in this sweep.
- The locked two-step `ClosureBridge -> AnchorReweight` path now runs on those stubborn signatures, but the 02/04 realized continuity delta remains `0`.
- The positive continuity movement now appears on other closure_deficit rows, which is the first sign that the new routing hook is doing work.
- Deterministic multi-operator chaining is active and replay-stable on closure_deficit paths, including 3-operator plans on hard cases.

Phase-status interpretation:

- Phase 6.4/6.5 are now structurally characterized but 02/04-incomplete.
- The 02/04 continuity failure is now treated as outside the current topology/anchor/contradiction operator axis.
- 02/04 are explicitly marked as requiring a higher-order continuity mechanism.

Tiny closure-deficit lift table from the same fresh sweep:

| holdout | operator | realized_continuity_delta | realized_region_delta | realized_anchor_delta |
| --- | --- | ---: | ---: | ---: |
| holdout_01 | ClosureBridge | +5 | +4 | +17 |
| holdout_03 | ContradictionRedirect | +3 | +6 | +15 |
| holdout_01 | ContradictionRedirect | +4 | +12 | +19 |
| holdout_01 | ClosureBridge | +4 | +12 | +19 |

Gate wording for Phase 6.4:

- Gate A: At least one closure_deficit holdout shows `realized_continuity_delta > 0` under Phase63 with Phase 6.4 operators.
- Gate B: No destructive regression on strong cases (01/05 continuity/external stay non-negative).
- Gate C: Operator choice is replay-stable and matches the learned success bands.

### Phase 6.6 Direction (Next Axis): Continuity Re-Basing and Memory-Window Reshaping

Objective: move beyond additional operator-layer tuning and change the continuity mechanism itself for 02/04-class failures.

Initial design sketch:

- Continuity metric re-basing: redefine continuity lift against a deterministic local baseline window rather than only the immediate pre-pass scalar.
- Memory-window reshaping: introduce replay-stable multi-step continuity windows (short, medium, long) so hard-case continuity can accumulate rather than cancel in one pass.
- Continuity-to-structure coupling: score continuity gain only when paired with non-negative region/anchor reinforcement under non-rising external pressure.

Readiness criteria for this axis:

- holdout_02 and holdout_04 achieve positive realized continuity deltas under the new continuity formulation.
- Strong-case safety remains intact (01/05 continuity/external non-negative).
- Replay tuple identity remains unchanged across consecutive runs.

Phase 6.6 continuity-rebase constants (frozen v1):

- Formula (telemetry surface):
    - `continuity_rebased = continuity_delta - alpha * contradiction_pressure_ratio_ppm + beta * region_delta + gamma * anchor_delta`
- Deterministic fixed constants:
    - `alpha = 4 / 1_000_000`
    - `beta = 2`
    - `gamma = 1`
- Scope:
    - Applied only in Phase 6.6 telemetry (`RUGC_PHASE62_KIND=phase66`) during this stage.
    - No constraint mutation or behavioral routing change is introduced by this formula pass.

## Integration with UGC-Model

RUGC depends on the geometric representations defined in UGC-Model (`CSIF`, `RWIF`, resonance geometry) but takes them forward into CPU-native implementation:

- **UGC-Model** defines the *theory* and *representational formats*
- **RUGC** provides the *runtime*, *compiler*, and *cognitive kernel*

Together they form a complete stack from geometric theory to deterministic execution.

## License

Licensed under the Apache License, Version 2.0.

See [LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0 for the full text.

## Contributing

RUGC is foundational work toward open-sourcing after validation on a 31B brain on this system. See the main workspace for context.

---

**RUGC is the counter-proposal to the statistical GPU-driven era.**  
**It is deterministic. It is auditable. It is cognition.**
