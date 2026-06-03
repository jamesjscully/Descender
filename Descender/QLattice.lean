import Descender.Residue

namespace Descender

/-!
Dual-first q-ary lattice interface.

This file deliberately contains only the dual relation surface needed by the
current program.  It does not introduce primal lattice geometry, smoothing, or
cryptographic reductions.
-/

variable {ι V : Type*}

def qaryDualLattice (q : Nat) [NeZero q] [Zero V]
    (AT : (ι -> ZMod q) -> V) : Set (ι -> Int) :=
  {z | AT (residueVec q z) = 0}

def ShortQaryDualRelation [Fintype ι] (q : Nat) [NeZero q] [Zero V]
    (AT : (ι -> ZMod q) -> V) (d : Nat) (z : ι -> Int) : Prop :=
  z ∈ qaryDualLattice q AT ∧ NontrivialResidue q z ∧ intL1Norm z < d

def ModqNoKernelVectorBelow [Fintype ι] [DecidableEq ι]
    (q : Nat) [NeZero q] [DecidableEq (ZMod q)] [Zero V]
    (AT : (ι -> ZMod q) -> V) (d : Nat) : Prop :=
  ∀ r : ι -> ZMod q, AT r = 0 -> r ≠ 0 -> d <= weight r

def ModqNoLeeKernelVectorBelow [Fintype ι]
    (q : Nat) [NeZero q] [Zero V]
    (AT : (ι -> ZMod q) -> V) (d : Nat) : Prop :=
  ∀ r : ι -> ZMod q, AT r = 0 -> r ≠ 0 -> d <= centeredNormL1 q r

/--
A concrete certificate violation for `ModqNoLeeKernelVectorBelow`: a nonzero
q-ary kernel vector whose centered Lee norm is below the advertised bound.
-/
structure ModqLeeKernelViolation [Fintype ι]
    (q : Nat) [NeZero q] [Zero V]
    (AT : (ι -> ZMod q) -> V) (d : Nat) where
  r : ι -> ZMod q
  kernel : AT r = 0
  nonzero : r ≠ 0
  short : centeredNormL1 q r < d

namespace ModqLeeKernelViolation

theorem impossible_of_noLeeKernelVectorBelow [Fintype ι]
    (q : Nat) [NeZero q] [Zero V]
    {AT : (ι -> ZMod q) -> V} {d : Nat}
    (hCode : ModqNoLeeKernelVectorBelow q AT d)
    (W : ModqLeeKernelViolation q AT d) :
    False :=
  Nat.not_lt_of_ge (hCode W.r W.kernel W.nonzero) W.short

end ModqLeeKernelViolation

theorem mem_qaryDualLattice_iff (q : Nat) [NeZero q] [Zero V]
    {AT : (ι -> ZMod q) -> V} {z : ι -> Int} :
    z ∈ qaryDualLattice q AT ↔ AT (residueVec q z) = 0 :=
  Iff.rfl

theorem no_short_integer_relation_from_modq_transversality
    [Fintype ι] [DecidableEq ι] (q : Nat) [NeZero q] [DecidableEq (ZMod q)]
    [Zero V] {AT : (ι -> ZMod q) -> V} {d : Nat}
    (hCode : ModqNoKernelVectorBelow q AT d)
    {z : ι -> Int}
    (hz : z ∈ qaryDualLattice q AT)
    (hzNontrivial : NontrivialResidue q z) :
    d <= intL1Norm z := by
  exact Nat.le_trans
    (hCode (residueVec q z) hz hzNontrivial)
    (residue_weight_le_intL1Norm q z)

theorem no_short_integer_relation_from_modq_lee_transversality
    [Fintype ι] (q : Nat) [NeZero q]
    [Zero V] {AT : (ι -> ZMod q) -> V} {d : Nat}
    (hCode : ModqNoLeeKernelVectorBelow q AT d)
    {z : ι -> Int}
    (hz : z ∈ qaryDualLattice q AT)
    (hzNontrivial : NontrivialResidue q z) :
    d <= intL1Norm z := by
  exact Nat.le_trans
    (hCode (residueVec q z) hz hzNontrivial)
    (centeredNormL1_residueVec_le_intL1Norm q z)

theorem no_short_qary_dual_relation
    [Fintype ι] [DecidableEq ι] (q : Nat) [NeZero q] [DecidableEq (ZMod q)]
    [Zero V] {AT : (ι -> ZMod q) -> V} {d : Nat}
    (hCode : ModqNoKernelVectorBelow q AT d) :
    ¬ ∃ z : ι -> Int, ShortQaryDualRelation q AT d z := by
  rintro ⟨z, hzKer, hzNontrivial, hzShort⟩
  have hd : d <= intL1Norm z :=
    no_short_integer_relation_from_modq_transversality q hCode
      hzKer hzNontrivial
  exact Nat.not_lt_of_ge hd hzShort

theorem no_short_qary_dual_relation_from_lee_transversality
    [Fintype ι] (q : Nat) [NeZero q]
    [Zero V] {AT : (ι -> ZMod q) -> V} {d : Nat}
    (hCode : ModqNoLeeKernelVectorBelow q AT d) :
    ¬ ∃ z : ι -> Int, ShortQaryDualRelation q AT d z := by
  rintro ⟨z, hzKer, hzNontrivial, hzShort⟩
  have hd : d <= intL1Norm z :=
    no_short_integer_relation_from_modq_lee_transversality q hCode
      hzKer hzNontrivial
  exact Nat.not_lt_of_ge hd hzShort

end Descender
