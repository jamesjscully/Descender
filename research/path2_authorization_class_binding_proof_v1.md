# Path 2 Authorization Class Binding Proof V1

Status: first proof attempt for the highest-risk remaining hidden-auth semantic
ambiguity.

Purpose: prove or precisely localize failure of:

```text
auth_compatible_c(q,auth)
  => AuthClass_c(auth)=p_spend(q)
```

under the active honest-create construction:

```text
authorizationLocal -> AuthClass_c(authorizationLocal) -> p_spend(q).
```

## Why This Is The Current Blocker

The lift split removed the need for consensus to prove:

```text
obs_c(q)=0 => exists h ∈ H_c(q)
```

But it did not remove the need to prove that signer-held private authorization
material cannot change public spend semantics after `q` is admitted.

The dangerous failure is:

```text
exists q auth auth'.
  auth_compatible_c(q,auth)
  ∧ auth_compatible_c(q,auth')
  ∧ AuthClass_c(auth) != AuthClass_c(auth')
```

If this is possible, one public account boundary admits multiple spend
meanings. Then authorization-local material is not purely local; part of it must
move into the public boundary.

## Definitions Needed

### Public Spend Projection

`p_spend(q)` must include the full public authorization class:

```text
p_spend(q) =
  (
    verifier_family,
    public_authorization_identity,
    spend_domain,
    checked_core_handle,
    allowed_state_transition_class,
    aggregation_mode
  )
```

It may use compact encodings or digests, but semantically it must identify the
whole class.

### Authorization Class

`AuthClass_c(auth)` is the public spend class realized by private
authorization-local material:

```text
AuthClass_c(auth) =
  (
    verifier_family(auth),
    public_authorization_identity(auth),
    spend_domain(auth),
    checked_core_handle(auth),
    allowed_state_transition_class(auth),
    aggregation_mode(auth)
  )
```

Private representation choices, pads, randomizers, witnesses, and local
trivializations are excluded.

### Compatibility Predicate

The proof only works if compatibility is defined as equality to the public spend
projection:

```text
auth_compatible_c(q,auth) :=
  AuthClass_c(auth)=p_spend(q)
  ∧ auth_local_wellformed_c(q,auth)
  ∧ auth_private_bounds_c(auth)
```

or as a predicate that entails that equality.

If `auth_compatible_c` is weaker, the theorem is an assumption rather than a
definitionally enforced property.

## Proof Strategy

Use construction-native binding:

1. Honest creation samples private authorization-local material:

   ```text
   auth <- AuthorizationSampler(seed)
   ```

2. It computes:

   ```text
   cls = AuthClass_c(auth)
   ```

3. It sets:

   ```text
   p_spend(q)=cls
   ```

4. It defines `auth_compatible_c(q,auth)` to require either:

   ```text
   AuthClass_c(auth)=p_spend(q)
   ```

   or a canonical commitment/opening relation whose public output is exactly
   `p_spend(q)`.

Then the theorem is immediate:

```text
auth_compatible_c(q,auth)
  => AuthClass_c(auth)=p_spend(q)
```

This is not circular; it is the intended boundary of what "compatible" means.
The nontrivial proof is that `AuthClass_c` contains every public semantic field
that spend verification can observe.

## Field Completeness Check

For the theorem to be meaningful, every consensus-visible spend semantic must
factor through `AuthClass_c` and `p_spend(q)`.

Required fields:

| field | must be in `AuthClass_c` | must be in `p_spend(q)` | reason |
| --- | --- | --- | --- |
| verifier family | yes | yes | selects verification relation |
| public authorization identity | yes | yes | binds account to signing authority |
| spend domain | yes | yes | prevents cross-domain replay |
| checked-core handle/frame id | yes | yes | binds downstream proof/check relation |
| state transition class | yes | yes | prevents authorization over wrong state object |
| lifecycle/nullifier scope if spend-visible | yes or in `p_lifecycle(q)` with edge | yes or via edge | replay safety |
| aggregation mode if semantics-affecting | yes | yes | batch verifier compatibility |

If any public spend behavior depends on a field outside this table, either add
it to the authorization class or prove it is a pure byte/data-availability
binding with no semantic effect.

## Interaction With `O_gluing`

`O_gluing` must contain an account-spend edge:

```text
E_AS
```

that checks:

```text
p_account(q).authorization_identity
  =
p_spend(q).public_authorization_identity
```

and a spend-state edge:

```text
E_SD
```

that checks:

```text
p_spend(q).allowed_state_transition_class
  =
p_state(q).state_transition_class
```

Authorization class binding alone proves compatible private auth realizes
`p_spend(q)`. `O_gluing` then proves that `p_spend(q)` is the spend face of the
same public account boundary.

## Theorem Shape

### Local Binding

```text
auth_class_binding_local_v1:
  auth_compatible_c(q,auth)
    =>
  AuthClass_c(auth)=p_spend(q)
```

This is the Lean-level `AuthorizationClassBinding`.

### Boundary Binding

```text
auth_class_boundary_binding_v1:
  obs_c(q)=0
  ∧ auth_compatible_c(q,auth)
    =>
  AuthClass_c(auth)=meaning_of_boundary_c(q).spendAuthorizationClass
```

This uses projection completeness:

```text
meaning_of_boundary_c(q).spendAuthorizationClass = p_spend(q)
```

### Spend Closure Consequence

```text
produced_spend_semantics_v1:
  obs_c(q)=0
  ∧ auth_compatible_c(q,auth)
  ∧ SpendProduced(c,q,auth,msg,state,sig)
  ∧ SpendAccept(c,q,sig,msg,state)
    =>
  accepted spend authorization = p_spend(q)
```

This is where binding becomes security-relevant.

## What Must Move Into `q`

If the theorem fails, the failing coordinate must be public.

Move into `p_spend(q)`:

- any verifier-family selector used by spend verification;
- any public key / account authorization identity;
- any checked-core relation handle;
- any domain separator affecting challenge or replay;
- any state transition class authorized by the spend;
- any aggregation mode that changes verification semantics.

Keep private:

- representative coordinates;
- signing witness;
- local opening randomness;
- gauge/trivialization;
- one-time masks;
- simulator randomness;
- trapdoor material whose public class is already fixed by `p_spend(q)`.

## Current Assessment

The theorem is likely achievable if we make `auth_compatible_c` a typed
predicate over `p_spend(q)`, not a loose "can produce some signature" relation.

The critical design rule is:

```text
compatibility is relative to a public class, not merely relative to q bytes.
```

With that rule, authorization class binding becomes definitional or a small
projection-completeness proof.

Without that rule, we are hoping that hidden signer material has no public
semantic variability, which is exactly the sketchy assumption this path was
trying to avoid.

## Required Lean Refinement

The current Lean interface has:

```text
authorizationCompatible : Boundary -> PrivateFiber -> Prop
authClass : PrivateFiber -> Nat
pSpend : Boundary -> Nat
```

and:

```text
AuthorizationClassMatches P Proj q h :=
  P.authClass h = Proj.pSpend q
```

The interface now documents that `authorizationCompatible` must entail
`AuthorizationClassMatches`.

Lean addition:

```text
def AuthorizationCompatibleOverClass
    (P : PrivateFiberPredicates)
    (Proj : PublicProjections)
    (q : Boundary) (h : PrivateFiber) : Prop :=
  P.authorizationCompatible q h ∧
  AuthorizationClassMatches P Proj q h
```

Then honest creation can return this stronger predicate directly, while
`AuthorizationClassBinding` remains the global no-ambiguity theorem.
`Descender/Path2FullSpendConcrete.lean` now instantiates this shape for the
selected Rust theorem-support slice through
`selectedRustAuthorizationCompatibleOverClass_of_compatible`,
`selectedRustAuthorizationClassBinding`,
`selectedRustHonestCreate_authorization_over_class`, and the selected
projection-completeness hook
`selectedRustMeaningProjection_spendAuthorizationClass_eq_pSpend`.

## Pass Criteria

Pass if:

- `p_spend(q)` is complete for all public spend semantics;
- `auth_compatible_c(q,auth)` entails equality with `p_spend(q)`;
- `O_gluing` links `p_spend(q)` to account, lifecycle, transcript, and state
  projections;
- spend verification consumes no hidden semantic handle outside `q`;
- all auth variation outside `AuthClass_c` is private gauge/randomizer/witness
  data.

## Fail Criteria

Fail or redesign if:

- spend verification reads a public semantic field not in `p_spend(q)`;
- two compatible auth values over one `q` can induce different verifier
  families or state transition classes;
- checked-core binds to an independent authorization handle not projected from
  `q`;
- aggregation mode changes acceptance but is not part of the public class;
- `auth_compatible_c` means only "can sign somehow" rather than "realizes this
  public class."

## Next Concrete Move

Consume the row-pinned sampler integration checklist in the full honest-create
sampler evidence path, then carry that into the external review package. The
proof chain is now explicit in the Lean interface, selected-Rust support hooks,
and `PATH2_ACCOUNT_CREATION_SAMPLER_INTEGRATION_ROWS`:

```text
HonestCreateSoundness
  => LocalValid
  => authorizationCompatible
  => AuthorizationClassMatches
  => AuthClass = pSpend(q)
```
