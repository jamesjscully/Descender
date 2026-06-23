# Blockchain Authorization Requirements

Status: ACTIVE_NORMATIVE

This file turns the project objective into product requirements. Descender is
not trying to be only a smaller post-quantum `CHECKSIG`; it is trying to be the
authorization stack a chain, rollup, bridge, or custody system can adopt without
discovering that the important chain-level work was left out.

## Requirement Classes

Descender requirements are split into three classes.

| Class | Meaning |
|---|---|
| `direct` | required for Mode A single-slot deployment |
| `mode_b` | required for aggregation, rollups, committees, or light clients |
| `policy` | required above the slot abstraction in account/state logic |

Direct requirements must not depend on `optional_aggregation_soundness`. Mode B
requirements may depend on a named proof system, accumulator, or recursive
wrapper. Policy requirements may depend on application/account state, but must
bind to the same `slot_authorization_digest` and the lifecycle model in
`spec/lifecycle_model.md`. The normative policy object model lives in
`spec/policy_model.md`.

Every capability row, adoption gate, and surface row must cite the
`TRACKER.md` rows that carry the corresponding work. A requirement without a
tracker row is not allowed to remain only prose. Adoption-gate and surface
readiness must use only the readiness states defined below.

## Readiness Semantics

Readiness labels are requirement status, not marketing language. They apply to
the cited gate or surface only and do not make broader deployment claims.

| Status | Meaning |
|---|---|
| `not_ready` | The required artifact is absent, exploratory, or unsupported by executable evidence; it is not adoption evidence. |
| `partial` | Some specification, implementation, proof, or test evidence exists, but at least one required artifact remains open; it is not adoption evidence. |
| `ready` | The required artifact exists, the cited tracker rows are all `verified`, and the row can be used as adoption evidence for its stated scope. |

## Capability Matrix

| Capability | Required class | Descender surface | Current stance | Tracker rows |
|---|---|---|---|---|
| Simple transaction authorization | `direct` | checked-core spend over one admitted slot | required first deployable path | `DIR-SPEND-001`, `DIR-CC-001`, `DIR-SEC-001` |
| One-use authorization lifecycle | `direct` | live slot registry plus `spec/lifecycle_model.md` consume-once successor state | required; target-chain reorg/finality profile still open | `SPEC-LIFE-001`, `DIR-LIFE-001` |
| Canonical public wire objects | `direct` | `PrevalidatedSlotLeafV1`, `AccountCreationEvidenceV1`, `Path2NativeCheckedCoreFrameV1` | required; KAT corpus still required | `DIR-LEAF-001`, `DIR-EVID-001`, `DIR-CC-001`, `DIR-KAT-001` |
| Deterministic consensus verification | `direct` | canonical parse, fixed rejection behavior, no implementation-defined signer state | required before deployment claim | `DIR-REJECT-001`, `DIR-CC-001`, `DIR-KAT-001` |
| Hardware-wallet/HSM safety | `direct` | `spec/signer_profile.md`: integer/field-only signing profile, no floating point, bounded memory, constant-time evidence | required before deployment claim | `SPEC-SIGNER-001`, `DIR-CT-001`, `ADOPT-HW-001` |
| M-of-N multisig | `policy` | account policy over multiple admitted slots or slot classes | not native signature aggregation | `POL-MSIG-001`, `POL-REQ-001` |
| Threshold custody | `policy` or `mode_b` | either true threshold construction or proof-compressed M-of-N independent authorizations | open design decision | `POL-THRESH-001`, `AGG-COMMITTEE-001` |
| Validator quorum authorization | `policy` and `mode_b` | weighted signer set, bitmap/root, aggregate proof over admitted authorizations | not a BLS replacement claim | `POL-REQ-001`, `AGG-COMMITTEE-001` |
| Bridge committee authorization | `policy` and `mode_b` | committee state root plus aggregate consume/proof semantics | requires Mode B proof semantics | `AGG-COMMITTEE-001`, `AGG-DA-001`, `AGG-LC-001` |
| Account recovery | `policy` | guardian/timelock/rotation rules over account state and lifecycle anchors | must be explicit state policy | `POL-RECOVERY-001`, `ADOPT-HW-STATE-001` |
| Adaptor-style workflows | `policy` | state/protocol conditions around slot consumption | not native Schnorr-style adaptor algebra | `POL-ADAPTOR-001`, `POL-REQ-001` |
| Batch verification | `mode_b` | batch account creation or batch spend proof over canonical leaves | optional after direct theorem closes | `AGG-BATCH-001`, `AGG-SPEND-001`, `AGG-ORDER-001` |
| Signature/proof aggregation | `mode_b` | proof/state aggregation of many authorizations into one proof/root | aggregation is proof-layer, not native signature combine | `AGG-MODE-001`, `AGG-ROLLUP-001`, `AGG-SPEND-001` |
| Rollup compression | `mode_b` | recursive proof verifying creation/spend/state transitions | target path; concrete recursion proof still under construction | `AGG-ROLLUP-001`, `ADOPT-ETH-ROLLUP-001` |
| Light-client proofs | `mode_b` | compact state/proof handle for admitted and consumed slots | needs final aggregate proof format | `AGG-LC-001`, `AGG-DA-001` |

## Specification Baseline Surfaces

These rows define the specification artifacts that make the rest of the
requirements usable. A spec row being present does not mean the corresponding
implementation, proof, or KAT evidence is complete.

| Surface | Required class | Readiness | Required artifact | Tracker rows |
|---|---|---|---|---|
| Normative spec spine | `direct` | `partial` | indexed current spec spine, ADR routing, selected-support anchors, and non-claim boundaries | `SPEC-DIR-001`, `DIR-RISK-001`, `DIR-SEC-001` |
| Blockchain requirements spine | `direct` | `partial` | capability matrix, adoption gates, surface tables, and tracker ownership for chain-native authorization work | `SPEC-REQ-001`, `DIR-KAT-001`, `DIR-REVIEW-001` |
| Conformance and KAT corpus model | `direct` | `partial` | active Path 2 corpus identity, vector classes, result schema, reproducibility, and versioning rules | `SPEC-KAT-001`, `DIR-KAT-001`, `DIR-REJECT-001` |
| Consensus verifier model | `direct` | `partial` | public verifier pipeline, deterministic rejection classes, state transition discipline, and consensus-client mapping | `SPEC-VERIFY-001`, `DIR-REJECT-001`, `DIR-KAT-001` |
| Deployment profile model | `direct` | `partial` | target-chain, rollup, bridge, Ethereum, gas/resource, finality, and data-availability profile requirements | `SPEC-DEPLOY-001`, `DIR-LIFE-001`, `DIR-REJECT-001` |
| Ethereum adoption profile model | `direct` | `partial` | Ethereum account-abstraction, delegated-EOA, secondary-signature, frame-transaction, and rollup profile requirements | `SPEC-ETH-001`, `ADOPT-ETH-001`, `SPEC-DEPLOY-001` |
| Entropy and stream profile model | `direct` | `partial` | entropy-source classes, domain-separated stream registry, deterministic test-mode separation, and mask-PRF drift evidence | `ADOPT-HW-ENT-001`, `SPEC-SIGNER-001`, `DIR-KAT-001` |
| Constant-time and leakage evidence model | `direct` | `partial` | secret/public classification, forbidden secret-dependent behavior, harness requirements, fixed-shape failures, and target/build profiles | `SPEC-LEAKAGE-001`, `DIR-CT-001`, `ADOPT-HW-CT-001` |
| Signer public API model | `direct` | `partial` | public command/result classes, diagnostic-build separation, transactional signer state, and transport-profile requirements | `ADOPT-HW-API-001`, `SPEC-SIGNER-001`, `DIR-REJECT-001` |
| Signer state and fault model | `direct` | `partial` | local slot records, restore anchors, release atomicity, retry/finality handling, and state/fault evidence matrix | `SPEC-STATE-001`, `ADOPT-HW-STATE-001`, `DIR-LIFE-001` |
| Signer and custody profile | `direct` | `partial` | signer, keygen, sampler, wallet, and HSM requirements with concrete evidence surfaces | `SPEC-SIGNER-001`, `DIR-CT-001`, `ADOPT-HW-001` |
| Lifecycle and state model | `direct` | `partial` | consume-once lifecycle, reorg/finality requirements, local signer state, and required lifecycle evidence | `SPEC-LIFE-001`, `DIR-LIFE-001`, `ADOPT-HW-STATE-001` |
| Policy model | `policy` | `partial` | policy object boundary, multisig, threshold custody, recovery, adaptor-style, quorum, and committee semantics | `SPEC-POLICY-001`, `POL-REQ-001`, `DIR-LIFE-001` |
| External review package model | `direct` | `partial` | review lanes, manifest, reproducibility, residual-assumption, and lane-acceptance requirements | `SPEC-REVIEW-001`, `DIR-REVIEW-001`, `DIR-SEC-001` |

## Direct-Mode Adoption Gates

A chain may consider Mode A only when all of these are satisfied:

1. final direct-mode theorem statement is in `spec/security_model.md`;
2. public wire formats are fixed and versioned;
3. every public field has positive and negative KAT coverage;
4. direct creation and direct spend have end-to-end vectors;
5. signer/keygen/sampler behavior satisfies `spec/signer_profile.md`;
6. verifier rejection is deterministic, panic-free, and consensus-safe;
7. lifecycle/reorg/finality policy is specified under `spec/lifecycle_model.md`
   for the target chain;
8. security level is stated honestly, including the current 124-bit floor unless
   a hard-128 profile has been adopted by ADR;
9. third-party cryptanalysis has reviewed the coded-SIS/lattice core, QROM/hash
   framing, Poseidon2/FRI usage, and lifecycle model.

Direct-mode gate traceability:

| Gate | Readiness | Tracker rows |
|---|---|---|
| 1 | `partial` | `DIR-RISK-001`, `DIR-SEC-001`, `DIR-QROM-001`, `DIR-ACTION-001` |
| 2 | `partial` | `DIR-LEAF-001`, `DIR-EVID-001`, `DIR-CC-001` |
| 3 | `not_ready` | `DIR-KAT-001`, `DIR-CC-001` |
| 4 | `not_ready` | `DIR-KAT-001` |
| 5 | `not_ready` | `SPEC-SIGNER-001`, `DIR-CT-001`, `ADOPT-HW-001` |
| 6 | `partial` | `DIR-REJECT-001`, `DIR-CC-001` |
| 7 | `not_ready` | `SPEC-LIFE-001`, `DIR-LIFE-001` |
| 8 | `partial` | `DIR-QROM-001`, `ADOPT-128-001` |
| 9 | `not_ready` | `DIR-REVIEW-001` |

## Direct-Mode Implementation And Proof Surfaces

Direct mode is the first deployable authorization target. It is not ready until
the theorem, wire formats, creation proof, spend relation, lifecycle, signer,
rejection behavior, and review evidence all converge into one consensus-safe
artifact set.

| Surface | Required class | Readiness | Required artifact | Tracker rows |
|---|---|---|---|---|
| Normative direct-mode spine | `direct` | `partial` | spec spine plus `Path2DirectTheoremBoundaryReportV1`, `Path2DirectQromActionBoundaryReportV1`, `Path2DirectLifecycleBoundaryReportV1`, generated direct theorem, QROM/action, and lifecycle boundary manifests, final direct-mode theorem, and non-claims routed through current authority | `SPEC-DIR-001`, `DIR-RISK-001`, `DIR-SEC-001` |
| Risk-first deployment blocker ledger | `direct` | `partial` | ranked blocker order, no-claim rule, preflight/boundary consistency gate, and tracker-owned burn-down sequence for core signature deployability | `DIR-RISK-001`, `DIR-SEC-001`, `DIR-PROOF-002` |
| Slot leaf and creation evidence frames | `direct` | `partial` | canonical frame bytes, versioning, positive and negative KATs, and malformed-frame rejection | `DIR-LEAF-001`, `DIR-EVID-001`, `DIR-KAT-001`, `DIR-REJECT-001` |
| Account-creation STARK API | `direct` | `partial` | account-creation prover/verifier API, canonical payload digest, `Path2AccountStarkPayloadByteModelReportV1`, `Path2AccountStarkPayloadKatBoundaryReportV1`, draft byte-model/boundary manifest regeneration, digest-pinned boundary vectors, manifest/vector consistency checks, targeted integration tests, and public-input contract | `DIR-STARK-001`, `DIR-BIND-001`, `DIR-KAT-001`, `DIR-REJECT-001` |
| Spend authorization relation | `direct` | `partial` | spend relation connected to Rust evidence, checked-core frame, lifecycle state, and KAT corpus | `DIR-SPEND-001`, `DIR-CC-001`, `DIR-LIFE-001`, `DIR-KAT-001` |
| Slot authorization handle | `direct` | `not_ready` | normative wire/state text and KATs for the handle interface | `DIR-HANDLE-001`, `DIR-CC-001`, `DIR-KAT-001` |
| Account-creation theorem boundary | `direct` | `partial` | direct creation theorem boundary, `Path2AccountCreationProofBoundaryReportV1`, `Path2AccountCreationSamplerTimingReportV1`, `Path2AccountCreationNoExportBoundaryReportV1`, `Path2AccountStarkFriAokBoundaryReportV1`, generated proof-boundary, sampler/timing, no-export theorem-target/public-input-allowlist, and FRI/AoK digest-chain evidence, field-group manifest evidence, sampler, leakage, timing, no-export, extractor, and accepting verifier closure | `DIR-CREATE-001`, `DIR-SEC-001`, `DIR-PROOF-001`, `DIR-PROOF-002`, `DIR-STARK-001`, `DIR-BIND-001`, `DIR-CT-001` |
| Account-creation negative controls | `direct` | `not_ready` | executable negative KATs or explicitly retired evidence for surviving creation-time failures | `DIR-CREATE-NEG-001`, `DIR-REJECT-001`, `DIR-KAT-001` |
| Checked-core frame and proof support | `direct` | `partial` | checked-core frame binding, byte audit, spend-reduction bridge, obstruction-zero bridge, and KAT plan | `DIR-CC-001`, `DIR-CC-002`, `DIR-QROM-002`, `DIR-KAT-001` |
| Consume-once lifecycle transition | `direct` | `not_ready` | `Path2DirectLifecycleBoundaryReportV1`, local consume-once successor-state manifest evidence, target-chain reorg/finality profile, and lifecycle KATs for admit, spend, replay, burn, recovery, and restore | `DIR-LIFE-001`, `SPEC-LIFE-001`, `ADOPT-HW-STATE-001` |
| Account-creation proof family | `direct` | `not_ready` | selected account-STARK proof family, `Path2AccountStarkResidualMatrixReportV1`, `Path2AccountStarkFriAokBoundaryReportV1`, generated proof-boundary manifest, no-export boundary manifest, FRI/AoK boundary manifest, byte model, simulator theorem, extractor theorem, executable residual preflight blockers with boundary consistency checking, and rejection surface | `DIR-PROOF-001`, `DIR-PROOF-002`, `DIR-STARK-001`, `DIR-BIND-001`, `DIR-REJECT-001` |
| Creation payload and fiber privacy | `direct` | `not_ready` | concrete payload theorem, backend sufficiency proof, no-export privacy statement, `Path2AccountCreationNoExportBoundaryReportV1` denylist/public-payload/simulator-input-allowlist evidence, and byte budget | `DIR-PAYLOAD-001`, `DIR-PROOF-001`, `DIR-CT-001` |
| Masked-trace relation stack | `direct` | `not_ready` | trace relation, binding bridge, two-view consistency tests, and transparent extractor contract | `DIR-TRACE-001`, `DIR-PROOF-001`, `DIR-KAT-001` |
| Descent/SIS proof-route support | `direct` | `not_ready` | ADR or tracker decision on whether descent/SIS support remains research or becomes deployable proof authority | `DIR-DESCENT-001`, `DIR-PROOF-002` |
| Native-boundary model | `direct` | `not_ready` | normative object/projection model, byte-state model, consensus carrier, and projection encoder KATs | `DIR-NB-001`, `DIR-CC-002`, `DIR-KAT-001` |
| Native-boundary knowledge stack | `direct` | `not_ready` | concrete proof-family obligations for trace assembly, parent-trace extraction, hiding, and knowledge soundness | `DIR-NB-002`, `DIR-PROOF-001`, `DIR-PROOF-002` |
| Native-boundary row and range model | `direct` | `not_ready` | concrete role, range, blinding, row-cost, and checked-core compatibility rows replacing estimates | `DIR-NB-003`, `DIR-CT-001` |
| Native-boundary implementation cross-check | `direct` | `not_ready` | Rust/Lean assembly checks, canonical state semantics, projection audit, and no-export public-view tests | `DIR-NB-004`, `DIR-KAT-001`, `DIR-REJECT-001` |
| Authorization-class binding | `direct` | `not_ready` | theorem or refactor proving private authorization-local material cannot change public spend semantics | `DIR-AUTHCLASS-001`, `DIR-SEC-001` |
| Hash/QROM and action-map budget | `direct` | `partial` | `Path2DirectQromActionBoundaryReportV1`, generated QROM/action boundary manifest, current 182-bit checked-core transcript budget, 163-bit Fq12 action-map floor, final theorem-linked QROM ledger, spend Fiat-Shamir Poseidon2 KATs, and separated Fq12 action-map term | `DIR-QROM-001`, `DIR-QROM-002`, `DIR-ACTION-001`, `DIR-REVIEW-001` |
| Conformance and KAT corpus | `direct` | `partial` | active Path 2 corpus manifest remains split across draft byte-model and boundary manifests, `Path2AccountStarkPayloadByteModelReportV1`, `Path2AccountStarkPayloadKatBoundaryReportV1`, deterministic digest-pinned account-STARK boundary vectors, byte-model/manifest/vector consistency checking, positive and negative vectors, lifecycle vectors, rejection vectors, full byte corpus, and CI manifest verification | `SPEC-KAT-001`, `DIR-KAT-001`, `DIR-REJECT-001`, `DIR-CC-001` |
| Active test gates | `direct` | `partial` | fast active Rust gate plus runtime-budgeted slow test cadence and manual/CI policy | `DIR-TEST-001`, `DIR-SLOWTEST-001` |
| Consensus-safe rejection | `direct` | `partial` | deterministic public errors, panic-free parse/reject behavior, malformed-frame KATs, and client failure mapping | `DIR-REJECT-001`, `DIR-KAT-001`, `SPEC-VERIFY-001` |
| Constant-time signer evidence | `direct` | `not_ready` | keygen, sampler, witness handling, rejection-loop, zeroization, leakage-harness, fixed-shape failure, and fault evidence satisfying signer and leakage profiles | `DIR-CT-001`, `SPEC-SIGNER-001`, `ADOPT-HW-CT-001`, `SPEC-LEAKAGE-001` |
| External review package | `direct` | `partial` | external cryptanalysis package for coded-SIS core, Poseidon2/FRI, QROM framing, lifecycle, signer, and verifier engineering | `DIR-REVIEW-001`, `DIR-SEC-001`, `SPEC-REVIEW-001` |

## Mode B Adoption Gates

Mode B may not be advertised as solved until these are concrete:

1. the batch or recursive proof system is named and parameterized;
2. aggregate statement format is fixed;
3. canonical batch ordering and duplicate rejection are specified;
4. data availability rules exist for any slot records not directly stored in
   consensus state;
5. aggregate consume-once semantics are proven or explicitly assumed;
6. light-client proof semantics are specified if light clients consume the
   aggregate;
7. proof-system soundness, Fiat-Shamir/QROM assumptions, and recursion hash
   assumptions are named separately from the direct-mode theorem.

Mode B gate traceability:

| Gate | Readiness | Tracker rows |
|---|---|---|
| 1 | `partial` | `AGG-BATCH-001`, `AGG-ROLLUP-001` |
| 2 | `not_ready` | `AGG-SPEND-001`, `AGG-LC-001` |
| 3 | `not_ready` | `AGG-ORDER-001` |
| 4 | `not_ready` | `AGG-DA-001` |
| 5 | `not_ready` | `AGG-SPEND-001`, `DIR-LIFE-001` |
| 6 | `not_ready` | `AGG-LC-001` |
| 7 | `not_ready` | `AGG-ROLLUP-001`, `DIR-QROM-001`, `DIR-SEC-001` |

## Aggregation And Rollup Surfaces

Mode B is the answer to batch verification, rollup compression, light clients,
and bridge or validator committees. It is optional for direct Mode A, and it
must not be advertised as a native signature-aggregation property until each
surface has a concrete proof or state artifact.

| Surface | Required class | Readiness | Required artifact | Tracker rows |
|---|---|---|---|---|
| Mode B theorem boundary | `mode_b` | `partial` | theorem-mode switch and assumption boundary proving direct mode does not depend on aggregation | `AGG-MODE-001`, `DIR-SEC-001` |
| Batch account creation API | `mode_b` | `partial` | targeted tests and aggregate statement binding for existing batch creation API | `AGG-BATCH-001`, `AGG-ORDER-001` |
| Batch spend verification | `mode_b` | `not_ready` | batch spend statement, duplicate-slot rejection, and consume-once semantics | `AGG-SPEND-001`, `DIR-LIFE-001`, `AGG-ORDER-001` |
| Canonical ordering and duplicates | `mode_b` | `not_ready` | ordering keys plus duplicate slot/account/message rejection KATs | `AGG-ORDER-001`, `DIR-REJECT-001` |
| Data availability rules | `mode_b` | `not_ready` | specification of slot records stored in consensus state, calldata, DA blobs, or proof public inputs | `AGG-DA-001`, `AGG-LC-001` |
| Recursive account-proof verifier | `mode_b` | `partial` | concrete recursion proof, verifier-AIR tests, and aggregate public-input contract | `AGG-ROLLUP-001`, `DIR-QROM-001` |
| Recursion challenger gadget | `mode_b` | `partial` | challenger gadget KATs and byte-level binding to aggregate-statement formats | `AGG-ROLLUP-002`, `DIR-KAT-001` |
| Light-client aggregate proof | `mode_b` | `not_ready` | compact proof/state handle with data-availability and finality assumptions | `AGG-LC-001`, `AGG-DA-001` |
| Bridge or validator committee proof | `mode_b` | `not_ready` | signer set, weights, bitmap/root, quorum rule, and aggregate proof semantics | `AGG-COMMITTEE-001`, `POL-THRESH-001`, `POL-REQ-001` |

## Security-Level Adoption Surfaces

Security-level positioning is an adoption surface because chain, bridge,
custody, and standards reviewers need the claim stated without ambiguity. The
current direct-mode path may disclose the 124-bit practical floor, but a hard
128-bit claim requires a separate ADR, implementation profile, KAT refresh, and
review package.

| Surface | Required class | Readiness | Required artifact | Tracker rows |
|---|---|---|---|---|
| Hard-128 profile decision | `direct` | `not_ready` | decision on whether the 124-bit practical floor is acceptable for the target adopter or a hard-128 profile is required | `ADOPT-128-001`, `DIR-QROM-001`, `DIR-REVIEW-001` |

## Ethereum Adoption Surfaces

Ethereum-facing work is an adoption target, not a claim that Ethereum has
selected Descender or that any draft EIP will ship unchanged. Each surface must
produce a concrete artifact before it can be used as adoption evidence.

| Surface | Required class | Readiness | Required artifact | Tracker rows |
|---|---|---|---|---|
| Positioning baseline | `policy` | `partial` | account-abstraction positioning note plus normative Ethereum profile split | `ADOPT-ETH-001`, `SPEC-ETH-001` |
| ERC-4337 smart-account verifier | `direct` | `partial` | profile ABI/envelope exists; concrete EVM contracts, gas report, bundler simulation, and KATs remain open | `ADOPT-ETH-AA-001`, `SPEC-ETH-001`, `DIR-REJECT-001`, `DIR-KAT-001` |
| EIP-7702 delegated EOA migration | `policy` | `partial` | migration profile exists; delegation, rotation, recovery, rollback, and no-long-term-ECDSA-reuse tests remain open | `ADOPT-ETH-7702-001`, `SPEC-ETH-001`, `POL-RECOVERY-001`, `ADOPT-HW-STATE-001` |
| Secondary-signature/precompile profile | `direct` | `partial` | registry/precompile profile exists; algorithm id, canonical encodings, gas model, malleability analysis, and KATs remain open | `ADOPT-ETH-7932-001`, `SPEC-ETH-001`, `DIR-CC-001`, `DIR-KAT-001` |
| Native frame-transaction validation | `direct` | `partial` | frame-validation profile exists; payload mapping tests, state-access rules, gas-payment separation, failure KATs, and draft-stability ledger remain open | `ADOPT-ETH-8141-001`, `SPEC-ETH-001`, `DIR-REJECT-001`, `DIR-LIFE-001` |
| Rollup aggregate proof handle | `mode_b` | `partial` | rollup handle profile exists; aggregate bytes, ordered statement, data-availability rules, duplicate-rejection tests, light-client semantics, and PQ-safe recursion endpoint remain open | `ADOPT-ETH-ROLLUP-001`, `SPEC-ETH-001`, `AGG-ROLLUP-001`, `AGG-DA-001`, `AGG-LC-001` |

## Hardware And Custody Adoption Surfaces

Signer-side safety is a first-class adoption surface, not an implementation
detail. Descender should not repeat the deployment problem where a compact PQ
signature is attractive on verifier bandwidth but fragile in hostile wallet,
secure-element, HSM, or custody environments.

| Surface | Required class | Readiness | Required artifact | Tracker rows |
|---|---|---|---|---|
| Signer profile baseline | `direct` | `partial` | normative signer/custody profile plus split concrete entropy, leakage, and state artifacts | `ADOPT-HW-001`, `SPEC-SIGNER-001` |
| Wallet/HSM API and public errors | `direct` | `partial` | public command/result model exists; concrete transport encodings, API KATs, diagnostic-separation tests, and crash/retry evidence remain open | `ADOPT-HW-API-001`, `DIR-REJECT-001`, `SPEC-SIGNER-001` |
| Entropy and domain separation | `direct` | `partial` | stream model and mask-PRF drift evidence exist; frozen deployment registry, source profiles, KATs, and test/deployment separation evidence remain open | `ADOPT-HW-ENT-001`, `SPEC-SIGNER-001`, `DIR-KAT-001` |
| Secret-side leakage harness | `direct` | `not_ready` | fixed-shape leakage tests or reviewed equivalent for keygen, sampler, pad expansion, witness handling, and signing | `ADOPT-HW-CT-001`, `DIR-CT-001`, `SPEC-SIGNER-001`, `SPEC-LEAKAGE-001` |
| Rollback and fault state model | `direct` | `partial` | signer state/fault model exists; concrete persistent schema, crash/restart tests, restore-anchor negatives, retry/timeout tests, and finality/reorg evidence remain open | `ADOPT-HW-STATE-001`, `DIR-LIFE-001`, `SPEC-LIFE-001`, `SPEC-STATE-001` |

## Policy Feature Surfaces

Policy features are account/state-layer objects over admitted Descender
authorizations. They are not hidden semantics inside a signature byte string,
and they are not native Schnorr/BLS algebra unless a separate ADR and proof
surface says so.

| Surface | Required class | Readiness | Required artifact | Tracker rows |
|---|---|---|---|---|
| Policy object baseline | `policy` | `partial` | first concrete policy byte/state objects, domain separator, version, and account namespace | `POL-REQ-001`, `DIR-CC-001` |
| M-of-N account policy | `policy` | `not_ready` | signer set representation, per-slot eligibility rule, duplicate prevention, and KATs | `POL-MSIG-001`, `POL-REQ-001`, `DIR-LIFE-001` |
| Threshold custody route | `policy` | `not_ready` | decision between true threshold signing and proof-compressed M-of-N independent authorizations | `POL-THRESH-001`, `AGG-COMMITTEE-001` |
| Account recovery policy | `policy` | `not_ready` | guardian, timelock, rotation, burn, restore, and lifecycle-anchor semantics | `POL-RECOVERY-001`, `ADOPT-HW-STATE-001`, `DIR-LIFE-001` |
| Adaptor-style workflow policy | `policy` | `not_ready` | protocol/state conditions around slot consumption without claiming native Schnorr-style adaptor algebra | `POL-ADAPTOR-001`, `POL-REQ-001`, `DIR-LIFE-001` |

## Policy-Layer Requirements

Policy objects must be state-level objects, not hidden extra meaning inside a
signature byte string. A policy may decide which admitted slots are eligible, but
the checked-core spend still binds a normalized `slot_authorization_digest`.
The normative object model and evidence contract live in `spec/policy_model.md`.

Every policy design must specify:

- policy domain separator and version;
- account namespace;
- signer/guardian/committee set representation;
- threshold or weight rule, if any;
- timelock, recovery, or rotation semantics, if any;
- replay and duplicate-prevention rule;
- lifecycle interaction with consumed slots;
- state root fields bound into the spend or aggregate proof.

Policy requirement traceability:

| Requirement | Tracker rows |
|---|---|
| policy domain separator and version | `POL-REQ-001` |
| account namespace | `POL-REQ-001`, `DIR-CC-001` |
| signer/guardian/committee set representation | `POL-MSIG-001`, `POL-THRESH-001`, `AGG-COMMITTEE-001` |
| threshold or weight rule | `POL-THRESH-001`, `AGG-COMMITTEE-001` |
| timelock, recovery, or rotation semantics | `POL-RECOVERY-001`, `ADOPT-HW-STATE-001` |
| replay and duplicate-prevention rule | `DIR-LIFE-001`, `AGG-ORDER-001` |
| lifecycle interaction with consumed slots | `DIR-LIFE-001`, `POL-REQ-001` |
| state root fields bound into the spend or aggregate proof | `DIR-CC-001`, `POL-REQ-001`, `AGG-LC-001` |

## Non-Claims

Descender must not imply these are solved by the base signature primitive:

- native BLS-style aggregation;
- Schnorr-style adaptor signatures;
- threshold signing as one compact native Descender signature;
- validator finality or bridge safety without explicit account/committee state;
- hard 128-bit security while the active path is still using the 124-bit
  Poseidon2/Merkle floor;
- deployability based only on internal notes or prototype tests.

## Product Bar

The intended bar is higher than "a PQ signature verifies." The target is:

```text
post-quantum spend authorization
+ account-state lifecycle safety
+ policy expressiveness
+ optional proof aggregation
+ conservative signer/verifier engineering
+ explicit adoption evidence
```

If a feature is necessary for a serious chain adopter but not yet implemented,
it belongs in `TRACKER.md` with a status below `verified`, not in marketing
language.
