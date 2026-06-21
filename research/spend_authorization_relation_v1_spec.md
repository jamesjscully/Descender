# SpendAuthorizationRelationV1 Spec

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This is a supporting relation spec note, not a normative spec. Current
> normative authority starts at `../spec/README.md`.

Status: proof relation specification for Path 2 spend-time authorization.

## Purpose

`SpendAuthorizationRelationV1` is the spend-time relation for the selected Path
2 design. It proves authorization to consume a live prevalidated slot plus
checked-core message binding. It does not prove or re-open the hidden-source
witness at spend time.

Hidden-source soundness is imported from account creation through the admitted
`PrevalidatedSlotLeafV1` and its `slot_authorization_digest`.

## Public Inputs

| input | role |
|---|---|
| `message` | application message signed by the spend |
| `public_key_identity` | key digest bound to the account and checked-core verifier |
| `account_id` | account namespace being spent |
| `current_account_state_root` | state root observed by the spend verifier |
| `slot_id` | one-use slot named by the spend |
| `checked_core_signature_body` | 252-byte Fq12 checked-core signature body |
| `slot_authorization_digest` | normalized Path 2 digest bound by checked-core challenge |
| `authorization_handle` | direct record, inclusion proof, or optional aggregated handle |
| `lifecycle_context` | consume-once state transition context |
| `verifier_version_digest` | spend verifier/profile version |

The concrete transaction format may include an inclusion path or proof handle,
but the relation consumes only the normalized output of handle validation.

## Normalized Handle Output

The authorization handle must validate to `ValidatedSlotAuthorizationV1`:

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

If handle validation cannot produce this object, spend authorization rejects.

## Relation Checks

`SpendAuthorizationRelationV1` holds iff all checks below hold.

### 1. Authorization Handle Validation

- the handle kind is enabled for the theorem mode;
- direct mode accepts only `DirectRecord`;
- inclusion mode additionally checks the account-root path;
- aggregated mode additionally consumes `optional_aggregation_soundness`;
- the handle recomputes `prevalidated_slot_leaf_digest`;
- the handle recomputes `slot_authorization_digest`;
- the leaf was admitted by the account-creation verifier under the current
  account namespace, or the optional aggregation theorem proves the same direct
  leaf meaning;
- missing direct record, unavailable inclusion data, or missing batch proof
  rejects.

### 2. Exact Field Agreement

The normalized handle fields must match the spend inputs:

- `account_id`;
- `public_key_identity`;
- `slot_id`;
- `current_account_state_root`;
- `verifier_version_digest`;
- `slot_authorization_digest`.

Mismatch rejects before checked-core acceptance is treated as authorization.

### 3. Checked-Core Message Binding

The checked-core verifier must accept the 252-byte Fq12 signature body under a
challenge frame that binds:

- `message`;
- public key;
- `account_id` or account namespace digest;
- `slot_id`;
- `current_account_state_root`;
- `slot_authorization_digest`;
- checked-core signature commitment/body.

Rebinding any one of these values must reject or export
`checked_core_message_binding_break`.

### 4. Live Slot And Lifecycle

- `live_status_marker == 0x01`;
- the slot is live before transition;
- the transition consumes or burns the exact leaf;
- the same leaf cannot authorize another accepted spend in any valid successor
  state;
- rollback, stale restore, or reorg behavior is charged through the lifecycle
  theorem and `slot_replay_or_rollback_break`.

### 5. No Spend-Time Hidden Opening

Spend-time authorization must not require raw pad seed, projection masks,
unmasked selected account projection residues, or private opening frames.
Those values either remain hidden or were already checked in a creation-time
proof/certificate view.

If a spend verifier accepts only after seeing those values, the protocol is no
longer using this relation and must switch to a separate opening-disclosure
theorem.

## Theorem Hook

The local spend theorem target is:

```text
spend_authorization_implies_admitted_leaf_or_break
```

If `SpendAuthorizationRelationV1` accepts, then either:

- the consumed leaf was admitted by `verify_prevalidated_slot_creation` for the
  same account id, public-key identity, slot id, verifier version, state root,
  opening commitment, pad-seed digest, hidden-source statement digest, and
  lifecycle anchor; or
- the adversary exports one of:
  - `prevalidation_registry_binding_break`;
  - `prevalidation_data_availability_break`;
  - `slot_replay_or_rollback_break`;
  - `checked_core_message_binding_break`;
  - `postchallenge_slot_creation_or_rewrite_break`;
  - `qrom_or_domain_separation_break`;
  - `optional_aggregation_soundness_break` only when aggregation is enabled.

## Direct Mode

Direct single-slot mode is the baseline deployment theorem.

It consumes:

- account-creation hidden-source soundness;
- direct record availability;
- registry binding;
- checked-core message binding;
- lifecycle consume-once semantics;
- Path 2 hash/domain assumptions.

It does not consume:

- accumulator assumptions;
- rollup proof assumptions;
- `optional_aggregation_soundness`.

## Optional Modes

`InclusionProof` and `AggregatedProofHandle` are representation changes after
handle validation. They cannot change the checked-core signature body or the
meaning of `slot_authorization_digest`.

If optional modes are enabled, they must additionally prove:

- direct-vs-batch leaf equivalence;
- canonical batch ordering where relevant;
- data availability for hidden records or proof references;
- accumulator/root/proof soundness.

## Required Negative Controls

Spend-time tests must reject:

- wrong account id;
- wrong public-key identity;
- wrong slot id;
- wrong current account state root;
- wrong verifier version;
- wrong `slot_authorization_digest`;
- stale or missing prevalidation record;
- non-live leaf;
- already-consumed leaf;
- direct/batch handle domain confusion;
- checked-core challenge rebinding to another message, key, slot, state root, or
  authorization digest;
- any path that requires raw hidden-source opening material at spend time.

## Current Conclusion

The relation is structurally achievable if the account-creation theorem and
registry binding theorem are completed. The core remaining risk is not the spend
relation itself; it is whether creation admission can provide a replayable
hidden-source certificate/proof view that preserves the live-slot leakage model.

## First Executable Direct-Admission Hook

`tools/parameter-search/src/production_reference.rs` now includes
`validate_path2_direct_slot_authorization_admission`. It validates the Path 2
leaf/evidence/certificate bindings, recomputes `slot_authorization_digest`, and
requires that exact digest to be present in an admitted-authorization registry.

Current KAT coverage:

- non-admitted leaf rejects;
- exact admitted leaf accepts;
- sibling leaf with different slot id rejects because its authorization digest
  is not admitted.
- state-namespace admission accepts only under the exact `(account_id,
  account_state_root_before, slot_authorization_digest)` tuple;
- wrong state-root namespace and wrong account namespace records reject.

This is a direct-mode authorization-to-admission skeleton with explicit state
namespace binding. The checked-core challenge frame binding is covered by the
next executable hook.

## First Executable Checked-Core Binding Hook

The same Rust module now includes
`path2_checked_core_authorization_challenge_frame`,
`path2_checked_core_authorization_challenge_bytes`, and
`require_path2_checked_core_authorization_challenge_binding`.

The Path 2 challenge frame binds:

- `DESCENDER_SLOT_CONSUME_SINGLE_V1`;
- public key;
- message;
- account id;
- account state root;
- slot id;
- checked-core witness commitment;
- `slot_authorization_digest`;
- checked-core proof commitment.

Current KAT coverage:

- a synthetic checked-core body carrying the Path 2 challenge accepts the
  binding check;
- changing `slot_authorization_digest` rejects;
- changing account state root rejects;
- the legacy checked-core challenge bytes differ from the Path 2 authorization
  challenge bytes.

The Rust module also includes `verify_path2_checked_core_authorization`, which
reuses the reviewed Fq12 checked-core equation path with the Path 2
authorization challenge binding replacing the legacy challenge binding.

Additional KAT coverage:

- a legacy checked-core body rejects at the Path 2 challenge-binding gate;
- a synthetic body with only the challenge bytes rewritten passes the Path 2
  challenge-binding gate and then rejects later in checked-core consistency,
  proving the function is not silently falling back to the legacy frame.

This closes the first executable challenge-binding composition slice. An
accepting Path 2 spend KAT exists for the zero-response fixture:
`path2_checked_core_authorization_accepts_signed_path2_fixture` derives the
Path 2 challenge, recomputes the challenge-dependent pad openings, and verifies
through `verify_path2_checked_core_authorization`.
`sign_path2_reviewed_fq12_authorization_checked_core` generalizes this into a
Path 2 checked-core signer helper: it derives the authorization-bound challenge,
solves the Fq12 response-syndrome preimage into a 22-residue masked response
vector, recomputes challenge-dependent pad openings for the supplied compact
responses, encodes the signature, and verifies it before returning. The KAT
`path2_checked_core_authorization_signer_accepts_nonzero_responses` covers a
nonzero-response signature.

The direct spend path is now composed by
`verify_path2_direct_single_slot_spend_authorization`. It verifies public-key
identity against the leaf, validates admission in the exact
`(account_id, account_state_root_before, slot_authorization_digest)` namespace,
runs the Path 2 checked-core verifier, rejects unless the decoded checked-core
slot id equals the prevalidated leaf slot id, and only then mutates the
consume-once set. KAT
`path2_direct_single_slot_spend_composes_admission_signature_and_consume_once`
accepts the composed direct path and rejects a second spend. KAT
`path2_direct_single_slot_spend_rejects_leaf_slot_rebinding` signs a valid
checked-core body for account slot 0 against another leaf authorization digest
and proves the composed verifier rejects before consumption.

The creation leaf's hidden-source statement is now fixed before the spend
message and signature exist.
The current reviewed Fq12 hidden-source statement frame is signature-dependent;
using it directly in the Path 2 leaf creates a circular dependency through
`slot_authorization_digest`. Path 2 spend authorization must therefore consume a
leaf bound to `CreationHiddenSourceStatementV1`, while the spend challenge binds
message, proof commitment, and `slot_authorization_digest` separately.
