# Path 2 Honest Create Sampler V1

Status: first wallet/keygen spendability theorem target.

Purpose: specify the signer-side construction that replaces the overly strong
consensus theorem:

```text
obs_c(q)=0 => H_c(q) nonempty
```

with an honest creation theorem:

```text
HonestCreate_c(seed)=Some(q,h)
  =>
obs_c(q)=0
∧ h ∈ H_c(q)
∧ LocalValid_c(q,h)
∧ AuthClass_c(h.auth)=p_spend(q).
```

## Role In The Active Path

Consensus admission only needs:

```text
AcceptCreate(c,q)
  =>
obs_c(q)=0
∧ public_boundary_sound_c(q)
```

Honest wallet/keygen creation must additionally ensure:

```text
exists h ∈ H_c(q). LocalValid_c(q,h)
```

This document specifies the latter. It is signer-side correctness and
spendability, not consensus extraction.

## Sampler Interface

Define:

```text
HonestCreate_c(seed, context, account_params)
  =
Some(q,h)
```

where:

```text
q : Q_c
h : H_c(q)
```

and:

```text
h =
  (
    liftCore,
    authorizationLocal,
    gauge,
    randomizer
  )
```

The sampler is allowed to use private randomness and local key material. It is
not required to make that material publicly extractable.

## Construction Order

The natural construction order is private-to-public:

```text
seed
  -> authorizationLocal
  -> spend authorization class
  -> lifecycle/account/state choices
  -> public boundary q
  -> canonical liftCore
  -> gauge/randomizer
  -> h
```

This is different from adversarial account admission, which starts with a public
`q`.

## Required Outputs

### Public Boundary

The sampler emits a public boundary:

```text
q =
  (
    contextFace,
    accountFace,
    spendFace,
    lifecycleFace,
    transcriptFace,
    stateFace,
    overlapFace,
    canonicalityFace
  )
```

such that:

```text
obs_c(q)=0
```

This follows by construction if the sampler populates every projection from one
semantic source:

```text
M_c =
  AccountNamespace_c
  × SpendAuthorizationClass_c
  × LifecycleIdentity_c
  × TranscriptBindingSurface_c
  × StateTransitionBoundary_c
  × ParameterContext_c
```

and then forms the faces as restrictions of that source.

### Authorization Local

The sampler creates:

```text
authorizationLocal
```

and derives:

```text
AuthClass_c(authorizationLocal)
```

before fixing:

```text
p_spend(q)
```

Required law:

```text
p_spend(q)=AuthClass_c(authorizationLocal)
```

This direction is crucial. We should not sample `q` first and hope compatible
authorization material exists. We sample authorization-local material first and
make `q` expose exactly its public spend class.

### Lift Core

The sampler defines:

```text
liftCore = canonical_lift_core_c(q)
```

or another deterministic local section:

```text
section_c(q) : LiftCore_c(q)
```

Required law:

```text
lift_valid_c(q,liftCore)
```

This is expected to follow from public boundary soundness and canonicality.

### Gauge

The sampler selects:

```text
gauge = identity_gauge_c(q)
```

or a randomized gauge in the same semantic class.

Required laws:

```text
gauge_valid_c(q,gauge)
boundary_c(gauge.h)=q
meaning_c(gauge.h)=meaning_of_boundary_c(q)
```

Nontrivial gauge randomness is a privacy optimization, not an existence
requirement.

### Randomizer

The sampler selects:

```text
randomizer <- RandomizerSampler_c(q, seed)
```

Required laws:

```text
randomizer_valid_c(q,randomizer)
randomizer changes no public semantic projection
```

Privacy/no-export will need a separate distributional theorem. For
spendability, validity and retained state are enough.

## Main Theorem

Target:

```text
honest_create_sampler_sound_v1:
  HonestCreate_c(seed, context, account_params)=Some(q,h)
    =>
  obs_c(q)=0
  ∧ LocalValid_c(q,h)
  ∧ AuthClass_c(h.authorizationLocal)=p_spend(q)
```

Expanded:

```text
obs_c(q)=0
∧ lift_valid_c(q,h.liftCore)
∧ auth_compatible_c(q,h.authorizationLocal)
∧ gauge_valid_c(q,h.gauge)
∧ randomizer_valid_c(q,h.randomizer)
∧ AuthClass_c(h.authorizationLocal)=p_spend(q)
```

## No Consensus Leakage

The public account creation transaction contains:

```text
q
```

and not:

```text
h.authorizationLocal
h.gauge
h.randomizer
```

Consensus checks:

```text
obs_c(q)=0
```

Wallet software retains:

```text
h
```

The proof obligation is that the retained `h` is sufficient to produce later
spends whose verifier frame is closed over `q`.

## Security Boundary

If an adversary submits a coherent `q` without knowing any compatible
authorization local, the account is unspendable. That is not a forgery.

It becomes a protocol problem only if:

- unspendable accounts are underpriced;
- state cannot be reclaimed or rent-charged;
- creation admits expensive downstream verification burden;
- unspendable accounts block other users or namespaces.

Those are state-policy constraints, not the native signature proof itself.

## Hidden Advantages

This construction order has three advantages.

1. Authorization class binding becomes construction-native:

   ```text
   p_spend(q)
   ```

   is derived from `authorizationLocal`, not guessed independently.

2. Public gluing stays public:

   `O_gluing` checks only faces of `q`, while wallet/keygen privately remembers
   why the spend face is usable.

3. Aggregation assumptions stay compatible:

   aggregated verification can batch public `q` projections without needing to
   aggregate creation-time hidden witness proofs.

## Remaining Risks

The critical remaining risk is not nonemptiness in the abstract. It is
authorization class ambiguity:

```text
auth_compatible_c(q,auth)
∧ auth_compatible_c(q,auth')
∧ AuthClass_c(auth) != AuthClass_c(auth')
```

This must be impossible or exported as:

```text
authorization_class_binding_break
```

The second risk is signer state loss. If the wallet loses `h`, the account may
become unspendable even though public `q` remains coherent. That is lifecycle
and key-management engineering, not a consensus proof failure.

## Dead Conditions

Reject this sampler design if:

- it emits `q` before deriving the authorization class;
- `p_spend(q)` is not the public class of the retained authorization local;
- `liftCore` requires hidden data not retained in `h`;
- `gauge` changes public semantic meaning;
- `randomizer` is reused in a way that affects spend privacy or replay safety;
- spend production needs a semantic handle not projected from `q`.

## Next Proof Move

Mechanize the full honest-create sampler theorem over the sampler integration
checklist now pinned by `Path2AccountCreationSamplerTimingReportV1`, then finish
the authorization class binding theorem:

```text
auth_compatible_c(q,auth)
  => AuthClass_c(auth)=p_spend(q)
```

for the construction order above. The current sampler/timing manifest now pins
the selected-Rust support hooks for the typed compatibility/projection bridge
and a five-row integration checklist covering the private-to-public order,
authorization bridge hooks, authorization-class audit import, accepted-boundary
import, and closed no-postchallenge timing slice. The accepted-boundary row now
requires
`selectedRustAcceptedBoundaryHonestSamplerSound_of_boundaryAccepted`,
which consumes the imported accepted-boundary/timing facts together with the
selected-Rust honest-create authorization bridge, pinned to the accepted spend's
own boundary and concrete leaf authorization fiber, while deriving the
`honestCreate` witness from accepted-boundary facts. It still keeps
`full_honest_sampler_integration_complete` and the honest sampler theorem flag
false until the full evidence path and review are complete.

This is now the highest-risk proof item. If it fails, private authorization
material still changes public spend semantics, and that semantic class must
move into `q` more explicitly.
