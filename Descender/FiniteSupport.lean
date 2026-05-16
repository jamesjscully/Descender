import Mathlib.Data.Fintype.Card
import Mathlib.Algebra.Module.Basic

namespace Descender

/-!
Finite-coordinate Hamming support and weight.

This module is the reusable support/weight layer for finite coordinate
functions.  It avoids matrices and coding-theory-specific objects.
-/

variable {F ι : Type*}

def support [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    (x : ι -> F) : Finset ι :=
  Finset.univ.filter fun i => x i ≠ 0

def weight [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    (x : ι -> F) : Nat :=
  (support x).card

@[simp]
theorem mem_support_iff [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    {x : ι -> F} {i : ι} : i ∈ support x ↔ x i ≠ 0 := by
  simp [support]

@[simp]
theorem not_mem_support_iff [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    {x : ι -> F} {i : ι} : i ∉ support x ↔ x i = 0 := by
  simp [support]

@[simp]
theorem support_zero [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F] :
    support (0 : ι -> F) = ∅ := by
  ext i
  simp

@[simp]
theorem weight_zero [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F] :
    weight (0 : ι -> F) = 0 := by
  simp [weight]

theorem weight_le_card [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    (x : ι -> F) : weight x <= Fintype.card ι := by
  simpa [weight] using Finset.card_le_univ (support x)

theorem support_eq_empty_iff [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    {x : ι -> F} : support x = ∅ ↔ x = 0 := by
  constructor
  · intro h
    funext i
    have hi : i ∉ support x := by simp [h]
    simpa using (not_mem_support_iff.mp hi)
  · intro h
    simp [h]

theorem weight_eq_zero_iff [Fintype ι] [DecidableEq ι] [Zero F] [DecidableEq F]
    {x : ι -> F} : weight x = 0 ↔ x = 0 := by
  rw [weight, Finset.card_eq_zero, support_eq_empty_iff]

theorem support_add_subset [Fintype ι] [DecidableEq ι] [AddMonoid F]
    [DecidableEq F] (x y : ι -> F) :
    support (x + y) ⊆ support x ∪ support y := by
  intro i hi
  simp only [mem_support_iff, Finset.mem_union] at hi ⊢
  by_contra h
  rw [not_or] at h
  push Not at h
  exact hi (by simp [Pi.add_apply, h.1, h.2])

theorem weight_add_le [Fintype ι] [DecidableEq ι] [AddMonoid F]
    [DecidableEq F] (x y : ι -> F) :
    weight (x + y) <= weight x + weight y := by
  calc
    weight (x + y) = (support (x + y)).card := rfl
    _ <= (support x ∪ support y).card :=
      Finset.card_le_card (support_add_subset x y)
    _ <= (support x).card + (support y).card := Finset.card_union_le _ _
    _ = weight x + weight y := rfl

theorem support_neg [Fintype ι] [DecidableEq ι] [AddGroup F] [DecidableEq F]
    (x : ι -> F) :
    support (fun i => -x i) = support x := by
  ext i
  simp [support]

theorem weight_neg [Fintype ι] [DecidableEq ι] [AddGroup F] [DecidableEq F]
    (x : ι -> F) :
    weight (fun i => -x i) = weight x := by
  simp [weight, support_neg]

theorem support_smul_subset [Fintype ι] [DecidableEq ι] [MonoidWithZero F]
    [DecidableEq F] (a : F) (x : ι -> F) :
    support (a • x) ⊆ support x := by
  intro i hi
  simp only [mem_support_iff] at hi ⊢
  intro hx
  exact hi (by simp [Pi.smul_apply, hx])

theorem weight_smul_le [Fintype ι] [DecidableEq ι] [MonoidWithZero F]
    [DecidableEq F] (a : F) (x : ι -> F) :
    weight (a • x) <= weight x := by
  simpa [weight] using Finset.card_le_card (support_smul_subset a x)

theorem mem_support_smul_iff_of_ne_zero [Fintype ι] [DecidableEq ι]
    [MonoidWithZero F] [IsCancelMulZero F] [DecidableEq F] {a : F}
    (ha : a ≠ 0) {x : ι -> F} {i : ι} :
    i ∈ support (a • x) ↔ i ∈ support x := by
  simp only [mem_support_iff, Pi.smul_apply]
  constructor
  · intro h hx
    exact h (by simp [hx])
  · intro h
    exact mul_ne_zero ha h

theorem support_smul_eq_of_ne_zero [Fintype ι] [DecidableEq ι]
    [MonoidWithZero F] [IsCancelMulZero F] [DecidableEq F] {a : F}
    (ha : a ≠ 0) (x : ι -> F) :
    support (a • x) = support x := by
  ext i
  exact mem_support_smul_iff_of_ne_zero ha

theorem weight_smul_eq_of_ne_zero [Fintype ι] [DecidableEq ι]
    [MonoidWithZero F] [IsCancelMulZero F] [DecidableEq F] {a : F}
    (ha : a ≠ 0) (x : ι -> F) :
    weight (a • x) = weight x := by
  simp [weight, support_smul_eq_of_ne_zero ha x]

end Descender
