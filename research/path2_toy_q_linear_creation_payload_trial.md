# Path 2 Toy q-Linear Creation Payload Trial

Status: executable proof-friendly payload trial, not deployable.

## What This Instantiates

`tools/parameter-search/src/production_reference.rs` now includes a toy
q12289-linear proof kind:

```text
PATH2_DIRECT_CREATION_PROOF_KIND_TOY_Q_LINEAR
Path2ToyQLinearCreationProofV1
verify_path2_direct_creation_toy_q_linear_proof_v1
```

The trial proves one reduced proof-friendly relation:

```text
masked_projection_i = pad_lane_i + account_projection_i mod q
```

for six active projection lanes.

It uses proof-friendly q commitments encoded into the
`Path2CreationProofBindingV1` fields:

```text
pad_source_commitment
projection_relation_commitment
packed_witness_commitment
```

The packed witness commitment supplies public masked projection residues through
the existing 18-residue q12289 packing. The pad and projection commitments are
toy Pedersen-style linear commitments over q12289.

## Verifier Shape

The verifier checks:

1. leaf/evidence/certificate bindings;
2. `Path2CreationProofBindingV1` digest and leaf context;
3. proof payload kind and binding digest;
4. canonical toy q-commitment encoding;
5. packed witness commitment opens to 18 canonical q residues;
6. Fiat-Shamir challenge binds leaf, binding, payload, commitments, nonces, and
   masked projection residues;
7. Schnorr-style commitment equations for pad lanes and projection lanes;
8. linear relation responses satisfy:

```text
z_pad_i + z_projection_i = challenge * masked_projection_i mod q
```

This exercises the intended proof-friendly structure without proving SHA-256
preimages for pad seeds or opening commitments.

## Executable Evidence

KAT `path2_toy_q_linear_creation_proof_accepts_and_rejects_relation_mutation`
proves:

- an honestly generated q-linear proof accepts;
- mutating a relation response rejects with
  `Path 2 toy q-linear masked projection relation mismatch`;
- malformed q-commitment padding rejects.

## Why This Is Not Deployable

The toy commitment group is the additive field `Z_q` with small `q = 12289`.
It is not a binding/hiding commitment suitable for production. The proof is
therefore a structural trial, not a cryptographic instantiation.

It does, however, prove the design direction:

- the relation can be expressed as small q12289 arithmetic;
- the online spend signature remains unchanged;
- the proof payload can be bound through `Path2CreationProofBindingV1`;
- the verifier can reject malformed algebraic payloads before any hidden-source
  theorem is promoted.

## Next Real Instantiation Requirement

Replace the toy q commitments with a real proof-friendly commitment/proof
system that provides:

- computational binding;
- hiding for pad and projection witnesses;
- efficient linear-relation proofs over q12289-compatible scalar arithmetic;
- a transcript challenge with reviewed QROM treatment;
- no raw pad seed, projection masks, or unmasked projection residues in public
  payload bytes.

Candidate families to evaluate next:

- Pedersen/vector commitments over a real prime-order group;
- IPA-style vector commitments if verifier cost is acceptable;
- SNARK/STARK proof over algebraic commitments if proof bytes are acceptable;
- lattice/SIS commitments only if they can be made small enough for account
  creation payloads.
