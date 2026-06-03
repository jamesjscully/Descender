# Path 2 Native Descent Alternatives To Generic ZK

Status: conceptual design fork.

Purpose: define alternatives to repeatedly wrapping the Path 2 admission
statement in generic zero-knowledge machinery. The goal is not to remove every
privacy argument. The goal is to make privacy and consistency consequences of
the native algebraic/descent structure whenever possible.

## Core Problem In Native Terms

Let:

```text
H_c        = hidden descent representatives in context c
~safe      = safety equivalence on H_c
B_c        : H_c -> Boundary_c
Local_c    subset H_c
Glue_c     subset H_c
```

Account creation should admit a public boundary `b` only when:

```text
exists h in H_c.
  h in Local_c
  h in Glue_c
  B_c(h) = b
```

and boundary equality must be safety-saturated:

```text
B_c(h) = B_c(h') => h ~safe h'
```

The generic ZK route treats `h` as a private witness and then proves this
statement externally. The native route should instead choose `H_c`, `B_c`,
`Local_c`, and `Glue_c` so the admitted public boundary is already a compact
descent certificate.

## The Real Constraint

The central obstruction is not "prove a witness without revealing it." It is:

```text
construct a public quotient Boundary_c whose fibers are large enough to hide
private representatives, but small enough that every representative in an
accepted fiber has the same spend/replay/lifecycle meaning.
```

Equivalently:

```text
privacy wants large fibers
soundness wants safety-saturated fibers
verification wants compact image membership
```

If `B_c` is too injective, it exports `h`. If `B_c` is too coarse, it admits
different spend meanings in one fiber. If image membership in `B_c(Local_c ∩
Glue_c)` is hard, we fall back into generic ZK.

## Native Route A: Constructive Quotient Boundary

Define:

```text
Boundary_c := (Local_c ∩ Glue_c) / ~safe
B_c(h)     := [h]_safe
```

Then implement `b` as a canonical encoding of the quotient class, not as
independent commitments to hidden roles.

This is the cleanest mathematical answer, but the practical blocker is finding
a compact canonical representative or verifier for the quotient class. If the
quotient encoding is merely a hash of hidden data, we are back to proving a
hidden opening.

Viability test:

```text
Can the quotient class be represented by public residues/syndromes/section
roots whose well-formedness is checked directly?
```

## Native Route B: Image-Coded Boundary

Choose `B_c` so the valid image has a small public code:

```text
b in Im(B_c | Local_c ∩ Glue_c)
```

checked by native equations:

```text
S_c(b) = 0
rank/gluing conditions hold
boundary cycle/coboundary equations hold
range/canonicality lives in the boundary code
```

This is attractive because the verifier never asks for `h`; it only checks that
`b` belongs to the public image code. Privacy comes from the kernel/fiber of
`B_c`, not from an external witness-hiding protocol.

The hard condition is completeness in the other direction:

```text
S_c(b) = 0 => exists h. Local_c(h) ∧ Glue_c(h) ∧ B_c(h)=b
```

or a named descent/coding break. This is the theorem to search for before doing
more proof-system engineering.

## Native Route C: Descent Section With Public Obstruction Class

Model account creation as a gluing problem:

```text
local sections s_i
restriction overlaps agree up to gauge
public obstruction class vanishes
```

The admitted object is not a commitment to every section. It is a compact
obstruction/boundary class:

```text
ob_c(b) = 0
```

plus the public safety projection. This is closest to descent language: the
verifier checks that the obstruction to gluing vanishes.

Potential win:

- no hidden trace extraction;
- public check is a native gluing/cocycle condition;
- aggregation may compose by adding obstruction classes.

Main risk:

- the vanishing obstruction may prove only that *some* object glues, not that
  it glues to the account-backed opening needed by spend.

So the obstruction class must be account-bound:

```text
ob_c(b)=0 and AccountBoundary_c(b)=expected account namespace
```

## Native Route D: Fiber-Saturated Spend Semantics

Relax account creation so it does not identify a unique hidden representative.
Instead require:

```text
forall h h'. B_c(h)=B_c(h') -> SpendMeaning_c(h)=SpendMeaning_c(h')
```

Then account creation only proves the existence of an accepted fiber, and spend
operates purely on the fiber-saturated boundary meaning.

This is not a proof trick. It changes the model boundary:

```text
the account is the fiber, not a representative inside the fiber
```

This can be very native if the spend relation already depends only on quotient
data. It fails if the spend witness needs a representative-specific secret that
is not recoverable or authorizable from the boundary class.

## Native Route E: Public Canonical Lift With Private Gauge

Choose a canonical public lift:

```text
canon_c(b) in H_c / Gauge_c
```

and move all privacy into a gauge action:

```text
h = g . canon_c(b)
```

where `g` is private but provably irrelevant to spend/replay/lifecycle
semantics.

Then verification checks `canon_c(b)` directly, while the private signer keeps
the gauge needed for spend-time authorization.

This is viable only if the gauge is:

- large enough to hide private structure;
- safety-irrelevant;
- not needed by public verification;
- compatible with one-time/lifecycle rules.

This is the most creative route, and it may match the project thesis well: the
public account object is a canonical descended object, while private data is a
choice of local trivialization/gauge.

## What These Routes Avoid

They avoid this recurring pattern:

```text
publish opaque commitment C
prove in ZK that C opens to h
prove h has all role restrictions
prove restrictions are mutually consistent
prove public boundary equals projections of h
prove no queried piece leaks h
```

That pattern is coherent but proof-system-heavy. The native alternatives try to
make at least the consistency and image-membership parts true by construction.

## What They Cannot Avoid

No route can avoid all security assumptions. Each native route still needs one
of:

- an image-membership theorem for the public boundary code;
- an effective descent theorem for vanishing obstruction classes;
- a SIS/module-SIS binding theorem for algebraic boundary commitments;
- a gauge-hiding theorem showing private trivializations are not exported;
- a break-export theorem for malformed public boundaries.

The improvement is that these are structural theorems over the project's own
objects, not generic ZK claims over an arbitrary witness circuit.

## Current Recommendation

Investigate these in order:

1. **Image-coded boundary**: find `S_c` such that `S_c(b)=0` characterizes the
   valid public image.
2. **Public obstruction class**: express account creation as vanishing of a
   descent/gluing obstruction plus account namespace binding.
3. **Public canonical lift with private gauge**: test whether private material
   can be moved into a safety-irrelevant gauge action.
4. **Constructive quotient boundary**: keep as the mathematical ideal, but do
   not assume compact canonical encoding exists.
5. **Fiber-saturated spend semantics**: use only if spend can be cleanly
   rephrased over boundary classes.

The next concrete proof question is:

```text
Does there exist a public verifier map S_c on candidate boundaries such that:

S_c(b)=0
  <=> exists h. Local_c(h) ∧ Glue_c(h) ∧ B_c(h)=b

and ker(B_c) subset ~safe?
```

If yes, account creation becomes a native descent-boundary verification problem
instead of a generic private-witness proof.

