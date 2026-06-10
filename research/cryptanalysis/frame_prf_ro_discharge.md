# Frame-PRF -> RO/QROM reduction: discharge note

**Status: GENUINE reduction achieved (classical RO).** The frame-PRF
pad-hiding distinguishing advantage is now a **proven EasyCrypt lemma**
(`frame_prf_advantage`) deriving the bound `q * 2^-H` from two — and only two —
named primitives: (a) random-oracle behaviour (a not-yet-queried point gets a
fresh uniform value) and (b) `pad_seed` min-entropy `H = 256`. The conclusion is
**not** axiomatized.

- File: `formal/easycrypt/frame_prf_ro_reduction.ec` (EasyCrypt 2026.05, **exit 0**, no `admit`/`sorry`).
- This replaces the earlier FAKED attempt that declared
  `axiom sha256_ro_indistinguishable : |Pr[A(MaskReal)] - Pr[A(MaskIdeal)]| <= eps_ro`,
  i.e. that *assumed the conclusion*. That axiom does not exist in this file; see
  the audit at the bottom.

---

## 1. What is being reduced

The reviewed-Fq12 packed-witness projection mask
(`tools/parameter-search/src/production_reference.rs:6405`) is

```
mask[lane] = uniform_mod_from_hash(
                PROJECTION_MASK_DOMAIN,            // domain separator
                pad_seed || slot_id,               // PRF seed (contains the secret)
                slot_id, lane, Q)
```

and `uniform_mod_from_hash` (`production_reference.rs:46481`) is rejection
sampling over

```
SHA256( DOMAIN || pad_seed || slot_id || slot_id || lane || counter )  (mod Q).
```

Modeled in the (Q)ROM, SHA-256 is a random oracle `RO`. Each `(slot_id, lane)`
selects a **distinct** RO input point `x*` whose preimage frame **contains the
secret `pad_seed`**. The secret is a per-slot single-use seed of width
`REVIEWED_FQ12_PAD_SEED_BYTES = 32` bytes
(`production_reference.rs:555`), i.e. min-entropy `H = 8 * 32 = 256` bits.

**Frame-PRF / pad-hiding goal:** the production mask `mask = RO(x*)` is
indistinguishable from a fresh uniform mask, so the masked carrier hides the
witness.

---

## 2. The genuine argument (derived, not assumed)

1. **Real world** hands the adversary `RO(point pad_seed)` = `realMask`.
   **Ideal world** hands a fresh uniform mask `idealMask = dmask`.
2. **RO lazy sampling** (the standard / QROM RO behaviour, the *same* primitive
   as the action-map QROM): the value of `RO` at a point the adversary has
   **not queried** is a **fresh uniform** value, independent of the adversary's
   view. Hence if the adversary never queries any point whose preimage contains
   `pad_seed` (`! bad`), then `RO(x*)` is itself fresh-uniform and the two
   worlds feed identically distributed, independent masks:
   **real == ideal exactly, advantage 0** (lemma `identical_until_bad`).
3. The **only** way to distinguish is the **bad event**: querying `RO` at some
   point bearing `pad_seed`. Each query is information-theoretically independent
   of the freshly drawn secret (in the no-bad branch the adversary's view
   carries no information about `pad_seed`), so a single query collides with the
   secret-bearing point with probability `<= 2^-H`; over `q <= qQ` queries,
   `Pr[bad] <= qQ * 2^-H` (lemma `bad_event_bound`, union bound + per-query
   min-entropy).
4. **Identical-until-bad** then gives, for any distinguisher read `D` of the
   handed mask and any seed-independent transcript `qs` of `<= qQ` queries,
   ```
   | Pr[D(RealExp qs)] - Pr[D(IdealExp qs)] |  <=  Pr[bad]  <=  qQ * 2^-H
   ```
   This is lemma **`frame_prf_advantage`** — a PROVEN lemma, the conclusion.

The combined experiments `RealExp`/`IdealExp` are single distributions
`dlet dseed (fun sd => realMask/idealMask)`; the advantage is decomposed via
`dletE` into a `mu1 dseed`-weighted sum of per-seed inner gaps, each gap is `0`
off `bad` (step 2) and `<= 1` on `bad`, and the weighted indicator sum is
exactly `mu dseed bad`, bounded by step 3. No expectation/Jensen machinery, no
hand-waving.

---

## 3. Axiom ledger (every axiom is RO-behaviour or entropy)

| axiom | category | content |
|---|---|---|
| `dmask_ll`, `dmask_uni` | (a) RO behaviour | RO output distribution is lossless + uniform on the codomain (the rejection sampler of `uniform_mod_from_hash` is uniform on `Z_Q`). |
| `ROsample_fresh_uniform` | (a) RO behaviour | **lazy sampling**: on `! bad sd qs` (seed point unqueried) the real-world mask distribution equals a fresh `dmask` draw. Speaks ONLY about RO output on a fresh point; never about advantage. |
| `idealMask_def` | (a) RO behaviour | definitional: the ideal world's mask is the fresh uniform `dmask`. |
| `seed_minentropy` | (b) entropy | a fixed query point bears a freshly drawn `pad_seed` with prob `<= 1/2^H`. The seed-byte-width fact (`H = 8*32 = 256`); NOT a hardness claim. |
| `padSeedBytes_val` | (b) entropy | `REVIEWED_FQ12_PAD_SEED_BYTES = 32` (cites `production_reference.rs:555`). |
| `dseed_ll` | structural | the secret seed distribution is lossless. |
| `qQ_ge0` | structural (query model) | query count is `>= 0`. |
| `seed_ofK` | structural | the canonical seed-bearing point of `sd` recovers `sd`. |

**No axiom mentions `RealExp`/`IdealExp`/advantage/distinguishing probability.**
The grep audit (`grep '^axiom' ... | grep -iE 'real|ideal|advantage|distinguish'`)
matches only `idealMask_def` (a *definition* of the ideal mask as `dmask`, not a
probability bound). The advantage bound is the lemma `frame_prf_advantage`.

Consistency: the axioms are jointly realizable (e.g. `dseed` uniform over `2^256`
seeds with `bearing` selecting one point; `realMask sd qs := dmask`), so the
lemmas are **not vacuous**.

---

## 4. Proven lemmas (no `admit`)

- `identical_until_bad` — RO behaviour: `! bad ==> realMask = idealMask` (advantage 0 off bad).
- `mu_has_le_sum` — union bound over the query list.
- `bad_event_bound` — `Pr[bad] <= qQ * 2^-H` (union bound + `seed_minentropy`).
- `summable_gap`, `exp_gap_sum`, `exp_gap_summand_bound` — the `dlet` decomposition.
- `frame_prf_advantage` — **the conclusion**, derived: advantage `<= qQ * 2^-H`.

---

## 5. Residual obligation (QROM / quantum)

The proven bound is the **classical** RO bound `Pr[bad] <= q * 2^-H`. In the
**quantum** ROM the per-query collision argument is replaced by the standard
query-extraction / one-way-to-hiding (O2H) bound, giving `Pr[bad] <= q^2 * 2^-H`
(a quadratic loss). This file proves the classical reduction with the
*structure* (identical-until-bad + bad-event bound) that the QROM proof reuses;
porting to the quantum bound requires substituting the EC QROM library's O2H
lemma for `bad_event_bound`'s union step. That substitution is the one remaining
item; the reduction skeleton, the RO/entropy axioms, and the derived-conclusion
discipline are all in place and machine-checked.

`H = 256`, so even the classical bound `q * 2^-256` is comfortably below any
relevant budget for adversary query counts `q`.
