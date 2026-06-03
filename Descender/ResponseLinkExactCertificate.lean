namespace Descender
namespace ResponseLinkExactCertificate

/-!
Lean-side shape for response-link exact bounded-kernel certificates.

This module is intentionally small.  Generated modules reflect the audited
enumeration output into Lean and prove the finite arithmetic consequences that
later proof layers consume.
-/

structure BoundedKernelExactSummary where
  q : Nat
  matrixRows : Nat
  matrixCols : Nat
  coords : List Nat
  responseDifferenceBound : Nat
  shortestCoeffBound : Nat
  minimumCoverCoeffBound : Nat
  combinedConstraintRank : Nat
  combinedKernelDimension : Nat
  status : String
  certificateSha256 : String
  manifestPayloadSha256 : String

namespace BoundedKernelExactSummary

def maxAbsCoeff : List Int -> Nat
  | [] => 0
  | x :: xs => max x.natAbs (maxAbsCoeff xs)

def l1Coeff : List Int -> Nat
  | [] => 0
  | x :: xs => x.natAbs + l1Coeff xs

def supportCount : List Int -> Nat
  | [] => 0
  | x :: xs => (if x = 0 then 0 else 1) + supportCount xs

def dotInt : List Int -> List Int -> Int
  | x :: xs, y :: ys => x * y + dotInt xs ys
  | _, _ => 0

def allRowsLength (cols : Nat) (rows : List (List Int)) : Bool :=
  rows.all fun row => row.length == cols

def matrixShape (rows cols : Nat) (matrix : List (List Int)) : Bool :=
  matrix.length == rows && allRowsLength cols matrix

def intModZero (q : Int) (value : Int) : Bool :=
  value.emod q == 0

def rejectionLimit (q : Nat) : Nat :=
  65535 - (65535 % q)

def u32 (value : Nat) : UInt32 :=
  UInt32.ofNat value

def rotr32 (value : UInt32) (bits : Nat) : UInt32 :=
  value.shiftRight (u32 bits) ||| value.shiftLeft (u32 (32 - bits))

def shr32 (value : UInt32) (bits : Nat) : UInt32 :=
  value.shiftRight (u32 bits)

def ch32 (x y z : UInt32) : UInt32 :=
  (x &&& y) ^^^ ((~~~x) &&& z)

def maj32 (x y z : UInt32) : UInt32 :=
  (x &&& y) ^^^ (x &&& z) ^^^ (y &&& z)

def bigSigma0 (x : UInt32) : UInt32 :=
  rotr32 x 2 ^^^ rotr32 x 13 ^^^ rotr32 x 22

def bigSigma1 (x : UInt32) : UInt32 :=
  rotr32 x 6 ^^^ rotr32 x 11 ^^^ rotr32 x 25

def smallSigma0 (x : UInt32) : UInt32 :=
  rotr32 x 7 ^^^ rotr32 x 18 ^^^ shr32 x 3

def smallSigma1 (x : UInt32) : UInt32 :=
  rotr32 x 17 ^^^ rotr32 x 19 ^^^ shr32 x 10

def sha256InitialHash : List UInt32 :=
  [ u32 0x6a09e667, u32 0xbb67ae85, u32 0x3c6ef372, u32 0xa54ff53a
  , u32 0x510e527f, u32 0x9b05688c, u32 0x1f83d9ab, u32 0x5be0cd19
  ]

def sha256RoundConstants : List UInt32 :=
  [ u32 0x428a2f98, u32 0x71374491, u32 0xb5c0fbcf, u32 0xe9b5dba5
  , u32 0x3956c25b, u32 0x59f111f1, u32 0x923f82a4, u32 0xab1c5ed5
  , u32 0xd807aa98, u32 0x12835b01, u32 0x243185be, u32 0x550c7dc3
  , u32 0x72be5d74, u32 0x80deb1fe, u32 0x9bdc06a7, u32 0xc19bf174
  , u32 0xe49b69c1, u32 0xefbe4786, u32 0x0fc19dc6, u32 0x240ca1cc
  , u32 0x2de92c6f, u32 0x4a7484aa, u32 0x5cb0a9dc, u32 0x76f988da
  , u32 0x983e5152, u32 0xa831c66d, u32 0xb00327c8, u32 0xbf597fc7
  , u32 0xc6e00bf3, u32 0xd5a79147, u32 0x06ca6351, u32 0x14292967
  , u32 0x27b70a85, u32 0x2e1b2138, u32 0x4d2c6dfc, u32 0x53380d13
  , u32 0x650a7354, u32 0x766a0abb, u32 0x81c2c92e, u32 0x92722c85
  , u32 0xa2bfe8a1, u32 0xa81a664b, u32 0xc24b8b70, u32 0xc76c51a3
  , u32 0xd192e819, u32 0xd6990624, u32 0xf40e3585, u32 0x106aa070
  , u32 0x19a4c116, u32 0x1e376c08, u32 0x2748774c, u32 0x34b0bcb5
  , u32 0x391c0cb3, u32 0x4ed8aa4a, u32 0x5b9cca4f, u32 0x682e6ff3
  , u32 0x748f82ee, u32 0x78a5636f, u32 0x84c87814, u32 0x8cc70208
  , u32 0x90befffa, u32 0xa4506ceb, u32 0xbef9a3f7, u32 0xc67178f2
  ]

def byteAt (bytes : List Nat) (idx : Nat) : Nat :=
  match bytes.drop idx with
  | byte :: _ => byte
  | [] => 0

def u32At (words : List UInt32) (idx : Nat) : UInt32 :=
  match words.drop idx with
  | word :: _ => word
  | [] => 0

def u32FromBigEndianBytes (bytes : List Nat) (idx : Nat) : UInt32 :=
  u32 ((byteAt bytes idx) * 16777216 +
    (byteAt bytes (idx + 1)) * 65536 +
    (byteAt bytes (idx + 2)) * 256 +
    byteAt bytes (idx + 3))

def u64BigEndianBytes (value : Nat) : List Nat :=
  [ (value / 72057594037927936) % 256
  , (value / 281474976710656) % 256
  , (value / 1099511627776) % 256
  , (value / 4294967296) % 256
  , (value / 16777216) % 256
  , (value / 65536) % 256
  , (value / 256) % 256
  , value % 256
  ]

def sha256PaddingZeroCount (byteLen : Nat) : Nat :=
  (55 + 64 - (byteLen % 64)) % 64

def sha256Pad (bytes : List Nat) : List Nat :=
  bytes ++ [128] ++ List.replicate (sha256PaddingZeroCount bytes.length) 0 ++
    u64BigEndianBytes (bytes.length * 8)

def chunk64At (bytes : List Nat) (chunk : Nat) : List Nat :=
  (bytes.drop (chunk * 64)).take 64

def sha256BlockCount (bytes : List Nat) : Nat :=
  (sha256Pad bytes).length / 64

def sha256InitialSchedule (block : List Nat) : List UInt32 :=
  (List.range 16).map fun idx => u32FromBigEndianBytes block (idx * 4)

partial def sha256ExtendScheduleFrom (idx : Nat) (words : List UInt32) : List UInt32 :=
  if idx < 64 then
    let next :=
      smallSigma1 (u32At words (idx - 2)) +
      u32At words (idx - 7) +
      smallSigma0 (u32At words (idx - 15)) +
      u32At words (idx - 16)
    sha256ExtendScheduleFrom (idx + 1) (words ++ [next])
  else
    words

def sha256Schedule (block : List Nat) : List UInt32 :=
  sha256ExtendScheduleFrom 16 (sha256InitialSchedule block)

structure Sha256State where
  a : UInt32
  b : UInt32
  c : UInt32
  d : UInt32
  e : UInt32
  f : UInt32
  g : UInt32
  h : UInt32
deriving DecidableEq, Repr

def stateFromHash (hash : List UInt32) : Sha256State where
  a := u32At hash 0
  b := u32At hash 1
  c := u32At hash 2
  d := u32At hash 3
  e := u32At hash 4
  f := u32At hash 5
  g := u32At hash 6
  h := u32At hash 7

def hashFromState (state : Sha256State) : List UInt32 :=
  [state.a, state.b, state.c, state.d, state.e, state.f, state.g, state.h]

def sha256Round (schedule : List UInt32) (idx : Nat) (state : Sha256State) :
    Sha256State :=
  let t1 :=
    state.h + bigSigma1 state.e + ch32 state.e state.f state.g +
      u32At sha256RoundConstants idx + u32At schedule idx
  let t2 := bigSigma0 state.a + maj32 state.a state.b state.c
  { a := t1 + t2
    b := state.a
    c := state.b
    d := state.c
    e := state.d + t1
    f := state.e
    g := state.f
    h := state.g }

partial def sha256RoundsFrom
    (schedule : List UInt32)
    (idx : Nat)
    (state : Sha256State) : Sha256State :=
  if idx < 64 then
    sha256RoundsFrom schedule (idx + 1) (sha256Round schedule idx state)
  else
    state

def sha256Compress (hash : List UInt32) (block : List Nat) : List UInt32 :=
  let old := stateFromHash hash
  let compressed := sha256RoundsFrom (sha256Schedule block) 0 old
  [ u32At hash 0 + compressed.a
  , u32At hash 1 + compressed.b
  , u32At hash 2 + compressed.c
  , u32At hash 3 + compressed.d
  , u32At hash 4 + compressed.e
  , u32At hash 5 + compressed.f
  , u32At hash 6 + compressed.g
  , u32At hash 7 + compressed.h
  ]

partial def sha256BlocksFrom
    (padded : List Nat)
    (idx blockCount : Nat)
    (hash : List UInt32) : List UInt32 :=
  if idx < blockCount then
    sha256BlocksFrom padded (idx + 1) blockCount
      (sha256Compress hash (chunk64At padded idx))
  else
    hash

def u32ToBigEndianBytes (word : UInt32) : List Nat :=
  let value := word.toNat
  [ (value / 16777216) % 256
  , (value / 65536) % 256
  , (value / 256) % 256
  , value % 256
  ]

def sha256DigestBytes (bytes : List Nat) : List Nat :=
  let padded := sha256Pad bytes
  let hash := sha256BlocksFrom padded 0 (padded.length / 64) sha256InitialHash
  hash.flatMap u32ToBigEndianBytes

theorem sha256_empty_known_answer :
    sha256DigestBytes [] =
      [ 0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14
      , 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24
      , 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c
      , 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55
      ] := by
  native_decide

theorem sha256_abc_known_answer :
    sha256DigestBytes [0x61, 0x62, 0x63] =
      [ 0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea
      , 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23
      , 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c
      , 0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad
      ] := by
  native_decide

def bytesWithinUInt8 : List Nat -> Bool
  | [] => true
  | byte :: rest => byte <= 255 && bytesWithinUInt8 rest

def leBytes16 (value : Nat) : List Nat :=
  [value % 256, (value / 256) % 256]

def leBytes32 (value : Nat) : List Nat :=
  [ value % 256
  , (value / 256) % 256
  , (value / 65536) % 256
  , (value / 16777216) % 256
  ]

def matrixEntryMessageBytes
    (domain : List Nat)
    (row col counter : Nat) : List Nat :=
  domain ++ leBytes16 row ++ leBytes16 col ++ leBytes32 counter

def digestLittleEndianU16Words (digest : List Nat) : List Nat :=
  (List.range 16).map fun idx =>
    byteAt digest (idx * 2) + 256 * byteAt digest (idx * 2 + 1)

partial def sha256EntrySamplesFrom
    (domain : List Nat)
    (row col counter remainingWords : Nat) : List Nat :=
  if remainingWords = 0 then
    []
  else
    let digest := sha256DigestBytes (matrixEntryMessageBytes domain row col counter)
    let words := digestLittleEndianU16Words digest
    if remainingWords <= 16 then
      words.take remainingWords
    else
      words ++ sha256EntrySamplesFrom domain row col (counter + 1) (remainingWords - 16)

def sha256EntrySamplesForWitness
    (domain : List Nat)
    (row col : Nat)
    (witness : List Nat) : List Nat :=
  sha256EntrySamplesFrom domain row col 0 witness.length

def sha256RowSamplesForWitness
    (domain : List Nat)
    (row : Nat)
    (witnessRow : List (List Nat)) : List (List Nat) :=
  (List.range witnessRow.length).map fun col =>
    sha256EntrySamplesForWitness domain row col
      (match witnessRow.drop col with
      | samples :: _ => samples
      | [] => [])

def sha256MatrixSamplesForWitness
    (domain : List Nat)
    (witness : List (List (List Nat))) : List (List (List Nat)) :=
  (List.range witness.length).map fun row =>
    sha256RowSamplesForWitness domain row
      (match witness.drop row with
      | witnessRow :: _ => witnessRow
      | [] => [])

def firstAcceptedResidue? (q : Nat) : List Nat -> Option Int
  | [] => none
  | sample :: rest =>
      if sample < rejectionLimit q then
        some (Int.ofNat (sample % q))
      else
        firstAcceptedResidue? q rest

def rowFromRejectionWitness? (q : Nat) :
    List (List Nat) -> Option (List Int)
  | [] => some []
  | samples :: rest =>
      match firstAcceptedResidue? q samples, rowFromRejectionWitness? q rest with
      | some value, some values => some (value :: values)
      | _, _ => none

def matrixFromRejectionWitness? (q : Nat) :
    List (List (List Nat)) -> Option (List (List Int))
  | [] => some []
  | row :: rest =>
      match rowFromRejectionWitness? q row, matrixFromRejectionWitness? q rest with
      | some values, some rows => some (values :: rows)
      | _, _ => none

def samplesWithinUInt16 : List Nat -> Bool
  | [] => true
  | sample :: rest => sample <= 65535 && samplesWithinUInt16 rest

def rejectionWitnessWithinUInt16 : List (List (List Nat)) -> Bool :=
  fun rows => rows.all fun row => row.all fun samples => samplesWithinUInt16 samples

def getAt? : List Int -> Nat -> Option Int
  | [], _ => none
  | x :: _, 0 => some x
  | _ :: xs, n + 1 => getAt? xs n

def rowSatisfiesMatrix (q : Int) (matrix : List (List Int)) (row : List Int) : Bool :=
  matrix.all fun matrixRow => intModZero q (dotInt matrixRow row)

def coordsFixedModQ (q : Int) (coords : List Nat) (row : List Int) : Bool :=
  coords.all fun coord =>
    match getAt? row coord with
    | some value => intModZero q value
    | none => false

def rowSatisfiesResponseLink
    (q : Int)
    (matrix : List (List Int))
    (coords : List Nat)
    (row : List Int) : Bool :=
  rowSatisfiesMatrix q matrix row && coordsFixedModQ q coords row

def basisSatisfiesResponseLink
    (q : Int)
    (matrix : List (List Int))
    (coords : List Nat)
    (basis : List (List Int)) : Bool :=
  basis.all fun row => rowSatisfiesResponseLink q matrix coords row

def rat (num : Int) (den : Nat) (h : den ≠ 0 := by decide) : Rat :=
  Rat.normalize num den h

def floorRat (value : Rat) : Int :=
  value.num / Int.ofNat value.den

def ceilRat (value : Rat) : Int :=
  -floorRat (-value)

partial def isqrtSearch (n lo hi : Nat) : Nat :=
  if hi <= lo + 1 then
    lo
  else
    let mid := (lo + hi) / 2
    if mid * mid <= n then
      isqrtSearch n mid hi
    else
      isqrtSearch n lo mid

def isqrt (n : Nat) : Nat :=
  isqrtSearch n 0 (n + 1)

def ceilRatToNat (value : Rat) : Nat :=
  Int.toNat (ceilRat value)

def ratSquare (value : Rat) : Rat :=
  value * value

def getIntAt : List Int -> Nat -> Int
  | [], _ => 0
  | x :: _, 0 => x
  | _ :: xs, n + 1 => getIntAt xs n

def getRatAt : List Rat -> Nat -> Rat
  | [], _ => 0
  | x :: _, 0 => x
  | _ :: xs, n + 1 => getRatAt xs n

def getIntRowAt : List (List Int) -> Nat -> List Int
  | [], _ => []
  | x :: _, 0 => x
  | _ :: xs, n + 1 => getIntRowAt xs n

def getRatRowAt : List (List Rat) -> Nat -> List Rat
  | [], _ => []
  | x :: _, 0 => x
  | _ :: xs, n + 1 => getRatRowAt xs n

def getRat2 (rows : List (List Rat)) (row col : Nat) : Rat :=
  getRatAt (getRatRowAt rows row) col

def setIntAt : List Int -> Nat -> Int -> List Int
  | [], _, _ => []
  | _ :: xs, 0, value => value :: xs
  | x :: xs, n + 1, value => x :: setIntAt xs n value

def addScaledRows : List Int -> Int -> List Int -> List Int
  | x :: xs, coeff, y :: ys => (x + coeff * y) :: addScaledRows xs coeff ys
  | [], coeff, y :: ys => coeff * y :: addScaledRows [] coeff ys
  | xs, _, [] => xs

partial def materializeWithIndex
    (basis : List (List Int))
    (coeffs : List Int)
    (idx : Nat)
    (acc : List Int) : List Int :=
  match coeffs with
  | [] => acc
  | coeff :: rest =>
      let row := getIntRowAt basis idx
      materializeWithIndex basis rest (idx + 1) (addScaledRows acc coeff row)

def materialize (basis : List (List Int)) (coeffs : List Int) : List Int :=
  materializeWithIndex basis coeffs 0 []

def transformBasis (transform : List (List Int)) (basis : List (List Int)) :
    List (List Int) :=
  transform.map fun coeffs => materialize basis coeffs

def sqInt (value : Int) : Nat :=
  value.natAbs * value.natAbs

def normSq : List Int -> Nat
  | [] => 0
  | x :: xs => sqInt x + normSq xs

def nonzeroVector : List Int -> Bool
  | [] => false
  | x :: xs => (x != 0) || nonzeroVector xs

structure EnumStats where
  visitedNodes : Nat
  visitedLeaves : Nat
  nonzeroInCover : Nat
  minCoeffBound : Option Nat
  boundedWitnessFound : Bool
deriving Repr, DecidableEq

def EnumStats.empty : EnumStats where
  visitedNodes := 0
  visitedLeaves := 0
  nonzeroInCover := 0
  minCoeffBound := none
  boundedWitnessFound := false

def EnumStats.visitNode (stats : EnumStats) : EnumStats :=
  { stats with visitedNodes := stats.visitedNodes + 1 }

def EnumStats.visitLeaf (stats : EnumStats) : EnumStats :=
  { stats with visitedLeaves := stats.visitedLeaves + 1 }

def minOptionNat : Option Nat -> Nat -> Option Nat
  | none, value => some value
  | some old, value => some (min old value)

def EnumStats.recordCoverVector
    (stats : EnumStats)
    (bound coeffBound : Nat) : EnumStats :=
  { stats with
    nonzeroInCover := stats.nonzeroInCover + 1
    minCoeffBound := minOptionNat stats.minCoeffBound coeffBound
    boundedWitnessFound := stats.boundedWitnessFound || coeffBound <= bound }

partial def offsetFrom
    (mu : List (List Rat))
    (coeffs : List Int)
    (row idx dimension : Nat) : Rat :=
  if idx < dimension then
    (Rat.ofInt (getIntAt coeffs idx)) * getRat2 mu idx row +
      offsetFrom mu coeffs row (idx + 1) dimension
  else
    0

partial def tightenLo (offset diag remaining : Rat) (lo hi : Int) : Int :=
  if lo <= hi && ratSquare ((Rat.ofInt lo) + offset) * diag > remaining then
    tightenLo offset diag remaining (lo + 1) hi
  else
    lo

partial def tightenHi (offset diag remaining : Rat) (lo hi : Int) : Int :=
  if lo <= hi && ratSquare ((Rat.ofInt hi) + offset) * diag > remaining then
    tightenHi offset diag remaining lo (hi - 1)
  else
    hi

def exactInterval (offset diag remaining : Rat) : Int × Int :=
  let cover := remaining / diag
  let sqrtUpper := isqrt (ceilRatToNat cover) + 2
  let center := -offset
  let rawLo := ceilRat (center - Rat.ofInt (Int.ofNat sqrtUpper))
  let rawHi := floorRat (center + Rat.ofInt (Int.ofNat sqrtUpper))
  let lo := tightenLo offset diag remaining rawLo rawHi
  let hi := tightenHi offset diag remaining lo rawHi
  (lo, hi)

mutual

partial def enumCoeffRange
    (basis : List (List Int))
    (mu : List (List Rat))
    (diagonal : List Rat)
    (bound radiusSq dimension rowPlusOne : Nat)
    (coeffs : List Int)
    (partialNorm : Rat)
    (lo hi : Int)
    (stats : EnumStats) : EnumStats :=
  if lo > hi then
    stats
  else
    let row := rowPlusOne - 1
    let coeffs' := setIntAt coeffs row lo
    let offset := offsetFrom mu coeffs' row rowPlusOne dimension
    let diag := getRatAt diagonal row
    let term := ratSquare ((Rat.ofInt lo) + offset) * diag
    let stats' :=
      enumerateFrom basis mu diagonal bound radiusSq dimension row coeffs'
        (partialNorm + term) stats
    enumCoeffRange basis mu diagonal bound radiusSq dimension rowPlusOne coeffs
      partialNorm (lo + 1) hi stats'

partial def enumerateFrom
    (basis : List (List Int))
    (mu : List (List Rat))
    (diagonal : List Rat)
    (bound radiusSq dimension rowPlusOne : Nat)
    (coeffs : List Int)
    (partialNorm : Rat)
    (stats : EnumStats) : EnumStats :=
  let stats := stats.visitNode
  if rowPlusOne = 0 then
    let stats := stats.visitLeaf
    if coeffs.all fun coeff => coeff == 0 then
      stats
    else
      let vector := materialize basis coeffs
      if normSq vector <= radiusSq then
        stats.recordCoverVector bound (maxAbsCoeff vector)
      else
        stats
  else
    let remaining := (Rat.ofInt (Int.ofNat radiusSq)) - partialNorm
    if remaining < 0 then
      stats
    else
      let row := rowPlusOne - 1
      let offset := offsetFrom mu coeffs row rowPlusOne dimension
      let diag := getRatAt diagonal row
      let interval := exactInterval offset diag remaining
      enumCoeffRange basis mu diagonal bound radiusSq dimension rowPlusOne coeffs
        partialNorm interval.1 interval.2 stats

end

def exactEnumerationStats
    (basis : List (List Int))
    (mu : List (List Rat))
    (diagonal : List Rat)
    (bound dimension : Nat) : EnumStats :=
  enumerateFrom basis mu diagonal bound (23 * bound * bound) dimension dimension
    (List.replicate dimension 0) 0 EnumStats.empty

def statusInfeasible (C : BoundedKernelExactSummary) : Prop :=
  C.status = "infeasible"

def excludesBoundedNonzeroKernel (C : BoundedKernelExactSummary) : Prop :=
  C.responseDifferenceBound < C.minimumCoverCoeffBound

theorem no_vector_with_coeff_bound_at_most_response_bound
    (C : BoundedKernelExactSummary)
    (hgap : C.excludesBoundedNonzeroKernel)
    {coeffBound : Nat}
    (hle : coeffBound <= C.responseDifferenceBound)
    (hshort : C.minimumCoverCoeffBound <= coeffBound) :
    False :=
  Nat.not_lt_of_ge (Nat.le_trans hshort hle) hgap

end BoundedKernelExactSummary

end ResponseLinkExactCertificate
end Descender
