# GAP C re-model result — second-preimage re-modeling of the binding breaks

**Date:** 2026-06-10
**Scope:** Close GAP C (the union-bound numeric closure being contingent on the
deployment query cap) by re-modeling the three live cryptographic binding
break events from FREE birthday collisions (ε ~ q²/2ⁿ) to TARGET /
SECOND-PREIMAGE games (ε ~ q/2ⁿ), wherever the committed digest/challenge is
genuinely FIXED by an honest party before the adversary acts.

**Bottom line (one line):** **The budget now closes at q = 2⁶⁴ WITHOUT any
classical query cap.** All three live binding breaks (#2, #3, #6) are genuinely
SECOND-PREIMAGE (the verifier recomputes the digest/challenge against a
leaf-fixed committed value before the adversary acts), and #6 (the 214-bit
checked-core, the old dominant term) is **second-preimage-FINE** — no challenge
widening is required.

---

## 1. Per-event free→target justification (verified per event)

The second-preimage re-model is honest ONLY where the target is genuinely fixed
before the adversary acts. We verified this for each event from the production
code and the verifier recompute path.

### #2 prevalidationLeafBinding — SECOND-PREIMAGE (n = 256)
- **Target fixed first:** the prevalidated slot's authorization digest
  `t = SHA256(leaf_frame leaf0)` is committed on-chain at slot-creation time, by
  the honest creator, for the honest leaf `leaf0`.
- **Verifier recomputes against the leaf-fixed value:** at spend time the
  verifier recomputes `d = SHA256(leaf_frame leaf)` and rejects unless `d` equals
  the slot's COMMITTED authorization digest `t` (SlotAuthorizationDigestFramesLeaf
  against the leaf-fixed committed value; the opening_commitment SHA-256 frame
  includes `account_witness_commitment`, production_reference.rs:44266 — verified
  in `creation_time_binding_VERDICT.md`).
- **Therefore:** an attacker wanting a different leaf accepted under the one live
  slot must find `leaf'` with `leaf_frame leaf' ≠ leaf_frame leaf0` and
  `SHA256(leaf_frame leaf') = t`. The target `t` was fixed FIRST — a
  **second-preimage**, not a free birthday.
- **New ε at q = 2⁶⁴:** ε_spr = q/2²⁵⁶ = 2⁶⁴/2²⁵⁶ = **2⁻¹⁹²** (no query cap).

### #3 prevalidationRegistryBinding — SECOND-PREIMAGE (n = 256)
- **Target fixed first:** the honest admission registry commits a UNIQUE leaf
  `committed k` per key k = (accountId, accountStateRootBefore) at slot-creation;
  its authorization digest `t_k = dig(committed k)` is the registry handle. The
  Lean conjunct `LeafFixedBeforeChallenge` is exactly this discipline.
- **Verifier recomputes against the leaf-fixed value:** the verifier accepts a
  presented leaf `lf` at key k iff `dig lf = t_k` (DirectAdmissionContains against
  the leaf-fixed committed value).
- **Therefore:** a forged-handle attack must produce `lf ≠ committed k` with
  `dig lf = t_k = dig(committed k)` — a **second-preimage** on the fixed target.
- **New ε at q = 2⁶⁴:** ε_spr = q/2²⁵⁶ = **2⁻¹⁹²** (no query cap).

### #6 checkedCoreMessageBinding — SECOND-PREIMAGE on a FIXED 214-bit target
*** This is the rigorous target-vs-free determination the task demanded. ***

- **Structure (production_reference.rs):** the checked-core challenge is
  `c = H(challenge_hash_frame(pk, message, slot_id, slot_digest, proof_commitment))`
  (line 43839/43860), H = truncate_and_mask ∘ SHA256 to **214 bits**
  (CHALLENGE_BYTES=27, top 2 bits masked by 0x3f at line 43864). The
  proof_commitment IS inside the frame (line 43847).
- **The verifier (lines 3159–3173):** (1) recomputes `c` from the bound frame
  (which includes proof_commitment), (2) checks
  `response == expected_syndrome(c, handle, proof_commitment)`. The Sigma response
  is bound to BOTH the challenge AND the proof_commitment.
- **The SUF-CMA / message-binding adversary** first obtains an honest signature on
  message m: an honest transcript whose response is valid only against the honest
  challenge `c = H(frame(m, pc))` for the honest proof_commitment `pc`. Minting a
  fresh valid response for a DIFFERENT challenge requires the SIS-hard secret —
  that is a SEPARATE event (response-link SIS solver, family (a)); the
  message-binding adversary does NOT get to mint responses.
- **Is there a FREE-collision surface at 214 bits?** **NO.** A free collision
  would require both m and m′ to be INDEPENDENTLY signable with the same challenge
  with no prior honest signature pinning it — but the response is the honest
  signer's secret-key-bound object, so one message's `(proof_commitment,
  challenge)` is always pinned FIRST. The proof_commitment is in the frame and
  bound to the response, so the adversary cannot swap in a fresh commitment to
  manufacture a free pair. To transfer the honest response to `m′ ≠ m` the
  adversary must keep the SAME pc and SAME c and find
  `frame(m′, pc) ≠ frame(m, pc)` with `H(frame(m′, pc)) = c`. The target
  `c = H(frame(m, pc))` is FIXED by the honest signature BEFORE the adversary
  commits m′ — a **second-preimage on the fixed 214-bit target**.
- **#6 VERDICT: SECOND-PREIMAGE-FINE.** ε_spr = q/2²¹⁴ = 2⁶⁴/2²¹⁴ = **2⁻¹⁵⁰** at
  q = 2⁶⁴ (clears the 2⁻¹³³ per-event budget by 17 bits, no query cap). It even
  clears 2⁻¹²⁸ at q = 2⁸⁰ (2⁻¹³⁴). **NO maintainer challenge-widening decision is
  required for #6;** the deployed 214-bit challenge width is unchanged.

### Structural-zero / unmodeled events (unchanged)
- #1 creationTimeHiddenSource = 0 under the named bridge axiom.
- #5 postchallengeSlotCreationOrRewrite, #7 slotReplayOrRollback = 0 structurally.
- #8 qromOrDomainSeparation, #9 liveSlotLeakageOrKeygen = 0 by construction.
- #4 prevalidationDataAvailability — still no EC reduction (placeholder; at n=256
  it now tracks leaf/registry as a second-preimage, i.e. ≤ 2⁻¹⁹² at q=2⁶⁴, but
  this is NOT yet mechanized).

---

## 2. The re-modeled EC reductions (machine-checked, exit 0)

| event | file | game | reduction lemma (PROVEN) | named axiom | ε at q=2⁶⁴ |
|---|---|---|---|---|---|
| #2 leaf | `break_prevalidation_leaf_binding.ec` | SPR (target `t` fixed at `init`) | `break_implies_second_preimage`, `break_le_spr`, `break_le_spr_bound` | `sha256_second_preimage_resistance` (ε_spr ~ q/2²⁵⁶) | 2⁻¹⁹² |
| #3 registry | `break_prevalidation_registry_binding.ec` | SPR (honest `committed` fixed first) | `step_break_sets_second_preimage`, `run_break_implies_second_preimage`, `B_wins_when_break`, `break_le_spr`, `registry_break_bound` | `sha256_second_preimage_resistance` (ε_spr ~ q/2²⁵⁶) | 2⁻¹⁹² |
| #6 checked-core | `break_checked_core_message_binding.ec` | SPR (honest frame `f0` fixed first) | `break_yields_second_preimage`, `break_le_spr`, `break_le_bound` | `sha256_second_preimage_resistance` (~ q/2²¹⁴) | 2⁻¹⁵⁰ |

The CR game (adversary outputs BOTH preimages) is GONE in all three; each now
fixes the honest target FIRST (`init` records the committed honest frame/leaf;
the adversary submits a candidate that must hit that fixed target). The
break⟹second-preimage-win implication is PROVEN in each (no admit, no sorry).
Each carries EXACTLY ONE named hardness axiom (SHA-256 second-preimage
resistance) plus the structural byte-encoding-injectivity facts.

---

## 3. Concrete ε wired into the spine (machine-checked, exit 0)

`formal/easycrypt/gap_c_remodel_spine_wiring.ec` binds the concrete bound
`ε_spr(n) = q / 2ⁿ` at `q = 2⁶⁴` and PROVES:

- `eps_spr_le_2pow_neg133` (general): for n with `2⁶⁴ ≤ 2^(n−133)`,
  `ε_spr n ≤ 2⁻¹³³`.
- `eps_checked_le_2pow_neg133`: n=214 ⇒ **2⁻¹⁵⁰ ≤ 2⁻¹³³** ✓ (no cap).
- `eps_leaf_le_2pow_neg133`, `eps_registry_le_2pow_neg133`: n=256 ⇒
  **2⁻¹⁹² ≤ 2⁻¹³³** ✓ (no cap).
- `remodelled_budget_closes`: the summed budget
  `eps_accepted(2⁻¹²⁹) + eps_leaf(2⁻¹⁹²) + eps_registry(2⁻¹⁹²) + eps_checked(2⁻¹⁵⁰)`
  + structural zeros **≤ 2⁻¹²⁸** ✓ at q = 2⁶⁴.
- `remodelled_per_event_meets_spine_hyp`: each re-modeled break term ≤ 2⁻¹³³ and
  accepted-spend ≤ 2⁻¹²⁹ — EXACTLY the hypothesis of
  `spike_union_bound.success_le_2pow_neg128`, so feeding these concrete
  second-preimage bounds into the spine yields `successProb ≤ 2⁻¹²⁸`.

All real arithmetic discharged by EC + Z3 (explicit exponent expansions; Z3
cannot expand symbolic powers). No admit, no sorry.

---

## 4. Summed budget at q = 2⁶⁴ (no query cap)

| term | model | log₂ ε | ≤ 2⁻¹³³? |
|---|---|---:|:--:|
| #2 leaf (256) | q=2⁶⁴, **second-preimage** | **−192** | yes |
| #3 registry (256) | q=2⁶⁴, **second-preimage** | **−192** | yes |
| #6 checked-core (214) | q=2⁶⁴, **second-preimage** | **−150** | yes |
| #1, #5, #7, #8, #9 | structural | −∞ (=0) | yes |
| accepted-spend | placeholder (AP2-N003) | −129 | (budget term) |

**TOTAL ≈ 2⁻¹²⁸·⁹⁹ ≈ 2⁻¹²⁹ (dominated by the accepted-spend placeholder); the
re-modeled break terms together contribute ≈ 2⁻¹⁵⁰, far below the budget.**
**Budget CLOSES at q = 2⁶⁴ with NO query cap.** It also closes at q = 2⁸⁰
(checked-core → 2⁻¹³⁴, still under 2⁻¹²⁸, and the accepted-spend placeholder
2⁻¹²⁹ remains the dominant term).

Contrast with the old FREE-birthday model (`per_event_epsilon_budget_closure.md`):
there the checked-core term was 2⁻⁸⁶ at q=2⁶⁴ (blowing the budget by 42 bits) and
closure required q_cl ≤ 2³³. The re-model removes that query-cap dependence for
the re-modeled events.

---

## 5. Honesty boundary / what remains

- The re-model is valid because the target IS genuinely fixed before the
  adversary acts in all three events (verified per event in §1 against the
  verifier recompute-against-leaf-fixed code). This is the operationally CORRECT
  model, not a weakening.
- The named hardness is now SHA-256 **second-preimage** resistance (ε ~ q/2ⁿ)
  rather than collision resistance (ε ~ q²/2ⁿ). Both are standard NIST FIPS 180-4
  assumptions; second-preimage is the stronger property of SHA-256 to lean on but
  is universally assumed and is the operationally faithful one here.
- `prevalidationDataAvailability` (#4) still has NO EC reduction (placeholder).
- `accepted-spend` (2⁻¹²⁹) is a placeholder, separately fail-closed pending
  AP2-N003; this analysis did not re-derive it. It is now the DOMINANT term in
  the closed budget.
- The spine file (`spike_union_bound.ec`) still carries the OB-1/OB-2 coupling
  axioms (the Lean↔EC seam); the re-model does not touch those — it supplies the
  concrete per-event break bounds those obligations consume.

---

## 6. Maintainer decisions flagged

**NONE required for the binding/challenge widths.** The 214-bit checked-core
challenge (#6) is second-preimage-FINE at q=2⁶⁴ (2⁻¹⁵⁰) and even q=2⁸⁰
(2⁻¹³⁴); the deployed challenge width was NOT changed. No widening to ≥274 bits
is needed.

The only standing maintainer items are pre-existing and unrelated to GAP C
binding closure: discharge AP2-N003 (accepted-spend term, now the dominant
2⁻¹²⁹), and provide an EC reduction for #4 prevalidationDataAvailability.

---

## 7. Typecheck status (EasyCrypt 2026.05 / Z3 4.16.0)

All `formal/easycrypt/*.ec` exit 0 (no regressions):

```
break_checked_core_message_binding.ec            EXIT 0   (re-modeled → SPR, 214-bit)
break_creation_time_hidden_source.ec             EXIT 0
break_postchallenge_slot_creation_or_rewrite.ec  EXIT 0
break_prevalidation_leaf_binding.ec              EXIT 0   (re-modeled → SPR, 256-bit)
break_prevalidation_registry_binding.ec          EXIT 0   (re-modeled → SPR, 256-bit)
break_slot_replay_or_rollback.ec                 EXIT 0
gap_c_remodel_spine_wiring.ec                    EXIT 0   (NEW — concrete ε wiring)
spike_union_bound.ec                             EXIT 0
```
