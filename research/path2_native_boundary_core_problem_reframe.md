# Path 2 Native Boundary Core Problem Reframe

Status: conceptual reframe for path search.

Purpose: step back from the current masked trace / STARK-shaped route and
define the core problem we actually need to solve.

## Core Problem

We need a public account-admission object `b` and a proof/certificate such that:

```text
AcceptCreate(c, b, proof)
  =>
exists h.
  B_c(h) = b
  LocalValid_c(h)
  DescentConsistent_c(h)
```

while the public verifier learns only the safe boundary class of `h`, not the
private representative.

That is the whole problem.

Everything else is implementation strategy:

- STARK/IOP trace extraction;
- MPC-in-the-head full witness extraction;
- lattice/SIS sigma extraction;
- typed role subcommitments;
- range gadgets;
- QROM transcript scheduling;
- byte budgets.

## Three Simultaneous Requirements

Any viable route must provide:

1. **Existence / Knowledge**

   The verifier must know that some complete hidden representative exists. A
   proof that only checks sampled public openings is not enough.

2. **Native Consistency**

   All private roles must be restrictions/views of one object. We cannot afford
   independent commitments plus a large equality proof unless that equality is
   cheap by construction.

3. **No Export**

   The verifier must not see the raw hidden representative, hidden digits, pad
   source, opening tuple, masks, blinding/noise, or range witnesses.

The tension is that knowledge wants extraction, consistency wants shared
structure, and no-export wants hiding.

## Current Route In This Frame

The current masked trace route solves native consistency by making one parent
trace/root the source of all roles:

```text
role_i = Restrict(parent_trace, profile, role_i)
```

It solves no-export by masking/simulating public openings.

Its hard point is existence/knowledge:

```text
Accept => full hidden trace exists and satisfies relation
```

That is why we ended up with:

```text
ParentTraceCommitmentExtractionSoundness
ExtractedTraceRelationKnowledgeSoundness
NoExportSoundness
```

This is coherent, but it is now a proof-system-heavy route.

## Bigger-Picture Interpretation

The project is not fundamentally looking for "a STARK." It is looking for a
compact way to prove membership in the image of a descent boundary map:

```text
b in Im(B_c | valid representatives)
```

with zero-knowledge for the representative.

Equivalently:

```text
public object = boundary of a valid hidden descent representative
```

So the creative path search should ask:

```text
Can we make membership in Im(B_c) cheap by construction?
```

rather than:

```text
Can a general proof system prove the current relation cheaply?
```

## Where Creativity Can Enter

### 1. Change The Boundary Map

If `B_c` is chosen so that valid representatives have a compact canonical
certificate, proof work drops.

Question:

```text
Can B_c expose a safe algebraic image that is easy to verify but still hides h?
```

This could mean a boundary map whose image has native low-degree checks,
syndrome checks, or descent-gluing checks that are much smaller than proving a
full trace.

### 2. Change The Representative

Maybe `H_c` is currently too operational. If the representative is redesigned
around the proof, the relation may shrink.

Question:

```text
Can h be represented directly as quotient/descent coordinates whose local
validity is cheap?
```

Instead of proving that operational fields agree, construct the operational
fields as projections of a proof-native representative.

### 3. Change The Commitment Primitive

The trace route uses hash commitments because it is transparent/PQ. But a
descent/SIS-native commitment may align better with the algebra.

Question:

```text
Can role restrictions be SIS/module-SIS commitments whose binding and hiding
match the descent quotient directly?
```

This may avoid the full trace-extraction machinery, but only if hiding and
norm bounds do not recreate the parked sigma byte floor.

### 4. Change The Proof Mode

Maybe the right object is not a transparent trace proof. A full-witness proof
may be more natural.

Question:

```text
Is it cheaper to prove the whole relation in MPC-in-the-head than to retrofit
ZK and knowledge extraction onto a transparent trace?
```

This sacrifices some aggregation elegance, but may give cleaner ZK/extraction.

### 5. Split Admission Into Two Layers

Maybe one proof should not do everything.

Question:

```text
Can account creation publish a small algebraic admission certificate, and defer
some checks to spend or to state transition without weakening security?
```

This only works if deferred checks cannot admit a bad live slot. The boundary
must still force the spend-safe class.

## Non-Negotiable Core Constraint

No creative route can escape this:

```text
same public boundary b
same spend/replay/lifecycle behavior
some valid hidden representative exists
hidden representative is not exported
```

If a route does not establish those four facts, it is not a deployable security
signature path.

## Current Best Reframe

The problem should be viewed as:

```text
Zero-knowledge image-membership proof for a native descent boundary map.
```

Not:

```text
Transparent proof over a 176-row relation.
```

The 176-row relation is one way to prove image membership. It may be a good
way, but it is not the mathematical essence.

## Next Creative Search Questions

1. Can `B_c` be redesigned so `Im(B_c)` has a compact algebraic membership
   test?
2. Can the role handles be native SIS/descent commitments that give binding and
   hiding without a general ZK trace?
3. Can `h` be quotient-normalized so that range/norm checks become structural
   rather than proven rows?
4. Can spend handoff consume a smaller boundary certificate while account
   creation keeps the full representative hidden?
5. Is MPC-in-the-head actually simpler once the relation is 176 rows and the
   transparent route needs ZK plus extraction?

## Immediate Recommendation

Before selecting a proof family, compare three creative routes against the same
core image-membership problem:

| route | core idea |
|---|---|
| `native_image_membership` | redesign `B_c` so valid-boundary membership is directly checkable/algebraic |
| `descent_sis_commitment_membership` | use SIS/descent commitments for role restrictions and prove compact ZK opening |
| `full_witness_hash_zk` | use MPC-in-the-head / hash ZK for the whole image-membership relation |

The current masked trace route remains the baseline, but it should not be the
only mental model.

## Non-Claims

- This does not replace the current active path.
- This does not prove a new theorem.
- This does not update byte estimates.
