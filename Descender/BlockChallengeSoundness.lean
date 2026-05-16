import Mathlib.Tactic
import Descender.SignatureDistribution

namespace Descender

/-!
Block-coverage and extraction boundary for product-block signatures.

The concrete sampler analysis shows that sparse product-block challenges can be
small.  Their soundness obligation is different from full-block challenges:
across the Fiat-Shamir repetitions, every block that matters must be hit by an
extracting challenge.  This file records the deterministic extraction theorem
behind that accounting.
-/

namespace BlockChallengeSoundness

open SignatureDistribution

/-- A sequence of challenges covers every block when each block is active at least once. -/
def CoversBlocks {Round Block : Type*} (active : Round -> Block -> Prop) : Prop :=
  ∀ b, ∃ r, active r b

/-- A sequence covers the blocks selected by a predicate. -/
def CoversBlocksOn {Round Block : Type*}
    (active : Round -> Block -> Prop) (needed : Block -> Prop) : Prop :=
  ∀ b, needed b -> ∃ r, active r b

/-- Coverage failure: at least one needed block is never active. -/
def MissesNeededBlock {Round Block : Type*}
    (active : Round -> Block -> Prop) (needed : Block -> Prop) : Prop :=
  ∃ b, needed b ∧ ∀ r, ¬ active r b

theorem not_missesNeededBlock_of_coversBlocksOn
    {Round Block : Type*}
    {active : Round -> Block -> Prop} {needed : Block -> Prop}
    (hcover : CoversBlocksOn active needed) :
    ¬ MissesNeededBlock active needed := by
  rintro ⟨b, hb, hmiss⟩
  rcases hcover b hb with ⟨r, hr⟩
  exact hmiss r hr

/--
An abstract local extractor.  Whenever a round is active on a block, it can
extract a locally valid witness for that block.
-/
structure LocalBlockExtractor
    (Round Block : Type*) (LocalWitness : Block -> Type*)
    (Valid : ∀ b, LocalWitness b -> Prop)
    (active : Round -> Block -> Prop) where
  extract : ∀ r b, active r b -> LocalWitness b
  sound : ∀ r b h, Valid b (extract r b h)

namespace LocalBlockExtractor

variable {Round Block : Type*}
variable {LocalWitness : Block -> Type*}
variable {Valid : ∀ b, LocalWitness b -> Prop}
variable {active : Round -> Block -> Prop}

noncomputable def globalWitness
    (E : LocalBlockExtractor Round Block LocalWitness Valid active)
    (hcover : CoversBlocks active) (b : Block) : LocalWitness b :=
  E.extract (Classical.choose (hcover b)) b
    (Classical.choose_spec (hcover b))

theorem globalWitness_valid
    (E : LocalBlockExtractor Round Block LocalWitness Valid active)
    (hcover : CoversBlocks active) (b : Block) :
    Valid b (E.globalWitness hcover b) := by
  unfold globalWitness
  exact E.sound (Classical.choose (hcover b)) b
    (Classical.choose_spec (hcover b))

end LocalBlockExtractor

/-- A product-block challenge is active on a block when its scalar is nonzero. -/
def ChallengeActive {q : Nat} [NeZero q] {Block : Type*}
    (chi : Block -> ZMod q) (b : Block) : Prop :=
  chi b ≠ 0

/--
If a signed-or-zero challenge is active, its scalar is one of the two signed
units.  This is the algebraic reason an active block is locally extractable
without a large scalar inverse.
-/
theorem signedUnitOrZero_eq_one_or_neg_one_of_active
    {q : Nat} [NeZero q] {a : ZMod q}
    (ha : SignedUnitOrZero a) (hactive : a ≠ 0) :
    a = 1 ∨ a = -1 := by
  rcases ha with hzero | hone | hneg
  · exact False.elim (hactive hzero)
  · exact Or.inl hone
  · exact Or.inr hneg

/--
For two responses using the same mask, subtracting cancels the mask and leaves
the block-local challenge delta times the secret.
-/
theorem productBlockLinearResponse_sub
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    (chi₀ chi₁ : Block -> ZMod q)
    (mask secret : Block × LocalFree -> ZMod q)
    (bf : Block × LocalFree) :
    productBlockLinearResponse chi₀ mask secret bf -
        productBlockLinearResponse chi₁ mask secret bf =
      (chi₀ bf.1 - chi₁ bf.1) * secret bf := by
  simp [productBlockLinearResponse, blockScaleFree]
  ring

/-- The challenge delta at a block is extractable if it has a left inverse. -/
def DeltaExtractable {q : Nat} [NeZero q] {Block : Type*}
    (chi₀ chi₁ : Block -> ZMod q) (b : Block) : Prop :=
  ∃ inv : ZMod q, inv * (chi₀ b - chi₁ b) = 1

theorem extract_coordinate_of_deltaExtractable
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    {chi₀ chi₁ : Block -> ZMod q}
    {mask secret : Block × LocalFree -> ZMod q}
    {bf : Block × LocalFree}
    {inv : ZMod q}
    (hinv : inv * (chi₀ bf.1 - chi₁ bf.1) = 1) :
    inv *
        (productBlockLinearResponse chi₀ mask secret bf -
          productBlockLinearResponse chi₁ mask secret bf) =
      secret bf := by
  rw [productBlockLinearResponse_sub]
  calc
    inv * ((chi₀ bf.1 - chi₁ bf.1) * secret bf) =
        (inv * (chi₀ bf.1 - chi₁ bf.1)) * secret bf := by
          ring
    _ = secret bf := by
          simp [hinv]

end BlockChallengeSoundness

end Descender
