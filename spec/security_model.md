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

## Direct Theorem Acceptance Boundary

Tracker ownership: `DIR-SEC-001`, `DIR-CREATE-001`, `DIR-HANDLE-001`,
`DIR-CC-001`, `DIR-LIFE-001`, `DIR-REJECT-001`, and `DIR-KAT-001`.

The direct theorem statement is fixed around one accepted spend tuple. Research
notes may refine supporting lemmas, but they must not change this tuple or its
acceptance hypotheses without updating this file and the direct-mode gates.

The accepted tuple contains:

- public key or public-key identity;
- message or message digest;
- account id;
- account state root before spend;
- slot id;
- checked-core signature body;
- canonical `PrevalidatedSlotLeafV1`;
- `AccountCreationEvidenceV1`;
- selected account-creation proof payload;
- direct admission namespace record;
- consume-once state before and after spend.

The proof may assume acceptance only if all checks below pass:

1. the leaf is canonical and live;
2. the evidence envelope duplicates and matches every leaf binding field;
3. `slot_authorization_digest` equals
   `SHA256(DESCENDER_SLOT_AUTHORIZATION_V1 || PrevalidatedSlotLeafV1_bytes)`;
4. the selected account-creation verifier accepted the evidence payload before
   the spend challenge was derived;
5. the direct admission namespace contains the exact
   `(account_id, account_state_root_before, slot_authorization_digest)` tuple;
6. the checked-core challenge frame binds the accepted public key, message,
   account id, state root, slot id, proof commitment, and
   `slot_authorization_digest`;
7. checked-core verification accepts under the active Poseidon2 challenge path
   and separated Fq12 action-map term;
8. the lifecycle transition consumes the exact authorization once in a valid
   successor state;
9. every rejection is deterministic, public, and state-preserving.

If a proof step needs a stronger, weaker, or differently shaped hypothesis, the
theorem statement is not yet closed for deployment. The proof must instead
export one of the named breaks below or update this boundary by ADR/spec change.

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

## Account-Creation Proof Boundary Contract

Tracker ownership: `DIR-CREATE-001`, `DIR-PROOF-001`, `DIR-PROOF-002`,
`DIR-PAYLOAD-001`, `DIR-TRACE-001`, `DIR-NB-001`, `DIR-NB-002`,
`DIR-NB-003`, `DIR-NB-004`, `DIR-AUTHCLASS-001`, `DIR-KAT-001`,
`DIR-REJECT-001`, `DIR-CT-001`, `DIR-QROM-001`, and `DIR-REVIEW-001`.

Direct account creation admits a leaf only if the public creation verifier
accepts `AccountCreationEvidenceV1` and an available selected payload for that
exact leaf. A digest, commitment root, transcript hash, or fail-closed
placeholder is not proof semantics by itself.

The selected account-creation boundary is:

```text
verify_prevalidated_slot_creation(
  PrevalidatedSlotLeafV1,
  AccountCreationEvidenceV1 {
    evidence_kind = direct_replayable_relation_certificate,
    evidence_payload_digest = digest(selected_payload)
  },
  selected_payload
) -> accept/reject
```

For the current Path 2 route, `selected_payload` is the transparent masked
two-view native-boundary proof payload over the creation relation. Admission
acceptance requires all of:

- canonical leaf and evidence framing;
- evidence/leaf equality for account id, public-key identity, slot id,
  verifier version, state root before admission, opening commitment,
  pad-seed digest, hidden-source statement digest, lifecycle anchor, and
  transcript digest;
- payload availability and payload digest equality;
- a public masked view authenticated under one `native_boundary_digest`;
- a hidden extractor view for the same native boundary;
- proof-family soundness extracting some `h in Fiber(B_c, b)` satisfying
  `ExtractedRolesComplete`, `LocalValid_c`, `DescentConsistent_c`,
  account-binding, and range predicates;
- zero-knowledge/no-export semantics for pad-source, projection, opening-tuple,
  range, witness, mask, and private trace material;
- QROM/domain separation for the proof transcript and query schedule;
- deterministic public rejection for malformed, unavailable, or
  policy-incompatible payloads.

The creation verifier must fail closed when any required payload bytes,
availability records, query openings, proof-family parameters, or native-boundary
projection checks are missing. It must not admit a leaf merely because
`prevalidation_transcript_digest`, `evidence_payload_digest`, or a commitment
root is well formed.

The boundary is specified but not verified. Open closure items are the concrete
masked two-view verifier, trace/relation encoding against the native-boundary
manifest, simulator theorem, extractor/soundness theorem, concrete proof-system
parameters, accepting honest-proof KATs, malformed-payload KATs, and external
review of the proof family.

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

## Risk-First Deployment Blocker Order

Tracker ownership: `DIR-RISK-001`, `DIR-SEC-001`, `DIR-CREATE-001`,
`DIR-PROOF-001`, `DIR-PROOF-002`, `DIR-PAYLOAD-001`, `DIR-TRACE-001`,
`DIR-NB-001`, `DIR-NB-002`, `DIR-NB-003`, `DIR-NB-004`,
`DIR-AUTHCLASS-001`, `DIR-KAT-001`, `SPEC-KAT-001`, `DIR-REJECT-001`,
`DIR-CT-001`, `SPEC-SIGNER-001`, `SPEC-LEAKAGE-001`, `DIR-LIFE-001`,
`SPEC-LIFE-001`, `SPEC-DEPLOY-001`, `DIR-REVIEW-001`, `SPEC-REVIEW-001`,
`DIR-QROM-001`, and `DIR-ACTION-001`.

Risk-first means direct-mode work must close deployment blockers in the order
below unless an append-only ADR explicitly retires or re-ranks an item. Lower
rows may receive scaffolding, probes, and KAT plumbing, but they must not be
used as deployment evidence while a higher-ranked blocker remains open.

| Rank | Blocker | Must close | Owning rows |
|---|---|---|---|
| 1 | Direct theorem closure | Final product theorem covers admission, spend, consume-once lifecycle, exact public bindings, and every named break; supporting lemmas cannot stand in for the deployment theorem. | `DIR-RISK-001`, `DIR-SEC-001`, `DIR-CREATE-001`, `DIR-PROOF-002`, `DIR-AUTHCLASS-001` |
| 2 | Account-creation proof boundary | Selected proof family has a concrete byte model, simulator theorem, extractor theorem, hidden-source payload statement, native-boundary semantics, and rejection behavior. | `DIR-CREATE-001`, `DIR-PROOF-001`, `DIR-PAYLOAD-001`, `DIR-TRACE-001`, `DIR-NB-001`, `DIR-NB-002`, `DIR-NB-003`, `DIR-NB-004` |
| 3 | Authorization-class binding | Private authorization-local material cannot change public spend semantics, fork account state, or bypass the checked-core public binding. | `DIR-AUTHCLASS-001`, `DIR-SEC-001`, `DIR-CC-001` |
| 4 | Consensus conformance | Canonical wire formats, public-field mutation vectors, end-to-end creation/spend/lifecycle vectors, and deterministic rejection behavior exist. | `DIR-KAT-001`, `SPEC-KAT-001`, `DIR-REJECT-001`, `DIR-CC-001`, `DIR-LEAF-001`, `DIR-EVID-001` |
| 5 | Signer leakage and fault safety | Signer, keygen, sampler, rejection loops, fault handling, errors, zeroization, target/build profiles, and leakage evidence satisfy the signer profile without floating-point signing assumptions. | `DIR-CT-001`, `SPEC-SIGNER-001`, `SPEC-LEAKAGE-001`, `ADOPT-HW-001`, `ADOPT-HW-CT-001`, `ADOPT-HW-ENT-001`, `ADOPT-HW-STATE-001` |
| 6 | Lifecycle target profile | Admit, spend, replay, burn, recovery, restore, finality, and reorg semantics are instantiated for the target chain or deployment profile. | `DIR-LIFE-001`, `SPEC-LIFE-001`, `SPEC-STATE-001`, `SPEC-DEPLOY-001` |
| 7 | External review and residual assumptions | Independent review covers the coded-SIS/lattice core, account-creation proof family, Poseidon2/FRI usage, QROM framing, signer/verifier engineering, lifecycle model, and residual-assumption dispositions. | `DIR-REVIEW-001`, `SPEC-REVIEW-001`, `DIR-QROM-001`, `DIR-ACTION-001`, `DIR-SEC-001` |

A direct-mode deployment claim can be made only after every ranked blocker above
is either `verified` in `TRACKER.md` or explicitly retired by an ADR that updates
this section and the requirement gates. Mode B aggregation, policy features,
Ethereum positioning, commercial demos, and wallet integrations must not be used
to compensate for an open blocker in ranks 1 through 7.

## Required Evidence Before Deployment Claim

Tracker ownership: `DIR-RISK-001`, `DIR-SEC-001`, `DIR-CREATE-001`,
`DIR-PROOF-001`, `DIR-PROOF-002`, `DIR-AUTHCLASS-001`, `DIR-KAT-001`,
`SPEC-KAT-001`, `DIR-REJECT-001`, `SPEC-SIGNER-001`,
`SPEC-LEAKAGE-001`, `DIR-CT-001`, `DIR-QROM-001`, `DIR-REVIEW-001`,
`DIR-LIFE-001`, `SPEC-DEPLOY-001`, and `SPEC-REVIEW-001`.

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
