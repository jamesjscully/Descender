# Path 2 Signature-Dependent Statement Circularity Blocker

> MIGRATION STATUS: `ACTIVE_RESEARCH`. This note records blocker history and the
> split-statement rationale. It is useful evidence, but current proof status
> lives in `TRACKER.md`, `spec/security_model.md`, and newer Path 2 theorem
> support notes.

Status: top proof blocker before an accepting Path 2 spend fixture.

## Finding

The current reviewed Fq12 active-lane statement is signature-dependent. Its
canonical frame includes both `message_digest` and `signature_digest`. That is
acceptable for the older spend-carried 308-byte/Fq12 route, but it creates a
cycle when the same statement digest is placed in a Path 2 prevalidated slot
leaf and the spend challenge is required to bind the resulting
`slot_authorization_digest`.

The dependency cycle is:

```text
hidden_source_statement_digest
  -> reviewed_fq12_hidden_source_statement_frame
  -> signature_digest
  -> signature bytes
  -> checked-core challenge
  -> path2_checked_core_authorization_challenge_frame
  -> slot_authorization_digest
  -> PrevalidatedSlotLeafV1
  -> hidden_source_statement_digest
```

The message digest is also spend-time data. Binding it into the creation-time
statement would make one prevalidated slot authorize only the message known at
creation time, which is not the intended blockchain spend model.

## Consequence

An accepting Path 2 spend fixture should not be forced by searching for a hash
fixed point or by rewriting only the checked-core challenge inside an old
signature. That would test an accidental circular object, not the selected
account-creation design.

The existing active-lane replay hook remains useful, but only as:

- a fail-closed baseline for the older 308-byte/Fq12 path;
- a negative-control source for statement/opening/pad/carrier binding;
- a temporary executable skeleton for leaf/evidence/certificate binding.

It is not the final Path 2 creation relation statement.

## Required Design Change

Path 2 needs a creation-time statement whose digest is fixed before the spend
challenge. The final statement must exclude spend-time `message_digest` and
`signature_digest`.

The replacement should be one of:

1. `CreationHiddenSourceStatementV1`: a creation-time public statement over
   account id, public-key identity, slot id, state root, opening commitment,
   pad-seed digest, public pad-opening statement digest, selected active-lane
   profile, verifier version, lifecycle anchor, and an optional
   aggregation-neutral context digest.
2. A split statement model: creation leaf binds the message/signature-independent
   hidden-source statement, while spend-time checked-core binds message,
   proof commitment, and `slot_authorization_digest` separately.

The second form is the expected implementation shape because it preserves the
blockchain property that a prevalidated slot can authorize a future message.

## Proof Obligation After The Split

The theorem target changes from "the current active-lane statement digest is in
the leaf" to:

```text
accepted creation leaf/evidence
  -> valid CreationHiddenSourceStatementV1 opening
  or named binding/soundness/ZK/hash break

accepted spend
  -> checked-core signature binds message and slot_authorization_digest
  and slot_authorization_digest names a previously admitted creation statement
  or named registry/lifecycle/checked-core break
```

This removes the circular dependency while keeping the strong property needed
for aggregation later: the creation statement is a stable public object, and the
spend challenge binds a compact digest of that object through the slot leaf.

## Immediate Work

- define canonical `CreationHiddenSourceStatementV1` bytes;
- update `Path2DirectReplayableRelationCertificateV1` to name that statement;
- replay existing active-lane negative controls against the split statement
  fields where possible;
- build the accepting Path 2 spend fixture only after the signer derives its
  checked-core responses from the non-circular Path 2 challenge.

First executable slice: `tools/parameter-search/src/production_reference.rs`
now includes `Path2CreationHiddenSourceStatementV1`,
`path2_creation_hidden_source_statement_v1_frame`, and
`path2_creation_hidden_source_statement_v1_digest`. The KAT
`path2_creation_hidden_source_statement_v1_is_canonical_and_non_circular`
proves the frame contains no `message_digest` or `signature_digest` label, is
stable under legacy message/signature rebinding, and still changes when the
pad-opening statement digest changes. The Path 2 relation replay fixture now
uses this digest as the leaf/evidence/certificate `hidden_source_statement_digest`
and keeps the reviewed Fq12 statement digest as a separate
`reviewed_active_lane_statement_digest` for direct replay.
