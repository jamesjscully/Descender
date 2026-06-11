# HIGH-3 fix — checked-core challenge widening (multi-target closure)

**Date:** 2026-06-10 · **Status:** ✅ APPLIED, tree green · Maintainer-approved (+4 sig bytes, no usage cap beyond T≤2³²).

## The finding (red-team HIGH-3)
The checked-core message-binding break (#6) is **MULTI-TARGET**: over T signing
queries the adversary obtains T distinct honest second-preimage targets and a break
against *any* of them suffices, so the advantage is `T·eps_spr(n) = T·q/2ⁿ`, not the
single-target `q/2ⁿ`. At the prior `n = 214`, `T = 2³²`, `q = 2⁶⁴`:
`T·q/2²¹⁴ = 2³²·2⁶⁴/2²¹⁴ = 2⁻¹¹⁸` — **breaches** the 2⁻¹²⁸ budget.

## The fix (widen the existing hardening field)
Widened the **existing** transcript-hardening field (NOT the algebraic challenge —
the action-vector input stays on the algebraic 23 bytes):

| constant | before | after |
|---|---|---|
| `CHALLENGE_HARDENING_BYTES` | 4 | **8** |
| `CHALLENGE_BYTES` (= 23 algebraic + hardening) | 27 | **31** (+4 B) |

The extra hardening bytes are additional SHA-256 output of the *same* FS frame,
truncated into the challenge (`hash_frame_to_challenge_bytes` already truncates to
`CHALLENGE_BYTES`). The algebraic challenge (23 B, 0x3f mask on byte 22) is unchanged.

### Effective message-binding width
`182` (algebraic-after-mask = (23−1)·8 + popcount(0x3f) = 176 + 6) `+ 8·8` (hardening)
`= **246 bits**` ✓ (≥246). At `n = 246`, `T = 2³²`, `q = 2⁶⁴`:
`T·q/2²⁴⁶ = 2⁹⁶/2²⁴⁶ = **2⁻¹⁵⁰** ≤ 2⁻¹³³` per-event budget (17 bits to spare).

## Before / after byte sizes
- **pk = 172 B** (unchanged).
- linear-sigma transaction signature: **260 → 264 B** (+4).
- Fq12 envelope (with fallback projection carrier): **314 → 318 B** (+4)
  = checked_core 258→**262** + carrier 56.
- reviewed-target sig 256→260, fq12+hidden-source-proof 290→294.
- KAT `signature_bytes` 260→**264**; challenge field length 27→31; offsets shifted +4;
  `producing_version_digest` regenerated (`--kat-generate` → `--kat-check` byte-for-byte ✓).

## EasyCrypt — the multi-target term (the core of the fix)
`break_checked_core_message_binding.ec` (width narrative 214→246) now adds the
multi-target layer on top of the proven single-target reduction:

- `MTSPR` game over a list of T fixed honest targets; win = second preimage of *some* target.
- `sha256_multi_target_spr` (standard T-fold union of the per-target game — no NEW
  hardness beyond the single-target axiom) +
  **`multi_target_break_le_bound`**: `Pr[break over T targets] ≤ (size fs) · sha256_second_preimage_bound`.
- `multi_target_break_le_budget` — the spine plug-in shape.

`gap_c_remodel_spine_wiring.ec` charges the **T-multiplied** term:
```
op tQ : real = 2%r ^ 32.                 (* T = 2^32 signing-query horizon *)
op eps_checked : real = tQ * eps_spr 246. (* MULTI-TARGET, n widened 214 -> 246 *)
op eps_leaf    : real = tQ * eps_spr 256.
op eps_registry: real = tQ * eps_spr 256.
```
Proven `multi_target_le_2pow_neg133 (n)`: `2^96 ≤ 2^(n−133)` ⇒ `tQ·eps_spr n ≤ 2⁻¹³³`,
instantiated at n=246 (n−133=113) and n=256 (n−133=123). Hence:
- `eps_checked_le_2pow_neg133`: `tQ·q/2²⁴⁶ = 2⁻¹⁵⁰ ≤ 2⁻¹³³` ✓
- `remodelled_budget_closes`: `2⁻¹²⁹ + 2⁻¹⁶⁰ + 2⁻¹⁶⁰ + 2⁻¹⁵⁰ + zeros ≤ 2⁻¹²⁸` ✓

All 10 `formal/easycrypt/*.ec` typecheck **exit 0** (EasyCrypt 2026.05 / Z3 4.16.0).

## Lean
`Descender/Path2HashBudget.lean`: `challengeHardeningBytes 4→8`,
`totalChallengeBitsAfterMask 214→246` (= 182 + 8·8, `native_decide`/`rfl` green).
`Descender/Path2NativeAccountDesign.lean`: regenerated the two frozen source-window
digests coupled to the budget window (`queryBudgetPricingDigest`,
`selectedRustFullSignatureReductionEvidenceDigest`). `lake build` **exit 0**.

## Test results (all green)
- `cargo build` (parameter-search bins) — OK.
- q12289 bin suite (`--bin signature_q12289_local_delta_stateful`): **30/30 pass**
  (includes in-process KAT byte-for-byte + sample-vector stability).
- `--kat-generate` then `--kat-check`: reproduces **byte-for-byte**.
- lib suite (`--lib`, production_reference incl. fail-closed preflight + renamed
  fixtures + Lean/Rust evidence-digest match): **pass** (219 tests).
- renamed fixtures + marker gate: `selected_projection_valid_314_byte_fixture_accepts`
  → `_318_` (fn def + `.find()` marker + `selected_test_checks` entry); preflight `.find()` resolves.
- `lake build` (Descender, incl. Path2HashBudget + Path2NativeAccountDesign): **exit 0**.
- all `formal/easycrypt/*.ec`: **exit 0**.

## Guardrail
+4 sig bytes (within the maintainer-approved ~+4). SIS box untouched
(RESPONSE_BOUND=48, 96<114). Multi-target horizon T ≤ 2³² is the only usage bound.
