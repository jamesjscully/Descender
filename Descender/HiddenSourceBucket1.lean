import Descender.ResponseLinkExactCertificate

namespace Descender
namespace HiddenSourceBucket1

/-!
Deterministic proof obligations for the selected hidden-source deployment
surface.

This module starts with the byte-level canonical q12289 lane decoder used by
the 56-byte fallback projection carrier.  The Rust production reference
decodes the first 24 bytes as 12 little-endian q12289 residues, then treats the
next 3 bytes as check bytes and the final 29 bytes as transcript binding.
-/

abbrev q : Nat := 12289
abbrev slotIdBytes : Nat := 4
abbrev challengeBytes : Nat := 21
abbrev algebraicChallengeBytes : Nat := 17
abbrev algebraicChallengeLastByteMask : Nat := 3
abbrev responseBound : Nat := 48
abbrev localFree : Nat := 2
abbrev reps : Nat := 26
abbrev activeResponseBytes : Nat := 52
abbrev localOpeningBytesPerRep : Nat := 4
abbrev localOpeningBytes : Nat := 104
abbrev linearSigmaProofCommitmentBytes : Nat := 32
abbrev reviewedFq12MaskedResponseBytes : Nat := 39
abbrev reviewedFq12ProofBytes : Nat := 71
abbrev reviewedFq12TransactionBytes : Nat := 252
abbrev fieldBits : Nat := 14
abbrev reviewedFq12ResponseResidues : Nat := 22
abbrev reviewedFq12MaskedResponseBits : Nat := 308
abbrev reviewedFq12MaskedResponseSlackBits : Nat := 4
abbrev sisFullResponseResidues : Nat := 23
abbrev fallbackProjectionCarrierBytes : Nat := 56
abbrev fallbackProjectionLaneCount : Nat := 12
abbrev fallbackAccountProjectionLaneCount : Nat := 6
abbrev fallbackPadProjectionLaneCount : Nat := 6
abbrev fallbackProjectionCheckBytes : Nat := 3
abbrev fallbackProjectionTranscriptBindingBytes : Nat := 29
abbrev padSeedBytes : Nat := 32

def byteAt (bytes : List Nat) (idx : Nat) : Nat :=
  match bytes.drop idx with
  | byte :: _ => byte
  | [] => 0

def bytesWithinUInt8 : List Nat -> Bool
  | [] => true
  | byte :: rest => byte <= 255 && bytesWithinUInt8 rest

def decodePadSeed? (seedBytes : List Nat) : Option (List Nat) :=
  if seedBytes.length == padSeedBytes && bytesWithinUInt8 seedBytes then
    some seedBytes
  else
    none

def le16At (bytes : List Nat) (offset : Nat) : Nat :=
  byteAt bytes offset + 256 * byteAt bytes (offset + 1)

def le32At (bytes : List Nat) (offset : Nat) : Nat :=
  byteAt bytes offset +
    256 * byteAt bytes (offset + 1) +
    65536 * byteAt bytes (offset + 2) +
    16777216 * byteAt bytes (offset + 3)

def le16Bytes (value : Nat) : List Nat :=
  [value % 256, (value / 256) % 256]

def le32Bytes (value : Nat) : List Nat :=
  [ value % 256
  , (value / 256) % 256
  , (value / 65536) % 256
  , (value / 16777216) % 256
  ]

def readBitsLE (bytes : List Nat) (start width : Nat) : Nat :=
  (List.range width).foldl
    (fun acc bit =>
      if Nat.testBit (byteAt bytes ((start + bit) / 8)) ((start + bit) % 8) then
        acc + 2 ^ bit
      else
        acc)
    0

def anyBitsSetLE (bytes : List Nat) (start width : Nat) : Bool :=
  (List.range width).any fun bit =>
    Nat.testBit (byteAt bytes ((start + bit) / 8)) ((start + bit) % 8)

def optionSequenceNat : List (Option Nat) -> Option (List Nat)
  | [] => some []
  | some x :: xs =>
      match optionSequenceNat xs with
      | some rest => some (x :: rest)
      | none => none
  | none :: _ => none

def listSubModQ (lhs rhs : List Nat) : List Nat :=
  (lhs.zip rhs).map fun pair => (pair.1 + q - pair.2) % q

def mulModQ (lhs rhs : Nat) : Nat :=
  (lhs * rhs) % q

def natAt (values : List Nat) (idx : Nat) : Nat :=
  match values.drop idx with
  | value :: _ => value
  | [] => 0

def decodeLane? (bytes : List Nat) (lane : Nat) : Option Nat :=
  let value := le16At bytes (2 * lane)
  if value < q then some value else none

def decodeLanes? (bytes : List Nat) : Option (List Nat) :=
  optionSequenceNat <|
    (List.range fallbackProjectionLaneCount).map fun lane => decodeLane? bytes lane

structure FallbackProjectionCarrierDecoded where
  lanes : List Nat
  checkBytes : List Nat
  transcriptBinding : List Nat
deriving DecidableEq, Repr

structure FallbackProjectionPublicBridgeStatement where
  projectionValues : List Nat
  checkBytes : List Nat
  transcriptBinding : List Nat
deriving DecidableEq, Repr

def decodeFallbackProjectionCarrier? (bytes : List Nat) :
    Option FallbackProjectionCarrierDecoded :=
  if bytes.length == fallbackProjectionCarrierBytes && bytesWithinUInt8 bytes then
    match decodeLanes? bytes with
    | some lanes =>
        some {
          lanes := lanes
          checkBytes := (bytes.drop 24).take fallbackProjectionCheckBytes
          transcriptBinding :=
            (bytes.drop 27).take fallbackProjectionTranscriptBindingBytes
        }
    | none => none
  else
    none

def verifyFallbackProjectionPublicBridge?
    (carrier : FallbackProjectionCarrierDecoded)
    (statement : FallbackProjectionPublicBridgeStatement) : Option Unit :=
  if statement.projectionValues.length == fallbackProjectionLaneCount &&
      statement.projectionValues.all (fun value => value < q) &&
      carrier.lanes == statement.projectionValues &&
      carrier.checkBytes == statement.checkBytes &&
      carrier.transcriptBinding == statement.transcriptBinding then
    some ()
  else
    none

def nonzeroFallbackProjectionCarrierBytes : List Nat :=
  [0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0,
   6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0,
   12, 13, 14] ++ List.replicate 29 15

def nonzeroFallbackProjectionStatement : FallbackProjectionPublicBridgeStatement :=
  {
    projectionValues := List.range 12
    checkBytes := [12, 13, 14]
    transcriptBinding := List.replicate 29 15
  }

theorem fallback_projection_carrier_byte_accounting :
    fallbackProjectionLaneCount * 2 +
      fallbackProjectionCheckBytes +
      fallbackProjectionTranscriptBindingBytes =
      fallbackProjectionCarrierBytes := by
  native_decide

theorem decode_zero_fallback_projection_carrier :
    decodeFallbackProjectionCarrier? (List.replicate 56 0) =
      some {
        lanes := List.replicate 12 0
        checkBytes := List.replicate 3 0
        transcriptBinding := List.replicate 29 0
      } := by
  native_decide

theorem decode_rejects_short_fallback_projection_carrier :
    decodeFallbackProjectionCarrier? (List.replicate 55 0) = none := by
  native_decide

theorem decode_rejects_non_uint8_fallback_projection_carrier :
    decodeFallbackProjectionCarrier? (256 :: List.replicate 55 0) = none := by
  native_decide

theorem decode_rejects_noncanonical_q12289_lane_zero :
    decodeFallbackProjectionCarrier? (1 :: 48 :: List.replicate 54 0) = none := by
  native_decide

theorem decode_nonzero_fallback_projection_carrier :
    decodeFallbackProjectionCarrier? nonzeroFallbackProjectionCarrierBytes =
      some {
        lanes := List.range 12
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      } := by
  native_decide

theorem fallback_projection_public_bridge_accepts_matching_vector :
    verifyFallbackProjectionPublicBridge?
      {
        lanes := List.range 12
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      nonzeroFallbackProjectionStatement = some () := by
  native_decide

theorem fallback_projection_public_bridge_rejects_lane_mismatch :
    verifyFallbackProjectionPublicBridge?
      {
        lanes := List.range 12
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      { nonzeroFallbackProjectionStatement with
        projectionValues := 99 :: List.range 11 } = none := by
  native_decide

theorem fallback_projection_public_bridge_rejects_check_mismatch :
    verifyFallbackProjectionPublicBridge?
      {
        lanes := List.range 12
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      { nonzeroFallbackProjectionStatement with checkBytes := [99, 13, 14] } = none := by
  native_decide

theorem fallback_projection_public_bridge_rejects_transcript_binding_mismatch :
    verifyFallbackProjectionPublicBridge?
      {
        lanes := List.range 12
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      { nonzeroFallbackProjectionStatement with
        transcriptBinding := 99 :: List.replicate 28 15 } = none := by
  native_decide

theorem fallback_projection_public_bridge_rejects_noncanonical_expected_lane :
    verifyFallbackProjectionPublicBridge?
      {
        lanes := List.range 12
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      { nonzeroFallbackProjectionStatement with
        projectionValues := q :: List.range 11 } = none := by
  native_decide

def checkedCoreTranscriptFrameDomainBytes : List Nat :=
  [68, 101, 115, 99, 101, 110, 100, 101, 114, 46, 104, 105, 100,
   100, 101, 110, 95, 115, 111, 117, 114, 99, 101, 46, 102, 97,
   108, 108, 98, 97, 99, 107, 95, 112, 114, 111, 106, 101, 99,
   116, 105, 111, 110, 46, 99, 104, 101, 99, 107, 101, 100, 95,
   99, 111, 114, 101, 95, 114, 101, 115, 112, 111, 110, 115,
   101, 95, 116, 114, 97, 110, 115, 99, 114, 105, 112, 116, 46,
   118, 49]

def checkedCoreTranscriptFrameProfileBytes : List Nat :=
  [110, 51, 50, 46, 113, 49, 50, 50, 56, 57, 46, 114, 101, 115,
   112, 111, 110, 115, 101, 95, 108, 105, 110, 107, 46, 98, 57,
   54, 46, 118, 49]

def checkedCoreTranscriptFrameBytes : Nat :=
  checkedCoreTranscriptFrameDomainBytes.length +
    checkedCoreTranscriptFrameProfileBytes.length +
    sisFullResponseResidues * 2

def encodeCheckedCoreTranscriptFrame? (response : List Nat) : Option (List Nat) :=
  if response.length == sisFullResponseResidues &&
      response.all (fun value => value < q) then
    some <|
      checkedCoreTranscriptFrameDomainBytes ++
      checkedCoreTranscriptFrameProfileBytes ++
      response.flatMap le16Bytes
  else
    none

def decodeCheckedCoreTranscriptFrame? (frame : List Nat) : Option (List Nat) :=
  if frame.length == checkedCoreTranscriptFrameBytes && bytesWithinUInt8 frame &&
      frame.take checkedCoreTranscriptFrameDomainBytes.length ==
        checkedCoreTranscriptFrameDomainBytes &&
      (frame.drop checkedCoreTranscriptFrameDomainBytes.length).take
        checkedCoreTranscriptFrameProfileBytes.length ==
        checkedCoreTranscriptFrameProfileBytes then
    let offset :=
      checkedCoreTranscriptFrameDomainBytes.length +
      checkedCoreTranscriptFrameProfileBytes.length
    let response := (List.range sisFullResponseResidues).map fun idx =>
      le16At frame (offset + idx * 2)
    if response.all (fun value => value < q) then some response else none
  else
    none

def checkedCoreTranscriptDifference? (lhs rhs : List Nat) : Option (List Nat) :=
  match decodeCheckedCoreTranscriptFrame? lhs, decodeCheckedCoreTranscriptFrame? rhs with
  | some lhsResponse, some rhsResponse => some (listSubModQ lhsResponse rhsResponse)
  | _, _ => none

def checkedCoreTranscriptA : List Nat :=
  (checkedCoreTranscriptFrameDomainBytes ++ checkedCoreTranscriptFrameProfileBytes) ++
  (List.replicate sisFullResponseResidues 10).flatMap le16Bytes

def checkedCoreTranscriptB : List Nat :=
  (checkedCoreTranscriptFrameDomainBytes ++ checkedCoreTranscriptFrameProfileBytes) ++
  ([10, 9, 10, 11] ++ List.replicate 19 10).flatMap le16Bytes

theorem checked_core_transcript_frame_domain_length :
    checkedCoreTranscriptFrameDomainBytes.length = 79 := by
  native_decide

theorem checked_core_transcript_frame_profile_length :
    checkedCoreTranscriptFrameProfileBytes.length = 31 := by
  native_decide

theorem checked_core_transcript_frame_byte_accounting :
    checkedCoreTranscriptFrameBytes = 156 := by
  native_decide

theorem encode_decode_checked_core_transcript_frame_vector :
    encodeCheckedCoreTranscriptFrame? (List.replicate 23 10) =
      some checkedCoreTranscriptA ∧
    decodeCheckedCoreTranscriptFrame? checkedCoreTranscriptA =
      some (List.replicate 23 10) := by
  native_decide

theorem checked_core_transcript_frame_rejects_wrong_domain :
    decodeCheckedCoreTranscriptFrame? (0 :: checkedCoreTranscriptA.drop 1) = none := by
  native_decide

theorem checked_core_transcript_frame_rejects_wrong_profile :
    decodeCheckedCoreTranscriptFrame?
      (checkedCoreTranscriptFrameDomainBytes ++
        0 :: checkedCoreTranscriptFrameProfileBytes.drop 1 ++
        (List.replicate sisFullResponseResidues 10).flatMap le16Bytes) = none := by
  native_decide

theorem checked_core_transcript_frame_rejects_noncanonical_response :
    decodeCheckedCoreTranscriptFrame?
      ((checkedCoreTranscriptFrameDomainBytes ++ checkedCoreTranscriptFrameProfileBytes) ++
        (List.replicate 22 10).flatMap le16Bytes ++ le16Bytes q) = none := by
  native_decide

theorem checked_core_transcript_difference_vector :
    checkedCoreTranscriptDifference? checkedCoreTranscriptA checkedCoreTranscriptB =
      some ([0, 1, 0, q - 1] ++ List.replicate 19 0) := by
  native_decide

def accountOpeningFrameDomainBytes : List Nat :=
  [68, 101, 115, 99, 101, 110, 100, 101, 114, 46, 104, 105, 100,
   100, 101, 110, 95, 115, 111, 117, 114, 99, 101, 46, 102, 97,
   108, 108, 98, 97, 99, 107, 95, 112, 114, 111, 106, 101, 99,
   116, 105, 111, 110, 46, 97, 99, 99, 111, 117, 110, 116, 95,
   119, 105, 116, 110, 101, 115, 115, 95, 111, 112, 101, 110,
   105, 110, 103, 95, 102, 114, 97, 109, 101, 46, 118, 49]

def accountOpeningFrameProfileBytes : List Nat :=
  [110, 51, 50, 46, 113, 49, 50, 50, 56, 57, 46, 102, 113, 49,
   50, 46, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 46,
   118, 49]

def accountOpeningFrameBytes : Nat :=
  accountOpeningFrameDomainBytes.length +
    accountOpeningFrameProfileBytes.length +
    fallbackAccountProjectionLaneCount * 2

def encodeAccountWitnessOpeningFrame? (selectedSourceValues : List Nat) :
    Option (List Nat) :=
  if selectedSourceValues.length == fallbackAccountProjectionLaneCount &&
      selectedSourceValues.all (fun value => value < q) then
    some <|
      accountOpeningFrameDomainBytes ++
      accountOpeningFrameProfileBytes ++
      selectedSourceValues.flatMap le16Bytes
  else
    none

def decodeAccountWitnessOpeningFrame? (frame : List Nat) : Option (List Nat) :=
  if frame.length == accountOpeningFrameBytes && bytesWithinUInt8 frame &&
      frame.take accountOpeningFrameDomainBytes.length ==
        accountOpeningFrameDomainBytes &&
      (frame.drop accountOpeningFrameDomainBytes.length).take
        accountOpeningFrameProfileBytes.length ==
        accountOpeningFrameProfileBytes then
    let offset :=
      accountOpeningFrameDomainBytes.length +
      accountOpeningFrameProfileBytes.length
    let values := (List.range fallbackAccountProjectionLaneCount).map fun idx =>
      le16At frame (offset + idx * 2)
    if values.all (fun value => value < q) then some values else none
  else
    none

def projectionLaneCoefficients : List Nat :=
  [6234, 2199, 6252, 1488, 2113, 823,
   1934, 5142, 10681, 10066, 5429, 5754]

def accountProjectionValuesFromOpening? (selectedSourceValues : List Nat) :
    Option (List Nat) :=
  if selectedSourceValues.length == fallbackAccountProjectionLaneCount &&
      selectedSourceValues.all (fun value => value < q) then
    some <|
      (projectionLaneCoefficients.take fallbackAccountProjectionLaneCount).zip
        selectedSourceValues |>.map fun pair => mulModQ pair.1 pair.2
  else
    none

def hiddenProjectionValuesFromReplay? (accountValues padValues : List Nat) :
    Option (List Nat) :=
  if accountValues.length == fallbackAccountProjectionLaneCount &&
      padValues.length == fallbackPadProjectionLaneCount &&
      accountValues.all (fun value => value < q) &&
      padValues.all (fun value => value < q) then
    some (accountValues ++ padValues)
  else
    none

def padPrfDomainBytes : List Nat :=
  [68, 101, 115, 99, 101, 110, 100, 101, 114, 46, 83, 105, 103,
   110, 97, 116, 117, 114, 101, 46, 113, 49, 50, 50, 56, 57, 46,
   108, 111, 99, 97, 108, 95, 100, 101, 108, 116, 97, 46, 112,
   97, 100, 46, 118, 49]

def padFrameBytes (block : Nat) : List Nat :=
  padPrfDomainBytes ++ [block]

def sha256DigestBytes (bytes : List Nat) : List Nat :=
  ResponseLinkExactCertificate.BoundedKernelExactSummary.sha256DigestBytes bytes

def padPrfBlockDigest (seed : List Nat) (block counter : Nat) : List Nat :=
  sha256DigestBytes (seed ++ padFrameBytes block ++ le32Bytes counter)

def digestLittleEndianU16Words (digest : List Nat) : List Nat :=
  (List.range 16).map fun idx =>
    byteAt digest (idx * 2) + 256 * byteAt digest (idx * 2 + 1)

def acceptedQWordsFromDigest (digest : List Nat) : List Nat :=
  (digestLittleEndianU16Words digest).filter fun word =>
    word < ResponseLinkExactCertificate.BoundedKernelExactSummary.rejectionLimit q

def padAcceptedResiduesFromSeedCounter
    (seed : List Nat) (block counter : Nat) : List Nat :=
  (acceptedQWordsFromDigest (padPrfBlockDigest seed block counter)).map
    fun word => word % q

abbrev padPrfCounterLimit : Nat := 16

def padResiduesFromSeedBlockWindow
    (seed : List Nat) (block counter fuel : Nat) : List Nat :=
  (List.range fuel).flatMap fun idx =>
    padAcceptedResiduesFromSeedCounter seed block (counter + idx)

def padResiduesFromSeedBlockBoundedFrom?
    (seed : List Nat) (block counter fuel remaining : Nat) : Option (List Nat) :=
  let accepted := padResiduesFromSeedBlockWindow seed block counter fuel
  if remaining <= accepted.length then
    some (accepted.take remaining)
  else
    none

def padResiduesFromSeedBlockBounded? (seed : List Nat) (block : Nat) :
    Option (List Nat) :=
  padResiduesFromSeedBlockBoundedFrom? seed block 0 padPrfCounterLimit localFree

def padResiduesFromSeedBlockFrom
    (seed : List Nat) (block counter remaining : Nat) : List Nat :=
  match padResiduesFromSeedBlockBoundedFrom? seed block counter padPrfCounterLimit
      remaining with
  | some residues => residues
  | none => []

def padResiduesFromSeedBlock (seed : List Nat) (block : Nat) : List Nat :=
  padResiduesFromSeedBlockFrom seed block 0 localFree

theorem pad_accepted_residues_from_counter_canonical
    (seed : List Nat) (block counter : Nat) :
    (padAcceptedResiduesFromSeedCounter seed block counter).all
      (fun value => value < q) = true := by
  simp [padAcceptedResiduesFromSeedCounter, q]
  intro word _
  exact Nat.mod_lt word (by decide)

theorem pad_residues_bounded_from_success_length
    (seed : List Nat) (block counter fuel remaining : Nat) (values : List Nat)
    (h :
      padResiduesFromSeedBlockBoundedFrom? seed block counter fuel remaining =
        some values) :
    values.length = remaining := by
  rw [padResiduesFromSeedBlockBoundedFrom?] at h
  let accepted := padResiduesFromSeedBlockWindow seed block counter fuel
  by_cases htake : remaining <= accepted.length
  · simp [accepted, htake] at h
    cases h
    rw [List.length_take, Nat.min_eq_left htake]
  · simp [accepted, htake] at h

theorem pad_residues_bounded_success_length
    (seed : List Nat) (block : Nat) (values : List Nat)
    (h : padResiduesFromSeedBlockBounded? seed block = some values) :
    values.length = localFree :=
  pad_residues_bounded_from_success_length seed block 0 padPrfCounterLimit
    localFree values h

theorem pad_residues_bounded_from_success_canonical
    (seed : List Nat) (block counter fuel remaining : Nat) (values : List Nat)
    (h :
      padResiduesFromSeedBlockBoundedFrom? seed block counter fuel remaining =
        some values) :
    values.all (fun value => value < q) = true := by
  rw [padResiduesFromSeedBlockBoundedFrom?] at h
  let accepted := padResiduesFromSeedBlockWindow seed block counter fuel
  by_cases htake : remaining <= accepted.length
  · simp [accepted, htake] at h
    cases h
    have haccepted :
        accepted.all (fun value => value < q) = true := by
      simp [accepted, padResiduesFromSeedBlockWindow]
      intro idx _
      have hcounter :=
        pad_accepted_residues_from_counter_canonical seed block (counter + idx)
      simpa using hcounter
    simp at haccepted
    simp
    intro value hvalue
    exact haccepted value (List.mem_of_mem_take hvalue)
  · simp [accepted, htake] at h

theorem pad_residues_bounded_success_canonical
    (seed : List Nat) (block : Nat) (values : List Nat)
    (h : padResiduesFromSeedBlockBounded? seed block = some values) :
    values.all (fun value => value < q) = true :=
  pad_residues_bounded_from_success_canonical seed block 0 padPrfCounterLimit
    localFree values h

theorem pad_residues_bounded_from_failure_exhausted
    (seed : List Nat) (block counter fuel remaining : Nat)
    (h :
      padResiduesFromSeedBlockBoundedFrom? seed block counter fuel remaining =
        none) :
    (padResiduesFromSeedBlockWindow seed block counter fuel).length < remaining := by
  rw [padResiduesFromSeedBlockBoundedFrom?] at h
  let accepted := padResiduesFromSeedBlockWindow seed block counter fuel
  by_cases htake : remaining <= accepted.length
  · simp [accepted, htake] at h
  · exact Nat.lt_of_not_ge htake

theorem pad_residues_bounded_failure_exhausted
    (seed : List Nat) (block : Nat)
    (h : padResiduesFromSeedBlockBounded? seed block = none) :
    (padResiduesFromSeedBlockWindow seed block 0 padPrfCounterLimit).length <
      localFree :=
  pad_residues_bounded_from_failure_exhausted seed block 0 padPrfCounterLimit
    localFree h

theorem pad_prf_counter_limit_positive :
    0 < padPrfCounterLimit := by
  native_decide

def padProjectionLaneMap : List (Nat × Nat × Nat) :=
  [(6, 2, 1), (7, 7, 0), (8, 12, 1),
   (9, 1, 0), (10, 6, 1), (11, 11, 0)]

def padProjectionValuesFromSeed? (seed : List Nat) : Option (List Nat) :=
  match padResiduesFromSeedBlockBounded? seed 2,
      padResiduesFromSeedBlockBounded? seed 7,
      padResiduesFromSeedBlockBounded? seed 12,
      padResiduesFromSeedBlockBounded? seed 1,
      padResiduesFromSeedBlockBounded? seed 6,
      padResiduesFromSeedBlockBounded? seed 11 with
  | some block2, some block7, some block12, some block1, some block6,
      some block11 =>
      some [
        mulModQ (natAt projectionLaneCoefficients 6) (natAt block2 1),
        mulModQ (natAt projectionLaneCoefficients 7) (natAt block7 0),
        mulModQ (natAt projectionLaneCoefficients 8) (natAt block12 1),
        mulModQ (natAt projectionLaneCoefficients 9) (natAt block1 0),
        mulModQ (natAt projectionLaneCoefficients 10) (natAt block6 1),
        mulModQ (natAt projectionLaneCoefficients 11) (natAt block11 0)
      ]
  | _, _, _, _, _, _ => none

theorem pad_projection_lane_map_length :
    padProjectionLaneMap.length = fallbackPadProjectionLaneCount := by
  native_decide

theorem pad_projection_values_from_seed_success_length
    (seed values : List Nat)
    (h : padProjectionValuesFromSeed? seed = some values) :
    values.length = fallbackPadProjectionLaneCount := by
  unfold padProjectionValuesFromSeed? at h
  split at h <;> cases h <;> simp [fallbackPadProjectionLaneCount]

theorem pad_projection_values_from_seed_success_canonical
    (seed values : List Nat)
    (h : padProjectionValuesFromSeed? seed = some values) :
    values.all (fun value => value < q) = true := by
  unfold padProjectionValuesFromSeed? at h
  split at h <;> cases h <;> simp [mulModQ, q]
  constructor
  · exact Nat.mod_lt _ (by decide)
  constructor
  · exact Nat.mod_lt _ (by decide)
  constructor
  · exact Nat.mod_lt _ (by decide)
  constructor
  · exact Nat.mod_lt _ (by decide)
  constructor
  · exact Nat.mod_lt _ (by decide)
  · exact Nat.mod_lt _ (by decide)

def padSeed42 : List Nat := List.replicate 32 0x42

def padProjectionValuesFromSeed42 : List Nat :=
  [
    mulModQ (natAt projectionLaneCoefficients 6)
      (natAt (padResiduesFromSeedBlock padSeed42 2) 1),
    mulModQ (natAt projectionLaneCoefficients 7)
      (natAt (padResiduesFromSeedBlock padSeed42 7) 0),
    mulModQ (natAt projectionLaneCoefficients 8)
      (natAt (padResiduesFromSeedBlock padSeed42 12) 1),
    mulModQ (natAt projectionLaneCoefficients 9)
      (natAt (padResiduesFromSeedBlock padSeed42 1) 0),
    mulModQ (natAt projectionLaneCoefficients 10)
      (natAt (padResiduesFromSeedBlock padSeed42 6) 1),
    mulModQ (natAt projectionLaneCoefficients 11)
      (natAt (padResiduesFromSeedBlock padSeed42 11) 0)
  ]

def replayHiddenProjectionBridge? (carrier : FallbackProjectionCarrierDecoded)
    (accountValues padValues : List Nat) : Option Unit :=
  match hiddenProjectionValuesFromReplay? accountValues padValues with
  | some values =>
      verifyFallbackProjectionPublicBridge? carrier {
        projectionValues := values
        checkBytes := carrier.checkBytes
        transcriptBinding := carrier.transcriptBinding
      }
  | none => none

def replayHiddenProjectionBridgeFromSeed?
    (carrier : FallbackProjectionCarrierDecoded)
    (accountValues seedBytes : List Nat) : Option Unit :=
  match decodePadSeed? seedBytes with
  | some seed =>
      match padProjectionValuesFromSeed? seed with
      | some padValues => replayHiddenProjectionBridge? carrier accountValues padValues
      | none => none
  | none => none

theorem replay_hidden_projection_bridge_from_seed_success_eq
    (carrier : FallbackProjectionCarrierDecoded)
    (accountValues seedBytes seed padValues : List Nat)
    (hdecode : decodePadSeed? seedBytes = some seed)
    (hderive : padProjectionValuesFromSeed? seed = some padValues) :
    replayHiddenProjectionBridgeFromSeed? carrier accountValues seedBytes =
      replayHiddenProjectionBridge? carrier accountValues padValues := by
  simp [replayHiddenProjectionBridgeFromSeed?, hdecode, hderive]

theorem replay_hidden_projection_bridge_from_seed_decode_failure_rejects
    (carrier : FallbackProjectionCarrierDecoded)
    (accountValues seedBytes : List Nat)
    (hdecode : decodePadSeed? seedBytes = none) :
    replayHiddenProjectionBridgeFromSeed? carrier accountValues seedBytes = none := by
  simp [replayHiddenProjectionBridgeFromSeed?, hdecode]

theorem replay_hidden_projection_bridge_from_seed_derivation_failure_rejects
    (carrier : FallbackProjectionCarrierDecoded)
    (accountValues seedBytes seed : List Nat)
    (hdecode : decodePadSeed? seedBytes = some seed)
    (hderive : padProjectionValuesFromSeed? seed = none) :
    replayHiddenProjectionBridgeFromSeed? carrier accountValues seedBytes = none := by
  simp [replayHiddenProjectionBridgeFromSeed?, hdecode, hderive]

def accountOpeningVector : List Nat := [1, 2, 3, 4, 5, 6]

def accountOpeningFrameVector : List Nat :=
  accountOpeningFrameDomainBytes ++ accountOpeningFrameProfileBytes ++
  accountOpeningVector.flatMap le16Bytes

def accountProjectionVector : List Nat :=
  [6234, 4398, 6467, 5952, 10565, 4938]

def padProjectionVector : List Nat :=
  [12169, 2446, 4817, 3574, 5914, 8556]

def hiddenProjectionCarrierBytes : List Nat :=
  (accountProjectionVector ++ padProjectionVector).flatMap le16Bytes ++
  [12, 13, 14] ++ List.replicate 29 15

theorem account_opening_frame_domain_length :
    accountOpeningFrameDomainBytes.length = 76 := by
  native_decide

theorem account_opening_frame_profile_length :
    accountOpeningFrameProfileBytes.length = 29 := by
  native_decide

theorem account_opening_frame_byte_accounting :
    accountOpeningFrameBytes = 117 := by
  native_decide

theorem encode_decode_account_opening_frame_vector :
    encodeAccountWitnessOpeningFrame? accountOpeningVector =
      some accountOpeningFrameVector ∧
    decodeAccountWitnessOpeningFrame? accountOpeningFrameVector =
      some accountOpeningVector := by
  native_decide

theorem account_opening_frame_rejects_wrong_domain :
    decodeAccountWitnessOpeningFrame? (0 :: accountOpeningFrameVector.drop 1) = none := by
  native_decide

theorem account_opening_frame_rejects_wrong_profile :
    decodeAccountWitnessOpeningFrame?
      (accountOpeningFrameDomainBytes ++
        0 :: accountOpeningFrameProfileBytes.drop 1 ++
        accountOpeningVector.flatMap le16Bytes) = none := by
  native_decide

theorem account_opening_frame_rejects_noncanonical_coordinate :
    decodeAccountWitnessOpeningFrame?
      (accountOpeningFrameDomainBytes ++ accountOpeningFrameProfileBytes ++
        [1, 2, 3, 4, 5].flatMap le16Bytes ++ le16Bytes q) = none := by
  native_decide

theorem account_projection_values_from_opening_vector :
    accountProjectionValuesFromOpening? accountOpeningVector =
      some accountProjectionVector := by
  native_decide

theorem hidden_projection_values_from_replay_vector :
    hiddenProjectionValuesFromReplay? accountProjectionVector padProjectionVector =
      some (accountProjectionVector ++ padProjectionVector) := by
  native_decide

theorem pad_prf_domain_length :
    padPrfDomainBytes.length = 45 := by
  native_decide

theorem pad_prf_frame_byte_accounting :
    (padFrameBytes 2).length = 46 := by
  native_decide

theorem pad_prf_seed42_block_residue_vectors :
    padResiduesFromSeedBlock padSeed42 1 = [2674, 9115] ∧
    padResiduesFromSeedBlock padSeed42 2 = [1311, 6011] ∧
    padResiduesFromSeedBlock padSeed42 6 = [4034, 7068] ∧
    padResiduesFromSeedBlock padSeed42 7 = [6329, 9937] ∧
    padResiduesFromSeedBlock padSeed42 11 = [6870, 8128] ∧
    padResiduesFromSeedBlock padSeed42 12 = [9169, 11132] := by
  native_decide

theorem pad_prf_seed42_block_bounded_residue_vectors :
    padResiduesFromSeedBlockBounded? padSeed42 1 = some [2674, 9115] ∧
    padResiduesFromSeedBlockBounded? padSeed42 2 = some [1311, 6011] ∧
    padResiduesFromSeedBlockBounded? padSeed42 6 = some [4034, 7068] ∧
    padResiduesFromSeedBlockBounded? padSeed42 7 = some [6329, 9937] ∧
    padResiduesFromSeedBlockBounded? padSeed42 11 = some [6870, 8128] ∧
    padResiduesFromSeedBlockBounded? padSeed42 12 = some [9169, 11132] := by
  native_decide

theorem pad_prf_projection_values_from_seed42 :
    padProjectionValuesFromSeed42 = padProjectionVector := by
  native_decide

theorem pad_prf_projection_lane_map_selected :
    padProjectionLaneMap =
      [(6, 2, 1), (7, 7, 0), (8, 12, 1),
       (9, 1, 0), (10, 6, 1), (11, 11, 0)] := by
  native_decide

theorem pad_prf_projection_values_from_seed42_universal :
    padProjectionValuesFromSeed? padSeed42 = some padProjectionVector := by
  native_decide

def padProjectionValuesFromSeed42MutatedLaneMap : List Nat :=
  [
    mulModQ (natAt projectionLaneCoefficients 6)
      (natAt (padResiduesFromSeedBlock padSeed42 2) 0),
    mulModQ (natAt projectionLaneCoefficients 7)
      (natAt (padResiduesFromSeedBlock padSeed42 7) 0),
    mulModQ (natAt projectionLaneCoefficients 8)
      (natAt (padResiduesFromSeedBlock padSeed42 12) 1),
    mulModQ (natAt projectionLaneCoefficients 9)
      (natAt (padResiduesFromSeedBlock padSeed42 1) 0),
    mulModQ (natAt projectionLaneCoefficients 10)
      (natAt (padResiduesFromSeedBlock padSeed42 6) 1),
    mulModQ (natAt projectionLaneCoefficients 11)
      (natAt (padResiduesFromSeedBlock padSeed42 11) 0)
  ]

theorem pad_seed_decode_accepts_seed42 :
    decodePadSeed? padSeed42 = some padSeed42 := by
  native_decide

theorem pad_seed_decode_rejects_short_seed :
    decodePadSeed? (List.replicate 31 0x42) = none := by
  native_decide

theorem pad_seed_decode_rejects_non_uint8_seed :
    decodePadSeed? (256 :: List.replicate 31 0x42) = none := by
  native_decide

theorem replay_hidden_projection_bridge_rejects_short_seed :
    replayHiddenProjectionBridgeFromSeed?
      {
        lanes := accountProjectionVector ++ padProjectionVector
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      accountProjectionVector (List.replicate 31 0x42) = none := by
  native_decide

theorem replay_hidden_projection_bridge_rejects_non_uint8_seed :
    replayHiddenProjectionBridgeFromSeed?
      {
        lanes := accountProjectionVector ++ padProjectionVector
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      accountProjectionVector (256 :: List.replicate 31 0x42) = none := by
  native_decide

theorem pad_prf_seed42_zero_counter_limit_exhausts :
    padResiduesFromSeedBlockBoundedFrom? padSeed42 2 0 0 localFree = none ∧
    (padResiduesFromSeedBlockWindow padSeed42 2 0 0).length < localFree := by
  native_decide

theorem pad_prf_seed42_mutated_lane_map_differs :
    padProjectionValuesFromSeed42MutatedLaneMap ≠ padProjectionVector := by
  native_decide

theorem hidden_projection_values_reject_noncanonical_pad_value :
    hiddenProjectionValuesFromReplay? accountProjectionVector
      ([7, 8, 9, 10, 11, q]) = none := by
  native_decide

theorem replay_hidden_projection_bridge_accepts_matching_material :
    replayHiddenProjectionBridge?
      {
        lanes := accountProjectionVector ++ padProjectionVector
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      accountProjectionVector padProjectionVector = some () := by
  native_decide

theorem replay_hidden_projection_bridge_rejects_mismatched_material :
    replayHiddenProjectionBridge?
      {
        lanes := accountProjectionVector ++ padProjectionVector
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      accountProjectionVector ([12169, 2446, 4817, 3574, 5914, 8557]) = none := by
  native_decide

theorem replay_hidden_projection_bridge_from_seed42_accepts_matching_material :
    replayHiddenProjectionBridgeFromSeed?
      {
        lanes := accountProjectionVector ++ padProjectionVector
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      accountProjectionVector padSeed42 = some () := by
  native_decide

theorem replay_hidden_projection_bridge_from_seed42_rejects_carrier_pad_mismatch :
    replayHiddenProjectionBridgeFromSeed?
      {
        lanes := accountProjectionVector ++
          [12169, 2446, 4817, 3574, 5914, 8557]
        checkBytes := [12, 13, 14]
        transcriptBinding := List.replicate 29 15
      }
      accountProjectionVector padSeed42 = none := by
  native_decide

theorem replay_hidden_projection_bridge_from_failed_seed_cannot_accept
    (carrier : FallbackProjectionCarrierDecoded)
    (accountValues seedBytes seed : List Nat)
    (hdecode : decodePadSeed? seedBytes = some seed)
    (hderive : padProjectionValuesFromSeed? seed = none) :
    replayHiddenProjectionBridgeFromSeed? carrier accountValues seedBytes ≠ some () := by
  rw [replay_hidden_projection_bridge_from_seed_derivation_failure_rejects carrier
    accountValues seedBytes seed hdecode hderive]
  simp

def bucket2NegativeControlNames : List String :=
  [
    "pad_seed_decode_rejects_short_seed",
    "pad_seed_decode_rejects_non_uint8_seed",
    "replay_hidden_projection_bridge_rejects_short_seed",
    "replay_hidden_projection_bridge_rejects_non_uint8_seed",
    "pad_prf_seed42_zero_counter_limit_exhausts",
    "pad_prf_seed42_mutated_lane_map_differs",
    "replay_hidden_projection_bridge_from_seed42_rejects_carrier_pad_mismatch",
    "replay_hidden_projection_bridge_from_failed_seed_cannot_accept"
  ]

theorem bucket2_negative_control_count :
    bucket2NegativeControlNames.length = 8 := by
  native_decide

theorem bucket2_negative_control_index_contains_seed_and_exhaustion_controls :
    "pad_seed_decode_rejects_short_seed" ∈ bucket2NegativeControlNames ∧
    "pad_seed_decode_rejects_non_uint8_seed" ∈ bucket2NegativeControlNames ∧
    "pad_prf_seed42_zero_counter_limit_exhausts" ∈
      bucket2NegativeControlNames ∧
    "pad_prf_seed42_mutated_lane_map_differs" ∈
      bucket2NegativeControlNames ∧
    "replay_hidden_projection_bridge_from_seed42_rejects_carrier_pad_mismatch" ∈
      bucket2NegativeControlNames := by
  native_decide

def bucket1NegativeControlNames : List String :=
  [
    "decode_rejects_short_fallback_projection_carrier",
    "decode_rejects_non_uint8_fallback_projection_carrier",
    "decode_rejects_noncanonical_q12289_lane_zero",
    "decode_reviewed_fq12_rejects_short_signature",
    "decode_reviewed_fq12_rejects_challenge_high_bits",
    "decode_reviewed_fq12_rejects_response_out_of_range",
    "decode_reviewed_fq12_rejects_noncanonical_opening",
    "decode_reviewed_fq12_rejects_masked_response_slack_bits",
    "checked_core_transcript_frame_rejects_wrong_domain",
    "checked_core_transcript_frame_rejects_wrong_profile",
    "checked_core_transcript_frame_rejects_noncanonical_response",
    "fallback_projection_public_bridge_rejects_lane_mismatch",
    "fallback_projection_public_bridge_rejects_check_mismatch",
    "fallback_projection_public_bridge_rejects_transcript_binding_mismatch",
    "fallback_projection_public_bridge_rejects_noncanonical_expected_lane",
    "account_opening_frame_rejects_wrong_domain",
    "account_opening_frame_rejects_wrong_profile",
    "account_opening_frame_rejects_noncanonical_coordinate",
    "hidden_projection_values_reject_noncanonical_pad_value",
    "replay_hidden_projection_bridge_rejects_mismatched_material"
  ]

theorem bucket1_negative_control_count :
    bucket1NegativeControlNames.length = 20 := by
  native_decide

theorem bucket1_negative_control_index_contains_parser_and_bridge_controls :
    "decode_reviewed_fq12_rejects_masked_response_slack_bits" ∈
        bucket1NegativeControlNames ∧
      "fallback_projection_public_bridge_rejects_transcript_binding_mismatch" ∈
        bucket1NegativeControlNames ∧
      "replay_hidden_projection_bridge_rejects_mismatched_material" ∈
        bucket1NegativeControlNames := by
  native_decide

def decodeChallenge? (bytes : List Nat) : Option (List Nat) :=
  if bytes.length == challengeBytes && bytesWithinUInt8 bytes &&
      (Nat.land (byteAt bytes (algebraicChallengeBytes - 1))
        (255 - algebraicChallengeLastByteMask)) == 0 then
    some bytes
  else
    none

def decodeResponses? (bytes : List Nat) : Option (List Int) :=
  if bytes.length == activeResponseBytes && bytesWithinUInt8 bytes &&
      bytes.all (fun byte => byte <= 2 * responseBound) then
    some (bytes.map fun byte => Int.ofNat byte - Int.ofNat responseBound)
  else
    none

def decodeOpenings? (bytes : List Nat) : Option (List Nat) :=
  if bytes.length == localOpeningBytes && bytesWithinUInt8 bytes then
    let values := (List.range (reps * localFree)).map fun idx =>
      le16At bytes (idx * 2)
    if values.all (fun value => value < q) then some values else none
  else
    none

def decodeReviewedFq12MaskedResponse? (bytes : List Nat) : Option (List Nat) :=
  if bytes.length == reviewedFq12MaskedResponseBytes && bytesWithinUInt8 bytes &&
      !anyBitsSetLE bytes reviewedFq12MaskedResponseBits
        reviewedFq12MaskedResponseSlackBits then
    let values := (List.range reviewedFq12ResponseResidues).map fun idx =>
      readBitsLE bytes (idx * fieldBits) fieldBits
    if values.all (fun value => value < q) then some values else none
  else
    none

structure ReviewedFq12Parsed where
  slotId : Nat
  challengeBytes : List Nat
  responses : List Int
  openings : List Nat
  proofCommitment : List Nat
  maskedResponseResidues : List Nat
deriving DecidableEq, Repr

def decodeReviewedFq12Signature? (bytes : List Nat) : Option ReviewedFq12Parsed :=
  if bytes.length == reviewedFq12TransactionBytes && bytesWithinUInt8 bytes then
    let responseStart := slotIdBytes + challengeBytes
    let openingStart := responseStart + activeResponseBytes
    let proofStart := openingStart + localOpeningBytes
    let challengeSlice := (bytes.drop slotIdBytes).take challengeBytes
    let responseSlice := (bytes.drop responseStart).take activeResponseBytes
    let openingSlice := (bytes.drop openingStart).take localOpeningBytes
    let proofSlice := bytes.drop proofStart
    let maskedSlice :=
      (proofSlice.drop linearSigmaProofCommitmentBytes).take
        reviewedFq12MaskedResponseBytes
    match decodeChallenge? challengeSlice,
        decodeResponses? responseSlice,
        decodeOpenings? openingSlice,
        decodeReviewedFq12MaskedResponse? maskedSlice with
    | some challenge, some responses, some openings, some masked =>
        some {
          slotId := le32At bytes 0
          challengeBytes := challenge
          responses := responses
          openings := openings
          proofCommitment := proofSlice.take linearSigmaProofCommitmentBytes
          maskedResponseResidues := masked
        }
    | _, _, _, _ => none
  else
    none

def zeroReviewedFq12Signature : List Nat :=
  [7, 0, 0, 0] ++
  [2] ++ List.replicate (challengeBytes - 1) 0 ++
  List.replicate activeResponseBytes responseBound ++
  (List.range (reps * localFree)).flatMap (fun _ => [1, 0]) ++
  List.replicate linearSigmaProofCommitmentBytes 3 ++
  List.replicate reviewedFq12MaskedResponseBytes 0

def nonzeroReviewedFq12MaskedResponseBytes : List Nat :=
  [5, 64, 1, 80, 0, 20, 0,
   5, 64, 1, 80, 0, 20, 0,
   5, 64, 1, 80, 0, 20, 0,
   5, 64, 1, 80, 0, 20, 0,
   5, 64, 1, 80, 0, 20, 0,
   5, 64, 1, 0]

def nonzeroReviewedFq12Signature : List Nat :=
  [7, 0, 0, 0] ++
  [2] ++ List.replicate (challengeBytes - 1) 0 ++
  List.replicate activeResponseBytes responseBound ++
  (List.range (reps * localFree)).flatMap (fun _ => [1, 0]) ++
  List.replicate linearSigmaProofCommitmentBytes 3 ++
  nonzeroReviewedFq12MaskedResponseBytes

theorem reviewed_fq12_byte_accounting :
    slotIdBytes + challengeBytes + activeResponseBytes +
      localOpeningBytes + reviewedFq12ProofBytes =
      reviewedFq12TransactionBytes := by
  native_decide

theorem reviewed_fq12_proof_byte_accounting :
    linearSigmaProofCommitmentBytes + reviewedFq12MaskedResponseBytes =
      reviewedFq12ProofBytes := by
  native_decide

theorem reviewed_fq12_masked_response_bit_accounting :
    reviewedFq12ResponseResidues * fieldBits = reviewedFq12MaskedResponseBits := by
  native_decide

theorem decode_zero_reviewed_fq12_signature :
    decodeReviewedFq12Signature? zeroReviewedFq12Signature =
      some {
        slotId := 7
        challengeBytes := [2] ++ List.replicate 20 0
        responses := List.replicate 52 0
        openings := List.replicate 52 1
        proofCommitment := List.replicate 32 3
        maskedResponseResidues := List.replicate 22 0
      } := by
  native_decide

theorem nonzero_reviewed_fq12_masked_response_vector_length :
    nonzeroReviewedFq12MaskedResponseBytes.length =
      reviewedFq12MaskedResponseBytes := by
  native_decide

theorem decode_nonzero_reviewed_fq12_masked_response_vector :
    decodeReviewedFq12MaskedResponse? nonzeroReviewedFq12MaskedResponseBytes =
      some (List.replicate 22 5) := by
  native_decide

theorem decode_nonzero_reviewed_fq12_signature :
    decodeReviewedFq12Signature? nonzeroReviewedFq12Signature =
      some {
        slotId := 7
        challengeBytes := [2] ++ List.replicate 20 0
        responses := List.replicate 52 0
        openings := List.replicate 52 1
        proofCommitment := List.replicate 32 3
        maskedResponseResidues := List.replicate 22 5
      } := by
  native_decide

theorem decode_reviewed_fq12_rejects_short_signature :
    decodeReviewedFq12Signature? (List.replicate 251 0) = none := by
  native_decide

theorem decode_reviewed_fq12_rejects_challenge_high_bits :
    decodeReviewedFq12Signature?
      (([7, 0, 0, 0] ++
        List.replicate 16 0 ++ [128] ++ List.replicate 4 0 ++
        List.replicate activeResponseBytes responseBound ++
        (List.range (reps * localFree)).flatMap (fun _ => [1, 0]) ++
        List.replicate linearSigmaProofCommitmentBytes 3 ++
        List.replicate reviewedFq12MaskedResponseBytes 0)) = none := by
  native_decide

theorem decode_reviewed_fq12_rejects_response_out_of_range :
    decodeReviewedFq12Signature?
      (([7, 0, 0, 0] ++
        [2] ++ List.replicate 20 0 ++
        [97] ++ List.replicate (activeResponseBytes - 1) responseBound ++
        (List.range (reps * localFree)).flatMap (fun _ => [1, 0]) ++
        List.replicate linearSigmaProofCommitmentBytes 3 ++
        List.replicate reviewedFq12MaskedResponseBytes 0)) = none := by
  native_decide

theorem decode_reviewed_fq12_rejects_noncanonical_opening :
    decodeReviewedFq12Signature?
      (([7, 0, 0, 0] ++
        [2] ++ List.replicate 20 0 ++
        List.replicate activeResponseBytes responseBound ++
        [1, 48] ++ List.replicate (localOpeningBytes - 2) 0 ++
        List.replicate linearSigmaProofCommitmentBytes 3 ++
        List.replicate reviewedFq12MaskedResponseBytes 0)) = none := by
  native_decide

theorem decode_reviewed_fq12_rejects_masked_response_slack_bits :
    decodeReviewedFq12Signature?
      (([7, 0, 0, 0] ++
        [2] ++ List.replicate 20 0 ++
        List.replicate activeResponseBytes responseBound ++
        (List.range (reps * localFree)).flatMap (fun _ => [1, 0]) ++
        List.replicate linearSigmaProofCommitmentBytes 3 ++
        List.replicate 38 0 ++ [240])) = none := by
  native_decide

end HiddenSourceBucket1
end Descender
