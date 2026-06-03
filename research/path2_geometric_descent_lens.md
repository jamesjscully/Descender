# Path 2 Geometric Descent Lens

Status: conceptual lens for construction, not a new assumption set.

Purpose: use geometric language only where it sharpens the design search. The
goal is to identify the right concrete account object, not to rename ordinary
cryptographic checks.

## Why Geometry May Help

The current Path 2 problem has the shape of a moduli/descent problem:

```text
public account object = global descended object / quotient class
private signer state  = local representative / trivialization / gauge choice
validity              = vanishing obstruction to gluing
privacy               = nontrivial fiber over the public object
soundness             = all points in the public fiber have same safe meaning
```

This is exactly where geometric language can be useful. It tells us which
objects to construct and which invariants must be checked before choosing a
proof system.

## Dictionary

### Moduli Object

Use for:

```text
Q_c = public account boundary quotient
```

Interpretation:

`Q_c` is the moduli space of protocol-visible account meanings. A point
`q : Q_c` is an admitted account boundary.

Design test:

```text
Does q classify exactly one spend/replay/lifecycle meaning?
```

If not, the quotient is too coarse.

### Local Trivialization

Use for:

```text
h : H_c
```

Interpretation:

The hidden signer state is a local presentation of the public descended object,
not the account object itself.

Design test:

```text
Can two different local trivializations over q produce different public spend
behavior?
```

If yes, the private structure is not pure gauge and the public object is
underspecified.

### Gauge / Torsor

Use for:

```text
G_c acts on Fiber(boundary_c, q)
```

Interpretation:

Private randomness, masks, pads, local section choices, and representation
choices should form a gauge fiber over the public object.

Good shape:

```text
Fiber(boundary_c, q) is a G_c-torsor or a union of safety-equivalent torsors.
```

Design test:

```text
Does the gauge action preserve meaning_c, p_spend, p_lifecycle, and p_state?
```

If not, gauge has leaked into semantics.

### Sheaf / Presheaf

Use for:

```text
local role data, section restrictions, overlap equations
```

Interpretation:

Role-specific data should be restrictions of one structured object, not
independent witnesses later glued by a proof.

Design test:

```text
Are pad, projection, opening, lifecycle, and checked-core views restrictions
of one presheaf-like section?
```

If not, we are recreating the independent-role-object bottleneck.

### Cech Cocycle / Obstruction Class

Use for:

```text
obs_c : Q_c -> O_c
```

Interpretation:

Validity of the public boundary is the statement that the obstruction to
gluing local data vanishes.

Good target:

```text
obs_c(q)=0
  <=> q is the boundary of a globally valid account object
```

Design test:

```text
Can account creation verify obstruction vanishing publicly?
```

If yes, we avoid generic witness proof for global consistency.

### Stack / Groupoid Quotient

Use carefully for:

```text
H_c / gauge equivalence
```

Interpretation:

If representatives have automorphisms or multiple equivalent presentations,
the account object is better viewed as a groupoid quotient than a set quotient.

Design test:

```text
Do automorphisms affect consensus-visible behavior?
```

If yes, the quotient is unsafe. If no, stack/groupoid language may prevent us
from accidentally choosing an injective encoding that destroys privacy.

### Fiber Product / Pullback

Use for:

```text
combining account boundary, lifecycle boundary, spend boundary, transcript frame
```

Interpretation:

The public account object can be constructed as a pullback of smaller semantic
boundaries over shared context.

Potential shape:

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

Design test:

```text
Are shared fields identified by construction rather than equality-proven later?
```

This is a principled way to avoid independent commitments.

## Geometric Candidate Families

### 1. Obstruction-Vanishing Account

Public account:

```text
q : Q_c
```

Verifier:

```text
obs_c(q)=0
```

Private state:

```text
h in boundary_c^{-1}(q)
```

The core theorem is an effective descent theorem:

```text
obs_c(q)=0 => exists h. boundary_c(h)=q ∧ LocalValid_c(h)
```

This is the cleanest geometric target.

### 2. Torsor Account

Public account:

```text
q : Q_c
```

Private state:

```text
h = g . canonical_lift(q)
```

where `g : G_c` is private gauge.

The core theorem is gauge invariance:

```text
meaning_c(g.h)=meaning_c(h)
```

and gauge hiding:

```text
q does not determine g.
```

This is attractive if signer-local material can be made pure gauge.

### 3. Pullback Boundary Account

Public account:

```text
q = (q_account, q_lifecycle, q_spend, q_transcript)
```

with all pieces living over the same context object.

Validity is membership in a pullback:

```text
context(q_account)=context(q_lifecycle)
context(q_account)=context(q_spend)
context(q_account)=context(q_transcript)
```

The key win is that consistency is categorical/product structure, not a later
proof tying unrelated commitments together.

### 4. Sheaf-Section Account

Public account:

```text
q = public restriction of a global section
```

Private state:

```text
h = local section data + private trivialization
```

Validity is the sheaf gluing condition:

```text
compatible local restrictions glue to a global section
```

This is closest to the original descent language.

## How This Guides Concrete Search

The geometry suggests the next questions should be:

1. What is the base/context object over which all account components live?
2. Are current role fields restrictions of one section, or unrelated objects?
3. What is the obstruction group/module `O_c`?
4. Is `obs_c(q)=0` public and compact?
5. Does vanishing obstruction imply existence of an account-backed local lift?
6. Is the fiber over `q` a gauge orbit/torsor?
7. Does the gauge action preserve all safety projections?
8. Can the public boundary be built as a pullback so consistency is native?

## Use And Non-Use

Use geometric language for:

- designing `Q_c`, `H_c`, `G_c`, and `obs_c`;
- identifying whether private data is true gauge;
- replacing equality proofs between independent objects with pullback structure;
- finding native obstruction checks;
- preventing unsafe quotient choices.

Do not use geometric language for:

- ordinary hash/domain binding;
- byte encoding;
- constant-time engineering;
- generic proof-system claims;
- hiding an unresolved witness problem behind terminology.

## Immediate Recommendation

Try to instantiate three candidate accounts using the same template:

```text
obstruction-vanishing account
torsor/gauge account
pullback boundary account
```

For each one, fill:

```text
base/context object
public quotient Q_c
private representative H_c
gauge G_c
obstruction/check object O_c
obs_c
proof of effective descent
proof of safety saturation
spend closure
```

The best concrete setup should be the one where obstruction vanishing,
safety saturation, and spend closure become theorems about the structure rather
than large external proof obligations.

