# SlotAuthorizationHandleV1 Spec

Status: proof-interface specification for spend-time Path 2 authorization.

This document defines how spend-time verification validates a
`slot_authorization_digest` without depending on whether the slot record is read
directly from consensus state, proved by an inclusion path, or later validated
through an aggregated proof handle.

## Design Rule

The checked-core spend signature binds exactly one normalized value:

```text
slot_authorization_digest
```

It does not bind the concrete representation used to validate that digest. This
keeps direct single-slot deployment and future aggregation on the same signature
frame.

## Output Of Handle Validation

Every valid authorization handle normalizes to:

```text
ValidatedSlotAuthorizationV1 =
  PrevalidatedSlotLeafV1_bytes
  prevalidated_slot_leaf_digest
  slot_authorization_digest
  account_state_root
  account_id
  public_key_identity
  slot_id
  verifier_version_digest
  opening_commitment
  pad_seed_digest
  hidden_source_statement_digest
  lifecycle_anchor
  live_status_marker
```

Spend-time proof obligations consume this normalized object.

## Handle Kinds

### DirectRecord

```text
DirectRecord(slot_leaf, admitted_record_metadata)
```

Required for first deployment.

Verifier obligations:

- recompute `prevalidated_slot_leaf_digest`;
- recompute `slot_authorization_digest`;
- check the direct record is consensus-visible under the current account state;
- check the record was admitted by `verify_prevalidated_slot_creation`;
- check `live_status_marker == 0x01`;
- reject stale, missing, wrong-account, wrong-key, wrong-slot, wrong-version, or
  already-consumed records.

Consumes no aggregation assumption.

### InclusionProof

```text
InclusionProof(slot_leaf, account_root, inclusion_path)
```

Optional.

Verifier obligations:

- all `DirectRecord` digest and field checks;
- verify inclusion under the account root;
- prove the account root is the current spend state root;
- fail closed if path data is absent or malformed.

Consumes an accumulator/Merkle binding assumption only if the target chain does
not make the leaf directly consensus-visible.

### AggregatedProofHandle

```text
AggregatedProofHandle(slot_leaf, batch_root, proof_ref)
```

Optional.

Verifier obligations:

- all normalized digest and field checks;
- verify the proof handle under the concrete batch/rollup/accumulator system;
- prove the batch system admitted the same canonical leaf meaning as direct
  admission;
- consume `optional_aggregation_soundness`.

This handle is inactive in the direct single-slot theorem.

## Spend-Time Binding Checks

After handle validation, spend verification must check:

- checked-core challenge frame binds `slot_authorization_digest`;
- checked-core challenge frame binds the same public-key identity;
- checked-core challenge frame binds the same slot id;
- the account state root is the spend verifier's current state root;
- the slot is live before transition;
- the transition consumes the exact leaf;
- the same leaf cannot validate a second spend in a valid successor state.

## Rejection Classes

The handle verifier must reject or export a named break for:

- malformed leaf frame;
- wrong leaf domain;
- wrong authorization domain;
- digest mismatch;
- wrong account id;
- wrong public-key identity;
- wrong slot id;
- wrong verifier version;
- wrong account state root;
- wrong opening commitment;
- wrong pad-seed digest;
- wrong hidden-source statement digest;
- wrong lifecycle anchor;
- non-live status marker;
- missing prevalidation record;
- stale prevalidation record;
- unavailable inclusion/certificate data;
- direct/batch domain confusion;
- duplicate spend attempt.

## Theorem Hook

`validate_slot_authorization_handle(handle, current_state)` either returns
`ValidatedSlotAuthorizationV1` or rejects. If it returns a validated object, then
one of these must hold:

- the leaf was admitted by direct single-slot creation under the current account
  namespace; or
- the optional aggregation theorem is enabled and proves the same direct leaf
  meaning; or
- the adversary exports a registry, data-availability, lifecycle, accumulator,
  or aggregation-soundness break.

## Proof Tracker Closure

This spec closes the proof vocabulary item "Define spend-time authorization
handle" at the interface level. Executable closure remains open until the direct
record validator, inclusion-proof validator, negative controls, and checked-core
frame binding tests exist.
