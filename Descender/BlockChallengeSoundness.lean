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

/--
For compact active-only transcripts, a block is exposed by both forked
responses only when both challenges are active.  Opposite signs give delta
`±2`, so extraction only needs an inverse for `2`.
-/
def OppositeSignedActive {q : Nat} [NeZero q] {Block : Type*}
    (chi₀ chi₁ : Block -> ZMod q) (b : Block) : Prop :=
  (chi₀ b = 1 ∧ chi₁ b = -1) ∨ (chi₀ b = -1 ∧ chi₁ b = 1)

theorem deltaExtractable_of_oppositeSignedActive
    {q : Nat} [NeZero q] {Block : Type*}
    {chi₀ chi₁ : Block -> ZMod q} {b : Block}
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hopp : OppositeSignedActive chi₀ chi₁ b) :
    DeltaExtractable chi₀ chi₁ b := by
  rcases hopp with ⟨h₀, h₁⟩ | ⟨h₀, h₁⟩
  · refine ⟨invTwo, ?_⟩
    rw [h₀, h₁]
    have hdelta : (1 : ZMod q) - (-1) = 2 := by
      ring
    rw [hdelta]
    exact hinvTwo
  · refine ⟨-invTwo, ?_⟩
    rw [h₀, h₁]
    have hdelta : (-1 : ZMod q) - 1 = -2 := by
      ring
    rw [hdelta]
    have h : (-invTwo) * (-(2 : ZMod q)) = invTwo * (2 : ZMod q) := by
      ring
    rw [h]
    exact hinvTwo

/-- Concrete inverse of `2` for the parameter-search modulus `q = 12289`. -/
theorem invTwo_zmod12289 :
    (6145 : ZMod 12289) * (2 : ZMod 12289) = 1 := by
  native_decide

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

/--
A forked linear round consists of two accepting responses for the same mask and
two block challenges.  The `hz₀` and `hz₁` fields are the special-soundness
idealization supplied by a future Fiat-Shamir forking game.
-/
structure ForkedLinearRound
    {q : Nat} [NeZero q] (Block LocalFree : Type*)
    (secret : Block × LocalFree -> ZMod q) where
  chi₀ : Block -> ZMod q
  chi₁ : Block -> ZMod q
  mask : Block × LocalFree -> ZMod q
  z₀ : Block × LocalFree -> ZMod q
  z₁ : Block × LocalFree -> ZMod q
  hz₀ : z₀ = productBlockLinearResponse chi₀ mask secret
  hz₁ : z₁ = productBlockLinearResponse chi₁ mask secret

namespace ForkedLinearRound

variable {q : Nat} [NeZero q]
variable {Block LocalFree : Type*}
variable {secret : Block × LocalFree -> ZMod q}

def DeltaActive (R : ForkedLinearRound Block LocalFree secret)
    (b : Block) : Prop :=
  DeltaExtractable R.chi₀ R.chi₁ b

noncomputable def extractCoordinate
    (R : ForkedLinearRound Block LocalFree secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) : ZMod q :=
  Classical.choose hdelta * (R.z₀ (b, f) - R.z₁ (b, f))

theorem extractCoordinate_eq_secret
    (R : ForkedLinearRound Block LocalFree secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    R.extractCoordinate b hdelta f = secret (b, f) := by
  unfold extractCoordinate
  rw [R.hz₀, R.hz₁]
  exact extract_coordinate_of_deltaExtractable
    (q := q) (bf := (b, f)) (Classical.choose_spec hdelta)

/--
If forked response pairs have extractable challenge deltas covering every
block, they recover the whole product-block secret.
-/
noncomputable def extractedSecret
    {Round : Type*}
    (round : Round -> ForkedLinearRound Block LocalFree secret)
    (hcover : CoversBlocks fun r b => (round r).DeltaActive b) :
    Block × LocalFree -> ZMod q :=
  fun bf =>
    let r := Classical.choose (hcover bf.1)
    (round r).extractCoordinate bf.1
      (Classical.choose_spec (hcover bf.1)) bf.2

theorem extractedSecret_eq
    {Round : Type*}
    (round : Round -> ForkedLinearRound Block LocalFree secret)
    (hcover : CoversBlocks fun r b => (round r).DeltaActive b) :
    extractedSecret round hcover = secret := by
  funext bf
  unfold extractedSecret
  exact (round (Classical.choose (hcover bf.1))).extractCoordinate_eq_secret
    bf.1 (Classical.choose_spec (hcover bf.1)) bf.2

end ForkedLinearRound

/--
A multi-fork linear round packages several accepting responses for one fixed
commitment/mask.  The Fiat-Shamir forking game is responsible for producing
the draws; this deterministic layer only uses the accepting transcript
equations.
-/
structure MultiForkLinearRound
    {q : Nat} [NeZero q] (Draw Block LocalFree : Type*)
    (secret : Block × LocalFree -> ZMod q) where
  chi : Draw -> Block -> ZMod q
  mask : Block × LocalFree -> ZMod q
  z : Draw -> Block × LocalFree -> ZMod q
  hz : ∀ d, z d = productBlockLinearResponse (chi d) mask secret

namespace MultiForkLinearRound

variable {q : Nat} [NeZero q]
variable {Draw Block LocalFree : Type*}
variable {secret : Block × LocalFree -> ZMod q}

def PairDeltaActive (R : MultiForkLinearRound Draw Block LocalFree secret)
    (d₀ d₁ : Draw) (b : Block) : Prop :=
  DeltaExtractable (R.chi d₀) (R.chi d₁) b

def MultiDeltaActive (R : MultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) : Prop :=
  ∃ d₀ d₁, R.PairDeltaActive d₀ d₁ b

def CompactOppositeActive
    (R : MultiForkLinearRound Draw Block LocalFree secret) (b : Block) :
    Prop :=
  ∃ d₀ d₁, OppositeSignedActive (R.chi d₀) (R.chi d₁) b

noncomputable def deltaLeft
    (R : MultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) (hdelta : R.MultiDeltaActive b) : Draw :=
  Classical.choose hdelta

noncomputable def deltaRight
    (R : MultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) (hdelta : R.MultiDeltaActive b) : Draw :=
  Classical.choose (Classical.choose_spec hdelta)

theorem deltaWitness_spec
    (R : MultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) (hdelta : R.MultiDeltaActive b) :
    R.PairDeltaActive (R.deltaLeft b hdelta)
      (R.deltaRight b hdelta) b := by
  unfold deltaLeft deltaRight
  exact Classical.choose_spec (Classical.choose_spec hdelta)

theorem multiDeltaActive_of_compactOppositeActive
    (R : MultiForkLinearRound Draw Block LocalFree secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    {b : Block}
    (hcompact : R.CompactOppositeActive b) :
    R.MultiDeltaActive b := by
  rcases hcompact with ⟨d₀, d₁, hopp⟩
  exact ⟨d₀, d₁,
    deltaExtractable_of_oppositeSignedActive hinvTwo hopp⟩

noncomputable def forkedPair
    (R : MultiForkLinearRound Draw Block LocalFree secret)
    (d₀ d₁ : Draw) : ForkedLinearRound Block LocalFree secret where
  chi₀ := R.chi d₀
  chi₁ := R.chi d₁
  mask := R.mask
  z₀ := R.z d₀
  z₁ := R.z d₁
  hz₀ := R.hz d₀
  hz₁ := R.hz d₁

noncomputable def extractCoordinate
    (R : MultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) (hdelta : R.MultiDeltaActive b) (f : LocalFree) :
    ZMod q :=
  ForkedLinearRound.extractCoordinate
    (R.forkedPair (R.deltaLeft b hdelta) (R.deltaRight b hdelta))
    b (R.deltaWitness_spec b hdelta) f

theorem extractCoordinate_eq_secret
    (R : MultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) (hdelta : R.MultiDeltaActive b) (f : LocalFree) :
    R.extractCoordinate b hdelta f = secret (b, f) := by
  unfold extractCoordinate
  exact ForkedLinearRound.extractCoordinate_eq_secret
    (R.forkedPair (R.deltaLeft b hdelta) (R.deltaRight b hdelta))
    b (R.deltaWitness_spec b hdelta) f

theorem coversMultiDelta_of_coversCompactOpposite
    {Round : Type*}
    (round : Round -> MultiForkLinearRound Draw Block LocalFree secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover : CoversBlocks fun r b => (round r).CompactOppositeActive b) :
    CoversBlocks fun r b => (round r).MultiDeltaActive b := by
  intro b
  rcases hcover b with ⟨r, hr⟩
  exact ⟨r, (round r).multiDeltaActive_of_compactOppositeActive
    hinvTwo hr⟩

/--
If the multi-fork draws give an extractable challenge delta for every block
across the repeated commitments, the extractor recovers the whole secret.
-/
noncomputable def extractedSecret
    {Round : Type*}
    (round : Round -> MultiForkLinearRound Draw Block LocalFree secret)
    (hcover : CoversBlocks fun r b => (round r).MultiDeltaActive b) :
    Block × LocalFree -> ZMod q :=
  fun bf =>
    let r := Classical.choose (hcover bf.1)
    (round r).extractCoordinate bf.1
      (Classical.choose_spec (hcover bf.1)) bf.2

theorem extractedSecret_eq
    {Round : Type*}
    (round : Round -> MultiForkLinearRound Draw Block LocalFree secret)
    (hcover : CoversBlocks fun r b => (round r).MultiDeltaActive b) :
    extractedSecret round hcover = secret := by
  funext bf
  unfold extractedSecret
  exact (round (Classical.choose (hcover bf.1))).extractCoordinate_eq_secret
    bf.1 (Classical.choose_spec (hcover bf.1)) bf.2

/--
Compact active-only multi-fork extraction: if every block has two accepting
draws with opposite signed active challenges in some repeated commitment, then
the extracted product-block secret is the real secret.
-/
theorem extractedSecret_eq_of_coversCompactOpposite
    {Round : Type*}
    (round : Round -> MultiForkLinearRound Draw Block LocalFree secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover : CoversBlocks fun r b => (round r).CompactOppositeActive b) :
    extractedSecret round
      (coversMultiDelta_of_coversCompactOpposite round hinvTwo hcover) =
        secret :=
  extractedSecret_eq round
    (coversMultiDelta_of_coversCompactOpposite round hinvTwo hcover)

end MultiForkLinearRound

end BlockChallengeSoundness

end Descender
