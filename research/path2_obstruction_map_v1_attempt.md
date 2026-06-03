# Path 2 Obstruction Map V1 Attempt

Status: first concrete obstruction-map attempt.

Purpose: define enough actual structure for:

```text
obs_c : Q_c -> O_c
```

to evaluate whether native obstruction vanishing can replace generic
hidden-witness proof for account creation.

## Design Position

Use the hybrid selected in `research/path2_native_candidate_gate_matrix.md`:

```text
Q_c = pullback-organized public semantic boundary
obs_c(q)=0 = native obstruction-vanishing validity
H_c = local lift of q plus private gauge
G_c = safety-invariant private trivialization/randomizer space
```

The key discipline:

Public local restrictions are not witness fragments. They are semantic faces of
the public boundary. Private pads, masks, opening-local values, and gauges live
in the fiber over those faces.

## First Concrete `Q_c`

Define:

```text
Q_c =
  ContextFace_c
  × AccountFace_c
  × SpendFace_c
  × LifecycleFace_c
  × TranscriptFace_c
  × StateFace_c
  × OverlapFace_c
  × CanonicalityFace_c
```

Each face has a context projection:

```text
ctx_i : Face_i -> ContextFace_c
```

and semantic projection into `M_c` components:

```text
sem_account    : AccountFace_c    -> AccountNamespace_c
sem_spend      : SpendFace_c      -> SpendAuthorizationClass_c
sem_lifecycle  : LifecycleFace_c  -> LifecycleIdentity_c
sem_transcript : TranscriptFace_c -> TranscriptBindingSurface_c
sem_state      : StateFace_c      -> StateTransitionBoundary_c
sem_context    : ContextFace_c    -> ParameterContext_c
```

The public meaning is:

```text
meaning_of_boundary_c(q) =
  (
    sem_account(q.account),
    sem_spend(q.spend),
    sem_lifecycle(q.lifecycle),
    sem_transcript(q.transcript),
    sem_state(q.state),
    sem_context(q.context)
  )
```

## First Concrete `O_c`

Define:

```text
O_c =
  O_context
  × O_pullback
  × O_gluing
  × O_meaning
  × O_spend_state
  × O_canonicality
```

where each component is a public check space:

```text
O_context      = context/domain mismatch space
O_pullback     = shared-base equality defect space
O_gluing       = descent overlap/cocycle defect space
O_meaning      = semantic realization defect space
O_spend_state  = spend/lifecycle/transcript/state compatibility defect space
O_canonicality = encoding/range/normal-form defect space
```

Zero is componentwise zero.

## Obstruction Map

Define:

```text
obs_c(q) =
  (
    context_defect(q),
    pullback_defect(q),
    gluing_defect(q),
    meaning_defect(q),
    spend_state_defect(q),
    canonicality_defect(q)
  )
```

Account creation accepts if:

```text
obs_c(q)=0
```

## Component 1: `context_defect`

Purpose:

Ensure the entire boundary is interpreted under one parameter/domain context.

Concrete shape:

```text
context_defect(q) =
  (
    domain_version_check(q.context),
    parameter_set_check(q.context),
    verifier_family_check(q.context),
    chain_or_epoch_scope_check(q.context)
  )
```

Accepted zero means:

```text
ContextFace_c is well formed and domain separated.
```

This component is deterministic byte/field validation, not a hidden proof.

## Component 2: `pullback_defect`

Purpose:

Make public cross-field consistency structural.

Concrete shape:

```text
pullback_defect(q) =
  (
    ctx_account(q.account)    - q.context,
    ctx_spend(q.spend)        - q.context,
    ctx_lifecycle(q.lifecycle)- q.context,
    ctx_transcript(q.transcript)-q.context,
    ctx_state(q.state)        - q.context
  )
```

in the appropriate context identifier module / equality-check space.

Accepted zero means:

```text
all public faces live over the same context.
```

This imports Candidate A into `obs_c`.

## Component 3: `gluing_defect`

Purpose:

Check that the public faces are compatible restrictions of one descended
account object.

Concrete first-pass shape:

```text
gluing_defect(q) =
  d_1(OverlapFace_c(q))
    - restrict_account_spend(q.account, q.spend)
    - restrict_account_lifecycle(q.account, q.lifecycle)
    - restrict_spend_transcript(q.spend, q.transcript)
    - restrict_lifecycle_state(q.lifecycle, q.state)
```

This is deliberately written as a coboundary/cocycle equation. The concrete
instantiation may be:

- a q12289 linear syndrome equation;
- a finite-module coboundary equation;
- a low-rank compatibility equation;
- a product of small public equality/syndrome checks.

Accepted zero means:

```text
the public faces are compatible as restrictions of a global boundary object.
```

Important limit:

`gluing_defect=0` must not require revealing private local section coordinates.
The public faces are semantic restrictions only.

## Component 4: `meaning_defect`

Purpose:

Prevent obstruction vanishing from proving only "some object glues." The glued
boundary must realize the declared semantic account meaning `M_c`.

Concrete shape:

```text
meaning_defect(q) =
  (
    declared_account_namespace(q) - sem_account(q.account),
    declared_spend_class(q)       - sem_spend(q.spend),
    declared_lifecycle(q)         - sem_lifecycle(q.lifecycle),
    declared_transcript(q)        - sem_transcript(q.transcript),
    declared_state(q)             - sem_state(q.state),
    declared_context(q)           - sem_context(q.context)
  )
```

If there is no separate declared meaning field, this component is the theorem
that `meaning_of_boundary_c(q)` is computed canonically and uniquely from the
faces.

Accepted zero means:

```text
q determines one M_c.
```

This is the safety-saturation anchor.

## Component 5: `spend_state_defect`

Purpose:

Prevent a bad public fiber where account creation and spend/lifecycle/state
surfaces disagree while still passing abstract gluing.

Concrete shape:

```text
spend_state_defect(q) =
  (
    transcript_binds_spend(q.transcript, q.spend),
    transcript_binds_lifecycle(q.transcript, q.lifecycle),
    spend_authorizes_state_transition(q.spend, q.state),
    lifecycle_authorizes_state_transition(q.lifecycle, q.state),
    nullifier_scope_matches_account(q.lifecycle, q.account)
  )
```

as zero-valued defects / boolean-to-field constraints.

Accepted zero means:

```text
spend, lifecycle, transcript, and state transition surfaces are one semantic
account action boundary.
```

This component is where replay and lifecycle safety live.

## Component 6: `canonicality_defect`

Purpose:

Avoid multiple public encodings for the same semantic account boundary and
prevent malformed public objects from entering the obstruction theorem.

Concrete shape:

```text
canonicality_defect(q) =
  (
    canonical_encoding_check(q),
    range_or_normal_form_check(q),
    no_ambiguous_domain_tags(q),
    no_duplicate_projection_names(q)
  )
```

Accepted zero means:

```text
q is a canonical public boundary representative.
```

Canonicality must not choose private gauge. It canonicalizes only public faces.

## Effective Descent Theorem V1

Target theorem:

```text
obs_zero_effective_descent_v1:
  obs_c(q)=0
    =>
  exists h.
    boundary_c(h)=q
    ∧ LocalValid_c(h)
    ∧ DescentConsistent_c(h)
    ∧ meaning_c(h)=meaning_of_boundary_c(q)
```

Proof decomposition:

1. `context_defect=0` gives a valid interpretation context.
2. `pullback_defect=0` gives all faces over one context.
3. `gluing_defect=0` gives a global public boundary object.
4. `meaning_defect=0` identifies the global object with `M_c`.
5. `spend_state_defect=0` proves spend/lifecycle/transcript/state closure.
6. `canonicality_defect=0` proves the public representative is canonical.
7. Candidate C supplies private gauge/lift existence:

   ```text
   exists h in boundary_c^{-1}(q)
   ```

   with gauge coordinates private and safety-invariant.

Critical observation:

This theorem still needs a lift-existence lemma. The obstruction map gets us
from public faces to a global public boundary. Candidate C must get us from
global public boundary to private representative fiber without leaking gauge.

## Local Restrictions

The local restrictions for V1 are:

```text
AccountFace_c
SpendFace_c
LifecycleFace_c
TranscriptFace_c
StateFace_c
```

They should be viewed as public restrictions of `M_c`, not as restrictions of
the raw private witness.

Private local section data belongs to:

```text
H_c = Lift_c(q) × Gauge_c × OpeningLocal_c × Randomizer_c
```

and is not part of `q`.

## Public Gluing Equations

The public gluing equations are:

```text
same context
account-spend compatibility
account-lifecycle compatibility
spend-transcript compatibility
lifecycle-state compatibility
state-account compatibility
canonical meaning agreement
```

These are initially semantic equations. The next refinement must choose an
algebraic carrier:

- direct field equality;
- q12289 syndrome/coboundary checks;
- compact module code;
- deterministic canonical byte-frame checks.

## Fiber Privacy Boundary

The following must remain private/gauge-local:

- pad source;
- mask source;
- opening-local coordinates;
- randomizers/blinding/noise;
- local section trivialization;
- proof query openings, if any proof system remains.

The following may be public:

- semantic account namespace;
- spend authorization class;
- lifecycle/nullifier boundary;
- transcript binding surface;
- state transition boundary;
- public overlap/check coordinates;
- canonical encoding/check metadata.

## Current Assessment

This V1 obstruction map is useful but incomplete.

What it solves:

- makes Candidate A's pullback consistency part of the native obstruction;
- binds the public boundary to `M_c`;
- gives a concrete place for replay/lifecycle/spend closure checks;
- keeps private witness fragments out of public local restrictions.

What remains hard:

```text
global public boundary => private lift fiber exists
```

This is exactly where Candidate C must be made concrete.

## Next Required Refinement

Create:

```text
research/path2_gauge_lift_v1_attempt.md
```

to define:

```text
Lift_c(q)
Gauge_c
OpeningLocal_c
Randomizer_c
canonical_lift_c(q) : H_c / G_c
```

and classify which private coordinates are pure gauge versus authorization
local versus semantic.

Only after that can we decide whether the obstruction map is enough for
effective descent or whether a residual proof/certificate is unavoidable.

