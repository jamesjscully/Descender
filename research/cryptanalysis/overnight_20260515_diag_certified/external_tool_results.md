# External Tool Results

Generated on 2026-05-15 from the exported validator-passing rows in this
dossier.

## Environment

- Built Docker image: `descender-lattice-estimator`
- Base image: `sagemath/sagemath:9.5`
- `lattice-estimator` source: `tools/external/lattice-estimator`
- `fpylll` available inside the Sage image: `0.5.6`

Host Python does not currently have `pip` or `venv`; the external tools were
therefore run through Docker rather than installed into the host Python.

## Commands

```bash
docker build -t descender-lattice-estimator \
  -f tools/external/lattice-estimator/docker/Dockerfile.dev \
  tools/external/lattice-estimator

docker run --rm -v "$PWD:/work" \
  -w /work/research/cryptanalysis/overnight_20260515_diag_certified \
  --entrypoint /bin/bash descender-lattice-estimator \
  -lc 'PYTHONPATH=/lattice-estimator sage -python run_lattice_estimator_jobs.py' \
  | tee research/cryptanalysis/overnight_20260515_diag_certified/lattice_estimator_proxy_output.txt

docker run --rm -v "$PWD:/work" \
  -w /work/research/cryptanalysis/overnight_20260515_diag_certified \
  --entrypoint /bin/bash descender-lattice-estimator \
  -lc 'sage -python run_fpylll_jobs.py' \
  | tee research/cryptanalysis/overnight_20260515_diag_certified/fpylll_bkz20_output.txt
```

## Summary

The first-pass `lattice-estimator` run used the deliberately conservative
LWE-like proxy emitted by `run_lattice_estimator_jobs.py`.  That proxy is not a
security proof for Descender, but it is useful as an early attack-model warning.
It reports low costs for every row:

| profile | best proxy rop |
|---|---:|
| descender-128 rows | about `2^34.6` |
| descender-192 rows | about `2^35.5` |
| descender-256 rows | about `2^37.6` to `2^38.0` |

This should be treated as a blocker for the current candidate family until the
Descender-to-LWE/BDD/SIS mapping is either corrected, rejected as inapplicable,
or replaced by a faithful attack model.

The concrete `fpylll` sanity run reduced the exported q-ary dual lattice bases
with LLL followed by BKZ-20.  It found short first vectors in every exported
dual lattice:

| row | dimension | BKZ-20 first squared norm |
|---|---:|---:|
| `0000` | 112 | 7939 |
| `0001` | 112 | 8217 |
| `0002` | 112 | 9137 |
| `0003` | 112 | 10115 |
| `0004` | 136 | 32897 |
| `0005` | 136 | 32643 |
| `0006` | 136 | 35860 |
| `0007` | 136 | 32961 |
| `0008` | 160 | 149447 |
| `0009` | 160 | 145963 |
| `0010` | 160 | 166847 |
| `0011` | 184 | 138583 |

These reductions do not by themselves identify a full key-recovery or
decapsulation attack, but they are concrete evidence that the dual-lattice
surface needs a faithful attack interpretation before any KEM claim.

The reduced vectors were then converted back into Descender public coordinates
in `reduced_dual_vectors_bkz20.json` and summarized in
`reduced_dual_vectors_bkz20.md`.  In all 12 rows:

- the reduced vector satisfies the exported adjoint constraint modulo `q`;
- the vector is dense in Descender coordinates, with support from `103` to
  `179`;
- the support is split across both guard and tilt coordinates;
- no vector violates the recorded low-Hamming dual certificate boundary, which
  only excluded supports up to `3` in these rows.

So the BKZ-20 vectors are not direct counterexamples to the current
low-Hamming proof certificate.  However, they are direct distinguishers for the
literal public sample game `t = A s + e` versus uniform whenever the noise is
small centered coordinate noise.

This is recorded in `direct_dual_distinguisher.md` and
`direct_dual_distinguisher.json`.  The direct test is:

```text
given z with A^T z = 0 mod q,
real sample:   <z, A s + e> = <z,e> mod q
uniform null:  <z,u> is uniform mod q
accept real if centeredAbs(<z,t>) <= eta * ||z||_1
```

For all 12 rows, the real acceptance probability is 1 under the stated
centered noise envelope, while the uniform acceptance probability is only about
`0.0146` to `0.0636`.  This gives distinguishing gaps from about `0.936` to
`0.985`.

That means the current rows should be treated as broken for the literal
`A s + e` vs uniform decision game.  The proof remains true as a statement
about low-Hamming dual witnesses, but that statement is too weak for this KEM
sample security goal.

## Next Cryptanalysis Tasks

1. Stop treating bounded Hamming dual search as a sufficient security filter.
2. Add a q-ary dual-amplitude filter: reject any public dual vector `z` with
   `eta * ||z||_1 << q`, because it gives the direct distinguisher above.
3. Decide the repair path:
   - make the public view have no short-L1 q-ary duals;
   - add enough noise to wrap modulo `q` along all public duals;
   - or change the KEM/security game so this statistic is unavailable.
4. Rerun parameter search only after this direct distinguisher condition is a
   hard validator constraint.
