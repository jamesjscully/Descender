import Descender.Path2NativeAccountDesign

namespace Descender
namespace Path2CheckedCoreByteBridge

open Path2NativeAccountDesign

/-!
# Path 2 Checked-Core Byte Bridge

This file is intentionally standalone.  It narrows the remaining
byte/parser-to-`Path2NativeCheckedCoreAcceptV1` gap without editing the active
account-design interface.
-/

/-- Parsed byte-level checked-core proof fields used by the native acceptor. -/
structure CheckedCoreParsedProof where
  decodedProofCommitment : Nat
  actualResponseSyndrome : Nat

/--
Concrete parser hooks for the checked-core proof carrier.

The predicates stay abstract because the production parser/hash functions live
outside this theorem interface.  The records below force any instantiation to
state exactly how those byte hooks agree with the native V1 acceptor fields.
-/
structure CheckedCoreProofByteParser where
  parse : Nat -> CheckedCoreParsedProof
  proofByteLength : Nat -> Nat
  proofBytesCanonical : Nat -> Prop

/-- The concrete byte parser uses the same length/canonicality predicates as V1. -/
structure ParserMatchesNativeHandle
    (P : CheckedCoreProofByteParser)
    (H : NativeCheckedCoreVerifierHandleFields) where
  proof_byte_length_eq :
    forall proofBytes, P.proofByteLength proofBytes = H.proofByteLength proofBytes
  proof_bytes_canonical_sound :
    forall proofBytes, P.proofBytesCanonical proofBytes ->
      H.proofBytesCanonical proofBytes

/-- The carried proof bytes are present, have the active length, and are canonical. -/
structure CanonicalProofBytes
    (P : CheckedCoreProofByteParser)
    (H : NativeCheckedCoreVerifierHandleFields)
    (proofBytes : Nat) where
  parserMatchesNative : ParserMatchesNativeHandle P H
  length_eq : P.proofByteLength proofBytes = H.requiredProofByteLength
  canonical : P.proofBytesCanonical proofBytes

/-- Canonical proof bytes satisfy the native V1 byte-length check. -/
theorem canonicalProofBytes_native_length
    {P : CheckedCoreProofByteParser}
    {H : NativeCheckedCoreVerifierHandleFields}
    {proofBytes : Nat}
    (C : CanonicalProofBytes P H proofBytes) :
    H.proofByteLength proofBytes = H.requiredProofByteLength := by
  rw [← C.parserMatchesNative.proof_byte_length_eq proofBytes]
  exact C.length_eq

/-- Canonical proof bytes satisfy the native V1 canonicality predicate. -/
theorem canonicalProofBytes_native_canonical
    {P : CheckedCoreProofByteParser}
    {H : NativeCheckedCoreVerifierHandleFields}
    {proofBytes : Nat}
    (C : CanonicalProofBytes P H proofBytes) :
    H.proofBytesCanonical proofBytes :=
  C.parserMatchesNative.proof_bytes_canonical_sound proofBytes C.canonical

/-- Decoding the proof bytes yields the commitment consumed by V1. -/
structure DecodedProofCommitment
    (P : CheckedCoreProofByteParser)
    (proofBytes proofCommitment : Nat) where
  decoded_eq :
    (P.parse proofBytes).decodedProofCommitment = proofCommitment

/-- The byte-level response parser yields the response syndrome carried by V1. -/
structure ParsedResponseSyndrome
    (P : CheckedCoreProofByteParser)
    (proofBytes actualResponseSyndrome : Nat) where
  actual_syndrome_eq :
    (P.parse proofBytes).actualResponseSyndrome = actualResponseSyndrome

/-- The byte-assembled frame is the native checked-core frame for these inputs. -/
structure FrameBytes
    {F : NativeCheckedCoreFrameFields}
    (B : NativeCheckedCoreFrameBinding F)
    (q : Boundary) (msg state proofCommitment frameBytes : Nat) where
  frame_bytes_eq :
    frameBytes = B.frame q msg state proofCommitment

/-- Parser equality tying concrete byte outputs to the evidence record fields. -/
structure ParserEquality
    (P : CheckedCoreProofByteParser)
    (proofBytes : Nat)
    (evidence : NativeCheckedCoreSpendEvidence) where
  proof_bytes_eq :
    evidence.checkedCoreProofBytes = proofBytes
  decoded_commitment_eq :
    evidence.decodedProofCommitment =
      (P.parse proofBytes).decodedProofCommitment
  actual_syndrome_eq :
    evidence.actualResponseSyndrome =
      (P.parse proofBytes).actualResponseSyndrome

/-- Hash/challenge and expected-syndrome equality for the native V1 frame. -/
structure HashAndSyndromeEquality
    (H : NativeCheckedCoreVerifierHandleFields)
    (evidence : NativeCheckedCoreSpendEvidence) where
  challenge_eq :
    evidence.checkedCoreChallenge =
      H.challengeFromFrame evidence.checkedCoreFrame
  expected_syndrome_eq :
    evidence.expectedResponseSyndrome =
      H.expectedResponseSyndrome
        evidence.checkedCoreChallenge
        evidence.checkedCoreVerifierHandleDigest
        evidence.proofCommitment
  actual_syndrome_matches :
    evidence.actualResponseSyndrome = evidence.expectedResponseSyndrome

/--
Small byte-level certificate sufficient to instantiate
`Path2NativeCheckedCoreAcceptV1`.
-/
structure CheckedCoreByteAcceptCertificate
    {F : NativeCheckedCoreFrameFields}
    (B : NativeCheckedCoreFrameBinding F)
    (H : NativeCheckedCoreVerifierHandleFields)
    (P : CheckedCoreProofByteParser)
    (q : Boundary) (msg state : Nat)
    (evidence : NativeCheckedCoreSpendEvidence) where
  canonicalProofBytes :
    CanonicalProofBytes P H evidence.checkedCoreProofBytes
  decodedProofCommitment :
    DecodedProofCommitment P
      evidence.checkedCoreProofBytes evidence.proofCommitment
  parsedResponseSyndrome :
    ParsedResponseSyndrome P
      evidence.checkedCoreProofBytes evidence.actualResponseSyndrome
  frameBytes :
    FrameBytes B q msg state evidence.proofCommitment
      evidence.checkedCoreFrame
  verifier_handle_eq :
    evidence.checkedCoreVerifierHandleDigest =
      H.checkedCoreVerifierHandleDigest q
  parserEquality :
    ParserEquality P evidence.checkedCoreProofBytes evidence
  hashAndSyndrome :
    HashAndSyndromeEquality H evidence

/-- The decoded commitment field of a byte certificate matches the V1 commitment. -/
theorem checkedCoreByteCertificate_decoded_commitment_eq
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (C : CheckedCoreByteAcceptCertificate B H P q msg state evidence) :
    evidence.decodedProofCommitment = evidence.proofCommitment := by
  rw [C.parserEquality.decoded_commitment_eq]
  exact C.decodedProofCommitment.decoded_eq

/-- The parsed response syndrome field agrees with the carried V1 syndrome. -/
theorem checkedCoreByteCertificate_parsed_response_eq
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (C : CheckedCoreByteAcceptCertificate B H P q msg state evidence) :
    evidence.actualResponseSyndrome =
      (P.parse evidence.checkedCoreProofBytes).actualResponseSyndrome :=
  C.parserEquality.actual_syndrome_eq

/--
The standalone byte certificate feeds the existing native checked-core accept
relation.
-/
theorem checkedCoreByteCertificate_accepts_native_v1
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (C : CheckedCoreByteAcceptCertificate B H P q msg state evidence) :
    Path2NativeCheckedCoreAcceptV1 B H q msg state evidence := by
  refine ⟨?_, ?_⟩
  · exact C.frameBytes.frame_bytes_eq
  · refine ⟨C.verifier_handle_eq, ?_⟩
    refine ⟨canonicalProofBytes_native_length C.canonicalProofBytes, ?_⟩
    refine ⟨canonicalProofBytes_native_canonical C.canonicalProofBytes, ?_⟩
    refine ⟨checkedCoreByteCertificate_decoded_commitment_eq C, ?_⟩
    refine ⟨C.hashAndSyndrome.challenge_eq, ?_⟩
    refine ⟨C.hashAndSyndrome.expected_syndrome_eq, ?_⟩
    exact C.hashAndSyndrome.actual_syndrome_matches

/--
Named byte-bridge residuals when the concrete parser/hash instantiation is not
available as a certificate.
-/
def CheckedCoreByteBridgeBreakAlternatives
    (V : VerifierPredicates) (q : Boundary) : Prop :=
  V.breakFor q BreakEvent.byteDataAvailabilityBreak ∨
    (V.breakFor q BreakEvent.byteParserCanonicalityBreak ∨
      (V.breakFor q BreakEvent.checkedCoreFrameBindingBreak ∨
        V.breakFor q BreakEvent.domainBindingOrQromBreak))

/--
Residual breaks after composing the selected Rust byte source with the native
checked-core soundness boundary.
-/
def CheckedCoreByteAndEquationBreakAlternatives
    (V : VerifierPredicates) (q : Boundary) : Prop :=
  V.breakFor q BreakEvent.byteDataAvailabilityBreak ∨
    (V.breakFor q BreakEvent.byteParserCanonicalityBreak ∨
      (V.breakFor q BreakEvent.checkedCoreFrameBindingBreak ∨
        (V.breakFor q BreakEvent.domainBindingOrQromBreak ∨
          V.breakFor q BreakEvent.checkedCoreEquationSoundnessBreak)))

theorem checkedCoreByteBreakAlternatives_to_byteAndEquation
    {V : VerifierPredicates} {q : Boundary}
    (h : CheckedCoreByteBridgeBreakAlternatives V q) :
    CheckedCoreByteAndEquationBreakAlternatives V q := by
  rcases h with hData | hParser | hFrame | hDomain
  · exact Or.inl hData
  · exact Or.inr (Or.inl hParser)
  · exact Or.inr (Or.inr (Or.inl hFrame))
  · exact Or.inr (Or.inr (Or.inr (Or.inl hDomain)))

theorem checkedCoreEquationBreak_to_byteAndEquation
    {V : VerifierPredicates} {q : Boundary}
    (h : V.breakFor q BreakEvent.checkedCoreEquationSoundnessBreak) :
    CheckedCoreByteAndEquationBreakAlternatives V q := by
  right
  right
  right
  right
  exact h

theorem checkedCoreFrameBreak_to_byteAndEquation
    {V : VerifierPredicates} {q : Boundary}
    (h : V.breakFor q BreakEvent.checkedCoreFrameBindingBreak) :
    CheckedCoreByteAndEquationBreakAlternatives V q := by
  right
  right
  left
  exact h

theorem checkedCoreDomainBreak_to_byteAndEquation
    {V : VerifierPredicates} {q : Boundary}
    (h : V.breakFor q BreakEvent.domainBindingOrQromBreak) :
    CheckedCoreByteAndEquationBreakAlternatives V q := by
  right
  right
  right
  left
  exact h

theorem checkedCoreParserBreak_to_byteAndEquation
    {V : VerifierPredicates} {q : Boundary}
    (h : V.breakFor q BreakEvent.byteParserCanonicalityBreak) :
    CheckedCoreByteAndEquationBreakAlternatives V q := by
  right
  left
  exact h

/--
A concrete byte certificate gives native V1 accept; otherwise the only
remaining exits are explicit parser/canonicality/frame/hash/data breaks.
-/
def CheckedCoreByteBridgeAcceptOrBreak
    {F : NativeCheckedCoreFrameFields}
    (B : NativeCheckedCoreFrameBinding F)
    (H : NativeCheckedCoreVerifierHandleFields)
    (P : CheckedCoreProofByteParser)
    (V : VerifierPredicates)
    (q : Boundary) (msg state : Nat)
    (evidence : NativeCheckedCoreSpendEvidence) : Prop :=
  CheckedCoreByteAcceptCertificate B H P q msg state evidence ∨
    CheckedCoreByteBridgeBreakAlternatives V q

/--
Selected Rust checked-core byte source for the active verifier.

This is the Lean-facing contract for
`path2_native_checked_core_accept_v1`: an instantiation must decompose a
successful Rust accept result into the concrete parser, canonicality, frame,
hash/challenge, and syndrome equalities consumed by
`CheckedCoreByteAcceptCertificate`. Keeping these fields separate prevents a
single opaque "Rust accepted" predicate from satisfying the active relation
without proving the byte-level correspondence.
-/
structure SelectedRustCheckedCoreByteSource
    {F : NativeCheckedCoreFrameFields}
    (B : NativeCheckedCoreFrameBinding F)
    (H : NativeCheckedCoreVerifierHandleFields)
    (P : CheckedCoreProofByteParser)
    (V : VerifierPredicates) where
  rustAcceptorAccepts :
    Boundary -> Nat -> Nat -> NativeCheckedCoreSpendEvidence -> Prop
  parserMatchesNative :
    ParserMatchesNativeHandle P H
  proof_length_eq :
    forall {q msg state evidence},
      rustAcceptorAccepts q msg state evidence ->
        P.proofByteLength evidence.checkedCoreProofBytes =
          H.requiredProofByteLength
  proof_canonical :
    forall {q msg state evidence},
      rustAcceptorAccepts q msg state evidence ->
        P.proofBytesCanonical evidence.checkedCoreProofBytes
  decoded_proof_commitment :
    forall {q msg state evidence},
      rustAcceptorAccepts q msg state evidence ->
        DecodedProofCommitment P
          evidence.checkedCoreProofBytes evidence.proofCommitment
  parsed_response_syndrome :
    forall {q msg state evidence},
      rustAcceptorAccepts q msg state evidence ->
        ParsedResponseSyndrome P
          evidence.checkedCoreProofBytes evidence.actualResponseSyndrome
  frame_bytes :
    forall {q msg state evidence},
      rustAcceptorAccepts q msg state evidence ->
        FrameBytes B q msg state evidence.proofCommitment
          evidence.checkedCoreFrame
  verifier_handle_eq :
    forall {q msg state evidence},
      rustAcceptorAccepts q msg state evidence ->
        evidence.checkedCoreVerifierHandleDigest =
          H.checkedCoreVerifierHandleDigest q
  parser_equality :
    forall {q msg state evidence},
      rustAcceptorAccepts q msg state evidence ->
        ParserEquality P evidence.checkedCoreProofBytes evidence
  hash_and_syndrome :
    forall {q msg state evidence},
      rustAcceptorAccepts q msg state evidence ->
        HashAndSyndromeEquality H evidence

/--
Implementation-oriented component package for the selected Rust checked-core
byte source.

This is intentionally one step below the deployment-closing concrete theorem:
it packages the exact parser/native-handle, canonical proof, frame, verifier
handle, challenge, and syndrome facts that a Rust/model instantiation must
supply, but it does not assert that those facts have already been derived from
the production Rust parser and hash functions.
-/
structure SelectedRustCheckedCoreByteInstantiationInputs
    {F : NativeCheckedCoreFrameFields}
    (B : NativeCheckedCoreFrameBinding F)
    (H : NativeCheckedCoreVerifierHandleFields)
    (P : CheckedCoreProofByteParser)
    (V : VerifierPredicates) where
  selectedRustCheckedCoreAcceptorAccepts :
    Boundary -> Nat -> Nat -> NativeCheckedCoreSpendEvidence -> Prop
  selectedRustCheckedCoreParserMatchesNative :
    ParserMatchesNativeHandle P H
  selectedRustCheckedCoreProofLengthEq :
    forall {q msg state evidence},
      selectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        P.proofByteLength evidence.checkedCoreProofBytes =
          H.requiredProofByteLength
  selectedRustCheckedCoreProofCanonical :
    forall {q msg state evidence},
      selectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        P.proofBytesCanonical evidence.checkedCoreProofBytes
  selectedRustCheckedCoreDecodedProofCommitment :
    forall {q msg state evidence},
      selectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        DecodedProofCommitment P
          evidence.checkedCoreProofBytes evidence.proofCommitment
  selectedRustCheckedCoreParsedResponseSyndrome :
    forall {q msg state evidence},
      selectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        ParsedResponseSyndrome P
          evidence.checkedCoreProofBytes evidence.actualResponseSyndrome
  selectedRustCheckedCoreFrameBytes :
    forall {q msg state evidence},
      selectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        FrameBytes B q msg state evidence.proofCommitment
          evidence.checkedCoreFrame
  selectedRustCheckedCoreVerifierHandleEq :
    forall {q msg state evidence},
      selectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        evidence.checkedCoreVerifierHandleDigest =
          H.checkedCoreVerifierHandleDigest q
  selectedRustCheckedCoreParserEquality :
    forall {q msg state evidence},
      selectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        ParserEquality P evidence.checkedCoreProofBytes evidence
  selectedRustCheckedCoreHashAndSyndrome :
    forall {q msg state evidence},
      selectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        HashAndSyndromeEquality H evidence

/--
Lean mirror of the public checked-core byte-instantiation evidence report.

The Rust preflight exports this as a digest-only report.  The final selected
Rust theorem should prove the report is verified and all component/negative
control facts are true, then use those proofs to build the theorem-side
certificate below.
-/
structure SelectedRustCheckedCoreByteEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  checkedCoreAcceptorAccepts : Bool
  parserMatchesNativeHandle : Bool
  proofLengthEq : Bool
  proofCanonical : Bool
  decodedProofCommitmentEq : Bool
  parsedResponseSyndromeEq : Bool
  frameBytesEq : Bool
  verifierHandleEq : Bool
  parserEquality : Bool
  hashAndSyndromeEq : Bool
  hashByteDomainEvidenceVerified : Bool
  hashByteDomainEvidenceDigestPinned : Bool
  hashByteDomainEvidenceFactsAllTrue : Bool
  checkedCoreFrameDomainMatchesHashEvidence : Bool
  checkedCoreFrameBytesUseHashDomainPrefix : Bool
  checkedCoreChallengeUsesHashDomainFrame : Bool
  shortProofRejected : Bool
  noncanonicalProofRejected : Bool
  commitmentMismatchRejected : Bool
  verifierHandleMismatchRejected : Bool

/-- The checked-core byte evidence report has the expected schema and digest. -/
def SelectedRustCheckedCoreByteEvidenceReport.ReportVerified
    (E : SelectedRustCheckedCoreByteEvidenceReport) : Prop :=
  E.schemaMatches = true ∧
  E.reportDigestVerified = true

/-- All checked-core byte component facts and negative controls are true. -/
def SelectedRustCheckedCoreByteEvidenceReport.FactsAllTrue
    (E : SelectedRustCheckedCoreByteEvidenceReport) : Prop :=
  E.checkedCoreAcceptorAccepts = true ∧
  E.parserMatchesNativeHandle = true ∧
  E.proofLengthEq = true ∧
  E.proofCanonical = true ∧
  E.decodedProofCommitmentEq = true ∧
  E.parsedResponseSyndromeEq = true ∧
  E.frameBytesEq = true ∧
  E.verifierHandleEq = true ∧
  E.parserEquality = true ∧
  E.hashAndSyndromeEq = true ∧
  E.hashByteDomainEvidenceVerified = true ∧
  E.hashByteDomainEvidenceDigestPinned = true ∧
  E.hashByteDomainEvidenceFactsAllTrue = true ∧
  E.checkedCoreFrameDomainMatchesHashEvidence = true ∧
  E.checkedCoreFrameBytesUseHashDomainPrefix = true ∧
  E.checkedCoreChallengeUsesHashDomainFrame = true ∧
  E.shortProofRejected = true ∧
  E.noncanonicalProofRejected = true ∧
  E.commitmentMismatchRejected = true ∧
  E.verifierHandleMismatchRejected = true

/--
Selected Rust checked-core public evidence certificate.

The Rust preflight emits a digest-only checked-core byte-instantiation evidence
report.  This Lean record is the theorem-side bridge from a verified report
with all component facts true into the component package consumed by the
checked-core byte source.  It keeps the final AP2-N002 instantiation from
constructing `SelectedRustCheckedCoreByteInstantiationInputs` without naming
the public evidence report it relies on.
-/
structure SelectedRustCheckedCoreByteEvidenceCertificate
    {F : NativeCheckedCoreFrameFields}
    (B : NativeCheckedCoreFrameBinding F)
    (H : NativeCheckedCoreVerifierHandleFields)
  (P : CheckedCoreProofByteParser)
  (V : VerifierPredicates) where
  evidenceReport : SelectedRustCheckedCoreByteEvidenceReport
  evidenceDigest : ByteArray
  hashByteDomainEvidenceDigest : ByteArray
  evidenceReportVerifiedProof : evidenceReport.ReportVerified
  evidenceFactsAllTrueProof : evidenceReport.FactsAllTrue
  evidenceSelectedRustCheckedCoreAcceptorAccepts :
    Boundary -> Nat -> Nat -> NativeCheckedCoreSpendEvidence -> Prop
  evidenceSelectedRustProductionCheckedCoreAcceptorAccepts :
    Boundary -> Nat -> Nat -> NativeCheckedCoreSpendEvidence -> Prop
  evidenceSelectedRustCheckedCoreAcceptorPinnedProof :
    evidenceSelectedRustCheckedCoreAcceptorAccepts =
      evidenceSelectedRustProductionCheckedCoreAcceptorAccepts
  evidenceSelectedRustCheckedCoreParserMatchesNative :
    ParserMatchesNativeHandle P H
  evidenceSelectedRustCheckedCoreProofLengthEq :
    forall {q msg state evidence},
      evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        P.proofByteLength evidence.checkedCoreProofBytes =
          H.requiredProofByteLength
  evidenceSelectedRustCheckedCoreProofCanonical :
    forall {q msg state evidence},
      evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        P.proofBytesCanonical evidence.checkedCoreProofBytes
  evidenceSelectedRustCheckedCoreDecodedProofCommitment :
    forall {q msg state evidence},
      evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        DecodedProofCommitment P
          evidence.checkedCoreProofBytes evidence.proofCommitment
  evidenceSelectedRustCheckedCoreParsedResponseSyndrome :
    forall {q msg state evidence},
      evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        ParsedResponseSyndrome P
          evidence.checkedCoreProofBytes evidence.actualResponseSyndrome
  evidenceSelectedRustCheckedCoreFrameBytes :
    forall {q msg state evidence},
      evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        FrameBytes B q msg state evidence.proofCommitment
          evidence.checkedCoreFrame
  evidenceSelectedRustCheckedCoreVerifierHandleEq :
    forall {q msg state evidence},
      evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        evidence.checkedCoreVerifierHandleDigest =
          H.checkedCoreVerifierHandleDigest q
  evidenceSelectedRustCheckedCoreParserEquality :
    forall {q msg state evidence},
      evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        ParserEquality P evidence.checkedCoreProofBytes evidence
  evidenceSelectedRustCheckedCoreHashAndSyndrome :
    forall {q msg state evidence},
      evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
        HashAndSyndromeEquality H evidence

/-- Named constructor for the selected Rust checked-core evidence certificate. -/
def SelectedRustCheckedCoreByteEvidenceCertificate.ofReportAndComponents
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {V : VerifierPredicates}
    (evidenceReport : SelectedRustCheckedCoreByteEvidenceReport)
    (evidenceDigest : ByteArray)
    (hashByteDomainEvidenceDigest : ByteArray)
    (evidenceReportVerifiedProof : evidenceReport.ReportVerified)
    (evidenceFactsAllTrueProof : evidenceReport.FactsAllTrue)
    (evidenceSelectedRustCheckedCoreAcceptorAccepts :
      Boundary -> Nat -> Nat -> NativeCheckedCoreSpendEvidence -> Prop)
    (evidenceSelectedRustProductionCheckedCoreAcceptorAccepts :
      Boundary -> Nat -> Nat -> NativeCheckedCoreSpendEvidence -> Prop)
    (evidenceSelectedRustCheckedCoreAcceptorPinnedProof :
      evidenceSelectedRustCheckedCoreAcceptorAccepts =
        evidenceSelectedRustProductionCheckedCoreAcceptorAccepts)
    (evidenceSelectedRustCheckedCoreParserMatchesNative :
      ParserMatchesNativeHandle P H)
    (evidenceSelectedRustCheckedCoreProofLengthEq :
      forall {q msg state evidence},
        evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
          P.proofByteLength evidence.checkedCoreProofBytes =
            H.requiredProofByteLength)
    (evidenceSelectedRustCheckedCoreProofCanonical :
      forall {q msg state evidence},
        evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
          P.proofBytesCanonical evidence.checkedCoreProofBytes)
    (evidenceSelectedRustCheckedCoreDecodedProofCommitment :
      forall {q msg state evidence},
        evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
          DecodedProofCommitment P
            evidence.checkedCoreProofBytes evidence.proofCommitment)
    (evidenceSelectedRustCheckedCoreParsedResponseSyndrome :
      forall {q msg state evidence},
        evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
          ParsedResponseSyndrome P
            evidence.checkedCoreProofBytes evidence.actualResponseSyndrome)
    (evidenceSelectedRustCheckedCoreFrameBytes :
      forall {q msg state evidence},
        evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
          FrameBytes B q msg state evidence.proofCommitment
            evidence.checkedCoreFrame)
    (evidenceSelectedRustCheckedCoreVerifierHandleEq :
      forall {q msg state evidence},
        evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
          evidence.checkedCoreVerifierHandleDigest =
            H.checkedCoreVerifierHandleDigest q)
    (evidenceSelectedRustCheckedCoreParserEquality :
      forall {q msg state evidence},
        evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
          ParserEquality P evidence.checkedCoreProofBytes evidence)
    (evidenceSelectedRustCheckedCoreHashAndSyndrome :
      forall {q msg state evidence},
        evidenceSelectedRustCheckedCoreAcceptorAccepts q msg state evidence ->
          HashAndSyndromeEquality H evidence) :
    SelectedRustCheckedCoreByteEvidenceCertificate B H P V where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  hashByteDomainEvidenceDigest := hashByteDomainEvidenceDigest
  evidenceReportVerifiedProof := evidenceReportVerifiedProof
  evidenceFactsAllTrueProof := evidenceFactsAllTrueProof
  evidenceSelectedRustCheckedCoreAcceptorAccepts :=
    evidenceSelectedRustCheckedCoreAcceptorAccepts
  evidenceSelectedRustProductionCheckedCoreAcceptorAccepts :=
    evidenceSelectedRustProductionCheckedCoreAcceptorAccepts
  evidenceSelectedRustCheckedCoreAcceptorPinnedProof :=
    evidenceSelectedRustCheckedCoreAcceptorPinnedProof
  evidenceSelectedRustCheckedCoreParserMatchesNative :=
    evidenceSelectedRustCheckedCoreParserMatchesNative
  evidenceSelectedRustCheckedCoreProofLengthEq :=
    evidenceSelectedRustCheckedCoreProofLengthEq
  evidenceSelectedRustCheckedCoreProofCanonical :=
    evidenceSelectedRustCheckedCoreProofCanonical
  evidenceSelectedRustCheckedCoreDecodedProofCommitment :=
    evidenceSelectedRustCheckedCoreDecodedProofCommitment
  evidenceSelectedRustCheckedCoreParsedResponseSyndrome :=
    evidenceSelectedRustCheckedCoreParsedResponseSyndrome
  evidenceSelectedRustCheckedCoreFrameBytes :=
    evidenceSelectedRustCheckedCoreFrameBytes
  evidenceSelectedRustCheckedCoreVerifierHandleEq :=
    evidenceSelectedRustCheckedCoreVerifierHandleEq
  evidenceSelectedRustCheckedCoreParserEquality :=
    evidenceSelectedRustCheckedCoreParserEquality
  evidenceSelectedRustCheckedCoreHashAndSyndrome :=
    evidenceSelectedRustCheckedCoreHashAndSyndrome

/-- Convert the verified public evidence certificate into component inputs. -/
def SelectedRustCheckedCoreByteEvidenceCertificate.toInstantiationInputs
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {V : VerifierPredicates}
    (Cert : SelectedRustCheckedCoreByteEvidenceCertificate B H P V) :
    SelectedRustCheckedCoreByteInstantiationInputs B H P V where
  selectedRustCheckedCoreAcceptorAccepts := by
    let _hReport : Cert.evidenceReport.ReportVerified :=
      Cert.evidenceReportVerifiedProof
    let _hFacts : Cert.evidenceReport.FactsAllTrue :=
      Cert.evidenceFactsAllTrueProof
    let _hPinned :
        Cert.evidenceSelectedRustCheckedCoreAcceptorAccepts =
          Cert.evidenceSelectedRustProductionCheckedCoreAcceptorAccepts :=
      Cert.evidenceSelectedRustCheckedCoreAcceptorPinnedProof
    exact Cert.evidenceSelectedRustCheckedCoreAcceptorAccepts
  selectedRustCheckedCoreParserMatchesNative :=
    Cert.evidenceSelectedRustCheckedCoreParserMatchesNative
  selectedRustCheckedCoreProofLengthEq :=
    Cert.evidenceSelectedRustCheckedCoreProofLengthEq
  selectedRustCheckedCoreProofCanonical :=
    Cert.evidenceSelectedRustCheckedCoreProofCanonical
  selectedRustCheckedCoreDecodedProofCommitment :=
    Cert.evidenceSelectedRustCheckedCoreDecodedProofCommitment
  selectedRustCheckedCoreParsedResponseSyndrome :=
    Cert.evidenceSelectedRustCheckedCoreParsedResponseSyndrome
  selectedRustCheckedCoreFrameBytes :=
    Cert.evidenceSelectedRustCheckedCoreFrameBytes
  selectedRustCheckedCoreVerifierHandleEq :=
    Cert.evidenceSelectedRustCheckedCoreVerifierHandleEq
  selectedRustCheckedCoreParserEquality :=
    Cert.evidenceSelectedRustCheckedCoreParserEquality
  selectedRustCheckedCoreHashAndSyndrome :=
    Cert.evidenceSelectedRustCheckedCoreHashAndSyndrome

/--
Build the selected Rust byte source from the implementation-oriented component
package.
-/
def selectedRustCheckedCoreByteSource_from_instantiation_inputs_v1
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {V : VerifierPredicates}
    (Inputs : SelectedRustCheckedCoreByteInstantiationInputs B H P V) :
    SelectedRustCheckedCoreByteSource B H P V where
  rustAcceptorAccepts :=
    Inputs.selectedRustCheckedCoreAcceptorAccepts
  parserMatchesNative :=
    Inputs.selectedRustCheckedCoreParserMatchesNative
  proof_length_eq :=
    Inputs.selectedRustCheckedCoreProofLengthEq
  proof_canonical :=
    Inputs.selectedRustCheckedCoreProofCanonical
  decoded_proof_commitment :=
    Inputs.selectedRustCheckedCoreDecodedProofCommitment
  parsed_response_syndrome :=
    Inputs.selectedRustCheckedCoreParsedResponseSyndrome
  frame_bytes :=
    Inputs.selectedRustCheckedCoreFrameBytes
  verifier_handle_eq :=
    Inputs.selectedRustCheckedCoreVerifierHandleEq
  parser_equality :=
    Inputs.selectedRustCheckedCoreParserEquality
  hash_and_syndrome :=
    Inputs.selectedRustCheckedCoreHashAndSyndrome

/--
A selected Rust accept result is enough to build the standalone byte
certificate, once the concrete Rust source contract has been instantiated.
-/
theorem selectedRustCheckedCoreByteSource_certificate
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {V : VerifierPredicates}
    (Source : SelectedRustCheckedCoreByteSource B H P V)
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (hAccept : Source.rustAcceptorAccepts q msg state evidence) :
    CheckedCoreByteAcceptCertificate B H P q msg state evidence where
  canonicalProofBytes :=
    { parserMatchesNative := Source.parserMatchesNative
      length_eq := Source.proof_length_eq hAccept
      canonical := Source.proof_canonical hAccept }
  decodedProofCommitment :=
    Source.decoded_proof_commitment hAccept
  parsedResponseSyndrome :=
    Source.parsed_response_syndrome hAccept
  frameBytes :=
    Source.frame_bytes hAccept
  verifier_handle_eq :=
    Source.verifier_handle_eq hAccept
  parserEquality :=
    Source.parser_equality hAccept
  hashAndSyndrome :=
    Source.hash_and_syndrome hAccept

/--
Selected Rust source accept, or the explicit residual break that prevents using
the selected byte source.
-/
def SelectedRustCheckedCoreByteSourceAcceptOrBreak
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {V : VerifierPredicates}
    (Source : SelectedRustCheckedCoreByteSource B H P V)
    (q : Boundary) (msg state : Nat)
    (evidence : NativeCheckedCoreSpendEvidence) : Prop :=
  Source.rustAcceptorAccepts q msg state evidence ∨
    CheckedCoreByteBridgeBreakAlternatives V q

/--
The selected Rust source feeds the native V1 checked-core relation, or exports
one of the named byte/parser/frame/domain residuals.
-/
theorem selectedRustCheckedCoreByteSource_accept_or_named_break
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {V : VerifierPredicates}
    (Source : SelectedRustCheckedCoreByteSource B H P V)
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (h :
      SelectedRustCheckedCoreByteSourceAcceptOrBreak
        Source q msg state evidence) :
    Path2NativeCheckedCoreAcceptV1 B H q msg state evidence ∨
      CheckedCoreByteBridgeBreakAlternatives V q := by
  cases h with
  | inl hAccept =>
      exact Or.inl
        (checkedCoreByteCertificate_accepts_native_v1
          (selectedRustCheckedCoreByteSource_certificate Source hAccept))
  | inr hBreak =>
      exact Or.inr hBreak

/--
Discharge the byte bridge into the existing native V1 accept relation or a
named residual break.
-/
theorem checkedCoreByteBridge_accept_or_named_break
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    {P : CheckedCoreProofByteParser}
    {V : VerifierPredicates}
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (h :
      CheckedCoreByteBridgeAcceptOrBreak B H P V q msg state evidence) :
    Path2NativeCheckedCoreAcceptV1 B H q msg state evidence ∨
      CheckedCoreByteBridgeBreakAlternatives V q := by
  cases h with
  | inl C =>
      exact Or.inl (checkedCoreByteCertificate_accepts_native_v1 C)
  | inr breaks =>
      exact Or.inr breaks

/--
Composing the byte bridge with the existing checked-core bridge feeds the
abstract `CheckedCoreSpendReduction.checkedCoreAccepts` predicate.
-/
theorem checkedCoreByteCertificate_checkedCoreAccepts
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    {Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F}
    {P : CheckedCoreProofByteParser}
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (C :
      CheckedCoreByteAcceptCertificate
        Bridge.nativeFrameBinding Bridge.verifierHandleFields
        P q msg state evidence) :
    Bridge.reduction.checkedCoreAccepts q msg state
      evidence.toCheckedCoreSpendEvidence :=
  path2_native_checked_core_accepts_checkedCoreAccepts Bridge
    (checkedCoreByteCertificate_accepts_native_v1 C)

/--
The selected Rust byte source plus the native V1 bridge feeds the abstract
checked-core predicate directly, or returns one of the named byte/parser/frame
or domain residuals.
-/
theorem selectedRustCheckedCoreByteSource_checkedCoreAccepts_or_named_break
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    {P : CheckedCoreProofByteParser}
    (Source :
      SelectedRustCheckedCoreByteSource
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V)
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (h :
      SelectedRustCheckedCoreByteSourceAcceptOrBreak
        Source q msg state evidence) :
    Bridge.reduction.checkedCoreAccepts q msg state
        evidence.toCheckedCoreSpendEvidence ∨
      CheckedCoreByteBridgeBreakAlternatives V q := by
  cases h with
  | inl hAccept =>
      exact Or.inl
        (path2_native_checked_core_accepts_checkedCoreAccepts Bridge
          (checkedCoreByteCertificate_accepts_native_v1
            (selectedRustCheckedCoreByteSource_certificate Source hAccept)))
  | inr hBreak =>
      exact Or.inr hBreak

/--
The selected Rust byte source, native V1 bridge, and checked-core soundness
boundary give valid checked-core evidence directly, or one of the named
byte/parser/frame/domain/equation residuals.
-/
theorem selectedRustCheckedCoreByteSource_valid_or_named_break
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    (Soundness :
      Path2NativeCheckedCoreAcceptV1Soundness
        V F Bridge.nativeFrameBinding Bridge.verifierHandleFields)
    {P : CheckedCoreProofByteParser}
    (Source :
      SelectedRustCheckedCoreByteSource
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V)
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (h :
      SelectedRustCheckedCoreByteSourceAcceptOrBreak
        Source q msg state evidence) :
    Soundness.validCheckedCoreEvidence q msg state evidence ∨
      CheckedCoreByteAndEquationBreakAlternatives V q := by
  cases h with
  | inl hAccept =>
      have hNative :
          Path2NativeCheckedCoreAcceptV1
            Bridge.nativeFrameBinding Bridge.verifierHandleFields
            q msg state evidence :=
        checkedCoreByteCertificate_accepts_native_v1
          (selectedRustCheckedCoreByteSource_certificate Source hAccept)
      rcases
          path2_native_checked_core_accept_valid_or_named_break
            Soundness hNative with hValid | hEquation | hFrame | hDomain | hParser
      · exact Or.inl hValid
      · exact Or.inr (checkedCoreEquationBreak_to_byteAndEquation hEquation)
      · exact Or.inr (checkedCoreFrameBreak_to_byteAndEquation hFrame)
      · exact Or.inr (checkedCoreDomainBreak_to_byteAndEquation hDomain)
      · exact Or.inr (checkedCoreParserBreak_to_byteAndEquation hParser)
  | inr hBreak =>
      exact Or.inr
        (checkedCoreByteBreakAlternatives_to_byteAndEquation hBreak)

/--
Implementation-oriented checked-core endpoint: once the selected Rust/model
instantiation supplies the named component facts, the existing native V1 bridge
and soundness boundary yield valid checked-core evidence or the same named
byte/parser/frame/domain/equation residual breaks.
-/
theorem selectedRustCheckedCoreByteSource_from_instantiation_inputs_valid_or_named_break
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    (Soundness :
      Path2NativeCheckedCoreAcceptV1Soundness
        V F Bridge.nativeFrameBinding Bridge.verifierHandleFields)
    {P : CheckedCoreProofByteParser}
    (Inputs :
      SelectedRustCheckedCoreByteInstantiationInputs
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V)
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (h :
      SelectedRustCheckedCoreByteSourceAcceptOrBreak
        (selectedRustCheckedCoreByteSource_from_instantiation_inputs_v1 Inputs)
        q msg state evidence) :
    Soundness.validCheckedCoreEvidence q msg state evidence ∨
      CheckedCoreByteAndEquationBreakAlternatives V q :=
  selectedRustCheckedCoreByteSource_valid_or_named_break
    Bridge Soundness
    (selectedRustCheckedCoreByteSource_from_instantiation_inputs_v1 Inputs)
    h

/--
Checked-core closure proposition supplied by the selected Rust/model component
package.

This is the proof obligation installed into the active accepted-spend boundary:
every selected Rust checked-core accept either gives valid checked-core
evidence or names the byte/parser/frame/domain/equation residual that prevents
using the accept as proof evidence.
-/
def SelectedRustCheckedCoreCompositionClosure
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    (Soundness :
      Path2NativeCheckedCoreAcceptV1Soundness
        V F Bridge.nativeFrameBinding Bridge.verifierHandleFields)
    {P : CheckedCoreProofByteParser}
    (Inputs :
      SelectedRustCheckedCoreByteInstantiationInputs
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V) : Prop :=
  forall {q : Boundary} {msg state : Nat}
      {evidence : NativeCheckedCoreSpendEvidence},
    SelectedRustCheckedCoreByteSourceAcceptOrBreak
        (selectedRustCheckedCoreByteSource_from_instantiation_inputs_v1 Inputs)
        q msg state evidence ->
      Soundness.validCheckedCoreEvidence q msg state evidence ∨
        CheckedCoreByteAndEquationBreakAlternatives V q

/--
The selected Rust/model component package closes checked-core composition up to
the named residual breaks.
-/
theorem selectedRustCheckedCoreCompositionClosure_from_instantiation_inputs
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    (Soundness :
      Path2NativeCheckedCoreAcceptV1Soundness
        V F Bridge.nativeFrameBinding Bridge.verifierHandleFields)
    {P : CheckedCoreProofByteParser}
    (Inputs :
      SelectedRustCheckedCoreByteInstantiationInputs
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V) :
    SelectedRustCheckedCoreCompositionClosure Bridge Soundness Inputs := by
  intro q msg state evidence hAcceptOrBreak
  exact
    selectedRustCheckedCoreByteSource_from_instantiation_inputs_valid_or_named_break
      Bridge Soundness Inputs hAcceptOrBreak

/--
Install selected Rust/model checked-core closure into the active accepted-spend
boundary, preserving every other proof obligation.
-/
def NativeAcceptedSpendSoundnessBoundary.withSelectedRustCheckedCoreClosure
    (B : NativeAcceptedSpendSoundnessBoundary)
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    (Soundness :
      Path2NativeCheckedCoreAcceptV1Soundness
        V F Bridge.nativeFrameBinding Bridge.verifierHandleFields)
    {P : CheckedCoreProofByteParser}
    (Inputs :
      SelectedRustCheckedCoreByteInstantiationInputs
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V) :
    NativeAcceptedSpendSoundnessBoundary :=
  { B with
    checkedCoreCompositionClosed :=
      SelectedRustCheckedCoreCompositionClosure Bridge Soundness Inputs }

/-- The accepted-spend wrapper exposes exactly the selected checked-core closure. -/
theorem accepted_spend_with_selected_rust_checked_core_closure_closed
    (B : NativeAcceptedSpendSoundnessBoundary)
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    (Soundness :
      Path2NativeCheckedCoreAcceptV1Soundness
        V F Bridge.nativeFrameBinding Bridge.verifierHandleFields)
    {P : CheckedCoreProofByteParser}
    (Inputs :
      SelectedRustCheckedCoreByteInstantiationInputs
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V) :
    (NativeAcceptedSpendSoundnessBoundary.withSelectedRustCheckedCoreClosure
      B Bridge Soundness Inputs).checkedCoreCompositionClosed :=
  selectedRustCheckedCoreCompositionClosure_from_instantiation_inputs
    Bridge Soundness Inputs

/--
Install selected Rust/model checked-core closure into the active full-signature
boundary, preserving every other full-signature proof obligation.
-/
def NativeFullSignatureSecurityBoundary.withSelectedRustCheckedCoreClosure
    (B : NativeFullSignatureSecurityBoundary)
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    (Soundness :
      Path2NativeCheckedCoreAcceptV1Soundness
        V F Bridge.nativeFrameBinding Bridge.verifierHandleFields)
    {P : CheckedCoreProofByteParser}
    (Inputs :
      SelectedRustCheckedCoreByteInstantiationInputs
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V) :
    NativeFullSignatureSecurityBoundary :=
  { B with
    acceptedSpendSoundness :=
      NativeAcceptedSpendSoundnessBoundary.withSelectedRustCheckedCoreClosure
        B.acceptedSpendSoundness Bridge Soundness Inputs
    fullSignatureSecurityClaimComplete := by
      rcases B.fullSignatureSecurityClaimComplete with
        ⟨hKeygen, hSigning, hCompleteness, hVerifier, hFullGame,
          hHiddenSource, _hCheckedCore, hLifecycle, hNoExport,
          hCheckedCoreDomain, hHiddenSourceDomain, hQueryBudget,
          hMultiTarget⟩
      exact
        ⟨hKeygen, hSigning, hCompleteness, hVerifier, hFullGame,
          hHiddenSource,
          selectedRustCheckedCoreCompositionClosure_from_instantiation_inputs
            Bridge Soundness Inputs,
          hLifecycle, hNoExport, hCheckedCoreDomain, hHiddenSourceDomain,
          hQueryBudget, hMultiTarget⟩ }

/-- The full-signature wrapper exposes exactly the selected checked-core closure. -/
theorem full_signature_with_selected_rust_checked_core_closure_closed
    (B : NativeFullSignatureSecurityBoundary)
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (Bridge : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    (Soundness :
      Path2NativeCheckedCoreAcceptV1Soundness
        V F Bridge.nativeFrameBinding Bridge.verifierHandleFields)
    {P : CheckedCoreProofByteParser}
    (Inputs :
      SelectedRustCheckedCoreByteInstantiationInputs
        Bridge.nativeFrameBinding Bridge.verifierHandleFields P V) :
    (NativeFullSignatureSecurityBoundary.withSelectedRustCheckedCoreClosure
      B Bridge Soundness Inputs).acceptedSpendSoundness.checkedCoreCompositionClosed :=
  selectedRustCheckedCoreCompositionClosure_from_instantiation_inputs
    Bridge Soundness Inputs

end Path2CheckedCoreByteBridge
end Descender
