# Path 2 High-Level Descent Design Space

Status: abstraction scaffold for principled construction.

Purpose: lift the Path 2 design problem above concrete field layouts so
candidate account-creation systems can be compared by their mathematical
obligations rather than by ad hoc byte choices.

## Goal

Construct a protocol account object from a small number of abstract ingredients:

```text
semantic object
local representatives
gauge/fiber action
boundary quotient
obstruction map
protocol projections
```

A concrete design is acceptable only if these ingredients instantiate the same
commutative picture and satisfy the same safety, privacy, and existence
obligations.

The complementary note
`research/path2_geometric_descent_lens.md` gives a geometric vocabulary for
the same scaffold: moduli object for `Q_c`, local trivializations for `H_c`,
gauge/torsor fibers for private randomness, obstruction classes for `obs_c`,
and pullbacks for native consistency between account, lifecycle, spend, and
transcript projections.

## Abstract Ingredients

For each account context `c`, define:

```text
M_c  semantic account meaning
L_c  local/lifted representatives
G_c  private gauge/randomizer space
H_c  private signer state / total representative space
Q_c  public boundary quotient
O_c  public obstruction/check space
```

with maps:

```text
meaning_c : H_c -> M_c
lift_c    : H_c -> L_c
boundary_c: H_c -> Q_c
obs_c     : Q_c -> O_c
```

and protocol projections:

```text
p_leaf        : Q_c -> LeafFields
p_spend       : Q_c -> SpendAuthorization
p_lifecycle   : Q_c -> LifecycleIdentity
p_transcript  : Q_c -> ChallengeBindingFrame
p_state       : Q_c -> StateTransitionBoundary
```

The public account object is `q : Q_c`.

## Universal Diagram

Every viable concrete design should instantiate this diagram:

```text
        H_c  ----------------->  M_c
         |                         ^
         | boundary_c              | meaning induced by q
         v                         |
        Q_c  ---- obs_c ----->    O_c
         |
         +--> p_leaf(q)
         +--> p_spend(q)
         +--> p_lifecycle(q)
         +--> p_transcript(q)
         +--> p_state(q)
```

with account creation checking:

```text
obs_c(q) = 0
```

or a small native certificate for `obs_c(q)=0`.

## Required Laws

### 1. Existence / Effective Descent

Public validity must imply a local representative exists:

```text
obs_c(q)=0
  => exists h in H_c. boundary_c(h)=q ∧ LocalValid_c(h)
```

This is the effective descent theorem. It replaces a generic hidden-witness
proof when possible.

### 2. Safety Saturation

All representatives over the same public boundary must have the same protocol
meaning:

```text
boundary_c(h)=boundary_c(h')
  => meaning_c(h)=meaning_c(h')
```

or at least:

```text
boundary_c(h)=boundary_c(h') => h ~safe h'
```

This is the core anti-replay/anti-malleability requirement.

### 3. Projection Completeness

Every public field used by consensus must factor through `Q_c`:

```text
field_i = p_i(q)
```

No consensus-facing field may secretly depend on a representative-specific
choice in `H_c`.

### 4. Gauge Privacy

The fiber over `q` must contain enough private variation to hide signer-local
choices:

```text
Fiber(boundary_c, q)
```

and that variation must lie inside the kernel of all safety projections:

```text
gauge variation changes H_c but not M_c, p_spend, p_lifecycle, or p_state.
```

### 5. Spend Closure

Spend must be definable over the public quotient plus signer-held local data:

```text
Spend_c(q, local_auth, msg, state)
```

and public acceptance must depend only on `q` and protocol state, not on which
representative in the fiber originally created the account.

### 6. Break Localization

Every failed law must export a named structural break:

```text
effective_descent_break
safety_saturation_break
projection_factorization_break
gauge_privacy_break
spend_closure_break
domain_binding_break
```

This keeps the theorem surface precise even before concrete assumptions are
chosen.

## Candidate Families

### A. Image-Code Family

```text
Q_c = public codeword space
obs_c(q) = syndrome/check map
```

Validity:

```text
obs_c(q)=0
```

means `q` lies in the valid descended image.

Best if the account boundary can be a codeword whose image has compact
syndrome/gluing checks.

### B. Obstruction-Class Family

```text
Q_c = public descent boundary data
O_c = obstruction group/module
obs_c = gluing obstruction
```

Validity is obstruction vanishing.

Best if account creation is naturally a local-section gluing problem.

### C. Canonical-Lift/Gauge Family

```text
Q_c = canonical descended object
H_c = canonical_lift(q) acted on by private G_c
```

Validity is mostly public canonicality; privacy comes from the private gauge.

Best if the private structure can be isolated as safety-irrelevant
trivialization data.

### D. Commitment-Quotient Family

```text
Q_c = algebraic commitment quotient
obs_c = native opening/consistency relation
```

This is closest to descent/SIS. It is viable only if commitment hiding and
binding are native and do not recreate independent role-object proofs.

### E. Hybrid Family

Use an image-code or obstruction-class public boundary for semantic validity,
and a small commitment quotient only for signer authorization material.

This may be the most practical route if pure public image membership fixes
account semantics but does not authenticate private spend authority by itself.

### F. Pullback Boundary Family

Construct `Q_c` as a pullback of smaller semantic boundaries over one shared
context object:

```text
Q_c =
  Q_account
    ×_{Q_context}
  Q_lifecycle
    ×_{Q_context}
  Q_spend
    ×_{Q_context}
  Q_transcript
```

This makes cross-field consistency native. It is attractive if the current
account/lifecycle/spend/transcript fields are individually simple but expensive
to prove mutually consistent after the fact.

### G. Torsor/Gauge Family

Construct the private representative fiber as a gauge orbit:

```text
Fiber(boundary_c, q) ~= G_c
```

or as a safety-equivalent union of `G_c`-orbits. Validity is mostly public
canonicality of `q`; privacy comes from the private gauge choice. This is
attractive if pad/mask/opening-local material can be made safety-irrelevant by
construction.

## Principled Search Procedure

Evaluate concrete designs in this order:

1. Define `M_c`.
2. Choose a candidate `Q_c`.
3. Define every consensus projection `p_i : Q_c -> Field_i`.
4. Define `obs_c : Q_c -> O_c`.
5. Prove or falsify effective descent:

   ```text
   obs_c(q)=0 => exists h. boundary_c(h)=q ∧ LocalValid_c(h)
   ```

6. Prove or falsify safety saturation:

   ```text
   boundary_c(h)=boundary_c(h') => h ~safe h'
   ```

7. Check fiber privacy/gauge size.
8. Check spend closure over `q`.
9. Only then choose byte encodings, commitments, or proof systems.

## Concrete Evaluation Template

For each candidate, fill:

```text
name:
family:
M_c:
H_c:
Q_c:
G_c:
obs_c:
public projections:
effective descent theorem:
safety saturation theorem:
privacy source:
spend closure:
expected public bytes:
expected private state:
new assumptions:
failure mode:
disposition:
```

## Immediate Use

The next concrete design should not start with pad/projection/opening fields.
It should start by filling:

```text
M_c
Q_c
obs_c
p_spend
p_lifecycle
p_transcript
```

Only after that should we decide what `H_c` contains.

This reverses the previous direction. Instead of:

```text
hidden witness -> commitments -> proof -> public account
```

we use:

```text
semantic account meaning -> public quotient -> obstruction check ->
private local representatives
```
