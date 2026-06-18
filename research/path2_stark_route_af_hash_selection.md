# Path 2 STARK Route — Arithmetization-Friendly Hash Selection (mask PRF + AIR hash)

Status: primitive SELECTED (Poseidon2 / x^5 over Fq=12289); parameterization is
RESEARCH-GRADE with two named open items. 2026-06-17.

## Why this decision, and why now

The maintainer approved option (b) of
`research/path2_stark_route_c1c2_sha_binding_bridge_risk.md`: switch the
projection-mask PRF from SHA-256 to an arithmetization-friendly (AF) hash so the
176-row creation AIR is recursion-friendly. The AF-hash choice is the GATING
decision because the same primitive is used for (1) the C2 mask PRF (shared with
the spend verifier), (2) the inner pad-source/opening-tuple commitments, and (3)
the STARK Merkle/FRI commitment hash. It fixes the hash used throughout the AIR
and the re-charged spend SUF-CMA assumption, so it must be pinned before any AIR
is written.

## Selected primitive

**Poseidon2 over the prime field Fq, q = 12289**, with:

- **S-box `x^5`.** Smallest permutation monomial: a monomial `x^a` permutes Fq
  iff `gcd(a, q-1) = 1`; `q-1 = 12288 = 2^12 * 3`, so **`x^3` FAILS (3 | q-1)** and
  `x^5` is the smallest valid power (`gcd(5, 12288) = 1`). Inverse exponent
  `5^-1 mod 12288 = 7373` (only relevant for Rescue-style inverse-S-box modes;
  Poseidon2 does not need it). This is the same constraint Mersenne31 faces
  (uses `x^5`), unlike BabyBear/KoalaBear which can use the cheaper `x^3`.
- **Sponge capacity c ≈ 19 lanes (≈258 bits) for 128-bit collision resistance.**
  Sponge CR is `c*log2(q)/2`; at `log2(12289) ≈ 13.585` bits/lane, 128-bit CR
  needs `c >= ~19` lanes. q=12289 is ~17 bits smaller than BabyBear, so this is
  ~2.1x more capacity lanes than a 31-bit field (19 vs 9) — a CONSTANT FACTOR
  (wider state / a couple more absorptions), not a structural break.
- **State width t = 24 (rate 5 + capacity 19).** This is REQUIRED, not just
  preferred, for single-permutation 128-bit collision resistance at rate 5:
  collision resistance is `capacity_bits / 2`, so 128-bit CR needs capacity
  >= 256 bits = >= 19 lanes (`19 * 13.585 = 258` bits). t = 16 / rate 5 gives
  only an 11-lane (~149-bit) capacity = ~74-bit collision resistance — NOT
  acceptable. (t = 20 / rate 1 also reaches the 19-lane capacity at lower S-box
  cost but slower absorption; left as an AIR-phase cost tuning.)
- **R_F = 8 full rounds + R_P ≈ 60 partial rounds (CONSERVATIVE) — do NOT use the
  formula's R_P=22.** Running the canonical Poseidon round-number reference
  (`ingonyama-zk/poseidon-hash poseidon/round_numbers.py`, identical logic to the
  Poseidon2 sage; tool VALIDATED against BLS12-381 ~255b t=3 -> 8/56 exact, and
  BabyBear 31b t=16 -> 8/13-14) for `prime_bit_len = 13.5851, M = 128, t = 24,
  alpha = 5, margin on` outputs **R_F=8, R_P=22**. **That output is a FORMULA
  ARTIFACT, not a safe parameter** (2026-06-17 small-field probe, open item 1
  turned into a finding): every algebraic-attack inequality in the formula is
  capped by `min(security_level, prime_bit_len)` / `prime_bit_len/2 ≈ 6.79`, so
  at 14 bits the FIELD (not the 128-bit target) is binding — R_P=22 effectively
  certifies ~14-bit, not 128-bit, algebraic resistance. The tool is being
  extrapolated ~18x below its validated field size. No AF-hash deployment or
  third-party cryptanalysis exists below ~31 bits (BabyBear/KoalaBear/Mersenne31);
  the 14-bit field also hands an algebraic attacker free per-variable degree caps
  (`x^q - x`, deg 12289) and cheap 2^14 per-element search; and the 2023-2026
  trend (HADES 2023/537 "formula optimistic / can under-estimate"; subspace-trail
  Gröbner 2025/954) is toward needing MORE partial-round margin, not less.
  **Defensible choice: ignore the field cap and use the un-capped large-field x^5
  count R_P ≈ 56, plus the tool's 7.5% margin => R_P ≈ 60 (R_F=8; R_F=10 as
  belt-and-suspenders).** This is nearly free: 24*8 + 60 = 252 S-boxes vs 214 at
  R_P=22 (~18% more), still ~2-3 orders below SHA-in-AIR. Rule if params change:
  recompute with `security_level` forced to 128 (drop the `prime_bit_len` cap in
  the algebraic inequalities) and floor R_P at the published large-field x^5
  count. Independent small-field cryptanalysis (open item 2) still recommended
  even at R_P≈60.
- **IOP / FRI soundness carried on Fq12 (~168 bits)**, NOT on the 14-bit base
  field. Hash collision resistance and IOP soundness are separate budgets; both
  must independently meet 128 bits. Fq12 comfortably covers FRI query soundness
  and Fiat-Shamir challenge sampling.

AIR cost (with the conservative R_P≈60): **252 S-boxes per permutation** (24*8
full + 60 partial), ~3 multiplication constraints per x^5 S-box => ~750 degree-5
constraints, ~68 trace rows (R_F + R_P) per permutation. The C2 mask PRF (absorb
domain ‖ pad_seed ‖ slot ‖ lane, squeeze 6 q12289 lanes at rate 5) is ~2-3
permutation calls => ~140-200 rows / ~500-750 S-boxes. Versus 12 SHA-256
compressions at ~25k-60k constraints each (~300k-720k total, bit-oriented and
hostile to a prime AIR), the replacement is still **~2-3 orders of magnitude
cheaper in-AIR** even at the padded round count. The only price is slower off-chain native hashing (one-time at
creation) and the small-field algebraic margin below.

## Why not the alternatives

- **Monolith — eliminated.** Its `Bars` layer is a byte-decomposition (8-bit
  lookup) gadget hard-wired to 31-/64-bit fields; it does not naturally
  instantiate over a 14-bit prime. (Also: 2 of 6 rounds have practical
  collisions; full unbroken.)
- **Rescue-Prime** — workable (`x^7373` inverse S-box) but ~2x Poseidon2 AIR
  cost and a narrowing recent algebraic margin (512-bit param broken by
  resultant/FreeLunch 2024).
- **Anemoi / Griffin** — higher algebraic-attack risk; FreeLunch/resultant
  attacks reached 11/21 (Anemoi) and 7-8/10 (Griffin) rounds in 2024-25.

Poseidon2 is the only mature candidate that is field-agnostic (pure `x^alpha`,
no field-specific gadget), most-deployed/most-scrutinized, with full-round
128-bit instances intact, and compatible with the forced `x^5`.

## Assumption-budget effect on the spend SUF-CMA spine

The current dossier charges `A_sha256_projection_mask_frame_prf` (NAMED-STANDARD)
+ an action-map QROM term. Under this switch that becomes:
"Poseidon2-permutation ideal (named-standard, comparable to SHA-256's current
status) + sponge indifferentiability." The sponge RO model is defensible both
classically (BDPV 2008, up to ~2^(c/2)) and **post-quantum** (sponge quantum-
indifferentiability, arXiv 2504.16887, 2025), so the QROM term gains a concrete
citation rather than a bare assumption. Net: a comparable, arguably
better-justified, named assumption — not a soundness loss. It MUST be re-charged
in the EC spend spine (`accepted_spend_forgery_reduction.ec`,
`frame_prf_ro_reduction.ec`).

## Open items before any SUF-CMA claim can lean on this

1. ~~**Compute R_P**~~ DONE 2026-06-17, and it became a FINDING: the formula's
   R_P=22 is a small-field artifact and is NOT safe; use the conservative
   un-capped **R_P ≈ 60** (see Rounds bullet). Compute step discharged; round-count
   safety folded into item 2.
2. **Independent small-field algebraic-margin review (primary residual, partly
   EXTERNAL).** 14-bit is below the >=31-bit range AF-hash papers parameterize and
   below any real deployment; FreeLunch/CheapLunch/resultant + subspace-trail
   Gröbner work (2024-2025) keeps eroding partial-round margins, and HADES
   (2023/537) showed the round formula can UNDER-estimate. Even at R_P≈60 a
   dedicated third-party cryptanalysis of x^5 Poseidon2 over a 14-bit prime is
   prudent before any SUF-CMA claim leans on it.
   NOTE — "run over Fq12" is NOT a clean drop-in: Fq12 = GF(q^12) is an EXTENSION
   field and power-map S-boxes need a prime field — x^5 fails over GF(q^12)
   (`5 | q^12-1`; so does x^7). Realistic alternatives if R_P≈60 on the q12289
   prime field is still distrusted: (a) a separate larger PRIME field for the hash
   + a hiding-preserving conversion to q12289 mask lanes (adds a field-conversion
   seam, HIGH-2-sensitive); (b) a lookup-augmented (Monolith-style) design
   re-instantiated for the small field. Default remains x^5 Poseidon2 over the
   q12289 prime field with R_P≈60.
3. **RO model certifies the sponge mode, not Poseidon2_pi itself** — the
   permutation's ideal-ness is the standing named assumption (same status as
   SHA-256 today).

## Downstream sequence (unchanged by this doc; recorded for ordering)

hash selected + R_P pinned (this doc) -> independent small-field review (item 2,
partly external) -> swap mask PRF in `production_reference.rs` behind a
fail-closed marker with KATs (touches spend verifier mask recomputation; spend
bytes unchanged) -> re-prove injectivity from the algebraic `pad_source_commitment`
root (LeafV2 `creation_proof_binding_digest` already scaffolded) -> re-charge EC
spend spine -> write the 176-row AIR using Poseidon2/x^5 throughout.

## Non-claims

- R_P pinned at the conservative un-capped value (~60); does not build the hash.
- Does not prove the small-field algebraic margin; flags it as the primary
  residual + partly-external review item.
- Does not change spend signature bytes (values + assumption change only).
- Does not instantiate the AIR or the injectivity re-proof.

## Implementation progress (2026-06-17)

- **Poseidon2 primitive over q12289 — BUILT + self-reviewed.**
  `tools/parameter-search/src/poseidon2_q12289.rs` (new module, additive; does NOT
  touch `production_reference.rs` or the deployed spend path). x^5 S-box, t=24,
  R_F=8, R_P=60, canonical external matrix (M_4 block construction, MDS-verified),
  canonical internal matrix (all-ones off-diagonal + diagonal, sum-trick), Grain
  LFSR round constants. Correctness ANCHORED: the Grain LFSR reproduces published
  circomlib BN254 t=2 round constants exactly. 12 self-tests pass (x^5 permutes /
  x^3 does not, M_E/M_I invertibility, permutation bijectivity via implemented
  inverse, fixed KAT regression vector, sponge). Self-review caught + fixed a real
  flaw mid-build: an initial pure-diagonal M_I (no partial-round diffusion) was
  replaced with the canonical all-ones+diagonal form.
  Still self-reviewed / DEFERRED: the q12289-specific internal-matrix diagonal's
  deep (subspace-trail) security and the R_P=60 small-field margin (open item 2).
- **Poseidon2 projection-mask PRF — BUILT (building block, not yet wired).**
  `projection_masks_poseidon2(pad_seed, slot_id) -> [u16;6]` in the same module.
  Byte-lane input encoding (each byte a field element in [0,256)) — injective and
  AIR-range-check-friendly, so the in-circuit C2 PRF and this native PRF will
  agree by construction. KAT: determinism, range, sensitivity to pad_seed/slot.
  NOT yet replacing the deployed SHA-256 mask PRF (that swap touches the spend
  verifier + KAT corpus + EC spine and is staged next).
- **Poseidon2 AIR (permutation arithmetization) — BUILT + tested.**
  `tools/parameter-search/src/poseidon2_air_q12289.rs` (additive). Framework-
  agnostic constraint system for one Poseidon2 permutation: 70-row x 24-col trace,
  1704 constraints (48 boundary + 1656 transition), max degree 5. Anchor test:
  the honest trace output equals the reference `permute()` over 64 random inputs;
  all 1704 residuals are 0 on honest traces; an exhaustive cell x delta sweep
  confirms any single-cell mutation breaks >=1 constraint. 5 tests pass. This is
  the AIR relation R for the hash, not yet a FRI proof. (Made 7 field/matrix
  helpers `pub` in the poseidon2 module to reuse the EXACT reference arithmetic.)

- **C2 obligation in-circuit (mask = PRF(pad_seed,slot)) — BUILT + tested.**
  `tools/parameter-search/src/poseidon2_prf_air_q12289.rs` (additive). Composes
  the single-permutation AIR into the full sponge PRF (block plan DERIVED from the
  sponge code: 9 absorb + 1 squeeze = 10 permutations), with absorb-link, squeeze-
  link, capacity-init, byte-range (boolean bits + recompose, rejects >=256), and
  mask-output boundary constraints. 17,665 constraints, max degree 5. Anchor: AIR
  mask outputs equal `projection_masks_poseidon2` over random inputs; honest trace
  residual 0; mutations (pad byte, intermediate cell, links, claimed mask, out-of-
  range byte) each break >=1 constraint. 9 tests pass. This is exactly the C1/C2/
  C4/C6 obligation the machine-checked negative theorem said the old sampled core
  failed to force, now a degree-<=5 relation. Next bricks: C4 (base-128 digit
  reconstruction + carrier seam + canonical repack), C1/C6 (commitment openings).

- **C4/range gadget (canonical base-128 reconstruction + range + carrier) — BUILT + tested.**
  `tools/parameter-search/src/base128_range_air_q12289.rs` (additive). The
  `qFieldRow` (digit_reconstruction + masked_projection carrier) and
  `base128RangeRow` (digit_range) core gadget. Handles the 14-bit-field
  canonicity subtlety correctly: since Fq arithmetic wraps, a field equation
  cannot enforce `residue < q`; the gadget enforces canonical base-128
  (q=96*128+1 => d_hi in [0,96] via a 7-bit decomposition of 96-d_hi, plus an
  is-zero edge gadget forcing d_hi==96 => d_lo==0). 29 constraints/residue, max
  degree 2. Verified: full 0..Q canonical sweep (all 12289 residues) satisfies;
  INJECTIVITY-CRITICAL non-canonical reps REJECTED (d_hi=96/d_lo=6 by is96*d_lo=0;
  d_hi=120 by the slack recompose); mutations break constraints. 8 tests pass.

- **Composed creation relation AIR (C2 + C4 + carrier, seams enforced) — BUILT + tested.**
  `tools/parameter-search/src/path2_creation_relation_air_q12289.rs` (additive).
  Composes the C2 PRF AIR and the C4/range gadget into one relation: public
  `masked_projection[6]`; hidden `pad_seed`/`slot`/`account_projection`; proves
  masked_projection_i = PRF(pad_seed,slot)_i + canonical account_projection_i.
  Seam binding is REAL (not vacuous colocation): the carrier consumes the PRF's
  actual mask-output cells and the reconstruction's actual residue cells, enforced
  by `SeamMask`/`SeamAcc` equality constraints; a forged masked_projection has no
  satisfying assignment. 17,863 constraints, max degree 5, 11 tests. A real
  soundness hole (public-bind/carrier folding letting the masked cell cancel) was
  caught by the mutation test and fixed (split into separate PublicBind + Carrier
  residuals). This is the C2+C4 joint obligation the machine-checked negative
  theorem flagged, now a sound degree-<=5 relation. Remaining relation classes
  before the full ~176-row relation: role-restriction/commitment (C1/C6),
  mask-consistency, lifecycle/context, checked-core-compat.

- **C1 pad-source commitment-opening, composed with shared pad_seed — BUILT + tested.**
  `tools/parameter-search/src/poseidon2_commit_air_q12289.rs` + extension of the
  composed relation. New reference `pad_source_commitment_poseidon2(pad_seed,
  blinding_nonce)` = Poseidon2 sponge over 32 pad_seed + 16 nonce byte-lanes,
  squeeze 10 lanes (~136-bit binding, hiding via the nonce), distinct domain. The
  C1 AIR (11 perms, 19,469 constraints) proves the public pad_source_commitment
  opens to the hidden pad_seed; `SeamPadSeed{byte}` constraints bind those 32
  pad_seed byte-lanes to the C2 sub-witness, so ONE pad_seed both opens the
  commitment and derives the masks (tamper fires the exact byte). Composed
  relation now jointly forces C1 AND C2 AND C4: 37,374 constraints, max degree 5,
  14 + 7 tests pass. This is the algebraic binding root replacing the SHA
  pad_seed_digest. Remaining: C6 opening-tuple/other role commitments, then
  mask-consistency / lifecycle-context / checked-core-compat classes.

- **C6 opening-tuple commitment, composed with shared account_projection — BUILT + tested.**
  `tools/parameter-search/src/poseidon2_opening_tuple_commit_air_q12289.rs` +
  relation extension. New reference `opening_tuple_commitment_poseidon2(
  account_projection, lifecycle_anchor, blinding_nonce)` = Poseidon2 over the 6
  account_projection FIELD lanes + 32 lifecycle + 16 nonce byte-lanes, squeeze 10
  (distinct domain). C6 AIR (12 perms, 21,197 constraints) proves the public
  opening-tuple commitment opens to the hidden account_projection + lifecycle;
  `SeamOpeningAcc{lane}` binds those 6 account_projection lanes to the C4
  reconstruction, so ONE account_projection both opens the commitment and feeds
  the carrier (tamper fires the exact lane). 16 + 8 tests pass.

  **MILESTONE: the composed relation now jointly forces C1 AND C2 AND C4 AND C6**
  — all four core obligations the machine-checked negative theorem identified as
  missing from the old sampled core — as a sound degree-5 AIR (58,587
  constraints), with shared-witness seams binding pad_seed across C1/C2 and
  account_projection across C4/C6. This is the heart of relation-soundness. It is
  the AIR relation R only (constraint evaluator); turning it into a deployable
  proof still needs the remaining relation classes (mask-consistency, lifecycle/
  context, other role-restrictions, checked-core-compat), then the FRI proof
  layer, ZK hiding, the deployed mask-PRF swap, and the Lean/EC re-proofs.

## SUPERSEDED (2026-06-17 PM): proof system re-hosted onto Plonky3

The hand-rolled FRI/STARK below (over q12289/Fq12) is CORRECT and proved the full
relation, but is SLOW (6-8 min prove) because q12289's 2-adicity (12) forces all
FFT/FRI into the Fq12 extension + naive single-threaded arithmetic. DECISION
(James): re-host the proof system onto **Plonky3** over a 31-bit FFT-friendly
field. Measured benchmark: ~37-perm scale prove drops **6-8 min -> ~3-12 ms**
(~10^5x), verify ~1.5 ms, on KoalaBear/BabyBear (KoalaBear leads on speed+size;
both recursion-friendly per the aggregation criterion). Plonky3 ALSO gives vetted
FRI (retires the hand-rolled-FRI soundness risk), ZK-FRI, and a recursion path.
The RELATION logic (C1/C2/C4/C6 below) carries over as Plonky3 AIRs (q12289
becomes the application modulus: 14-bit embed + witnessed mod-q reduction). First
component re-hosted: C2 (mask PRF) as a field-generic Plonky3 AIR, proved on
BabyBear+KoalaBear with completeness+soundness (tools/stark-field-bench/). The
hand-rolled stack below is retained as the correct reference. See memory
[[stark-field-rehost]].

## MILESTONE: full account-creation relation proved as a single STARK (2026-06-17)

`tools/parameter-search/src/full_relation_air_q12289.rs` (additive). The ENTIRE
C1 ∧ C2 ∧ C4 ∧ C6 account-creation relation, with all cross-component seams, is
now a single STARK-provable AIR proved end-to-end by the generic DEEP-ALI STARK:
one committed trace, one composition, one FRI. Composite: 1024 rows x 453 cols,
degree-reduced (x^5->3), domain 16384, rate 1/16. C1/C2/C6 sponge chains as
parallel column groups; seams as same-row cross-group residuals (SeamPadSeed
C1==C2 over 32 pad_seed lanes; SeamOpeningAcc C4==C6 over 6 account_projection
lanes; carrier masked=mask+acc via constant-propagation columns; commit/public
binds). COMPLETENESS: honest secrets -> proof VERIFIES and the public outputs
(pad_source_commitment, opening_tuple_commitment, masked_projection) equal the
reference functions. SOUNDNESS: 6 tamper classes (component cell, pad_seed seam,
acc seam, wrong public commitment, C4 non-canonical, forged masked_projection)
all REJECTED. Verifying it certifies ONE hidden pad_seed (C1<->C2) and ONE
canonical hidden account_projection (C4<->C6, into the carrier) jointly produce
the public commitments + masked_projection, modulo the named FRI proximity-gap +
DEEP-ALI assumptions. Prove ~6-8 min, proof heavy (16 queries for test
tractability). LIMITS (next bricks): (1) NOT zero-knowledge yet — the trace is
Merkle-committed IN THE CLEAR (sound but reveals the witness; this is the reopened
HIGH-2 surface — ZK hiding via R(X) randomizer is next); (2) production query
count (16 -> ~80+ for full soundness bits); (3) deployed mask-PRF swap, Lean
injectivity re-proof + EC re-charge, gate flip.

## FRI proof layer — foundations (the gate to deployable; multi-session)

- **GF(12289^12) extension field — BUILT + tested.**
  `tools/parameter-search/src/fq12_q12289.rs` (additive). The FRI challenge/
  soundness field: FRI over the 14-bit base field needs challenges from a >=128-bit
  extension; q^12 ≈ 2^163. Irreducible modulus f(x) = x^12 - 11 (g=11 is the first
  irreducible binomial; the small binomials g=2,3,5,7 are reducible), and
  irreducibility is MACHINE-VERIFIED via a from-scratch Rabin test (with reducible
  controls x^12, x^12-1, x^12+x^6 confirming the test isn't vacuous). Full field
  API (add/sub/neg/mul/inv-via-Fermat/pow/frobenius/from_base). 11 tests: field
  axioms, inverse round-trip, Frobenius is a base-fixing automorphism, a^(q^12)=a,
  base-field consistency. This is the prerequisite for FRI soundness; remaining FRI
  bricks: Merkle-over-Poseidon2 commitment, low-degree extension, FRI folding with
  Fq12 challenges, DEEP/constraint composition, query phase, Fiat-Shamir.

- **Merkle-over-Poseidon2 vector commitment — BUILT + tested.**
  `tools/parameter-search/src/merkle_poseidon2_q12289.rs` (additive). 10-lane
  digests; domain-separated LEAF/NODE hashing (prevents leaf/node collision);
  `commit`/`root`/`open`/`verify`; LSB-first index folding; verify RECOMPUTES the
  leaf digest (doesn't trust a supplied one). 8 tests (completeness on 16 & 1024
  leaves, tamper detection on leaf/sibling/index/root, domain separation,
  collision sanity). PERF TODO: the sponge rebuilds Poseidon2 params per call
  (1024-leaf test ~34s); memoize params before FRI does thousands of hashes.

- **FRI evaluation-domain design — DECIDED (machine-computed 2-adicity).**
  q12289: q-1 = 2^12*3 => 2-adicity 12 => max base-field FFT domain = 4096. The
  composed relation trace is ~2310 rows (33 Poseidon2 perms x 70) => pads to 4096,
  leaving NO room for FRI blowup on the base field. q^12-1 has 2-adicity 14 => max
  Fq12 FFT domain = 16384. DECISION: **FRI runs over Fq12** — interpolate the
  trace via a base-field NTT at <=4096 (base-field 2^12 roots exist), evaluate the
  LDE codeword on the 16384-point Fq12 domain (**4x blowup, rate 1/4** = exactly
  Fq12's 2^14 maximum; 8x does not fit even in Fq12), and run FRI folding +
  queries over Fq12. Conservative Johnson-bound query count per the proximity-gap
  caution. Next FRI bricks: base-field NTT (interpolation), Fq12 LDE evaluation,
  FRI folding, DEEP/constraint composition, query phase, Fiat-Shamir (over Fq12).

- **Generic AIR STARK + first real component proof (C2: masks = PRF) — BUILT + tested.**
  `src/air.rs` (generic `Air` trait), `src/poseidon2_perm_air_adapter.rs`,
  `src/c2_prf_air_q12289.rs`, refactored `src/stark_q12289.rs`. The STARK now
  consumes any `Air` (auto-selects trace/FRI domain from n_rows + max_degree, rate
  <=1/4, <=16384); the 6 single-permutation STARK tests pass through it unchanged.
  FIRST REAL COMPONENT: the C2 PRF chain (masks = projection_masks_poseidon2, a
  10-permutation sponge) is expressed as an Air and PROVED: n_rows=1024, n_cols=93,
  **degree-reduced x^5 -> 3** (aux square cols: x^5 = sq^2*a, binding sq = a^2 —
  NECESSARY because degree-5 composition 5114 > 4096 won't fit Fq12 2-adicity;
  degree-3 = 3068 fits), domain 16384, blowup 16, rate 1/4. C2 COMPLETENESS
  (honest trace verifies, committed masks = reference PRF) + SOUNDNESS (6 tampers
  incl. wrong-mask all rejected). proof ~347 KiB, prove ~133s, verify ~491ms (24
  queries; release). This genuinely proves "exists a byte-range-checked hidden
  (pad_seed,slot) whose Poseidon2 sponge yields the committed masks" modulo the
  named FRI proximity-gap + DEEP-ALI assumptions. The degree-reduction validates
  the earlier layout-grounding recommendation. Next: C1/C6/C4 as Airs + cross-
  component seams -> full relation proof.

- **DEEP-ALI STARK pipeline (end-to-end) — BUILT + tested.**
  `tools/parameter-search/src/stark_q12289.rs` (additive). The full STARK glue on
  the single Poseidon2 permutation AIR (establishing the pipeline before scaling):
  trace LDE-commit (n=128, N=4096 coset, rate 1/4) -> composition H = Sum alpha_j *
  C_j / Z_j (boundary + transition vanishing polys; batched-inversion eval off the
  trace subgroup) -> DEEP out-of-domain point z (FS), OOD evals + identity check ->
  batched DEEP quotients -> FRI low-degree test. 6 tests (release ~52s):
  COMPLETENESS (honest trace verifies, output = permute()), SOUNDNESS (6 trace
  tampers all rejected), OOD-binding (lying about z-evals caught), Merkle binding,
  FS determinism. A real DEEP-quotient denominator bug was caught+fixed during
  bring-up. HONEST: soundness rests on FRI proximity-gap + DEEP-ALI argument
  (named/standard); code enforces FS + Merkle + OOD identity + per-query DEEP
  consistency + FRI verdict. This is "FRI actually proves the relation", proven on
  the permutation AIR; next = scale to the full C1^C2^C4^C6 relation via the
  parallel layout.

- **Fq12 NTT + LDE — BUILT + tested.** `tools/parameter-search/src/fq12_ntt_q12289.rs`
  (additive). OMEGA14 = 8246*x^9 (order EXACTLY 16384, machine-verified via the
  (q^12-1)/2^14 exponent), forward/inverse Fq12 NTT, `lde_from_base_coeffs` /
  `lde_coset` (evaluate base-field trace polys on the 16384-pt Fq12 domain = the
  FRI codeword). 8 tests: omega order, round-trip, naive-DFT-over-Fq12 anchor,
  LDE-vs-naive + base-NTT consistency.
- **FRI low-degree test (prover + verifier + Fiat-Shamir) over Fq12 — BUILT + tested.**
  `tools/parameter-search/src/fri_q12289.rs` (additive). Standard 2-to-1 folding
  (even + beta*odd), rate 1/4, coset shift 7+x (DEEP-disjoint), Merkle-committed
  layers (20-lane), Poseidon2 Fiat-Shamir for beta_i + query positions, stop at
  domain 8, Q=80 queries. Verify checks fold-consistency + Merkle binding +
  final-layer degree at each query. 10 tests: COMPLETENESS (low-degree passes,
  k=8/k=12), SOUNDNESS (random codeword rejected 30/30, tampered 30/30 = 100%),
  fold-correctness cross-check vs even/odd decomposition, FS determinism, Merkle
  binding. HONEST: the 128-bit soundness BOUND rests on the FRI proximity-gap
  theorem (BBHR18/BCIKS20) + query count = NAMED ASSUMPTION; the code enforces
  the locally-checkable obligations (fold-consistency, Merkle binding, FS). This
  is the core of the proof system.

- **Base-field NTT over q12289 — BUILT + tested.**
  `tools/parameter-search/src/ntt_q12289.rs` (additive). Generator g=11 (order
  exactly 12288), omega = g^3 = 1331 (order EXACTLY 4096 = base-field 2-adicity);
  radix-2 forward/inverse NTT, interpolate/evaluate, coset variants for LDE.
  8 tests incl. omega-order, NTT round-trip (n up to 4096), and the naive-DFT
  cross-check anchor. Max domain 4096 (base 2-adicity); larger LDE needs the Fq12
  NTT (next). This is the trace-interpolation engine.
- **Poseidon2 param memoization (perf) — DONE.** `poseidon2_sponge` now uses a
  process-wide `OnceLock<Poseidon2Params>` instead of rebuilding Grain constants
  per call (Merkle suite 40s->14s, ~2.85x). Output values unchanged. Prerequisite
  for the FRI hashing loop.

- **STARK trace-layout / domain budget — DECIDED (machine-computed; retires a real fit risk).**
  The relation must fit the Fq12 FFT domain (max 16384 = 2-adicity 14). NAIVE
  vertical concatenation of all sponge permutations = 2590 rows -> 4096, which at
  degree-5 constraints + rate 1/4 needs a 32768-point domain => DOES NOT FIT.
  DECISION: **parallel column-group layout** — C1/C2/C6 sponge chains laid out
  side-by-side, so trace LENGTH = max sponge depth = 980 -> 1024 (column count
  doesn't affect the FFT domain). At degree-5 the composition quotient ~4092 =>
  rate-1/4 FRI domain = 16384 = exactly Fq12's max: FITS. Remaining relation
  classes add degree<=2 copy/boundary constraints (more columns, same trace
  length) — within budget. RECOMMENDED refinement: degree-reduce the x^5 S-box to
  degree 2 via auxiliary x^2/x^4 columns => composition degree 2 => domain 4096
  (blowup 4) => ~4x faster proving + headroom. This confirms FRI/STARK over the
  constrained 14-bit field is viable for the full relation. Next: DEEP/constraint
  composition (bind the AIR constraints to FRI), built on this layout.

## Small-field Poseidon2 assumption — RIGOROUSLY TESTED (2026-06-17)

An empirical algebraic-margin probe (SageMath: degree-growth + Groebner CICO over
GF(12289), width t=3) tested the highest-risk assumption (x^5 Poseidon2 at 14
bits). Results:

- **Degree saturates at round 6** (5^6 > q-1; vs ~110 rounds at a 255-bit field —
  ~18x faster, the small-field danger signal). BUT Groebner CICO solving cost
  still grows steeply (~89x per partial round, ~231x per full round, MEASURED to
  r<=6 partial / r<=4 full), NO collapse at saturation. Extrapolated ~21-37
  partial rounds for 2^128 GB cost => **R_P=60 plausibly sufficient but UNKNOWN**
  (extrapolated across the unmeasured width axis t=3->t=24 and the saturated
  regime r>6). Flagged as a NAMED RESIDUAL assumption (like the external
  coded-SIS) — not proven; revisit via width-scaling GB or external review.

- **CONCRETE FLAW FOUND + FIXED: commitment/Merkle outputs were 10 lanes (~136
  bits) => only ~68-bit COLLISION resistance** (binding is a collision property =
  output/2). Fixed to **20 lanes** (capacity-limited ~129-bit collision; capacity
  19 lanes/258 bits is adequate). Applied to `pad_source_commitment_poseidon2`,
  `opening_tuple_commitment_poseidon2`, the C1/C6 AIRs (perm counts re-derived
  11->13, 12->14), and the Merkle tree (DIGEST_LANES 10->20). Mask PRF left at 6
  lanes (NOT a binding commitment; security = capacity). Composed relation now
  65,539 constraints, max degree 5; all tests green.

- **Capacity (19 lanes / 258 bits) is adequate** for 128-bit (collision 129-bit,
  preimage 258-bit). Generic security = q^k, k = capacity in FIELD elements.

- **Monolith does NOT cleanly fit** a 14-bit prime (its Bars lookup layer is tied
  to 31-/64-bit fields — already eliminated in the AF-hash selection). So the
  realistic options remain x^5 Poseidon2 (this design, R_P residual flagged) or a
  larger PRIME hash field + a conversion seam. Staying with x^5 + R_P=60 as the
  pragmatic path with the margin as a named residual.

## Working-tree health note (2026-06-17)

The full `cargo test --lib` is currently RED on ONE pre-existing test,
`production_reference::tests::path2_transparent_zk_creation_proof_negatives_reject`
(panics at production_reference.rs:73705, "transparent full hidden relation
subproof envelope payload digest mismatch"). This is part of the uncommitted
HIGH-2 transparent-descent blob, NOT a regression from the Poseidon2 work
(`production_reference.rs` has zero references to the new modules). Earlier
narrow health-guard filters missed it. It reinforces that the held blob is WIP
and should not be committed as-is. All new Poseidon2 modules are green.

## Sources

Poseidon2 (ePrint 2023/323), Poseidon (2019/458), HorizenLabs poseidon2
round-search sage; HADES cryptanalysis (2023/537); FreeLunch (2024) and
resultant attack (2024/886), Anemoi Gröbner (2024/250); Monolith (2023/1025) and
Tip5-family/Monolith collisions (2024/1900); sponge indifferentiability (BDPV
2008) and quantum sponge indifferentiability (arXiv 2504.16887, 2025);
small-fields-in-Plonky3 (HackMD), Plonky3 repo.
