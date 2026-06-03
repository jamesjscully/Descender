# Path 2 Candidate A: Pullback Boundary

Status: first candidate sheet.

Purpose: evaluate whether the native account boundary can be built as a
pullback of smaller semantic boundary objects over one shared context. This
candidate tries to make cross-field consistency structural rather than proving
equality between independently committed role objects.

## Candidate Summary

Construct the public boundary quotient as:

```text
Q_c =
  Q_account
    ×_{Q_context}
  Q_lifecycle
    ×_{Q_context}
  Q_spend
    ×_{Q_context}
  Q_transcript
    ×_{Q_context}
  Q_state
```

An account boundary is:

```text
q = (q_account, q_lifecycle, q_spend, q_transcript, q_state)
```

with all components agreeing on:

```text
ctx(q_i) = q_context
```

The native validity check is pullback membership:

```text
PullbackValid_c(q)
```

meaning all component boundaries are well formed and share the same context.

## Intended Win

The old design pressure comes from fields that are individually meaningful but
expensive to prove mutually consistent:

```text
account boundary
lifecycle/slot state
spend authorization handle
checked-core transcript frame
state transition boundary
```

The pullback candidate says these are not independent objects. They are
components of one public semantic boundary over a common base.

This could eliminate an entire class of role-equality/linking proofs.

## Semantic Target `M_c`

This candidate uses `research/path2_meaning_v1.md` directly:

```text
M_c =
  AccountNamespace_c
  × SpendAuthorizationClass_c
  × LifecycleIdentity_c
  × TranscriptBindingSurface_c
  × StateTransitionBoundary_c
  × ParameterContext_c
```

The boundary meaning map is:

```text
meaning_of_boundary_c(q) =
  (
    account_namespace(q_account),
    spend_authorization(q_spend),
    lifecycle_identity(q_lifecycle),
    transcript_surface(q_transcript),
    state_boundary(q_state),
    q_context
  )
```

provided all components agree over `q_context`.

## Component Quotients

### `Q_context`

Shared base object.

Carries:

- native boundary version;
- parameter-set identity;
- verifier family/version;
- chain/application domain;
- account creation domain;
- optional aggregation mode/domain.

Every component quotient must project to `Q_context`.

### `Q_account`

Carries:

- account namespace;
- account public identity/root namespace;
- admitted account boundary digest or native identifier;
- context projection.

Projection:

```text
ctx_account : Q_account -> Q_context
```

### `Q_lifecycle`

Carries:

- lifecycle slot identity;
- live/consumed namespace;
- nullifier/successor-state identity;
- replay domain;
- context projection.

Projection:

```text
ctx_lifecycle : Q_lifecycle -> Q_context
```

### `Q_spend`

Carries:

- spend authorization class;
- checked-core or native spend verifier handle;
- allowed signer-local authorization mode;
- message/domain binding semantics;
- context projection.

Projection:

```text
ctx_spend : Q_spend -> Q_context
```

### `Q_transcript`

Carries:

- challenge-binding account surface;
- spend authorization projection;
- lifecycle projection;
- verifier/domain separator;
- aggregation/batch context if enabled;
- context projection.

Projection:

```text
ctx_transcript : Q_transcript -> Q_context
```

### `Q_state`

Carries:

- predecessor-state condition;
- successor-state meaning;
- account-state data-availability boundary;
- consumed/live transition semantics;
- context projection.

Projection:

```text
ctx_state : Q_state -> Q_context
```

## Public Projections

All consensus-facing fields must be projections of:

```text
q : Q_c
```

Required projections:

```text
p_leaf(q)       = leaf fields from q_account, q_lifecycle, q_state
p_spend(q)      = authorization frame from q_spend
p_lifecycle(q)  = lifecycle/nullifier frame from q_lifecycle
p_transcript(q) = challenge frame from q_transcript
p_state(q)      = state transition boundary from q_state
```

Byte digests may bind encodings of these projections, but they do not define
independent semantic objects.

## Native Validity

The native check is:

```text
PullbackValid_c(q) :=
  WellFormedContext(q_context)
  ∧ WellFormedAccount(q_account)
  ∧ WellFormedLifecycle(q_lifecycle)
  ∧ WellFormedSpend(q_spend)
  ∧ WellFormedTranscript(q_transcript)
  ∧ WellFormedState(q_state)
  ∧ ctx_account(q_account)=q_context
  ∧ ctx_lifecycle(q_lifecycle)=q_context
  ∧ ctx_spend(q_spend)=q_context
  ∧ ctx_transcript(q_transcript)=q_context
  ∧ ctx_state(q_state)=q_context
```

This is public and compact if each component is public and well-formedness is
native.

## Private Representative Shape

First-pass:

```text
H_c =
  MeaningLift_c
  × OpeningLocal_c
  × Gauge_c
  × Randomizer_c
```

Boundary map:

```text
boundary_c(h) =
  pullback boundary induced by MeaningLift_c(h)
```

Gauge and randomizer coordinates must not affect `boundary_c` or `meaning_c`.

## Gate A1: Projection Completeness

First-pass result: likely pass if implemented strictly.

Reason:

The candidate is designed so all public fields are projections of `q`. Any
field that cannot be expressed as one of:

```text
p_leaf, p_spend, p_lifecycle, p_transcript, p_state
```

must be reclassified as byte/data-availability binding or treated as a
redesign blocker.

Open audit:

- older checked-core carrier compatibility names;
- creation proof binding digest;
- transparent proof public view fields;
- role subcommitment handles.

## Gate A2: Safety Saturation

First-pass result: plausible but not automatic.

Target:

```text
boundary_c(h)=boundary_c(h') => meaning_c(h)=meaning_c(h')
```

This holds if `boundary_c` is defined by projecting `MeaningLift_c` into the
five component quotients and if `meaning_c` ignores `Gauge_c`, `OpeningLocal_c`
where non-semantic, and `Randomizer_c`.

Risk:

If `OpeningLocal_c` contains representative-specific spend authority that can
change accepted spend behavior without changing `q`, safety saturation fails.

## Gate A3: Effective Descent

First-pass result: main blocker.

Target:

```text
PullbackValid_c(q)
  => exists h. boundary_c(h)=q ∧ LocalValid_c(h)
```

Pullback consistency alone may only show that public components agree. It does
not automatically construct a signer-held local representative.

This candidate needs one of:

1. component well-formedness strong enough to include existence of local lifts;
2. a canonical lift construction:

   ```text
   canonical_lift_c(q) : H_c / G_c
   ```

3. a small native certificate that the pullback object has a private lift.

If the lift certificate becomes a generic hidden-witness proof, this candidate
should be demoted or hybridized.

## Gate A4: Fiber Privacy

First-pass result: plausible if `q` excludes local/gauge coordinates.

Required:

```text
Fiber(boundary_c,q)
```

must hide:

- pad source;
- masks/randomizers;
- opening-local coordinates;
- private gauge/trivialization;
- proof randomness.

Risk:

If the component quotients encode canonical lift coordinates too explicitly,
`q` can become injective in private structure. That would make the account
boundary valid but non-private.

## Gate A5: Spend Closure

First-pass result: plausible.

Spend should be stated as:

```text
SpendAccept(c, q, sig, msg, state)
```

with public account semantics derived from:

```text
p_spend(q), p_lifecycle(q), p_transcript(q), p_state(q)
```

Signer-held local authorization may depend on a private representative in the
fiber, but public acceptance must not.

Open issue:

Determine whether the checked-core spend verifier can bind only to the
pullback-derived transcript surface, or whether it still expects an independent
opening handle.

## Gate A6: Native Verification

First-pass result: pass for consistency, unclear for lift existence.

Pullback membership is native and public. The risk is that the existence of a
private lift is not implied by pullback membership.

Therefore:

```text
native consistency: likely pass
native effective descent: unclear
```

## Preliminary Disposition

Disposition: `[active-but-incomplete]`.

Reason:

The pullback candidate is strong for projection completeness and public
consistency. It may be insufficient alone because it does not obviously prove
existence of an account-backed private local representative.

Likely role:

This may become the outer semantic boundary for any final design, even if the
effective descent/lift part is supplied by the obstruction or canonical-gauge
candidate.

## Next Questions

1. Can component well-formedness be strengthened so `PullbackValid_c(q)`
   implies existence of a lift?
2. Is there a canonical lift from the pullback object to `H_c / G_c`?
3. Which existing public fields fail to factor through the pullback boundary?
4. Does checked-core spend authorization bind cleanly to `p_transcript(q)`?
5. Are role subcommitment handles semantic projections, native restrictions,
   or non-semantic proof artifacts?

