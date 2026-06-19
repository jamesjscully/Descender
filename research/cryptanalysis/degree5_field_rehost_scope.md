# Challenge-field re-host scope — clearing the field-bound soundness term

**Date:** 2026-06-19. **Status:** SCOPE (analysis only; no code changed).
**Motivation:** `fri_aok_extractor_instantiation.md` (§5) showed the deployed leaf
STARK's COMPOSED knowledge-soundness is field-bound at **≈ 2⁻⁹²…⁻¹⁰⁰**, not 2⁻¹²⁸ —
the FRI commit/fold + DEEP additive terms scale as `|L|·log|L| / |F|`, and the
degree-4 KoalaBear challenge extension is only `|F| = p⁴ = 2¹²³·⁹⁵`. This doc scopes
the fix and corrects the original "degree-5 binomial" recommendation.

## 0. FEASIBILITY CORRECTION — degree-5 BINOMIAL is impossible over KoalaBear

`p = 2³¹−2²⁴+1`, `p−1 = 2²⁴·127`, so `gcd(5, p−1) = 1`. The 5th-power map is then a
BIJECTION on F_p, i.e. every element has a 5th root, so `x⁵ − W` is REDUCIBLE for
every `W`. **No degree-5 binomial extension exists for KoalaBear.** This is why
Plonky3 0.6.1 ships a *trinomial* quintic (`impl TrinomialQuinticData for
KoalaBearParameters`, p3-koala-bear-0.6.1/src/koala_bear.rs:129) and only *binomial*
degrees 4 and 8 (`BinomialExtensionData<4>` :91, `<8>` :109). The
`fri_aok_extractor_instantiation.md` §5 line "verify KoalaBear admits an irreducible
degree-5 binomial" resolves to **NO** — use a trinomial-5 or binomial-8 instead.

## 1. Viable field options (verified availability + margins)

Composed FRI-AoK margin estimated as `|F|_bits − log2(|L|·log|L|)`, `|L| = D·2⁴`:

| Option | Plonky3 type | \|F\| bits | D=2¹⁶ | D=2¹⁸ | D=2²⁰ | D=2²⁴ |
|---|---|---|---|---|---|---|
| **A. deg-8 binomial** | `BinomialExtensionField<Val,8>` | 247.9 | 2⁻²²⁴ | 2⁻²²²| 2⁻²¹⁹ | 2⁻²¹⁵ |
| **B. deg-5 trinomial** | Plonky3 quintic ext type (TBD) | 154.9 | 2⁻¹³¹ | 2⁻¹²⁹ | 2⁻¹²⁶ | 2⁻¹²² |

- **Option A (deg-8 binomial)** clears the FRI-AoK term with overwhelming margin for
  ANY realistic D — no D measurement needed. Cost: 8 challenge limbs vs 4 (~2× the
  challenge-field arithmetic; ~+100% on the Challenge-bearing proof components → est.
  proof-size increase, see §3). Type change is a one-liner (`<Val,4>`→`<Val,8>`).
- **Option B (deg-5 trinomial)** is the lighter fix (~+25% bytes, ~1.5× ext mults) but
  only CLEANLY clears 128 if the leaf codeword degree **D ≲ 2¹⁸** — so it is
  conditional on measuring D (§2). It also requires identifying the correct Plonky3
  extension type that wraps the KoalaBear trinomial quintic (NOT `BinomialExtensionField`,
  which would fail to compile) — an implementation unknown to resolve first.

## 2. MEASURED: D = batch_size × 16 (one trace row per account)

Measured from the trace generator (no heavy proof needed; this is exact):
- **Single-instance trace height H = 2** (`lib.rs:393`, `generate_full_trace(.., 2)`).
- **Batch trace height H = next_pow2(batch_size)** — ONE row per account
  (`air.rs:1881`, `num_rows = n_real.next_power_of_two()`). The Poseidon2 perms are
  unrolled ACROSS COLUMNS within a row, NOT across rows, so the trace is short-and-wide
  and H tracks the batch size, not the perm count.
- Constraint degree ≈ 3 (Poseidon2 S-box `(3,0)=3`, `p3-poseidon2-air sbox_constraint_degree`)
  → +1 for ZK ⇒ 4 ⇒ `log_quotient_degree = log2_ceil(4-1) = 2`. This adds 2^2 quotient
  CHUNKS (columns), but every committed codeword is LDE'd to `H·2^log_blowup`, so the FRI
  domain `|L|` is **independent of the quotient degree**.
- Therefore **D ≡ |L| = H · 2^PROD_LOG_BLOWUP = batch_size · 2^4** (PROD_LOG_BLOWUP = 4).

Production batch sizes (verdict memo `path2_descent_native_vs_stark_creation_verdict.md`):
N = 1024 (~1.6 KB/acct) and N = 4096 (~0.49 KB/acct).

| N (accounts) | H | \|L\|=H·16 | deg-4 (now) | deg-5 trinom | deg-8 binom |
|---|---|---|---|---|---|
| 1 (single)   | 2     | 2⁵  | 2⁻¹¹⁷ | 2⁻¹⁴⁸ | 2⁻²⁴¹ |
| **1024**     | 2¹⁰   | 2¹⁴ | 2⁻¹⁰⁶ | **2⁻¹³⁷** | 2⁻²³⁰ |
| **4096**     | 2¹²   | 2¹⁶ | 2⁻¹⁰⁴ | **2⁻¹³⁵** | 2⁻²²⁸ |
| 2¹⁶          | 2¹⁶   | 2²⁰ | 2⁻¹⁰⁰ | 2⁻¹³¹ | 2⁻²²⁴ |
| 2²⁰          | 2²⁰   | 2²⁴ | 2⁻⁹⁶  | 2⁻¹²⁷ (marginal) | 2⁻²²⁰ |

(composed ≈ |F|_bits − log2|L| − ~2 bits DEEP/list slack, under the FRI doc's small-list-
constant accounting where deg-4 @ |L|=2²⁰ ≈ 2⁻¹⁰⁰.) The doc's "92–100 bits" is the
LARGE-batch (N≈2¹⁶–2²⁰) corner; at the deployed N=1024–4096 deg-4 is ~2⁻¹⁰⁴…⁻¹⁰⁶ — still a
real sub-128 shortfall, so the fix is still required.

**Implication:** at the production batch sizes BOTH deg-5 trinomial and deg-8 binomial clear
a hard 2⁻¹²⁸ with margin. deg-5's margin erodes toward 2⁻¹²⁷ only at N≈2²⁰ accounts and is
sensitive to the FRI list-size constant `(m+½)⁷` (the doc took it small); deg-8's +124-bit
field margin is unconditional — batch-size- and list-constant-independent.

## 3. Change surface (from the 2026-06-19 codebase map)

- **Canonical edit:** `tools/descender-account-stark/src/config.rs:104`
  `pub type Challenge = BinomialExtensionField<Val, 4>;` → the chosen option.
- **6 benchmark mirrors** (must match or they drift): `stark-field-bench/src/{batch_bench:102,
  full_bench:115, c2_bench:97, main:143, zk_full_bench:210, prod_bench:190}`.
- **No degree-4-specific logic found**: no `from_basis_coefficients`, no `.0/.1/.2/.3`
  limb indexing, no `0..4` challenge-limb loops — Challenge is used opaquely via the
  Plonky3 trait API. So the change is type-driven, not logic-driven. (Re-verify after
  the switch that nothing hard-assumed 4 limbs.)
- **Proof BYTE layout DOES change** (larger): `Proof = p3_uni_stark::Proof<ZkConfig>`
  (descender-account-stark/src/lib.rs:97) embeds Challenge elements (FRI fold
  commitments, query openings, DEEP point). The verifier bundle `BundleWire.proof:
  Vec<u8>` (descender-account-verifier/src/lib.rs:144-149) is bincode of that proof, so
  its size grows (~+25% for deg-5, ~+100% on challenge components for deg-8). `PublicWire`
  is unaffected (only `Val` + raw bytes). Re-snapshot any proof-size assertions/KAT-like
  fixtures and the deployed-binding tests after the switch.
- **Recursion impact:** `descender-recursion` consumes the leaf config transitively; the
  in-AIR challenger gadget (PHASE1_GADGET1_CHALLENGER_SPEC.md) samples Challenge as D
  base-field draws per `sample()` — D=5 (or 8) changes the per-sample draw count and the
  §6 challenge-field-margin budget. Gadget-1 spec must be updated to the new D before impl.

## 4. The SPONGE floor is INDEPENDENT (and becomes the new binding constraint)

`frame_prf_poseidon2_ro_reduction.md` §2.3: the Poseidon2 sponge indifferentiability
floor is `c/2 = 123.95 bits`, set by CAPACITY `c = 8` KoalaBear elements — NOT by the
challenge extension. So after EITHER field option:
- FRI-AoK term: cleared (≥ ~122, or ≫128 for deg-8).
- Sponge indiff: STILL ~124 bits.

Therefore the honest global headline after a field-only re-host is **2⁻¹²⁴**, sponge-
bound. To reach a HARD 2⁻¹²⁸ you must ALSO widen sponge capacity to `c = 9`
(RATE 8→7 or WIDTH 16→18), which re-parameterizes Poseidon2 and BREAKS byte-identity
with the in-circuit AIR + the Plonky3 default-16 instance + the mask-PRF/FS sponge —
a heavy, cross-cutting re-host (cf. MEMORY stark-field-rehost). This is a separate,
larger decision from the challenge-field bump.

## 5. Decision points for James (gates the scope)

1. **Headline target.** Accept **2⁻¹²⁴** (relabel the dossier headline, itemize the
   sponge floor; field bump still needed to lift FRI-AoK off 92-100 up to the 124 line)
   — OR pursue **hard 2⁻¹²⁸** (field bump AND sponge-capacity c=9 re-host).
2. **Field option.** **A (deg-8 binomial)** — robust, D-independent, heavier bytes,
   one-line type — vs **B (deg-5 trinomial)** — lighter, but conditional on D≲2¹⁸ and on
   resolving the Plonky3 trinomial-ext type.
3. **Sequencing:** measure D first (§2) regardless — it informs whether B is even viable
   and confirms A's margin.

## 6. Recommendation (D measured; James chose HARD 2⁻¹²⁸)

D is measured (§2): |L| = batch_size·16, production N=1024–4096 ⇒ |L| = 2¹⁴–2¹⁶.

**Field part — recommend Option A (deg-8 binomial).** James wants a HARD 2⁻¹²⁸. deg-8 gives
a +124-bit field margin that is unconditional (batch-size- and list-constant-independent),
is a one-line type change (`BinomialExtensionField<Val,8>`, no trinomial-ext-type unknown to
resolve), and the cost (~2× challenge arithmetic, ~+100% on challenge-bearing proof bytes) is
acceptable for a creation proof that is batched (~1.6 KB/acct → est. ~2–3 KB/acct) and
recursion-folded. deg-5 trinomial is sufficient for current batch targets (≥2⁻¹³⁵ @ N≤4096)
and would save bytes, but its margin is sensitive to the FRI list-size constant and erodes at
N≳2²⁰ — so it is the lighter fallback, NOT the hard-128-robust choice.

**Sponge part (REQUIRED for hard 2⁻¹²⁸) — separate, heavier.** §4: capacity c=8→9 to lift the
sponge indifferentiability floor from 123.95 to ~139.4. This re-parameterizes Poseidon2-16
(RATE 8→7 or WIDTH→18) and BREAKS byte-identity with: the in-circuit account-creation AIR,
the deployed `descender-mask-prf` C2 mask PRF, the re-hosted FS challenge sponge, and the
Plonky3 `default_koalabear_poseidon2_16` instance + its KAT. This is a cross-cutting re-host
on the scale of the original mask-PRF swap — scope it as its own work item with its own KAT
regen and equivalence tests. Without it, a deg-8 field bump alone still leaves the honest
headline at 2⁻¹²⁴ (sponge-bound).

**Suggested sequencing:** (1) deg-8 field bump in `descender-account-stark` + 6 bench mirrors +
re-snapshot proof-size/byte assertions and the verifier-binding tests (smaller, self-contained);
(2) update gadget-1 challenger spec to D=8 draws/sample; (3) sponge c=9 re-host as a separate
tracked item (mask-PRF + FS + AIR + KATs). Production stays fail-closed throughout; nothing
flips live until external review.
