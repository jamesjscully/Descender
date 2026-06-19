# Sponge capacity c=8→9 hardening scope (the second half of hard 2⁻¹²⁸)

**Date:** 2026-06-19. **Status:** SCOPE — analysis only, NO code changed. Flagged for
James's go-ahead BEFORE execution because it re-touches BOTH deployed signatures.
**Motivation:** `frame_prf_poseidon2_ro_reduction.md` §2.3 — the Poseidon2 sponge
indifferentiability floor is `c/2 = 8·log2(p)/2 = 123.95 bits < 128`, set by CAPACITY
`c = 8` KoalaBear elements. The deg-8 challenge-field bump (commit 4c06f6b) cleared the
FRI-AoK field term but NOT this; until capacity is widened the honest headline stays
2⁻¹²⁴. James chose a HARD 2⁻¹²⁸, so this is required.

## 1. Is the 124-bit floor real, or a conservative provable artifact?

**Real.** It is not just the indifferentiability *proof* bound — a birthday collision on
the `c = 8`-element (≈247.9-bit) capacity is a genuine ≈2^123.95 attack: two absorb
sequences that collide in the capacity state yield a sponge-state collision ⇒ RO/length-
extension break. The same `c/2` bound is ALSO the collision-resistance floor of the
Poseidon2-Merkle leaf sponge (`PaddingFreeSponge`). So c=8 genuinely caps FS-challenge RO
quality, mask-PRF RO quality, AND Merkle CR at ~124 bits. To reach 128 the capacity must
grow: `c ≥ 2·128/30.989 = 8.26 ⇒ c = 9` ⇒ floor `9·30.989/2 = 139.4 bits`. (A_poseidon2_
perm_ideal does not rescue this — the capacity birthday is generic, independent of any
structural distinguisher.)

## 2. Cleanest realization: RATE 8→7, SAME permutation

WIDTH stays 16; set RATE = 7 ⇒ CAPACITY = 16−7 = 9. The pinned
`default_koalabear_poseidon2_16` permutation (round constants, MDS, S-box) is BYTE-
IDENTICAL — only the rate/capacity SPLIT of each sponge changes. This avoids a WIDTH/
constants re-host (which would be far worse). Cost: ~1 extra permutation call per
8/7-lane block absorbed (rate 7 < 8), i.e. modestly more perms; no new constants, no
Plonky3 permutation fork.

## 3. Blast radius (every RATE=8 sponge) — RE-TOUCHES BOTH DEPLOYED SIGNATURES

| Sponge | Where | Effect of RATE 8→7 |
|---|---|---|
| `DuplexChallenger<Val,Perm,16,8>` | account-stark `config.rs:150` | all STARK FS challenges change ⇒ creation proof bytes change ⇒ account-stark fixtures regen; **gadget-1 challenger spec must target RATE=7** |
| `PaddingFreeSponge<Perm,16,8,8>` | account-stark `config.rs:133` | Merkle leaf hash CR floor 124→139; changes Merkle roots ⇒ proof bytes change |
| `descender-mask-prf RATE=8` | `mask.rs:21` | **C2 projection masks change** ⇒ BOTH the deployed SPEND signature (kat_v0 regen AGAIN) AND the creation relation; the FS checked-core byte-hash also changes (relation+spend FS); the in-AIR C2 sponge in `air.rs` must match RATE=7; `mask_prf_equivalence.rs` must be re-pinned |
| `TruncatedPermutation<Perm,2,8,16>` | account-stark `config.rs` | 2-to-1 compression; CR ~ digest(8 elems)=248 bits, NOT rate-bound ⇒ **no change needed** |

So this change cuts across: descender-mask-prf (deployed signer dep), descender-account-
stark (creation), descender-account-verifier (proof bytes), parameter-search (spend KAT
regen — `formal/kat/kat_v0.json` AGAIN), the in-AIR gadgets, and the recursion gadget-1
spec. It is on the scale of the original mask-PRF Poseidon2 swap, and like that one it
changes BYTES the deployed signatures produce.

## 4. Required work items (sequenced)

1. **descender-mask-prf RATE 8→7.** Flip `RATE`, re-run the standalone tests; this is the
   root — masks + FS byte-hash change. Re-pin `poseidon2_hash_bytes` property test
   (values change, properties hold).
2. **In-AIR C2 sponge (air.rs) RATE→7** to stay byte-identical to (1); re-run
   `mask_prf_equivalence.rs` (drift guard) — must still match standalone.
3. **account-stark config**: `DuplexChallenger<…,16,7>` + `PaddingFreeSponge<…,16,7,8>`;
   rebuild; regenerate account-stark roundtrip/soundness fixtures (recompute, not
   hand-edit); re-measure proof size (slightly larger from more perms + nothing from
   capacity).
4. **parameter-search spend FS / mask consumers**: the deployed spend signature uses the
   mask-PRF; regenerate `formal/kat/kat_v0.json` + `.md` from the real pipeline (same
   discipline as commit 108c10c); preserve the archived SHA anchor. Re-run full suite.
5. **descender-account-verifier**: proof bytes change; re-run deployed_binding.
6. **Recursion gadget-1 spec**: update RATE 8→7 (capacity 9) before implementing the
   in-AIR challenger so it mirrors the new sponge.
7. **Assumption ledger / dossier**: record capacity 8→9, sponge floor 123.95→139.4, and
   that the global headline now reaches a hard 2⁻¹²⁸ (field term from deg-8 commit 4c06f6b
   + sponge floor from this change). Re-run the EC RO-model note (abstract RO meaning
   unchanged; only the concrete capacity parameter moves).

## 5. Risk / discipline

- Changes deployed signature BYTES (spend + creation) ⇒ KAT regen REQUIRED, from the real
  pipeline only, never hand-edited; verify byte-reproduction tests pass post-regen.
- Test mask/FS hiding by ATTEMPTED RECOVERY, never substring scan (standing lesson).
- Heavy proving for fixture regen ⇒ serial-to-disk, no parallel proofs (RAM discipline).
- Production stays fail-closed throughout; nothing flips live (external-review-gated).
- This is a SIGNIFICANT deployed-crypto change → do not execute without James's go-ahead.

## 6. Recommendation

Proceed with RATE 8→7 (capacity 9) as scoped — it is the surgical realization (no
permutation/constants change) and is genuinely required for the hard 2⁻¹²⁸ James chose. Do
it as ONE atomic cross-crate change (mask-prf → AIR → config → spend KAT → verifier) so CI
never sees a half-migrated sponge, with both deployed KAT corpora regenerated in the same
commit. Sequence it AFTER confirming the deg-8 commit is accepted, and BEFORE implementing
gadget-1 (which must target the final RATE). Estimated effort: comparable to the mask-PRF
swap — a focused multi-step change, not a research project, since the math (capacity 9 ⇒
139-bit floor) is settled.
