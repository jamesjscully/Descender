# Path 2 Descent/SIS Commitment Viability

Status: viable primary research path; not yet a deployable commitment
instantiation.

## Question

Can the original Descender thesis be reused here: derive the Path 2
proof-friendly commitment from the same descent/category-theoretic lattice/SIS
approach used for the key-generation and account-link work?

## Short Answer

Yes, this is the most coherent next path.

The current Path 2 blocker is exactly the kind of problem the descent interface
was meant to organize:

```text
public verifier object
  -> public quotient/fiber
  -> local representative checks
  -> gluing to hidden account-backed state
  -> or named binding/soundness/hiding break
```

The existing Lean sidecar already has the abstract shape:

- `Descender.DescentInterface.LocalQuotientDescent`;
- `Descender.DescentInterface.EffectiveDescentCertificate`;
- `Descender.HiddenCommittedQuotientLink.Interface`;
- `accepts_glues_or_breaks_of_effectiveDescent`.

This does not yet instantiate a cryptographic commitment. It does show that the
proof architecture needed by Path 2 is aligned with the original thesis rather
than a detour from it.

## Why It Fits Path 2

Path 2 needs a public account-creation payload that proves hidden data exists
without revealing it:

```text
masked_projection_i = pad_lane_i + account_projection_i mod q
```

The toy q-linear verifier already shows the local relation is small and
q12289-native. What is missing is a real hidden commitment layer.

The descent/SIS route would make the payload statement:

```text
accepted local q12289 relation
  either descends/glues to a hidden account-backed opening
  or exports a named break:
    - SIS/module-SIS binding break
    - hiding/sampler break
    - link-equation proof break
    - public-projection leakage break
    - Fiat-Shamir/QROM break
```

That is almost exactly the theorem target already used in the previous
quotient-link research, but moved from spend-time account-link compression into
account-creation proof payload semantics.

## What Can Be Reused

### Reusable Proof Shape

The abstract Lean interfaces are directly reusable:

- `LocalQuotientDescent` for public quotient/fiber/local representative
  structure;
- `EffectiveDescentCertificate` for local-valid implies glues-or-breaks;
- `HiddenCommittedQuotientLink.Interface` for hidden link values behind
  commitments;
- named break granularity: commitment binding, link equation, public projection,
  and sampler hiding.

### Reusable Negative Lessons

Earlier quotient-link work already found two important constraints:

- public quotient links condition the hidden sampler too strongly;
- hidden commitments only help if the link proof is not just a public projection
  of the sigma equation.

Those constraints should become first-class Path 2 design rules. A Path 2
descent/SIS commitment must keep link values hidden and must prove a genuine
hidden relation, not expose a public linear shortcut.

### Reusable Security Surface

The prior keygen/SIS work gives the right assumption vocabulary:

- exact public matrix distribution;
- SIS/module-SIS binding game;
- sampler hiding/conditioning analysis;
- bounded-kernel or short-relation attack review;
- QROM Fiat-Shamir accounting.

The parameters cannot simply be copied. The Path 2 payload commits different
vectors and needs different norm/range bounds.

## What Is Not Yet Reusable

The old hidden-commitment quotient-link branch was rejected for deployment in
that context because it did not save bytes or simplify the spend path. That
rejection should not be over-applied here.

For Path 2, the goal is different:

- not saving spend bytes;
- not replacing the checked-core signature;
- not compressing a spend-time account-link object;
- instead, giving account creation a public hiding proof while keeping spend at
  252 bytes.

So the old "no deployable byte win" result does not kill this path. It only
tells us what to avoid:

- do not make link values public;
- do not add hidden commitments without a non-public link proof;
- do not claim existing account commitments give the link equation for free.

## Candidate Path 2 Commitment Shape

A concrete descent/SIS candidate should start with a committed vector:

```text
v =
  pad_lanes[0..5]
  account_projection_lanes[0..5]
  opening_tuple_scalars
  auxiliary range/norm witnesses
  blinding/noise
```

and public masked projection:

```text
m_i = pad_lanes_i + account_projection_lanes_i mod q
```

A plausible commitment form is:

```text
C = A * v + e mod q
```

or a module/SIS variant with role-separated matrices:

```text
C_pad   = A_pad * pad_vector + B_pad * r_pad mod q
C_proj  = A_proj * projection_vector + B_proj * r_proj mod q
C_open  = A_open * opening_vector + B_open * r_open mod q
```

The proof then shows:

```text
open(C_pad), open(C_proj), open(C_open)
linear relation m = pad + projection
range/norm bounds for openings and blindings
Path2CreationProofBindingV1 context agreement
```

without revealing `pad`, `projection`, or private opening frames.

## Main Viability Risks

### 1. Binding And Hiding May Fight Each Other

SIS commitments are naturally binding under short-kernel hardness. Hiding
requires enough randomness/noise or a statistically hiding construction. The
exact dimensions must support both simultaneously.

### 2. Proof Bytes May Grow

The account-creation payload can tolerate more bytes than spend, but this still
has blockchain cost. The first feasibility pass must estimate:

- commitment bytes;
- proof bytes;
- state bytes;
- validator q-ops.

### 3. Norm Bounds Are Security-Critical

A linear proof over q12289 is not enough. We need range/norm checks that prevent
large modular witnesses from satisfying the relation while escaping the intended
hidden-source semantics.

### 4. Sampler Conditioning Must Be Rechecked

The previous public-link branch failed because public link values conditioned
the sampler. The new commitment must preserve hiding under the exact public
Path 2 view:

```text
leaf + creation_proof_binding_digest + commitments + proof transcript
```

### 5. It Needs A Concrete Extractor Story

The final theorem must say that an accepting payload either yields an extractor
for a valid hidden opening or exports a named break. The descent interface gives
the wrapper; the SIS proof must supply the candidate-specific extractor.

## Recommended Next Work

Create a small feasibility artifact for a `Path2DescentSisCommitmentV1`:

1. Define the exact witness vector and public quotient.
2. Define role-separated matrix derivation domains.
3. Define commitment equations and byte encodings.
4. Define the local relation checked by the proof.
5. State the gluing theorem using `LocalQuotientDescent`.
6. Estimate bytes and validator q-ops.
7. Add negative controls:
   - public projection shortcut;
   - wrong masked projection;
   - wrong leaf/context binding;
   - non-short witness opening;
   - duplicate or malleated commitment;
   - sampler-conditioning failure.

## First Feasibility Artifact

`scripts/research/path2_descent_sis_feasibility.py` now generates:

```text
research/path2_descent_sis_feasibility.md
research/path2_descent_sis_feasibility.json
```

The artifact pins the first candidate shape:

- public quotient: six masked projection residues from lanes 12..17 of the
  canonical 18-residue packed witness commitment;
- local relation:
  `masked_projection_i = pad_lane_i + account_projection_i mod q`;
- committed roles:
  - `pad_source`: six hidden pad lanes plus 24 blinding/noise lanes;
  - `projection_relation`: six hidden account-projection lanes plus 24
    blinding/noise lanes;
  - `opening_tuple`: placeholder scalarization with 8 secret lanes, 32
    blinding/noise lanes, and 8 range-witness lanes.

The first byte/q-op rows are:

| candidate | commitment rows | commitment bytes | payload lower bound | verifier mul-adds | status |
|---|---:|---:|---:|---:|---|
| `path2_descent_sis_rows_32` | 32 | 168 | 557 | 3456 | wiring only |
| `path2_descent_sis_rows_48` | 48 | 252 | 725 | 5184 | primary feasibility band |
| `path2_descent_sis_rows_64` | 64 | 336 | 893 | 6912 | primary feasibility band |
| `path2_descent_sis_rows_96` | 96 | 504 | 1229 | 10368 | larger fallback band |

These are lower bounds, not proof-size claims. They exclude rejection-sampling
slack, norm proof overhead, repetition, extractor machinery, and any concrete
SIS hardness margin. Their value is that the candidate object is now precise
enough to attack.

The next bottleneck is no longer "what vector are we committing?" It is:

```text
Can the 48-row or 64-row feasibility band be given concrete SIS binding,
hiding/sampler, norm/range, and Fiat-Shamir extraction proofs without pushing
account-creation payload bytes past the transparent-proof fallback?
```

## Decision

This should replace the generic "lattice/SIS commitment" option as the primary
Path 2 line of attack.

The path is viable because it matches the original Descender proof philosophy
and gives a post-quantum alternative to Pedersen-style commitments. It is not
yet deployable because the concrete commitment parameters, hiding proof,
extractor, and byte model do not exist.

The correct next gate is not "pick STARK vs lattice" in the abstract. It is:

```text
Can Path2DescentSisCommitmentV1 instantiate the descent/SIS gluing theorem
with acceptable account-creation bytes and validator cost?
```

If yes, it is the preferred production route. If no, transparent hash-based
proofs remain the fallback.
