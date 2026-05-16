import Mathlib.Tactic

namespace Descender

open Polynomial
open scoped BigOperators

/-!
Minimal Cauchy-matrix interfaces for the deterministic guard layer.

The Cauchy support and guard-block primal-distance bounds below are proved by
clearing denominators and applying the elementary root-count bound for a
nonzero polynomial.
-/

universe u v w

abbrev Vector (ι : Type u) (F : Type v) := ι -> F
abbrev Matrix (Row : Type u) (Col : Type v) (F : Type w) := Row -> Col -> F

def zeroVec [Zero F] : Vector ι F :=
  fun _ => 0

def supportOn [Zero F] [DecidableEq F] (indices : List ι) (x : Vector ι F) :
    List ι :=
  indices.filter (fun i => decide (x i ≠ 0))

def weightOn [Zero F] [DecidableEq F] (indices : List ι) (x : Vector ι F) :
    Nat :=
  (supportOn indices x).length

def nonzeroOn [Zero F] (indices : List ι) (x : Vector ι F) : Prop :=
  ∃ i, i ∈ indices ∧ x i ≠ 0

theorem mem_supportOn [Zero F] [DecidableEq F]
    {indices : List ι} {x : Vector ι F} {i : ι} :
    i ∈ supportOn indices x ↔ i ∈ indices ∧ x i ≠ 0 := by
  unfold supportOn
  simp

theorem supportOn_sublist [Zero F] [DecidableEq F]
    (indices : List ι) (x : Vector ι F) :
    (supportOn indices x).Sublist indices := by
  unfold supportOn
  exact List.filter_sublist (p := fun i => decide (x i ≠ 0)) (l := indices)

theorem supportOn_nodup [Zero F] [DecidableEq F]
    {indices : List ι} {x : Vector ι F}
    (hindices : indices.Nodup) :
    (supportOn indices x).Nodup := by
  unfold supportOn
  exact hindices.filter _

theorem nonzeroOn_iff_exists_mem_supportOn [Zero F] [DecidableEq F]
    {indices : List ι} {x : Vector ι F} :
    nonzeroOn indices x ↔ ∃ i, i ∈ supportOn indices x := by
  constructor
  · intro hx
    rcases hx with ⟨i, hi, hxi⟩
    exact ⟨i, mem_supportOn.mpr ⟨hi, hxi⟩⟩
  · intro hx
    rcases hx with ⟨i, hi⟩
    exact ⟨i, (mem_supportOn.mp hi).1, (mem_supportOn.mp hi).2⟩

theorem supportOn_eq_nil_iff [Zero F] [DecidableEq F]
    {indices : List ι} {x : Vector ι F} :
    supportOn indices x = [] ↔ ∀ i, i ∈ indices -> x i = 0 := by
  unfold supportOn
  simp [List.filter_eq_nil_iff]

theorem weightOn_eq_zero_iff [Zero F] [DecidableEq F]
    {indices : List ι} {x : Vector ι F} :
    weightOn indices x = 0 ↔ ¬ nonzeroOn indices x := by
  unfold weightOn
  rw [List.length_eq_zero_iff, supportOn_eq_nil_iff]
  constructor
  · intro hzero hx
    rcases hx with ⟨i, hi, hxi⟩
    exact hxi (hzero i hi)
  · intro hnone i hi
    by_contra hxi
    exact hnone ⟨i, hi, hxi⟩

theorem weightOn_le_length [Zero F] [DecidableEq F]
    (indices : List ι) (x : Vector ι F) :
    weightOn indices x <= indices.length := by
  unfold weightOn supportOn
  exact List.length_filter_le _ _

theorem nonzeroOn_of_forall_mem_of_ne_zero [Zero F]
    {indices : List ι} {x : Vector ι F}
    (hmem : ∀ i, i ∈ indices) (hx : x ≠ zeroVec) :
    nonzeroOn indices x := by
  by_contra h
  apply hx
  funext i
  by_contra hxi
  exact h ⟨i, hmem i, hxi⟩

theorem weightOn_pos_of_nonzeroOn [Zero F] [DecidableEq F]
    {indices : List ι} {x : Vector ι F}
    (hx : nonzeroOn indices x) :
    0 < weightOn indices x := by
  rcases hx with ⟨i, hi, hxi⟩
  unfold weightOn supportOn
  exact List.length_pos_iff_ne_nil.mpr
    (List.ne_nil_of_mem (List.mem_filter.mpr ⟨hi, by simpa using hxi⟩))

theorem nonzeroOn_of_weightOn_pos [Zero F] [DecidableEq F]
    {indices : List ι} {x : Vector ι F}
    (hx : 0 < weightOn indices x) :
    nonzeroOn indices x := by
  unfold weightOn supportOn at hx
  have hne : indices.filter (fun i => decide (x i ≠ 0)) ≠ [] :=
    List.length_pos_iff_ne_nil.mp hx
  rcases List.exists_mem_of_ne_nil _ hne with ⟨i, hi⟩
  exact ⟨i, (List.mem_filter.mp hi).1, by simpa using (List.mem_filter.mp hi).2⟩

def matVec [Zero F] [Add F] [Mul F]
    (cols : List Col) (B : Matrix Row Col F) (r : Vector Col F) :
    Vector Row F :=
  fun i => cols.foldl (fun acc j => acc + B i j * r j) 0

def transposeMatVec [Zero F] [Add F] [Mul F]
    (rows : List Row) (B : Matrix Row Col F) (s : Vector Row F) :
    Vector Col F :=
  fun j => rows.foldl (fun acc i => acc + B i j * s i) 0

theorem list_foldl_add_eq_add_sum [AddCommMonoid F]
    (xs : List ι) (f : ι -> F) (a : F) :
    xs.foldl (fun acc x => acc + f x) a = a + (xs.map f).sum := by
  induction xs generalizing a with
  | nil => simp
  | cons x xs ih =>
      rw [List.foldl_cons, ih]
      simp [add_assoc]

theorem list_foldl_add_zero_eq_sum [AddCommMonoid F]
    (xs : List ι) (f : ι -> F) :
    xs.foldl (fun acc x => acc + f x) 0 = (xs.map f).sum := by
  simpa using list_foldl_add_eq_add_sum xs f 0

theorem list_sum_map_eq_finset_sum_toFinset_of_nodup
    [AddCommMonoid F] [DecidableEq ι]
    {xs : List ι} (hxs : xs.Nodup) (f : ι -> F) :
    (xs.map f).sum = ∑ x ∈ xs.toFinset, f x := by
  induction xs with
  | nil => simp
  | cons x xs ih =>
      have hxnot : x ∉ xs := (List.nodup_cons.mp hxs).1
      have hxs' : xs.Nodup := (List.nodup_cons.mp hxs).2
      rw [List.map_cons, List.sum_cons, ih hxs']
      simp [hxnot]

theorem list_foldl_add_zero_eq_finset_sum_toFinset_of_nodup
    [AddCommMonoid F] [DecidableEq ι]
    {xs : List ι} (hxs : xs.Nodup) (f : ι -> F) :
    xs.foldl (fun acc x => acc + f x) 0 = ∑ x ∈ xs.toFinset, f x := by
  rw [list_foldl_add_zero_eq_sum, list_sum_map_eq_finset_sum_toFinset_of_nodup hxs]

theorem matVec_eq_finset_sum_toFinset_of_nodup
    [Field F] [DecidableEq Col]
    {cols : List Col} (hcols : cols.Nodup)
    (B : Matrix Row Col F) (r : Vector Col F) (i : Row) :
    matVec cols B r i = ∑ j ∈ cols.toFinset, B i j * r j := by
  exact list_foldl_add_zero_eq_finset_sum_toFinset_of_nodup hcols
    (fun j => B i j * r j)

theorem transposeMatVec_eq_finset_sum_toFinset_of_nodup
    [Field F] [DecidableEq Row]
    {rows : List Row} (hrows : rows.Nodup)
    (B : Matrix Row Col F) (s : Vector Row F) (j : Col) :
    transposeMatVec rows B s j = ∑ i ∈ rows.toFinset, B i j * s i := by
  exact list_foldl_add_zero_eq_finset_sum_toFinset_of_nodup hrows
    (fun i => B i j * s i)

def cauchyEntry [Sub F] [Inv F]
    (x : Row -> F) (alpha : Col -> F) (i : Row) (j : Col) : F :=
  (x i - alpha j)⁻¹

def cauchyDenomProduct [One F] [Mul F] [Sub F]
    (x : Row -> F) (alpha : Col -> F) (i : Row) (cols : List Col) : F :=
  (cols.map (fun j => x i - alpha j)).prod

def CauchySupportLowerBound [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  ∀ r : Vector Col F,
    weightOn cols r <= rows.length ->
    r ≠ zeroVec ->
    rows.length - weightOn cols r + 1 <= weightOn rows (matVec cols B r)

def CauchySupportLowerBoundOn [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  ∀ r : Vector Col F,
    weightOn cols r <= rows.length ->
    nonzeroOn cols r ->
    rows.length - weightOn cols r + 1 <= weightOn rows (matVec cols B r)

def CauchyGuardPrimalDistance [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  ∀ s : Vector Row F,
    s ≠ zeroVec ->
    cols.length - rows.length + 1 <= weightOn cols (transposeMatVec rows B s)

def CauchyGuardPrimalDistanceOn [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  ∀ s : Vector Row F,
    nonzeroOn rows s ->
    cols.length - rows.length + 1 <= weightOn cols (transposeMatVec rows B s)

def CauchyGuardPrimalDistanceStrong [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  ∀ s : Vector Row F,
    nonzeroOn rows s ->
    cols.length - weightOn rows s + 1 <=
      weightOn cols (transposeMatVec rows B s)

structure CauchyData (Row : Type u) (Col : Type v) (F : Type w) [Field F] where
  x : Row -> F
  alpha : Col -> F
  rowIndices : List Row
  colIndices : List Col
  hRowsCover : ∀ i, i ∈ rowIndices
  hColsCover : ∀ j, j ∈ colIndices
  hRowIndicesNodup : rowIndices.Nodup
  hColIndicesNodup : colIndices.Nodup
  hDistinctX : Function.Injective x
  hDistinctAlpha : Function.Injective alpha
  hDisjoint : ∀ i j, x i ≠ alpha j

def CauchyData.matrix [Field F]
    (D : CauchyData Row Col F) : Matrix Row Col F :=
  fun i j => cauchyEntry D.x D.alpha i j

theorem cauchy_denom_ne_zero [Field F]
    (D : CauchyData Row Col F) (i : Row) (j : Col) :
    D.x i - D.alpha j ≠ 0 :=
  sub_ne_zero_of_ne (D.hDisjoint i j)

theorem cauchy_entry_ne_zero [Field F]
    (D : CauchyData Row Col F) (i : Row) (j : Col) :
    D.matrix i j ≠ 0 := by
  unfold CauchyData.matrix cauchyEntry
  exact inv_ne_zero (cauchy_denom_ne_zero D i j)

theorem cauchy_entry_mul_denom [Field F]
    (D : CauchyData Row Col F) (i : Row) (j : Col) :
    D.matrix i j * (D.x i - D.alpha j) = 1 := by
  unfold CauchyData.matrix cauchyEntry
  exact inv_mul_cancel₀ (cauchy_denom_ne_zero D i j)

theorem cauchy_denom_mul_entry [Field F]
    (D : CauchyData Row Col F) (i : Row) (j : Col) :
    (D.x i - D.alpha j) * D.matrix i j = 1 := by
  unfold CauchyData.matrix cauchyEntry
  exact mul_inv_cancel₀ (cauchy_denom_ne_zero D i j)

theorem cauchy_denom_product_ne_zero [Field F]
    (D : CauchyData Row Col F) (i : Row) (cols : List Col) :
    cauchyDenomProduct D.x D.alpha i cols ≠ 0 := by
  unfold cauchyDenomProduct
  apply List.prod_ne_zero
  intro hzero
  rcases List.mem_map.mp hzero with ⟨j, _hj, hdenom⟩
  exact cauchy_denom_ne_zero D i j hdenom

theorem cauchy_same_row_denom_injective [Field F]
    (D : CauchyData Row Col F) (i : Row) :
    Function.Injective (fun j : Col => D.x i - D.alpha j) := by
  intro j k h
  exact D.hDistinctAlpha (sub_right_inj.mp h)

theorem cauchy_same_col_denom_injective [Field F]
    (D : CauchyData Row Col F) (j : Col) :
    Function.Injective (fun i : Row => D.x i - D.alpha j) := by
  intro i k h
  exact D.hDistinctX (sub_left_inj.mp h)

theorem cauchy_same_row_entry_injective [Field F]
    (D : CauchyData Row Col F) (i : Row) :
    Function.Injective (fun j : Col => D.matrix i j) := by
  intro j k h
  apply cauchy_same_row_denom_injective D i
  unfold CauchyData.matrix cauchyEntry at h
  exact inv_injective h

theorem cauchy_same_col_entry_injective [Field F]
    (D : CauchyData Row Col F) (j : Col) :
    Function.Injective (fun i : Row => D.matrix i j) := by
  intro i k h
  apply cauchy_same_col_denom_injective D j
  unfold CauchyData.matrix cauchyEntry at h
  exact inv_injective h

def cauchySquareSubmatrix [Field F]
    (D : CauchyData Row Col F) {κ : Type*}
    (rowSel : κ -> Row) (colSel : κ -> Col) : _root_.Matrix κ κ F :=
  fun a b => D.matrix (rowSel a) (colSel b)

theorem cauchy_square_submatrix_apply [Field F]
    (D : CauchyData Row Col F) {κ : Type*}
    (rowSel : κ -> Row) (colSel : κ -> Col) (a b : κ) :
    cauchySquareSubmatrix D rowSel colSel a b =
      D.matrix (rowSel a) (colSel b) :=
  rfl

theorem cauchy_square_submatrix_entry_ne_zero [Field F]
    (D : CauchyData Row Col F) {κ : Type*}
    (rowSel : κ -> Row) (colSel : κ -> Col) (a b : κ) :
    cauchySquareSubmatrix D rowSel colSel a b ≠ 0 := by
  exact cauchy_entry_ne_zero D (rowSel a) (colSel b)

theorem cauchy_one_by_one_minor_det_ne_zero [Field F]
    (D : CauchyData Row Col F) (i : Row) (j : Col) :
    _root_.Matrix.det (cauchySquareSubmatrix D
      (fun _ : Fin 1 => i) (fun _ : Fin 1 => j)) ≠ 0 := by
  rw [_root_.Matrix.det_fin_one]
  exact cauchy_entry_ne_zero D i j

theorem cauchy_two_by_two_minor_det_ne_zero [Field F]
    (D : CauchyData Row Col F)
    {i₀ i₁ : Row} (hi : i₀ ≠ i₁)
    {j₀ j₁ : Col} (hj : j₀ ≠ j₁) :
    _root_.Matrix.det (cauchySquareSubmatrix D
      (fun k : Fin 2 => if k = 0 then i₀ else i₁)
      (fun k : Fin 2 => if k = 0 then j₀ else j₁)) ≠ 0 := by
  rw [_root_.Matrix.det_fin_two]
  simp [cauchySquareSubmatrix, CauchyData.matrix, cauchyEntry]
  set x₀ := D.x i₀ with hx₀
  set x₁ := D.x i₁ with hx₁
  set a₀ := D.alpha j₀ with ha₀
  set a₁ := D.alpha j₁ with ha₁
  have hx_ne : x₀ ≠ x₁ := by
    intro h
    apply hi
    apply D.hDistinctX
    simpa [hx₀, hx₁] using h
  have ha_ne : a₀ ≠ a₁ := by
    intro h
    apply hj
    apply D.hDistinctAlpha
    simpa [ha₀, ha₁] using h
  have h00 : x₀ - a₀ ≠ 0 := by
    simpa [hx₀, ha₀] using cauchy_denom_ne_zero D i₀ j₀
  have h01 : x₀ - a₁ ≠ 0 := by
    simpa [hx₀, ha₁] using cauchy_denom_ne_zero D i₀ j₁
  have h10 : x₁ - a₀ ≠ 0 := by
    simpa [hx₁, ha₀] using cauchy_denom_ne_zero D i₁ j₀
  have h11 : x₁ - a₁ ≠ 0 := by
    simpa [hx₁, ha₁] using cauchy_denom_ne_zero D i₁ j₁
  field_simp [h00, h01, h10, h11]
  ring_nf
  have hprod : (x₀ - x₁) * (a₁ - a₀) ≠ 0 :=
    mul_ne_zero (sub_ne_zero_of_ne hx_ne) (sub_ne_zero_of_ne (Ne.symm ha_ne))
  convert hprod using 1
  ring_nf

/--
Total nonsingularity target for the square-submatrix route.

The public support and guard-distance bounds below now use the polynomial
root-count route directly.  This predicate remains useful if future work wants
the equivalent square-minor theorem as a reusable Cauchy API.
-/
def CauchyTotalNonsingular [Field F] (D : CauchyData Row Col F) : Prop :=
  ∀ {κ : Type*} [Fintype κ] [DecidableEq κ]
    {rowSel : κ -> Row} {colSel : κ -> Col},
    Function.Injective rowSel ->
    Function.Injective colSel ->
    _root_.Matrix.det (cauchySquareSubmatrix D rowSel colSel) ≠ 0

def supportFinsetOn [Zero F] [DecidableEq ι] [DecidableEq F]
    (indices : List ι) (x : Vector ι F) : Finset ι :=
  (supportOn indices x).toFinset

theorem supportFinsetOn_card_le_weightOn [Zero F] [DecidableEq ι] [DecidableEq F]
    (indices : List ι) (x : Vector ι F) :
    (supportFinsetOn indices x).card <= weightOn indices x := by
  unfold supportFinsetOn weightOn
  exact List.toFinset_card_le _

theorem supportFinsetOn_card_eq_weightOn_of_nodup
    [Zero F] [DecidableEq ι] [DecidableEq F]
    {indices : List ι} (x : Vector ι F)
    (hindices : indices.Nodup) :
    (supportFinsetOn indices x).card = weightOn indices x := by
  unfold supportFinsetOn weightOn
  exact List.toFinset_card_of_nodup (supportOn_nodup hindices)

theorem supportFinsetOn_eq_filter_toFinset
    [Zero F] [DecidableEq ι] [DecidableEq F]
    (indices : List ι) (x : Vector ι F) :
    supportFinsetOn indices x = indices.toFinset.filter (fun i => x i ≠ 0) := by
  ext i
  simp [supportFinsetOn, supportOn]

theorem finset_sum_eq_sum_supportFinsetOn [Field F] [DecidableEq ι] [DecidableEq F]
    (indices : List ι) (x : Vector ι F) (f : ι -> F) :
    ∑ i ∈ indices.toFinset, f i * x i =
      ∑ i ∈ supportFinsetOn indices x, f i * x i := by
  rw [supportFinsetOn_eq_filter_toFinset]
  rw [Finset.sum_filter]
  apply Finset.sum_congr rfl
  intro i hi
  by_cases hxi : x i ≠ 0
  · simp [hxi]
  · have hxi0 : x i = 0 := by simpa using hxi
    simp [hxi0]

theorem supportFinsetOn_nonempty_of_nonzeroOn
    [Zero F] [DecidableEq ι] [DecidableEq F]
    {indices : List ι} {x : Vector ι F}
    (hx : nonzeroOn indices x) :
    (supportFinsetOn indices x).Nonempty := by
  rcases hx with ⟨i, hi, hxi⟩
  exact ⟨i, by
    unfold supportFinsetOn supportOn
    simp [hi, hxi]⟩

theorem mem_supportFinsetOn_iff
    [Zero F] [DecidableEq ι] [DecidableEq F]
    {indices : List ι} {x : Vector ι F} {i : ι} :
    i ∈ supportFinsetOn indices x ↔ i ∈ indices ∧ x i ≠ 0 := by
  unfold supportFinsetOn supportOn
  simp

noncomputable def cauchyDenomPolynomial [Field F] [DecidableEq Col]
    (alpha : Col -> F) (cols : Finset Col) : F[X] :=
  ∏ j ∈ cols, (X - C (alpha j))

noncomputable def cauchyClearedNumerator [Field F] [DecidableEq Col]
    (alpha : Col -> F) (r : Vector Col F) (cols : Finset Col) : F[X] :=
  ∑ j ∈ cols, C (r j) * ∏ k ∈ cols.erase j, (X - C (alpha k))

theorem polynomial_root_finset_card_le_natDegree
    [Field F] [DecidableEq F]
    {p : F[X]} {s : Finset F}
    (hp : p ≠ 0) (hs : ∀ x, x ∈ s -> p.eval x = 0) :
    s.card <= p.natDegree := by
  calc
    s.card <= p.roots.toFinset.card := by
      apply Finset.card_mono
      intro x hx
      exact Multiset.mem_toFinset.mpr
        ((Polynomial.mem_roots hp).mpr (by
          rw [Polynomial.IsRoot]
          exact hs x hx))
    _ <= p.roots.card := Multiset.toFinset_card_le _
    _ <= p.natDegree := Polynomial.card_roots' p

theorem cauchyClearedNumerator_natDegree_le_pred
    [Field F] [DecidableEq Col]
    (alpha : Col -> F) (r : Vector Col F) (cols : Finset Col) :
    (cauchyClearedNumerator alpha r cols).natDegree <= cols.card - 1 := by
  unfold cauchyClearedNumerator
  refine Polynomial.natDegree_sum_le_of_forall_le cols
    (fun j => C (r j) * ∏ k ∈ cols.erase j, (X - C (alpha k))) ?_
  intro j hj
  calc
    (C (r j) * ∏ k ∈ cols.erase j, (X - C (alpha k))).natDegree
        <= (C (r j)).natDegree +
            (∏ k ∈ cols.erase j, (X - C (alpha k))).natDegree :=
      Polynomial.natDegree_mul_le
    _ <= 0 + (cols.erase j).card := by
      gcongr
      · exact (Polynomial.natDegree_C (r j)).le
      · simp
    _ <= cols.card - 1 := by
      simp [Finset.card_erase_of_mem hj]

theorem cauchyClearedNumerator_eval_eq_term
    [Field F] [DecidableEq Col]
    {alpha : Col -> F} {r : Vector Col F} {cols : Finset Col} {j : Col}
    (hj : j ∈ cols) :
    (cauchyClearedNumerator alpha r cols).eval (alpha j) =
      r j * ∏ k ∈ cols.erase j, (alpha j - alpha k) := by
  unfold cauchyClearedNumerator
  rw [Polynomial.eval_finset_sum, Finset.sum_eq_single j]
  · rw [Polynomial.eval_mul, Polynomial.eval_C, Polynomial.eval_prod]
    simp [Polynomial.eval_sub]
  · intro k hk hkj
    rw [Polynomial.eval_mul, Polynomial.eval_C]
    apply mul_eq_zero_of_right
    rw [Polynomial.eval_prod]
    exact Finset.prod_eq_zero (Finset.mem_erase.mpr ⟨hkj.symm, hj⟩) (by simp)
  · intro hnot
    exact (hnot hj).elim

theorem cauchyClearedNumerator_eval_ne_zero_of_mem_support
    [Field F] [DecidableEq Col]
    {alpha : Col -> F} {r : Vector Col F} {cols : Finset Col} {j : Col}
    (hj : j ∈ cols) (hrj : r j ≠ 0) (halpha : Function.Injective alpha) :
    (cauchyClearedNumerator alpha r cols).eval (alpha j) ≠ 0 := by
  rw [cauchyClearedNumerator_eval_eq_term hj]
  apply mul_ne_zero hrj
  rw [Finset.prod_ne_zero_iff]
  intro k hk
  have hkj : k ≠ j := (Finset.mem_erase.mp hk).1
  exact sub_ne_zero_of_ne (fun h => hkj ((halpha h).symm))

theorem cauchyClearedNumerator_ne_zero_of_mem_support
    [Field F] [DecidableEq Col]
    {alpha : Col -> F} {r : Vector Col F} {cols : Finset Col} {j : Col}
    (hj : j ∈ cols) (hrj : r j ≠ 0) (halpha : Function.Injective alpha) :
    cauchyClearedNumerator alpha r cols ≠ 0 := by
  intro hzero
  have hEvalZero : (cauchyClearedNumerator alpha r cols).eval (alpha j) = 0 := by
    rw [hzero, Polynomial.eval_zero]
  exact (cauchyClearedNumerator_eval_ne_zero_of_mem_support hj hrj halpha) hEvalZero

theorem polynomial_root_list_length_le_natDegree
    [Field F] [DecidableEq F]
    {p : F[X]} {xs : List F}
    (hp : p ≠ 0) (hxs : xs.Nodup)
    (hroot : ∀ x, x ∈ xs -> p.eval x = 0) :
    xs.length <= p.natDegree := by
  have hcard : xs.toFinset.card = xs.length := List.toFinset_card_of_nodup hxs
  rw [← hcard]
  exact polynomial_root_finset_card_le_natDegree hp (by
    intro x hx
    exact hroot x (by simpa using hx))

theorem cauchyClearedNumerator_root_list_length_le_pred
    [Field F] [DecidableEq F] [DecidableEq Col]
    {alpha : Col -> F} {r : Vector Col F} {cols : Finset Col} {xs : List F} {j : Col}
    (hj : j ∈ cols) (hrj : r j ≠ 0) (halpha : Function.Injective alpha)
    (hxs : xs.Nodup)
    (hroot : ∀ x, x ∈ xs -> (cauchyClearedNumerator alpha r cols).eval x = 0) :
    xs.length <= cols.card - 1 := by
  exact Nat.le_trans
    (polynomial_root_list_length_le_natDegree
      (cauchyClearedNumerator_ne_zero_of_mem_support hj hrj halpha) hxs hroot)
    (cauchyClearedNumerator_natDegree_le_pred alpha r cols)

theorem cauchyClearedNumerator_eval_eq_denom_mul_sum
    [Field F] [DecidableEq Col]
    {alpha : Col -> F} {r : Vector Col F} {cols : Finset Col} {t : F}
    (hden : ∀ j, j ∈ cols -> t - alpha j ≠ 0) :
    (cauchyClearedNumerator alpha r cols).eval t =
      (∏ k ∈ cols, (t - alpha k)) *
        ∑ j ∈ cols, (t - alpha j)⁻¹ * r j := by
  unfold cauchyClearedNumerator
  rw [Polynomial.eval_finset_sum, Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro j hj
  rw [Polynomial.eval_mul, Polynomial.eval_C, Polynomial.eval_prod]
  simp only [Polynomial.eval_sub, Polynomial.eval_X, Polynomial.eval_C]
  have hprod : ∏ k ∈ cols, (t - alpha k) =
      (t - alpha j) * ∏ k ∈ cols.erase j, (t - alpha k) := by
    exact (Finset.mul_prod_erase cols (fun k => t - alpha k) hj).symm
  rw [hprod]
  field_simp [hden j hj]

theorem cauchyClearedNumerator_eval_eq_zero_of_sum_eq_zero
    [Field F] [DecidableEq Col]
    {alpha : Col -> F} {r : Vector Col F} {cols : Finset Col} {t : F}
    (hden : ∀ j, j ∈ cols -> t - alpha j ≠ 0)
    (hsum : ∑ j ∈ cols, (t - alpha j)⁻¹ * r j = 0) :
    (cauchyClearedNumerator alpha r cols).eval t = 0 := by
  rw [cauchyClearedNumerator_eval_eq_denom_mul_sum hden, hsum, mul_zero]

theorem cauchy_row_values_nodup [Field F]
    (D : CauchyData Row Col F) :
    (D.rowIndices.map D.x).Nodup := by
  exact D.hRowIndicesNodup.map D.hDistinctX

theorem cauchy_col_values_nodup [Field F]
    (D : CauchyData Row Col F) :
    (D.colIndices.map D.alpha).Nodup := by
  exact D.hColIndicesNodup.map D.hDistinctAlpha

theorem cauchy_support_row_indices_nodup [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) (s : Vector Row F) :
    (supportOn D.rowIndices s).Nodup :=
  supportOn_nodup D.hRowIndicesNodup

theorem cauchy_support_col_indices_nodup [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) (r : Vector Col F) :
    (supportOn D.colIndices r).Nodup :=
  supportOn_nodup D.hColIndicesNodup

theorem cauchy_support_row_values_nodup [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) (s : Vector Row F) :
    ((supportOn D.rowIndices s).map D.x).Nodup := by
  exact (cauchy_support_row_indices_nodup D s).map D.hDistinctX

theorem cauchy_support_col_values_nodup [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) (r : Vector Col F) :
    ((supportOn D.colIndices r).map D.alpha).Nodup := by
  exact (cauchy_support_col_indices_nodup D r).map D.hDistinctAlpha

theorem cauchy_mem_support_col_iff [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) (r : Vector Col F) (j : Col) :
    j ∈ supportOn D.colIndices r ↔ j ∈ D.colIndices ∧ r j ≠ 0 :=
  mem_supportOn

theorem cauchy_mem_support_row_iff [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) (s : Vector Row F) (i : Row) :
    i ∈ supportOn D.rowIndices s ↔ i ∈ D.rowIndices ∧ s i ≠ 0 :=
  mem_supportOn

theorem cauchy_support_col_nonzero [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) {r : Vector Col F} {j : Col}
    (hj : j ∈ supportOn D.colIndices r) :
    r j ≠ 0 :=
  (cauchy_mem_support_col_iff D r j).mp hj |>.2

theorem cauchy_matVec_eq_active_sum [Field F] [DecidableEq F] [DecidableEq Col]
    (D : CauchyData Row Col F) (r : Vector Col F) (i : Row) :
    matVec D.colIndices D.matrix r i =
      ∑ j ∈ supportFinsetOn D.colIndices r, (D.x i - D.alpha j)⁻¹ * r j := by
  rw [matVec_eq_finset_sum_toFinset_of_nodup D.hColIndicesNodup]
  rw [finset_sum_eq_sum_supportFinsetOn D.colIndices r (fun j => D.matrix i j)]
  simp [CauchyData.matrix, cauchyEntry]

theorem cauchy_transposeMatVec_eq_active_sum [Field F] [DecidableEq F] [DecidableEq Row]
    (D : CauchyData Row Col F) (s : Vector Row F) (j : Col) :
    transposeMatVec D.rowIndices D.matrix s j =
      ∑ i ∈ supportFinsetOn D.rowIndices s, (D.x i - D.alpha j)⁻¹ * s i := by
  rw [transposeMatVec_eq_finset_sum_toFinset_of_nodup D.hRowIndicesNodup]
  rw [finset_sum_eq_sum_supportFinsetOn D.rowIndices s (fun i => D.matrix i j)]
  simp [CauchyData.matrix, cauchyEntry]

theorem cauchyClearedNumerator_eval_eq_zero_of_matVec_eq_zero [Field F]
    [DecidableEq F] [DecidableEq Col]
    (D : CauchyData Row Col F) {r : Vector Col F} {i : Row}
    (hzero : matVec D.colIndices D.matrix r i = 0) :
    (cauchyClearedNumerator D.alpha r (supportFinsetOn D.colIndices r)).eval
      (D.x i) = 0 := by
  refine cauchyClearedNumerator_eval_eq_zero_of_sum_eq_zero ?_ ?_
  · intro j hj
    exact cauchy_denom_ne_zero D i j
  · simpa [cauchy_matVec_eq_active_sum D r i] using hzero

theorem cauchyClearedNumerator_eval_eq_zero_of_transposeMatVec_eq_zero [Field F]
    [DecidableEq F] [DecidableEq Row]
    (D : CauchyData Row Col F) {s : Vector Row F} {j : Col}
    (hzero : transposeMatVec D.rowIndices D.matrix s j = 0) :
    (cauchyClearedNumerator D.x s (supportFinsetOn D.rowIndices s)).eval
      (D.alpha j) = 0 := by
  refine cauchyClearedNumerator_eval_eq_zero_of_sum_eq_zero ?_ ?_
  · intro i hi
    exact sub_ne_zero_of_ne (Ne.symm (D.hDisjoint i j))
  · have hsum :
        ∑ i ∈ supportFinsetOn D.rowIndices s, (D.x i - D.alpha j)⁻¹ * s i = 0 := by
      simpa [cauchy_transposeMatVec_eq_active_sum D s j] using hzero
    have hneg :
        ∑ i ∈ supportFinsetOn D.rowIndices s, (D.alpha j - D.x i)⁻¹ * s i =
          -∑ i ∈ supportFinsetOn D.rowIndices s, (D.x i - D.alpha j)⁻¹ * s i := by
      rw [← Finset.sum_neg_distrib]
      apply Finset.sum_congr rfl
      intro i hi
      have hsign : D.alpha j - D.x i = -(D.x i - D.alpha j) := by ring
      rw [hsign, inv_neg]
      ring
    rw [hneg, hsum, neg_zero]

theorem cauchyClearedNumerator_ne_zero_of_nonzeroOn [Field F]
    [DecidableEq F] [DecidableEq Col]
    (D : CauchyData Row Col F) {r : Vector Col F}
    (hr : nonzeroOn D.colIndices r) :
    cauchyClearedNumerator D.alpha r (supportFinsetOn D.colIndices r) ≠ 0 := by
  rcases hr with ⟨j, hjCol, hrj⟩
  have hj : j ∈ supportFinsetOn D.colIndices r := by
    rw [mem_supportFinsetOn_iff]
    exact ⟨hjCol, hrj⟩
  exact cauchyClearedNumerator_ne_zero_of_mem_support hj hrj D.hDistinctAlpha

theorem cauchy_support_row_nonzero [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) {s : Vector Row F} {i : Row}
    (hi : i ∈ supportOn D.rowIndices s) :
    s i ≠ 0 :=
  (cauchy_mem_support_row_iff D s i).mp hi |>.2

theorem cauchyClearedNumerator_root_row_values_length_le_support_pred [Field F]
    [DecidableEq F] [DecidableEq Col]
    (D : CauchyData Row Col F) {r : Vector Col F} {rows : List Row}
    (hr : nonzeroOn D.colIndices r)
    (hrows : rows.Nodup)
    (hroot :
      ∀ i, i ∈ rows ->
        (cauchyClearedNumerator D.alpha r (supportFinsetOn D.colIndices r)).eval
          (D.x i) = 0) :
    rows.length <= weightOn D.colIndices r - 1 := by
  rcases hr with ⟨j, hjCol, hrj⟩
  have hj : j ∈ supportFinsetOn D.colIndices r := by
    rw [mem_supportFinsetOn_iff]
    exact ⟨hjCol, hrj⟩
  have hbound :
      (rows.map D.x).length <= (supportFinsetOn D.colIndices r).card - 1 :=
    cauchyClearedNumerator_root_list_length_le_pred
      (alpha := D.alpha) (r := r) (cols := supportFinsetOn D.colIndices r)
      (xs := rows.map D.x) (j := j) hj hrj D.hDistinctAlpha
      (hrows.map D.hDistinctX)
      (by
        intro x hx
        rcases List.mem_map.mp hx with ⟨i, hi, rfl⟩
        exact hroot i hi)
  simpa [supportFinsetOn_card_eq_weightOn_of_nodup r D.hColIndicesNodup] using hbound

theorem cauchyClearedNumerator_root_values_length_le_support_pred [Field F]
    [DecidableEq F] [DecidableEq Col]
    {Point : Type*}
    {alpha : Col -> F} {point : Point -> F}
    {indices : List Col} {r : Vector Col F} {points : List Point}
    (hindices : indices.Nodup)
    (hr : nonzeroOn indices r)
    (halpha : Function.Injective alpha)
    (hpoint : Function.Injective point)
    (hpoints : points.Nodup)
    (hroot :
      ∀ p, p ∈ points ->
        (cauchyClearedNumerator alpha r (supportFinsetOn indices r)).eval
          (point p) = 0) :
    points.length <= weightOn indices r - 1 := by
  rcases hr with ⟨j, hjIndex, hrj⟩
  have hj : j ∈ supportFinsetOn indices r := by
    rw [mem_supportFinsetOn_iff]
    exact ⟨hjIndex, hrj⟩
  have hbound :
      (points.map point).length <= (supportFinsetOn indices r).card - 1 :=
    cauchyClearedNumerator_root_list_length_le_pred
      (alpha := alpha) (r := r) (cols := supportFinsetOn indices r)
      (xs := points.map point) (j := j) hj hrj halpha
      (hpoints.map hpoint)
      (by
        intro x hx
        rcases List.mem_map.mp hx with ⟨p, hp, rfl⟩
        exact hroot p hp)
  simpa [supportFinsetOn_card_eq_weightOn_of_nodup r hindices] using hbound

theorem cauchy_support_lower_bound_of_on
    [Zero F] [Add F] [Mul F] [DecidableEq F]
    {rows : List Row} {cols : List Col} {B : Matrix Row Col F}
    (hMDS : CauchySupportLowerBoundOn rows cols B)
    (hcols : ∀ j : Col, j ∈ cols) :
    CauchySupportLowerBound rows cols B := by
  intro r hSmall hr
  exact hMDS r hSmall (nonzeroOn_of_forall_mem_of_ne_zero hcols hr)

theorem cauchy_guard_primal_distance_on_of_strong
    [Zero F] [Add F] [Mul F] [DecidableEq F]
    {rows : List Row} {cols : List Col} {B : Matrix Row Col F}
    (hMDS : CauchyGuardPrimalDistanceStrong rows cols B) :
    CauchyGuardPrimalDistanceOn rows cols B := by
  intro s hs
  exact Nat.le_trans (by
    have hWeight : weightOn rows s <= rows.length := weightOn_le_length rows s
    omega) (hMDS s hs)

theorem cauchy_guard_primal_distance_of_on
    [Zero F] [Add F] [Mul F] [DecidableEq F]
    {rows : List Row} {cols : List Col} {B : Matrix Row Col F}
    (hMDS : CauchyGuardPrimalDistanceOn rows cols B)
    (hrows : ∀ i : Row, i ∈ rows) :
    CauchyGuardPrimalDistance rows cols B := by
  intro s hs
  exact hMDS s (nonzeroOn_of_forall_mem_of_ne_zero hrows hs)

theorem cauchy_guard_primal_distance_of_strong
    [Zero F] [Add F] [Mul F] [DecidableEq F]
    {rows : List Row} {cols : List Col} {B : Matrix Row Col F}
    (hMDS : CauchyGuardPrimalDistanceStrong rows cols B)
    (hrows : ∀ i : Row, i ∈ rows) :
    CauchyGuardPrimalDistance rows cols B :=
  cauchy_guard_primal_distance_of_on
    (cauchy_guard_primal_distance_on_of_strong hMDS) hrows

theorem cauchy_support_lower_bound_on
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) :
    CauchySupportLowerBoundOn D.rowIndices D.colIndices D.matrix := by
  classical
  intro r hSmall hr
  let y : Vector Row F := matVec D.colIndices D.matrix r
  let zeroRows : List Row := D.rowIndices.filter (fun i => decide (y i = 0))
  have hzeroNodup : zeroRows.Nodup := by
    unfold zeroRows
    exact D.hRowIndicesNodup.filter _
  have hroot :
      ∀ i, i ∈ zeroRows ->
        (cauchyClearedNumerator D.alpha r (supportFinsetOn D.colIndices r)).eval
          (D.x i) = 0 := by
    intro i hi
    have hyi : y i = 0 := by
      unfold zeroRows at hi
      simpa using (List.mem_filter.mp hi).2
    exact cauchyClearedNumerator_eval_eq_zero_of_matVec_eq_zero D hyi
  have hzeroBound : zeroRows.length <= weightOn D.colIndices r - 1 :=
    cauchyClearedNumerator_root_row_values_length_le_support_pred D hr hzeroNodup hroot
  have hWeightPos : 0 < weightOn D.colIndices r := weightOn_pos_of_nonzeroOn hr
  have hzeroSucc : zeroRows.length + 1 <= weightOn D.colIndices r := by
    omega
  have hlen : D.rowIndices.length = zeroRows.length + weightOn D.rowIndices y := by
    unfold zeroRows weightOn supportOn y
    rw [List.length_eq_length_filter_add
      (l := D.rowIndices)
      (f := fun i => decide (matVec D.colIndices D.matrix r i = 0))]
    congr 1
    apply congrArg List.length
    apply List.filter_congr
    intro i hi
    by_cases h : matVec D.colIndices D.matrix r i = 0
    · simp [h]
    · simp [h]
  change D.rowIndices.length - weightOn D.colIndices r + 1 <= weightOn D.rowIndices y
  omega

theorem cauchy_guard_primal_distance_strong_of_weight_le_cols
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) :
    ∀ s : Vector Row F,
      weightOn D.rowIndices s <= D.colIndices.length ->
      nonzeroOn D.rowIndices s ->
      D.colIndices.length - weightOn D.rowIndices s + 1 <=
        weightOn D.colIndices (transposeMatVec D.rowIndices D.matrix s) := by
  classical
  intro s hSmall hs
  let y : Vector Col F := transposeMatVec D.rowIndices D.matrix s
  let zeroCols : List Col := D.colIndices.filter (fun j => decide (y j = 0))
  have hzeroNodup : zeroCols.Nodup := by
    unfold zeroCols
    exact D.hColIndicesNodup.filter _
  have hroot :
      ∀ j, j ∈ zeroCols ->
        (cauchyClearedNumerator D.x s (supportFinsetOn D.rowIndices s)).eval
          (D.alpha j) = 0 := by
    intro j hj
    have hyj : y j = 0 := by
      unfold zeroCols at hj
      simpa using (List.mem_filter.mp hj).2
    exact cauchyClearedNumerator_eval_eq_zero_of_transposeMatVec_eq_zero D hyj
  have hzeroBound : zeroCols.length <= weightOn D.rowIndices s - 1 :=
    cauchyClearedNumerator_root_values_length_le_support_pred
      (hindices := D.hRowIndicesNodup) (hr := hs)
      (halpha := D.hDistinctX) (hpoint := D.hDistinctAlpha)
      (hpoints := hzeroNodup) (hroot := hroot)
  have hWeightPos : 0 < weightOn D.rowIndices s := weightOn_pos_of_nonzeroOn hs
  have hzeroSucc : zeroCols.length + 1 <= weightOn D.rowIndices s := by
    omega
  have hlen : D.colIndices.length = zeroCols.length + weightOn D.colIndices y := by
    unfold zeroCols weightOn supportOn y
    rw [List.length_eq_length_filter_add
      (l := D.colIndices)
      (f := fun j => decide (transposeMatVec D.rowIndices D.matrix s j = 0))]
    congr 1
    apply congrArg List.length
    apply List.filter_congr
    intro j hj
    by_cases h : transposeMatVec D.rowIndices D.matrix s j = 0
    · simp [h]
    · simp [h]
  change D.colIndices.length - weightOn D.rowIndices s + 1 <= weightOn D.colIndices y
  omega

theorem cauchy_guard_primal_distance_strong
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F)
    (hRowsLeCols : D.rowIndices.length <= D.colIndices.length) :
    CauchyGuardPrimalDistanceStrong D.rowIndices D.colIndices D.matrix := by
  intro s hs
  exact cauchy_guard_primal_distance_strong_of_weight_le_cols D s
    (Nat.le_trans (weightOn_le_length D.rowIndices s) hRowsLeCols) hs

theorem cauchy_support_lower_bound
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) :
    CauchySupportLowerBound D.rowIndices D.colIndices D.matrix := by
  exact cauchy_support_lower_bound_of_on
    (cauchy_support_lower_bound_on D) D.hColsCover

theorem cauchy_guard_primal_distance
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F)
    (hRowsLeCols : D.rowIndices.length <= D.colIndices.length) :
    CauchyGuardPrimalDistance D.rowIndices D.colIndices D.matrix := by
  exact cauchy_guard_primal_distance_of_strong
    (cauchy_guard_primal_distance_strong D hRowsLeCols) D.hRowsCover

abbrev GuardSupportLowerBoundOn [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  CauchySupportLowerBoundOn rows cols B

abbrev GuardSupportLowerBound [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  CauchySupportLowerBound rows cols B

abbrev GuardBlockPrimalDistanceStrong [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  CauchyGuardPrimalDistanceStrong rows cols B

abbrev GuardBlockPrimalDistanceOn [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  CauchyGuardPrimalDistanceOn rows cols B

abbrev GuardBlockPrimalDistance [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List Row) (cols : List Col) (B : Matrix Row Col F) : Prop :=
  CauchyGuardPrimalDistance rows cols B

theorem cauchy_proves_guard_support_lower_bound_on
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) :
    GuardSupportLowerBoundOn D.rowIndices D.colIndices D.matrix :=
  cauchy_support_lower_bound_on D

theorem cauchy_proves_guard_support_lower_bound
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F) :
    GuardSupportLowerBound D.rowIndices D.colIndices D.matrix :=
  cauchy_support_lower_bound D

theorem cauchy_proves_guard_block_primal_distance_strong
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F)
    (hRowsLeCols : D.rowIndices.length <= D.colIndices.length) :
    GuardBlockPrimalDistanceStrong D.rowIndices D.colIndices D.matrix :=
  cauchy_guard_primal_distance_strong D hRowsLeCols

theorem cauchy_proves_guard_block_primal_distance
    [Field F] [DecidableEq F]
    (D : CauchyData Row Col F)
    (hRowsLeCols : D.rowIndices.length <= D.colIndices.length) :
    GuardBlockPrimalDistance D.rowIndices D.colIndices D.matrix :=
  cauchy_guard_primal_distance D hRowsLeCols

noncomputable def cauchyGuardPrimalDistanceStrongCounterexampleMatrix :
    Matrix Bool PUnit ℚ :=
  fun i j =>
    cauchyEntry
      (fun b : Bool => if b then (1 : ℚ) else 0)
      (fun _ : PUnit => (2 : ℚ)) i j

noncomputable def cauchyGuardPrimalDistanceStrongCounterexampleVector :
    Vector Bool ℚ :=
  fun i => if i then (-1 : ℚ) else 2

noncomputable def cauchyGuardPrimalDistanceStrongCounterexampleData :
    CauchyData Bool PUnit ℚ where
  x := fun b => if b then (1 : ℚ) else 0
  alpha := fun _ => (2 : ℚ)
  rowIndices := [false, true]
  colIndices := [PUnit.unit]
  hRowsCover := by
    intro i
    cases i <;> simp
  hColsCover := by
    intro j
    cases j
    simp
  hRowIndicesNodup := by simp
  hColIndicesNodup := by simp
  hDistinctX := by
    intro a b h
    cases a <;> cases b <;> simp at h ⊢
  hDistinctAlpha := by
    intro a b h
    cases a
    cases b
    rfl
  hDisjoint := by
    intro i j
    cases i <;> cases j <;> norm_num

theorem cauchy_guard_primal_distance_strong_counterexample :
    ¬ CauchyGuardPrimalDistanceStrong [false, true] [PUnit.unit]
      cauchyGuardPrimalDistanceStrongCounterexampleMatrix := by
  intro h
  have hs : nonzeroOn [false, true]
      cauchyGuardPrimalDistanceStrongCounterexampleVector := by
    refine ⟨false, ?_, ?_⟩
    · simp
    · simp [cauchyGuardPrimalDistanceStrongCounterexampleVector]
  have hb := h cauchyGuardPrimalDistanceStrongCounterexampleVector hs
  norm_num [
    weightOn,
    supportOn,
    transposeMatVec,
    cauchyGuardPrimalDistanceStrongCounterexampleMatrix,
    cauchyGuardPrimalDistanceStrongCounterexampleVector,
    cauchyEntry
  ] at hb

theorem cauchy_guard_primal_distance_strong_counterexample_data :
    ¬ CauchyGuardPrimalDistanceStrong
      cauchyGuardPrimalDistanceStrongCounterexampleData.rowIndices
      cauchyGuardPrimalDistanceStrongCounterexampleData.colIndices
      cauchyGuardPrimalDistanceStrongCounterexampleData.matrix := by
  simpa [
    cauchyGuardPrimalDistanceStrongCounterexampleData,
    CauchyData.matrix,
    cauchyGuardPrimalDistanceStrongCounterexampleMatrix
  ] using cauchy_guard_primal_distance_strong_counterexample

end Descender
