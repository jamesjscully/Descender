# Path 2 Authorization-Local Classification

Status: first classification pass.

Purpose: decide how signer-held authorization/opening material fits into the
native account model:

```text
H_c(q) =
  LiftCore_c(q)
  × AuthorizationLocal_c(q)
  × Gauge_c(q)
  × Randomizer_c(q)
```

The key question is whether private authorization-local data changes public
spend semantics. If yes, it cannot remain private gauge. If no, it can stay as
signer-held local authorization over a public class fixed by `q`.

## Main Finding

`AuthorizationLocal_c(q)` should be treated as:

```text
private semantic-local signing material over a fixed public authorization class
```

not as pure gauge.

That is:

```text
auth ∈ AuthorizationLocal_c(q)
```

may be necessary to produce a spend, but all compatible `auth` values over the
same public boundary must realize the same public spend class:

```text
auth_compatible_c(q, auth)
  => AuthClass_c(auth) = p_spend(q)
```

This keeps account meaning fiber-saturated while allowing private signing
material to remain hidden.

## Classification

### Public Semantic Data

Belongs in:

```text
q : Q_c
M_c
```

Includes:

- account namespace;
- spend authorization class;
- lifecycle/nullifier identity;
- transcript binding surface;
- state transition boundary;
- verifier/parameter/domain context.

Reason:

These determine public account behavior and must be fixed by account creation.

### Authorization-Local Data

Belongs in:

```text
AuthorizationLocal_c(q)
```

Includes:

- private signing/opening material needed to produce spend evidence;
- private trapdoor/local witness for the spend relation;
- one-time local authorization state, if its public lifecycle effect is fixed
  by `q`;
- checked-core private opening material, if it only realizes the public
  authorization class fixed by `p_spend(q)`.

Allowed only if:

```text
AuthClass_c(auth) = p_spend(q)
```

for every compatible `auth`.

### Pure Gauge

Belongs in:

```text
Gauge_c(q)
```

Includes:

- representation choices;
- local trivialization choices;
- pad/mask presentation choices that do not affect authorization class;
- equivalent opening presentations with the same `AuthClass_c`.

Allowed only if:

```text
boundary_c(g.h)=boundary_c(h)
meaning_c(g.h)=meaning_c(h)
AuthClass_c(g.auth)=AuthClass_c(auth)
```

### Randomizer

Belongs in:

```text
Randomizer_c(q)
```

Includes:

- blinding/noise;
- one-time pads/masks used only for hiding;
- residual proof randomness;
- simulator-compatible transcript randomness.

Allowed only if:

```text
randomizer changes no public semantic class.
```

## Public Spend Class Law

The native design must state:

```text
public_spend_class_law:
  forall q auth,
    obs_c(q)=0
    -> auth_compatible_c(q, auth)
    -> AuthClass_c(auth)=p_spend(q)
```

This is the central authorization-local theorem.

It is weaker and cleaner than proving that the verifier knows the whole private
opening at account creation. It only proves that any valid private opening over
the admitted boundary belongs to the public authorization class.

## Spend Closure Law

Spend should be stated as:

```text
SpendAccept(c, q, sig, msg, state)
```

with public verification using:

```text
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
```

Signer-side production may use:

```text
auth ∈ AuthorizationLocal_c(q)
```

but verifier acceptance must not depend on a hidden semantic field outside
`q`.

Target:

```text
spend_closure_v1:
  obs_c(q)=0
  ∧ auth_compatible_c(q,auth)
  ∧ SpendProduced(c,q,auth,msg,state,sig)
  ∧ SpendAccept(c,q,sig,msg,state)
    =>
  spend semantics are determined by meaning_of_boundary_c(q)
```

## Dead Condition

Reject any design where:

```text
exists q auth auth'.
  obs_c(q)=0
  ∧ auth_compatible_c(q,auth)
  ∧ auth_compatible_c(q,auth')
  ∧ AuthClass_c(auth) != AuthClass_c(auth')
```

because then one public account boundary admits multiple spend meanings.

Such data must be moved into:

```text
p_spend(q)
```

or the design needs an account-creation proof that binds the selected
authorization class into `q`.

## What This Changes

The old hidden-witness framing asked account creation to prove:

```text
there exists a full private opening witness h
```

The native framing asks account creation to fix:

```text
p_spend(q)
```

and requires signer-held `auth` to be a private realization of that class.

This turns the account-creation theorem into:

```text
accepted q => public authorization class is well formed
```

and the signer/keygen theorem into:

```text
honest setup can sample auth compatible with q
```

It does not require public account creation to reveal or prove every
authorization-local coordinate.

## Honest Sampling Requirement

The construction must provide:

```text
sample_auth_c(q) -> auth
```

such that:

```text
auth_compatible_c(q,auth)
```

with enough entropy/hiding:

```text
q does not determine auth
```

This is a keygen/signer-state theorem, not necessarily an account verifier
theorem.

If only the honest signer needs to know `auth`, account creation can verify
`q` publicly while signer setup separately guarantees the wallet has a valid
private fiber point.

## Break Surface

Named breaks:

```text
authorization_class_binding_break
authorization_local_sampling_break
authorization_local_hiding_break
spend_closure_break
lifecycle_authorization_mismatch_break
transcript_authorization_binding_break
```

These are narrower than the previous generic hidden-witness proof failures.

## Impact On Candidate Hybrid

Candidate A:

Pullback boundary should include `p_spend(q)` as a public semantic projection.

Candidate B:

`O_spend_state` must include a defect checking that transcript/lifecycle/state
surfaces bind the same public spend class:

```text
p_spend(q)
```

Candidate C:

`AuthorizationLocal_c(q)` is not pure gauge. The gauge action may change its
presentation, but not its public `AuthClass_c`.

## Current Assessment

The native route remains viable under this classification.

Reason:

We do not need authorization-local material to be gauge. We only need:

```text
all compatible authorization locals over q have the same public class.
```

This is a much smaller theorem than full hidden-witness account-opening proof.

Remaining hard question:

Can the concrete spend verifier be redesigned or interpreted so that its public
verification key/authorization handle is exactly:

```text
p_spend(q)
```

and not an independent digest or hidden opening commitment?

## Next Required Work

Create:

```text
research/path2_spend_closure_over_q.md
```

to state the spend relation over the native public boundary:

```text
SpendAccept(c, q, sig, msg, state)
```

and audit whether checked-core/current spend verification can consume
`p_spend(q)`, `p_lifecycle(q)`, `p_transcript(q)`, and `p_state(q)` without
independent semantic handles.

