# Path 2 STARK Route — Highest-Risk Item: the C1/C2 Consensus-SHA ↔ Proof-Friendly-Commitment Binding Bridge

Status: risk-first obstruction analysis for the SELECTED transparent STARK/FRI
route (Family A). Surfaces a redesign fork that needs a maintainer decision; does
NOT auto-apply a redesign. 2026-06-17.

## Why this is the highest-risk active-route item

With Family A selected (see
`research/path2_proof_family_route_clarity.md`), the AoK test returned
CONDITIONAL-PASS. Of the five instantiation-debt conditions, four are bounded
engineering (write the AIR, pin conservative FRI params, rebuild the ZK opening,
carry QROM as a named seam). One is genuinely redesign-forcing and sits under
Q2/Q4/Q6: **how the hidden relation forces C1 and C2 when those are SHA-256
statements over a hidden `pad_seed`.** This is the riskiest because it can force
a change to a *deployed* assumption and to the *spend* verifier — exactly the
class the risk-first policy says to attack first so the redesign lands early.

## The obligations, with measured SHA cost

From `research/path2_direct_creation_hiding_payload_cost_model.md`:

| obligation | meaning | SHA-256 compressions in-AIR | q-ops |
|---|---|---:|---:|
| C1 | `pad_seed` opens to leaf `pad_seed_digest` (SHA-256 preimage over hidden seed) | 1 | 0 |
| C2 | 6 projection masks = SHA-256 PRF(`pad_seed`, slot) | **12** | 6 |
| (opening commitment) | state-assisted opening tuple frame is digest-shaped | 4 | 0 |
| C4 | account projection + pad repack to `packed_witness_commitment` | 0 | 18 |
| C6 | role commitments opened by extracted trace | 0 | small |

Direct-SHA total: **23–55 SHA-256 compressions ⇒ ≈460k–1.65M AIR constraints**,
"SHA-dominated." The 176-row algebraic relation is tiny by comparison. **C2 (the
mask PRF) is the dominant cost** — 12 of the 23 compressions.

Verified in code (2026-06-17):
- `uniform_mod_from_hash` (production_reference.rs:62799) is **SHA-256**-based.
- the mask PRF `reviewed_fq12_packed_witness_state18_projection_masks`
  (:7485) feeds `reviewed_fq12_pack_masked_witness_state18_from_parts` (:7538),
  which builds `masked_projection = account_projection + mask` — i.e. the
  projection lanes (residues 12..17) of the `packed_witness_commitment` that the
  **spend** signature binds. **The C2 mask PRF is shared between the creation
  proof and the spend path.**

## The fork

### Option (a): prove SHA-256 in the AIR

Keep `pad_seed_digest` and the SHA mask PRF unchanged; prove them inside the
hidden relation.

- **Pro:** the existing binding/injectivity theorem is UNCHANGED. The
  `creation_time_binding_VERDICT` proved injectivity *from* SHA leaf-fixing
  (`opening_commitment = SHA256(frame ⊇ account_witness_commitment)` pins ONE
  commitment under SHA-CR) + the assumption-free 18-residue bijective packing. No
  re-proof needed. The spend path and the named-standard assumption
  `A_sha256_projection_mask_frame_prf` are untouched.
- **Con — and it directly undercuts why we picked Family A:** 23–55 SHA-256
  compressions blow up the creation AIR (≈0.5–1.65M constraints) and, worse, make
  the proof **recursion-hostile**. STARK recursion cost is dominated by
  in-circuit hashing of the verifier; baking SHA-256 into the *base* AIR compounds
  that. The aggregation/recursion analysis
  (`research/path2_proof_family_aggregation_recursion_analysis.md`) names this as
  the dominant practical risk and the reason recursion rewards an
  arithmetization-friendly hash. Option (a) sacrifices the recursion advantage
  that was the decisive reason A beat the field.

### Option (b): arithmetization-friendly inner commitments + algebraic PRF

The repo's recommended direction
(`research/path2_proof_friendly_inner_commitment_sketch.md`): keep SHA-256 only
as the *outer consensus handle*; use an arithmetization-friendly hash
(Poseidon/Rescue/Monolith-class — PQ under RO/CR, cheap in-AIR) for the inner
pad-source/opening-tuple commitments and for the **C2 mask derivation**. Cost
model: 0 SHA in-proof, ≈106 q-ops — recursion-friendly.

This is the right answer for "best future blockchain," but it has three
non-trivial, coupled consequences:

1. **Re-prove injectivity from the new binding root.** Demoting `pad_seed_digest`
   to an auxiliary duplicate-detection handle moves the cryptographic pad-source
   binding to an algebraic `pad_source_commitment`. The "ONE commitment is
   pinned" step of the injectivity argument then rests on the algebraic
   commitment's binding, not SHA-CR. The 18-residue bijective packing carries
   over unchanged; the leaf-fixing step must be restated. Scaffold exists:
   `Path2PrevalidatedSlotLeafV2` already adds `creation_proof_binding_digest`
   (+32 B leaf, +0 B spend) so the leaf can bind the algebraic commitment
   directly — this is the clean bridge (b1): the leaf commits both, the algebraic
   commitment is the in-proof root, SHA `pad_seed_digest` becomes auxiliary.
2. **Duplicate prevention moves.** Section-C duplicate controls key on
   `pad_seed_digest` + (account_id, slot_id). Primary duplicate prevention must
   move to `pad_source_commitment` / `creation_proof_binding_digest` and be
   re-verified.
3. **C2 change touches the SPEND verifier and swaps a deployed assumption.**
   This is the cross-cutting consequence and the real maintainer decision.
   Because the mask PRF is shared, replacing SHA-256 mask derivation with an
   algebraic hash changes the `masked_projection` *values* (NOT the byte layout —
   spend stays 262/318 B), the spend verifier's mask recomputation, and replaces
   the named-standard assumption `A_sha256_projection_mask_frame_prf` with an
   algebraic-hash-PRF (RO) assumption that must be re-charged in the EC SUF-CMA
   spine. This is an assumption SWAP, not a soundness loss (both are
   PRF/RO-style), but it is a change to a deployed, externally-review-gated object.

A middle path (b2) keeps the spend mask PRF as SHA and pays only C1's single
in-AIR SHA compression for the consensus link while using algebraic commitments
elsewhere — but it CANNOT avoid C2's 12 compressions, because if the spend masks
remain SHA outputs then proving "masks ARE those outputs" (C2) requires the SHA
PRF in-AIR. **C2 is only cheap if the mask PRF itself becomes algebraic, which is
exactly what touches spend.** There is no free lunch that keeps both the SHA
spend masks and a recursion-friendly creation AIR.

## Recommendation

For the stated goal (best future blockchain), **pursue option (b)** — the
recursion advantage is the entire reason Family A won, and option (a) forfeits
it. Concretely:

1. Adopt an arithmetization-friendly hash (Poseidon-class) for the C2 mask PRF
   and the inner pad-source/opening-tuple commitments; keep SHA-256 as the outer
   consensus handle only.
2. Use the (b1) bridge: promote `Path2PrevalidatedSlotLeafV2`
   (`creation_proof_binding_digest`) to bind the algebraic `pad_source_commitment`
   as the in-proof binding root; demote `pad_seed_digest` to an auxiliary
   duplicate handle.
3. Re-prove the injectivity sub-claim from the algebraic binding root (restate
   the leaf-fixing step; the bijective packing is reused) and move duplicate
   prevention accordingly.
4. Re-charge the spend SUF-CMA EC spine from `A_sha256_projection_mask_frame_prf`
   to the algebraic-hash-PRF assumption.

## Maintainer decision required (do not auto-apply — touches deployed spend path)

The C2 mask-PRF change is the one cross-cutting item. **Approve switching the
projection-mask PRF from SHA-256 to an arithmetization-friendly hash?** It keeps
spend at 262/318 B but changes the spend verifier's mask recomputation and swaps
the deployed frame-PRF assumption. Approving it unblocks a recursion-friendly
creation AIR (option b); declining it forces option (a) (SHA-in-AIR, ≈0.5–1.65M
constraints, recursion-hostile) and keeps the spend path and injectivity theorem
frozen.

This is the obstruction the risk-first sequencing was meant to surface early:
the relation-soundness backend's arithmetization choice reaches back into the
deployed spend signature's mask derivation. It should be decided before any
176-row AIR is written, because it determines the hash used throughout that AIR.

## Non-claims

- Does not instantiate the AIR or any proof system.
- Does not claim Poseidon-class parameters; the family/parameters are a separate
  selection.
- Does not change spend signature bytes under either option.
- Does not re-prove injectivity here; it scopes the re-proof obligation under
  option (b).
