import Descender.SystematicLee

namespace Descender

/-!
Shifted moment-code Lee certificates.

The explicit algebraic expander candidate is a systematic graph code whose
pivot block is a shifted, column-scaled moment matrix:

`P t j = c j * alpha t ^ (exponentStart + j)`.

This file does not assert that the family is always good.  It names the exact
anti-concentration hypothesis needed for this family and proves that it is
precisely enough to feed the existing q-ary Lee kernel boundary.
-/

def shiftedMomentMatrix (q : Nat) [NeZero q]
    {Rows Free : Type*} [Fintype Rows]
    (alpha : Rows -> ZMod q) (scale : Free -> ZMod q)
    (exponent : Free -> Nat) : Rows -> Free -> ZMod q :=
  fun t j => scale j * alpha t ^ exponent j

def finShiftedMomentMatrix (q : Nat) [NeZero q]
    {r g : Nat} (alpha : Fin r -> ZMod q) (scale : Fin g -> ZMod q)
    (exponentStart : Nat) : Fin r -> Fin g -> ZMod q :=
  shiftedMomentMatrix q alpha scale fun j => exponentStart + j.val

def MomentLeeAntiConcentration (q : Nat) [NeZero q]
    {Rows Free : Type*} [Fintype Rows] [Fintype Free]
    (alpha : Rows -> ZMod q) (scale : Free -> ZMod q)
    (exponent : Free -> Nat) (d : Nat) : Prop :=
  SystematicLeeDistanceAtLeast q
    (shiftedMomentMatrix q alpha scale exponent) d

def FinMomentLeeAntiConcentration (q : Nat) [NeZero q]
    {r g : Nat} (alpha : Fin r -> ZMod q) (scale : Fin g -> ZMod q)
    (exponentStart d : Nat) : Prop :=
  SystematicLeeDistanceAtLeast q
    (finShiftedMomentMatrix q alpha scale exponentStart) d

structure MomentLeeCertificate (q : Nat) [NeZero q]
    (Rows Free : Type*) [Fintype Rows] [Fintype Free] where
  alpha : Rows -> ZMod q
  scale : Free -> ZMod q
  exponent : Free -> Nat
  lowerBound : Nat
  antiConcentration :
    MomentLeeAntiConcentration q alpha scale exponent lowerBound

def MomentLeeCertificate.matrix
    {q : Nat} [NeZero q] {Rows Free : Type*} [Fintype Rows] [Fintype Free]
    (C : MomentLeeCertificate q Rows Free) : Rows -> Free -> ZMod q :=
  shiftedMomentMatrix q C.alpha C.scale C.exponent

def MomentLeeCertificate.toSystematicLeeCertificate
    {q : Nat} [NeZero q] {Rows Free : Type*} [Fintype Rows] [Fintype Free]
    (C : MomentLeeCertificate q Rows Free) :
    SystematicLeeCertificate q Rows Free C.matrix where
  lowerBound := C.lowerBound
  lowerBoundProof := C.antiConcentration

def MomentLeeCertificate.modqNoLeeKernelVectorBelow
    {q : Nat} [NeZero q] {Rows Free : Type*} [Fintype Rows] [Fintype Free]
    (C : MomentLeeCertificate q Rows Free) :
    ModqNoLeeKernelVectorBelow q
      (systematicDualMap q C.matrix) C.lowerBound :=
  C.toSystematicLeeCertificate.modqNoLeeKernelVectorBelow

theorem modqNoLeeKernelVectorBelow_of_moment_antiConcentration
    [Fintype Rows] [Fintype Free]
    (q : Nat) [NeZero q]
    (alpha : Rows -> ZMod q) (scale : Free -> ZMod q)
    (exponent : Free -> Nat) {d : Nat}
    (hMoment : MomentLeeAntiConcentration q alpha scale exponent d) :
    ModqNoLeeKernelVectorBelow q
      (systematicDualMap q (shiftedMomentMatrix q alpha scale exponent)) d :=
  modqNoLeeKernelVectorBelow_of_systematic_lower_bound q
    (shiftedMomentMatrix q alpha scale exponent) hMoment

theorem modqNoLeeKernelVectorBelow_of_fin_moment_antiConcentration
    (q : Nat) [NeZero q] {r g : Nat}
    (alpha : Fin r -> ZMod q) (scale : Fin g -> ZMod q)
    (exponentStart : Nat) {d : Nat}
    (hMoment :
      FinMomentLeeAntiConcentration q alpha scale exponentStart d) :
    ModqNoLeeKernelVectorBelow q
      (systematicDualMap q
        (finShiftedMomentMatrix q alpha scale exponentStart)) d :=
  modqNoLeeKernelVectorBelow_of_systematic_lower_bound q
    (finShiftedMomentMatrix q alpha scale exponentStart) hMoment

end Descender
