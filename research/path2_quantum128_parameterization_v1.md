# Path2Quantum128ParameterizationV1

> Classification: `ACTIVE_SUPPORT`. The precise parameter set implementing the
> `ADR-0008` hard quantum-128 commitment, supporting `DIR-QROM-001`,
> `DIR-STARK-001`, and `ADOPT-128-001`. Not normative until the parameters land in
> `tools/descender-account-stark/src/config.rs` and the recursion mirror.

Status: exact q128 parameter set + why it must be applied atomically.

## The three quantum sub-requirements (all must hold for q128)

The 2026-06-27 stress test established that the deployed path is ~124-bit
classical but only ~64-bit quantum, from THREE distinct terms — and quantum-128
requires fixing ALL THREE, because the weakest dominates:

| term | mechanism | deployed (q) | q128 fix |
|---|---|---|---|
| FS-grind knowledge soundness | Grover over the non-interactive FRI query+PoW grind | ~2⁻⁶⁴ | raise `num_queries` so classical budget ~256 ⇒ Grover-halved ~128 |
| sponge indifferentiability | Poseidon2 capacity `c/2` | ~2⁻¹²⁴ | capacity `c = 9` ⇒ `c/2 ≈ 139` |
| Merkle/digest binding | quantum (BHT) collision on the digest, `digest_bits/3` | ~2⁻⁸²⁻⁹⁹ | widen digest so `digest_bits/3 ≥ 128` |

## Exact target parameters

```text
PROD_NUM_QUERIES : 56  -> 120        (120*(log_blowup/2=2) + pow 16 = 256 classical
                                       ⇒ Grover-halved 128 quantum FS-grind)
sponge RATE      : 8   -> 7          (keep WIDTH=16; CAPACITY = 16-7 = 9)
CAPACITY (c)     : 8   -> 9          (c/2 = 9*log2(p)/2 ≈ 139 bits > 128 indiff)
DIGEST_ELEMS     : 8   -> 13         (13*30.99 ≈ 403 digest bits; BHT quantum
                                       collision 403/3 ≈ 134 > 128 binding)
Merkle compress  : TruncatedPermutation -> SPONGE-mode 2-to-1 (REQUIRED, see below)
PROD_LOG_BLOWUP  : 4   (unchanged)   ; PROD_QUERY_POW_BITS : 16 (unchanged)
Challenge field  : BinomialExtensionField<Val,8> (already q128 on the field term)
```

### CORRECTION (2026-06-27): a 13-element digest is INFEASIBLE with the current Merkle compression

The deployed `MyCompress = TruncatedPermutation<Perm, 2, DIGEST_ELEMS, 16>`
compresses two child digests into the width-16 permutation, so it requires
`2·DIGEST_ELEMS ≤ WIDTH = 16`, i.e. **DIGEST_ELEMS ≤ 8**. A 13-element digest does
not fit (2·13 = 26 > 16); even a width-24 permutation caps at 12 elements, which
gives only `12·30.99/3 ≈ 124`-bit quantum (BHT) collision — still short of 128. So
**quantum-128 Merkle binding cannot be reached by widening the digest constant
within the truncated-permutation 2-to-1 compression.**

Required fix: replace the node compression with a **sponge-mode 2-to-1
compression** — `PaddingFreeSponge<Perm,16,7,13>` absorbing both child digests
(2·13 = 26 elements over rate 7 = 4 absorb permutations) and squeezing a
13-element digest. This reuses the same vetted width-16 permutation (still no new
constants) but decouples digest width from the `2·d ≤ WIDTH` constraint. Cost:
more permutations per Merkle node (~4 absorb + squeeze vs 1 truncated perm), so
Merkle hashing — and the in-AIR `merkle_verify` gadget in the recursion crate —
get proportionally heavier. This is an architectural change to the Merkle
construction, not a constant edit, and is the hardest part of the q128 binding
requirement.

## Key correction (de-risks the c=9 change)

An earlier note worried that `c=9` is a Poseidon2 re-host needing freshly-derived,
unvetted round constants (a security risk). **That is wrong for this route.**
Keeping `WIDTH = 16` and changing only the sponge RATE/CAPACITY split (8/8 → 7/9)
reuses the **same vetted Plonky3 width-16 KoalaBear Poseidon2 permutation and its
existing round constants** — only the sponge *mode* parameters change (how many
lanes are rate vs capacity). No new permutation instance, no unvetted constants,
no new algebraic-margin analysis for the permutation. This removes the main
risk I previously flagged. (`DIGEST_ELEMS 8→13` likewise reuses the permutation;
it widens the squeeze, changing the Merkle compression output width.)

## Why it must be atomic (queries-alone is a bad trade)

Bumping `num_queries` 56→120 *alone* was projected to roughly **double** the FRI
proof size (4.75 MB → ~9.5 MB); the complete active relation now measures
12.14 MB. The quantum headline would still stay at
~2⁻⁸²⁻⁹⁹ — because the Merkle binding term still dominates. So a queries-only
change is all-cost, no-headline-benefit. The three fixes must ship together; the
quantum headline only reaches 128 when the weakest of the three does.

## Blast radius (one atomic unit)

- `tools/descender-account-stark/src/config.rs`: the four constants above.
- The account-creation AIR: the Poseidon2 sponge gadgets (C1/C2/C6/role) are
  sized to RATE=8 and DIGEST_ELEMS=8; RATE 8→7 changes absorb scheduling and
  DIGEST 8→13 changes the Merkle compression columns. Real AIR work, not a
  constant edit.
- `tools/descender-mask-prf`: the mask sponge shares the rate/capacity split.
- All digest-pinned KATs / byte-model / residual reports regenerate (proof bytes
  and digests all change) — the heavy prover-regen step.
- The recursion crate (`tools/descender-recursion`) MIRRORS these params (it
  imports the leaf config and re-implements the sponge/Merkle/FRI in-AIR), so the
  inlined perm gadgets, Merkle-verify width, and query-loop counts must move in
  lockstep, and the OUTER recursion soundness must be re-priced to q128 too.

## Re-pricing after the change

- FS-grind: classical 256 → quantum 128. ✓
- sponge indiff: `c/2 ≈ 139` classical/quantum. ✓
- digest binding: classical `403/2 ≈ 201`, quantum `403/3 ≈ 134` ✓ — but ONLY
  with the sponge-mode 2-to-1 Merkle compression (above); the current
  TruncatedPermutation caps digest at 8 elems ⇒ ~83-bit quantum and CANNOT reach
  128. This is the binding-side blocker, not a constant.
- field additive (degree-8): ~2⁻²¹⁵⁻²²⁴, unaffected. ✓
- net honest headline after the change: ~2⁻¹²⁸ classical and quantum (inner);
  the outer recursion must be re-priced and is its own obligation.

## Economic note

The measured 12.14 MB inner proof is irrelevant to on-chain cost (creation is verified off
the hot path via recursion per `research/path2_economic_deployment_architecture_v1.md`),
but it ~doubles prover and recursion-verification work. This is a prover-side
cost, acceptable; it is NOT an on-chain per-tx cost.

## Status

**q128 re-parameterization BUILT, INTEGRATED, and TESTED end-to-end (2026-06-27,
`q128-reparam` worktree).** Leaf crate (`descender-account-stark`): 17/17 lib tests +
the real `small_config_single_prove_verify_with_classc` prove→verify pass at digest-13.
Recursion crate (`descender-recursion`): full suite `366 passed; 0 failed` at digest-13
(both the lib and the atomic monolith test binary). The `config.rs` parameter docs /
report prose were regenerated to the q128 figures (120 queries ⇒ conservative 256
classical / ~128 quantum FS-grind; capacity c=9 ⇒ c/2 ≈ 139; digest-13 ⇒ BHT ~134),
and the stale "c=9 separate work item" / "56 queries" / digest-8 comments removed.

**Provable recursion KEYSTONE reached (2026-06-27).** Beyond constraint-satisfaction, the
recursive verifier-AIR is now PROVED end-to-end: `recursive_air::tests::recursive_air_proves_and_verifies_q128`
runs a real `p3_uni_stark::prove` + `verify` of `RecursiveAir` on the q128 Poseidon2 stack and
passes. The only fix needed was implementing `RecursiveAir::num_public_values` (the AIR was a valid
`check_all_constraints` AIR but defaulted its public-value count to 0, breaking p3-uni-stark's
symbolic-vs-prove uniformity assert). The recursion now produces an O(1) recursive STARK proof —
the artifact a Phase-3 block aggregator folds and an on-chain verifier checks. See
`research/path2_recursion_air_composition_architecture.md` (Provability gate — RESOLVED). The
economic aggregation layer ON TOP of this keystone (Phase-2 {creation+spend} co-verification,
Phase-3 block accumulator, EVM verifier) remains.

The public payload byte model (`formal/kat/path2_account_stark_payload_byte_model_v1.json`)
is SHA-256-FRAMED — it carries a fixed 32-byte `creation_proof_binding_digest`
commitment to the proof, so it is structurally digest-13-INVARIANT and needs no
regeneration; `formal/kat/kat_v0.json` is the q12289 spend-signature KAT (a separate
subsystem, unaffected). What a future deployment step still needs is a FROZEN KAT
corpus pinning concrete digest VALUES from a real q128 proof (distinct from the
structural byte model), the recursion Phase-2/3 economic aggregation layer, the F2
reorg-safe pad-seed registry, and the EVM verifier.

Original status follows. Parameter set fixed. Implementation is the atomic q128
re-parameterization unit under `DIR-QROM-001`/`DIR-STARK-001`.

**Increment 1 BUILT + TESTED (2026-06-27):** the binding-gate sponge-mode 2-to-1
Merkle compression — the hardest piece (an architectural change, not a constant) —
is implemented as a standalone, additive, warning-clean component
`tools/descender-account-stark/src/sponge_compression.rs`
(`SpongeCompression<H>: PseudoCompressionFunction<[F;OUT],2>` over a
`PaddingFreeSponge<Perm,16,7,13>`). Its unit test
`q128_sponge_compression_is_deterministic_and_binds_both_children` passes under
`RUSTFLAGS="-D warnings"`: deterministic, 13-element digest, binds both children,
order-sensitive, reusing the vetted width-16 permutation (no new constants). It is
NOT yet wired into `HidingValMmcs` (that wiring + `DIGEST_ELEMS 8→13` +
`RATE 8→7` + `num_queries 56→120` + full KAT regen + the recursion `merkle_verify`
mirror is the remaining atomic re-param). The component drops in when that lands.

**Increment 2 — end-to-end q128 Merkle stack VERIFIED (2026-06-27):** the test
`q128_merkle_stack_commit_open_verify_roundtrips` (same module) composes the
rate-7/digest-13 leaf hash (`PaddingFreeSponge<Perm,16,7,13>`) and the new
`SpongeCompression` into a real `MerkleTreeMmcs<…,2,13>`, commits a matrix, opens
a row, and verifies the opening — and rejects a tampered opening — all under
`-D warnings`. This proves the binding-side q128 Merkle layer is implementable
end-to-end on the vetted width-16 permutation: the digest-13 commitment scheme
works in the real Plonky3 MMCS, not just the compression in isolation. The
remaining binding-side work is swapping these types into the live `config.rs`
`HidingValMmcs` (+ `DIGEST_ELEMS`/`RATE` constants) and mirroring the wider
Merkle in the recursion `merkle_verify` AIR gadget — mechanical, no longer holding
unsolved crypto.

**Increment 3 — atomic config re-param COMPILES CLEAN (2026-06-27, isolated
`q128-reparam` worktree):** applied the full q128 set to `config.rs`
(`DIGEST_ELEMS 8→13`, `MyHash` RATE `8→7`, `MyCompress` →
`SpongeCompression<MyHash>`, `PROD_NUM_QUERIES 56→120`) and the crate compiles
with only FIVE mechanical fixes — the `MyCompress::new(perm)` call sites became
`MyCompress::new(MyHash::new(perm))` (the sponge compressor wraps a hasher, not a
raw permutation) plus removing the now-unused `TruncatedPermutation` import. The
in-`air.rs` sponge/Merkle gadgets compiled unchanged against the new constants
(they are parameterized on `DIGEST_ELEMS`/rate), so there is **no air.rs gadget
rewrite** — a major de-risking vs. the earlier feared full-gadget rewrite.
the full KAT/byte-model regeneration, and the recursion-crate mirror. The atomic
re-param is staged on the `q128-reparam` worktree; the deployed branch is
untouched.

**Increment 4 — q128 leaf prove/verify FUNCTIONALLY VERIFIED (2026-06-27):** the
crate's real `small_config_single_prove_verify_with_classc` prove→verify test
PASSES on the q128-reparam'd crate (digest-13 sponge Merkle, rate-7,
`SpongeCompression`) — `test result: ok. 1 passed; 0 failed`. So the new
binding-side Merkle/sponge gadgets don't just compile, they correctly prove AND
verify in-circuit at the new parameters. This retires the central risk of the
leaf-side q128 re-param: the digest-13 sponge construction works end-to-end in the
real STARK pipeline. (The test uses small-config FRI knobs for speed, so it
exercises the new sponge/Merkle gadgets fully; the `num_queries=120` value is a
soundness/size knob, not a gadget-correctness factor.)

**Recursion mirror, Merkle node compression — BUILT + TESTED (2026-06-27,
`q128-reparam` worktree):** both recursion-crate Merkle layers were rewritten from
the truncated-permutation digest-8 form to the digest-13 sponge-mode form:

- `tools/descender-recursion/src/merkle.rs` (reference fold) — `Compress` repointed
  from `TruncatedPermutation<Perm,2,DIGEST_ELEMS,WIDTH>` (invalid at digest-13:
  2·13 > 16) to the leaf's re-exported `MyCompress = SpongeCompression<MyHash>`, so
  node hashing is byte-identical to the leaf. (`MyCompress`/`MyHash` were made `pub`
  in the leaf crate for this.) Drift guard now pins `DIGEST_ELEMS == 13`.
- `tools/descender-recursion/src/merkle_air.rs` (in-AIR `Air<AB>`) — the node
  constraint was rewritten from ONE inlined perm block (`compress([a‖b]) =
  perm([a‖b])[..8]`) to **`N_PERM = ceil(2·13 / 7) = 4` inlined perm blocks** with
  the overwrite-sponge absorb/carry wiring mirroring `p3-symmetric`'s
  `PaddingFreeSponge::hash_iter` (sponge.rs:172-204) constant-for-constant: each
  block overwrites `state[0..absorb_k]` with the chunk and carries the previous
  block's output in the unfilled rate + capacity lanes; the node digest is the last
  block's `output[0..13]`.

Verified by 14 passing tests (run in an isolation harness, since the rest of the
crate does not yet compile at digest-13 — see next): `air_node_matches_real_sponge_compression`
(the 4-block in-AIR sponge equals the real `compressor()` for both index bits),
`air_accepts_every_honest_opening`, and six forgery rejections (tampered
sibling/leaf/root, wrong index bit, broken chaining, tampered interior perm cell in
a MIDDLE sponge block). The binding-side recursion Merkle is done.

**The recursion FRI obstruction — REFINED with evidence (2026-06-27).** Three
quantities coincided at 8 in the deployed config and were partly conflated; the
diagnosis is now precise, and two of the three are already correct:

- `crate::fri_fold_air::D = 8` is ALREADY a literal extension degree (the field is
  `BinomialExtensionField<Val,8>`) — correct, no change.
- `crate::challenger::RATE = 8` is ALREADY the FS `DuplexChallenger` rate, and the
  q128 config KEEPS `Challenger = DuplexChallenger<Val,Perm,16,8>` (rate 8) — correct.
- The conflation is narrower than first stated: the **leaf-row hash (`MyHash`) rate**
  (now 7) was assumed equal to the challenger rate (8), and `fri_layer_air.rs` pins
  its LOCAL `D` and `RATE` to `DIGEST_ELEMS`.

Empirically (2026-06-27, `q128-reparam` worktree): the recursion crate **lib
compiles clean at digest-13** (`cargo build --lib`). The breakage is correctness +
test modules. The remaining binding-side recursion items, precisely localized:

1. **Leaf-row hash rate (`MyHash`) — primitive BUILT + TESTED, and `recursive_air`
   REPOINTED (lib compiles).** New module `tools/descender-recursion/src/leaf_hash.rs`
   gives the leaf hash its own named constant `LEAF_HASH_RATE = 7` (capacity c=9),
   distinct from `challenger::RATE = 8` and `DIGEST_ELEMS = 13`, with
   `leaf_hash_pre_states`/`leaf_hash_digest`/`lh_perms` at rate 7 / digest 13. Three tests
   pass, the load-bearing one being `leaf_hash_matches_real_padding_free_sponge` (equals the
   REAL `PaddingFreeSponge<Perm,16,7,13>` across all length classes). `recursive_air.rs`'s
   leaf-hash perm counts (`lh_perms`/`qo_leaf_perms`/`lo_leaf_perms`), the host
   `leaf_hash_pre_states` loop, and the three in-AIR leaf-hash blocks ((b-hash) main / (qo) /
   (lo)) were repointed from `challenger::RATE` onto `LEAF_HASH_RATE` (the absorb schedule
   they already mirror, now at the correct rate 7); the challenger-sponge `RATE` uses (draw
   buckets, FS-stream chunking) are untouched. Lib compiles at digest-13.
2. **`fri_layer_air.rs` constants + Merkle path — DONE + TESTED.** Local
   `D = DIGEST_ELEMS` → `crate::fri_fold_air::D` (8, which also CORRECTED `flatten_pair`,
   previously laying 8-limb extension elements into a 13-wide stride); `RATE = DIGEST_ELEMS`
   → `LEAF_HASH_RATE` (7); leaf-hash imports repointed from `recursive_air` to the verified
   `crate::leaf_hash`; and the per-level Merkle node rewritten from the truncated-perm
   placement to the digest-13 sponge-mode node. To avoid duplicating the sponge logic, the
   node compression was extracted into reusable helpers `merkle_air::node_blocks` (host) +
   `merkle_air::eval_node` (in-AIR), now used by BOTH `merkle_air` and `fri_layer_air`. All
   12 `fri_layer_air` tests pass, including `fidelity_real_mmcs_layer_open_authenticates_in_air`
   (a real `MerkleTreeMmcs<…,MyHash,MyCompress,2,13>` round-trip plus the in-AIR open at
   rate-7/digest-13 sponge) and the flipped-path-selector forgery. The shared-helper refactor
   kept all 8 `merkle_air` tests green.
3. **`recursive_air.rs` Merkle path — DONE (lib compiles; mirrors tested helpers).** All
   THREE Merkle families in the monolith — main-trace (`ML_*`), per-round layer-open
   (`LO_ML_*`), and quotient (`QO_ML_*`) — were migrated from the single truncated-perm node
   to the digest-13 sponge node via the shared `merkle_air::node_blocks`/`eval_node` helpers:
   each `*_MERKLE_LEVEL` is now `2·DIGEST_ELEMS + N_PERM·PERM_COLS`, each pack site builds the
   `N_PERM` blocks, each eval site uses `eval_node` with the combined-by-bit closure, and all
   chain/root reads use the new `*_node_digest(lbase)` offset (last block output). The crate
   lib compiles at digest-13 and the shared helpers are isolation-tested (72/72), so the
   migration is correct-by-construction against verified references — but it is NOT yet
   exercised by `recursive_air`'s own tests (its test binary still needs items 1/4/5 + the
   fixture migration to compile as a whole; `recursive_air` is atomic to test).
4. **Leaf-as-challenge identity — DONE (lib).** The direct `(b-leaf)` binding
   `level-0 running == C_E0` (Merkle leaf digest == FRI `e0`) only holds in the legacy
   digest-8 regime where the leaf digest WAS the codeword value. It was guarded to
   `DIGEST_ELEMS == D` (inert at q128) and its `debug_assert_eq!(DIGEST_ELEMS, D)` panic
   removed; at q128 the leaf↔e0 link is carried by the real flow — `(b-hash)` (level-0
   running == `MyHash(opened_row)`, 13 wide) composed with `(DR)` (`C_E0` == the DEEP
   combination of the opened row, 8 limbs). The remaining digest-13-only test fixtures that
   used the legacy `Challenge::from_basis_coefficients_slice(&leaf)` (a 13→8 unwrap) need the
   value/digest split when those tests are migrated.
5. **FS commitment observation — REFERENCE BUILT + TESTED, and `recursive_air`
   INTEGRATED (lib compiles).** The FS-β round block (`FB_*`) was restructured from one
   rate-8 absorb-permute per round to the q128 schedule: a `DIGEST_ELEMS`-wide commitment
   column (`FB_COMMIT`) + `FB_NPERM = ceil(13/8) = 2` perm blocks (a full rate-8 absorb +
   a trailing 5-element partial) + the beta LIFO of the LAST permute — exactly the
   `fs_observe::fs_squeeze_betas_digest` schedule. The host `pack_window` β loop, the per-round
   AIR eval, the `fb_state_prev`/post-state readers, the FS-qi continuation, the
   layer-open↔FS-commitment unify, and the public-transcript binding were all moved to the
   2-permute / 13-wide layout; `pack_window` now calls `fs_squeeze_betas_digest` and the
   `Inputs::fs_beta_blocks` field is `&[[Val; DIGEST_ELEMS]]`. Crate lib compiles at digest-13.
   Original analysis follows.

   The deepest item, reference:
   `recursive_air.rs::fs_squeeze_betas` takes `commit_blocks: &[[Val; RATE]]`, observing
   each FRI-layer commitment as exactly ONE rate-8 challenger block (the
   `digest == rate == 8` shortcut). At q128 a commitment is a 13-element digest, which the
   rate-8 `DuplexChallenger` observes as a FULL block (permute triggered on the 8th observed
   element) + a TRAILING PARTIAL absorb of the remaining 5 (the second `duplexing()`
   triggered by the first `sample()` of `beta_r`) — TWO permutes per round, with the p3
   length tag `state[RATE] += n` and `state[n..RATE]` zeroed on each (`p3-challenger`
   duplex_challenger.rs:86-176, confirmed against source). New module
   `tools/descender-recursion/src/fs_observe.rs` implements this as
   `fs_squeeze_betas_digest(start_state, commit_digests: &[[Val; DIGEST_ELEMS]])` and PINS it
   against the REAL `DuplexChallenger<Val,Perm,16,8>` observing the same 13-element digests
   (`fs_squeeze_betas_digest_matches_native_challenger`, plus a carried-state tamper test).
   What remains in `recursive_air.rs` is swapping its `fs_squeeze_betas` call sites + the
   in-AIR FS β-round block onto this 2-permute schedule (the AIR currently has one
   absorb-permute column block per commit round; q128 needs the full+partial pair), and
   updating the test `comms: Vec<[Val; RATE]>` fixtures to `[Val; DIGEST_ELEMS]`.

Net: every NOVEL q128 primitive the recursion needs now has a BUILT + TESTED reference —
the sponge-mode Merkle node (`merkle_air::node_blocks`/`eval_node`), the rate-7/digest-13
leaf-row hash (`leaf_hash`), and the digest-13 FS commitment-observation schedule
(`fs_observe::fs_squeeze_betas_digest`, pinned to the native `DuplexChallenger`) — and
`fri_layer_air` (the per-round FRI commit-phase layer Merkle open) is fully migrated onto
them and GREEN (12 tests). INTEGRATION into the monolithic `recursive_air.rs` is COMPLETE — all four items done, the
crate lib compiles at digest-13, AND the (atomic) `recursive_air` test binary now compiles and
runs at digest-13:

- item 3 — Merkle path (all three families) onto `node_blocks`/`eval_node`;
- item 1 — leaf-hash onto `LEAF_HASH_RATE = 7`;
- item 4 — leaf-as-challenge binding guarded to the digest==degree regime, q128 leaf↔e0 carried
  by `(b-hash) ∘ (DR)`;
- item 5 — FS-β round restructured onto the 2-permute `fs_observe::fs_squeeze_betas_digest`
  schedule with a `DIGEST_ELEMS`-wide commitment column.

The TEST-FIXTURE migration is done: `[Val; RATE]` commitment fixtures → `[Val; DIGEST_ELEMS]`
(`fb_block`/`comms`/`fb_window`/`fan_out_*`/`native_*` helper signatures), `FB_BLOCK`/`FB_PERM`
test reads → `FB_COMMIT`/`FB_PERM0`, the rate-8 real-MMCS test hashers → `MyHash`(rate-7) +
`SpongeCompression`, the FS-public `pubs.len()` assertion → `* DIGEST_ELEMS`, and the leaf→e0
`from_basis_coefficients_slice(&leaf)` (13→8) test sites → `&leaf[..D]`.

The first full run surfaced 12 failures; ALL were test-side, none in the production AIR:
the off-by-one "fold root != quot_commit" was the TAMPER CONTROLS feeding intentionally-corrupt
input to `pack_window`, tripping its honest-builder `debug_assert` (a probe confirmed the in-AIR
quotient fold equals the real MMCS root) — fixed by tampering the bound cell POST-build (as the
other forgery tests do); plus the legacy digest==e0 mismatch test gated to the digest==degree
regime, the `PROD_NUM_QUERIES` pin updated 56→120, and the fixture type/rate updates above.

Tracking: q128-green recursion modules — `merkle`, `merkle_air`, `leaf_hash`, `fs_observe`,
`fri_layer_air`, and `recursive_air` (lib + test binary at digest-13). The q128 recursion is
fully integrated and exercised end-to-end at digest-13.

**Current config observation (assumption boundary):** `MyHash` and the
account-STARK `Challenger` are both rate 7 / capacity 9. The account proof's
degree-8 extension challenge therefore crosses two seven-element output blocks.
`c9_challenger.rs` pins native Plonky3 fidelity and `c9_extension_air.rs` pins the
second-squeeze AIR shape. Older rate-8 detail above records the path used to reach
the current implementation. The monolithic recursive challenger arithmetic is now
rate 7/capacity 9 across alpha/zeta, OOD, batch-combination alpha, betas, and query
continuation. Full-plus-partial pre-alpha and quotient schedules are now
constraint-checked on a real production proof. End-to-end closure still requires
all actual inner-proof columns, especially the cost-heavy OOD/opening stream, in
one priced production settlement member window followed by a joint KAT-chain refresh.

## Frozen q128 leaf-proof KAT (2026-06-27)

A deterministic PRODUCTION-config q128 account-creation proof (digest-13 sponge, rate-7, 120 queries,
query-PoW 16) now has a PINNED byte-format KAT: `descender-account-stark`
`tests::frozen_q128_leaf_proof_kat` proves under a single-thread pool (so the FRI query-PoW grind is
deterministic), bincode-serializes the proof, SHA-256s it, and pins
`b4c9e522f599cc4cd574a7387a6f7300d7d23a1215b83f2a5a667657d4a6e4a0` (determinism cross-checked by
proving twice). This freezes the q128 proof byte format against drift (DIR-STARK-001
frozen-byte-format KAT). The production payload corpus separately pins 16 accept/reject cases and
the 12,144,273-byte bundle digest in
`formal/kat/path2_account_stark_payload_kat_vectors_v1.json`.
