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

end Descender
