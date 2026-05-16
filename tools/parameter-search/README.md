# Descender Rust Parameter Search

This is the high-throughput concrete-parameter search engine.  It implements
the same coarse filters and output schema as the Python reference harness in
`scripts/parameter_search`, but is intended to become the primary engine for
large grids and exact finite-field diagnostics.

## Build

```bash
cargo build --release --manifest-path tools/parameter-search/Cargo.toml
```

## Run

```bash
tools/parameter-search/target/release/descender-parameter-search --profile descender-128
tools/parameter-search/target/release/descender-parameter-search --profile descender-192
tools/parameter-search/target/release/descender-parameter-search --profile descender-256
```

Outputs are written to `research/parameter_search/` by default.  Use
`--output-dir` for scratch comparisons.

## Validate a Frontier

After a bandwidth search, run the expanded seeded-frontier validator:

```bash
tools/parameter-search/target/release/validate_frontier \
  --input-dir research/parameter_search/bandwidth_canonical \
  --output-dir research/parameter_search/bandwidth_canonical_expanded_validation \
  --top 64 \
  --dual-search-weight 2
```

This derives deterministic public coefficient, incidence, and diagonal seeds
from each candidate, expands a concrete sparse tilt matrix with exactly three
nonzero row entries per tilt column, combines it with Cauchy guard/base blocks,
and checks exact finite-field rank, guard-block rank, column support, guard
distance, and bounded dual witnesses.

Validation reports now preserve `proof_strategy` and `view_coords`.  A row with
a non-full viewed sample is deliberately marked with
`view_reduction_certificate_missing` until a support-covered, lossy-budget, or
guard-block lossy-diagonal view certificate has been supplied for that concrete
selected coordinate family.  This keeps smaller public-sample rows visible in
search while preventing them from being mistaken for deployable parameters.

Validation also preserves the primal-distance model.  Rows using
`combined-proxy` are marked with
`combined_primal_distance_certificate_missing` until the combined-matrix
primal-distance theorem is supplied.

For deeper checks on only the first row per profile:

```bash
tools/parameter-search/target/release/validate_frontier \
  --input-dir research/parameter_search/bandwidth_canonical \
  --output-dir research/parameter_search/bandwidth_canonical_expanded_validation_w3_top1 \
  --top 1 \
  --no-pareto \
  --dual-search-weight 3
```

The engine is multithreaded by default.  It uses the host's available parallelism
unless `--threads N` is supplied.  The chosen thread count is recorded in the
JSON output under `search_space.threads`.

## Larger Searches

The engine accepts comma-separated grids.  Grid mode is exhaustive over the
declared lists:

```bash
tools/parameter-search/target/release/descender-parameter-search \
  --mode grid \
  --profile descender-192 \
  --q-values 12289,40961,65537,131071 \
  --n-values 128,192,256,384,512,768,1024 \
  --m0-overheads 16,32,64,128,256,512 \
  --m1-multipliers 1,2,3,4,6,8 \
  --fiber-dims 4,8,16,32 \
  --eta-values 1,2,3,4,6,8,12 \
  --view-coords 0 \
  --threads 8
```

It also supports simulated annealing over the same discrete parameter lists:

```bash
tools/parameter-search/target/release/descender-parameter-search \
  --mode anneal \
  --objective bandwidth \
  --public-key-model canonical \
  --coefficient-model seeded \
  --incidence-model seeded \
  --profile descender-192 \
  --q-values 12289,40961,65537,131071,262147,524309 \
  --n-values 64,80,96,112,128,160,192,224,256,320,384,512 \
  --m0-overheads 8,16,24,32,48,64,96,128,192,256,384 \
  --m0-values 32,40,48,56,64,80,96,112,128,160,192,224,256,320,384 \
  --m1-multipliers 1,2,3,4,5,6,8,10 \
  --fiber-dims 4,6,8,12,16,24,32,48 \
  --eta-values 1,2,3,4,5,6,8,10,12,16 \
  --view-coords 0,48,64,80,96,128,160,192,256 \
  --proof-strategies full-surjective,support-covered,lossy-budget \
  --primal-distance-models guard-only,combined-proxy \
  --anneal-restarts 128 \
  --anneal-iterations 100000 \
  --threads 16
```

Rows are sorted by `objective_score`.  Use `--objective screening` for the older
coarse filter that balances security proxy, public size, ciphertext size,
secret-key size, large `q`, tiny noise, and guard redundancy.

Use `--objective bandwidth` after the representation model is fixed.  This
treats proof-side constraints as hard filters, caps extra security credit with
`--security-extra-cap`, and strongly prioritizes ciphertext bytes before public
key bytes and total dimension.  This is the right mode for finding whether the
current theorem conditions force large measurements.

Useful bandwidth knobs:

```bash
--security-extra-cap 16
--guard-slack-target 8
```

`security_proxy_bits` is still reported because it is useful diagnostic context,
but accepted rows require exact finite-field and lattice/coding follow-up before
any security claim.

## Public-Key Representation

The `--public-key-model` flag controls how much setup material is charged to
each public key:

- `component`: legacy accounting.  Stores a setup seed, Cauchy description,
  coefficient setup data, incidence setup data, diagonal setup data, and the
  noisy public sample.
- `canonical`: optimized accounting.  Stores one master public seed plus the
  noisy public sample `t = A s + e`.  Cauchy generation, coefficient rules,
  incidence rules, and diagonal rules are fixed by the parameter set and
  regenerated by domain-separated seed expansion.

The canonical model is the intended structural optimization path.  It removes
per-key Cauchy descriptions, collapses component seeds into one master seed, and
moves rule descriptors into the specification rather than the serialized public
key.

## Public Sample Compression

The `--sample-bits` flag searches bit-packed encodings for the public sample
`t = A s + e` and the matching ciphertext coordinate vector:

```bash
--sample-bits 10,11,12,13,14,15,16,0
```

`0` means full field-element width. Nonzero values pack each coordinate into
that many bits. The search model treats truncation/rounding as extra centered
noise before applying the guard-distance correctness filter:

```text
effective_eta = eta + sample_compression_error
required_guard_distance = 2 * effective_eta + 1 + guard_slack
```

This is intentionally conservative. A compressed row is accepted only when the
current proof-side correctness budget can absorb the quantization error.

For exact full-view strategies, public-sample compression is modeled as
deterministic post-processing of the uncompressed public sample.  The intended
proof obligation is a small data-processing/rounding lemma: security follows
from the uncompressed sample because rounding is a deterministic function, while
correctness uses `effective_eta` in place of `eta`.
The abstract Lean bridge for this is:

```text
Advantage.DistinguishingGame.advantage_map_eq
SampleView.restrictExperiment_advantage_eq_pullback
P_TND_Decision.withRoundingNoise_noiseSupport_of_bounds
PTNDCorrectnessCertificate.withRoundingNoise
PublicSampleSerializationProfile
Descender128ProductBlockCompressedProfile.serialization
Descender128ProductBlockCompressedProfile.rounding_bound
Descender128ProductBlockCompressedProfile.publicKeyBytes_eq
Descender128ProductBlockCompressedProfile.ciphertextBytes_eq
```

The current concrete 128-bit row is wired as
`Descender128ProductBlockCompressedProfile.serialization`: `q = 32771`,
`m0 + m1 = 136` public-sample coordinates, `sampleBits = 12`,
`roundingEta = 5`, `effectiveEta = 13`, `encodedSampleBytes = 204`,
`publicKeyBytes = 268`, and `ciphertextBytes = 236`.

Executable search output now mirrors this proof object in
`metrics.public_sample_serialization`.  Canonical public-key and ciphertext
byte counts are derived from that object:

```text
encodedSampleBytes = packedBytes(viewCoordinateCount, sampleBits)
publicKeyBytes = seedBytes + encodedSampleBytes
ciphertextBytes = encodedSampleBytes + ciphertextOverheadBytes
```

The search rejects compressed rows whose recorded rounding profile does not
satisfy the same arithmetic bound used by the Lean profile.

To emit a concrete size/layout artifact for the current canonical row:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin canonical_serialization_vector
```

This writes JSON/Markdown layout metadata and zero-filled PK/CT skeleton files
under `research/parameter_search/canonical_serialization_vectors/`.  The
skeleton binaries are for byte-size and layout benchmarking only; they are not
cryptographic known-answer tests.

## Proof Strategy Comparison

The `--proof-strategies` flag makes the proof route an explicit search axis:

```bash
--proof-strategies full-surjective,support-covered,lossy-budget
```

- `full-surjective`: conservative baseline.  Publishes the full selected view
  and uses the surjective diagonal-amplification theorem.
- `support-covered`: exact coordinate-subset strategy.  Requires a selected
  view that hits every relevant secret-pair support; this is the main route for
  future structural public-sample reduction.
- `lossy-budget`: permits coordinate-view reduction or otherwise lossy viewed
  samples by charging the rounding error to the effective viewed noise budget.

Exact strategies still require an exact proof for the uncompressed public
sample, but they may use nonzero `--sample-bits` for serialization when the
rounding error is absorbed by `effective_eta`.  Coordinate-view reduction
remains separate: `full-surjective` still rejects smaller views, while
`support-covered` and `lossy-budget` keep them as candidates for later proof
work.

The `--view-coords` flag controls that coordinate-view size:

```bash
--view-coords 0,48,64,80,96,128
```

`0` means publish the full coordinate set of size `m0 + m1`.  Nonzero values
price only that many viewed public-sample/ciphertext coordinates.  The
`full-surjective` proof strategy rejects smaller views; `support-covered` and
`lossy-budget` keep them as candidates, with the understanding that follow-up
validation must prove the selected-coordinate hitting condition or the explicit
lossy view budget.

The current concrete view model selects a prefix of the combined coordinate
list, with guard coordinates first.  A compressed row may still use the
`guard-only` primal-distance model when the selected prefix contains the full
guard block, i.e. `view_coords >= m0`.  The validator reports
`guard_only_requires_guard_view` when a row tries to use guard-only correctness
after hiding some guard coordinates.

## Lee Expander Probes

The `vandermonde_lee_probe` binary tests explicit systematic Lee-code blocks of
the form

```text
G(y) = (P y, y)
P[t,j] = scale_j * alpha_t^(exponent_start + j)
```

It is a counterexample/probe tool, not a distance prover.  A below-target vector
is a real obstruction for that concrete block; a clean run only identifies a
candidate family for a later certificate or Lean theorem.

Useful 128-bit probe:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin vandermonde_lee_probe -- \
  --q 32771 \
  --group-size 32 \
  --rows 46 \
  --target-l1 3277 \
  --alpha-start 1 \
  --exponent-start 1 \
  --column-scale random \
  --unit-bound 16385 \
  --pair-bound 128 \
  --coeff-bound 4096 \
  --random-samples 500000 \
  --max-random-sparsity 8 \
  --anneal-restarts 256 \
  --anneal-steps 30000 \
  --seed 1 \
  --output-json research/parameter_search/vandermonde_lee_probe_g32_r46_shift1_random_seed1_deep_128.json \
  --output-md research/parameter_search/vandermonde_lee_probe_g32_r46_shift1_random_seed1_deep_128.md
```

Two design lessons from the initial probes are important:

- `exponent_start=0` is structurally bad because the degree-zero column is a
  repeated scalar row and admits tiny one-coordinate Lee relations.
- midpoint/geometric column scales are too structured.  Shifted moments with
  deterministic random nonzero column scales are the first algebraic family
  that survived the current bounded attacks near the random-code row budgets.

The main search engine also accepts:

```bash
--qary-amplitude-strategies moment-lee
```

In this mode `--lee-repetitions` is interpreted as the claimed/proved Lee lower
bound for a shifted scaled moment graph-code block, and the engine prices
key-generation work as `n * m1` field multiply-adds.  The Lean boundary for this
strategy is `Descender.MomentLee`: the concrete family still needs an
anti-concentration certificate before a row is deployable.

Use `tools/cryptanalysis/moment_lee_cpsat_check.py` to ask the exact finite
certificate question for a probe artifact:

```bash
python3 tools/cryptanalysis/moment_lee_cpsat_check.py \
  --probe-json research/parameter_search/vandermonde_lee_probe_g32_r42_shift1_random_seed1_128.json \
  --output-json research/parameter_search/moment_lee_cpsat_g32_r42_seed1_128.json \
  --output-md research/parameter_search/moment_lee_cpsat_g32_r42_seed1_128.md \
  --timeout-s 120 \
  --workers 8
```

`INFEASIBLE` is an external certificate for the concrete block.  `FEASIBLE` or
`OPTIMAL` is a real counterexample.  `UNKNOWN` is not a certificate.

For a faster structural split, use `moment_lee_stratified`:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin moment_lee_stratified -- \
  --input-json research/parameter_search/vandermonde_lee_probe_g32_r42_shift1_random_seed1_128.json \
  --output-json research/parameter_search/moment_lee_stratified_g32_r42_seed1_anchored_128.json \
  --output-md research/parameter_search/moment_lee_stratified_g32_r42_seed1_anchored_128.md \
  --coeff-bound 3276 \
  --max-support 4 \
  --restarts-per-support 128 \
  --coordinate-rounds 4
```

This exhaustively certifies support 1 and then runs anchored coordinate-descent
attacks for larger supports.  Only support 1 is complete; larger-support rows
are counterexample searches.

For exact support 2:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin moment_lee_pair_exact -- \
  --input-json research/parameter_search/vandermonde_lee_probe_g32_r42_shift1_random_seed1_128.json \
  --output-json research/parameter_search/moment_lee_pair_exact_g32_r42_seed1_128.json \
  --output-md research/parameter_search/moment_lee_pair_exact_g32_r42_seed1_128.md \
  --threads 12
```

This is exhaustive over support exactly 2 and centered coefficient `l1` below
the target.  For the current `g=32,r=42` candidate it checked `10643068800`
assignments and found no below-target pair.

Validation keeps these reduced-view routes separate:

- `support-covered-needs-hitting-set`: exact structural compression.  The
  current Lean predicate is strong: the selected coordinates must cover every
  relevant secret-pair support.  This is likely useful only for structured
  selected views, not arbitrary tiny prefixes.
- `lossy-budget-candidate`: cheap diagnostics say the effective viewed noise
  budget and viewed-separation proxy are both satisfied.  These rows are the
  best targets for the next proof effort.
- `guard-block-lossy-candidate`: the selected prefix contains all guard
  coordinates, so guard-only separation plus the lossy viewed-noise budget gives
  the right shape for a reduced-view certificate.  It still needs the support
  distance to amplitude-separation bridge required by centered P-TND
  correctness.
- `guard-block-lossy-diagonal-candidate`: the guard-prefix lossy route also
  passes the current multi-coordinate diagonal-amplification screening proxy.
  This is close, but it is not accepted unless the exact validator can discharge
  the guard-block pair-coordinate certificate.
- `guard-block-lossy-diagonal-certified`: the selected prefix contains the full
  Cauchy guard block, the field is prime, the guard-block distance supplies the
  pair-coordinate support lower bound, the lossy viewed-noise budget passes,
  and the diagonal bad-seed inequality has nonnegative slack.  These rows pass
  the current reduced-view certificate boundary.
- `lossy-diagonal-candidate`: the same diagonal-amplification proxy passes for
  a non-guard-only lossy route.  These rows still need the corresponding primal
  distance certificate.
- `lossy-budget-needs-noise-or-separation`: the row is small, but it has not
  even passed the cheap budget screen.

`lossy-budget-candidate`, `guard-block-lossy-candidate`, and non-certified
diagonal candidate rows still fail final validation with
`view_reduction_certificate_missing` until the Lean `ViewReductionBudget`
obligations, or proof-carrying external evidence for those obligations, are
supplied.  The guard-block lossy-diagonal certified route is the currently
implemented reduced-view exception.

The `lossy_diagonal_required_distance` and `lossy_diagonal_slack_bits` columns
are screening diagnostics for the public-diagonal support-to-amplitude route.
They estimate whether the viewed support distance is large enough for a
multi-coordinate anti-concentration proof at the effective viewed noise radius.
Positive slack should guide the search; it is not by itself a security proof.
The proof-facing version is `LossyBudgetPairCoordinateStrategyObligations`,
which requires explicit nonzero viewed-coordinate witnesses for every ordered
secret pair plus the summed bad-seed product budget.  The compact version for
search output is `LossyBudgetUniformPairCoordinateStrategyObligations`, which
uses one uniform per-pair product budget.

Final KEM validation also enforces the q-ary amplitude condition used by the
abstract proof layer.  A row must satisfy:

```text
qary_amplitude_l1_certified >= ceil((q - 1) / (2 * effective_eta))
```

where `effective_eta` is the actual centered-noise radius after the selected
compression/view strategy.  If a row does not supply an exact
`qary_amplitude_l1_certified` value, the tooling falls back to the conservative
support-distance lower bound `n + 1`; this is proof-valid but usually far too
weak for compact candidates.  The intended next certificate is an exact lower
bound on the centered/Lee `l1` norm of every nontrivial q-ary dual relation,
not another low-Hamming search.

The search can now compare q-ary amplitude certificate strategies:

```bash
  --qary-amplitude-strategies fallback-support,solved-repetition,direct-solved-lee,shared-pair-lee
--lee-repetitions 1,2,4,8,16,256,512,1024
--m1-values 4,8,16,32,64
```

- `fallback-support`: the previous conservative bound `n + 1`.
- `solved-repetition`: models a proof-friendly solved map where each free
  coordinate is repeated through `lee_repetition` direct pivot-coordinate reads,
  matching the Lean theorem `modqNoLeeKernelVectorBelow_of_coordinate_reads`.
  It certifies only `lee_repetition + 1` and requires
  `lee_repetition * m1 <= n`, so it is mainly a baseline for pricing direct
  repetition and key-generation cost.  Rows report
  `qary_amplitude_certificate_rows` and `qary_amplitude_keygen_mul_ops` in JSON.
- `direct-solved-lee`: screening model for a proof-carrying systematic solved
  map `P` with certified graph Lee distance `lee_repetition`.  This is the
  abstraction recommended for future generated Lee expanders:
  `A^T = B0 [I | P]` and
  `min_{y != 0} centeredNormL1 q (-P y, y) >= lee_repetition`.  It prices
  dense solved-map keygen work as `n * m1` multiply-adds unless a later sparse
  evaluator model is added.
- `shared-pair-lee`: proof-carrying model for a sparse solved map where pivot
  rows are shared inside two-coordinate free blocks.  The search uses
  `lee_repetition` as the certified graph Lee distance, charges `n` certificate
  rows, and prices key generation as `2n` multiply-adds.  This strategy must be
  backed by a sidecar certificate from `shared_lee_cert`.

This direct repetition route is intentionally conservative.  If it is too weak,
the next family should use a stronger arithmetic Lee-expander certificate for
the solved map `B0^{-1} B1`, not merely more low-support dual screening.

The search and validator also report a first-principles geometry screen for the
public view.  For a viewed matrix with `N` public coordinates and rank `r`, the
q-ary relation lattice has determinant `q^r`, so Minkowski's `L1`-ball bound
gives a nonzero relation with approximately

```text
||z||_1 <= (N! * q^r)^(1/N)
```

If this upper bound is already below
`ceil((q - 1) / (2 * effective_eta))`, the row is rejected with
`qary_amplitude_minkowski_obstruction`.  This is a necessary-condition failure:
no seed choice or sparse-tilt certificate with the same public-view dimensions
can satisfy the no-narrow-dual amplitude requirement.  Rows that pass this
geometry screen still require view-only q-ary centered-`L1` reduction and exact
bounded Lee-dual follow-up.

The first construction-side certificate tool is:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin solved_lee_cert -- \
  --q 32771 \
  --n 32 \
  --m1 32 \
  --family optimized-scalar-block \
  --target-l1 3277 \
  --output-json research/parameter_search/solved_lee_cert.json \
  --output-md research/parameter_search/solved_lee_cert.md
```

It emits a proof-carrying sidecar for block-diagonal scalar solved maps where
each pivot row reads exactly one free coordinate with a recorded scalar.  The
certificate is exact for that family:

```text
min_j min_{x != 0 mod q} Lee(x) + sum_t Lee(a_{j,t} x)
```

This is currently a rigorous baseline and generator testbed, not yet the final
compact family.  It lets us measure how much centered-Lee expansion can be
obtained from certifiable scalar blocks and how much pivot/key-generation
budget that route consumes.

The first shared-row certificate tool is:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin shared_lee_cert -- \
  --q 32771 \
  --n 80 \
  --m1 32 \
  --rows-per-group 5 \
  --target-l1 3277 \
  --output-json research/parameter_search/shared_lee_cert.json \
  --output-md research/parameter_search/shared_lee_cert.md
```

It certifies direct sums of identical two-coordinate blocks.  Each block has
`rows_per_group` pivot rows, and each row reads both free coordinates.  The
verifier enumerates every centered pair whose free-coordinate `l1` is below the
target; if no pair has total graph Lee weight below the target, the block is an
exact local certificate for `shared-pair-lee`.  Because the global matrix is a
block-diagonal direct sum of those local blocks, the certificate composes into a
global `SystematicLeeDistanceAtLeast` proof through
`productBlockComposableSystematicLeeCertificate`, then into the public-view
Lee-dual certificate through `composableSystematicLeeDualCertificate`.
The preferred first-class strategy name for this route is now
`product-block-lee`; `shared-pair-lee` remains accepted as a compatibility
alias.
In search rows, `lee_repetition` is the certified Lee lower bound and
`lee_rows_per_group` is the `shared_lee_cert --rows-per-group` value.  The
modeled product-block pivot rows are:

```text
qary_amplitude_product_block_used_rows = lee_rows_per_group * (m1 / 2)
qary_amplitude_keygen_mul_ops = 2 * qary_amplitude_product_block_used_rows
```

This already improves on scalar blocks for the 128-bit toy shape:
`rows_per_group = 5` certifies the `q=32771, m1=32, target=3277` bound with
`n=80`, while scalar blocks needed `n=128`.  A concrete run is recorded at:

```text
research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.md
```

Independently verify a shared-row sidecar with:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin verify_shared_lee_cert -- \
  --input-json research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.json \
  --output-json research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.verify.json \
  --output-md research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.verify.md
```

This verifier recomputes every centered local pair in the declared domain and
checks that the artifact's pass flag, checked count, worst pair, and worst
relation agree.  For the recorded `q=32771, n=80, m1=32` sidecar, it checks
`21470904` pairs and reports actual minimum local Lee `l1 = 3283`, so the
artifact's `3277` bound is conservative.

When validating frontier rows that use `product-block-lee` or
`shared-pair-lee`, pass the matching sidecar explicitly:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin validate_frontier -- \
  --input-dir research/parameter_search/... \
  --output-dir research/parameter_search/..._validated \
  --shared-lee-cert-json \
    research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.json
```

Or pass a whole sidecar library:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin validate_frontier -- \
  --input-dir research/parameter_search/... \
  --output-dir research/parameter_search/..._validated \
  --shared-lee-cert-dir \
    research/parameter_search/product_block_lee_sidecars_20260515
```

Without the sidecar, validator rows using this strategy fail with
`shared_pair_global_certificate_missing`; this prevents the search model from
treating the product-block proof route as an unverified heuristic.

To generate a reusable sidecar library across field sizes and eta-derived
Lee thresholds, run:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin product_block_lee_sweep -- \
  --q-values 12289,32771 \
  --eta-values 5,6,8,10 \
  --m1 32 \
  --rows-per-group-values 3,4,5 \
  --trials 8 \
  --local-search-steps 8 \
  --seed 20260515 \
  --output-dir research/parameter_search/product_block_lee_sidecars_20260515
```

The sweep derives `target_l1 = ceil((q - 1) / (2 eta))`, searches random
two-coordinate local product blocks, optionally repairs the current worst
failing local pair for a bounded number of steps, writes exact passing sidecars,
and records a manifest.  This is the direct bridge from the local proof-object
construction to the frontier validator: search rows can lower
`lee_rows_per_group` exactly when the sidecar library contains the matching
local certificate.

The parameter search can consume the same manifest directly:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin descender-parameter-search -- \
  --profile descender-128 \
  --qary-amplitude-strategies product-block-lee \
  --product-block-sidecar-manifest \
    research/parameter_search/product_block_lee_sidecars_20260515/manifest.json \
  --m1-values 32
```

When this flag is present, product-block rows are generated from passing
manifest entries for the selected `(q, m1, eta)` rather than from the Cartesian
product of `--lee-repetitions` and `--lee-rows-per-group`.

For a more principled design sweep, use the random-code union-bound analyzer:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin lee_random_code_bound -- \
  --q 32771 \
  --target-l1 3277 \
  --m1 32 \
  --group-sizes 1,2,4,8,16,32 \
  --max-rows 80 \
  --failure-bits 80 \
  --output-json research/parameter_search/lee_random_code_bound.json \
  --output-md research/parameter_search/lee_random_code_bound.md
```

This treats each shared block as a random linear code in the Lee metric.  For
any fixed nonzero free vector, a random row dot product is uniform over `F_q`;
the tool combines exact Lee-ball volumes with the exact lower-tail distribution
of row Lee sums.  It therefore gives a principled row budget before we spend
time on deterministic sidecar search.  For the same 128-bit target, the 80-bit
union-bound sweep suggests:

```text
group size 8:  total rows 104
group size 16: total rows 66
group size 32: total rows 46
```

The tradeoff is verification: larger groups look better by the random-code
bound, but exact deterministic sidecar checking becomes much harder.  The
current concrete sidecar is exact for group size 2; larger groups should use a
new verifier or a proof route that justifies rejection-sampled random blocks.

The first construction-side search tool for the stronger proof direction is:

```bash
cargo run --manifest-path tools/parameter-search/Cargo.toml \
  --bin systematic_lee_search -- \
  --q 65537 \
  --pivot-rows 48 \
  --free-cols 32 \
  --eta 10 \
  --families dense-random,block-random,toeplitz-scrambled,vandermonde-scrambled \
  --seeds 16 \
  --output-json research/parameter_search/systematic_lee_search.json \
  --output-md research/parameter_search/systematic_lee_search.md
```

It generates systematic public views `A = [I | P]` and probes the exact q-ary
relation surface `(-P y, y)`.  The output records the target Lee lower bound,
the best centered-`l1` relation found by unit/pair/random/annealing probes, and
the descriptor cost for the generated `P`.  A counterexample is definitive for
that generated matrix.  A survivor is only a search sidecar: it must still be
checked by view-only BKZ/LLL and an exact bounded Lee-dual certificate before it
can instantiate `SystematicLeeDistanceAtLeast` in Lean.

The bounded deterministic follow-up is:

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

This regenerates each selected `P` and exhaustively checks every nonzero free
vector in the declared bounded domain:

```text
support(y) <= max_support
|y_i| <= coeff_bound
||y||_1 < target_l1
```

`bounded_pass = true` means there is no bad systematic relation in that exact
domain.  It is a certificate sidecar with explicit scope, not yet a global
`SystematicLeeDistanceAtLeast` proof unless the checked domain covers every
possible nonzero `y` below the target.  When `--basis-dir` is supplied, the same
run also exports fplll/fpylll-compatible q-ary dual bases for
`{z : [I | P] z = 0 mod q}`, giving the concrete lattice-reduction handoff for
each checked survivor.

For a true global proof, the construction must expose composable structure.
Lean now provides `ComposableSystematicLeeCertificate`, which turns exact local
Lee-distance certificates plus a local-to-global dominance proof into a global
`SystematicLeeDistanceAtLeast` certificate.  The corresponding public-view
entry point is `composableSystematicLeeDualCertificate`.  This is the intended
upgrade path for shared-block or expander-lifted families: bounded search is
used to design and falsify blocks, while the final accepted family is proved by
local certificates that compose over all nonzero global free vectors.

## Primal-Distance Model

The `--primal-distance-models` flag controls where correctness distance is
charged:

```bash
--primal-distance-models guard-only,combined-proxy
```

- `guard-only`: current proven path.  Correctness distance is `m0 - n + 1`,
  and rows require `m0 >= n`.
- `combined-proxy`: experimental path.  Correctness distance is priced from the
  viewed combined measurement shape, roughly `view_coords - n + 1`, and rows
  require `view_coords >= n`.

`combined-proxy` is a search mode, not a completed theorem.  It lets us measure
the possible benefit of reducing `m0` before we invest in the combined-matrix
primal-distance proof.

To keep this mode near the real proof obligations, each row reports cheap
certification-risk diagnostics:

- `combined_rank_deficit`: how many Cauchy guard coordinates were removed below
  `n`.
- `combined_distance_gap`: how much of the correctness distance must be supplied
  by the combined matrix beyond the guard-only theorem.
- `combined_certification_margin`: combined correctness slack after paying for
  that gap.
- `combined_certification_pressure`: the objective penalty input, equal to the
  rank deficit plus the distance gap.

The search objective subtracts
`--combined-certification-weight * combined_certification_pressure`, defaulting
to `0.25`.  This is intentionally cheap: it nudges annealing and frontier
ranking toward rows that should be easier to certify, while the validator still
marks every `combined-proxy` row with
`combined_primal_distance_certificate_missing` until an exact matrix-specific
certificate exists.

Use `--m0-values` when exploring `combined-proxy`; it supplies direct guard
sizes and can include `m0 < n`.  When `--m0-values` is omitted, the legacy
`--m0-overheads` mode is used and always sets `m0 = n + overhead`.

The current compressed canonical sweep is in
`research/parameter_search/bandwidth_canonical_compressed/`, with expanded
validation in
`research/parameter_search/bandwidth_canonical_compressed_expanded_validation/`.
The best validated rows are:

| profile | q | n | m0 | m1 | k | eta | sample bits | round err | effective eta | pk bytes | ct bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| descender-128 | 32771 | 32 | 52 | 32 | 24 | 2 | 13 | 3 | 5 | 201 | 169 |
| descender-192 | 16411 | 56 | 88 | 56 | 32 | 2 | 11 | 5 | 7 | 262 | 230 |
| descender-256 | 65537 | 64 | 104 | 64 | 48 | 2 | 12 | 9 | 11 | 316 | 284 |

Compared with the uncompressed canonical rows, this reduces public key and
ciphertext sizes from `224/192`, `304/272`, and `384/352` bytes respectively to
`201/169`, `262/230`, and `316/284` bytes.

## Coefficient Representation

The `--coefficient-model` flag controls the public-key accounting for wedge
fiber coefficients:

- `explicit`: store both local fiber vectors for every wedge.
- `seeded`: store a public coefficient seed and a small deterministic rule
  descriptor; the verifier recomputes coefficients and checks local
  independence.

The seeded model is the first key-size optimization lever.  It preserves the
transparent setup requirement while removing the dominant
`m1 * fiber_dim * log(q)` public-key term from the model.

## Incidence Representation

The `--incidence-model` flag controls the public-key accounting for selected
wedge endpoints and diagonal tilt data:

- `explicit`: store endpoint metadata and diagonal values per wedge.
- `seeded`: store public thinning and diagonal seeds plus deterministic rule
  descriptors; the verifier reconstructs the selected wedges and diagonal.

This is the second key-size optimization lever.  It preserves the requirement
that setup be reproducible and auditable, but moves the cost from public key
bytes into deterministic setup verification.

The current metrics are still screening proxies, not security estimates.  The
next Rust additions should be exact rank checks, projected-distance checks, and
bounded dual-witness enumeration.
