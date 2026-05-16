# Efficient Public View Research Direction

Generated on 2026-05-15.

## What Changes

The next improvement should not be another parameter sweep over Cauchy guard
prefixes.  The replaceable bottleneck is the public-view construction itself.

The old route separates responsibilities:

```text
Cauchy/guard prefix        -> correctness and Hamming support
sparse tilt/fiber layer    -> compactness and rank shape
q-ary dual amplitude       -> checked after the fact
```

The `n=32, m1=32` investigation shows this is the wrong decomposition.  The
guard prefix is efficient for correctness, but its public q-ary relation code
has dense low-height Lee relations.  That creates a direct centered-noise
statistic.

The replacement target is a single public-view object that is efficient and
proof-carrying by design:

```text
public view A_view
  correctness: separates secrets at the viewed noise radius
  security:    ker(A_view) has large centered Lee minimum
  efficiency:  small seed, fast multiplication, cheap certificate validation
```

## Lean Boundary

This direction is now represented by:

```text
Descender/EfficientPublicView.lean
```

The central structures are:

```text
PublicViewEfficiency
PublicViewLeeDualCertificate
EfficientPublicViewCertificate
```

`PublicViewLeeDualCertificate` packages the exact security condition:

```text
∀ r, dualMap r = 0 -> r ≠ 0 ->
  leeMin <= centeredNormL1 q r

q <= 2 * eta * leeMin + 1
```

and derives:

```text
NoNarrowQaryDualRelations q eta dualMap
```

This is the proof-facing object that replaces low-Hamming dual certificates.

## Construction Target

The most direct construction family is systematic:

```text
A_view = [I | P]
ker(A_view) = { (-P y, y) : y ≠ 0 }
```

The proof obligation becomes:

```text
min_y!=0 centeredNormL1(q, (-P y, y)) >= leeMin
```

Equivalently, choose `P` as a seeded Lee expander.  The file
`EfficientPublicView.lean` includes `systematicLeeDualCertificate`, which turns
a `SystematicLeeDistanceAtLeast` proof for `P` into a public-view Lee-dual
certificate.

## Design Implications

The construction should be searched in this order:

1. Pick public-view dimensions that pass the Minkowski necessary condition.
2. Generate a structured candidate `P`.
3. Run view-only BKZ/LLL and centered-L1 probes.
4. Reject on any relation with `eta_eff * ||z||_1 < q / 2`.
5. For survivors, produce an exact sidecar:

```text
SystematicLeeDistanceAtLeast q P leeMin
```

6. Only then optimize public bytes and multiplication.

## Candidate Families To Try

Start with families that can plausibly be fast and proof-carrying:

- seeded dense `P`, as the baseline for what random Lee behavior costs;
- block-random `P`, guided by `lee_random_code_bound`;
- Toeplitz/circulant `P` with independent row/column diagonal scramblers;
- expander-lifted `P` where each block has a local random-code Lee certificate;
- arithmetic/Vandermonde blocks only if BKZ confirms they do not recreate the
  low-height relation problem seen in the Cauchy prefix.

## Current Scale Estimate

The random-code Lee planner suggests the following rough row budgets for
`m1=32` with a `2^-80` bad-relation budget:

```text
q=65537, target L1=3277:
  fully shared random block: about 41 rows
  grouped designs: about 60-92 rows

q=32771, target L1=4097:
  fully shared random block: about 49 rows
  grouped designs: about 70-108 rows
```

These estimates are not parameters.  They set the scale for the next
construction search.

## Practical Next Step

The first generator/search mode for systematic Lee expanders is now:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin systematic_lee_search -- \
  --q 65537 \
  --pivot-rows 48 \
  --free-cols 32 \
  --eta 10 \
  --families dense-random,block-random,toeplitz-scrambled,vandermonde-scrambled \
  --output-json research/parameter_search/systematic_lee_search.json \
  --output-md research/parameter_search/systematic_lee_search.md
```

The search output is a candidate family descriptor for `P` plus a sidecar
containing:

```text
leeMin
best found L1 on the systematic relation surface
unit/pair/random/anneal probe status
seed/descriptor size
mul/eval cost
probe/certificate validation cost estimate
```

This is the direct path from the cryptanalytic failure to an algorithmic
replacement.  It still needs two follow-up pieces before it becomes proof input:
view-only lattice reduction against each survivor, then an exact bounded
Lee-dual checker that emits the `SystematicLeeDistanceAtLeast` witness consumed
by `EfficientPublicView.lean`.

The first bounded checker is:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin systematic_lee_cert -- \
  --input-json research/parameter_search/systematic_lee_search.json \
  --max-candidates 4 \
  --max-support 2 \
  --coeff-bound 64 \
  --basis-dir research/parameter_search/systematic_lee_fplll \
  --output-json research/parameter_search/systematic_lee_cert.json \
  --output-md research/parameter_search/systematic_lee_cert.md
```

It is exact for the declared bounded domain
`support(y) <= max_support`, `|y_i| <= coeff_bound`,
`||y||_1 < leeMin`.  This makes the certification scope explicit and gives the
next implementation target a precise gap: either enlarge the exact domain via a
solver/global branch-and-bound, or replace the bounded checker with a family
theorem proving the same Lee lower bound for all nonzero `y`.  The optional
`--basis-dir` output is the BKZ/fplll handoff: it writes an integer basis for
the systematic q-ary relation lattice `{z : [I | P] z = 0 mod q}` for each
checked survivor.

## Global Proof Route

The Lean boundary now supports a real local-to-global proof:

```text
ComposableSystematicLeeCertificate
productBlockComposableSystematicLeeCertificate
blockDiagonalProductComposableSystematicLeeCertificate
blockDiagonalProductComposableCertificateOfCheckedLocal
ComposableSystematicLeeCertificate.toSystematicLeeCertificate
composableSystematicLeeDualCertificate
blockDiagonalCheckedLeeDualCertificate
```

For the current compressed public view, the concrete 128-bit profile is also
named in Lean:

```text
PublicSampleSerializationProfile
Descender128ProductBlockCompressedProfile.serialization
Descender128ProductBlockCompressedProfile.viewBudget
Descender128ProductBlockCompressedProfile.rounding_bound
Descender128ProductBlockCompressedProfile.effective_amplitude_bound
Descender128ProductBlockCompressedProfile.publicKeyBytes_eq
Descender128ProductBlockCompressedProfile.ciphertextBytes_eq
```

This records the deployed arithmetic bridge for the best row found so far:
`q = 32771`, `eta = 8`, 12-bit public-sample coordinates, `roundingEta = 5`,
effective eta `13`, 204 encoded sample bytes, 268 public-key bytes, and 236
ciphertext bytes.  The parameter-search executable now emits the corresponding
`metrics.public_sample_serialization` object and derives canonical public-key
and ciphertext sizes from it, with a Rust regression test checking this row
against the Lean profile name.

This is the route that makes an efficient construction proof-carrying.  Instead
of trying to certify an arbitrary dense `P` by bounded search, build `P` from
local blocks.  For each block, prove an exact local Lee lower bound over all
nonzero local free vectors.  Then prove two structural facts:

```text
1. every nonzero global y activates some local slice;
2. the activated local systematic Lee norm is bounded by the global systematic
   Lee norm.
```

Those facts compose into a global `SystematicLeeDistanceAtLeast` proof, which
then feeds the public-view no-narrow-dual certificate through
`composableSystematicLeeDualCertificate`.  Dense random candidates can still be
useful as benchmarks, but a deployed construction should be a composable block
family unless we add a trusted global solver certificate for the full free
space.

The block-diagonal route no longer leaves the local-to-global domination fact
as an external obligation.  Lean now proves:

```text
blockDiagonalSystematicPivotImage_eq
blockDiagonalSystematicRelation_local_le_global
```

and packages exact local bounded-free-vector checks through:

```text
blockDiagonalProductComposableCertificateOfCheckedLocal
blockDiagonalCheckedLeeDualCertificate
```

So the remaining proof input for a product-block construction is not a global
search over all `q^m1` free vectors.  It is a finite local certificate for each
block:

```text
BoundedFreeLeeCheckCertificate q LocalPivot LocalFree (Plocal b)
```

plus the arithmetic wrap inequality
`q <= 2 * eta * lowerBound + 1`.

The first concrete global artifact is:

```text
research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.md
```

It certifies a direct sum of 16 identical two-free-coordinate blocks:

```text
q = 32771
m1 = 32
rows per block = 5
used pivot rows = 80
target Lee l1 = 3277
checked local centered pairs = 21470904
global SystematicLeeDistanceAtLeast certified = true
```

This is the first example where the improvement comes from changing the
construction/proof object itself, not from changing the parameter filters.
In the parameter search this route should be selected as `product-block-lee`
(`shared-pair-lee` is the older compatibility name), and validation should be
run with the matching `--shared-lee-cert-json` sidecar so rows cannot pass on an
uncertified product-block assumption.

The same candidate is now wired into Lean as:

```text
Descender/SharedRowProductBlock.lean
```

with public names:

```text
SharedRowProductBlock32771.pairBlockMatrix
SharedRowProductBlock32771.globalMatrix
SharedRowProductBlock32771.dualMap
SharedRowProductBlock32771.wrap_eta_target
SharedRowProductBlock32771.leeDualCertificateFromRepeatedLocal
SharedRowProductBlock32771.noNarrowDualFromRepeatedLocal
SharedRowProductBlock32771.amplitudePromiseFromRepeatedLocal
```

This module proves the arithmetic wrap condition

```text
32771 <= 2 * 5 * 3277 + 1
```

and turns one externally checked local
`BoundedFreeLeeCheckCertificate` for the selected `5 x 2` block into the global
public-view `NoNarrowQaryDualRelations` promise for the repeated 16-block
matrix.  It does not hide the finite enumeration: the local certificate remains
an explicit input.

The local enumeration artifact is now independently re-checkable with:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin verify_shared_lee_cert -- \
  --input-json research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.json \
  --output-json research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.verify.json \
  --output-md research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.verify.md
```

The verifier recomputes every centered local pair in the declared domain and
checks that the artifact's count, pass flag, worst pair, and worst relation
match.  Current verification result:

```text
artifact consistent = true
verifier passes target = true
checked centered pairs = 21470904
target l1 = 3277
actual minimum l1 found = 3283
worst pair = (-614, -14)
worst relation = [1044, -435, -359, -400, -417, -614, -14]
```

So the `3277` local bound consumed by Lean is conservative by 6 for this
artifact.  The remaining trust boundary is now explicit: Lean checks the
global composition and wrap theorem, while the finite local enumeration is
checked by the independent Rust verifier.

A second sidecar shows the bandwidth/correctness tradeoff directly:

```text
research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2049_rows_per_group4.json
research/parameter_search/product_block_lee_sidecars_q32771_eta6_8_m32_r3_4/q32771_m132_target2049_rows_per_group4.verify.md
```

This certificate uses only 4 pivot rows per two-free-coordinate block, so the
16-block product uses 64 pivot rows instead of 80:

```text
q = 32771
m1 = 32
eta = 8
target Lee l1 = 2049
actual minimum l1 found = 2136
checked centered pairs = 8392704
artifact consistent = true
```

Feeding the eta-5 and eta-8 sidecar manifests into the parameter search and
validating the frontier gives a new proof-carrying 128-bit screening row:

```text
research/parameter_search/product_block_sidecar_eta8_floor_search_fullwidth
research/parameter_search/product_block_sidecar_eta8_floor_search_fullwidth_validated

q = 32771
n = 64
m0 = 88
m1 = 32
k = 112
eta = 8
pk = 304 B
ct = 272 B
security proxy = 179.0 bits
correctness slack = 8
guard slack = 8
amp rows = 64
validation = pass
```

This improves the previous eta-5 validated row (`352 B` public key, `320 B`
ciphertext) by lowering the proof-object row floor.  The cost is that eta rises
from 5 to 8 and the candidate sits exactly on the configured correctness/guard
slack boundary.  Also, the exact product-block amplitude route currently
requires full-width samples in the search model; compressed samples are rejected
until the compression error is folded into the exact certificate or a separate
rounding-compatible theorem is added.

The parameter search now has a provisional rounding-compatible mode for exact
proof strategies: compressed public samples are treated as deterministic
post-processing of the uncompressed public sample.  The intended proof split is:

```text
security:    compressed(t) is a deterministic function of t, so
             indistinguishability follows by data processing;
correctness: decoding sees t plus a bounded rounding error, so replace eta by
             effective_eta = eta + sample_compression_error;
amplitude:   require the Lee certificate to cover the effective eta threshold
             used by the search/validator.
```

With the eta-8, 64-row sidecar, this gives a smaller validated screening row:

```text
research/parameter_search/product_block_sidecar_compressed_samples_search
research/parameter_search/product_block_sidecar_compressed_samples_search_validated

q = 32771
n = 64
m0 = 104
m1 = 32
k = 112
eta = 8
sample bits = 12
rounding error = 5
effective eta = 13
pk = 268 B
ct = 236 B
security proxy = 229.5 bits
correctness slack = 14
amp l1 required = 1261
amp l1 certified = 2049
validation = pass
```

This is the current best concrete target.  The compression theorem is now
formalized as an abstract Lean bridge:

```text
Advantage.DistinguishingGame.advantage_map_eq
Advantage.DistinguishingGame.secureAgainst_map_of_source_secure
SampleView.restrictExperiment_advantage_eq_pullback
SampleView.restrictExperiment_secureAgainst_of_full
P_TND_Decision.withRoundingNoise_noiseSupport_of_bounds
P_TND_Decision.withRoundingNoise_realSearch_success_of_bounds
PTNDCorrectnessCertificate.withRoundingNoise
```

This says deterministic sample serialization is harmless for distinguishing
advantage by data processing, and correctness can charge a decoded rounding
vector by replacing `eta` with `eta + roundingEta`.  The remaining concrete task
is to instantiate the 12-bit decoder's rounding bound and make the KEM
serialization layer produce that `roundingEta = 5` witness.

## First Systematic Search Run

The first direct systematic-view run was completed with:

```text
q = 65537
pivot rows = 48
free columns = 32
eta = 10
target Lee l1 = 3277
families = dense-random, block-random, toeplitz-scrambled,
           vandermonde-scrambled
seeds per family = 32
```

Artifacts:

```text
research/parameter_search/systematic_lee_search.json
research/parameter_search/systematic_lee_search.md
research/parameter_search/systematic_lee_cert.json
research/parameter_search/systematic_lee_cert.md
research/parameter_search/systematic_lee_fpylll_reduction.json
research/parameter_search/systematic_lee_fpylll_reduction.md
research/parameter_search/systematic_lee_fplll/
```

Search result:

```text
top heuristic/probe best l1 values: 491177 to 500845
target l1: 3277
found counterexample: false for all top rows
best families: mainly toeplitz-scrambled, with vandermonde-scrambled and
               dense-random also surviving
```

Bounded exact certification was run on the top six candidates:

```text
max support(y) = 2
|y_i| <= 64
checked vectors per candidate = 8130560
bounded_pass = true for all six
best bounded l1 values = 428377 to 469203
```

The run also exported q-ary relation-lattice bases for each checked survivor.
After installing `fpylll` and `cysignals` in the user Python site, a view-only
LLL/BKZ sanity pass through BKZ-30 found no vector below target:

```text
best reduced-basis l1 values = 12953 to 14304
target l1 = 3277
below target = false for all six
```

A stronger interactive BKZ-40/top-two pass did not finish within a 180-second
cap, so the completed lattice-reduction evidence for this iteration is BKZ-30
only.  These results are encouraging as cryptanalytic screening, but they are
not yet a global Lean certificate.  The remaining proof gap is still exactly:

```text
∀ y != 0, centeredNormL1(q, (-P y, y)) >= 3277
```

for a concrete efficient family.  The bounded search and BKZ artifacts are
evidence for candidate selection; the deployable proof route should still use
either a composable block certificate or a global exact verifier whose coverage
is sufficient to instantiate `SystematicLeeDistanceAtLeast`.
