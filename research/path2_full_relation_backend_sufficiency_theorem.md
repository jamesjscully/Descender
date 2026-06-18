# Path 2 Full-Relation Backend Sufficiency Theorem

Status: positive theorem target machine-checked at the interface level; concrete
backend not instantiated.

Purpose: push blocker 1 in the positive direction without hiding the current
negative results. The current executable masked-share core is not sound for the
hidden-opening relation. A replacement backend can close blocker 1 only if it
supplies the full selected transparent relation certificate named here.

## Formal Lean Anchor

`Descender/Path2NativeBoundary.lean` now defines:

```text
SelectedTransparentFullRelationBackendCertificate(P)
```

and proves:

```text
selected_transparent_full_relation_backend_certificate_implies_account_admission
  (P : Predicates)
  (C : SelectedTransparentFullRelationBackendCertificate P)
  : AccountAdmissionSoundness P
```

This is the positive blocker-1 sufficiency theorem. It says that if the concrete
backend supplies the selected transparent full-relation certificate, accepted
account creation yields a hidden representative in the accepted boundary fiber,
with local validity and descent consistency, or exports a named break.

## Certificate Contents

The certificate packages the assumptions that the current core is missing:

- account-create-to-selected-view binding;
- parent/root role binding;
- role-restriction soundness;
- parent trace commitment extraction;
- double-trace binding;
- masked-opening consistency;
- extracted-trace relation knowledge;
- trace assembly into a hidden representative;
- no-export / role-authentication-material no-export;
- mask freshness;
- unique mask domains for opened masked cells, including the selected nonzero
  role-domain injectivity lemma in addition to mask freshness;
- QROM schedule and extractor-rewind bounds;
- hybrid base-128 range-row soundness;
- blinding/noise row soundness.

In Rust terms, this is the theorem-level counterpart of replacing the current
uninstantiated
`Path2TransparentFullHiddenRelationSubproofEnvelope.v1` backend with a real
backend for `Path2TransparentFullHiddenRelationProofContractV1`.

## Rust Anchor

The executable metadata now binds both sides of the blocker-1 fork.

The current-core negative status is bound through:

```text
Path2TransparentCurrentCoreNegativeTargetV1
```

That target hashes the Lean negative theorem names and Rust regression names,
including the combined verdict theorem
`current_transparent_route_blocker1_negative_witness`. It records that the
sampled masked-share core is only a core and rejects any production soundness
claim for the current core.

The narrow redesign target is bound through:

```text
Path2TransparentTraceToBindingRelationLayerV1
```

This is the missing verifier-visible bridge between committed trace roots and
the account-creation binding. It requires the backend to prove that the same
hidden opening supplies the leaf pad-seed digest, pad-seed/slot PRF masks,
packed-witness repack commitment, role commitments, native role root,
trace-root role columns, public masked projection seam, and zero-knowledge
witness hiding. Its canonical `trace_to_binding_relation_implemented` flag
requires both the hidden-relation constraint plan and the public full-relation
certificate proof-verifier bit, and is currently false. Implementing the hidden
constraints alone therefore cannot make the route advertise this bridge; it
remains fail-closed until a concrete backend proves the bridge without exposing
row blinds or hidden openings.
The layer separately records the partial executable bridge now present in Rust:
the hidden-relation plan records reference pad-seed binding, PRF-mask
derivation, packed-witness-repack, and role-commitment consistency checks as
implemented, canonical local trace-binding check records are required and
carried, and the outer proof suite must be bound to a nonempty
full-hidden-relation subproof. The local bridge digest check, direct statement
bridge, and reference certificate bridge are also required. These checks jointly set
`reference_trace_to_binding_bridge_implemented = true`, but they do not make the
full `trace_to_binding_relation_implemented` bit true. The proof-family
capability likewise keeps `full_trace_extractable` and full witness-relation
knowledge tied to aggregate production-soundness discharge, not merely to the
hidden-constraint plan.

The current fail-closed chain is:

1. `proof_friendly_inner_commitments_implemented = false`, because the
   proof-friendly inner commitment plan is pinned but not deployably
   instantiated.
2. `pad_seed_binding_implemented`, `prf_mask_derivation_implemented`,
   `packed_witness_repack_implemented`, and
   `role_commitment_consistency_implemented` are all false for the actual
   private relation constraints. The corresponding reference checks are true,
   but those checks only support construction/regression tests.
3. `full_hidden_relation_constraints_implemented = false`, because it requires
   the deployable inner commitment layer and all four private constraints.
4. `proof_payload_contract_implemented = false`, so the backend verifier result
   can bind typed payload/receipt bytes but cannot accept a backend proof.
5. `concrete_backend_certificate_instantiated = false` and
   `external_coded_sis_review_complete = false`, so
   `full_hidden_relation_public_certificate_proof_verified` cannot be promoted.
6. `trace_to_binding_relation_implemented = false` and aggregate
   `production_soundness_obligations_discharged = false`; the public API must
   keep returning the production-soundness-disabled error.

The Lean boundary now mirrors the backend part of that chain with
`FullHiddenRelationBackendImplementationSoundness`. The selected backend
certificate derives that aggregate from `proofFriendlyInnerCommitments`,
`padSeedBindingConstraint`, `prfMaskDerivationConstraint`,
`packedWitnessRepackConstraint`, `roleCommitmentConsistencyConstraint`, and
`externalCodedSisReview` before proving account admission. This makes those
certificate fields proof-relevant instead of merely serialized Rust labels.

The current executable core has one bridge strengthening: trace and constraint
Merkle leaves are no longer context-free. They hash a public
`TraceBindingContext` digest over the non-root public statement components,
including the slot and hidden-source statement digest, so opened rows
authenticate only for the leaf/binding/source/masked-projection context they
were committed under. The transparent public view now carries those two source
anchors directly, and both its transcript/public-view digest and the top-level
proof-bytes envelope bind them. The proof-bytes envelope verifier rejects any
slot or hidden-source statement mismatch against the public view before the
full-hidden subproof bridge check. This does not instantiate the full
certificate; it only removes anonymous trace-root replay and outer-envelope
source rebinding as subproblems.

It also has one verifier-safe public commitment check: the public masked
projection in `Path2TransparentDescentPublicViewV1` must match the projected
lanes inside `packed_witness_commitment`, and `pad_source_commitment` must be
the canonical role commitment to that same masked projection. These checks make
the public seam less forgeable, but they do not replace the private
full-relation obligations for pad-seed binding, PRF mask derivation,
packed-witness repack, or role-opening consistency.

The hidden-relation plan now also carries
`Path2TransparentProofFriendlyInnerCommitmentPlanV1`. This pins the positive
backend route to proof-friendly inner commitments for the private roles instead
of a direct SHA-preimage proof over `pad_seed_digest`. A backend certificate may
still retain SHA-256 for outer consensus binding, but it cannot mark the full
hidden relation implemented unless the inner commitment layer is instantiated
and no raw hidden openings are exported.

The replacement-backend sufficiency target is bound through:

```text
Path2TransparentFullRelationBackendSufficiencyTargetV1
```

That target hashes the Lean certificate name, Lean theorem name, the
`AccountAdmissionSoundness` conclusion, a digest of the 32 certificate field
obligations, the hidden-relation constraint-plan digest, and the proof-contract
schema. The canonical target records `concrete_backend_certificate_instantiated
= false`; attempts to drift the theorem name, change the obligation digest, omit
the account-admission adapter, or mark the certificate instantiated are rejected
by the target digest validator.

The 32-field obligation vector is, in order:
`createToView`, `parentSound`, `roleSound`, `commitExtract`,
`doubleTraceBinding`, `maskedConsistency`, `relationKnowledge`,
`traceAssemblyExists`, `traceAssemblyRoles`, `traceAssemblyDerived`,
`traceAssemblyFiber`, `traceAssemblyLocal`, `traceAssemblyDescent`,
`noExportSound`, `roleAuthNoExportSound`, `maskFreshnessSound`,
`uniqueMaskDomains`, `queryOpeningComponentsBound`, `qromSound`,
`qromRewindSound`, `externalCodedSisReview`, `traceRangeRows`, `rangeRows`,
`rangeRowsToNorm`, `traceBlindRows`, `blindRows`, `blindRowsToBound`,
`proofFriendlyInnerCommitments`, `padSeedBindingConstraint`,
`prfMaskDerivationConstraint`, `packedWitnessRepackConstraint`, and
`roleCommitmentConsistencyConstraint`.

Lean keeps `maskFreshnessSound` and `uniqueMaskDomains` separate: mask
freshness is the accepted-view no-leakage/domain-separation predicate, while
unique mask domains are the selected profile's nonzero opened-cell domain
injectivity table. This matches the two distinct Rust certificate fields.

`Path2TransparentFullHiddenRelationProofContractV1`,
`Path2TransparentProductionSoundnessObligationsV1`, and
`Path2TransparentProofFamilyCapabilityV1` all carry both the negative-target
digest, the trace-to-binding layer digest, and the positive sufficiency-target
digest. Public inputs bind the trace-to-binding layer directly and bind the
remaining targets transitively through the full-hidden-relation proof-contract
digest.

The Rust backend payload boundary now separates two checks:

- `validate_path2_transparent_full_relation_backend_certificate_payload_shape_v1`
  checks only the typed wire shape, statement/verifier-key context, obligation
  labels (`obligation_set_bound = true`), no-export flags, and the backend
  proof-system id / backend acceptance-verifier digest / verified proof-payload
  digest / backend verifier-result digest / backend verifier-receipt digest. In
  the current profile the verifier digest names the uninstantiated disabled
  verifier, the proof-payload digest is the disabled-backend marker, the
  verifier-result digest binds the same statement, verifier key, proof payload,
  obligation set, trace roots, check digests, no-export status, a
  per-obligation verification bitmap, per-obligation proof digests, and explicit
  hidden/trace witness-check verified bits plus
  trace/constraint-root-to-hidden-witness verified bits, and the verifier-receipt
  digest binds the exact typed receipt envelope for the disabled backend proof
  bytes. In the raw-digest constructor those explicit verified bits are false;
  `path2_transparent_full_relation_backend_certificate_payload_from_checks_v1`
  sets the hidden/trace witness-check verified bits and the trace/constraint
  root bridge verified bits only after validating the actual check objects and
  their shared statement context. Those checks now mint a typed
  `Path2TransparentLocalTraceBindingProofV1` object whose digest is carried
  separately from the 32 full backend obligation proof digests and is repeated
  by the certificate bundle, full-hidden proof payload, and subproof envelope.
  The hidden-relation and trace-origin check records both carry the
  pre-proof direct-payload statement bridge digest, full-hidden-relation
  proof-contract digest, and hidden-relation constraint-plan digest explicitly,
  so the trace-root recomputation record cannot be rebound to a neighboring
  direct-payload statement or contract/plan context while sharing the same
  trace-to-binding layer digest.
  The local trace-binding proof and backend verifier result also carry and hash
  that same statement bridge digest, proof-contract digest, and constraint-plan
  digest, so the reduced local bridge identity itself cannot be replayed under a
  neighboring statement or contract/plan context. The backend verifier receipt
  repeats those same anchors alongside the local bridge digest, so the receipt
  cannot reduce the bridge back to only a verifier-result digest plus
  trace-layer digest.
  The standalone digest validators for the hidden witness-check record,
  trace-origin witness-check record, full-hidden subproof statement, local
  trace-binding proof, backend verifier result, backend verifier receipt,
  certificate payload, and final proof payload also reject a mismatched
  transparent proof-system id. A local bridge object therefore cannot be
  re-digested as if it belonged to a neighboring proof family.
  The certificate payload, final full-hidden proof payload, and outer subproof
  envelope also carry and hash those contract/plan anchors directly; shape
  validation compares them across the statement, embedded certificate, receipt,
  and envelope before the proof-carrying backend gate is reached.
  The certificate bundle header now carries and hashes that local trace-binding
  proof digest directly, and the bundle also carries the canonical
  hidden-relation and trace-origin witness-check record bytes; when the
  certificate claims those checks verified, bundle validation decodes the
  records, binds their digests and the local bridge digest to the certificate
  and receipt, binds the trace-origin record back to the hidden-relation record,
  and recomputes the local trace-binding proof digest. A checked digest-only
  bundle is rejected. The backend verifier result and certificate payload also
  reject a nonzero local trace-binding proof digest unless both
  trace-root-to-hidden-witness verified flags are set, so a local proof digest
  cannot be treated as bridge evidence without the corresponding verified check
  path. They also reject an asymmetric one-root verified state, so the local
  bridge is all-or-nothing across trace and constraint roots. The digest-only
  local proof validator is only a structural check; the
  bridge-evidence verifier reconstructs the typed local proof from the statement
  and carried hidden/trace check records before accepting it. The backend-result
  local-proof constructor remains internal for digest recomputation, not a public
  bridge-evidence constructor. The typed local proof hashes the hidden-check digest,
  trace-origin-check digest, full-hidden proof-contract digest, hidden-relation
  constraint-plan digest, verified check bits, no-export bit, transparent
  transcript digest, trace schedule
  (`trace_rows`, `query_count`, and `query_index`), public-view/binding anchors,
  and trace roots. The proof payload and outer full-hidden subproof envelope
  also repeat the hidden-check digest, trace-origin-check digest, transparent
  transcript digest, and trace schedule from the statement/payload context, and
  a forged envelope with a repaired outer payload digest but tampered carried
  check bytes is rejected by the public-envelope shape verifier.
  The envelope carries the local bridge digest: the current
  statement-only debug envelope must carry zero, but a typed proof-payload
  envelope that has been checked up to the backend-unimplemented gate must carry
  the proof payload's nonzero local bridge digest plus nonzero hidden/trace
  witness-check digests and reject any mismatch.
  The top-level proof-bytes envelope also carries a digest of the exact
  full-hidden subproof segment parsed from the proof-suite body, plus the
  decoded subproof envelope's local trace-binding proof digest and hidden/trace
  witness-check digests, so the outer proof bytes bind the bridge segment
  directly instead of only through the aggregate proof-body digest.
  The top-level transparent-descent payload repeats that full-hidden subproof
  digest, local bridge digest, and hidden/trace check digests and includes them
  in its payload digest; the proof-bytes boundary decodes the envelope and
  rejects a zero or mismatched bridge handle before any sampled-opening or
  backend path can accept.
  Transparent descent uses a two-stage direct-payload bridge. The full-hidden
  public inputs carry a pre-proof `direct_payload_statement_bridge_digest`,
  computed from the certificate relation transcript, creation binding, selected
  public view, trace roots, transcript, schedule, and trace-binding context
  before proof bytes exist. The backend-facing public inputs, subproof
  statement, backend verifier result, backend verifier receipt, certificate
  payload, final full-hidden proof payload, and subproof envelope carry and hash
  that bridge. The top transparent proof-bytes envelope repeats the same bridge
  digest from the full-hidden subproof envelope, and its wire version is bumped
  for the added field. The direct-creation proof-bytes boundary compares it with
  the transparent payload before the backend-gated verifier path. The final
  replay payload digest and proof-family binding digest are intentionally not
  placed inside the full-hidden subproof, because both are computed from proof
  bytes that contain that subproof.
  The generic direct creation proof payload carries one post-proof proof-family
  public binding digest: it must be zero for non-transparent proof kinds, and for
  transparent descent it hashes the transparent public view, the pre-proof
  statement bridge, and the same payload/proof-byte/subproof/local-bridge/check
  handles. The selected-view bridge compares both the statement bridge and that
  digest before the backend gate, so the outer creation payload no longer sees
  the bridge only through the opaque replay payload digest.
  The proof-carrying transparent descent boundary calls the stricter proof-suite
  envelope check, so the generic outer-envelope parser is not the bridge
  acceptance point for a proof body that omits the full-hidden segment or carries
  only a statement-only full-hidden envelope with a zero local bridge digest. The
  standalone public opening verifier and named proof-suite binding verifier both
  enforce that nonzero local bridge requirement. The standalone public opening
  verifier remains a rejection-only shape checker: it has no leaf/evidence/
  certificate/binding inputs, so it cannot recompute the direct statement bridge
  and is not an account-creation acceptance endpoint even after future aggregate
  production-soundness discharge. The proof-carrying account-creation boundary is
  the accepting endpoint because it recomputes the direct statement bridge before
  the backend gate. The full-hidden constraint-plan bit alone cannot make the
  trace-to-binding relation accepting, even though the reference trace-to-binding
  bridge status is now true and digest-bound. The trace-to-binding layer,
  production obligations, and proof-family capability also digest-bind the
  public-view-only verifier as rejection-only and the proof-bytes
  account-creation boundary as required, plus the backend certificate
  payload-only verifier as rejection-only and the backend certificate proof
  bundle endpoint as required.
  The obligation
  bitmap remains zero. The backend acceptance bit and aggregate obligation-proof
  bit remain false while the backend proof contract is unimplemented.
- `validate_path2_transparent_full_relation_backend_certificate_payload_v1`
  is a payload-only legacy guard and always rejects after shape validation; it
  cannot become an accepting path because it has no backend proof bytes.
- `verify_path2_transparent_full_relation_backend_certificate_payload_with_proof_v1`
  is the proof-carrying acceptance boundary. It first decodes a typed backend
  verifier receipt, checks that the receipt digest equals
  `backend_verifier_receipt_digest`, checks that the receipt's verifier-result
  digest equals `backend_verifier_result_digest`, checks that the receipt's
  public-view digest, creation binding digest, all six creation role handles,
  native role-root commitment, masked-projection digest, and trace-binding
  context digest match the
  certificate payload, checks that the receipt's transparent transcript digest
  and trace schedule match the certificate payload, and checks that the
  receipt's backend proof payload digest equals
  `backend_verified_proof_payload_digest`.
  It then
  remains fail-closed while `proof_payload_contract_implemented = false` or
  `all_obligation_proofs_verified = false` or
  `backend_acceptance_proof_verified = false`, and a future accepting path must
  also have the hidden/trace witness-check verified bits and both explicit
  trace/constraint-root bridge verified bits true. The aggregate
  `all_obligation_proofs_verified` bit must agree with the full per-obligation
  bitmap, and each verified obligation bit must carry a nonzero proof digest, so
  a well-formed payload cannot be mistaken for an inhabitant of
  `SelectedTransparentFullRelationBackendCertificate`.
- `path2_transparent_full_hidden_relation_proof_payload_v1` consumes a typed
  `Path2TransparentFullRelationBackendCertificateProofBundleV1` containing both
  certificate payload bytes and backend verifier receipt bytes, then calls the
  proof-carrying acceptance boundary. Bare certificate payload bytes are rejected
  as the wrong proof-payload shape. The bundle, embedded certificate payload,
  backend verifier result, and backend verifier receipt schemas are bound by the
  backend payload contract and verifier key, so neither the accepting proof-byte
  shape nor the embedded backend wire language can drift independently of the
  verifier configuration. The bundle layer also checks that the receipt's
  verifier-result digest, verified proof-payload digest, and no-export flag
  agree with the certificate fields, and rejects carried witness-check bytes
  unless the certificate payload marks those witness checks verified, before the
  proof-carrying verifier is
  reached.
  Raw backend verifier-result construction/digesting and backend
  verifier-receipt construction/digesting/byte decoding/encoding are crate-local
  bridge plumbing; public callers must use the check-carrying certificate
  payload/bundle constructors or proof-carrying verifiers.
- The subproof statement, backend verifier result, backend verifier receipt, and
  certificate payload carry the public trace-to-binding anchors explicitly:
  proof-system id, relation id, column profile, leaf context,
  creation-proof binding digest, matrix-domain transcript, masked-projection
  digest, aggregation context, all six creation role handles
  (`packed_witness_commitment`, `randomizer_commitment`,
  `pad_source_commitment`, `opening_tuple_commitment`,
  `projection_relation_commitment`, and `gauge_commitment`), and native
  role-root commitment. The backend
  verifier result, verifier receipt, and certificate payload also repeat the
  transparent transcript digest and trace schedule. These fields are hashed
  into their respective carriers and checked across the statement/payload and
  payload/receipt boundaries, so the
  future backend proof cannot switch the relation language, column policy,
  transparent transcript, trace schedule, creation binding, individual role
  handle, or role-root target while preserving only an opaque
  `trace_binding_context_digest`. The public transparent view, transparent
  transcript digest, and trace-binding context digest now also include
  `native_role_root_commitment`, giving the sampled trace Merkle roots the same
  role-root target as the certificate carrier. The selected-view bridge and
  commitment API verifier directly compare the public-view role-root field
  against the creation binding's native role root before the fail-closed backend
  boundary. The top-level transparent proof payload and typed proof-bytes
  envelope also repeat the leaf/matrix/aggregation source anchors, trace and
  constraint commitment roots, transparent transcript digest, and trace
  schedule fields alongside the full transparent public-view digest, creation
  binding digest, native role-root commitment, masked-projection digest, and
  trace-binding context digest. The full-hidden public-input digest also
  recomputes the native role-root commitment from the carried packed-witness,
  randomizer, pad-source, opening-tuple, projection-relation, and gauge
  commitments, so public-input decoding cannot preserve the public-view role
  root while rebinding one role commitment. The query-opening manifest carries
  the public-view/context anchors, `query_count`, and per-query records. Their
  digest frames hash those anchors, so accepted sampled openings/proof bytes
  are tied to exactly the public view, transcript, schedule, and binding target
  used by the trace-to-binding bridge.
  The canonical full-hidden public-input constructor also checks the creation
  binding's account, slot, verifier, state-root, lifecycle, opening-tuple, and
  creation-statement fields against the leaf before it emits the public-input
  digest, so the canonical bridge cannot pair a rebound creation-binding digest
  with the old leaf statement.
  The hidden-relation witness-check boundary repeats the slot and hidden-source
  statement comparison against the creation binding, so decoded public inputs
  with a rebound slot or statement digest fail before any trace-origin receipt
  can be minted. The hidden-relation and trace-origin witness-check receipts
  also carry and hash that slot and hidden-source statement digest, and local
  bridge/certificate assembly rejects mixed receipts whose carried slot or
  statement context differs.
  The public opening verifier's full-hidden subproof envelope shape check now
  derives those same anchors from the selected public view and rejects a
  neighboring-view subproof envelope before the fail-closed production
  obligation gate.
  The proof-suite full-hidden binding helper also compares the decoded
  proof-bytes envelope's relation id, active relation-layout digest, public-view
  digest, leaf/matrix/aggregation anchors, slot, hidden-source statement digest,
  trace roots, creation binding digest, native role root, masked-projection
  digest, trace-binding context digest, transcript digest, and schedule fields against
  the canonical full-hidden public inputs before it verifies the embedded
  subproof. The public suite-shape verifier also compares the proof-bytes
  envelope slot/source pair against the decoded inner full-hidden subproof
  envelope before the backend gate.
  The backend-facing full-hidden public inputs, subproof statement, backend
  verifier result, backend verifier receipt, certificate payload, hidden/trace
  witness-check receipts, full-hidden proof payload, and subproof envelope carry
  and hash the same slot, hidden-source statement digest, relation id, column
  profile, transparent transcript digest, trace schedule, public-view digest,
  creation binding digest, all six creation role handles, native role-root
  commitment, masked-projection digest, trace-binding context digest, and
  pre-proof direct-payload statement bridge digest.
  Certificate validation reconstructs the verifier result from those fields
  before accepting the carrier shape. The
  statement/result/receipt/certificate/proof-payload/subproof carriers and the
  hidden/trace witness-check receipts therefore pin the future backend extractor
  to the verifier-accepted opening view, relation, column policy, transparent
  transcript, trace schedule, backend statement, and binding context.
  The final full-hidden proof payload and subproof envelope additionally repeat
  those anchors from the statement/public inputs. The subproof envelope also
  exposes the local trace-binding proof digest to the public envelope check:
  today's disabled statement-only envelope rejects any forged nonzero value,
  rejects a zero hidden-source statement digest, and compares the envelope
  slot/source pair against a decoded statement-only or typed proof-payload
  carrier; a checked typed proof-payload envelope must repeat the digest already
  checked in the proof payload/certificate path and then pass the strict public
  proof-payload verifier before it reaches the backend-unimplemented gate. The
  outer subproof envelope, backend
  verifier receipt, and final proof payload also repeat the hidden-relation and
  trace-origin witness-check receipt digests, trace roots, trace-to-binding
  layer digest, and leaf/matrix/aggregation source anchors. The backend
  verifier receipt also repeats the backend
  statement digest, public-input digest, verifier-key digest, backend
  proof-system id, backend acceptance-verifier digest, and backend obligation-set
  digest, plus the local trace-binding proof digest, trace commitment root, and
  constraint commitment root, trace-to-binding layer digest, and
  leaf/matrix/aggregation source anchors. Bundle and proof-payload validation
  compare those fields against the embedded certificate payload before the
  backend acceptance gate.
- The production-obligations and proof-family capability records now expose this
  intermediate bridge state explicitly: the local trace-binding proof-payload
  shape check, public subproof-envelope check, canonical check-record carriage,
  full-hidden proof-suite binding check, nonzero local bridge digest check, and
  direct-payload statement bridge check are true and digest-bound; the reference
  certificate bridge verifier bit is true and digest-bound as local-shape
  evidence. A separate public certificate proof verifier bit is false and
  digest-bound, so the reference bundle cannot satisfy production discharge even
  if every local bridge shape check is present.
  The full hidden relation constraints, full backend certificate,
  `p2_sis_extractor`, hidden range extractor, external coded-SIS review, and
  aggregate production-soundness discharge remain false.
  The reference proof-carrying certificate path now treats the carried
  witness-check records as a local trace-binding bridge shape only: it consumes
  the full certificate proof bundle rather than raw receipt bytes, verifies that
  the embedded certificate payload matches the caller payload, then verifies the
  records, receipt, and recomputed local bridge digest. The public certificate
  proof verifier and public subproof-envelope verifier still reject this
  disabled-backend reference bundle with the unimplemented-backend error; only
  the backend-gated shape helper may inspect it for construction/reference tests
  before the route fails closed at the remaining full-hidden/backend soundness
  gate.
  The hidden-relation and trace-origin witness-check receipts additionally carry
  and hash the proof-system id, trace-to-binding layer digest,
  leaf/matrix/aggregation source anchors, creation binding digest, native
  role-root commitment, masked-projection digest, transparent transcript digest,
  trace roots, trace schedule, and trace-binding context digest; certificate
  assembly requires those fields to match the statement, not merely the opaque
  public-input digest.

## Relation To C1/C2/C4/C6

The backend must prove the full hidden-opening relation against the canonical
public input frame:

```text
Path2TransparentFullHiddenRelationPublicInputsV1
```

At minimum, this includes:

- C1: `pad_seed` opens to the accepted leaf `pad_seed_digest`;
- C2: masks are derived from the pad-seed/slot PRF;
- C4: account projection and pad seed repack to
  `packed_witness_commitment`;
- C5/C7: the sampled masked-share seam and digit/range relation are consistent
  with the same extracted trace;
- C6: pad/projection/opening role commitments in
  `Path2CreationProofBindingV1` are opened by the extracted trace.

The current core negative theorems show why C1/C2/C4/C6 cannot be inferred from
sampled masked-share acceptance or transcript co-location alone.

## Consequence

Blocker 1 now has a precise fork:

```text
negative for current core:
  current_transparent_route_blocker1_negative_witness
  current_masked_share_core_acceptance_does_not_imply_full_hidden_opening
  digest_colocation_does_not_imply_trace_role_binding
  path2_current_core_negative_theorem
  path2_current_view_binding_negative_theorem

positive if backend certificate exists:
  selected_transparent_full_relation_backend_certificate_implies_account_admission
```

So the remaining work is no longer "hope for a hidden range extractor." It is
exactly one of:

1. instantiate the full-relation backend certificate for the transparent route;
2. switch to another proof family, such as MITH, that supplies an equivalent
   full-witness certificate;
3. accept the negative result for the current route and redesign the account
   admission object.

## Verification

Checked locally:

```text
lake env lean Descender/Path2NativeBoundary.lean
```

## Non-Claims

- This does not instantiate the certificate.
- This does not prove the current Rust core sound.
- This does not prove byte-size feasibility.
- This does not discharge hidden range extraction; it shows blocker 1 must
  clear first through a full-relation backend certificate.
