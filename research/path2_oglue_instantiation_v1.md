# Path 2 `O_gluing` Instantiation V1

Status: first concrete gluing-obstruction candidate.

Purpose: instantiate the previously named obstruction component:

```text
O_gluing
```

as a public algebraic check over semantic faces of `q`, without exposing raw
private representative coordinates.

## Design Goal

We need:

```text
gluing_defect(q)=0
```

to mean:

```text
AccountFace
SpendFace
LifecycleFace
TranscriptFace
StateFace
```

are compatible restrictions of one public semantic boundary.

This is not yet the full private lift theorem:

```text
obs_c(q)=0 => H_c(q) nonempty
```

It is the first half:

```text
obs_c(q)=0 => q is one globally coherent public account boundary.
```

## Public Face Modules

Instantiate each public face as an element of a finite module over the selected
native arithmetic domain:

```text
R_c = q12289 module / byte-digest equality module / product module
```

First-pass:

```text
AccountFace_c    ∈ A_c
SpendFace_c      ∈ S_c
LifecycleFace_c  ∈ L_c
TranscriptFace_c ∈ T_c
StateFace_c      ∈ D_c
OverlapFace_c    ∈ E_c
```

The faces may be digest-backed, but the gluing check operates on their public
semantic projections:

```text
sem_account
sem_spend
sem_lifecycle
sem_transcript
sem_state
```

not on hidden witness bytes.

## Compatibility Edge Modules

Define pairwise compatibility edge spaces:

```text
E_AS  account-spend compatibility
E_AL  account-lifecycle compatibility
E_ST  spend-transcript compatibility
E_LT  lifecycle-transcript compatibility
E_LD  lifecycle-state compatibility
E_AD  account-state compatibility
E_SD  spend-state compatibility
```

These are public modules or equality-check spaces.

Each edge has a public restriction map:

```text
r_AS : A_c × S_c -> E_AS
r_AL : A_c × L_c -> E_AL
r_ST : S_c × T_c -> E_ST
r_LT : L_c × T_c -> E_LT
r_LD : L_c × D_c -> E_LD
r_AD : A_c × D_c -> E_AD
r_SD : S_c × D_c -> E_SD
```

The intended zero value means the two faces agree on their shared semantic
boundary.

## Concrete Gluing Defect

Define:

```text
O_gluing =
  E_AS × E_AL × E_ST × E_LT × E_LD × E_AD × E_SD
```

and:

```text
gluing_defect(q) =
  (
    r_AS(q.account, q.spend),
    r_AL(q.account, q.lifecycle),
    r_ST(q.spend, q.transcript),
    r_LT(q.lifecycle, q.transcript),
    r_LD(q.lifecycle, q.state),
    r_AD(q.account, q.state),
    r_SD(q.spend, q.state)
  )
```

Accepted zero:

```text
gluing_defect(q)=0
```

means every adjacent semantic face agrees on its overlap.

This is simpler than the earlier abstract expression:

```text
d_1(OverlapFace_c(q)) - restrictions = 0
```

but equivalent in purpose. The explicit edge product is easier to audit and
later optimize into a coboundary/syndrome equation if desired.

## Edge Contents

### `E_AS`: Account-Spend

Checks:

```text
account identity in p_account(q)
  =
authorization identity / public key identity in p_spend(q)
```

and:

```text
spend verifier domain belongs to account creation domain
```

Break:

```text
meaning_binding_break
authorization_class_binding_break
```

### `E_AL`: Account-Lifecycle

Checks:

```text
account namespace in p_account(q)
  =
nullifier/replay namespace in p_lifecycle(q)
```

and:

```text
slot/lifecycle identity is scoped to this account.
```

Break:

```text
spend_state_closure_break
```

### `E_ST`: Spend-Transcript

Checks:

```text
p_transcript(q) binds p_spend(q)
```

and:

```text
checked-core frame challenge input includes the spend authorization class.
```

and:

```text
p_transcript(q).aggregation_mode = p_spend(q).aggregation_mode
```

Break:

```text
checked_core_frame_binding_break
domain_binding_or_qrom_break
```

### `E_LT`: Lifecycle-Transcript

Checks:

```text
p_transcript(q) binds p_lifecycle(q)
```

so a spend challenge cannot be replayed across lifecycle/nullifier classes.

Break:

```text
spend_state_closure_break
checked_core_frame_binding_break
```

### `E_LD`: Lifecycle-State

Checks:

```text
p_state(q) consumes the live state named by p_lifecycle(q)
```

and:

```text
successor/consumed state belongs to the same lifecycle class.
```

Break:

```text
spend_state_closure_break
```

### `E_AD`: Account-State

Checks:

```text
predecessor account state root belongs to p_account(q)
```

and:

```text
admitted boundary id is recorded in the account state namespace.
```

Break:

```text
projection_factorization_break
byte_data_availability_break
```

### `E_SD`: Spend-State

Checks:

```text
p_spend(q) authorizes the state transition class p_state(q)
```

Concretely:

```text
p_spend(q).allowed_state_transition_class
  =
p_state(q).state_transition_class
```

This is the edge that prevents a public boundary from admitting a spend class
that can act on a different state object.

Break:

```text
spend_closure_break
spend_state_closure_break
```

## Algebraic Carrier Choices

There are three viable carriers.

### Carrier 1: Product Of Equality/Digest Checks

Each edge is a tuple of canonical byte/digest equality checks.

Pros:

- simplest;
- likely enough for early Lean interface;
- no hidden witness proof.

Cons:

- less elegant;
- may duplicate fields in public `q`;
- gives weaker algebraic compression.

Disposition:

```text
active_first_instantiation
```

### Carrier 2: q12289 Linear Syndrome

Encode semantic face identifiers as field/module elements and define:

```text
gluing_defect(q) = G_c · encode_faces(q)
```

for a public sparse matrix `G_c`.

Pros:

- matches native arithmetic;
- compact verifier operations;
- aggregation-friendly.

Cons:

- requires canonical field encoding of semantic digests/handles;
- needs collision/domain assumptions for digest-to-field mapping;
- can hide semantic mismatches if the map is underconstrained.

Disposition:

```text
research_next_after_equality_product
```

### Carrier 3: Cech/Coboundary Module

Treat faces as a cover and overlaps as a public one-cocycle:

```text
gluing_defect = d_1(overlap_data)
```

Pros:

- most geometrically faithful;
- likely best aggregation/composition story.

Cons:

- requires careful definition of cover, sections, and overlap modules;
- may be overkill for direct deployment.

Disposition:

```text
defer_until_product_edges_are_stable
```

## Effective Public Boundary Theorem

First theorem target:

```text
gluing_zero_public_boundary_v1:
  context_defect(q)=0
  ∧ pullback_defect(q)=0
  ∧ gluing_defect(q)=0
  ∧ meaning_defect(q)=0
  ∧ spend_state_defect(q)=0
  ∧ canonicality_defect(q)=0
    =>
  exists publicBoundary.
    publicBoundary.meaning = meaning_of_boundary(q)
    ∧ publicBoundary.account = p_account(q)
    ∧ publicBoundary.spend = p_spend(q)
    ∧ publicBoundary.lifecycle = p_lifecycle(q)
    ∧ publicBoundary.transcript = p_transcript(q)
    ∧ publicBoundary.state = p_state(q)
```

This does **not** claim private lift nonemptiness. It gives the global public
semantic object.

## Relation To Lift Nonemptiness

After this theorem, the remaining lift theorem is narrower:

```text
publicBoundary over M_c
  => H_c(q) nonempty
```

or more concretely:

```text
publicBoundary
  => exists auth gauge randomizer.
       auth_compatible_c(q,auth)
       ∧ gauge_valid_c(q,gauge)
       ∧ randomizer_valid_c(q,randomizer)
```

This split is useful because failure of `O_gluing` and failure of private
authorization sampling are now different blockers.

## Pass / Fail Criteria

Pass this instantiation if:

- every edge check is public and deterministic;
- edge zero implies projection compatibility;
- no edge requires raw private coordinates;
- checked-core transcript binding can be expressed in `E_ST` and `E_LT`;
- lifecycle/state closure can be expressed in `E_LD`, `E_AD`, and `E_SD`.

Fail or redesign if:

- any edge requires proving a hidden witness relation;
- any edge depends on private pad/mask/opening coordinates;
- an independent handle is needed that does not factor through a projection;
- digest equality is used as semantic proof of hidden opening rather than byte
  binding for a public projection.

## Current Assessment

`O_gluing` can be instantiated concretely as a product of public edge
compatibility checks.

This is enough to make the public boundary coherent. It does not solve private
lift nonemptiness or authorization-local sampling, but it removes the ambiguity
around "some object glues."

Next active blocker:

```text
lift nonemptiness / authorization-local sampling
```

because `O_gluing` no longer needs to carry hidden witness existence by itself.

## Next Required Work

Create:

```text
research/path2_lift_nonemptiness_v1.md
```

to prove or state:

```text
obs_c(q)=0 => H_c(q) nonempty
```

under the new split:

```text
public gluing first, private fiber construction second.
```
