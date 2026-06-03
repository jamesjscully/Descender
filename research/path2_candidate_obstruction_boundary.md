# Path 2 Candidate B: Obstruction-Vanishing Boundary

Status: second candidate sheet.

Purpose: evaluate whether account creation can be a public obstruction
vanishing check:

```text
obs_c(q)=0
```

where vanishing is not just public consistency, but an effective descent
certificate for an account-backed private lift.

## Candidate Summary

Construct:

```text
Q_c = public descent boundary data
O_c = obstruction/check object
obs_c : Q_c -> O_c
```

Account creation accepts when:

```text
obs_c(q)=0
```

and the core theorem is:

```text
obs_c(q)=0
  => exists h. boundary_c(h)=q ∧ LocalValid_c(h) ∧ AccountBacked_c(h)
```

This is the most direct native replacement for a generic hidden-witness proof.

## Intended Win

Candidate A made consistency native but did not obviously produce a private
lift. Candidate B tries to make lift existence the meaning of the public check.

Instead of:

```text
prove hidden h exists
```

the public account object carries enough descended boundary data that:

```text
obstruction vanishes => h exists
```

by a structural theorem.

## Semantic Target `M_c`

Use `research/path2_meaning_v1.md`:

```text
M_c =
  AccountNamespace_c
  × SpendAuthorizationClass_c
  × LifecycleIdentity_c
  × TranscriptBindingSurface_c
  × StateTransitionBoundary_c
  × ParameterContext_c
```

The obstruction candidate is viable only if:

```text
obs_c(q)=0 => q determines one M_c
```

and all lifts over `q` have that same `M_c`.

## Public Boundary Data `Q_c`

First-pass shape:

```text
Q_c =
  ContextBoundary_c
  × MeaningBoundary_c
  × LocalRestrictionBoundary_c
  × OverlapBoundary_c
  × SpendStateBoundary_c
```

### `ContextBoundary_c`

Pins:

- native boundary version;
- parameter set;
- verifier family;
- domain/network/fork scope;
- optional aggregation mode.

### `MeaningBoundary_c`

Carries the public semantic projection into `M_c`:

```text
meaning_of_boundary_c : Q_c -> M_c
```

This must be explicit. The obstruction map must not merely prove that some
local sections glue; it must prove they glue to this account meaning.

### `LocalRestrictionBoundary_c`

Carries public restrictions of the would-be local object. These are not
independent commitments. They are public boundary restrictions that the
obstruction equation interprets as faces/views of one descent object.

Possible contents:

- public account projection quotient;
- lifecycle restriction;
- spend authorization restriction;
- transcript restriction;
- state-transition restriction.

Private pads, masks, opening-local values, and gauges must not be present.

### `OverlapBoundary_c`

Carries public overlap data needed to check compatibility of restrictions.

Possible forms:

- syndrome/check coordinates;
- coboundary/cocycle coordinates;
- rank or consistency code;
- shared context projections;
- public normalization data.

### `SpendStateBoundary_c`

Carries the public portion needed to close spend and state semantics:

- spend authorization class;
- lifecycle/nullifier boundary;
- state transition boundary;
- challenge-binding surface.

This can overlap with `MeaningBoundary_c`, but is listed separately because it
is the highest-risk safety surface.

## Obstruction Object `O_c`

First-pass shape:

```text
O_c =
  O_context
  × O_gluing
  × O_meaning
  × O_spend_state
  × O_canonicality
```

where:

- `O_context` detects context mismatch;
- `O_gluing` detects failure of local restrictions to glue;
- `O_meaning` detects mismatch between glued object and declared `M_c`;
- `O_spend_state` detects mismatch among spend/lifecycle/state surfaces;
- `O_canonicality` detects noncanonical or malformed public boundary encoding.

Zero means:

```text
obs_c(q)=0
```

all obstruction components vanish.

## Obstruction Map

First-pass:

```text
obs_c(q) =
  (
    context_defect(q),
    gluing_defect(q),
    meaning_defect(q),
    spend_state_defect(q),
    canonicality_defect(q)
  )
```

The exact algebra is open, but the map should be native:

- linear syndrome equations;
- coboundary/cocycle equations;
- pullback equality over context;
- rank/check constraints;
- canonical range/normalization constraints.

It should not be:

```text
hash hidden witness and prove opening
```

or:

```text
opaque proof says q is valid
```

## Public Validity

Verifier accepts the native boundary if:

```text
obs_c(q)=0
```

or if a compact public certificate verifies a native obstruction equation.

Allowed certificate:

- public linear-algebra certificate;
- public rank/syndrome certificate;
- public data-availability witness for boundary components;
- deterministic normalization certificate.

Disallowed as the core validity mechanism:

- generic hidden-witness ZK;
- SHA preimage opening of `h`;
- independent role commitments plus equality proof.

## Effective Descent Theorem Candidate

Target:

```text
theorem obstruction_effective_descent:
  obs_c(q)=0
    =>
  exists h.
    boundary_c(h)=q
    ∧ LocalValid_c(h)
    ∧ DescentConsistent_c(h)
    ∧ meaning_c(h)=meaning_of_boundary_c(q)
```

This is the candidate's central theorem.

The theorem should be understood as an algebraic descent theorem:

```text
vanishing obstruction => compatible local data glues to a global representative
```

plus an account theorem:

```text
global representative realizes the declared account meaning
```

## Account-Backed Lift Requirement

The biggest risk is proving existence of the wrong lift.

Bad theorem:

```text
obs_c(q)=0 => exists some glued object
```

Required theorem:

```text
obs_c(q)=0 => exists account-backed h with meaning_of_boundary_c(q)
```

So `O_meaning` and `O_spend_state` are not optional. They are what prevents the
obstruction map from certifying a meaningless global section.

## Private Representative Shape

If the obstruction theorem holds, private representatives should be local lifts
of the public boundary:

```text
H_c =
  Lift_c(q)
  × Gauge_c
  × OpeningLocal_c
  × Randomizer_c
```

where:

```text
boundary_c(h)=q
```

and gauge/randomizer/opening-local components do not change `M_c`.

## Gate B1: Projection Completeness

First-pass result: plausible.

Because `MeaningBoundary_c` and `SpendStateBoundary_c` are explicit parts of
`Q_c`, public fields should factor through `q`.

Open audit:

- whether all checked-core challenge fields are captured by
  `TranscriptBindingSurface_c`;
- whether creation proof binding digest is semantic or only byte binding;
- whether role subcommitment handles are eliminated or reinterpreted as public
  restrictions.

## Gate B2: Safety Saturation

First-pass result: strong if `O_meaning` is correct.

Target:

```text
boundary_c(h)=boundary_c(h') => meaning_c(h)=meaning_c(h')
```

The obstruction candidate should force this by making `meaning_of_boundary_c(q)`
part of `Q_c` and requiring any lift to realize that meaning.

Risk:

If `OpeningLocal_c` or gauge choices can change spend authority while preserving
`q`, safety fails. This must be tested directly.

## Gate B3: Effective Descent

First-pass result: strongest candidate for this gate, but theorem-heavy.

This is exactly what the candidate is designed to supply:

```text
obs_c(q)=0 => exists account-backed h
```

The blocker is finding a concrete `obs_c` with a real algebraic theorem, not
just naming the obstruction.

## Gate B4: Fiber Privacy

First-pass result: plausible if `Q_c` contains only boundary data.

Privacy source:

```text
Fiber(boundary_c,q)
```

The fiber must contain private variation in:

- gauge/local trivialization;
- randomizers;
- masks/pads;
- opening-local coordinates.

Risk:

If `LocalRestrictionBoundary_c` is too detailed, it may expose private
representative data. The design must keep restrictions at the semantic boundary
level, not the witness-coordinate level.

## Gate B5: Spend Closure

First-pass result: plausible if `O_spend_state` is strong.

Spend should consume:

```text
p_spend(q), p_lifecycle(q), p_transcript(q), p_state(q)
```

and signer-held local authorization data.

Open issue:

Define exactly what local authorization data may vary inside the fiber without
changing public spend semantics.

## Gate B6: Native Verification

First-pass result: pass if `obs_c` can be made explicit and compact.

This candidate lives or dies on whether:

```text
obs_c(q)=0
```

is an actual public algebraic check.

If checking `obs_c(q)=0` itself requires a generic hidden-witness proof, this
candidate collapses into the old problem.

## Preliminary Disposition

Disposition: `[active-highest-upside]`.

Reason:

This is the candidate most directly aimed at replacing generic ZK. It can solve
Candidate A's effective-descent blocker if a concrete obstruction object can be
found.

Main blocker:

```text
construct concrete O_c and obs_c with an effective descent theorem that is
account-backed, not merely abstract gluing.
```

Likely role:

Candidate B may supply the core native validity theorem, while Candidate A's
pullback structure supplies the outer semantic boundary organization.

## Next Questions

1. What are the local restrictions whose compatibility should define the
   account object?
2. Can `O_c` be a linear syndrome/coboundary module over the existing q12289
   arithmetic?
3. What public data is necessary for `obs_c(q)=0` without leaking private lift
   coordinates?
4. How does vanishing obstruction bind to `MeaningBoundary_c`?
5. Can spend authorization be derived from the same glued object rather than
   carried as an independent handle?

