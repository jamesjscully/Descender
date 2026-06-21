# Security Model

Status: ACTIVE_NORMATIVE

## Main Claim Shape

The direct-mode deployable theorem must have this shape:

```text
If direct single-slot Path 2 spend authorization accepts, then:

1. the checked-core signature is valid for the message, public key,
   account id, state root, slot id, proof commitment, and
   slot_authorization_digest;
2. slot_authorization_digest names a canonical PrevalidatedSlotLeafV1;
3. that leaf was admitted before the spend challenge under the same account
   and state-root namespace;
4. admission accepted a replayable creation-time relation proof/certificate;
5. the exact authorization is consumed once in a valid successor state
   satisfying `spec/lifecycle_model.md`;

or the adversary exports a named break.
```

This is the product theorem. Partial statements such as "the core relation
proves C1/C2/C4/C6" are supporting lemmas, not the deployable claim.

## Named Break Classes

Direct mode may name, but must not hide, these break classes:

- `creation_time_hidden_source_soundness_break`;
- `prevalidation_leaf_binding_break`;
- `prevalidation_registry_binding_break`;
- `prevalidation_data_availability_break`;
- `postchallenge_slot_creation_or_rewrite_break`;
- `checked_core_message_binding_break`;
- `slot_replay_or_rollback_break`;
- `qrom_or_domain_separation_break`;
- `live_slot_leakage_or_keygen_break`;
- proof-family soundness breaks for the selected account-creation payload proof;
- external coded-SIS / lattice-core assumption breaks.

## Current Security-Level Position

The active Poseidon2/Plonky3 path has a real generic floor around 124 bits from
the width-16 Poseidon2 sponge capacity and 8-element Merkle digest birthday
bound. This is acceptable as the current practical design target only if it is
stated plainly.

The project must not claim "hard 128-bit security everywhere" while the width-16
Poseidon2 floor remains active. A future hard-128 profile requires a separate
decision and implementation path.

## Challenge Budget Terminology

Challenge-bit statements must distinguish these quantities:

- decoded local-control bits: `REPS * CHALLENGE_BITS_PER_REP = 130`; older
  retired-path reports sometimes combine these with the 64 hardening bits and
  report 194 bits;
- byte/mask transcript budget: `ALGEBRAIC_CHALLENGE_BYTES = 23` with
  `ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK = 0x3f`, giving 182 algebraic byte/mask
  bits plus 64 hardening bits, for 246 total challenge bytes after masking;
- selected deployment transcript term: 182 bits after the current
  multi-target/QROM pricing ledger.

The byte/mask transcript budget is the quantity used by the current reference
query-budget object and concrete-security smoke check. The decoded
local-control count is a parser/control-flow quantity, not the transcript
budget.

The active transaction Fiat-Shamir challenge bytes are Poseidon2-derived. The
reviewed Fq12 checked-core action map is a separate map from those challenge
bytes into `F_q^12` (`reviewed_fq12_challenge_action_vector`) and is charged as
a distinct action-collision term. A SHA-256-derived action map in cryptanalysis
evidence must not be read as the transaction Fiat-Shamir hash path.

For the current deployment ledger, the generated multi-target arithmetic caps
the aggregate public-key/message/account target ledger at `log2(T) <= 54`.
Deployment profiles that need a larger target horizon must either widen the
algebraic byte/mask challenge budget or lower the query/target ledger.

## Required Evidence Before Deployment Claim

Tracker ownership: `DIR-SEC-001`, `DIR-KAT-001`, `SPEC-KAT-001`,
`DIR-REJECT-001`,
`SPEC-SIGNER-001`, `SPEC-LEAKAGE-001`, `DIR-CT-001`, `DIR-QROM-001`,
`DIR-REVIEW-001`, `DIR-LIFE-001`, `SPEC-DEPLOY-001`, and
`SPEC-REVIEW-001`.

The deployment claim requires all of:

- final canonical byte/frame specs;
- executable KATs for every public field mutation;
- direct-mode end-to-end signer/verifier test vectors;
- panic-free and canonical rejection behavior;
- signer/keygen/sampler evidence satisfying `spec/signer_profile.md`;
- no secret-bearing errors, logs, or public views;
- QROM/hash/domain budget ledger tied to the final theorem;
- external cryptanalysis of the coded-SIS/lattice core;
- external review of the account-creation proof family and Poseidon2 usage;
- external review package satisfying `spec/review_package.md`;
- lifecycle/fork/reorg/resource policy instantiated for the target chain under
  `spec/lifecycle_model.md` and `spec/deployment_profiles.md`.

## Aggregation Mode

Aggregation mode is a separate theorem mode. It imports all direct-mode
assumptions and adds:

- concrete accumulator or proof-system soundness;
- canonical batch ordering;
- duplicate prevention;
- data availability for any records not directly stored in consensus state;
- aggregate consume-once semantics;
- light-client proof semantics if the aggregate is used by light clients.

Direct mode must remain valid without these assumptions.

## Non-Claims

Current non-claims:

- native BLS-style signature aggregation;
- threshold signing as a single compact native Descender signature;
- adaptor signatures;
- consensus finality or arbitrary reorg handling inside the cryptographic
  theorem;
- hard 128-bit security without a future hardening profile;
- deployability based only on internal tests or internal proof notes.
