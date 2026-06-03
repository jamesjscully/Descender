# Path 2 Concrete Solution Search Strategy

Status: active strategy for identifying the concrete native account design.

Purpose: convert the high-level descent/geometric framing into an ordered work
program that can find or reject concrete constructions without blind field
iteration.

## Strategy Summary

Proceed in four phases:

```text
1. Define the semantic target M_c.
2. Generate a small set of structurally different Q_c candidates.
3. Falsify candidates against hard laws before byte/proof work.
4. Promote the surviving candidate into Lean/theorem/byte models.
```

The search must start from account meaning and public quotient structure, not
from current pad/projection/opening witness fields.

## Phase 1: Pin The Semantic Target

Define:

```text
M_c = semantic account meaning fixed by account creation
```

Minimum contents:

- account namespace;
- spend authorization class;
- lifecycle/nullifier identity;
- checked-core transcript binding surface;
- state-transition boundary;
- context/version/domain binding.

Deliverable:

```text
Path2MeaningV1
```

with projections:

```text
m_spend
m_lifecycle
m_transcript
m_state
m_account_namespace
```

Promotion gate:

No candidate `Q_c` should be evaluated until we can say exactly what semantic
meaning it must classify.

## Phase 2: Generate Three Concrete Candidate Families

Do not generate many variants. Generate one serious candidate from each of the
three most promising families.

### Candidate A: Pullback Boundary

Construct:

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

Goal:

Make account/lifecycle/spend/transcript consistency native by construction.

Best if:

- existing public fields are individually sensible;
- the hard part is proving they all refer to the same object;
- consistency can be encoded as shared context/projection equality.

Primary risk:

This may still only organize public fields and fail to prove existence of a
private signer lift.

### Candidate B: Obstruction-Vanishing Boundary

Construct:

```text
Q_c = public descent boundary data
O_c = obstruction module/group
obs_c : Q_c -> O_c
```

Verifier checks:

```text
obs_c(q)=0
```

Goal:

Make account creation a public gluing/cocycle validity check.

Best if:

- role data can be modeled as local restrictions of one section;
- the obstruction object is compact;
- vanishing obstruction implies a valid local lift.

Primary risk:

The obstruction may show that something glues, but not that it glues to the
right account-backed spend authority.

### Candidate C: Canonical Lift + Private Gauge

Construct:

```text
q in Q_c
canonical_lift_c(q)
H_c(q) = G_c . canonical_lift_c(q)
```

Goal:

Make private material a safety-irrelevant gauge/trivialization over a public
canonical account object.

Best if:

- pad/mask/opening-local choices can be isolated as gauge;
- spend authorization can use private gauge without changing public meaning;
- the public object is directly checkable.

Primary risk:

The private gauge may actually carry spend authority, making it semantic rather
than safety-irrelevant.

## Phase 3: Fast Falsification Gates

Evaluate each candidate in the same order. Stop as soon as a gate fails.

### Gate 1: Projection Completeness

Every consensus/public field must factor through `q : Q_c`:

```text
field_i = p_i(q)
```

Fail if any required field depends on a representative-specific choice.

### Gate 2: Safety Saturation

The public fiber must be safety-saturated:

```text
boundary_c(h)=boundary_c(h') => h ~safe h'
```

Fail if two representatives over the same `q` can change spend, replay,
lifecycle, transcript binding, or state transition behavior.

### Gate 3: Effective Descent

Public validity must imply a private lift exists:

```text
obs_c(q)=0 => exists h. boundary_c(h)=q ∧ LocalValid_c(h)
```

Fail if validity is just a public consistency check with no account-backed
private representative.

### Gate 4: Fiber Privacy

The fiber over `q` must hide signer-local choices:

```text
Fiber(boundary_c, q)
```

Fail if `q` is injective in private pads, masks, opening locals, or other
representative-specific data.

### Gate 5: Spend Closure

Spend must be expressible over:

```text
q + signer-held local authorization
```

Fail if spend requires public semantics that were not fixed by `q`.

### Gate 6: Native Verification

The verifier should check native structure:

```text
obs_c(q)=0
```

or a compact native certificate. Fail or demote if the candidate immediately
requires a generic hidden-witness proof for its core validity.

## Phase 4: Promotion Path

When a candidate survives the gates:

1. write its concrete evaluation sheet;
2. define Lean interface types for `M_c`, `Q_c`, `O_c`, `H_c`, and maps;
3. state the five structural theorems:

   ```text
   projection_completeness
   safety_saturation
   effective_descent
   fiber_privacy_or_no_export
   spend_closure
   ```

4. update byte/state model;
5. decide whether any residual proof system is needed;
6. only then map to concrete encodings.

## Work Order

### Step 1: Meaning Object

Create:

```text
research/path2_meaning_v1.md
```

with the minimal semantic account object and required projections.

### Step 2: Candidate Sheets

Create:

```text
research/path2_candidate_pullback_boundary.md
research/path2_candidate_obstruction_boundary.md
research/path2_candidate_canonical_gauge.md
```

Each sheet must fill:

```text
M_c
Q_c
H_c
G_c
O_c
obs_c
public projections
effective descent theorem
safety saturation theorem
privacy source
spend closure
failure mode
```

### Step 3: Gate Review

Compare the three candidates in:

```text
research/path2_native_candidate_gate_matrix.md
```

Use `[pass]`, `[fail]`, `[unclear]`, or `[demote]` for every gate.

### Step 4: Promote One Active Candidate

Once one candidate is clearly strongest, create:

```text
research/path2_active_native_account_design_v1.md
Descender/Path2NativeAccountDesign.lean
```

The Lean file should be theorem-interface level first. Concrete crypto
instantiation comes after the structural laws are stated.

## Decision Rule

Prefer the candidate that makes the largest number of facts structural:

```text
projection completeness
safety saturation
effective descent
fiber privacy
spend closure
```

The right solution is not necessarily the smallest immediate byte encoding. It
is the one that minimizes generic hidden-witness proof burden while preserving
blockchain-verifiable account semantics.

## Fallback Rule

If all three native candidates fail:

- keep the masked-trace route as the measured fallback;
- use the failure sheets to identify the exact theorem that native structure
  cannot provide;
- do not return to ad hoc commitments without explaining which native gate they
  failed and why.

