# Path 2 Holistic Descent Boundary Construction

Status: first-principles design frame.

Purpose: stop treating the hidden representative `h` as a fixed object that we
must somehow prove about. The private structure, public boundary, local
validity relation, and spend handoff should be constructed together so the
account object is native by design.

## Core Principle

Do not start with:

```text
given h, find a compact proof that h is valid
```

Start with:

```text
define the account as a descended public object whose private representatives
are local choices/trivializations of that object
```

Then derive:

```text
H_c        private local representatives / gauges
B_c        boundary/descent map
Boundary_c public descended object
S_c        public verifier equation / obstruction map
Spend_c    spend semantics over boundary fibers
```

as one system.

## First-Principles Requirement

For context `c`, construct a diagram:

```text
        H_c
      /  |  \
 local gauge private opening
      \  |  /
       B_c
        |
   Boundary_c
        |
      S_c
        |
       0
```

with these properties:

```text
S_c(B_c(h)) = 0
```

for every well-formed representative, and conversely:

```text
S_c(b)=0
  => exists h. LocalValid_c(h) ∧ DescentConsistent_c(h) ∧ B_c(h)=b
```

The kernel/fiber condition is equally important:

```text
B_c(h)=B_c(h') => h ~safe h'
```

or else the public boundary does not define one account.

## Design Degrees Of Freedom

All of these are open to redesign:

- the private representative space `H_c`;
- which coordinates are private gauge versus safety-relevant structure;
- the public boundary object `Boundary_c`;
- the boundary map `B_c`;
- the public verifier map `S_c`;
- the spend authorization boundary;
- lifecycle/nullifier semantics;
- role restrictions and packed residues;
- whether the signer stores a representative, a gauge, or a lift.

The only fixed requirement is the external behavior:

```text
admitted account boundary
spend authorization
replay/lifecycle uniqueness
no export of representative-specific secrets
post-quantum soundness assumptions
blockchain-verifiable public data
```

## Constructive Program

### 1. Define Safety Meaning First

Before choosing bytes or commitments, define the semantic object:

```text
Meaning_c
```

containing exactly the behavior that must be fixed by account creation:

- account namespace;
- spend authorization class;
- lifecycle/nullifier identity;
- checked-core challenge binding surface;
- allowed future state transition.

Then require:

```text
meaning_c : H_c -> Meaning_c
B_c(h)=B_c(h') => meaning_c(h)=meaning_c(h')
```

This prevents hidden representative choices from changing security behavior.

### 2. Split Representative Coordinates By Role

Design `H_c` as:

```text
H_c = MeaningLift_c × Gauge_c × OpeningLocal_c × Randomizer_c
```

where:

- `MeaningLift_c` carries the safety-relevant descended object;
- `Gauge_c` carries private choices that must be safety-irrelevant;
- `OpeningLocal_c` carries spend-time local data;
- `Randomizer_c` carries hiding/noise/mask material.

This replaces the current habit of accumulating operational fields and later
proving they are consistent.

### 3. Define Boundary As Meaning Plus Public Obstruction Data

The public boundary should look like:

```text
Boundary_c =
  public meaning projection
  + obstruction/check code
  + lifecycle/spend handle
```

not:

```text
commitment_to_pad
commitment_to_projection
commitment_to_opening
proof_that_they_match
```

Role commitments are only acceptable if they are projections of the same
boundary object or native restrictions of the same descended section.

### 4. Make Validity A Public Image Condition

Try to make account creation check:

```text
S_c(b)=0
```

where `S_c` is built from native algebra:

- cycle/coboundary equations;
- descent overlap/gluing equations;
- syndrome/rank equations;
- quotient normalization checks;
- lifecycle namespace equations.

If `S_c(b)=0` needs a witness, redesign `b` or `H_c` before reaching for a
generic ZK proof.

### 5. Put Privacy In The Fiber

Privacy should come from:

```text
Fiber(B_c, b)
```

being large, well-distributed, and safety-saturated.

The privacy theorem should say:

```text
public b samples only the boundary class,
not the representative, gauge, opening-local coordinates, or randomizer.
```

This is a structural hiding statement, not primarily a transcript simulator
statement.

### 6. Define Spend Over The Boundary Class

Spend should consume:

```text
b
```

or a projection of `b`, not an independent account-opening commitment.

The target theorem is:

```text
B_c(h)=b ∧ SpendAccept(c,b,sig,msg,state)
  => spend semantics depend only on b, not on the chosen h in the fiber.
```

If spend needs a representative-specific private key, that key must live in a
gauge/opening component whose public effect is already fixed by `b`.

## Failure Modes To Avoid

### Independent Role Objects

Dead shape:

```text
C_pad, C_projection, C_opening
```

plus a proof that they secretly match. This recreates the generic ZK bottleneck.

### Boundary As Hash Of Hidden Structure

Dead shape:

```text
b = H(h)
```

unless the security theorem needs only byte binding. If semantic verification
requires opening `H(h)`, we have not built a native boundary.

### Validity Hidden In The Proof

Dead shape:

```text
proof says b is valid
```

while `b` itself has no native validity condition. This makes the proof system
the design, rather than a compression/checking layer for the design.

### Unsafe Fiber

Dead shape:

```text
B_c(h)=B_c(h') but h and h' spend differently
```

No amount of proof engineering repairs this. It means the public account object
is underspecified.

## Immediate Construction Questions

1. What is the minimal `Meaning_c` that must be fixed at account creation?
2. Can `H_c` be factored as `MeaningLift × Gauge × OpeningLocal × Randomizer`?
3. What native obstruction/check map `S_c` can characterize valid public
   boundaries?
4. Can `S_c(b)=0` imply existence of a local representative without revealing
   one?
5. Is the fiber of `B_c` provably contained in `~safe`?
6. Can spend be stated entirely over `b` plus signer-held gauge/opening data?

## Current Working Hypothesis

The most promising holistic construction is:

```text
Boundary_c = descended account meaning + public obstruction certificate
H_c        = local lift of Boundary_c + private safety-irrelevant gauge
S_c        = native descent/gluing obstruction equation
Spend_c    = operation on the boundary class with private gauge authorization
```

If this works, account creation no longer needs to prove arbitrary hidden
witness validity. It checks that a public descended object is well formed, and
the private signer state is merely a local representative of that object.

