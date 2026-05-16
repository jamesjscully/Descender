# Lee Expander Path Recommendation

Date: 2026-05-15

## Question

We need a practical way to certify the q-ary centered-Lee amplitude condition:

```text
min_{y != 0} Lee(y) + Lee(P y) >= L
```

for the solved systematic map `A^T = B0 [I | P]`.  The immediate choice is
between:

1. deterministic sidecar verification for each generated `P`;
2. a random-block theorem showing a transparent random `P` is good except with
   negligible probability;
3. a fully explicit algebraic expander family with a direct proof.

## Current Evidence

The exact scalar-block and pair-block sidecars show the problem is real, but
also expose the scaling limit.

For the current 128-bit amplitude target `q=32771, m1=32, L=3277`:

```text
scalar block:
  n=128, rows/free=4  -> passes

shared pair block:
  n=64,  rows/pair=4 -> misses by 1 Lee unit
  n=80,  rows/pair=5 -> passes
```

The random-code union-bound analyzer gives the more general row-budget picture.
At an 80-bit expected-bad budget:

```text
128-bit shape: q=32771, L=3277, m1=32
  group 8:   total rows 104, qamp ops 832
  group 16:  total rows 66,  qamp ops 1056
  group 32:  total rows 46,  qamp ops 1472

192-bit shape: q=16411, L=1173, m1=56
  group 8:   total rows 161, qamp ops 1288
  group 14:  total rows 108, qamp ops 1512
  group 28:  total rows 72,  qamp ops 2016
  group 56:  total rows 51,  qamp ops 2856

256-bit shape: q=65537, L=2979, m1=64
  group 8:   total rows 176, qamp ops 1408
  group 16:  total rows 116, qamp ops 1856
  group 32:  total rows 80,  qamp ops 2560
  group 64:  total rows 59,  qamp ops 3776
```

The pattern is stable: larger shared blocks substantially reduce row count, but
increase per-row key-generation work and make exact sidecar verification much
harder.

## Path Assessment

### Exact Deterministic Sidecar

This is strongest at the deployed-key level: the public artifact says exactly
why this `P` is safe.  It is also the hardest to scale.  The current pair
sidecar enumerates all centered pairs below the target.  For group size `g`,
the naive verifier scales like a Lee ball in dimension `g`, which is fine for
`g=2` and quickly becomes infeasible for `g=8,16,32`.

Use this path as a validation/audit tool, not as the primary design route.  It
is still valuable for:

- group size 2 certificates;
- testing small toy versions of larger constructions;
- finding counterexamples to proposed deterministic families;
- checking selected seeds when a solver can finish.

### Random-Block Theorem

This is the best primary path.  The theorem shape is clean:

```text
For each nonzero y, each random shared row dot product is uniform in F_q.
Therefore Pr[Lee(y) + Lee(P y) < L] is the exact Lee lower tail.
Union bound over all y with Lee(y) < L and over all groups.
```

This theorem directly matches the row-budget tool.  It gives a transparent
setup story: sample the public seed by domain-separated expansion from public
entropy; the probability of a bad setup is bounded by the theorem.  There is no
secret trapdoor because the setup is sampled from a public, fixed distribution,
and the proof bounds the bad event.

The practical generator can use rejection sampling when we have a cheap
screening test, but the security proof should not depend on an expensive exact
checker.  The theorem itself is the certificate boundary.

### Fully Explicit Algebraic Expander

This would be ideal if found: no randomized setup failure probability and no
large deterministic sidecar.  But it is the least certain research path.  We do
not yet have a concrete algebraic family with a centered-Lee lower bound strong
enough for the parameters.

Keep it as a secondary research track.  The random-code theorem gives a target
to beat: an explicit family should approach the group-size row budgets above.

Follow-up note:
`research/parameter_search/explicit_algebraic_lee_expander_abstraction.md`
reframes this path as systematic Lee-code construction and compares
Lee-BCH/RS, Vandermonde/moment graph codes, integer `l1` expanders, and
character/cyclotomic pseudorandom blocks.

The first probe makes the explicit path less vague: the useful target is not
plain Vandermonde.  Degree-zero moment columns and midpoint/geometric scales
produce concrete one-coordinate Lee counterexamples.  Shifted moments with
deterministic random nonzero column scales are the first explicit family that
survives bounded attacks near the random-code row budgets; for the 128-bit
`g=32` shape, seed 1 survives at `r=42` and fails at `r=41`.

## Recommendation

Follow the random-block theorem as the main proof path, with deterministic
sidecars as auxiliary validation.

Concrete next tasks:

1. Add a Lean theorem interface for `RandomSharedLeeBlockGood`, separating the
   counting theorem from the generated setup.
2. Prove the deterministic reduction from per-block Lee goodness to
   `ModqNoLeeKernelVectorBelow` for direct sums of blocks.
3. Formalize or at least artifact-certify the row-budget calculation:
   Lee-ball volume by free `l1` and row-sum lower-tail probability.
4. Extend parameter search with a `random-shared-lee` strategy carrying
   `group_size`, `rows_per_group`, and `failure_bits`.
5. Keep `shared_lee_cert` for exact pair-block audit and counterexample search.

Near-term parameter strategy:

- Use group size 8 or 16 as the first engineering target.  These are a better
  tradeoff than full group size: they reduce rows substantially while keeping
  key-generation work moderate and leaving some hope for sidecar diagnostics.
- Treat group size 32/56/64 as a theoretical lower-row target, but do not
  commit to it before key-generation and setup-validation costs are modeled.

Bottom line: deterministic sidecars answer "is this exact sampled block good?"
but do not scale well enough to drive design.  The random-block theorem answers
"how many shared rows make bad blocks negligibly unlikely?" and is the right
primary path for the next proof layer.
