# RISK-FIRST VERDICT — Creation-Time Full-Witness-Opening Binding

**Date:** 2026-06-09 · **n=32, q=12289, Fq12** · **Role:** SYNTHESIS of 3 independent attacks
**Target claim:** does the creation-time certificate (esp. `masked_state18` repack-to-fixed-commitment)
make the account-witness map INJECTIVE — closing the spend-only two-witnesses survivor
(`projection_carrier_tuple_does_not_prove_full_witness_opening`, lib.rs:2205)?

---

## VERDICT: SOUND for the WITNESS-BINDING (injectivity) sub-claim — but the overarching theorem is CONDITIONAL/GAP, and the gap is NOT in binding.

The creation-time certificate **does** make the account-witness opening map injective. The
spend-only two-witnesses survivor **cannot occur at creation time**. This is SOUND modulo named
standard assumptions. **It does NOT, however, discharge the full theorem**
`direct_replayable_relation_certificate_soundness_or_break` — the residual gap is relation-validity
and data-availability, which binding was never claimed to close.

This is a clean RISK-FIRST result: the riskiest binding obstruction (the survivor that spend-only
provably leaves open) is **killed**, and the analysis precisely relocates the remaining risk so the
maintainer attacks the right thing next.

---

## DO THE THREE ATTACKS AGREE? — YES, unanimously, with NO material contradiction.

| | Attack 1 (empirical survivor search) | Attack 2 (structural rank) | Attack 3 (formal characterization) |
|---|---|---|---|
| **Survivor verdict** | no-survivor / binding-holds (high) | no-survivor / binding-holds (high) | no-survivor / binding-holds (high) |
| **Why injective** | leaf pins ONE commitment + 18-residue packing is a bijection | opening_commitment SHA-256 over full 18-residue frame + slot/checked-core | SHA256-CR on opening frame + UNCONDITIONAL bijection of state18 packing |
| **`masked_state18` repack role** | tautology (masked=(masked−mask)+mask); adds NO independent constraint | NOT sufficient alone; leaves 12-dim syndrome-lane nullity | repack IS a binding opening, but binding power is the leaf-fix, not the repack |
| **Residual** | SHA-256 collision (named break) | SHA-256 CR + checked-core/QROM (named breaks) | (A) 52-column coverage + (B) relation soundness/availability |

All three converge on the **same mechanism** and the **same residual**. The apparent "disagreement"
is only emphasis, and it is the **most important finding**, so it is reconciled explicitly below.

### The one nuance the three surface (and it is correct): WHERE the injectivity actually comes from
Attack 1 calls the `masked_state18` repack a **tautology**; Attack 2 shows `masked_state18` **alone**
leaves a **12-dimensional syndrome-lane nullity**; Attack 3 confirms the repack is a binding opening
but that its power is the **leaf-fixing**, not the repack identity. These are consistent: the repack
re-derives `account_projection = masked − mask (mod q)` and re-packs — an algebraic identity that
constrains nothing **by itself**. **Injectivity is produced by the leaf**, via two hops, both verified
in code this session:

1. **opening_commitment = SHA256(frame)** where the frame **includes `account_witness_commitment`**
   (`production_reference.rs:44265`, confirmed: `frame.extend_from_slice(&opening.account_witness_commitment)`),
   checked equal to leaf and slot (26311 / 36381). Under SHA-256 CR a leaf-fixed `opening_commitment`
   pins exactly ONE frame, hence ONE `account_witness_commitment`.
2. **`account_witness_commitment` → 18 residues is an UNCONDITIONAL bijection** (no assumption):
   pack writes 18×14 = 252 bits; `unpack_reviewed_fq12_packed_witness_state18` (46019, confirmed)
   **rejects any of the 4 slack bits set** (`any_bits_set_le`) **and any residue ≥ q** — exactly one
   canonical 18-tuple per commitment. The 6-lane `account_projection` is therefore **read out of the
   fixed commitment**, uniquely determined per leaf; the repack (6556, confirmed:
   `if &repacked_witness_commitment != witness_commitment { return Err(... repack mismatch) }`)
   fails closed.

**Most credible framing for the maintainer = Attack 2's mislocation warning.** It is the one with
operational teeth: **do NOT cite `masked_state18` repack alone as the survivor-closer** (decision-doc
line 119 phrases it that way and is, standalone, insufficient). The load-bearing closer is the
**opening_commitment SHA-256 leaf binding + checked-core sourcing**. Attacks 1 and 3 agree on the
mechanism; Attack 2 names the documentation/regression hazard. None contradicts the SOUND verdict.

---

## EVIDENCE (verified this session, not merely quoted)

- **Ground-truth Rust tests PASS (re-run):** `projection_carrier_tuple_does_not_prove_full_witness_opening`,
  `packed_witness_state18_projection_carrier_closes_projection_rebinding`,
  `masked_witness_state18_unmasks_projection_with_pad_seed` → **3 passed; 0 failed.** The spend-only
  survivor accepts two witnesses; the creation/masked path **rejects** the cross-commitment carrier
  move ("exported a hidden-material break").
- **Survivor-search JSON** (`creation_time_witness_binding_survivor_search_n32_q12289.json`):
  `creation_survivor_found: false`; bijection 20000/20000 distinct→distinct + round-trip, **0 collisions**;
  repack tautology 2000/2000; the lib.rs:2205 pair (`witness_a` syndrome=0 vs `witness_b` syndrome[0]=1)
  maps to **two DISTINCT opening_commitments** (`be32f0…` vs `e68400…`) while the leaf pins exactly one ⇒
  at most one survives a given leaf.
- **Code, line-checked this session:** opening frame includes `account_witness_commitment` (44265);
  unpack guards 4 slack bits + residue<q (46019–46043); masked repack asserts repacked==commitment,
  fail-closed (6556).
- **Active-subspace replay JSON:** `active_projection_nullity: 0`, `old_projection_columns: 64`,
  `old_inactive_projection_columns: 52` — the 12 active lanes have **no internal nullity**; 52 inactive
  source columns are **excluded** as non-public hidden material (this is the *separate* claim, see gap).

---

## THE ASSUMPTION(S) — and the now-tractable theorem path

The witness-binding injectivity reduces cleanly to, and ONLY to:

1. **SHA-256 collision resistance** on `reviewed_fq12_state_assisted_opening_commitment_frame`
   (and the `witness_opening_digest` frame) — maps to **`prevalidation_leaf_binding_break` /
   `qrom_or_domain_separation_break`**.
2. **Unconditional bijectivity** of the 18-residue 14-bit packing — **NO new commitment assumption**;
   proven by construction (252 packed bits, 4 guarded slack bits, residue canonicity).

Under these named assumptions, the injectivity lemma is **provable**, and the theorem branch
"creation-accept ⇒ a UNIQUE account-witness opening per leaf" is **tractable now**.

---

## THE RESIDUAL GAP (why the *overarching* theorem is CONDITIONAL, not yet SOUND)

Binding ≠ soundness. Injectivity proves **uniqueness** of the opening, **not** that the unique opening
is **relation-valid**. Two live obstructions remain (all three attacks flag at least one; Attack 3
states both fully). **These are the RISK-FIRST items to attack next:**

- **(A) COVERAGE.** `masked_state18` binds only **18 of 64** hidden-source coordinates (12 syndrome +
  6 account-projection). The **52 inactive source columns are unbound**; their irrelevance rests on the
  **separate** active-subspace nullity-0 + verifier-exporter bridge claim — substance of
  **`creation_time_hidden_source_soundness_break`**, not commitment binding.
- **(B) RELATION SOUNDNESS / DATA AVAILABILITY.** The active-lane masked-projection soundness
  certificate consumes **hidden harness inputs**, and the public `DirectReplayableRelationCertificateV1`
  payload remains **unimplemented / fail-closed**. Relation-soundness/extractor is **undischarged**.
  **Tracker confirms:** "Prove account creation implies relation validity or named break" is **`[~]`**
  (line 130), not `[x]`; `prevalidation_data_availability_break` / `creation_time_hidden_source_soundness_break`
  remain **OPEN**.

None of these is a binding-survivor. They are the *correctly relocated* risk.

---

## MANDATORY REGRESSION (Attack 2's teeth — add to prevent silent reopening)

Add a test asserting **`opening_commitment` CHANGES under a syndrome-only witness mutation**. If a
future refactor (a) drops `account_witness_commitment` from the opening_commitment frame, or
(b) accepts a **verify-time / non-slot-sourced** witness commitment, the **12-dim syndrome-lane nullity
survivor reopens immediately**. The injectivity is leaf-binding-dependent, not intrinsic to the carrier.

---

## SINGLE CONCRETE NEXT ACTION

**Attack relation-validity, not binding.** Specifically: discharge **(B)** — implement/replay the
public `DirectReplayableRelationCertificateV1` and prove the active-lane masked-projection soundness
certificate is a **sound extractor** for the 12 checked-core syndrome lanes (then close (A)'s
inactive-52-column exclusion via the verifier-exporter bridge). Concretely, **write the negative
control that feeds a creation certificate whose unique-but-RELATION-INVALID account_projection still
repacks to the leaf-fixed commitment, and confirm the soundness certificate REJECTS it** — if it
accepts, `creation_time_hidden_source_soundness_break` is real and forces redesign now; if it rejects,
the relation arm of `direct_replayable_relation_certificate_soundness_or_break` is on track.
And, immediately and cheaply: land the `opening_commitment`-changes-under-syndrome-mutation regression
test above.
