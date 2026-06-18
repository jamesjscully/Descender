# Path 2 Proof-Friendly Inner Commitment Sketch

Status: design sketch for the recommended hiding proof payload route.

## Motivation

`research/path2_direct_creation_hiding_payload_cost_model.md` shows that a
direct proof over the current SHA-256-shaped creation commitments is
SHA-circuit dominated:

- lower-bound relation: about 23 SHA-256 compressions inside the hiding proof;
- full public pad statement: about 55 SHA-256 compressions;
- q12289 arithmetic relation: small by comparison.

Therefore the next viable deployable path is to keep SHA-256 for outer
consensus binding, but add proof-friendly inner commitments for the objects the
creation proof must open.

## Core Principle

The hiding proof should prove openings of algebraic/proof-friendly commitments,
not SHA-256 preimages.

SHA-256 remains useful for:

- canonical leaf bytes;
- public certificate/payload digest binding;
- consensus roots and data availability;
- challenge/domain separation.

But SHA-256 should not be the primary in-circuit commitment for:

- pad source;
- state-assisted opening tuple;
- projection masks;
- packed witness opening relation.

## Proposed Public Inputs

Add a first-class Path 2 proof-binding object:

```text
Path2CreationProofBindingV1 {
  schema,
  proof_binding_version,
  account_id,
  public_key_identity,
  slot_id,
  verifier_version_digest,
  account_state_root_before,
  lifecycle_anchor,
  packed_witness_commitment,
  randomizer_commitment,
  pad_source_commitment,
  opening_tuple_commitment,
  projection_relation_commitment,
  creation_statement_digest,
  aggregation_context_digest
}
```

Then bind this object to the existing leaf/certificate by digest:

```text
creation_proof_binding_digest = SHA256(Path2CreationProofBindingV1)
```

The digest can be carried by the direct creation proof payload or by a future
leaf version. If it is only carried by the payload, the certificate must bind it
and the creation verifier must reject unless the payload and certificate agree.
If it is carried by the leaf, the account state cost increases by 32 bytes but
the binding is cleaner.

## Recommended Leaf Change

For a deployable permissionless verifier, prefer adding one 32-byte field to the
leaf:

```text
creation_proof_binding_digest: [u8; 32]
```

This avoids treating payload-only binding as weaker than leaf binding. The spend
signature remains unchanged because spend consumes only `slot_authorization_digest`.

Estimated cost:

- account creation/state leaf preimage: +32 bytes;
- spend signature: +0 bytes;
- public admission payload: +32 bytes if the full binding object is available
  by digest/root, more if stored directly for validator convenience.

## Proof-Friendly Commitments Needed

The proof payload should use a commitment family with:

- binding under a named algebraic assumption;
- hiding for pad source and projection residues;
- efficient q12289 linear/arithmetic opening checks;
- domain-separated commitments for each role.

Minimum roles:

| role | hidden opening | public commitment | purpose |
|---|---|---|---|
| `pad_source_commitment` | pad seed or pad-source witness | algebraic commitment | derive/verify pad lanes and projection masks without SHA preimage proof |
| `opening_tuple_commitment` | state-assisted opening tuple witness | algebraic commitment | bind lifecycle, pad source, witness commitment, and carrier-handle material |
| `packed_witness_commitment` | 18 q12289 residues | existing 32-byte packed value or inner algebraic commitment | prove 12 checked-core residues plus 6 masked projection residues |
| `projection_relation_commitment` | unmasked projection residues and blinding | algebraic commitment | prove carrier/check relation without revealing projection residues |

The existing 32-byte packed witness field is already relatively proof-friendly
because it is canonical residue packing, not a SHA digest. The largest redesign
pressure is the pad source/opening tuple side.

## Hiding Payload Relation

`Path2DirectCreationProofPayloadV1` should eventually prove:

1. public `Path2CreationProofBindingV1` matches the leaf/certificate context;
2. prover knows openings to `pad_source_commitment` and
   `opening_tuple_commitment`;
3. `opening_tuple_commitment` binds the same slot id, lifecycle anchor,
   witness commitment, randomizer commitment, and pad source as the public
   binding object;
4. prover knows 18 q12289 residues that pack to `packed_witness_commitment`;
5. residues 0..11 are the checked-core syndrome residues;
6. residues 12..17 are masked projection residues;
7. pad-source opening derives projection masks and pad-source lanes under
   proof-friendly domains;
8. unmasking residues 12..17 yields hidden account projection residues;
9. projection/account/pad lanes satisfy the active-lane carrier relation;
10. proof reveals no raw pad seed, projection masks, unmasked account projection
    residues, or private opening frames.

## Remaining Binding Question

The existing `pad_seed_digest` field is SHA-shaped. If the deployable proof no
longer proves a SHA-256 preimage for `pad_seed_digest`, then this field cannot
remain the primary cryptographic pad-source binding.

There are two choices:

1. **Keep `pad_seed_digest` as an auxiliary duplicate-detection handle.**
   The proof theorem must say pad-source security comes from
   `pad_source_commitment`, not from the digest. Duplicate prevention should
   move to `pad_source_commitment` or to `creation_proof_binding_digest`.

2. **Still prove `pad_seed_digest` inside the hiding proof.**
   This preserves the current field semantics but keeps at least one SHA-256
   preimage in the proof.

Recommendation: move primary pad-source binding and duplicate prevention to the
proof-friendly commitment/binding digest, and keep `pad_seed_digest` only as a
legacy/debug compatibility handle if needed.

## Immediate Next Implementation Step

Add a fail-closed typed boundary for:

```text
Path2CreationProofBindingV1
```

and a KAT proving:

- its digest changes under account/key/slot/state/lifecycle/pad/witness
  rebinding;
- the current `Path2DirectCreationProofPayloadV1` rejects if its binding digest
  does not match the certificate/leaf path;
- no spend signature bytes change.

This is the smallest code change that tests the recommended redesign without
committing to a final proof system.

## Executable Boundary Progress

`tools/parameter-search/src/production_reference.rs` now includes:

```text
Path2CreationProofBindingV1
path2_creation_proof_binding_v1
path2_creation_proof_binding_v1_digest
```

and `Path2DirectCreationProofPayloadV1` carries
`proof_binding_digest`.

KAT `path2_direct_creation_proof_payload_is_fail_closed_until_instantiated`
now also proves:

- a payload whose `proof_binding_digest` differs from the binding rejects;
- a binding rebound to a different slot rejects against the leaf context;
- the proof remains fail-closed even when the binding is well-formed.

This does not instantiate the hiding proof. It confirms the schema direction:
the future proof payload will be bound to a typed proof-friendly commitment
object rather than to an unstructured digest-only handle.

## Leaf-Bound V2 Trial

`tools/parameter-search/src/production_reference.rs` now also includes a
fail-closed V2 leaf frame:

```text
Path2PrevalidatedSlotLeafV2
path2_prevalidated_slot_leaf_v2_from_v1
path2_prevalidated_slot_leaf_v2_digest
path2_slot_authorization_v2_digest
```

V2 adds exactly one field:

```text
creation_proof_binding_digest: [u8; 32]
```

KAT `path2_prevalidated_slot_leaf_v2_binds_creation_proof_binding_digest`
proves:

- V2 leaf bytes are 385 bytes, exactly 32 bytes larger than V1;
- V2 has a distinct leaf domain and authorization domain;
- changing `creation_proof_binding_digest` changes both the V2 leaf digest and
  the V2 authorization digest;
- checked-core spend signature bytes remain 252.

This is the first executable trial of the new path. It does not replace V1 yet;
it proves the recommended schema expansion is byte-bounded and keeps the online
spend path unchanged.

## Toy q-Linear Payload Trial

`tools/parameter-search/src/production_reference.rs` now includes a deliberately
non-deployable q12289-linear proof trial:

```text
PATH2_DIRECT_CREATION_PROOF_KIND_TOY_Q_LINEAR
Path2ToyQLinearCreationProofV1
verify_path2_direct_creation_toy_q_linear_proof_v1
```

The trial proves the reduced algebraic relation:

```text
masked_projection_i = pad_lane_i + account_projection_i mod q
```

for the six active hidden-source projection lanes. It binds the proof through
`Path2CreationProofBindingV1`, reads the public masked projection lanes from the
canonical 18-residue packed witness field, and checks Schnorr-style response
equations against role-separated q-linear commitments.

KAT `path2_toy_q_linear_creation_proof_accepts_and_rejects_relation_mutation`
proves:

- an honestly generated q-linear proof accepts;
- mutating a relation response rejects at the masked-projection relation check;
- malformed q-commitment encoding rejects.

This validates the new proof shape: the core hidden-source payload relation can
be expressed as small q12289 arithmetic once SHA-shaped openings are moved out
of the proof.

## Transparent Bridge Contract Pin

`tools/parameter-search/src/production_reference.rs` now also carries the same
decision into the transparent opening route:

```text
Path2TransparentProofFriendlyInnerCommitmentPlanV1
path2_transparent_proof_friendly_inner_commitment_plan_v1
```

The plan is hashed into `Path2TransparentHiddenRelationConstraintPlanV1`. That
means the transparent full-hidden-relation contract cannot claim C1/C2/C4/C6
unless the proof-friendly inner commitment layer is also instantiated. The
canonical plan requires:

- SHA-256 outer consensus handles remain retained;
- packed witness remains canonical residue packing;
- pad-source, opening-tuple, and projection-relation roles use inner
  proof-friendly commitments;
- openings are role-separated algebraic openings;
- raw pad seed, projection masks, and unmasked account projection are not
  exported;
- current SHA `pad_seed_digest` is not treated as the primary in-proof binding.

KAT `path2_transparent_proof_family_capability_pins_active_assumptions` now
rejects omission of the inner role-opening requirement, treating SHA
`pad_seed_digest` as the primary binding, and premature claims that the inner
commitment layer is instantiated.
of the inner proof. It is not a cryptographic instantiation. The toy commitment
group is `Z_q` with `q = 12289`, so it is neither binding nor hiding at the
security level required for deployment.

## Current Bottleneck

The bottleneck has moved from "can the payload relation be expressed compactly?"
to "which real proof-friendly commitment/proof system should replace the toy
q-linear commitments?"

The next deployable candidate must provide:

- computational binding for pad/projection/opening commitments;
- hiding for pad lanes, projection residues, and private opening frames;
- efficient linear-relation proofs over q12289-compatible values;
- Fiat-Shamir/QROM treatment for the payload transcript;
- no raw pad seed, projection masks, unmasked projection residues, or private
  opening frames in public bytes.

Candidate families still requiring evaluation:

- Pedersen/vector commitments over a real prime-order group;
- IPA-style vector commitments if verifier cost and proof bytes are acceptable;
- SNARK/STARK proof over the algebraic relation if account-creation proof bytes
  are acceptable;
- lattice/SIS commitments only if the account-creation payload remains small
  enough for the blockchain target.
