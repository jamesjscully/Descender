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

The production reference emits `Path2DirectTheoremBoundaryReportV1`, which pins
the accepted tuple, acceptance hypotheses, per-hypothesis obligation matrix,
named break classes, and current open blocker for this theorem boundary. This
report is not the proof; it is the mechanical guard that prevents deployment
preflight from drifting away from the theorem shape while the composition
theorem remains open.

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
- `consensus_rejection_or_state_mutation_break`;
- external coded-SIS / lattice-core assumption breaks.

## Account-Creation Proof Boundary Contract

Tracker ownership: `DIR-CREATE-001`, `DIR-PROOF-001`, `DIR-PROOF-002`,
`DIR-STARK-001`, `DIR-BIND-001`, `DIR-PAYLOAD-001`,
`DIR-AUTHCLASS-001`, `DIR-KAT-001`, `DIR-REJECT-001`, `DIR-CT-001`,
`DIR-QROM-001`, and `DIR-REVIEW-001`.

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

For the current Path 2 route, `selected_payload` is
`Path2AccountStarkCreationPayloadV1`: an opaque
`descender-account-verifier` bundle carrying `(AccountCreationPublic, Proof)`
plus plain consensus anchors. The accepting endpoint is
`verify_path2_direct_creation_account_stark_payload_boundary`; the transparent
descent/native-boundary paths are retained only as fail-closed research or
fallback support unless a later ADR selects them.

Admission acceptance requires all of:

- canonical leaf and evidence framing;
- evidence/leaf equality for account id, public-key identity, slot id,
  verifier version, state root before admission, opening commitment,
  pad-seed digest, hidden-source statement digest, lifecycle anchor, and
  transcript digest;
- certificate/evidence equality where `evidence_payload_digest` binds the
  replay certificate and `public_replay_payload_digest` equals the canonical
  `path2_account_stark_creation_payload_v1_digest(selected_payload)`;
- payload availability, nonempty STARK bundle bytes, and canonical digest
  equality for every plain payload field and the opaque proof bundle;
- payload equality for leaf context, creation-proof binding digest,
  aggregation context, and slot id;
- successful verification of the packed account STARK bundle;
- `DeployedCreationAnchors` equality for account id, slot id, state root,
  lifecycle anchor, raw verifier version, masked projection,
  packed-witness commitment, and the named role-root export bridge;
- zero-knowledge/no-export semantics for pad source, projection, syndrome,
  opening tuple, witness, mask, and private trace material;
- QROM/domain separation for the account STARK transcript, Poseidon2/FRI
  challenge flow, and deployed digest frames;
- deterministic public rejection for malformed, unavailable, or
  policy-incompatible payloads.

The creation verifier must fail closed when any required payload bytes,
availability records, account STARK proof bytes, public anchors, production
gate, or proof-family parameters are missing. It must not admit a leaf merely
because `prevalidation_transcript_digest`, `evidence_payload_digest`, or a
commitment root is well formed.

The production reference emits `Path2AccountCreationProofBoundaryReportV1`,
which pins the selected payload type, verifier endpoints, replay-payload digest
function, acceptance requirements, exact leaf/evidence binding fields,
certificate/evidence binding fields, binding/leaf context fields,
payload-binding fields, deployed-anchor fields, no-export private-material
field classes, and current proof-family closure blockers. The report is the
executable boundary map for this section; it is not a simulator, extractor,
FRI/QROM, KAT, or external-review substitute.

The boundary is implemented but not verified. The active Rust test suite now has
an initial `Path2AccountStarkCreationPayloadV1` malformed-payload mutation
matrix, including empty-bundle, digest-bound, rebound-anchor, corrupted-bundle,
and production-gate checks. Open closure items are the final composition
theorem, account STARK simulator theorem, extractor/soundness theorem,
FRI/recursion residual-assumption ledger, complete frozen byte-format KATs,
signer leakage evidence, external proof-family review, and the explicit
production-gate decision.

## Selected Account-STARK Residual Matrix

Tracker ownership: `DIR-PROOF-001`, `DIR-PROOF-002`, `DIR-STARK-001`,
`DIR-BIND-001`, `DIR-KAT-001`, `DIR-REJECT-001`, `DIR-QROM-001`, and
`DIR-REVIEW-001`.

This matrix is the normative residual-assumption ledger for the selected
account-creation proof family. Older transparent, native-boundary, masked-trace,
or descent/SIS proof notes are support history unless they are explicitly mapped
to one row here. No deployment claim may rely on a support note that bypasses
this matrix. The production deployment preflight must carry the open rows below
as explicit account-STARK residual blockers until each row is discharged,
accepted by a deployment-profile decision, or retired by ADR.
`Path2AccountStarkResidualMatrixReportV1` is the executable report for this
matrix; `Path2AccountCreationProofBoundaryReportV1` binds to its digest so the
account-creation boundary cannot be treated as complete while these rows drift
or remain open.
`Path2AccountStarkPayloadKatBoundaryReportV1` pins the current
`Path2AccountStarkCreationPayloadV1` mutation cases and maps their internal
diagnostics to consensus-facing public rejection classes. It is still precursor
coverage: `formal/kat/path2_account_stark_payload_kat_boundary_v1.json`
regenerates and diffs the draft boundary manifest, but full frozen vector files
remain open. `formal/kat/path2_account_stark_payload_kat_vectors_v1.json`
regenerates the current-row digest-pinned boundary vectors under the real
account-STARK verifier feature. It uses the explicit deterministic KAT producer
`path2_account_stark_prove_creation_with_mask_seed_v1` to pin payload-frame and
account-STARK bundle digests, and it runs the proof inside a single-thread Rayon
pool so proof-of-work witness selection is reproducible. The deployment producer
remains fresh-randomized.

| Residual item | Current evidence | Required closure before deployment | Named break |
|---|---|---|---|
| `Path2AccountStarkCreationPayloadV1` byte and replay binding | Wire-format digest frame, certificate/evidence binding, deployed anchor checks, active mutation tests, `Path2AccountStarkPayloadKatBoundaryReportV1` public rejection-class mapping, draft manifest regeneration, and deterministic digest-pinned current-row boundary vectors exist. | Full frozen byte corpus and public rejection-class mapping for every consensus-visible malformed payload. | `account_stark_payload_binding_break` |
| AIR relation soundness | `research/path2_account_stark_air_soundness_audit.md` reports all relation classes wired and MED findings remediated. | External proof-family review confirms the AIR constraints match the claimed creation relation and no false statement accepts except with a named proof break. | `account_stark_air_soundness_break` |
| FRI argument-of-knowledge extractor | The AIR is fully constrained, but the audit still names the `p2_sis_extractor`-style FRI-IOP extractor as open. | A concrete extractor theorem for the selected Plonky3/q12289 configuration shows accepting proof implies an extractable account-creation witness or exports a named FRI/AoK break. | `account_stark_fri_aok_extractor_break` |
| Zero-knowledge simulator and no-export statement | Boundary text forbids export of pad source, projection, syndrome, opening tuple, witness, mask, and private trace material; current tests cover public payload mutations, not simulator semantics. | Simulator theorem for the selected payload proves the accepted public bundle can be simulated from public anchors and allowed commitments without revealing private creation material. | `account_stark_simulator_or_no_export_break` |
| FRI soundness margin and recursion ledger | Current `PROD_SOUNDNESS_BITS=128` is a conservative-Johnson label over the active field/challenge configuration; the security model separately states the ~124-bit generic floor. | Residual-assumption ledger prices FRI queries, field size, Poseidon2 transcript, recursion if used, and multi-target/QROM terms against the selected deployment profile. | `fri_soundness_margin_break` |
| Production admission gate and review disposition | `verify_path2_direct_creation_account_stark_payload_boundary_for_production` remains fail-closed and regression-tested. | External review, tracker disposition, and explicit maintainer decision flip the production gate; otherwise production admission must continue to reject. | `account_stark_production_gate_break` |
| Legacy proof-route disposition | Transparent descent/native-boundary paths are fail-closed support and cannot be selected by implication. | Any route change requires an ADR that updates this matrix, wire formats, KAT scope, and tracker ownership before the route can become deployment authority. | `proof_route_authority_break` |

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
`DIR-PROOF-001`, `DIR-PROOF-002`, `DIR-STARK-001`, `DIR-BIND-001`,
`DIR-PAYLOAD-001`, `DIR-AUTHCLASS-001`, `DIR-KAT-001`, `SPEC-KAT-001`,
`DIR-REJECT-001`, `DIR-CT-001`, `SPEC-SIGNER-001`, `SPEC-LEAKAGE-001`,
`DIR-LIFE-001`,
`SPEC-LIFE-001`, `SPEC-DEPLOY-001`, `DIR-REVIEW-001`, `SPEC-REVIEW-001`,
`DIR-QROM-001`, and `DIR-ACTION-001`.

Risk-first means direct-mode work must close deployment blockers in the order
below unless an append-only ADR explicitly retires or re-ranks an item. Lower
rows may receive scaffolding, probes, and KAT plumbing, but they must not be
used as deployment evidence while a higher-ranked blocker remains open.

| Rank | Blocker | Must close | Owning rows |
|---|---|---|---|
| 1 | Direct theorem closure | `Path2DirectTheoremBoundaryReportV1` pins the product theorem boundary, and the final product theorem covers admission, spend, consume-once lifecycle, exact public bindings, and every named break; supporting lemmas cannot stand in for the deployment theorem. | `DIR-RISK-001`, `DIR-SEC-001`, `DIR-CREATE-001`, `DIR-PROOF-002`, `DIR-AUTHCLASS-001` |
| 2 | Account-creation proof boundary | `Path2AccountCreationProofBoundaryReportV1` pins the selected account-STARK payload boundary, and the payload has a concrete byte model, canonical replay-payload digest, deployed anchor binding, simulator theorem, extractor theorem, hidden-source payload statement, production gate, and rejection behavior. | `DIR-CREATE-001`, `DIR-PROOF-001`, `DIR-STARK-001`, `DIR-BIND-001`, `DIR-PAYLOAD-001`, `DIR-KAT-001`, `DIR-REJECT-001` |
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
- selected account-STARK residual-assumption matrix with every row either
  discharged, externally reviewed, accepted by explicit deployment profile, or
  retired by ADR;
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
