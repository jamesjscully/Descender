# Per-event ε budget closure — GAP C numeric-closure analysis

> MIGRATION STATUS (2026-06-20): `ACTIVE_RESEARCH`.
> This document records the historical free-birthday analysis and later
> correction. Current security-level policy and claim wording live in
> `../../spec/security_model.md` and ADR-0006.

> ## ✅ RESOLVED (2026-06-10) — HIGH-3 multi-target widening
> The historical analysis below is the **free-birthday** model that motivated the
> GAP-C re-model; it is retained for the record. **Final state:** the binding
> events were re-modelled as **second-preimage** games against an honestly-fixed
> target, and (HIGH-3) charged at the correct **MULTI-TARGET** rate
> **T·eps_spr(n) = T·q/2ⁿ** over the T = 2³² signing-query horizon. The checked-
> core challenge was **WIDENED to n = 246 effective bits** (`CHALLENGE_HARDENING_-
> BYTES` 4→8 ⇒ `CHALLENGE_BYTES` 27→31, +4 sig bytes; 182 algebraic-after-mask +
> 64 hardening = 246). At T=2³², q=2⁶⁴ the dominant multi-target term is
> **T·q/2²⁴⁶ = 2⁻¹⁵⁰ ≤ 2⁻¹³³** (was 2⁻¹¹⁸ at n=214 — a breach). Machine-checked:
> `break_checked_core_message_binding.ec` `multi_target_break_le_bound` +
> `gap_c_remodel_spine_wiring.ec` `multi_target_le_2pow_neg133` /
> `remodelled_budget_closes` (sum 2⁻¹²⁹ + 2⁻¹⁶⁰ + 2⁻¹⁶⁰ + 2⁻¹⁵⁰ + zeros ≤ 2⁻¹²⁸),
> all `.ec` exit 0. **No usage cap beyond the multi-target horizon T ≤ 2³².**

**Date:** 2026-06-10
**Scope:** Decide whether the union-bound spine
(`formal/easycrypt/spike_union_bound.ec`) actually closes at **deployed
parameters** — each break event ≤ 2^-133, accepted-spend ≤ 2^-129, sum ≤
2^-128 — once the per-event ε's are computed at the **deployed digest widths**
under an **explicit, stated query model**.

**Verdict (one line):** **CLOSES under the deployed query cap (q_cl = 2^33,
q_q = 2^16) with ~1 bit of margin; FAILS catastrophically at q = 2^64 / 2^80.**
The closure is entirely contingent on the deployment query cap. Every binding
break is a **FREE birthday collision**, not a target/second-preimage, so the
relevant exponent is q²/2^n (classical) / q³/2^n (quantum BHT), and at a large
free-query model the budget blows by tens of bits.

---

## 1. The 9 break events and their game classification

The `BreakEvent` family (`Descender/Path2FullSpendGame.lean:63-72`) has 9
members. Each EC reduction (`formal/easycrypt/break_*.ec`) was read; the Rust
digest construction (`tools/parameter-search/src/production_reference.rs`)
fixes the widths.

| # | BreakEvent | Reduction file / game | Game kind | n (bits) | Reduction status |
|---|---|---|---|---|---|
| 1 | `creationTimeHiddenSourceSoundness` | `break_creation_time_hidden_source.ec` | verifier-exporter **bridge** (NOT a hash collision); =0 under one named bridge axiom | — | =0 given bridge axiom |
| 2 | `prevalidationLeafBinding` | `break_prevalidation_leaf_binding.ec` | SHA-256 CR, adversary outputs **both** preimages → **FREE birthday** | **256** | reduction PROVEN, ε = `eps_cr` (unbound) |
| 3 | `prevalidationRegistryBinding` | `break_prevalidation_registry_binding.ec` | SHA-256 CR, `B(A)` outputs **both** leaves → **FREE birthday** (honest `committed` exists but reduction picks `fun _ => witness`) | **256** | reduction PROVEN, ε = `eps_cr` (unbound) |
| 4 | `prevalidationDataAvailability` | (none yet) | mixed CR + data-availability | 256 | NOT yet reduced (per `break_reduction_status.md`); placeholder |
| 5 | `postchallengeSlotCreationOrRewrite` | `break_postchallenge_slot_creation_or_rewrite.ec` | structural state-machine | — | **=0, no assumption** |
| 6 | `checkedCoreMessageBinding` | `break_checked_core_message_binding.ec` | truncated/masked SHA-256 CR, adversary outputs **both** frames → **FREE birthday** | **214** | reduction PROVEN, ε = `sha256_collision_bound` (unbound) |
| 7 | `slotReplayOrRollback` | `break_slot_replay_or_rollback.ec` | structural consume-once | — | **=0, no assumption** |
| 8 | `qromOrDomainSeparation` | (wired False in concrete classifier) | — | — | =0 by construction |
| 9 | `liveSlotLeakageOrKeygen` | (wired False in concrete classifier) | — | — | =0 by construction |

**Decisive finding (FREE vs TARGET):** the three live cryptographic binding
events (#2, #3, #6) are **FREE birthday collisions**, NOT
target/second-preimage. The EC CR games literally let the adversary output
**both** sides:

- `break_prevalidation_leaf_binding.ec` — `CR.submit(m1,m2)` wins on
  `m1 <> m2 /\ sha256 m1 = sha256 m2`; the "committed" `live_digest` is set by
  the adversary's **own first spend**, so there is no honestly-fixed target.
- `break_prevalidation_registry_binding.ec` — `CR.main` wins on
  `x <> y /\ dig x = dig y` and the reduction runs with
  `committed = fun _ => witness`; both colliding leaves are adversary-reachable.
- `break_checked_core_message_binding.ec` — `CR.main` wins on
  `f1 <> f2 /\ H f1 = H f2`; both frames are adversary-chosen.

This matters because the per-event ε is q²/2^n (free birthday), **not** q/2^n
(target). At n=256 the gap is the difference between 2^-128 (free, q=2^64) and
2^-192 (target, q=2^64).

NOTE on operational reality: events #2/#3 *could* be re-modelled as
**target** collisions (the honestly-admitted leaf/registry entry is fixed by an
honest party before the adversary acts, so the attacker faces a
second-preimage, not a free collision). The EC games as written do NOT enforce
that — they state the strictly weaker free-CR game. The closure verdict below
uses the games **as written** (free birthday), which is the honest, conservative
reading. If the games were tightened to target/second-preimage, n=256 events
would be ~2^-223 even at q=2^33 and the budget would close with enormous margin
under *any* query model.

---

## 2. Deployed digest widths (from `production_reference.rs`)

- `DIGEST_BYTES = 32` (line 187) → `path2_slot_authorization_digest`
  (line 26075) and `prevalidated_slot_leaf_digest` are **full SHA-256, n = 256**
  (`hasher.finalize().into()`, no truncation). Events #2, #3.
- Checked-core challenge (`hash_frame_to_challenge_bytes`, line 43860):
  `CHALLENGE_BYTES = ALGEBRAIC_CHALLENGE_BYTES(23) + CHALLENGE_HARDENING_BYTES(4)
  = 27 bytes = 216 bits`, then `out[ALGEBRAIC_CHALLENGE_BYTES-1] &= 0x3f` clears
  the top **2 bits** → effective collision width **n = 214 bits**. Event #6.
  This is the **narrowest** committed width and therefore the dominant term.

---

## 3. Per-event ε arithmetic (free birthday = q²/2^n; quantum BHT = q³/2^n)

Two query models stated explicitly:

- **Deployment cap** (the ledger's stated model,
  `research/path2_hash_qrom_assumption_ledger.md:131-134`): classical hash-query
  cap `q_cl = 2^33`, quantum cap `q_q = 2^16`. (This is a *deployment-enforced*
  cap, not a generic adversary; it is the model the project actually claims.)
- **Adversarial generic models**: classical `q = 2^64`, `q = 2^80`; quantum BHT
  `q = 2^40`, `q = 2^64`.

| Event (n) | model | log2(ε) | ≤ 2^-133? |
|---|---|---:|:--:|
| leaf / registry (256) | q_cl = 2^33, free | **−190** | yes |
| checked-core (214) | q_cl = 2^33, free | **−148** | yes |
| checked-core (214) | q_q = 2^16, BHT | **−166** | yes |
| leaf / registry (256) | q = 2^64, free | **−128** | **NO** |
| checked-core (214) | q = 2^64, free | **−86** | **NO** |
| checked-core (214) | q = 2^80, free | **−54** | **NO** |
| checked-core (214) | q_q = 2^64, BHT | **−22** | **NO** |

(`creationTimeHiddenSource` = 0 under bridge axiom; `postchallenge`,
`slotReplay`, `qrom`, `liveSlot` = 0 structurally; `dataAvailability`
unmodelled — at n=256 free it tracks leaf/registry.)

---

## 4. Sum and dominant term

Active terms = 3 free-birthday collisions (#2,#3,#6) + accepted-spend (placeholder
2^-129, separately fail-closed pending AP2-N003). All others = 0.

| query model | each break worst | TOTAL log2 | ≤ 2^-128? |
|---|---:|---:|:--:|
| **deployment cap** (q_cl=2^33, q_q=2^16) | −148 (checked-core) | **−129.0** | **YES (≈1 bit margin)** |
| classical q = 2^64 | −86 (checked-core) | **−86.0** | **NO** |
| classical q = 2^80 | −54 (checked-core) | **−54.0** | **NO** |
| quantum BHT q = 2^64 | −22 (checked-core) | **−22** | **NO** |

**Dominant term in every model: the checked-core message-binding break (#6),
ε = q²/2^214 classical** — because the checked-core challenge is truncated +
masked to **214 bits**, the narrowest committed width in the construction. At
the deployment cap it is 2^-148 (still under 2^-133); at q=2^64 it is 2^-86,
which alone blows the entire 2^-128 budget by 42 bits.

This exactly matches the ledger's own "transcript term after QROM loss = 130
bits, minimum deployment margin = 1 bit" line
(`path2_hash_qrom_assumption_ledger.md:133-135`): the construction is
deliberately sized to clear 128 bits **only under the deployment query cap**,
with ~1–2 bits to spare. The spike's `breakBound ≤ 2^-133` per event is met at
the deployment cap (worst is 2^-148) — so the spike's `success_le_2pow_neg128`
arithmetic is sound **conditional on q_cl ≤ ~2^39 and q_q ≤ ~2^16**.

---

## 5. Verdict

**CLOSES at deployed parameters under the deployment query cap model
(q_cl = 2^33 classical, q_q = 2^16 quantum).** Total ≈ 2^-129, inside 2^-128.
Each individual break term ≤ 2^-148 < 2^-133. The spike spine's per-event
2^-133 hypothesis is satisfiable at these widths **only because q is capped**.

**Does NOT close** at a generic free-query adversary (q = 2^64 or 2^80
classical; q = 2^64 BHT quantum). The dominant checked-core term degrades to
2^-86 (q=2^64) / 2^-54 (q=2^80), violating 2^-133 by 47–79 bits.

### The free-birthday risk, stated loudly

**All three live binding breaks are FREE birthday collisions, and the
checked-core one (#6) collides on a 214-bit truncated/masked digest.** A free
birthday at n=214 hits 2^-133 only when q ≤ 2^40.5; the 2^-128 budget survives
only because the deployment enforces q_cl = 2^33. **The 2^-133-per-event claim
is NOT a property of "256 bits is plenty" — it is purchased entirely by the
2^33 query cap, and the cap is load-bearing.**

### Required for an unconditional close (any of):

1. **Justify and pin the deployment query cap** as a hard, enforced consensus
   parameter (q_cl ≤ 2^39, q_q ≤ 2^16) — this is what the ledger already
   asserts; it must be a genuinely enforced rate limit, not an assumption about
   adversary politeness. This is the cheapest fix and matches current design.
2. **Re-model events #2/#3/#6 as TARGET / second-preimage games** in the EC
   reductions (bind one side to the honestly-admitted leaf / honestly-recomputed
   challenge, which the verifier *does* fix before the adversary acts). Then ε =
   q/2^n: n=256 events → 2^-192 at q=2^64; the 214-bit checked-core → 2^-150 at
   q=2^64, 2^-134 at q=2^80 — closes for any practical q without a query cap.
   This is the *correct* model operationally (the verifier recomputes against a
   leaf-fixed value) and is the recommended honest fix.
3. **Widen the checked-core challenge** from 214 to ≥ 274 effective bits if it
   must stay a free-collision target under q = 2^80 (274 = 2·80 + 114). This is
   the most expensive (transcript/signature size) and only needed if (1) and (2)
   are both rejected.

### Caveats / unfinished

- `prevalidationDataAvailability` (#4) has **no** EC reduction yet; its term is a
  placeholder. At n=256 free it matches leaf/registry.
- `accepted-spend` term is a placeholder 2^-129, separately fail-closed pending
  **AP2-N003**; this analysis did not re-derive it.
- The EC files prove the *reduction implications* (break ⇒ CR-win) but leave
  `eps_cr` / `sha256_collision_bound` as **unbound `op`s** — the per-event ε is
  NOT numerically wired into the spine. This document supplies the missing
  arithmetic; wiring `eps_cr := q²/2^n` at the stated cap into the spike's
  `breakBound` is the remaining mechanization step.
