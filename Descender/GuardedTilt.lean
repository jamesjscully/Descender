import Descender.Cauchy

namespace Descender

/-!
Guarded tilted projection predicates and deterministic transversality
statements.

The model is deliberately finite-coordinate and interface-first.  The exported
deterministic theorem is phrased around one guarded operator `A` and its
adjoint action `A.adjoint`; the locality hypotheses attach only to the tilted
adjoint summand.
-/

def addVec [Add F] (x y : Vector ι F) : Vector ι F :=
  fun i => x i + y i

def single [Zero F] [One F] [DecidableEq ι] (j : ι) : Vector ι F :=
  fun k => if k = j then 1 else 0

def sumIndices (left : List ι) (right : List κ) : List (Sum ι κ) :=
  left.map Sum.inl ++ right.map Sum.inr

def pairToSum (r : Vector J0 F × Vector J1 F) : Vector (Sum J0 J1) F
  | Sum.inl j => r.1 j
  | Sum.inr j => r.2 j

def pairWeightOn [Zero F] [DecidableEq F]
    (left : List J0) (right : List J1) (r : Vector J0 F × Vector J1 F) :
    Nat :=
  weightOn left r.1 + weightOn right r.2

def zeroPair [Zero F] : Vector J0 F × Vector J1 F :=
  (zeroVec, zeroVec)

def combinedMatrix [Zero F]
    (B0 : Matrix I J0 F) (B1 : Matrix I J1 F) :
    Matrix I (Sum J0 J1) F
  | i, Sum.inl j => B0 i j
  | i, Sum.inr j => B1 i j

def guardedBase [Zero F] [Add F] [Mul F]
    (j0 : List J0) (j1 : List J1)
    (B0 : Matrix I J0 F) (B1 : Matrix I J1 F)
    (r : Vector J0 F × Vector J1 F) : Vector I F :=
  matVec (sumIndices j0 j1) (combinedMatrix B0 B1) (pairToSum r)

def guardedAT [Zero F] [Add F] [Mul F]
    (j0 : List J0) (j1 : List J1)
    (B0 : Matrix I J0 F) (B1 : Matrix I J1 F)
    (H : Vector J1 F -> Vector I F)
    (r : Vector J0 F × Vector J1 F) : Vector I F :=
  addVec (guardedBase j0 j1 B0 B1 r) (H r.2)

def dotOn [Zero F] [Add F] [Mul F]
    (indices : List ι) (x y : Vector ι F) : F :=
  indices.foldl (fun acc i => acc + x i * y i) 0

structure TiltAdjointPair [Zero F] [Add F] [Mul F]
    (rows : List I) (cols : List J)
    (Tilt : Vector I F -> Vector J F)
    (H : Vector J F -> Vector I F) : Prop where
  adjoint : ∀ s r, dotOn rows s (H r) = dotOn cols (Tilt s) r

structure GuardedTiltOperator [Zero F] [Add F] [Mul F]
    (rows : List I) (j0 : List J0) (j1 : List J1) where
  B0 : Matrix I J0 F
  B1 : Matrix I J1 F
  tilt : Vector I F -> Vector J1 F
  tiltAdjoint : Vector J1 F -> Vector I F
  hTiltAdjoint : TiltAdjointPair rows j1 tilt tiltAdjoint

namespace GuardedTiltOperator

variable [Zero F] [Add F] [Mul F]

def baseMatrix {rows : List I} {j0 : List J0} {j1 : List J1}
    (A : GuardedTiltOperator (F := F) rows j0 j1) : Matrix I (Sum J0 J1) F :=
  combinedMatrix A.B0 A.B1

def primal {rows : List I} {j0 : List J0} {j1 : List J1}
    (A : GuardedTiltOperator (F := F) rows j0 j1) (s : Vector I F) :
    Vector J0 F × Vector J1 F :=
  (transposeMatVec rows A.B0 s, addVec (transposeMatVec rows A.B1 s) (A.tilt s))

def adjoint {rows : List I} {j0 : List J0} {j1 : List J1}
    (A : GuardedTiltOperator (F := F) rows j0 j1)
    (r : Vector J0 F × Vector J1 F) : Vector I F :=
  guardedAT j0 j1 A.B0 A.B1 A.tiltAdjoint r

end GuardedTiltOperator

def ColumnSupportBound [Zero F] [One F] [DecidableEq F] [DecidableEq J]
    (rows : List I) (cols : List J) (H : Vector J F -> Vector I F)
    (L : Nat) : Prop :=
  ∀ j, j ∈ cols -> weightOn rows (H (single j)) <= L

def ColumnwiseSupportControlled [Zero F] [One F] [DecidableEq F] [DecidableEq J]
    (rows : List I) (cols : List J) (H : Vector J F -> Vector I F) :
    Prop :=
  ∀ r : Vector J F,
    (supportOn rows (H r)).Sublist
      ((supportOn cols r).map
        (fun j => supportOn rows (H (single j)))).flatten

def finiteColumnCombination [Zero F] [Add F] [Mul F] [One F] [DecidableEq J]
    (cols : List J) (H : Vector J F -> Vector I F) (r : Vector J F) :
    Vector I F :=
  fun i => cols.foldl (fun acc j => acc + r j * H (single j) i) 0

def FiniteColumnCombination [Zero F] [Add F] [Mul F] [One F] [DecidableEq J]
    (cols : List J) (H : Vector J F -> Vector I F) : Prop :=
  ∀ r : Vector J F, H r = finiteColumnCombination cols H r

namespace GuardedTiltOperator

variable [Zero F] [Add F] [Mul F]

def TiltColumnSupportBound [One F] [DecidableEq F] [DecidableEq J1]
    {rows : List I} {j0 : List J0} {j1 : List J1}
    (A : GuardedTiltOperator (F := F) rows j0 j1) (L : Nat) : Prop :=
  ColumnSupportBound rows j1 A.tiltAdjoint L

def TiltColumnwiseSupportControlled [One F] [DecidableEq F] [DecidableEq J1]
    {rows : List I} {j0 : List J0} {j1 : List J1}
    (A : GuardedTiltOperator (F := F) rows j0 j1) : Prop :=
  ColumnwiseSupportControlled rows j1 A.tiltAdjoint

def TiltFiniteColumnCombination [One F] [DecidableEq J1]
    {rows : List I} {j0 : List J0} {j1 : List J1}
    (A : GuardedTiltOperator (F := F) rows j0 j1) : Prop :=
  FiniteColumnCombination j1 A.tiltAdjoint

end GuardedTiltOperator

def ZeroSumWeightDominates [Zero F] [Add F] [DecidableEq F]
    (rows : List I) : Prop :=
  ∀ x y : Vector I F,
    addVec x y = zeroVec ->
    weightOn rows x <= weightOn rows y

theorem zeroSumWeightDominates_of_addGroup
    [AddGroup F] [DecidableEq F] (rows : List I) :
    ZeroSumWeightDominates (I := I) (F := F) rows := by
  intro x y hxy
  unfold weightOn supportOn
  have hpred :
      ∀ i ∈ rows, decide (x i ≠ 0) = decide (y i ≠ 0) := by
    intro i _hi
    have hcoord : x i + y i = 0 := by
      simpa [addVec, zeroVec] using congrFun hxy i
    have hiff : x i ≠ 0 ↔ y i ≠ 0 := by
      constructor
      · intro hx hy
        apply hx
        simpa [hy] using hcoord
      · intro hy hx
        apply hy
        simpa [hx] using hcoord
    by_cases hx : x i ≠ 0
    · have hy : y i ≠ 0 := hiff.mp hx
      simp [hx, hy]
    · have hy : ¬ y i ≠ 0 := by
        intro hy
        exact hx (hiff.mpr hy)
      simp [hx, hy]
  have hfilter :
      List.filter (fun i => decide (x i ≠ 0)) rows =
        List.filter (fun i => decide (y i ≠ 0)) rows :=
    List.filter_congr hpred
  rw [hfilter]

def NoKernelVectorBelow [Zero F] [Add F] [Mul F] [DecidableEq F]
    (_rows : List I) (j0 : List J0) (j1 : List J1)
    (B0 : Matrix I J0 F) (B1 : Matrix I J1 F)
    (H : Vector J1 F -> Vector I F) (d : Nat) : Prop :=
  ∀ r : Vector J0 F × Vector J1 F,
    guardedAT j0 j1 B0 B1 H r = zeroVec ->
    r ≠ zeroPair ->
    d <= pairWeightOn j0 j1 r

def guardedA [Add F]
    (A0 : Vector I F -> Vector J0 F)
    (A1 Tilt : Vector I F -> Vector J1 F)
    (s : Vector I F) : Vector J0 F × Vector J1 F :=
  (A0 s, addVec (A1 s) (Tilt s))

def PrimalDistanceAtLeast [Zero F] [Add F] [DecidableEq F]
    (_iRows : List I) (j0 : List J0) (j1 : List J1)
    (A0 : Vector I F -> Vector J0 F)
    (A1 Tilt : Vector I F -> Vector J1 F) (d : Nat) : Prop :=
  ∀ s : Vector I F,
    s ≠ zeroVec ->
    d <= pairWeightOn j0 j1 (guardedA A0 A1 Tilt s)

def NoKernelVectorBelowOp [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1) (d : Nat) : Prop :=
  ∀ r : Vector J0 F × Vector J1 F,
    A.adjoint r = zeroVec ->
    r ≠ zeroPair ->
    d <= pairWeightOn j0 j1 r

def PrimalDistanceAtLeastOp [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1) (d : Nat) : Prop :=
  ∀ s : Vector I F,
    s ≠ zeroVec ->
    d <= pairWeightOn j0 j1 (A.primal s)

private theorem supportColumnFlatten_length_le
    [Zero F] [One F] [DecidableEq F] [DecidableEq J]
    (rows : List I) (cols active : List J)
    (H : Vector J F -> Vector I F)
    (L : Nat)
    (hH : ColumnSupportBound rows cols H L)
    (hactive : ∀ j, j ∈ active -> j ∈ cols) :
    ((active.map (fun j => supportOn rows (H (single j)))).flatten).length <=
      L * active.length := by
  induction active with
  | nil => simp
  | cons j js ih =>
      have hj : j ∈ cols := hactive j (by simp)
      have hjs : ∀ k, k ∈ js -> k ∈ cols := by
        intro k hk
        exact hactive k (by simp [hk])
      have hcol : (supportOn rows (H (single j))).length <= L := hH j hj
      have htail :
          ((js.map (fun j => supportOn rows (H (single j)))).flatten).length <=
            L * js.length := ih hjs
      calc
        ((List.map (fun j => supportOn rows (H (single j))) (j :: js)).flatten).length
            = (supportOn rows (H (single j))).length +
                ((js.map (fun j => supportOn rows (H (single j)))).flatten).length := by
              simp
        _ <= L + L * js.length := Nat.add_le_add hcol htail
        _ = L * (j :: js).length := by
              simp [Nat.mul_succ, Nat.add_comm]

private def columnSupportBudget [Zero F] [One F] [DecidableEq F] [DecidableEq J]
    (rows : List I) (active : List J) (H : Vector J F -> Vector I F) :
    Nat :=
  (active.map (fun j => weightOn rows (H (single j)))).sum

private theorem columnSupportBudget_cons
    [Zero F] [One F] [DecidableEq F] [DecidableEq J]
    (i : I) (rows : List I) (active : List J)
    (H : Vector J F -> Vector I F) :
    columnSupportBudget (i :: rows) active H =
      (active.filter (fun j => decide (H (single j) i ≠ 0))).length +
        columnSupportBudget rows active H := by
  induction active with
  | nil =>
      simp [columnSupportBudget]
  | cons j js ih =>
      have ih' :
          (List.map (fun j => weightOn (i :: rows) (H (single j))) js).sum =
            (List.filter (fun j => decide (H (single j) i ≠ 0)) js).length +
              (List.map (fun j => weightOn rows (H (single j))) js).sum := by
        simpa [columnSupportBudget] using ih
      have ih'' :
          (List.map
              (fun j =>
                (List.filter (fun i => !decide (H (single j) i = 0))
                  (i :: rows)).length) js).sum =
            (List.filter (fun j => !decide (H (single j) i = 0)) js).length +
              (List.map
                (fun j =>
                  (List.filter (fun i => !decide (H (single j) i = 0))
                    rows).length) js).sum := by
        simpa [weightOn, supportOn] using ih'
      by_cases hj : H (single j) i ≠ 0
      · simp [columnSupportBudget, weightOn, supportOn, hj]
        rw [ih'']
        omega
      · simp [columnSupportBudget, weightOn, supportOn, hj]
        rw [ih'']
        omega

private theorem columnSupportBudget_eq_flatten_length
    [Zero F] [One F] [DecidableEq F] [DecidableEq J]
    (rows : List I) (active : List J)
    (H : Vector J F -> Vector I F) :
    columnSupportBudget rows active H =
      ((active.map (fun j => supportOn rows (H (single j)))).flatten).length := by
  induction active with
  | nil =>
      simp [columnSupportBudget]
  | cons j js ih =>
      simp [columnSupportBudget, weightOn, Function.comp_def]

private theorem weightOn_le_columnSupportBudget_of_pointwise
    [Zero F] [One F] [DecidableEq F] [DecidableEq J]
    (rows : List I) (active : List J)
    (H : Vector J F -> Vector I F) (r : Vector J F)
    (hpoint :
      ∀ i, i ∈ rows -> H r i ≠ 0 ->
        ∃ j, j ∈ active ∧ H (single j) i ≠ 0) :
    weightOn rows (H r) <= columnSupportBudget rows active H := by
  induction rows with
  | nil =>
      simp [weightOn, supportOn, columnSupportBudget]
  | cons i rows ih =>
      have htail :
          ∀ k, k ∈ rows -> H r k ≠ 0 ->
            ∃ j, j ∈ active ∧ H (single j) k ≠ 0 := by
        intro k hk hkH
        exact hpoint k (by simp [hk]) hkH
      have ihTail :
          weightOn rows (H r) <= columnSupportBudget rows active H := ih htail
      by_cases hi : H r i ≠ 0
      · rcases hpoint i (by simp) hi with ⟨j, hjActive, hjH⟩
        have hmem :
            j ∈ active.filter (fun j => decide (H (single j) i ≠ 0)) := by
          exact List.mem_filter.mpr ⟨hjActive, by simpa using hjH⟩
        have hpos :
            0 < (active.filter (fun j => decide (H (single j) i ≠ 0))).length :=
          List.length_pos_iff_ne_nil.mpr (List.ne_nil_of_mem hmem)
        have hbudget := columnSupportBudget_cons i rows active H
        have hle :
            weightOn rows (H r) + 1 <=
              columnSupportBudget (i :: rows) active H := by
          rw [hbudget]
          omega
        simpa [weightOn, supportOn, hi, Nat.add_comm] using hle
      · have hbudget := columnSupportBudget_cons i rows active H
        have hle :
            weightOn rows (H r) <=
              columnSupportBudget (i :: rows) active H := by
          rw [hbudget]
          omega
        simpa [weightOn, supportOn, hi] using hle

private theorem foldl_add_eq_zero_of_forall_zero
    [AddMonoid F] (xs : List α) (f : α -> F)
    (hzero : ∀ x, x ∈ xs -> f x = 0) :
    xs.foldl (fun acc x => acc + f x) 0 = 0 := by
  induction xs with
  | nil =>
      simp
  | cons x xs ih =>
      have hx : f x = 0 := hzero x (by simp)
      have hxs : ∀ y, y ∈ xs -> f y = 0 := by
        intro y hy
        exact hzero y (by simp [hy])
      simpa [hx] using ih hxs

private theorem exists_mem_of_foldl_add_ne_zero
    [AddMonoid F] {xs : List α} {f : α -> F}
    (h : xs.foldl (fun acc x => acc + f x) 0 ≠ 0) :
    ∃ x, x ∈ xs ∧ f x ≠ 0 := by
  by_contra hnone
  have hzero : ∀ x, x ∈ xs -> f x = 0 := by
    intro x hx
    by_contra hxne
    exact hnone ⟨x, hx, hxne⟩
  exact h (foldl_add_eq_zero_of_forall_zero xs f hzero)

private theorem finiteColumnCombination_pointwise_support
    [Field F] [DecidableEq F] [DecidableEq J]
    (cols : List J) (H : Vector J F -> Vector I F)
    (hCombo : FiniteColumnCombination cols H)
    (r : Vector J F) :
    ∀ i, H r i ≠ 0 ->
      ∃ j, j ∈ supportOn cols r ∧ H (single j) i ≠ 0 := by
  intro i hi
  have hcoord : H r i = finiteColumnCombination cols H r i :=
    congrFun (hCombo r) i
  have hfold :
      cols.foldl (fun acc j => acc + r j * H (single j) i) 0 ≠ 0 := by
    intro hzero
    exact hi (by simpa [finiteColumnCombination, hzero] using hcoord)
  rcases exists_mem_of_foldl_add_ne_zero
      (xs := cols) (f := fun j => r j * H (single j) i) hfold with
    ⟨j, hjCols, hjTerm⟩
  have hrj : r j ≠ 0 := by
    intro hrj
    exact hjTerm (by simp [hrj])
  have hjH : H (single j) i ≠ 0 := by
    intro hjH
    exact hjTerm (by simp [hjH])
  exact ⟨j, List.mem_filter.mpr ⟨hjCols, by simpa using hrj⟩, hjH⟩

private theorem weightOn_sumIndices_pairToSum [Zero F] [DecidableEq F]
    (j0 : List J0) (j1 : List J1)
    (r : Vector J0 F × Vector J1 F) :
    weightOn (sumIndices j0 j1) (pairToSum r) =
      pairWeightOn j0 j1 r := by
  unfold weightOn supportOn sumIndices pairWeightOn pairToSum
  rw [List.filter_append]
  simp only [List.length_append]
  rw [List.filter_map, List.filter_map]
  simp [Function.comp_def, weightOn, supportOn]

private theorem pairToSum_ne_zeroVec [Zero F]
    {r : Vector J0 F × Vector J1 F} (hr : r ≠ zeroPair) :
    pairToSum r ≠ zeroVec := by
  intro h
  apply hr
  cases r with
  | mk r0 r1 =>
      simp [zeroPair]
      constructor
      · funext j
        exact congrFun h (Sum.inl j)
      · funext j
        exact congrFun h (Sum.inr j)

theorem tilt_weight_bound_of_column_support
    [Zero F] [One F] [Add F] [DecidableEq F] [DecidableEq J1]
    (rows : List I) (cols : List J1) (H : Vector J1 F -> Vector I F)
    (L : Nat)
    (hH : ColumnSupportBound rows cols H L)
    (hLocal : ColumnwiseSupportControlled rows cols H)
    (r : Vector J1 F) :
    weightOn rows (H r) <= L * weightOn cols r := by
  have hactive : ∀ j, j ∈ supportOn cols r -> j ∈ cols := by
    intro j hj
    exact (List.mem_filter.mp hj).1
  calc
    weightOn rows (H r)
        <= ((supportOn cols r).map
              (fun j => supportOn rows (H (single j)))).flatten.length :=
          (hLocal r).length_le
    _ <= L * (supportOn cols r).length :=
          supportColumnFlatten_length_le rows cols (supportOn cols r) H L hH hactive
    _ = L * weightOn cols r := by
          rfl

theorem tilt_weight_bound_of_finite_column_combination
    [Field F] [DecidableEq F] [DecidableEq J1]
    (rows : List I) (cols : List J1) (H : Vector J1 F -> Vector I F)
    (L : Nat)
    (hH : ColumnSupportBound rows cols H L)
    (hCombo : FiniteColumnCombination cols H)
    (r : Vector J1 F) :
    weightOn rows (H r) <= L * weightOn cols r := by
  let active := supportOn cols r
  have hactive : ∀ j, j ∈ active -> j ∈ cols := by
    intro j hj
    exact (List.mem_filter.mp hj).1
  have hpoint :
      ∀ i, i ∈ rows -> H r i ≠ 0 ->
        ∃ j, j ∈ active ∧ H (single j) i ≠ 0 := by
    intro i _hi hi
    exact finiteColumnCombination_pointwise_support cols H hCombo r i hi
  calc
    weightOn rows (H r)
        <= columnSupportBudget rows active H :=
          weightOn_le_columnSupportBudget_of_pointwise rows active H r hpoint
    _ = ((active.map (fun j => supportOn rows (H (single j)))).flatten).length :=
          columnSupportBudget_eq_flatten_length rows active H
    _ <= L * active.length :=
          supportColumnFlatten_length_le rows cols active H L hH hactive
    _ = L * weightOn cols r := by
          rfl

theorem guarded_tilt_no_low_weight_dual
    [Field F] [DecidableEq F] [DecidableEq J1]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (B0 : Matrix I J0 F) (B1 : Matrix I J1 F)
    (H : Vector J1 F -> Vector I F)
    (n L : Nat)
    (hCauchy :
      CauchySupportLowerBound rows (sumIndices j0 j1) (combinedMatrix B0 B1))
    (hH : ColumnSupportBound rows j1 H L)
    (hCombo : FiniteColumnCombination j1 H)
    (hRows : n <= rows.length)
    {r : Vector J0 F × Vector J1 F}
    (hAT : guardedAT j0 j1 B0 B1 H r = zeroVec)
    (hr : r ≠ zeroPair)
    (hSmall : pairWeightOn j0 j1 r <= n) :
    n + 1 <= (L + 1) * pairWeightOn j0 j1 r := by
  let w := pairWeightOn j0 j1 r
  have hwRows : w <= rows.length := Nat.le_trans hSmall hRows
  have hsumWeight :
      weightOn (sumIndices j0 j1) (pairToSum r) = w := by
    exact weightOn_sumIndices_pairToSum j0 j1 r
  have hsumSmall :
      weightOn (sumIndices j0 j1) (pairToSum r) <= rows.length := by
    rw [hsumWeight]
    exact hwRows
  have hsumNonzero : pairToSum r ≠ zeroVec := pairToSum_ne_zeroVec hr
  have hC :
      rows.length - w + 1 <=
        weightOn rows (guardedBase j0 j1 B0 B1 r) := by
    simpa [guardedBase, hsumWeight] using
      hCauchy (pairToSum r) hsumSmall hsumNonzero
  have hBaseTilt :
      weightOn rows (guardedBase j0 j1 B0 B1 r) <=
        weightOn rows (H r.2) := by
    exact (zeroSumWeightDominates_of_addGroup (I := I) (F := F) rows)
      (guardedBase j0 j1 B0 B1 r) (H r.2)
      (by simpa [guardedAT] using hAT)
  have hTilt :
      weightOn rows (H r.2) <= L * weightOn j1 r.2 :=
    tilt_weight_bound_of_finite_column_combination rows j1 H L
      hH hCombo r.2
  have hRight :
      L * weightOn j1 r.2 <= L * w := by
    exact Nat.mul_le_mul_left L (by
      unfold w pairWeightOn
      exact Nat.le_add_left _ _)
  have hCtoL : rows.length - w + 1 <= L * w :=
    Nat.le_trans hC (Nat.le_trans hBaseTilt (Nat.le_trans hTilt hRight))
  have hRowsPlus : rows.length + 1 <= L * w + w := by
    omega
  have hnRows : n + 1 <= rows.length + 1 := Nat.succ_le_succ hRows
  have hn : n + 1 <= L * w + w := Nat.le_trans hnRows hRowsPlus
  simpa [Nat.succ_mul] using hn

theorem guarded_tilt_dual_distance
    [Field F] [DecidableEq F] [DecidableEq J1]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (B0 : Matrix I J0 F) (B1 : Matrix I J1 F)
    (H : Vector J1 F -> Vector I F)
    (n L : Nat)
    (hCauchy :
      CauchySupportLowerBound rows (sumIndices j0 j1) (combinedMatrix B0 B1))
    (hH : ColumnSupportBound rows j1 H L) :
    FiniteColumnCombination j1 H ->
    n <= rows.length ->
    ∀ r : Vector J0 F × Vector J1 F,
      guardedAT j0 j1 B0 B1 H r = zeroVec ->
      r ≠ zeroPair ->
      pairWeightOn j0 j1 r <= n ->
      pairWeightOn j0 j1 r < (n + 1) / (L + 1) ->
      False := by
  intro hCombo hRows r hAT hr hSmall hBelow
  have hNoLow :
      n + 1 <= (L + 1) * pairWeightOn j0 j1 r :=
    guarded_tilt_no_low_weight_dual rows j0 j1 B0 B1 H n L
      hCauchy hH hCombo hRows hAT hr hSmall
  have hMulLtSub :
      pairWeightOn j0 j1 r * (L + 1) < n + 1 - ((L + 1) - 1) :=
    (Nat.lt_div_iff_mul_lt (Nat.succ_pos L)).mp hBelow
  have hMulLt : pairWeightOn j0 j1 r * (L + 1) < n + 1 :=
    Nat.lt_of_lt_of_le hMulLtSub (Nat.sub_le _ _)
  have hMulLt' : (L + 1) * pairWeightOn j0 j1 r < n + 1 := by
    simpa [Nat.mul_comm] using hMulLt
  exact Nat.not_lt_of_ge hNoLow hMulLt'

theorem guarded_tilt_no_low_weight_dual_operator
    [Field F] [DecidableEq F] [DecidableEq J1]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1)
    (n L : Nat)
    (hCauchy :
      CauchySupportLowerBound rows (sumIndices j0 j1) A.baseMatrix)
    (hH : A.TiltColumnSupportBound L)
    (hCombo : A.TiltFiniteColumnCombination)
    (hRows : n <= rows.length)
    {r : Vector J0 F × Vector J1 F}
    (hAT : A.adjoint r = zeroVec)
    (hr : r ≠ zeroPair)
    (hSmall : pairWeightOn j0 j1 r <= n) :
    n + 1 <= (L + 1) * pairWeightOn j0 j1 r := by
  exact guarded_tilt_no_low_weight_dual rows j0 j1 A.B0 A.B1 A.tiltAdjoint
    n L hCauchy hH hCombo hRows hAT hr hSmall

theorem guarded_tilt_dual_distance_operator
    [Field F] [DecidableEq F] [DecidableEq J1]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1)
    (n L : Nat)
    (hCauchy :
      CauchySupportLowerBound rows (sumIndices j0 j1) A.baseMatrix)
    (hH : A.TiltColumnSupportBound L) :
    A.TiltFiniteColumnCombination ->
    n <= rows.length ->
    ∀ r : Vector J0 F × Vector J1 F,
      A.adjoint r = zeroVec ->
      r ≠ zeroPair ->
      pairWeightOn j0 j1 r <= n ->
      pairWeightOn j0 j1 r < (n + 1) / (L + 1) ->
      False := by
  intro hCombo hRows r hAT hr hSmall hBelow
  exact guarded_tilt_dual_distance rows j0 j1 A.B0 A.B1 A.tiltAdjoint
    n L hCauchy hH hCombo hRows r hAT hr hSmall hBelow

theorem guarded_tilt_primal_distance
    [Zero F] [Add F] [DecidableEq F]
    (iRows : List I) (j0 : List J0) (j1 : List J1)
    (A0 : Vector I F -> Vector J0 F)
    (A1 Tilt : Vector I F -> Vector J1 F)
    (mu : Nat)
    (hB0 : ∀ s : Vector I F, s ≠ zeroVec -> mu + 1 <= weightOn j0 (A0 s)) :
    PrimalDistanceAtLeast iRows j0 j1 A0 A1 Tilt (mu + 1) := by
  intro s hs
  exact Nat.le_trans (hB0 s hs) (Nat.le_add_right _ _)

theorem guarded_tilt_primal_distance_operator
    [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1)
    (mu : Nat)
    (hB0 :
      ∀ s : Vector I F,
        s ≠ zeroVec ->
        mu + 1 <= weightOn j0 (transposeMatVec rows A.B0 s)) :
    PrimalDistanceAtLeastOp rows j0 j1 A (mu + 1) := by
  intro s hs
  exact Nat.le_trans (hB0 s hs) (Nat.le_add_right _ _)

/--
Combined-matrix primal-distance adapter.

Unlike `guarded_tilt_primal_distance_operator`, this theorem does not require
the guard block alone to carry the primal distance.  It consumes the exact
distance certificate for the full public primal output
`(B0ᵀ s, B1ᵀ s + tilt s)`.
-/
theorem guarded_tilt_primal_distance_combined_operator
    [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1)
    (d : Nat)
    (hCombined :
      ∀ s : Vector I F,
        s ≠ zeroVec ->
        d <= pairWeightOn j0 j1 (A.primal s)) :
    PrimalDistanceAtLeastOp rows j0 j1 A d := by
  intro s hs
  exact hCombined s hs

structure GuardedTransversality
    [Zero F] [Add F] [Mul F] [DecidableEq F]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1)
    (dualCutoff primalCutoff : Nat) : Prop where
  dual : NoKernelVectorBelowOp rows j0 j1 A dualCutoff
  primal : PrimalDistanceAtLeastOp rows j0 j1 A primalCutoff

theorem guarded_tilt_deterministic_transversality
    [Field F] [DecidableEq F] [DecidableEq J1]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1)
    (n L mu : Nat)
    (hCauchy :
      CauchySupportLowerBound rows (sumIndices j0 j1) A.baseMatrix)
    (hH : A.TiltColumnSupportBound L)
    (hCombo : A.TiltFiniteColumnCombination)
    (hRows : n <= rows.length)
    (hB0 :
      ∀ s : Vector I F,
        s ≠ zeroVec ->
        mu + 1 <= weightOn j0 (transposeMatVec rows A.B0 s)) :
    GuardedTransversality rows j0 j1 A ((n + 1) / (L + 1)) (mu + 1) := by
  refine ⟨?dual, guarded_tilt_primal_distance_operator rows j0 j1 A mu hB0⟩
  intro r hAT hr
  by_contra hnot
  have hBelow : pairWeightOn j0 j1 r < (n + 1) / (L + 1) :=
    Nat.lt_of_not_ge hnot
  have hSmall : pairWeightOn j0 j1 r <= n := by
    exact Nat.le_of_lt_succ
      (Nat.lt_of_lt_of_le hBelow (Nat.div_le_self (n + 1) (L + 1)))
  exact guarded_tilt_dual_distance_operator rows j0 j1 A n L
    hCauchy hH hCombo hRows r hAT hr hSmall hBelow

/--
Deterministic transversality using a combined primal-distance certificate.

This is the proof hook for parameter sets with `m0 < n`: the dual theorem still
uses the Cauchy support bound and sparse tilt control, but primal correctness is
supplied by a certificate for the full combined public output instead of by the
guard block alone.
-/
theorem guarded_tilt_deterministic_transversality_combined
    [Field F] [DecidableEq F] [DecidableEq J1]
    (rows : List I) (j0 : List J0) (j1 : List J1)
    (A : GuardedTiltOperator (F := F) rows j0 j1)
    (n L primalCutoff : Nat)
    (hCauchy :
      CauchySupportLowerBound rows (sumIndices j0 j1) A.baseMatrix)
    (hH : A.TiltColumnSupportBound L)
    (hCombo : A.TiltFiniteColumnCombination)
    (hRows : n <= rows.length)
    (hPrimal :
      ∀ s : Vector I F,
        s ≠ zeroVec ->
        primalCutoff <= pairWeightOn j0 j1 (A.primal s)) :
    GuardedTransversality rows j0 j1 A ((n + 1) / (L + 1)) primalCutoff := by
  refine ⟨?dual, guarded_tilt_primal_distance_combined_operator rows j0 j1 A
    primalCutoff hPrimal⟩
  intro r hAT hr
  by_contra hnot
  have hBelow : pairWeightOn j0 j1 r < (n + 1) / (L + 1) :=
    Nat.lt_of_not_ge hnot
  have hSmall : pairWeightOn j0 j1 r <= n := by
    exact Nat.le_of_lt_succ
      (Nat.lt_of_lt_of_le hBelow (Nat.div_le_self (n + 1) (L + 1)))
  exact guarded_tilt_dual_distance_operator rows j0 j1 A n L
    hCauchy hH hCombo hRows r hAT hr hSmall hBelow

structure GuardedCoordinateFrame (I J0 J1 : Type*) where
  rows : List I
  j0 : List J0
  j1 : List J1
  hRowsCover : ∀ i : I, i ∈ rows
  hJ0Cover : ∀ j : J0, j ∈ j0
  hJ1Cover : ∀ j : J1, j ∈ j1
  hRowsNodup : rows.Nodup
  hJ0Nodup : j0.Nodup
  hJ1Nodup : j1.Nodup

structure GuardedDeterministicExportBundle
    [Field F] [DecidableEq F] [DecidableEq J1]
    (frame : GuardedCoordinateFrame I J0 J1) where
  A : GuardedTiltOperator (F := F) frame.rows frame.j0 frame.j1
  dualWindow : Nat
  tiltColumnBudget : Nat
  primalCutoff : Nat
  hDualWindowRows : dualWindow <= frame.rows.length
  hGuardSupport :
    CauchySupportLowerBound frame.rows (sumIndices frame.j0 frame.j1) A.baseMatrix
  hGuardPrimal :
    ∀ s : Vector I F,
      s ≠ zeroVec ->
      primalCutoff <= weightOn frame.j0 (transposeMatVec frame.rows A.B0 s)
  hTiltColumnSupport : A.TiltColumnSupportBound tiltColumnBudget
  hTiltFiniteColumnCombination : A.TiltFiniteColumnCombination

namespace GuardedDeterministicExportBundle

theorem dual_integral
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedDeterministicExportBundle (F := F) frame)
    {r : Vector J0 F × Vector J1 F}
    (hAT : B.A.adjoint r = zeroVec)
    (hr : r ≠ zeroPair)
    (hSmall : pairWeightOn frame.j0 frame.j1 r <= B.dualWindow) :
    B.dualWindow + 1 <=
      (B.tiltColumnBudget + 1) * pairWeightOn frame.j0 frame.j1 r := by
  exact guarded_tilt_no_low_weight_dual_operator frame.rows frame.j0 frame.j1
    B.A B.dualWindow B.tiltColumnBudget
    B.hGuardSupport B.hTiltColumnSupport B.hTiltFiniteColumnCombination
    B.hDualWindowRows hAT hr hSmall

theorem primal_distance
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedDeterministicExportBundle (F := F) frame) :
    PrimalDistanceAtLeastOp frame.rows frame.j0 frame.j1 B.A B.primalCutoff := by
  intro s hs
  exact Nat.le_trans (B.hGuardPrimal s hs) (Nat.le_add_right _ _)

end GuardedDeterministicExportBundle

structure GuardedCombinedDeterministicExportBundle
    [Field F] [DecidableEq F] [DecidableEq J1]
    (frame : GuardedCoordinateFrame I J0 J1) where
  A : GuardedTiltOperator (F := F) frame.rows frame.j0 frame.j1
  dualWindow : Nat
  tiltColumnBudget : Nat
  primalCutoff : Nat
  hDualWindowRows : dualWindow <= frame.rows.length
  hGuardSupport :
    CauchySupportLowerBound frame.rows (sumIndices frame.j0 frame.j1) A.baseMatrix
  hCombinedPrimal :
    ∀ s : Vector I F,
      s ≠ zeroVec ->
      primalCutoff <= pairWeightOn frame.j0 frame.j1 (A.primal s)
  hTiltColumnSupport : A.TiltColumnSupportBound tiltColumnBudget
  hTiltFiniteColumnCombination : A.TiltFiniteColumnCombination

namespace GuardedCombinedDeterministicExportBundle

theorem dual_integral
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedCombinedDeterministicExportBundle (F := F) frame)
    {r : Vector J0 F × Vector J1 F}
    (hAT : B.A.adjoint r = zeroVec)
    (hr : r ≠ zeroPair)
    (hSmall : pairWeightOn frame.j0 frame.j1 r <= B.dualWindow) :
    B.dualWindow + 1 <=
      (B.tiltColumnBudget + 1) * pairWeightOn frame.j0 frame.j1 r := by
  exact guarded_tilt_no_low_weight_dual_operator frame.rows frame.j0 frame.j1
    B.A B.dualWindow B.tiltColumnBudget
    B.hGuardSupport B.hTiltColumnSupport B.hTiltFiniteColumnCombination
    B.hDualWindowRows hAT hr hSmall

theorem primal_distance
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedCombinedDeterministicExportBundle (F := F) frame) :
    PrimalDistanceAtLeastOp frame.rows frame.j0 frame.j1 B.A B.primalCutoff := by
  exact guarded_tilt_primal_distance_combined_operator frame.rows frame.j0
    frame.j1 B.A B.primalCutoff B.hCombinedPrimal

theorem deterministic_transversality
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedCombinedDeterministicExportBundle (F := F) frame) :
    GuardedTransversality frame.rows frame.j0 frame.j1 B.A
      ((B.dualWindow + 1) / (B.tiltColumnBudget + 1)) B.primalCutoff := by
  refine ⟨?dual, B.primal_distance⟩
  intro r hAT hr
  by_contra hnot
  have hBelow :
      pairWeightOn frame.j0 frame.j1 r <
        (B.dualWindow + 1) / (B.tiltColumnBudget + 1) :=
    Nat.lt_of_not_ge hnot
  have hSmall : pairWeightOn frame.j0 frame.j1 r <= B.dualWindow := by
    exact Nat.le_of_lt_succ
      (Nat.lt_of_lt_of_le hBelow
        (Nat.div_le_self (B.dualWindow + 1) (B.tiltColumnBudget + 1)))
  exact guarded_tilt_dual_distance_operator frame.rows frame.j0 frame.j1
    B.A B.dualWindow B.tiltColumnBudget B.hGuardSupport
    B.hTiltColumnSupport B.hTiltFiniteColumnCombination
    B.hDualWindowRows r hAT hr hSmall hBelow

end GuardedCombinedDeterministicExportBundle

end Descender
