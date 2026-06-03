# Path 2 Lift Nonemptiness V1

Status: first lift/nonemptiness decomposition after `O_gluing` instantiation.

Purpose: analyze the theorem:

```text
obs_c(q)=0 => H_c(q) nonempty
```

under the active split:

```text
public gluing first, private fiber construction second.
```

## Main Finding

The statement:

```text
obs_c(q)=0 => H_c(q) nonempty
```

is too strong as a consensus safety theorem unless `q` already includes enough
public material to certify that signer-held authorization material exists.

The better split is:

```text
obs_c(q)=0
  => q is a coherent public semantic boundary.
```

and:

```text
HonestCreate_c(seed)
  => obs_c(q)=0
  ∧ exists h ∈ H_c(q). LocalValid_c(q,h).
```

This matters because a maliciously submitted `q` with no private signing lift is
usually unspendable, not forgeable. That is a liveness/state-policy problem,
not automatically a signature-safety problem.

For blockchain deployment, the protocol still needs to decide whether
unspendable account creation is acceptable under fees/state rent, or whether
account creation must carry a spendability certificate. But the native
signature theorem should not silently require consensus extraction of hidden
signer material unless that is truly necessary.

## Fiber Decomposition

Current active fiber:

```text
H_c(q) =
  LiftCore_c(q)
  × AuthorizationLocal_c(q)
  × Gauge_c(q)
  × Randomizer_c(q)
```

Local validity:

```text
LocalValid_c(q,h) :=
  obs_c(q)=0
  ∧ lift_valid_c(q,h.lift)
  ∧ auth_compatible_c(q,h.auth)
  ∧ gauge_valid_c(q,h.gauge)
  ∧ randomizer_valid_c(q,h.randomizer)
```

So nonemptiness decomposes into four smaller obligations:

```text
LiftCore_c(q) nonempty
AuthorizationLocal_c(q) nonempty
Gauge_c(q) nonempty
Randomizer_c(q) nonempty
```

Only the second is truly dangerous.

## Component Assessment

### `LiftCore_c(q)`

Expected status: structural.

If `O_gluing`, `O_meaning`, and `O_canonicality` have done their job, then the
public boundary itself should define the canonical lift core.

Target:

```text
public_boundary_sound(q)
  => exists lift. lift_valid_c(q,lift)
```

This should be a native descent theorem, not a hidden-witness proof.

Fail condition:

```text
obs_c(q)=0
```

but no canonical or locally chosen lift core exists. If this happens, the public
boundary object is under-specified and more structure must move into `q`.

### `AuthorizationLocal_c(q)`

Expected status: signer-side/honest-creation obligation.

This is the component that may not follow from public `q` alone. It represents
private signing/opening material over the public spend class:

```text
p_spend(q)
```

The core deployable theorem should be:

```text
honest_authorization_sampler(seed,q)
  => exists auth.
       auth_compatible_c(q,auth)
       ∧ AuthClass_c(auth)=p_spend(q)
```

Consensus can safely admit a coherent public `q` without knowing `auth`, but a
wallet cannot honestly claim it has created a usable account unless it stores
such an `auth`.

Fail condition for security:

```text
exists q auth auth'.
  auth_compatible_c(q,auth)
  ∧ auth_compatible_c(q,auth')
  ∧ AuthClass_c(auth) != AuthClass_c(auth')
```

Fail condition for deployable account creation:

```text
HonestCreate_c(seed) emits q
```

but the signer cannot recover compatible `auth`.

### `Gauge_c(q)`

Expected status: structural.

At minimum, identity gauge should exist:

```text
exists gauge. gauge_valid_c(q,gauge)
```

Hiding-quality gauge choices may require randomness, but nonemptiness should not.

Fail condition:

```text
gauge_valid_c(q,gauge)
```

changes public spend, lifecycle, transcript, state, or meaning semantics. That
is a `gauge_invariance_break`.

### `Randomizer_c(q)`

Expected status: structural for existence, sampler-backed for privacy.

At minimum, the randomizer type should be inhabited:

```text
exists randomizer. randomizer_valid_c(q,randomizer)
```

However, privacy requires the distribution used by signing or account creation
to be simulator-compatible:

```text
randomizer_sampler_c(q; rho) indistinguishable from simulator randomness
```

Nonemptiness and hiding should remain separate theorem obligations.

Fail condition:

Public creation or spend verification depends on randomizer-specific semantics
instead of projections of `q`.

## Corrected Theorem Split

### Safety Admission

Consensus-facing safety theorem:

```text
safe_admission_v1:
  AcceptCreate(c,q)
    =>
  obs_c(q)=0
  ∧ public_boundary_sound_c(q)
```

or a named break:

```text
context_binding_break
pullback_consistency_break
gluing_obstruction_break
meaning_binding_break
spend_state_closure_break
canonicality_break
byte_data_availability_break
```

This theorem says accepted public account state has one coherent public
semantic meaning.

### Honest Spendability

Wallet/keygen-facing theorem:

```text
honest_create_spendability_v1:
  HonestCreate_c(seed)=Some(q,h)
    =>
  obs_c(q)=0
  ∧ h ∈ H_c(q)
  ∧ LocalValid_c(q,h)
  ∧ AuthClass_c(h.auth)=p_spend(q)
```

or a named break:

```text
lift_existence_break
authorization_local_sampling_break
authorization_class_binding_break
gauge_invariance_break
fiber_privacy_break
```

This theorem says the honest creator can actually spend the account it creates.

### Optional Spendability Admission

If the chain wants to reject unspendable account state at creation time, add a
public spendability certificate:

```text
SpendabilityCert_c(q)
```

with:

```text
VerifySpendabilityCert_c(q,cert)
  =>
exists auth.
  auth_compatible_c(q,auth)
```

This is optional for signature safety and potentially costly. It should not be
introduced unless state-policy analysis says fees/state rent are insufficient.

## Blockchain Consequence

There are two deployment modes.

### Mode A: Safety-Only Admission

Account creation verifies:

```text
obs_c(q)=0
```

The chain admits coherent public boundaries. If a user submits a boundary with
no private authorization lift, they have created unusable state.

This is acceptable if:

- account creation is paid;
- state rent or deposit covers storage;
- unspendable accounts do not create special consensus load;
- spend verification remains closed over projections of `q`.

This mode keeps account creation compact.

### Mode B: Spendability-Certified Admission

Account creation verifies:

```text
obs_c(q)=0
∧ VerifySpendabilityCert_c(q,cert)
```

This prevents knowingly unspendable public accounts but reintroduces a proof or
certificate at creation time.

This mode is necessary only if unspendable state is a protocol-level DoS or
economic issue that fees cannot price.

## Active Recommendation

Use Mode A as the active native signature-security path.

Reason:

```text
nonempty AuthorizationLocal_c(q)
```

is not needed for consensus safety if spend acceptance is closed over `q` and
authorization class binding holds for every compatible authorization local.

Require honest wallet/keygen to prove:

```text
HonestCreate_c(seed)=Some(q,h)
```

with retained local `h`, but do not require consensus account creation to
extract or verify `h`.

## Revised Proof Obligations

Highest-risk remaining obligations are now:

1. `public_boundary_sound_c(q)` from `obs_c(q)=0`.
2. `honest_create_spendability_v1` for wallet/keygen.
3. Authorization class binding:

   ```text
   auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)
   ```

4. Spend closure:

   ```text
   SpendAccept(c,q,sig,msg,state)
   ```

   has semantics determined only by projections of `q`.

5. Fiber privacy/no-export for authorization, gauge, and randomizer fields.

## Lean Interface Change Needed

Current Lean interface bundles lift existence into `EffectiveDescent`:

```text
ObstructionZero O q -> exists h, LocalValid O P q h
```

This should be split into:

```text
PublicBoundarySound O q
```

and:

```text
HonestCreateSound seed q h
```

with `EffectiveDescent` kept only as an optional stronger certificate.

Recommended next Lean change:

```text
structure PublicAdmissionSoundness ...
structure HonestCreationSoundness ...
```

Then keep the older `AccountAdmissionSoundness` as the spendability-certified
variant, not the default consensus theorem.

## Current Verdict

The lift nonemptiness blocker is not a fatal flaw in the native path. It exposes
a boundary mistake in the theorem statement.

The deployable Path 2 model should prove:

```text
accepted q is safe and coherent
```

at consensus creation time, and:

```text
honest keygen creates q together with spendable private h
```

at wallet/keygen time.

The next concrete proof move is to update the Lean theorem interface to reflect
this split and then prove the honest-creation sampler obligation separately.
