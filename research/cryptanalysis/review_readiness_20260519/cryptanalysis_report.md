# Frozen q12289 Cryptanalysis Report

Date: 2026-05-19

Status: internal review artifact for the frozen
`validated-q12289-n48-m1-32-single-carrier` target.

## Scope

This report covers the cryptanalysis gate items tied to A4 and A9 for the
frozen q12289 verifier target:

- exact bad-vector/witness search on the pinned repeated `3 x 2` block;
- lattice reduction on the exact q-ary kernel basis for that frozen block;
- search/MITM interpretation for the repeated-block structure;
- transcript malleability and parser-side mutation checks against the frozen
  verifier relation;
- a concrete attack table with cost, memory, data, confidence, and caveats.

It does not claim deployment closure for key generation or signing.  Those
limitations are stated explicitly below.

## Artifact Inventory

| artifact | purpose |
|---|---|
| `research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.json` | pinned frozen local-block sidecar |
| `frozen_q12289_shared_lee_verify.json` / `.md` | independent rerun of the frozen sidecar verification |
| `frozen_q12289_exact_search.py` + `.json` / `.md` | exact local bad-vector search and derived multi-block bounds |
| `frozen_q12289_lattice_probe.py` + `_bkz20.json` / `.md` | exact q-ary kernel basis reduction with LLL+BKZ-20 |
| `frozen_q12289_conformance_rerun.json` | fresh verifier-generated positive/negative conformance bundle |
| `frozen_q12289_public_key_seed_mutation_verify.json` | canonical public-key seed-byte mutation rejection |
| `frozen_q12289_message_mutation_verify.json` | message mutation rejection |
| `frozen_q12289_sidecar_rerun/` | family-level rerun of sidecar generation; not the pinned coefficients |
| `*.time.txt` | archived runtimes for the bounded checks |

## Results

| attack class | method | parameters | result | cost / memory / data | confidence | caveats |
|---|---|---|---|---|---|---|
| bad-vector / keygen-break on frozen coefficients | exact exhaustive local search | pinned sidecar coefficients `[[10849,1171],[1597,1615],[6102,11189]]`; `q=12289`; `16` repeated blocks; all nonzero centered pairs with local free `l1 < 512` | no counterexample below `512`; exact minimum is `533` at pair `(-78,-75)` with relation `[-83,89,-208,-78,-75]` | `523264` local pairs; `real 3.73s`; negligible memory; public matrix only | high | this closes the search only for the pinned repeated block, not for a future seeded/distributional keygen |
| repeated-block MITM / structured search | direct-sum reduction from local block to global witness cost | identical `3 x 2` block repeated across `16` groups | best `h`-block witness has cost at least `h * 533`; in particular `2` blocks already force `1066 > 512` | analytic; negligible memory; no extra data | high | relies on the current direct-sum structure; any structural keygen change invalidates it |
| lattice reduction on frozen target | exact q-ary kernel basis, LLL then BKZ-20 | `80 x 80` basis for the frozen systematic kernel lattice; BKZ block size `20` | first reduced vector is the same single-block witness with centered `l1 = 533`, support `5`; no sub-`512` vector was found in this run | `real 0.21s`; negligible memory; public matrix only | medium-high | BKZ-20 is a bounded sanity check, not a full sieving campaign; stronger BKZ or enum could still be run |
| transcript malleability: challenge / response / parser | verifier-generated conformance negatives plus canonical message/pk mutations | fresh conformance rerun on frozen verifier; extra checks flip one public-key seed bit and one message bit | all tested mutations reject; reasons are canonicality failure, range failure, or recomputed `challenge mismatch` | conformance rerun `real 0.00s`; negligible memory; one positive vector plus targeted mutations | high for tested surfaces | archived tests do not cover a full signing oracle, repeated randomness, or SHA-256 collision-style attacks |
| family-level bad-vector rerun | rerun sidecar generator for `q=12289`, `target=512`, `rows/group=3` | `product_block_lee_sweep --q 12289 --target-l1 512 --m1 32 --rows-per-group 3 --trials 16 --seed 1` | another passing block was found at trial `15`; this supports the sidecar family but is not the frozen coefficient set | one bounded rerun; negligible memory; no secret data | low-medium | not evidence about the exact frozen block, so it should not be used to replace the pinned sidecar evidence |

## Transcript Malleability Notes

Archived verifier-side failures now cover:

- noncanonical challenge high bits;
- claimed challenge mutation;
- response mutation with unchanged challenge;
- out-of-range response coordinate;
- noncanonical public-key q-coordinate;
- truncated signature;
- canonical public-key seed mutation;
- canonical message mutation.

For the last two, the verifier recomputes a different hash frame SHA-256 and
rejects with `challenge mismatch`:

- mutated public-key seed byte:
  `7684608687808cea01477990ea6027367243dc6d1260de691b55ebf7ffd6f044`
- mutated message byte:
  `ba5e4f55a36108a8bd27541ec23237daf0ff438b269138998b8d704dd69a8fe3`

This is evidence that the frozen verifier binds the full serialized public key,
message, and reconstructed commitments into the challenge point.  It is not yet
evidence about signer misuse cases, because there is no frozen signer.

## Concrete Security Interpretation

The frozen target currently has two distinct security stories:

1. For the pinned repeated local block, the exact witness search and BKZ-20
   rerun both land at the same minimum centered-`l1` relation cost `533`,
   which stays above the certified threshold `512` by a margin of `21`.
2. For the full signature scheme, the formal `2^-133` query-miss term only
   applies after the verifier and keygen assumptions are satisfied.  The
   archived cryptanalysis here does not upgrade that into a deployable EUF-CMA
   claim because key generation and signing remain open.

So the correct concrete claim today is narrow:

- the pinned frozen public-view block survives the bounded exact search,
  direct-sum MITM reasoning, and a quick BKZ-20 reduction;
- the frozen verifier rejects the documented transcript and parser mutations;
- deployment-level signature security remains conditional on unclosed keygen and
  signer obligations.

## Known Limitations

- The largest remaining gap is still A4: no frozen keygen algorithm is closed.
  Current evidence is for the exact archived block, not for a seed-expansion or
  rejection-sampling distribution.
- The lattice work is bounded.  BKZ-20 was run because it is cheap and exact on
  the frozen basis, but there is no BKZ-30/40, progressive-BKZ, enumeration, or
  sieving archive yet.
- There is still no frozen signing algorithm.  That means no repeated-message,
  repeated-randomness, or multi-signature leakage experiment against a real
  signer can be claimed complete.
- Transcript attack coverage is verifier-side only.  It does not address
  side-channel leakage, RNG failures, or standard-model hash assumptions.
- The family-level sidecar rerun found a different passing block.  That is only
  corroboration for the search machinery; it does not substitute for evidence on
  the pinned coefficients.
- Any change to coefficients, encoding, hash domain, frame fields, or response
  bound invalidates these results and requires a new archive.

## Next Long Runs

If more lattice depth is wanted before external review, the next exact frozen
commands are:

```bash
python3 research/cryptanalysis/review_readiness_20260519/frozen_q12289_lattice_probe.py \
  research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.json \
  30 \
  research/cryptanalysis/review_readiness_20260519/frozen_q12289_lattice_probe_bkz30.json \
  research/cryptanalysis/review_readiness_20260519/frozen_q12289_lattice_probe_bkz30.md

python3 research/cryptanalysis/review_readiness_20260519/frozen_q12289_lattice_probe.py \
  research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.json \
  40 \
  research/cryptanalysis/review_readiness_20260519/frozen_q12289_lattice_probe_bkz40.json \
  research/cryptanalysis/review_readiness_20260519/frozen_q12289_lattice_probe_bkz40.md
```

If a seeded keygen is introduced, the minimum required reruns are:

```bash
cargo run --release --manifest-path tools/parameter-search/Cargo.toml --bin verify_shared_lee_cert -- \
  --input-json <new-sidecar.json> \
  --output-json research/cryptanalysis/review_readiness_20260519/<new-sidecar>.verify.json \
  --output-md research/cryptanalysis/review_readiness_20260519/<new-sidecar>.verify.md

python3 research/cryptanalysis/review_readiness_20260519/frozen_q12289_exact_search.py \
  <new-sidecar.json> \
  research/cryptanalysis/review_readiness_20260519/<new-sidecar>.exact.json \
  research/cryptanalysis/review_readiness_20260519/<new-sidecar>.exact.md
```
