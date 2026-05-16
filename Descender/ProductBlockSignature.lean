import Descender.SystematicLee

namespace Descender

/-!
Product-block signature proof boundary.

The previous product-block theorem shows that block-local challenge scaling
commutes with a block-diagonal systematic relation.  This file adds the
corresponding correctness/noise side: if challenge coefficients are signed
units or zero, then applying the challenge to a noisy public view does not
increase the centered coordinate noise bound.
-/

variable {q : Nat} [NeZero q]

/-- Signature challenges use only `0`, `1`, and `-1` as block scalars. -/
def SignedUnitOrZero (a : ZMod q) : Prop :=
  a = 0 ∨ a = 1 ∨ a = -1

/-- A block challenge is valid when every block scalar is `0`, `1`, or `-1`. -/
def BlockSignedUnitOrZero {Block : Type*} (chi : Block -> ZMod q) : Prop :=
  ∀ b, SignedUnitOrZero (chi b)

omit [NeZero q] in
theorem centeredAbs_mul_signedUnitOrZero_le
    {a x : ZMod q} (ha : SignedUnitOrZero a) :
    centeredAbs q (a * x) <= centeredAbs q x := by
  rcases ha with hzero | hone | hneg
  · subst hzero
    simp
  · subst hone
    simp
  · subst hneg
    simp

omit [NeZero q] in
theorem centeredResidueNoiseSupport_blockScaleFree
    {Block LocalFree : Type*} [Fintype Block] [Fintype LocalFree]
    {chi : Block -> ZMod q} {e : Block × LocalFree -> ZMod q}
    {eta : Nat}
    (hchi : BlockSignedUnitOrZero chi)
    (he : centeredResidueNoiseSupport q eta e) :
    centeredResidueNoiseSupport q eta (blockScaleFree chi e) := by
  intro bf
  exact Nat.le_trans
    (centeredAbs_mul_signedUnitOrZero_le (q := q) (hchi bf.1))
    (he bf)

omit [NeZero q] in
theorem centeredResidueNoiseSupport_blockScalePivot
    {Block LocalPivot : Type*} [Fintype Block] [Fintype LocalPivot]
    {chi : Block -> ZMod q} {e : Block × LocalPivot -> ZMod q}
    {eta : Nat}
    (hchi : BlockSignedUnitOrZero chi)
    (he : centeredResidueNoiseSupport q eta e) :
    centeredResidueNoiseSupport q eta (blockScalePivot chi e) := by
  intro bp
  exact Nat.le_trans
    (centeredAbs_mul_signedUnitOrZero_le (q := q) (hchi bp.1))
    (he bp)

omit [NeZero q] in
theorem centeredResidueNoiseSupport_blockScaleView
    {Block LocalPivot LocalFree : Type*}
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    {chi : Block -> ZMod q}
    {e : Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q}
    {eta : Nat}
    (hchi : BlockSignedUnitOrZero chi)
    (he : centeredResidueNoiseSupport q eta e) :
    centeredResidueNoiseSupport q eta (blockScaleView chi e) := by
  intro x
  cases x with
  | inl bp =>
      exact Nat.le_trans
        (centeredAbs_mul_signedUnitOrZero_le (q := q) (hchi bp.1))
        (he (Sum.inl bp))
  | inr bf =>
      exact Nat.le_trans
        (centeredAbs_mul_signedUnitOrZero_le (q := q) (hchi bf.1))
        (he (Sum.inr bf))

/--
Applying a block challenge to a noisy systematic public view preserves the
same systematic shape: the relation part is challenged algebraically, and the
error part is simply block-scaled.
-/
theorem blockScaleView_systematicRelation_add_error
    {Block LocalPivot LocalFree : Type*} [DecidableEq Block]
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (y : Block × LocalFree -> ZMod q)
    (e : Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q) :
    blockScaleView chi
        (fun x =>
          systematicRelation q (blockDiagonalSystematicMatrix Plocal) y x +
            e x) =
      fun x =>
        systematicRelation q (blockDiagonalSystematicMatrix Plocal)
            (blockScaleFree chi y) x +
          blockScaleView chi e x := by
  have hrel :=
    blockScaleView_systematicRelation
      (q := q) Plocal chi y
  funext x
  calc
    blockScaleView chi
        (fun x =>
          systematicRelation q (blockDiagonalSystematicMatrix Plocal) y x +
            e x) x =
        blockScaleView chi
            (systematicRelation q (blockDiagonalSystematicMatrix Plocal) y) x +
          blockScaleView chi e x := by
          cases x <;> simp [blockScaleView, mul_add]
    _ =
        systematicRelation q (blockDiagonalSystematicMatrix Plocal)
            (blockScaleFree chi y) x +
          blockScaleView chi e x := by
          rw [congrFun hrel x]

/--
Verifier-side consequence for sparse signed product-block challenges: a noisy
public relation with coordinate error bounded by `eta` remains a noisy public
relation with the same error bound after challenge application.
-/
theorem blockScaleView_noisySystematicRelation_preserves_error
    {Block LocalPivot LocalFree : Type*} [DecidableEq Block]
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    {chi : Block -> ZMod q}
    (hchi : BlockSignedUnitOrZero chi)
    (y : Block × LocalFree -> ZMod q)
    {eta : Nat}
    {e : Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q}
    (he : centeredResidueNoiseSupport q eta e) :
    ∃ e' : Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q,
      centeredResidueNoiseSupport q eta e' ∧
        blockScaleView chi
            (fun x =>
              systematicRelation q (blockDiagonalSystematicMatrix Plocal) y x +
                e x) =
          fun x =>
            systematicRelation q (blockDiagonalSystematicMatrix Plocal)
                (blockScaleFree chi y) x +
              e' x := by
  refine ⟨blockScaleView chi e, ?_, ?_⟩
  · exact centeredResidueNoiseSupport_blockScaleView
      (q := q) hchi he
  · exact blockScaleView_systematicRelation_add_error
      (q := q) Plocal chi y e

/--
Serialized/high-bit public views are modeled proof-side as true noise plus
rounding error.  Sparse signed block challenges preserve the combined
`eta + roundingEta` coordinate bound.
-/
theorem blockScaleView_noisySerializedSystematicRelation_preserves_error
    {Block LocalPivot LocalFree : Type*} [DecidableEq Block]
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    {chi : Block -> ZMod q}
    (hchi : BlockSignedUnitOrZero chi)
    (y : Block × LocalFree -> ZMod q)
    {eta roundingEta : Nat}
    {e rounding :
      Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q}
    (he : centeredResidueNoiseSupport q eta e)
    (hrounding : centeredResidueNoiseSupport q roundingEta rounding) :
    ∃ e' : Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q,
      centeredResidueNoiseSupport q (eta + roundingEta) e' ∧
        blockScaleView chi
            (fun x =>
              systematicRelation q (blockDiagonalSystematicMatrix Plocal) y x +
                (e x + rounding x)) =
          fun x =>
            systematicRelation q (blockDiagonalSystematicMatrix Plocal)
                (blockScaleFree chi y) x +
              e' x := by
  let combined :
      Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q :=
    fun x => e x + rounding x
  have hcombined : centeredResidueNoiseSupport q (eta + roundingEta) combined :=
    centeredResidueNoiseSupport_add he hrounding
  exact blockScaleView_noisySystematicRelation_preserves_error
    (q := q) Plocal hchi y hcombined

end Descender
