# Path 2 Direct Relation Certificate Payload Blocker

Status: remaining internal proof blocker for direct-mode Path 2 creation
soundness.

## Problem

The Path 2 binding, admission, spend, leakage, lifecycle, and hash/QROM layers
now have executable direct-mode closure. The remaining cryptographic blocker is
the payload semantics of `DirectReplayableRelationCertificateV1`.

Current executable code has two different objects:

1. public admission certificate fields:
   - account/key/slot/state binding digests;
   - `hidden_source_statement_digest`;
   - `reviewed_active_lane_statement_digest`;
   - `public_replay_payload_digest`;
   - relation transcript digest;
2. test-harness replay inputs:
   - reviewed Fq12 account state;
   - signature-with-carrier;
   - state-assisted opening;
   - reviewed hidden-source statement;
   - raw pad seed.

`verify_path2_prevalidated_slot_creation_relation_replay` proves that, given the
test-harness replay inputs, the Path 2 wrapper binds the active-lane relation to
the leaf and rejects key/slot/pad/statement/profile/verifier rebinding. That is
useful negative-control evidence, but it is not yet a deployment proof object
because validators cannot rely on hidden private replay inputs.

## What Is Proved Now

The following are now executable:

- canonical leaf/evidence/certificate binding;
- payload digest availability fail-closed behavior;
- public-key identity binding to the actual replayed key;
- non-circular `CreationHiddenSourceStatementV1`;
- reviewed active-lane replay as a negative-control oracle;
- public admission JSON no-export;
- relation-replay rejection no-export;
- state-namespace admission and direct spend composition.

Therefore, if a verifier is given a valid replay/proof payload for the
creation-time hidden-source relation, the surrounding Path 2 system binds that
payload to the exact live leaf and later spend.

## What Is Not Proved

The current public certificate does not itself contain enough public proof bytes
to let an untrusted validator establish:

```text
CreationHiddenSourceRelationV1(leaf, evidence, certificate)
```

without access to hidden opening material.

The field `public_replay_payload_digest` is only a binding handle. It is not
proof semantics. Treating it as proof semantics would be a hash-only proof
mistake.

## Required Next Design Choice

To close `direct_replayable_relation_certificate_soundness_or_break`, Path 2
must choose one concrete payload semantics:

### Option 1: Public Opening Payload

Publish enough opening material for validators to replay the relation directly.

This is simplest to verify but switches the theorem to
`public_creation_opening_model`. It likely breaks the current pre-spend hiding
claim unless the revealed values are strictly limited to already-public
statements and commitments.

### Option 2: Hiding Proof Payload

Publish a proof object that proves the hidden-source relation without revealing
raw pad seed, projection masks, unmasked selected projection residues, or
private opening frames.

This preserves the current Path 2 hiding model. It requires specifying the
proof system and then adding byte-level verifier/KAT coverage for:

- statement/leaf binding;
- pad-seed digest opening without pad-seed reveal;
- packed witness commitment opening without unmasked projection reveal;
- active-lane relation checks;
- no-export rejection behavior.

### Option 3: State/DA-Gated Private Admission

Allow a permissioned account-creation verifier to see hidden material and only
publish the resulting certificate digest.

This is not a normal permissionless blockchain verifier model. It would require
a trust or committee assumption and should be treated as a different deployment
theorem, not as direct public validation.

## Recommended Path

Use Option 2 as the target for deployable direct mode.

The surrounding Path 2 system has been structured so that the proof payload can
be swapped in without changing the 252-byte spend signature:

- the spend consumes only `slot_authorization_digest`;
- the leaf binds `CreationHiddenSourceStatementV1`;
- the certificate already has a payload digest field;
- public serialization excludes private payload material;
- optional aggregation remains inactive.

## Minimal Closure Requirement

The next implementable artifact should be:

```text
Path2DirectCreationProofPayloadV1
```

with a verifier:

```text
verify_path2_direct_creation_proof_payload_v1(
  leaf,
  evidence,
  certificate,
  payload
) -> Result<(), CreationProofError>
```

The verifier must reject unless the payload proves the same hidden-source
relation currently replayed by the test harness. The first implementation may
be a structurally conservative proof placeholder only if it is explicitly
fail-closed and does not let `public_replay_payload_digest` stand in for proof
semantics.

## Executable Boundary Added

`tools/parameter-search/src/production_reference.rs` now includes the
fail-closed boundary:

```text
Path2DirectCreationProofPayloadV1
verify_path2_direct_creation_proof_payload_v1
```

The verifier validates the surrounding leaf/evidence/certificate bindings and
checks that the payload schema, statement digest, transcript digest, and payload
digest match the public certificate. It then rejects every payload kind because
no public proof system has been instantiated yet.

KAT `path2_direct_creation_proof_payload_is_fail_closed_until_instantiated`
proves:

- a digest-only/uninstantiated payload is not accepted;
- a mismatched payload digest rejects before any proof-kind handling;
- an unsupported nonzero proof kind also rejects.

This deliberately keeps the production preflight blocker active while giving the
future proof payload a concrete typed boundary.

## Cost-Model Push

`research/path2_direct_creation_hiding_payload_cost_model.md` now decomposes the
hiding proof payload into relation components and ranks three implementation
tracks.

The result identifies the actual bottleneck:

```text
SHA-256 preimage/PRF proving for pad seed, opening commitment, projection
masks, and pad-source lanes dominates the hiding payload if current commitments
are used directly.
```

The current lower-bound hash-circuit route requires about 23 SHA-256
compressions inside the hiding proof before proving a full public pad statement;
the full-pad variant reaches about 55 compressions. The q12289 arithmetic
relation itself is small by comparison. Therefore the recommended next path is
not a blind generic proof attempt, but a proof-friendly inner commitment layer:
retain SHA-256 outer consensus handles, but add algebraic/proof-friendly
commitments for the creation-time objects the hiding proof must open.

`research/path2_proof_friendly_inner_commitment_sketch.md` turns that conclusion
into a concrete design sketch. The key recommendation is to add a first-class
`creation_proof_binding_digest` for a `Path2CreationProofBindingV1` object, so
the hiding proof opens proof-friendly commitments rather than SHA-256 digests.
This adds account-creation/state bytes, not spend-signature bytes.

## Proof-Friendly Payload Trial

The first executable trial of that direction now exists as
`PATH2_DIRECT_CREATION_PROOF_KIND_TOY_Q_LINEAR`.

It is intentionally not deployable, but it does answer the immediate structural
question. The verifier can bind a proof payload through
`Path2CreationProofBindingV1`, recover public masked projection lanes from the
canonical packed-witness commitment, and check a zero-knowledge-style
q12289-linear relation:

```text
masked_projection_i = pad_lane_i + account_projection_i mod q
```

KAT `path2_toy_q_linear_creation_proof_accepts_and_rejects_relation_mutation`
proves an honest trial proof accepts and that relation-response mutation plus
malformed commitment encodings reject.

The remaining blocker is therefore narrower than before. Path 2 no longer needs
to discover whether the hidden-source payload relation can be wired through a
compact algebraic verifier; the toy trial demonstrates that shape. What remains
is selecting and instantiating a real commitment/proof family with production
binding, hiding, and QROM assumptions. The toy q commitments are over the small
additive field `Z_q` and are only a wiring test.

## Tracker Impact

This blocker is the residual reason the production preflight still says:

```text
Path 2 account-creation prevalidation proof semantics are not complete
```

All remaining negative-control work should now target this payload boundary,
not the already-closed leaf/admission/spend/leakage wrappers. The next useful
negative controls should be written against a real candidate commitment family,
not against the toy q-linear commitment beyond structural regression coverage.
