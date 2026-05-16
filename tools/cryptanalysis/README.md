# Descender External Cryptanalysis Harness

This directory is the bridge from Descender's internal parameter validator to
standard external cryptanalysis tooling.

The current workflow is:

1. Generate candidate rows with `tools/parameter-search`.
2. Validate proof-side and finite-field conditions with `validate_frontier`.
3. Export passing rows with `descender_cryptanalysis.py`.
4. Run external estimators/reduction tools from the exported job files.

The exporter does not make security claims.  It creates reproducible handoff
artifacts and marks every off-the-shelf estimator mapping that still needs
human cryptanalysis review.

## Export Validated Rows

```bash
tools/cryptanalysis/descender_cryptanalysis.py \
  --validation-json \
    research/parameter_search/overnight_20260515_big/diag_targeted/descender-128_validation_certified/validation.json \
    research/parameter_search/overnight_20260515_big/diag_targeted/descender-192_validation_certified/validation.json \
    research/parameter_search/overnight_20260515_big/diag_targeted/descender-256_validation_certified/validation.json \
  --output-dir research/cryptanalysis/overnight_20260515_diag_certified \
  --limit-per-profile 4
```

Outputs:

- `README.md`: ranked candidate dossier.
- `candidates.json`: normalized rows and margins.
- `estimator_jobs.json`: mappings for external estimator drivers.
- `matrices/*.adjoint_modq.json`: regenerated public adjoint matrices.
- `fplll/*.dual_qary_basis.txt`: integer bases for
  `{x : A^T x = 0 mod q}`.
- `run_fpylll_jobs.py`: optional fpylll sanity runner.
- `run_lattice_estimator_jobs.py`: optional lattice-estimator proxy runner.

## External Tools

Recommended tools:

- `malb/lattice-estimator` for first-pass LWE/BDD/SIS-style attack estimates.
- `fplll` / `fpylll` for concrete LLL/BKZ reduction experiments on exported
  q-ary lattice bases.
- `fplll/g6k` for heavier sieving/BKZ experiments once candidates stabilize.
- `Crypto-TII/CryptographicEstimators` for code/syndrome-decoding style
  estimates when a faithful mapping is identified.

Run the setup helper:

```bash
tools/cryptanalysis/setup_external_tools.sh
```

The helper intentionally keeps external tools outside the core build.  Some
packages, especially `lattice-estimator`, may require Sage or system libraries;
the generated job files remain useful even when the local environment is not
ready to run every tool.

## Q-ary Lee-L1 Dual Checks

The KEM-facing amplitude condition is a Lee/centered-`l1` condition on q-ary
dual relations, not a Hamming condition.  The direct decision problem is:

```text
exists z != 0:
  A z = 0 mod q
  centered_l1(z) < ceil((q - 1) / (2 * effective_eta))
```

Use the fast probe first:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml \
  --bin qary_l1_dual_probe -- \
  --input research/parameter_search/.../validation.json \
  --output research/parameter_search/.../qary_l1_dual_probe.json \
  --top 4 --coeff-bound 32 --random-samples 500000 \
  --max-random-sparsity 24 --anneal-restarts 256 --anneal-steps 50000
```

This is a falsification tool.  A found vector is a real direct-dual
counterexample; failure to find one is not a proof.

On the proof side, `Descender.SystematicLee` isolates the algebraic certificate
shape used by the generated matrices.  After choosing an invertible pivot block,
the q-ary kernel is represented as `y -> (-P y, y)`.  Proving a Lee lower bound
for every nonzero `y` in that systematic map is enough to discharge
`ModqNoLeeKernelVectorBelow`, which then feeds the q-ary amplitude theorem.
Lean also proves the useful pivot-expansion form:
`modqNoLeeKernelVectorBelow_of_systematic_pivot_expansion` turns a lower bound
on `centeredNormL1 q (P y)` into a full systematic relation bound with the
additional `+1` contribution from the nonzero free coordinate.
There is also a direct-coordinate certificate,
`modqNoLeeKernelVectorBelow_of_coordinate_reads`, for setups where generated
pivot rows explicitly read free coordinates.  Use it only when the generated
matrix artifact records those exact coordinate-read rows; generic Cauchy-solved
pivot matrices still need a real Lee-expansion proof or exact solver
certificate.
The support bridge
`modqNoLeeKernelVectorBelow_of_systematic_support_expansion` is available, but
it only converts Hamming/support expansion into a Lee lower bound of the same
size.  That is not enough for the current amplitude thresholds; useful
candidate validation needs an arithmetic Lee certificate for `B0^{-1} B1`, not
only absence of low-support q-ary relations.
For the shifted Cauchy part, Lean proves the duplicate-column identity through
`modqNoLeeKernelVectorBelow_of_duplicate_solved_block`: duplicate rest columns
become coordinate reads after solving with a pivot left inverse.  Current
candidate hardness therefore hinges on the remaining sparse-tilt arithmetic
inside `B0^{-1} * tilt`, which must be certified exactly or proved by a
stronger family theorem.

The exact solver adapters ask the same decision problem through generic
integer solvers:

```bash
python3 tools/cryptanalysis/qary_l1_z3_check.py ...
python3 tools/cryptanalysis/qary_l1_cpsat_check.py ...
```

`unsat`/`INFEASIBLE` is a usable amplitude certificate.  `sat`/`FEASIBLE` is a
counterexample.  `unknown` only means the generic solver did not resolve the
row; it should not be treated as security evidence.

## Interpretation Rules

- A passing `validate_frontier` row means the current Descender proof-side
  certificate boundary accepts the row.
- A passing external estimator run is only evidence under the stated mapping.
- Any short vector, low BKZ block-size break, or unexpectedly cheap estimator
  result blocks the candidate until the attack model is understood.
- The final KEM selection should require at least:
  - proof-side validation,
  - no low-weight dual witness at a substantially higher search depth,
  - lattice-estimator estimates above target with margin,
  - concrete fplll/fpylll sanity reductions on generated bases,
  - a documented decision for whether code/syndrome estimators apply.
