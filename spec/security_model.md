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
`Path2DirectLifecycleBoundaryReportV1`, and the current
`Path2AccountCreationProofBoundaryReportV1` by a local and CI gate, so the
direct theorem target, conclusion, QROM/action digest, lifecycle digest,
proof-boundary digest, and selected field groups cannot drift silently. The
production report validator separately rejects weakened
acceptance-hypothesis/obligation-row coverage, missing pinned accepted-evidence
hooks, missing direct closure-dependency rows, unexpectedly closed theorem
flags, missing per-row closure-dependency evidence, deployment-ready flags, or
direct theorem blocker drift. The boundary and preflight consistency gates also
compare each direct closure-dependency evidence row field-for-field against its
source closure-dependency row. The
`formal/kat/path2_direct_theorem_obligation_vectors_v1.json` corpus records one
digest-pinned audit case for each open direct-theorem obligation and binds that
row list to the current QROM/action, lifecycle, and account-creation
proof-boundary reports while also carrying the direct closure-dependency rows
and derived evidence rows that tie the nine accepted-tuple hypotheses to their
still-open source reports/support surfaces. It keeps composition,
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
report when applicable, and still-open direct theorem obligation. The
`verify_path2_direct_single_slot_spend_authorization_with_tuple_evidence_v1`
hook runs the real direct verifier, snapshots the consume-once transition, and
checks the public result wrapper independently accepts the same slot from the
same before-state. The
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
This does not replace panic-free malformed-input KATs for every public object,
state-no-mutation coverage for every direct-mode rejection, or full
target-chain consensus-client mapping still required for closure. The
QROM/action report pins the checked-core direct query budget, 182-bit priced
transcript term,
246-bit full challenge term, 163-bit Fq12 action floor, direct-mode `q_chal`
and inactive `q_batch` status, and generated QROM artifacts, while keeping
theorem import, external review, theorem-linked QROM/action closure, and
deployment readiness false. The lifecycle report pins the local
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
deployment.

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
FRI/QROM residual pricing, frozen payload KATs, and production-gate review. Six
rows remain open, while the no-postchallenge timing and frozen payload-KAT rows
are closed by imported evidence. Each open row carries a named break and the
fail-closed account-creation proof-boundary blocker. A separate
closure-dependency row table maps those eight proof obligations to their source
report flag groups and, where
applicable, the selected account-STARK residual matrix row. Derived
closure-dependency evidence rows pin each dependency to the open proof
obligation, open source support surface, named break, and blocker; the production
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
selected-Rust typed compatibility/projection bridge hooks. The
sampler/timing report imports that audit digest, and
`formal/kat/path2_authorization_class_binding_audit_v1.json` is regenerated and
checked before sampler/timing and proof-boundary manifests. This is executable
audit evidence for the authorization-class binding residual; it keeps the
authorization-class theorem, full honest-create sampler integration, external
review, and deployment readiness false.
`formal/kat/path2_account_creation_proof_obligation_vectors_v1.json`
turns that row table into a digest-pinned audit corpus bound to both the
proof-boundary and direct-theorem reports; it records the current 6-open /
2-closed proof-boundary accounting and keeps deployment fail-closed. A separate
requirement-coverage matrix maps every
acceptance requirement to its current evidence surface, vector evidence kind,
current vector case ids where they exist, public rejection classes for rejection
vectors, closure target, and blocker; it records that all ten requirements have
current frame-KAT, account-STARK payload, no-export audit, or FRI/QROM domain
audit vectors. The FRI/QROM domain audit corpus pins the selected proof-family
domain, Poseidon2 width and sponge floor, FRI challenge schedule, checked-core
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
digest-pinned no-export audit vector case ids, and named
`account_stark_simulator_or_no_export_break`, while keeping simulator theorem,
no-export theorem, accepted-bundle simulability, frozen theorem-level vectors,
external review, and deployment readiness false.
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
absence, and opaque proof-bundle manifest policy.
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
proof-obligation audit-vector corpus (6 open / 2 closed) and accepted
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
the executable boundary map for this section; they are not an honest-sampler
theorem, authorization-class binding theorem, no-postchallenge timing theorem,
simulator, no-export theorem, extractor, FRI/QROM, KAT, or external-review
substitute.

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
and false deployment/frozen claims cannot drift independently.
`Path2AccountStarkFriQromResidualReportV1` pins the active account-STARK FRI
schedule, degree-8 challenge field, Poseidon2 sponge floor, and the existing
checked-core spend-transcript QROM target cap. Its standalone manifest
`formal/kat/path2_account_stark_fri_qrom_residual_v1.json` and the
`path2_account_stark_fri_qrom_residual_consistency` gate compare those claims
against `tools/descender-account-stark/src/config.rs` and the generated QROM
JSON artifacts. This is still not residual closure: the manifest explicitly
records that the account-STARK FRI transcript QROM theorem, recursion pricing,
external FRI/QROM review, deployment-profile acceptance, and deployment-ready
status are all false. `Path2AccountStarkFriAokBoundaryReportV1` adds the
separate extractor boundary: it binds the AIR audit evidence, selected
Plonky3/q12289 proof family, current FRI/QROM residual digest, and residual
matrix digest while keeping the FRI/AoK extractor theorem, accepting-proof
extraction claim, external extractor review, and deployment-ready status false.
Its standalone manifest
`formal/kat/path2_account_stark_fri_aok_boundary_v1.json` is regenerated and
checked by `path2_account_stark_fri_aok_boundary_consistency`.

| Residual item | Current evidence | Required closure before deployment | Named break |
|---|---|---|---|
| `Path2AccountStarkCreationPayloadV1` byte and replay binding | Wire-format digest frame, certificate/evidence binding, deployed anchor checks, active mutation tests, `Path2AccountStarkPayloadByteModelReportV1` frame/overhead/non-inlined-bundle manifest plus consistency gate, `Path2AccountStarkPayloadKatBoundaryReportV1` public rejection-class mapping, frozen manifest regeneration, deterministic digest-pinned current-row boundary vectors, manifest/vector consistency gates, and accepted-path decoded-anchor evidence for replay digest, raw verifier version, masked projection, role-root export bridge, and packed-witness commitment exist. | Full frozen byte corpus and public rejection-class mapping for every consensus-visible malformed payload beyond the current-row boundary vectors. | `account_stark_payload_binding_break` |
| AIR relation soundness | `research/path2_account_stark_air_soundness_audit.md` reports all relation classes wired and MED findings remediated. | External proof-family review confirms the AIR constraints match the claimed creation relation and no false statement accepts except with a named proof break. | `account_stark_air_soundness_break` |
| FRI argument-of-knowledge extractor | The AIR is fully constrained, and `Path2AccountStarkFriAokBoundaryReportV1` plus `formal/kat/path2_account_stark_fri_aok_boundary_v1.json` pin the support audit, selected proof family, FRI/QROM residual digest, residual-matrix digest, AIR coverage counts, MED remediation counts, and open extractor/review/deployment flags. | A concrete extractor theorem for the selected Plonky3/q12289 configuration shows accepting proof implies an extractable account-creation witness or exports a named FRI/AoK break. | `account_stark_fri_aok_extractor_break` |
| Zero-knowledge simulator and no-export statement | Boundary text forbids export of pad source, projection, syndrome, opening tuple, witness, mask, and private trace material; `Path2AccountCreationNoExportBoundaryReportV1` plus `formal/kat/path2_account_creation_no_export_boundary_v1.json` pin the support file, selected payload endpoints, payload byte-model/KAT-boundary digests, denylist, simulator/no-export theorem targets, accepted-bundle simulator public-input allowlist, public-payload-only byte model, mutation coverage, and fail-closed production gate while leaving simulator, no-export, accepted-bundle simulability, review, and deployment flags false. | Simulator theorem for the selected payload proves the accepted public bundle can be simulated from public anchors and allowed commitments without revealing private creation material. | `account_stark_simulator_or_no_export_break` |
| FRI soundness margin and recursion ledger | `Path2AccountStarkFriQromResidualReportV1` pins `log_blowup = 4`, `num_queries = 120`, query PoW 16, conservative-Johnson FRI label 256 bits, degree-8 challenge field, 139-bit Poseidon2 sponge floor, and checked-core QROM `log2(T) <= 54`; `formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json` adds digest-pinned audit cases for the selected proof-family domain, FRI schedule, checked-core QROM import, and open theorem flags; the consistency gates compare these to active config, checked-in manifests, and generated QROM artifacts. | Residual-assumption ledger prices FRI queries, field size, the account-STARK Poseidon2/FRI transcript theorem, recursion if used, and multi-target/QROM terms against the selected deployment profile, then obtains external FRI/QROM review. | `fri_soundness_margin_break` |
| Production admission gate and review disposition | `verify_path2_direct_creation_account_stark_payload_boundary_for_production` remains fail-closed and regression-tested. | External review, tracker disposition, and explicit maintainer decision flip the production gate; otherwise production admission must continue to reject. | `account_stark_production_gate_break` |
| Legacy proof-route disposition | Transparent descent/native-boundary paths are fail-closed support and cannot be selected by implication. | Any route change requires an ADR that updates this matrix, wire formats, KAT scope, and tracker ownership before the route can become deployment authority. | `proof_route_authority_break` |

## Current Security-Level Position

The active Poseidon2/Plonky3 path has been re-parameterized to the q128 profile:
120 FRI queries, query PoW 16, and width-16 Poseidon2 with capacity 9 and a
13-element Merkle digest. The generated residual ledger now prices the
conservative-Johnson FRI label at 256 bits and the Poseidon2/Merkle generic
floor at 139 bits. This is the honest current parameter floor and must be stated
plainly.

The pre-q128 POST-QUANTUM shortfall remains important historical evidence
(assumption-boundary stress test,
`research/cryptanalysis/assumption_boundary_stress_test_20260627.md`,
2026-06-27): under non-interactive Fiat-Shamir the full FRI query plus
proof-of-work grinding budget is a single Grover search, so the earlier
56-query/capacity-8 profile could not support a hard quantum-128 claim. The
current q128 profile is the response to that finding; it does not by itself
clear the deployment gate.

`ADR-0008` commits the deployable bar to hard quantum-128 and is the governing
decision. The current implementation has landed the q128 parameter profile for
the active account-STARK/direct path, but a deployment-ready hard-128 claim still
depends on the full consensus-visible KAT corpus, external FRI/QROM and
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
