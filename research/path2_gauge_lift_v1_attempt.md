# Path 2 Gauge/Lift V1 Attempt

Status: first concrete lift/fiber attempt.

Purpose: address the residual blocker from
`research/path2_obstruction_map_v1_attempt.md`:

```text
global public boundary q
  => private representative fiber exists
```

without returning to a generic hidden-witness proof.

## Design Position

The current hybrid design says:

```text
obs_c(q)=0
```

should produce a well-formed global public boundary and semantic meaning:

```text
meaning_of_boundary_c(q) : M_c
```

The lift/gauge theorem must then supply:

```text
exists h.
  boundary_c(h)=q
  ∧ LocalValid_c(h)
  ∧ meaning_c(h)=meaning_of_boundary_c(q)
```

with private material living in the fiber over `q`.

## Core Construction

For a valid public boundary `q`, define:

```text
H_c(q) =
  LiftCore_c(q)
  × AuthorizationLocal_c(q)
  × Gauge_c(q)
  × Randomizer_c(q)
```

and:

```text
H_c = Sigma q : Q_c, H_c(q)
```

The boundary map forgets private coordinates:

```text
boundary_c(q, lift, auth, gauge, randomizer) = q
```

The semantic map depends only on `q`:

```text
meaning_c(q, lift, auth, gauge, randomizer) =
  meaning_of_boundary_c(q)
```

provided `auth` is compatible with `q`.

This makes safety saturation structural:

```text
boundary_c(h)=boundary_c(h') => meaning_c(h)=meaning_c(h')
```

## Coordinate Classes

### `LiftCore_c(q)`

Role:

Canonical or semi-canonical local lift of the public boundary.

Allowed contents:

- canonical local section modulo gauge;
- public-face realization data;
- deterministic normalization witnesses that are not secret;
- finite choice data that does not affect semantics.

Must satisfy:

```text
lift_valid_c(q, lift)
```

and:

```text
lift_valid_c(q, lift) => boundary_core(lift)=q
```

Privacy rule:

`LiftCore_c(q)` may be canonical or small. It is not the hiding source.

### `AuthorizationLocal_c(q)`

Role:

Signer-held local material needed to produce future spends.

This is the dangerous class. It is not automatically gauge.

Allowed contents:

- local authorization secret/trapdoor tied to `p_spend(q)`;
- opening-local data that produces a spend proof/signature;
- private state needed for one-time/lifecycle consumption.

Must satisfy:

```text
auth_compatible_c(q, auth)
```

and public spend semantics must be determined by `q`:

```text
auth_compatible_c(q, auth)
  => public_spend_meaning(auth) = p_spend(q)
```

Classification:

`AuthorizationLocal_c(q)` is semantic-local, not pure gauge. It may be private,
but changes inside this class must not change public account meaning.

### `Gauge_c(q)`

Role:

Safety-irrelevant local trivialization choices.

Allowed contents:

- pad gauge;
- mask gauge;
- local section basis/trivialization choices;
- equivalent representation choices;
- non-semantic opening presentation choices.

Must satisfy invariance:

```text
meaning_c(g.h)=meaning_c(h)
boundary_c(g.h)=boundary_c(h)
auth_compatible_c(q, auth) preserved
```

Dead if:

```text
gauge changes accepted spend behavior
```

### `Randomizer_c(q)`

Role:

Hiding/noise/sampling material.

Allowed contents:

- blinding/noise;
- proof-system randomness if a residual proof remains;
- mask sampling randomness;
- simulator-compatible public-view randomness.

Must satisfy:

```text
randomizer changes neither boundary nor meaning
```

and must be sampled with enough entropy under the public boundary `q`.

## Lift Existence Theorem V1

Target:

```text
lift_exists_v1:
  obs_c(q)=0
    =>
  exists lift auth gauge randomizer.
    lift_valid_c(q, lift)
    ∧ auth_compatible_c(q, auth)
    ∧ gauge_valid_c(q, gauge)
    ∧ randomizer_valid_c(q, randomizer)
```

Then define:

```text
h = (q, lift, auth, gauge, randomizer)
```

and get:

```text
boundary_c(h)=q
meaning_c(h)=meaning_of_boundary_c(q)
LocalValid_c(h)
```

where:

```text
LocalValid_c(h) :=
  obs_c(q)=0
  ∧ lift_valid_c(q,lift)
  ∧ auth_compatible_c(q,auth)
  ∧ gauge_valid_c(q,gauge)
  ∧ randomizer_valid_c(q,randomizer)
```

## Important Shift

This does not try to recover a pre-existing hidden witness.

It defines private signer state as a fiber over the public boundary:

```text
private state is chosen after / alongside q as a local lift
```

This is allowed because the project can redesign private structure and public
boundary together.

## Spend Authority Classification

Spend authority must be handled carefully.

### Acceptable Model 1: Boundary-Determined Authority

```text
p_spend(q)
```

fully determines public verification semantics. Private `auth` only enables
the signer to produce valid spend evidence.

This is preferred.

### Acceptable Model 2: Local Authorization Over Fixed Public Class

There may be many valid `auth` values over one `q`, but every compatible `auth`
has the same public authorization class:

```text
auth_compatible_c(q,auth)
  => AuthClass(auth)=p_spend(q)
```

This preserves safety saturation.

### Dead Model: Gauge Changes Authority

Reject:

```text
exists auth auth'.
  auth_compatible_c(q,auth)
  ∧ auth_compatible_c(q,auth')
  ∧ AuthClass(auth) != AuthClass(auth')
```

unless the differing authorization class is moved into `q` and therefore into
`M_c`.

## Fiber Privacy Theorem V1

Target:

```text
fiber_privacy_v1:
  public q reveals meaning_of_boundary_c(q)
  but not auth, gauge, or randomizer
```

Concrete sufficient conditions:

1. `boundary_c` erases `auth`, `gauge`, and `randomizer`;
2. public projections factor only through `q`;
3. `auth_compatible_c(q,auth)` has many valid private choices or is protected
   by an independent hard assumption;
4. `Gauge_c(q)` and `Randomizer_c(q)` have sufficient entropy;
5. sampling `q` does not condition private choices into a small subset.

This theorem may still require a cryptographic hiding assumption for
authorization-local material. That is acceptable if the semantic consistency
and lift existence are native.

## Gauge Invariance Theorem V1

Target:

```text
gauge_invariance_v1:
  gauge_valid_c(q,g)
    =>
  boundary_c(g.h)=boundary_c(h)
  ∧ meaning_c(g.h)=meaning_c(h)
  ∧ spend_public_semantics(g.h)=spend_public_semantics(h)
  ∧ lifecycle_semantics(g.h)=lifecycle_semantics(h)
```

This theorem is what permits gauge to be private and uncommitted.

## Local Validity Without Generic ZK

The local validity relation is redesigned:

```text
LocalValid_c(q,lift,auth,gauge,randomizer)
```

not:

```text
old_hidden_witness_layout_is_valid(h)
```

So account creation can proceed as:

```text
1. verify obs_c(q)=0 publicly;
2. signer stores any compatible private fiber point h over q;
3. future spend proves/uses auth as required by spend relation.
```

This may shift some authentication proof burden to spend, but only if spend
closure over `q` is preserved.

## Open Classification Table

| coordinate | proposed class | safety note |
|---|---|---|
| account namespace | public meaning `q` | semantic |
| spend authorization class | public meaning `q` | semantic |
| lifecycle/nullifier identity | public meaning `q` | semantic |
| transcript binding surface | public meaning `q` | semantic |
| state transition boundary | public meaning `q` | semantic |
| checked-core private opening local | `AuthorizationLocal_c(q)` | private, not gauge unless public class fixed |
| pad source | `Gauge_c(q)` or `Randomizer_c(q)` | must not affect spend semantics |
| mask source | `Gauge_c(q)` or `Randomizer_c(q)` | must not affect spend semantics |
| blinding/noise | `Randomizer_c(q)` | hiding only |
| local section coordinates | `LiftCore_c(q)` plus gauge | semantic projection must be fixed by q |
| role presentation choices | `Gauge_c(q)` | dead if role choice changes public meaning |
| proof query randomness | `Randomizer_c(q)` | only if residual proof remains |

## Current Assessment

This lift/gauge model makes the remaining proof problem much narrower:

Old problem:

```text
prove arbitrary hidden witness h is valid and non-leaking
```

New problem:

```text
construct private fiber H_c(q) over a public valid boundary q
and classify which private coordinates are semantic-local vs pure gauge.
```

The biggest unresolved issue is:

```text
AuthorizationLocal_c(q)
```

If authorization-local material can be sampled privately while its public class
is fixed by `q`, the native route remains viable. If not, the authorization
material must either move into the public boundary or require a residual proof.

## Next Required Work

Create:

```text
research/path2_authorization_local_classification.md
```

to decide:

1. What exactly is `AuthorizationLocal_c(q)`?
2. Does the checked-core/private opening material determine only spend evidence,
   or does it alter public spend class?
3. Can multiple authorization locals over one `q` be valid without changing
   public spend semantics?
4. If authorization-local hiding needs an assumption, is it smaller than the
   old full hidden-witness proof?

