import Descender.Path2DescentSIS

namespace Descender

/-!
# Path 2 Native Boundary Interface

This module names the proof-side objects for the Path 2 native-boundary route.
It is intentionally an interface: concrete vector commitments, transparent
proof soundness, zero-knowledge, and norm/range arguments remain separate
obligations.

The central object is a public boundary `b` that should be read as `B_c(h)` for
one hidden representative `h`. Public leaf, transparent-view, and spend
authorization data are projections of that boundary, not independent hidden
witness commitments.
-/

namespace Path2NativeBoundary

/-- Six active-lane residues shared with the Path 2 descent/SIS interface. -/
abbrev Lane := Fin 6

/-- Public account/admission context over which hidden representatives live. -/
structure Context where
  profileVersionDigest : Nat
  verifierVersionDigest : Nat
  accountId : Nat
  publicKeyIdentity : Nat
  slotId : Nat
  accountStateRootBefore : Nat
  lifecycleAnchor : Nat
  aggregationContextDigest : Nat

/-- Role handles that must be typed restrictions of one parent section root. -/
structure RoleHandles where
  packedWitnessHandle : Nat
  randomizerHandle : Nat
  padSourceHandle : Nat
  openingTupleHandle : Nat
  projectionRelationHandle : Nat

/-- Encoded public boundary object for the native Path 2 account-creation route. -/
structure Boundary where
  boundaryVersion : Nat
  context : Context
  parentSectionRoot : Nat
  roleProfileDigest : Nat
  roleHandles : RoleHandles
  maskedProjection : Lane -> Nat
  liveStatusMarker : Nat
  creationStatementDigest : Nat
  nativeBoundaryDigest : Nat

/--
Digest fields for the byte-level native projection assembly.

The boundary digest is acyclic: it is assembled from the non-transcript child
projection digests. The transcript digest is assembled afterward and binds the
resulting boundary digest.
-/
structure ProjectionDigestFields where
  contextDigest : Nat
  accountDigest : Nat
  spendDigest : Nat
  lifecycleDigest : Nat
  stateDigest : Nat
  nativeBoundaryDigest : Nat
  transcriptDigest : Nat

/-- Hidden representative space for one context. -/
structure HiddenRepresentative where
  nativeBoundaryDigest : Nat
  representativeDigest : Nat
  localSection : Nat
  packedResidues : Nat
  randomizer : Nat
  selectedAccountProjection : Nat
  padSource : Nat
  openingTuple : Nat
  projectionRelation : Nat
  lifecycleSlot : Nat
  traceWitness : Nat
  gauge : Nat

/--
Operational private-fiber digests used by signer/lifecycle machinery but not
by the hidden-source representative relation.

`randomizerDigest` here is the private-fiber operational randomizer digest; it
is distinct from `ReviewedFq12HiddenSourcePayload.randomizerCommitment`, which
is part of the reviewed hidden-source statement and is bound into the native
role root.
-/
structure OperationalPrivateFiberDigests where
  authorizationLocalDigest : Nat
  operationalRandomizerDigest : Nat
  walletRecoveryDigest : Nat

/-- Public view consumed by the two-view transparent payload theorem. -/
structure TransparentPublicView where
  proofSystemId : Nat
  relationId : Nat
  traceRows : Nat
  queryCount : Nat
  columnProfile : Nat
  leafContextDigest : Nat
  nativeBoundaryDigest : Nat
  matrixDomainTranscriptDigest : Nat
  maskedProjection : Lane -> Nat
  aggregationContextDigest : Nat
  traceCommitmentRoot : Nat
  constraintCommitmentRoot : Nat
  queryIndex : Nat
  transparentTranscriptDigest : Nat

/-- Extractor-side full trace object for the active native-boundary proof. -/
structure ExtractedTrace where
  parentSectionRoot : Nat
  roleProfileDigest : Nat
  roleHandles : RoleHandles
  traceWitness : Nat
  constraintWitness : Nat
  maskWitness : Nat
  rangeWitness : Nat
  blindingWitness : Nat

/-- Spend-facing projection of an admitted boundary. -/
structure SlotAuthorization where
  accountId : Nat
  publicKeyIdentity : Nat
  slotId : Nat
  authorizationDigest : Nat
  nativeBoundaryDigest : Nat

/--
Reviewed Fq12 hidden-source payload carried by the active deployment verifier.

The fields are theorem-level handles for the Rust reviewed-Fq12 statement,
carrier tuple, checked-core transcript, and proof bytes. Concrete byte parsers
and digest encoders live in the Rust/bridge layer; this structure is the Lean
semantic boundary consumed by the soundness interface below.
-/
structure ReviewedFq12HiddenSourcePayload where
  statementDigest : Nat
  creationStatementDigest : Nat
  creationProofBindingDigest : Nat
  randomizerCommitment : Nat
  openingCommitment : Nat
  padSeedDigest : Nat
  projectionCarrierDigest : Nat
  checkedCoreTranscriptDigest : Nat
  proofDigest : Nat
  nativeBoundaryDigest : Nat
  slotId : Nat

/--
Executable extraction certificate emitted by the reviewed-Fq12 bridge after an
accepted tuple has passed the payload-binding and selected hidden-relation
checks.

`creationStatementDigest` is the Path 2 account-creation statement digest;
`reviewedStatementDigest` is the legacy/reviewed Fq12 active-lane statement
digest. Keeping both names in the theorem boundary prevents statement
confusion.
-/
structure ReviewedFq12HiddenRelationExtractionCertificate where
  payloadBindingDigest : Nat
  nativeBoundaryDigest : Nat
  slotId : Nat
  creationStatementDigest : Nat
  reviewedStatementDigest : Nat
  openingCommitment : Nat
  padSeedDigest : Nat
  accountProjectionDigest : Nat
  accountWitnessOpeningFrameDigest : Nat
  carrierLanesDigest : Nat
  checkedCoreChallengeDigest : Nat
  checkedCoreWitnessSyndromeDigest : Nat
  checkedCoreResponseSyndromeDigest : Nat
  extractionCertificateDigest : Nat

/--
Full native representative opening needed to turn selected reviewed-Fq12
extraction evidence into an actual member of the native boundary fiber.

The selected extraction certificate alone exposes only selected projection and
checked-core digests.  This opening names the remaining native private roles
that `HiddenRepresentative` requires.
-/
structure FullNativeRepresentativeOpening where
  nativeBoundaryDigest : Nat
  nativeRoleRootCommitment : Nat
  localSectionDigest : Nat
  packedResiduesDigest : Nat
  randomizerDigest : Nat
  selectedAccountProjectionDigest : Nat
  padSourceDigest : Nat
  openingTupleDigest : Nat
  projectionRelationDigest : Nat
  lifecycleSlotDigest : Nat
  traceWitnessDigest : Nat
  gaugeDigest : Nat
  openingDigest : Nat

/--
Digest-level certificate that the selected reviewed-Fq12 extraction evidence
has been lifted into all roles of a full native representative opening.

This is the proof-side object that the Rust bridge now frames explicitly.  It
does not close P2FS-001 by itself; it makes the missing role-lift assumption
first class and checkable.
-/
structure ReviewedFq12FullNativeOpeningCertificate where
  payloadBindingDigest : Nat
  extractionCertificateDigest : Nat
  creationProofBindingDigest : Nat
  nativeRoleRootCommitment : Nat
  nativeBoundaryDigest : Nat
  slotId : Nat
  creationStatementDigest : Nat
  reviewedStatementDigest : Nat
  opening : FullNativeRepresentativeOpening
  roleLiftCertificateDigest : Nat

/--
Digest-level certificate that a full native opening has been assembled into a
native representative candidate.

This mirrors the Rust `Path2ReviewedFq12NativeRepresentativeAssemblyCertificateV1`
object.  It is deliberately separate from the role-lift certificate: role lift
binds the private role handles, while assembly binds those role coordinates to
one representative digest under the active native boundary.
-/
structure ReviewedFq12NativeRepresentativeAssemblyCertificate where
  roleLiftCertificateDigest : Nat
  payloadBindingDigest : Nat
  extractionCertificateDigest : Nat
  creationProofBindingDigest : Nat
  nativeBoundaryDigest : Nat
  slotId : Nat
  creationStatementDigest : Nat
  reviewedStatementDigest : Nat
  nativeRoleRootCommitment : Nat
  fullOpeningDigest : Nat
  localSectionDigest : Nat
  packedResiduesDigest : Nat
  randomizerDigest : Nat
  selectedAccountProjectionDigest : Nat
  padSourceDigest : Nat
  openingTupleDigest : Nat
  projectionRelationDigest : Nat
  lifecycleSlotDigest : Nat
  traceWitnessDigest : Nat
  gaugeDigest : Nat
  opening : FullNativeRepresentativeOpening
  representativeDigest : Nat
  assemblyCertificateDigest : Nat

/--
Digest-level certificate that the reviewed-Fq12 extraction relation and the
native representative assembly name the same hidden representative.

This is the final compatibility object for P2FS-001. It binds the selected
reviewed-Fq12 account projection, payload/certificate identifiers, full opening
digest, and assembled representative digest into one compatibility handle.
-/
structure ReviewedFq12ExtractionCompatibilityCertificate where
  assemblyCertificateDigest : Nat
  roleLiftCertificateDigest : Nat
  payloadBindingDigest : Nat
  extractionCertificateDigest : Nat
  nativeBoundaryDigest : Nat
  slotId : Nat
  creationStatementDigest : Nat
  reviewedStatementDigest : Nat
  fullOpeningDigest : Nat
  selectedAccountProjectionDigest : Nat
  representativeDigest : Nat
  compatibilityCertificateDigest : Nat

/--
Digest-level aggregate certificate for the active full-opening bridge.

This mirrors the Rust `Path2ReviewedFq12FullOpeningBridgeCertificateV1` object:
it does not replace the theorem-side `ReviewedFq12FullOpeningBridgeCertificate`
package, but gives the byte-level bridge a named object whose fields must agree
with the payload, extraction, role-lift, assembly, and compatibility
certificates.
-/
structure ReviewedFq12FullOpeningBridgeDigestCertificate where
  payloadBindingDigest : Nat
  extractionCertificateDigest : Nat
  creationProofBindingDigest : Nat
  roleLiftCertificateDigest : Nat
  assemblyCertificateDigest : Nat
  compatibilityCertificateDigest : Nat
  nativeBoundaryDigest : Nat
  slotId : Nat
  creationStatementDigest : Nat
  reviewedStatementDigest : Nat
  nativeRoleRootCommitment : Nat
  fullOpeningDigest : Nat
  selectedAccountProjectionDigest : Nat
  representativeDigest : Nat
  bridgeCertificateDigest : Nat

/--
Field-level binding contract for the byte-level aggregate full-opening bridge.

This is the Lean mirror of the Rust aggregate verifier: the bridge digest
certificate is meaningful only when it pins the same payload, extraction
certificate, role-lift certificate, assembly certificate, compatibility
certificate, native role root, full opening, selected account projection, and
representative digest.  The final `bridgeCertificateDigest` field is the byte
frame's own hash handle and is intentionally not expanded here.
-/
def ReviewedFq12FullOpeningBridgeDigestCertificateBinds
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (bridgeCert : ReviewedFq12FullOpeningBridgeDigestCertificate) :
    Prop :=
  bridgeCert.payloadBindingDigest = cert.payloadBindingDigest ∧
    bridgeCert.payloadBindingDigest = liftCert.payloadBindingDigest ∧
    bridgeCert.payloadBindingDigest = assemblyCert.payloadBindingDigest ∧
    bridgeCert.payloadBindingDigest = compatCert.payloadBindingDigest ∧
    bridgeCert.extractionCertificateDigest =
      cert.extractionCertificateDigest ∧
    bridgeCert.extractionCertificateDigest =
      liftCert.extractionCertificateDigest ∧
    bridgeCert.extractionCertificateDigest =
      assemblyCert.extractionCertificateDigest ∧
    bridgeCert.extractionCertificateDigest =
      compatCert.extractionCertificateDigest ∧
    bridgeCert.creationProofBindingDigest =
      payload.creationProofBindingDigest ∧
    bridgeCert.creationProofBindingDigest =
      liftCert.creationProofBindingDigest ∧
    bridgeCert.creationProofBindingDigest =
      assemblyCert.creationProofBindingDigest ∧
    bridgeCert.roleLiftCertificateDigest =
      liftCert.roleLiftCertificateDigest ∧
    bridgeCert.roleLiftCertificateDigest =
      assemblyCert.roleLiftCertificateDigest ∧
    bridgeCert.roleLiftCertificateDigest =
      compatCert.roleLiftCertificateDigest ∧
    bridgeCert.assemblyCertificateDigest =
      assemblyCert.assemblyCertificateDigest ∧
    bridgeCert.assemblyCertificateDigest =
      compatCert.assemblyCertificateDigest ∧
    bridgeCert.compatibilityCertificateDigest =
      compatCert.compatibilityCertificateDigest ∧
    bridgeCert.nativeBoundaryDigest = payload.nativeBoundaryDigest ∧
    bridgeCert.nativeBoundaryDigest = cert.nativeBoundaryDigest ∧
    bridgeCert.nativeBoundaryDigest = liftCert.nativeBoundaryDigest ∧
    bridgeCert.nativeBoundaryDigest =
      liftCert.opening.nativeBoundaryDigest ∧
    bridgeCert.nativeBoundaryDigest = assemblyCert.nativeBoundaryDigest ∧
    bridgeCert.nativeBoundaryDigest = compatCert.nativeBoundaryDigest ∧
    bridgeCert.slotId = payload.slotId ∧
    bridgeCert.slotId = cert.slotId ∧
    bridgeCert.slotId = liftCert.slotId ∧
    bridgeCert.slotId = assemblyCert.slotId ∧
    bridgeCert.slotId = compatCert.slotId ∧
    bridgeCert.creationStatementDigest =
      payload.creationStatementDigest ∧
    bridgeCert.creationStatementDigest =
      cert.creationStatementDigest ∧
    bridgeCert.creationStatementDigest =
      liftCert.creationStatementDigest ∧
    bridgeCert.creationStatementDigest =
      assemblyCert.creationStatementDigest ∧
    bridgeCert.creationStatementDigest =
      compatCert.creationStatementDigest ∧
    bridgeCert.reviewedStatementDigest = payload.statementDigest ∧
    bridgeCert.reviewedStatementDigest = cert.reviewedStatementDigest ∧
    bridgeCert.reviewedStatementDigest = liftCert.reviewedStatementDigest ∧
    bridgeCert.reviewedStatementDigest =
      assemblyCert.reviewedStatementDigest ∧
    bridgeCert.reviewedStatementDigest =
      compatCert.reviewedStatementDigest ∧
    bridgeCert.nativeRoleRootCommitment =
      liftCert.nativeRoleRootCommitment ∧
    bridgeCert.nativeRoleRootCommitment =
      liftCert.opening.nativeRoleRootCommitment ∧
    bridgeCert.nativeRoleRootCommitment =
      assemblyCert.nativeRoleRootCommitment ∧
    assemblyCert.opening = liftCert.opening ∧
    bridgeCert.fullOpeningDigest = liftCert.opening.openingDigest ∧
    bridgeCert.fullOpeningDigest = assemblyCert.fullOpeningDigest ∧
    bridgeCert.fullOpeningDigest =
      assemblyCert.opening.openingDigest ∧
    bridgeCert.fullOpeningDigest = compatCert.fullOpeningDigest ∧
    bridgeCert.selectedAccountProjectionDigest =
      cert.accountProjectionDigest ∧
    bridgeCert.selectedAccountProjectionDigest =
      liftCert.opening.selectedAccountProjectionDigest ∧
    bridgeCert.selectedAccountProjectionDigest =
      assemblyCert.selectedAccountProjectionDigest ∧
    bridgeCert.selectedAccountProjectionDigest =
      assemblyCert.opening.selectedAccountProjectionDigest ∧
    bridgeCert.selectedAccountProjectionDigest =
      compatCert.selectedAccountProjectionDigest ∧
    bridgeCert.representativeDigest = assemblyCert.representativeDigest ∧
    bridgeCert.representativeDigest = compatCert.representativeDigest

/-- Named break events for the native-boundary route. -/
inductive BreakEvent where
  | boundaryBindingBreak
  | descentSoundnessBreak
  | innerCommitmentBindingBreak
  | transparentProofSoundnessBreak
  | normOrRangeBreak
  | noExportBreak
  | qromOrDomainSeparationBreak
  | prevalidationLeafBindingBreak
  | prevalidationDataAvailabilityBreak
  | checkedCoreMessageBindingBreak
  | slotReplayOrRollbackBreak
  | reviewedFq12PayloadBindingBreak
  | reviewedFq12HiddenSourceSoundnessBreak
  | reviewedFq12DomainSeparationBreak
  deriving DecidableEq

/-- Role identifiers used by the native-boundary column profile. -/
inductive RoleId where
  | contextPublic
  | packedWitness
  | randomizer
  | padSource
  | openingTuple
  | projectionRelation
  | rangeWitness
  | blinding
  | constraint
  deriving DecidableEq

/-- Public-opening classes for two-view trace columns. -/
inductive VisibilityClass where
  | vcPublic
  | vcDerivedPublic
  | vcMaskedPublic
  | vcCommittedHidden
  | vcExtractorHidden
  | vcRangePrivate
  | vcBlindingPrivate
  | vcSimulatorOnly
  deriving DecidableEq

/-- Authentication mode used for a role under the parent trace root. -/
inductive RoleAuthenticationMode where
  | publicPathToMaskedLeaf
  | simulatorOnlyPath
  | extractorOnlyOpening
  | publicPathToRawLeaf
  | publicLinearCommitment
  deriving DecidableEq

/-- Norm/range representation mode selected by a column profile. -/
inductive NormRangeMode where
  | rawCenteredResidues
  | hybridBase128HiddenRoles
  | base128AllSemanticRoles
  | base16HiddenRoles
  deriving DecidableEq

/-- Extraction capability claimed by a candidate transparent proof family. -/
inductive TraceExtractionMode where
  | sampledOpeningOnly
  | proximityOnly
  | fullTraceExtractable
  | fullWitnessExtractable
  deriving DecidableEq

/-- Relation soundness capability claimed by a candidate proof family. -/
inductive RelationKnowledgeMode where
  | sampledRowChecksOnly
  | proximityWithoutWitness
  | proximityWithWitnessBridge
  | directRelationKnowledge
  | fullWitnessRelationKnowledge
  deriving DecidableEq

/-- Constraint classes in the native-boundary trace relation. -/
inductive TraceConstraintClass where
  | qFieldRow
  | base128RangeRow
  | maskConsistencyRow
  | roleRestrictionRow
  | openingTupleAccountBindingRow
  | checkedCoreResidueCompatibilityRow
  | lifecycleContextRow
  | proofSystemBookkeepingRow
  deriving DecidableEq

/-- The theorem-level row inventory for a native-boundary trace relation. -/
structure TraceRelationProfile where
  relationId : Nat
  rowCount : Nat
  constraintClass : Nat -> TraceConstraintClass

/-- A trace relation contains at least one row of the requested class. -/
def ConstraintClassCovered
    (relation : TraceRelationProfile)
    (constraintClass : TraceConstraintClass) : Prop :=
  exists row, row < relation.rowCount ∧
    relation.constraintClass row = constraintClass

/--
The active native-boundary relation must cover every mandatory row class. This
prevents relation-knowledge soundness from being claimed for only the old
66-row semantic core or for sampled/proximity checks that omit mask, role,
opening-tuple account binding, checked-core compatibility, lifecycle/context,
or proof-system bookkeeping rows.
-/
def TraceRelationCoversNativeBoundary
    (relation : TraceRelationProfile) : Prop :=
  ConstraintClassCovered relation TraceConstraintClass.qFieldRow ∧
    ConstraintClassCovered relation TraceConstraintClass.base128RangeRow ∧
      ConstraintClassCovered relation TraceConstraintClass.maskConsistencyRow ∧
        ConstraintClassCovered relation TraceConstraintClass.roleRestrictionRow ∧
          ConstraintClassCovered relation
              TraceConstraintClass.openingTupleAccountBindingRow ∧
            ConstraintClassCovered relation
                TraceConstraintClass.checkedCoreResidueCompatibilityRow ∧
              ConstraintClassCovered relation
                  TraceConstraintClass.lifecycleContextRow ∧
                ConstraintClassCovered relation
                  TraceConstraintClass.proofSystemBookkeepingRow

/-- Conservative selected working row floor for the active native-boundary relation. -/
def selectedNativeBoundaryTraceRelationRowFloor : Nat :=
  176

/--
The selected active profile must use the native-boundary relation scale, not a
one-row-per-class placeholder relation.
-/
def TraceRelationMeetsSelectedNativeBoundaryRowFloor
    (relation : TraceRelationProfile) : Prop :=
  selectedNativeBoundaryTraceRelationRowFloor <= relation.rowCount

/-- Count rows assigned to one constraint class in a relation profile. -/
def TraceRelationClassRowCount
    (relation : TraceRelationProfile)
    (constraintClass : TraceConstraintClass) : Nat :=
  (List.range relation.rowCount).filter
    (fun row => relation.constraintClass row = constraintClass) |>.length

/-- A relation profile has at least the selected number of rows for one class. -/
def TraceRelationClassRowsAtLeast
    (relation : TraceRelationProfile)
    (constraintClass : TraceConstraintClass)
    (minimumRows : Nat) : Prop :=
  minimumRows <= TraceRelationClassRowCount relation constraintClass

def selectedQFieldRelationRowsFloor : Nat := 26
def selectedBase128RangeRelationRowsFloor : Nat := 40
def selectedMaskConsistencyRelationRowsFloor : Nat := 58
def selectedRoleRestrictionRelationRowsFloor : Nat := 5
def selectedOpeningTupleAccountBindingRelationRowsFloor : Nat := 1
def selectedCheckedCoreResidueCompatibilityRelationRowsFloor : Nat := 1
def selectedLifecycleContextRelationRowsFloor : Nat := 2
def selectedProofSystemBookkeepingRelationRowsFloor : Nat := 2

/--
Conservative selected per-class floors for the active native-boundary relation.
These are lower bounds from the row inventory, not final exact row indices.
-/
def TraceRelationMeetsSelectedNativeBoundaryClassFloors
    (relation : TraceRelationProfile) : Prop :=
  TraceRelationClassRowsAtLeast relation
      TraceConstraintClass.qFieldRow selectedQFieldRelationRowsFloor ∧
    TraceRelationClassRowsAtLeast relation
        TraceConstraintClass.base128RangeRow
        selectedBase128RangeRelationRowsFloor ∧
      TraceRelationClassRowsAtLeast relation
          TraceConstraintClass.maskConsistencyRow
          selectedMaskConsistencyRelationRowsFloor ∧
        TraceRelationClassRowsAtLeast relation
            TraceConstraintClass.roleRestrictionRow
            selectedRoleRestrictionRelationRowsFloor ∧
          TraceRelationClassRowsAtLeast relation
              TraceConstraintClass.openingTupleAccountBindingRow
              selectedOpeningTupleAccountBindingRelationRowsFloor ∧
            TraceRelationClassRowsAtLeast relation
                TraceConstraintClass.checkedCoreResidueCompatibilityRow
                selectedCheckedCoreResidueCompatibilityRelationRowsFloor ∧
              TraceRelationClassRowsAtLeast relation
                  TraceConstraintClass.lifecycleContextRow
                  selectedLifecycleContextRelationRowsFloor ∧
                TraceRelationClassRowsAtLeast relation
                    TraceConstraintClass.proofSystemBookkeepingRow
                    selectedProofSystemBookkeepingRelationRowsFloor

/-- One role/column entry in a native-boundary column profile. -/
structure ColumnProfileEntry where
  roleId : RoleId
  columnSetId : Nat
  maskedColumnSetId : Nat
  visibilityClass : VisibilityClass
  authenticationMode : RoleAuthenticationMode
  maskDomainLabel : Nat
  rangePolicyId : Nat
  queryPolicyId : Nat
  restrictionRuleId : Nat

/-- The theorem-level column profile consumed by the two-view proof. -/
structure ColumnProfile where
  profileId : Nat
  entry : Nat -> ColumnProfileEntry
  relationProfile : TraceRelationProfile
  normRangeMode : NormRangeMode
  extractionMode : TraceExtractionMode
  relationKnowledgeMode : RelationKnowledgeMode
  queryOpeningPolicyId : Nat
  rejectionPolicyId : Nat

/-- Bound policy for blinding/noise coordinates in role commitments. -/
structure BlindingNoisePolicy where
  policyId : Nat
  padSourceBound : Nat
  projectionRelationBound : Nat
  openingTupleBound : Nat
  rangeAuxiliaryBound : Nat

/-- The selected extractor-side blinding/noise policy id for the active route. -/
def selectedBlindingNoisePolicyId : Nat :=
  1

/-- The selected centered bound used by the active private blinding policy. -/
def selectedBlindingNoiseCenteredBound : Nat :=
  2048

/-- The selected private role-matrix row band for active SIS role handles. -/
def selectedSisRoleMatrixRows : Nat :=
  64

/-- The selected range policy id for hidden base-128 semantic role digits. -/
def selectedHybridBase128SemanticRangePolicyId : Nat :=
  1

/-- The selected range policy id for private range-witness columns. -/
def selectedHybridBase128WitnessRangePolicyId : Nat :=
  2

/-- The active route uses one binding-compatible blinding/noise policy. -/
def UsesSelectedBlindingNoisePolicy
    (policy : BlindingNoisePolicy) : Prop :=
  policy.policyId = selectedBlindingNoisePolicyId ∧
    policy.padSourceBound = selectedBlindingNoiseCenteredBound ∧
      policy.projectionRelationBound = selectedBlindingNoiseCenteredBound ∧
        policy.openingTupleBound = selectedBlindingNoiseCenteredBound ∧
          policy.rangeAuxiliaryBound = selectedBlindingNoiseCenteredBound

/-- The public context projection from a native boundary. -/
def pContext (b : Boundary) : Context :=
  b.context

/-- The parent section root projection from a native boundary. -/
def pSectionRoot (b : Boundary) : Nat :=
  b.parentSectionRoot

/-- The role-profile projection from a native boundary. -/
def pRoleProfile (b : Boundary) : Nat :=
  b.roleProfileDigest

/-- The typed role-handle projection from a native boundary. -/
def pRoleHandles (b : Boundary) : RoleHandles :=
  b.roleHandles

/-- The public six-lane masked projection from a native boundary. -/
def pMaskedProjection (b : Boundary) : Lane -> Nat :=
  b.maskedProjection

/-- The creation statement digest projected from a native boundary. -/
def pStatement (b : Boundary) : Nat :=
  b.creationStatementDigest

/-- Acyclic byte-level boundary digest assembly over non-transcript projections. -/
def assembledNativeBoundaryDigest (d : ProjectionDigestFields) : Nat :=
  d.contextDigest +
    d.accountDigest +
      d.spendDigest +
        d.lifecycleDigest +
          d.stateDigest

/-- Transcript digest assembly after the boundary digest has been fixed. -/
def assembledTranscriptDigest (d : ProjectionDigestFields) : Nat :=
  d.nativeBoundaryDigest +
    d.contextDigest +
      d.accountDigest +
        d.spendDigest +
          d.lifecycleDigest +
            d.stateDigest

/-- The byte implementation's non-recursive assembly order. -/
structure AcyclicProjectionDigestAssembly (d : ProjectionDigestFields) where
  boundaryDigestEq : d.nativeBoundaryDigest = assembledNativeBoundaryDigest d
  transcriptDigestEq : d.transcriptDigest = assembledTranscriptDigest d

/-- Boundary assembly excludes the transcript digest, avoiding recursive bytes. -/
theorem nativeBoundaryDigest_acyclic
    {d : ProjectionDigestFields}
    (h : AcyclicProjectionDigestAssembly d) :
    d.nativeBoundaryDigest =
      d.contextDigest +
        d.accountDigest +
          d.spendDigest +
            d.lifecycleDigest +
              d.stateDigest :=
  h.boundaryDigestEq

/-- Transcript assembly binds the already-fixed native boundary digest. -/
theorem transcriptDigest_binds_boundary
    {d : ProjectionDigestFields}
    (h : AcyclicProjectionDigestAssembly d) :
    d.transcriptDigest =
      d.nativeBoundaryDigest +
        d.contextDigest +
          d.accountDigest +
            d.spendDigest +
              d.lifecycleDigest +
                d.stateDigest :=
  h.transcriptDigestEq

/-- Profile entries whose raw values must not be public authentication leaves. -/
def VisibilityRequiresPrivateAuthentication
    (visibilityClass : VisibilityClass) : Prop :=
  visibilityClass = VisibilityClass.vcCommittedHidden ∨
    visibilityClass = VisibilityClass.vcExtractorHidden ∨
      visibilityClass = VisibilityClass.vcRangePrivate ∨
        visibilityClass = VisibilityClass.vcBlindingPrivate ∨
          visibilityClass = VisibilityClass.vcSimulatorOnly

/-- Allowed role-authentication modes for private native-boundary columns. -/
def RoleAuthenticationModeAllowedForPrivate
    (authenticationMode : RoleAuthenticationMode) : Prop :=
  authenticationMode = RoleAuthenticationMode.publicPathToMaskedLeaf ∨
    authenticationMode = RoleAuthenticationMode.simulatorOnlyPath ∨
      authenticationMode = RoleAuthenticationMode.extractorOnlyOpening

/--
Private entries in the active column profile must be authenticated through
masked, simulator-only, or extractor-only role restrictions.  This rejects the
two dead downgrades found by the rank/hiding audit: public raw hidden leaves and
public linear role commitments for private roles.
-/
def ColumnProfilePrivateAuthModesSafe
    (profile : ColumnProfile) : Prop :=
  forall index,
    VisibilityRequiresPrivateAuthentication
        (profile.entry index).visibilityClass ->
      RoleAuthenticationModeAllowedForPrivate
        (profile.entry index).authenticationMode

/--
The transparent view's compact column-profile id is compatible with the native
boundary profile, and private entries in that profile use only active-safe role
authentication modes.
-/
def ColumnProfileCompatibleWithBoundary
    (b : Boundary) (view : TransparentPublicView)
    (profile : ColumnProfile) : Prop :=
  view.columnProfile = profile.profileId ∧
    profile.profileId = b.roleProfileDigest ∧
      ColumnProfilePrivateAuthModesSafe profile

theorem columnProfileCompatible_profile_eq
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (h : ColumnProfileCompatibleWithBoundary b view profile) :
    view.columnProfile = b.roleProfileDigest := by
  exact Eq.trans h.1 h.2.1

theorem columnProfileCompatible_private_auth_modes_safe
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (h : ColumnProfileCompatibleWithBoundary b view profile) :
    ColumnProfilePrivateAuthModesSafe profile :=
  h.2.2

/-- The selected native-boundary norm/range mode. -/
def selectedNormRangeMode : NormRangeMode :=
  NormRangeMode.hybridBase128HiddenRoles

/-- The selected theorem-level layout for hybrid base-128 range checks. -/
structure HybridBase128RangePolicy where
  semanticRangePolicyId : Nat
  witnessRangePolicyId : Nat
  privateRoleMatrixRows : Nat
  padSemanticValues : Nat
  padDigitColumns : Nat
  projectionSemanticValues : Nat
  projectionDigitColumns : Nat
  openingTupleSemanticValues : Nat
  openingTupleDigitColumns : Nat
  digitRangeRows : Nat
  reconstructionRows : Nat
  rangeWitnessColumns : Nat
  digitMax : Nat

/--
Profile entries that must carry the selected hidden base-128 range policy.

This binds the theorem-level range obligation to the typed active roles rather
than to an after-the-fact `NormRangeValid` claim.
-/
def RoleUsesRangePolicyWithVisibility
    (profile : ColumnProfile) (roleId : RoleId)
    (policyId : Nat) (visibilityClass : VisibilityClass) : Prop :=
  exists index,
    (profile.entry index).roleId = roleId ∧
      (profile.entry index).rangePolicyId = policyId ∧
        (profile.entry index).visibilityClass = visibilityClass

/-- The active route uses one selected hybrid base-128 range layout. -/
def UsesSelectedHybridBase128RangePolicy
    (profile : ColumnProfile) (policy : HybridBase128RangePolicy) : Prop :=
  profile.normRangeMode = selectedNormRangeMode ∧
    policy.semanticRangePolicyId =
      selectedHybridBase128SemanticRangePolicyId ∧
      policy.witnessRangePolicyId =
        selectedHybridBase128WitnessRangePolicyId ∧
        policy.privateRoleMatrixRows = selectedSisRoleMatrixRows ∧
          policy.padSemanticValues = 6 ∧
            policy.padDigitColumns = 12 ∧
              policy.projectionSemanticValues = 6 ∧
                policy.projectionDigitColumns = 12 ∧
                  policy.openingTupleSemanticValues = 8 ∧
                    policy.openingTupleDigitColumns = 16 ∧
                      policy.digitRangeRows = 40 ∧
                        policy.reconstructionRows = 20 ∧
                          policy.rangeWitnessColumns = 40 ∧
                            policy.digitMax = 127 ∧
                              ConstraintClassCovered profile.relationProfile
                                TraceConstraintClass.base128RangeRow ∧
                                  ConstraintClassCovered profile.relationProfile
                                    TraceConstraintClass.qFieldRow ∧
                                  RoleUsesRangePolicyWithVisibility profile
                                    RoleId.padSource
                                    policy.semanticRangePolicyId
                                    VisibilityClass.vcExtractorHidden ∧
                                    RoleUsesRangePolicyWithVisibility profile
                                      RoleId.projectionRelation
                                      policy.semanticRangePolicyId
                                      VisibilityClass.vcExtractorHidden ∧
                                      RoleUsesRangePolicyWithVisibility profile
                                        RoleId.openingTuple
                                        policy.semanticRangePolicyId
                                        VisibilityClass.vcExtractorHidden ∧
                                        RoleUsesRangePolicyWithVisibility profile
                                          RoleId.rangeWitness
                                          policy.witnessRangePolicyId
                                          VisibilityClass.vcRangePrivate

/-- A column profile uses the selected hybrid base-128 hidden-role mode. -/
def UsesSelectedNormRangeMode (profile : ColumnProfile) : Prop :=
  profile.normRangeMode = selectedNormRangeMode

theorem selectedNormRangeMode_is_hybrid :
    selectedNormRangeMode =
      NormRangeMode.hybridBase128HiddenRoles := by
  rfl

/-- A mandatory role is covered explicitly or by a pinned derivation rule. -/
def MandatoryRoleCovered (profile : ColumnProfile) (roleId : RoleId) : Prop :=
  exists index, (profile.entry index).roleId = roleId

/-- The active native-boundary profile covers every mandatory proof role. -/
def ProfileRoleComplete (profile : ColumnProfile) : Prop :=
  MandatoryRoleCovered profile RoleId.contextPublic ∧
    MandatoryRoleCovered profile RoleId.packedWitness ∧
      MandatoryRoleCovered profile RoleId.randomizer ∧
        MandatoryRoleCovered profile RoleId.padSource ∧
          MandatoryRoleCovered profile RoleId.openingTuple ∧
            MandatoryRoleCovered profile RoleId.projectionRelation ∧
              MandatoryRoleCovered profile RoleId.rangeWitness ∧
                MandatoryRoleCovered profile RoleId.blinding ∧
                  MandatoryRoleCovered profile RoleId.constraint

/-- The active route rejects sampled-opening-only extraction surfaces. -/
def SupportsFullTraceExtraction (profile : ColumnProfile) : Prop :=
  profile.extractionMode = TraceExtractionMode.fullTraceExtractable ∨
    profile.extractionMode = TraceExtractionMode.fullWitnessExtractable

/-- The active route rejects sampled-row-only relation checks. -/
def SupportsRelationKnowledge (profile : ColumnProfile) : Prop :=
  (profile.relationKnowledgeMode =
        RelationKnowledgeMode.proximityWithWitnessBridge ∨
      profile.relationKnowledgeMode =
        RelationKnowledgeMode.directRelationKnowledge ∨
      profile.relationKnowledgeMode =
        RelationKnowledgeMode.fullWitnessRelationKnowledge) ∧
    TraceRelationCoversNativeBoundary profile.relationProfile

theorem supports_relation_knowledge_covers_native_boundary
    {profile : ColumnProfile}
    (h : SupportsRelationKnowledge profile) :
    TraceRelationCoversNativeBoundary profile.relationProfile :=
  h.2

/-- The selected extraction mode for the active native-boundary profile. -/
def selectedTraceExtractionMode : TraceExtractionMode :=
  TraceExtractionMode.fullTraceExtractable

/-- The selected relation-knowledge mode for the active native-boundary profile. -/
def selectedRelationKnowledgeMode : RelationKnowledgeMode :=
  RelationKnowledgeMode.fullWitnessRelationKnowledge

/-- The active route uses one selected extraction/relation-knowledge pair. -/
def UsesSelectedTraceKnowledgeModes (profile : ColumnProfile) : Prop :=
  profile.extractionMode = selectedTraceExtractionMode ∧
    profile.relationKnowledgeMode = selectedRelationKnowledgeMode ∧
      TraceRelationCoversNativeBoundary profile.relationProfile ∧
        TraceRelationMeetsSelectedNativeBoundaryRowFloor profile.relationProfile ∧
          TraceRelationMeetsSelectedNativeBoundaryClassFloors
            profile.relationProfile

theorem selected_trace_knowledge_modes_support_full_trace
    {profile : ColumnProfile}
    (h : UsesSelectedTraceKnowledgeModes profile) :
    SupportsFullTraceExtraction profile :=
  Or.inl h.1

theorem selected_trace_knowledge_modes_support_relation_knowledge
    {profile : ColumnProfile}
    (h : UsesSelectedTraceKnowledgeModes profile) :
    SupportsRelationKnowledge profile :=
  ⟨Or.inr (Or.inr h.2.1), h.2.2.1⟩

theorem selected_trace_knowledge_modes_relation_row_floor
    {profile : ColumnProfile}
    (h : UsesSelectedTraceKnowledgeModes profile) :
    TraceRelationMeetsSelectedNativeBoundaryRowFloor profile.relationProfile :=
  h.2.2.2.1

theorem selected_trace_knowledge_modes_relation_class_floors
    {profile : ColumnProfile}
    (h : UsesSelectedTraceKnowledgeModes profile) :
    TraceRelationMeetsSelectedNativeBoundaryClassFloors
      profile.relationProfile :=
  h.2.2.2.2

/-- The active Rust two-view column profile id mirrored at theorem level. -/
def selectedTwoViewColumnProfileId : Nat :=
  1

/-- The active typed-role query-opening policy id. -/
def selectedTwoViewQueryOpeningPolicyId : Nat :=
  1

/-- The active unknown-profile rejection policy id. -/
def selectedTwoViewRejectionPolicyId : Nat :=
  1

/-- Hidden column-set ids for the active two-view column profile. -/
def selectedRoleHiddenColumnSet : RoleId -> Nat
  | RoleId.contextPublic => 101
  | RoleId.packedWitness => 102
  | RoleId.randomizer => 103
  | RoleId.padSource => 104
  | RoleId.openingTuple => 105
  | RoleId.projectionRelation => 106
  | RoleId.rangeWitness => 107
  | RoleId.blinding => 108
  | RoleId.constraint => 109

/-- Masked column-set ids for the active two-view column profile. -/
def selectedRoleMaskedColumnSet : RoleId -> Nat
  | RoleId.contextPublic => 201
  | RoleId.packedWitness => 202
  | RoleId.randomizer => 203
  | RoleId.padSource => 204
  | RoleId.openingTuple => 205
  | RoleId.projectionRelation => 206
  | RoleId.rangeWitness => 207
  | RoleId.blinding => 208
  | RoleId.constraint => 209

/-- Mask-domain labels for the active two-view column profile. -/
def selectedRoleMaskDomain : RoleId -> Nat
  | RoleId.contextPublic => 0
  | RoleId.packedWitness => 1
  | RoleId.randomizer => 2
  | RoleId.padSource => 3
  | RoleId.openingTuple => 4
  | RoleId.projectionRelation => 5
  | RoleId.rangeWitness => 6
  | RoleId.blinding => 7
  | RoleId.constraint => 8

/-- Visibility class selected for each active theorem-level role. -/
def selectedRoleVisibility : RoleId -> VisibilityClass
  | RoleId.contextPublic => VisibilityClass.vcPublic
  | RoleId.packedWitness => VisibilityClass.vcExtractorHidden
  | RoleId.randomizer => VisibilityClass.vcBlindingPrivate
  | RoleId.padSource => VisibilityClass.vcExtractorHidden
  | RoleId.openingTuple => VisibilityClass.vcExtractorHidden
  | RoleId.projectionRelation => VisibilityClass.vcExtractorHidden
  | RoleId.rangeWitness => VisibilityClass.vcRangePrivate
  | RoleId.blinding => VisibilityClass.vcBlindingPrivate
  | RoleId.constraint => VisibilityClass.vcCommittedHidden

/-- Role-authentication mode selected for each active theorem-level role. -/
def selectedRoleAuthenticationMode : RoleId -> RoleAuthenticationMode
  | RoleId.contextPublic => RoleAuthenticationMode.publicPathToMaskedLeaf
  | RoleId.packedWitness => RoleAuthenticationMode.extractorOnlyOpening
  | RoleId.randomizer => RoleAuthenticationMode.simulatorOnlyPath
  | RoleId.padSource => RoleAuthenticationMode.extractorOnlyOpening
  | RoleId.openingTuple => RoleAuthenticationMode.extractorOnlyOpening
  | RoleId.projectionRelation => RoleAuthenticationMode.extractorOnlyOpening
  | RoleId.rangeWitness => RoleAuthenticationMode.extractorOnlyOpening
  | RoleId.blinding => RoleAuthenticationMode.simulatorOnlyPath
  | RoleId.constraint => RoleAuthenticationMode.simulatorOnlyPath

/-- Range-policy id selected for each active theorem-level role. -/
def selectedRoleRangePolicyId : RoleId -> Nat
  | RoleId.packedWitness => selectedHybridBase128SemanticRangePolicyId
  | RoleId.padSource => selectedHybridBase128SemanticRangePolicyId
  | RoleId.openingTuple => selectedHybridBase128SemanticRangePolicyId
  | RoleId.projectionRelation => selectedHybridBase128SemanticRangePolicyId
  | RoleId.rangeWitness => selectedHybridBase128WitnessRangePolicyId
  | RoleId.contextPublic => 0
  | RoleId.randomizer => 0
  | RoleId.blinding => 0
  | RoleId.constraint => 0

/-- One active role entry matches the selected Rust two-view profile table. -/
def SelectedActiveRoleEntry
    (profile : ColumnProfile) (roleId : RoleId) : Prop :=
  exists index,
    (profile.entry index).roleId = roleId ∧
      (profile.entry index).columnSetId =
        selectedRoleHiddenColumnSet roleId ∧
        (profile.entry index).maskedColumnSetId =
          selectedRoleMaskedColumnSet roleId ∧
          (profile.entry index).visibilityClass =
            selectedRoleVisibility roleId ∧
            (profile.entry index).authenticationMode =
              selectedRoleAuthenticationMode roleId ∧
              (profile.entry index).maskDomainLabel =
                selectedRoleMaskDomain roleId ∧
                (profile.entry index).rangePolicyId =
                  selectedRoleRangePolicyId roleId ∧
                  (profile.entry index).queryPolicyId =
                    selectedTwoViewQueryOpeningPolicyId

/--
The selected active column profile pins every role/column-set entry used by
the Rust masked-base128 two-view profile.
-/
def SelectedActiveColumnProfile (profile : ColumnProfile) : Prop :=
  profile.profileId = selectedTwoViewColumnProfileId ∧
    profile.queryOpeningPolicyId =
      selectedTwoViewQueryOpeningPolicyId ∧
      profile.rejectionPolicyId =
        selectedTwoViewRejectionPolicyId ∧
        UsesSelectedTraceKnowledgeModes profile ∧
          ColumnProfilePrivateAuthModesSafe profile ∧
            SelectedActiveRoleEntry profile RoleId.contextPublic ∧
              SelectedActiveRoleEntry profile RoleId.packedWitness ∧
                SelectedActiveRoleEntry profile RoleId.randomizer ∧
                  SelectedActiveRoleEntry profile RoleId.padSource ∧
                    SelectedActiveRoleEntry profile RoleId.openingTuple ∧
                      SelectedActiveRoleEntry profile
                        RoleId.projectionRelation ∧
                        SelectedActiveRoleEntry profile
                          RoleId.rangeWitness ∧
                          SelectedActiveRoleEntry profile RoleId.blinding ∧
                            SelectedActiveRoleEntry profile RoleId.constraint

theorem selected_active_role_entry_covers
    {profile : ColumnProfile} {roleId : RoleId}
    (h : SelectedActiveRoleEntry profile roleId) :
    MandatoryRoleCovered profile roleId := by
  rcases h with ⟨index, hRole, _hRest⟩
  exact ⟨index, hRole⟩

theorem selected_active_column_profile_role_complete
    {profile : ColumnProfile}
    (h : SelectedActiveColumnProfile profile) :
    ProfileRoleComplete profile := by
  rcases h with
    ⟨_hProfileId, _hQueryPolicy, _hRejectionPolicy,
      _hTraceModes, _hPrivateAuth, hContext, hPacked, hRandomizer,
      hPad, hOpening, hProjection, hRange, hBlinding, hConstraint⟩
  exact
    ⟨selected_active_role_entry_covers hContext,
      selected_active_role_entry_covers hPacked,
      selected_active_role_entry_covers hRandomizer,
      selected_active_role_entry_covers hPad,
      selected_active_role_entry_covers hOpening,
      selected_active_role_entry_covers hProjection,
      selected_active_role_entry_covers hRange,
      selected_active_role_entry_covers hBlinding,
      selected_active_role_entry_covers hConstraint⟩

theorem selected_active_column_profile_trace_modes
    {profile : ColumnProfile}
    (h : SelectedActiveColumnProfile profile) :
    UsesSelectedTraceKnowledgeModes profile := by
  rcases h with
    ⟨_hProfileId, _hQueryPolicy, _hRejectionPolicy,
      hTraceModes, _hRest⟩
  exact hTraceModes

/-- Native boundary masked projection as the older Path2DescentSIS quotient. -/
def toDescentSISMaskedProjection
    (b : Boundary) : Path2DescentSIS.MaskedProjection where
  lane := b.maskedProjection

/--
Compatibility projection from the native boundary to the older Path2DescentSIS
public object. The field `creationProofBindingDigest` is intentionally filled
with `nativeBoundaryDigest`.
-/
def toDescentSISPublic
    (b : Boundary)
    (leafDigest proofPayloadDigest relationTranscriptDigest : Nat) :
    Path2DescentSIS.Public where
  leafDigest := leafDigest
  creationProofBindingDigest := b.nativeBoundaryDigest
  proofPayloadDigest := proofPayloadDigest
  relationTranscriptDigest := relationTranscriptDigest
  maskedProjection := toDescentSISMaskedProjection b

/--
Compatibility projection from native role handles to the older
Path2DescentSIS representative shape. These commitments are projections of the
native boundary, not independent hidden-source commitments.
-/
def toDescentSISRepresentative
    (b : Boundary) (proofTranscriptCommitment : Nat) :
    Path2DescentSIS.Representative where
  padCommitment := b.roleHandles.padSourceHandle
  projectionCommitment := b.roleHandles.projectionRelationHandle
  openingTupleCommitment := b.roleHandles.openingTupleHandle
  proofTranscriptCommitment := proofTranscriptCommitment
  maskedProjection := toDescentSISMaskedProjection b

theorem toDescentSISPublic_uses_nativeBoundaryDigest
    (b : Boundary)
    (leafDigest proofPayloadDigest relationTranscriptDigest : Nat) :
    (toDescentSISPublic b leafDigest proofPayloadDigest
      relationTranscriptDigest).creationProofBindingDigest =
      b.nativeBoundaryDigest := by
  rfl

theorem toDescentSISPublic_quotient_eq_boundary_projection
    (b : Boundary)
    (leafDigest proofPayloadDigest relationTranscriptDigest : Nat) :
    Path2DescentSIS.quotientOfPublic
      (toDescentSISPublic b leafDigest proofPayloadDigest
        relationTranscriptDigest) =
      toDescentSISMaskedProjection b := by
  rfl

theorem toDescentSISRepresentative_quotient_eq_boundary_projection
    (b : Boundary) (proofTranscriptCommitment : Nat) :
    Path2DescentSIS.quotientOfRepresentative
      (toDescentSISRepresentative b proofTranscriptCommitment) =
      toDescentSISMaskedProjection b := by
  rfl

/--
Transparent public view is compatible with a boundary when its semantic fields
are projections of that boundary. Proof-family metadata remains separate.
-/
def TransparentViewProjectsFrom
    (b : Boundary) (view : TransparentPublicView) : Prop :=
  view.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
    view.columnProfile = b.roleProfileDigest ∧
      view.maskedProjection = b.maskedProjection ∧
        view.aggregationContextDigest =
          b.context.aggregationContextDigest ∧
          view.traceCommitmentRoot = b.parentSectionRoot

/-- Abstract predicate package for native-boundary admission and handoff. -/
structure Predicates where
  boundaryOf : Context -> HiddenRepresentative -> Boundary
  localValid : Context -> HiddenRepresentative -> Prop
  descentConsistent : Context -> HiddenRepresentative -> Prop
  normRangeValid : Boundary -> HiddenRepresentative -> Prop
  blindingNoiseBounded : Boundary -> BlindingNoisePolicy -> HiddenRepresentative -> Prop
  safeEquivalent : Context -> HiddenRepresentative -> HiddenRepresentative -> Prop
  boundaryBreakFor : Context -> Boundary -> BreakEvent -> Prop
  accountCreateAccepts : Context -> Boundary -> Prop
  admitted : Context -> Boundary -> Prop
  parentRootBindsPrivateRoles : Boundary -> ColumnProfile -> Prop
  roleRestrictionsAuthenticate : Boundary -> ColumnProfile -> Prop
  noExportHolds : Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  roleAuthenticationMaterialSimulatable :
    Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  maskFreshnessHolds : Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  qromScheduleBound : Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  qromExtractorRewindBound : Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  rangeRowsBindRepresentative :
    Boundary ->
      TransparentPublicView ->
        ColumnProfile ->
          HybridBase128RangePolicy -> HiddenRepresentative -> Prop
  rangeRowsBindTraceRepresentative :
    Boundary ->
      TransparentPublicView ->
        ColumnProfile ->
          HybridBase128RangePolicy ->
            ExtractedTrace -> HiddenRepresentative -> Prop
  blindingRowsBoundRepresentative :
    Boundary ->
      TransparentPublicView ->
        ColumnProfile -> BlindingNoisePolicy -> HiddenRepresentative -> Prop
  blindingRowsBoundTraceRepresentative :
    Boundary ->
      TransparentPublicView ->
        ColumnProfile ->
          BlindingNoisePolicy ->
            ExtractedTrace -> HiddenRepresentative -> Prop
  transparentAccepts : Boundary -> TransparentPublicView -> Prop
  transparentExtractsTrace :
    Boundary -> TransparentPublicView -> ExtractedTrace -> Prop
  transparentExtracts :
    Boundary -> TransparentPublicView -> HiddenRepresentative -> Prop
  reviewedFq12PayloadAccepts :
    Boundary -> ReviewedFq12HiddenSourcePayload -> Prop
  reviewedFq12PayloadProjectsFrom :
    Boundary -> ReviewedFq12HiddenSourcePayload -> Prop
  reviewedFq12PayloadHasExtractionCertificate :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate -> Prop
  reviewedFq12ExtractionCertificateValid :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate -> Prop
  reviewedFq12CertificateExtracts :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        HiddenRepresentative -> Prop
  reviewedFq12CertificateLiftsToOpening :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> Prop
  reviewedFq12PayloadHasRoleLiftCertificate :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate -> Prop
  reviewedFq12RoleLiftCertificateValid :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate -> Prop
  reviewedFq12RoleLiftCertificateOpens :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          FullNativeRepresentativeOpening -> Prop
  fullNativeOpeningAssemblesRepresentative :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative -> Prop
  reviewedFq12OpeningHasAssemblyCertificate :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate -> Prop
  reviewedFq12AssemblyCertificateValid :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate -> Prop
  reviewedFq12AssemblyCertificateAssembles :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            HiddenRepresentative -> Prop
  reviewedFq12HasExtractionCompatibilityCertificate :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  reviewedFq12ExtractionCompatibilityCertificateValid :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  reviewedFq12ExtractionCompatibilityCertificateExtracts :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  reviewedFq12HasFullOpeningBridgeDigestCertificate :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            ReviewedFq12ExtractionCompatibilityCertificate ->
              ReviewedFq12FullOpeningBridgeDigestCertificate -> Prop
  reviewedFq12FullOpeningBridgeDigestCertificateValid :
    Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            ReviewedFq12ExtractionCompatibilityCertificate ->
              ReviewedFq12FullOpeningBridgeDigestCertificate -> Prop
  reviewedFq12PayloadExtracts :
    Boundary -> ReviewedFq12HiddenSourcePayload -> HiddenRepresentative -> Prop
  traceOpensBoundary :
    Boundary -> ColumnProfile -> ExtractedTrace -> Prop
  tracesSafetyEquivalent :
    Boundary -> ColumnProfile -> ExtractedTrace -> ExtractedTrace -> Prop
  maskedOpeningsConsistent :
    Boundary -> TransparentPublicView -> ColumnProfile -> ExtractedTrace -> Prop
  traceRelationValid :
    Boundary -> ColumnProfile -> ExtractedTrace -> Prop
  traceAssemblesRepresentative :
    ExtractedTrace -> HiddenRepresentative -> Prop
  extractedRolesComplete :
    Boundary -> ColumnProfile -> HiddenRepresentative -> Prop
  traceAssemblyRolesComplete :
    Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  traceAssemblyDerivedRolesBound :
    Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  traceAssemblyInFiber :
    Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  traceAssemblyLocalValid :
    Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  traceAssemblyDescentConsistent :
    Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  slotAuthorizationOf : Boundary -> SlotAuthorization
  spendAccepts : Context -> SlotAuthorization -> Prop
  spendWitnessValid : Context -> HiddenRepresentative -> SlotAuthorization -> Prop

/-- A hidden representative lies in the fiber of a public boundary. -/
def InFiber
    (P : Predicates) (c : Context) (b : Boundary)
    (h : HiddenRepresentative) : Prop :=
  P.boundaryOf c h = b

/-- Native account admission soundness assumption, stated as an interface. -/
def AccountAdmissionSoundness (P : Predicates) : Prop :=
  forall {c : Context} {b : Boundary},
    P.accountCreateAccepts c b ->
      (exists h,
        InFiber P c b h ∧
          P.localValid c h ∧
            P.descentConsistent c h) ∨
      (exists breakEvent, P.boundaryBreakFor c b breakEvent)

/--
Account-create/view bridge for the selected transparent proof-family route.
This is the missing account-admission adapter: an accepted account creation
must provide the accepted transparent public view, selected active column
profile, and selected range/blinding policies consumed by the active
selected-profile endpoint.
-/
def AccountCreateSelectedTransparentViewSoundness (P : Predicates) : Prop :=
  forall {c : Context} {b : Boundary},
    P.accountCreateAccepts c b ->
      c = b.context ∧
        exists view profile rangePolicy policy,
          P.transparentAccepts b view ∧
            TransparentViewProjectsFrom b view ∧
              ColumnProfileCompatibleWithBoundary b view profile ∧
                SelectedActiveColumnProfile profile ∧
                  UsesSelectedHybridBase128RangePolicy profile rangePolicy ∧
                    UsesSelectedBlindingNoisePolicy policy

/-- Native two-view extraction assumption for accepted transparent views. -/
def TwoViewExtractionSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        (exists h,
          P.transparentExtracts b view h ∧
            InFiber P b.context b h ∧
              P.localValid b.context h ∧
                P.descentConsistent b.context h) ∨
        (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Hidden-extractor completeness assumption: the extracted representative covers
every private typed role named by the compatible profile. This rules out an
extractor that returns only sampled query openings instead of a full boundary
fiber representative.
-/
def HiddenExtractorRoleCompletenessSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          ProfileRoleComplete profile ->
          P.transparentExtracts b view h ->
            InFiber P b.context b h ->
              P.extractedRolesComplete b profile h ∨
                (exists breakEvent,
                  P.boundaryBreakFor b.context b breakEvent)

theorem hidden_extractor_roles_complete_or_break
    (P : Predicates)
    (hSound : HiddenExtractorRoleCompletenessSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {h : HiddenRepresentative}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hExtracts : P.transparentExtracts b view h)
    (hFiber : InFiber P b.context b h) :
    P.extractedRolesComplete b profile h ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile hRoleComplete hExtracts hFiber

/--
Role-restriction authentication assumption: for an accepted transparent view
with a compatible profile, the role handles projected from the boundary
authenticate the intended column restrictions under the parent section root, or
the verifier exports a named break.
-/
def RoleRestrictionSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.roleRestrictionsAuthenticate b profile ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem role_restrictions_authenticate_or_break
    (P : Predicates)
    (hSound : RoleRestrictionSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile) :
    P.roleRestrictionsAuthenticate b profile ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile

/--
Parent-root binding assumption: the native boundary parent section root binds
the private typed role restrictions described by the compatible column profile,
or exports a named break.
-/
def ParentRootRoleBindingSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.parentRootBindsPrivateRoles b profile ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem parent_root_binds_private_roles_or_break
    (P : Predicates)
    (hSound : ParentRootRoleBindingSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile) :
    P.parentRootBindsPrivateRoles b profile ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile

/--
Full-trace extraction assumption: an accepted transparent view extracts a
complete trace object opening the parent section root and satisfying the trace
relation, or exports a named break.
-/
def ParentTraceCommitmentExtractionSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          SupportsFullTraceExtraction profile ->
            (exists trace,
              P.transparentExtractsTrace b view trace ∧
                P.traceOpensBoundary b profile trace) ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Double-trace binding assumption: two full extractor outputs that both open the
same accepted native-boundary parent trace are safety-equivalent, or the proof
exports a named trace/commitment binding break. This is the Lean counterpart of
the Rust `double_trace_binding_break_exported` interface flag.
-/
def ParentTraceDoubleBindingSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {traceA traceB : ExtractedTrace},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.transparentExtractsTrace b view traceA ->
            P.traceOpensBoundary b profile traceA ->
              P.transparentExtractsTrace b view traceB ->
                P.traceOpensBoundary b profile traceB ->
                  P.tracesSafetyEquivalent b profile traceA traceB ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

/--
Relation-knowledge assumption: an extracted full trace that opens the native
boundary satisfies the native-boundary trace relation, or exports a named break.
-/
def ExtractedTraceRelationKnowledgeSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          SupportsRelationKnowledge profile ->
            P.transparentExtractsTrace b view trace ->
              P.traceOpensBoundary b profile trace ->
                P.traceRelationValid b profile trace ∨
                  (exists breakEvent,
                    P.boundaryBreakFor b.context b breakEvent)

/--
Masked-opening consistency assumption: the verifier-visible masked/simulated
query openings are views of the same extracted hidden trace that opens the
native boundary, or the proof exports a named break. This rules out a disjoint
public masked view paired with an unrelated extractor witness.
-/
def MaskedOpeningConsistencySoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.transparentExtractsTrace b view trace ->
            P.traceOpensBoundary b profile trace ->
              P.maskedOpeningsConsistent b view profile trace ∨
                (exists breakEvent,
                  P.boundaryBreakFor b.context b breakEvent)

def ParentTraceExtractionSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedTraceKnowledgeModes profile ->
            (exists trace,
              P.transparentExtractsTrace b view trace ∧
                P.traceOpensBoundary b profile trace ∧
                  P.tracesSafetyEquivalent b profile trace trace ∧
                    P.maskedOpeningsConsistent b view profile trace ∧
                      P.traceRelationValid b profile trace) ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem commitment_extraction_and_relation_knowledge_imply_parent_trace_extraction
    (P : Predicates)
    (hCommitExtract : ParentTraceCommitmentExtractionSoundness P)
    (hDoubleTraceBinding : ParentTraceDoubleBindingSoundness P)
    (hMaskedConsistency : MaskedOpeningConsistencySoundness P)
    (hRelationKnowledge : ExtractedTraceRelationKnowledgeSoundness P) :
    ParentTraceExtractionSoundness P := by
  intro b view profile hAccept hProjects hProfile hSelected
  have hFullTrace :
      SupportsFullTraceExtraction profile :=
    selected_trace_knowledge_modes_support_full_trace hSelected
  have hRelationMode :
      SupportsRelationKnowledge profile :=
    selected_trace_knowledge_modes_support_relation_knowledge hSelected
  cases hCommitExtract hAccept hProjects hProfile hFullTrace with
  | inl hTrace =>
      rcases hTrace with ⟨trace, hExtractsTrace, hOpens⟩
      cases
          hDoubleTraceBinding hAccept hProjects hProfile
            hExtractsTrace hOpens hExtractsTrace hOpens with
      | inl hTraceStable =>
          cases hMaskedConsistency hAccept hProjects hProfile hExtractsTrace hOpens with
          | inl hMasked =>
              cases
                  hRelationKnowledge hAccept hProjects hProfile hRelationMode
                    hExtractsTrace hOpens with
              | inl hRelation =>
                  exact
                    Or.inl
                      ⟨trace, hExtractsTrace, hOpens, hTraceStable, hMasked,
                        hRelation⟩
              | inr hBreak =>
                  exact Or.inr hBreak
          | inr hBreak =>
              exact Or.inr hBreak
      | inr hBreak =>
          exact Or.inr hBreak
  | inr hBreak =>
      exact Or.inr hBreak

theorem parent_trace_extracts_or_break
    (P : Predicates)
    (hSound : ParentTraceExtractionSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hSelected : UsesSelectedTraceKnowledgeModes profile) :
    (exists trace,
    P.transparentExtractsTrace b view trace ∧
        P.traceOpensBoundary b profile trace ∧
          P.tracesSafetyEquivalent b profile trace trace ∧
            P.maskedOpeningsConsistent b view profile trace ∧
              P.traceRelationValid b profile trace) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile hSelected

/--
Trace assembly assumption: a full extracted trace that opens the boundary and
satisfies the relation assembles into a role-complete hidden representative in
the native-boundary fiber, or exports a named break.
-/
def TraceRepresentativeAssemblyExistenceSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          ProfileRoleComplete profile ->
          P.transparentExtractsTrace b view trace ->
            P.traceOpensBoundary b profile trace ->
              P.traceRelationValid b profile trace ->
                (exists h,
                  P.traceAssemblesRepresentative trace h ∧
                    P.transparentExtracts b view h) ∨
                (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

def TraceAssemblyRolesCompleteSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          ProfileRoleComplete profile ->
          P.transparentExtractsTrace b view trace ->
            P.traceOpensBoundary b profile trace ->
              P.traceRelationValid b profile trace ->
                P.traceAssemblesRepresentative trace h ->
                  P.transparentExtracts b view h ->
                    P.traceAssemblyRolesComplete b profile trace h ∧
                      P.extractedRolesComplete b profile h

def TraceAssemblyDerivedRolesSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          ProfileRoleComplete profile ->
          P.transparentExtractsTrace b view trace ->
            P.traceOpensBoundary b profile trace ->
              P.traceRelationValid b profile trace ->
                P.traceAssemblesRepresentative trace h ->
                  P.transparentExtracts b view h ->
                    P.traceAssemblyRolesComplete b profile trace h ->
                      P.traceAssemblyDerivedRolesBound b profile trace h

def TraceAssemblyFiberSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.transparentExtractsTrace b view trace ->
            P.traceOpensBoundary b profile trace ->
              P.traceRelationValid b profile trace ->
                P.traceAssemblesRepresentative trace h ->
                  P.transparentExtracts b view h ->
                    P.traceAssemblyInFiber b profile trace h ∧
                      InFiber P b.context b h

def TraceAssemblyLocalValiditySoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.transparentExtractsTrace b view trace ->
            P.traceOpensBoundary b profile trace ->
              P.traceRelationValid b profile trace ->
                P.traceAssemblesRepresentative trace h ->
                  P.transparentExtracts b view h ->
                    P.traceAssemblyLocalValid b profile trace h ∧
                      P.localValid b.context h

def TraceAssemblyDescentConsistencySoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.transparentExtractsTrace b view trace ->
            P.traceOpensBoundary b profile trace ->
              P.traceRelationValid b profile trace ->
                P.traceAssemblesRepresentative trace h ->
                  P.transparentExtracts b view h ->
                    P.traceAssemblyDescentConsistent b profile trace h ∧
                      P.descentConsistent b.context h

def TraceAssemblySoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {trace : ExtractedTrace},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          ProfileRoleComplete profile ->
          P.transparentExtractsTrace b view trace ->
            P.traceOpensBoundary b profile trace ->
              P.traceRelationValid b profile trace ->
                (exists h,
                  P.traceAssemblesRepresentative trace h ∧
                    P.transparentExtracts b view h ∧
                      P.extractedRolesComplete b profile h ∧
                        P.traceAssemblyDerivedRolesBound b profile trace h ∧
                          InFiber P b.context b h ∧
                            P.localValid b.context h ∧
                              P.descentConsistent b.context h) ∨
                (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem decomposed_trace_assembly_implies_trace_assembly_soundness
    (P : Predicates)
    (hAssemble : TraceRepresentativeAssemblyExistenceSoundness P)
    (hRoles : TraceAssemblyRolesCompleteSoundness P)
    (hDerived : TraceAssemblyDerivedRolesSoundness P)
    (hFiber : TraceAssemblyFiberSoundness P)
    (hLocal : TraceAssemblyLocalValiditySoundness P)
    (hDescent : TraceAssemblyDescentConsistencySoundness P) :
    TraceAssemblySoundness P := by
  intro b view profile trace hAccept hProjects hProfile hRoleComplete
    hTrace hOpens hRelation
  cases
      hAssemble hAccept hProjects hProfile hRoleComplete
        hTrace hOpens hRelation with
  | inl hAssembly =>
      rcases hAssembly with ⟨h, hTraceAssembles, hTransparent⟩
      have hRolesComplete :
          P.traceAssemblyRolesComplete b profile trace h ∧
            P.extractedRolesComplete b profile h :=
        hRoles hAccept hProjects hProfile hRoleComplete
          hTrace hOpens hRelation hTraceAssembles hTransparent
      exact Or.inl
        ⟨h, hTraceAssembles, hTransparent,
          hRolesComplete.2,
          hDerived hAccept hProjects hProfile hRoleComplete
            hTrace hOpens hRelation hTraceAssembles hTransparent
            hRolesComplete.1,
          (hFiber hAccept hProjects hProfile
            hTrace hOpens hRelation hTraceAssembles hTransparent).2,
          (hLocal hAccept hProjects hProfile
            hTrace hOpens hRelation hTraceAssembles hTransparent).2,
          (hDescent hAccept hProjects hProfile
            hTrace hOpens hRelation hTraceAssembles hTransparent).2⟩
  | inr hBreak =>
      exact Or.inr hBreak

theorem trace_assembles_representative_or_break
    (P : Predicates)
    (hSound : TraceAssemblySoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {trace : ExtractedTrace}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hTrace : P.transparentExtractsTrace b view trace)
    (hOpens : P.traceOpensBoundary b profile trace)
    (hRelation : P.traceRelationValid b profile trace) :
    (exists h,
      P.traceAssemblesRepresentative trace h ∧
        P.transparentExtracts b view h ∧
          P.extractedRolesComplete b profile h ∧
            P.traceAssemblyDerivedRolesBound b profile trace h ∧
              InFiber P b.context b h ∧
                P.localValid b.context h ∧
                  P.descentConsistent b.context h) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile hRoleComplete hTrace hOpens hRelation

/--
Composed native-boundary transparent soundness surface: an accepted view with a
compatible profile both authenticates role restrictions and extracts a hidden
representative in the boundary fiber, or exports a named break.
-/
theorem transparent_view_authenticates_roles_and_extracts_or_break
    (P : Predicates)
    (hParentSound : ParentRootRoleBindingSoundness P)
    (hRoleSound : RoleRestrictionSoundness P)
    (hExtractSound : TwoViewExtractionSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile) :
    (P.roleRestrictionsAuthenticate b profile ∧
      P.parentRootBindsPrivateRoles b profile ∧
      exists h,
        P.transparentExtracts b view h ∧
          InFiber P b.context b h ∧
            P.localValid b.context h ∧
              P.descentConsistent b.context h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  cases hParentSound hAccept hProjects hProfile with
  | inl hRole =>
      cases hRoleSound hAccept hProjects hProfile with
      | inl hRestriction =>
          cases hExtractSound hAccept hProjects with
          | inl hExtract =>
              exact Or.inl ⟨hRestriction, hRole, hExtract⟩
          | inr hBreak =>
              exact Or.inr hBreak
      | inr hBreak =>
          exact Or.inr hBreak
  | inr hBreak =>
      exact Or.inr hBreak

/--
No-export assumption: an accepted transparent view with a compatible native
boundary and profile exposes only allowed public/masked/simulated material, or
exports a named break.
-/
def NoExportSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.noExportHolds b view profile ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem no_export_holds_or_break
    (P : Predicates)
    (hSound : NoExportSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile) :
    P.noExportHolds b view profile ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile

/--
Role-authentication material no-export assumption: public authentication paths,
simulator messages, and extractor-only role handles are simulatable from the
public native boundary view and do not reveal raw private role vectors, or the
proof exports a named break.
-/
def RoleAuthenticationMaterialNoExportSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.roleAuthenticationMaterialSimulatable b view profile ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem role_authentication_material_simulatable_or_break
    (P : Predicates)
    (hSound : RoleAuthenticationMaterialNoExportSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile) :
    P.roleAuthenticationMaterialSimulatable b view profile ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile

/--
Mask-freshness assumption: every verifier-visible masked opening is bound to a
fresh native-boundary/role/column/row/query/transcript domain, or the proof
exports a named leakage or domain-separation break.
-/
def MaskFreshnessSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.maskFreshnessHolds b view profile ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem mask_freshness_holds_or_break
    (P : Predicates)
    (hSound : MaskFreshnessSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile) :
    P.maskFreshnessHolds b view profile ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile

/--
QROM schedule-binding assumption: an accepted transparent view binds the native
boundary, role handles, role authentication modes, proof-system identifiers,
mask domains, range mode, and blinding/noise policy before deriving queries or
challenges.
-/
def NativeBoundaryTraceQromScheduleSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.qromScheduleBound b view profile ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem native_boundary_trace_qrom_schedule_bound_or_break
    (P : Predicates)
    (hSound : NativeBoundaryTraceQromScheduleSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile) :
    P.qromScheduleBound b view profile ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile

/--
QROM extractor-rewind assumption: the accepted transparent view's pinned query
schedule also carries the concrete measure-and-reprogram/forking loss needed
by the trace extractor, or exports a named break.
-/
def NativeBoundaryTraceQromExtractorRewindSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          P.qromExtractorRewindBound b view profile ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem native_boundary_trace_qrom_extractor_rewind_bound_or_break
    (P : Predicates)
    (hSound : NativeBoundaryTraceQromExtractorRewindSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile) :
    P.qromExtractorRewindBound b view profile ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile

/--
Selected range-row soundness: accepted rows for the selected hybrid base-128
policy bind the extracted representative to the authenticated digit/range
witness columns, or export a named break.
-/
def HybridBase128RangeRowSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedHybridBase128RangePolicy profile rangePolicy ->
            P.transparentExtracts b view h ->
              InFiber P b.context b h ->
                P.rangeRowsBindRepresentative b view profile rangePolicy h ∨
                  (exists breakEvent,
                    P.boundaryBreakFor b.context b breakEvent)

/--
Selected range-row completeness: once accepted rows are bound to the extracted
representative, their reconstruction/range facts imply the theorem-level
`NormRangeValid` predicate, or export a named break.
-/
def HybridBase128RangeRowsToNormRangeSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedHybridBase128RangePolicy profile rangePolicy ->
            P.transparentExtracts b view h ->
              InFiber P b.context b h ->
                P.rangeRowsBindRepresentative b view profile rangePolicy h ->
                  P.normRangeValid b h ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

def TraceRangeRowsBindRepresentativeSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
      {trace : ExtractedTrace} {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedHybridBase128RangePolicy profile rangePolicy ->
            P.transparentExtractsTrace b view trace ->
              P.traceOpensBoundary b profile trace ->
                P.traceRelationValid b profile trace ->
                  P.traceAssemblesRepresentative trace h ->
                    P.transparentExtracts b view h ->
                      InFiber P b.context b h ->
                        P.rangeRowsBindTraceRepresentative
                          b view profile rangePolicy trace h ∧
                          P.rangeRowsBindRepresentative
                            b view profile rangePolicy h

/--
Norm/range assumption: every representative extracted from an accepted
compatible transparent view satisfies the canonicality/range bounds needed by
the native-boundary binding theorem, or exports a named break.
-/
def NormRangeSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedHybridBase128RangePolicy profile rangePolicy ->
            P.transparentExtracts b view h ->
              InFiber P b.context b h ->
                P.normRangeValid b h ∨
                  (exists breakEvent,
                    P.boundaryBreakFor b.context b breakEvent)

theorem hybrid_base128_range_rows_imply_norm_range_soundness
    (P : Predicates)
    (hRows : HybridBase128RangeRowSoundness P)
    (hRowsToNorm : HybridBase128RangeRowsToNormRangeSoundness P) :
    NormRangeSoundness P := by
  intro b view profile rangePolicy h hAccept hProjects hProfile
    hRangePolicy hExtracts hFiber
  cases
      hRows hAccept hProjects hProfile hRangePolicy hExtracts
        hFiber with
  | inl hBoundRows =>
      exact
        hRowsToNorm hAccept hProjects hProfile hRangePolicy
          hExtracts hFiber hBoundRows
  | inr hBreak =>
      exact Or.inr hBreak

theorem trace_range_rows_bind_representative_or_break
    (P : Predicates)
    (hTraceExtractSound : ParentTraceExtractionSoundness P)
    (hTraceAssemblySound : TraceAssemblySoundness P)
    (hTraceRangeRows : TraceRangeRowsBindRepresentativeSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hSelected : UsesSelectedTraceKnowledgeModes profile)
    (hRangePolicy : UsesSelectedHybridBase128RangePolicy profile rangePolicy) :
    (exists trace,
      P.transparentExtractsTrace b view trace ∧
        P.traceOpensBoundary b profile trace ∧
          P.traceRelationValid b profile trace ∧
            exists h,
              P.traceAssemblesRepresentative trace h ∧
                P.transparentExtracts b view h ∧
                  InFiber P b.context b h ∧
                    P.rangeRowsBindTraceRepresentative
                      b view profile rangePolicy trace h ∧
                      P.rangeRowsBindRepresentative
                        b view profile rangePolicy h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  cases
      hTraceExtractSound hAccept hProjects hProfile hSelected with
  | inl hTrace =>
      rcases hTrace with
        ⟨trace, hExtractsTrace, hOpens, _hTraceStable, _hMasked, hRelation⟩
      cases
          hTraceAssemblySound hAccept hProjects hProfile hRoleComplete
            hExtractsTrace hOpens hRelation with
      | inl hAssembled =>
          rcases hAssembled with
            ⟨h, hTraceAssembles, hTransparent, _hComplete,
              _hDerived, hFiber, _hLocal, _hDescent⟩
          exact Or.inl
            ⟨trace, hExtractsTrace, hOpens, hRelation, h,
              hTraceAssembles, hTransparent, hFiber,
              hTraceRangeRows hAccept hProjects hProfile hRangePolicy
                hExtractsTrace hOpens hRelation hTraceAssembles
                hTransparent hFiber⟩
      | inr hBreak =>
          exact Or.inr hBreak
  | inr hBreak =>
      exact Or.inr hBreak

theorem norm_range_valid_or_break
    (P : Predicates)
    (hSound : NormRangeSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
    {h : HiddenRepresentative}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRangePolicy : UsesSelectedHybridBase128RangePolicy profile rangePolicy)
    (hExtracts : P.transparentExtracts b view h)
    (hFiber : InFiber P b.context b h) :
    P.normRangeValid b h ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile hRangePolicy hExtracts hFiber

/--
Blinding/noise bound assumption: the extracted representative's blinding and
range-auxiliary coordinates satisfy the selected binding-compatible bound
policy, or export a named break.
-/
def BlindingNoiseBoundRowSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {policy : BlindingNoisePolicy}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedBlindingNoisePolicy policy ->
            P.transparentExtracts b view h ->
              InFiber P b.context b h ->
                P.blindingRowsBoundRepresentative b view profile policy h ∨
                  (exists breakEvent,
                    P.boundaryBreakFor b.context b breakEvent)

def BlindingNoiseRowsToBoundSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {policy : BlindingNoisePolicy}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedBlindingNoisePolicy policy ->
            P.transparentExtracts b view h ->
              InFiber P b.context b h ->
                P.blindingRowsBoundRepresentative b view profile policy h ->
                  P.blindingNoiseBounded b policy h ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

def TraceBlindingRowsBoundRepresentativeSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {policy : BlindingNoisePolicy}
      {trace : ExtractedTrace} {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedBlindingNoisePolicy policy ->
            P.transparentExtractsTrace b view trace ->
              P.traceOpensBoundary b profile trace ->
                P.traceRelationValid b profile trace ->
                  P.traceAssemblesRepresentative trace h ->
                    P.transparentExtracts b view h ->
                      InFiber P b.context b h ->
                        P.blindingRowsBoundTraceRepresentative
                          b view profile policy trace h ∧
                          P.blindingRowsBoundRepresentative
                            b view profile policy h

def BlindingNoiseBoundSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {view : TransparentPublicView}
      {profile : ColumnProfile} {policy : BlindingNoisePolicy}
      {h : HiddenRepresentative},
    P.transparentAccepts b view ->
      TransparentViewProjectsFrom b view ->
        ColumnProfileCompatibleWithBoundary b view profile ->
          UsesSelectedBlindingNoisePolicy policy ->
            P.transparentExtracts b view h ->
              InFiber P b.context b h ->
                P.blindingNoiseBounded b policy h ∨
                  (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

theorem blinding_noise_rows_imply_bound_soundness
    (P : Predicates)
    (hRows : BlindingNoiseBoundRowSoundness P)
    (hRowsToBound : BlindingNoiseRowsToBoundSoundness P) :
    BlindingNoiseBoundSoundness P := by
  intro b view profile policy h hAccept hProjects hProfile
    hPolicy hExtracts hFiber
  cases
      hRows hAccept hProjects hProfile hPolicy hExtracts
        hFiber with
  | inl hBoundRows =>
      exact
        hRowsToBound hAccept hProjects hProfile hPolicy
          hExtracts hFiber hBoundRows
  | inr hBreak =>
      exact Or.inr hBreak

theorem trace_blinding_rows_bound_representative_or_break
    (P : Predicates)
    (hTraceExtractSound : ParentTraceExtractionSoundness P)
    (hTraceAssemblySound : TraceAssemblySoundness P)
    (hTraceBlindingRows : TraceBlindingRowsBoundRepresentativeSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {policy : BlindingNoisePolicy}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hSelected : UsesSelectedTraceKnowledgeModes profile)
    (hPolicy : UsesSelectedBlindingNoisePolicy policy) :
    (exists trace,
      P.transparentExtractsTrace b view trace ∧
        P.traceOpensBoundary b profile trace ∧
          P.traceRelationValid b profile trace ∧
            exists h,
              P.traceAssemblesRepresentative trace h ∧
                P.transparentExtracts b view h ∧
                  InFiber P b.context b h ∧
                    P.blindingRowsBoundTraceRepresentative
                      b view profile policy trace h ∧
                      P.blindingRowsBoundRepresentative
                        b view profile policy h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  cases
      hTraceExtractSound hAccept hProjects hProfile hSelected with
  | inl hTrace =>
      rcases hTrace with
        ⟨trace, hExtractsTrace, hOpens, _hTraceStable, _hMasked, hRelation⟩
      cases
          hTraceAssemblySound hAccept hProjects hProfile hRoleComplete
            hExtractsTrace hOpens hRelation with
      | inl hAssembled =>
          rcases hAssembled with
            ⟨h, hTraceAssembles, hTransparent, _hComplete,
              _hDerived, hFiber, _hLocal, _hDescent⟩
          exact Or.inl
            ⟨trace, hExtractsTrace, hOpens, hRelation, h,
              hTraceAssembles, hTransparent, hFiber,
              hTraceBlindingRows hAccept hProjects hProfile hPolicy
                hExtractsTrace hOpens hRelation hTraceAssembles
                hTransparent hFiber⟩
      | inr hBreak =>
          exact Or.inr hBreak
  | inr hBreak =>
      exact Or.inr hBreak

theorem blinding_noise_bounded_or_break
    (P : Predicates)
    (hSound : BlindingNoiseBoundSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {policy : BlindingNoisePolicy}
    {h : HiddenRepresentative}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hPolicy : UsesSelectedBlindingNoisePolicy policy)
    (hExtracts : P.transparentExtracts b view h)
    (hFiber : InFiber P b.context b h) :
    P.blindingNoiseBounded b policy h ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects hProfile hPolicy hExtracts hFiber

/-- Minimum knowledge-soundness bundle required from the active trace proof. -/
def NativeBoundaryTraceKnowledgeSoundness (P : Predicates) : Prop :=
  ParentRootRoleBindingSoundness P ∧
    RoleRestrictionSoundness P ∧
      ParentTraceCommitmentExtractionSoundness P ∧
        ParentTraceDoubleBindingSoundness P ∧
          ExtractedTraceRelationKnowledgeSoundness P ∧
            MaskedOpeningConsistencySoundness P ∧
              ParentTraceExtractionSoundness P ∧
                TraceAssemblySoundness P

/--
The decomposed full-trace extractor assumptions imply the older two-view
extraction and role-completeness surfaces for an accepted compatible view.
-/
theorem trace_extraction_implies_hidden_representative_or_break
    (P : Predicates)
    (hTraceExtractSound : ParentTraceExtractionSoundness P)
    (hTraceAssemblySound : TraceAssemblySoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hSelected : UsesSelectedTraceKnowledgeModes profile) :
    (exists trace,
      P.transparentExtractsTrace b view trace ∧
        P.traceOpensBoundary b profile trace ∧
          P.maskedOpeningsConsistent b view profile trace ∧
            P.traceRelationValid b profile trace ∧
              exists h,
                P.traceAssemblesRepresentative trace h ∧
                  P.transparentExtracts b view h ∧
                    P.extractedRolesComplete b profile h ∧
                      P.traceAssemblyDerivedRolesBound b profile trace h ∧
                        InFiber P b.context b h ∧
                          P.localValid b.context h ∧
                            P.descentConsistent b.context h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  cases
      hTraceExtractSound hAccept hProjects hProfile hSelected with
  | inl hTrace =>
      rcases hTrace with
        ⟨trace, hExtractsTrace, hOpens, _hTraceStable, hMasked, hRelation⟩
      cases
          hTraceAssemblySound hAccept hProjects hProfile hRoleComplete
            hExtractsTrace hOpens hRelation with
      | inl hAssembled =>
          exact Or.inl
            ⟨trace, hExtractsTrace, hOpens, hMasked, hRelation, hAssembled⟩
      | inr hBreak =>
          exact Or.inr hBreak
  | inr hBreak =>
      exact Or.inr hBreak

/-- Minimum zero-knowledge/no-export bundle required from the active trace proof. -/
def NativeBoundaryTraceZeroKnowledge (P : Predicates) : Prop :=
  NoExportSoundness P ∧
    RoleAuthenticationMaterialNoExportSoundness P ∧
      MaskFreshnessSoundness P

/-- The full named proof-family assumption, still abstract over the construction. -/
def NativeBoundaryZkTraceBindingAndHiding (P : Predicates) : Prop :=
  NativeBoundaryTraceKnowledgeSoundness P ∧
    NativeBoundaryTraceZeroKnowledge P ∧
      NativeBoundaryTraceQromScheduleSoundness P ∧
        NativeBoundaryTraceQromExtractorRewindSoundness P ∧
          NormRangeSoundness P ∧
            BlindingNoiseBoundSoundness P

/--
Composed transparent account-creation proof surface before norm/range
instantiation: an accepted compatible view authenticates roles, extracts a
hidden representative in the boundary fiber, and satisfies no-export, or
exports a named break.
-/
theorem transparent_view_authenticates_extracts_no_export_or_break
    (P : Predicates)
    (hParentSound : ParentRootRoleBindingSoundness P)
    (hRoleSound : RoleRestrictionSoundness P)
    (hTraceExtractSound : ParentTraceExtractionSoundness P)
    (hTraceAssemblySound : TraceAssemblySoundness P)
    (hNoExportSound : NoExportSoundness P)
    (hRoleAuthNoExportSound : RoleAuthenticationMaterialNoExportSoundness P)
    (hMaskFreshnessSound : MaskFreshnessSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hSelected : UsesSelectedTraceKnowledgeModes profile) :
    (P.roleRestrictionsAuthenticate b profile ∧
      P.noExportHolds b view profile ∧
        P.roleAuthenticationMaterialSimulatable b view profile ∧
          P.maskFreshnessHolds b view profile ∧
            P.parentRootBindsPrivateRoles b profile ∧
            exists h,
              P.transparentExtracts b view h ∧
                P.extractedRolesComplete b profile h ∧
                InFiber P b.context b h ∧
                  P.localValid b.context h ∧
                    P.descentConsistent b.context h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  cases hParentSound hAccept hProjects hProfile with
  | inl hParent =>
      cases hRoleSound hAccept hProjects hProfile with
      | inl hRole =>
          cases
              trace_extraction_implies_hidden_representative_or_break
                P hTraceExtractSound hTraceAssemblySound
                hAccept hProjects hProfile hRoleComplete hSelected with
          | inl hTrace =>
              rcases hTrace with
                ⟨_trace, _hExtractsTrace, _hOpens, _hMasked, _hRelation,
                  hAssembled⟩
              rcases hAssembled with
                ⟨h, _hTraceAssembles, hTransparent, hComplete,
                  _hDerived, hFiber, hLocal, hDescent⟩
              cases hNoExportSound hAccept hProjects hProfile with
              | inl hNoExport =>
                  cases hRoleAuthNoExportSound hAccept hProjects hProfile with
                  | inl hRoleAuthNoExport =>
                      cases hMaskFreshnessSound hAccept hProjects hProfile with
                      | inl hMaskFresh =>
                          exact Or.inl
                            ⟨hRole, hNoExport, hRoleAuthNoExport,
                              hMaskFresh, hParent, h, hTransparent,
                              hComplete, hFiber, hLocal, hDescent⟩
                      | inr hBreak =>
                          exact Or.inr hBreak
                  | inr hBreak =>
                      exact Or.inr hBreak
              | inr hBreak =>
                  exact Or.inr hBreak
          | inr hBreak =>
              exact Or.inr hBreak
      | inr hBreak =>
          exact Or.inr hBreak
  | inr hBreak =>
      exact Or.inr hBreak

/--
Full native-boundary transparent interface surface before instantiating the
concrete proof family: accepted compatible views authenticate roles, satisfy
no-export, extract a hidden representative in the boundary fiber, and validate
norm/range semantics, or export a named break.
-/
theorem transparent_view_full_interface_or_break
    (P : Predicates)
    (hParentSound : ParentRootRoleBindingSoundness P)
    (hRoleSound : RoleRestrictionSoundness P)
    (hTraceExtractSound : ParentTraceExtractionSoundness P)
    (hTraceAssemblySound : TraceAssemblySoundness P)
    (hNoExportSound : NoExportSoundness P)
    (hRoleAuthNoExportSound : RoleAuthenticationMaterialNoExportSoundness P)
    (hMaskFreshnessSound : MaskFreshnessSoundness P)
    (hQromSound : NativeBoundaryTraceQromScheduleSoundness P)
    (hQromRewindSound : NativeBoundaryTraceQromExtractorRewindSoundness P)
    (hNormRangeSound : NormRangeSoundness P)
    (hBlindSound : BlindingNoiseBoundSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
    {policy : BlindingNoisePolicy}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hSelected : UsesSelectedTraceKnowledgeModes profile)
    (hRangePolicy : UsesSelectedHybridBase128RangePolicy profile rangePolicy)
    (hPolicy : UsesSelectedBlindingNoisePolicy policy) :
    (P.roleRestrictionsAuthenticate b profile ∧
      P.noExportHolds b view profile ∧
        P.roleAuthenticationMaterialSimulatable b view profile ∧
          P.maskFreshnessHolds b view profile ∧
            P.qromScheduleBound b view profile ∧
              P.qromExtractorRewindBound b view profile ∧
                P.parentRootBindsPrivateRoles b profile ∧
                exists h,
                  P.transparentExtracts b view h ∧
                    P.extractedRolesComplete b profile h ∧
                    InFiber P b.context b h ∧
                      P.localValid b.context h ∧
                        P.descentConsistent b.context h ∧
                          P.normRangeValid b h ∧
                            P.blindingNoiseBounded b policy h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  cases
      transparent_view_authenticates_extracts_no_export_or_break
        P hParentSound hRoleSound hTraceExtractSound
        hTraceAssemblySound hNoExportSound
        hRoleAuthNoExportSound hMaskFreshnessSound
        hAccept hProjects hProfile hRoleComplete hSelected with
  | inl hAll =>
      rcases hAll with
        ⟨hRole, hNoExport, hRoleAuthNoExport, hMaskFresh, hParent,
          hExtract⟩
      cases hQromSound hAccept hProjects hProfile with
      | inl hQrom =>
          cases hQromRewindSound hAccept hProjects hProfile with
          | inl hQromRewind =>
              rcases hExtract with
                ⟨h, hTransparent, hComplete, hFiber, hLocal, hDescent⟩
              cases
                  hNormRangeSound hAccept hProjects hProfile hRangePolicy
                    hTransparent hFiber with
              | inl hNorm =>
                  cases
                      hBlindSound hAccept hProjects hProfile hPolicy
                        hTransparent hFiber with
                  | inl hBlind =>
                      exact Or.inl
                        ⟨hRole, hNoExport, hRoleAuthNoExport, hMaskFresh,
                          hQrom, hQromRewind, hParent, h, hTransparent,
                          hComplete, hFiber, hLocal, hDescent, hNorm,
                          hBlind⟩
                  | inr hBreak =>
                      exact Or.inr hBreak
              | inr hBreak =>
                  exact Or.inr hBreak
          | inr hBreak =>
              exact Or.inr hBreak
      | inr hBreak =>
          exact Or.inr hBreak
  | inr hBreak =>
      exact Or.inr hBreak

/--
Fully decomposed active transparent interface: the same endpoint as
`transparent_view_full_interface_or_break`, but with assembly, range, and
blinding/noise soundness supplied through their narrower proof obligations.
-/
theorem transparent_view_full_interface_from_decomposed_or_break
    (P : Predicates)
    (hParentSound : ParentRootRoleBindingSoundness P)
    (hRoleSound : RoleRestrictionSoundness P)
    (hTraceExtractSound : ParentTraceExtractionSoundness P)
    (hTraceAssemblyExists : TraceRepresentativeAssemblyExistenceSoundness P)
    (hTraceAssemblyRoles : TraceAssemblyRolesCompleteSoundness P)
    (hTraceAssemblyDerived : TraceAssemblyDerivedRolesSoundness P)
    (hTraceAssemblyFiber : TraceAssemblyFiberSoundness P)
    (hTraceAssemblyLocal : TraceAssemblyLocalValiditySoundness P)
    (hTraceAssemblyDescent : TraceAssemblyDescentConsistencySoundness P)
    (hNoExportSound : NoExportSoundness P)
    (hRoleAuthNoExportSound : RoleAuthenticationMaterialNoExportSoundness P)
    (hMaskFreshnessSound : MaskFreshnessSoundness P)
    (hQromSound : NativeBoundaryTraceQromScheduleSoundness P)
    (hQromRewindSound : NativeBoundaryTraceQromExtractorRewindSoundness P)
    (hTraceRangeRows : TraceRangeRowsBindRepresentativeSoundness P)
    (hRangeRows : HybridBase128RangeRowSoundness P)
    (hRangeRowsToNorm : HybridBase128RangeRowsToNormRangeSoundness P)
    (hTraceBlindRows : TraceBlindingRowsBoundRepresentativeSoundness P)
    (hBlindRows : BlindingNoiseBoundRowSoundness P)
    (hBlindRowsToBound : BlindingNoiseRowsToBoundSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
    {policy : BlindingNoisePolicy}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hSelected : UsesSelectedTraceKnowledgeModes profile)
    (hRangePolicy : UsesSelectedHybridBase128RangePolicy profile rangePolicy)
    (hPolicy : UsesSelectedBlindingNoisePolicy policy) :
    (P.roleRestrictionsAuthenticate b profile ∧
      P.noExportHolds b view profile ∧
        P.roleAuthenticationMaterialSimulatable b view profile ∧
          P.maskFreshnessHolds b view profile ∧
            P.qromScheduleBound b view profile ∧
              P.qromExtractorRewindBound b view profile ∧
                P.parentRootBindsPrivateRoles b profile ∧
                exists h,
                  P.transparentExtracts b view h ∧
                    P.extractedRolesComplete b profile h ∧
                    InFiber P b.context b h ∧
                      P.localValid b.context h ∧
                        P.descentConsistent b.context h ∧
                          P.normRangeValid b h ∧
                            P.blindingNoiseBounded b policy h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  by
    have hTraceAssemblySound : TraceAssemblySoundness P :=
      decomposed_trace_assembly_implies_trace_assembly_soundness P
        hTraceAssemblyExists hTraceAssemblyRoles hTraceAssemblyDerived
        hTraceAssemblyFiber hTraceAssemblyLocal hTraceAssemblyDescent
    cases hParentSound hAccept hProjects hProfile with
    | inl hParent =>
        cases hRoleSound hAccept hProjects hProfile with
        | inl hRole =>
            cases hTraceExtractSound hAccept hProjects hProfile hSelected with
            | inl hTrace =>
                rcases hTrace with
                  ⟨trace, hExtractsTrace, hOpens, _hTraceStable,
                    _hMasked, hRelation⟩
                cases
                    hTraceAssemblySound hAccept hProjects hProfile
                      hRoleComplete hExtractsTrace hOpens hRelation with
                | inl hAssembled =>
                    rcases hAssembled with
                      ⟨h, hTraceAssembles, hTransparent, hComplete,
                        _hDerived, hFiber, hLocal, hDescent⟩
                    have _hSameTraceRangeRows :
                        P.rangeRowsBindTraceRepresentative
                            b view profile rangePolicy trace h ∧
                          P.rangeRowsBindRepresentative
                            b view profile rangePolicy h :=
                      hTraceRangeRows hAccept hProjects hProfile
                        hRangePolicy hExtractsTrace hOpens hRelation
                        hTraceAssembles hTransparent hFiber
                    have _hSameTraceBlindRows :
                        P.blindingRowsBoundTraceRepresentative
                            b view profile policy trace h ∧
                          P.blindingRowsBoundRepresentative
                            b view profile policy h :=
                      hTraceBlindRows hAccept hProjects hProfile
                        hPolicy hExtractsTrace hOpens hRelation
                        hTraceAssembles hTransparent hFiber
                    cases
                        hNoExportSound hAccept hProjects hProfile with
                    | inl hNoExport =>
                        cases
                            hRoleAuthNoExportSound hAccept hProjects
                              hProfile with
                        | inl hRoleAuthNoExport =>
                            cases
                                hMaskFreshnessSound hAccept hProjects
                                  hProfile with
                            | inl hMaskFresh =>
                                cases hQromSound hAccept hProjects hProfile with
                                | inl hQrom =>
                                    cases
                                        hQromRewindSound hAccept hProjects
                                          hProfile with
                                    | inl hQromRewind =>
                                        cases
                                            hRangeRows hAccept hProjects
                                              hProfile hRangePolicy
                                              hTransparent hFiber with
                                        | inl hBoundRangeRows =>
                                            cases
                                                hRangeRowsToNorm hAccept
                                                  hProjects hProfile
                                                  hRangePolicy hTransparent
                                                  hFiber hBoundRangeRows with
                                            | inl hNorm =>
                                                cases
                                                    hBlindRows hAccept
                                                      hProjects hProfile
                                                      hPolicy hTransparent
                                                      hFiber with
                                                | inl hBoundBlindRows =>
                                                    cases
                                                        hBlindRowsToBound
                                                          hAccept hProjects
                                                          hProfile hPolicy
                                                          hTransparent hFiber
                                                          hBoundBlindRows with
                                                    | inl hBlind =>
                                                        exact Or.inl
                                                          ⟨hRole, hNoExport,
                                                            hRoleAuthNoExport,
                                                            hMaskFresh, hQrom,
                                                            hQromRewind,
                                                            hParent, h,
                                                            hTransparent,
                                                            hComplete, hFiber,
                                                            hLocal, hDescent,
                                                            hNorm, hBlind⟩
                                                    | inr hBreak =>
                                                        exact Or.inr hBreak
                                                | inr hBreak =>
                                                    exact Or.inr hBreak
                                            | inr hBreak =>
                                                exact Or.inr hBreak
                                        | inr hBreak =>
                                            exact Or.inr hBreak
                                    | inr hBreak =>
                                        exact Or.inr hBreak
                                | inr hBreak =>
                                    exact Or.inr hBreak
                            | inr hBreak =>
                                exact Or.inr hBreak
                        | inr hBreak =>
                            exact Or.inr hBreak
                    | inr hBreak =>
                        exact Or.inr hBreak
                | inr hBreak =>
                    exact Or.inr hBreak
            | inr hBreak =>
                exact Or.inr hBreak
        | inr hBreak =>
            exact Or.inr hBreak
    | inr hBreak =>
        exact Or.inr hBreak

/--
Atomic-gate active transparent interface: the same endpoint as
`transparent_view_full_interface_or_break`, but parent-trace extraction is also
derived from commitment extraction, masked-opening consistency, and relation
knowledge.
-/
theorem transparent_view_full_interface_from_atomic_gates_or_break
    (P : Predicates)
    (hParentSound : ParentRootRoleBindingSoundness P)
    (hRoleSound : RoleRestrictionSoundness P)
    (hCommitExtract : ParentTraceCommitmentExtractionSoundness P)
    (hDoubleTraceBinding : ParentTraceDoubleBindingSoundness P)
    (hMaskedConsistency : MaskedOpeningConsistencySoundness P)
    (hRelationKnowledge : ExtractedTraceRelationKnowledgeSoundness P)
    (hTraceAssemblyExists : TraceRepresentativeAssemblyExistenceSoundness P)
    (hTraceAssemblyRoles : TraceAssemblyRolesCompleteSoundness P)
    (hTraceAssemblyDerived : TraceAssemblyDerivedRolesSoundness P)
    (hTraceAssemblyFiber : TraceAssemblyFiberSoundness P)
    (hTraceAssemblyLocal : TraceAssemblyLocalValiditySoundness P)
    (hTraceAssemblyDescent : TraceAssemblyDescentConsistencySoundness P)
    (hNoExportSound : NoExportSoundness P)
    (hRoleAuthNoExportSound : RoleAuthenticationMaterialNoExportSoundness P)
    (hMaskFreshnessSound : MaskFreshnessSoundness P)
    (hQromSound : NativeBoundaryTraceQromScheduleSoundness P)
    (hQromRewindSound : NativeBoundaryTraceQromExtractorRewindSoundness P)
    (hTraceRangeRows : TraceRangeRowsBindRepresentativeSoundness P)
    (hRangeRows : HybridBase128RangeRowSoundness P)
    (hRangeRowsToNorm : HybridBase128RangeRowsToNormRangeSoundness P)
    (hTraceBlindRows : TraceBlindingRowsBoundRepresentativeSoundness P)
    (hBlindRows : BlindingNoiseBoundRowSoundness P)
    (hBlindRowsToBound : BlindingNoiseRowsToBoundSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
    {policy : BlindingNoisePolicy}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hRoleComplete : ProfileRoleComplete profile)
    (hSelected : UsesSelectedTraceKnowledgeModes profile)
    (hRangePolicy : UsesSelectedHybridBase128RangePolicy profile rangePolicy)
    (hPolicy : UsesSelectedBlindingNoisePolicy policy) :
    (P.roleRestrictionsAuthenticate b profile ∧
      P.noExportHolds b view profile ∧
        P.roleAuthenticationMaterialSimulatable b view profile ∧
          P.maskFreshnessHolds b view profile ∧
            P.qromScheduleBound b view profile ∧
              P.qromExtractorRewindBound b view profile ∧
                P.parentRootBindsPrivateRoles b profile ∧
                exists h,
                  P.transparentExtracts b view h ∧
                    P.extractedRolesComplete b profile h ∧
                    InFiber P b.context b h ∧
                      P.localValid b.context h ∧
                        P.descentConsistent b.context h ∧
                          P.normRangeValid b h ∧
                            P.blindingNoiseBounded b policy h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  transparent_view_full_interface_from_decomposed_or_break P
    hParentSound hRoleSound
    (commitment_extraction_and_relation_knowledge_imply_parent_trace_extraction
      P hCommitExtract hDoubleTraceBinding hMaskedConsistency hRelationKnowledge)
    hTraceAssemblyExists hTraceAssemblyRoles hTraceAssemblyDerived
    hTraceAssemblyFiber hTraceAssemblyLocal hTraceAssemblyDescent
    hNoExportSound hRoleAuthNoExportSound hMaskFreshnessSound hQromSound
    hQromRewindSound hTraceRangeRows hRangeRows hRangeRowsToNorm
    hTraceBlindRows hBlindRows hBlindRowsToBound
    hAccept hProjects hProfile hRoleComplete hSelected
    hRangePolicy hPolicy

/--
Selected-profile atomic transparent interface: the active closure endpoint with
role completeness and trace-knowledge modes derived from the selected Rust
masked-base128 two-view column profile.
-/
theorem transparent_view_full_interface_from_selected_profile_atomic_gates_or_break
    (P : Predicates)
    (hParentSound : ParentRootRoleBindingSoundness P)
    (hRoleSound : RoleRestrictionSoundness P)
    (hCommitExtract : ParentTraceCommitmentExtractionSoundness P)
    (hDoubleTraceBinding : ParentTraceDoubleBindingSoundness P)
    (hMaskedConsistency : MaskedOpeningConsistencySoundness P)
    (hRelationKnowledge : ExtractedTraceRelationKnowledgeSoundness P)
    (hTraceAssemblyExists : TraceRepresentativeAssemblyExistenceSoundness P)
    (hTraceAssemblyRoles : TraceAssemblyRolesCompleteSoundness P)
    (hTraceAssemblyDerived : TraceAssemblyDerivedRolesSoundness P)
    (hTraceAssemblyFiber : TraceAssemblyFiberSoundness P)
    (hTraceAssemblyLocal : TraceAssemblyLocalValiditySoundness P)
    (hTraceAssemblyDescent : TraceAssemblyDescentConsistencySoundness P)
    (hNoExportSound : NoExportSoundness P)
    (hRoleAuthNoExportSound : RoleAuthenticationMaterialNoExportSoundness P)
    (hMaskFreshnessSound : MaskFreshnessSoundness P)
    (hQromSound : NativeBoundaryTraceQromScheduleSoundness P)
    (hQromRewindSound : NativeBoundaryTraceQromExtractorRewindSoundness P)
    (hTraceRangeRows : TraceRangeRowsBindRepresentativeSoundness P)
    (hRangeRows : HybridBase128RangeRowSoundness P)
    (hRangeRowsToNorm : HybridBase128RangeRowsToNormRangeSoundness P)
    (hTraceBlindRows : TraceBlindingRowsBoundRepresentativeSoundness P)
    (hBlindRows : BlindingNoiseBoundRowSoundness P)
    (hBlindRowsToBound : BlindingNoiseRowsToBoundSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    {profile : ColumnProfile} {rangePolicy : HybridBase128RangePolicy}
    {policy : BlindingNoisePolicy}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view)
    (hProfile : ColumnProfileCompatibleWithBoundary b view profile)
    (hSelectedProfile : SelectedActiveColumnProfile profile)
    (hRangePolicy : UsesSelectedHybridBase128RangePolicy profile rangePolicy)
    (hPolicy : UsesSelectedBlindingNoisePolicy policy) :
    (P.roleRestrictionsAuthenticate b profile ∧
      P.noExportHolds b view profile ∧
        P.roleAuthenticationMaterialSimulatable b view profile ∧
          P.maskFreshnessHolds b view profile ∧
            P.qromScheduleBound b view profile ∧
              P.qromExtractorRewindBound b view profile ∧
                P.parentRootBindsPrivateRoles b profile ∧
                exists h,
                  P.transparentExtracts b view h ∧
                    P.extractedRolesComplete b profile h ∧
                    InFiber P b.context b h ∧
                      P.localValid b.context h ∧
                        P.descentConsistent b.context h ∧
                          P.normRangeValid b h ∧
                            P.blindingNoiseBounded b policy h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  transparent_view_full_interface_from_atomic_gates_or_break P
    hParentSound hRoleSound hCommitExtract hDoubleTraceBinding
    hMaskedConsistency hRelationKnowledge hTraceAssemblyExists hTraceAssemblyRoles
    hTraceAssemblyDerived hTraceAssemblyFiber hTraceAssemblyLocal
    hTraceAssemblyDescent hNoExportSound hRoleAuthNoExportSound
    hMaskFreshnessSound hQromSound hQromRewindSound hTraceRangeRows
    hRangeRows hRangeRowsToNorm hTraceBlindRows hBlindRows hBlindRowsToBound
    hAccept hProjects hProfile
    (selected_active_column_profile_role_complete hSelectedProfile)
    (selected_active_column_profile_trace_modes hSelectedProfile)
    hRangePolicy hPolicy

/--
Selected transparent proof-family account-admission adapter: once accepted
account creation is known to expose the selected transparent view/profile/
policy tuple, the selected-profile atomic endpoint closes the native admission
surface or exports the same named boundary break.
-/
theorem selected_transparent_view_atomic_gates_imply_account_admission
    (P : Predicates)
    (hCreateToView : AccountCreateSelectedTransparentViewSoundness P)
    (hParentSound : ParentRootRoleBindingSoundness P)
    (hRoleSound : RoleRestrictionSoundness P)
    (hCommitExtract : ParentTraceCommitmentExtractionSoundness P)
    (hDoubleTraceBinding : ParentTraceDoubleBindingSoundness P)
    (hMaskedConsistency : MaskedOpeningConsistencySoundness P)
    (hRelationKnowledge : ExtractedTraceRelationKnowledgeSoundness P)
    (hTraceAssemblyExists : TraceRepresentativeAssemblyExistenceSoundness P)
    (hTraceAssemblyRoles : TraceAssemblyRolesCompleteSoundness P)
    (hTraceAssemblyDerived : TraceAssemblyDerivedRolesSoundness P)
    (hTraceAssemblyFiber : TraceAssemblyFiberSoundness P)
    (hTraceAssemblyLocal : TraceAssemblyLocalValiditySoundness P)
    (hTraceAssemblyDescent : TraceAssemblyDescentConsistencySoundness P)
    (hNoExportSound : NoExportSoundness P)
    (hRoleAuthNoExportSound : RoleAuthenticationMaterialNoExportSoundness P)
    (hMaskFreshnessSound : MaskFreshnessSoundness P)
    (hQromSound : NativeBoundaryTraceQromScheduleSoundness P)
    (hQromRewindSound : NativeBoundaryTraceQromExtractorRewindSoundness P)
    (hTraceRangeRows : TraceRangeRowsBindRepresentativeSoundness P)
    (hRangeRows : HybridBase128RangeRowSoundness P)
    (hRangeRowsToNorm : HybridBase128RangeRowsToNormRangeSoundness P)
    (hTraceBlindRows : TraceBlindingRowsBoundRepresentativeSoundness P)
    (hBlindRows : BlindingNoiseBoundRowSoundness P)
    (hBlindRowsToBound : BlindingNoiseRowsToBoundSoundness P) :
    AccountAdmissionSoundness P := by
  intro c b hCreate
  rcases hCreateToView hCreate with
    ⟨hContext, view, profile, rangePolicy, policy, hAccept, hProjects,
      hProfile, hSelectedProfile, hRangePolicy, hPolicy⟩
  subst c
  cases
      transparent_view_full_interface_from_selected_profile_atomic_gates_or_break
        P hParentSound hRoleSound hCommitExtract hDoubleTraceBinding
        hMaskedConsistency hRelationKnowledge hTraceAssemblyExists
        hTraceAssemblyRoles hTraceAssemblyDerived hTraceAssemblyFiber
        hTraceAssemblyLocal hTraceAssemblyDescent hNoExportSound
        hRoleAuthNoExportSound hMaskFreshnessSound hQromSound
        hQromRewindSound hTraceRangeRows hRangeRows hRangeRowsToNorm
        hTraceBlindRows hBlindRows hBlindRowsToBound
        hAccept hProjects hProfile hSelectedProfile hRangePolicy hPolicy with
  | inl hClosed =>
      rcases hClosed with
        ⟨_hRole, _hNoExport, _hRoleAuthNoExport, _hMaskFresh, _hQrom,
          _hQromRewind, _hParent, h, _hTransparent, _hComplete,
          hFiber, hLocal, hDescent, _hNorm, _hBlind⟩
      exact Or.inl ⟨h, hFiber, hLocal, hDescent⟩
  | inr hBreak =>
      exact Or.inr hBreak

/--
If the native admission interface holds, every boundary is either explained by
a locally valid descent-consistent hidden representative or exports a named
break.
-/
theorem account_admission_soundness_or_break
    (P : Predicates)
    (hSound : AccountAdmissionSoundness P)
    {c : Context} {b : Boundary} :
    P.accountCreateAccepts c b ->
    (exists h,
      InFiber P c b h ∧
        P.localValid c h ∧
          P.descentConsistent c h) ∨
    (exists breakEvent, P.boundaryBreakFor c b breakEvent) :=
  hSound

/--
Accepted transparent public views over a native boundary extract a hidden
representative in that boundary fiber, or export a named break.
-/
theorem transparent_view_extracts_fiber_or_break
    (P : Predicates)
    (hSound : TwoViewExtractionSoundness P)
    {b : Boundary} {view : TransparentPublicView}
    (hAccept : P.transparentAccepts b view)
    (hProjects : TransparentViewProjectsFrom b view) :
    (exists h,
      P.transparentExtracts b view h ∧
        InFiber P b.context b h ∧
          P.localValid b.context h ∧
            P.descentConsistent b.context h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects

/--
Soundness interface for the reviewed-Fq12 hidden-source payload used by the
active deployment path.

This is the critical P2FS-001 obligation: an accepted reviewed-Fq12 payload
must either extract a locally valid, descent-consistent representative in the
native boundary fiber, or export a named break.  The break may be hidden-source
soundness, payload binding, checked-core binding, or domain/QROM separation,
depending on the concrete failure.
-/
def ReviewedFq12HiddenSourcePayloadSoundness (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        (exists h,
          P.reviewedFq12PayloadExtracts b payload h ∧
            InFiber P b.context b h ∧
              P.localValid b.context h ∧
                P.descentConsistent b.context h) ∨
        (exists breakEvent,
          P.boundaryBreakFor b.context b breakEvent)

/--
Extraction-certificate availability for accepted reviewed-Fq12 payloads.

This is the implementation bridge now present in Rust: an accepted payload that
projects from the native boundary emits an extraction certificate, or the bridge
names a break before acceptance can be promoted.
-/
def ReviewedFq12ExtractionCertificateAvailability
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        (exists cert,
          P.reviewedFq12PayloadHasExtractionCertificate b payload cert ∧
            P.reviewedFq12ExtractionCertificateValid b payload cert) ∨
        (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
The remaining P2FS-001 theorem obligation after the Rust bridge emits a
selected extraction certificate: a valid certificate assembles a native hidden
representative in the boundary fiber, or exposes a named break.
-/
def ReviewedFq12ExtractionCertificateSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            (exists h,
              P.reviewedFq12CertificateExtracts b payload cert h ∧
                P.reviewedFq12PayloadExtracts b payload h ∧
                  InFiber P b.context b h ∧
                    P.localValid b.context h ∧
                      P.descentConsistent b.context h) ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
The selected reviewed-Fq12 certificate must lift to the missing native private
roles. This is the newly isolated blocker: selected projection evidence is not
itself the full native representative opening.
-/
def ReviewedFq12SelectedCertificateRoleLiftSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            (exists opening,
              P.reviewedFq12CertificateLiftsToOpening
                b payload cert opening) ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Field-level binding required for the full native opening role-lift
certificate.  The selected extraction certificate and the role-lift certificate
must agree on payload, boundary, slot, and both statement digests.
-/
def ReviewedFq12RoleLiftCertificateBindsExtraction
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate) : Prop :=
  liftCert.payloadBindingDigest = cert.payloadBindingDigest ∧
    liftCert.extractionCertificateDigest = cert.extractionCertificateDigest ∧
    liftCert.creationProofBindingDigest = payload.creationProofBindingDigest ∧
    liftCert.nativeBoundaryDigest = payload.nativeBoundaryDigest ∧
    liftCert.nativeBoundaryDigest = cert.nativeBoundaryDigest ∧
    liftCert.slotId = payload.slotId ∧
    liftCert.slotId = cert.slotId ∧
    liftCert.creationStatementDigest = payload.creationStatementDigest ∧
    liftCert.creationStatementDigest = cert.creationStatementDigest ∧
    liftCert.reviewedStatementDigest = payload.statementDigest ∧
    liftCert.reviewedStatementDigest = cert.reviewedStatementDigest ∧
    liftCert.opening.nativeBoundaryDigest =
      liftCert.nativeBoundaryDigest ∧
    liftCert.opening.nativeRoleRootCommitment =
      liftCert.nativeRoleRootCommitment ∧
    liftCert.opening.selectedAccountProjectionDigest =
      cert.accountProjectionDigest ∧
    liftCert.opening.randomizerDigest = payload.randomizerCommitment ∧
    liftCert.opening.openingTupleDigest = payload.openingCommitment ∧
    liftCert.opening.padSourceDigest = payload.padSeedDigest ∧
    liftCert.opening.projectionRelationDigest =
      payload.projectionCarrierDigest

/--
Executable bridge availability for the new role-lift object: every valid
selected extraction certificate has a full-native-opening role certificate, or
the bridge names a break.
-/
def ReviewedFq12RoleLiftCertificateAvailability
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            (exists liftCert,
              P.reviewedFq12PayloadHasRoleLiftCertificate
                b payload cert liftCert ∧
                P.reviewedFq12RoleLiftCertificateValid
                  b payload cert liftCert) ∨
            (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Soundness of the role-lift certificate: a valid certificate opens to the
`FullNativeRepresentativeOpening` it carries, or exports a named break.
-/
def ReviewedFq12RoleLiftCertificateSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {liftCert : ReviewedFq12FullNativeOpeningCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12PayloadHasRoleLiftCertificate b payload cert liftCert ->
              P.reviewedFq12RoleLiftCertificateValid b payload cert liftCert ->
                ReviewedFq12RoleLiftCertificateBindsExtraction
                    payload cert liftCert ->
                  (exists opening,
                    P.reviewedFq12RoleLiftCertificateOpens
                        b payload cert liftCert opening ∧
                      P.reviewedFq12CertificateLiftsToOpening
                        b payload cert opening) ∨
                  (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Role-root binding assumption for Option 2.

The Rust bridge now pins the accepted payload to an account-creation proof
binding digest and pins the role-lift certificate to a native role root.  This
assumption is the remaining cryptographic binding statement: a valid
role-lift certificate with matching extraction fields opens the carried
`FullNativeRepresentativeOpening`, or exposes a named binding break.
-/
def ReviewedFq12NativeRoleRootBindingSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {liftCert : ReviewedFq12FullNativeOpeningCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12PayloadHasRoleLiftCertificate b payload cert liftCert ->
              P.reviewedFq12RoleLiftCertificateValid b payload cert liftCert ->
                ReviewedFq12RoleLiftCertificateBindsExtraction
                    payload cert liftCert ->
                  P.reviewedFq12RoleLiftCertificateOpens
                    b payload cert liftCert liftCert.opening ∨
                  (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
If the native role root opens the carried full opening, and that opened tuple
is accepted by the certificate-lift relation, then the role-lift certificate
soundness obligation is discharged.
-/
theorem reviewed_fq12_role_lift_soundness_from_native_role_root
    (P : Predicates)
    (hRoot : ReviewedFq12NativeRoleRootBindingSoundness P)
    (hLift :
      forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
          {cert : ReviewedFq12HiddenRelationExtractionCertificate}
          {liftCert : ReviewedFq12FullNativeOpeningCertificate},
        P.reviewedFq12PayloadAccepts b payload ->
          P.reviewedFq12PayloadProjectsFrom b payload ->
            P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
              P.reviewedFq12ExtractionCertificateValid b payload cert ->
                P.reviewedFq12PayloadHasRoleLiftCertificate b payload cert liftCert ->
                  P.reviewedFq12RoleLiftCertificateValid b payload cert liftCert ->
                    ReviewedFq12RoleLiftCertificateBindsExtraction
                        payload cert liftCert ->
                      P.reviewedFq12RoleLiftCertificateOpens
                        b payload cert liftCert liftCert.opening ->
                        P.reviewedFq12CertificateLiftsToOpening
                          b payload cert liftCert.opening) :
    ReviewedFq12RoleLiftCertificateSoundness P := by
  intro b payload cert liftCert hAccept hProjects hHas hValid
      hHasLift hLiftValid hBinds
  rcases
      hRoot hAccept hProjects hHas hValid hHasLift hLiftValid hBinds with
    hOpens | hBreak
  · exact Or.inl
      ⟨liftCert.opening, hOpens,
        hLift hAccept hProjects hHas hValid hHasLift hLiftValid
          hBinds hOpens⟩
  · exact Or.inr hBreak

/--
Package for the Option 2 role-root route.

This is the active decomposition of the P2FS-001 role-lift step: executable
availability of a full-native-opening certificate, binding of its fields to
the selected extraction evidence, native role-root opening soundness, and the
local lift relation from that opened tuple to `reviewedFq12CertificateLiftsToOpening`.
-/
structure ReviewedFq12RoleRootBridgeCertificate
    (P : Predicates) where
  roleLiftAvailability :
    ReviewedFq12RoleLiftCertificateAvailability P
  roleLiftPayloadBinding :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
        P.reviewedFq12PayloadProjectsFrom b payload ->
          P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
            P.reviewedFq12ExtractionCertificateValid b payload cert ->
              P.reviewedFq12PayloadHasRoleLiftCertificate b payload cert liftCert ->
                P.reviewedFq12RoleLiftCertificateValid b payload cert liftCert ->
                  ReviewedFq12RoleLiftCertificateBindsExtraction
                    payload cert liftCert
  nativeRoleRootSoundness :
    ReviewedFq12NativeRoleRootBindingSoundness P
  openedCertificateLifts :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
        P.reviewedFq12PayloadProjectsFrom b payload ->
          P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
            P.reviewedFq12ExtractionCertificateValid b payload cert ->
              P.reviewedFq12PayloadHasRoleLiftCertificate b payload cert liftCert ->
                P.reviewedFq12RoleLiftCertificateValid b payload cert liftCert ->
                  ReviewedFq12RoleLiftCertificateBindsExtraction
                      payload cert liftCert ->
                    P.reviewedFq12RoleLiftCertificateOpens
                      b payload cert liftCert liftCert.opening ->
                      P.reviewedFq12CertificateLiftsToOpening
                        b payload cert liftCert.opening

/--
The original selected-certificate role-lift obligation follows from
availability of the full-native-opening certificate plus its soundness.
-/
theorem reviewed_fq12_selected_role_lift_from_opening_certificate
    (P : Predicates)
    (hAvail : ReviewedFq12RoleLiftCertificateAvailability P)
    (hSound : ReviewedFq12RoleLiftCertificateSoundness P)
    (hBind :
      forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
          {cert : ReviewedFq12HiddenRelationExtractionCertificate}
          {liftCert : ReviewedFq12FullNativeOpeningCertificate},
        P.reviewedFq12PayloadAccepts b payload ->
          P.reviewedFq12PayloadProjectsFrom b payload ->
            P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
              P.reviewedFq12ExtractionCertificateValid b payload cert ->
                P.reviewedFq12PayloadHasRoleLiftCertificate b payload cert liftCert ->
                  P.reviewedFq12RoleLiftCertificateValid b payload cert liftCert ->
                    ReviewedFq12RoleLiftCertificateBindsExtraction
                      payload cert liftCert) :
    ReviewedFq12SelectedCertificateRoleLiftSoundness P := by
  intro b payload cert hAccept hProjects hHas hValid
  rcases hAvail hAccept hProjects hHas hValid with hLiftCert | hBreak
  · rcases hLiftCert with ⟨liftCert, hHasLift, hLiftValid⟩
    have hBinds :=
      hBind hAccept hProjects hHas hValid hHasLift hLiftValid
    rcases
        hSound hAccept hProjects hHas hValid hHasLift hLiftValid hBinds with
      hOpening | hBreak
    · rcases hOpening with ⟨opening, _hOpens, hLifts⟩
      exact Or.inl ⟨opening, hLifts⟩
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
The Option 2 role-root bridge supplies the selected-certificate role-lift
soundness obligation used by the extraction bridge.
-/
theorem reviewed_fq12_selected_role_lift_from_role_root_bridge
    (P : Predicates)
    (C : ReviewedFq12RoleRootBridgeCertificate P) :
    ReviewedFq12SelectedCertificateRoleLiftSoundness P :=
  reviewed_fq12_selected_role_lift_from_opening_certificate
    P C.roleLiftAvailability
    (reviewed_fq12_role_lift_soundness_from_native_role_root
      P C.nativeRoleRootSoundness C.openedCertificateLifts)
    C.roleLiftPayloadBinding

/--
A full native representative opening must assemble into a representative in
the boundary fiber and be linked back to the reviewed-Fq12 payload extraction
relation.
-/
def ReviewedFq12FullNativeOpeningSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              (exists h,
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ∧
                  P.reviewedFq12CertificateExtracts b payload cert h ∧
                    P.reviewedFq12PayloadExtracts b payload h ∧
                      InFiber P b.context b h ∧
                        P.localValid b.context h ∧
                          P.descentConsistent b.context h) ∨
              (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Field-level interpretation of a full native opening.

The reviewed-Fq12 role-root bridge produces digest handles for every private
native role.  Closing the proof requires an explicit representative whose
coordinates are those handles, not merely an abstract statement that some
representative exists.
-/
def FullNativeOpeningMatchesRepresentative
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative) : Prop :=
  h.nativeBoundaryDigest = opening.nativeBoundaryDigest ∧
  h.localSection = opening.localSectionDigest ∧
    h.packedResidues = opening.packedResiduesDigest ∧
    h.randomizer = opening.randomizerDigest ∧
      h.selectedAccountProjection =
        opening.selectedAccountProjectionDigest ∧
        h.padSource = opening.padSourceDigest ∧
          h.openingTuple = opening.openingTupleDigest ∧
            h.projectionRelation = opening.projectionRelationDigest ∧
              h.lifecycleSlot = opening.lifecycleSlotDigest ∧
                h.traceWitness = opening.traceWitnessDigest ∧
                  h.gauge = opening.gaugeDigest

/--
Option 1 boundary: operational private-fiber digests are irrelevant to the
hidden-source opening semantics.

Changing authorization-local, operational randomizer, or wallet-recovery
digests cannot change whether an already fixed full native opening matches an
already fixed hidden representative. Those digests must still be handled by
lifecycle/signing-oracle/no-export obligations; they are just outside P2FS-001.
-/
theorem full_native_opening_match_ignores_operational_private_fiber
    {opening : FullNativeRepresentativeOpening}
    {h : HiddenRepresentative}
    (_opsA _opsB : OperationalPrivateFiberDigests)
    (hMatches : FullNativeOpeningMatchesRepresentative opening h) :
    FullNativeOpeningMatchesRepresentative opening h :=
  hMatches

/--
Field-level interpretation of a native representative assembly certificate.

This links the certificate's representative digest to the semantic
`HiddenRepresentative`, so later compatibility certificates cannot bind only a
free-standing digest while extracting a different representative.
-/
def AssemblyCertificateMatchesRepresentative
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (h : HiddenRepresentative) : Prop :=
  h.representativeDigest = assemblyCert.representativeDigest

/--
Field-level binding required for the native representative assembly
certificate.

This prevents an assembly certificate from being replayed across a different
payload, extraction certificate, creation binding, boundary, slot, statement
role, or opening tuple.
-/
def ReviewedFq12AssemblyCertificateBindsOpening
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop :=
  assemblyCert.payloadBindingDigest = cert.payloadBindingDigest ∧
    assemblyCert.extractionCertificateDigest =
      cert.extractionCertificateDigest ∧
    assemblyCert.creationProofBindingDigest =
      payload.creationProofBindingDigest ∧
    assemblyCert.nativeBoundaryDigest = payload.nativeBoundaryDigest ∧
    assemblyCert.nativeBoundaryDigest = cert.nativeBoundaryDigest ∧
    assemblyCert.slotId = payload.slotId ∧
    assemblyCert.slotId = cert.slotId ∧
    assemblyCert.creationStatementDigest =
      payload.creationStatementDigest ∧
    assemblyCert.creationStatementDigest =
      cert.creationStatementDigest ∧
    assemblyCert.reviewedStatementDigest = payload.statementDigest ∧
    assemblyCert.reviewedStatementDigest = cert.reviewedStatementDigest ∧
    assemblyCert.nativeBoundaryDigest = opening.nativeBoundaryDigest ∧
    assemblyCert.nativeRoleRootCommitment =
      opening.nativeRoleRootCommitment ∧
    assemblyCert.fullOpeningDigest = opening.openingDigest ∧
    assemblyCert.localSectionDigest = opening.localSectionDigest ∧
    assemblyCert.packedResiduesDigest = opening.packedResiduesDigest ∧
    assemblyCert.randomizerDigest = opening.randomizerDigest ∧
    assemblyCert.selectedAccountProjectionDigest =
      opening.selectedAccountProjectionDigest ∧
    assemblyCert.padSourceDigest = opening.padSourceDigest ∧
    assemblyCert.openingTupleDigest = opening.openingTupleDigest ∧
    assemblyCert.projectionRelationDigest =
      opening.projectionRelationDigest ∧
    assemblyCert.lifecycleSlotDigest = opening.lifecycleSlotDigest ∧
    assemblyCert.traceWitnessDigest = opening.traceWitnessDigest ∧
    assemblyCert.gaugeDigest = opening.gaugeDigest ∧
    assemblyCert.opening = opening

/--
The aggregate bridge binding confirms that the aggregate object and the
assembly certificate agree on the opened role-lift tuple identifiers it
actually carries.

It does not prove full assembly-frame binding: the aggregate bridge is a final
audit object and does not carry every native role coordinate.
-/
theorem reviewed_fq12_assembly_opening_agrees_from_full_opening_bridge_digest
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {bridgeCert : ReviewedFq12FullOpeningBridgeDigestCertificate}
    (hBridge :
      ReviewedFq12FullOpeningBridgeDigestCertificateBinds
        payload cert liftCert assemblyCert compatCert bridgeCert) :
    assemblyCert.opening = liftCert.opening ∧
      assemblyCert.fullOpeningDigest = liftCert.opening.openingDigest ∧
        assemblyCert.selectedAccountProjectionDigest =
          liftCert.opening.selectedAccountProjectionDigest := by
  rcases hBridge with
    ⟨_hPayloadCert, _hPayloadLift, _hPayloadAssembly, _hPayloadCompat,
      _hExtractionCert, _hExtractionLift, _hExtractionAssembly,
      _hExtractionCompat, _hCreationPayload, _hCreationLift,
      _hCreationAssembly, _hRoleLiftDigest, _hRoleAssemblyDigest,
      _hRoleCompatDigest, _hAssemblyDigest, _hAssemblyCompat,
      _hCompatDigest, _hNativePayload, _hNativeCert, _hNativeLift,
      _hNativeLiftOpening, _hNativeAssembly, _hNativeCompat, _hSlotPayload, _hSlotCert,
      _hSlotLift, _hSlotAssembly, _hSlotCompat, _hCreationStmtPayload,
      _hCreationStmtCert, _hCreationStmtLift, _hCreationStmtAssembly,
      _hCreationStmtCompat, _hReviewedStmtPayload, _hReviewedStmtCert,
      _hReviewedStmtLift, _hReviewedStmtAssembly, _hReviewedStmtCompat,
      _hRootLift, _hRootOpening, _hRootAssembly, hOpeningEq,
      hFullLift, hFullAssemblyDigest, _hFullAssemblyOpening, _hFullCompat,
      _hProjectionCert, hProjectionLift, hProjectionAssemblyDigest,
      _hProjectionAssemblyOpening, _hProjectionCompat,
      _hRepresentativeAssembly, _hRepresentativeCompat⟩
  exact
    ⟨hOpeningEq,
      hFullAssemblyDigest.symm.trans hFullLift,
      hProjectionAssemblyDigest.symm.trans hProjectionLift⟩

/--
Executable availability of the native representative assembly certificate for
an opened full native tuple.
-/
def ReviewedFq12RepresentativeAssemblyCertificateAvailability
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              (exists assemblyCert,
                P.reviewedFq12OpeningHasAssemblyCertificate
                    b payload cert opening assemblyCert ∧
                  P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert) ∨
              (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Soundness of the native representative assembly certificate.

A valid certificate must assemble the opened tuple into a representative with
matching role coordinates, in the native boundary fiber, satisfying the local
and descent laws; otherwise the verifier must export a named break.
-/
def ReviewedFq12RepresentativeAssemblyCertificateSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    (exists h,
                      FullNativeOpeningMatchesRepresentative opening h ∧
                        AssemblyCertificateMatchesRepresentative
                            assemblyCert h ∧
                        P.reviewedFq12AssemblyCertificateAssembles
                            b payload cert opening assemblyCert h ∧
                          P.fullNativeOpeningAssemblesRepresentative
                              b payload cert opening h ∧
                            InFiber P b.context b h ∧
                              P.localValid b.context h ∧
                                P.descentConsistent b.context h) ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

/--
Field-construction part of assembly soundness.

This is the executable/certificate obligation: a valid assembly certificate
bound to an opened tuple constructs a representative with the opened role
coordinates and the certificate's representative digest. It does not by itself
claim the representative is in the native fiber or satisfies local/descent
laws.
-/
def ReviewedFq12RepresentativeAssemblyConstructionSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    (exists h,
                      FullNativeOpeningMatchesRepresentative opening h ∧
                        AssemblyCertificateMatchesRepresentative
                            assemblyCert h ∧
                          P.reviewedFq12AssemblyCertificateAssembles
                              b payload cert opening assemblyCert h ∧
                            P.fullNativeOpeningAssemblesRepresentative
                                b payload cert opening h) ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

/--
Native-validity part of assembly soundness.

Once the certificate has assembled a concrete representative from the opened
role coordinates, the native boundary relation must place that representative
in the boundary fiber and prove local/descent validity, or name a break.
-/
def ReviewedFq12AssembledRepresentativeNativeValiditySoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
      {h : HiddenRepresentative},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    FullNativeOpeningMatchesRepresentative opening h ->
                      AssemblyCertificateMatchesRepresentative
                          assemblyCert h ->
                        P.reviewedFq12AssemblyCertificateAssembles
                            b payload cert opening assemblyCert h ->
                          P.fullNativeOpeningAssemblesRepresentative
                              b payload cert opening h ->
                            (InFiber P b.context b h ∧
                              P.localValid b.context h ∧
                                P.descentConsistent b.context h) ∨
                            (exists breakEvent,
                              P.boundaryBreakFor b.context b breakEvent)

/--
The two smaller assembly obligations recover the original certificate
soundness surface.
-/
theorem reviewed_fq12_representative_assembly_soundness_from_parts
    (P : Predicates)
    (hConstruct :
      ReviewedFq12RepresentativeAssemblyConstructionSoundness P)
    (hNative :
      ReviewedFq12AssembledRepresentativeNativeValiditySoundness P) :
    ReviewedFq12RepresentativeAssemblyCertificateSoundness P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas hValid
    hLifts hHasAssembly hAssemblyValid hBinds
  rcases
      hConstruct hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid hBinds with
    hConstructed | hBreak
  · rcases hConstructed with
      ⟨h, hMatches, hAssemblyMatches, hCertAssembles,
        hOpeningAssembles⟩
    rcases
        hNative hAccept hProjects hHas hValid hLifts hHasAssembly
          hAssemblyValid hBinds hMatches hAssemblyMatches hCertAssembles
          hOpeningAssembles with
      hNativeValid | hBreak
    · rcases hNativeValid with ⟨hFiber, hLocal, hDescent⟩
      exact Or.inl
        ⟨h, hMatches, hAssemblyMatches, hCertAssembles,
          hOpeningAssembles, hFiber, hLocal, hDescent⟩
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
Assembly soundness for a full native opening, separated from extraction
compatibility.

This is the concrete native-side obligation behind
`fullNativeOpeningAssemblesRepresentative`: the opening must assemble into a
representative with matching role coordinates, in the native boundary fiber,
and satisfying the local/descent laws.
-/
def ReviewedFq12FullNativeOpeningAssemblySoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              (exists h,
                FullNativeOpeningMatchesRepresentative opening h ∧
                  P.fullNativeOpeningAssemblesRepresentative
                      b payload cert opening h ∧
                    InFiber P b.context b h ∧
                      P.localValid b.context h ∧
                        P.descentConsistent b.context h) ∨
              (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Assembly-certificate availability and soundness instantiate the native
full-opening assembly obligation.
-/
theorem reviewed_fq12_full_native_opening_assembly_from_certificate
    (P : Predicates)
    (hAvail : ReviewedFq12RepresentativeAssemblyCertificateAvailability P)
    (hSound : ReviewedFq12RepresentativeAssemblyCertificateSoundness P)
    (hBind :
      forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
          {cert : ReviewedFq12HiddenRelationExtractionCertificate}
          {opening : FullNativeRepresentativeOpening}
          {assemblyCert :
            ReviewedFq12NativeRepresentativeAssemblyCertificate},
        P.reviewedFq12PayloadAccepts b payload ->
          P.reviewedFq12PayloadProjectsFrom b payload ->
            P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
              P.reviewedFq12ExtractionCertificateValid b payload cert ->
                P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
                  P.reviewedFq12OpeningHasAssemblyCertificate
                      b payload cert opening assemblyCert ->
                    P.reviewedFq12AssemblyCertificateValid
                        b payload cert opening assemblyCert ->
                      ReviewedFq12AssemblyCertificateBindsOpening
                        payload cert opening assemblyCert) :
    ReviewedFq12FullNativeOpeningAssemblySoundness P := by
  intro b payload cert opening hAccept hProjects hHas hValid hLifts
  rcases hAvail hAccept hProjects hHas hValid hLifts with
    hAssemblyCert | hBreak
  · rcases hAssemblyCert with ⟨assemblyCert, hHasAssembly, hAssemblyValid⟩
    have hBinds :=
      hBind hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid
    rcases
        hSound hAccept hProjects hHas hValid hLifts hHasAssembly
          hAssemblyValid hBinds with
      hAssembled | hBreak
    · rcases hAssembled with
        ⟨h, hMatches, _hAssemblyCertMatches,
          _hAssemblyCertAssembles, hAssembles,
          hFiber, hLocal, hDescent⟩
      exact Or.inl
        ⟨h, hMatches, hAssembles, hFiber, hLocal, hDescent⟩
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
Compatibility between the native representative assembled from the full
opening and the reviewed-Fq12 extraction relations.

This is intentionally separate from assembly: if a future implementation can
assemble a native representative but cannot show that the reviewed extractor
and payload relation name the same representative, the proof stops here rather
than hiding that mismatch inside the assembly predicate.
-/
def ReviewedFq12FullNativeOpeningExtractionCompatibility
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening h ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ->
                  InFiber P b.context b h ->
                    P.localValid b.context h ->
                      P.descentConsistent b.context h ->
                        (P.reviewedFq12CertificateExtracts b payload cert h ∧
                          P.reviewedFq12PayloadExtracts b payload h) ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
Field-level binding required for the extraction-compatibility certificate.

The certificate must agree with the payload, extraction certificate, opening,
and assembled representative on the identifiers that define "same
representative".
-/
def ReviewedFq12ExtractionCompatibilityCertificateBinds
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop :=
  compatCert.payloadBindingDigest = cert.payloadBindingDigest ∧
    compatCert.extractionCertificateDigest =
      cert.extractionCertificateDigest ∧
    compatCert.nativeBoundaryDigest = payload.nativeBoundaryDigest ∧
    compatCert.nativeBoundaryDigest = cert.nativeBoundaryDigest ∧
    compatCert.nativeBoundaryDigest = opening.nativeBoundaryDigest ∧
    compatCert.slotId = payload.slotId ∧
    compatCert.slotId = cert.slotId ∧
    compatCert.creationStatementDigest =
      payload.creationStatementDigest ∧
    compatCert.creationStatementDigest =
      cert.creationStatementDigest ∧
    compatCert.reviewedStatementDigest = payload.statementDigest ∧
    compatCert.reviewedStatementDigest = cert.reviewedStatementDigest ∧
    compatCert.fullOpeningDigest = opening.openingDigest ∧
    compatCert.selectedAccountProjectionDigest =
      opening.selectedAccountProjectionDigest ∧
    compatCert.selectedAccountProjectionDigest =
      cert.accountProjectionDigest ∧
    compatCert.representativeDigest = h.representativeDigest ∧
    FullNativeOpeningMatchesRepresentative opening h

/--
The aggregate bridge binding recovers the compatibility certificate's
field-level binding once assembly soundness has identified the representative
and opening role coordinates.
-/
theorem reviewed_fq12_compatibility_binding_from_full_opening_bridge_digest
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {bridgeCert : ReviewedFq12FullOpeningBridgeDigestCertificate}
    {h : HiddenRepresentative}
    (hBridge :
      ReviewedFq12FullOpeningBridgeDigestCertificateBinds
        payload cert liftCert assemblyCert compatCert bridgeCert)
    (hAssemblyMatches :
      AssemblyCertificateMatchesRepresentative assemblyCert h)
    (hOpeningMatches :
      FullNativeOpeningMatchesRepresentative liftCert.opening h) :
    ReviewedFq12ExtractionCompatibilityCertificateBinds
      payload cert liftCert.opening h compatCert := by
  rcases hBridge with
    ⟨hPayloadCert, _hPayloadLift, _hPayloadAssembly, hPayloadCompat,
      hExtractionCert, _hExtractionLift, _hExtractionAssembly,
      hExtractionCompat, _hCreationPayload, _hCreationLift,
      _hCreationAssembly, _hRoleLiftDigest, _hRoleAssemblyDigest,
      _hRoleCompatDigest, _hAssemblyDigest, _hAssemblyCompat,
      _hCompatDigest, hNativePayload, hNativeCert, _hNativeLift,
      _hNativeLiftOpening, _hNativeAssembly, hNativeCompat, hSlotPayload, hSlotCert,
      _hSlotLift, _hSlotAssembly, hSlotCompat, hCreationStmtPayload,
      hCreationStmtCert, _hCreationStmtLift, _hCreationStmtAssembly,
      hCreationStmtCompat, hReviewedStmtPayload, hReviewedStmtCert,
      _hReviewedStmtLift, _hReviewedStmtAssembly, hReviewedStmtCompat,
      _hRootLift, _hRootOpening, _hRootAssembly, _hOpeningEq,
      hFullLift, _hFullAssemblyDigest, _hFullAssemblyOpening,
      hFullCompat, hProjectionCert, hProjectionLift,
      _hProjectionAssemblyDigest, _hProjectionAssemblyOpening,
      hProjectionCompat,
      hRepresentativeAssembly, hRepresentativeCompat⟩
  refine
    ⟨hPayloadCompat.symm.trans hPayloadCert,
      hExtractionCompat.symm.trans hExtractionCert,
      hNativeCompat.symm.trans hNativePayload,
      hNativeCompat.symm.trans hNativeCert,
      hNativeCompat.symm.trans _hNativeLiftOpening,
      hSlotCompat.symm.trans hSlotPayload,
      hSlotCompat.symm.trans hSlotCert,
      hCreationStmtCompat.symm.trans hCreationStmtPayload,
      hCreationStmtCompat.symm.trans hCreationStmtCert,
      hReviewedStmtCompat.symm.trans hReviewedStmtPayload,
      hReviewedStmtCompat.symm.trans hReviewedStmtCert,
      hFullCompat.symm.trans hFullLift,
      hProjectionCompat.symm.trans hProjectionLift,
      hProjectionCompat.symm.trans hProjectionCert,
      ?_, hOpeningMatches⟩
  exact hRepresentativeCompat.symm.trans
    (hRepresentativeAssembly.trans hAssemblyMatches.symm)

/--
Executable binding obligation for the aggregate full-opening bridge object.

This is the theorem-side name for the Rust verifier check on
`Path2ReviewedFq12FullOpeningBridgeCertificateV1`: when the aggregate object is
present and valid, its fields must instantiate the aggregate binding contract.
-/
def ReviewedFq12FullOpeningBridgeDigestCertificateBinding
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {liftCert : ReviewedFq12FullNativeOpeningCertificate}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
      {bridgeCert : ReviewedFq12FullOpeningBridgeDigestCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12PayloadHasRoleLiftCertificate
                b payload cert liftCert ->
              P.reviewedFq12RoleLiftCertificateValid
                  b payload cert liftCert ->
                P.reviewedFq12OpeningHasAssemblyCertificate
                    b payload cert liftCert.opening assemblyCert ->
                  P.reviewedFq12AssemblyCertificateValid
                      b payload cert liftCert.opening assemblyCert ->
                    P.reviewedFq12HasExtractionCompatibilityCertificate
                        b payload cert liftCert.opening h compatCert ->
                      P.reviewedFq12ExtractionCompatibilityCertificateValid
                          b payload cert liftCert.opening h compatCert ->
                        P.reviewedFq12HasFullOpeningBridgeDigestCertificate
                            b payload cert liftCert assemblyCert compatCert
                            bridgeCert ->
                          P.reviewedFq12FullOpeningBridgeDigestCertificateValid
                              b payload cert liftCert assemblyCert compatCert
                              bridgeCert ->
                            ReviewedFq12FullOpeningBridgeDigestCertificateBinds
                              payload cert liftCert assemblyCert compatCert
                              bridgeCert

/--
Availability obligation for the aggregate bridge object emitted by the Rust
full-opening verifier chain.
-/
def ReviewedFq12FullOpeningBridgeDigestCertificateAvailability
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {liftCert : ReviewedFq12FullNativeOpeningCertificate}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12PayloadHasRoleLiftCertificate
                b payload cert liftCert ->
              P.reviewedFq12RoleLiftCertificateValid
                  b payload cert liftCert ->
                P.reviewedFq12OpeningHasAssemblyCertificate
                    b payload cert liftCert.opening assemblyCert ->
                  P.reviewedFq12AssemblyCertificateValid
                      b payload cert liftCert.opening assemblyCert ->
                    P.reviewedFq12HasExtractionCompatibilityCertificate
                        b payload cert liftCert.opening h compatCert ->
                      P.reviewedFq12ExtractionCompatibilityCertificateValid
                          b payload cert liftCert.opening h compatCert ->
                        (exists bridgeCert,
                          P.reviewedFq12HasFullOpeningBridgeDigestCertificate
                              b payload cert liftCert assemblyCert compatCert
                              bridgeCert ∧
                            P.reviewedFq12FullOpeningBridgeDigestCertificateValid
                                b payload cert liftCert assemblyCert compatCert
                                bridgeCert) ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
Availability of a compatibility certificate once a full native opening has
assembled a representative in the native fiber.
-/
def ReviewedFq12ExtractionCompatibilityCertificateAvailability
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening h ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ->
                  InFiber P b.context b h ->
                    P.localValid b.context h ->
                      P.descentConsistent b.context h ->
                        (exists compatCert,
                          P.reviewedFq12HasExtractionCompatibilityCertificate
                              b payload cert opening h compatCert ∧
                            P.reviewedFq12ExtractionCompatibilityCertificateValid
                              b payload cert opening h compatCert) ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
Soundness of the compatibility certificate: a valid, bound compatibility
certificate makes both reviewed-Fq12 extraction predicates name the assembled
representative, or exports a named break.
-/
def ReviewedFq12ExtractionCompatibilityCertificateSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening h ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ->
                  InFiber P b.context b h ->
                    P.localValid b.context h ->
                      P.descentConsistent b.context h ->
                        P.reviewedFq12HasExtractionCompatibilityCertificate
                            b payload cert opening h compatCert ->
                          P.reviewedFq12ExtractionCompatibilityCertificateValid
                              b payload cert opening h compatCert ->
                            ReviewedFq12ExtractionCompatibilityCertificateBinds
                                payload cert opening h compatCert ->
                              (P.reviewedFq12ExtractionCompatibilityCertificateExtracts
                                  b payload cert opening h compatCert ∧
                                P.reviewedFq12CertificateExtracts
                                  b payload cert h ∧
                                P.reviewedFq12PayloadExtracts b payload h) ∨
                              (exists breakEvent,
                                P.boundaryBreakFor b.context b breakEvent)

/--
Compatibility-certificate availability and soundness instantiate the raw
same-representative extraction compatibility surface.
-/
theorem reviewed_fq12_extraction_compatibility_from_certificate
    (P : Predicates)
    (hAvail :
      ReviewedFq12ExtractionCompatibilityCertificateAvailability P)
    (hSound :
      ReviewedFq12ExtractionCompatibilityCertificateSoundness P)
    (hBind :
      forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
          {cert : ReviewedFq12HiddenRelationExtractionCertificate}
          {opening : FullNativeRepresentativeOpening}
          {h : HiddenRepresentative}
          {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
        P.reviewedFq12PayloadAccepts b payload ->
          P.reviewedFq12PayloadProjectsFrom b payload ->
            P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
              P.reviewedFq12ExtractionCertificateValid b payload cert ->
                P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
                  FullNativeOpeningMatchesRepresentative opening h ->
                    P.fullNativeOpeningAssemblesRepresentative
                        b payload cert opening h ->
                      InFiber P b.context b h ->
                        P.localValid b.context h ->
                          P.descentConsistent b.context h ->
                            P.reviewedFq12HasExtractionCompatibilityCertificate
                                b payload cert opening h compatCert ->
                              P.reviewedFq12ExtractionCompatibilityCertificateValid
                                  b payload cert opening h compatCert ->
                                ReviewedFq12ExtractionCompatibilityCertificateBinds
                                  payload cert opening h compatCert) :
    ReviewedFq12FullNativeOpeningExtractionCompatibility P := by
  intro b payload cert opening h hAccept hProjects hHas hValid hLifts
      hMatches hAssembles hFiber hLocal hDescent
  rcases
      hAvail hAccept hProjects hHas hValid hLifts hMatches hAssembles
        hFiber hLocal hDescent with
    hCompatCert | hBreak
  · rcases hCompatCert with ⟨compatCert, hHasCompat, hCompatValid⟩
    have hBinds :=
      hBind hAccept hProjects hHas hValid hLifts hMatches hAssembles
        hFiber hLocal hDescent hHasCompat hCompatValid
    rcases
        hSound hAccept hProjects hHas hValid hLifts hMatches hAssembles
          hFiber hLocal hDescent hHasCompat hCompatValid hBinds with
      hExtracts | hBreak
    · rcases hExtracts with
        ⟨_hCompatExtracts, hCertExtracts, hPayloadExtracts⟩
      exact Or.inl ⟨hCertExtracts, hPayloadExtracts⟩
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
The field-level assembly obligation plus extraction compatibility imply the
older full-native-opening soundness surface.
-/
theorem reviewed_fq12_full_native_opening_soundness_from_assembly
    (P : Predicates)
    (hAssembly : ReviewedFq12FullNativeOpeningAssemblySoundness P)
    (hCompat : ReviewedFq12FullNativeOpeningExtractionCompatibility P) :
    ReviewedFq12FullNativeOpeningSoundness P := by
  intro b payload cert opening hAccept hProjects hHas hValid hLifts
  rcases hAssembly hAccept hProjects hHas hValid hLifts with
    hAssembled | hBreak
  · rcases hAssembled with
      ⟨h, hMatches, hAssembles, hFiber, hLocal, hDescent⟩
    rcases
        hCompat hAccept hProjects hHas hValid hLifts hMatches
          hAssembles hFiber hLocal hDescent with
      hExtracts | hBreak
    · rcases hExtracts with ⟨hCertExtracts, hPayloadExtracts⟩
      exact Or.inl
        ⟨h, hAssembles, hCertExtracts, hPayloadExtracts,
          hFiber, hLocal, hDescent⟩
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
Package for closing the reviewed-Fq12 extraction certificate through the active
Option 2 full-opening route.

This is the current concrete shape of P2FS-001 after the role-root decision:
role-root soundness gives a full native opening, assembly construction builds a
representative from that opening, native-validity soundness places that
representative in the boundary fiber, and extraction compatibility shows the
reviewed-Fq12 certificate/payload relation names that same element.
-/
structure ReviewedFq12FullOpeningBridgeCertificate
    (P : Predicates) where
  roleRootBridge :
    ReviewedFq12RoleRootBridgeCertificate P
  assemblyAvailability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  assemblyPayloadBinding :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
        P.reviewedFq12PayloadProjectsFrom b payload ->
          P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
            P.reviewedFq12ExtractionCertificateValid b payload cert ->
              P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
                P.reviewedFq12OpeningHasAssemblyCertificate
                    b payload cert opening assemblyCert ->
                  P.reviewedFq12AssemblyCertificateValid
                      b payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert
  assemblyConstructionSoundness :
    ReviewedFq12RepresentativeAssemblyConstructionSoundness P
  assembledRepresentativeNativeValiditySoundness :
    ReviewedFq12AssembledRepresentativeNativeValiditySoundness P
  compatibilityAvailability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  compatibilityPayloadBinding :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {h : HiddenRepresentative}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
        P.reviewedFq12PayloadProjectsFrom b payload ->
          P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
            P.reviewedFq12ExtractionCertificateValid b payload cert ->
              P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
                FullNativeOpeningMatchesRepresentative opening h ->
                  P.fullNativeOpeningAssemblesRepresentative
                      b payload cert opening h ->
                    InFiber P b.context b h ->
                      P.localValid b.context h ->
                        P.descentConsistent b.context h ->
                          P.reviewedFq12HasExtractionCompatibilityCertificate
                              b payload cert opening h compatCert ->
                            P.reviewedFq12ExtractionCompatibilityCertificateValid
                                b payload cert opening h compatCert ->
                              ReviewedFq12ExtractionCompatibilityCertificateBinds
                                payload cert opening h compatCert
  compatibilitySoundness :
    ReviewedFq12ExtractionCompatibilityCertificateSoundness P

/--
Certificate-to-representative soundness follows from the two smaller facts:
the selected certificate lifts to a full native representative opening, and
that opening assembles into a valid native representative.
-/
theorem reviewed_fq12_certificate_soundness_from_role_lift
    (P : Predicates)
    (hLift : ReviewedFq12SelectedCertificateRoleLiftSoundness P)
    (hOpeningSound : ReviewedFq12FullNativeOpeningSoundness P) :
    ReviewedFq12ExtractionCertificateSoundness P := by
  intro b payload cert hAccept hProjects hHas hValid
  rcases hLift hAccept hProjects hHas hValid with hOpening | hBreak
  · rcases hOpening with ⟨opening, hLifts⟩
    rcases
        hOpeningSound hAccept hProjects hHas hValid hLifts with
      hExtract | hBreak
    · rcases hExtract with
        ⟨h, _hAssembles, hCertExtracts, hPayloadExtracts,
          hFiber, hLocal, hDescent⟩
      exact Or.inl
        ⟨h, hCertExtracts, hPayloadExtracts, hFiber, hLocal, hDescent⟩
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
The active full-opening bridge discharges extraction-certificate soundness.
-/
theorem reviewed_fq12_certificate_soundness_from_full_opening_bridge
    (P : Predicates)
    (C : ReviewedFq12FullOpeningBridgeCertificate P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_role_lift P
    (reviewed_fq12_selected_role_lift_from_role_root_bridge
      P C.roleRootBridge)
    (reviewed_fq12_full_native_opening_soundness_from_assembly
      P
      (reviewed_fq12_full_native_opening_assembly_from_certificate
        P C.assemblyAvailability
        (reviewed_fq12_representative_assembly_soundness_from_parts
          P C.assemblyConstructionSoundness
          C.assembledRepresentativeNativeValiditySoundness)
        C.assemblyPayloadBinding)
      (reviewed_fq12_extraction_compatibility_from_certificate
        P C.compatibilityAvailability C.compatibilitySoundness
        C.compatibilityPayloadBinding))

/--
The Rust payload-binding layer must make the two statement roles explicit.
This prevents an accepted reviewed-Fq12 active-lane statement from being
silently treated as the Path 2 creation statement.
-/
def ReviewedFq12ExtractionCertificateBindsPayload
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate) : Prop :=
  cert.nativeBoundaryDigest = payload.nativeBoundaryDigest ∧
    cert.slotId = payload.slotId ∧
      cert.creationStatementDigest = payload.creationStatementDigest ∧
        cert.reviewedStatementDigest = payload.statementDigest ∧
          cert.openingCommitment = payload.openingCommitment ∧
            cert.padSeedDigest = payload.padSeedDigest

/-- Binding the certificate to the payload recovers the Path 2 creation digest. -/
theorem reviewed_fq12_extraction_certificate_creation_statement_eq
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    (hBind : ReviewedFq12ExtractionCertificateBindsPayload payload cert) :
    cert.creationStatementDigest = payload.creationStatementDigest :=
  hBind.2.2.1

/--
Binding the certificate to the payload recovers the reviewed active-lane
statement digest separately from the Path 2 creation statement digest.
-/
theorem reviewed_fq12_extraction_certificate_reviewed_statement_eq
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    (hBind : ReviewedFq12ExtractionCertificateBindsPayload payload cert) :
    cert.reviewedStatementDigest = payload.statementDigest :=
  hBind.2.2.2.1

/--
Payload-binding soundness for extraction certificates. This is the theorem-side
mirror of the Rust check that a certificate carries both statement roles,
native boundary digest, slot, opening commitment, and pad-seed digest from the
same accepted payload.
-/
def ReviewedFq12ExtractionCertificatePayloadBindingSoundness
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            ReviewedFq12ExtractionCertificateBindsPayload payload cert ∨
              (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Once certificate availability and certificate soundness are supplied, they
instantiate the original reviewed-Fq12 payload soundness obligation.
-/
theorem reviewed_fq12_payload_soundness_from_extraction_certificate
    (P : Predicates)
    (hAvail : ReviewedFq12ExtractionCertificateAvailability P)
    (hCertSound : ReviewedFq12ExtractionCertificateSoundness P) :
    ReviewedFq12HiddenSourcePayloadSoundness P := by
  intro b payload hAccept hProjects
  rcases hAvail hAccept hProjects with hCert | hBreak
  · rcases hCert with ⟨cert, hHas, hValid⟩
    rcases hCertSound hAccept hProjects hHas hValid with hExtract | hBreak
    · rcases hExtract with
        ⟨h, _hCertExtracts, hPayloadExtracts, hFiber, hLocal, hDescent⟩
      exact Or.inl ⟨h, hPayloadExtracts, hFiber, hLocal, hDescent⟩
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
Smaller certificate package for the active reviewed-Fq12 extraction bridge.
This is the current P2FS-001 shape: payload projection, executable certificate
availability, certificate/payload binding, and certificate-to-representative
soundness.
-/
structure ReviewedFq12ExtractionBridgeCertificate
    (P : Predicates) where
  payloadProjectsFrom :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload},
      P.reviewedFq12PayloadAccepts b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload
  certificateAvailability :
    ReviewedFq12ExtractionCertificateAvailability P
  certificatePayloadBinding :
    ReviewedFq12ExtractionCertificatePayloadBindingSoundness P
  certificateSoundness :
    ReviewedFq12ExtractionCertificateSoundness P

/--
Certificate package required to close the reviewed-Fq12 hidden-source semantics
gate for the active deployment path.

The projection law prevents a payload from being accepted against one native
boundary while carrying another boundary digest or slot. The soundness law is
the extractor/break statement that turns accepted payloads into native
representatives.
-/
structure ReviewedFq12HiddenSourceSemanticsCertificate
    (P : Predicates) where
  payloadProjectsFrom :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload},
      P.reviewedFq12PayloadAccepts b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload
  payloadSoundness : ReviewedFq12HiddenSourcePayloadSoundness P

/--
The extraction bridge certificate derives the original reviewed-Fq12
hidden-source semantics certificate.
-/
theorem reviewed_fq12_semantics_certificate_from_extraction_bridge
    (P : Predicates)
    (C : ReviewedFq12ExtractionBridgeCertificate P) :
    ReviewedFq12HiddenSourceSemanticsCertificate P where
  payloadProjectsFrom := C.payloadProjectsFrom
  payloadSoundness :=
    reviewed_fq12_payload_soundness_from_extraction_certificate
      P C.certificateAvailability C.certificateSoundness

/--
Accepted reviewed-Fq12 hidden-source payloads force a native boundary
representative, or expose a named break.
-/
theorem reviewed_fq12_hidden_source_payload_extracts_fiber_or_break
    (P : Predicates)
    (hSound : ReviewedFq12HiddenSourcePayloadSoundness P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload) :
    (exists h,
      P.reviewedFq12PayloadExtracts b payload h ∧
        InFiber P b.context b h ∧
          P.localValid b.context h ∧
            P.descentConsistent b.context h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  hSound hAccept hProjects

/--
Reviewed-Fq12 hidden-source semantics closure from the certificate package.

This is the concrete theorem shape needed by P2FS-001: an accepting deployment
payload with the native boundary digest and slot bound either yields a valid
native hidden representative or a named break.
-/
theorem reviewed_fq12_hidden_source_semantics_closed_or_break
    (P : Predicates)
    (C : ReviewedFq12HiddenSourceSemanticsCertificate P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hDigest : payload.nativeBoundaryDigest = b.nativeBoundaryDigest)
    (hSlot : payload.slotId = b.context.slotId) :
    (exists h,
      P.reviewedFq12PayloadExtracts b payload h ∧
        InFiber P b.context b h ∧
          P.localValid b.context h ∧
            P.descentConsistent b.context h) ∨
    (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) :=
  C.payloadSoundness hAccept
    (C.payloadProjectsFrom hAccept ⟨hDigest, hSlot⟩)

/--
Spend handoff assumption: spending an admitted boundary consumes the
authorization projected from that boundary and is witnessed by some member of
the same fiber, or exports a named break.
-/
def SpendHandoffSoundness (P : Predicates) : Prop :=
  forall {c : Context} {b : Boundary},
    P.admitted c b ->
      P.spendAccepts c (P.slotAuthorizationOf b) ->
      (exists h,
        InFiber P c b h ∧
          P.spendWitnessValid c h (P.slotAuthorizationOf b)) ∨
      (exists breakEvent, P.boundaryBreakFor c b breakEvent)

theorem spend_handoff_or_break
    (P : Predicates)
    (hSound : SpendHandoffSoundness P)
    {c : Context} {b : Boundary}
    (hAdmitted : P.admitted c b)
    (hSpend : P.spendAccepts c (P.slotAuthorizationOf b)) :
    (exists h,
      InFiber P c b h ∧
        P.spendWitnessValid c h (P.slotAuthorizationOf b)) ∨
    (exists breakEvent, P.boundaryBreakFor c b breakEvent) :=
  hSound hAdmitted hSpend

end Path2NativeBoundary
end Descender
