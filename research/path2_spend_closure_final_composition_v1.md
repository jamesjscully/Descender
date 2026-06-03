# Path 2 Spend Closure Final Composition V1

Status: first composition target for the active native proof side.

Purpose: compose the pinned proof pieces into the deployable active theorem:

```text
accepted spends are semantic actions over meaning_of_boundary_c(q)
```

without requiring consensus to extract private signer state at account creation.

## Inputs Now Pinned

The active interface now has:

```text
PublicAdmissionSoundness
HonestCreationSoundness
AuthorizationClassBinding
ProjectionCompleteness
CheckedCoreFrameBinding
FiberNoExportSoundness
SpendClosure
```

The older stronger:

```text
EffectiveDescent
AccountAdmissionSoundness
```

is retained only for optional spendability-certified account admission.

## Consensus Account Admission

Consensus account creation proves:

```text
AcceptCreate(c,q)
  =>
obs_c(q)=0
∧ public_boundary_sound_c(q)
```

or a named break.

This establishes one coherent public account boundary. It does not prove that
every adversarial `q` is spendable.

## Honest Wallet/Keygen Spendability

Honest creation proves:

```text
HonestCreate_c(seed)=Some(q,h)
  =>
obs_c(q)=0
∧ LocalValid_c(q,h)
∧ AuthorizationCompatibleOverClass_c(q,h)
```

This means honest users have signer-held private state over the public boundary
they publish.

## Authorization Binding

For any compatible private authorization:

```text
auth_compatible_c(q,auth)
  =>
AuthClass_c(auth)=p_spend(q)
```

This prevents private authorization-local material from selecting a different
public spend meaning after account admission.

## Projection Completeness

Projection completeness states:

```text
meaning_of_boundary_c(q) =
  (
    p_account(q),
    p_spend(q),
    p_lifecycle(q),
    p_transcript(q),
    p_state(q),
    p_context(q)
  )
```

Every consensus-visible semantic field must be a projection of `q` or a byte
binding/data-availability handle for one projection.

The recent spend projection audit strengthened:

```text
p_spend(q)
```

to include or edge-link:

```text
allowed_state_transition_class
aggregation_mode
```

## Checked-Core Frame Binding

Checked-core spend verification is allowed only under:

```text
checked_core_frame(q,msg,state,commitment)
  =
frame(
  p_spend(q),
  p_lifecycle(q),
  p_transcript(q),
  p_state(q),
  msg,
  state,
  p_context(q),
  commitment
)
```

Any acceptance under a different independent handle exports:

```text
checked_core_frame_binding_break
```

or:

```text
domain_binding_or_qrom_break
```

## Fiber No-Export

Public views must be simulatable from:

```text
q, projections of q, msg, state, signature/proof bytes, public metadata
```

without:

```text
authorizationLocal
gauge
randomizer
private lift representative coordinates
```

or a named privacy/no-export break is exported.

This ensures the active route is not merely sound, but also avoids leaking the
private fiber.

## Final Spend Theorem

Target:

```text
active_spend_soundness_v1:
  AcceptCreate(c,q)
  ∧ SpendAccept(c,q,sig,msg,state)
    =>
  exists action.
    action.meaning = meaning_of_boundary_c(q)
    ∧ action.message = msg
    ∧ action.authorization = p_spend(q)
    ∧ action.lifecycle = p_lifecycle(q)
    ∧ action.state_transition = p_state(q)
```

or one named break from:

```text
context_binding_break
pullback_consistency_break
gluing_obstruction_break
meaning_binding_break
spend_state_closure_break
canonicality_break
authorization_class_binding_break
spend_closure_break
checked_core_frame_binding_break
domain_binding_or_qrom_break
byte_data_availability_break
fiber_privacy_break
authorization_local_hiding_break
```

## Honest Produced Spend Theorem

For wallet-created accounts:

```text
honest_produced_spend_soundness_v1:
  HonestCreate_c(seed)=Some(q,h)
  ∧ SpendProduced(c,q,h,msg,state,sig)
  ∧ SpendAccept(c,q,sig,msg,state)
    =>
  exists action.
    action.meaning = meaning_of_boundary_c(q)
    ∧ action.authorization = AuthClass_c(h)
    ∧ action.authorization = p_spend(q)
```

The second equality follows from:

```text
HonestCreationSoundness
AuthorizationClassBinding
```

or directly from:

```text
AuthorizationCompatibleOverClass
```

## Composition Argument

1. `AcceptCreate(c,q)` gives `obs_c(q)=0` and
   `public_boundary_sound_c(q)`, or a public admission break.
2. `obs_c(q)=0` and public boundary soundness give a coherent
   `meaning_of_boundary_c(q)`.
3. Projection completeness identifies every public semantic component with a
   projection of `q`.
4. `CheckedCoreSpendReduction` shows every accepted spend exports checked-core
   evidence and binds the frame to projections of `q`, message, state, and
   prechallenge proof commitment.
5. That reduction derives `SpendClosure`, yielding an action whose
   authorization/lifecycle/state fields are `p_spend(q)`, `p_lifecycle(q)`,
   and `p_state(q)`.
6. If signer production is included, authorization class binding shows the
   private authorization local realizes exactly `p_spend(q)`.
7. Fiber no-export shows the public view does not reveal private fiber
   coordinates beyond allowed public projections and proof bytes.

Therefore an accepted spend is a public semantic action over the admitted
native boundary.

## What This Does Not Prove

This does not prove:

- concrete lattice/check-core hardness;
- implementation constant-time behavior;
- Rust serializer no-export tests;
- economic/state-rent policy for unspendable accounts;
- external review.

Those are separate deployability gates. This composition is the proof-side
native boundary theorem shape.

## Remaining Proof-Side Gaps

The theorem interfaces are now aligned, but the following still need concrete
instantiation before claiming the proof side is closed:

1. Concrete encoding of `p_spend(q)` after adding
   `allowed_state_transition_class` and `aggregation_mode`.
2. Concrete `O_context`/pullback equalities for verifier family, domain, and
   aggregation mode.
3. Concrete checked-core frame byte encoder audit against the theorem field
   list.
4. Concrete public-view/no-export allowlist and denylist for active serializers.
5. Byte/state model refresh after projection-strengthening.

## Verdict

No new cryptographic impossibility surfaced in the composition step.

The proof side now appears to be a concrete instantiation problem rather than a
hidden-witness extraction problem:

```text
define exact projection encodings
define exact gluing checks
bind checked-core frame to projections
enforce no-export on public views
```

The next highest-value move is to refresh the byte/state model after the
projection-strengthening, because this checks whether the proof cleanup changed
the blockchain-facing size story.
