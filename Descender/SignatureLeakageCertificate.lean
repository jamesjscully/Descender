import Descender.SignatureDistribution

namespace Descender
namespace SignatureDistribution

/-!
Lean-side shape of the rational leakage proof objects emitted by the concrete
sampler verifier.

The Rust verifier checks the JSON certificate against the canonical dyadic
Gaussian weight table and recomputes each total-variation bound exactly.  The
generated Lean modules import this file and expose the verified rational
constants as ordinary Lean definitions.
-/

noncomputable section

/-- A closed rational interval `[lower, upper] / denominator`. -/
structure RationalIntervalProof where
  lowerNumerator : Nat
  upperNumerator : Nat
  denominator : Nat
  upperDecimal : String

namespace RationalIntervalProof

def lowerReal (I : RationalIntervalProof) : ℝ :=
  (I.lowerNumerator : ℝ) / I.denominator

def upperReal (I : RationalIntervalProof) : ℝ :=
  (I.upperNumerator : ℝ) / I.denominator

theorem lowerReal_nonneg (I : RationalIntervalProof) :
    0 ≤ I.lowerReal := by
  unfold lowerReal
  exact div_nonneg (Nat.cast_nonneg _) (Nat.cast_nonneg _)

theorem upperReal_nonneg (I : RationalIntervalProof) :
    0 ≤ I.upperReal := by
  unfold upperReal
  exact div_nonneg (Nat.cast_nonneg _) (Nat.cast_nonneg _)

theorem upperReal_le_one_of_upper_le_denominator
    (I : RationalIntervalProof)
    (h : I.upperNumerator ≤ I.denominator) :
    I.upperReal ≤ 1 := by
  unfold upperReal
  by_cases hden : I.denominator = 0
  · have hnum : I.upperNumerator = 0 := Nat.eq_zero_of_le_zero (by simpa [hden] using h)
    simp [hden, hnum]
  · have hdenNat : 0 < I.denominator := Nat.pos_of_ne_zero hden
    have hdenReal : (0 : ℝ) < I.denominator := by exact_mod_cast hdenNat
    rw [div_le_iff₀ hdenReal]
    have hcast : (I.upperNumerator : ℝ) ≤ (I.denominator : ℝ) := by
      exact_mod_cast h
    simpa using hcast

end RationalIntervalProof

/-- One verified dyadic conditioned-Gaussian leakage row. -/
structure DyadicGaussianLeakageProofRow where
  sampler : String
  proofStatus : String
  blockWeight : Nat
  repetitions : Nat
  serializedResponseCoordinates : Nat
  acceptedZBound : Nat
  eta : Nat
  sigma : Nat
  weightPrecisionBits : Nat
  supportMin : Int
  supportMax : Int
  worstShiftLeft : Int
  worstShiftRight : Int
  perCoordinateTvExact : RationalIntervalProof
  signatureTvBound : RationalIntervalProof

namespace DyadicGaussianLeakageProofRow

def perCoordinateLeakage (row : DyadicGaussianLeakageProofRow) : ℝ :=
  row.perCoordinateTvExact.upperReal

def signatureLeakage (row : DyadicGaussianLeakageProofRow) : ℝ :=
  row.signatureTvBound.upperReal

theorem perCoordinateLeakage_nonneg (row : DyadicGaussianLeakageProofRow) :
    0 ≤ row.perCoordinateLeakage := by
  exact row.perCoordinateTvExact.upperReal_nonneg

theorem signatureLeakage_nonneg (row : DyadicGaussianLeakageProofRow) :
    0 ≤ row.signatureLeakage := by
  exact row.signatureTvBound.upperReal_nonneg

theorem signatureLeakage_le_one
    (row : DyadicGaussianLeakageProofRow)
    (h : row.signatureTvBound.upperNumerator ≤ row.signatureTvBound.denominator) :
    row.signatureLeakage ≤ 1 := by
  exact row.signatureTvBound.upperReal_le_one_of_upper_le_denominator h

end DyadicGaussianLeakageProofRow

/-- Metadata and rows for one checked certificate artifact. -/
structure DyadicGaussianLeakageCertificate where
  schema : String
  profile : String
  sourcePath : String
  sourceSha256 : String
  q : Nat
  n : Nat
  eta : Nat
  productBlockGroups : Nat
  localFreeCoordinatesPerBlock : Nat
  targetBits : String
  digestBytes : Nat
  maxLog2Retries : String
  maxExtraRepetitions : Nat
  forkChallengesPerRound : Nat
  rowSource : String
  rowLimit : Nat
  rows : List DyadicGaussianLeakageProofRow

/--
Specialize a bounded-leakage certificate to an externally supplied real bound.

Generated rational certificate modules use this theorem with their verified
`signatureLeakage` constants.
-/
theorem RejectionSampler.LeakageBoundedCertificate.responseAdvantage_le_of_leakage_eq
    {Secret Challenge Mask Response : Type*}
    [Fintype Mask] [Fintype Response]
    {S : RejectionSampler Secret Challenge Mask Response}
    (C : S.LeakageBoundedCertificate)
    {bound : ℝ}
    (hbound : C.leakage = bound)
    (s₀ s₁ : Secret) (c : Challenge) (A : Response -> Bool) :
    C.responseAdvantage s₀ s₁ c A ≤ bound := by
  simpa [hbound] using C.responseAdvantage_le s₀ s₁ c A

/--
Data-processing specialization for an externally supplied real leakage bound.
-/
theorem RejectionSampler.LeakageBoundedCertificate.encoded_responseAdvantage_le_of_leakage_eq
    {Secret Challenge Mask Response Encoded : Type*}
    [Fintype Mask] [Fintype Response]
    {S : RejectionSampler Secret Challenge Mask Response}
    (C : S.LeakageBoundedCertificate)
    {bound : ℝ}
    (hbound : C.leakage = bound)
    (encode : Response -> Encoded)
    (s₀ s₁ : Secret) (c : Challenge) (A : Encoded -> Bool) :
    C.responseAdvantage s₀ s₁ c (fun r => A (encode r)) ≤ bound := by
  simpa [hbound] using C.encoded_responseAdvantage_le encode s₀ s₁ c A

end

end SignatureDistribution
end Descender
