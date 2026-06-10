# Creation-Time Account-Witness Binding — Structural / Rank Analysis

Role: STRUCTURAL/RANK BREAK probe of the Path-2 creation certificate.

Question under attack: does the creation-time certificate (specifically the
`masked_state18` repack-to-fixed-commitment opening) make the account-witness
map INJECTIVE — closing the two-witnesses survivor that the spend-only
`projection_carrier_tuple_does_not_prove_full_witness_opening`
(`tools/production-reference/src/lib.rs:2205`) leaves open — or does a
creation-time survivor still exist?

VERDICT: **no-survivor — binding holds**, but NOT where the design narrative
places it. The injectivity is NOT supplied by the `masked_state18` opening in
isolation; that sub-verifier has a 12-dimensional residual nullity (the
checked-core syndrome lanes pass through unconstrained). Injectivity is supplied
by the `opening_commitment` leaf binding, which hashes the full 18-residue
witness commitment, and by the checked-core / statement recomputation that
sources the witness commitment from the account slot rather than from
adversary-chosen verify-time input. The masked_state18 opening alone does not
close the spend-only survivor; the full certificate does. This is a load-bearing
correction to the claim in `account_creation_proof_mechanism_decision.md:119`.

All numbers below are computed, not assumed. Probe:
`tools/parameter-search/tests/creation_time_binding_rank_probe.rs`
(run: `cargo test --test creation_time_binding_rank_probe -- --nocapture`,
5/5 pass).

## Dimensions (computed)

- `q = 12289`, `FIELD_BITS = 14`, `DIGEST_BYTES = 32`.
- checked-core syndrome rows `ROWS = 12`.
- account-projection lanes `LANES = 6`.
- `state18` total residues = `18` (`= ROWS + LANES`).
- witness-commitment packed bits = `18 * 14 = 252`; digest bits = `256`;
  slack bits = `4` (required zero by `unpack_reviewed_fq12_packed_witness_state18`
  via `any_bits_set_le`).

The account witness commitment is therefore NOT a hiding/binding hash. It is a
**reversible bit-packing** of 18 canonical `F_q` residues into a 32-byte digest
(`pack_reviewed_fq12_packed_witness_state18` /
`unpack_reviewed_fq12_packed_witness_state18`,
`production_reference.rs:36850 / 46019`). `unpack(pack(x)) == x` and `pack` is
injective on canonical residue vectors. The account-witness "space" is exactly
`F_q^18` restricted to canonical residues; commitment equality is residue-vector
equality. (Probe 1.)

## The masked_state18 frame algebra

`reviewed_fq12_masked_state18_packed_witness_opening`
(`production_reference.rs:6528`) does, for a given `witness_commitment`,
`pad_seed`, `slot_id`:

1. `residues = unpack18(witness_commitment)` — 12 syndrome + 6 masked
   projection.
2. `masks = PRF(pad_seed, slot_id)` (6 lanes,
   `reviewed_fq12_packed_witness_state18_projection_masks:6376`).
3. `account_projection[i] = (masked_projection[i] - masks[i]) mod q`.
4. repack `(syndrome, account_projection -> remask)` and require
   `repacked == witness_commitment`.

The masking is, per lane, an **additive bijection** `x -> x + mask_i (mod q)` for
fixed `(pad_seed, slot_id)`; recovery is exact and unique (Probe 2). The repack
check is therefore a tautology on the 6 projection lanes given canonical input —
it confirms self-consistency, it does not add an independent constraint linking
the witness to anything external.

Crucially, `masked_state18` reads the 12 syndrome residues
(`witness_syndrome = residues[..12]`) and carries them out, but imposes **no
equation on them**. The only externally-checked constraint inside the opening
sub-verifier (`verify_..._masked_witness_state18_packed_witness_opening:6704`) is:

- `statement.witness_commitment == opening.account_witness_commitment` (6720),
  and
- `digest(frame(account_projection)) == opening.witness_opening_digest` (6739) —
  which constrains only the **6 projection lanes**.

## Residual nullity of masked_state18 in isolation: 12

Probe 5 (`probe5_masked_state18_verifier_rejects_witness_swap`) replicates the
exact spend-only survivor move at the masked_state18 level using the same data
as the 2205 negative control (`selected_source_values = [3,5,8,13,21,34]`,
`syndrome_a = 0`, `syndrome_b[0] = 1`):

- (a) honest opening for `commit_a` ACCEPTS.
- (b) swapping ONLY `statement.witness_commitment` to `commit_b` while leaving
  the opening bound to `commit_a` is REJECTED — by the `6720` equality check.
- (c) swapping BOTH `opening.account_witness_commitment` AND
  `statement.witness_commitment` to `commit_b` is **ACCEPTED** by
  `verify_..._masked_witness_state18_packed_witness_opening`, because `commit_b`
  differs from `commit_a` only in the 12 syndrome residues, recovers the SAME 6
  projection lanes `[3,5,8,13,21,34]`, hence the SAME `witness_opening_digest`.

So the masked_state18 packed-witness-opening sub-verifier has full column rank
only over the 6 projection lanes and **nullity 12 over the syndrome lanes**. By
itself it does NOT make the account-witness map injective: it leaves precisely
the spend-only survivor (`commit_a` vs `commit_b`) open. The design note at
`account_creation_proof_mechanism_decision.md:119` ("masked-state18 packed
witness opening repacks to the account witness commitment") is true but does NOT
by itself close the survivor — the 12 syndrome coordinates are free in this map.

## Where injectivity actually comes from (the binding that closes it)

The two-witness survivor is closed one level up, by constraints OUTSIDE the
masked_state18 opening:

1. **opening_commitment hashes the full 18-residue commitment.**
   `reviewed_fq12_state_assisted_opening_commitment_frame`
   (`production_reference.rs:44258`) embeds `account_witness_commitment`
   verbatim. `opening_commitment = SHA256(frame)`. So `commit_a` and `commit_b`
   yield DIFFERENT `opening_commitment` digests (Probe 3), even though they share
   all 6 projection lanes and `witness_opening_digest`. `opening_commitment` is a
   leaf-FIXED field (`leaf.opening_commitment`, checked in
   `verify_path2_prevalidated_slot_creation_payload_bindings:26311` and
   `verify_reviewed_fq12_state_assisted_slot_opening:36381`). Two distinct
   witnesses therefore force two distinct leaves. Binding here is modulo SHA-256
   collision resistance on the opening frame.

2. **witness_commitment is sourced from the account slot, not verify-time
   input.** `verify_..._masked_projection_candidate_verifier:38453` recomputes
   `expected_statement = reviewed_fq12_state_assisted_hidden_source_statement(...)`
   (`37335`), whose `witness_commitment` is taken from
   `slot.commitment().witness_commitment()`, and requires
   `expected_statement == statement` (38495). And
   `verify_reviewed_fq12_state_assisted_slot_opening:36369` requires
   `opening.account_witness_commitment == slot.commitment().witness_commitment()`.
   So the adversary cannot present `commit_b` without an account slot whose
   committed witness is `commit_b`.

3. **the 12 syndrome lanes are checked-core bound.** The same candidate verifier
   runs `verify_reviewed_fq12_state_assisted_checked_core` and exports
   `checked_core_witness_syndrome` (= the 12 syndrome residues) and
   `checked_core_response_syndrome` into the soundness certificate
   (`38574-38575`). The syndrome lanes that masked_state18 leaves free are
   pinned by the checked-core challenge/response relation, i.e. by the spend-time
   message binding, not by the creation opening.

Net: the residual 12-nullity of masked_state18 is removed by (1) the
opening_commitment SHA-256 over the full commitment and (2) the slot/checked-core
sourcing of the witness. The composite account-witness map (active lanes ⊕
masked_state18 repack ⊕ opening tuple ⊕ opening_commitment leaf-fix ⊕
checked-core) has full column rank over the 18-residue witness space, hence
nullity 0. No two distinct canonical witnesses can share the same leaf.

## Comparison to the spend-only survivor

The spend-only carrier tuple
(`verify_reviewed_fq12_state_assisted_projection_carrier_tuple`) computed
`check_bytes`/`transcript_binding` by re-deriving them from whatever
`witness_commitment` was supplied; it never hashed the witness commitment into a
leaf-FIXED field and never re-sourced it from a slot. That is exactly why the
2205 test rebinds the same 6-value projection frame to two different packed Fq12
witness commitments and BOTH accept: nullity > 0 over the witness. The
creation-time certificate adds the leaf-fixed `opening_commitment` (full-18-bit
hash) and slot sourcing, which is the rank amplification the spend path lacked.

This is consistent with the design fork's "one added carrier lane buys one
linear constraint, no rank amplification": extra carrier lanes are linear lane
equalities and do not bind the witness commitment; the leaf-fixed
opening_commitment hash is a categorically different, full-rank binding.

## Caveats / what the binding rests on

- Binding is computational, modulo (a) SHA-256 collision resistance on the
  opening-commitment frame and on the leaf, and (b) the checked-core / QROM
  soundness for the 12 syndrome lanes. These are exactly the named breaks
  `prevalidation_leaf_binding`, `qrom_or_domain_sep`, and
  `creation_time_hidden_source_soundness_break` in the target theorem — they are
  NOT structural-rank survivors. The structural map itself is injective.
- The masked_state18 opening alone is NOT the closing argument and should not be
  cited as such. If a future refactor were to drop `account_witness_commitment`
  from the opening_commitment frame, or accept a verify-time witness commitment
  not re-sourced from the slot, the 12-nullity survivor (Probe 5(c)) would
  immediately reopen. Recommend pinning a regression test that asserts
  `opening_commitment` changes under a syndrome-only witness mutation.

## Artifacts

- Probe (5 tests, all pass):
  `tools/parameter-search/tests/creation_time_binding_rank_probe.rs`
- Spend-only survivor (still valid as negative control):
  `tools/production-reference/src/lib.rs:2205`
- Closing-binding test (projection-level):
  `tools/production-reference/src/lib.rs` `packed_witness_state18_projection_carrier_closes_projection_rebinding`
