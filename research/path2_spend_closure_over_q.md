# Path 2 Spend Closure Over Native Boundary `q`

Status: first spend-closure contract.

Purpose: state the spend relation over the native public boundary:

```text
SpendAccept(c, q, sig, msg, state)
```

and identify what must be true for checked-core/current spend verification to
consume projections of `q` rather than independent semantic handles.

## Core Rule

After native account creation, spend must be public-boundary closed:

```text
SpendAccept(c, q, sig, msg, state)
```

may inspect only:

```text
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
message
current consensus state
signature bytes
```

as public semantic inputs.

Signer-side production may use:

```text
auth ∈ AuthorizationLocal_c(q)
```

but verifier acceptance must not depend on any hidden semantic field outside
`q`.

## Relation To `M_c`

The semantic account meaning from `research/path2_meaning_v1.md` is:

```text
M_c =
  AccountNamespace_c
  × SpendAuthorizationClass_c
  × LifecycleIdentity_c
  × TranscriptBindingSurface_c
  × StateTransitionBoundary_c
  × ParameterContext_c
```

Spend consumes exactly these public projections:

```text
p_spend(q)      : SpendAuthorizationClass_c
p_lifecycle(q)  : LifecycleIdentity_c
p_transcript(q) : TranscriptBindingSurface_c
p_state(q)      : StateTransitionBoundary_c
p_context(q)    : ParameterContext_c
```

The verifier must not recover account meaning from:

- an independent opening commitment;
- a digest whose semantic preimage is outside `q`;
- a proof transcript field not projected from `q`;
- a private representative coordinate.

## Spend Closure Law V1

Target theorem:

```text
spend_closure_over_q_v1:
  obs_c(q)=0
  ∧ SpendAccept(c,q,sig,msg,state)
    =>
  exists public_action.
    public_action.meaning = meaning_of_boundary_c(q)
    ∧ public_action.message = msg
    ∧ public_action.state_transition = p_state(q)
    ∧ public_action.lifecycle = p_lifecycle(q)
    ∧ public_action.authorization = p_spend(q)
```

This theorem says an accepted spend has no semantic dependency outside the
native boundary.

## Production Soundness Law

Signer-side production may use private authorization-local data:

```text
SpendProduced(c,q,auth,msg,state,sig)
```

but only under:

```text
auth_compatible_c(q,auth)
```

Target:

```text
spend_production_soundness_v1:
  obs_c(q)=0
  ∧ auth_compatible_c(q,auth)
  ∧ SpendProduced(c,q,auth,msg,state,sig)
  ∧ SpendAccept(c,q,sig,msg,state)
    =>
  AuthClass_c(auth)=p_spend(q)
```

This connects `AuthorizationLocal_c(q)` to the public authorization class
without making account creation prove the full private authorization local.

## Checked-Core Interpretation

The current checked-core signature can remain a downstream spend object if its
public verifier frame is rewritten/interpreted as:

```text
checked_core_frame(q,msg,state) =
  frame(
    p_spend(q),
    p_lifecycle(q),
    p_transcript(q),
    p_state(q),
    msg,
    state,
    context(q)
  )
```

Allowed:

- checked-core verifier consumes residues or public authorization handles that
  are projections of `q`;
- checked-core challenge binds `p_transcript(q)`;
- checked-core lifecycle/replay check consumes `p_lifecycle(q)`;
- checked-core state transition check consumes `p_state(q)`;
- private opening/local data is used only by the signer to produce `sig`.

Disallowed:

- checked-core verifier binds to an independent `slot_authorization_digest`
  whose semantic meaning does not factor through `q`;
- verifier accepts a spend using an opening commitment that was not projected
  from `q`;
- future spend challenge depends on account-creation proof transcript bytes
  that are not part of `p_transcript(q)`;
- lifecycle/nullifier semantics are reconstructed from hidden authorization
  local data rather than from `p_lifecycle(q)`.

## Native Boundary Projections Needed

The final native boundary must supply:

```text
p_spend(q)
```

Public authorization class. This may include a checked-core public key,
authorization digest, verifier handle, or native replacement, but it must be
semantic and projected from `q`.

```text
p_lifecycle(q)
```

Live/consumed/nullifier/replay identity.

```text
p_transcript(q)
```

Challenge-binding frame. It must include or derive the account boundary
identifier, spend class, lifecycle identity, context, and any aggregation
domain.

```text
p_state(q)
```

State transition boundary: valid predecessor/successor semantics and data
availability expectations.

## Independent Handle Failure Case

Mark dead unless redesigned:

```text
SpendAccept(c, handle, sig, msg, state)
```

where:

```text
handle != p_spend(q)
```

or where the verifier cannot prove:

```text
meaning(handle)=p_spend(q)
```

from public boundary data.

This is exactly the kind of independent semantic handle that recreates the old
hidden-witness consistency proof.

## Lifecycle Closure

Spend must consume lifecycle meaning from `q`:

```text
LifecycleAccept(c, p_lifecycle(q), state)
```

Target:

```text
lifecycle_closure_over_q:
  obs_c(q)=0
  ∧ SpendAccept(c,q,sig,msg,state)
    =>
  replay/nullifier/consume-once behavior is determined by p_lifecycle(q)
```

If private authorization-local data changes lifecycle behavior, it must move
into `p_lifecycle(q)` or the candidate fails safety saturation.

## Transcript Closure

Challenge derivation must bind the native boundary meaning:

```text
challenge = H_or_RO(
  domain,
  p_transcript(q),
  msg,
  state,
  signature prechallenge material
)
```

Target:

```text
transcript_closure_over_q:
  accepted spend challenge binds p_transcript(q)
```

Failure if:

- the challenge omits `q` or its semantic digest;
- challenge binds an old digest with non-factoring semantics;
- challenge can be replayed under another `q` with different `M_c`.

## State Closure

State transition must be determined by:

```text
p_state(q)
```

and current consensus state.

Target:

```text
state_closure_over_q:
  accepted spend maps an admitted live boundary to the successor state class
  specified by p_state(q)
```

This is the state-machine part of `M_c`.

## Residual Proof Burden

This contract does not prove checked-core soundness. It scopes it.

Residual checked-core theorem:

```text
checked_core_soundness_over_q:
  checked-core accepted under frame(q,msg,state)
    =>
  authorization class p_spend(q) was satisfied
```

This can reuse existing checked-core transcript/forking work if the frame
inputs are all projections of `q`.

Residual authorization theorem:

```text
auth_compatible_c(q,auth)
  => AuthClass_c(auth)=p_spend(q)
```

This belongs to signer/keygen/native-account setup, not to public spend
verification.

## Current Assessment

Spend closure is viable if we enforce one design rule:

```text
the checked-core / spend verifier consumes q-derived projections only.
```

This means `slot_authorization_digest` or any replacement can remain as a byte
handle only if its semantic meaning is:

```text
p_spend(q) or p_transcript(q)
```

not an independent account object.

## Next Required Work

Create:

```text
research/path2_native_boundary_projection_audit.md
```

to audit all current public fields and classify each as:

```text
projection of q
byte/data-availability binding for q
private authorization-local
pure gauge/randomizer
dead independent semantic handle
```

