# Path 2 Native Full Signature Obligation Burn-Down

Date: 2026-06-02

Scope: active deployment path named by Rust production preflight: Path 2 native
/ reviewed Fq12 active-lane projection-carrier route.

## Current Position

`Descender/Path2NativeAccountDesign.lean` now contains the active full-signature
boundary:

- `NativeHashBudgetBoundary`
- `NativeAcceptedSpendSoundnessBoundary`
- `NativeFullSignatureSecurityBoundary`
- `NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound`

The latest reduction batch adds standalone active-path wrappers:

- `Descender/Path2FullSpendGame.lean` for the direct accepted-spend and
  full-forgery lifecycle boundary.
- `Descender/Path2HashBudget.lean` for the executable direct query-budget mirror
  and `NativeHashBudgetBoundary.queryBudgetPriced` adapter.
- `Descender/Path2HashDomainAndMultiTarget.lean` for the active hash-domain
  registry and widened multi-target/account-horizon certificate.
- `Descender/Path2NoExport.lean` for public-output allowlist/denylist closure
  over active accept/reject/storage surfaces.
- `Descender/Path2RoleRootBinding.lean`,
  `Descender/Path2AssemblyConstruction.lean`, and
  `Descender/Path2CompatibilitySoundness.lean` for non-aggregate P2FS-001
  decomposition.
- `Descender/Path2RoleRootConcrete.lean`,
  `Descender/Path2AssemblyNativeValidity.lean`, and
  `Descender/Path2CompatibilityConcrete.lean` for field-level P2FS-001
  residuals.
- `Descender/Path2FullOpeningFieldEvidence.lean`,
  `Descender/Path2FullOpeningFieldEvidencePredicates.lean`, and
  `Descender/Path2FullOpeningFieldEvidenceByteBridge.lean` for the
  pre-aggregate field-evidence mirror, sidecar semantic bridge, byte
  parser/frame hook certificate, and byte-source-to-sidecar constructor.
- `Descender/Path2FullOpeningFieldEvidenceComposition.lean` for packaging
  byte-backed field evidence into the exact active reduction facts used by
  role-root, assembly, and compatibility reductions.
- `Descender/Path2FullOpeningSourceBridge.lean` for non-circular
  binding-or-break assembly/compatibility source obligations that derive
  extraction-certificate soundness without using the aggregate bridge as a
  premise.
- `Descender/Path2FullOpeningSourceConcrete.lean` for packaging the
  deployable non-aggregate sources: assembly availability, field-level
  assembly binding, deterministic assembly construction,
  assembled-representative native validity, compatibility availability,
  field-level compatibility binding, and concrete same-representative
  extraction rules.
- `Descender/Path2FullOpeningFieldEvidenceSourceAdapters.lean` for projecting
  concrete assembly and compatibility field certificates from byte field
  evidence for an already-witnessed role-lift/assembly/compatibility chain.
- `Descender/Path2CheckedCoreByteBridge.lean` for byte/parser certificates that
  imply the checked-core native accept relation.
- `Descender/Path2NoExportConcrete.lean` for tagged accept/reject/storage
  serializer allowlist certificates.
- `Descender/Path2FullSpendConcrete.lean` for the concrete carrier around
  prevalidated slot leaves, admission, checked-core binding, consume-once state,
  and rollback/reorg breaks.

This is the active replacement for trying to complete the q12289 FS-core
full-game proof. The q12289 theorem remains useful as a reference shape, but it
does not certify the current deployment verifier.

2026-06-02 update: `Path2FullOpeningSourceConcrete.lean` now packages the
deployable non-aggregate theorem route as `ReviewedFq12DeployableHiddenSourceInputs`. The remaining P2FS-001 proof inputs are
no longer an abstract source-bridge instantiation; they are the concrete
role-root bridge, named assembly availability,
`ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, deterministic assembly
predicate facts, native-fiber/local/descent facts for the constructed
representative, compatibility availability,
`ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak`, and concrete
same-representative extraction rules. The compatibility-indexed field-evidence
byte source remains a separate Rust parser/digest bridge, not the generic
source of pre-compatibility assembly. The concrete source package also includes
strict-source adapters and field-or-break adapters: exact assembly and
compatibility binding verifier theorems are sufficient to instantiate the
binding-or-break bridge surface, while assembly and compatibility binding can
also be supplied through `ReviewedFq12AssemblyConcreteFieldCertificate` and
`SameRepresentativeConcreteFieldCertificate` field certificates. The active theorem route is
`reviewed_fq12_certificate_soundness_from_deployable_hidden_source_inputs`, over a single `ReviewedFq12DeployableHiddenSourceInputs` package:
assembly availability is supplied by
`ReviewedFq12AssemblyAvailabilityOrBreak`, assembly binding by
`ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, deterministic assembly
local facts by `ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak`,
native validity by `ReviewedFq12AssemblyConstructedNativeValidityOrBreak`,
compatibility availability by `ReviewedFq12CompatibilityAvailabilityOrBreak`,
compatibility binding by
`ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak`, and extraction by
`ReviewedFq12ConcreteCompatibilityExtractionOrBreak`.
`Path2FullOpeningFieldEvidenceSourceAdapters.lean` separately projects the
same concrete assembly and compatibility field certificates from byte field
evidence for an already-witnessed chain; this remains a compatibility-indexed
parser/digest adapter, not generic assembly availability. It now also names
`ReviewedFq12OpeningRoleLiftWitnessSource`,
`ReviewedFq12AssemblySourceRouteWitnessSource`,
`ReviewedFq12CompatibilitySourceRouteWitnessSource`,
`source_route_byte_source_assembly_field_binding_of_route_witness`,
`source_route_byte_source_compatibility_field_binding_of_route_witness`,
`source_route_byte_source_compatibility_extraction_of_route_witness`, and
`ReviewedFq12SourceRouteDeployableHiddenSourceInputs`, making the extra route
witnesses explicit: source-route binding requires the selected role-lift
certificate for the semantic opening plus assembly/compatibility witnesses
connecting the generic representative to the constructed representative.
Rust now also exposes `Path2ReviewedFq12FullOpeningFieldSourceRouteV1`, a
digest-bound source-route certificate over the same ordering: the assembly
certificate binds the full-opening fields, the compatibility certificate binds
the same representative fields, and the field evidence is only an adapter with
`aggregate_bridge_used_as_source = false`. The targeted regression now pins
this route before constructing the aggregate bridge and rejects aggregate-source
misuse, stale source-route digests, bad assembly field sources, and bad
compatibility field sources. The Rust production domain registry now also
includes the reviewed-Fq12 assembly, compatibility, field-evidence,
source-route, and aggregate-bridge frame domains, covered by the global
disjointness regression.
Lean now mirrors this Rust route with
`ReviewedFq12FullOpeningFieldSourceRouteCertificate`, byte-parser/source
structures, `source_route_byte_predicate_source_active_reduction_facts_or_named_break`,
and `source_route_byte_source_concrete_field_certificates_or_named_break`.
The byte bridge also adds
`field_evidence_byte_source_of_source_route_byte_source`, which lets the
stronger paired source-route theorem feed callers that only need the plain
field-evidence byte-source interface while retaining the route frame inside the
derived predicate.
It also exposes ordinary-boundary-break projections for combined, assembly-only,
and compatibility-only field certificates:
`source_route_byte_source_concrete_field_certificates_or_boundary_break`,
`source_route_byte_source_assembly_concrete_fields_or_boundary_break`, and
`source_route_byte_source_compatibility_concrete_fields_or_boundary_break`.
The route-aware local endpoints now also expose ordinary assembly predicate,
native-validity, and compatibility-extraction facts:
`source_route_byte_source_local_assembly_predicates_or_break`,
`source_route_byte_source_local_native_validity_or_break`, and
`source_route_byte_source_local_compatibility_extracts_or_break`. Source
adapters now package local assembly predicates and native validity into the
concrete-source certificate shapes with
`byte_source_concrete_local_assembly_predicates_or_boundary_break`,
`source_route_byte_source_concrete_local_assembly_predicates_or_boundary_break`,
`byte_source_concrete_native_validity_or_boundary_break`, and
`source_route_byte_source_concrete_native_validity_or_boundary_break`. The
plain byte-field source-adapter theorem
`byte_source_concrete_compatibility_extraction_or_boundary_break` and the
source-route boundary theorem
`source_route_byte_source_concrete_compatibility_extraction_or_boundary_break`
turn local compatibility extraction facts into the concrete compatibility
extraction certificate for the witnessed deterministic representative, or
export the ordinary boundary break. The witnessed-chain package
`ReviewedFq12FullOpeningFieldEvidenceWitnessedConcreteSourceCertificates`
groups the resulting concrete fields, named local assembly facts, native
validity, and concrete compatibility extraction certificate for one
already-witnessed role-lift/assembly/compatibility chain; the plain byte and
source-route constructors are
`byte_source_witnessed_concrete_source_certificates_or_boundary_break` and
`source_route_byte_source_witnessed_concrete_source_certificates_or_boundary_break`.
The source-route deployable package
`ReviewedFq12SourceRouteDeployableHiddenSourceInputs` now derives
`ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak` via
`source_route_byte_source_assembly_local_predicates_of_route_witness`,
`ReviewedFq12AssemblyConstructedNativeValidityOrBreak` via
`source_route_byte_source_assembly_native_validity_of_route_witness`, and
`ReviewedFq12ConcreteCompatibilityExtractionOrBreak` via
`source_route_byte_source_compatibility_extraction_of_route_witness`, so the
route plus local predicate source supplies source-route field bindings,
deterministic assembly local/native facts, and compatibility extraction while
role-root plus assembly/compatibility availability remain explicit.
`ReviewedFq12SourceRouteLocalInstantiationInputs` is now an intermediate
source-route package. The boundary-availability Lean-side package before the
Rust verifier/model theorem is
`ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs`, which
repackages base active-boundary assembly/compatibility availability and
concrete role-root field binding into the source-route endpoint. It still
requires the byte predicate source, local predicate source, and route witnesses
for the selected parser/frame/digest implementation.
`ReviewedFq12FactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs`
now splits those route witnesses into an opening-to-role-lift witness, a
constructed-assembly compatibility witness, and a compatibility
assembly-representative witness.
`SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource` now names the
selected Rust source-route byte-source decomposition contract, and
`ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs`
replaces the generic byte predicate source. The compact-local endpoint is
`ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs`, which also
replaces the free local predicate source with compact assembly
availability/local/native and compatibility availability/extraction-rules
records.
`ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs`
derives the opening-to-role-lift
witness from the role-root bridge plus
`ReviewedFq12CertificateLiftOpeningFunctionality`, so the remaining lift
blocker is functionality of the selected certificate lift relation rather than
an opaque witness source.
`ReviewedFq12SelectedRustConcreteLocalFunctionalLiftSourceRouteInputs` keeps
that functional-lift derivation and bundles the two compatibility-side route
witnesses into `ReviewedFq12CompatibilityRouteWitnessSource`.
`ReviewedFq12SelectedRustPackagedSourceRouteInputs` groups role-root/lift facts as
`ReviewedFq12RoleRootFunctionalLiftSource` and assembly/compatibility local
facts as `ReviewedFq12AssemblyCompatibilityLocalModelSource`.
This packaged endpoint is the active P2FS-001 closure target. The field/local
packaged record is only a convenience adapter that consumes already-named
concrete field/local assembly and compatibility source records and forgets
their redundant field-binding facts, because selected source-route bytes supply
those bindings. The Rust hidden-source semantics audit now digest-binds the
packaged endpoint plus the field/local adapter presence
with source-route local-instantiation, boundary-availability/concrete-role-root
source-route endpoint, factored boundary-availability/concrete-role-root
source-route endpoint, selected-Rust source-route byte-source endpoint,
selected-Rust compact-local endpoint, selected-Rust functional-lift endpoint,
selected-Rust witnessed functional-lift endpoint, selected-Rust packaged
endpoint, selected-Rust field/local packaged adapter,
concrete role-root field binding, certificate lift-opening functionality,
assembly representative digest lookup source, compatibility route witness
source, assembly/compatibility local-model source, field-evidence, source-route, and
source-route negative-control flags; the targeted Rust test now also rejects
removing the narrowed Lean endpoint flag, the factored Lean endpoint flag, the
selected-Rust source-route endpoint flag, the compact-local endpoint flag, or
the witnessed functional-lift endpoint flag, the packaged endpoint flag, the
field/local packaged adapter flag, the lift-functionality flag, the
assembly representative digest lookup flag, the compatibility route witness
source flag, or the assembly/compatibility local-model source flag. It remains
fail-closed because `extractor_theorem_instantiated` and
`semantics_bridge_complete` are still false. The production hidden-source
semantics gate and readiness report now carry the bridge-audit digest and
`semantics_bridge_complete` bit, so deployability is tied to the narrowed audit
surface and cannot pass merely by satisfying the acceptance-obligation bitmap.
The top-level production preflight also reports that audit as present while
keeping the bridge incomplete, so the hidden-source semantics blocker remains
visible in the active deployment surface.
That keeps the source-route frame as ordering evidence while the detailed
field-evidence binding remains the source of concrete field equalities. The
assembly projection is still post-compatibility only and must not be used as
generic pre-compatibility assembly availability or binding.

### P2FS-001 Packaged Active-Endpoint Blockers

The active target for hidden-source proof semantics is
`ReviewedFq12SelectedRustPackagedSourceRouteInputs`,
proved by
`reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs`.
The remaining blockers are deliberately model-facing, not alternate-path
trackers:

| Blocker | Why it remains real |
|---|---|
| Role-root/lift source | `ReviewedFq12RoleRootFunctionalLiftSource` still needs concrete role-root field binding plus selected-certificate lift-opening functionality. The selected source-route byte source is a single witnessed-chain parser/digest bridge; it does not prove global pairwise role-root/opening uniqueness or functionality of `reviewedFq12CertificateLiftsToOpening`. |
| Selected Rust source-route byte source | The implementation still needs the Lean/Rust theorem that the selected Rust field-evidence/source-route bytes instantiate `SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource`. |
| Assembly/compatibility local-model source | The endpoint consumes `ReviewedFq12AssemblyCompatibilityLocalModelSource`: assembly availability/local/native facts plus compatibility availability/extraction rules. It deliberately does not require the redundant field-binding facts carried by the older concrete field/local records, because selected source-route bytes supply binding. |
| Compatibility route witness source | `ReviewedFq12CompatibilityRouteWitnessSource` bundles constructed-assembly compatibility plus compatibility-side assembly representative lookup. New adapters `constructed_compatibility_witness_of_assembly_field_binding_and_local_model` and `compatibility_route_witness_source_of_assembly_field_binding_local_model_and_digest_lookup` show that direct `ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, the local-model source, and `ReviewedFq12AssemblyRepresentativeDigestLookupSource` are enough to build the full route-witness package. The digest lookup is still a real Lean/Rust bridge blocker: `FullNativeOpeningMatchesRepresentative` omits `representativeDigest`, so the model must prove that a generic compatibility representative has the digest of the available assembly certificate. Lean now narrows that to the selected Rust native representative digest frame: `ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource` supplies both representative-frame canonicality and selected assembly-frame recomputation from the 11-input `path2_reviewed_fq12_native_representative_digest_v1` shape, `shared_representative_frame_source_of_native_digest_frame_source` derives the shared frame, and `representative_canonical_digest_source_of_shared_representative_frame_source` gives the digest lookup source directly. `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs` is now the narrowest current P2FS-001 route: it derives constructed-assembly compatibility from independent direct assembly binding plus the local model and converts through the shared-frame adapter to the active packaged endpoint using direct assembly binding for compatibility-route construction. Selected source-route bytes remain the parser/frame bridge for an already witnessed active route and are not used to source the direct assembly-binding premise. The Rust fail-closed audit pins `lean_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_endpoint_present`, `rust_native_representative_digest_frame_source_present`, `lean_selected_rust_direct_assembly_shared_representative_frame_packaged_source_route_endpoint_present`, `rust_shared_representative_frame_source_present`, and `rust_independent_direct_assembly_binding_source_present`; direct binding is backed by `path2_reviewed_fq12_assembly_binds_full_opening_fields_v1`, and regressions assert the valid assembly/full-opening pair binds and a local-section mutation fails before source-route acceptance. Direct Rust inspection found that `path2_reviewed_fq12_native_representative_digest_v1` canonically recomputes the digest from `native_boundary_digest` plus the role tuple, assembly certificate framing rejects a stale representative digest, field evidence recomputes the same digest, and compatibility/source-route frames carry the assembly digest. Negative controls now mutate only `assembly_cert.representative_digest` while leaving role fields unchanged, mutate only the source-route `representative_digest` and require verifier reconstruction to reject, and independently mutate each representative-digest input to check that every active boundary/role digest affects `path2_reviewed_fq12_native_representative_digest_v1`. This points to instantiating the native digest-frame source, not a protocol redesign, unless some non-inspected production path bypasses these constructors. Selected source-route byte binding remains compatibility-indexed and cannot supply this direct pre-compatibility assembly binding without circularity. |

## Redesign-Risk Priority Order

This queue is ordered by likelihood of exposing a real issue that forces a
parameter or route redesign, not by theorem dependency order.

| Priority | Obligations | Why this comes first | Current blue-team result | Next move |
|---|---|---|---|---|
| 1 | P2FS-008, P2FS-007 | The old one-bit active multi-target horizon invalidated deployment security even if all structural proofs closed. | Started 2026-06-02. Re-ran `scripts/research/qrom_multitarget_margin.py`; the old artifact failed closed above `log2(T)=1`. Redesign route chosen 2026-06-02: auxiliary Fq12 action bits. The first closure target keeps 26 compact repetitions, raises the algebraic challenge frame to 23 bytes with mask `0x3f`, keeps 4 hardening bytes, and prices 182 algebraic / 214 total challenge bits for `log2(T)=32`. Rust/Lean update in progress: QROM-discounted transcript term `182`, strict deployment margin `53`, active aggregate horizon `32`, over-budget negative control `54`, transaction signature `258`, and transaction plus hidden-source proof `290`. | Finish implementing and proving the auxiliary-Fq12 widened challenge budget. |
| 2 | P2FS-012, P2FS-002, P2FS-005, P2FS-006 | A byte-frame or implementation mismatch can certify the wrong verifier and can also invalidate the QROM/domain model. | Started 2026-06-02. Checked-core is now narrowed by `SelectedRustCheckedCoreByteSource` in `Path2CheckedCoreByteBridge.lean`, which decomposes the selected Rust acceptor into the exact Lean byte-certificate fields. Hash domains are now narrowed by `ActiveHashByteDomainInstantiation` in `Path2HashDomainAndMultiTarget.lean`, which names the concrete byte-domain-to-active-tag bridge. Targeted Rust tests passed for canonical proof rejection, shared assembly-frame derivation, proof-generation/expected-syndrome agreement, active native domain uniqueness, and global domain/schema disjointness. | Instantiate `SelectedRustCheckedCoreByteSource` with the concrete Rust parser/frame/hash functions and instantiate `ActiveHashByteDomainInstantiation` from the Rust registry, then continue this item with hidden-source frame bytes and parser canonicality. |
| 3 | P2FS-001 | Hidden-source role-root/full-opening binding is the deepest cryptographic assumption and a plausible route-level redesign trigger. | Started in this ordering pass. Interface narrowed to `ReviewedFq12SelectedRustPackagedSourceRouteInputs`; concrete Rust/model instantiation open. Rechecked the active field-evidence/source-route shape: Lean keeps the source route compatibility-indexed and Rust rejects aggregate-source use; full-opening role-binding and role-root rebinding regressions pass. The field/local packaged theorem remains an adapter, not the active target. The latest Lean reduction adds a direct-assembly native-digest-frame wrapper that derives constructed-assembly compatibility from independent direct assembly binding plus the local model and derives both representative-frame and assembly-frame canonicality from the 11-input selected Rust digest frame. Rust already exposes the independent direct assembly field-binding helper and fail-closed audit flags. | Instantiate `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs` from role-root/lift source, selected Rust source-route byte source, independent direct assembly field binding, local model, and native representative digest-frame source. |
| 4 | P2FS-003, P2FS-013 | Lifecycle semantics can force redesign if blockchain success does not match accepted-spend semantics. | Started 2026-06-02. `Path2FullSpendConcrete.lean` now has `ConcreteFullSignatureExecutionSource`, a non-tautological production execution source requiring full-signature success to imply concrete verifier acceptance or a named break. Production EUF-CMA/blockchain binding remains open. | Instantiate `ConcreteFullSignatureExecutionSource` from production account admission, slot consumption, rollback/reorg policy, verifier routing, and full-forgery game. |
| 5 | P2FS-009, P2FS-010, P2FS-011 | Keygen/signer/completeness can force productization redesign, but the proof route can be evaluated after the verifier and budget surfaces are known. | Productization blockers intentionally remain active. | Model keygen/signing/completeness over the selected verifier after signer/verifier pipeline, CT, and storage gates are retired. |
| 6 | P2FS-004 | No-export is deployment-critical, but less likely than the above to force algebraic parameter redesign. | Lean serializer allowlist bridge exists; Rust binding open. | Bind all concrete accept/reject/storage serializers to the tagged Lean allowlist. |

## Active Obligations

| ID | Boundary field | Current status | Next proof move | Redesign trigger |
|---|---|---|---|---|
| P2FS-001 | `acceptedSpendSoundness.hiddenSourceProofSemanticsClosed` | Interface closed; concrete instantiation narrowed to non-aggregate role-root, assembly, compatibility, field-evidence, source-route, and compact local-source layers. Rust has pre-aggregate field-evidence/source-route frames plus negative controls for aggregate-source misuse, stale digests, route-source misuse, and role-root/full-opening/representative rebinding. Lean now exposes `ReviewedFq12SelectedRustPackagedSourceRouteInputs` and theorem `reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs`, packaging role-root/lift facts, selected Rust source-route bytes, assembly/compatibility local-model facts, and compatibility-side route witnesses as explicit source packages. It also exposes `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs`, which derives compatibility-route witnesses from independent direct assembly binding, the local model, and the selected Rust 11-input native representative digest frame. The representative/canonicality issue is now narrowed to instantiating `ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource` against `path2_reviewed_fq12_native_representative_digest_v1`. Selected route bytes cannot be recycled as direct assembly binding without circularity; Rust now marks the independent direct assembly field-binding and native digest-frame sources present in the fail-closed audit. The shared-frame and field/local packaged theorems are tracked as convenience adapters only. `extractor_theorem_instantiated` remains false, so the Rust bridge audit and deployment preflight stay fail-closed. | Instantiate `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs` from the Rust verifier/model: role-root/lift source; selected Rust source-route byte source; independent direct assembly field binding; assembly/compatibility local-model source; and native representative digest-frame source for `path2_reviewed_fq12_native_representative_digest_v1`. Use the aggregate verifier object only as a downstream audit bridge, not as the source of assembly soundness. Track operational private-fiber digests under P2FS-003/P2FS-010/P2FS-004. | Redesign to Option 2 private lifecycle root if any operational private-fiber digest becomes verifier-facing, affects accepted hidden-source semantics, or is needed to prove lifecycle/signing-oracle/no-export. |

| P2FS-002 | `acceptedSpendSoundness.checkedCoreCompositionClosed` | Byte bridge stated; concrete Lean/Rust parser/hash instantiation open. Lean now has `Path2NativeCheckedCoreAcceptV1`, `Path2NativeCheckedCoreAcceptV1Bridge`, a theorem that accepted native checked-core evidence instantiates `CheckedCoreSpendReduction.checkedCoreAccepts`, a frame-correspondence theorem tying accepted native evidence to both the assembled native frame and Lean reduction frame, and named checked-core equation/frame/domain/parser breaks. `Path2CheckedCoreByteBridge.lean` adds parser hooks, canonical proof-byte certificates, decoded commitment and response-syndrome records, frame-byte equality, hash/syndrome equality, and a theorem composing a byte certificate into `checkedCoreAccepts`. It now also exposes `SelectedRustCheckedCoreByteSource`, the selected-implementation contract for decomposing `path2_native_checked_core_accept_v1` into those byte-certificate fields instead of treating Rust acceptance as opaque. Rust has `Path2NativeCheckedCoreFrameV1`, `path2_native_checked_core_accept_v1`, and `assemble_path2_native_boundary_v1`; targeted tests show frame fields affect the challenge, invalid checked-core context is rejected, legacy private handles are excluded, boundary assembly derives one public surface, independent digest inputs are rejected, proof-commitment mismatch is rejected, syndrome mismatch is rejected, active proof byte length is exactly `PATH2_NATIVE_CHECKED_CORE_PROOF_BYTES`, and short/noncanonical/commitment-mismatched proof bytes reject before acceptance. The latest pass also reran canonical proof rejection, assembly-frame derivation, and proof-generation/expected-syndrome checks. | Instantiate `SelectedRustCheckedCoreByteSource` against concrete Rust bytes/parser/hash behavior; prove `Path2NativeCheckedCoreFrameV1` byte/frame correspondence to `CheckedCoreSpendReduction.frameEncoder`; discharge checked-core equation soundness or export final named breaks. | Redesign if the concrete checked-core verifier can accept proof bytes whose frame is not assembled from the same native boundary, message digest, state digest, and proof commitment consumed by the spend predicate. |
| P2FS-003 | `acceptedSpendSoundness.lifecycleReplayClosed` | Full-spend theorem boundary and concrete carrier stated; production game binding open. Rust tests cover reservation monotonicity, reserve/sign/consume, stale reservation rejection, verify-and-consume replay rejection, spent-ledger stale-wallet replay blocking, reorg revealed-signature burns, public-only lifecycle/spent-ledger serialization, prevalidated-slot consume-once, admission namespace duplicate rejection, and selected-projection replay rejection. `Descender/Path2FullSpendGame.lean` defines `AcceptedSpendConclusion`, `account_creation_prevalidated_slot_authorization_or_break`, and a full-forgery wrapper requiring the same prechallenge admitted leaf, checked-core tuple binding, and exact consume-once successor transition or a named break. `Path2FullSpendConcrete.lean` adds concrete carrier records for prevalidated leaves, creation evidence, admission records, direct certificates, signature bodies, consume state, checked-core binding, consume-once transition, rollback/reorg breaks, and concrete accepted-spend soundness. It now also provides `ConcreteFullSignatureExecutionSource` so production execution success can be modeled independently from verifier acceptance while still proving the accepted-spend lift. | Instantiate `ConcreteFullSignatureExecutionSource` and bind `Path2FullSpendConcrete` to the real one-use slot lifecycle, account-creation/admission registry, spend verifier, state transition, and rollback/reorg semantics; prove accepted forgery maps to the same prechallenge admitted leaf and exact consume-once transition. | Redesign if target-chain lifecycle semantics can make a spent/revealed/burned/rollback-ambiguous slot spendable again without a named break. |
| P2FS-004 | `acceptedSpendSoundness.noExportClosed` | Lean no-export wrapper and concrete serializer allowlist bridge added; Rust serializer binding open. Lean has `FiberNoExportSoundness`, `PublicOutputNoExport`, `NoExportSoundness`, no-export-or-break theorem hooks, `Path2NoExport.ActiveNativeOutputConformance`, and `Path2NoExportConcrete.ActiveSerializerAllowlistCertificate`, which reduces tagged accept/reject/storage public-output frames to `ActiveNativeNoExportClosure` and `NativeAcceptedSpendSoundnessBoundary.noExportClosed`. Rust tests cover public native account/spend view allowlists, digest-only proof surfaces, selected-material helper absence from public APIs, sanitized creation-replay rejection paths, no selected material in checked account-state records, no selected material in consensus-carrier preflight reports, and selected-material helper exclusion from the production-reference facade. | Bind the tagged Lean accept/reject/storage frames to concrete Rust serializers, prove each public output conforms to the allowlist, and ensure every error/report/storage serializer excludes raw pad seed, opening tuple, unmasked projection, gauge, randomizer, trapdoor, private representative, and decoded hidden lanes. | Redesign if any verifier, serializer, helper API, or reject diagnostic must expose selected hidden material before final acceptance. |
| P2FS-005 | `hashBoundary.checkedCoreFrameDomainSeparated` | Lean active-domain certificate added; concrete byte-frame bridge still open. Rust `Path2NativeCheckedCoreFrameV1` uses an explicit domain-prefixed canonical byte frame; tests cover golden-vector roundtrip, invalid context rejection, mutation-induced challenge changes, exclusion of legacy private handles, and active native domain/schema uniqueness. `Path2HashDomainAndMultiTarget.lean` pins disjoint active tags, fills the checked-core domain-separated proposition for the native hash boundary, and now names `ActiveHashByteDomainInstantiation` for binding concrete Rust domain bytes to those active tags. | Instantiate `ActiveHashByteDomainInstantiation` for `PATH2_NATIVE_CHECKED_CORE_FRAME_DOMAIN` and the competing active domains, then tie `path2_native_checked_core_frame_v1_bytes` and challenge derivation to that byte-domain object. | Redesign if checked-core hash frame overlaps another domain, omits a spend-binding field, or becomes cyclic. |
| P2FS-006 | `hashBoundary.hiddenSourceFrameDomainSeparated` | Lean active-domain certificate added; concrete hidden-source frame bridge still open. Rust and docs include Path 2 domain/schema registry checks, direct-vs-batch mode separation, native hash/QROM review, and hidden-source/pad/carrier public transcript assumptions. `Path2HashDomainAndMultiTarget.lean` pins disjoint active tags, fills the hidden-source domain-separated proposition for the native hash boundary, and now names `ActiveHashByteDomainInstantiation` for the concrete byte-domain registry. | Instantiate `ActiveHashByteDomainInstantiation` for the hidden-source statement/proof/opening/pad/carrier/source-route domains, then tie those byte frames and direct/batch mode separation to actual hash inputs. | Redesign if any hidden-source frame shares a challenge/commitment domain ambiguously or if direct/batch domains can be replayed across modes. |
| P2FS-007 | `hashBoundary.queryBudgetPriced` | Lean budget mirror and native hash-boundary adapter added; final full-forgery reduction open. The widened route updates `path2_direct_query_budget_v1` to the selected auxiliary-Fq12 challenge accounting: 128-bit target, `2^16` quantum hash-query cap, `2^33` classical cap, 32-bit QROM quadratic loss, 182-bit transcript term after QROM loss, 53-bit strict deployment margin, and inactive `q_batch`. `Descender/Path2HashBudget.lean` mirrors those constants and packages the query pricing together with domain and multi-target props as `activeNativeHashBudgetBoundary`. | Prove the full-forgery reduction's numeric expression against the widened challenge, including every active direct-mode query symbol. | Redesign if any additional loss consumes the widened deployment margin or if the Fq12 action does not consume the full canonical challenge bytes. |
| P2FS-008 | `hashBoundary.multiTargetLossPriced` | `Path2HashDomainAndMultiTarget.lean` defines an explicit `ActiveMultiTargetHorizon`; the widened route now instantiates an active-lane horizon with account log2 = 32, spend log2 = 0, message log2 = 0, and aggregate target log2 = 32, with a 54-bit over-budget negative control. Rust deployment preflight is being updated so the QROM horizon no longer blocks before signer/verifier, CT, and storage gates. The selected route is auxiliary Fq12 action bits, not compact coefficient widening: first closure keeps 26 repetitions and targets `log2(T)=32`. | Prove multi-target pricing for the widened action, including the Fq12 action-collision floor, canonical byte-frame binding, and the `log2(T)=32` horizon. | Parameter redesign if account/network scale leaves insufficient margin after QROM and theorem composition losses. |
| P2FS-009 | `keygenDistributionModeled` | Productization blocker pinned; theorem open. Lean has `HonestCreationSoundness` and helper theorems for honest creation returning zero obstruction, local validity, and authorization-class compatibility. Rust scope KATs intentionally report the signer/keygen productization scope as not productized, with secret ownership, entropy, constructor, zeroization, and harness requirements pinned. Public prevalidation accepts valid projections and rejects invalid projections before private entropy. | Prove the production keygen/account-creation distribution emits native boundaries with valid private fibers and compatible authorization classes, except named sampler/RNG/constructor breaks; then flip productization only after the scope no longer reports the blocker. | Redesign if keygen can emit boundaries outside the native/Fq12 soundness relation or if secret ownership/zeroization cannot be made compatible with verifier-facing evidence. |
| P2FS-010 | `signingOracleModeled` | Signer evidence present; oracle theorem open. Rust tests pin public rejection cleanup, prevalidated signer execution reports, SHA-256 signer boundary evidence, CT evidence ledger separation, and no private reject output; the CT ledger still explicitly separates pinned evidence from open whole-signer timing claims. | Model the signing oracle over reserved live/prevalidated slots and prove every signing result is either a valid accepted spend candidate over the active frame or a public failure, with no secret-dependent output and named CT/RNG/sampler breaks. | Redesign if signer behavior cannot be represented without leaking/reusing hidden source material or if CT/productization blockers cannot be closed. |
| P2FS-011 | `completenessModeled` | Honest public-boundary flow and gate-separation evidence added; production completeness still blocked. Rust rejects randomizer mismatch and pins the private-fiber bridge scope. `Path2NativeSignerVerifierPipelineEvidenceV1` shows prevalidated honest construction reaches selected-verifier public-boundary acceptance, while production acceptance remains fail-closed and the smallest named gate is `PATH2_NATIVE_DEPLOYMENT_BLOCKER_SIGNER_VERIFIER_PIPELINE`; a new test confirms CT and storage remain separate blockers/non-claims rather than being hidden behind public-boundary acceptance. | Prove honest create plus honest prevalidated signing plus valid live state verifies under the selected Path 2 native/Fq12 production verifier and consumes the intended slot, after signer/verifier pipeline, CT, and storage gates are retired. | Redesign if the honest flow requires verifier inputs outside the active public boundary or if constructor/verifier frame equality cannot be made exact. |
| P2FS-012 | `verifierRelationMatchesImplementation` | Fail-closed implementation evidence plus honest public-boundary and gate-separation evidence present; byte-for-byte Lean bridge open. Rust preflight names the selected Path 2 native/Fq12 route and keeps deployment not ready. `path2_native_deployment_preflight_v1` reports public-boundary preflight, projection encoders, checked-core frame, public serializers, consensus carrier, verifier report, registry, hash review, and facade exports present. `Path2NativeSignerVerifierPipelineEvidenceV1` additionally confirms the honest prevalidated signer output reaches selected-verifier public-boundary acceptance, but production still reports `production_accepted = false` with signer/verifier pipeline, CT, and storage blockers, and tests pin those blockers as separate gates. Checked-core now has a Lean selected-Rust source contract, but the full verifier/pipeline byte-for-byte theorem remains open. | Instantiate the checked-core selected-Rust source, then add executable evidence bridges for signature bytes, carrier bytes, hidden-source frame domains, parser canonicality, public preflight, fail-closed production acceptance, and negative tests. | Redesign if the implementation path diverges from the Lean Path 2 native relation or if production acceptance requires fields outside the Lean public boundary. |
| P2FS-013 | `fullForgeryImpliesAcceptedSpendForgery` and `signatureForgerySuccess_le_acceptedSpend` | Abstract full-game lift and concrete carrier stated; production EUF-CMA/blockchain instantiation open. `Descender/Path2FullSpendGame.lean` states `account_creation_prevalidated_slot_authorization_or_break` and `fullForgeryImpliesAcceptedSpendForgery`. `Descender/Path2FullSpendConcrete.lean` instantiates those wrappers over native/account-shaped records for account creation evidence, prevalidated admission, direct certificates, checked-core binding, consume-once state, rollback/reorg breaks, and concrete verifier acceptance. It now adds `ConcreteFullSignatureExecutionSource` plus `concrete_fullExecutionImpliesAcceptedSpendForgery`, avoiding reliance on the tautological `ConcreteFullSignatureForgerySucceeds = ConcreteSpendVerificationAccepts` path for production closure. Local components exist, but no theorem currently maps arbitrary production signature forgeries into that execution source and numeric bound. | Bind `ConcreteFullSignatureExecutionSource` to the production EUF-CMA/blockchain spend-forgery game over account creation, signing, prevalidated slot admission, spend verification, state transition, and reorg/rollback handling; prove every successful forgery induces an accepted-spend event or one of the priced bad events, then supply `signatureForgerySuccess_le_acceptedSpend`. | Redesign if the blockchain notion of success includes outcomes not captured by accepted spend, or if postchallenge slot creation/rewrite cannot be excluded by the timing game. |

## Immediate Priority

By proof dependency, P2FS-001 remains the deepest active obligation. By
redesign-risk ordering, P2FS-008/P2FS-007 come first because the current budget
only supports the fail-closed `log2(T) <= 1` target horizon. Lean now proves the
two-bit aggregate horizon unpriced under the current budget, and Rust preflight
blocks production on the QROM target-horizon gate. Any broader deployment horizon
must redesign parameters or query caps before a deployable security theorem can
be claimed.

After the target-horizon decision, return to P2FS-001. The abstract theorem
shape is now in Lean. `Descender/Path2NativeBoundary.lean` defines
`ReviewedFq12HiddenSourcePayload`,
`ReviewedFq12HiddenSourceSemanticsCertificate`, and
`reviewed_fq12_hidden_source_semantics_closed_or_break`, which takes an
accepted reviewed-Fq12 hidden-source payload plus native boundary digest/slot
agreement and returns:

```text
valid_native_boundary_representative
  OR hidden_source_soundness_break
  OR checked_core_binding_break
  OR domain/qrom_break
```

The next concrete move is to supply the byte predicate source theorem for the
Rust reviewed-Fq12 verifier, generate the sidecar bridge from it, and use the
source/local predicate facts to instantiate
`ReviewedFq12AssemblyCertificateBindingOrBreak` and
`ReviewedFq12ExtractionCompatibilityCertificateBindingOrBreak`. The Rust
bridge audit keeps this distinction explicit:
the current implementation evidence supports binding/no-export claims, carries
the native boundary digest through a concrete payload binding, and emits a
selected hidden-relation extraction certificate for accepted tuples. Lean now
packages the active bridge as `ReviewedFq12ExtractionBridgeCertificate` and
derives the older `ReviewedFq12HiddenSourceSemanticsCertificate` from it.
The certificate-to-native-representative step is decomposed again: selected
evidence must first lift to a `FullNativeRepresentativeOpening`, then that
opening must assemble into the hidden representative. This confirms the core
constraint: the current selected certificate does not itself contain all native
private roles.
