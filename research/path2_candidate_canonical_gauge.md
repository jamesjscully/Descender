# Path 2 Candidate C: Canonical Lift + Private Gauge

Status: third candidate sheet.

Purpose: evaluate whether account creation can admit a canonical public object
whose private signer state is a safety-irrelevant gauge/trivialization over
that object.

## Candidate Summary

Construct:

```text
q : Q_c
canonical_lift_c(q) : H_c / G_c
Fiber(boundary_c,q) ~= G_c . canonical_lift_c(q)
```

Account creation verifies public well-formedness/canonicality of `q`. The
signer privately holds:

```text
h = g . canonical_lift_c(q)
```

for private gauge/randomizer `g : G_c`.

The central law is gauge invariance:

```text
meaning_c(g.h)=meaning_c(h)
```

and the central privacy source is that `q` does not determine `g`.

## Intended Win

Candidate B tries to prove that public obstruction vanishing implies existence
of a lift. Candidate C avoids some of that burden by constructing `Q_c` so a
canonical lift exists by definition.

Private material is then not the semantic object. It is a local trivialization
or gauge choice over the public object.

This is attractive if pads, masks, randomizers, and local section choices can
be made pure gauge.

## Semantic Target `M_c`

Use `research/path2_meaning_v1.md`:

```text
M_c =
  AccountNamespace_c
  × SpendAuthorizationClass_c
  × LifecycleIdentity_c
  × TranscriptBindingSurface_c
  × StateTransitionBoundary_c
  × ParameterContext_c
```

The canonical public object must determine:

```text
meaning_of_boundary_c(q) : M_c
```

and gauge action must preserve it.

## Public Object `Q_c`

First-pass:

```text
Q_c =
  CanonicalMeaningBoundary_c
  × CanonicalSpendBoundary_c
  × CanonicalLifecycleBoundary_c
  × CanonicalTranscriptBoundary_c
  × CanonicalStateBoundary_c
  × CanonicalityCertificate_c
```

This can be organized as the pullback boundary from Candidate A, but with a
stronger requirement:

```text
WellFormedCanonical_c(q) => canonical_lift_c(q) exists
```

## Canonical Lift

Target:

```text
canonical_lift_c : Q_c -> H_c / G_c
```

Meaning:

`q` determines the representative only up to gauge, not as a raw hidden vector.

The lift may be:

- a canonical section modulo private pads/masks;
- a canonical quotient representative with private randomizer action;
- a canonical account object whose signer stores a local authorization
  trivialization.

The lift must not expose private gauge coordinates.

## Private Gauge `G_c`

Candidate gauge components:

```text
G_c =
  PadGauge_c
  × MaskGauge_c
  × RandomizerGauge_c
  × LocalSectionGauge_c
  × OpeningLocalGauge_c
```

Each component must be classified:

- pure gauge: affects representation but not account meaning;
- authorization-local: needed for signing but public effect fixed by `q`;
- semantic: not allowed in `G_c` unless moved into `M_c`.

## Gauge Action

Target:

```text
act_c : G_c -> H_c -> H_c
```

with:

```text
boundary_c(act_c(g,h)) = boundary_c(h)
meaning_c(act_c(g,h)) = meaning_c(h)
```

and:

```text
Fiber(boundary_c,q)
```

is either a `G_c`-torsor or a safety-equivalent union of gauge orbits.

## Spend Authority Question

This is the candidate's main risk.

If private gauge changes whether a spend is accepted, then gauge is not
safety-irrelevant. There are only three acceptable outcomes:

1. spend authority is fully determined by `q`;
2. spend authority uses signer-held local data, but public acceptance semantics
   are fully determined by `q`;
3. the authority-changing data is moved out of `G_c` and into `M_c`.

Dead outcome:

```text
gauge changes public spend semantics while q is unchanged
```

## Public Validity

Verifier checks:

```text
WellFormedCanonical_c(q)
```

where well-formedness includes:

- context/domain validity;
- canonical boundary encoding;
- spend/lifecycle/transcript/state consistency;
- existence of canonical lift modulo gauge;
- no duplicate semantic representation if canonicality claims uniqueness.

This check should be native and public.

## Effective Descent

Target:

```text
WellFormedCanonical_c(q)
  => exists h. boundary_c(h)=q ∧ LocalValid_c(h)
```

This is supplied by:

```text
h := choose representative of canonical_lift_c(q)
```

modulo gauge.

The proof burden shifts to constructing `canonical_lift_c` and proving it is
local-valid.

## Gate C1: Projection Completeness

First-pass result: plausible.

`Q_c` explicitly includes canonical spend, lifecycle, transcript, and state
boundaries, so public fields should factor through `q`.

Open audit:

- whether checked-core expects noncanonical or witness-derived fields;
- whether creation proof binding digest can be canonical-boundary digest;
- whether role handles disappear or become canonical restrictions.

## Gate C2: Safety Saturation

First-pass result: strong if gauge invariance holds.

Target:

```text
boundary_c(h)=boundary_c(h') => meaning_c(h)=meaning_c(h')
```

This follows if all fiber movement is gauge and gauge preserves `M_c`.

Risk:

If fiber contains multiple non-gauge components with different authorization
semantics, the candidate fails.

## Gate C3: Effective Descent

First-pass result: strong if canonical lift exists.

This candidate has the cleanest existence story:

```text
q valid => canonical_lift_c(q) exists
```

But that may be too strong. We must check whether a canonical public object can
be constructed without revealing private signer material.

## Gate C4: Fiber Privacy

First-pass result: central requirement.

The public object must not determine `g`.

Required:

```text
|G_c| or entropy(G_c) large enough for hiding
```

and public sampling of `q` must not condition `G_c` into a small or
distinguishable subset.

Risk:

Canonicalization can accidentally collapse the fiber and export private
structure. If the canonical lift is too explicit, privacy fails.

## Gate C5: Spend Closure

First-pass result: uncertain.

Spend closure is viable if:

```text
SpendAccept(c,q,sig,msg,state)
```

depends only on `q` publicly, while signer-held local gauge/authorization data
is used only to produce `sig`.

The key test:

```text
two gauges over q cannot produce different public authorization meanings
```

## Gate C6: Native Verification

First-pass result: pass if canonicality is public.

The verifier checks canonical boundary well-formedness, not a hidden witness
proof. The risk is that proving canonical lift existence may itself require a
hidden-witness proof unless `Q_c` is constructed carefully.

## Preliminary Disposition

Disposition: `[active-high-risk-high-reward]`.

Reason:

This candidate has the cleanest route to effective descent if canonical lifts
can be constructed without leaking private data. It is risky because canonical
public objects often destroy privacy by collapsing the fiber.

Likely role:

Candidate C may combine with Candidate B:

```text
obstruction vanishing proves canonical descended object;
private gauge supplies signer-local hiding and authorization.
```

## Next Questions

1. Which private coordinates are genuinely gauge?
2. Is spend authorization semantic or gauge-local?
3. Can `canonical_lift_c(q)` exist modulo gauge without revealing the gauge?
4. How large is the gauge fiber under realistic parameters?
5. Does the old quotient/fiber Lean work already supply a model for this
   candidate?

