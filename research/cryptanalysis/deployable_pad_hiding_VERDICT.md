# Task #24 — Does the DEPLOYABLE Fq12 design INHERIT the scaffold's pad-in-clear leak?

**RISK-FIRST verdict: CONDITIONAL-GAP-FIXABLE.** The deployable Fq12 / prevalidation design
does **NOT** inherit the scaffold's key-free pad leak. The pad is hidden pre-spend by a
**PRF-fresh, per-instance, domain-separated mask** (not the scaffold's constant `[12, Q-12]`),
the on-chain serialization emits **only masked/committed forms** (no raw pad seed, no masks, no
unmasked projection), and the residual is **finishing an already-scoped, fail-closed composition
proof** (5 carried assumptions, 13 named breaks, 20 open items) — none of which is a *fundamental*
pad-hiding gap. The single load-bearing assumption is the SHA-256 frame-PRF / RO-PoK assumption,
which is explicitly carried, not contradicted.

The scaffold leak (`reference_sign_with_proof`, `stateful.rs:4703/4746`: constant mask + openings
= const + pad(seed)) is a property of the **non-deployable linear-sigma byte anchor** and is
**absent from the deployable path** verified below.

---

## CLAIM 1 — Serialization excludes the pad. CONFIRMED in code.

The deployable on-chain proof/opening structs carry **digests and masked residues only**:

- `ProductionReviewedFq12StateAssistedOpening` (`production_reference.rs:777-785`) carries only
  `slot_id` + five 32-byte digests (`witness_opening_digest`, `pad_seed_digest`,
  `carrier_handle_secret_digest`, etc.). **No raw pad seed, no masks, no unmasked projection.**
- `ProductionReviewedFq12AccountCarriedProof` (`7274-7277`) on the wire = `proof_commitment` +
  `masked_quotient_response` only.
- The packed witness commitment carries `syndrome || masked_projection` where
  `masked_projection[idx] = (account_projection[idx] + masks[idx]) mod Q`
  (`reviewed_fq12_pack_masked_witness_state18_from_parts`, `6423-6442`). The **masked** form is
  what is committed/packed; the unmasked `account_projection` is recomputed verifier-side from
  the pad seed witness, not stored.
- The `verify_reviewed_fq12_state_assisted_*projection_carrier_tuple` functions
  (`6854-7043`) take `pad_seed` as a **verifier-side witness argument**, not a serialized field —
  the pad seed is supplied to the verifier, never emitted on the public surface.
- Public spend view is a **digest-only proof surface**: test
  `public_native_spend_view_v1_serializes_digest_only_proof_surface` (`49218-49271`) asserts
  `assert_no_active_native_private_material(&serialized)` and that no raw proof bytes appear.
- Budget artifact `pre_spend_live_slot_budget.forbidden_public_values` explicitly forbids
  `"pad seed bytes"`, `"six projection-mask residues"`, `"six unmasked selected account
  projection residues"`, `"account_witness_projection_opening_frame"` pre-spend
  (`pre_signature_projection_spend_time_leakage_budget_n32_q12289.json`).

> **CAVEAT (intentional, not a leak):** the `ProductionReviewedFq12MaskedState18PackedWitnessOpening`
> *frame* (`6462-6517`) DOES serialize `projection_mask` (6502) and `account_projection_residue`
> (6507). This is the **accepted-spend-certificate** opening, where the 6 selected projection
> residues are *intentionally* disclosed and **charged** as spend-opening disclosure
> (`spend_opening_budget.selected_projection_residues_intentionally_public_after_spend: true`).
> That is a deliberate post-spend opening of the *projection being proven*, NOT a pre-spend pad
> leak, and NOT the scaffold's pad-in-clear. Distinct from the scaffold: there the pad was
> recoverable key-free *before* any spend, from every signature.

Tracker corroboration (`account_creation_prevalidated_slot_proof_tracker.md:930/942/946`): JSON
view excludes raw pad seed / projection masks / unmasked account projection.

## CLAIM 2 — The mask is PRF-FRESH / hiding, NOT a constant. CONFIRMED — this is the crux.

The deployable mask is **not** the scaffold's hardcoded `[12, Q-12]`. It is:

```
reviewed_fq12_packed_witness_state18_projection_masks(pad_seed, slot_id):   (6376-6394)
  mask[idx] = uniform_mod_from_hash(MASK_DOMAIN, pad_seed||slot_id, slot_id, idx, Q)
```

where `uniform_mod_from_hash` (`46290-46308`) is rejection-sampled
`SHA256(domain || seed || stream || index || counter) mod Q` — **per-lane, per-slot**, derived
from the pad seed under a **distinct domain**.

**Domain separation verified directly:**
- Mask PRF domain: `REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_MASK_DOMAIN`
  = `"...local_delta.reviewed_fq12_packed_witness_state18_projection_mask.v1"` (`production_reference.rs:242`).
- Pad PRF domain: `REVIEWED_FQ12_PAD_DOMAIN` = `"...local_delta.pad.v1"` (`237`), consumed by
  `reviewed_fq12_pad_prf_block` (`45624-45634`) = `SHA256(pad_seed || frame[pad.v1||block] || counter)`.

The two PRF input streams are **byte-frame disjoint**: pad PRF input length **82 B**, mask PRF
input length **154 B**; `lengths_disjoint: true`, `bounded_disjoint: true`
(`pre_signature_projection_mask_frame_disjointness_n32_q12289.json`). So the mask stream is
computationally independent of the pad-residue stream under SHA-256-as-RO, even though both are
keyed by the same `pad_seed`.

**Empirical independence:** mask-independence probe over **50,000 pad seeds** finds **no obvious
marginal/pairwise correlation**; `max_abs_pearson_centered = 0.004880432860077769`
(`pre_signature_projection_mask_independence_probe_n32_q12289.json`). The simulator's live-slot
rule states the 6 masked residues may be **replaced with independent uniform Q12289 residues
under the mask-PRF assumption** — i.e., the masking is genuinely hiding pre-spend.

> The named test `projection_mask_prf_frame_is_disjoint_from_pad_prf_frame` does **not** exist
> in `production_reference.rs` by that literal name. The disjointness is realized instead by the
> distinct domain constants (`237` vs `242`), the `#[cfg(test)]` frame helper
> `reviewed_fq12_packed_witness_state18_projection_mask_prf_input_frame` (`6396-6421`), and the
> executable `mask_frame_disjointness` artifact. Substance present; the exact test name claimed
> in the prompt is not. Minor doc/name drift, not a substantive gap.

## CLAIM 3 — Residual obligation. Pad-hiding is (b) TRACKED-BUT-INCOMPLETE, modulo named assumptions.

From `pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json` (fail-closed,
20 open items, 13 named breaks) and `hidden_source_ro_pok_leakage_global_composition_theorem`:

**5 CARRIED ASSUMPTIONS:**
1. `A_sha256_projection_mask_frame_prf` — `Adv_sha256_projection_mask_frame_prf(q_hash, t)`.
   *not_discharged_explicit_assumption_carried.* **This is the load-bearing pad-hiding assumption:
   the masks hide the pad iff SHA-256 is a good frame-PRF on the mask domain.**
2. `A_global_domain_registry` — `Adv_frame_disjointness(q_hash)`. Registered, no exact collisions,
   needs byte-level disjointness proof across all frames. (Supports pad/mask separation.)
3. `A_lifecycle_same_game` — `Adv_postchallenge_slot_creation + Adv_pad_seed_duplicate +
   Adv_witness_rewrite + Adv_projection_overread`. Partially discharged on the reference path.
   **Pad-relevant:** `Adv_pad_seed_duplicate` (pad-seed reuse across slots) is the principal
   pad-hiding lifecycle risk — the mask is only fresh if the pad seed is single-use per slot.
4. `A_spend_time_projection_budget` — `Adv_spend_opening_leakage(q_open)`. fail-closed not proved;
   needs helper/API surface restriction + no-export negative tests so the pad/mask are not
   exported pre-spend.
5. `A_reject_equivalence` — `Adv_reject_mismatch(q_reject, t)`. Constant-time uniform reject so
   rejection timing is not pad-correlated.

**Most load-bearing OPEN items for PAD-HIDING specifically (of the 20):**
- (1) `sha256_instantiation` — accept/replace the mask frame-PRF assumption. **Top pad-hiding gate.**
- (7) `pad_seed_digest_uniqueness_implementation` — enforce pad-seed single-use so the mask is fresh.
- (8) `packed_witness_immutability_negative_control` — masked witness fixed before challenge.
- (12/13) `no_pre_spend_projection_export_{static_boundary,negative_tests}` — prove the unmasked
  projection / masks are never exported before an accepted spend.
- (2) `lifecycle_same_game_lift` — lift the lifecycle (pad-seed reuse / reorg / restore) into the
  same-game proof.

Global theorem `ro_pok_leakage_global_composition_or_break` is **conditional** on external
acceptance of the 32-byte RO-PoK assumption; the carrier-handle leakage channel explicitly lists
`"pad seed"` in `must_not_reveal` with closure conditioned on the RO-PoK zero-knowledge/leakage
bound. **Pad-hiding is SPECIFIED, REDUCED to named assumptions, and TRACKED — not unaddressed,
and not fundamentally broken.**

## CLAIM 4 — Is `active_pad_constraint` charged in the DEPLOYABLE budget? YES (two-phase), and hiding survives pre-spend.

The scaffold cert excluded the pad and never charged it. The deployable budget
(`pre_signature_projection_spend_time_leakage_budget.py`) is **two-phase** and explicit:

- **Pre-spend (`pre_spend_live_slot_budget`):** the 6 packed projection slots are charged ONLY as
  `masked_projection = account_projection + mask mod q`; pad seed bytes, the 6 masks, and the 6
  unmasked residues are all **forbidden public values**. The live-slot simulator may swap the
  masked residues for independent uniform residues under the mask-PRF assumption ⇒ **hiding holds
  pre-spend with the pad charged-as-masked.**
- **Accepted-spend (`spend_opening_budget`):** the 6 selected projection residues are
  *intentionally* disclosed and **charged** as spend-opening disclosure
  (`selected_projection_residues_intentionally_public_after_spend: true`); the doc is explicit this
  is "not a claim" to hide them post-spend — they are charged, and replay is prevented by
  transcript/lifecycle binding.

So unlike the scaffold, the pad disclosure IS accounted: masked (and simulator-replaceable)
pre-spend, charged-disclosure post-spend. A raw-pad-seed-reveal alternative exists but is a
**different/weaker security model** the deployable path does NOT use
(`pre_signature_projection_spend_time_pad_seed_reveal`: 340 B,
`"feasible_only_as_a_different_security_model"`).

---

## VERDICT: CONDITIONAL-GAP-FIXABLE

Inherited? **NO.** The deployable Fq12 design does not inherit the scaffold's pad-in-clear leak.
The fatal scaffold properties — constant mask `[12, Q-12]`, static keygen seed, openings publishing
`const + pad(seed)` key-free — are **all absent**: the mask is a per-slot per-lane SHA-256 frame-PRF
on a domain disjoint from the pad PRF, the serialized surface is digest/masked-only, and the pad
disclosure is charged (masked pre-spend, accounted post-spend). The residual is the already-scoped,
fail-closed composition proof, dominated by one named cryptographic assumption.

**Single concrete next action:** discharge or formally accept carried assumption
`A_sha256_projection_mask_frame_prf` (open item #1, `sha256_instantiation`) via the
`pre_signature_projection_mask_prf_ro_reduction` route — i.e., land the SHA-256-frame-PRF→RO
reduction (or an explicit accept-or-redesign decision in
`pre_signature_projection_sha256_assumption_accept_or_redesign.py`), since pad-hiding for the whole
deployable path reduces to exactly that assumption plus the pad-seed-single-use lifecycle term
(`pad_seed_digest_uniqueness_implementation`).

### Verified code citations (all `tools/parameter-search/src/production_reference.rs` unless noted)
- 777-785 — on-chain opening = digests only (no pad seed/mask/unmasked projection)
- 6376-6394 — PRF-fresh per-slot per-lane projection masks (the crux)
- 46290-46308 — `uniform_mod_from_hash` SHA-256 mask PRF
- 237 vs 242 — pad PRF domain `pad.v1` vs mask PRF domain `projection_mask.v1` (disjoint)
- 45624-45634 — pad PRF block
- 6423-6442 — masked packing (`account_projection + mask mod q`)
- 6854-7043 — `verify_*projection_carrier_tuple`: pad_seed is a verifier witness arg, not serialized
- 49218-49271 — digest-only public spend view + `assert_no_active_native_private_material`
- artifacts: `mask_frame_disjointness` (82 B vs 154 B disjoint), `mask_independence_probe`
  (max |Pearson| = 0.004880), `spend_time_leakage_budget` (two-phase charging),
  `mask_prf_leakage_theorem_composition` (5 carried assumptions / 13 breaks / 20 open),
  `hidden_source_ro_pok_leakage_global_composition_theorem` (conditional, pad in must_not_reveal)
