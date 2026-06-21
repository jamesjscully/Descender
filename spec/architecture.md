# Chain-Native PQ Authorization Architecture

Status: ACTIVE_NORMATIVE

## Objective

Descender is being organized around the following product objective:

```text
be the default post-quantum authorization stack for account-based chains,
rollups, bridges, and high-throughput execution layers
```

The project should optimize for the blockchain authorization surface:

- compact spend authorization;
- deterministic consensus verification;
- state and fork/reorg binding;
- one-use authorization safety;
- account policy expressiveness;
- batch and recursive proof aggregation;
- light-client and rollup compatibility;
- hardware-wallet and custody deployability.

Concrete adoption requirements for these surfaces are normative in
`spec/blockchain_requirements.md`. Signer, wallet, HSM, and custody safety
requirements are normative in `spec/signer_profile.md`. Slot replay, rollback,
and recovery semantics are normative in `spec/lifecycle_model.md`.

## Mode A: Direct Single-Slot Authorization

Mode A is the first deployable target.

Creation or refresh:

```text
verify_prevalidated_slot_creation(slot_leaf, creation_evidence) -> accept
```

Spend:

```text
verify_direct_single_slot_spend(
  message,
  checked_core_signature,
  slot_authorization_digest,
  direct_slot_record,
  account_state
) -> accept and consume slot
```

Mode A consumes no aggregation assumption. It must be secure and complete on its
own.

Required properties:

- canonical slot leaf;
- admitted slot registry binding;
- checked-core message binding;
- exact slot-id agreement;
- consume-once successor-state transition;
- public-key and account namespace binding;
- no spend-time hidden opening;
- deterministic public rejection behavior.

The consume-once transition is not a local cache hint. It is a consensus-visible
lifecycle transition that must satisfy `spec/lifecycle_model.md`.

## Mode B: Aggregated / Rollup Authorization

Mode B is the scale target after Mode A.

The spend signature body should not change. Only the representation proving the
same `slot_authorization_digest` changes:

```text
DirectRecord(slot_leaf)
InclusionProof(slot_leaf, account_root, path)
AggregatedProofHandle(slot_leaf, batch_root, proof_ref)
```

Mode B may add:

- batch account creation;
- aggregated slot admission;
- aggregated consume-once proofs;
- block-level proof of many spends;
- committee or bridge authorization proofs;
- light-client proof objects.

Mode B activates `optional_aggregation_soundness` only after the concrete proof
system or accumulator is selected.

## What Descender Should Not Claim

Descender should not claim native BLS-style aggregation. Its aggregation story is
proof/state aggregation:

```text
many PQ authorizations + policy/state checks -> one proof/root
```

Descender should not claim that multisig, threshold custody, bridge committee
quorums, account recovery, or rollup compression are solved by the base
signature primitive alone. They are account/state/proof-layer features.

## Policy Layer

Policy features belong above the admitted slot abstraction:

- M-of-N multisig;
- account recovery / guardian policy;
- timelocks and rate limits;
- validator quorum checks;
- bridge committee threshold checks;
- application-specific authorization classes.

The long-term target is to make these policies cheap through aggregation and
recursion, not to force them into a single native algebraic signature combine.

## Competitive Positioning

Against raw PQ signatures, Descender should win by solving the chain-level
problem:

| Raw PQ signature schemes provide | Descender must provide |
|---|---|
| one public key, one message, one signature | account/state authorization |
| verifier for one signature | direct and aggregated authorization modes |
| no native lifecycle model | consume-once state semantics |
| no policy compression | account policy plus proof aggregation |
| no light-client compression | rollup/block proof path |
| no custody/wallet lifecycle story | signer/keygen/hardware-wallet discipline |
