import Mathlib.Data.ZMod.ValMinAbs
import Descender.FiniteSupport

namespace Descender

/-!
Shared residue and centered-lift conventions.

This module is intentionally small.  It owns the common integer residue map,
integer support/norms, and centered representatives used by the q-ary lattice
and P-TND definition layers.
-/

variable {ι : Type*}

def residueVec (q : Nat) [NeZero q] (z : ι -> Int) : ι -> ZMod q :=
  fun i => (z i : ZMod q)

def intSupport [Fintype ι] [DecidableEq ι] (z : ι -> Int) : Finset ι :=
  Finset.univ.filter fun i => z i ≠ 0

def intL1Norm [Fintype ι] (z : ι -> Int) : Nat :=
  ∑ i, (z i).natAbs

def NontrivialResidue (q : Nat) [NeZero q] (z : ι -> Int) : Prop :=
  residueVec q z ≠ 0

def centeredLift (q : Nat) (a : ZMod q) : Int :=
  a.valMinAbs

def centeredAbs (q : Nat) (a : ZMod q) : Nat :=
  (centeredLift q a).natAbs

def centeredNormL1 [Fintype ι] (q : Nat) (x : ι -> ZMod q) : Nat :=
  ∑ i, centeredAbs q (x i)

def centeredNormLinf [Fintype ι] (q : Nat) (x : ι -> ZMod q) : Nat :=
  Finset.univ.sup fun i => centeredAbs q (x i)

def centeredResidueNoiseSupport [Fintype ι] (q eta : Nat)
    (e : ι -> ZMod q) : Prop :=
  ∀ i, centeredAbs q (e i) <= eta

@[simp]
theorem residueVec_apply (q : Nat) [NeZero q] (z : ι -> Int) (i : ι) :
    residueVec q z i = (z i : ZMod q) :=
  rfl

@[simp]
theorem centeredLift_zero (q : Nat) :
    centeredLift q (0 : ZMod q) = 0 := by
  simp [centeredLift, ZMod.valMinAbs_zero]

@[simp]
theorem centeredAbs_zero (q : Nat) :
    centeredAbs q (0 : ZMod q) = 0 := by
  simp [centeredAbs]

theorem centeredAbs_le_of_noiseSupport [Fintype ι] {q eta : Nat}
    {e : ι -> ZMod q} (h : centeredResidueNoiseSupport q eta e) (i : ι) :
    centeredAbs q (e i) <= eta :=
  h i

theorem centeredResidueNoiseSupport_mono [Fintype ι] {q eta eta' : Nat}
    {e : ι -> ZMod q} (heta : eta <= eta')
    (h : centeredResidueNoiseSupport q eta e) :
    centeredResidueNoiseSupport q eta' e := by
  intro i
  exact Nat.le_trans (h i) heta

theorem centeredNormLinf_le_of_noiseSupport [Fintype ι] {q eta : Nat}
    {e : ι -> ZMod q} (h : centeredResidueNoiseSupport q eta e) :
    centeredNormLinf q e <= eta := by
  unfold centeredNormLinf
  apply Finset.sup_le
  intro i _hi
  exact h i

theorem centeredAbs_add_le (q : Nat) (a b : ZMod q) :
    centeredAbs q (a + b) <= centeredAbs q a + centeredAbs q b := by
  unfold centeredAbs centeredLift
  exact Nat.le_trans (ZMod.natAbs_valMinAbs_add_le a b)
    (Int.natAbs_add_le _ _)

theorem centeredResidueNoiseSupport_add [Fintype ι] {q eta rho : Nat}
    {e r : ι -> ZMod q}
    (he : centeredResidueNoiseSupport q eta e)
    (hr : centeredResidueNoiseSupport q rho r) :
    centeredResidueNoiseSupport q (eta + rho) (fun i => e i + r i) := by
  intro i
  calc
    centeredAbs q (e i + r i) <= centeredAbs q (e i) + centeredAbs q (r i) :=
      centeredAbs_add_le q (e i) (r i)
    _ <= eta + rho := Nat.add_le_add (he i) (hr i)

theorem centeredAbs_sub_le (q : Nat) (a b : ZMod q) :
    centeredAbs q (a - b) <= centeredAbs q a + centeredAbs q b := by
  simpa [sub_eq_add_neg, centeredAbs, centeredLift] using
    centeredAbs_add_le q a (-b)

@[simp]
theorem centeredAbs_neg (q : Nat) (a : ZMod q) :
    centeredAbs q (-a) = centeredAbs q a := by
  simp [centeredAbs, centeredLift, ZMod.natAbs_valMinAbs_neg]

theorem centeredNormLinf_add_le [Fintype ι] (q : Nat)
    (x y : ι -> ZMod q) :
    centeredNormLinf q (fun i => x i + y i) <=
      centeredNormLinf q x + centeredNormLinf q y := by
  unfold centeredNormLinf
  apply Finset.sup_le
  intro i _hi
  calc
    centeredAbs q (x i + y i) <= centeredAbs q (x i) + centeredAbs q (y i) :=
      centeredAbs_add_le q (x i) (y i)
    _ <= Finset.univ.sup (fun i => centeredAbs q (x i)) +
          Finset.univ.sup (fun i => centeredAbs q (y i)) := by
      exact Nat.add_le_add
        (Finset.le_sup (s := Finset.univ) (f := fun i => centeredAbs q (x i))
          (Finset.mem_univ i))
        (Finset.le_sup (s := Finset.univ) (f := fun i => centeredAbs q (y i))
          (Finset.mem_univ i))

theorem centeredNormLinf_sub_le [Fintype ι] (q : Nat)
    (x y : ι -> ZMod q) :
    centeredNormLinf q (fun i => x i - y i) <=
      centeredNormLinf q x + centeredNormLinf q y := by
  simpa [sub_eq_add_neg, centeredNormLinf, centeredAbs, centeredLift] using
    centeredNormLinf_add_le q x (fun i => -y i)

theorem centeredNormL1_sub_le [Fintype ι] (q : Nat)
    (x y : ι -> ZMod q) :
    centeredNormL1 q (fun i => x i - y i) <=
      centeredNormL1 q x + centeredNormL1 q y := by
  unfold centeredNormL1
  calc
    ∑ i, centeredAbs q (x i - y i) <=
        ∑ i, (centeredAbs q (x i) + centeredAbs q (y i)) := by
          exact Finset.sum_le_sum (by
            intro i _hi
            exact centeredAbs_sub_le q (x i) (y i))
    _ = ∑ i, centeredAbs q (x i) + ∑ i, centeredAbs q (y i) := by
          rw [Finset.sum_add_distrib]

theorem centeredNormL1_le_card_mul_of_noiseSupport [Fintype ι]
    {q eta : Nat} {e : ι -> ZMod q}
    (h : centeredResidueNoiseSupport q eta e) :
    centeredNormL1 q e <= Fintype.card ι * eta := by
  unfold centeredNormL1
  calc
    ∑ i, centeredAbs q (e i) <= ∑ _i : ι, eta := by
      exact Finset.sum_le_sum (by
        intro i _hi
        exact h i)
    _ = Fintype.card ι * eta := by
      simp only [Finset.sum_const, Finset.card_univ, nsmul_eq_mul,
        Nat.cast_id]

theorem centeredNormL1_sub_le_card_mul_add_of_noiseSupport [Fintype ι]
    {q eta rho : Nat} {x y : ι -> ZMod q}
    (hx : centeredResidueNoiseSupport q eta x)
    (hy : centeredResidueNoiseSupport q rho y) :
    centeredNormL1 q (fun i => x i - y i) <=
      Fintype.card ι * (eta + rho) := by
  unfold centeredNormL1
  calc
    ∑ i, centeredAbs q (x i - y i) <=
        ∑ i, (eta + rho) := by
          exact Finset.sum_le_sum (by
            intro i _hi
            exact Nat.le_trans (centeredAbs_sub_le q (x i) (y i))
              (Nat.add_le_add (hx i) (hy i)))
    _ = Fintype.card ι * (eta + rho) := by
          simp only [Finset.sum_const, Finset.card_univ, nsmul_eq_mul,
            Nat.cast_id]

theorem centeredAbs_sum_le [Fintype ι] (q : Nat)
    (f : ι -> ZMod q) :
    centeredAbs q (∑ i, f i) <= ∑ i, centeredAbs q (f i) := by
  classical
  let s : Finset ι := Finset.univ
  have h :
      centeredAbs q (∑ i ∈ s, f i) <= ∑ i ∈ s, centeredAbs q (f i) := by
    refine Finset.induction_on s ?base ?step
    · simp
    · intro a s ha ih
      rw [Finset.sum_insert ha, Finset.sum_insert ha]
      calc
        centeredAbs q (f a + ∑ x ∈ s, f x)
            <= centeredAbs q (f a) + centeredAbs q (∑ x ∈ s, f x) :=
              centeredAbs_add_le q (f a) (∑ x ∈ s, f x)
        _ <= centeredAbs q (f a) + ∑ x ∈ s, centeredAbs q (f x) :=
              Nat.add_le_add_left ih (centeredAbs q (f a))
  simpa [s] using h

@[simp]
theorem centeredNormLinf_neg [Fintype ι] (q : Nat) (x : ι -> ZMod q) :
    centeredNormLinf q (fun i => -x i) = centeredNormLinf q x := by
  unfold centeredNormLinf
  apply Finset.sup_congr rfl
  intro i _hi
  simp

theorem residue_weight_le_intL1Norm [Fintype ι] [DecidableEq ι]
    (q : Nat) [NeZero q] [DecidableEq (ZMod q)] (z : ι -> Int) :
    weight (residueVec q z) <= intL1Norm z := by
  unfold weight support intL1Norm
  rw [Finset.card_eq_sum_ones]
  calc
    ∑ i ∈ Finset.univ.filter (fun i => residueVec q z i ≠ 0), 1
        <= ∑ i ∈ Finset.univ.filter (fun i => residueVec q z i ≠ 0),
              (z i).natAbs := by
          apply Finset.sum_le_sum
          intro i hi
          have hres : residueVec q z i ≠ 0 := by
            simpa using (Finset.mem_filter.mp hi).2
          have hz : z i ≠ 0 := by
            intro hzi
            apply hres
            simp [residueVec, hzi]
          exact Nat.succ_le_of_lt (Int.natAbs_pos.mpr hz)
    _ <= ∑ i, (z i).natAbs := by
          exact Finset.sum_le_sum_of_subset (Finset.filter_subset _ _)

theorem centeredAbs_intCast_le_natAbs (q : Nat) [NeZero q] (z : Int) :
    centeredAbs q (z : ZMod q) <= z.natAbs := by
  unfold centeredAbs centeredLift
  exact ZMod.natAbs_min_of_le_div_two q ((z : ZMod q).valMinAbs) z
    (by simp)
    (ZMod.natAbs_valMinAbs_le (z : ZMod q))

theorem centeredNormL1_residueVec_le_intL1Norm [Fintype ι]
    (q : Nat) [NeZero q] (z : ι -> Int) :
    centeredNormL1 q (residueVec q z) <= intL1Norm z := by
  unfold centeredNormL1 intL1Norm residueVec
  exact Finset.sum_le_sum (by
    intro i _hi
    exact centeredAbs_intCast_le_natAbs q (z i))

theorem centeredAbs_eq_zero_iff (q : Nat) [NeZero q] (x : ZMod q) :
    centeredAbs q x = 0 ↔ x = 0 := by
  unfold centeredAbs centeredLift
  rw [Int.natAbs_eq_zero, ZMod.valMinAbs_eq_zero]

theorem centeredNormL1_eq_zero_iff [Fintype ι]
    (q : Nat) [NeZero q] (x : ι -> ZMod q) :
    centeredNormL1 q x = 0 ↔ x = 0 := by
  classical
  unfold centeredNormL1
  rw [Finset.sum_eq_zero_iff_of_nonneg]
  · constructor
    · intro h
      funext i
      exact (centeredAbs_eq_zero_iff q (x i)).mp (h i (Finset.mem_univ i))
    · intro h i _hi
      simp [h]
  · intro i _hi
    exact Nat.zero_le _

theorem weight_le_centeredNormL1 [Fintype ι] [DecidableEq ι]
    (q : Nat) [NeZero q] (x : ι -> ZMod q) :
    weight x <= centeredNormL1 q x := by
  classical
  unfold weight centeredNormL1
  rw [Finset.card_eq_sum_ones]
  calc
    ∑ i ∈ support x, 1 <= ∑ i ∈ support x, centeredAbs q (x i) := by
      exact Finset.sum_le_sum (by
        intro i hi
        have hxi : x i ≠ 0 := (mem_support_iff).mp hi
        exact Nat.succ_le_of_lt (Nat.pos_of_ne_zero (by
          intro hzero
          exact hxi ((centeredAbs_eq_zero_iff q (x i)).mp hzero))))
    _ <= ∑ i, centeredAbs q (x i) := by
      exact Finset.sum_le_sum_of_subset_of_nonneg
        (by intro i hi; exact Finset.mem_univ i)
        (by intro i hi hnot; exact Nat.zero_le _)

end Descender
