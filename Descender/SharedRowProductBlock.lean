import Descender.EfficientPublicView

namespace Descender

/-!
Concrete product-block public-view candidate from the shared-row Lee
certificate artifact.

This module wires the artifact
`research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.*`
into the proof-facing Lean boundary.  The expensive finite enumeration remains
an explicit `BoundedFreeLeeCheckCertificate` input; Lean proves the repeated
block-diagonal composition and the exact `eta = 5` q/eta wrap condition.
-/

namespace SharedRowProductBlock32771

abbrev q : Nat := 32771
abbrev eta : Nat := 5
abbrev targetLee : Nat := 3277

abbrev Block := Fin 16
abbrev LocalPivot := Fin 5
abbrev LocalFree := Fin 2
abbrev Pivot := Block × LocalPivot
abbrev Free := Block × LocalFree
abbrev ViewCoord := Sum Pivot Free
abbrev DualTarget := Pivot -> ZMod q

/-- Coefficients of the selected `5 x 2` local block. -/
def localCoeff : LocalPivot -> LocalFree -> Nat
  | ⟨0, _⟩, ⟨0, _⟩ => 3604
  | ⟨0, _⟩, ⟨1, _⟩ => 19764
  | ⟨1, _⟩, ⟨0, _⟩ => 17179
  | ⟨1, _⟩, ⟨1, _⟩ => 2683
  | ⟨2, _⟩, ⟨0, _⟩ => 26747
  | ⟨2, _⟩, ⟨1, _⟩ => 13757
  | ⟨3, _⟩, ⟨0, _⟩ => 5265
  | ⟨3, _⟩, ⟨1, _⟩ => 17244
  | ⟨4, _⟩, ⟨0, _⟩ => 27751
  | ⟨4, _⟩, ⟨1, _⟩ => 30589

def pairBlockMatrix : LocalPivot -> LocalFree -> ZMod q :=
  fun p f => localCoeff p f

def localMatrix (_ : Block) : LocalPivot -> LocalFree -> ZMod q :=
  pairBlockMatrix

def globalMatrix : Pivot -> Free -> ZMod q :=
  blockDiagonalSystematicMatrix (q := q) localMatrix

def dualMap : (ViewCoord -> ZMod q) -> DualTarget :=
  systematicDualMap q globalMatrix

theorem wrap_eta_target : q <= 2 * eta * targetLee + 1 := by
  native_decide

/--
Public-view Lee-dual certificate from separately checked local block
certificates.  This is useful if future artifacts use non-identical blocks.
-/
def leeDualCertificateFromLocal
    (Clocal :
      ∀ b, BoundedFreeLeeCheckCertificate q LocalPivot LocalFree (localMatrix b))
    (hLower : ∀ b, targetLee <= (Clocal b).lowerBound) :
    PublicViewLeeDualCertificate (q := q) dualMap eta :=
  blockDiagonalCheckedLeeDualCertificate
    (q := q) localMatrix Clocal hLower wrap_eta_target

/--
The artifact repeats one certified pair block across all 16 blocks.  Supplying
that single local check is enough for the global public-view Lee certificate.
-/
def leeDualCertificateFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    PublicViewLeeDualCertificate (q := q) dualMap eta :=
  leeDualCertificateFromLocal (fun _ => C) (by
    intro b
    exact hLower)

theorem noNarrowDualFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    NoNarrowQaryDualRelations q eta dualMap :=
  (leeDualCertificateFromRepeatedLocal C hLower).noNarrowDual

def amplitudePromiseFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    QaryDualAmplitudePromise q ViewCoord DualTarget dualMap :=
  (leeDualCertificateFromRepeatedLocal C hLower).amplitudePromise

end SharedRowProductBlock32771

/- The smaller-row eta-8 sidecar found by the product-block sweep. -/
namespace SharedRowProductBlock32771Eta8

abbrev q : Nat := 32771
abbrev eta : Nat := 8
abbrev targetLee : Nat := 2049

abbrev Block := Fin 16
abbrev LocalPivot := Fin 4
abbrev LocalFree := Fin 2
abbrev Pivot := Block × LocalPivot
abbrev Free := Block × LocalFree
abbrev ViewCoord := Sum Pivot Free
abbrev DualTarget := Pivot -> ZMod q

/- Coefficients of the selected `4 x 2` local block from
`q32771_m132_target2049_rows_per_group4.json`. -/
def localCoeff : LocalPivot -> LocalFree -> Nat
  | ⟨0, _⟩, ⟨0, _⟩ => 7978
  | ⟨0, _⟩, ⟨1, _⟩ => 19524
  | ⟨1, _⟩, ⟨0, _⟩ => 30061
  | ⟨1, _⟩, ⟨1, _⟩ => 30164
  | ⟨2, _⟩, ⟨0, _⟩ => 27905
  | ⟨2, _⟩, ⟨1, _⟩ => 28646
  | ⟨3, _⟩, ⟨0, _⟩ => 21983
  | ⟨3, _⟩, ⟨1, _⟩ => 17633

def pairBlockMatrix : LocalPivot -> LocalFree -> ZMod q :=
  fun p f => localCoeff p f

def localMatrix (_ : Block) : LocalPivot -> LocalFree -> ZMod q :=
  pairBlockMatrix

def globalMatrix : Pivot -> Free -> ZMod q :=
  blockDiagonalSystematicMatrix (q := q) localMatrix

def dualMap : (ViewCoord -> ZMod q) -> DualTarget :=
  systematicDualMap q globalMatrix

theorem wrap_eta_target : q <= 2 * eta * targetLee + 1 := by
  native_decide

def leeDualCertificateFromLocal
    (Clocal :
      ∀ b, BoundedFreeLeeCheckCertificate q LocalPivot LocalFree (localMatrix b))
    (hLower : ∀ b, targetLee <= (Clocal b).lowerBound) :
    PublicViewLeeDualCertificate (q := q) dualMap eta :=
  blockDiagonalCheckedLeeDualCertificate
    (q := q) localMatrix Clocal hLower wrap_eta_target

def leeDualCertificateFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    PublicViewLeeDualCertificate (q := q) dualMap eta :=
  leeDualCertificateFromLocal (fun _ => C) (by
    intro b
    exact hLower)

theorem noNarrowDualFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    NoNarrowQaryDualRelations q eta dualMap :=
  (leeDualCertificateFromRepeatedLocal C hLower).noNarrowDual

def amplitudePromiseFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    QaryDualAmplitudePromise q ViewCoord DualTarget dualMap :=
  (leeDualCertificateFromRepeatedLocal C hLower).amplitudePromise

end SharedRowProductBlock32771Eta8

/- The q=12289, n=48, m1=32 coordinate-subview sidecar. -/
namespace SharedRowProductBlock12289Rows3

abbrev q : Nat := 12289
abbrev eta : Nat := 12
abbrev targetLee : Nat := 512

abbrev Block := Fin 16
abbrev LocalPivot := Fin 3
abbrev LocalFree := Fin 2
abbrev Pivot := Block × LocalPivot
abbrev Free := Block × LocalFree
abbrev ViewCoord := Sum Pivot Free
abbrev DualTarget := Pivot -> ZMod q

/- Coefficients of the selected `3 x 2` local block from
`q12289_m132_target512_rows_per_group3.json`. -/
def localCoeff : LocalPivot -> LocalFree -> Nat
  | ⟨0, _⟩, ⟨0, _⟩ => 10849
  | ⟨0, _⟩, ⟨1, _⟩ => 1171
  | ⟨1, _⟩, ⟨0, _⟩ => 1597
  | ⟨1, _⟩, ⟨1, _⟩ => 1615
  | ⟨2, _⟩, ⟨0, _⟩ => 6102
  | ⟨2, _⟩, ⟨1, _⟩ => 11189

def pairBlockMatrix : LocalPivot -> LocalFree -> ZMod q :=
  fun p f => localCoeff p f

def localMatrix (_ : Block) : LocalPivot -> LocalFree -> ZMod q :=
  pairBlockMatrix

def globalMatrix : Pivot -> Free -> ZMod q :=
  blockDiagonalSystematicMatrix (q := q) localMatrix

def dualMap : (ViewCoord -> ZMod q) -> DualTarget :=
  systematicDualMap q globalMatrix

theorem wrap_eta_target : q <= 2 * eta * targetLee + 1 := by
  native_decide

abbrev localLeeCheckRadius : Nat := targetLee - 1
abbrev localLeeCheckWidth : Nat := 2 * localLeeCheckRadius + 1

def centeredCheckIndex (i : Fin localLeeCheckWidth) : Int :=
  (i.1 : Int) - (localLeeCheckRadius : Int)

def freeOfCenteredPair (x y : Int) : LocalFree -> ZMod q
  | ⟨0, _⟩ => (x : ZMod q)
  | ⟨1, _⟩ => (y : ZMod q)

def localRelationL1OfCenteredPair (x y : Int) : Nat :=
  centeredNormL1 q
    (systematicRelation q pairBlockMatrix (freeOfCenteredPair x y))

def boundedCenteredPairPasses (x y : Int) : Bool :=
  if freeOfCenteredPair x y = 0 then
    true
  else if centeredNormL1 q (freeOfCenteredPair x y) < targetLee then
    decide (targetLee <= localRelationL1OfCenteredPair x y)
  else
    true

def boundedCenteredPairCheck : Bool :=
  decide
    (∀ ix : Fin localLeeCheckWidth, ∀ iy : Fin localLeeCheckWidth,
      boundedCenteredPairPasses
        (centeredCheckIndex ix) (centeredCheckIndex iy) = true)

set_option maxHeartbeats 2000000 in
theorem boundedCenteredPairCheck_eq_true :
    boundedCenteredPairCheck = true := by
  native_decide

theorem centeredCheckIndex_surjective_of_bounds
    {x : Int}
    (hlo : -(localLeeCheckRadius : Int) <= x)
    (hhi : x <= (localLeeCheckRadius : Int)) :
    ∃ ix : Fin localLeeCheckWidth, centeredCheckIndex ix = x := by
  change ∃ ix : Fin 1023, (ix.1 : Int) - 511 = x
  have hlo' : -511 <= x := by
    simpa [localLeeCheckRadius, targetLee] using hlo
  have hhi' : x <= 511 := by
    simpa [localLeeCheckRadius, targetLee] using hhi
  let n : Nat := (x + 511).toNat
  have hnlt : n < 1023 := by
    dsimp [n]
    have hnonneg : 0 <= x + 511 := by omega
    have hto : ((x + 511).toNat : Int) = x + 511 :=
      Int.toNat_of_nonneg hnonneg
    omega
  refine ⟨⟨n, hnlt⟩, ?_⟩
  dsimp [n]
  have hnonneg : 0 <= x + 511 := by omega
  have hto : ((x + 511).toNat : Int) = x + 511 :=
    Int.toNat_of_nonneg hnonneg
  omega

theorem boundedCenteredPairPasses_of_bounds
    {x y : Int}
    (hloX : -(localLeeCheckRadius : Int) <= x)
    (hhiX : x <= (localLeeCheckRadius : Int))
    (hloY : -(localLeeCheckRadius : Int) <= y)
    (hhiY : y <= (localLeeCheckRadius : Int)) :
    boundedCenteredPairPasses x y = true := by
  rcases centeredCheckIndex_surjective_of_bounds
      (x := x) hloX hhiX with ⟨ix, rfl⟩
  rcases centeredCheckIndex_surjective_of_bounds
      (x := y) hloY hhiY with ⟨iy, rfl⟩
  exact (of_decide_eq_true boundedCenteredPairCheck_eq_true ix iy)

theorem boundedCenteredPairPasses_sound
    {x y : Int}
    (hpass : boundedCenteredPairPasses x y = true)
    (hnz : freeOfCenteredPair x y ≠ 0)
    (hsmall : centeredNormL1 q (freeOfCenteredPair x y) < targetLee) :
    targetLee <= localRelationL1OfCenteredPair x y := by
  unfold boundedCenteredPairPasses at hpass
  simp [hnz, hsmall] at hpass
  exact hpass

def internalLocalLeeCertificate :
    BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix where
  lowerBound := targetLee
  checkedBelow := by
    intro y hy hsmall
    let x : Int := centeredLift q (y ⟨0, by decide⟩)
    let z : Int := centeredLift q (y ⟨1, by decide⟩)
    have hnorm :
        centeredNormL1 q y =
          centeredAbs q (y ⟨0, by decide⟩) +
            centeredAbs q (y ⟨1, by decide⟩) := by
      simp [centeredNormL1, Fin.sum_univ_two]
    have hxabs : x.natAbs < targetLee := by
      have hxle :
          centeredAbs q (y ⟨0, by decide⟩) <= centeredNormL1 q y := by
        rw [hnorm]
        exact Nat.le_add_right _ _
      have :
          centeredAbs q (y ⟨0, by decide⟩) < targetLee :=
        Nat.lt_of_le_of_lt hxle hsmall
      simpa [x, centeredAbs, centeredLift] using this
    have hzabs : z.natAbs < targetLee := by
      have hzle :
          centeredAbs q (y ⟨1, by decide⟩) <= centeredNormL1 q y := by
        rw [hnorm]
        exact Nat.le_add_left _ _
      have :
          centeredAbs q (y ⟨1, by decide⟩) < targetLee :=
        Nat.lt_of_le_of_lt hzle hsmall
      simpa [z, centeredAbs, centeredLift] using this
    have hxbounds :
        -(localLeeCheckRadius : Int) <= x ∧
          x <= localLeeCheckRadius := by
      have hInt : (x.natAbs : Int) < (targetLee : Int) := by
        exact_mod_cast hxabs
      rw [← Int.abs_eq_natAbs] at hInt
      have hb := abs_lt.mp hInt
      norm_num [localLeeCheckRadius, targetLee] at hb ⊢
      omega
    have hzbounds :
        -(localLeeCheckRadius : Int) <= z ∧
          z <= localLeeCheckRadius := by
      have hInt : (z.natAbs : Int) < (targetLee : Int) := by
        exact_mod_cast hzabs
      rw [← Int.abs_eq_natAbs] at hInt
      have hb := abs_lt.mp hInt
      norm_num [localLeeCheckRadius, targetLee] at hb ⊢
      omega
    have hfree : freeOfCenteredPair x z = y := by
      funext f
      fin_cases f <;>
        simp [freeOfCenteredPair, x, z, centeredLift, ZMod.coe_valMinAbs]
    have hpass :=
      boundedCenteredPairPasses_of_bounds
        hxbounds.1 hxbounds.2 hzbounds.1 hzbounds.2
    have hs :=
      boundedCenteredPairPasses_sound hpass
        (by simpa [hfree] using hy)
        (by simpa [hfree] using hsmall)
    simpa [localRelationL1OfCenteredPair, hfree] using hs

theorem internalLocalLeeCertificate_lower :
    targetLee <= internalLocalLeeCertificate.lowerBound := by
  rfl

def leeDualCertificateFromLocal
    (Clocal :
      ∀ b, BoundedFreeLeeCheckCertificate q LocalPivot LocalFree (localMatrix b))
    (hLower : ∀ b, targetLee <= (Clocal b).lowerBound) :
    PublicViewLeeDualCertificate (q := q) dualMap eta :=
  blockDiagonalCheckedLeeDualCertificate
    (q := q) localMatrix Clocal hLower wrap_eta_target

def leeDualCertificateFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    PublicViewLeeDualCertificate (q := q) dualMap eta :=
  leeDualCertificateFromLocal (fun _ => C) (by
    intro b
    exact hLower)

theorem noNarrowDualFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    NoNarrowQaryDualRelations q eta dualMap :=
  (leeDualCertificateFromRepeatedLocal C hLower).noNarrowDual

def amplitudePromiseFromRepeatedLocal
    (C : BoundedFreeLeeCheckCertificate q LocalPivot LocalFree pairBlockMatrix)
    (hLower : targetLee <= C.lowerBound) :
    QaryDualAmplitudePromise q ViewCoord DualTarget dualMap :=
  (leeDualCertificateFromRepeatedLocal C hLower).amplitudePromise

end SharedRowProductBlock12289Rows3

end Descender
