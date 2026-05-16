import Descender.FiniteSupport

namespace Descender
namespace LinearCode

/-!
Predicate-oriented linear-code interfaces.

This first version deliberately uses plain functions and sets rather than
bundled linear maps. That keeps the proof skeleton stable while the algebraic
layers are still being designed.
-/

universe u v

variable {F : Type u} {V M ι : Type v}

def imageCode (A : V -> M) : Set M :=
  {y | ∃ x, A x = y}

def dualKernel [Zero V] (AT : M -> V) : Set M :=
  {r | AT r = 0}

def HasMinWeightAtLeast [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    (C : Set (ι -> F)) (d : Nat) : Prop :=
  ∀ x : ι -> F, x ∈ C -> x ≠ 0 -> d <= weight x

def NoKernelVectorBelow [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    [Zero V] (AT : (ι -> F) -> V) (d : Nat) : Prop :=
  ∀ r : ι -> F, AT r = 0 -> r ≠ 0 -> d <= weight r

theorem imageCode_mem_iff (A : V -> M) (y : M) :
    y ∈ imageCode A ↔ ∃ x, A x = y := by
  rfl

theorem dualKernel_mem_iff [Zero V] (AT : M -> V) (r : M) :
    r ∈ dualKernel AT ↔ AT r = 0 := by
  rfl

theorem hasMinWeightAtLeast_mono [Fintype ι] [DecidableEq ι]
    [Zero F] [DecidableEq F] {C : Set (ι -> F)} {d e : Nat}
    (hde : e <= d) (hC : HasMinWeightAtLeast C d) :
    HasMinWeightAtLeast C e := by
  intro x hxC hx0
  exact Nat.le_trans hde (hC x hxC hx0)

theorem noKernelVectorBelow_mono [Fintype ι] [DecidableEq ι]
    [Zero F] [DecidableEq F] [Zero V]
    {A : (ι -> F) -> V} {d e : Nat}
    (hde : e <= d) (hA : NoKernelVectorBelow A d) :
    NoKernelVectorBelow A e := by
  intro r hrKer hr0
  exact Nat.le_trans hde (hA r hrKer hr0)

theorem noKernelVectorBelow_iff_hasMinWeightAtLeast_ker
    [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F] [Zero V]
    {A : (ι -> F) -> V} {d : Nat} :
    NoKernelVectorBelow A d ↔ HasMinWeightAtLeast ({r | A r = 0} : Set (ι -> F)) d := by
  constructor
  · intro h r hrKer hr0
    exact h r hrKer hr0
  · intro h r hrKer hr0
    exact h r hrKer hr0

theorem hasMinWeightAtLeast_univ_zero [Fintype ι] [DecidableEq ι]
    [Zero F] [DecidableEq F] :
    HasMinWeightAtLeast (Set.univ : Set (ι -> F)) 0 := by
  intro x _ _
  exact Nat.zero_le _

end LinearCode
end Descender
