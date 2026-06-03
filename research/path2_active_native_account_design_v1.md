# Path 2 Active Native Account Design V1

Status: active native design consolidation.

Purpose: consolidate the selected hybrid strategy into one theorem target before
adding a Lean interface or updating byte/proof-system models.

This is the current active path for escaping the generic hidden-witness ZK
bottleneck.

## Selected Design

The selected design is the hybrid:

```text
Q_c      = pullback-organized semantic public boundary
M_c      = semantic account meaning fixed by q
O_c      = obstruction/check object
obs_c    : Q_c -> O_c
H_c(q)   = private fiber over q
G_c(q)   = private gauge/randomizer action
```

Account creation verifies:

```text
obs_c(q)=0
```

Spend verifies:

```text
SpendAccept(c,q,sig,msg,state)
```

Private signer state is:

```text
h ∈ H_c(q)
```

not an independent hidden witness that the verifier must reconstruct at account
creation.

## Semantic Meaning `M_c`

Use:

```text
M_c =
  AccountNamespace_c
  × SpendAuthorizationClass_c
  × LifecycleIdentity_c
  × TranscriptBindingSurface_c
  × StateTransitionBoundary_c
  × ParameterContext_c
```

The public boundary determines meaning:

```text
meaning_of_boundary_c : Q_c -> M_c
```

and every private representative over `q` must have that meaning:

```text
h ∈ H_c(q) => meaning_c(h)=meaning_of_boundary_c(q)
```

Safety equivalence:

```text
h ~safe h' iff meaning_c(h)=meaning_c(h')
```

## Public Boundary `Q_c`

First active shape:

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

Each face is public semantic boundary data, not raw private witness material.

Semantic projections:

```text
sem_account    : AccountFace_c    -> AccountNamespace_c
sem_spend      : SpendFace_c      -> SpendAuthorizationClass_c
sem_lifecycle  : LifecycleFace_c  -> LifecycleIdentity_c
sem_transcript : TranscriptFace_c -> TranscriptBindingSurface_c
sem_state      : StateFace_c      -> StateTransitionBoundary_c
sem_context    : ContextFace_c    -> ParameterContext_c
```

Boundary meaning:

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

## Obstruction Object `O_c`

First active shape:

```text
O_c =
  O_context
  × O_pullback
  × O_gluing
  × O_meaning
  × O_spend_state
  × O_canonicality
```

Obstruction map:

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

Accepted account boundary:

```text
obs_c(q)=0
```

The obstruction components mean:

- `context_defect=0`: parameter/domain context is well formed;
- `pullback_defect=0`: all public faces live over one context;
- `gluing_defect=0`: public faces are compatible restrictions of one boundary;
- `meaning_defect=0`: the boundary realizes one `M_c`;
- `spend_state_defect=0`: spend/lifecycle/transcript/state surfaces agree;
- `canonicality_defect=0`: public encoding is canonical and unambiguous.

## Private Fiber `H_c(q)`

For valid `q`, define:

```text
H_c(q) =
  LiftCore_c(q)
  × AuthorizationLocal_c(q)
  × Gauge_c(q)
  × Randomizer_c(q)
```

Boundary projection:

```text
boundary_c(q,lift,auth,gauge,randomizer)=q
```

Semantic projection:

```text
meaning_c(q,lift,auth,gauge,randomizer)=meaning_of_boundary_c(q)
```

provided:

```text
auth_compatible_c(q,auth)
```

## Authorization Local

`AuthorizationLocal_c(q)` is private semantic-local signing material over a
fixed public authorization class. It is not pure gauge.

Key law:

```text
auth_compatible_c(q,auth)
  => AuthClass_c(auth)=p_spend(q)
```

Dead condition:

```text
exists auth auth'.
  auth_compatible_c(q,auth)
  ∧ auth_compatible_c(q,auth')
  ∧ AuthClass_c(auth) != AuthClass_c(auth')
```

unless the differing authorization class is moved into `q`.

## Gauge And Randomizer

`Gauge_c(q)` carries private representation/trivialization choices.

`Randomizer_c(q)` carries hiding/noise/sampling material.

Gauge/randomizer invariance:

```text
boundary_c(g.h)=boundary_c(h)
meaning_c(g.h)=meaning_c(h)
spend_public_semantics(g.h)=spend_public_semantics(h)
lifecycle_semantics(g.h)=lifecycle_semantics(h)
```

Gauge is dead if it changes public spend, lifecycle, transcript, or state
semantics.

## Public Projections

The active boundary exports:

```text
p_context(q)    : ParameterContext_c
p_account(q)    : AccountNamespace_c
p_spend(q)      : SpendAuthorizationClass_c
p_lifecycle(q)  : LifecycleIdentity_c
p_transcript(q) : TranscriptBindingSurface_c
p_state(q)      : StateTransitionBoundary_c
```

`research/path2_projection_surfaces_v1.md` defines the first concrete contents
of these projections. The design rule is that every consensus-visible semantic
input must be one of these projections, while digest/handle fields are only
byte or data-availability bindings for them.

Compatibility aliases may be defined:

```text
native_boundary_digest(q)
slot_authorization_digest(q)
opening_commitment_alias(q)
pad_seed_digest_alias(q)
```

but they are valid only as projections or byte bindings for `q`. They are not
independent semantic handles.

## Account Creation Theorem

Target:

```text
native_account_admission_v1:
  AcceptCreate(c,q)
    =>
  obs_c(q)=0
  ∧ exists h ∈ H_c(q).
      LocalValid_c(q,h)
      ∧ meaning_c(h)=meaning_of_boundary_c(q)
```

where:

```text
LocalValid_c(q,h) :=
  obs_c(q)=0
  ∧ lift_valid_c(q,h.lift)
  ∧ auth_compatible_c(q,h.auth)
  ∧ gauge_valid_c(q,h.gauge)
  ∧ randomizer_valid_c(q,h.randomizer)
```

or a named break.

This theorem replaces the old account-creation target:

```text
accepting proof extracts a full pre-existing hidden witness
```

with:

```text
accepted public boundary has a valid private fiber.
```

## Effective Descent Decomposition

Proof should decompose as:

```text
obs_c(q)=0
  => global public semantic boundary over M_c
```

plus:

```text
global public semantic boundary
  => H_c(q) nonempty
```

plus:

```text
H_c(q) nonempty and auth compatible
  => semantic safety and spend closure
```

The second line is the remaining construction-specific lift theorem.

## Spend Theorem

Spend is stated over `q`:

```text
SpendAccept(c,q,sig,msg,state)
```

with semantic public inputs:

```text
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
msg
state
sig
```

Target:

```text
spend_closure_v1:
  obs_c(q)=0
  ∧ SpendAccept(c,q,sig,msg,state)
    =>
  accepted spend semantics are determined by meaning_of_boundary_c(q)
```

Signer production may use:

```text
auth ∈ AuthorizationLocal_c(q)
```

but verifier acceptance cannot depend on hidden semantic fields outside `q`.

## Checked-Core Role

Checked-core remains downstream if its frame is:

```text
checked_core_frame(q,msg,state) =
  frame(
    p_spend(q),
    p_lifecycle(q),
    p_transcript(q),
    p_state(q),
    msg,
    state,
    p_context(q)
  )
```

Dead if it binds to an independent semantic handle not projected from `q`.

## Field Disposition

Kept as projections of `q`:

- account/context/version fields;
- public key identity or spend authorization class;
- slot/lifecycle/nullifier identity;
- state boundary;
- transcript binding surface;
- masked public projection if it is a semantic/check face.

Kept only as byte bindings for `q`:

- native boundary digest;
- leaf/context/slot digests;
- proof transcript digests;
- canonical encoding digests.

Kept only as typed restrictions or proof metadata:

- parent section root;
- role handles;
- trace roots;
- column/profile handles.

Private:

- authorization-local opening/signing material;
- pad/mask sources;
- randomizers/blinding/noise;
- local trivialization/gauge.

Dead unless redesigned:

- independent opening commitment;
- independent pad digest;
- independent slot authorization handle;
- role handles as SHA preimage commitments;
- checked-core frame that does not bind `q` projections.

## Named Break Surface

Minimum active break surface:

```text
context_binding_break
pullback_consistency_break
gluing_obstruction_break
meaning_binding_break
spend_state_closure_break
canonicality_break
lift_existence_break
authorization_class_binding_break
authorization_local_sampling_break
authorization_local_hiding_break
gauge_invariance_break
fiber_privacy_break
spend_closure_break
checked_core_frame_binding_break
domain_binding_or_qrom_break
byte_data_availability_break
```

## Current Open Obligations

1. Instantiate concrete algebra for `O_gluing`.
2. Define exact encodings for `p_spend`, `p_lifecycle`, `p_transcript`, and
   `p_state`.
3. Prove or assume lift nonemptiness:

   ```text
   obs_c(q)=0 => H_c(q) nonempty
   ```

4. Prove authorization class binding:

   ```text
   auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)
   ```

5. Prove checked-core frame binding over `q`.
6. Decide whether parent section root/role handles are needed in the active
   native path or only in residual proof/fallback paths.
7. Build Lean theorem-interface module.
8. Reset byte/state model after the theorem interface stabilizes.

## Next Step

Create:

```text
Descender/Path2NativeAccountDesign.lean
```

as a theorem-interface module reflecting this design. Do not instantiate
cryptographic assumptions or byte sizes yet.
