# Vandermonde Lee Probe Summary

Date: 2026-05-15

## Question

Can an explicit algebraic moment-code block replace random shared Lee blocks?

The tested block has systematic graph-code form:

```text
G(y) = (P y, y)
P[t,j] = c_j alpha_t^(e0 + j)
```

The certificate target is:

```text
Lee(P y) + Lee(y) >= L for every nonzero y in F_q^g.
```

## Findings

The naive Vandermonde family is not viable.  With `e0 = 0`, the first column is
constant across rows, and one-coordinate vectors give low-Lee repeated row
values.  Midpoint and geometric scaling do not fix this.

Shifted moments with `e0 = 1` remove the constant-column defect.  Midpoint and
geometric scales still create small arithmetic-progressions in the first
column, but deterministic random nonzero column scales survive the current
bounded attacks at realistic row budgets.

For the 128-bit shape `q = 32771`, target `L = 3277`, `g = 32`, seed `1`:

| rows | best found Lee weight | result |
|---:|---:|---|
| 40 | 3183 | counterexample |
| 41 | 3265 | counterexample |
| 42 | 3349 | no counterexample found |
| 44 | 3523 | no counterexample found |
| 45 | 3613 | no counterexample found |
| 46 | 3705 | no counterexample found |

The deeper `r = 46` run used exact one-coordinate search up to `q/2`, pair
search up to coefficient `128`, 500000 sparse samples, and annealing.  The best
vector was still one-coordinate, so the immediate obstruction is column quality
rather than detected multi-column cancellation.

An exact CP-SAT encoding was added after these probes.  It verifies known
below-target rows by checking the direct finite condition

```text
exists y != 0 with Lee(P y) + Lee(y) < L.
```

For `r = 40`, CP-SAT verifies the probe counterexample immediately:

```text
y = (-1543, 0, ..., 0)
Lee(y) = 1543
Lee(P y) = 1640
total = 3183 < 3277.
```

For `r = 42`, the same exact CP-SAT encoding returned `UNKNOWN` after 120
seconds.  Z3 also returned `unknown` on the `r = 42` instance.  Therefore `r =
42` remains only a strong candidate, not a certificate.

A support-stratified checker was then added.  It proves the support-1 part
externally by exhaustive enumeration over every column and every centered
coefficient below the target.  For the `g=32,r=42` seed-1 instance, the exact
support-1 minimum is:

```text
y = (1543, 0, ..., 0)
Lee(y) + Lee(P y) = 3349 >= 3277.
```

Anchored support-2/3/4 coordinate-descent attacks did not improve this
bottleneck.  This is useful evidence that the first passing candidate is not
being protected by a missed one-column accident.  It is still not a full
certificate, because supports at least 2 are not exhaustively covered.

Support 2 is now externally closed for the same concrete block.  The exact
support-2 scanner enumerated every centered two-coordinate assignment with
coefficient `l1 < 3277`:

```text
checked assignments = 10643068800
found counterexample = false
```

Thus the remaining concrete finite gap begins at support at least 3.

## Interpretation

This is not a proof.  A clean probe means only that the construction is worth a
real certificate.  The useful theorem is now clear:

```text
For fixed public nonzero scales c_j and points alpha_t,
every nonzero y in F_q^g satisfies
Lee((sum_j c_j alpha_t^(1+j) y_j)_t) + Lee(y) >= L.
```

The proof route is arithmetic anti-concentration for scaled low-degree
polynomial values in centered Lee norm.  A plain MDS/Hamming argument is too
weak.

The Lean boundary for this proof is now `Descender.MomentLee`.  A concrete
certificate should prove `MomentLeeAntiConcentration` for the selected
evaluation points, column scales, and target `L`; Lean then derives
`ModqNoLeeKernelVectorBelow` through
`modqNoLeeKernelVectorBelow_of_moment_antiConcentration`.

The parameter-search strategy name for this route is `moment-lee`.

## Artifacts

- Probe implementation:
  `tools/parameter-search/src/bin/vandermonde_lee_probe.rs`
- Exact finite CP-SAT checker:
  `tools/cryptanalysis/moment_lee_cpsat_check.py`
- Exact finite Z3 checker:
  `tools/cryptanalysis/moment_lee_z3_check.py`
- Support-stratified checker:
  `tools/parameter-search/src/bin/moment_lee_stratified.rs`
- Exact support-2 scanner:
  `tools/parameter-search/src/bin/moment_lee_pair_exact.rs`
- Lean interface:
  `Descender/MomentLee.lean`
- Deep 128-bit `g=32,r=46` output:
  `research/parameter_search/vandermonde_lee_probe_g32_r46_shift1_random_seed1_deep_128.json`
- Smallest passing seed-1 row count:
  `research/parameter_search/vandermonde_lee_probe_g32_r42_shift1_random_seed1_128.json`
