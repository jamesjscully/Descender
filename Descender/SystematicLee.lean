import Mathlib.Data.Fintype.Sum
import Descender.QLattice

namespace Descender

/-!
Systematic q-ary Lee-distance certificates.

If the public adjoint matrix is written with an invertible pivot block, every
kernel vector is determined by its free coordinates `y` and has the systematic
form `(-P y, y)`.  This file records the algebraic proof boundary needed by the
parameter search: a lower bound for the Lee norm of all nonzero systematic
relations implies the `ModqNoLeeKernelVectorBelow` promise consumed by the
q-ary amplitude theorem.
-/

variable {Pivot Free : Type*}

def systematicPivotImage [Fintype Free] (q : Nat) [NeZero q]
    (P : Pivot -> Free -> ZMod q) (y : Free -> ZMod q) : Pivot -> ZMod q :=
  fun p => ∑ f, P p f * y f

def systematicRelation [Fintype Free] (q : Nat) [NeZero q]
    (P : Pivot -> Free -> ZMod q) (y : Free -> ZMod q) :
    Sum Pivot Free -> ZMod q
  | Sum.inl p => -systematicPivotImage q P y p
  | Sum.inr f => y f

def systematicDualMap [Fintype Free] (q : Nat) [NeZero q]
    (P : Pivot -> Free -> ZMod q) (r : Sum Pivot Free -> ZMod q) :
    Pivot -> ZMod q :=
  fun p => r (Sum.inl p) + ∑ f, P p f * r (Sum.inr f)

theorem systematicDualMap_relation_eq_zero [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q)
    (y : Free -> ZMod q) :
    systematicDualMap q P (systematicRelation q P y) = 0 := by
  funext p
  simp [systematicDualMap, systematicRelation, systematicPivotImage]

theorem systematicRelation_eq_zero_iff [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q)
    {y : Free -> ZMod q} :
    systematicRelation q P y = 0 ↔ y = 0 := by
  constructor
  · intro h
    funext f
    have hf := congrFun h (Sum.inr f)
    simpa [systematicRelation] using hf
  · intro h
    subst h
    funext x
    cases x <;> simp [systematicRelation, systematicPivotImage]

theorem systematicRelation_nonzero_iff [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q)
    {y : Free -> ZMod q} :
    systematicRelation q P y ≠ 0 ↔ y ≠ 0 := by
  simpa [not_iff_not] using systematicRelation_eq_zero_iff q P (y := y)

theorem systematicRelation_lee_norm [Fintype Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q)
    (y : Free -> ZMod q) :
    centeredNormL1 q (systematicRelation q P y) =
      centeredNormL1 q (systematicPivotImage q P y) +
        centeredNormL1 q y := by
  classical
  unfold centeredNormL1 systematicRelation
  rw [Fintype.sum_sum_type]
  simp [centeredAbs_neg]

theorem eq_systematicRelation_of_systematicDualMap_eq_zero [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q)
    {r : Sum Pivot Free -> ZMod q}
    (hr : systematicDualMap q P r = 0) :
    r = systematicRelation q P (fun f => r (Sum.inr f)) := by
  funext x
  cases x with
  | inl p =>
      have hp := congrFun hr p
      simp [systematicDualMap, systematicRelation, systematicPivotImage] at hp ⊢
      exact eq_neg_of_add_eq_zero_left hp
  | inr f =>
      simp [systematicRelation]

theorem modqNoLeeKernelVectorBelow_of_systematic_lower_bound
    [Fintype Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) {d : Nat}
    (hLower : ∀ y : Free -> ZMod q, y ≠ 0 ->
      d <= centeredNormL1 q (systematicRelation q P y)) :
    ModqNoLeeKernelVectorBelow q (systematicDualMap q P) d := by
  intro r hr hrne
  let y : Free -> ZMod q := fun f => r (Sum.inr f)
  have hrSys : r = systematicRelation q P y :=
    eq_systematicRelation_of_systematicDualMap_eq_zero q P hr
  have hyne : y ≠ 0 := by
    intro hy
    apply hrne
    rw [hrSys]
    exact (systematicRelation_eq_zero_iff q P).mpr hy
  have h := hLower y hyne
  rw [hrSys]
  exact h

def SystematicLeeDistanceAtLeast [Fintype Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) (d : Nat) : Prop :=
  ∀ y : Free -> ZMod q, y ≠ 0 ->
    d <= centeredNormL1 q (systematicRelation q P y)

def SystematicLeeExpansionAtLeast [Fintype Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) (pivotGain : Nat) : Prop :=
  ∀ y : Free -> ZMod q, y ≠ 0 ->
    pivotGain <= centeredNormL1 q (systematicPivotImage q P y)

structure SystematicLeeExpansionCertificate (q : Nat) [NeZero q]
    (Pivot Free : Type*) [Fintype Pivot] [Fintype Free]
    (P : Pivot -> Free -> ZMod q) where
  pivotGain : Nat
  pivotGainProof : SystematicLeeExpansionAtLeast q P pivotGain

def SystematicSupportExpansionAtLeast [Fintype Pivot] [DecidableEq Pivot]
    [Fintype Free] (q : Nat) [NeZero q]
    (P : Pivot -> Free -> ZMod q) (pivotGain : Nat) : Prop :=
  ∀ y : Free -> ZMod q, y ≠ 0 ->
    pivotGain <= weight (systematicPivotImage q P y)

theorem systematicLeeExpansionAtLeast_of_support_expansion
    [Fintype Pivot] [DecidableEq Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) {pivotGain : Nat}
    (hSupport : SystematicSupportExpansionAtLeast q P pivotGain) :
    SystematicLeeExpansionAtLeast q P pivotGain := by
  intro y hy
  exact Nat.le_trans (hSupport y hy)
    (weight_le_centeredNormL1 q (systematicPivotImage q P y))

def PivotCoordinateReads (P : Pivot -> Free -> ZMod q) (p : Pivot) (f : Free) :
    Prop :=
  P p f = 1 ∧ ∀ g : Free, g ≠ f -> P p g = 0

theorem systematicPivotImage_eq_of_coordinate_read
    [Fintype Free] [DecidableEq Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q)
    {p : Pivot} {f : Free} (hread : PivotCoordinateReads P p f)
    (y : Free -> ZMod q) :
    systematicPivotImage q P y p = y f := by
  classical
  unfold systematicPivotImage
  rw [Finset.sum_eq_single f]
  · simp [hread.left]
  · intro g _hg hgf
    simp [hread.right g hgf]
  · intro hf
    simp at hf

theorem systematicLeeExpansionAtLeast_of_coordinate_reads
    [Fintype Pivot] [DecidableEq Pivot] [Fintype Free] [DecidableEq Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q)
    (rows : Free -> Finset Pivot) {pivotGain : Nat}
    (hread : ∀ f p, p ∈ rows f -> PivotCoordinateReads P p f)
    (hcard : ∀ f, pivotGain <= (rows f).card) :
    SystematicLeeExpansionAtLeast q P pivotGain := by
  intro y hy
  classical
  have hExists : ∃ f, y f ≠ 0 := by
    by_contra h
    apply hy
    funext f
    by_contra hf
    exact h ⟨f, hf⟩
  rcases hExists with ⟨f, hyf⟩
  have hAbsPos : 1 <= centeredAbs q (y f) := by
    exact Nat.succ_le_of_lt (Nat.pos_of_ne_zero (by
      intro hzero
      exact hyf ((centeredAbs_eq_zero_iff q (y f)).mp hzero)))
  have hsum_eq :
      (∑ p ∈ rows f, centeredAbs q (systematicPivotImage q P y p)) =
        (rows f).card * centeredAbs q (y f) := by
    have hsum_congr :
        (∑ p ∈ rows f, centeredAbs q (systematicPivotImage q P y p)) =
          ∑ _p ∈ rows f, centeredAbs q (y f) := by
      apply Finset.sum_congr rfl
      intro p hp
      rw [systematicPivotImage_eq_of_coordinate_read q P (hread f p hp) y]
    rw [hsum_congr, Finset.sum_const, nsmul_eq_mul, Nat.cast_id]
  have hpartial_le_total :
      (∑ p ∈ rows f, centeredAbs q (systematicPivotImage q P y p)) <=
        centeredNormL1 q (systematicPivotImage q P y) := by
    unfold centeredNormL1
    exact Finset.sum_le_sum_of_subset_of_nonneg
      (by intro p _hp; exact Finset.mem_univ p)
      (by intro p _hp _hnot; exact Nat.zero_le _)
  calc
    pivotGain <= (rows f).card := hcard f
    _ <= (rows f).card * centeredAbs q (y f) :=
      Nat.le_mul_of_pos_right (rows f).card
        (Nat.lt_of_lt_of_le Nat.zero_lt_one hAbsPos)
    _ = ∑ p ∈ rows f, centeredAbs q (systematicPivotImage q P y p) :=
      hsum_eq.symm
    _ <= centeredNormL1 q (systematicPivotImage q P y) :=
      hpartial_le_total

structure SystematicLeeCertificate (q : Nat) [NeZero q]
    (Pivot Free : Type*) [Fintype Pivot] [Fintype Free]
    (P : Pivot -> Free -> ZMod q) where
  lowerBound : Nat
  lowerBoundProof : SystematicLeeDistanceAtLeast q P lowerBound

namespace SystematicLeeCertificate

def modqNoLeeKernelVectorBelow
    {q : Nat} [NeZero q] {Pivot Free : Type*}
    [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q}
    (C : SystematicLeeCertificate q Pivot Free P) :
    ModqNoLeeKernelVectorBelow q (systematicDualMap q P) C.lowerBound :=
  modqNoLeeKernelVectorBelow_of_systematic_lower_bound q P C.lowerBoundProof

end SystematicLeeCertificate

/--
Promote an exact bounded-free-vector check into a global systematic Lee
certificate.

The checker only needs to consider free vectors whose free Lee norm is below
`d`: if `centeredNormL1 q y >= d`, then the full systematic relation
`(-P y, y)` already has Lee norm at least `d` because it contains `y`.
-/
theorem systematicLeeDistanceAtLeast_of_checked_free_l1_below
    [Fintype Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) {d : Nat}
    (hChecked :
      ∀ y : Free -> ZMod q,
        y ≠ 0 ->
        centeredNormL1 q y < d ->
        d <= centeredNormL1 q (systematicRelation q P y)) :
    SystematicLeeDistanceAtLeast q P d := by
  intro y hy
  by_cases hSmall : centeredNormL1 q y < d
  · exact hChecked y hy hSmall
  · have hFree : d <= centeredNormL1 q y := Nat.le_of_not_gt hSmall
    rw [systematicRelation_lee_norm q P y]
    exact Nat.le_trans hFree
      (Nat.le_add_left
        (centeredNormL1 q y)
        (centeredNormL1 q (systematicPivotImage q P y)))

/--
Proof-carrying shape for exhaustive local Lee checks, matching the artifact
emitted by `shared_lee_cert`: the external checker enumerates every nonzero
free vector below `lowerBound` and proves the full systematic relation is still
at least `lowerBound`.
-/
structure BoundedFreeLeeCheckCertificate (q : Nat) [NeZero q]
    (Pivot Free : Type*) [Fintype Pivot] [Fintype Free]
    (P : Pivot -> Free -> ZMod q) where
  lowerBound : Nat
  checkedBelow :
    ∀ y : Free -> ZMod q,
      y ≠ 0 ->
      centeredNormL1 q y < lowerBound ->
      lowerBound <= centeredNormL1 q (systematicRelation q P y)

def BoundedFreeLeeCheckCertificate.toSystematicLeeCertificate
    {q : Nat} [NeZero q] {Pivot Free : Type*} [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q}
    (C : BoundedFreeLeeCheckCertificate q Pivot Free P) :
    SystematicLeeCertificate q Pivot Free P where
  lowerBound := C.lowerBound
  lowerBoundProof :=
    systematicLeeDistanceAtLeast_of_checked_free_l1_below q P C.checkedBelow

/--
Generic local-to-global Lee certificate for structured systematic matrices.

This is the proof boundary for efficient constructions that are assembled from
small certifiable pieces.  The local blocks prove Lee distance for every
nonzero local free vector.  The `freeSlice_nonzero` field says every nonzero
global free vector activates at least one local block.  The `localDominates`
field says the local systematic Lee norm of that activated block is bounded by
the full global systematic relation norm.

Together these fields turn local exact certificates into a global
`SystematicLeeDistanceAtLeast` proof, avoiding any bounded search over the full
`q ^ m1` global free space.
-/
structure ComposableSystematicLeeCertificate
    (q : Nat) [NeZero q]
    (Block LocalPivot LocalFree Pivot Free : Type*)
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    [Fintype Pivot] [Fintype Free]
    (Pglobal : Pivot -> Free -> ZMod q)
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (freeSlice : Block -> (Free -> ZMod q) -> LocalFree -> ZMod q) where
  lowerBound : Nat
  localBound : Block -> Nat
  localDistance :
    ∀ b, SystematicLeeDistanceAtLeast q (Plocal b) (localBound b)
  lowerBound_le_local : ∀ b, lowerBound <= localBound b
  freeSlice_nonzero :
    ∀ y : Free -> ZMod q, y ≠ 0 -> ∃ b, freeSlice b y ≠ 0
  localDominates :
    ∀ b (y : Free -> ZMod q),
      centeredNormL1 q
          (systematicRelation q (Plocal b) (freeSlice b y)) <=
        centeredNormL1 q (systematicRelation q Pglobal y)

namespace ComposableSystematicLeeCertificate

def toSystematicLeeCertificate
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree Pivot Free : Type*}
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    [Fintype Pivot] [Fintype Free]
    {Pglobal : Pivot -> Free -> ZMod q}
    {Plocal : Block -> LocalPivot -> LocalFree -> ZMod q}
    {freeSlice : Block -> (Free -> ZMod q) -> LocalFree -> ZMod q}
    (C : ComposableSystematicLeeCertificate q
      Block LocalPivot LocalFree Pivot Free Pglobal Plocal freeSlice) :
    SystematicLeeCertificate q Pivot Free Pglobal where
  lowerBound := C.lowerBound
  lowerBoundProof := by
    intro y hy
    rcases C.freeSlice_nonzero y hy with ⟨b, hb⟩
    exact Nat.le_trans (C.lowerBound_le_local b)
      (Nat.le_trans (C.localDistance b (freeSlice b y) hb)
        (C.localDominates b y))

def modqNoLeeKernelVectorBelow
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree Pivot Free : Type*}
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    [Fintype Pivot] [Fintype Free]
    {Pglobal : Pivot -> Free -> ZMod q}
    {Plocal : Block -> LocalPivot -> LocalFree -> ZMod q}
    {freeSlice : Block -> (Free -> ZMod q) -> LocalFree -> ZMod q}
    (C : ComposableSystematicLeeCertificate q
      Block LocalPivot LocalFree Pivot Free Pglobal Plocal freeSlice) :
    ModqNoLeeKernelVectorBelow q (systematicDualMap q Pglobal) C.lowerBound :=
  C.toSystematicLeeCertificate.modqNoLeeKernelVectorBelow

end ComposableSystematicLeeCertificate

def productFreeSlice {Block LocalFree : Type*}
    (b : Block) (y : Block × LocalFree -> ZMod q) : LocalFree -> ZMod q :=
  fun f => y (b, f)

/--
Block-diagonal systematic matrix assembled from local Lee-expander blocks.

Rows are indexed by `Block × LocalPivot`, free columns by
`Block × LocalFree`, and off-block entries are zero.  This is the canonical
global matrix shape for the `shared_lee_cert`/`product-block-lee` route.
-/
def blockDiagonalSystematicMatrix
    {Block LocalPivot LocalFree : Type*} [DecidableEq Block]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q) :
    Block × LocalPivot -> Block × LocalFree -> ZMod q :=
  fun bp bf => if bp.1 = bf.1 then Plocal bp.1 bp.2 bf.2 else 0

theorem blockDiagonalSystematicPivotImage_eq
    (q : Nat) [NeZero q]
    {Block LocalPivot LocalFree : Type*} [DecidableEq Block]
    [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (b : Block) (p : LocalPivot) (y : Block × LocalFree -> ZMod q) :
    systematicPivotImage q (blockDiagonalSystematicMatrix Plocal) y (b, p) =
      systematicPivotImage q (Plocal b) (productFreeSlice (q := q) b y) p := by
  classical
  unfold systematicPivotImage blockDiagonalSystematicMatrix productFreeSlice
  rw [Fintype.sum_prod_type]
  simp

/--
For a block-diagonal systematic matrix, every local systematic relation embeds
as a subset of the global systematic relation's coordinates.  This discharges
the `localDominates` side condition in the generic composable certificate.
-/
theorem blockDiagonalSystematicRelation_local_le_global
    (q : Nat) [NeZero q]
    {Block LocalPivot LocalFree : Type*} [DecidableEq Block]
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (b : Block) (y : Block × LocalFree -> ZMod q) :
    centeredNormL1 q
        (systematicRelation q (Plocal b) (productFreeSlice (q := q) b y)) <=
      centeredNormL1 q
        (systematicRelation q (blockDiagonalSystematicMatrix Plocal) y) := by
  classical
  rw [systematicRelation_lee_norm q (Plocal b) (productFreeSlice (q := q) b y)]
  rw [systematicRelation_lee_norm q (blockDiagonalSystematicMatrix Plocal) y]
  apply Nat.add_le_add
  · unfold centeredNormL1
    calc
      ∑ p, centeredAbs q
            (systematicPivotImage q (Plocal b)
              (productFreeSlice (q := q) b y) p)
          =
        ∑ p, centeredAbs q
            (systematicPivotImage q (blockDiagonalSystematicMatrix Plocal)
              y (b, p)) := by
            apply Finset.sum_congr rfl
            intro p _hp
            rw [blockDiagonalSystematicPivotImage_eq q]
      _ <= ∑ bp : Block × LocalPivot,
            centeredAbs q
              (systematicPivotImage q (blockDiagonalSystematicMatrix Plocal)
                y bp) := by
            rw [Fintype.sum_prod_type]
            let F : Block -> Nat := fun b' =>
              ∑ p, centeredAbs q
                (systematicPivotImage q (blockDiagonalSystematicMatrix Plocal)
                  y (b', p))
            change F b <= ∑ x, F x
            exact Finset.single_le_sum (fun _ _ => Nat.zero_le _)
              (Finset.mem_univ b)
  · unfold centeredNormL1 productFreeSlice
    calc
      ∑ f, centeredAbs q (y (b, f)) <=
        ∑ bf : Block × LocalFree, centeredAbs q (y bf) := by
          rw [Fintype.sum_prod_type]
          let F : Block -> Nat := fun b' =>
            ∑ f, centeredAbs q (y (b', f))
          change F b <= ∑ x, F x
          exact Finset.single_le_sum (fun _ _ => Nat.zero_le _)
            (Finset.mem_univ b)

theorem exists_nonzero_productFreeSlice
    {Block LocalFree : Type*}
    (y : Block × LocalFree -> ZMod q) (hy : y ≠ 0) :
    ∃ b, productFreeSlice (q := q) b y ≠ 0 := by
  by_contra h
  apply hy
  funext bf
  rcases bf with ⟨b, f⟩
  have hb : productFreeSlice (q := q) b y = 0 := by
    by_contra hb
    exact h ⟨b, hb⟩
  exact congrFun hb f

/--
Product-block specialization of the composable certificate.

Use this for direct sums of local systematic blocks, with global free variables
indexed by `Block × LocalFree`.  The only construction-specific proof left is
`hDominates`: the selected local block's systematic Lee norm is bounded by the
global relation norm.  For genuinely block-diagonal matrices this is the
disjoint-support fact.
-/
def productBlockComposableSystematicLeeCertificate
    (q : Nat) [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    (Pglobal : Block × LocalPivot -> Block × LocalFree -> ZMod q)
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    {lowerBound : Nat}
    (localBound : Block -> Nat)
    (hLocal : ∀ b, SystematicLeeDistanceAtLeast q (Plocal b) (localBound b))
    (hLower : ∀ b, lowerBound <= localBound b)
    (hDominates :
      ∀ b (y : Block × LocalFree -> ZMod q),
        centeredNormL1 q
            (systematicRelation q (Plocal b)
              (productFreeSlice (q := q) b y)) <=
          centeredNormL1 q (systematicRelation q Pglobal y)) :
    ComposableSystematicLeeCertificate q
      Block LocalPivot LocalFree (Block × LocalPivot) (Block × LocalFree)
      Pglobal Plocal (productFreeSlice (q := q)) where
  lowerBound := lowerBound
  localBound := localBound
  localDistance := hLocal
  lowerBound_le_local := hLower
  freeSlice_nonzero := exists_nonzero_productFreeSlice (q := q)
  localDominates := hDominates

/--
Block-diagonal product-block certificate with the domination proof discharged
by `blockDiagonalSystematicRelation_local_le_global`.
-/
def blockDiagonalProductComposableSystematicLeeCertificate
    (q : Nat) [NeZero q]
    {Block LocalPivot LocalFree : Type*} [DecidableEq Block]
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    {lowerBound : Nat}
    (localBound : Block -> Nat)
    (hLocal : ∀ b, SystematicLeeDistanceAtLeast q (Plocal b) (localBound b))
    (hLower : ∀ b, lowerBound <= localBound b) :
    ComposableSystematicLeeCertificate q
      Block LocalPivot LocalFree (Block × LocalPivot) (Block × LocalFree)
      (blockDiagonalSystematicMatrix Plocal) Plocal
      (productFreeSlice (q := q)) :=
  productBlockComposableSystematicLeeCertificate q
    (blockDiagonalSystematicMatrix Plocal) Plocal
    localBound hLocal hLower
    (blockDiagonalSystematicRelation_local_le_global (q := q) Plocal)

/--
Use exact local bounded-free-vector checks as the local certificates for a
block-diagonal global systematic matrix.
-/
def blockDiagonalProductComposableCertificateOfCheckedLocal
    (q : Nat) [NeZero q]
    {Block LocalPivot LocalFree : Type*} [DecidableEq Block]
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (Clocal :
      ∀ b, BoundedFreeLeeCheckCertificate q LocalPivot LocalFree (Plocal b))
    {lowerBound : Nat}
    (hLower : ∀ b, lowerBound <= (Clocal b).lowerBound) :
    ComposableSystematicLeeCertificate q
      Block LocalPivot LocalFree (Block × LocalPivot) (Block × LocalFree)
      (blockDiagonalSystematicMatrix Plocal) Plocal
      (productFreeSlice (q := q)) :=
  blockDiagonalProductComposableSystematicLeeCertificate q Plocal
    (fun b => (Clocal b).lowerBound)
    (fun b => (Clocal b).toSystematicLeeCertificate.lowerBoundProof)
    hLower

theorem systematicRelation_lee_lower_bound_of_pivot_expansion
    [Fintype Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) {pivotGain d : Nat}
    (hExpansion : SystematicLeeExpansionAtLeast q P pivotGain)
    (hBound : d <= pivotGain) :
    SystematicLeeDistanceAtLeast q P d := by
  intro y hy
  have hPivot : pivotGain <= centeredNormL1 q (systematicPivotImage q P y) :=
    hExpansion y hy
  rw [systematicRelation_lee_norm q P y]
  exact Nat.le_trans hBound (Nat.le_trans hPivot (Nat.le_add_right _ _))

theorem systematicRelation_lee_lower_bound_of_pivot_expansion_with_free
    [Fintype Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) {pivotGain d : Nat}
    (hExpansion : SystematicLeeExpansionAtLeast q P pivotGain)
    (hBound : d <= pivotGain + 1) :
    SystematicLeeDistanceAtLeast q P d := by
  intro y hy
  have hPivot : pivotGain <= centeredNormL1 q (systematicPivotImage q P y) :=
    hExpansion y hy
  have hFree : 1 <= centeredNormL1 q y := by
    exact Nat.succ_le_of_lt (Nat.pos_of_ne_zero (by
      intro hzero
      exact hy ((centeredNormL1_eq_zero_iff q y).mp hzero)))
  rw [systematicRelation_lee_norm q P y]
  exact Nat.le_trans hBound (Nat.add_le_add hPivot hFree)

theorem modqNoLeeKernelVectorBelow_of_systematic_pivot_expansion
    [Fintype Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) {pivotGain d : Nat}
    (hExpansion : SystematicLeeExpansionAtLeast q P pivotGain)
    (hBound : d <= pivotGain + 1) :
    ModqNoLeeKernelVectorBelow q (systematicDualMap q P) d :=
  modqNoLeeKernelVectorBelow_of_systematic_lower_bound q P
    (systematicRelation_lee_lower_bound_of_pivot_expansion_with_free
      q P hExpansion hBound)

def SystematicLeeExpansionCertificate.toDistanceCertificate
    {q : Nat} [NeZero q] {Pivot Free : Type*} [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q}
    (C : SystematicLeeExpansionCertificate q Pivot Free P) :
    SystematicLeeCertificate q Pivot Free P where
  lowerBound := C.pivotGain + 1
  lowerBoundProof :=
    systematicRelation_lee_lower_bound_of_pivot_expansion_with_free
      q P C.pivotGainProof le_rfl

def SystematicLeeExpansionCertificate.modqNoLeeKernelVectorBelow
    {q : Nat} [NeZero q] {Pivot Free : Type*} [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q}
    (C : SystematicLeeExpansionCertificate q Pivot Free P) :
    ModqNoLeeKernelVectorBelow q (systematicDualMap q P) (C.pivotGain + 1) :=
  modqNoLeeKernelVectorBelow_of_systematic_pivot_expansion q P
    C.pivotGainProof le_rfl

theorem modqNoLeeKernelVectorBelow_of_systematic_support_expansion
    [Fintype Pivot] [DecidableEq Pivot] [Fintype Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q) {pivotGain d : Nat}
    (hSupport : SystematicSupportExpansionAtLeast q P pivotGain)
    (hBound : d <= pivotGain + 1) :
    ModqNoLeeKernelVectorBelow q (systematicDualMap q P) d :=
  modqNoLeeKernelVectorBelow_of_systematic_pivot_expansion q P
    (systematicLeeExpansionAtLeast_of_support_expansion q P hSupport)
    hBound

theorem modqNoLeeKernelVectorBelow_of_coordinate_reads
    [Fintype Pivot] [DecidableEq Pivot] [Fintype Free] [DecidableEq Free]
    (q : Nat) [NeZero q] (P : Pivot -> Free -> ZMod q)
    (rows : Free -> Finset Pivot) {pivotGain d : Nat}
    (hread : ∀ f p, p ∈ rows f -> PivotCoordinateReads P p f)
    (hcard : ∀ f, pivotGain <= (rows f).card)
    (hBound : d <= pivotGain + 1) :
    ModqNoLeeKernelVectorBelow q (systematicDualMap q P) d :=
  modqNoLeeKernelVectorBelow_of_systematic_pivot_expansion q P
    (systematicLeeExpansionAtLeast_of_coordinate_reads q P rows hread hcard)
    hBound

def MatrixLeftInverseOn [Fintype Pivot] [DecidableEq Pivot]
    (B : Pivot -> Pivot -> ZMod q)
    (L : Pivot -> Pivot -> ZMod q) : Prop :=
  ∀ p p', ∑ i, L p i * B i p' = if p = p' then 1 else 0

def SolvedBlock [Fintype Pivot] (Brest : Pivot -> Free -> ZMod q)
    (L : Pivot -> Pivot -> ZMod q) : Pivot -> Free -> ZMod q :=
  fun p f => ∑ i, L p i * Brest i f

def BlockColumnDuplicates (Bpivot : Pivot -> Pivot -> ZMod q)
    (Brest : Pivot -> Free -> ZMod q) (select : Free -> Pivot) : Prop :=
  ∀ i f, Brest i f = Bpivot i (select f)

theorem solvedBlock_coordinate_read_of_duplicate_column
    [Fintype Pivot] [DecidableEq Pivot]
    (Bpivot : Pivot -> Pivot -> ZMod q)
    (Brest : Pivot -> Free -> ZMod q)
    (L : Pivot -> Pivot -> ZMod q)
    (select : Free -> Pivot)
    (hSelect : Function.Injective select)
    (hLeft : MatrixLeftInverseOn Bpivot L)
    (hDup : BlockColumnDuplicates Bpivot Brest select)
    (f : Free) :
    PivotCoordinateReads (SolvedBlock Brest L) (select f) f := by
  constructor
  · unfold SolvedBlock
    calc
      ∑ i, L (select f) i * Brest i f =
          ∑ i, L (select f) i * Bpivot i (select f) := by
            apply Finset.sum_congr rfl
            intro i _hi
            rw [hDup i f]
      _ = 1 := by
            simpa using hLeft (select f) (select f)
  · intro g hg
    unfold SolvedBlock
    have hne : select f ≠ select g := by
      intro hsel
      exact hg (hSelect hsel.symm)
    calc
      ∑ i, L (select f) i * Brest i g =
          ∑ i, L (select f) i * Bpivot i (select g) := by
            apply Finset.sum_congr rfl
            intro i _hi
            rw [hDup i g]
      _ = 0 := by
            simpa [hne] using hLeft (select f) (select g)

theorem modqNoLeeKernelVectorBelow_of_duplicate_solved_block
    [Fintype Pivot] [DecidableEq Pivot] [Fintype Free] [DecidableEq Free]
    (q : Nat) [NeZero q]
    (Bpivot : Pivot -> Pivot -> ZMod q)
    (Brest : Pivot -> Free -> ZMod q)
    (L : Pivot -> Pivot -> ZMod q)
    (select : Free -> Pivot)
    (rows : Free -> Finset Pivot) {pivotGain d : Nat}
    (hSelect : Function.Injective select)
    (hLeft : MatrixLeftInverseOn Bpivot L)
    (hDup : BlockColumnDuplicates Bpivot Brest select)
    (hrows : ∀ f p, p ∈ rows f -> p = select f)
    (hcard : ∀ f, pivotGain <= (rows f).card)
    (hBound : d <= pivotGain + 1) :
    ModqNoLeeKernelVectorBelow q (systematicDualMap q (SolvedBlock Brest L)) d := by
  refine modqNoLeeKernelVectorBelow_of_coordinate_reads q (SolvedBlock Brest L)
    rows ?_ hcard hBound
  intro f p hp
  rw [hrows f p hp]
  exact solvedBlock_coordinate_read_of_duplicate_column Bpivot Brest L select hSelect
    hLeft hDup f

end Descender
