# Path 2 Semantic Account Meaning V1

Status: first-pass semantic target for native solution search.

Purpose: define `M_c`, the protocol meaning account creation must fix before
we choose the public boundary quotient `Q_c`, private representative space
`H_c`, obstruction map `obs_c`, or proof system.

This document is intentionally upstream of byte layout. It defines what must be
the same for every representative in one accepted public boundary fiber.

## Core Definition

For account context `c`, define:

```text
M_c =
  AccountNamespace_c
  × SpendAuthorizationClass_c
  × LifecycleIdentity_c
  × TranscriptBindingSurface_c
  × StateTransitionBoundary_c
  × ParameterContext_c
```

An admitted public account boundary `q : Q_c` is deployable only if it induces
one well-defined semantic meaning:

```text
meaning_of_boundary_c : Q_c -> M_c
```

and every private representative over `q` has that same meaning:

```text
boundary_c(h)=q => meaning_c(h)=meaning_of_boundary_c(q)
```

Equivalently, the fiber of `boundary_c` over `q` must be semantically
saturated:

```text
boundary_c(h)=boundary_c(h') => meaning_c(h)=meaning_c(h')
```

## Component 1: Account Namespace

`AccountNamespace_c` is the consensus-visible identity of the admitted account.

It must include:

- chain/application domain;
- verifier family/version;
- account creation domain;
- account public identity or account root namespace;
- parameter-set identity;
- any fork/epoch/network scope that changes accepted semantics.

It must not include:

- private pad source;
- private opening tuple;
- private masks/randomizers;
- proof transcript bytes except where they alter account identity;
- implementation storage keys that are not consensus semantics.

Required projection:

```text
m_account_namespace : M_c -> AccountNamespace_c
```

Safety law:

```text
same boundary => same account namespace
```

## Component 2: Spend Authorization Class

`SpendAuthorizationClass_c` is the public class a future spend must bind to.
It is not necessarily a public key in the classical sense; it is the descended
authorization meaning of the account.

It must fix:

- which spend verifier relation is authorized;
- which public account boundary/projection the spend challenge must bind;
- which signer-held local authorization data may be used;
- the checked-core authorization handle or its native replacement;
- message/domain binding semantics for spend.

It must not expose:

- representative-specific opening locals;
- gauge/trivialization coordinates;
- per-spend masks or pads;
- private source coordinates.

Required projection:

```text
m_spend : M_c -> SpendAuthorizationClass_c
```

Spend closure target:

```text
SpendAccept(c, q, sig, msg, state)
```

must depend on the account only through:

```text
m_spend(meaning_of_boundary_c(q))
```

plus signer-held local authorization data.

## Component 3: Lifecycle / Nullifier Identity

`LifecycleIdentity_c` is the consume-once and replay-prevention meaning fixed
at account creation.

It must fix:

- slot/account creation uniqueness namespace;
- live/consumed state identity;
- nullifier or successor-state identity;
- replay domain;
- reorg/fork scope if consensus-visible;
- whether the account is one-time, bounded-use, or lifecycle-indexed.

It must not depend on:

- which representative in the boundary fiber the signer sampled;
- hidden gauge;
- proof randomness;
- non-consensus storage layout.

Required projection:

```text
m_lifecycle : M_c -> LifecycleIdentity_c
```

Safety law:

```text
boundary_c(h)=boundary_c(h')
  => lifecycle_c(h)=lifecycle_c(h')
```

This law is non-negotiable. If it fails, the public boundary admits multiple
replay meanings.

## Component 4: Transcript Binding Surface

`TranscriptBindingSurface_c` is the public semantic material that must be bound
into checked-core or native spend challenges.

It must fix:

- account boundary digest or native boundary identifier;
- spend authorization projection;
- lifecycle/nullifier projection;
- verifier/domain separator;
- message-binding role;
- aggregation/batch context if enabled.

It must separate:

- semantic transcript inputs, which belong in `M_c`;
- byte/data-availability digests, which may bind encodings but do not define
  account meaning by themselves.

Required projection:

```text
m_transcript : M_c -> TranscriptBindingSurface_c
```

Challenge law:

```text
same M_c => same challenge-binding account surface
```

If a challenge can bind to an independent digest whose meaning does not factor
through `M_c`, the candidate fails projection completeness.

## Component 5: State Transition Boundary

`StateTransitionBoundary_c` is the public state-machine effect authorized by
the account.

It must fix:

- admitted account state object;
- required predecessor-state condition;
- valid successor-state meaning;
- consumed/live transition semantics;
- data-availability expectations for account creation evidence;
- optional aggregation state boundary, if aggregation is enabled.

It does not need to fix:

- local database layout;
- wallet storage format;
- non-consensus cache keys;
- implementation-specific restore anchors unless consensus-visible.

Required projection:

```text
m_state : M_c -> StateTransitionBoundary_c
```

State law:

```text
SpendAccept(c, q, sig, msg, state)
  => state transition is determined by m_state(meaning_of_boundary_c(q))
```

up to allowed consensus nondeterminism such as block ordering.

## Component 6: Parameter Context

`ParameterContext_c` pins the parameter and domain assumptions under which the
account meaning is valid.

It must include:

- modulus/field/ring parameter identity;
- matrix/domain derivation identity if used;
- verifier version;
- hash/QROM domain tags when semantically relevant;
- proof-system family only if validity relies on a proof-system theorem;
- native boundary version.

Required projection:

```text
m_context : M_c -> ParameterContext_c
```

Context law:

```text
same boundary meaning => same parameter interpretation
```

No candidate may reuse public bytes across parameter contexts unless the domain
binding explicitly says they have the same semantics.

## Abstract Projection From Private Representatives

Every candidate must provide:

```text
meaning_c : H_c -> M_c
```

where `H_c` is the candidate's private representative space.

For the holistic construction, `H_c` should be factored as:

```text
H_c = MeaningLift_c × Gauge_c × OpeningLocal_c × Randomizer_c
```

and:

```text
meaning_c(h)
```

must depend only on `MeaningLift_c` or on boundary-derived semantic data, not
on safety-irrelevant gauge/randomizer coordinates.

## Semantic Safety Equivalence

First-pass definition:

```text
h ~safe h' iff meaning_c(h)=meaning_c(h')
```

This may be strengthened if a candidate reveals additional public safety
behavior outside `M_c`. It may not be weakened unless the omitted behavior is
proved irrelevant to spend, replay, transcript binding, and state transition.

Boundary safety requirement:

```text
boundary_c(h)=boundary_c(h') => h ~safe h'
```

Boundary meaning requirement:

```text
exists meaning_of_boundary_c : Q_c -> M_c.
forall h, meaning_c(h)=meaning_of_boundary_c(boundary_c(h)).
```

This is the cleanest way to state that a public boundary classifies account
meaning.

## Non-Meaning Fields

These should not be part of `M_c` unless a candidate proves they affect public
semantics:

- pad source bytes;
- projection masks;
- opening tuple private coordinates;
- randomizers/blinding/noise;
- proof query openings;
- proof transcript randomness;
- role subcommitment internal openings;
- wallet-local recovery metadata;
- byte digests used only for data availability.

If any of these fields must enter `M_c`, the candidate must explain why they
are semantic rather than gauge/private implementation data.

## Candidate Gate Impact

Every candidate must now answer:

```text
Q_c -> M_c:
  how does the public boundary determine semantic meaning?

H_c -> M_c:
  which private coordinates actually affect semantic meaning?

Fiber(boundary_c,q):
  which coordinates vary inside the fiber, and are they all non-semantic?
```

Fail a candidate if:

- `Q_c` does not determine `M_c`;
- a representative-specific private coordinate changes `M_c` inside one fiber;
- a spend/transcript/lifecycle field does not factor through `M_c`;
- account creation admits `q` without fixing lifecycle/nullifier semantics;
- checked-core or native spend challenges bind to bytes outside the semantic
  surface.

## Immediate Consequences

1. The native design search can now evaluate candidates before byte layout.
2. Pullback candidates should treat `M_c` as the shared semantic target.
3. Obstruction candidates must prove `obs_c(q)=0` produces an account-backed
   lift with a well-defined `M_c`.
4. Gauge candidates must prove the gauge action preserves `M_c`.
5. The masked-trace fallback remains a fallback proof strategy, not the
   definition of account meaning.

