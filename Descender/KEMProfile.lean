import Descender.KEMCertificate
import Descender.StrategyComparison

namespace Descender

/-!
End-to-end KEM profile certificates.

This module ties the proof-strategy layer to the system-level security
certificate.  It intentionally keeps finite-field validation as proof-carrying
external evidence: the Rust validators can produce the facts, but Lean should
not treat a parameter row as a deployable KEM profile until those facts are
present as certificate fields.
-/

namespace SampleView

variable {q : Nat}
variable {FullCoord ViewCoord Secret Noise : Type*}

/--
Reusable certificate payload for the exact support-covered strategy.

The long theorem in `StrategyComparison.lean` is the constructor; this structure
names the obligations that a concrete selected-coordinate family must satisfy.
-/
structure SupportCoveredStrategyObligations
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport eta failureBudget : Nat) where
  hMinSupport : 0 < minSupport
  hCover :
    ∀ pair : Secret × Secret,
      pair.1 ≠ pair.2 ->
      minSupport <=
        weight (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i) ->
      SupportCoveredByView (q := q) select
        (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i)
  hNoise :
    ∀ seed : FullDiagonalSeed q ViewCoord,
      (scaledCoordinateRestriction
        (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmall P eta
  hSupport :
    ∀ s t, s ≠ t ->
      minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i)
  hFailureBudget :
    ((Finset.univ : Finset Secret).product
      (Finset.univ : Finset Secret)).card *
      ((2 * (2 * eta) + 1) *
        Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
      failureBudget
  hBudget : failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)

namespace SupportCoveredStrategyObligations

def certificate
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    {select : ViewCoord -> FullCoord}
    {P : P_TND_Decision q FullCoord Secret Noise}
    {minSupport eta failureBudget : Nat}
    (O : SupportCoveredStrategyObligations
      (q := q) select P minSupport eta failureBudget)
    (costs : StrategyCostProfile) :
    StrategyCertificate q ViewCoord Secret Noise
      (FullDiagonalSeed q ViewCoord) :=
  supportCoveredStrategyCertificate (q := q) costs select P minSupport eta
    failureBudget O.hMinSupport O.hCover O.hNoise O.hSupport
    O.hFailureBudget O.hBudget

end SupportCoveredStrategyObligations

/--
Reusable certificate payload for the lossy-budget strategy.

This is the proof-side counterpart of the search parameters `sample_bits`,
`view_coords`, and the resulting effective noise radius.
-/
structure LossyBudgetStrategyObligations
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord]
    (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (B : ViewReductionBudget) where
  hNoise : V.NoiseSmallWithBudget P B
  hSeparated : V.EvalSeparatedWithBudget P B

namespace LossyBudgetStrategyObligations

def certificate
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord]
    {V : SampleView q FullCoord ViewCoord}
    {P : P_TND_Decision q FullCoord Secret Noise}
    {B : ViewReductionBudget}
    (O : LossyBudgetStrategyObligations V P B)
    (costs : StrategyCostProfile) :
    StrategyCertificate q ViewCoord Secret Noise PUnit :=
  lossyBudgetStrategyCertificate (q := q) costs V P B
    O.hNoise O.hSeparated

end LossyBudgetStrategyObligations

/--
Reusable payload for a lossy-budget strategy whose viewed separation is
certified by public diagonal amplification.

This is the certificate shape that corresponds to compressed public views with
sample quantization: the search must provide the support-distance lower bound,
the support-covered selected view, a viewed-noise bound at `B.viewEta`, and a
bad-diagonal budget below the diagonal seed space.
-/
structure LossyBudgetDiagonalStrategyObligations
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget) where
  hMinSupport : 0 < minSupport
  hCover :
    ∀ pair : Secret × Secret,
      pair.1 ≠ pair.2 ->
      minSupport <=
        weight (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i) ->
      SupportCoveredByView (q := q) select
        (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i)
  hNoise :
    ∀ seed : FullDiagonalSeed q ViewCoord,
      (scaledCoordinateRestriction
        (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmallWithBudget
        P B
  hSupport :
    ∀ s t, s ≠ t ->
      minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i)
  hFailureBudget :
    ((Finset.univ : Finset Secret).product
      (Finset.univ : Finset Secret)).card *
      ((2 * (2 * B.viewEta) + 1) *
        Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
      B.failureBudget
  hBudget : B.failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)

namespace LossyBudgetDiagonalStrategyObligations

def certificate
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    {select : ViewCoord -> FullCoord}
    {P : P_TND_Decision q FullCoord Secret Noise}
    {minSupport : Nat} {B : ViewReductionBudget}
    (O : LossyBudgetDiagonalStrategyObligations
      (q := q) select P minSupport B)
    (costs : StrategyCostProfile) :
    StrategyCertificate q ViewCoord Secret Noise
      (FullDiagonalSeed q ViewCoord) :=
  lossyBudgetSupportCoveredStrategyCertificate
    (q := q) costs select P minSupport B O.hMinSupport O.hCover O.hNoise
    O.hSupport O.hFailureBudget O.hBudget

end LossyBudgetDiagonalStrategyObligations

/--
Generic payload for a lossy-budget diagonal strategy.

Use this shape when the bad-seed count comes from a sharper argument than the
simple support-covered one-coordinate union bound above.  This is the intended
target for multi-coordinate diagonal anti-concentration certificates suggested
by the parameter search diagnostics.
-/
structure LossyBudgetDiagonalGenericObligations
    [NeZero q]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq Seed] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget) where
  hNoise :
    ∀ seed,
      (scaledCoordinateRestriction (q := q) select (scale seed)).NoiseSmallWithBudget
        P B
  hSupport :
    ∀ s t, s ≠ t ->
      minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i)
  hFailureCount :
    DiagonalAmplificationFailureCountAtMost (q := q) select scale P.eval
      minSupport B.viewEta B.failureBudget
  hBudget : B.failureBudget < Fintype.card Seed

namespace LossyBudgetDiagonalGenericObligations

def certificate
    [NeZero q]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq Seed] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    {select : ViewCoord -> FullCoord}
    {scale : Seed -> ViewCoord -> ZMod q}
    {P : P_TND_Decision q FullCoord Secret Noise}
    {minSupport : Nat} {B : ViewReductionBudget}
    (O : LossyBudgetDiagonalGenericObligations
      (q := q) select scale P minSupport B)
    (costs : StrategyCostProfile) :
    StrategyCertificate q ViewCoord Secret Noise Seed :=
  lossyBudgetDiagonalStrategyCertificate
    (q := q) costs select scale P minSupport B O.hNoise O.hSupport
    O.hFailureCount O.hBudget

end LossyBudgetDiagonalGenericObligations

/--
Concrete payload for the multi-coordinate lossy diagonal route.

This refines `LossyBudgetDiagonalGenericObligations`: instead of asking for an
opaque bad-seed count, it asks for explicit per-pair viewed coordinates with
nonzero differences and the corresponding summed product budget.
-/
structure LossyBudgetPairCoordinateStrategyObligations
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget) where
  pairCoords : Secret × Secret -> Finset ViewCoord
  hNoise :
    ∀ seed : FullDiagonalSeed q ViewCoord,
      (scaledCoordinateRestriction
        (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmallWithBudget
        P B
  hSupport :
    ∀ s t, s ≠ t ->
      minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i)
  hNonzero :
    ∀ pair j,
      j ∈ pairCoords pair ->
        P.eval pair.1 (select j) - P.eval pair.2 (select j) ≠ 0
  hFailureBudget :
    (∑ pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
      ∏ j : ViewCoord,
        if j ∈ pairCoords pair then
          2 * (2 * B.viewEta) + 1
        else
          Fintype.card (ZMod q)) <= B.failureBudget
  hBudget : B.failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)

namespace LossyBudgetPairCoordinateStrategyObligations

def failureCount
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    {select : ViewCoord -> FullCoord}
    {P : P_TND_Decision q FullCoord Secret Noise}
    {minSupport : Nat} {B : ViewReductionBudget}
    (O : LossyBudgetPairCoordinateStrategyObligations
      (q := q) select P minSupport B) :
    DiagonalAmplificationFailureCountAtMost
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) P.eval minSupport B.viewEta
      B.failureBudget :=
  failureCountAtMost_full_of_pairwise_coordinateSets_prime
    (q := q) select P.eval minSupport B.viewEta B.failureBudget
    O.pairCoords O.hNonzero O.hFailureBudget

def certificate
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    {select : ViewCoord -> FullCoord}
    {P : P_TND_Decision q FullCoord Secret Noise}
    {minSupport : Nat} {B : ViewReductionBudget}
    (O : LossyBudgetPairCoordinateStrategyObligations
      (q := q) select P minSupport B)
    (costs : StrategyCostProfile) :
    StrategyCertificate q ViewCoord Secret Noise
      (FullDiagonalSeed q ViewCoord) :=
  lossyBudgetDiagonalStrategyCertificate
    (q := q) costs select (fullDiagonalScale (q := q)) P minSupport B
    O.hNoise O.hSupport O.failureCount O.hBudget

end LossyBudgetPairCoordinateStrategyObligations

/--
Uniform per-pair-budget variant of
`LossyBudgetPairCoordinateStrategyObligations`.

This is the most compact certificate target for parameter-search output: the
search reports a single `pairBudget` and proves every pair-coordinate product
is below it.
-/
structure LossyBudgetUniformPairCoordinateStrategyObligations
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget) where
  pairCoords : Secret × Secret -> Finset ViewCoord
  pairBudget : Nat
  hNoise :
    ∀ seed : FullDiagonalSeed q ViewCoord,
      (scaledCoordinateRestriction
        (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmallWithBudget
        P B
  hSupport :
    ∀ s t, s ≠ t ->
      minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i)
  hNonzero :
    ∀ pair j,
      j ∈ pairCoords pair ->
        P.eval pair.1 (select j) - P.eval pair.2 (select j) ≠ 0
  hPairBudget :
    ∀ pair,
      (∏ j : ViewCoord,
        if j ∈ pairCoords pair then
          2 * (2 * B.viewEta) + 1
        else
          Fintype.card (ZMod q)) <= pairBudget
  hFailureBudget :
    ((Finset.univ : Finset Secret).product
      (Finset.univ : Finset Secret)).card * pairBudget <= B.failureBudget
  hBudget : B.failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)

namespace LossyBudgetUniformPairCoordinateStrategyObligations

def failureCount
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    {select : ViewCoord -> FullCoord}
    {P : P_TND_Decision q FullCoord Secret Noise}
    {minSupport : Nat} {B : ViewReductionBudget}
    (O : LossyBudgetUniformPairCoordinateStrategyObligations
      (q := q) select P minSupport B) :
    DiagonalAmplificationFailureCountAtMost
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) P.eval minSupport B.viewEta
      B.failureBudget :=
  failureCountAtMost_full_of_pairwise_coordinateSets_uniform_prime
    (q := q) select P.eval minSupport B.viewEta B.failureBudget O.pairBudget
    O.pairCoords O.hNonzero O.hPairBudget O.hFailureBudget

def certificate
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    {select : ViewCoord -> FullCoord}
    {P : P_TND_Decision q FullCoord Secret Noise}
    {minSupport : Nat} {B : ViewReductionBudget}
    (O : LossyBudgetUniformPairCoordinateStrategyObligations
      (q := q) select P minSupport B)
    (costs : StrategyCostProfile) :
    StrategyCertificate q ViewCoord Secret Noise
      (FullDiagonalSeed q ViewCoord) :=
  lossyBudgetDiagonalStrategyCertificate
    (q := q) costs select (fullDiagonalScale (q := q)) P minSupport B
    O.hNoise O.hSupport O.failureCount O.hBudget

end LossyBudgetUniformPairCoordinateStrategyObligations

end SampleView

/-- Number of bytes needed to pack `entries` fixed-width `bits` fields. -/
def packedBytes (entries bits : Nat) : Nat :=
  (entries * bits + 7) / 8

/--
Serialization facts for a public sample view.

The rounding bound is the arithmetic link between the bit-level encoder and the
proof-side centered-noise radius: if a coordinate modulo `q` is rounded to
`sampleBits` payload bits, then this profile records the `roundingEta` that the
correctness proof must add to the original noise radius.
-/
structure PublicSampleSerializationProfile where
  q : Nat
  coordinateCount : Nat
  sampleBits : Nat
  roundingEta : Nat
  seedBytes : Nat
  ciphertextOverheadBytes : Nat
  encodedSampleBytes : Nat
  publicKeyBytes : Nat
  ciphertextBytes : Nat
  hEncodedSampleBytes :
    encodedSampleBytes = packedBytes coordinateCount sampleBits
  hPublicKeyBytes : publicKeyBytes = seedBytes + encodedSampleBytes
  hCiphertextBytes : ciphertextBytes = encodedSampleBytes + ciphertextOverheadBytes
  hRoundingBound : q <= roundingEta * 2 ^ (sampleBits + 1)

namespace PublicSampleSerializationProfile

def effectiveEta (S : PublicSampleSerializationProfile) (eta : Nat) : Nat :=
  eta + S.roundingEta

theorem rounding_bound (S : PublicSampleSerializationProfile) :
    S.q <= S.roundingEta * 2 ^ (S.sampleBits + 1) :=
  S.hRoundingBound

end PublicSampleSerializationProfile

/--
Current proof-carrying 128-bit candidate:

* `q = 32771`;
* `n = 64`, `m0 = 104`, `m1 = 32`;
* eta-8 product-block Lee certificate with 64 public rows;
* 12-bit sample serialization, charged as `roundingEta = 5`;
* effective correctness/amplitude radius `13`;
* 268-byte public key and 236-byte ciphertext in the current compact profile.
-/
namespace Descender128ProductBlockCompressedProfile

abbrev q : Nat := 32771
abbrev n : Nat := 64
abbrev m0 : Nat := 104
abbrev m1 : Nat := 32
abbrev secretBits : Nat := 112
abbrev eta : Nat := 8
abbrev sampleBits : Nat := 12
abbrev roundingEta : Nat := 5
abbrev effectiveEta : Nat := eta + roundingEta
abbrev productBlockRows : Nat := 64
abbrev certifiedLeeTarget : Nat := 2049
abbrev seedBytes : Nat := 64
abbrev ciphertextOverheadBytes : Nat := 32

def serialization : PublicSampleSerializationProfile where
  q := q
  coordinateCount := m0 + m1
  sampleBits := sampleBits
  roundingEta := roundingEta
  seedBytes := seedBytes
  ciphertextOverheadBytes := ciphertextOverheadBytes
  encodedSampleBytes := 204
  publicKeyBytes := 268
  ciphertextBytes := 236
  hEncodedSampleBytes := by native_decide
  hPublicKeyBytes := by native_decide
  hCiphertextBytes := by native_decide
  hRoundingBound := by native_decide

def viewBudget : SampleView.ViewReductionBudget where
  fullEta := eta
  viewEta := effectiveEta
  failureBudget := 0
  eta_le_viewEta := by native_decide

theorem effectiveEta_eq : effectiveEta = 13 := by
  native_decide

theorem encodedSampleBytes_eq :
    serialization.encodedSampleBytes = 204 := by
  native_decide

theorem publicKeyBytes_eq :
    serialization.publicKeyBytes = 268 := by
  native_decide

theorem ciphertextBytes_eq :
    serialization.ciphertextBytes = 236 := by
  native_decide

theorem rounding_bound :
    q <= roundingEta * 2 ^ (sampleBits + 1) := by
  native_decide

theorem product_block_amplitude_bound :
    q <= 2 * eta * certifiedLeeTarget + 1 := by
  native_decide

theorem effective_amplitude_bound :
    q <= 2 * effectiveEta * certifiedLeeTarget + 1 := by
  native_decide

theorem correctness_distance_eq :
    m0 - n + 1 = 41 := by
  native_decide

theorem effective_correctness_radius_eq :
    2 * effectiveEta + 1 = 27 := by
  native_decide

theorem correctness_slack_eq :
    (m0 - n + 1) - (2 * effectiveEta + 1) = 14 := by
  native_decide

end Descender128ProductBlockCompressedProfile

namespace ExactValidationCertificate

inductive PrimalDistanceModel where
  | guardOnly
  | combined
deriving DecidableEq, Repr

end ExactValidationCertificate

/--
Exact validation evidence for a concrete parameter row.

The propositions are intentionally abstract because the corresponding checks may
come from exact finite-field computation, bounded exhaustive search, or later
Lean proofs.  The certificate is proof-carrying: a profile cannot include a
validation claim without providing witnesses for these propositions.
-/
structure ExactValidationCertificate where
  strategy : SampleView.StrategyKind
  primalDistanceModel : ExactValidationCertificate.PrimalDistanceModel
  fullCoordinateCount : Nat
  viewedCoordinateCount : Nat
  fieldRankValidation : Prop
  projectedDistanceValidation : Prop
  boundedDualWitnessValidation : Prop
  parameterRowValidation : Prop
  fieldRankValid : fieldRankValidation
  projectedDistanceValid : projectedDistanceValidation
  boundedDualWitnessValid : boundedDualWitnessValidation
  parameterRowValid : parameterRowValidation
  view_le_full : viewedCoordinateCount <= fullCoordinateCount

namespace ExactValidationCertificate

def isFullView (C : ExactValidationCertificate) : Prop :=
  C.viewedCoordinateCount = C.fullCoordinateCount

def hasCompressedView (C : ExactValidationCertificate) : Prop :=
  C.viewedCoordinateCount < C.fullCoordinateCount

theorem compressed_view_le_full
    (C : ExactValidationCertificate) (h : C.hasCompressedView) :
    C.viewedCoordinateCount <= C.fullCoordinateCount :=
  Nat.le_of_lt h

def usesGuardOnlyDistance (C : ExactValidationCertificate) : Prop :=
  C.primalDistanceModel = PrimalDistanceModel.guardOnly

def usesCombinedDistance (C : ExactValidationCertificate) : Prop :=
  C.primalDistanceModel = PrimalDistanceModel.combined

end ExactValidationCertificate

/--
A proof-strategy certificate plus exact finite-field validation for the same
strategy row.
-/
structure ValidatedStrategyCertificate
    (q : Nat) [NeZero q] (Coord Secret Noise Seed : Type*) [Fintype Coord] where
  strategy : SampleView.StrategyCertificate q Coord Secret Noise Seed
  exact : ExactValidationCertificate
  strategy_matches :
    exact.strategy = strategy.kind

namespace ValidatedStrategyCertificate

@[reducible] def correctness
    {q : Nat} [NeZero q] {Coord Secret Noise Seed : Type*} [Fintype Coord]
    (C : ValidatedStrategyCertificate q Coord Secret Noise Seed) :
    Nonempty (Σ _seed : Seed, PTNDCorrectnessCertificate q Coord Secret Noise) :=
  C.strategy.correctness

theorem publicBits_le_budget
    {q : Nat} [NeZero q] {Coord Secret Noise Seed : Type*} [Fintype Coord]
    (C : ValidatedStrategyCertificate q Coord Secret Noise Seed)
    (B : SampleView.StrategyCostProfile)
    (h : C.strategy.costs.BoundedBy B) :
    C.strategy.publicBits <= B.publicSize.totalBits :=
  C.strategy.publicBits_le_budget B h

end ValidatedStrategyCertificate

/--
The deployable-profile boundary: correctness strategy, exact validation, and
system-level KEM security must all be present.
-/
structure CertifiedKEMProfile
    (q : Nat) [NeZero q] (Coord Secret Noise Seed : Type) [Fintype Coord]
    (SetupSeed Parameters Description Step PrivateState Measurement
      PrimalWitness DualTarget : Type)
    [Zero DualTarget] where
  validatedStrategy :
    ValidatedStrategyCertificate q Coord Secret Noise Seed
  kem :
    DescenderKEMCertificate q Coord Secret Noise SetupSeed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget

namespace CertifiedKEMProfile

variable {q : Nat} [NeZero q] {Coord Secret Noise Seed : Type}
variable [Fintype Coord]
variable {SetupSeed Parameters Description Step PrivateState Measurement
  PrimalWitness DualTarget : Type}
variable [Zero DualTarget]

theorem ind_cca_secure
    (C : CertifiedKEMProfile q Coord Secret Noise Seed SetupSeed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    NoBreak C.kem.BreaksCCA :=
  C.kem.ind_cca_secure

theorem average_ptnd_hard
    (C : CertifiedKEMProfile q Coord Secret Noise Seed SetupSeed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    Hard C.kem.core.SolvesAveragePTND :=
  C.kem.average_ptnd_hard

theorem no_narrow_qary_dual_relations_at_noise
    (C : CertifiedKEMProfile q Coord Secret Noise Seed SetupSeed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    NoNarrowQaryDualRelations q C.kem.noisePromise.eta
      C.kem.qaryPromise.dualMap :=
  C.kem.no_narrow_qary_dual_relations_at_noise

@[reducible] def correctness
    (C : CertifiedKEMProfile q Coord Secret Noise Seed SetupSeed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    Nonempty (Σ _seed : Seed, PTNDCorrectnessCertificate q Coord Secret Noise) :=
  C.validatedStrategy.correctness

def securityCertificate
    (C : CertifiedKEMProfile q Coord Secret Noise Seed SetupSeed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    DescenderKEMSecurityCertificate :=
  C.kem.kemSecurityCertificate

end CertifiedKEMProfile

end Descender
