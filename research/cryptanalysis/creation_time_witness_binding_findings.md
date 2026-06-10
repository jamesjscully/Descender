# Creation-Time Account-Witness Binding: Survivor Search Findings

Status: EMPIRICAL BREAK role, RISK-FIRST attack on the load-bearing Path-2
prevalidation bet. Date 2026-06-09.

Question attacked: does the creation-time certificate (specifically the
`masked_state18` packed-witness opening that REPACKS to a leaf-fixed
`account_witness_commitment`) make the account-witness map INJECTIVE, closing
the spend-only two-witnesses survivor
(`projection_carrier_tuple_does_not_prove_full_witness_opening`,
tools/production-reference/src/lib.rs:2205)?

## Verdict

NO creation-time algebraic survivor exists. The spend-only two-witnesses
survivor is KILLED at creation time. The binding holds, and reduces to SHA-256
collision resistance plus the 14-bit canonical packing — i.e. only a NAMED break
(`prevalidation_leaf_binding_break` / `qrom_or_domain_separation_break`) remains,
not an open algebraic gap.

Confidence: HIGH. The conclusion follows from a closed algebraic chain that I
mirrored in Python and cross-checked against the passing Rust ground-truth tests.

## Why The Spend-Only Survivor Existed

The spend-only negative control (lib.rs:2205) builds two accepting tuples for
two DIFFERENT packed witness commitments `witness_a` (syndrome all-zero) and
`witness_b` (syndrome[0]=1), keeping the SAME six-value projection-opening frame
and the SAME selected carrier lanes. Both accept because
`account_witness_commitment` is a FREE input to
`verify_reviewed_fq12_state_assisted_projection_carrier_tuple`: nothing in the
spend tuple pins it to a single fixed value. The carrier check/transcript
binding is simply recomputed for whichever commitment you plug in. So the spend
carrier proves a projection OPENING, not a fixed account WITNESS.

## Why Creation Closes It — The Binding Chain

The creation certificate is replayed against a LEAF-FIXED `opening_commitment`.
The chain (all in tools/parameter-search/src/production_reference.rs):

1. `PrevalidatedSlotLeafV1` fixes `opening_commitment` at leaf offset 192
   (research/prevalidated_slot_leaf_v1_frame.md:46).
   `verify_path2_prevalidated_slot_creation_payload_bindings`
   (production_reference.rs:26311) enforces
   `certificate.opening_commitment == leaf.opening_commitment`.

2. `opening_commitment = SHA256(domain || profile || slot_id ||
   account_witness_commitment || randomizer || restore_anchor ||
   witness_opening_digest || pad_seed_digest || carrier_handle_secret)`
   (`reviewed_fq12_state_assisted_opening_commitment_frame`,
   production_reference.rs:44258, the `account_witness_commitment` is hashed at
   offset after slot_id). The soundness certificate recomputes this commitment
   from the supplied opening
   (`verify_reviewed_fq12_state_assisted_slot_opening`, called at
   production_reference.rs:38543) and the leaf comparison forces it to equal the
   fixed leaf value. Therefore the leaf pins ONE `account_witness_commitment`
   (modulo SHA-256 collision on the opening frame).

3. `account_witness_commitment` is a BIJECTIVE 14-bit packing of 18 residues:
   12 checked-core syndrome residues + 6 masked projection residues, into a
   32-byte digest with 4 slack bits forced to zero
   (`pack_reviewed_fq12_packed_witness_state18` at production_reference.rs:36850;
   `unpack_reviewed_fq12_packed_witness_state18` at production_reference.rs:46019;
   FIELD_BITS=14, 18*14=252 bits, SLACK_BITS=4 must be zero). A fixed commitment
   fixes all 18 residues.

4. `reviewed_fq12_masked_state18_packed_witness_opening`
   (production_reference.rs:6528) reads the 18 residues out of the fixed
   commitment, computes `account_projection[i] = (masked[i] - mask[i]) mod Q`
   where `mask` is deterministic in `(pad_seed, slot_id)`
   (`reviewed_fq12_packed_witness_state18_projection_masks`,
   production_reference.rs:6376), then REPACKS and asserts repack == commitment
   (production_reference.rs:6556).

## Key Algebraic Facts (each verified empirically)

- The repack-to-commitment check is an ALGEBRAIC TAUTOLOGY. Repack recomputes
  `masked'[i] = (account_projection[i] + mask[i]) = ((masked[i]-mask[i])+mask[i])
  = masked[i]`, so it always equals the original commitment for any canonically
  unpackable commitment. Probe: 2000/2000 random commitments repacked exactly.
  CONSEQUENCE: the repack adds NO independent constraint. The binding power is
  entirely that the LEAF fixes the commitment.

- `witness_commitment <-> (12 syndrome residues, 6 account_projection residues)`
  is a BIJECTION for fixed `(pad_seed, slot_id)`: bit-packing is a bijection on
  canonical 18-tuples, and the per-lane mask map
  `account_proj |-> (account_proj + mask) mod Q` is a bijection on Z_Q. Probe:
  20000/20000 distinct parts gave distinct commitments AND round-tripped exactly,
  zero collisions. CONSEQUENCE: a leaf-fixed commitment fixes the 6-lane account
  projection opening UNIQUELY.

- Replaying the exact spend-only survivor under the creation constraint:
  `witness_a` and `witness_b` (the lib.rs:2205 pair) map to TWO DISTINCT
  `opening_commitments`. The leaf fixes exactly ONE. So at most one of
  {witness_a, witness_b} can satisfy any given leaf. The two-witnesses survivor
  is killed.

## Rust Ground-Truth Cross-Check

The creation-time closure is already an executable, passing test:
`packed_witness_state18_projection_carrier_closes_projection_rebinding`
(lib.rs:2302) and `masked_witness_state18_unmasks_projection_with_pad_seed`
(lib.rs:2459) both PASS. In the first, building tuples for two distinct
commitments still accepts INDIVIDUALLY (each with its own opening_commitment and
frame), but cross-binding carrier_a with opening_b is REJECTED with
`"reviewed Fq12 packed witness state18 projection carrier tuple exported a
hidden-material break"` (lib.rs:2446-2455). This matches the algebra: each
distinct witness corresponds to a distinct opening_commitment / leaf, so they
are not the same leaf's two witnesses — they are two different leaves.

## Residual Question (the only remaining attack surface)

There is NO algebraic creation-time survivor. The only ways to produce two
distinct account witnesses for the SAME leaf are:
- a SHA-256 collision on the `opening_commitment` frame (two commitments hashing
  to the same `opening_commitment`), or
- a SHA-256 collision on the `witness_opening_digest` frame.

Both are exactly the NAMED breaks in the target theorem
(`direct_replayable_relation_certificate_soundness_or_break`,
account_creation_proof_mechanism_decision.md:147-167):
`prevalidation_leaf_binding_break` / `qrom_or_domain_separation_break`. They are
collision-resistance assumptions, not an open creation-time gap.

## Caveat / Honesty Notes

- I mirrored the projection-mask PRF with a concrete deterministic SHA-256 model
  rather than the exact `uniform_mod_from_hash` bytes. The bijection and
  tautology conclusions are MASK-VALUE-INDEPENDENT (masks cancel in the repack
  and are a fixed per-lane permutation of Z_Q), so the exact PRF bytes do not
  affect the verdict. The Rust ground-truth tests use the real PRF and agree.
- This finding addresses the WITNESS-BINDING injectivity question only. It does
  NOT (re)validate the underlying hidden-source RELATION soundness (that the
  fixed account_projection actually satisfies the active-lane lattice relation),
  which is the separate `creation_time_hidden_source_soundness_break` arm and
  the checked-core kernel question handled by the response-link exact
  certificate. The masked_state18 binding only guarantees the opening is UNIQUE
  per leaf; it does not by itself prove that unique opening is RELATION-VALID.

## Artifacts

- Probe: scripts/research/creation_time_witness_binding_survivor_search.py
- Output:
  research/cryptanalysis/creation_time_witness_binding_survivor_search_n32_q12289.json
