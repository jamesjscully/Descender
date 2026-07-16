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
the theorem target, conclusion label, accepted tuple, acceptance hypotheses,
per-hypothesis obligation matrix, direct obligation audit-vector case list,
named break classes, the direct closure-dependency row table, derived
closure-dependency evidence rows, and current open blocker for this theorem
boundary.
`formal/kat/path2_direct_theorem_boundary_v1.json` is regenerated from that
report, the current `Path2DirectQromActionBoundaryReportV1`, the current
`Path2DirectLifecycleBoundaryReportV1`, the current
`Path2DirectAdmissionBoundaryReportV1`, and the current
`Path2AccountCreationProofBoundaryReportV1` by a local and CI gate, so the
direct theorem target, conclusion, QROM/action digest, lifecycle digest,
admission-state digest, proof-boundary digest, and selected field groups cannot
drift silently. The
production report validator separately rejects weakened
acceptance-hypothesis/obligation-row coverage, missing pinned accepted-evidence
hooks, missing direct closure-dependency rows, unexpectedly closed theorem
flags, missing per-row closure-dependency evidence, deployment-ready flags, or
direct theorem blocker drift. The boundary and preflight consistency gates also
compare each direct closure-dependency evidence row field-for-field against its
source closure-dependency row. The
`formal/kat/path2_direct_theorem_obligation_vectors_v1.json` corpus records one
digest-pinned audit case for each direct-theorem obligation and binds that
row list to the current QROM/action, lifecycle, admission-state, and account-creation
proof-boundary reports while also carrying the direct closure-dependency rows
and derived evidence rows that tie the nine accepted-tuple hypotheses to their
open or theorem-scoped imported source reports/support surfaces. It keeps composition,
imported-boundary verification, and deployment flags false.
The Lean concrete full-spend carrier mirrors this: `AccountCreationAccepted`
now requires `AccountCreationProofBoundaryEvidenceAccepted`, so an accepted
creation must carry the selected proof-boundary report digest, accepted
proof-boundary evidence digest, replay-payload digest, deployed-anchor digest,
sampler/timing report digest, no-postchallenge timing obligation digest,
no-export boundary digest, account-STARK residual matrix digest, FRI/QROM
residual digest, FRI/AoK boundary digest, payload-KAT boundary digest,
validated proof-boundary evidence flags, verifier acceptance, and fail-closed
production-gate fact before the direct theorem may treat the slot as admitted.
The same carrier requires public evidence that the creation statement excludes
the spend message and signature bytes, is fixed before the spend challenge, and
that the accepted creation and frozen leaf precede the challenge; those are
boundary facts, not the no-postchallenge timing theorem itself.
It also requires explicit tracked-open proof-family residual flags for the
simulator/no-export, FRI/QROM, FRI/AoK, and payload-KAT obligations, so the
final theorem carrier cannot consume a generic proof-family acceptance bit.
The same Lean carrier now mirrors the two-tier break taxonomy: detailed
account-creation proof-boundary residual rows such as simulator/no-export,
FRI/AoK, FRI/QROM, frozen payload KATs, and production gate/review must map to
the direct theorem's `selected_account_creation_payload_proof_family_break`
umbrella class. This closes only the taxonomy bridge; the underlying residual
theorems and reviews remain open.
It also bridges every finite `Path2FullSpendGame.BreakEvent` into the
report-facing `DirectNamedBreakClass` taxonomy and proves those finite events
round-trip, while proof-family, consensus-rejection/state-mutation, and
external core-assumption classes remain report-only.
The bridge is fail-closed: accepted proof-boundary evidence exposes the
residual package and taxonomy coverage, and missing residual coverage exports
`creation_time_hidden_source_soundness_break` rather than admitting the slot.
The concrete carrier also names the positive
`AccountCreationProofFamilyClosureWitness` target and proves that the current
accepted-boundary route cannot provide it while
`proofFamilyObligationsStillOpen` is part of the selected residual package.
The production-reference source-shape gate checks this account-creation
proof-boundary route alongside the full-signature execution source, so the
direct theorem cannot silently collapse back to a generic `acceptCreate`
predicate.
`Path2DirectAcceptedSpendTupleEvidenceV1` records
an accepted-path tuple digest for the current direct spend endpoint: public-key
identity, message digest, account id, state root, slot id, checked-core
signature/challenge/proof-commitment digests, leaf digest, evidence transcript
digest, selected creation-payload digest, exact admission namespace digest,
slot authorization digest, consume-once before/after state digests, imported
QROM/action, lifecycle, and proof-boundary report digests, and the public
`accept_spend_slot` result. It also carries an eleven-row
accepted-tuple-field evidence checklist tying every direct theorem tuple field
to its accepted-path evidence source, accepted-path check, imported boundary
report when applicable, and open or theorem-scoped imported direct obligation. The
`verify_path2_direct_single_slot_spend_authorization_with_tuple_evidence_from_sovereign_state_v1`
hook loads and rechecks the canonical admission record from
`Path2SovereignDirectAdmissionStateV1`, runs the real direct verifier, snapshots
the consume-once transition, and checks the public result wrapper independently
accepts the same slot from the same before-state. The
`Path2DirectAcceptedCompositionEvidenceV1` feature-gated evidence path now
uses the selected account-STARK creation fixture and the checked-core direct
spend verifier together, binding the accepted creation proof-boundary evidence,
accepted direct spend tuple, direct theorem boundary digest, QROM/action
digest, lifecycle digest, proof-boundary digest, exact admission namespace, and
consume-once before/after transition for one accepted slot.
`formal/kat/path2_direct_accepted_spend_tuple_evidence_v1.json` now
digest-pins that accepted direct spend tuple as a standalone deterministic KAT
artifact, including checked-core signature/challenge/proof-commitment digests,
the exact admission namespace, consume-once before/after state digests, public
`accept_spend_slot`, and replay rejection against the consume-once set.
The direct accepted-spend tuple KAT validator now replays the canonical
deterministic tuple wrapper before accepting serialized KAT evidence, so a
digest-updated but noncanonical tuple field cannot satisfy the KAT boundary.
`formal/kat/path2_direct_accepted_composition_evidence_v1.json` digest-pins
that accepted composition evidence under deterministic KAT proof-byte mode:
the generator and checker run inside a one-thread Rayon pool so the fixed
account-STARK proof seed also fixes Plonky3 proof-of-work witness selection.
The artifact records the full accepted evidence digest chain, including the
selected account-creation payload digest, proof-boundary evidence digest,
direct spend tuple digest, leaf/evidence transcript digests, admission
namespace, authorization digest, and consume-once before/after state digests.
The accepted-composition validator replays the canonical deterministic
composition artifact before accepting serialized evidence, so a digest-updated
but noncanonical composition field cannot satisfy the direct composition KAT
boundary.
It also records a nine-row accepted-hypothesis evidence checklist: the eight
positive accepted-path hypotheses must have concrete accepted-path evidence,
while the public rejection/state-preservation hypothesis is explicitly marked
as deferred to the direct rejection KAT suite. The compact accepted-composition
artifact imported by the direct theorem and preflight manifests carries those
accepted-hypothesis counts, exact hypothesis identities,
hypothesis-to-direct-obligation identities, and deferred-rejection flags, and
the consistency gates fail if the compact summary no longer matches the full
accepted evidence shape.
The consistency gate also checks the direct spend tuple digest against the
standalone tuple artifact and the selected payload digest against the
account-STARK payload KAT vector corpus. The accepted proof-boundary, accepted
spend tuple, and accepted-composition consistency gates also call
production-reference validators that enforce digest/report binding, required
accepted-path flags, namespace binding, replay/consume-once evidence, and
fail-closed theorem/deployment flags. The accepted-composition validator
recomputes the direct admission namespace digest from the recorded account id,
state root, and slot authorization digest, and rejects identical consume-once
before/after state digests. This is deterministic KAT evidence only;
deployment producers still use fresh proof randomness, and the artifacts do not
close the composition theorem or account-creation proof family. The
`path2_direct_single_slot_spend_tuple_evidence_pins_accepted_tuple` test is
accepted-path evidence for the theorem shape, but it is not the composition
theorem and does not verify the account-creation proof family. The
feature-gated `path2_direct_accepted_composition_evidence_binds_creation_and_spend`
test gates the current accepted creation-to-spend composition evidence while
keeping composition, proof-boundary verification, and deployment flags false.
The Lean carrier now also proves
`selectedRustDirectModeAuthorizationTheoremReduction_v1`, which composes the
selected-Rust bound slot, prechallenge admission, checked-core binding, and
consume-once transition into one product conclusion while naming account-STARK
proof-family review, QROM/action review, deterministic public rejection,
signer/keygen review, and external coded-SIS/lattice-core review as explicit
hypotheses. `Path2DirectTheoremBoundaryReportV1` pins that reduction and its
explicit-hypothesis shape, but keeps `composition_theorem_instantiated = false`
until those hypotheses are supplied by accepted proof/review evidence.
The status-neutral capstone
`selectedRustDirectModeAuthorizationClosedUnderExplicitAssumptions_v1` now
specializes that reduction using the five accepted-boundary proof-family
residual reductions, an explicit post-quantum target, the selected sovereign F2
state invariant, reviewed QROM/action, signer/keygen, and external core
assumptions. The report records
`composition_capstone_under_explicit_named_assumptions_imported = true`; this is
not composition instantiation, proof-family verification, external review, or
deployment readiness.
The
public-rejection obligation row now records
`Path2PublicResultClassV1`, `Path2DirectSpendPublicResultV1`, the direct spend
public-classifier helper, and the
`path2_direct_single_slot_spend_rejections_preserve_consume_once_state` test,
which checks selected direct spend rejections preserve consume-once state before
the mutation point. It also records
`path2_direct_single_slot_spend_malformed_signature_kats_are_panic_free`, which
checks selected malformed Fq12 signature byte strings reject without panic,
map to the malformed public class, and preserve consume-once state.
`path2_direct_single_slot_spend_public_object_rejection_kats_are_panic_free`
adds selected direct leaf/evidence/certificate malformed, lifecycle, and binding
failures with panic-free rejection, public class mapping, and no consume-once
mutation. `path2_direct_single_slot_spend_public_result_wrapper_hides_internal_diagnostics`
checks accept, malformed, checked-core, and lifecycle outcomes through the public
wrapper without internal diagnostic exposure and with rejection-state rollback.
The Lean model now proves
`selectedRustDirectPublicRejectionStatePreserving_v1`: for the selected wrapper
control shape, every explicit verifier error and caught panic returns the exact
pre-call state. `Path2DirectTheoremBoundaryReportV1` source-pins that theorem to
the Rust wrapper's two restore branches, exhaustive eleven-class public result
table, classifier fallback, and fail-closed panic class. It records the local
state-preservation and classifier-totality imports while keeping the full
consensus-KAT hypothesis explicit.
`path2_direct_public_rejection_classes_are_exhaustive_and_state_preserving`
now exercises every one of the nine rejection classes through the centralized
rollback helper, mutates state before each injected rejection, checks exact
rollback, checks panic-to-internal-error rollback, and keeps a success mutation
as the negative control.
`PATH2_DIRECT_PUBLIC_OBJECT_MUTATION_KAT_CASES_V1` names all nine typed public
request/state objects consumed by the direct spend wrapper, and
`path2_direct_all_consensus_visible_public_object_mutations_are_panic_free`
executes one semantic mutation per object through the real wrapper while
checking the selected client result and exact consume-once rollback.
`PATH2_SOVEREIGN_DIRECT_CLIENT_MAPPING_ROWS_V1` pins all eleven result classes
to unique consensus codes and the ADR-0009/ADR-0010 sovereign-rollup state
effects; `path2_sovereign_direct_client_mapping_is_exhaustive_and_state_preserving`
checks every rejection maps to an invalid transaction with no state mutation.
Lean proves `selectedRustDirectDeterministicPublicRejectionClosed_v1` and
`selectedRustDirectModeAuthorizationWithClosedPublicRejectionReduction_v1`, so
the theorem-scoped public-rejection obligation is closed and the boundary flags
for all typed public objects and target-client mapping are true.
The account-creation frame corpus is also frozen for the current leaf/evidence
boundary: one honest frame plus seventeen one-surface mutation/rejection cases
cover the current frame rows, all fifteen duplicated leaf/evidence binding
fields, public rejection classes, and unit-test coverage. The generated
boundary and vector artifacts set `frozen_corpus_complete = true` while keeping
`deployment_ready = false`. Lean packages the accepted leaf and evidence facts
as `SelectedRustDirectLeafAndEvidenceClosure` and proves
`selectedRustDirectLeafAndEvidenceClosure_of_concreteSpendAccepts`; the direct
boundary additionally requires the verified ADR-0010 D2 lifecycle profile.
Consequently `direct_obligation_leaf_canonical_live` and
`direct_obligation_evidence_leaf_bindings` are theorem-scoped closed rows. The
nine-row direct audit is now two open and seven closed. The selected ADR-0009
sovereign profile stores the full canonical leaf record in consensus state under
the exact `(account_id, account_state_root_before,
slot_authorization_digest)` key, rechecks leaf and evidence digests on spend,
rejects missing, corrupt, and duplicate records, and applies ADR-0010 D2
finalized-head reorg semantics. `Path2DirectAdmissionBoundaryReportV1`,
`formal/kat/path2_direct_admission_boundary_v1.json`, and
`selectedRustDirectAdmissionNamespaceAndDataAvailabilityClosure_of_concreteSpendAccepts`
therefore close the theorem-scoped admission namespace/data-availability row.
The remaining open rows are creation-before-challenge/proof-family consumption
and checked-core QROM/action acceptance.
This does not claim the broader consensus verifier is verified: byte-string
parser corpora, positive byte-for-byte admission/spend vectors, target-specific
database integration and external storage review, resource/gas profiles, and
adoption-specific client integration remain open. The
QROM/action report pins the checked-core direct query budget, 182-bit priced
transcript term,
246-bit full challenge term, 163-bit Fq12 action floor, direct-mode `q_chal`
and inactive `q_batch` status, and generated QROM artifacts. The concrete Lean
carrier independently states deterministic checked-core validity, imports the
production direct verifier's canonical decode, commitment, challenge, equation,
public-fiber, pad-opening, and slot checks, proves accepted implies valid or a
concrete equation/frame/domain-QROM/parser break, and couples the domain/QROM
branch to the direct trace event. The report records those local links as
present while keeping the external QROM/action assumption, reviewed event
bound, theorem budget import, external review, theorem-linked QROM/action
closure, and deployment readiness false. The lifecycle report pins the local
consume-once successor-state model, replay rejection, spent-slot ledger,
reorg-burn support, restore-anchor support, recovery-burn support,
snapshot-recovery support, file-store recovery support, and
lifecycle/spent-record persistence evidence. The lifecycle transition vector
corpus now digest-pins local admit/spend/consume, replay, persistence,
reorg-burn, recovery-burn, restore-anchor, file-store recovery, and ADR-0010 D2
finality-window cases. The lifecycle profile review and verification artifacts
now digest-pin the ADR-0010 D2 safe-default target-chain lifecycle review and
production-reference lifecycle-slice verification against finality,
pending-release, dropped-before-finality, finalized-reorg, recovery-burn,
restore-anchor, and persist-before-release requirements, while keeping
deployment readiness false. A
separate boundary manifest consistency gate checks that
the direct theorem manifest embeds the same proof-boundary digest and
field-group object as the standalone proof-boundary manifest. This report and
manifest are not the proof; they are mechanical guards that prevent deployment
preflight from drifting away from the theorem shape while the composition
theorem and broader deployment-profile readiness remain open. The
`path2_preflight_boundary_consistency` gate separately checks that production
preflight stays fail-closed on the direct theorem blocker, account-creation
proof-boundary blocker, and selected account-STARK residual blockers. It also
requires each false direct/proof/residual closure flag to be backed by the
expected still-open dependency, obligation, or residual row, so preflight cannot
drift into a deployable posture while the closure graph is still open.

Accepted-spend numeric evidence is also fail-closed at the report boundary.
`SelectedRustAcceptedSpendNumericDigestSlotsCertificate` supplies the structural
digest-slot certificate consumed by
`selectedRustAcceptedSpendEvidenceReportVerifiedProof`: the selected report
must carry the aggregate numeric-evidence verification bit and nonzero
finite-trace, outcome-carrier, accepted-event, break-family, named-break,
union-pricing, and priced-bound digest slots before any future pricing witness
can route through the accepted-spend source certificate. This hardens the
structural handoff only; it does not supply the concrete trace distribution,
per-event cryptographic bounds, or priced accepted-spend theorem required for
deployment. `SelectedRustAcceptedSpendPricingObligationGap` and
`SelectedRustFullSignatureReductionProductionObligationGap` mirror the current
fail-closed diagnostics in Lean and prove that those missing hard halves block
the selected pricing and production-completeness facts.

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
- zero-knowledge/no-export semantics for pad seed, account projection,
  opening-tuple preimage, authorization witness, mask, commitment nonce, and
  private trace material; the decoded public statement's checked-core syndrome
  is an explicitly allowed public authorization anchor;
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
which pins the proof-boundary target, conclusion label, selected payload type,
verifier endpoints, replay-payload digest function, accepted-boundary evidence
schema and endpoint, acceptance requirements, exact leaf/evidence binding
fields, certificate/evidence binding fields, binding/leaf context fields,
payload-binding fields, deployed-anchor fields, no-export private-material field
classes, the `Path2AccountCreationSamplerTimingReportV1` digest, the
`Path2AuthorizationClassBindingAuditReportV1` digest, the
`Path2AccountCreationNoExportBoundaryReportV1` digest, and current proof-family
closure blockers. It also pins accepted-path binding evidence flags for the
replay-payload digest chain and deployed-anchor public statement comparison
without treating those flags as a theorem closure. It also pins a structured
proof-obligation row list for honest sampler soundness, authorization-class binding,
no-postchallenge timing, simulator/no-export closure, FRI/AoK extraction,
FRI/QROM residual pricing, frozen payload KATs, and production-gate review. Four
rows remain open, while honest sampler soundness, authorization-class binding,
no-postchallenge timing, and frozen payload KATs are closed by imported evidence.
Each open row carries a named break and the
fail-closed account-creation proof-boundary blocker. A separate
closure-dependency row table maps those eight proof obligations to their source
report flag groups and, where
applicable, the selected account-STARK residual matrix row. Derived
closure-dependency evidence rows pin each dependency to its open obligation or
imported theorem-scoped closure, source support surface, named break, and
blocker; the production
validator rejects the proof-boundary report if a false sampler, no-export,
FRI/AoK, FRI/QROM, payload-KAT, or production-gate flag is not backed by the
expected open obligation and support surface. The boundary and preflight gates
also compare each proof-boundary closure-dependency evidence row field-for-field
against its source closure-dependency row.
`Descender/Path2FullSpendConcrete.lean` mirrors that routing at the accepted
full-spend carrier by requiring accepted account-creation proof-boundary
evidence to carry a nonzero authorization-class obligation digest, a tracked
obligation flag, and the public leaf authorization class equal to
`pSpend(boundary)`. It also requires the nonzero sampler/timing report digest,
the nonzero no-postchallenge timing obligation digest, prechallenge-fixed
statement evidence, spend-message/signature exclusion evidence, and accepted
creation/frozen-leaf timing inequalities. The carrier also pins the no-export,
residual-matrix, FRI/QROM, FRI/AoK, and payload-KAT report digests plus
tracked-open residual flags, and it proves that every detailed proof-family
residual row collapses only to the direct theorem's selected account-creation
proof-family break class. These are deployment-boundary and taxonomy evidence,
not theorem closures. Missing residual or taxonomy coverage is routed to the
creation-time hidden-source break in the concrete carrier.
The carrier separates digest-chain pinning from open-obligation tracking and
defines a proof-family closure witness that is impossible for the current
selected accepted boundary; deployable closure must replace the open residual
flag with real simulator/no-export, FRI/QROM, FRI/AoK, frozen-KAT, and
production-review evidence.
`Path2AuthorizationClassBindingAuditReportV1` pins the current
authorization-class field-completeness audit: every revised `p_spend` field is
named, every row maps to the deployed projection/check surface, the direct spend
verifier is checked for absence of private authorization-local material reads,
and the source-shape hook list covers the Rust hooks, Lean boundary hooks, and
selected-Rust typed compatibility/projection bridge hooks plus the local
authorization-class closure hook. The concrete Lean carrier now proves the
selected-Rust typed no-ambiguity consequence in
`selectedRustAuthorizationCompatibleOverClass_noAmbiguity`, imports it into the
accepted-boundary support hook
`selectedRustAcceptedBoundaryAuthorizationClassBindingSupport_of_boundaryAccepted`,
and closes the local authorization-class theorem with
`selectedRustAuthorizationClassBindingTheoremClosed_of_boundaryAccepted`.
The
sampler/timing report imports that audit digest, and
`formal/kat/path2_authorization_class_binding_audit_v1.json` is regenerated and
checked before sampler/timing and proof-boundary manifests. This is executable
audit evidence for the authorization-class binding row; it keeps full
honest-create sampler integration, external review, and deployment readiness
false.
The sampler/timing report also pins a five-row honest-create sampler integration
checklist over the private-to-public construction order, selected-Rust bridge
hooks, authorization-class audit import, accepted proof-boundary import, and
closed no-postchallenge timing slice. The private-to-public and
authorization-bridge rows now point at
`selectedRustHonestCreateSamplerSoundTheoremClosed`, the Lean theorem that
instantiates the selected-Rust deterministic `honest_create_sampler_sound_v1`
target: every `honestCreate` witness gives zero obstruction, local validity,
and `authClass = p_spend(q)` through the selected bridge hooks. The
authorization-class
row now points at
`selectedRustAuthorizationClassBindingTheoremClosed_of_boundaryAccepted`,
which proves the accepted leaf authorization class, every same-boundary
compatible private authorization local, the semantic spend class, and the
selected checked-core spend digest all agree with `p_spend(q)` while the audit
digest remains imported by the report. The accepted-boundary
row now points at
`selectedRustFullHonestSamplerIntegrationComplete_of_boundaryAccepted`,
the Lean support hook that consumes the private-to-public order row,
authorization-bridge checklist row, authorization-class support row,
accepted-boundary sampler row, deterministic honest sampler theorem, and closed
timing row from accepted-boundary evidence. Its accepted-boundary sampler
component derives the `honestCreate` witness from accepted-boundary facts and
binds `q,h` to the accepted spend's own creation boundary and concrete leaf
authorization fiber. The checklist records zero open sampler-integration rows,
keeps deployment readiness false, and imports the deterministic honest-sampler
theorem flag as true.
`formal/kat/path2_account_creation_proof_obligation_vectors_v1.json`
turns that row table into a digest-pinned audit corpus bound to both the
proof-boundary and direct-theorem reports; its honest-sampler obligation row now
names the same
`selectedRustFullHonestSamplerIntegrationComplete_of_boundaryAccepted`
support package plus the deterministic sampler theorem while closed, records the
current 4-open / 4-closed proof-boundary accounting, and keeps deployment
fail-closed. Its
authorization-class obligation row is now closed by
`selectedRustAuthorizationClassBindingTheoremClosed_of_boundaryAccepted` plus the
audit report's `p_spend` completeness and no-private-material checks. A separate
requirement-coverage matrix maps every
acceptance requirement to its current evidence surface, vector evidence kind,
current vector case ids where they exist, public rejection classes for rejection
vectors, closure target, and blocker; it records that all ten requirements have
current frame-KAT, account-STARK payload, no-export audit, or FRI/QROM domain
audit vectors. The FRI/QROM domain audit corpus pins the selected proof-family
domain, Poseidon2 width and Merkle/MMCS sponge floor, FRI challenge schedule, checked-core
QROM budget import, and fail-closed open theorem flags; it is not an
account-STARK transcript QROM theorem or residual-pricing closure. The
sampler/timing report pins the honest-create
sampler support file, no-postchallenge timing game, construction order,
selected-Rust honest-create authorization bridge hooks, prechallenge
fixed-through step, spend-challenge derivation step, and executable hooks showing
that the creation hidden-source statement excludes the spend message/signature
while the checked-core challenge binds message, state root, slot authorization
digest, and proof commitment. Those bridge hooks are support evidence, not the
full honest-sampler theorem closure. The no-export boundary report
pins the support file, selected account-STARK payload endpoints, public
payload byte-model digest, payload KAT-boundary digest, forbidden
private-material field denylist, simulator theorem target,
no-export theorem target, accepted-bundle simulator public-input allowlist,
digest-pinned no-export audit vector case ids, selected Rust public-bundle
denylist predicate theorem
`selectedRustAccountCreationPublicBundleNoExport_instantiation_v1`, selected
Rust simulator-input allowlist theorem
`selectedRustAccountCreationSimulatorInputNoExport_instantiation_v1`, selected
Rust accepted public-anchor package theorem
`selectedRustAcceptedPublicAnchorPackageNoExport_instantiation_v1`, selected
Rust accepted-bundle public-anchor simulation-boundary theorem
`selectedRustAcceptedBundlePublicAnchorSimulationBoundary_v1`, selected Rust
accepted-bundle public-bundle projection theorem
`selectedRustAcceptedBundlePublicBundleFromPublicAnchors_v1`, selected Rust
accepted-bundle simulator-input projection theorem
`selectedRustAcceptedBundleSimulatorInputFromPublicAnchors_v1`, selected Rust
accepted-bundle simulator/no-export reduction theorem
`selectedRustAcceptedBundleSimulatorNoExportReduction_v1`, selected Rust
accepted-boundary simulator/no-export reduction theorem
`selectedRustAcceptedBoundarySimulatorNoExportReduction_v1`, and named
`account_stark_simulator_or_no_export_break`, while keeping simulator theorem,
no-export theorem, accepted-bundle simulability, external review, and deployment
readiness false.
The simulator/no-export proof-obligation row now names
`selectedRustAcceptedBoundaryNoExportSupport_of_boundaryAccepted`, which packages
the accepted no-export boundary digest, nonzero evidence, simulator/no-export
tracking, direct proof-family break mapping, and the still-open proof-family
flag; `selectedRustAcceptedBoundarySimulatorNoExportReduction_v1` reduces that
accepted-boundary support package to a supplied simulator/no-export theorem
without instantiating it. The no-export report/vector corpus pins the selected Rust public-bundle
denylist predicate, selected Rust simulator-input allowlist predicate, and
selected Rust accepted public-anchor package binding predicate plus the selected
Rust accepted-bundle public-anchor simulation-boundary predicate
and accepted-bundle public-bundle projection, simulator-input projection, and
simulator/no-export reduction predicates plus the full-spend accepted-boundary
simulator/no-export reduction predicate
without claiming simulator/no-export closure.
The FRI/AoK extractor obligation row similarly names
`selectedRustAcceptedBoundaryFriAokSupport_of_boundaryAccepted`, which packages
the accepted FRI/AoK boundary digest, nonzero evidence, extractor-obligation
tracking, direct proof-family break mapping, and still-open proof-family flag
without claiming the selected Plonky3/q12289 extractor theorem.
The FRI/QROM residual-pricing row now names
`selectedRustAcceptedBoundaryFriQromSupport_of_boundaryAccepted`, which packages
the accepted FRI/QROM residual digest, nonzero evidence, residual-pricing
tracking, direct proof-family break mapping, and still-open proof-family flag
without claiming the account-STARK transcript QROM theorem or pricing closure.
The production-gate-and-review row now names
`selectedRustAcceptedBoundaryProductionGateSupport_of_boundaryAccepted`, which
packages the selected proof-boundary and accepted-evidence digests, validation
facts, verifier acceptance, production-gate fail-closed tracking, direct
proof-family break mapping, and still-open proof-family flag without claiming
external review, maintainer approval, or deployment readiness.
`selectedRustAcceptedBoundaryProductionGateReviewReduction_v1` then reduces
that accepted-boundary support package to a supplied external-review/maintainer
gate theorem while keeping that theorem external and the production-gate,
external-review, and deployment-ready flags false.
The accepted-boundary proof-family aggregate hook
`selectedRustAcceptedBoundaryProofFamilyClosureReductions_v1` now packages the
simulator/no-export, FRI/AoK, FRI/QROM, frozen payload-KAT, and production-gate
reduction rows into a single residual bundle. The follow-on theorem
`selectedRustAcceptedBoundaryProofFamilyClosureWitnessReduction_v1` only proves
nonempty closure-witness availability when those five residual theorem
instances are supplied together with an explicit post-quantum target, global
pad-seed uniqueness invariant, and cleared `proofFamilyObligationsStillOpen`
flag, so the current boundary remains fail-closed.
`selectedRustPadSeedGloballyReorgSafeUniqueReduction_v1` now names the F2
pad-seed seam reduction from a global admission registry plus the ADR-0010 D2
finalized-head lifecycle facts into that pad-seed uniqueness shape; it is
lifecycle/proof-boundary accounting, not deployment readiness.
For the selected sovereign state, the stronger adapter
`selectedRustSovereignPadSeedReorgSafeUniqueness_of_verifiedNullifierRegistry_v1`
reduces deterministic account-STARK nullifier output, the raw canonical global
nullifier index, atomic candidate-block admission, and D2 reorg behavior to F2
under an explicit `SelectedRustSovereignVerifiedNullifierRegistryContract`
contract. The report records this selected-state reduction as imported, not as
a cross-language proof that Rust satisfies the Lean contract. Trusted restore
anchor placement, target database adoption, and external review remain open.
`formal/kat/path2_account_creation_proof_boundary_v1.json` is regenerated from
that report, the selected `Path2AccountStarkResidualMatrixReportV1`, the current
`Path2AccountStarkPayloadKatBoundaryReportV1`, the current sampler/timing
report, and the current no-export boundary report by local and CI gates, so the
proof-boundary target, conclusion, digest chain, selected field groups,
sampler/timing hooks, no-export denylist, requirement-coverage rows,
proof-obligation rows, closure-dependency rows, closure-dependency evidence
rows, residual rows, and payload-KAT boundary blocker cannot drift silently.
`Path2AccountCreationAcceptedProofBoundaryEvidenceV1` is the accepted-path
creation analogue of the direct spend tuple evidence. Under the real
account-STARK verifier feature it constructs the deterministic current KAT
payload, runs `verify_path2_direct_creation_account_stark_payload_boundary`,
records only public digests and anchors for the leaf, evidence, replay
certificate, creation binding, payload frame, STARK bundle, deployed anchor
set, decoded public account-STARK anchor set, imported boundary reports, and
accepted verifier result, and confirms the production entrypoint still fails
closed. The decoded-anchor comparison separately pins account id, slot id,
state root, lifecycle anchor, raw verifier version, masked projection,
role-root export bridge, and packed-witness commitment against the deployed
leaf/binding anchors. The accepted-evidence validator also replays the selected
account-STARK KAT fixture and recomputes the leaf, evidence, certificate,
payload, creation-binding, payload-frame, bundle, deployed-anchor, and
decoded-anchor digests before accepting a serialized artifact. The artifact
is also consumed by the Lean concrete full-spend carrier through
`AccountCreationProofBoundaryEvidenceAccepted`, which forces the final theorem
shape to name the proof-boundary report/evidence digests and fail-closed
production-gate fact, plus the sampler/timing and no-postchallenge timing
obligation evidence and the selected proof-family residual report digests,
instead of relying only on generic creation acceptance; the detailed
proof-family residual rows are also bridged to the direct theorem's selected
account-creation proof-family break class.
The accepted-boundary route now also proves the current positive proof-family
closure witness impossible until the open residual package is replaced by
closed proof-family evidence.
The artifact
also carries a ten-row
accepted-requirement evidence checklist: eight requirements are
happy-path-applicable accepted checks, `account_stark_qrom_domain_separation`
is deferred to the residual theorem ledger, `deterministic_public_rejection`
is deferred to rejection KATs, and every separate closure requirement remains
tied to an open proof-boundary obligation. This evidence pins the accepted
creation boundary shape;
`formal/kat/path2_account_creation_accepted_proof_boundary_evidence_v1.json`
now digest-pins that evidence as a deterministic one-thread KAT artifact and
checks it back to the account-STARK payload KAT corpus. It is not the simulator
theorem, extractor theorem, account-STARK transcript QROM theorem,
production-gate decision, or external review.
`formal/kat/path2_account_creation_no_export_boundary_v1.json` is
also regenerated and checked before the proof-boundary manifest so the
no-export boundary cannot remain prose-only, and
`formal/kat/path2_account_creation_no_export_vectors_v1.json` pins the current
public-payload allowlist, simulator-input allowlist, private-material denylist
absence, opaque proof-bundle manifest policy, selected Rust public-bundle Lean
predicate evidence, selected Rust accepted public-anchor package evidence,
selected Rust accepted-bundle public-bundle projection evidence, selected Rust
accepted-bundle simulator-input projection evidence, selected Rust
accepted-bundle simulator/no-export reduction evidence, selected Rust
accepted-boundary simulator/no-export reduction evidence, and frozen no-export
vector closure flag.
`formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json` pins the
current FRI/QROM domain audit cases before the proof-boundary manifest is
checked. `formal/kat/path2_account_creation_proof_obligation_vectors_v1.json`
pins the open proof-obligation cases and checks the standalone accepted
proof-boundary evidence artifact plus the no-export, FRI/QROM domain, and
payload KAT vector corpora before the direct theorem manifest is checked, and
`formal/kat/path2_direct_theorem_obligation_vectors_v1.json` pins the direct
theorem's open per-hypothesis obligations after that manifest is
checked. The shared proof-boundary object now also embeds digest-pinned
requirement-vector-corpus evidence for the frame KAT, no-export, FRI/QROM
domain, and payload KAT corpora plus the digest-pinned 8-case
proof-obligation audit-vector corpus (4 open / 4 closed) and accepted
proof-boundary KAT evidence artifact. The direct theorem manifest embeds the
same proof-boundary object and
binds its own 9-case open direct-obligation audit-vector corpus plus accepted
direct-composition KAT evidence artifact; `path2_boundary_manifest_consistency`
requires the standalone proof-boundary and direct theorem manifests to agree on
the shared proof-boundary evidence while preserving false theorem and deployment
flags. The boundary and preflight gates also compare the standalone accepted
proof-boundary requirement evidence rows back to the proof/direct
proof-boundary requirement-coverage rows, field-for-field for requirement
identity, coverage surface, vector evidence kind, current case ids,
evidence-present status, and blocker. The compact accepted-proof,
accepted-tuple, and accepted-composition
artifact summaries expose the accepted requirement, tuple-field, and hypothesis
evidence counts, exact row identity lists, accepted hypothesis-to-obligation
bindings, and their deferred rejection/QROM flags; both boundary and preflight
consistency gates check those summaries before importing them. They also compare
the standalone direct accepted tuple-field rows and accepted-hypothesis rows
back to the direct theorem accepted tuple fields, acceptance hypotheses, open
obligation ids, obligation open flags, and blocker rows, so the accepted direct
KAT evidence cannot become a separate theorem shape. The
compact proof-obligation and direct-obligation corpus summaries also retain the
raw open external-review, QROM theorem/import, lifecycle-finality, and
proof-boundary readiness flags, plus the proof-obligation corpus flags showing
that the no-export, FRI/QROM-domain, and payload-KAT vector corpora still cover
the current boundary cases. They also retain the exact proof-boundary and
direct-theorem case-id lists plus named-break taxonomies, so those obligations
cannot disappear, be reordered, or be renamed when a standalone corpus is
imported by a boundary or preflight manifest.
The reports and
manifests are
the executable boundary map for this section. Aside from the explicitly imported
local authorization-class binding and no-postchallenge timing theorem slices,
they are not an honest-sampler theorem, simulator, no-export theorem, extractor,
FRI/QROM, KAT, or external-review substitute.

The boundary is implemented but not verified. The active Rust test suite now has
an initial `Path2AccountStarkCreationPayloadV1` malformed-payload mutation
matrix, including empty-bundle, digest-bound, rebound-anchor, corrupted-bundle,
and production-gate checks, plus the feature-gated
`path2_account_creation_accepted_proof_boundary_evidence_pins_selected_payload`
test for the accepted-boundary evidence object. Open closure items are the
final composition theorem, honest-create sampler soundness,
authorization-class binding, account STARK simulator theorem,
extractor/soundness theorem, FRI/recursion residual-assumption ledger, full
consensus-visible malformed-payload KAT coverage beyond the current frozen
rows, signer leakage evidence, external proof-family review, and the explicit
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
or remain open. `formal/kat/path2_account_creation_proof_boundary_v1.json`
regenerates and diffs the report chain, including the residual rows and
payload-KAT boundary digest, before the direct theorem manifest is checked. The
boundary manifest consistency gate then compares the standalone proof-boundary
object against the proof-boundary object embedded by the direct theorem
manifest. The `path2_preflight_boundary_consistency` gate ties those reports to
the production deployment preflight, including blocker group ordering and
per-open-row blocker coverage.
`Path2AccountStarkPayloadByteModelReportV1` pins the current
`Path2AccountStarkCreationPayloadV1` canonical frame order, fixed overhead,
bundle non-empty rule, digest function, verifier version, and non-inlined
proof-byte policy. `formal/kat/path2_account_stark_payload_byte_model_v1.json`
regenerates and diffs that draft byte-model manifest, and
`path2_account_stark_payload_byte_model_consistency` checks it against the
digest-pinned vector corpus. `Path2AccountStarkPayloadKatBoundaryReportV1` then
binds that byte-model report digest while pinning current mutation cases and
mapping internal diagnostics to consensus-facing public rejection classes. This
is still precursor coverage: `formal/kat/path2_account_stark_payload_kat_boundary_v1.json`
regenerates and diffs the draft boundary manifest, but full frozen vector files
remain open. `formal/kat/path2_account_stark_payload_kat_vectors_v1.json`
regenerates the current-row digest-pinned boundary vectors under the real
account-STARK verifier feature. It uses the explicit deterministic KAT producer
`path2_account_stark_prove_creation_with_mask_seed_v1` to pin payload-frame and
account-STARK bundle digests, and it runs the proof inside a single-thread Rayon
pool so proof-of-work witness selection is reproducible. The deployment producer
remains fresh-randomized. The
`path2_account_stark_payload_kat_vector_consistency` gate cross-checks the
checked-in boundary manifest and vector corpus so their boundary-report digest,
row count, generated-vector flag, rejection rows, public classes, error markers,
selected Rust accepted-boundary payload-KAT corpus reduction hook, and false
deployment/frozen claims cannot drift independently.
`Path2AccountStarkFriQromResidualReportV1` pins the active account-STARK FRI
schedule, degree-8 challenge field, Merkle/MMCS Poseidon2 sponge floor, and the existing
checked-core spend-transcript QROM target cap, plus the selected Rust
accepted-boundary FRI/QROM transcript reduction theorem
`selectedRustAcceptedBoundaryFriQromTranscriptReduction_v1`. Its standalone manifest
`formal/kat/path2_account_stark_fri_qrom_residual_v1.json` and the
`path2_account_stark_fri_qrom_residual_consistency` gate compare those claims
against `tools/descender-account-stark/src/config.rs` and the generated QROM
JSON artifacts. The same consistency gate pins the account-STARK's migrated
rate-7/capacity-9 Fiat-Shamir challenger source shape and the native-fidelity
reference used to migrate the recursive transcript. The monolithic recursive
AIR now uses rate 7/capacity 9 for alpha, zeta, OOD, batch-combination alpha,
commit-phase betas, and query-index continuation, with native-equivalence and
tamper tests. Full-plus-partial pre-alpha and quotient rounds are constraint-checked
against a real production proof's transcript. The end-to-end migration flag stays
false. The real production OOD stream now passes a constant-width vertical
rate-7 accumulator AIR with public start/end states, native terminal-state fidelity,
tamper rejection, and a production-q128 proof. A block-wide companion now enforces
element-for-element ordered equality between independently supplied transcript and
PCS-claim streams, derives the terminal endpoint, binds it to each verified member
root, folds the same augmented roots as `BlockRecursiveAir`, and is paired with the
block proof by `BoundBlockProofV1` under one public root and an expected shape profile.
The genuine proof shape is now pinned to 23,254 extension opening terms
(`random[12] || trace[23,146] || quotient[8][12]`). A 68-column arithmetic
reference AIR reproduces the exact Plonky3 alpha-power/reduced-opening recurrence.
The production-shaped `PcsQueryAir` closes the former host-only seam for one query:
the same ordered `p_at_x` cells are absorbed into the three salted input-MMCS leaf
digests and consumed by the reduction; the `p_at_z` limbs are duplexed from the
public post-zeta state to the public OOD endpoint; the real batch-combination alpha
is derived from that endpoint; and the public terminal reduction is the selected
first value of the native-order FRI fold chain. The separate 717-column vertical
MMCS AIR now public-binds each salted leaf digest as well as its root and verifier
index. Synthetic cross-binding attacks, q128 prove/verify tests, and a genuine-proof
capstone covering the complete 23,254-term query, all three real MMCS batches, the
real OOD endpoint/challenge, and the real FRI terminal polynomial pass. The
production query-link trace at the genuine relation-v3 log height 6 is 2,265
columns, 3,325 active rows, height 4,096, 102 public field values, and 37,109,760
base-witness bytes before LDE/prover costs. `PcsQuerySettlementProfileV1` and
`PcsQuerySettlementProofV1` now provide the versioned composition boundary for
one query: the query-link proof, all three MMCS proofs, and the FRI-chain proof
verify against one typed caller-supplied statement, including every FRI beta.
The genuine relation-v3 query proves and verifies through that envelope at q128
knobs. The production profile fixes 120 queries, log height 6, two FRI rounds,
and exactly five component proofs per query. `AllPcsQueryStatementsV1` and
the checked all-query materializer now provides the compact pre-proof input
contract: the genuine relation-v3 capstone derives all 120 statements from one
verified proof, recomputes every query-link output, three MMCS roots, FRI terminal,
and both ordered roots, and evaluates all five component AIR traces per query. The
600 host-side constraint checks are performed and dropped one query at a time; they
are not recursive proofs and do not establish economical aggregation.
The recursive FRI layer-opening layout now uses the native binary-fold tree
depths 5 then 4 rather than assuming one fixed path length for both rounds.
`formal/kat/path2_pcs_all_query_statement_kat_v1.json` freezes deterministic
single-worker proof construction, all 120 query indices, both KoalaBear roots, exact reproduction
command, one observed debug runtime, and explicit false flags for the missing
all-window proof, economical aggregation, and deployment readiness. It is a
compact statement KAT, not a frozen aggregate-proof byte/runtime artifact.
Its explicit unaggregated price remains
600 proofs and 5,879,402,880 base-witness bytes before LDE/prover memory. This
is diagnostic evidence that the envelope is functionally composed but not yet
an economically viable all-query settlement consumer. The envelope also carries
domain-separated ordered roots for the transcript-owned anchors and the complete
typed statements; verification recomputes both, giving the recursive fan-out
proof an exact public export target. `FanOutAnchorAirV1` now consumes those exact
recursive cells in the same canonical 116-field preimage, constrains the
transcript-observed trace/quotient roots and final polynomial equal to the cells
consumed by the recursive opening checks, verifies exactly 120 active rows, and
uses eight inactive rows to reach proving height 128. Its q128 proof test crosses
the real proof boundary on a smaller fan-out and composes the exported anchor with
the existing chain-state root under the domain-separated
`transcript_anchored_block_root_v1`; wrong settlement, anchor, and chain roots
reject. The genuine relation-v3 capstone additionally pins all 120 real queries'
recursive source-map preimages equal, element-for-element, to their native settlement
statements. A genuine relation-v3 all-120 recursive-window trace and proof are not yet
materialized, and the diagnostic 600 component proofs have not been replaced by
an economical aggregated all-query consumer. All-query proof bytes, runtime, and
the KAT chain are likewise not materialized/frozen. The
genuine relation-v3 capstone pins 186,032 OOD field elements (26,576 rate-7
rounds); the 299-column companion has 26,589 active rows per member and a
39,190,528-byte one-member base witness matrix after power-of-two padding. This
does not include LDE/prover memory, time, proof bytes, or deployment costs. The
report also pins the published BCS error shape,
`Q = 2^16`, `T = 2^32`, candidate `kappa = 247`, conditional
`epsilon_rbr <= 2^-256`, and a conservative `C_qrom <= 2^32` lift constant.
`selectedAccountStarkFriQromRbrTerm_le_q130_v1` and
`selectedAccountStarkFriQromBcsOracleTerm_le_q130_v1` bound the two normalized
terms by `2^-130`; `selectedAccountStarkFriQromNormalizedBudget_le_q128_v1`
bounds their sum by `2^-128`. The separate exact single-leaf substitution uses
`degree_bits = 2` and `|L0| = 64`, bounds the published RBR candidate by
`2^-218`, and proves the adaptive-one-forgery arithmetic remains below q128.
`ADR-0011` selects that adaptive game with a global adversary query bound and
does not reuse the checked-core target count in the BCS term; the stronger
lifetime-union expression remains a non-selected negative control. This is
still not residual closure: the manifest
explicitly keeps the challenger-output-to-`kappa` mapping, RBR theorem, reviewed
QROM constant, full account-STARK FRI transcript QROM theorem, recursion pricing,
external FRI/QROM review, deployment-profile acceptance, and deployment-ready
status are all false. `Path2AccountStarkFriAokBoundaryReportV1` adds the
separate extractor boundary: it binds the AIR audit evidence, selected
Plonky3/q12289 proof family, current FRI/QROM residual digest, and residual
matrix digest plus the selected Rust accepted-boundary FRI/AoK extractor
reduction theorem `selectedRustAcceptedBoundaryFriAokExtractorReduction_v1`,
while keeping the FRI/AoK extractor theorem, accepting-proof extraction claim,
external extractor review, and deployment-ready status false.
Its standalone manifest
`formal/kat/path2_account_stark_fri_aok_boundary_v1.json` is regenerated and
checked by `path2_account_stark_fri_aok_boundary_consistency`.

| Residual item | Current evidence | Required closure before deployment | Named break |
|---|---|---|---|
| `Path2AccountStarkCreationPayloadV1` byte and replay binding | Wire-format digest frame, certificate/evidence binding, deployed anchor checks, active mutation tests, `Path2AccountStarkPayloadByteModelReportV1` frame/overhead/non-inlined-bundle manifest plus consistency gate, `Path2AccountStarkPayloadKatBoundaryReportV1` public rejection-class mapping, frozen manifest regeneration, deterministic digest-pinned current-row boundary vectors, manifest/vector consistency gates, accepted-boundary payload-KAT support via `selectedRustAcceptedBoundaryPayloadKatSupport_of_boundaryAccepted`, selected Rust payload-KAT corpus reduction via `selectedRustAcceptedBoundaryPayloadKatCorpusReduction_v1`, and accepted-path decoded-anchor evidence for replay digest, raw verifier version, masked projection, role-root export bridge, and packed-witness commitment exist. | Full frozen byte corpus and public rejection-class mapping for every consensus-visible malformed payload beyond the current-row boundary vectors. | `account_stark_payload_binding_break` |
| AIR relation soundness | `research/path2_account_stark_air_soundness_audit.md` reports all relation classes wired and MED findings remediated. | External proof-family review confirms the AIR constraints match the claimed creation relation and no false statement accepts except with a named proof break. | `account_stark_air_soundness_break` |
| FRI argument-of-knowledge extractor | The AIR is fully constrained, and `Path2AccountStarkFriAokBoundaryReportV1` plus `formal/kat/path2_account_stark_fri_aok_boundary_v1.json` pin the support audit, selected proof family, FRI/QROM residual digest, residual-matrix digest, AIR coverage counts, MED remediation counts, selected Rust accepted-boundary FRI/AoK extractor reduction predicate, and open extractor/review/deployment flags. | A concrete extractor theorem for the selected Plonky3/q12289 configuration shows accepting proof implies an extractable account-creation witness or exports a named FRI/AoK break. | `account_stark_fri_aok_extractor_break` |
| Zero-knowledge simulator and no-export statement | Boundary text forbids export of pad seed, account projection, opening-tuple preimage, authorization witness, mask, private trace material, and commitment nonce. The decoded `AccountCreationPublic` checked-core syndrome is explicitly classified as an allowed public authorization anchor, and all 14 nested public-statement fields are audited rather than hidden behind the opaque-bundle label. `Path2AccountCreationNoExportBoundaryReportV1` pins that nested audit, simulator/no-export theorem targets, the non-circular simulator input, private-material denylist, frozen vectors, selected Rust public projections/reductions, and fail-closed production flags. The selected hiding stack uses a 256-bit-seeded ChaCha20 CSPRNG with independent input-MMCS, FRI-MMCS, and PCS substreams, while `SelectedAccountStarkSimulatorHybridBounds.advantage_le_totalError` and `selectedRustAcceptedBundleSimulatorNoExportFromHybrid_v1` machine-check the additive A1/A2/A3 hybrid under concrete supplied hop bounds. For A2, the C2 AIR and standalone signer absorb the explicit `MASK` domain, squeeze 36 disjoint KoalaBear coordinates, and use six digits per each of six q12289 masks. `selectedAccountMaskWideReductionError_le_q128_v1` prices the full six-lane term at the selected `2^32` account horizon, `selectedRustAccountMaskPrfFramesFresh_of_verifiedNullifierRegistry_v1` imports proof-bound admitted-frame freshness, and `selectedRustUniformMaskTranslationBijective_v1` closes the modular translation. For A3, relation v3 absorbs explicit `PADS`/`OPEN` domains and the deployment prover supplies fresh 40-byte commitment nonces. `selectedAccountCommitmentQromSearchError_le_q128_v1` prices two commitments per account, `2^32` targets, and `2^54` QROM queries at `2^-179`; the frame/domain arithmetic is machine-checked. The bounded-query concrete Poseidon2 mask-PRF and commitment/QROM replacement theorems remain open. Simulator, accepted-bundle simulability, review, and deployment flags remain false. | Import reviewed concrete zk-FRI HVZK, bounded-query Poseidon2 sponge/permutation PRF, and Poseidon2 commitment-hiding bounds into the selected hybrid certificate so the accepted bundle is simulatable without private creation material beyond the explicitly public statement. | `account_stark_simulator_or_no_export_break` |
| FRI soundness margin and recursion ledger | `Path2AccountStarkFriQromResidualReportV1` pins `log_blowup = 4`, `num_queries = 120`, query PoW 16, conservative-Johnson FRI schedule label 256 bits, degree-8 challenge field, and the 139-bit rate-7/capacity-9 Merkle/MMCS and Fiat-Shamir sponge floors. The account-STARK executable config and the monolithic recursive challenger arithmetic now use rate 7/capacity 9. The recursive AIR constrains full-plus-partial pre-alpha and quotient observes, alpha/zeta refills, OOD observation, batch-combination alpha, 13-element commit-phase digests and betas, and final/query continuation. Native-equivalence and permutation/partial-tail tamper tests cover those schedules; a real-proof capstone constraint-checks the production pre-alpha/quotient blocks against supplied transcript values and reproduces every production challenge and query index. The same capstone materializes the real production OOD stream through a constant-width vertical accumulator and independently reconstructs the PCS claim order. `BlockOodBindingAir` proves their ordered equality, endpoint/member-root augmentation, and block chain; `BoundBlockProofV1` verifies that companion and `BlockRecursiveAir` against one root and one expected shape, with q128 transport/mixup tests. `PcsQueryAir` now uses the same cells for salted input-leaf hashing and the exact opening reduction, derives the batch alpha from the OOD endpoint, shares public leaf digests with the vertical MMCS proofs, and public-binds the reduction into the native-order FRI chain. `PcsQuerySettlementProofV1` additionally verifies that query link, all three MMCS openings, and the beta-bound FRI chain against one expected statement; a genuine relation-v3 query proves through it. The fixed all-120 profile exposes the unaggregated cost as 600 proofs and 5,879,402,880 base-witness bytes. `FanOutAnchorAirV1` implements the exact canonical transcript-anchor export with 120 active / 128 total rows, transcript-consumer seams, and a versioned block-root composition; a smaller-fan-out q128 proof and coherent root-substitution attacks pass/reject as expected. Genuine all-120 recursive-window materialization, economical all-query aggregation, and proof-byte/runtime/KAT closure remain open, so the end-to-end migration flag remains false. The conditional lifetime stress ledger imports `Q = 2^16` and `T = 2^32`, while the exact single-leaf substitution pins `degree_bits = 2`, `|L0| = 64`, and the published candidate RBR terms at `<= 2^-218`. `ADR-0011` selects the theorem-native adaptive game with global `Q` and no account-count multiplier; its arithmetic clears q128 (`2^-154` and `2^-165` terms). The seven-case FRI/QROM domain corpus keeps the end-to-end challenger migration, RBR hypothesis mapping, challenger-output mapping, QROM constant review, transcript theorem, recursion, and review flags open. | Materialize and prove the genuine 120 recursive query windows against the exported anchor; implement and price an economically aggregated all-query consumer rather than the 600-proof diagnostic envelope; freeze proof bytes/runtime/KATs; map every relevant Plonky3 challenger output to effective `kappa >= 247`; instantiate the selected RBR/BCS/QROM theorem with an explicit lift constant within the `2^32` allowance; compose the extractor/AIR/MMCS/primitive terms; and obtain external FRI/QROM review. | `fri_soundness_margin_break` |
| Production admission gate and review disposition | `verify_path2_direct_creation_account_stark_payload_boundary_for_production` remains fail-closed and regression-tested; the proof-boundary report and proof-obligation corpus pin `selectedRustAcceptedBoundaryProductionGateReviewReduction_v1` as a reduction from accepted-boundary fail-closed support to a supplied review/maintainer-gate theorem without instantiating that theorem. | External review, tracker disposition, and explicit maintainer decision flip the production gate; otherwise production admission must continue to reject. | `account_stark_production_gate_break` |
| Legacy proof-route disposition | Transparent descent/native-boundary paths are fail-closed support and cannot be selected by implication. | Any route change requires an ADR that updates this matrix, wire formats, KAT scope, and tracker ownership before the route can become deployment authority. | `proof_route_authority_break` |

## Current Security-Level Position

The active Poseidon2/Plonky3 path carries the candidate q128 schedule: 120 FRI
queries, query PoW 16, rate-7/capacity-9 width-16 Poseidon2 Merkle/MMCS and
Fiat-Shamir sponges, and a 13-element Merkle digest. The generated residual
ledger records the conservative-Johnson schedule label at 256 bits and both
generic sponge floors at 139 bits. The account-STARK migration and recursive
rate-7/capacity-9 challenger arithmetic are complete, including constraint-checked
real pre-alpha/quotient streams and the constant-width vertical accumulation of the
real production OOD stream. Ordered transcript/claim equality, endpoint/member-root
binding, and paired production-settlement transport are implemented. One genuine
query now carries those claim-side values through same-cell salted-leaf hashing,
MMCS leaf-digest authentication, exact opening reduction, and a beta-bound public
native-order FRI-chain boundary. A versioned per-query settlement envelope verifies
all five component proofs against one expected statement, and its production profile
fixes the all-120 count and exposes the unaggregated 600-proof / 5,879,402,880-byte
base-witness price. The envelope recomputes ordered, domain-separated transcript-anchor
and complete-statement roots. The production-shaped fan-out wrapper now fixes 120
active rows in a 128-row trace and implements the canonical anchor export and
versioned block-root composition, with a smaller-fan-out q128 proof/mutation test.
One genuine relation-v3 query also pins the recursive source-map preimage to the
native settlement statement.
Materializing the genuine relation-v3 all-120 windows/proof, replacing the diagnostic
600-proof form with an economically aggregated all-query proof, and freezing proof
bytes/runtime/KATs are not complete. The conditional BCS arithmetic fits
q128 for `Q = 2^16`, `T = 2^32`, candidate `kappa = 247`, a `2^-256` RBR
input, and a QROM lift constant at most `2^32`. The explicit published-bound
candidate for the actual single leaf is `2^-218`. `ADR-0011` selects its
adaptive-one-forgery game with a global query bound and no account-count
multiplier. The end-to-end production transcript/KAT migration, challenger mapping,
RBR theorem mapping, and constant review are not instantiated. Therefore neither the
schedule label, sponge floors, nor conditional arithmetic is a composed
deployment security level.

The pre-q128 post-quantum shortfall remains important historical evidence
(assumption-boundary stress test,
`research/cryptanalysis/assumption_boundary_stress_test_20260627.md`,
2026-06-27): the earlier 56-query/capacity-8 profile did not have an instantiated
hard quantum-128 argument. Its "single Grover search" calculation is retained as
a negative-control sizing heuristic, not as the selected theorem. The published
FRI BCS/QROM result instead requires explicit round-by-round error, oracle-output
length, and bounded oracle-query terms. The current 120-query candidate profile
is a response to that finding; it does not by itself clear the theorem or
deployment gate.

`ADR-0008` commits the deployable bar to hard quantum-128 and is the governing
decision. The current implementation has landed a candidate q128 FRI and
Merkle/MMCS parameter set plus a machine-checked conditional BCS budget, while
the Fiat-Shamir challenger-output mapping and bounded-query QROM theorem remain
open. A deployment-ready hard-128 claim
also depends on the full consensus-visible KAT corpus, external FRI/QROM and
implementation review, the production-gate decision, and any selected
aggregation/recursion review scope. Until those gates close, the project must
not claim "hard 128-bit deployment readiness."

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
| 1 | Direct theorem closure | `Path2DirectTheoremBoundaryReportV1`, `Path2DirectAcceptedSpendTupleEvidenceV1`, and `formal/kat/path2_direct_theorem_boundary_v1.json` pin the product theorem boundary and accepted-path tuple evidence, and the final product theorem covers admission, spend, consume-once lifecycle, exact public bindings, and every named break; supporting lemmas and executable accepted-path evidence cannot stand in for the deployment theorem. | `DIR-RISK-001`, `DIR-SEC-001`, `DIR-CREATE-001`, `DIR-PROOF-002`, `DIR-AUTHCLASS-001` |
| 2 | Account-creation proof boundary | `Path2AccountCreationProofBoundaryReportV1` and `Path2AccountCreationAcceptedProofBoundaryEvidenceV1` pin the selected account-STARK payload boundary, accepted public digest chain, decoded public-anchor equality, and fail-closed production gate; the payload still needs a concrete frozen byte model, theorem-level replay-payload and deployed-anchor binding closure, simulator theorem, extractor theorem, hidden-source payload statement, production-gate decision, and full rejection behavior. | `DIR-CREATE-001`, `DIR-PROOF-001`, `DIR-STARK-001`, `DIR-BIND-001`, `DIR-PAYLOAD-001`, `DIR-KAT-001`, `DIR-REJECT-001` |
| 3 | Authorization-class binding | Private authorization-local material cannot change public spend semantics, fork account state, or bypass the checked-core public binding. | `DIR-AUTHCLASS-001`, `DIR-SEC-001`, `DIR-CC-001` |
| 4 | Consensus conformance | Canonical wire formats, public-field mutation vectors, end-to-end creation/spend/lifecycle vectors, and deterministic rejection behavior exist. | `DIR-KAT-001`, `SPEC-KAT-001`, `DIR-REJECT-001`, `DIR-CC-001`, `DIR-LEAF-001`, `DIR-EVID-001` |
| 5 | Signer leakage and fault safety | Signer, keygen, sampler, rejection loops, fault handling, errors, zeroization, target/build profiles, and leakage evidence satisfy the signer profile without floating-point signing assumptions. | `DIR-CT-001`, `SPEC-SIGNER-001`, `SPEC-LEAKAGE-001`, `ADOPT-HW-001`, `ADOPT-HW-CT-001`, `ADOPT-HW-ENT-001`, `ADOPT-HW-STATE-001` |
| 6 | Lifecycle target profile | Admit, spend, replay, burn, recovery, restore, finality, and reorg semantics are instantiated for the target chain or deployment profile. | `DIR-LIFE-001`, `SPEC-LIFE-001`, `SPEC-STATE-001`, `SPEC-DEPLOY-001` |
| 7 | External review and residual assumptions | Independent review covers the coded-SIS/lattice core, account-creation proof family, Poseidon2/FRI usage, QROM framing, signer/verifier engineering, lifecycle model, and residual-assumption dispositions. | `DIR-REVIEW-001`, `SPEC-REVIEW-001`, `DIR-QROM-001`, `DIR-ACTION-001`, `DIR-SEC-001` |

A direct-mode deployment claim can be made only after every ranked blocker above
is either `verified` in `TRACKER.md` or explicitly retired by an ADR that updates
this section and the requirement gates. Mode B aggregation, policy features,
Ethereum positioning, commercial demos, and wallet integrations must not be used
to compensate for an open blocker in ranks 1 through 7. The local and CI
`path2_preflight_boundary_consistency` gate enforces that the production
preflight remains blocked by the current rank-1/rank-2 theorem and proof-family
reports until those reports are explicitly closed.

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
- deployment-ready hard 128-bit security without q128 KAT freeze, external
  review, and production-gate acceptance;
- deployability based only on internal tests or internal proof notes.
