# Creation-Time Binding Soundness Characterization

Status: FORMAL-PROVE pass on `direct_replayable_relation_certificate_soundness`
(the creation-accept ⟹ witness-bound direction), distinguished from the refuted
spend-only version.

Role: characterize PRECISELY the assumption that closes the two-witnesses
survivor at creation time, and state the exact residual gap.

Scope of files read: `research/account_creation_proof_mechanism_decision.md`,
`research/selected_hidden_source_path_decision.md`,
`research/prevalidated_slot_leaf_v1_frame.md`,
`research/account_creation_prevalidated_slot_proof_tracker.md`,
`tools/production-reference/src/lib.rs:2205` (+ the `packed_witness_state18` /
`masked_witness_state18` regression tests at 2301/2458),
`tools/parameter-search/src/production_reference.rs` (packing/unpacking,
opening-commitment frame, slot-opening verifier, masked-state18 verifier,
`verify_path2_prevalidated_slot_creation_payload_bindings`,
`verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate`),
and the active-subspace replay artifact
`hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.{json,md}`.

---

## 1. The spend-only survivor, restated structurally

`projection_carrier_tuple_does_not_prove_full_witness_opening`
(`tools/production-reference/src/lib.rs:2205`) succeeds as an attack because the
spend-only carrier tuple binds only:

- the SAME six-value account projection-opening frame
  (`selected_source_values = [3,5,8,13,21,34]`, one fixed `account_opening_digest`),
- the SAME selected carrier lanes,

but lets the adversary supply TWO DIFFERENT `account_witness_commitment` values
(`witness_a` = packed zero syndrome, `witness_b` = packed syndrome with
`syndrome[0]=1`) and BOTH ACCEPT. The reason is mechanical: in the spend-only
predicate the carrier check/transcript binding is recomputed per-tuple
(`reviewed_fq12_state_assisted_projection_carrier_binding`), and nothing in the
accepting relation pins `account_witness_commitment` to a single externally fixed
value. The witness commitment is a *free input* to the predicate. So the map

    accepting-tuple  ⟶  account_witness_commitment

is NOT a function (one frame, two commitments) — i.e. the account-witness map is
not injective. That is the survivor.

---

## 2. What changes at creation time: the witness commitment stops being free

The structural fact that closes the survivor is that at creation time the
account-witness commitment is no longer a free input. It is determined by a
leaf-FIXED field through a two-hop chain, and — critically — the commitment is a
**bijective bit-packing, not a hash**.

### 2a. The leaf fixes `opening_commitment`, which fixes the opening frame (SHA256 CR)

`PrevalidatedSlotLeafV1` (`prevalidated_slot_leaf_v1_frame.md:46`) carries
`opening_commitment` at offset 192 as a *leaf-fixed* field. The leaf does NOT
carry `account_witness_commitment` directly; it carries

    opening_commitment = SHA256( reviewed_fq12_state_assisted_opening_commitment_frame(opening) )

(`production_reference.rs:44278`). The opening frame
(`production_reference.rs:44258`) is a fixed-width preimage that INCLUDES
`opening.account_witness_commitment` (line 44265) alongside slot_id,
randomizer_commitment, restore_anchor, witness_opening_digest, pad_seed_digest,
and carrier_handle_secret_digest.

`verify_path2_prevalidated_slot_creation_payload_bindings`
(`production_reference.rs:26283`) enforces
`certificate.opening_commitment == leaf.opening_commitment` (line 26311), and
`verify_reviewed_fq12_state_assisted_slot_opening`
(`production_reference.rs:36358`) recomputes the opening-commitment digest from
the supplied opening and requires it to equal the slot's stored
`hidden_source_opening_commitment` (line 36381–36383). Therefore, under SHA-256
collision resistance, the leaf-fixed `opening_commitment` determines the entire
opening frame, and in particular determines `opening.account_witness_commitment`
to a SINGLE value (any second opening frame that hashes to the same
`opening_commitment` is a SHA-256 collision = `prevalidation_leaf_binding_break`).

### 2b. `account_witness_commitment` is a BIJECTIVE encoding of 18 residues

`account_witness_commitment` in the state18 path is produced by
`pack_reviewed_fq12_packed_witness_state18(_from_parts)`
(`production_reference.rs:36850/36865`): it writes 18 residues × `FIELD_BITS=14`
bits = 252 bits into the 32-byte (256-bit) field via `write_bits_le`, leaving 4
slack bits zero. `unpack_reviewed_fq12_packed_witness_state18`
(`production_reference.rs:46019`) reads the 18 fields back and REJECTS if any of
the 4 high slack bits is set (`REVIEWED_FQ12_PACKED_WITNESS_STATE18_SLACK_BITS`)
or if any residue ≥ q.

This is an **invertible encoding**, NOT a hiding/binding hash:

- distinct canonical 18-tuples (each residue in `[0,q)`) differ in some 14-bit
  field ⟹ produce distinct digests (injective forward map);
- every accepted digest decodes to exactly one 18-tuple (the slack-bit and
  canonicity guards make the decode a total inverse on its image).

So `account_witness_commitment ⟷ (12 syndrome residues, 6 account-projection
residues)` is a **bijection** on the canonical set. There is no commitment-CR or
opening-binding assumption needed to invert it — it is information-theoretically
determined. (This is stronger than a SHA256 commitment opening: there is exactly
ONE witness 18-tuple per commitment, unconditionally.)

### 2c. The masked-state18 repack re-derives the SAME commitment

`reviewed_fq12_masked_state18_packed_witness_opening`
(`production_reference.rs:6528`) unpacks the 18 residues, derives the 6
projection masks from `(pad_seed, slot_id)` via a PRF
(`reviewed_fq12_packed_witness_state18_projection_masks`, line 6429), unmasks the
6 account-projection residues, then REPACKS
(`reviewed_fq12_pack_masked_witness_state18_from_parts`) and asserts
`repacked_witness_commitment == witness_commitment` (line 6556), failing closed
otherwise. `validate_reviewed_fq12_masked_state18_packed_witness_opening`
(line 6575) additionally recomputes the opening from
`(witness_commitment, pad_seed, slot_id)` and requires field-for-field equality
plus a frame-digest match. The masking is a per-`(pad_seed, slot_id)`
field-wise additive permutation of the 6 projection residues; it does not change
the bijectivity — it only hides the unmasked projection from a party without the
pad seed. The pad seed is itself pinned by `leaf.pad_seed_digest` (offset 224)
and re-checked in the slot opening (`production_reference.rs:36378`).

---

## 3. Lemma (creation-time witness binding)

> **Lemma (CREATION-WITNESS-BINDING).** Fix a `PrevalidatedSlotLeafV1` `L` with
> leaf-fixed fields `opening_commitment = L.opening_commitment`,
> `pad_seed_digest = L.pad_seed_digest`, `slot_id = L.slot_id`. Suppose the
> creation verifier accepts (`verify_path2_prevalidated_slot_creation_relation_replay`
> ⊇ `verify_path2_prevalidated_slot_creation_payload_bindings` +
> `verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate`)
> for two openings `O, O'` and pad seeds `s, s'` both binding to `L`. Then EITHER
>
>   (i) `O` and `O'` determine the **same** 18-residue witness state
>       `(syndrome[0..12], account_projection[0..6])`, and hence the same active
>       account-projection witness consumed by the hidden-source relation; OR
>
>   (ii) the adversary has exhibited one of the named breaks:
>        - a SHA-256 collision on the opening-commitment frame
>          (`prevalidation_leaf_binding_break`), OR
>        - a SHA-256 collision on the pad-seed opening digest
>          (`prevalidation_leaf_binding_break` / `qrom_or_domain_separation_break`
>          for the mask PRF), OR
>        - a `creation_time_hidden_source_soundness_break` exported by the
>          active-lane masked-projection soundness certificate verifier.

**Proof sketch.** Acceptance forces
`recompute(opening_commitment_digest(O)) == L.opening_commitment` (§2a). If
`O' ≠ O` as opening frames yet both hash to `L.opening_commitment`, that frame
pair IS a SHA-256 collision — case (ii). Otherwise `O = O'` as frames, so
`O.account_witness_commitment` is one fixed 32-byte value. By the bijection in
§2b, that value decodes to one and only one 18-tuple, unconditionally. The masked
opening verifier (§2c) re-derives that 18-tuple and the 6 account-projection
residues deterministically from `(account_witness_commitment, pad_seed, slot_id)`,
all leaf-pinned, and fails closed on any repack/digest mismatch. Hence both
accepting transcripts expose the identical active account-projection witness —
case (i). ∎

**Consequence for the survivor.** The spend-only attack in §1 required two
DIFFERENT `account_witness_commitment` values to both accept against ONE shared
frame. At creation time the commitment is forced to the single value decoded from
`L.opening_commitment`. The masked-state18 regression test makes this concrete:
`masked_witness_state18_unmasks_projection_with_pad_seed`
(`tools/production-reference/src/lib.rs:2458`) builds two witnesses with
`opening_commitment_a != opening_commitment_b` and feeds the carrier of A under
the leaf-fixed commitment of B (line 2675); the verifier REJECTS with
`"reviewed Fq12 masked witness state18 projection carrier tuple exported a
hidden-material break"`. The two distinct commitments the spend test exploited
are now ONE leaf field, so the two-witnesses rebinding cannot occur. (KATs
`projection_carrier_tuple_does_not_prove_full_witness_opening`,
`packed_witness_state18_projection_carrier_closes_projection_rebinding`,
`masked_witness_state18_unmasks_projection_with_pad_seed` all pass:
`cargo test -p descender-production-reference -- does_not_prove_full_witness
masked_witness_state18 packed_witness_state18` → 3 passed.)

---

## 4. Reduction surface (named standard assumptions)

The creation-accept ⟹ unique-witness direction reduces to:

| step | assumption | break event (decision.md:159–164) |
|---|---|---|
| leaf-fixed `opening_commitment` ⟹ unique opening frame ⟹ unique `account_witness_commitment` | SHA-256 collision resistance | `prevalidation_leaf_binding_break` |
| `account_witness_commitment` ⟹ unique 18 residues | **none** (bijective bit-packing; information-theoretic) | n/a |
| `pad_seed_digest` ⟹ unique pad seed; mask-PRF determinism | SHA-256 collision resistance + PRF/QROM of the projection-mask domain | `prevalidation_leaf_binding_break` / `qrom_or_domain_separation_break` |
| 18-residue witness ⟹ active-lane hidden-source relation valid | active-lane masked-projection soundness (the certificate verifier) | `creation_time_hidden_source_soundness_break` |
| payload is actually present and replayable | data availability | `prevalidation_data_availability_break` |
| slot is live / not post-challenge / not key-leaked | lifecycle + leakage ledger | `live_slot_leakage_or_keygen_break` |

The injective-witness step itself is therefore a **clean reduction**: the
"two-witnesses survivor" is closed by (a) SHA-256 CR on the opening-commitment
frame (commitment binding) and (b) the *unconditional* bijectivity of the
state18 packing. No new commitment assumption beyond SHA-256 CR is introduced by
the injectivity argument. The masked_state18 repack IS a binding opening: given
the leaf-fixed `account_witness_commitment`, **at most one** witness 18-tuple
repacks to it, and in fact exactly one, unconditionally.

---

## 5. THE RESIDUAL GAP (early-obstruction signal — read this)

The injectivity argument closes the *spend-only rebinding* survivor cleanly, but
it does NOT by itself discharge `creation_time_hidden_source_soundness`, and the
coverage is NARROWER than "all witness coordinates" in two concrete senses. Per
RISK-FIRST policy these are reported loudly:

### Gap A — state18 binds 18 residues, but the hidden source has 64 columns

`masked_state18` binds exactly **18** residues: 12 checked-core syndrome rows
(`REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS`) + 6 account-projection lanes
(`HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES`). But the underlying
hidden-source object is a **64-column** source vector
(`active_subspace_exact_replay` md: `old source projection columns: 64`,
`excluded inactive source columns: 52`, `active projection lanes: 12`). The
12 carrier projection lanes (`HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES`) split as
6 account-derived + 6 pad-PRF-derived
(`hidden_source_fallback_projection_values_from_hidden_material`,
`production_reference.rs:7166`). So:

- The 52 INACTIVE source coordinates are **not** committed by `account_witness_commitment`
  and are **not** bound by the injectivity lemma. They remain "hidden witness
  material only" (replay md row `inactive_projection_coordinates:
  excluded_by_relation_redesign`).
- Whether those 52 unbound coordinates matter is exactly what the replay artifact
  resolves: it claims the accepting relation has **nullity 0 over the 12 active
  lanes** and routes inactive-coordinate disagreement to
  "semantic bridge/export checks" (`verifier_exporter_bridge_semantics: closed`),
  so a forged inactive coordinate is `invalid_for_active_subspace_relation`
  rather than an accepting kernel vector.

The lemma in §3 therefore proves UNIQUENESS of the bound (active) witness, but it
relies on the SEPARATE active-subspace replay claim (nullity 0 + bridge
semantics) to argue that binding 18 residues is *sufficient* to pin the
relation-relevant witness. That sufficiency is **not** part of the
commitment-binding reduction; it is the substance of
`creation_time_hidden_source_soundness_break`. Binding-injectivity ≠ relation
soundness.

### Gap B — the soundness certificate consumes hidden harness inputs (payload not yet public/replayable)

The creation theorem is still `[~]` (in progress) in the tracker, not `[x]`.
`verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate`
takes `pad_seed`, `opening`, `statement`, and a full signature-with-carrier as
**direct test-harness inputs**, and the public
`DirectReplayableRelationCertificateV1` currently carries only a payload
digest/handle. Per the tracker (lines 171–226) and
`path2_direct_relation_certificate_payload_blocker.md`, the deployable proof
still needs a concrete public hiding payload (the transparent
`zk_masked_trace_iop_native_boundary` route), and that verifier returns
`proof-verifier-not-implemented` fail-closed today. So the §3 lemma proves
witness UNIQUENESS conditioned on an accepting masked-projection certificate, but
the certificate's own soundness (extractor ⟹ valid relation, no-export, QROM) is
not yet discharged executably. This is `creation_time_hidden_source_soundness_break`
remaining OPEN, not closed.

### Net assessment

- The TWO-WITNESSES SURVIVOR exploited by the spend-only test (2205) **is
  closed** at creation time. It reduces cleanly to SHA-256 CR on the
  opening-commitment frame plus the unconditional bijectivity of the state18
  packing. No survivor of *that specific shape* exists at creation. (confidence:
  high — directly evidenced by passing KAT `masked_witness_state18_unmasks_...`
  at lib.rs:2675 rejecting the cross-commitment carrier.)
- This does NOT mean the creation theorem is discharged. The injectivity binds
  18 of 64 hidden-source coordinates; sufficiency of that 18-residue binding rests
  on the active-subspace replay's nullity-0 + bridge-semantics claim (Gap A), and
  the active-lane relation soundness / public replayable payload is still
  unimplemented and fail-closed (Gap B). Those are the live obstructions, and
  they are the `creation_time_hidden_source_soundness_break` and
  `prevalidation_data_availability_break` rows, not the leaf/commitment-binding
  row.

---

## 6. Map onto break events (decision.md:159–164)

- `creation_time_hidden_source_soundness_break` — the LIVE residual. Binding
  injectivity does not produce it; it is produced only if an accepting
  active-lane masked-projection certificate exists for a witness that does not
  satisfy the hidden-source relation. Gap A (18-of-64 coverage, sufficiency via
  active-subspace replay) and Gap B (payload soundness unimplemented) both live
  here.
- `prevalidation_leaf_binding_break` — the binding the §3 lemma DEPENDS ON and
  reduces to: a SHA-256 collision on the opening-commitment frame (or pad-seed
  opening digest) would be required to make `account_witness_commitment`
  non-unique for a fixed leaf. Absent such a collision, the witness is unique.
- `qrom_or_domain_separation_break` — needed only for the projection-mask PRF
  (`REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_MASK_DOMAIN`) and the
  certificate's Fiat-Shamir; orthogonal to the injectivity step.
- `prevalidation_data_availability_break` — Gap B's fail-closed boundary: absent
  a replayable public payload, creation rejects (`Path 2 evidence payload
  unavailable`, `production_reference.rs:26290`).
- `live_slot_leakage_or_keygen_break` — orthogonal lifecycle/leakage charge.

---

## 7. One-line answer to the question

Does masked_state18 make the account-witness map injective and close the
two-witnesses survivor? **YES for the survivor, conditionally for the theorem.**
The map `leaf.opening_commitment ⟶ account_witness_commitment ⟶ 18-residue
witness` is injective under SHA-256 CR (first hop) and *unconditionally*
bijective (second hop, bit-packing), so the spend-only two-commitments rebinding
cannot occur once `account_witness_commitment` is pinned by a single leaf field.
The repack IS a binding opening (≤ one witness per commitment; in fact exactly
one). The residual obstruction is NOT injectivity but COVERAGE+SOUNDNESS: only 18
of 64 hidden-source coordinates are bound (the other 52 inactive coordinates rely
on the separate active-subspace nullity-0 + bridge-semantics claim), and the
active-lane relation-soundness payload is still unimplemented/fail-closed. Those
are `creation_time_hidden_source_soundness_break` and
`prevalidation_data_availability_break`, and they remain the live early-risk
items.
