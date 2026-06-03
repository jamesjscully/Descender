# Path 2 Theorem Mode Switch

Status: theorem-mode contract for direct and optional aggregation modes.

Path 2 has one spend signature frame. The theorem mode changes only how
validators prove that `slot_authorization_digest` names an admitted live leaf.

## Mode A: Direct Single-Slot

Mode A is the first deployment proof target.

Required inputs:

- canonical `PrevalidatedSlotLeafV1`;
- `AccountCreationEvidenceV1`;
- `DirectReplayableRelationCertificateV1`;
- direct namespace record for
  `(account_id, account_state_root_before, slot_authorization_digest)`;
- consume-once state.

Assumptions consumed:

- direct replayable relation certificate soundness;
- SHA-256/RO domain separation for Path 2 frames;
- checked-core reviewed Fq12 soundness for the spend body;
- lifecycle/reorg policy for valid successor states;
- live-slot leakage/keygen assumptions.

Assumptions not consumed:

- aggregation soundness;
- accumulator soundness;
- batch inclusion soundness;
- rollup proof soundness.

## Mode B: Optional Aggregated Admission

Mode B is inactive unless the implementation enables aggregated/batch
admission.

Additional required inputs:

- canonical batch or accumulator root;
- deterministic leaf ordering rule;
- duplicate-rejection proof for `(account_id, slot_id)` and `pad_seed_digest`;
- inclusion/data-availability proof for the leaf and direct-equivalent evidence;
- concrete aggregation proof system identifier.

Additional assumptions consumed:

- `optional_aggregation_soundness`;
- accumulator/root binding;
- canonical batch ordering;
- direct-vs-batch leaf equivalence;
- batch data availability;
- any rollup/proof-system-specific soundness assumptions.

## Invariant Across Modes

The spend signature frame is identical in both modes:

```text
DESCENDER_SLOT_CONSUME_SINGLE_V1
public_key
message
account_id
account_state_root
slot_id
witness_commitment
slot_authorization_digest
proof_commitment
```

Aggregation changes only the validation path for `slot_authorization_digest`.
It must not change checked-core challenge derivation, signature bytes, or the
meaning of `CreationHiddenSourceStatementV1`.

## Promotion Rule

No document may claim Mode B readiness until the concrete aggregation proof
system is named and its assumptions are added to the proof boundary ledger.
Mode A deployment proof work must not depend on Mode B assumptions.
