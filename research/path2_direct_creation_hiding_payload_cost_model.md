# Path 2 Direct Creation Hiding Payload Cost Model

Status: bottleneck model for the remaining Path 2 proof-payload issue.

This artifact separates the current relation into public wrapper checks,
SHA-heavy hidden checks, and q12289 arithmetic checks. It is not a
deployment proof; it identifies where a hiding payload will get expensive.

## Decision

- status: `bottleneck_identified`
- deployment ready: `False`
- identified bottleneck: SHA-256 preimage/PRF proving for pad seed, opening commitment, projection masks, and pad-source lanes dominates the hiding payload if current commitments are used directly.
- recommended next path: `proof_friendly_inner_commitments`

## Scenario Estimates

| scenario | SHA-256 compressions | constraint range | q ops | classification |
|---|---:|---:|---:|---|
| `existing_sha_commitments_hash_circuit_lower_bound` | `23` | `460000..690000` | `42` | `sha_dominated` |
| `existing_sha_commitments_full_pad_statement` | `55` | `1100000..1650000` | `74` | `strongly_sha_dominated` |
| `proof_friendly_inner_commitments` | `0` | `0..0` | `106` | `algebraic_dominated_after_redesign` |

## Relation Components

| component | private witness | SHA compressions in proof | q ops | notes |
|---|---|---:|---:|---|
| `leaf_evidence_certificate_binding` | `False` | `0` | `0` | already executable outside the hiding proof; do not duplicate unless the proof system must bind its own transcript |
| `pad_seed_digest_opening` | `True` | `1` | `0` | SHA-256 preimage check for a 32-byte pad seed digest |
| `state_assisted_opening_commitment` | `True` | `4` | `0` | opening tuple frame is digest-shaped; proving it directly is SHA-heavy |
| `packed_witness_repacking` | `True` | `0` | `18` | mostly range/packing arithmetic if the 32-byte witness field is interpreted as canonical packed residues, not a hash |
| `projection_mask_prf` | `True` | `12` | `6` | six SHA-derived mask lanes dominate the unmasking relation if proven inside the circuit |
| `carrier_lane_checks` | `True` | `0` | `12` | cheap algebraically if carrier lanes are represented as field elements inside the proof |
| `pad_source_projection_lanes` | `True` | `6` | `6` | lower-bound model for proving six pad-source lanes; full public pad statement costs more |
| `full_public_pad_statement` | `True` | `32` | `32` | worst direct proof if every block pad residue is derived inside the proof |

## Options

### hash_heavy_circuit

instantiate a SNARK/STARK-style proof over the current SHA-256 commitment design

- estimated bottleneck: 16 to 55 SHA-256 compressions inside the proof, depending on pad-statement scope
- recommendation: use only as a baseline or if proof-system tooling makes SHA cheap enough

Pros:
- minimal consensus schema redesign
- keeps existing leaf and digest fields
- clear soundness story if the hash circuit is implemented correctly

Cons:
- SHA-256 preimage checks dominate prover cost
- proof payload likely needs a real general-purpose proof system
- debugging relation failures is harder because every commitment is opaque

### proof_friendly_inner_commitments

add algebraic commitments/proof statements at account creation while retaining SHA-256 outer handles for consensus binding

- estimated bottleneck: commitment redesign and binding theorem, not arithmetic relation size
- recommendation: most promising deployable path

Pros:
- moves the hiding proof into q12289-friendly arithmetic
- keeps the 252-byte spend path unchanged
- lets validators check a public proof without raw openings

Cons:
- requires account-creation/state schema expansion
- must prove inner commitments are bound to outer SHA leaf fields
- adds a new commitment assumption and proof API

### opaque_ro_pok_handle

treat a compact digest-sized proof handle as a named random-oracle proof-of-knowledge assumption

- estimated bottleneck: assumption acceptability
- recommendation: keep as fail-closed research control, not the first deployable target

Pros:
- smallest byte footprint
- matches earlier 32-byte handle research track

Cons:
- nonstandard and likely too strong without external review
- does not give ordinary validators independently checkable proof semantics
- negative controls are oracle-model controls, not concrete verifier controls

## Next Artifacts

- Path2DirectCreationProofPayloadV1 relation statement
- proof-friendly inner commitment binding sketch
- byte/state delta estimate for account creation
- negative-control KAT plan for the new payload verifier

## Do Not Do

- do not accept public_replay_payload_digest as proof semantics
- do not instantiate a public opening payload unless deliberately switching to public_creation_opening_model
- do not add spend-signature bytes for this proof; keep it account-creation/admission scoped
