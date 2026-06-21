# WS4 #11 — Empirical Leakage Battery over REAL Signatures (LEAK FOUND)

> MIGRATION STATUS (2026-06-20): `NEGATIVE_CONTROL`.
> This records leakage evidence against historical transcript paths. It is
> retained as a regression and review seed, not as current deployable status.
> Current signer and leakage requirements live in `../../spec/signer_profile.md`.

Created 2026-06-10. RISK-FIRST: this is a **break / redesign signal**, reported
loudly. The statistical witness-hiding of REAL `reference_sign_with_proof`
transcripts does **not** hold against the deployed signer: the public signature
openings reveal the secret-derived pad table **exactly** (0 hiding bits for the
pad witness), directly contradicting the theoretical
`residual_hiding_meets_140` (>=140 hiding bits) claim **as it applies to the
deployed transcript**.

## What was run (real, reproducible)

- **Harness:** new `--leak-sample` subcommand in
  `tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`
  (function `run_leak_sample`, just above `main`). For a FIXED keypair/witness it
  signs varied messages over multiple slots via the REAL
  `reference_sign_with_proof`, and dumps decoded transcript components plus the
  KNOWN ground-truth witness (the 23 secret full-response residues and the 16x2
  secret pad table per slot).
- **Corpus:** `research/cryptanalysis/leakage_corpus.jsonl` — **100,000 real
  signatures**, 4 slots (25k/slot), 60 MB, generated in ~11 s.
- **Battery:** `tools/parameter-search/leakage_battery.py` (numpy/scipy). Tests
  A–E below; 13 collected p-values with Bonferroni + Benjamini-Hochberg
  correction.

Reproduce:

```
cargo build --release --bin signature_q12289_local_delta_stateful   # in tools/parameter-search
./tools/parameter-search/target/release/signature_q12289_local_delta_stateful \
    --leak-sample --count 100000 --slots 4 --out research/cryptanalysis/leakage_corpus.jsonl
python3 tools/parameter-search/leakage_battery.py research/cryptanalysis/leakage_corpus.jsonl
```

## Results

### (D) Witness-correlation / key-recovery — THE LEAK
The public signature **openings** are `open[rep,coord] = (12 or Q-12) +
pad(seed, block[rep])[coord] mod Q`, a **deterministic** function of the secret
pad seed and the challenge-selected block. Over the corpus, **all 128 observed
secret pad cells (16 blocks x 2 coords x 4 slots) were reconstructed bit-for-bit**
(`recovered 32/32` per slot, exact=True for every slot).

- Pearson(`open - mask`, `true_pad`): **r = 1.000000, p ≈ 0** over 5,200,000
  paired observations. BH-q = 1.18e-300 (survives correction by ~300 orders of
  magnitude).

These pad outputs are exactly the **"active pad outputs"** that the existing
HIGH-severity caveat at `stateful.rs:6459` says must be **EXCLUDED** from the
zero-TV hiding sampler:
> "the zero-TV hiding certificate is valid only for the allowed conditioned
> quotient-fiber sampler; conditioning on active pad outputs would invalidate
> it" (`masked_response_sampler_certificate.active_pad_outputs_conditioned`
> is false; `excluded_conditioning` lists "active pad outputs", "active local
> openings", "challenge-selected block pad equations").

The deployed `reference_sign_with_proof` (stateful.rs:4724-4747) **emits those
excluded quantities in the clear** in the `LocalOpenings`. So the transcript the
verifier and any observer sees publishes the witness-bound pad directly. The
empirical hiding for the pad witness is **0 bits**, not >=140.

### (C) 3x2 repeated local-block-structure — STRUCTURAL FINGERPRINT
Within a single signature, whenever the challenge selects the same block in two
reps, the openings are **byte-identical** (deterministic pad): **39,091/39,091**
intra-signature repeated-block pairs identical (100%). The repeated-local-block
structure flagged in the cryptanalysis plan is fully present and observable; it
links reps to the same block and accelerates pad-table read-off.

### (E) Response determinism — no extra leak, but no hiding either
The 52 signed response residues are an **EXACT deterministic public function**
of (challenge, `public_free`): `resp[rep,coord] = center(mask[coord] +
sign[rep]*public_free[block[rep]][coord])` reproduced exactly for all 100k
signatures. Responses leak nothing beyond the public key (anyone can recompute
them) — but they also carry no masking randomness, so there is no statistical
hiding margin in the responses to begin with.

### (A) Challenge uniformity — clean
Challenge block selection chi-square chi2=17.7, dof=15, **p=0.279** (N=2.6M);
sign bit chi2=0.02, p=0.893. The Fiat–Shamir challenge is statistically uniform
— the leak is **not** in the challenge; it is in the deterministic openings.

### (B) Autocorrelation — significant, but explained by the codebook structure
Lag-1..5 autocorrelations of the opening streams are all p≈0 (z up to ~94).
This is a real, correction-surviving signal, but its mechanism is the same
determinism: openings take only 16 distinct (per coord) values from the secret
pad codebook, with intra-signature repeated blocks, so the stream is far from
white noise. It is a *consequence* of (D)/(C), not an independent channel.

## Multiple-testing summary
13 hypotheses, Bonferroni alpha = 0.05/13 = 3.8e-3. **11 hypotheses are
BH-significant**: the opening↔pad correlation and all 10 opening
autocorrelations (BH-q ~ 1e-300). The two challenge-uniformity tests are
correctly **non**-significant.

## Power / scope honesty
- 100,000 real signatures, 25,000 per slot. The leak is **deterministic**, so it
  is not a marginal statistical effect requiring large N — even a handful of
  signatures per block exposes the pad cell. With 25k/slot every one of the 16
  blocks is hit thousands of times, so all 128 cells are observed and the
  recovery is exhaustive, not extrapolated.
- The witness-correlation test uses **ground-truth** secret pad/witness emitted
  by the harness META record, so r=1.0 is a verified exact match against the real
  secret, not a proxy.
- Caveat on test (B): autocorrelation p-values use the white-noise z≈r·sqrt(N)
  approximation and reps within a signature are not independent; the p-values are
  directionally valid (strongly non-white) but the magnitude is inflated by
  intra-signature dependence. The headline finding (D) does not rely on it.

## Verdict vs `residual_hiding_meets_140`
**LEAK = redesign signal.** The theoretical certificate
(`MaskedResponseSamplerCertificate`, `statistical_distance_is_zero = true`,
`residual_hiding_meets_140 = true`) is certified **only for the conditioned
quotient-fiber sampler that EXCLUDES active pad outputs**. The deployed signer
publishes the active pad outputs in the openings, so the certificate's
precondition is violated by the real transcript. The
`required_closure` at `stateful.rs:6464` — "enforce the allowed rejection
predicate in the production signer API and add tests that active pad equations
are checked outside the hidden representative sampler" — is **NOT met** by
`reference_sign_with_proof` / `production_sign`.

## Redesign implication
The openings must not expose the witness-bound pad in the clear. Either (a) the
pad must be re-randomized per signature (the current seed is fixed per slot, so
the pad table is static and fully read-off after one pass over the blocks), or
(b) the active-pad/opening equations must be moved into a hidden,
commitment-bound check (the certificate's "existential quotient-opening
predicate") so the verifier checks them without the signer publishing the pad,
or (c) the transcript must publish only blinded openings whose distribution is
the conditioned fiber law the cert actually covers. Until then the
`>=140 hiding bits` claim does not describe the deployed transcript.

## Artifacts
- Harness: `tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`
  (`run_leak_sample`, `--leak-sample`).
- Battery: `tools/parameter-search/leakage_battery.py`.
- Corpus: `research/cryptanalysis/leakage_corpus.jsonl` (100k real signatures + META).
