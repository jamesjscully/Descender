# Path 2 Native Candidate Gate Matrix

Status: first comparison pass.

Purpose: compare the three native account candidates against the same hard
structural gates before byte modeling or proof-system selection.

Candidates:

- Candidate A: pullback boundary
- Candidate B: obstruction-vanishing boundary
- Candidate C: canonical lift + private gauge

Gate values:

- `[pass]`: structurally satisfied in first pass
- `[likely-pass]`: likely satisfied, audit still needed
- `[unclear]`: real open theorem/design issue
- `[blocker]`: central unresolved blocker
- `[demote]`: not enough alone, may be component of final hybrid
- `[fail]`: candidate should be dead if not redesigned

## Matrix

| gate | Candidate A: pullback | Candidate B: obstruction | Candidate C: canonical gauge |
|---|---|---|---|
| projection completeness | `[likely-pass]` | `[likely-pass]` | `[likely-pass]` |
| safety saturation | `[unclear]` | `[likely-pass]` if `O_meaning` is real | `[unclear]` until gauge classified |
| effective descent | `[blocker]` | `[blocker]` but directly targeted | `[unclear]` if canonical lift exists |
| fiber privacy | `[unclear]` | `[unclear]` | `[blocker]` due fiber-collapse risk |
| spend closure | `[unclear]` | `[unclear]` | `[unclear]` |
| native verification | `[likely-pass]` for consistency only | `[blocker]` until concrete `obs_c` | `[unclear]` until canonicality/lift theorem |

## Candidate A: Pullback Boundary

Strengths:

- best public organization;
- makes cross-field context consistency structural;
- likely useful as the outer semantic boundary of any final design;
- gives clean projections into leaf, spend, lifecycle, transcript, and state.

Weakness:

```text
PullbackValid_c(q)
```

does not by itself imply:

```text
exists h. boundary_c(h)=q ∧ LocalValid_c(h)
```

Disposition:

```text
[demote-to-outer-boundary-component]
```

Candidate A should probably not be the full solution alone, but it is likely
the right organizing structure for `Q_c`.

## Candidate B: Obstruction-Vanishing Boundary

Strengths:

- directly targets effective descent;
- most native replacement for generic hidden-witness proof;
- can potentially express validity as public algebraic obstruction vanishing;
- integrates naturally with descent/sheaf/cocycle language.

Weakness:

The concrete object is not yet known:

```text
O_c
obs_c : Q_c -> O_c
```

and the main theorem must be account-backed:

```text
obs_c(q)=0
  => exists h.
       boundary_c(h)=q
     ∧ LocalValid_c(h)
     ∧ meaning_c(h)=meaning_of_boundary_c(q)
```

not merely:

```text
exists some glued object
```

Disposition:

```text
[active-core-candidate]
```

Candidate B should be the next theorem/search focus.

## Candidate C: Canonical Lift + Private Gauge

Strengths:

- cleanest lift-existence story if `canonical_lift_c(q)` exists;
- matches geometric torsor/gauge framing;
- may provide the privacy mechanism for Candidate B;
- can make private state a local trivialization rather than semantic account
  data.

Weakness:

Canonicalization can collapse privacy:

```text
q determines too much of h or g
```

The main theorem is gauge invariance:

```text
meaning_c(g.h)=meaning_c(h)
```

and the main privacy condition is nontrivial gauge fiber size.

Disposition:

```text
[active-supporting-candidate]
```

Candidate C is probably not the full public validity mechanism by itself, but
it may be the right private-state model for the final design.

## Current Best Hybrid

The strongest emerging shape is:

```text
Q_c = pullback-organized semantic boundary
obs_c(q)=0 = obstruction-vanishing validity over Q_c
H_c = local lift of q plus private gauge
G_c = safety-invariant private trivialization/randomizer space
```

In words:

```text
Candidate A supplies the public quotient organization.
Candidate B supplies the native validity/effective descent theorem.
Candidate C supplies the private representative/gauge model.
```

This is better than selecting exactly one candidate.

## Next Concrete Blocker

Define the first concrete obstruction candidate:

```text
O_c = O_context × O_gluing × O_meaning × O_spend_state × O_canonicality
obs_c : Q_c -> O_c
```

but now with actual algebraic content, not just names.

The next document should be:

```text
research/path2_obstruction_map_v1_attempt.md
```

and should answer:

1. What are the local restrictions?
2. What overlap/gluing equations are public?
3. What algebraic space does `O_gluing` live in?
4. How does `O_meaning` bind the glued object to `M_c`?
5. What part of `O_spend_state` prevents a bad spend/lifecycle fiber?
6. Which data remains private gauge?

## Selection

Selected strategy:

```text
hybrid A+B+C
```

with priority:

```text
1. Candidate B obstruction map construction
2. Candidate C gauge classification
3. Candidate A projection audit
```

Reason:

Candidate A alone lacks effective descent. Candidate B alone needs public
boundary organization and private fiber model. Candidate C alone risks making
canonicality either too public or too weak. The hybrid uses each candidate for
the structural role where it is strongest.

