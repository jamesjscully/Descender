# n=32, m1=32 Structural Findings

Generated on 2026-05-15.

## Scope

This dossier investigates the `descender-128` `n=32, m1=32` candidate family in
three places:

- older seeded/full-view screening rows around `q=32771, m0=48`;
- compressed canonical rows around `q=32771, m0=48/52`;
- later guard-block lossy-diagonal rows around `q=65537, m0=80, view=80`.

## Main Finding

The apparent strength is not coming from the sparse tilt/fiber layer.  The
dominant exposed structure is the Cauchy/guard public view, and that view has
short centered-L1 q-ary dual relations.

For the later `q=65537, m0=80, view=80` row, a view-only BKZ-20 reduction found
a valid public-view dual relation with:

```text
required centered L1: 3277
found centered L1:    1464
support:              78 / 80
linf:                 96
uniform hit prob:     about 0.447
distinguishing gap:   about 0.553
```

This vector uses only the selected public view, so hiding the tilt coordinates
does not remove the direct dual statistic.

For the older `q=32771` rows, view/full reductions also found direct
counterexamples:

```text
m0=52, effective eta=5: required L1 3277, found L1 about 1106-1130
m0=48, effective eta=4: required L1 4097, found L1 about 1219-1323
m0=48, effective eta=3: required L1 5462, found L1 about 1160-1280
```

## Interpretation

The Cauchy guard block gives excellent rank and Hamming-distance behavior: every
prefix reaches rank `32` at width `32`, and the low-Hamming certificate is not
being violated.  But the q-ary dual-amplitude condition is much stronger than
low-Hamming distance.  BKZ finds dense, small-coefficient integer relations
across the Cauchy view.

So the mathematical structure that makes these rows compact also creates the
attack surface:

- the guard view is a highly structured Cauchy/GRS-like code;
- its q-ary dual contains dense low-height relations;
- centered bounded noise does not wrap enough along those relations;
- the direct statistic survives coordinate-view reduction.

## Artifacts

- `view_reduced_dual_vectors_bkz20.md`: view-only BKZ check for the later
  `q=65537, m0=80` rows.
- `view_direct_dual_distinguisher.md`: direct distinguisher calculation for
  those view-only BKZ vectors using the effective public-sample noise radius.
- `canonical_compressed_validation_reduced_bkz20.md`: BKZ check for compressed
  canonical `q=32771` screening rows.
- `seeded_validation_reduced_bkz20.md`: BKZ check for older seeded screening
  rows.
- `structure_dossier.md`: heuristic full-vs-view L1 probe and rank profile.
- `efficient_public_view_direction.md`: proof and algorithm refactor target
  for replacing the Cauchy guard-prefix public view.

## Next Direction

Do not invest in proving the current guard-prefix view as secure.  The next
candidate family needs to make the q-ary dual-amplitude condition a hard search
filter.  Concretely, search should reject any viewed public matrix for which
BKZ/LLL or exact bounded search finds a relation `z` with:

```text
effective_eta * ||z||_1 < q / 2
```

The likely repair paths are:

- replace consecutive Cauchy guard prefixes with a view whose q-ary dual has
  large centered L1;
- add enough noise/rounding budget to wrap these dense relations, if
  correctness can survive it;
- or make the public view include a structured mixing layer that breaks the
  low-height Cauchy dual relations rather than merely hiding tilt coordinates.

As an initial construction-side bound, the random-code Lee planner says that
for `m1=32` and a `2^-80` bad-relation budget:

```text
q=65537, target L1=3277: about 41 fully shared random rows, or 60-92 rows in
  grouped designs, should make low-Lee relations rare.
q=32771, target L1=4097: about 49 fully shared random rows, or 70-108 rows in
  grouped designs, should make low-Lee relations rare.
```

Those are not deployment parameters, but they are the right scale for the next
systematic Lee-expander search.
