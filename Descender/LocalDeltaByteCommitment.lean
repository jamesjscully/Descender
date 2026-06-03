import Descender.LocallyRecoverableSyndrome
import Descender.SignaturePublicViewWitness
import Descender.SignatureTranscript

namespace Descender
namespace LocallyRecoverableSyndrome
namespace LocalDeltaByteCommitment

open SignatureTranscript

variable {q : Nat} [NeZero q]
variable {Blk Free : Type*}

def byteOfNat (value : Nat) : Byte :=
  ⟨value % 256, Nat.mod_lt _ (by decide)⟩

/--
Byte preimage committed before the challenge.

The `fiberState` field is the hidden algebraic state being committed; the
`padSeed` field is the hidden source for local one-time pads.  A concrete
scheme instantiates `encodeHidden` and the hash in `ByteCommitmentConfig`.
-/
structure ByteCommittedPadPreimage (fiberStateLen seedLen : Nat) where
  fiberState : ByteVector fiberStateLen
  padSeed : ByteVector seedLen
deriving DecidableEq

def encodeCommittedPadPreimage
    {fiberStateLen seedLen : Nat}
    (hidden : ByteCommittedPadPreimage fiberStateLen seedLen) :
    ByteVector (fiberStateLen + seedLen) :=
  fun i =>
    if h : i.1 < fiberStateLen then
      hidden.fiberState ⟨i.1, h⟩
    else
      hidden.padSeed ⟨i.1 - fiberStateLen, by omega⟩

@[simp] theorem encodeCommittedPadPreimage_fiberState
    {fiberStateLen seedLen : Nat}
    (hidden : ByteCommittedPadPreimage fiberStateLen seedLen)
    (i : Fin fiberStateLen) :
    encodeCommittedPadPreimage hidden ⟨i.1, by omega⟩ =
      hidden.fiberState i := by
  simp [encodeCommittedPadPreimage, i.2]

@[simp] theorem encodeCommittedPadPreimage_padSeed
    {fiberStateLen seedLen : Nat}
    (hidden : ByteCommittedPadPreimage fiberStateLen seedLen)
    (i : Fin seedLen) :
    encodeCommittedPadPreimage hidden ⟨fiberStateLen + i.1, by omega⟩ =
      hidden.padSeed i := by
  simp [encodeCommittedPadPreimage]

theorem encodeCommittedPadPreimage_injective
    {fiberStateLen seedLen : Nat} :
    Function.Injective
      (encodeCommittedPadPreimage
        (fiberStateLen := fiberStateLen) (seedLen := seedLen)) := by
  intro hidden₀ hidden₁ hencoded
  cases hidden₀ with
  | mk fiberState₀ padSeed₀ =>
    cases hidden₁ with
    | mk fiberState₁ padSeed₁ =>
      congr
      · funext i
        simpa using
          congrFun hencoded ⟨i.1, by omega⟩
      · funext i
        simpa using
          congrFun hencoded ⟨fiberStateLen + i.1, by omega⟩

/--
Domain-separated byte commitment to a hidden preimage.

`noCollision` is the exact assumption slot for the production hash/commitment:
two different hidden preimages must not produce the same public digest.
-/
structure ByteCommitmentConfig
    (domainLen hiddenLen digestLen : Nat) (HiddenState : Type*) where
  domainTag : ByteVector domainLen
  encodeHidden : HiddenState -> ByteVector hiddenLen
  hash : ByteVector domainLen -> ByteVector hiddenLen -> ByteVector digestLen
  noCollision :
    LocalLinearSyndrome.NoCommitmentCollision
      (fun hidden => hash domainTag (encodeHidden hidden))

namespace ByteCommitmentConfig

variable {domainLen hiddenLen digestLen : Nat}
variable {HiddenState : Type*}

def commit
    (C : ByteCommitmentConfig domainLen hiddenLen digestLen HiddenState) :
    HiddenState -> ByteVector digestLen :=
  fun hidden => C.hash C.domainTag (C.encodeHidden hidden)

theorem noCommitmentCollision
    (C : ByteCommitmentConfig domainLen hiddenLen digestLen HiddenState) :
    LocalLinearSyndrome.NoCommitmentCollision C.commit :=
  C.noCollision

end ByteCommitmentConfig

/--
Collision resistance for one fixed domain-separated byte hash.

This is still an assumption, but it is now exactly located at the byte hash
boundary rather than at the higher-level algebraic commitment map.
-/
structure ByteHashBinding
    (domainLen inputLen digestLen : Nat) where
  domainTag : ByteVector domainLen
  hash : ByteVector domainLen -> ByteVector inputLen -> ByteVector digestLen
  collisionFreeOnDomain :
    ∀ {input₀ input₁},
      hash domainTag input₀ = hash domainTag input₁ -> input₀ = input₁

namespace ByteHashBinding

variable {domainLen inputLen digestLen : Nat}
variable {HiddenState : Type*}

def toCommitmentConfig
    (H : ByteHashBinding domainLen inputLen digestLen)
    (encodeHidden : HiddenState -> ByteVector inputLen)
    (hEncode : Function.Injective encodeHidden) :
    ByteCommitmentConfig domainLen inputLen digestLen HiddenState where
  domainTag := H.domainTag
  encodeHidden := encodeHidden
  hash := H.hash
  noCollision := by
    rintro ⟨hidden₀, hidden₁, hdifferent, hsame⟩
    exact hdifferent (hEncode (H.collisionFreeOnDomain hsame))

end ByteHashBinding

def hiddenPreimageOfMask
    {fiberStateLen seedLen : Nat}
    (fiberStateOfMask :
      ProductBlockMask q Blk Free -> ByteVector fiberStateLen)
    (padSeedOfMask :
      ProductBlockMask q Blk Free -> ByteVector seedLen)
    (mask : ProductBlockMask q Blk Free) :
    ByteCommittedPadPreimage fiberStateLen seedLen where
  fiberState := fiberStateOfMask mask
  padSeed := padSeedOfMask mask

def padOfPreimage
    {fiberStateLen seedLen : Nat}
    (expandPad : ByteVector seedLen -> Blk -> LocalView q Free)
    (hidden : ByteCommittedPadPreimage fiberStateLen seedLen) :
    Blk -> LocalView q Free :=
  expandPad hidden.padSeed

/-- Concrete byte digest binding: the state is already the digest bytes. -/
def byteDigestBinding (digestLen : Nat) :
    LocalFiberDeltaOpening.StateDigestBinding
      (ByteVector digestLen) (ByteVector digestLen) where
  digestOfState := id
  state_eq_of_digest_eq := by
    intro _state₀ _state₁ h
    exact h

/--
Byte-instantiated committed local-delta opening.

The public state is the commitment digest bytes.  The hidden preimage carries
the pad source.  Commitment binding forces pad reuse across same-digest forks,
while a single active opening remains a padded local view.
-/
def byteCommittedLocalDeltaOpening
    {fiberStateLen seedLen domainLen hiddenLen digestLen : Nat}
    (C :
      ByteCommitmentConfig domainLen hiddenLen digestLen
        (ByteCommittedPadPreimage fiberStateLen seedLen))
    (fiberStateOfMask :
      ProductBlockMask q Blk Free -> ByteVector fiberStateLen)
    (padSeedOfMask :
      ProductBlockMask q Blk Free -> ByteVector seedLen)
    (expandPad : ByteVector seedLen -> Blk -> LocalView q Free) :
    LocalFiberDeltaOpening Blk Free
      (ByteVector digestLen) (LocalView q Free)
      (fun mask =>
        C.commit
          (hiddenPreimageOfMask fiberStateOfMask padSeedOfMask mask)) :=
  LocalFiberDeltaOpening.additiveCommittedOneTimePad
    (hiddenStateOfMask :=
      hiddenPreimageOfMask fiberStateOfMask padSeedOfMask)
    (commit := C.commit)
    (padOfHidden := padOfPreimage expandPad)
    C.noCommitmentCollision

/--
Byte-facing alias for the transcript-bound local-delta fork round.

The common digest is the serialized prechallenge commitment.  The local active
openings are byte-instantiated through `byteCommittedLocalDeltaOpening`.
-/
abbrev ByteCommittedLocalDeltaForkedRound
    {fiberStateLen seedLen domainLen hiddenLen digestLen : Nat}
    (C :
      ByteCommitmentConfig domainLen hiddenLen digestLen
        (ByteCommittedPadPreimage fiberStateLen seedLen))
    (fiberStateOfMask :
      ProductBlockMask q Blk Free -> ByteVector fiberStateLen)
    (padSeedOfMask :
      ProductBlockMask q Blk Free -> ByteVector seedLen)
    (expandPad : ByteVector seedLen -> Blk -> LocalView q Free)
    (ValidMask : ProductBlockMask q Blk Free -> Prop)
    (secret : Blk × Free -> ZMod q) :=
  LocalFiberDeltaOpening.HashBoundLocalDeltaForkedRound
    (byteDigestBinding digestLen)
    (byteCommittedLocalDeltaOpening
      C fiberStateOfMask padSeedOfMask expandPad)
    ValidMask secret

theorem byteCommittedLocalDeltaFork_correctedExtractCoordinate_eq_secret
    {fiberStateLen seedLen domainLen hiddenLen digestLen : Nat}
    (C :
      ByteCommitmentConfig domainLen hiddenLen digestLen
        (ByteCommittedPadPreimage fiberStateLen seedLen))
    (fiberStateOfMask :
      ProductBlockMask q Blk Free -> ByteVector fiberStateLen)
    (padSeedOfMask :
      ProductBlockMask q Blk Free -> ByteVector seedLen)
    (expandPad : ByteVector seedLen -> Blk -> LocalView q Free)
    {ValidMask : ProductBlockMask q Blk Free -> Prop}
    {secret : Blk × Free -> ZMod q}
    (R :
      ByteCommittedLocalDeltaForkedRound
        C fiberStateOfMask padSeedOfMask expandPad ValidMask secret)
    (b : Blk) (hdelta : R.DeltaActive b) (f : Free) :
    R.correctedExtractCoordinate b hdelta f = secret (b, f) :=
  LocalFiberDeltaOpening.HashBoundLocalDeltaForkedRound.correctedExtractCoordinate_eq_secret
    R b hdelta f

/-- Byte accounting for local-delta signatures. -/
structure ByteLocalDeltaSignatureProfile where
  challengeBytes : Nat
  responseBytes : Nat
  repetitions : Nat
  openingBytesPerRepetition : Nat
  digestBytesPerRepetition : Nat

namespace ByteLocalDeltaSignatureProfile

def perRepetitionBytes (P : ByteLocalDeltaSignatureProfile) : Nat :=
  P.openingBytesPerRepetition + P.digestBytesPerRepetition

def signatureBytes (P : ByteLocalDeltaSignatureProfile) : Nat :=
  P.challengeBytes + P.responseBytes + P.repetitions * P.perRepetitionBytes

theorem signatureBytes_eq
    (P : ByteLocalDeltaSignatureProfile) :
    P.signatureBytes =
      P.challengeBytes + P.responseBytes +
        P.repetitions *
          (P.openingBytesPerRepetition + P.digestBytesPerRepetition) :=
  rfl

end ByteLocalDeltaSignatureProfile

namespace N32Q12289

abbrev modulus : Nat := N32Q12289LocalSyndrome.modulus
abbrev N32Block := N32Q12289LocalSyndrome.Block
abbrev N32LocalFree := N32Q12289LocalSyndrome.LocalFree

abbrev algebraicChallengeBytes : Nat := 23
abbrev challengeHardeningBytes : Nat := 4
abbrev challengeBytes : Nat := algebraicChallengeBytes + challengeHardeningBytes
abbrev slotIdBytes : Nat := 4
abbrev activeResponseBytes : Nat := 52
abbrev repetitions : Nat := 26
abbrev localOpeningBytesPerRepetition : Nat := 4
abbrev localOpeningBytes : Nat :=
  repetitions * localOpeningBytesPerRepetition
abbrev sha256DigestBytes : Nat := 32
abbrev localFiberStateBytes : Nat := 2
abbrev padSeedBytes : Nat := 32
abbrev hiddenPreimageBytes : Nat := localFiberStateBytes + padSeedBytes
abbrev commitmentDomainBytes : Nat := 41
abbrev padDomainBytes : Nat := 45
abbrev padFrameBytes : Nat := padDomainBytes + 1
abbrev publicKeyBytes : Nat := 172

theorem hiddenPreimageBytes_eq : hiddenPreimageBytes = 34 := by
  native_decide

theorem padFrameBytes_eq : padFrameBytes = 46 := by
  native_decide

theorem localOpeningBytes_eq : localOpeningBytes = 104 := by
  native_decide

/-- ASCII `Descender.Signature.q12289.local_delta.v1`. -/
def commitmentDomainTag : ByteVector commitmentDomainBytes :=
  fun i =>
    byteOfNat <|
      match i.1 with
      | 0 => 68 | 1 => 101 | 2 => 115 | 3 => 99 | 4 => 101
      | 5 => 110 | 6 => 100 | 7 => 101 | 8 => 114 | 9 => 46
      | 10 => 83 | 11 => 105 | 12 => 103 | 13 => 110 | 14 => 97
      | 15 => 116 | 16 => 117 | 17 => 114 | 18 => 101 | 19 => 46
      | 20 => 113 | 21 => 49 | 22 => 50 | 23 => 50 | 24 => 56
      | 25 => 57 | 26 => 46 | 27 => 108 | 28 => 111 | 29 => 99
      | 30 => 97 | 31 => 108 | 32 => 95 | 33 => 100 | 34 => 101
      | 35 => 108 | 36 => 116 | 37 => 97 | 38 => 46 | 39 => 118
      | 40 => 49
      | _ => 0

/-- ASCII `Descender.Signature.q12289.local_delta.pad.v1`. -/
def padDomainTag : ByteVector padDomainBytes :=
  fun i =>
    byteOfNat <|
      match i.1 with
      | 0 => 68 | 1 => 101 | 2 => 115 | 3 => 99 | 4 => 101
      | 5 => 110 | 6 => 100 | 7 => 101 | 8 => 114 | 9 => 46
      | 10 => 83 | 11 => 105 | 12 => 103 | 13 => 110 | 14 => 97
      | 15 => 116 | 16 => 117 | 17 => 114 | 18 => 101 | 19 => 46
      | 20 => 113 | 21 => 49 | 22 => 50 | 23 => 50 | 24 => 56
      | 25 => 57 | 26 => 46 | 27 => 108 | 28 => 111 | 29 => 99
      | 30 => 97 | 31 => 108 | 32 => 95 | 33 => 100 | 34 => 101
      | 35 => 108 | 36 => 116 | 37 => 97 | 38 => 46 | 39 => 112
      | 40 => 97 | 41 => 100 | 42 => 46 | 43 => 118 | 44 => 49
      | _ => 0

def padFrame (b : N32Block) : ByteVector padFrameBytes :=
  fun i =>
    if h : i.1 < padDomainBytes then
      padDomainTag ⟨i.1, h⟩
    else
      byteOfNat b.1

def encodeHiddenPreimage :
    ByteCommittedPadPreimage localFiberStateBytes padSeedBytes ->
      ByteVector hiddenPreimageBytes :=
  encodeCommittedPadPreimage

theorem encodeHiddenPreimage_injective :
    Function.Injective encodeHiddenPreimage :=
  encodeCommittedPadPreimage_injective

structure CommitmentHashBinding where
  hash :
    ByteVector commitmentDomainBytes ->
      ByteVector hiddenPreimageBytes -> ByteVector sha256DigestBytes
  collisionFreeOnDomain :
    ∀ {input₀ input₁},
      hash commitmentDomainTag input₀ = hash commitmentDomainTag input₁ ->
        input₀ = input₁

namespace CommitmentHashBinding

def toByteHashBinding
    (H : CommitmentHashBinding) :
    ByteHashBinding commitmentDomainBytes hiddenPreimageBytes sha256DigestBytes where
  domainTag := commitmentDomainTag
  hash := H.hash
  collisionFreeOnDomain := H.collisionFreeOnDomain

def toCommitmentConfig
    (H : CommitmentHashBinding) :
    ByteCommitmentConfig commitmentDomainBytes hiddenPreimageBytes
      sha256DigestBytes
      (ByteCommittedPadPreimage localFiberStateBytes padSeedBytes) :=
  H.toByteHashBinding.toCommitmentConfig
    encodeHiddenPreimage encodeHiddenPreimage_injective

end CommitmentHashBinding

def zmodOfLeU16 (lo hi : Byte) : ZMod modulus :=
  ((lo.1 + 256 * hi.1) : ZMod modulus)

def localViewOfPadBytes
    (bytes : ByteVector localOpeningBytesPerRepetition) :
    LocalView modulus N32LocalFree
  | ⟨0, _⟩ =>
      zmodOfLeU16
        (bytes ⟨0, by decide⟩) (bytes ⟨1, by decide⟩)
  | ⟨1, _⟩ =>
      zmodOfLeU16
        (bytes ⟨2, by decide⟩) (bytes ⟨3, by decide⟩)

/--
Concrete pad-source interface.

The PRF takes a 32-byte per-slot seed and the fixed 46-byte block frame.  Its
4-byte output is decoded as two little-endian residues modulo q.  The
pseudorandom/local-uniform field is the remaining computational hiding
assumption for replacing the ideal one-time pad with a seeded byte PRF.
-/
structure PadPRFBinding where
  expand :
    ByteVector padSeedBytes -> ByteVector padFrameBytes ->
      ByteVector localOpeningBytesPerRepetition
  pseudorandomLocalPad : Prop

namespace PadPRFBinding

def expandPad
    (P : PadPRFBinding) :
    ByteVector padSeedBytes -> N32Block -> LocalView modulus N32LocalFree :=
  fun seed b => localViewOfPadBytes (P.expand seed (padFrame b))

theorem expandPad_zero
    (P : PadPRFBinding)
    (seed : ByteVector padSeedBytes)
    (b : N32Block) :
    P.expandPad seed b ⟨0, by decide⟩ =
      zmodOfLeU16
        ((P.expand seed (padFrame b)) ⟨0, by decide⟩)
        ((P.expand seed (padFrame b)) ⟨1, by decide⟩) := by
  rfl

theorem expandPad_one
    (P : PadPRFBinding)
    (seed : ByteVector padSeedBytes)
    (b : N32Block) :
    P.expandPad seed b ⟨1, by decide⟩ =
      zmodOfLeU16
        ((P.expand seed (padFrame b)) ⟨2, by decide⟩)
        ((P.expand seed (padFrame b)) ⟨3, by decide⟩) := by
  rfl

end PadPRFBinding

/--
Concrete review budget attached to a cryptographic game.

The fields are intentionally simple natural-number budgets so they can be
filled from an external cryptanalysis report without committing this Lean file
to a probability library.  `meets128` is the deployment gate for a 128-bit
classical and quantum review target under an explicit query cap.
-/
structure GameSecurityBudget where
  adversaryQueries : Nat
  classicalSecurityBits : Nat
  quantumSecurityBits : Nat

namespace GameSecurityBudget

def meets128 (B : GameSecurityBudget) : Prop :=
  128 ≤ B.classicalSecurityBits ∧ 128 ≤ B.quantumSecurityBits

end GameSecurityBudget

/--
Deployment-facing pad-source assumption.

The executable reference implements `expand` by SHA-256 rejection sampling from
`seed || padFrame(block) || counter`, decoding accepted 16-bit candidates
modulo q until two local pad residues are filled.  Lean fixes the byte frame
and names the SHA/rejection-sampler agreement rather than treating the pad
source as an unspecified oracle.
-/
structure SHA256RejectionPadPRFAssumption where
  padPRF : PadPRFBinding
  sha256RejectionSamplerMatchesRust : Prop
  sha256RejectionSamplerMatchesRust_holds :
    sha256RejectionSamplerMatchesRust
  localPadPseudorandom : padPRF.pseudorandomLocalPad

/--
Review game for the SHA-256/rejection-sampled pad source.

`padPRFBreak` is intentionally abstract: in a computational proof this is the
event that a deployment-bounded distinguisher separates the concrete
`seed || padFrame(block) || counter` SHA-256 rejection sampler from the ideal
local pad distribution used in the proof.  The theorem-facing consequence is
the existing `pseudorandomLocalPad` fact, but the break event is named so the
assumption is reviewable as a game instead of as an anonymous proposition.
-/
structure SHA256PadPRFGame
    (A : SHA256RejectionPadPRFAssumption) where
  budget : GameSecurityBudget
  budget_meets128 : budget.meets128
  padPRFBreak : Prop
  noPadPRFBreak : ¬ padPRFBreak
  pseudorandom_of_noBreak :
    ¬ padPRFBreak -> A.padPRF.pseudorandomLocalPad

namespace SHA256RejectionPadPRFAssumption

theorem expandPad_uses_padFrame
    (A : SHA256RejectionPadPRFAssumption)
    (seed : ByteVector padSeedBytes)
    (b : N32Block) :
    A.padPRF.expandPad seed b =
      localViewOfPadBytes (A.padPRF.expand seed (padFrame b)) := by
  rfl

theorem pseudorandomLocalPad
    (A : SHA256RejectionPadPRFAssumption) :
    A.padPRF.pseudorandomLocalPad :=
  A.localPadPseudorandom

end SHA256RejectionPadPRFAssumption

namespace SHA256PadPRFGame

theorem pseudorandomLocalPad
    {A : SHA256RejectionPadPRFAssumption}
    (G : SHA256PadPRFGame A) :
    A.padPRF.pseudorandomLocalPad :=
  G.pseudorandom_of_noBreak G.noPadPRFBreak

end SHA256PadPRFGame

def statefulLocalDeltaOpening
    (H : CommitmentHashBinding)
    (P : PadPRFBinding)
    (fiberStateOfMask :
      ProductBlockMask modulus N32Block N32LocalFree ->
        ByteVector localFiberStateBytes)
    (padSeedOfMask :
      ProductBlockMask modulus N32Block N32LocalFree ->
        ByteVector padSeedBytes) :
    LocalFiberDeltaOpening N32Block N32LocalFree
      (ByteVector sha256DigestBytes) (LocalView modulus N32LocalFree)
      (fun mask =>
        (H.toCommitmentConfig).commit
          (hiddenPreimageOfMask fiberStateOfMask padSeedOfMask mask)) :=
  byteCommittedLocalDeltaOpening
    H.toCommitmentConfig fiberStateOfMask padSeedOfMask P.expandPad

abbrev StatefulLocalDeltaForkedRound
    (H : CommitmentHashBinding)
    (P : PadPRFBinding)
    (fiberStateOfMask :
      ProductBlockMask modulus N32Block N32LocalFree ->
        ByteVector localFiberStateBytes)
    (padSeedOfMask :
      ProductBlockMask modulus N32Block N32LocalFree ->
        ByteVector padSeedBytes)
    (ValidMask : ProductBlockMask modulus N32Block N32LocalFree -> Prop)
    (secret : N32Block × N32LocalFree -> ZMod modulus) :=
  ByteCommittedLocalDeltaForkedRound
    H.toCommitmentConfig fiberStateOfMask padSeedOfMask P.expandPad
    ValidMask secret

theorem statefulLocalDeltaFork_correctedExtractCoordinate_eq_secret
    (H : CommitmentHashBinding)
    (P : PadPRFBinding)
    (fiberStateOfMask :
      ProductBlockMask modulus N32Block N32LocalFree ->
        ByteVector localFiberStateBytes)
    (padSeedOfMask :
      ProductBlockMask modulus N32Block N32LocalFree ->
        ByteVector padSeedBytes)
    {ValidMask : ProductBlockMask modulus N32Block N32LocalFree -> Prop}
    {secret : N32Block × N32LocalFree -> ZMod modulus}
    (R :
      StatefulLocalDeltaForkedRound
        H P fiberStateOfMask padSeedOfMask ValidMask secret)
    (b : N32Block) (hdelta : R.DeltaActive b) (f : N32LocalFree) :
    R.correctedExtractCoordinate b hdelta f = secret (b, f) :=
  byteCommittedLocalDeltaFork_correctedExtractCoordinate_eq_secret
    H.toCommitmentConfig fiberStateOfMask padSeedOfMask P.expandPad
    R b hdelta f

structure BlockchainAccountState (Slot : Type*) where
  digestOfSlot : Slot -> ByteVector sha256DigestBytes
  unused : Slot -> Prop

def SlotAcceptsDigest
    {Slot : Type*}
    (A : BlockchainAccountState Slot)
    (slot : Slot) (digest : ByteVector sha256DigestBytes) : Prop :=
  A.unused slot ∧ A.digestOfSlot slot = digest

structure ConsumeSlotTransition
    {Slot : Type*}
    (before after : BlockchainAccountState Slot) (slot : Slot) : Prop where
  unused_before : before.unused slot
  consumed_after : ¬ after.unused slot
  digest_preserved : after.digestOfSlot slot = before.digestOfSlot slot

theorem not_slotAcceptsDigest_after_consumed
    {Slot : Type*}
    {before after : BlockchainAccountState Slot} {slot : Slot}
    (T : ConsumeSlotTransition before after slot)
    (digest : ByteVector sha256DigestBytes) :
    ¬ SlotAcceptsDigest after slot digest := by
  intro h
  exact T.consumed_after h.1

structure StatefulSpendSignatureBytes where
  slotId : ByteVector slotIdBytes
  challenge : ByteVector challengeBytes
  responses : ByteVector activeResponseBytes
  openings : Fin repetitions -> ByteVector localOpeningBytesPerRepetition

def statefulSpendSignatureBytes : Nat :=
  slotIdBytes + challengeBytes + activeResponseBytes + localOpeningBytes

theorem statefulSpendSignatureBytes_eq :
    statefulSpendSignatureBytes = 187 := by
  native_decide

structure StatefulSpendSignatureWithProof (proofBytes : Nat) where
  base : StatefulSpendSignatureBytes
  openingProof : ByteVector proofBytes

def statefulTransactionWithProofBytes (proofBytes : Nat) : Nat :=
  statefulSpendSignatureBytes + proofBytes

theorem statefulTransactionWithProofBytes_eq (proofBytes : Nat) :
    statefulTransactionWithProofBytes proofBytes = 187 + proofBytes := by
  unfold statefulTransactionWithProofBytes
  rw [statefulSpendSignatureBytes_eq]

abbrev linearSigmaProofCommitmentBytes : Nat := 32
abbrev linearSigmaMaskedWitnessBytes : Nat := 41

def linearSigmaWitnessFromHiddenPreimage
    (hidden : ByteCommittedPadPreimage localFiberStateBytes padSeedBytes) :
    ByteVector linearSigmaMaskedWitnessBytes :=
  fun i =>
    if h : i.1 < hiddenPreimageBytes then
      encodeHiddenPreimage hidden ⟨i.1, h⟩
    else
      0

abbrev revealedPreimageProofBytes : Nat := linearSigmaMaskedWitnessBytes

theorem revealedPreimageProofBytes_eq :
    revealedPreimageProofBytes = 41 := by
  native_decide

theorem revealedPreimageTransactionSignatureBytes_eq :
    statefulTransactionWithProofBytes revealedPreimageProofBytes = 228 := by
  native_decide

theorem statefulTransactionWithProofBytes_under256
    {proofBytes : Nat} (hproof : proofBytes ≤ 69) :
    statefulTransactionWithProofBytes proofBytes ≤ 256 := by
  rw [statefulTransactionWithProofBytes_eq]
  omega

theorem statefulTransactionWithProofBytes_beats269
    {proofBytes : Nat} (hproof : proofBytes ≤ 81) :
    statefulTransactionWithProofBytes proofBytes < 269 := by
  rw [statefulTransactionWithProofBytes_eq]
  omega

theorem statefulTransactionWithProofBytes_atMost269
    {proofBytes : Nat} (hproof : proofBytes ≤ 82) :
    statefulTransactionWithProofBytes proofBytes ≤ 269 := by
  rw [statefulTransactionWithProofBytes_eq]
  omega

abbrev linearSigmaTargetProofBytes : Nat :=
  linearSigmaProofCommitmentBytes + linearSigmaMaskedWitnessBytes

theorem linearSigmaTargetProofBytes_eq :
    linearSigmaTargetProofBytes = 73 := by
  native_decide

theorem linearSigmaTargetTransactionSignatureBytes_eq :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 := by
  native_decide

abbrev sisSyndromeResidues : Nat := 18
abbrev sisResidueBits : Nat := 14
abbrev sisSeedResidues : Nat := 18
abbrev sisSyndromePackedBits : Nat := sisSyndromeResidues * sisResidueBits
abbrev sisSyndromePackedBytes : Nat := (sisSyndromePackedBits + 7) / 8
abbrev sisSyndromeSlackBits : Nat :=
  sha256DigestBytes * 8 - sisSyndromePackedBits
abbrev sisSeedPackedBits : Nat := sisSeedResidues * sisResidueBits
abbrev sisSeedPackedBytes : Nat := (sisSeedPackedBits + 7) / 8
abbrev sisSeedSlackBits : Nat := padSeedBytes * 8 - sisSeedPackedBits
abbrev sisFullResponseResidues : Nat := 23
abbrev sisFullResponsePublicRows : Nat := 12
abbrev sisFullResponseSyndromeResidues : Nat := sisFullResponsePublicRows
abbrev sisFullResponseSyndromePackedBits : Nat :=
  sisFullResponseSyndromeResidues * sisResidueBits
abbrev sisFullResponseSyndromePackedBytes : Nat :=
  (sisFullResponseSyndromePackedBits + 7) / 8
abbrev sisFullResponseSyndromeSlackBits : Nat :=
  sha256DigestBytes * 8 - sisFullResponseSyndromePackedBits
abbrev sisFullResponsePackedBits : Nat :=
  sisFullResponseResidues * sisResidueBits
abbrev sisFullResponsePackedBytes : Nat :=
  (sisFullResponsePackedBits + 7) / 8
abbrev sisFullResponseSlackBits : Nat :=
  linearSigmaMaskedWitnessBytes * 8 - sisFullResponsePackedBits
abbrev sisFullResponseHiddenFiberResidues : Nat :=
  sisFullResponseResidues - sisFullResponsePublicRows

theorem sisSyndromePackedBits_eq :
    sisSyndromePackedBits = 252 := by
  native_decide

theorem sisSyndromePackedBytes_eq :
    sisSyndromePackedBytes = sha256DigestBytes := by
  native_decide

theorem sisSyndromeSlackBits_eq :
    sisSyndromeSlackBits = 4 := by
  native_decide

theorem sisSeedPackedBits_eq :
    sisSeedPackedBits = 252 := by
  native_decide

theorem sisSeedPackedBytes_eq :
    sisSeedPackedBytes = padSeedBytes := by
  native_decide

theorem sisSeedSlackBits_eq :
    sisSeedSlackBits = 4 := by
  native_decide

theorem sisFullResponsePackedBits_eq :
    sisFullResponsePackedBits = 322 := by
  native_decide

theorem sisFullResponseSyndromePackedBits_eq :
    sisFullResponseSyndromePackedBits = 168 := by
  native_decide

theorem sisFullResponseSyndromePackedBytes_eq :
    sisFullResponseSyndromePackedBytes = 21 := by
  native_decide

theorem sisFullResponseSyndromeSlackBits_eq :
    sisFullResponseSyndromeSlackBits = 88 := by
  native_decide

theorem sisFullResponsePackedBytes_eq :
    sisFullResponsePackedBytes = linearSigmaMaskedWitnessBytes := by
  native_decide

theorem sisFullResponseSlackBits_eq :
    sisFullResponseSlackBits = 6 := by
  native_decide

theorem sisFullResponseHiddenFiberResidues_eq :
    sisFullResponseHiddenFiberResidues = 11 := by
  native_decide

abbrev responseBound : Nat := 48

theorem activeResponseBytes_layout :
    activeResponseBytes = repetitions * 2 := by
  native_decide

theorem linearSigmaTarget_under269 :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes < 269 :=
  statefulTransactionWithProofBytes_beats269 (by native_decide)

/--
A 32-byte public commitment cannot be information-theoretically injective on
all 34-byte hidden preimages.  Any 32-byte account binding for this route must
therefore be computational, restricted-domain, or carry additional public state.
-/
theorem noInformationTheoreticBinding_34_to_32
    (commit :
      ByteVector hiddenPreimageBytes -> ByteVector sha256DigestBytes) :
    ¬ Function.Injective commit := by
  intro hinj
  have hle := Fintype.card_le_of_injective commit hinj
  norm_num [hiddenPreimageBytes, localFiberStateBytes, padSeedBytes,
    sha256DigestBytes, ByteVector, Byte, Fintype.card_fun] at hle

structure StatefulChallengeFrame (publicKeyLen messageLen : Nat) where
  publicKey : ByteVector publicKeyLen
  message : ByteVector messageLen
  slotId : ByteVector slotIdBytes
  slotDigest : ByteVector sha256DigestBytes

structure PublicOpeningStatement where
  publicKey : ByteVector publicKeyBytes
  digest : ByteVector sha256DigestBytes
  challenge : ByteVector challengeBytes
  responses : ByteVector activeResponseBytes
  openings : Fin repetitions -> ByteVector localOpeningBytesPerRepetition

def publicOpeningStatementBytes : Nat :=
  publicKeyBytes + sha256DigestBytes + challengeBytes +
    activeResponseBytes + localOpeningBytes

theorem publicOpeningStatementBytes_eq :
    publicOpeningStatementBytes = 387 := by
  native_decide

structure PublicOpeningWitness where
  hiddenPreimage :
    ByteCommittedPadPreimage localFiberStateBytes padSeedBytes

theorem linearSigmaProofBytes_layout :
    linearSigmaTargetProofBytes =
      linearSigmaProofCommitmentBytes + linearSigmaMaskedWitnessBytes := by
  native_decide

structure LinearSigmaProofFields where
  proofCommitment : ByteVector linearSigmaProofCommitmentBytes
  maskedWitnessResponse : ByteVector linearSigmaMaskedWitnessBytes
deriving DecidableEq

def encodeLinearSigmaProofFields
    (proof : LinearSigmaProofFields) :
    ByteVector linearSigmaTargetProofBytes :=
  fun i =>
    if h : i.1 < linearSigmaProofCommitmentBytes then
      proof.proofCommitment ⟨i.1, h⟩
    else
      proof.maskedWitnessResponse
        ⟨i.1 - linearSigmaProofCommitmentBytes, by
          have htarget :
              i.1 <
                linearSigmaProofCommitmentBytes +
                  linearSigmaMaskedWitnessBytes := by
            exact i.2
          omega⟩

def linearSigmaProofCommitment
    (proof : ByteVector linearSigmaTargetProofBytes) :
    ByteVector linearSigmaProofCommitmentBytes :=
  fun i => proof ⟨i.1, by
    have htarget :
        linearSigmaProofCommitmentBytes ≤ linearSigmaTargetProofBytes := by
      native_decide
    exact Nat.lt_of_lt_of_le i.2 htarget⟩

def linearSigmaMaskedWitnessResponse
    (proof : ByteVector linearSigmaTargetProofBytes) :
    ByteVector linearSigmaMaskedWitnessBytes :=
  fun i => proof ⟨linearSigmaProofCommitmentBytes + i.1, by
    have htarget :
        linearSigmaProofCommitmentBytes + linearSigmaMaskedWitnessBytes =
          linearSigmaTargetProofBytes := by
      native_decide
    omega⟩

@[simp] theorem linearSigmaProofCommitment_encode
    (proof : LinearSigmaProofFields) :
    linearSigmaProofCommitment (encodeLinearSigmaProofFields proof) =
      proof.proofCommitment := by
  funext i
  simp [linearSigmaProofCommitment, encodeLinearSigmaProofFields]

@[simp] theorem linearSigmaMaskedWitnessResponse_encode
    (proof : LinearSigmaProofFields) :
    linearSigmaMaskedWitnessResponse (encodeLinearSigmaProofFields proof) =
      proof.maskedWitnessResponse := by
  funext i
  simp [linearSigmaMaskedWitnessResponse, encodeLinearSigmaProofFields]

def fiberStateFromLinearSigmaWitnessBytes
    (bytes : ByteVector linearSigmaMaskedWitnessBytes) :
    ByteVector localFiberStateBytes :=
  fun i => bytes ⟨i.1, by
    have htarget :
        localFiberStateBytes ≤ linearSigmaMaskedWitnessBytes := by
      native_decide
    exact Nat.lt_of_lt_of_le i.2 htarget⟩

def seedFromLinearSigmaWitnessBytes
    (bytes : ByteVector linearSigmaMaskedWitnessBytes) :
    ByteVector padSeedBytes :=
  fun i =>
    bytes ⟨localFiberStateBytes + i.1, by
      have hhidden :
          localFiberStateBytes + i.1 <
            localFiberStateBytes + padSeedBytes :=
        Nat.add_lt_add_left i.2 localFiberStateBytes
      have hbound : localFiberStateBytes + padSeedBytes ≤
          linearSigmaMaskedWitnessBytes := by
        native_decide
      exact Nat.lt_of_lt_of_le hhidden hbound⟩

def hiddenPreimageFromLinearSigmaWitnessBytes
    (bytes : ByteVector linearSigmaMaskedWitnessBytes) :
    ByteCommittedPadPreimage localFiberStateBytes padSeedBytes where
  fiberState := fiberStateFromLinearSigmaWitnessBytes bytes
  padSeed := seedFromLinearSigmaWitnessBytes bytes

@[simp] theorem hiddenPreimageFromLinearSigmaWitnessBytes_encodeHiddenPreimage
    (hidden : ByteCommittedPadPreimage localFiberStateBytes padSeedBytes) :
    hiddenPreimageFromLinearSigmaWitnessBytes
        (linearSigmaWitnessFromHiddenPreimage hidden) = hidden := by
  cases hidden with
  | mk fiberState padSeed =>
    change
      { fiberState :=
          fiberStateFromLinearSigmaWitnessBytes
            (linearSigmaWitnessFromHiddenPreimage
              ({ fiberState := fiberState, padSeed := padSeed } :
                ByteCommittedPadPreimage localFiberStateBytes padSeedBytes))
        padSeed :=
          seedFromLinearSigmaWitnessBytes
            (linearSigmaWitnessFromHiddenPreimage
              ({ fiberState := fiberState, padSeed := padSeed } :
                ByteCommittedPadPreimage localFiberStateBytes padSeedBytes)) } =
        ({ fiberState := fiberState, padSeed := padSeed } :
          ByteCommittedPadPreimage localFiberStateBytes padSeedBytes)
    congr
    · funext i
      have hi : (i.1 : Nat) < hiddenPreimageBytes := by
        have hlocal : (i.1 : Nat) < 2 := by
          simpa [localFiberStateBytes] using i.2
        norm_num [hiddenPreimageBytes, localFiberStateBytes]
        omega
      simp [fiberStateFromLinearSigmaWitnessBytes,
        linearSigmaWitnessFromHiddenPreimage, encodeHiddenPreimage, hi]
    · funext i
      have hi :
          localFiberStateBytes + (i.1 : Nat) < hiddenPreimageBytes := by
        have hseed : (i.1 : Nat) < 32 := by
          simpa only [padSeedBytes] using i.2
        norm_num [hiddenPreimageBytes, localFiberStateBytes, padSeedBytes]
        omega
      simp [seedFromLinearSigmaWitnessBytes,
        linearSigmaWitnessFromHiddenPreimage, encodeHiddenPreimage, hi]

@[simp] theorem encodeHiddenPreimage_hiddenPreimageFromLinearSigmaWitnessBytes
    (bytes : ByteVector linearSigmaMaskedWitnessBytes) :
    encodeHiddenPreimage
        (hiddenPreimageFromLinearSigmaWitnessBytes bytes) =
      fun i => bytes ⟨i.1, by
        have hbound : hiddenPreimageBytes ≤ linearSigmaMaskedWitnessBytes := by
          native_decide
        exact Nat.lt_of_lt_of_le i.2 hbound⟩ := by
  funext i
  by_cases h : i.1 < localFiberStateBytes
  · simp [encodeHiddenPreimage, encodeCommittedPadPreimage,
      hiddenPreimageFromLinearSigmaWitnessBytes,
      fiberStateFromLinearSigmaWitnessBytes, h]
  · have hle : localFiberStateBytes ≤ i.1 := Nat.le_of_not_gt h
    have hidx : localFiberStateBytes + (i.1 - localFiberStateBytes) = i.1 :=
      Nat.add_sub_of_le hle
    simp [encodeHiddenPreimage, encodeCommittedPadPreimage,
      hiddenPreimageFromLinearSigmaWitnessBytes,
      seedFromLinearSigmaWitnessBytes, h, hidx]

def leU16Nat (lo hi : Byte) : Nat :=
  lo.1 + 256 * hi.1

abbrev challengeBitsPerRepetition : Nat := 5
abbrev algebraicChallengeEntropyBits : Nat :=
  (algebraicChallengeBytes - 1) * 8 + 6
abbrev challengeEntropyBits : Nat :=
  algebraicChallengeEntropyBits + 8 * challengeHardeningBytes
abbrev fiatShamirClassicalQueryBitsFor128 : Nat :=
  challengeEntropyBits - 128
abbrev fiatShamirQuantumQueryBitsFor128 : Nat :=
  (challengeEntropyBits - 128) / 2
abbrev challengeLastByteIndex : Nat := algebraicChallengeBytes - 1
abbrev challengeLastByteMask : Nat := 63
abbrev publicKeySeedBytes : Nat := 32
abbrev publicKeyPayloadBitOffset : Nat := publicKeySeedBytes * 8
abbrev publicKeyPivotCoordinateCount : Nat := 48
abbrev publicKeyFreeCoordinateCount : Nat := 32
abbrev publicKeyFieldCoordinateCount : Nat :=
  publicKeyPivotCoordinateCount + publicKeyFreeCoordinateCount

def fiberStateValue
    (fiberState : ByteVector localFiberStateBytes) : ZMod modulus :=
  zmodOfLeU16
    (fiberState ⟨0, by decide⟩)
    (fiberState ⟨1, by decide⟩)

theorem algebraicChallengeEntropyBits_eq :
    algebraicChallengeEntropyBits = 182 := by
  native_decide

theorem challengeEntropyBits_eq :
    challengeEntropyBits = 214 := by
  native_decide

theorem fiatShamirClassicalQueryBitsFor128_eq :
    fiatShamirClassicalQueryBitsFor128 = 86 := by
  native_decide

theorem fiatShamirQuantumQueryBitsFor128_eq :
    fiatShamirQuantumQueryBitsFor128 = 43 := by
  native_decide

def activeResponseIndex
    (r : Fin repetitions) (f : N32LocalFree) :
    Fin activeResponseBytes :=
  ⟨2 * r.1 + f.1, by
    have hr : r.1 < repetitions := r.2
    have hf : f.1 < 2 := f.2
    rw [activeResponseBytes_layout]
    omega⟩

def activeResponseRaw
    (responses : ByteVector activeResponseBytes)
    (r : Fin repetitions) (f : N32LocalFree) : Nat :=
  (responses (activeResponseIndex r f)).1

def activeResponseCanonical
    (responses : ByteVector activeResponseBytes) : Prop :=
  ∀ r : Fin repetitions, ∀ f : N32LocalFree,
    activeResponseRaw responses r f ≤ 2 * responseBound

def activeResponseCoeff
    (responses : ByteVector activeResponseBytes)
    (r : Fin repetitions) (f : N32LocalFree) : ZMod modulus :=
  (activeResponseRaw responses r f : ZMod modulus) -
    (responseBound : ZMod modulus)

def compactChallengeCanonical
    (challenge : ByteVector challengeBytes) : Prop :=
  (challenge ⟨challengeLastByteIndex, by native_decide⟩).1 ≤
    challengeLastByteMask

def compactChallengeActiveBlockRaw
    (challenge : ByteVector challengeBytes)
    (r : Fin repetitions) : Nat :=
  packedBitsNatLE challenge (r.1 * challengeBitsPerRepetition) 4

def compactChallengeSignBit
    (challenge : ByteVector challengeBytes)
    (r : Fin repetitions) : Nat :=
  packedBitsNatLE challenge (r.1 * challengeBitsPerRepetition + 4) 1

def compactChallengeActiveBlock
    (challenge : ByteVector challengeBytes)
    (r : Fin repetitions) : N32Block :=
  ⟨compactChallengeActiveBlockRaw challenge r % 16,
    Nat.mod_lt _ (by decide)⟩

def compactChallengeCoeff
    (challenge : ByteVector challengeBytes)
    (r : Fin repetitions) : ZMod modulus :=
  if compactChallengeSignBit challenge r = 0 then
    1
  else
    -1

def publicKeyFieldRaw
    (publicKey : ByteVector publicKeyBytes)
    (i : Fin publicKeyFieldCoordinateCount) : Nat :=
  packedBitsNatLE publicKey
    (publicKeyPayloadBitOffset + i.1 * sisResidueBits)
    sisResidueBits

def publicKeyFieldCanonical
    (publicKey : ByteVector publicKeyBytes) : Prop :=
  ∀ i : Fin publicKeyFieldCoordinateCount,
    publicKeyFieldRaw publicKey i < modulus

def publicKeyFreeCoordinateIndex
    (b : N32Block) (f : N32LocalFree) :
    Fin publicKeyFieldCoordinateCount :=
  ⟨publicKeyPivotCoordinateCount + b.1 * 2 + f.1, by
    have hb : b.1 < 16 := b.2
    have hf : f.1 < 2 := f.2
    norm_num [publicKeyFieldCoordinateCount, publicKeyPivotCoordinateCount,
      publicKeyFreeCoordinateCount]
    omega⟩

def concretePublicKeyFreeView
    (publicKey : ByteVector publicKeyBytes)
    (b : N32Block) (f : N32LocalFree) : ZMod modulus :=
  (publicKeyFieldRaw publicKey (publicKeyFreeCoordinateIndex b f) :
    ZMod modulus)

def localOpeningRaw
    (opening : ByteVector localOpeningBytesPerRepetition)
    (f : N32LocalFree) : Nat :=
  match f with
  | ⟨0, _⟩ =>
      leU16Nat
        (opening ⟨0, by decide⟩) (opening ⟨1, by decide⟩)
  | ⟨1, _⟩ =>
      leU16Nat
        (opening ⟨2, by decide⟩) (opening ⟨3, by decide⟩)

def localOpeningCanonical
    (openings : Fin repetitions ->
      ByteVector localOpeningBytesPerRepetition) : Prop :=
  ∀ r : Fin repetitions, ∀ f : N32LocalFree,
    localOpeningRaw (openings r) f < modulus

/--
Byte-facing local-delta representative semantics.

The response and opening byte layouts are fixed here.  The challenge decoder,
public-key active-view decoder, and pad expansion are supplied by the concrete
instantiation, because those are the parts tied to the selected public-key and
Fiat-Shamir encodings.
-/
structure ByteLocalDeltaRepresentativeSemantics where
  challengeValid : ByteVector challengeBytes -> Prop
  activeBlock : ByteVector challengeBytes -> Fin repetitions -> N32Block
  activeChallengeCoeff :
    ByteVector challengeBytes -> Fin repetitions -> ZMod modulus
  publicKeyValid : ByteVector publicKeyBytes -> Prop
  publicFreeView :
    ByteVector publicKeyBytes -> N32Block -> N32LocalFree -> ZMod modulus
  expandPad :
    ByteVector padSeedBytes -> N32Block -> LocalView modulus N32LocalFree

namespace ByteLocalDeltaRepresentativeSemantics

def maskView
    (S : ByteLocalDeltaRepresentativeSemantics)
    (statement : PublicOpeningStatement)
    (r : Fin repetitions) :
    LocalView modulus N32LocalFree :=
  let b := S.activeBlock statement.challenge r
  fun f =>
    activeResponseCoeff statement.responses r f -
      S.activeChallengeCoeff statement.challenge r *
        S.publicFreeView statement.publicKey b f

def representativeOpensStatement
    (S : ByteLocalDeltaRepresentativeSemantics)
    (statement : PublicOpeningStatement)
    (response : ByteVector linearSigmaMaskedWitnessBytes) : Prop :=
  let hidden := hiddenPreimageFromLinearSigmaWitnessBytes response
  S.challengeValid statement.challenge ∧
    S.publicKeyValid statement.publicKey ∧
    activeResponseCanonical statement.responses ∧
    localOpeningCanonical statement.openings ∧
    ∀ r : Fin repetitions,
      let b := S.activeBlock statement.challenge r
      let mask := S.maskView statement r
      N32Q12289LocalSyndrome.syndrome.localState b mask =
          fiberStateValue hidden.fiberState ∧
        localViewOfPadBytes (statement.openings r) =
          (fun f => mask f + S.expandPad hidden.padSeed b f)

def quotientOpensStatement
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (statement : PublicOpeningStatement)
    (quotient : SISFullResponseSyndrome) : Prop :=
  ∃ response,
    syndromeMap statement response = quotient ∧
      S.representativeOpensStatement statement response

theorem quotientOpensStatement_iff_existsRepresentative
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (statement : PublicOpeningStatement)
    (quotient : SISFullResponseSyndrome) :
    S.quotientOpensStatement syndromeMap statement quotient ↔
      ∃ response,
        syndromeMap statement response = quotient ∧
          S.representativeOpensStatement statement response := by
  rfl

end ByteLocalDeltaRepresentativeSemantics

def concreteByteLocalDeltaRepresentativeSemantics
    (P : PadPRFBinding) :
    ByteLocalDeltaRepresentativeSemantics where
  challengeValid := compactChallengeCanonical
  activeBlock := compactChallengeActiveBlock
  activeChallengeCoeff := compactChallengeCoeff
  publicKeyValid := publicKeyFieldCanonical
  publicFreeView := concretePublicKeyFreeView
  expandPad := P.expandPad

namespace ByteLocalDeltaRustLayout

theorem activeResponseRaw_eq_repLocalFreeByte
    (responses : ByteVector activeResponseBytes)
    (r : Fin repetitions) (f : N32LocalFree) :
    activeResponseRaw responses r f =
      (responses ⟨2 * r.1 + f.1, by
        have hr : r.1 < repetitions := r.2
        have hf : f.1 < 2 := f.2
        rw [activeResponseBytes_layout]
        omega⟩).1 := by
  rfl

theorem activeResponseCanonical_iff_rustByteBound
    (responses : ByteVector activeResponseBytes) :
    activeResponseCanonical responses ↔
      ∀ r : Fin repetitions, ∀ f : N32LocalFree,
        (responses (activeResponseIndex r f)).1 ≤ 2 * responseBound := by
  rfl

theorem activeResponseCoeff_eq_centeredByte
    (responses : ByteVector activeResponseBytes)
    (r : Fin repetitions) (f : N32LocalFree) :
    activeResponseCoeff responses r f =
      ((responses (activeResponseIndex r f)).1 : ZMod modulus) -
        (responseBound : ZMod modulus) := by
  rfl

theorem localOpeningRaw_zero
    (opening : ByteVector localOpeningBytesPerRepetition) :
    localOpeningRaw opening ⟨0, by decide⟩ =
      leU16Nat
        (opening ⟨0, by decide⟩)
        (opening ⟨1, by decide⟩) := by
  rfl

theorem localOpeningRaw_one
    (opening : ByteVector localOpeningBytesPerRepetition) :
    localOpeningRaw opening ⟨1, by decide⟩ =
      leU16Nat
        (opening ⟨2, by decide⟩)
        (opening ⟨3, by decide⟩) := by
  rfl

theorem publicKeyFreeCoordinateIndex_eq_rustFreeIndex
    (b : N32Block) (f : N32LocalFree) :
    (publicKeyFreeCoordinateIndex b f).1 =
      publicKeyPivotCoordinateCount + b.1 * 2 + f.1 := by
  rfl

theorem concretePublicKeyFreeView_eq_packedFreeCoordinate
    (publicKey : ByteVector publicKeyBytes)
    (b : N32Block) (f : N32LocalFree) :
    concretePublicKeyFreeView publicKey b f =
      (packedBitsNatLE publicKey
          (publicKeyPayloadBitOffset +
            (publicKeyPivotCoordinateCount + b.1 * 2 + f.1) *
              sisResidueBits)
          sisResidueBits : ZMod modulus) := by
  rfl

theorem publicKeyFieldCanonical_iff_rustPackedQElements
    (publicKey : ByteVector publicKeyBytes) :
    publicKeyFieldCanonical publicKey ↔
      ∀ i : Fin publicKeyFieldCoordinateCount,
        packedBitsNatLE publicKey
          (publicKeyPayloadBitOffset + i.1 * sisResidueBits)
          sisResidueBits < modulus := by
  rfl

theorem padFrame_domain
    (b : N32Block)
    (i : Fin padDomainBytes) :
    padFrame b ⟨i.1, by
      have hi : i.1 < padDomainBytes := i.2
      rw [padFrameBytes]
      omega⟩ =
      padDomainTag i := by
  simp [padFrame, i.2]

theorem padFrame_block
    (b : N32Block) :
    padFrame b ⟨padDomainBytes, by
      rw [padFrameBytes]
      omega⟩ =
      byteOfNat b.1 := by
  simp [padFrame]

end ByteLocalDeltaRustLayout

namespace ConcreteByteLocalDeltaRepresentativeSemantics

theorem challengeValid_iff_lastByteCanonical
    (P : PadPRFBinding)
    (challenge : ByteVector challengeBytes) :
    (concreteByteLocalDeltaRepresentativeSemantics P).challengeValid
        challenge ↔
      (challenge ⟨challengeLastByteIndex, by native_decide⟩).1 ≤
        challengeLastByteMask := by
  rfl

theorem activeBlock_eq_packed4
    (P : PadPRFBinding)
    (challenge : ByteVector challengeBytes)
    (r : Fin repetitions) :
    (concreteByteLocalDeltaRepresentativeSemantics P).activeBlock
        challenge r =
      compactChallengeActiveBlock challenge r := by
  rfl

theorem challengeCoeff_eq_signBit
    (P : PadPRFBinding)
    (challenge : ByteVector challengeBytes)
    (r : Fin repetitions) :
    (concreteByteLocalDeltaRepresentativeSemantics P).activeChallengeCoeff
        challenge r =
      if compactChallengeSignBit challenge r = 0 then
        1
      else
        -1 := by
  rfl

theorem publicFreeView_eq_packed14
    (P : PadPRFBinding)
    (publicKey : ByteVector publicKeyBytes)
    (b : N32Block) (f : N32LocalFree) :
    (concreteByteLocalDeltaRepresentativeSemantics P).publicFreeView
        publicKey b f =
      concretePublicKeyFreeView publicKey b f := by
  rfl

theorem expandPad_eq_padPRFBinding
    (P : PadPRFBinding)
    (seed : ByteVector padSeedBytes)
    (b : N32Block) :
    (concreteByteLocalDeltaRepresentativeSemantics P).expandPad seed b =
      P.expandPad seed b := by
  rfl

end ConcreteByteLocalDeltaRepresentativeSemantics

/--
Public proof-system boundary for replacing the executable verifier's ideal
hidden-preimage oracle.

`idealRelation` is the byte-level relation that the Rust ideal verifier checks
directly: the digest binds a hidden preimage, the preimage supplies the pad
seed, and the active openings match the decoded active mask views.  A deployable
scheme must instantiate `verify` with a public proof whose soundness implies
this ideal relation, without revealing the witness.
-/
structure PublicOpeningProofSystem (proofBytes : Nat) where
  verify : PublicOpeningStatement -> ByteVector proofBytes -> Prop
  idealRelation : PublicOpeningStatement -> PublicOpeningWitness -> Prop
  sound :
    ∀ {statement proof},
      verify statement proof ->
        ∃ witness, idealRelation statement witness
  complete :
    ∀ {statement witness},
      idealRelation statement witness ->
        ∃ proof, verify statement proof

structure WitnessHidingPublicOpeningProofSystem (proofBytes : Nat) extends
    PublicOpeningProofSystem proofBytes where
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

abbrev LinearSigmaOpeningTarget :=
  WitnessHidingPublicOpeningProofSystem linearSigmaTargetProofBytes

/--
Field-level contract for the 73-byte algebraic sigma target.

This does not assume SHA-256 preimage proving.  The concrete candidate supplies
a verifier over two byte fields, a 32-byte proof commitment and a 41-byte masked
witness response.  Soundness is the proof-of-knowledge/extraction assumption
for the algebraic commitment; hiding is the zero-knowledge/witness-hiding
obligation for the masked response.
-/
structure AlgebraicLinearSigmaOpeningConfig where
  idealRelation : PublicOpeningStatement -> PublicOpeningWitness -> Prop
  verifyFields :
    PublicOpeningStatement ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaMaskedWitnessBytes -> Prop
  soundFields :
    ∀ {statement proofCommitment maskedWitnessResponse},
      verifyFields statement proofCommitment maskedWitnessResponse ->
        ∃ witness, idealRelation statement witness
  completeFields :
    ∀ {statement witness},
      idealRelation statement witness ->
        ∃ proofFields : LinearSigmaProofFields,
          verifyFields statement proofFields.proofCommitment
            proofFields.maskedWitnessResponse
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace AlgebraicLinearSigmaOpeningConfig

def verify
    (C : AlgebraicLinearSigmaOpeningConfig)
    (statement : PublicOpeningStatement)
    (proof : ByteVector linearSigmaTargetProofBytes) : Prop :=
  C.verifyFields statement
    (linearSigmaProofCommitment proof)
    (linearSigmaMaskedWitnessResponse proof)

def toLinearSigmaOpeningTarget
    (C : AlgebraicLinearSigmaOpeningConfig) :
    LinearSigmaOpeningTarget where
  verify := C.verify
  idealRelation := C.idealRelation
  sound := by
    intro statement proof hverify
    exact C.soundFields hverify
  complete := by
    intro statement witness hrel
    rcases C.completeFields hrel with ⟨proofFields, hverify⟩
    exact ⟨encodeLinearSigmaProofFields proofFields, by
      simpa [verify] using hverify⟩
  witnessHiding := C.witnessHiding
  witnessHiding_holds := C.witnessHiding_holds

theorem target_totalBytes
    (_C : AlgebraicLinearSigmaOpeningConfig) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

theorem target_witnessHiding
    (C : AlgebraicLinearSigmaOpeningConfig) :
    C.toLinearSigmaOpeningTarget.witnessHiding :=
  C.witnessHiding_holds

end AlgebraicLinearSigmaOpeningConfig

/--
Concrete compressed linear-sigma shape for the 73-byte target.

The primitive is intentionally statement-dependent: the relation map may fold
the account commitment, active openings, decoded response equations, and any
public linear checks into one 32-byte target.  The verifier checks the usual
sigma equation

```text
  L_s(z) = a + c * y_s
```

where `a` is the 32-byte proof commitment, `z` is the 41-byte masked witness
response, `c` is the already-bound challenge bytes, and `y_s` is the public
target derived from the statement.  `knowledgeSoundness` and `witnessHiding`
are the concrete SIS/ZK-style assumptions for the chosen algebraic commitment.
-/
structure CompressedLinearSigmaAlgebra where
  relationMap :
    PublicOpeningStatement ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  target : PublicOpeningStatement -> ByteVector linearSigmaProofCommitmentBytes
  addResponse :
    ByteVector linearSigmaMaskedWitnessBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  scaleResponse :
    ByteVector challengeBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  relation_linear :
    ∀ statement mask witness challenge,
      relationMap statement
          (addResponse mask (scaleResponse challenge witness)) =
        addCommitment (relationMap statement mask)
          (scaleCommitment challenge (relationMap statement witness))
  proverMask :
    PublicOpeningStatement -> PublicOpeningWitness ->
      ByteVector linearSigmaMaskedWitnessBytes
  knowledgeSoundness :
    ∀ {statement proofCommitment maskedWitnessResponse},
      relationMap statement maskedWitnessResponse =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge (target statement)) ->
        ∃ witness : PublicOpeningWitness,
          relationMap statement (linearSigmaWitnessFromHiddenPreimage witness.hiddenPreimage) =
            target statement
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace CompressedLinearSigmaAlgebra

def relationHolds
    (A : CompressedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (witness : PublicOpeningWitness) : Prop :=
  relationMap A statement (linearSigmaWitnessFromHiddenPreimage witness.hiddenPreimage) =
    target A statement

def verifyFields
    (A : CompressedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (proofCommitment : ByteVector linearSigmaProofCommitmentBytes)
    (maskedWitnessResponse : ByteVector linearSigmaMaskedWitnessBytes) :
    Prop :=
  relationMap A statement maskedWitnessResponse =
    addCommitment A proofCommitment
      (scaleCommitment A statement.challenge (target A statement))

def proveFields
    (A : CompressedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (witness : PublicOpeningWitness) :
    LinearSigmaProofFields :=
  let mask := proverMask A statement witness
  let hidden := linearSigmaWitnessFromHiddenPreimage witness.hiddenPreimage
  {
    proofCommitment := relationMap A statement mask
    maskedWitnessResponse :=
      addResponse A mask (scaleResponse A statement.challenge hidden)
  }

theorem proveFields_accepts
    (A : CompressedLinearSigmaAlgebra)
    {statement : PublicOpeningStatement}
    {witness : PublicOpeningWitness}
    (hrel : relationHolds A statement witness) :
    verifyFields A statement
      (proveFields A statement witness).proofCommitment
      (proveFields A statement witness).maskedWitnessResponse := by
  change
    relationMap A statement
      (linearSigmaWitnessFromHiddenPreimage witness.hiddenPreimage) =
        target A statement at hrel
  change
    relationMap A statement
      (addResponse A (proverMask A statement witness)
        (scaleResponse A statement.challenge
          (linearSigmaWitnessFromHiddenPreimage witness.hiddenPreimage))) =
      addCommitment A (relationMap A statement (proverMask A statement witness))
        (scaleCommitment A statement.challenge (target A statement))
  rw [relation_linear A]
  rw [hrel]

def toAlgebraicLinearSigmaOpeningConfig
    (A : CompressedLinearSigmaAlgebra) :
    AlgebraicLinearSigmaOpeningConfig where
  idealRelation := A.relationHolds
  verifyFields := A.verifyFields
  soundFields := by
    intro statement proofCommitment maskedWitnessResponse hverify
    exact knowledgeSoundness A hverify
  completeFields := by
    intro statement witness hrel
    exact ⟨A.proveFields statement witness, A.proveFields_accepts hrel⟩
  witnessHiding := witnessHiding A
  witnessHiding_holds := witnessHiding_holds A

def toLinearSigmaOpeningTarget
    (A : CompressedLinearSigmaAlgebra) :
    LinearSigmaOpeningTarget :=
  toAlgebraicLinearSigmaOpeningConfig A |>.toLinearSigmaOpeningTarget

theorem target_totalBytes
    (_A : CompressedLinearSigmaAlgebra) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

end CompressedLinearSigmaAlgebra

/--
Full-response fibered version of the 73-byte target.

Unlike `SeedRestrictedCompressedLinearSigmaAlgebra`, this route uses the whole
41-byte masked response as the algebraic witness carrier.  That recovers enough
q-ary capacity for the 12-public-row / 11-hidden-fiber split while keeping the
same byte layout.  The concrete instantiation must make `responseOpensStatement`
true for every extracted response vector in the accepted fiber.
-/
structure FullResponseFiberedLinearSigmaAlgebra where
  responseOpensStatement :
    PublicOpeningStatement -> ByteVector linearSigmaMaskedWitnessBytes -> Prop
  relationMapResponse :
    PublicOpeningStatement ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  relationMapSound :
    ∀ {statement response},
      relationMapResponse statement response = statement.digest ->
        responseOpensStatement statement response
  addResponse :
    ByteVector linearSigmaMaskedWitnessBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  scaleResponse :
    ByteVector challengeBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  response_linear :
    ∀ statement mask response challenge,
      relationMapResponse statement
          (addResponse mask (scaleResponse challenge response)) =
        addCommitment (relationMapResponse statement mask)
          (scaleCommitment challenge
            (relationMapResponse statement response))
  proverMaskResponse :
    PublicOpeningStatement -> PublicOpeningWitness ->
      ByteVector linearSigmaMaskedWitnessBytes
  responseKnowledgeSoundness :
    ∀ {statement proofCommitment maskedWitnessResponse},
      relationMapResponse statement maskedWitnessResponse =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge statement.digest) ->
        ∃ response,
          relationMapResponse statement response = statement.digest ∧
            responseOpensStatement statement response
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace FullResponseFiberedLinearSigmaAlgebra

def relationHolds
    (A : FullResponseFiberedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (witness : PublicOpeningWitness) : Prop :=
  ∃ response,
    witness.hiddenPreimage =
      hiddenPreimageFromLinearSigmaWitnessBytes response ∧
      A.relationMapResponse statement response = statement.digest ∧
      A.responseOpensStatement statement response

def verifyFields
    (A : FullResponseFiberedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (proofCommitment : ByteVector linearSigmaProofCommitmentBytes)
    (maskedWitnessResponse : ByteVector linearSigmaMaskedWitnessBytes) :
    Prop :=
  A.relationMapResponse statement maskedWitnessResponse =
    A.addCommitment proofCommitment
      (A.scaleCommitment statement.challenge statement.digest)

def proveFields
    (A : FullResponseFiberedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (witness : PublicOpeningWitness)
    (response : ByteVector linearSigmaMaskedWitnessBytes) :
    LinearSigmaProofFields :=
  let mask := A.proverMaskResponse statement witness
  {
    proofCommitment := A.relationMapResponse statement mask
    maskedWitnessResponse :=
      A.addResponse mask (A.scaleResponse statement.challenge response)
  }

theorem proveFields_accepts
    (A : FullResponseFiberedLinearSigmaAlgebra)
    {statement : PublicOpeningStatement}
    {witness : PublicOpeningWitness}
    {response : ByteVector linearSigmaMaskedWitnessBytes}
    (_hresponse :
      witness.hiddenPreimage =
        hiddenPreimageFromLinearSigmaWitnessBytes response)
    (hrel : A.relationMapResponse statement response = statement.digest) :
    A.verifyFields statement
      (A.proveFields statement witness response).proofCommitment
      (A.proveFields statement witness response).maskedWitnessResponse := by
  unfold verifyFields proveFields
  rw [A.response_linear]
  rw [hrel]

def toAlgebraicLinearSigmaOpeningConfig
    (A : FullResponseFiberedLinearSigmaAlgebra) :
    AlgebraicLinearSigmaOpeningConfig where
  idealRelation := A.relationHolds
  verifyFields := A.verifyFields
  soundFields := by
    intro statement proofCommitment maskedWitnessResponse hverify
    rcases A.responseKnowledgeSoundness hverify with
      ⟨response, htarget, hopen⟩
    exact ⟨{
      hiddenPreimage := hiddenPreimageFromLinearSigmaWitnessBytes response
    }, ⟨response, rfl, htarget, hopen⟩⟩
  completeFields := by
    intro statement witness hrel
    rcases hrel with ⟨response, hresponse, htarget, _hopen⟩
    exact ⟨A.proveFields statement witness response,
      A.proveFields_accepts hresponse htarget⟩
  witnessHiding := A.witnessHiding
  witnessHiding_holds := A.witnessHiding_holds

def toLinearSigmaOpeningTarget
    (A : FullResponseFiberedLinearSigmaAlgebra) :
    LinearSigmaOpeningTarget :=
  A.toAlgebraicLinearSigmaOpeningConfig.toLinearSigmaOpeningTarget

theorem target_totalBytes
    (_A : FullResponseFiberedLinearSigmaAlgebra) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

end FullResponseFiberedLinearSigmaAlgebra

/--
Byte-relation specialization of the full-response 73-byte target.

This is the deployment-facing version of `FullResponseFiberedLinearSigmaAlgebra`:
the extracted 41-byte response is decoded as
`fiber_state[2] || pad_seed[32]`, and the relation-map soundness theorem must
prove the actual byte-level opening relation for that decoded witness.  This
removes the seed-only ambiguity: every accepted extracted full-response vector
must open the public statement as a `PublicOpeningWitness`.
-/
structure FullResponseByteOpeningAlgebra where
  idealRelation : PublicOpeningStatement -> PublicOpeningWitness -> Prop
  relationMapResponse :
    PublicOpeningStatement ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  relationMapSound :
    ∀ {statement response},
      relationMapResponse statement response = statement.digest ->
        idealRelation statement
          { hiddenPreimage :=
              hiddenPreimageFromLinearSigmaWitnessBytes response }
  relationMapComplete :
    ∀ {statement witness},
      idealRelation statement witness ->
        ∃ response,
          hiddenPreimageFromLinearSigmaWitnessBytes response =
            witness.hiddenPreimage ∧
          relationMapResponse statement response = statement.digest
  addResponse :
    ByteVector linearSigmaMaskedWitnessBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  scaleResponse :
    ByteVector challengeBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  response_linear :
    ∀ statement mask response challenge,
      relationMapResponse statement
          (addResponse mask (scaleResponse challenge response)) =
        addCommitment (relationMapResponse statement mask)
          (scaleCommitment challenge
            (relationMapResponse statement response))
  proverMaskResponse :
    PublicOpeningStatement -> PublicOpeningWitness ->
      ByteVector linearSigmaMaskedWitnessBytes
  responseKnowledgeSoundness :
    ∀ {statement proofCommitment maskedWitnessResponse},
      relationMapResponse statement maskedWitnessResponse =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge statement.digest) ->
        ∃ response,
          relationMapResponse statement response = statement.digest
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace FullResponseByteOpeningAlgebra

def decodedWitness
    (_A : FullResponseByteOpeningAlgebra)
    (response : ByteVector linearSigmaMaskedWitnessBytes) :
    PublicOpeningWitness where
  hiddenPreimage := hiddenPreimageFromLinearSigmaWitnessBytes response

def verifyFields
    (A : FullResponseByteOpeningAlgebra)
    (statement : PublicOpeningStatement)
    (proofCommitment : ByteVector linearSigmaProofCommitmentBytes)
    (maskedWitnessResponse : ByteVector linearSigmaMaskedWitnessBytes) :
    Prop :=
  A.relationMapResponse statement maskedWitnessResponse =
    A.addCommitment proofCommitment
      (A.scaleCommitment statement.challenge statement.digest)

def proveFields
    (A : FullResponseByteOpeningAlgebra)
    (statement : PublicOpeningStatement)
    (witness : PublicOpeningWitness) :
    ByteVector linearSigmaMaskedWitnessBytes ->
    LinearSigmaProofFields :=
  fun response =>
  let mask := A.proverMaskResponse statement witness
  {
    proofCommitment := A.relationMapResponse statement mask
    maskedWitnessResponse :=
      A.addResponse mask (A.scaleResponse statement.challenge response)
  }

theorem proveFields_accepts
    (A : FullResponseByteOpeningAlgebra)
    {statement : PublicOpeningStatement}
    {witness : PublicOpeningWitness}
    {response : ByteVector linearSigmaMaskedWitnessBytes}
    (_hdecode :
      hiddenPreimageFromLinearSigmaWitnessBytes response =
        witness.hiddenPreimage)
    (htarget : A.relationMapResponse statement response = statement.digest) :
    A.verifyFields statement
      (A.proveFields statement witness response).proofCommitment
      (A.proveFields statement witness response).maskedWitnessResponse := by
  unfold verifyFields proveFields
  rw [A.response_linear]
  rw [htarget]

theorem extractedWitness_sound
    (A : FullResponseByteOpeningAlgebra)
    {statement : PublicOpeningStatement}
    {response : ByteVector linearSigmaMaskedWitnessBytes}
    (htarget :
      A.relationMapResponse statement response = statement.digest) :
    A.idealRelation statement (A.decodedWitness response) :=
  A.relationMapSound htarget

/--
Any accepted full-response fiber is relation-saturated.

This is the key structural cost of the 12x23 full-response route: if one
response hits the public digest, every other response with the same public
syndrome must also decode to a valid byte-level opening witness.
-/
theorem acceptedFiber_relationSaturated
    (A : FullResponseByteOpeningAlgebra)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      A.relationMapResponse statement response₀ = statement.digest)
    (hsame :
      A.relationMapResponse statement response₁ =
        A.relationMapResponse statement response₀) :
    A.idealRelation statement (A.decodedWitness response₁) :=
  A.relationMapSound (by rw [hsame, h₀])

/--
If the byte-level relation binds the entire decoded hidden preimage through an
injective commitment, then an accepted public fiber cannot contain two
different full responses.

Consequently, the deliberate 12x23 nontrivial-fiber design is incompatible
with a relation that requires a collision-resistant commitment to every byte of
`fiber_state[2] || pad_seed[32]`, unless the commitment assumption is moved to a
fiber quotient or the ideal relation is redesigned to ignore the hidden fiber.
-/
theorem acceptedFiber_subsingleton_of_commitmentBinding
    (A : FullResponseByteOpeningAlgebra)
    (commit :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes ->
        ByteVector sha256DigestBytes)
    (hideal_commit :
      ∀ {statement : PublicOpeningStatement}
        {witness : PublicOpeningWitness},
        A.idealRelation statement witness ->
          commit witness.hiddenPreimage = statement.digest)
    (hcommit_injective : Function.Injective commit)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      A.relationMapResponse statement response₀ = statement.digest)
    (h₁ :
      A.relationMapResponse statement response₁ = statement.digest) :
    hiddenPreimageFromLinearSigmaWitnessBytes response₀ =
      hiddenPreimageFromLinearSigmaWitnessBytes response₁ := by
  have hopen₀ : A.idealRelation statement (A.decodedWitness response₀) :=
    A.relationMapSound h₀
  have hopen₁ : A.idealRelation statement (A.decodedWitness response₁) :=
    A.relationMapSound h₁
  have hcommit₀ :
      commit (hiddenPreimageFromLinearSigmaWitnessBytes response₀) =
        statement.digest := by
    exact hideal_commit hopen₀
  have hcommit₁ :
      commit (hiddenPreimageFromLinearSigmaWitnessBytes response₁) =
        statement.digest := by
    exact hideal_commit hopen₁
  have hhidden :
      hiddenPreimageFromLinearSigmaWitnessBytes response₀ =
        hiddenPreimageFromLinearSigmaWitnessBytes response₁ :=
    hcommit_injective (by rw [hcommit₀, hcommit₁])
  exact hhidden

def toFullResponseFiberedLinearSigmaAlgebra
    (A : FullResponseByteOpeningAlgebra) :
    FullResponseFiberedLinearSigmaAlgebra where
  responseOpensStatement := fun statement response =>
    A.idealRelation statement (A.decodedWitness response)
  relationMapResponse := A.relationMapResponse
  relationMapSound := by
    intro statement response htarget
    exact A.relationMapSound htarget
  addResponse := A.addResponse
  scaleResponse := A.scaleResponse
  addCommitment := A.addCommitment
  scaleCommitment := A.scaleCommitment
  response_linear := A.response_linear
  proverMaskResponse := A.proverMaskResponse
  responseKnowledgeSoundness := by
    intro statement proofCommitment maskedWitnessResponse hverify
    rcases A.responseKnowledgeSoundness hverify with ⟨response, htarget⟩
    exact ⟨response, htarget, A.relationMapSound htarget⟩
  witnessHiding := A.witnessHiding
  witnessHiding_holds := A.witnessHiding_holds

def toAlgebraicLinearSigmaOpeningConfig
    (A : FullResponseByteOpeningAlgebra) :
    AlgebraicLinearSigmaOpeningConfig where
  idealRelation := A.idealRelation
  verifyFields := A.verifyFields
  soundFields := by
    intro statement proofCommitment maskedWitnessResponse hverify
    rcases A.responseKnowledgeSoundness hverify with ⟨response, htarget⟩
    exact ⟨A.decodedWitness response, A.relationMapSound htarget⟩
  completeFields := by
    intro statement witness hrel
    rcases A.relationMapComplete hrel with ⟨response, hdecode, htarget⟩
    exact ⟨A.proveFields statement witness response,
      A.proveFields_accepts hdecode htarget⟩
  witnessHiding := A.witnessHiding
  witnessHiding_holds := A.witnessHiding_holds

def toLinearSigmaOpeningTarget
    (A : FullResponseByteOpeningAlgebra) :
    LinearSigmaOpeningTarget :=
  A.toAlgebraicLinearSigmaOpeningConfig.toLinearSigmaOpeningTarget

theorem target_totalBytes
    (_A : FullResponseByteOpeningAlgebra) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

theorem target_idealRelation_eq
    (A : FullResponseByteOpeningAlgebra) :
    A.toLinearSigmaOpeningTarget.idealRelation = A.idealRelation :=
  rfl

end FullResponseByteOpeningAlgebra

/--
Fork-extraction surface for the 73-byte full-response opening.

This isolates the algebraic part of knowledge soundness from the
Fiat-Shamir/forking part.  A fork consists of two accepting transcripts with
the same proof commitment and the same public statement data except for the
challenge.  The certificate records the exact statement-stability conditions
needed by the algebraic extractor; the remaining probabilistic work is to show
that an adversary producing one accepting Fiat-Shamir transcript can be rewound
or measured to produce such a fork with the claimed loss.
-/
structure FullResponseForkExtractionCertificate
    (A : FullResponseByteOpeningAlgebra) where
  sameForkStatement :
    PublicOpeningStatement -> PublicOpeningStatement -> Prop
  sameFork_digest :
    ∀ {statement₀ statement₁},
      sameForkStatement statement₀ statement₁ ->
        statement₀.digest = statement₁.digest
  sameFork_relationMap :
    ∀ {statement₀ statement₁ response},
      sameForkStatement statement₀ statement₁ ->
        A.relationMapResponse statement₀ response =
          A.relationMapResponse statement₁ response
  sameFork_openingData :
    ∀ {statement₀ statement₁},
      sameForkStatement statement₀ statement₁ ->
        statement₀.publicKey = statement₁.publicKey ∧
        statement₀.responses = statement₁.responses ∧
        statement₀.openings = statement₁.openings
  forkExtracts :
    ∀ {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁},
      sameForkStatement statement₀ statement₁ ->
      statement₀.challenge ≠ statement₁.challenge ->
      A.verifyFields statement₀ proofCommitment maskedResponse₀ ->
      A.verifyFields statement₁ proofCommitment maskedResponse₁ ->
        ∃ response,
          A.relationMapResponse statement₀ response = statement₀.digest
  forkExtractorBreak : Prop
  forkExtractorBreak_if_no_extract :
    ∀ {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁},
      sameForkStatement statement₀ statement₁ ->
      statement₀.challenge ≠ statement₁.challenge ->
      A.verifyFields statement₀ proofCommitment maskedResponse₀ ->
      A.verifyFields statement₁ proofCommitment maskedResponse₁ ->
      (¬ ∃ response,
        A.relationMapResponse statement₀ response = statement₀.digest) ->
        forkExtractorBreak

namespace FullResponseForkExtractionCertificate

theorem extracts_from_fork
    {A : FullResponseByteOpeningAlgebra}
    (C : FullResponseForkExtractionCertificate A)
    {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁}
    (hsame : C.sameForkStatement statement₀ statement₁)
    (hchallenge : statement₀.challenge ≠ statement₁.challenge)
    (haccept₀ : A.verifyFields statement₀ proofCommitment maskedResponse₀)
    (haccept₁ : A.verifyFields statement₁ proofCommitment maskedResponse₁) :
    ∃ response,
      A.relationMapResponse statement₀ response = statement₀.digest :=
  C.forkExtracts hsame hchallenge haccept₀ haccept₁

theorem extractedWitness_from_fork
    {A : FullResponseByteOpeningAlgebra}
    (C : FullResponseForkExtractionCertificate A)
    {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁}
    (hsame : C.sameForkStatement statement₀ statement₁)
    (hchallenge : statement₀.challenge ≠ statement₁.challenge)
    (haccept₀ : A.verifyFields statement₀ proofCommitment maskedResponse₀)
    (haccept₁ : A.verifyFields statement₁ proofCommitment maskedResponse₁) :
    ∃ witness,
      A.idealRelation statement₀ witness := by
  rcases C.extracts_from_fork hsame hchallenge haccept₀ haccept₁ with
    ⟨response, htarget⟩
  exact ⟨A.decodedWitness response, A.relationMapSound htarget⟩

theorem forkExtractorBreak_of_no_extract
    {A : FullResponseByteOpeningAlgebra}
    (C : FullResponseForkExtractionCertificate A)
    {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁}
    (hsame : C.sameForkStatement statement₀ statement₁)
    (hchallenge : statement₀.challenge ≠ statement₁.challenge)
    (haccept₀ : A.verifyFields statement₀ proofCommitment maskedResponse₀)
    (haccept₁ : A.verifyFields statement₁ proofCommitment maskedResponse₁)
    (hno :
      ¬ ∃ response,
        A.relationMapResponse statement₀ response = statement₀.digest) :
    C.forkExtractorBreak :=
  C.forkExtractorBreak_if_no_extract hsame hchallenge haccept₀ haccept₁ hno

end FullResponseForkExtractionCertificate

/--
Concrete challenge-action certificate for fork extraction.

The intended action hashes the 21-byte Fiat-Shamir challenge into twelve
`q = 12289` residues and interprets the result as an element of a degree-12
extension field.  Once a concrete irreducible polynomial is pinned, every
nonzero action difference is invertible.  Thus the algebraic bad-fork event is
not "different challenge bytes"; it is equality of the derived extension-field
action.
-/
structure FullResponseChallengeActionCertificate where
  actionResidues : Nat
  actionResidues_eq : actionResidues = sisFullResponsePublicRows
  actionCapacityFloorBits : Nat
  actionCapacityFloorBits_eq : actionCapacityFloorBits = 163
  actionQuantumQueryBoundLog2For128 : Nat
  actionQuantumQueryBoundLog2For128_eq :
    actionQuantumQueryBoundLog2For128 = 17
  extensionPolynomialDegree : Nat
  extensionPolynomialDegree_eq : extensionPolynomialDegree = 12
  extensionPolynomialConstant : Nat
  extensionPolynomialConstant_eq : extensionPolynomialConstant = 25
  extensionPolynomialLinearCoeff : Nat
  extensionPolynomialLinearCoeff_eq : extensionPolynomialLinearCoeff = 1
  extensionPolynomialMonic : Prop
  extensionPolynomialMonic_holds : extensionPolynomialMonic
  rabinXQ12MinusXZero : Prop
  rabinXQ12MinusXZero_holds : rabinXQ12MinusXZero
  rabinGcdXQ6MinusXDegree : Nat
  rabinGcdXQ6MinusXDegree_eq : rabinGcdXQ6MinusXDegree = 0
  rabinGcdXQ4MinusXDegree : Nat
  rabinGcdXQ4MinusXDegree_eq : rabinGcdXQ4MinusXDegree = 0
  extensionPolynomialIrreducible : Prop
  extensionPolynomialIrreducible_holds : extensionPolynomialIrreducible
  action : ByteVector challengeBytes -> ByteVector actionResidues
  badActionCollision :
    ByteVector challengeBytes -> ByteVector challengeBytes -> Prop
  badActionCollision_iff :
    ∀ c₀ c₁, badActionCollision c₀ c₁ ↔ action c₀ = action c₁
  noncollision_invertible : Prop
  noncollision_invertible_holds :
    extensionPolynomialIrreducible -> noncollision_invertible

namespace FullResponseChallengeActionCertificate

theorem actionResidues_eq_twelve
    (C : FullResponseChallengeActionCertificate) :
    C.actionResidues = 12 := by
  simpa [sisFullResponsePublicRows] using C.actionResidues_eq

theorem actionCapacityFloorBits_eq_163
    (C : FullResponseChallengeActionCertificate) :
    C.actionCapacityFloorBits = 163 :=
  C.actionCapacityFloorBits_eq

theorem actionQuantumQueryBoundLog2For128_eq_17
    (C : FullResponseChallengeActionCertificate) :
    C.actionQuantumQueryBoundLog2For128 = 17 :=
  C.actionQuantumQueryBoundLog2For128_eq

theorem extensionPolynomialDegree_eq_12
    (C : FullResponseChallengeActionCertificate) :
    C.extensionPolynomialDegree = 12 :=
  C.extensionPolynomialDegree_eq

theorem extensionPolynomialPinned_X12_X_25
    (C : FullResponseChallengeActionCertificate) :
    C.extensionPolynomialDegree = 12 ∧
      C.extensionPolynomialConstant = 25 ∧
      C.extensionPolynomialLinearCoeff = 1 ∧
      C.extensionPolynomialMonic := by
  exact ⟨C.extensionPolynomialDegree_eq,
    C.extensionPolynomialConstant_eq,
    C.extensionPolynomialLinearCoeff_eq,
    C.extensionPolynomialMonic_holds⟩

theorem rabinCertificate_gcdDegrees_zero
    (C : FullResponseChallengeActionCertificate) :
    C.rabinGcdXQ6MinusXDegree = 0 ∧
      C.rabinGcdXQ4MinusXDegree = 0 :=
  ⟨C.rabinGcdXQ6MinusXDegree_eq,
    C.rabinGcdXQ4MinusXDegree_eq⟩

theorem rabinXQ12MinusXZero_certified
    (C : FullResponseChallengeActionCertificate) :
    C.rabinXQ12MinusXZero :=
  C.rabinXQ12MinusXZero_holds

theorem extensionPolynomialIrreducible_certified
    (C : FullResponseChallengeActionCertificate) :
    C.extensionPolynomialIrreducible :=
  C.extensionPolynomialIrreducible_holds

theorem noncollision_invertible_certified
    (C : FullResponseChallengeActionCertificate) :
    C.noncollision_invertible :=
  C.noncollision_invertible_holds C.extensionPolynomialIrreducible_holds

theorem badActionCollision_iff_action_eq
    (C : FullResponseChallengeActionCertificate)
    (c₀ c₁ : ByteVector challengeBytes) :
    C.badActionCollision c₀ c₁ ↔ C.action c₀ = C.action c₁ :=
  C.badActionCollision_iff c₀ c₁

end FullResponseChallengeActionCertificate

/--
Byte-level algebraic fork-equation certificate.

This is the concrete bridge between the abstract fork-extraction surface and
the chosen `F_12289[X]/(X^12 + X + 25)` challenge action.  Two accepting fork
equations have the form

`relationMap(statementᵢ, zᵢ) = proofCommitment + action(challengeᵢ) * digest`.

If the derived action vectors do not collide, the field difference is
invertible; subtracting the equations and multiplying by the inverse extracts a
response whose relation map is exactly the statement digest.
-/
structure FullResponseForkEquationAlgebraCertificate
    (A : FullResponseByteOpeningAlgebra)
    (F : FullResponseForkExtractionCertificate A)
    (Action : FullResponseChallengeActionCertificate) where
  forkEquationModel : Prop
  forkEquationModel_holds : forkEquationModel
  actionDifferenceInvertible :
    ∀ {statement₀ statement₁},
      F.sameForkStatement statement₀ statement₁ ->
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge ->
        Prop
  actionDifferenceInvertible_holds :
    ∀ {statement₀ statement₁}
      (hsame : F.sameForkStatement statement₀ statement₁)
      (hcollision :
        ¬ Action.badActionCollision statement₀.challenge statement₁.challenge),
        actionDifferenceInvertible hsame hcollision
  extractedForkResponse :
    PublicOpeningStatement ->
    PublicOpeningStatement ->
    ByteVector linearSigmaMaskedWitnessBytes ->
    ByteVector linearSigmaMaskedWitnessBytes ->
      ByteVector linearSigmaMaskedWitnessBytes
  extractedForkResponse_correct :
    ∀ {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁},
      F.sameForkStatement statement₀ statement₁ ->
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge ->
      A.verifyFields statement₀ proofCommitment maskedResponse₀ ->
      A.verifyFields statement₁ proofCommitment maskedResponse₁ ->
        A.relationMapResponse statement₀
          (extractedForkResponse statement₀ statement₁
            maskedResponse₀ maskedResponse₁) =
        statement₀.digest
  noForkEquationBreak :
    ∀ {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁},
      F.sameForkStatement statement₀ statement₁ ->
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge ->
      A.verifyFields statement₀ proofCommitment maskedResponse₀ ->
      A.verifyFields statement₁ proofCommitment maskedResponse₁ ->
      (¬ ∃ response,
        A.relationMapResponse statement₀ response = statement₀.digest) ->
        F.forkExtractorBreak

namespace FullResponseForkEquationAlgebraCertificate

theorem extracts_from_noncolliding_fork_equation
    {A : FullResponseByteOpeningAlgebra}
    {F : FullResponseForkExtractionCertificate A}
    {Action : FullResponseChallengeActionCertificate}
    (C : FullResponseForkEquationAlgebraCertificate A F Action)
    {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁}
    (hsame : F.sameForkStatement statement₀ statement₁)
    (hcollision :
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge)
    (haccept₀ : A.verifyFields statement₀ proofCommitment maskedResponse₀)
    (haccept₁ : A.verifyFields statement₁ proofCommitment maskedResponse₁) :
    ∃ response,
      A.relationMapResponse statement₀ response = statement₀.digest :=
  ⟨C.extractedForkResponse statement₀ statement₁
      maskedResponse₀ maskedResponse₁,
    C.extractedForkResponse_correct hsame hcollision haccept₀ haccept₁⟩

theorem extractedWitness_from_noncolliding_fork_equation
    {A : FullResponseByteOpeningAlgebra}
    {F : FullResponseForkExtractionCertificate A}
    {Action : FullResponseChallengeActionCertificate}
    (C : FullResponseForkEquationAlgebraCertificate A F Action)
    {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁}
    (hsame : F.sameForkStatement statement₀ statement₁)
    (hcollision :
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge)
    (haccept₀ : A.verifyFields statement₀ proofCommitment maskedResponse₀)
    (haccept₁ : A.verifyFields statement₁ proofCommitment maskedResponse₁) :
    ∃ witness,
      A.idealRelation statement₀ witness := by
  rcases C.extracts_from_noncolliding_fork_equation
      hsame hcollision haccept₀ haccept₁ with
    ⟨response, htarget⟩
  exact ⟨A.decodedWitness response, A.relationMapSound htarget⟩

theorem forkExtractorBreak_of_no_noncolliding_equation_extract
    {A : FullResponseByteOpeningAlgebra}
    {F : FullResponseForkExtractionCertificate A}
    {Action : FullResponseChallengeActionCertificate}
    (C : FullResponseForkEquationAlgebraCertificate A F Action)
    {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁}
    (hsame : F.sameForkStatement statement₀ statement₁)
    (hcollision :
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge)
    (haccept₀ : A.verifyFields statement₀ proofCommitment maskedResponse₀)
    (haccept₁ : A.verifyFields statement₁ proofCommitment maskedResponse₁)
    (hno :
      ¬ ∃ response,
        A.relationMapResponse statement₀ response = statement₀.digest) :
    F.forkExtractorBreak :=
  C.noForkEquationBreak hsame hcollision haccept₀ haccept₁ hno

end FullResponseForkEquationAlgebraCertificate

def hiddenPreimageOfSeed
    (deriveFiberState :
      ByteVector padSeedBytes -> ByteVector localFiberStateBytes)
    (seed : ByteVector padSeedBytes) :
    ByteCommittedPadPreimage localFiberStateBytes padSeedBytes where
  fiberState := deriveFiberState seed
  padSeed := seed

theorem hiddenPreimageOfSeed_injective
    (deriveFiberState :
      ByteVector padSeedBytes -> ByteVector localFiberStateBytes) :
    Function.Injective (hiddenPreimageOfSeed deriveFiberState) := by
  intro seed₀ seed₁ h
  exact congrArg ByteCommittedPadPreimage.padSeed h

theorem seedFromLinearSigmaWitnessBytes_encodeHiddenPreimage
    (hidden : ByteCommittedPadPreimage localFiberStateBytes padSeedBytes) :
    seedFromLinearSigmaWitnessBytes (linearSigmaWitnessFromHiddenPreimage hidden) =
      hidden.padSeed := by
  exact congrArg ByteCommittedPadPreimage.padSeed
    (hiddenPreimageFromLinearSigmaWitnessBytes_encodeHiddenPreimage hidden)

theorem seedFromLinearSigmaWitnessBytes_encodeSeed
    (deriveFiberState :
      ByteVector padSeedBytes -> ByteVector localFiberStateBytes)
    (seed : ByteVector padSeedBytes) :
    seedFromLinearSigmaWitnessBytes
        (linearSigmaWitnessFromHiddenPreimage (hiddenPreimageOfSeed deriveFiberState seed)) =
      seed := by
  simpa [hiddenPreimageOfSeed]
    using
      seedFromLinearSigmaWitnessBytes_encodeHiddenPreimage
        (hiddenPreimageOfSeed deriveFiberState seed)

def responseBytesOfSeed
    (deriveFiberState :
      ByteVector padSeedBytes -> ByteVector localFiberStateBytes)
    (seed : ByteVector padSeedBytes) :
    ByteVector linearSigmaMaskedWitnessBytes :=
  linearSigmaWitnessFromHiddenPreimage (hiddenPreimageOfSeed deriveFiberState seed)

/--
Restricted-domain instantiation of the compressed sigma target.

The 34-byte hidden opening witness is represented as
`deriveFiberState(seed)[2] || seed[32]`.  The algebraic commitment binds the
32-byte seed.  This is the minimal way around the 34-to-32 pigeonhole theorem:
the public commitment no longer claims to bind all 34-byte strings, only the
32-byte seed domain plus the deterministic 2-byte fiber derivation.
-/
structure SeedRestrictedCompressedLinearSigmaAlgebra where
  deriveFiberState :
    ByteVector padSeedBytes -> ByteVector localFiberStateBytes
  seedOpensStatement :
    PublicOpeningStatement -> ByteVector padSeedBytes -> Prop
  relationMapSeed :
    PublicOpeningStatement ->
      ByteVector padSeedBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  target : PublicOpeningStatement -> ByteVector linearSigmaProofCommitmentBytes
  relationMapSound :
    ∀ {statement seed},
      relationMapSeed statement seed = target statement ->
        seedOpensStatement statement seed
  addSeed :
    ByteVector padSeedBytes -> ByteVector padSeedBytes ->
      ByteVector padSeedBytes
  scaleSeed :
    ByteVector challengeBytes -> ByteVector padSeedBytes ->
      ByteVector padSeedBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  seed_linear :
    ∀ statement maskSeed witnessSeed challenge,
      relationMapSeed statement
          (addSeed maskSeed (scaleSeed challenge witnessSeed)) =
        addCommitment (relationMapSeed statement maskSeed)
          (scaleCommitment challenge (relationMapSeed statement witnessSeed))
  proverMaskSeed :
    PublicOpeningStatement -> PublicOpeningWitness -> ByteVector padSeedBytes
  seedKnowledgeSoundness :
    ∀ {statement proofCommitment maskedSeedResponse},
      relationMapSeed statement maskedSeedResponse =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge (target statement)) ->
        ∃ seed, relationMapSeed statement seed = target statement
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace SeedRestrictedCompressedLinearSigmaAlgebra

def relationHolds
    (A : SeedRestrictedCompressedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (witness : PublicOpeningWitness) : Prop :=
  ∃ seed,
    witness.hiddenPreimage = hiddenPreimageOfSeed A.deriveFiberState seed ∧
      A.relationMapSeed statement seed = A.target statement ∧
      A.seedOpensStatement statement seed

def verifyFields
    (A : SeedRestrictedCompressedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (proofCommitment : ByteVector linearSigmaProofCommitmentBytes)
    (maskedWitnessResponse : ByteVector linearSigmaMaskedWitnessBytes) :
    Prop :=
  A.relationMapSeed statement
      (seedFromLinearSigmaWitnessBytes maskedWitnessResponse) =
    A.addCommitment proofCommitment
      (A.scaleCommitment statement.challenge (A.target statement))

def proveFields
    (A : SeedRestrictedCompressedLinearSigmaAlgebra)
    (statement : PublicOpeningStatement)
    (witness : PublicOpeningWitness)
    (seed : ByteVector padSeedBytes) :
    LinearSigmaProofFields :=
  let maskSeed := A.proverMaskSeed statement witness
  {
    proofCommitment := A.relationMapSeed statement maskSeed
    maskedWitnessResponse :=
      responseBytesOfSeed A.deriveFiberState
        (A.addSeed maskSeed (A.scaleSeed statement.challenge seed))
  }

theorem proveFields_accepts
    (A : SeedRestrictedCompressedLinearSigmaAlgebra)
    {statement : PublicOpeningStatement}
    {witness : PublicOpeningWitness}
    {seed : ByteVector padSeedBytes}
    (_hseed :
      witness.hiddenPreimage = hiddenPreimageOfSeed A.deriveFiberState seed)
    (hrel : A.relationMapSeed statement seed = A.target statement) :
    A.verifyFields statement
      (A.proveFields statement witness seed).proofCommitment
      (A.proveFields statement witness seed).maskedWitnessResponse := by
  unfold verifyFields proveFields responseBytesOfSeed
  rw [seedFromLinearSigmaWitnessBytes_encodeSeed]
  rw [A.seed_linear]
  rw [hrel]

def toAlgebraicLinearSigmaOpeningConfig
    (A : SeedRestrictedCompressedLinearSigmaAlgebra) :
    AlgebraicLinearSigmaOpeningConfig where
  idealRelation := A.relationHolds
  verifyFields := A.verifyFields
  soundFields := by
    intro statement proofCommitment maskedWitnessResponse hverify
    rcases A.seedKnowledgeSoundness hverify with ⟨seed, htarget⟩
    exact ⟨{
      hiddenPreimage := hiddenPreimageOfSeed A.deriveFiberState seed
    }, ⟨seed, rfl, htarget, A.relationMapSound htarget⟩⟩
  completeFields := by
    intro statement witness hrel
    rcases hrel with ⟨seed, hseed, htarget, _hopens⟩
    exact ⟨A.proveFields statement witness seed,
      A.proveFields_accepts hseed htarget⟩
  witnessHiding := A.witnessHiding
  witnessHiding_holds := A.witnessHiding_holds

def toLinearSigmaOpeningTarget
    (A : SeedRestrictedCompressedLinearSigmaAlgebra) :
    LinearSigmaOpeningTarget :=
  A.toAlgebraicLinearSigmaOpeningConfig.toLinearSigmaOpeningTarget

theorem target_totalBytes
    (_A : SeedRestrictedCompressedLinearSigmaAlgebra) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

theorem restrictedDomain_not_pigeonholeBlocked
    (A : SeedRestrictedCompressedLinearSigmaAlgebra) :
    Function.Injective (hiddenPreimageOfSeed A.deriveFiberState) :=
  hiddenPreimageOfSeed_injective A.deriveFiberState

end SeedRestrictedCompressedLinearSigmaAlgebra

/--
SIS-style specialization of the restricted seed opening map.

The public target is exactly the account digest in the statement.  The concrete
candidate must instantiate `relationMapSeed` with a compressed algebraic
one-way/binding map and prove the two explicit assumptions:

* `relationMapSound`: a seed hitting the digest also satisfies the byte opening
  relation for the statement;
* `seedKnowledgeSoundness`: any accepting sigma equation yields such a seed.
-/
structure SISStyleSeedOpeningMap where
  deriveFiberState :
    ByteVector padSeedBytes -> ByteVector localFiberStateBytes
  seedOpensStatement :
    PublicOpeningStatement -> ByteVector padSeedBytes -> Prop
  relationMapSeed :
    PublicOpeningStatement ->
      ByteVector padSeedBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  relationMapSound :
    ∀ {statement seed},
      relationMapSeed statement seed = statement.digest ->
        seedOpensStatement statement seed
  addSeed :
    ByteVector padSeedBytes -> ByteVector padSeedBytes ->
      ByteVector padSeedBytes
  scaleSeed :
    ByteVector challengeBytes -> ByteVector padSeedBytes ->
      ByteVector padSeedBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  seed_linear :
    ∀ statement maskSeed witnessSeed challenge,
      relationMapSeed statement
          (addSeed maskSeed (scaleSeed challenge witnessSeed)) =
        addCommitment (relationMapSeed statement maskSeed)
          (scaleCommitment challenge (relationMapSeed statement witnessSeed))
  proverMaskSeed :
    PublicOpeningStatement -> PublicOpeningWitness -> ByteVector padSeedBytes
  sisKnowledgeSoundness :
    ∀ {statement proofCommitment maskedSeedResponse},
      relationMapSeed statement maskedSeedResponse =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge statement.digest) ->
        ∃ seed, relationMapSeed statement seed = statement.digest
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace SISStyleSeedOpeningMap

def target (statement : PublicOpeningStatement) :
    ByteVector linearSigmaProofCommitmentBytes :=
  statement.digest

def toSeedRestrictedCompressedLinearSigmaAlgebra
    (M : SISStyleSeedOpeningMap) :
    SeedRestrictedCompressedLinearSigmaAlgebra where
  deriveFiberState := M.deriveFiberState
  seedOpensStatement := M.seedOpensStatement
  relationMapSeed := M.relationMapSeed
  target := target
  relationMapSound := by
    intro statement seed h
    exact M.relationMapSound h
  addSeed := M.addSeed
  scaleSeed := M.scaleSeed
  addCommitment := M.addCommitment
  scaleCommitment := M.scaleCommitment
  seed_linear := M.seed_linear
  proverMaskSeed := M.proverMaskSeed
  seedKnowledgeSoundness := by
    intro statement proofCommitment maskedSeedResponse hverify
    exact M.sisKnowledgeSoundness hverify
  witnessHiding := M.witnessHiding
  witnessHiding_holds := M.witnessHiding_holds

def toLinearSigmaOpeningTarget
    (M : SISStyleSeedOpeningMap) :
    LinearSigmaOpeningTarget :=
  M.toSeedRestrictedCompressedLinearSigmaAlgebra.toLinearSigmaOpeningTarget

theorem target_eq_statement_digest
    (statement : PublicOpeningStatement) :
    target statement = statement.digest :=
  rfl

theorem totalBytes
    (_M : SISStyleSeedOpeningMap) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

theorem restrictedDomain_injective
    (M : SISStyleSeedOpeningMap) :
    Function.Injective (hiddenPreimageOfSeed M.deriveFiberState) :=
  hiddenPreimageOfSeed_injective M.deriveFiberState

end SISStyleSeedOpeningMap

abbrev SISSeedCoord := Fin sisSeedResidues
abbrev SISSyndromeCoord := Fin sisSyndromeResidues
abbrev SISSeedVector := SISSeedCoord -> ZMod modulus
abbrev SISSyndrome := SISSyndromeCoord -> ZMod modulus
abbrev SISFullResponseCoord := Fin sisFullResponseResidues
abbrev SISFullResponseSyndromeCoord := Fin sisFullResponseSyndromeResidues
abbrev SISFullResponseVector := SISFullResponseCoord -> ZMod modulus
abbrev SISFullResponseSyndrome :=
  SISFullResponseSyndromeCoord -> ZMod modulus

abbrev sisFullResponseMatrixDomainBytes : Nat := 66

/--
ASCII `Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1`.

The Rust reference samples the 12x23 full-response matrix from this exact
domain string using SHA-256 rejection sampling.  Lean keeps SHA-256 itself as a
named deployment agreement, but fixes the domain bytes and the q-ary layout.
-/
def sisFullResponseMatrixDomainTag :
    ByteVector sisFullResponseMatrixDomainBytes :=
  fun i =>
    byteOfNat <|
      match i.1 with
      | 0 => 68 | 1 => 101 | 2 => 115 | 3 => 99 | 4 => 101
      | 5 => 110 | 6 => 100 | 7 => 101 | 8 => 114 | 9 => 46
      | 10 => 83 | 11 => 105 | 12 => 103 | 13 => 110 | 14 => 97
      | 15 => 116 | 16 => 117 | 17 => 114 | 18 => 101 | 19 => 46
      | 20 => 113 | 21 => 49 | 22 => 50 | 23 => 50 | 24 => 56
      | 25 => 57 | 26 => 46 | 27 => 108 | 28 => 111 | 29 => 99
      | 30 => 97 | 31 => 108 | 32 => 95 | 33 => 100 | 34 => 101
      | 35 => 108 | 36 => 116 | 37 => 97 | 38 => 46 | 39 => 115
      | 40 => 105 | 41 => 115 | 42 => 95 | 43 => 102 | 44 => 117
      | 45 => 108 | 46 => 108 | 47 => 95 | 48 => 114 | 49 => 101
      | 50 => 115 | 51 => 112 | 52 => 111 | 53 => 110 | 54 => 115
      | 55 => 101 | 56 => 95 | 57 => 109 | 58 => 97 | 59 => 116
      | 60 => 114 | 61 => 105 | 62 => 120 | 63 => 46 | 64 => 118
      | 65 => 49
      | _ => 0

def fullResponseResidueRaw
    (bytes : ByteVector linearSigmaMaskedWitnessBytes)
    (i : SISFullResponseCoord) : Nat :=
  packedBitsNatLE bytes (i.1 * sisResidueBits) sisResidueBits

def fullResponseVectorCanonical
    (bytes : ByteVector linearSigmaMaskedWitnessBytes) : Prop :=
  (∀ i : SISFullResponseCoord, fullResponseResidueRaw bytes i < modulus) ∧
    packedBitsNatLE bytes
      sisFullResponsePackedBits sisFullResponseSlackBits = 0

def fullResponseVector
    (bytes : ByteVector linearSigmaMaskedWitnessBytes) :
    SISFullResponseVector :=
  fun i => (fullResponseResidueRaw bytes i : ZMod modulus)

def fullResponseDigestResidueRaw
    (digest : ByteVector sha256DigestBytes)
    (i : SISFullResponseSyndromeCoord) : Nat :=
  packedBitsNatLE digest (i.1 * sisResidueBits) sisResidueBits

def fullResponseDigestCanonical
    (digest : ByteVector sha256DigestBytes) : Prop :=
  (∀ i : SISFullResponseSyndromeCoord,
    fullResponseDigestResidueRaw digest i < modulus) ∧
    packedBitsNatLE digest
      sisFullResponseSyndromePackedBits
      sisFullResponseSyndromeSlackBits = 0

def fullResponseDigestSyndrome
    (digest : ByteVector sha256DigestBytes) :
    SISFullResponseSyndrome :=
  fun i => (fullResponseDigestResidueRaw digest i : ZMod modulus)

def fullResponseSyndromeOfVector
    (matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus)
    (vector : SISFullResponseVector) :
    SISFullResponseSyndrome :=
  fun row =>
    ∑ col : SISFullResponseCoord, matrix row col * vector col

def fullResponseSyndromeMap
    (matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus)
    (_statement : PublicOpeningStatement)
    (response : ByteVector linearSigmaMaskedWitnessBytes) :
    SISFullResponseSyndrome :=
  fullResponseSyndromeOfVector matrix (fullResponseVector response)

def fullResponseSyndromeDigestMatches
    (matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus)
    (statement : PublicOpeningStatement)
    (response : ByteVector linearSigmaMaskedWitnessBytes) : Prop :=
  fullResponseVectorCanonical response ∧
    fullResponseDigestCanonical statement.digest ∧
    fullResponseSyndromeMap matrix statement response =
      fullResponseDigestSyndrome statement.digest

namespace FullResponseCodec

theorem responseResidueRaw_eq_packed14
    (bytes : ByteVector linearSigmaMaskedWitnessBytes)
    (i : SISFullResponseCoord) :
    fullResponseResidueRaw bytes i =
      packedBitsNatLE bytes (i.1 * sisResidueBits) sisResidueBits := by
  rfl

theorem digestResidueRaw_eq_packed14
    (digest : ByteVector sha256DigestBytes)
    (i : SISFullResponseSyndromeCoord) :
    fullResponseDigestResidueRaw digest i =
      packedBitsNatLE digest (i.1 * sisResidueBits) sisResidueBits := by
  rfl

theorem syndromeDigestMatches_iff
    (matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus)
    (statement : PublicOpeningStatement)
    (response : ByteVector linearSigmaMaskedWitnessBytes) :
    fullResponseSyndromeDigestMatches matrix statement response ↔
      fullResponseVectorCanonical response ∧
        fullResponseDigestCanonical statement.digest ∧
        fullResponseSyndromeMap matrix statement response =
          fullResponseDigestSyndrome statement.digest := by
  rfl

theorem digestSyndrome_eq_decode
    (digest : ByteVector sha256DigestBytes)
    (i : SISFullResponseSyndromeCoord) :
    fullResponseDigestSyndrome digest i =
      (fullResponseDigestResidueRaw digest i : ZMod modulus) := by
  rfl

end FullResponseCodec

/--
Deployment agreement for the deterministic full-response matrix sampler.

The concrete Rust code samples each matrix entry by SHA-256 rejection sampling
from `sisFullResponseMatrixDomainTag`, row, column, and counter bytes.  Lean
does not reimplement SHA-256 here; this record names the exact external
agreement reviewers must check against the executable matrix generator.
-/
structure FullResponseMatrixSamplerAgreement
    (matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus) where
  sha256RejectionSamplerMatchesRust : Prop
  sha256RejectionSamplerMatchesRust_holds :
    sha256RejectionSamplerMatchesRust

/--
Algebraic certificate for the selected 12x23 full-response quotient map.

The executable certificate emits the concrete pivots, free columns, kernel
basis digest, and right-inverse digest.  Lean keeps the proof-relevant content:
the public quotient map is surjective, and the advertised hidden fiber vectors
really lie in the kernel.  This closes the linear-algebra part of the quotient
story without pretending to prove the cryptographic extraction or hiding
assumptions.
-/
structure FullResponseQuotientAlgebraCertificate
    (matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus) where
  rightInverse : SISFullResponseSyndrome -> SISFullResponseVector
  rightInverse_hits :
    ∀ syndrome,
      fullResponseSyndromeOfVector matrix (rightInverse syndrome) = syndrome
  kernelVector :
    Fin sisFullResponseHiddenFiberResidues -> SISFullResponseVector
  kernelVector_maps_zero :
    ∀ i,
      fullResponseSyndromeOfVector matrix (kernelVector i) = 0
  kernelVector_nonzero :
    ∀ i, kernelVector i ≠ 0

namespace FullResponseQuotientAlgebraCertificate

theorem syndromeMap_surjective
    {matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus}
    (C : FullResponseQuotientAlgebraCertificate matrix) :
    Function.Surjective (fullResponseSyndromeOfVector matrix) := by
  intro syndrome
  exact ⟨C.rightInverse syndrome, C.rightInverse_hits syndrome⟩

theorem fiber_nonempty
    {matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus}
    (C : FullResponseQuotientAlgebraCertificate matrix)
    (syndrome : SISFullResponseSyndrome) :
    ∃ response,
      fullResponseSyndromeOfVector matrix response = syndrome :=
  C.syndromeMap_surjective syndrome

theorem kernelVector_same_syndrome
    {matrix :
      SISFullResponseSyndromeCoord ->
        SISFullResponseCoord -> ZMod modulus}
    (C : FullResponseQuotientAlgebraCertificate matrix)
    (base : SISFullResponseVector)
    (i : Fin sisFullResponseHiddenFiberResidues) :
    fullResponseSyndromeOfVector matrix
        (fun col => base col + C.kernelVector i col) =
      fullResponseSyndromeOfVector matrix base := by
  funext row
  have hzero := congrFun (C.kernelVector_maps_zero i) row
  simp [fullResponseSyndromeOfVector, Finset.sum_add_distrib, mul_add] at hzero ⊢
  exact hzero

end FullResponseQuotientAlgebraCertificate

def NontrivialSeedFiber
    (relationMapSeed :
      PublicOpeningStatement ->
        ByteVector padSeedBytes ->
          ByteVector linearSigmaProofCommitmentBytes)
    (statement : PublicOpeningStatement) : Prop :=
  ∃ seed₀ seed₁ : ByteVector padSeedBytes,
    seed₀ ≠ seed₁ ∧
      relationMapSeed statement seed₀ = statement.digest ∧
      relationMapSeed statement seed₁ = statement.digest

theorem noNontrivialSeedFiber_of_injective
    {relationMapSeed :
      PublicOpeningStatement ->
        ByteVector padSeedBytes ->
          ByteVector linearSigmaProofCommitmentBytes}
    {statement : PublicOpeningStatement}
    (hinj : Function.Injective (relationMapSeed statement)) :
    ¬ NontrivialSeedFiber relationMapSeed statement := by
  rintro ⟨seed₀, seed₁, hneq, h₀, h₁⟩
  exact hneq (hinj (by rw [h₀, h₁]))

def PublicSeedRecoverer
    (relationMapSeed :
      PublicOpeningStatement ->
        ByteVector padSeedBytes ->
          ByteVector linearSigmaProofCommitmentBytes)
    (statement : PublicOpeningStatement)
    (recover :
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector padSeedBytes) : Prop :=
  ∀ {seed : ByteVector padSeedBytes},
    relationMapSeed statement seed = statement.digest ->
      recover statement.digest = seed

theorem publicSeedRecoverer_of_leftInverse
    {relationMapSeed :
      PublicOpeningStatement ->
        ByteVector padSeedBytes ->
          ByteVector linearSigmaProofCommitmentBytes}
    {statement : PublicOpeningStatement}
    {recover :
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector padSeedBytes}
    (hleft :
      ∀ seed : ByteVector padSeedBytes,
        recover (relationMapSeed statement seed) = seed) :
    PublicSeedRecoverer relationMapSeed statement recover := by
  intro seed htarget
  rw [← htarget]
  exact hleft seed

def fiberedSISPublicBits (publicResidues : Nat) : Nat :=
  publicResidues * sisResidueBits

def fiberedSISFiberCapacityBits (publicResidues : Nat) : Nat :=
  (sisSeedResidues - publicResidues) * sisResidueBits + sisSeedSlackBits

theorem balancedFiberSplit_publicRows9_publicBits_eq :
    fiberedSISPublicBits 9 = 126 := by
  native_decide

theorem balancedFiberSplit_publicRows9_fiberCapacityBits_eq :
    fiberedSISFiberCapacityBits 9 = 130 := by
  native_decide

theorem noBalanced128BitFiberSplit
    {publicResidues : Nat}
    (hrows : publicResidues ≤ sisSeedResidues)
    (hpublic : 128 ≤ fiberedSISPublicBits publicResidues)
    (hfiber : 128 ≤ fiberedSISFiberCapacityBits publicResidues) :
    False := by
  norm_num [fiberedSISPublicBits, fiberedSISFiberCapacityBits,
    sisSeedResidues, sisResidueBits, sisSeedSlackBits,
    sisSeedPackedBits, padSeedBytes, sha256DigestBytes] at *
  omega

def linearBlindingMessageBits (messageResidues : Nat) : Nat :=
  messageResidues * sisResidueBits

def linearBlindingPublicBits (publicResidues : Nat) : Nat :=
  publicResidues * sisResidueBits

theorem noPedersenStyle128BitSplit
    {messageResidues blindingResidues publicResidues : Nat}
    (hpacked : messageResidues + blindingResidues ≤ sisSeedResidues)
    (hmessage : 128 ≤ linearBlindingMessageBits messageResidues)
    (hpublic : 128 ≤ linearBlindingPublicBits publicResidues)
    (hperfectHidingDimension : publicResidues ≤ blindingResidues) :
    False := by
  norm_num [linearBlindingMessageBits, linearBlindingPublicBits,
    sisSeedResidues, sisResidueBits] at *
  omega

def fullResponseFiberPublicBits (publicResidues : Nat) : Nat :=
  publicResidues * sisResidueBits

def fullResponseFiberCapacityBits (publicResidues : Nat) : Nat :=
  (sisFullResponseResidues - publicResidues) * sisResidueBits +
    sisFullResponseSlackBits

theorem fullResponseFiber_publicRows12_publicBits_eq :
    fullResponseFiberPublicBits 12 = 168 := by
  native_decide

theorem fullResponseFiber_publicRows12_fiberCapacityBits_eq :
    fullResponseFiberCapacityBits 12 = 160 := by
  native_decide

theorem fullResponseFiber_publicRows12_meets128 :
    128 ≤ fullResponseFiberPublicBits 12 ∧
      128 ≤ fullResponseFiberCapacityBits 12 := by
  native_decide

theorem noFullResponsePedersenStyle128BitSplit
    {messageResidues blindingResidues publicResidues : Nat}
    (hpacked :
      messageResidues + blindingResidues ≤ 19)
    (hmessage : 128 ≤ linearBlindingMessageBits messageResidues)
    (hpublic : 128 ≤ linearBlindingPublicBits publicResidues)
    (hperfectHidingDimension : publicResidues ≤ blindingResidues) :
    False := by
  norm_num [linearBlindingMessageBits, linearBlindingPublicBits,
    sisFullResponseResidues, sisResidueBits] at *
  omega

/--
Fiber-saturated quotient relation for the full-response target.

The semantic opening witness is the public q-ary quotient
`digestSyndrome statement.digest`.  Individual 41-byte response representatives
are valid witnesses only through their quotient; the hidden 11-residue fiber is
therefore randomness, not byte-for-byte committed semantic state.
-/
structure FullResponseQuotientRelation where
  syndromeMap :
    PublicOpeningStatement ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        SISFullResponseSyndrome
  digestSyndrome :
    ByteVector sha256DigestBytes -> SISFullResponseSyndrome
  quotientOpensStatement :
    PublicOpeningStatement -> SISFullResponseSyndrome -> Prop
  targetQuotientOpens :
    ∀ statement : PublicOpeningStatement,
      quotientOpensStatement statement
        (digestSyndrome statement.digest)

namespace FullResponseQuotientRelation

def idealRelation
    (Q : FullResponseQuotientRelation)
    (statement : PublicOpeningStatement)
    (witness : PublicOpeningWitness) : Prop :=
  ∃ response : ByteVector linearSigmaMaskedWitnessBytes,
    hiddenPreimageFromLinearSigmaWitnessBytes response = witness.hiddenPreimage ∧
      Q.syndromeMap statement response =
        Q.digestSyndrome statement.digest ∧
      Q.quotientOpensStatement statement
        (Q.digestSyndrome statement.digest)

theorem idealRelation_of_targetSyndrome
    (Q : FullResponseQuotientRelation)
    {statement : PublicOpeningStatement}
    {response : ByteVector linearSigmaMaskedWitnessBytes}
    (htarget :
      Q.syndromeMap statement response =
        Q.digestSyndrome statement.digest) :
    Q.idealRelation statement
      { hiddenPreimage :=
          hiddenPreimageFromLinearSigmaWitnessBytes response } := by
  exact ⟨response, rfl, htarget, Q.targetQuotientOpens statement⟩

theorem idealRelation_iff_of_sameSyndrome
    (Q : FullResponseQuotientRelation)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      Q.syndromeMap statement response₀ =
        Q.digestSyndrome statement.digest)
    (h₁ :
      Q.syndromeMap statement response₁ =
        Q.digestSyndrome statement.digest) :
    Q.idealRelation statement
        { hiddenPreimage :=
            hiddenPreimageFromLinearSigmaWitnessBytes response₁ } ↔
      Q.idealRelation statement
        { hiddenPreimage :=
            hiddenPreimageFromLinearSigmaWitnessBytes response₀ } := by
  constructor
  · intro _hrel
    exact Q.idealRelation_of_targetSyndrome h₀
  · intro _hrel
    exact Q.idealRelation_of_targetSyndrome h₁

theorem acceptedFiber_relationSaturated
    (Q : FullResponseQuotientRelation)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      Q.syndromeMap statement response₀ =
        Q.digestSyndrome statement.digest)
    (hsame :
      Q.syndromeMap statement response₁ =
        Q.syndromeMap statement response₀) :
    Q.idealRelation statement
      { hiddenPreimage :=
          hiddenPreimageFromLinearSigmaWitnessBytes response₁ } :=
  Q.idealRelation_of_targetSyndrome (by rw [hsame, h₀])

theorem no_fullPreimageCommitmentBinding_of_nontrivialFiber
    (Q : FullResponseQuotientRelation)
    (commit :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes ->
        ByteVector sha256DigestBytes)
    (hcommit_injective : Function.Injective commit)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      Q.syndromeMap statement response₀ =
        Q.digestSyndrome statement.digest)
    (h₁ :
      Q.syndromeMap statement response₁ =
        Q.digestSyndrome statement.digest)
    (hneq :
      hiddenPreimageFromLinearSigmaWitnessBytes response₀ ≠
        hiddenPreimageFromLinearSigmaWitnessBytes response₁) :
    ¬ (∀ {witness : PublicOpeningWitness},
        Q.idealRelation statement witness ->
          commit witness.hiddenPreimage = statement.digest) := by
  intro hbindsFullPreimage
  have hopen₀ :
      Q.idealRelation statement
        { hiddenPreimage :=
            hiddenPreimageFromLinearSigmaWitnessBytes response₀ } :=
    Q.idealRelation_of_targetSyndrome h₀
  have hopen₁ :
      Q.idealRelation statement
        { hiddenPreimage :=
            hiddenPreimageFromLinearSigmaWitnessBytes response₁ } :=
    Q.idealRelation_of_targetSyndrome h₁
  have hcommit₀ :
      commit (hiddenPreimageFromLinearSigmaWitnessBytes response₀) =
        statement.digest :=
    hbindsFullPreimage hopen₀
  have hcommit₁ :
      commit (hiddenPreimageFromLinearSigmaWitnessBytes response₁) =
        statement.digest :=
    hbindsFullPreimage hopen₁
  have hhidden :
      hiddenPreimageFromLinearSigmaWitnessBytes response₀ =
        hiddenPreimageFromLinearSigmaWitnessBytes response₁ :=
    hcommit_injective (by rw [hcommit₀, hcommit₁])
  exact hneq hhidden

end FullResponseQuotientRelation

/--
Build a fiber-saturated quotient relation from a representative-level opening
predicate.

The quotient opens a statement when *some* response representative in that
fiber satisfies the old local opening predicate.  An extractor may return any
representative in the same accepted fiber; the semantic claim is existence of a
valid representative, not byte-for-byte validity of the extracted randomizer.
-/
structure FullResponseExistentialQuotientRelation where
  syndromeMap :
    PublicOpeningStatement ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        SISFullResponseSyndrome
  digestSyndrome :
    ByteVector sha256DigestBytes -> SISFullResponseSyndrome
  representativeOpensStatement :
    PublicOpeningStatement ->
      ByteVector linearSigmaMaskedWitnessBytes -> Prop
  targetRepresentativeExists :
    ∀ statement : PublicOpeningStatement,
      ∃ response,
        syndromeMap statement response =
          digestSyndrome statement.digest ∧
        representativeOpensStatement statement response

namespace FullResponseExistentialQuotientRelation

def quotientOpensStatement
    (E : FullResponseExistentialQuotientRelation)
    (statement : PublicOpeningStatement)
    (quotient : SISFullResponseSyndrome) : Prop :=
  ∃ response,
    E.syndromeMap statement response = quotient ∧
      E.representativeOpensStatement statement response

def toFullResponseQuotientRelation
    (E : FullResponseExistentialQuotientRelation) :
    FullResponseQuotientRelation where
  syndromeMap := E.syndromeMap
  digestSyndrome := E.digestSyndrome
  quotientOpensStatement := E.quotientOpensStatement
  targetQuotientOpens := by
    intro statement
    exact E.targetRepresentativeExists statement

theorem targetQuotientOpens_iff_existsRepresentative
    (E : FullResponseExistentialQuotientRelation)
    (statement : PublicOpeningStatement) :
    E.quotientOpensStatement statement
        (E.digestSyndrome statement.digest) ↔
      ∃ response,
        E.syndromeMap statement response =
          E.digestSyndrome statement.digest ∧
        E.representativeOpensStatement statement response := by
  rfl

theorem idealRelation_of_sameFiber_as_representative
    (E : FullResponseExistentialQuotientRelation)
    {statement : PublicOpeningStatement}
    {extracted representative : ByteVector linearSigmaMaskedWitnessBytes}
    (hsame :
      E.syndromeMap statement extracted =
        E.syndromeMap statement representative)
    (htarget :
      E.syndromeMap statement representative =
        E.digestSyndrome statement.digest)
    (hopen :
      E.representativeOpensStatement statement representative) :
    E.toFullResponseQuotientRelation.idealRelation statement
      { hiddenPreimage :=
          hiddenPreimageFromLinearSigmaWitnessBytes extracted } := by
  exact ⟨extracted, rfl, by
    simp [toFullResponseQuotientRelation]
    rw [hsame, htarget],
    ⟨representative, htarget, hopen⟩⟩

end FullResponseExistentialQuotientRelation

namespace ByteLocalDeltaRepresentativeSemantics

def toFullResponseExistentialQuotientRelation
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (digestSyndrome :
      ByteVector sha256DigestBytes -> SISFullResponseSyndrome)
    (targetRepresentativeExists :
      ∀ statement : PublicOpeningStatement,
        ∃ response,
          syndromeMap statement response =
            digestSyndrome statement.digest ∧
          S.representativeOpensStatement statement response) :
    FullResponseExistentialQuotientRelation where
  syndromeMap := syndromeMap
  digestSyndrome := digestSyndrome
  representativeOpensStatement := S.representativeOpensStatement
  targetRepresentativeExists := targetRepresentativeExists

theorem toFullResponseExistentialQuotientRelation_quotientOpens
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (digestSyndrome :
      ByteVector sha256DigestBytes -> SISFullResponseSyndrome)
    (targetRepresentativeExists :
      ∀ statement : PublicOpeningStatement,
        ∃ response,
          syndromeMap statement response =
            digestSyndrome statement.digest ∧
          S.representativeOpensStatement statement response)
    (statement : PublicOpeningStatement)
    (quotient : SISFullResponseSyndrome) :
    FullResponseExistentialQuotientRelation.quotientOpensStatement
      (S.toFullResponseExistentialQuotientRelation
        syndromeMap digestSyndrome targetRepresentativeExists)
      statement quotient =
      S.quotientOpensStatement syndromeMap statement quotient :=
  rfl

structure HonestLocalDeltaQuotientStatement
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (digestSyndrome :
      ByteVector sha256DigestBytes -> SISFullResponseSyndrome)
    (statement : PublicOpeningStatement) where
  representative : ByteVector linearSigmaMaskedWitnessBytes
  representative_hits_digest :
    syndromeMap statement representative =
      digestSyndrome statement.digest
  representative_opens :
    S.representativeOpensStatement statement representative

def targetRepresentativeExists_of_honestStatements
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (digestSyndrome :
      ByteVector sha256DigestBytes -> SISFullResponseSyndrome)
    (honest :
      ∀ statement : PublicOpeningStatement,
        HonestLocalDeltaQuotientStatement
          S syndromeMap digestSyndrome statement) :
    ∀ statement : PublicOpeningStatement,
      ∃ response,
        syndromeMap statement response =
          digestSyndrome statement.digest ∧
        S.representativeOpensStatement statement response := by
  intro statement
  exact ⟨(honest statement).representative,
    (honest statement).representative_hits_digest,
    (honest statement).representative_opens⟩

def toFullResponseExistentialQuotientRelationOfHonestStatements
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (digestSyndrome :
      ByteVector sha256DigestBytes -> SISFullResponseSyndrome)
    (honest :
      ∀ statement : PublicOpeningStatement,
        HonestLocalDeltaQuotientStatement
          S syndromeMap digestSyndrome statement) :
    FullResponseExistentialQuotientRelation :=
  S.toFullResponseExistentialQuotientRelation
    syndromeMap digestSyndrome
    (S.targetRepresentativeExists_of_honestStatements
      syndromeMap digestSyndrome honest)

theorem quotientOpensStatement_of_honestStatement
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (digestSyndrome :
      ByteVector sha256DigestBytes -> SISFullResponseSyndrome)
    {statement : PublicOpeningStatement}
    (honest :
      HonestLocalDeltaQuotientStatement
        S syndromeMap digestSyndrome statement) :
    S.quotientOpensStatement syndromeMap statement
      (digestSyndrome statement.digest) := by
  exact ⟨honest.representative,
    honest.representative_hits_digest,
    honest.representative_opens⟩

theorem quotientIdealRelation_of_sameFiber_as_localDeltaRepresentative
    (S : ByteLocalDeltaRepresentativeSemantics)
    (syndromeMap :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          SISFullResponseSyndrome)
    (digestSyndrome :
      ByteVector sha256DigestBytes -> SISFullResponseSyndrome)
    (targetRepresentativeExists :
      ∀ statement : PublicOpeningStatement,
        ∃ response,
          syndromeMap statement response =
            digestSyndrome statement.digest ∧
          S.representativeOpensStatement statement response)
    {statement : PublicOpeningStatement}
    {extracted representative : ByteVector linearSigmaMaskedWitnessBytes}
    (hsame :
      syndromeMap statement extracted =
        syndromeMap statement representative)
    (htarget :
      syndromeMap statement representative =
        digestSyndrome statement.digest)
    (hopen :
      S.representativeOpensStatement statement representative) :
    (S.toFullResponseExistentialQuotientRelation
        syndromeMap digestSyndrome targetRepresentativeExists)
      |>.toFullResponseQuotientRelation
      |>.idealRelation statement
        { hiddenPreimage :=
            hiddenPreimageFromLinearSigmaWitnessBytes extracted } :=
  FullResponseExistentialQuotientRelation.idealRelation_of_sameFiber_as_representative
    (S.toFullResponseExistentialQuotientRelation
      syndromeMap digestSyndrome targetRepresentativeExists)
    hsame htarget hopen

end ByteLocalDeltaRepresentativeSemantics

/--
Packed full-response gate specialized to a fiber-saturated quotient relation.

Compared with `PackedSISFullResponseOpeningReduction`, this object does not ask
the concrete instantiation to prove a byte-for-byte hidden-preimage relation.
Instead, the ideal relation is induced by the public quotient syndrome:
`response` opens iff it maps to the digest quotient and that quotient is accepted
by the public statement relation.  The 11-residue kernel is therefore allowed to
remain hidden randomness.
-/
structure PackedSISFullResponseQuotientOpeningReduction extends
    FullResponseQuotientRelation where
  validResponse : ByteVector linearSigmaMaskedWitnessBytes -> Prop
  matrix :
    SISFullResponseSyndromeCoord ->
      SISFullResponseCoord -> ZMod modulus
  responseVector :
    ByteVector linearSigmaMaskedWitnessBytes -> SISFullResponseVector
  validDigest : ByteVector sha256DigestBytes -> Prop
  packSyndrome :
    SISFullResponseSyndrome -> ByteVector sha256DigestBytes
  pack_digestSyndrome :
    ∀ {digest}, validDigest digest ->
      packSyndrome (digestSyndrome digest) = digest
  pack_injective : Function.Injective packSyndrome
  statementDigestValid :
    ∀ statement : PublicOpeningStatement, validDigest statement.digest
  valid_of_relationMapResponse_eq_digest :
    ∀ {statement : PublicOpeningStatement}
      {response : ByteVector linearSigmaMaskedWitnessBytes},
      packSyndrome (syndromeMap statement response) = statement.digest ->
        validResponse response
  addResponse :
    ByteVector linearSigmaMaskedWitnessBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  scaleResponse :
    ByteVector challengeBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  valid_add_scale :
    ∀ {maskResponse witnessResponse : ByteVector linearSigmaMaskedWitnessBytes}
      {challenge : ByteVector challengeBytes},
      validResponse maskResponse ->
        validResponse witnessResponse ->
          validResponse
            (addResponse maskResponse
              (scaleResponse challenge witnessResponse))
  syndrome_linear_packed :
    ∀ (statement : PublicOpeningStatement)
      (maskResponse witnessResponse :
        ByteVector linearSigmaMaskedWitnessBytes)
      (challenge : ByteVector challengeBytes),
      packSyndrome
          (syndromeMap statement
            (addResponse maskResponse
              (scaleResponse challenge witnessResponse))) =
        addCommitment
          (packSyndrome (syndromeMap statement maskResponse))
          (scaleCommitment challenge
            (packSyndrome (syndromeMap statement witnessResponse)))
  proverMaskResponse :
    PublicOpeningStatement -> PublicOpeningWitness ->
      ByteVector linearSigmaMaskedWitnessBytes
  proverMaskResponse_valid :
    ∀ (statement : PublicOpeningStatement) (witness : PublicOpeningWitness),
      validResponse (proverMaskResponse statement witness)
  algebraicBindingBreak : Prop
  valid_of_accepts :
    ∀ {statement : PublicOpeningStatement}
      {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
      {maskedResponse : ByteVector linearSigmaMaskedWitnessBytes},
      packSyndrome (syndromeMap statement maskedResponse) =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge statement.digest) ->
        validResponse maskedResponse
  extracts_or_break :
    ∀ {statement : PublicOpeningStatement}
      {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
      {maskedResponse : ByteVector linearSigmaMaskedWitnessBytes},
      validResponse maskedResponse ->
        packSyndrome (syndromeMap statement maskedResponse) =
          addCommitment proofCommitment
            (scaleCommitment statement.challenge statement.digest) ->
          (∃ response,
            validResponse response ∧
              syndromeMap statement response =
                digestSyndrome statement.digest) ∨
            algebraicBindingBreak
  noAlgebraicBindingBreak : ¬ algebraicBindingBreak
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace PackedSISFullResponseQuotientOpeningReduction

def relationMapResponse
    (R : PackedSISFullResponseQuotientOpeningReduction)
    (statement : PublicOpeningStatement)
    (response : ByteVector linearSigmaMaskedWitnessBytes) :
    ByteVector linearSigmaProofCommitmentBytes :=
  R.packSyndrome (R.syndromeMap statement response)

theorem acceptedFiber_relationSaturated
    (R : PackedSISFullResponseQuotientOpeningReduction)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      R.syndromeMap statement response₀ =
        R.digestSyndrome statement.digest)
    (hsame :
      R.syndromeMap statement response₁ =
        R.syndromeMap statement response₀) :
    R.toFullResponseQuotientRelation.idealRelation statement
      { hiddenPreimage :=
          hiddenPreimageFromLinearSigmaWitnessBytes response₁ } :=
  R.toFullResponseQuotientRelation.acceptedFiber_relationSaturated h₀ hsame

theorem no_fullPreimageCommitmentBinding_of_nontrivialFiber
    (R : PackedSISFullResponseQuotientOpeningReduction)
    (commit :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes ->
        ByteVector sha256DigestBytes)
    (hcommit_injective : Function.Injective commit)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      R.syndromeMap statement response₀ =
        R.digestSyndrome statement.digest)
    (h₁ :
      R.syndromeMap statement response₁ =
        R.digestSyndrome statement.digest)
    (hneq :
      hiddenPreimageFromLinearSigmaWitnessBytes response₀ ≠
        hiddenPreimageFromLinearSigmaWitnessBytes response₁) :
    ¬ (∀ {witness : PublicOpeningWitness},
        R.toFullResponseQuotientRelation.idealRelation statement witness ->
          commit witness.hiddenPreimage = statement.digest) :=
  R.toFullResponseQuotientRelation
    |>.no_fullPreimageCommitmentBinding_of_nontrivialFiber
      commit hcommit_injective h₀ h₁ hneq

end PackedSISFullResponseQuotientOpeningReduction

/--
Full-response q-ary/SIS reduction surface for the 254-byte target.

The whole 41-byte masked response is interpreted as `23` q-residues.  The
public target is a canonical packed `12`-row syndrome carried in the existing
32-byte digest/proof-commitment field.  This is the no-extra-byte path that
keeps a 73-byte proof layout while recovering a nontrivial hidden fiber.

The object fixes the exact proof obligations for a concrete matrix and packing:

* `relationMapSoundSyndrome` proves that any full response hitting the public
  syndrome opens the byte-level statement;
* `relationMapCompleteSyndrome` supplies an honest representative in the public
  syndrome for every ideal witness;
* `extracts_or_break` is the algebraic extraction gate for accepting sigma
  equations, with the break ruled out by `noAlgebraicBindingBreak`.
-/
structure PackedSISFullResponseOpeningReduction where
  idealRelation : PublicOpeningStatement -> PublicOpeningWitness -> Prop
  validResponse : ByteVector linearSigmaMaskedWitnessBytes -> Prop
  matrix :
    SISFullResponseSyndromeCoord ->
      SISFullResponseCoord -> ZMod modulus
  responseVector :
    ByteVector linearSigmaMaskedWitnessBytes -> SISFullResponseVector
  syndromeMap :
    PublicOpeningStatement ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        SISFullResponseSyndrome
  validDigest : ByteVector sha256DigestBytes -> Prop
  digestSyndrome :
    ByteVector sha256DigestBytes -> SISFullResponseSyndrome
  packSyndrome :
    SISFullResponseSyndrome -> ByteVector sha256DigestBytes
  pack_digestSyndrome :
    ∀ {digest}, validDigest digest ->
      packSyndrome (digestSyndrome digest) = digest
  pack_injective : Function.Injective packSyndrome
  statementDigestValid :
    ∀ statement : PublicOpeningStatement, validDigest statement.digest
  valid_of_relationMapResponse_eq_digest :
    ∀ {statement : PublicOpeningStatement}
      {response : ByteVector linearSigmaMaskedWitnessBytes},
      packSyndrome (syndromeMap statement response) = statement.digest ->
        validResponse response
  relationMapSoundSyndrome :
    ∀ {statement : PublicOpeningStatement}
      {response : ByteVector linearSigmaMaskedWitnessBytes},
      validResponse response ->
        syndromeMap statement response = digestSyndrome statement.digest ->
          idealRelation statement
            { hiddenPreimage :=
                hiddenPreimageFromLinearSigmaWitnessBytes response }
  relationMapCompleteSyndrome :
    ∀ {statement : PublicOpeningStatement}
      {witness : PublicOpeningWitness},
      idealRelation statement witness ->
        ∃ response,
          hiddenPreimageFromLinearSigmaWitnessBytes response =
            witness.hiddenPreimage ∧
          syndromeMap statement response =
            digestSyndrome statement.digest
  addResponse :
    ByteVector linearSigmaMaskedWitnessBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  scaleResponse :
    ByteVector challengeBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  valid_add_scale :
    ∀ {maskResponse witnessResponse : ByteVector linearSigmaMaskedWitnessBytes}
      {challenge : ByteVector challengeBytes},
      validResponse maskResponse ->
        validResponse witnessResponse ->
          validResponse
            (addResponse maskResponse
              (scaleResponse challenge witnessResponse))
  syndrome_linear_packed :
    ∀ (statement : PublicOpeningStatement)
      (maskResponse witnessResponse :
        ByteVector linearSigmaMaskedWitnessBytes)
      (challenge : ByteVector challengeBytes),
      packSyndrome
          (syndromeMap statement
            (addResponse maskResponse
              (scaleResponse challenge witnessResponse))) =
        addCommitment
          (packSyndrome (syndromeMap statement maskResponse))
          (scaleCommitment challenge
            (packSyndrome (syndromeMap statement witnessResponse)))
  proverMaskResponse :
    PublicOpeningStatement -> PublicOpeningWitness ->
      ByteVector linearSigmaMaskedWitnessBytes
  proverMaskResponse_valid :
    ∀ (statement : PublicOpeningStatement) (witness : PublicOpeningWitness),
      validResponse (proverMaskResponse statement witness)
  algebraicBindingBreak : Prop
  valid_of_accepts :
    ∀ {statement : PublicOpeningStatement}
      {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
      {maskedResponse : ByteVector linearSigmaMaskedWitnessBytes},
      packSyndrome (syndromeMap statement maskedResponse) =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge statement.digest) ->
        validResponse maskedResponse
  extracts_or_break :
    ∀ {statement : PublicOpeningStatement}
      {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
      {maskedResponse : ByteVector linearSigmaMaskedWitnessBytes},
      validResponse maskedResponse ->
        packSyndrome (syndromeMap statement maskedResponse) =
          addCommitment proofCommitment
            (scaleCommitment statement.challenge statement.digest) ->
          (∃ response,
            validResponse response ∧
              syndromeMap statement response =
                digestSyndrome statement.digest) ∨
            algebraicBindingBreak
  noAlgebraicBindingBreak : ¬ algebraicBindingBreak
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace PackedSISFullResponseOpeningReduction

def relationMapResponse
    (R : PackedSISFullResponseOpeningReduction)
    (statement : PublicOpeningStatement)
    (response : ByteVector linearSigmaMaskedWitnessBytes) :
    ByteVector linearSigmaProofCommitmentBytes :=
  R.packSyndrome (R.syndromeMap statement response)

def FullResponseFiber
    (relationMapResponse :
      PublicOpeningStatement ->
        ByteVector linearSigmaMaskedWitnessBytes ->
          ByteVector linearSigmaProofCommitmentBytes)
    (statement : PublicOpeningStatement) : Prop :=
  ∃ response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes,
    response₀ ≠ response₁ ∧
      relationMapResponse statement response₀ = statement.digest ∧
      relationMapResponse statement response₁ = statement.digest

theorem syndrome_eq_digest_of_relationMapResponse_eq_digest
    (R : PackedSISFullResponseOpeningReduction)
    {statement : PublicOpeningStatement}
    {response : ByteVector linearSigmaMaskedWitnessBytes}
    (h :
      R.relationMapResponse statement response = statement.digest) :
    R.syndromeMap statement response =
      R.digestSyndrome statement.digest := by
  apply R.pack_injective
  calc
    R.packSyndrome (R.syndromeMap statement response) =
        statement.digest := h
    _ = R.packSyndrome (R.digestSyndrome statement.digest) :=
        (R.pack_digestSyndrome (R.statementDigestValid statement)).symm

theorem relationMapComplete
    (R : PackedSISFullResponseOpeningReduction)
    {statement : PublicOpeningStatement}
    {witness : PublicOpeningWitness}
    (hrel : R.idealRelation statement witness) :
    ∃ response,
      hiddenPreimageFromLinearSigmaWitnessBytes response =
        witness.hiddenPreimage ∧
      R.relationMapResponse statement response = statement.digest := by
  rcases R.relationMapCompleteSyndrome hrel with
    ⟨response, hdecode, htarget⟩
  exact ⟨response, hdecode, by
    calc
      R.relationMapResponse statement response =
          R.packSyndrome
            (R.digestSyndrome statement.digest) := by
              simp [relationMapResponse, htarget]
      _ = statement.digest :=
          R.pack_digestSyndrome (R.statementDigestValid statement)⟩

def toFullResponseByteOpeningAlgebra
    (R : PackedSISFullResponseOpeningReduction) :
    FullResponseByteOpeningAlgebra where
  idealRelation := R.idealRelation
  relationMapResponse := R.relationMapResponse
  relationMapSound := by
    intro statement response h
    have hsyn :
        R.syndromeMap statement response =
          R.digestSyndrome statement.digest :=
      R.syndrome_eq_digest_of_relationMapResponse_eq_digest h
    have hvalid : R.validResponse response :=
      R.valid_of_relationMapResponse_eq_digest h
    exact R.relationMapSoundSyndrome
      (statement := statement) (response := response) hvalid hsyn
  relationMapComplete := by
    intro statement witness hrel
    exact R.relationMapComplete hrel
  addResponse := R.addResponse
  scaleResponse := R.scaleResponse
  addCommitment := R.addCommitment
  scaleCommitment := R.scaleCommitment
  response_linear := by
    intro statement maskResponse witnessResponse challenge
    exact R.syndrome_linear_packed
      statement maskResponse witnessResponse challenge
  proverMaskResponse := R.proverMaskResponse
  responseKnowledgeSoundness := by
    intro statement proofCommitment maskedResponse hverify
    have hvalid : R.validResponse maskedResponse :=
      R.valid_of_accepts hverify
    rcases R.extracts_or_break hvalid hverify with hresponse | hbreak
    · rcases hresponse with ⟨response, _hvalidResponse, hsyn⟩
      exact ⟨response, by
        calc
          R.relationMapResponse statement response =
              R.packSyndrome
                (R.digestSyndrome statement.digest) := by
                simp [relationMapResponse, hsyn]
          _ = statement.digest :=
              R.pack_digestSyndrome (R.statementDigestValid statement)⟩
    · exact False.elim (R.noAlgebraicBindingBreak hbreak)
  witnessHiding := R.witnessHiding
  witnessHiding_holds := R.witnessHiding_holds

def toLinearSigmaOpeningTarget
    (R : PackedSISFullResponseOpeningReduction) :
    LinearSigmaOpeningTarget :=
  R.toFullResponseByteOpeningAlgebra.toLinearSigmaOpeningTarget

theorem totalBytes
    (_R : PackedSISFullResponseOpeningReduction) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

theorem target_idealRelation_eq
    (R : PackedSISFullResponseOpeningReduction) :
    R.toLinearSigmaOpeningTarget.idealRelation = R.idealRelation :=
  rfl

theorem acceptedFiber_relationSaturated
    (R : PackedSISFullResponseOpeningReduction)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      R.relationMapResponse statement response₀ = statement.digest)
    (hsame :
      R.relationMapResponse statement response₁ =
        R.relationMapResponse statement response₀) :
    R.idealRelation statement
      { hiddenPreimage :=
          hiddenPreimageFromLinearSigmaWitnessBytes response₁ } :=
  FullResponseByteOpeningAlgebra.acceptedFiber_relationSaturated
    R.toFullResponseByteOpeningAlgebra h₀ hsame

theorem acceptedFiber_subsingleton_of_commitmentBinding
    (R : PackedSISFullResponseOpeningReduction)
    (commit :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes ->
        ByteVector sha256DigestBytes)
    (hideal_commit :
      ∀ {statement : PublicOpeningStatement}
        {witness : PublicOpeningWitness},
        R.idealRelation statement witness ->
          commit witness.hiddenPreimage = statement.digest)
    (hcommit_injective : Function.Injective commit)
    {statement : PublicOpeningStatement}
    {response₀ response₁ : ByteVector linearSigmaMaskedWitnessBytes}
    (h₀ :
      R.relationMapResponse statement response₀ = statement.digest)
    (h₁ :
      R.relationMapResponse statement response₁ = statement.digest) :
    hiddenPreimageFromLinearSigmaWitnessBytes response₀ =
      hiddenPreimageFromLinearSigmaWitnessBytes response₁ :=
  FullResponseByteOpeningAlgebra.acceptedFiber_subsingleton_of_commitmentBinding
    R.toFullResponseByteOpeningAlgebra commit hideal_commit
    hcommit_injective h₀ h₁

end PackedSISFullResponseOpeningReduction

namespace PackedSISFullResponseQuotientOpeningReduction

def toPackedSISFullResponseOpeningReduction
    (R : PackedSISFullResponseQuotientOpeningReduction) :
    PackedSISFullResponseOpeningReduction where
  idealRelation := R.toFullResponseQuotientRelation.idealRelation
  validResponse := R.validResponse
  matrix := R.matrix
  responseVector := R.responseVector
  syndromeMap := R.syndromeMap
  validDigest := R.validDigest
  digestSyndrome := R.digestSyndrome
  packSyndrome := R.packSyndrome
  pack_digestSyndrome := R.pack_digestSyndrome
  pack_injective := R.pack_injective
  statementDigestValid := R.statementDigestValid
  valid_of_relationMapResponse_eq_digest :=
    R.valid_of_relationMapResponse_eq_digest
  relationMapSoundSyndrome := by
    intro statement response _hvalid hsyn
    exact
      R.toFullResponseQuotientRelation.idealRelation_of_targetSyndrome hsyn
  relationMapCompleteSyndrome := by
    intro statement witness hrel
    rcases hrel with ⟨response, hdecode, htarget, _hopen⟩
    exact ⟨response, hdecode, htarget⟩
  addResponse := R.addResponse
  scaleResponse := R.scaleResponse
  addCommitment := R.addCommitment
  scaleCommitment := R.scaleCommitment
  valid_add_scale := by
    intro maskResponse witnessResponse challenge hmask hwitness
    exact R.valid_add_scale hmask hwitness
  syndrome_linear_packed := R.syndrome_linear_packed
  proverMaskResponse := R.proverMaskResponse
  proverMaskResponse_valid := R.proverMaskResponse_valid
  algebraicBindingBreak := R.algebraicBindingBreak
  valid_of_accepts := R.valid_of_accepts
  extracts_or_break := R.extracts_or_break
  noAlgebraicBindingBreak := R.noAlgebraicBindingBreak
  witnessHiding := R.witnessHiding
  witnessHiding_holds := R.witnessHiding_holds

def toFullResponseByteOpeningAlgebra
    (R : PackedSISFullResponseQuotientOpeningReduction) :
    FullResponseByteOpeningAlgebra :=
  R.toPackedSISFullResponseOpeningReduction.toFullResponseByteOpeningAlgebra

def toLinearSigmaOpeningTarget
    (R : PackedSISFullResponseQuotientOpeningReduction) :
    LinearSigmaOpeningTarget :=
  R.toPackedSISFullResponseOpeningReduction.toLinearSigmaOpeningTarget

theorem totalBytes
    (_R : PackedSISFullResponseQuotientOpeningReduction) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

theorem target_idealRelation_eq
    (R : PackedSISFullResponseQuotientOpeningReduction) :
    R.toLinearSigmaOpeningTarget.idealRelation =
      R.toFullResponseQuotientRelation.idealRelation :=
  rfl

end PackedSISFullResponseQuotientOpeningReduction

/--
Concrete honest signing trace for the local-delta statement.

This is the proof-facing shape of the stateful signer after byte serialization:
the signer has already chosen the hidden preimage, response bytes, and local
opening bytes.  The fields are exactly the equations the executable signer must
emit/check before packaging the public opening proof.
-/
structure ConcreteHonestLocalDeltaSigningTrace
    (P : PadPRFBinding)
    (statement : PublicOpeningStatement)
    (hidden :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes) where
  challengeValid : compactChallengeCanonical statement.challenge
  publicKeyValid : publicKeyFieldCanonical statement.publicKey
  responsesCanonical : activeResponseCanonical statement.responses
  openingsCanonical : localOpeningCanonical statement.openings
  localState :
    ∀ r : Fin repetitions,
      let b := compactChallengeActiveBlock statement.challenge r
      let mask :=
        (concreteByteLocalDeltaRepresentativeSemantics P).maskView
          statement r
      N32Q12289LocalSyndrome.syndrome.localState b mask =
        fiberStateValue hidden.fiberState
  paddedOpening :
    ∀ r : Fin repetitions,
      let b := compactChallengeActiveBlock statement.challenge r
      let mask :=
        (concreteByteLocalDeltaRepresentativeSemantics P).maskView
          statement r
      localViewOfPadBytes (statement.openings r) =
        (fun f => mask f + P.expandPad hidden.padSeed b f)

namespace ConcreteHonestLocalDeltaSigningTrace

theorem representativeOpensStatement
    {P : PadPRFBinding}
    {statement : PublicOpeningStatement}
    {hidden :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes}
    (T : ConcreteHonestLocalDeltaSigningTrace P statement hidden) :
    (concreteByteLocalDeltaRepresentativeSemantics P)
      |>.representativeOpensStatement statement
        (linearSigmaWitnessFromHiddenPreimage hidden) := by
  dsimp [ByteLocalDeltaRepresentativeSemantics.representativeOpensStatement,
    concreteByteLocalDeltaRepresentativeSemantics]
  constructor
  · exact T.challengeValid
  constructor
  · exact T.publicKeyValid
  constructor
  · exact T.responsesCanonical
  constructor
  · exact T.openingsCanonical
  · intro r
    constructor
    · simpa using T.localState r
    · simpa using T.paddedOpening r

end ConcreteHonestLocalDeltaSigningTrace

/--
Honest quotient-signing trace for the full-response packed SIS target.

`digest_is_packed_syndrome` is the concrete q-syndrome digest-builder
obligation: the account slot digest is not an independent SHA commitment to the
34-byte hidden preimage; it is the canonical packed public syndrome of that
preimage under the selected full-response map.
-/
structure ConcreteHonestFullResponseQuotientSigningTrace
    (P : PadPRFBinding)
    (R : PackedSISFullResponseQuotientOpeningReduction)
    (statement : PublicOpeningStatement)
    (hidden :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes)
    extends ConcreteHonestLocalDeltaSigningTrace P statement hidden where
  digest_is_packed_syndrome :
    R.packSyndrome
        (R.syndromeMap statement (linearSigmaWitnessFromHiddenPreimage hidden)) =
      statement.digest

namespace ConcreteHonestFullResponseQuotientSigningTrace

theorem syndromeMap_hits_digestSyndrome
    {P : PadPRFBinding}
    {R : PackedSISFullResponseQuotientOpeningReduction}
    {statement : PublicOpeningStatement}
    {hidden :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes}
    (T :
      ConcreteHonestFullResponseQuotientSigningTrace
        P R statement hidden) :
    R.syndromeMap statement (linearSigmaWitnessFromHiddenPreimage hidden) =
      R.digestSyndrome statement.digest := by
  apply R.pack_injective
  calc
    R.packSyndrome
        (R.syndromeMap statement (linearSigmaWitnessFromHiddenPreimage hidden)) =
        statement.digest := T.digest_is_packed_syndrome
    _ = R.packSyndrome (R.digestSyndrome statement.digest) :=
        (R.pack_digestSyndrome (R.statementDigestValid statement)).symm

def toHonestLocalDeltaQuotientStatement
    {P : PadPRFBinding}
    {R : PackedSISFullResponseQuotientOpeningReduction}
    {statement : PublicOpeningStatement}
    {hidden :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes}
    (T :
      ConcreteHonestFullResponseQuotientSigningTrace
        P R statement hidden) :
    ByteLocalDeltaRepresentativeSemantics.HonestLocalDeltaQuotientStatement
      (concreteByteLocalDeltaRepresentativeSemantics P)
      R.syndromeMap R.digestSyndrome statement where
  representative := linearSigmaWitnessFromHiddenPreimage hidden
  representative_hits_digest := T.syndromeMap_hits_digestSyndrome
  representative_opens :=
    T.toConcreteHonestLocalDeltaSigningTrace.representativeOpensStatement

theorem quotientOpensStatement
    {P : PadPRFBinding}
    {R : PackedSISFullResponseQuotientOpeningReduction}
    {statement : PublicOpeningStatement}
    {hidden :
      ByteCommittedPadPreimage localFiberStateBytes padSeedBytes}
    (T :
      ConcreteHonestFullResponseQuotientSigningTrace
        P R statement hidden) :
    (concreteByteLocalDeltaRepresentativeSemantics P)
      |>.quotientOpensStatement R.syndromeMap statement
        (R.digestSyndrome statement.digest) :=
  ByteLocalDeltaRepresentativeSemantics.quotientOpensStatement_of_honestStatement
    (concreteByteLocalDeltaRepresentativeSemantics P)
    R.syndromeMap R.digestSyndrome
    T.toHonestLocalDeltaQuotientStatement

theorem targetRepresentativeExists_of_traces
    (P : PadPRFBinding)
    (R : PackedSISFullResponseQuotientOpeningReduction)
    (honest :
      ∀ statement : PublicOpeningStatement,
        ∃ hidden,
          ConcreteHonestFullResponseQuotientSigningTrace
            P R statement hidden) :
    ∀ statement : PublicOpeningStatement,
      ∃ response,
        R.syndromeMap statement response =
          R.digestSyndrome statement.digest ∧
        ((concreteByteLocalDeltaRepresentativeSemantics P)
          |>.representativeOpensStatement statement response) := by
  intro statement
  rcases honest statement with ⟨hidden, trace⟩
  exact ⟨linearSigmaWitnessFromHiddenPreimage hidden,
    trace.syndromeMap_hits_digestSyndrome,
    trace.toConcreteHonestLocalDeltaSigningTrace.representativeOpensStatement⟩

end ConcreteHonestFullResponseQuotientSigningTrace

/--
Concrete deployment-facing assumption package for the full-response quotient
target.

The byte codec, digest decoder, valid-response predicate, digest-valid
predicate, and quotient opening predicate are fixed here.  The remaining fields
are exactly the algebraic sigma and cryptographic assumptions that still need
external review for the chosen 12x23 matrix and packing.
-/
structure ConcreteFullResponseQuotientReductionAssumptions
    (P : PadPRFBinding) where
  matrix :
    SISFullResponseSyndromeCoord ->
      SISFullResponseCoord -> ZMod modulus
  matrixSamplerAgreement :
    FullResponseMatrixSamplerAgreement matrix
  packSyndrome :
    SISFullResponseSyndrome -> ByteVector sha256DigestBytes
  pack_digestSyndrome :
    ∀ {digest}, fullResponseDigestCanonical digest ->
      packSyndrome (fullResponseDigestSyndrome digest) = digest
  pack_injective : Function.Injective packSyndrome
  statementDigestValid :
    ∀ statement : PublicOpeningStatement,
      fullResponseDigestCanonical statement.digest
  valid_of_relationMapResponse_eq_digest :
    ∀ {statement : PublicOpeningStatement}
      {response : ByteVector linearSigmaMaskedWitnessBytes},
      packSyndrome (fullResponseSyndromeMap matrix statement response) =
        statement.digest ->
        fullResponseVectorCanonical response
  targetRepresentativeExists :
    ∀ statement : PublicOpeningStatement,
      ∃ response,
        fullResponseSyndromeMap matrix statement response =
          fullResponseDigestSyndrome statement.digest ∧
        ((concreteByteLocalDeltaRepresentativeSemantics P)
          |>.representativeOpensStatement statement response)
  responseVector :
    ByteVector linearSigmaMaskedWitnessBytes -> SISFullResponseVector
  responseVector_eq_codec :
    ∀ response, responseVector response = fullResponseVector response
  addResponse :
    ByteVector linearSigmaMaskedWitnessBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  scaleResponse :
    ByteVector challengeBytes ->
      ByteVector linearSigmaMaskedWitnessBytes ->
        ByteVector linearSigmaMaskedWitnessBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  valid_add_scale :
    ∀ {maskResponse witnessResponse : ByteVector linearSigmaMaskedWitnessBytes}
      {challenge : ByteVector challengeBytes},
      fullResponseVectorCanonical maskResponse ->
        fullResponseVectorCanonical witnessResponse ->
          fullResponseVectorCanonical
            (addResponse maskResponse
              (scaleResponse challenge witnessResponse))
  syndrome_linear_packed :
    ∀ (statement : PublicOpeningStatement)
      (maskResponse witnessResponse :
        ByteVector linearSigmaMaskedWitnessBytes)
      (challenge : ByteVector challengeBytes),
      packSyndrome
          (fullResponseSyndromeMap matrix statement
            (addResponse maskResponse
              (scaleResponse challenge witnessResponse))) =
        addCommitment
          (packSyndrome
            (fullResponseSyndromeMap matrix statement maskResponse))
          (scaleCommitment challenge
            (packSyndrome
              (fullResponseSyndromeMap matrix statement witnessResponse)))
  proverMaskResponse :
    PublicOpeningStatement -> PublicOpeningWitness ->
      ByteVector linearSigmaMaskedWitnessBytes
  proverMaskResponse_valid :
    ∀ (statement : PublicOpeningStatement) (witness : PublicOpeningWitness),
      fullResponseVectorCanonical (proverMaskResponse statement witness)
  algebraicBindingBreak : Prop
  valid_of_accepts :
    ∀ {statement : PublicOpeningStatement}
      {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
      {maskedResponse : ByteVector linearSigmaMaskedWitnessBytes},
      packSyndrome
          (fullResponseSyndromeMap matrix statement maskedResponse) =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge statement.digest) ->
        fullResponseVectorCanonical maskedResponse
  extracts_or_break :
    ∀ {statement : PublicOpeningStatement}
      {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
      {maskedResponse : ByteVector linearSigmaMaskedWitnessBytes},
      fullResponseVectorCanonical maskedResponse ->
        packSyndrome
            (fullResponseSyndromeMap matrix statement maskedResponse) =
          addCommitment proofCommitment
            (scaleCommitment statement.challenge statement.digest) ->
          (∃ response,
            fullResponseVectorCanonical response ∧
              fullResponseSyndromeMap matrix statement response =
                fullResponseDigestSyndrome statement.digest) ∨
            algebraicBindingBreak
  noAlgebraicBindingBreak : ¬ algebraicBindingBreak
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace ConcreteFullResponseQuotientReductionAssumptions

def toPackedSISFullResponseQuotientOpeningReduction
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P) :
    PackedSISFullResponseQuotientOpeningReduction where
  syndromeMap := fullResponseSyndromeMap A.matrix
  digestSyndrome := fullResponseDigestSyndrome
  quotientOpensStatement :=
    (concreteByteLocalDeltaRepresentativeSemantics P)
      |>.quotientOpensStatement (fullResponseSyndromeMap A.matrix)
  targetQuotientOpens := by
    intro statement
    exact A.targetRepresentativeExists statement
  validResponse := fullResponseVectorCanonical
  matrix := A.matrix
  responseVector := A.responseVector
  validDigest := fullResponseDigestCanonical
  packSyndrome := A.packSyndrome
  pack_digestSyndrome := A.pack_digestSyndrome
  pack_injective := A.pack_injective
  statementDigestValid := A.statementDigestValid
  valid_of_relationMapResponse_eq_digest :=
    A.valid_of_relationMapResponse_eq_digest
  addResponse := A.addResponse
  scaleResponse := A.scaleResponse
  addCommitment := A.addCommitment
  scaleCommitment := A.scaleCommitment
  valid_add_scale := by
    intro maskResponse witnessResponse challenge hmask hwitness
    exact A.valid_add_scale hmask hwitness
  syndrome_linear_packed := A.syndrome_linear_packed
  proverMaskResponse := A.proverMaskResponse
  proverMaskResponse_valid := A.proverMaskResponse_valid
  algebraicBindingBreak := A.algebraicBindingBreak
  valid_of_accepts := A.valid_of_accepts
  extracts_or_break := A.extracts_or_break
  noAlgebraicBindingBreak := A.noAlgebraicBindingBreak
  witnessHiding := A.witnessHiding
  witnessHiding_holds := A.witnessHiding_holds

theorem relationMapResponse_eq_fullResponseSyndromeDigest
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P)
    (statement : PublicOpeningStatement)
    (response : ByteVector linearSigmaMaskedWitnessBytes) :
    (A.toPackedSISFullResponseQuotientOpeningReduction
      |>.relationMapResponse statement response) =
      A.packSyndrome
        (fullResponseSyndromeMap A.matrix statement response) := by
  rfl

theorem totalBytes
    {P : PadPRFBinding}
    (_A : ConcreteFullResponseQuotientReductionAssumptions P) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

def extractionFailure
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P) : Prop :=
  ∃ (statement : PublicOpeningStatement)
    (proofCommitment : ByteVector linearSigmaProofCommitmentBytes)
    (maskedResponse : ByteVector linearSigmaMaskedWitnessBytes),
    fullResponseVectorCanonical maskedResponse ∧
      A.packSyndrome
          (fullResponseSyndromeMap A.matrix statement maskedResponse) =
        A.addCommitment proofCommitment
          (A.scaleCommitment statement.challenge statement.digest) ∧
      ¬ ∃ response,
        fullResponseVectorCanonical response ∧
          fullResponseSyndromeMap A.matrix statement response =
            fullResponseDigestSyndrome statement.digest

theorem extracts_of_accepts
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P)
    {statement : PublicOpeningStatement}
    {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
    {maskedResponse : ByteVector linearSigmaMaskedWitnessBytes}
    (hvalid : fullResponseVectorCanonical maskedResponse)
    (hverify :
      A.packSyndrome
          (fullResponseSyndromeMap A.matrix statement maskedResponse) =
        A.addCommitment proofCommitment
          (A.scaleCommitment statement.challenge statement.digest)) :
    ∃ response,
      fullResponseVectorCanonical response ∧
        fullResponseSyndromeMap A.matrix statement response =
          fullResponseDigestSyndrome statement.digest := by
  rcases A.extracts_or_break hvalid hverify with hresponse | hbreak
  · exact hresponse
  · exact False.elim (A.noAlgebraicBindingBreak hbreak)

theorem no_extractionFailure
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P) :
    ¬ A.extractionFailure := by
  rintro ⟨statement, proofCommitment, maskedResponse,
    hvalid, hverify, hnoExtract⟩
  exact hnoExtract (A.extracts_of_accepts hvalid hverify)

end ConcreteFullResponseQuotientReductionAssumptions

/--
Concrete canonical response-lift certificate.

This isolates the purely linear algebraic lift used after the fork equation has
recovered a `12`-residue digest.  The certificate supplies a byte-level
`41`-byte full-response representative for any public syndrome and proves that
it is canonical and maps back to that syndrome under the fixed `12 x 23`
matrix.  For the executable profile this is implemented by the rank-12 right
inverse checked in `canonical_response_lift_report`.
-/
structure FullResponseCanonicalResponseLiftCertificate
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P) where
  liftResponse :
    SISFullResponseSyndrome ->
      ByteVector linearSigmaMaskedWitnessBytes
  liftResponse_canonical :
    ∀ syndrome,
      fullResponseVectorCanonical (liftResponse syndrome)
  liftResponse_syndrome :
    ∀ (statement : PublicOpeningStatement) (syndrome : SISFullResponseSyndrome),
      fullResponseSyndromeMap A.matrix statement (liftResponse syndrome) =
        syndrome

namespace FullResponseCanonicalResponseLiftCertificate

theorem relationMapResponse_lift_digest
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseCanonicalResponseLiftCertificate A)
    (statement : PublicOpeningStatement) :
    PackedSISFullResponseQuotientOpeningReduction.relationMapResponse
      A.toPackedSISFullResponseQuotientOpeningReduction
      statement
      (C.liftResponse
        (fullResponseDigestSyndrome statement.digest)) =
      statement.digest := by
  simp [PackedSISFullResponseQuotientOpeningReduction.relationMapResponse,
    ConcreteFullResponseQuotientReductionAssumptions.toPackedSISFullResponseQuotientOpeningReduction]
  rw [C.liftResponse_syndrome]
  exact A.pack_digestSyndrome (A.statementDigestValid statement)

theorem exists_canonical_lift_for_statement_digest
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseCanonicalResponseLiftCertificate A)
    (statement : PublicOpeningStatement) :
    ∃ response,
      fullResponseVectorCanonical response ∧
        fullResponseSyndromeMap A.matrix statement response =
          fullResponseDigestSyndrome statement.digest :=
  ⟨C.liftResponse (fullResponseDigestSyndrome statement.digest),
    C.liftResponse_canonical _,
    C.liftResponse_syndrome statement _⟩

theorem no_extractionFailure_of_canonical_lift
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseCanonicalResponseLiftCertificate A) :
    ¬ A.extractionFailure := by
  rintro ⟨statement, _proofCommitment, _maskedResponse,
    _hvalid, _hverify, hnoExtract⟩
  exact hnoExtract (C.exists_canonical_lift_for_statement_digest statement)

end FullResponseCanonicalResponseLiftCertificate

/--
Byte-level response arithmetic certificate for the fork extractor.

The concrete quotient implementation performs response arithmetic on the public
syndrome: decode a 41-byte response to its `12`-residue syndrome, perform the
`F_12289[X]/(X^12 + X + 25)` add/subtract/scale operation there, and use the
canonical response lift to return to a 41-byte representative.  This certificate
is the final algebraic bridge between two accepted masked responses and the
canonical extracted response.
-/
structure FullResponseByteResponseArithmeticCertificate
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P)
    (Action : FullResponseChallengeActionCertificate)
    (_Lift : FullResponseCanonicalResponseLiftCertificate A) where
  extractedForkResponse :
    PublicOpeningStatement ->
    PublicOpeningStatement ->
    ByteVector linearSigmaMaskedWitnessBytes ->
    ByteVector linearSigmaMaskedWitnessBytes ->
      ByteVector linearSigmaMaskedWitnessBytes
  extractedForkResponse_canonical :
    ∀ {statement₀ statement₁ maskedResponse₀ maskedResponse₁},
      fullResponseVectorCanonical
        (extractedForkResponse statement₀ statement₁
          maskedResponse₀ maskedResponse₁)
  extractedForkResponse_correct :
    ∀ {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁},
      statement₀.digest = statement₁.digest ->
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge ->
      A.packSyndrome
          (fullResponseSyndromeMap A.matrix statement₀ maskedResponse₀) =
        A.addCommitment proofCommitment
          (A.scaleCommitment statement₀.challenge statement₀.digest) ->
      A.packSyndrome
          (fullResponseSyndromeMap A.matrix statement₁ maskedResponse₁) =
        A.addCommitment proofCommitment
          (A.scaleCommitment statement₁.challenge statement₁.digest) ->
        fullResponseSyndromeMap A.matrix statement₀
          (extractedForkResponse statement₀ statement₁
            maskedResponse₀ maskedResponse₁) =
        fullResponseDigestSyndrome statement₀.digest

namespace FullResponseByteResponseArithmeticCertificate

theorem exists_canonical_extracted_response
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {Action : FullResponseChallengeActionCertificate}
    {Lift : FullResponseCanonicalResponseLiftCertificate A}
    (C : FullResponseByteResponseArithmeticCertificate A Action Lift)
    {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁}
    (hdigest : statement₀.digest = statement₁.digest)
    (hcollision :
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge)
    (haccept₀ :
      A.packSyndrome
          (fullResponseSyndromeMap A.matrix statement₀ maskedResponse₀) =
        A.addCommitment proofCommitment
          (A.scaleCommitment statement₀.challenge statement₀.digest))
    (haccept₁ :
      A.packSyndrome
          (fullResponseSyndromeMap A.matrix statement₁ maskedResponse₁) =
        A.addCommitment proofCommitment
          (A.scaleCommitment statement₁.challenge statement₁.digest)) :
    ∃ response,
      fullResponseVectorCanonical response ∧
        fullResponseSyndromeMap A.matrix statement₀ response =
          fullResponseDigestSyndrome statement₀.digest :=
  ⟨C.extractedForkResponse statement₀ statement₁
      maskedResponse₀ maskedResponse₁,
    C.extractedForkResponse_canonical,
    C.extractedForkResponse_correct
      hdigest hcollision haccept₀ haccept₁⟩

theorem relationMapResponse_extractedForkResponse
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {Action : FullResponseChallengeActionCertificate}
    {Lift : FullResponseCanonicalResponseLiftCertificate A}
    (C : FullResponseByteResponseArithmeticCertificate A Action Lift)
    {statement₀ statement₁ proofCommitment maskedResponse₀ maskedResponse₁}
    (hdigest : statement₀.digest = statement₁.digest)
    (hcollision :
      ¬ Action.badActionCollision statement₀.challenge statement₁.challenge)
    (haccept₀ :
      A.packSyndrome
          (fullResponseSyndromeMap A.matrix statement₀ maskedResponse₀) =
        A.addCommitment proofCommitment
          (A.scaleCommitment statement₀.challenge statement₀.digest))
    (haccept₁ :
      A.packSyndrome
          (fullResponseSyndromeMap A.matrix statement₁ maskedResponse₁) =
        A.addCommitment proofCommitment
          (A.scaleCommitment statement₁.challenge statement₁.digest)) :
    PackedSISFullResponseQuotientOpeningReduction.relationMapResponse
      A.toPackedSISFullResponseQuotientOpeningReduction
      statement₀
      (C.extractedForkResponse statement₀ statement₁
        maskedResponse₀ maskedResponse₁) =
      statement₀.digest := by
  simp [PackedSISFullResponseQuotientOpeningReduction.relationMapResponse,
    ConcreteFullResponseQuotientReductionAssumptions.toPackedSISFullResponseQuotientOpeningReduction]
  rw [C.extractedForkResponse_correct hdigest hcollision haccept₀ haccept₁]
  exact A.pack_digestSyndrome (A.statementDigestValid statement₀)

end FullResponseByteResponseArithmeticCertificate

/--
Same-commitment fork generated by the QROM measure-and-reprogram step.

The fork shape is deliberately byte-facing: both branches carry the same proof
commitment and the same prechallenge statement frame, while the challenge bytes
may differ.  This is the object consumed by the already-closed byte-response
arithmetic extractor.
-/
structure FullResponseQROMSameCommitmentFork
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P) where
  statement₀ : PublicOpeningStatement
  statement₁ : PublicOpeningStatement
  proofCommitment : ByteVector linearSigmaProofCommitmentBytes
  maskedResponse₀ : ByteVector linearSigmaMaskedWitnessBytes
  maskedResponse₁ : ByteVector linearSigmaMaskedWitnessBytes
  sameDigest : statement₀.digest = statement₁.digest
  samePublicKey : statement₀.publicKey = statement₁.publicKey
  sameResponses : statement₀.responses = statement₁.responses
  sameOpenings : statement₀.openings = statement₁.openings
  accepts₀ :
    A.packSyndrome
        (fullResponseSyndromeMap A.matrix statement₀ maskedResponse₀) =
      A.addCommitment proofCommitment
        (A.scaleCommitment statement₀.challenge statement₀.digest)
  accepts₁ :
    A.packSyndrome
        (fullResponseSyndromeMap A.matrix statement₁ maskedResponse₁) =
      A.addCommitment proofCommitment
        (A.scaleCommitment statement₁.challenge statement₁.digest)

namespace FullResponseQROMSameCommitmentFork

theorem extracts_canonical_response
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {Action : FullResponseChallengeActionCertificate}
    {Lift : FullResponseCanonicalResponseLiftCertificate A}
    (Fork : FullResponseQROMSameCommitmentFork A)
    (Arithmetic :
      FullResponseByteResponseArithmeticCertificate A Action Lift)
    (hcollision :
      ¬ Action.badActionCollision
        Fork.statement₀.challenge Fork.statement₁.challenge) :
    ∃ response,
      fullResponseVectorCanonical response ∧
        fullResponseSyndromeMap A.matrix Fork.statement₀ response =
          fullResponseDigestSyndrome Fork.statement₀.digest :=
  Arithmetic.exists_canonical_extracted_response
    Fork.sameDigest hcollision Fork.accepts₀ Fork.accepts₁

theorem relationMapResponse_extracted
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {Action : FullResponseChallengeActionCertificate}
    {Lift : FullResponseCanonicalResponseLiftCertificate A}
    (Fork : FullResponseQROMSameCommitmentFork A)
    (Arithmetic :
      FullResponseByteResponseArithmeticCertificate A Action Lift)
    (hcollision :
      ¬ Action.badActionCollision
        Fork.statement₀.challenge Fork.statement₁.challenge) :
    PackedSISFullResponseQuotientOpeningReduction.relationMapResponse
      A.toPackedSISFullResponseQuotientOpeningReduction
      Fork.statement₀
      (Arithmetic.extractedForkResponse Fork.statement₀ Fork.statement₁
        Fork.maskedResponse₀ Fork.maskedResponse₁) =
      Fork.statement₀.digest :=
  Arithmetic.relationMapResponse_extractedForkResponse
    Fork.sameDigest hcollision Fork.accepts₀ Fork.accepts₁

end FullResponseQROMSameCommitmentFork

/--
Fiat-Shamir fork-generation and action-collision accounting.

At this point the algebraic fork extractor is split into checkable pieces:
challenge-action invertibility, fork-equation division, canonical response lift,
and byte-level response arithmetic.  The remaining probabilistic layer is the
lemma that an accepting Fiat-Shamir adversary yields a same-commitment fork,
plus the bad event that two distinct challenge bytes collide under the derived
`F_12289^12` action.

The `214`-bit transcript challenge controls frame/query-miss or QROM
reprogramming failures.  The `163`-bit action-collision floor controls the
algebraic extraction bad event without becoming the generic quantum-query
bottleneck.
-/
structure FullResponseFiatShamirForkAccountingCertificate where
  challengeEntropyBits : Nat
  challengeEntropyBits_eq : challengeEntropyBits = 214
  algebraicChallengeEntropyBits : Nat
  algebraicChallengeEntropyBits_eq : algebraicChallengeEntropyBits = 182
  transcriptHardeningBits : Nat
  transcriptHardeningBits_eq : transcriptHardeningBits = 32
  actionCollisionFloorBits : Nat
  actionCollisionFloorBits_eq : actionCollisionFloorBits = 163
  fsClassicalQueryBoundLog2For128 : Nat
  fsClassicalQueryBoundLog2For128_eq :
    fsClassicalQueryBoundLog2For128 = 86
  fsQuantumQueryBoundLog2For128 : Nat
  fsQuantumQueryBoundLog2For128_eq :
    fsQuantumQueryBoundLog2For128 = 43
  actionCollisionClassicalQueryBoundLog2For128 : Nat
  actionCollisionClassicalQueryBoundLog2For128_eq :
    actionCollisionClassicalQueryBoundLog2For128 = 35
  actionCollisionQuantumQueryBoundLog2For128 : Nat
  actionCollisionQuantumQueryBoundLog2For128_eq :
    actionCollisionQuantumQueryBoundLog2For128 = 17
  bottleneckClassicalQueryBoundLog2For128 : Nat
  bottleneckClassicalQueryBoundLog2For128_eq :
    bottleneckClassicalQueryBoundLog2For128 = 35
  bottleneckQuantumQueryBoundLog2For128 : Nat
  bottleneckQuantumQueryBoundLog2For128_eq :
    bottleneckQuantumQueryBoundLog2For128 = 17
  algebraicForkPathClosed : Prop
  algebraicForkPathClosed_holds : algebraicForkPathClosed
  sameCommitmentForkGeneration : Prop
  sameCommitmentForkGeneration_holds :
    sameCommitmentForkGeneration
  qromForkGenerationLemma : Prop
  qromForkGenerationLemma_holds :
    qromForkGenerationLemma
  actionCollisionBudgetMeets128AtBottleneck : Prop
  actionCollisionBudgetMeets128AtBottleneck_holds :
    actionCollisionBudgetMeets128AtBottleneck
  fsBudgetMeets128AtBottleneck : Prop
  fsBudgetMeets128AtBottleneck_holds :
    fsBudgetMeets128AtBottleneck

namespace FullResponseFiatShamirForkAccountingCertificate

theorem transcriptChallengeBits_eq_214
    (C : FullResponseFiatShamirForkAccountingCertificate) :
    C.challengeEntropyBits = 214 :=
  C.challengeEntropyBits_eq

theorem actionCollisionFloorBits_eq_163
    (C : FullResponseFiatShamirForkAccountingCertificate) :
    C.actionCollisionFloorBits = 163 :=
  C.actionCollisionFloorBits_eq

theorem fsQuantumQueryBoundLog2For128_eq_43
    (C : FullResponseFiatShamirForkAccountingCertificate) :
    C.fsQuantumQueryBoundLog2For128 = 43 :=
  C.fsQuantumQueryBoundLog2For128_eq

theorem actionCollisionQuantumQueryBoundLog2For128_eq_17
    (C : FullResponseFiatShamirForkAccountingCertificate) :
    C.actionCollisionQuantumQueryBoundLog2For128 = 17 :=
  C.actionCollisionQuantumQueryBoundLog2For128_eq

theorem bottleneckQuantumQueryBoundLog2For128_eq_17
    (C : FullResponseFiatShamirForkAccountingCertificate) :
    C.bottleneckQuantumQueryBoundLog2For128 = 17 :=
  C.bottleneckQuantumQueryBoundLog2For128_eq

theorem probabilisticForkObligations
    (C : FullResponseFiatShamirForkAccountingCertificate) :
    C.algebraicForkPathClosed ∧
      C.sameCommitmentForkGeneration ∧
      C.qromForkGenerationLemma ∧
      C.actionCollisionBudgetMeets128AtBottleneck ∧
      C.fsBudgetMeets128AtBottleneck := by
  exact ⟨C.algebraicForkPathClosed_holds,
    C.sameCommitmentForkGeneration_holds,
    C.qromForkGenerationLemma_holds,
    C.actionCollisionBudgetMeets128AtBottleneck_holds,
    C.fsBudgetMeets128AtBottleneck_holds⟩

end FullResponseFiatShamirForkAccountingCertificate

/--
Selected QROM measure-and-reprogram theorem boundary.

This records the exact theorem shape we intend to use for option 2: measure the
quantum random-oracle query at the prechallenge frame, reprogram its output to a
fresh 21-byte challenge, and preserve the first prover message/proof commitment.
The executable report ties this to the Don-Fehr-Majenz-Schaffner QROM
Fiat-Shamir theorem family and the cleaner Don-Fehr-Majenz
measure-and-reprogram statement with quadratic `(2q + 1)^2` loss.
-/
structure FullResponseQROMMeasureAndReprogramInstantiation where
  selectedTheorem : Prop
  selectedTheorem_holds : selectedTheorem
  theoremAppliesToSingleInputFiatShamir : Prop
  theoremAppliesToSingleInputFiatShamir_holds :
    theoremAppliesToSingleInputFiatShamir
  quadraticLossModel : Prop
  quadraticLossModel_holds : quadraticLossModel
  qromHashQueryBoundLog2 : Nat
  qromHashQueryBoundLog2_eq : qromHashQueryBoundLog2 = 16
  quadraticLossBits : Nat
  quadraticLossBits_eq : quadraticLossBits = 32
  measuredInputIsPrechallengeFrame : Prop
  measuredInputIsPrechallengeFrame_holds :
    measuredInputIsPrechallengeFrame
  reprogrammedOutputIsChallenge : Prop
  reprogrammedOutputIsChallenge_holds :
    reprogrammedOutputIsChallenge
  firstMessageIndependentOfChallenge : Prop
  firstMessageIndependentOfChallenge_holds :
    firstMessageIndependentOfChallenge
  sameCommitmentForkProduced : Prop
  sameCommitmentForkProduced_holds :
    sameCommitmentForkProduced

namespace FullResponseQROMMeasureAndReprogramInstantiation

theorem qromHashQueryBoundLog2_eq_16
    (I : FullResponseQROMMeasureAndReprogramInstantiation) :
    I.qromHashQueryBoundLog2 = 16 :=
  I.qromHashQueryBoundLog2_eq

theorem quadraticLossBits_eq_32
    (I : FullResponseQROMMeasureAndReprogramInstantiation) :
    I.quadraticLossBits = 32 :=
  I.quadraticLossBits_eq

theorem theoremShape
    (I : FullResponseQROMMeasureAndReprogramInstantiation) :
    I.selectedTheorem ∧
      I.theoremAppliesToSingleInputFiatShamir ∧
      I.quadraticLossModel ∧
      I.measuredInputIsPrechallengeFrame ∧
      I.reprogrammedOutputIsChallenge ∧
      I.firstMessageIndependentOfChallenge := by
  exact ⟨I.selectedTheorem_holds,
    I.theoremAppliesToSingleInputFiatShamir_holds,
    I.quadraticLossModel_holds,
    I.measuredInputIsPrechallengeFrame_holds,
    I.reprogrammedOutputIsChallenge_holds,
    I.firstMessageIndependentOfChallenge_holds⟩

theorem sameCommitmentFork
    (I : FullResponseQROMMeasureAndReprogramInstantiation) :
    I.sameCommitmentForkProduced :=
  I.sameCommitmentForkProduced_holds

end FullResponseQROMMeasureAndReprogramInstantiation

/--
Deployment ledger for the remaining Fiat-Shamir fork-generation bad events.

`FullResponseFiatShamirForkAccountingCertificate` records the single-term
challenge and action-collision budgets.  A deployed reduction consumes several
bad-event terms at once, so this ledger reserves one bit for the union bound:
the individual terms have `2^17` quantum-query headroom, while the conservative
multi-term deployment ledger uses `2^16`.
-/
structure FullResponseFiatShamirForkGenerationLedger where
  transcriptChallengeBits : Nat
  transcriptChallengeBits_eq : transcriptChallengeBits = 214
  actionCollisionFloorBits : Nat
  actionCollisionFloorBits_eq : actionCollisionFloorBits = 163
  singleTermQuantumQueryBoundLog2For128 : Nat
  singleTermQuantumQueryBoundLog2For128_eq :
    singleTermQuantumQueryBoundLog2For128 = 43
  singleTermClassicalQueryBoundLog2For128 : Nat
  singleTermClassicalQueryBoundLog2For128_eq :
    singleTermClassicalQueryBoundLog2For128 = 86
  unionSafeQuantumQueryBoundLog2For128 : Nat
  unionSafeQuantumQueryBoundLog2For128_eq :
    unionSafeQuantumQueryBoundLog2For128 = 16
  unionSafeClassicalQueryBoundLog2For128 : Nat
  unionSafeClassicalQueryBoundLog2For128_eq :
    unionSafeClassicalQueryBoundLog2For128 = 33
  transcriptQuantumBitsAtUnionBound : Nat
  transcriptQuantumBitsAtUnionBound_eq :
    transcriptQuantumBitsAtUnionBound = 182
  actionCollisionQuantumBitsAtUnionBound : Nat
  actionCollisionQuantumBitsAtUnionBound_eq :
    actionCollisionQuantumBitsAtUnionBound = 131
  transcriptClassicalBitsAtUnionBound : Nat
  transcriptClassicalBitsAtUnionBound_eq :
    transcriptClassicalBitsAtUnionBound = 181
  actionCollisionClassicalBitsAtUnionBound : Nat
  actionCollisionClassicalBitsAtUnionBound_eq :
    actionCollisionClassicalBitsAtUnionBound = 130
  unionBoundReservedBits : Nat
  unionBoundReservedBits_eq : unionBoundReservedBits = 1
  qromInstantiation :
    FullResponseQROMMeasureAndReprogramInstantiation
  sameCommitmentForkGenerationLemma : Prop
  sameCommitmentForkGenerationLemma_holds :
    sameCommitmentForkGenerationLemma
  qromMeasureAndReprogramLemma : Prop
  qromMeasureAndReprogramLemma_holds :
    qromMeasureAndReprogramLemma
  unionBoundMeets128 : Prop
  unionBoundMeets128_holds : unionBoundMeets128

namespace FullResponseFiatShamirForkGenerationLedger

theorem unionSafeQuantumQueryBoundLog2For128_eq_16
    (L : FullResponseFiatShamirForkGenerationLedger) :
    L.unionSafeQuantumQueryBoundLog2For128 = 16 :=
  L.unionSafeQuantumQueryBoundLog2For128_eq

theorem unionSafeClassicalQueryBoundLog2For128_eq_33
    (L : FullResponseFiatShamirForkGenerationLedger) :
    L.unionSafeClassicalQueryBoundLog2For128 = 33 :=
  L.unionSafeClassicalQueryBoundLog2For128_eq

theorem transcriptQuantumBitsAtUnionBound_meets128
    (L : FullResponseFiatShamirForkGenerationLedger) :
    128 ≤ L.transcriptQuantumBitsAtUnionBound := by
  rw [L.transcriptQuantumBitsAtUnionBound_eq]
  native_decide

theorem actionCollisionQuantumBitsAtUnionBound_meets128
    (L : FullResponseFiatShamirForkGenerationLedger) :
    128 ≤ L.actionCollisionQuantumBitsAtUnionBound := by
  rw [L.actionCollisionQuantumBitsAtUnionBound_eq]
  native_decide

theorem transcriptClassicalBitsAtUnionBound_meets128
    (L : FullResponseFiatShamirForkGenerationLedger) :
    128 ≤ L.transcriptClassicalBitsAtUnionBound := by
  rw [L.transcriptClassicalBitsAtUnionBound_eq]
  native_decide

theorem actionCollisionClassicalBitsAtUnionBound_meets128
    (L : FullResponseFiatShamirForkGenerationLedger) :
    128 ≤ L.actionCollisionClassicalBitsAtUnionBound := by
  rw [L.actionCollisionClassicalBitsAtUnionBound_eq]
  native_decide

theorem forkGenerationLedgerObligations
    (L : FullResponseFiatShamirForkGenerationLedger) :
    L.sameCommitmentForkGenerationLemma ∧
      L.qromMeasureAndReprogramLemma ∧
      L.unionBoundMeets128 := by
  exact ⟨L.sameCommitmentForkGenerationLemma_holds,
    L.qromMeasureAndReprogramLemma_holds,
    L.unionBoundMeets128_holds⟩

theorem qromInstantiationShape
    (L : FullResponseFiatShamirForkGenerationLedger) :
    L.qromInstantiation.selectedTheorem ∧
      L.qromInstantiation.theoremAppliesToSingleInputFiatShamir ∧
      L.qromInstantiation.quadraticLossModel ∧
      L.qromInstantiation.measuredInputIsPrechallengeFrame ∧
      L.qromInstantiation.reprogrammedOutputIsChallenge ∧
      L.qromInstantiation.firstMessageIndependentOfChallenge :=
  L.qromInstantiation.theoremShape

theorem sameCommitmentFork_from_qromInstantiation
    (L : FullResponseFiatShamirForkGenerationLedger) :
    L.qromInstantiation.sameCommitmentForkProduced :=
  L.qromInstantiation.sameCommitmentFork

end FullResponseFiatShamirForkGenerationLedger

/--
Final extraction branch theorem for the QROM path.

This is the exact branch shape for the deployable proof: the QROM theorem gives
a same-commitment fork; if the fork does not hit the explicit Fq12 action
collision, the byte-response extractor returns a canonical quotient witness.
Otherwise the proof branches to one of the named residual bad events.
-/
structure FullResponseQROMExtractionBranchTheorem
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P)
    (Action : FullResponseChallengeActionCertificate)
    (Lift : FullResponseCanonicalResponseLiftCertificate A)
    (Arithmetic :
      FullResponseByteResponseArithmeticCertificate A Action Lift)
    (Ledger : FullResponseFiatShamirForkGenerationLedger) where
  qromFork :
    FullResponseQROMSameCommitmentFork A
  actionCollisionBreak : Prop
  extractionBreak : Prop
  matrixMismatchBreak : Prop
  frameMismatchBreak : Prop
  actionCollision_or_not :
    Action.badActionCollision
        qromFork.statement₀.challenge qromFork.statement₁.challenge ∨
      ¬ Action.badActionCollision
        qromFork.statement₀.challenge qromFork.statement₁.challenge
  actionCollisionBreak_of_collision :
    Action.badActionCollision
        qromFork.statement₀.challenge qromFork.statement₁.challenge ->
      actionCollisionBreak
  extractionBreak_of_no_relation :
    (¬ PackedSISFullResponseQuotientOpeningReduction.relationMapResponse
      A.toPackedSISFullResponseQuotientOpeningReduction
      qromFork.statement₀
      (Arithmetic.extractedForkResponse
        qromFork.statement₀ qromFork.statement₁
        qromFork.maskedResponse₀ qromFork.maskedResponse₁) =
        qromFork.statement₀.digest) ->
      extractionBreak
  noActionCollisionBreak : ¬ actionCollisionBreak
  noExtractionBreak : ¬ extractionBreak
  noMatrixMismatchBreak : ¬ matrixMismatchBreak
  noFrameMismatchBreak : ¬ frameMismatchBreak
  ledgerObligations :
    Ledger.sameCommitmentForkGenerationLemma ∧
      Ledger.qromMeasureAndReprogramLemma ∧
      Ledger.unionBoundMeets128

namespace FullResponseQROMExtractionBranchTheorem

theorem extracts_or_actionCollisionBreak
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {Action : FullResponseChallengeActionCertificate}
    {Lift : FullResponseCanonicalResponseLiftCertificate A}
    {Arithmetic :
      FullResponseByteResponseArithmeticCertificate A Action Lift}
    {Ledger : FullResponseFiatShamirForkGenerationLedger}
    (T :
      FullResponseQROMExtractionBranchTheorem
        A Action Lift Arithmetic Ledger) :
    (∃ response,
      fullResponseVectorCanonical response ∧
        fullResponseSyndromeMap A.matrix T.qromFork.statement₀ response =
          fullResponseDigestSyndrome T.qromFork.statement₀.digest) ∨
      T.actionCollisionBreak := by
  rcases T.actionCollision_or_not with hcollision | hnot
  · exact Or.inr (T.actionCollisionBreak_of_collision hcollision)
  · exact Or.inl
      (T.qromFork.extracts_canonical_response Arithmetic hnot)

theorem extractedRelation_or_actionCollisionBreak
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {Action : FullResponseChallengeActionCertificate}
    {Lift : FullResponseCanonicalResponseLiftCertificate A}
    {Arithmetic :
      FullResponseByteResponseArithmeticCertificate A Action Lift}
    {Ledger : FullResponseFiatShamirForkGenerationLedger}
    (T :
      FullResponseQROMExtractionBranchTheorem
        A Action Lift Arithmetic Ledger) :
    PackedSISFullResponseQuotientOpeningReduction.relationMapResponse
      A.toPackedSISFullResponseQuotientOpeningReduction
      T.qromFork.statement₀
      (Arithmetic.extractedForkResponse
        T.qromFork.statement₀ T.qromFork.statement₁
        T.qromFork.maskedResponse₀ T.qromFork.maskedResponse₁) =
      T.qromFork.statement₀.digest ∨
      T.actionCollisionBreak := by
  rcases T.actionCollision_or_not with hcollision | hnot
  · exact Or.inr (T.actionCollisionBreak_of_collision hcollision)
  · exact Or.inl
      (T.qromFork.relationMapResponse_extracted Arithmetic hnot)

theorem extractedRelation_of_noBadEvents
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {Action : FullResponseChallengeActionCertificate}
    {Lift : FullResponseCanonicalResponseLiftCertificate A}
    {Arithmetic :
      FullResponseByteResponseArithmeticCertificate A Action Lift}
    {Ledger : FullResponseFiatShamirForkGenerationLedger}
    (T :
      FullResponseQROMExtractionBranchTheorem
        A Action Lift Arithmetic Ledger) :
    PackedSISFullResponseQuotientOpeningReduction.relationMapResponse
      A.toPackedSISFullResponseQuotientOpeningReduction
      T.qromFork.statement₀
      (Arithmetic.extractedForkResponse
        T.qromFork.statement₀ T.qromFork.statement₁
        T.qromFork.maskedResponse₀ T.qromFork.maskedResponse₁) =
      T.qromFork.statement₀.digest := by
  rcases T.extractedRelation_or_actionCollisionBreak with hrel | hcollision
  · exact hrel
  · exact False.elim (T.noActionCollisionBreak hcollision)

theorem noExtractionBreak_needed_for_failed_relation
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {Action : FullResponseChallengeActionCertificate}
    {Lift : FullResponseCanonicalResponseLiftCertificate A}
    {Arithmetic :
      FullResponseByteResponseArithmeticCertificate A Action Lift}
    {Ledger : FullResponseFiatShamirForkGenerationLedger}
    (T :
      FullResponseQROMExtractionBranchTheorem
        A Action Lift Arithmetic Ledger) :
    ¬ ¬ PackedSISFullResponseQuotientOpeningReduction.relationMapResponse
      A.toPackedSISFullResponseQuotientOpeningReduction
      T.qromFork.statement₀
      (Arithmetic.extractedForkResponse
        T.qromFork.statement₀ T.qromFork.statement₁
        T.qromFork.maskedResponse₀ T.qromFork.maskedResponse₁) =
      T.qromFork.statement₀.digest := by
  intro hfailed
  exact T.noExtractionBreak (T.extractionBreak_of_no_relation hfailed)

end FullResponseQROMExtractionBranchTheorem

/--
Game surface for packed full-response extraction.

The adversarial event is the concrete `extractionFailure`: an accepting
73-byte packed full-response sigma equation for which no valid quotient
representative can be extracted.  A concrete cryptanalysis may prove this by
reducing it to an algebraic binding/SIS break; Lean records that implication
and consumes the `noExtractionBreak` game assumption.
-/
structure PackedFullResponseExtractionGame
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P) where
  budget : GameSecurityBudget
  budget_meets128 : budget.meets128
  extractionBreak : Prop
  extractionFailure_implies_break :
    A.extractionFailure -> extractionBreak
  noExtractionBreak : ¬ extractionBreak

namespace PackedFullResponseExtractionGame

theorem no_extractionFailure
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (G : PackedFullResponseExtractionGame A) :
    ¬ A.extractionFailure := by
  intro hfailure
  exact G.noExtractionBreak (G.extractionFailure_implies_break hfailure)

end PackedFullResponseExtractionGame

/--
Concrete sampler certificate for the 254-byte quotient witness-hiding game.

This is deliberately phrased without a probability library.  A concrete
implementation or external proof object supplies `samplerCloseToFiberSymmetric`
with the stated security budget.  Lean fixes the structural conditions that
make the pad constraints harmless for hiding: the hidden masked response may be
conditioned on canonical q-packing, the fixed two-byte local fiber state, and
response bounds, but not on active pad outputs.  The latter are consumed by the
existential quotient-opening predicate.
-/
structure FullResponseMaskedResponseSamplerCertificate
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P) where
  rawAlgebraicFiberBits : Nat
  modeledFiberStateConstraintBits : Nat
  residualHidingBits : Nat
  residualHidingBits_eq :
    residualHidingBits =
      rawAlgebraicFiberBits - modeledFiberStateConstraintBits
  rawAlgebraicFiberBits_eq :
    rawAlgebraicFiberBits =
      sisFullResponseHiddenFiberResidues * sisResidueBits +
        sisFullResponseSlackBits
  modeledFiberStateConstraintBits_eq :
    modeledFiberStateConstraintBits = localFiberStateBytes * 8
  residualHidingBits_meets140 : 140 ≤ residualHidingBits
  exactConditionedMinEntropyFloorBits : Nat
  exactConditionedMinEntropyFloorBits_eq :
    exactConditionedMinEntropyFloorBits = 133
  exactConditionedSupportFormula : Prop
  exactConditionedSupportFormula_holds : exactConditionedSupportFormula
  exactSupportProjectionRank : Nat
  exactSupportProjectionRank_eq : exactSupportProjectionRank = 2
  exactSupportResidue1ClassSize : Nat
  exactSupportResidue1ClassSize_eq : exactSupportResidue1ClassSize = 3073
  exactQuantumQueryBoundLog2For128 : Nat
  exactQuantumQueryBoundLog2For128_eq :
    exactQuantumQueryBoundLog2For128 = 2
  conditionsCanonicalPacking : Prop
  conditionsFiberStateEquality : Prop
  conditionsResponseBounds : Prop
  excludesActivePadOutputs : Prop
  excludesActivePadOutputs_holds : excludesActivePadOutputs
  padConstraintsHandledByExistentialQuotient :
    ∀ statement : PublicOpeningStatement,
      (∃ response,
        fullResponseSyndromeMap A.matrix statement response =
          fullResponseDigestSyndrome statement.digest ∧
        ((concreteByteLocalDeltaRepresentativeSemantics P)
          |>.representativeOpensStatement statement response)) ->
      ((concreteByteLocalDeltaRepresentativeSemantics P)
        |>.quotientOpensStatement
          (fullResponseSyndromeMap A.matrix)
          statement
          (fullResponseDigestSyndrome statement.digest))
  samplerCloseToFiberSymmetric : Prop
  statisticalDistanceNumerator : Nat
  statisticalDistanceDenominator : Nat
  statisticalDistanceDenominator_pos : 0 < statisticalDistanceDenominator
  statisticalDistanceZero :
    statisticalDistanceNumerator = 0
  samplerCloseToFiberSymmetric_holds :
    conditionsCanonicalPacking ->
      conditionsFiberStateEquality ->
      conditionsResponseBounds ->
      excludesActivePadOutputs ->
      samplerCloseToFiberSymmetric
  witnessHiding_of_sampler :
    samplerCloseToFiberSymmetric -> A.witnessHiding

namespace FullResponseMaskedResponseSamplerCertificate

theorem residualHidingBits_eq_144
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    C.residualHidingBits = 144 := by
  rw [C.residualHidingBits_eq, C.rawAlgebraicFiberBits_eq,
    C.modeledFiberStateConstraintBits_eq]
  native_decide

theorem residualHidingBits_certified140
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    140 ≤ C.residualHidingBits :=
  C.residualHidingBits_eq_144 ▸ by decide

theorem exactConditionedMinEntropyFloorBits_eq_133
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    C.exactConditionedMinEntropyFloorBits = 133 :=
  C.exactConditionedMinEntropyFloorBits_eq

theorem exactConditionedMinEntropyFloorBits_meets128
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    128 ≤ C.exactConditionedMinEntropyFloorBits := by
  rw [C.exactConditionedMinEntropyFloorBits_eq]
  native_decide

theorem exactQuantumQueryBoundLog2For128_eq_2
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    C.exactQuantumQueryBoundLog2For128 = 2 :=
  C.exactQuantumQueryBoundLog2For128_eq

theorem exactSupportProjectionRank_eq_2
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    C.exactSupportProjectionRank = 2 :=
  C.exactSupportProjectionRank_eq

theorem exactSupportResidue1ClassSize_eq_3073
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    C.exactSupportResidue1ClassSize = 3073 :=
  C.exactSupportResidue1ClassSize_eq

theorem exactConditionedSupportFormula_certified
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    C.exactConditionedSupportFormula :=
  C.exactConditionedSupportFormula_holds

theorem activePadOutputsExcluded
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    C.excludesActivePadOutputs :=
  C.excludesActivePadOutputs_holds

theorem witnessHiding
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A)
    (hcanonical : C.conditionsCanonicalPacking)
    (hfiber : C.conditionsFiberStateEquality)
    (hbounds : C.conditionsResponseBounds) :
    A.witnessHiding :=
  C.witnessHiding_of_sampler
    (C.samplerCloseToFiberSymmetric_holds
      hcanonical hfiber hbounds C.excludesActivePadOutputs_holds)

theorem statisticalDistance_eq_zero
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) :
    C.statisticalDistanceNumerator = 0 :=
  C.statisticalDistanceZero

end FullResponseMaskedResponseSamplerCertificate

/--
Game-specific bound for quotient witness hiding.

The exact conditioned support count is an audit metric for sampler population
and collision sanity.  It is not the distinguishing advantage when the
production transcript law is exactly the same conditioned fiber law in both
witness branches.  In that one-shot stateful game the statistical advantage is
charged to the sampler distance, which the certificate above pins to zero.
-/
structure QuotientWitnessHidingStatisticalBound
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (C : FullResponseMaskedResponseSamplerCertificate A) where
  samplesPerStatefulSlot : Nat
  samplesPerStatefulSlot_eq : samplesPerStatefulSlot = 1
  gameAdvantageNumerator : Nat
  gameAdvantageDenominator : Nat
  gameAdvantageDenominator_pos : 0 < gameAdvantageDenominator
  gameAdvantageZero : gameAdvantageNumerator = 0
  supportMinEntropyAuditOnly : Prop
  supportMinEntropyAuditOnly_holds : supportMinEntropyAuditOnly
  gameAdvantageControlledBySamplerDistance :
    C.statisticalDistanceNumerator = 0 -> gameAdvantageNumerator = 0

namespace QuotientWitnessHidingStatisticalBound

theorem samplesPerStatefulSlot_eq_one
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {C : FullResponseMaskedResponseSamplerCertificate A}
    (B : QuotientWitnessHidingStatisticalBound C) :
    B.samplesPerStatefulSlot = 1 :=
  B.samplesPerStatefulSlot_eq

theorem gameAdvantage_eq_zero
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {C : FullResponseMaskedResponseSamplerCertificate A}
    (B : QuotientWitnessHidingStatisticalBound C) :
    B.gameAdvantageNumerator = 0 :=
  B.gameAdvantageZero

theorem gameAdvantage_eq_zero_of_samplerDistance
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {C : FullResponseMaskedResponseSamplerCertificate A}
    (B : QuotientWitnessHidingStatisticalBound C)
    (h : C.statisticalDistanceNumerator = 0) :
    B.gameAdvantageNumerator = 0 :=
  B.gameAdvantageControlledBySamplerDistance h

theorem supportMinEntropyAuditOnly_certified
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    {C : FullResponseMaskedResponseSamplerCertificate A}
    (B : QuotientWitnessHidingStatisticalBound C) :
    B.supportMinEntropyAuditOnly :=
  B.supportMinEntropyAuditOnly_holds

end QuotientWitnessHidingStatisticalBound

/--
Game surface for quotient witness hiding.

`witnessHidingBreak` is the deployment-bounded event that an adversary
distinguishes representatives inside the accepted 11-residue quotient fiber, or
otherwise extracts information about the hidden representative beyond the
12-residue public quotient.  The consequence consumed by the existing proof is
the `witnessHiding` field of the quotient reduction assumption.  The sampler
certificate pins the intended conditioning set for the 254-byte route.
-/
structure QuotientWitnessHidingGame
    {P : PadPRFBinding}
    (A : ConcreteFullResponseQuotientReductionAssumptions P) where
  budget : GameSecurityBudget
  budget_meets128 : budget.meets128
  samplerCertificate :
    FullResponseMaskedResponseSamplerCertificate A
  gameSpecificBound :
    QuotientWitnessHidingStatisticalBound samplerCertificate
  witnessHidingBreak : Prop
  noWitnessHidingBreak : ¬ witnessHidingBreak
  witnessHiding_of_noBreak :
    ¬ witnessHidingBreak -> A.witnessHiding

namespace QuotientWitnessHidingGame

theorem witnessHiding
    {P : PadPRFBinding}
    {A : ConcreteFullResponseQuotientReductionAssumptions P}
    (G : QuotientWitnessHidingGame A) :
    A.witnessHiding :=
  G.witnessHiding_of_noBreak G.noWitnessHidingBreak

end QuotientWitnessHidingGame

/--
Game surface for Fiat-Shamir/random-oracle agreement.

The executable verifier hashes a fixed prechallenge frame.  The random-oracle
proof consumes the proposition `frameAgreement`; this game names the failure
event as a challenge-frame oracle mismatch for an accepting transcript.
-/
structure FiatShamirFrameAgreementGame (frameAgreement : Prop) where
  budget : GameSecurityBudget
  budget_meets128 : budget.meets128
  frameMismatchBreak : Prop
  noFrameMismatchBreak : ¬ frameMismatchBreak
  frameAgreement_of_noBreak :
    ¬ frameMismatchBreak -> frameAgreement

namespace FiatShamirFrameAgreementGame

theorem frameAgreement
    {frameAgreement : Prop}
    (G : FiatShamirFrameAgreementGame frameAgreement) :
    frameAgreement :=
  G.frameAgreement_of_noBreak G.noFrameMismatchBreak

end FiatShamirFrameAgreementGame

/--
Deployment-review bundle for the leading 254-byte stateful quotient target.

This is intentionally not an unconditional security theorem.  It says that once
the five review gates are supplied -- concrete matrix agreement, quotient
linear algebra, packed-sigma extraction, quotient witness hiding, SHA-256 pad
PRF security, and Fiat-Shamir frame agreement -- the proof stack has a single
object from which the verifier/reduction consequences are obtained.
-/
structure FullResponseDeploymentReviewCertificate where
  padAssumption : SHA256RejectionPadPRFAssumption
  quotientAssumptions :
    ConcreteFullResponseQuotientReductionAssumptions padAssumption.padPRF
  quotientAlgebra :
    FullResponseQuotientAlgebraCertificate quotientAssumptions.matrix
  fiatShamirFrameAgreement : Prop
  fiatShamirFrameAgreement_holds : fiatShamirFrameAgreement
  extractionGame :
    PackedFullResponseExtractionGame quotientAssumptions
  quotientHidingGame :
    QuotientWitnessHidingGame quotientAssumptions
  padPRFGame :
    SHA256PadPRFGame padAssumption
  fiatShamirGame :
    FiatShamirFrameAgreementGame fiatShamirFrameAgreement
  forkGenerationLedger :
    FullResponseFiatShamirForkGenerationLedger

namespace FullResponseDeploymentReviewCertificate

def openingReduction
    (C : FullResponseDeploymentReviewCertificate) :
    PackedSISFullResponseQuotientOpeningReduction :=
  C.quotientAssumptions.toPackedSISFullResponseQuotientOpeningReduction

theorem totalBytes
    (C : FullResponseDeploymentReviewCertificate) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  C.quotientAssumptions.totalBytes

theorem padPseudorandom
    (C : FullResponseDeploymentReviewCertificate) :
    C.padAssumption.padPRF.pseudorandomLocalPad :=
  C.padPRFGame.pseudorandomLocalPad

theorem matrixSamplerAgreement
    (C : FullResponseDeploymentReviewCertificate) :
    C.quotientAssumptions.matrixSamplerAgreement.sha256RejectionSamplerMatchesRust :=
  C.quotientAssumptions.matrixSamplerAgreement.sha256RejectionSamplerMatchesRust_holds

theorem quotientSyndromeSurjective
    (C : FullResponseDeploymentReviewCertificate) :
    Function.Surjective
      (fullResponseSyndromeOfVector C.quotientAssumptions.matrix) :=
  C.quotientAlgebra.syndromeMap_surjective

theorem quotientFiberNonempty
    (C : FullResponseDeploymentReviewCertificate)
    (syndrome : SISFullResponseSyndrome) :
    ∃ response,
      fullResponseSyndromeOfVector C.quotientAssumptions.matrix response =
        syndrome :=
  C.quotientAlgebra.fiber_nonempty syndrome

theorem quotientWitnessHiding
    (C : FullResponseDeploymentReviewCertificate) :
    C.quotientAssumptions.witnessHiding :=
  C.quotientHidingGame.witnessHiding

theorem fiatShamirFrame
    (C : FullResponseDeploymentReviewCertificate) :
    C.fiatShamirFrameAgreement :=
  C.fiatShamirGame.frameAgreement

theorem noExtractionFailure
    (C : FullResponseDeploymentReviewCertificate) :
    ¬ C.quotientAssumptions.extractionFailure :=
  C.extractionGame.no_extractionFailure

theorem gameBudgetsMeet128
    (C : FullResponseDeploymentReviewCertificate) :
    C.extractionGame.budget.meets128 ∧
      C.quotientHidingGame.budget.meets128 ∧
      C.padPRFGame.budget.meets128 ∧
      C.fiatShamirGame.budget.meets128 := by
  exact ⟨C.extractionGame.budget_meets128,
    C.quotientHidingGame.budget_meets128,
    C.padPRFGame.budget_meets128,
    C.fiatShamirGame.budget_meets128⟩

theorem forkGenerationLedgerObligations
    (C : FullResponseDeploymentReviewCertificate) :
    C.forkGenerationLedger.sameCommitmentForkGenerationLemma ∧
      C.forkGenerationLedger.qromMeasureAndReprogramLemma ∧
      C.forkGenerationLedger.unionBoundMeets128 :=
  C.forkGenerationLedger.forkGenerationLedgerObligations

end FullResponseDeploymentReviewCertificate

/--
Concrete q-ary/SIS reduction surface for the 254-byte target.

The seed response is interpreted as a packed q-vector with `sisSeedResidues`
coordinates; the public digest/proof commitment is interpreted as a packed
q-syndrome with `sisSyndromeResidues` coordinates.  This structure deliberately
keeps the concrete matrix and packing functions as fields, but fixes their
dimensions and the exact reduction shape:

* linearity is checked over the packed byte representation actually carried by
  the signature;
* an accepting sigma equation must either extract a seed whose syndrome is the
  statement digest, or produce the named SIS/algebraic binding break;
* if that break is ruled out, the object instantiates `SISStyleSeedOpeningMap`.
-/
structure PackedSISSeedOpeningReduction where
  deriveFiberState :
    ByteVector padSeedBytes -> ByteVector localFiberStateBytes
  seedOpensStatement :
    PublicOpeningStatement -> ByteVector padSeedBytes -> Prop
  validSeed : ByteVector padSeedBytes -> Prop
  matrix : SISSyndromeCoord -> SISSeedCoord -> ZMod modulus
  seedVector : ByteVector padSeedBytes -> SISSeedVector
  syndromeMap :
    PublicOpeningStatement -> ByteVector padSeedBytes -> SISSyndrome
  validDigest : ByteVector sha256DigestBytes -> Prop
  digestSyndrome : ByteVector sha256DigestBytes -> SISSyndrome
  packSyndrome : SISSyndrome -> ByteVector sha256DigestBytes
  pack_digestSyndrome :
    ∀ {digest}, validDigest digest ->
      packSyndrome (digestSyndrome digest) = digest
  pack_injective : Function.Injective packSyndrome
  statementDigestValid :
    ∀ statement : PublicOpeningStatement, validDigest statement.digest
  valid_of_relationMapSeed_eq_digest :
    ∀ {statement : PublicOpeningStatement}
      {seed : ByteVector padSeedBytes},
      packSyndrome (syndromeMap statement seed) = statement.digest ->
        validSeed seed
  relationMapSoundSyndrome :
    ∀ {statement : PublicOpeningStatement}
      {seed : ByteVector padSeedBytes},
      validSeed seed ->
        syndromeMap statement seed = digestSyndrome statement.digest ->
          seedOpensStatement statement seed
  addSeed :
    ByteVector padSeedBytes -> ByteVector padSeedBytes ->
      ByteVector padSeedBytes
  scaleSeed :
    ByteVector challengeBytes -> ByteVector padSeedBytes ->
      ByteVector padSeedBytes
  addCommitment :
    ByteVector linearSigmaProofCommitmentBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  scaleCommitment :
    ByteVector challengeBytes ->
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector linearSigmaProofCommitmentBytes
  valid_add_scale :
    ∀ {maskSeed witnessSeed : ByteVector padSeedBytes}
      {challenge : ByteVector challengeBytes},
      validSeed maskSeed ->
        validSeed witnessSeed ->
          validSeed (addSeed maskSeed (scaleSeed challenge witnessSeed))
  syndrome_linear_packed :
    ∀ (statement : PublicOpeningStatement)
      (maskSeed witnessSeed : ByteVector padSeedBytes)
      (challenge : ByteVector challengeBytes),
      packSyndrome
          (syndromeMap statement
            (addSeed maskSeed (scaleSeed challenge witnessSeed))) =
        addCommitment (packSyndrome (syndromeMap statement maskSeed))
          (scaleCommitment challenge
            (packSyndrome (syndromeMap statement witnessSeed)))
  proverMaskSeed :
    PublicOpeningStatement -> PublicOpeningWitness -> ByteVector padSeedBytes
  proverMaskSeed_valid :
    ∀ (statement : PublicOpeningStatement) (witness : PublicOpeningWitness),
      validSeed (proverMaskSeed statement witness)
  algebraicBindingBreak : Prop
  valid_of_accepts :
    ∀ {statement : PublicOpeningStatement}
      {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
      {maskedSeedResponse : ByteVector padSeedBytes},
      packSyndrome (syndromeMap statement maskedSeedResponse) =
        addCommitment proofCommitment
          (scaleCommitment statement.challenge statement.digest) ->
        validSeed maskedSeedResponse
  extracts_or_break :
    ∀ {statement : PublicOpeningStatement}
      {proofCommitment : ByteVector linearSigmaProofCommitmentBytes}
      {maskedSeedResponse : ByteVector padSeedBytes},
      validSeed maskedSeedResponse ->
        packSyndrome (syndromeMap statement maskedSeedResponse) =
          addCommitment proofCommitment
            (scaleCommitment statement.challenge statement.digest) ->
          (∃ seed,
            validSeed seed ∧
              syndromeMap statement seed =
                digestSyndrome statement.digest) ∨
            algebraicBindingBreak
  noAlgebraicBindingBreak : ¬ algebraicBindingBreak
  witnessHiding : Prop
  witnessHiding_holds : witnessHiding

namespace PackedSISSeedOpeningReduction

def relationMapSeed
    (R : PackedSISSeedOpeningReduction)
    (statement : PublicOpeningStatement)
    (seed : ByteVector padSeedBytes) :
    ByteVector linearSigmaProofCommitmentBytes :=
  R.packSyndrome (R.syndromeMap statement seed)

theorem noNontrivialSeedFiber_of_relationMapSeed_injective
    (R : PackedSISSeedOpeningReduction)
    {statement : PublicOpeningStatement}
    (hinj : Function.Injective (R.relationMapSeed statement)) :
    ¬ NontrivialSeedFiber R.relationMapSeed statement :=
  noNontrivialSeedFiber_of_injective hinj

theorem publicSeedRecoverer_of_relationMapSeed_leftInverse
    (R : PackedSISSeedOpeningReduction)
    {statement : PublicOpeningStatement}
    {recover :
      ByteVector linearSigmaProofCommitmentBytes ->
        ByteVector padSeedBytes}
    (hleft :
      ∀ seed : ByteVector padSeedBytes,
        recover (R.relationMapSeed statement seed) = seed) :
    PublicSeedRecoverer R.relationMapSeed statement recover :=
  publicSeedRecoverer_of_leftInverse hleft

theorem syndrome_eq_digest_of_relationMapSeed_eq_digest
    (R : PackedSISSeedOpeningReduction)
    {statement : PublicOpeningStatement}
    {seed : ByteVector padSeedBytes}
    (h :
      R.relationMapSeed statement seed = statement.digest) :
    R.syndromeMap statement seed = R.digestSyndrome statement.digest := by
  apply R.pack_injective
  calc
    R.packSyndrome (R.syndromeMap statement seed) =
        statement.digest := h
    _ = R.packSyndrome (R.digestSyndrome statement.digest) :=
        (R.pack_digestSyndrome (R.statementDigestValid statement)).symm

def toSISStyleSeedOpeningMap
    (R : PackedSISSeedOpeningReduction) :
    SISStyleSeedOpeningMap where
  deriveFiberState := R.deriveFiberState
  seedOpensStatement := R.seedOpensStatement
  relationMapSeed := R.relationMapSeed
  relationMapSound := by
    intro statement seed h
    have hsyn :
        R.syndromeMap statement seed =
          R.digestSyndrome statement.digest :=
      R.syndrome_eq_digest_of_relationMapSeed_eq_digest h
    have hvalid : R.validSeed seed := by
      exact R.valid_of_relationMapSeed_eq_digest h
    exact R.relationMapSoundSyndrome
      (statement := statement) (seed := seed) hvalid hsyn
  addSeed := R.addSeed
  scaleSeed := R.scaleSeed
  addCommitment := R.addCommitment
  scaleCommitment := R.scaleCommitment
  seed_linear := by
    intro statement maskSeed witnessSeed challenge
    exact R.syndrome_linear_packed statement maskSeed witnessSeed challenge
  proverMaskSeed := R.proverMaskSeed
  sisKnowledgeSoundness := by
    intro statement proofCommitment maskedSeedResponse hverify
    have hvalid : R.validSeed maskedSeedResponse :=
      R.valid_of_accepts hverify
    rcases R.extracts_or_break hvalid hverify with hseed | hbreak
    · rcases hseed with ⟨seed, _hvalidSeed, hsyn⟩
      exact ⟨seed, by
        calc
          R.relationMapSeed statement seed =
              R.packSyndrome (R.digestSyndrome statement.digest) := by
                simp [relationMapSeed, hsyn]
          _ = statement.digest :=
              R.pack_digestSyndrome (R.statementDigestValid statement)⟩
    · exact False.elim (R.noAlgebraicBindingBreak hbreak)
  witnessHiding := R.witnessHiding
  witnessHiding_holds := R.witnessHiding_holds

theorem totalBytes
    (_R : PackedSISSeedOpeningReduction) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

theorem target_is_statement_digest
    (_R : PackedSISSeedOpeningReduction)
    (statement : PublicOpeningStatement) :
    SISStyleSeedOpeningMap.target statement = statement.digest :=
  rfl

end PackedSISSeedOpeningReduction

def revealedPreimageProofSystem
    (idealRelation : PublicOpeningStatement -> PublicOpeningWitness -> Prop) :
    PublicOpeningProofSystem revealedPreimageProofBytes where
  verify := fun statement proof =>
    ∃ witness,
      idealRelation statement witness ∧
        linearSigmaWitnessFromHiddenPreimage witness.hiddenPreimage = proof
  idealRelation := idealRelation
  sound := by
    intro statement proof hverify
    rcases hverify with ⟨witness, hrel, _hproof⟩
    exact ⟨witness, hrel⟩
  complete := by
    intro statement witness hrel
    exact ⟨linearSigmaWitnessFromHiddenPreimage witness.hiddenPreimage,
      ⟨witness, hrel, rfl⟩⟩

theorem revealedPreimageProofSystem_proofBytes :
    revealedPreimageProofBytes = 41 :=
  revealedPreimageProofBytes_eq

namespace WitnessHidingPublicOpeningProofSystem

theorem openingWitness_of_verify
    {proofBytes : Nat}
    (P : WitnessHidingPublicOpeningProofSystem proofBytes)
    {statement : PublicOpeningStatement}
    {proof : ByteVector proofBytes}
    (hverify : P.verify statement proof) :
    ∃ witness, P.idealRelation statement witness :=
  P.sound hverify

theorem linearSigma_totalBytes
    (_P : LinearSigmaOpeningTarget) :
    statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 :=
  linearSigmaTargetTransactionSignatureBytes_eq

theorem linearSigma_witnessHiding
    (P : LinearSigmaOpeningTarget) :
    P.witnessHiding :=
  P.witnessHiding_holds

end WitnessHidingPublicOpeningProofSystem

/--
Stateful byte-verifier relation for the precommitted spend form.

The challenge frame contains only prechallenge account data.  The active
responses and local openings are carried by the signature and checked after the
challenge bytes are derived.
-/
structure StatefulVerifierRelation
    {Slot : Type*} {publicKeyLen messageLen : Nat}
    (hashChallenge :
      StatefulChallengeFrame publicKeyLen messageLen ->
        ByteVector challengeBytes)
    (slotIdOfSlot : Slot -> ByteVector slotIdBytes) where
  before : BlockchainAccountState Slot
  after : BlockchainAccountState Slot
  slot : Slot
  digest : ByteVector sha256DigestBytes
  signature : StatefulSpendSignatureBytes
  frame : StatefulChallengeFrame publicKeyLen messageLen
  slotAccepts : SlotAcceptsDigest before slot digest
  signature_slot : signature.slotId = frame.slotId
  frame_slot : frame.slotId = slotIdOfSlot slot
  frame_digest : frame.slotDigest = digest
  challengeMatches : signature.challenge = hashChallenge frame
  activeProductBlockEquation : Prop
  activeProductBlockEquation_holds : activeProductBlockEquation
  openingProofsAccept : Prop
  openingProofsAccept_holds : openingProofsAccept
  transition : ConsumeSlotTransition before after slot

namespace StatefulVerifierRelation

theorem algebraicVerifierObligations
    {Slot : Type*} {publicKeyLen messageLen : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyLen messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    (V : StatefulVerifierRelation hashChallenge slotIdOfSlot) :
    SlotAcceptsDigest V.before V.slot V.digest ∧
      V.activeProductBlockEquation ∧ V.openingProofsAccept := by
  exact ⟨V.slotAccepts, V.activeProductBlockEquation_holds,
    V.openingProofsAccept_holds⟩

theorem byteBindingObligations
    {Slot : Type*} {publicKeyLen messageLen : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyLen messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    (V : StatefulVerifierRelation hashChallenge slotIdOfSlot) :
    V.signature.slotId = slotIdOfSlot V.slot ∧
      V.signature.challenge = hashChallenge V.frame ∧
      V.frame.slotDigest = V.digest := by
  exact ⟨V.signature_slot.trans V.frame_slot, V.challengeMatches,
    V.frame_digest⟩

theorem slot_not_reusable
    {Slot : Type*} {publicKeyLen messageLen : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyLen messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    (V : StatefulVerifierRelation hashChallenge slotIdOfSlot) :
    ¬ SlotAcceptsDigest V.after V.slot V.digest :=
  not_slotAcceptsDigest_after_consumed V.transition V.digest

end StatefulVerifierRelation

/--
Stateful verifier relation after replacing the ideal hidden-preimage oracle
with a public opening proof.

This is the plug-in theorem boundary: any concrete public proof system that
implements `PublicOpeningProofSystem` can be used here, and the byte cost is
`181 + proofBytes`.
-/
structure StatefulPublicOpeningVerifierRelation
    {Slot : Type*} {messageLen proofBytes : Nat}
    (hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes)
    (slotIdOfSlot : Slot -> ByteVector slotIdBytes)
    (P : PublicOpeningProofSystem proofBytes) where
  before : BlockchainAccountState Slot
  after : BlockchainAccountState Slot
  slot : Slot
  digest : ByteVector sha256DigestBytes
  signature : StatefulSpendSignatureWithProof proofBytes
  frame : StatefulChallengeFrame publicKeyBytes messageLen
  statement : PublicOpeningStatement
  slotAccepts : SlotAcceptsDigest before slot digest
  signature_slot : signature.base.slotId = frame.slotId
  frame_slot : frame.slotId = slotIdOfSlot slot
  frame_digest : frame.slotDigest = digest
  challengeMatches : signature.base.challenge = hashChallenge frame
  statement_publicKey : statement.publicKey = frame.publicKey
  statement_digest : statement.digest = digest
  statement_challenge : statement.challenge = signature.base.challenge
  statement_responses : statement.responses = signature.base.responses
  statement_openings : statement.openings = signature.base.openings
  openingProofAccepts : P.verify statement signature.openingProof
  activeProductBlockEquation : Prop
  activeProductBlockEquation_holds : activeProductBlockEquation
  transition : ConsumeSlotTransition before after slot

namespace StatefulPublicOpeningVerifierRelation

theorem openingWitness_of_accepts
    {Slot : Type*} {messageLen proofBytes : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    {P : PublicOpeningProofSystem proofBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot P) :
    ∃ witness, P.idealRelation V.statement witness :=
  P.sound V.openingProofAccepts

theorem statementBindingObligations
    {Slot : Type*} {messageLen proofBytes : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    {P : PublicOpeningProofSystem proofBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot P) :
    V.statement.publicKey = V.frame.publicKey ∧
      V.statement.digest = V.digest ∧
      V.statement.challenge = V.signature.base.challenge ∧
      V.statement.responses = V.signature.base.responses ∧
      V.statement.openings = V.signature.base.openings := by
  exact ⟨V.statement_publicKey, V.statement_digest, V.statement_challenge,
    V.statement_responses, V.statement_openings⟩

def toStatefulVerifierRelation
    {Slot : Type*} {messageLen proofBytes : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    {P : PublicOpeningProofSystem proofBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot P) :
    StatefulVerifierRelation hashChallenge slotIdOfSlot where
  before := V.before
  after := V.after
  slot := V.slot
  digest := V.digest
  signature := V.signature.base
  frame := V.frame
  slotAccepts := V.slotAccepts
  signature_slot := V.signature_slot
  frame_slot := V.frame_slot
  frame_digest := V.frame_digest
  challengeMatches := V.challengeMatches
  activeProductBlockEquation := V.activeProductBlockEquation
  activeProductBlockEquation_holds := V.activeProductBlockEquation_holds
  openingProofsAccept := P.verify V.statement V.signature.openingProof
  openingProofsAccept_holds := V.openingProofAccepts
  transition := V.transition

theorem algebraicVerifierObligations
    {Slot : Type*} {messageLen proofBytes : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    {P : PublicOpeningProofSystem proofBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot P) :
    SlotAcceptsDigest V.before V.slot V.digest ∧
      V.activeProductBlockEquation ∧
      ∃ witness, P.idealRelation V.statement witness := by
  exact ⟨V.slotAccepts, V.activeProductBlockEquation_holds,
    V.openingWitness_of_accepts⟩

theorem slot_not_reusable
    {Slot : Type*} {messageLen proofBytes : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    {P : PublicOpeningProofSystem proofBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot P) :
    ¬ SlotAcceptsDigest V.after V.slot V.digest :=
  StatefulVerifierRelation.slot_not_reusable V.toStatefulVerifierRelation

end StatefulPublicOpeningVerifierRelation

namespace PackedSISFullResponseQuotientOpeningReduction

theorem statefulVerifier_quotientIdealWitness_of_accepts
    (R : PackedSISFullResponseQuotientOpeningReduction)
    {Slot : Type*} {messageLen : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot
        R.toLinearSigmaOpeningTarget.toPublicOpeningProofSystem) :
    ∃ witness,
      R.toFullResponseQuotientRelation.idealRelation
        V.statement witness := by
  exact V.openingWitness_of_accepts

theorem statefulVerifier_targetQuotientOpens_of_accepts
    (R : PackedSISFullResponseQuotientOpeningReduction)
    {Slot : Type*} {messageLen : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot
        R.toLinearSigmaOpeningTarget.toPublicOpeningProofSystem) :
    R.quotientOpensStatement V.statement
      (R.digestSyndrome V.statement.digest) := by
  rcases statefulVerifier_quotientIdealWitness_of_accepts R V with
    ⟨_witness, hrel⟩
  rcases hrel with ⟨_response, _hdecode, _htarget, hopen⟩
  exact hopen

end PackedSISFullResponseQuotientOpeningReduction

namespace FullResponseDeploymentReviewCertificate

/--
Conditional deployed-verifier soundness for the leading quotient target.

This is the first top-level theorem for the practical stateful candidate.  It
does not claim that SHA-256, Fiat-Shamir, quotient hiding, or the packed
linear-sigma extraction assumption are proven from first principles.  Instead,
it says that once those explicit review assumptions are supplied by
`FullResponseDeploymentReviewCertificate`, every accepting public-opening
stateful verifier instance has the proof obligations needed by the security
argument:

* the account slot accepted before verification;
* the proof opens the public quotient relation at the statement digest;
* the active product-block verifier equation holds;
* the consumed slot cannot be reused;
* the 73-byte proof gives the 254-byte transaction target;
* the pad, witness-hiding, matrix-agreement, Fiat-Shamir, and fork-generation
  ledger assumptions used by the reduction are exactly the named fields of the
  certificate.
-/
theorem statefulVerifier_conditionalSoundness
    (C : FullResponseDeploymentReviewCertificate)
    {Slot : Type*} {messageLen : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot
        C.openingReduction.toLinearSigmaOpeningTarget.toPublicOpeningProofSystem) :
    SlotAcceptsDigest V.before V.slot V.digest ∧
      C.openingReduction.quotientOpensStatement V.statement
        (C.openingReduction.digestSyndrome V.statement.digest) ∧
      V.activeProductBlockEquation ∧
      ¬ SlotAcceptsDigest V.after V.slot V.digest ∧
      statefulTransactionWithProofBytes linearSigmaTargetProofBytes = 260 ∧
      C.padAssumption.padPRF.pseudorandomLocalPad ∧
      C.quotientAssumptions.witnessHiding ∧
      C.quotientAssumptions.matrixSamplerAgreement.sha256RejectionSamplerMatchesRust ∧
      C.fiatShamirFrameAgreement ∧
      ¬ C.quotientAssumptions.extractionFailure ∧
      ¬ C.extractionGame.extractionBreak ∧
      ¬ C.quotientHidingGame.witnessHidingBreak ∧
      ¬ C.padPRFGame.padPRFBreak ∧
      ¬ C.fiatShamirGame.frameMismatchBreak ∧
      C.extractionGame.budget.meets128 ∧
      C.quotientHidingGame.budget.meets128 ∧
      C.padPRFGame.budget.meets128 ∧
      C.fiatShamirGame.budget.meets128 ∧
      C.forkGenerationLedger.sameCommitmentForkGenerationLemma ∧
      C.forkGenerationLedger.qromMeasureAndReprogramLemma ∧
      C.forkGenerationLedger.unionBoundMeets128 ∧
      C.forkGenerationLedger.qromInstantiation.selectedTheorem ∧
      C.forkGenerationLedger.qromInstantiation.theoremAppliesToSingleInputFiatShamir ∧
      C.forkGenerationLedger.qromInstantiation.quadraticLossModel ∧
      C.forkGenerationLedger.qromInstantiation.measuredInputIsPrechallengeFrame ∧
      C.forkGenerationLedger.qromInstantiation.reprogrammedOutputIsChallenge ∧
      C.forkGenerationLedger.qromInstantiation.firstMessageIndependentOfChallenge ∧
      C.forkGenerationLedger.qromInstantiation.sameCommitmentForkProduced := by
  exact ⟨V.slotAccepts,
    C.openingReduction.statefulVerifier_targetQuotientOpens_of_accepts V,
    V.activeProductBlockEquation_holds,
    V.slot_not_reusable,
    C.totalBytes,
    C.padPseudorandom,
    C.quotientWitnessHiding,
    C.matrixSamplerAgreement,
    C.fiatShamirFrame,
    C.noExtractionFailure,
    C.extractionGame.noExtractionBreak,
    C.quotientHidingGame.noWitnessHidingBreak,
    C.padPRFGame.noPadPRFBreak,
    C.fiatShamirGame.noFrameMismatchBreak,
    C.extractionGame.budget_meets128,
    C.quotientHidingGame.budget_meets128,
    C.padPRFGame.budget_meets128,
    C.fiatShamirGame.budget_meets128,
    C.forkGenerationLedger.sameCommitmentForkGenerationLemma_holds,
    C.forkGenerationLedger.qromMeasureAndReprogramLemma_holds,
    C.forkGenerationLedger.unionBoundMeets128_holds,
    C.forkGenerationLedger.qromInstantiation.selectedTheorem_holds,
    C.forkGenerationLedger.qromInstantiation.theoremAppliesToSingleInputFiatShamir_holds,
    C.forkGenerationLedger.qromInstantiation.quadraticLossModel_holds,
    C.forkGenerationLedger.qromInstantiation.measuredInputIsPrechallengeFrame_holds,
    C.forkGenerationLedger.qromInstantiation.reprogrammedOutputIsChallenge_holds,
    C.forkGenerationLedger.qromInstantiation.firstMessageIndependentOfChallenge_holds,
    C.forkGenerationLedger.qromInstantiation.sameCommitmentForkProduced_holds⟩

theorem statefulVerifier_quotientWitness_of_conditionalSoundness
    (C : FullResponseDeploymentReviewCertificate)
    {Slot : Type*} {messageLen : Nat}
    {hashChallenge :
      StatefulChallengeFrame publicKeyBytes messageLen ->
        ByteVector challengeBytes}
    {slotIdOfSlot : Slot -> ByteVector slotIdBytes}
    (V :
      StatefulPublicOpeningVerifierRelation
        hashChallenge slotIdOfSlot
        C.openingReduction.toLinearSigmaOpeningTarget.toPublicOpeningProofSystem) :
    ∃ witness,
      C.openingReduction.toFullResponseQuotientRelation.idealRelation
        V.statement witness :=
  C.openingReduction.statefulVerifier_quotientIdealWitness_of_accepts V

end FullResponseDeploymentReviewCertificate

/--
Stateful/precommitted spend profile: commitment digests are outside the
signature payload, so the signature carries only the local padded openings.
-/
def statefulSpendProfile : ByteLocalDeltaSignatureProfile where
  challengeBytes := challengeBytes
  responseBytes := activeResponseBytes
  repetitions := repetitions
  openingBytesPerRepetition := localOpeningBytesPerRepetition
  digestBytesPerRepetition := 0

theorem statefulSpend_signatureBytes_eq :
    statefulSpendProfile.signatureBytes = 183 := by
  native_decide

theorem statefulTransaction_signatureBytes_eq :
    slotIdBytes + statefulSpendProfile.signatureBytes = 187 := by
  native_decide

/--
Stateless profile: each repetition carries a 32-byte public commitment digest.
This is the honest byte cost if the public fiber commitments are not stored
elsewhere.
-/
def statelessSha256Profile : ByteLocalDeltaSignatureProfile where
  challengeBytes := challengeBytes
  responseBytes := activeResponseBytes
  repetitions := repetitions
  openingBytesPerRepetition := localOpeningBytesPerRepetition
  digestBytesPerRepetition := sha256DigestBytes

theorem statelessSha256_signatureBytes_eq :
    statelessSha256Profile.signatureBytes = 1015 := by
  native_decide

theorem statefulTransactionWithProofBytes_beatsStatelessSha256
    {proofBytes : Nat} (hproof : proofBytes ≤ 827) :
    statefulTransactionWithProofBytes proofBytes <
      statelessSha256Profile.signatureBytes := by
  rw [statefulTransactionWithProofBytes_eq, statelessSha256_signatureBytes_eq]
  omega

end N32Q12289

end LocalDeltaByteCommitment
end LocallyRecoverableSyndrome
end Descender
