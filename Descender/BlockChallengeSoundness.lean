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

/--
Fork subtraction when the two accepting openings use different masks.  The
extra term is exactly the same-state mask-collision term that a fibered
commitment proof must either rule out or convert into a separate witness.
-/
theorem productBlockLinearResponse_sub_with_mask_delta
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    (chi₀ chi₁ : Block -> ZMod q)
    (mask₀ mask₁ secret : Block × LocalFree -> ZMod q)
    (bf : Block × LocalFree) :
    productBlockLinearResponse chi₀ mask₀ secret bf -
        productBlockLinearResponse chi₁ mask₁ secret bf =
      (mask₀ bf - mask₁ bf) +
        (chi₀ bf.1 - chi₁ bf.1) * secret bf := by
  simp [productBlockLinearResponse, blockScaleFree]
  ring

/--
If the mask delta is known or proved zero on the extracted coordinate, the
usual fork extractor still recovers the secret.  Without that term, a fibered
commitment proof must treat the delta as a binding/collision witness.
-/
theorem extract_coordinate_of_deltaExtractable_with_mask_delta
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    {chi₀ chi₁ : Block -> ZMod q}
    {mask₀ mask₁ secret : Block × LocalFree -> ZMod q}
    {bf : Block × LocalFree}
    {inv : ZMod q}
    (hinv : inv * (chi₀ bf.1 - chi₁ bf.1) = 1) :
    inv *
        (productBlockLinearResponse chi₀ mask₀ secret bf -
          productBlockLinearResponse chi₁ mask₁ secret bf -
          (mask₀ bf - mask₁ bf)) =
      secret bf := by
  rw [productBlockLinearResponse_sub_with_mask_delta]
  calc
    inv *
        ((mask₀ bf - mask₁ bf) +
          (chi₀ bf.1 - chi₁ bf.1) * secret bf -
          (mask₀ bf - mask₁ bf)) =
        (inv * (chi₀ bf.1 - chi₁ bf.1)) * secret bf := by
          ring
    _ = secret bf := by
          simp [hinv]

/--
Any single affine response can be explained by any secret if the mask is free.

This is the one-view obstruction: the equation `z = mask + chi * secret` alone
does not identify the secret.  A linear one-view proof therefore needs an
additional binding/extraction mechanism for the mask, or a different
keygen-breaking witness.
-/
def maskExplainingResponse
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    (chi : Block -> ZMod q)
    (secret response : Block × LocalFree -> ZMod q) :
    Block × LocalFree -> ZMod q :=
  fun bf => response bf - blockScaleFree chi secret bf

theorem productBlockLinearResponse_maskExplainingResponse
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    (chi : Block -> ZMod q)
    (secret response : Block × LocalFree -> ZMod q) :
    productBlockLinearResponse chi
        (maskExplainingResponse chi secret response) secret =
  response := by
  funext bf
  simp [productBlockLinearResponse, maskExplainingResponse, blockScaleFree]

theorem productBlockLinearResponse_single_view_secret_ambiguous
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    (chi : Block -> ZMod q)
    (mask₀ secret₀ secret₁ : Block × LocalFree -> ZMod q) :
    ∃ mask₁,
      productBlockLinearResponse chi mask₁ secret₁ =
        productBlockLinearResponse chi mask₀ secret₀ := by
  refine ⟨maskExplainingResponse chi secret₁
    (productBlockLinearResponse chi mask₀ secret₀), ?_⟩
  exact productBlockLinearResponse_maskExplainingResponse chi secret₁
    (productBlockLinearResponse chi mask₀ secret₀)

/-- The challenge delta at a block is extractable if it has a left inverse. -/
def DeltaExtractable {q : Nat} [NeZero q] {Block : Type*}
    (chi₀ chi₁ : Block -> ZMod q) (b : Block) : Prop :=
  ∃ inv : ZMod q, inv * (chi₀ b - chi₁ b) = 1

/-- A single challenge exposes a block if its scalar has a left inverse. -/
def ScalarExtractable {q : Nat} [NeZero q] {Block : Type*}
    (chi : Block -> ZMod q) (b : Block) : Prop :=
  ∃ inv : ZMod q, inv * chi b = 1

theorem scalarExtractable_of_signedActive
    {q : Nat} [NeZero q] {Block : Type*}
    {chi : Block -> ZMod q} {b : Block}
    (hactive : chi b = 1 ∨ chi b = -1) :
    ScalarExtractable chi b := by
  rcases hactive with hone | hneg
  · refine ⟨1, ?_⟩
    simp [hone]
  · refine ⟨-1, ?_⟩
    simp [hneg]

theorem scalarExtractable_of_signedUnitOrZero_active
    {q : Nat} [NeZero q] {Block : Type*}
    {chi : Block -> ZMod q} {b : Block}
    (hsigned : SignedUnitOrZero (chi b))
    (hactive : chi b ≠ 0) :
    ScalarExtractable chi b := by
  rcases signedUnitOrZero_eq_one_or_neg_one_of_active hsigned hactive with
    hone | hneg
  · refine ⟨1, ?_⟩
    simp [hone]
  · refine ⟨-1, ?_⟩
    simp [hneg]

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

theorem extract_coordinate_of_scalarExtractable
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    {chi : Block -> ZMod q}
    {mask secret response : Block × LocalFree -> ZMod q}
    {bf : Block × LocalFree}
    {inv : ZMod q}
    (hinv : inv * chi bf.1 = 1)
    (hresponse : response = productBlockLinearResponse chi mask secret) :
    inv * (response bf - mask bf) = secret bf := by
  rw [hresponse]
  simp only [productBlockLinearResponse, blockScaleFree]
  calc
    inv * (mask bf + chi bf.1 * secret bf - mask bf) =
        (inv * chi bf.1) * secret bf := by
          ring
    _ = secret bf := by
          simp [hinv]

/--
A single linear response with a known mask.  This is the deterministic target
for a redesigned one-view proof: once the mask is available, active signed
challenge coordinates reveal the corresponding secret coordinates directly.
-/
structure SingleViewLinearRound
    {q : Nat} [NeZero q] (Block LocalFree : Type*)
    (secret : Block × LocalFree -> ZMod q) where
  chi : Block -> ZMod q
  mask : Block × LocalFree -> ZMod q
  z : Block × LocalFree -> ZMod q
  hz : z = productBlockLinearResponse chi mask secret

namespace SingleViewLinearRound

variable {q : Nat} [NeZero q]
variable {Block LocalFree : Type*}
variable {secret : Block × LocalFree -> ZMod q}

def Active (R : SingleViewLinearRound Block LocalFree secret)
    (b : Block) : Prop :=
  ScalarExtractable R.chi b

noncomputable def extractCoordinate
    (R : SingleViewLinearRound Block LocalFree secret)
    (b : Block) (hactive : R.Active b) (f : LocalFree) : ZMod q :=
  Classical.choose hactive * (R.z (b, f) - R.mask (b, f))

theorem extractCoordinate_eq_secret
    (R : SingleViewLinearRound Block LocalFree secret)
    (b : Block) (hactive : R.Active b) (f : LocalFree) :
    R.extractCoordinate b hactive f = secret (b, f) := by
  exact extract_coordinate_of_scalarExtractable
    (q := q) (bf := (b, f))
    (Classical.choose_spec hactive) R.hz

noncomputable def extractedSecret
    {Round : Type*}
    (R : Round -> SingleViewLinearRound Block LocalFree secret)
    (hcover : CoversBlocks fun r b => (R r).Active b) :
    Block × LocalFree -> ZMod q :=
  fun bf =>
    (R (Classical.choose (hcover bf.1))).extractCoordinate bf.1
      (Classical.choose_spec (hcover bf.1)) bf.2

theorem extractedSecret_eq
    {Round : Type*}
    (R : Round -> SingleViewLinearRound Block LocalFree secret)
    (hcover : CoversBlocks fun r b => (R r).Active b) :
    extractedSecret R hcover = secret := by
  funext bf
  exact (R (Classical.choose (hcover bf.1))).extractCoordinate_eq_secret
    bf.1 (Classical.choose_spec (hcover bf.1)) bf.2

end SingleViewLinearRound

/--
Compact active-only single-view round.

This is the one-view analogue of `CompactMultiForkLinearRound`: the response is
available only on signed active blocks.  If the mask is known or extractable,
one signed active challenge coordinate reveals the corresponding secret
coordinate.
-/
structure CompactSingleViewLinearRound
    {q : Nat} [NeZero q] (Block LocalFree : Type*)
    (secret : Block × LocalFree -> ZMod q) where
  chi : Block -> ZMod q
  mask : Block × LocalFree -> ZMod q
  z :
    ∀ b, (chi b = 1 ∨ chi b = -1) ->
      LocalFree -> ZMod q
  hz :
    ∀ b h f,
      z b h f = productBlockLinearResponse chi mask secret (b, f)

namespace CompactSingleViewLinearRound

variable {q : Nat} [NeZero q]
variable {Block LocalFree : Type*}
variable {secret : Block × LocalFree -> ZMod q}

def SignedActive
    (R : CompactSingleViewLinearRound Block LocalFree secret)
    (b : Block) : Prop :=
  R.chi b = 1 ∨ R.chi b = -1

noncomputable def extractCoordinate
    (R : CompactSingleViewLinearRound Block LocalFree secret)
    (b : Block) (hactive : R.SignedActive b) (f : LocalFree) :
    ZMod q :=
  Classical.choose (scalarExtractable_of_signedActive hactive) *
    (R.z b hactive f - R.mask (b, f))

theorem extractCoordinate_eq_secret
    (R : CompactSingleViewLinearRound Block LocalFree secret)
    (b : Block) (hactive : R.SignedActive b) (f : LocalFree) :
    R.extractCoordinate b hactive f = secret (b, f) := by
  unfold extractCoordinate
  rw [R.hz b hactive f]
  simp only [productBlockLinearResponse, blockScaleFree]
  calc
    Classical.choose (scalarExtractable_of_signedActive hactive) *
        (R.mask (b, f) + R.chi b * secret (b, f) - R.mask (b, f)) =
        (Classical.choose (scalarExtractable_of_signedActive hactive) *
          R.chi b) * secret (b, f) := by
          ring
    _ = secret (b, f) := by
          simp [Classical.choose_spec
            (scalarExtractable_of_signedActive hactive)]

noncomputable def extractedSecret
    {Round : Type*}
    (R : Round -> CompactSingleViewLinearRound Block LocalFree secret)
    (hcover : CoversBlocks fun r b => (R r).SignedActive b) :
    Block × LocalFree -> ZMod q :=
  fun bf =>
    (R (Classical.choose (hcover bf.1))).extractCoordinate bf.1
      (Classical.choose_spec (hcover bf.1)) bf.2

theorem extractedSecret_eq
    {Round : Type*}
    (R : Round -> CompactSingleViewLinearRound Block LocalFree secret)
    (hcover : CoversBlocks fun r b => (R r).SignedActive b) :
    extractedSecret R hcover = secret := by
  funext bf
  exact (R (Classical.choose (hcover bf.1))).extractCoordinate_eq_secret
    bf.1 (Classical.choose_spec (hcover bf.1)) bf.2

end CompactSingleViewLinearRound

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
A same-state mask collision witness for a fibered commitment representation.

`stateOfMask` is the map from the sampled signing mask to the reconstructed
commitment state.  `ValidMask` is the hook for the concrete proof's acceptance
and norm bounds.  A collision is useful only when the two masks are both valid,
different, and nevertheless map to the same commitment state.
-/
structure SameStateMaskCollision
    {q : Nat} [NeZero q] (Block LocalFree CommitState : Type*)
    (stateOfMask : (Block × LocalFree -> ZMod q) -> CommitState)
    (ValidMask : (Block × LocalFree -> ZMod q) -> Prop) where
  mask₀ : Block × LocalFree -> ZMod q
  mask₁ : Block × LocalFree -> ZMod q
  valid₀ : ValidMask mask₀
  valid₁ : ValidMask mask₁
  sameState : stateOfMask mask₀ = stateOfMask mask₁
  different : mask₀ ≠ mask₁

namespace SameStateMaskCollision

variable {q : Nat} [NeZero q]
variable {Block LocalFree CommitState : Type*}
variable {stateOfMask : (Block × LocalFree -> ZMod q) -> CommitState}
variable {ValidMask : (Block × LocalFree -> ZMod q) -> Prop}

/-- The q-ary kernel vector exposed by a same-state mask collision. -/
def maskDelta
    (C :
      SameStateMaskCollision Block LocalFree CommitState stateOfMask
        ValidMask) :
    Block × LocalFree -> ZMod q :=
  fun bf => C.mask₀ bf - C.mask₁ bf

theorem maskDelta_nonzero
    (C :
      SameStateMaskCollision Block LocalFree CommitState stateOfMask
        ValidMask) :
    C.maskDelta ≠ 0 := by
  intro hdelta
  apply C.different
  funext bf
  have hcoord := congrFun hdelta bf
  exact sub_eq_zero.mp hcoord

/--
Convert a same-state mask collision into a q-ary Lee-kernel violation once the
commitment-state map is subtractive and the mask delta is short.
-/
def toModqLeeKernelViolation_of_short
    [Fintype (Block × LocalFree)] [Zero CommitState]
    {leeMin : Nat}
    (C :
      SameStateMaskCollision Block LocalFree CommitState stateOfMask
        ValidMask)
    (hstate_sub :
      ∀ y₀ y₁,
        stateOfMask y₀ = stateOfMask y₁ ->
          stateOfMask (fun bf => y₀ bf - y₁ bf) = 0)
    (hshort : centeredNormL1 q C.maskDelta < leeMin) :
    ModqLeeKernelViolation q stateOfMask leeMin where
  r := C.maskDelta
  kernel := hstate_sub C.mask₀ C.mask₁ C.sameState
  nonzero := C.maskDelta_nonzero
  short := hshort

end SameStateMaskCollision

/--
Concrete validity predicate for accepted masks whose centered coordinates are
bounded by `eta`.
-/
def BoundedMaskValid
    {q : Nat} [NeZero q] {Coord : Type*} [Fintype Coord]
    (eta : Nat) (mask : Coord -> ZMod q) : Prop :=
  centeredResidueNoiseSupport q eta mask

/--
Same-state bounded-mask collisions are q-ary Lee-kernel violations.

This is the construction-specific bridge needed by the fibered 69-byte route:
instantiate `stateOfMask` with the concrete commitment-state map and prove
`hstate_sub`; instantiate `ValidMask` with `BoundedMaskValid`; then the
accepted-mask bound supplies the shortness of the collision witness.
-/
def SameStateMaskCollision.toModqLeeKernelViolation_of_boundedMasks
    {q : Nat} [NeZero q]
    {Block LocalFree CommitState : Type*}
    [Fintype (Block × LocalFree)] [Zero CommitState]
    {stateOfMask : (Block × LocalFree -> ZMod q) -> CommitState}
    {eta leeMin : Nat}
    (C :
      SameStateMaskCollision Block LocalFree CommitState stateOfMask
        (BoundedMaskValid (q := q) (Coord := Block × LocalFree) eta))
    (hstate_sub :
      ∀ y₀ y₁,
        stateOfMask y₀ = stateOfMask y₁ ->
          stateOfMask (fun bf => y₀ bf - y₁ bf) = 0)
    (hbound : Fintype.card (Block × LocalFree) * (eta + eta) < leeMin) :
    ModqLeeKernelViolation q stateOfMask leeMin :=
  C.toModqLeeKernelViolation_of_short hstate_sub
    (lt_of_le_of_lt
      (centeredNormL1_sub_le_card_mul_add_of_noiseSupport
        (q := q) (eta := eta) (rho := eta)
        (x := C.mask₀) (y := C.mask₁) C.valid₀ C.valid₁)
      hbound)

/--
Certificate that a concrete fibered commitment-state map rules out valid
same-state mask collisions.

The active information-set proof still has to reconstruct this state from the
small active view.  This certificate only closes the collision branch once such
a state map has been chosen.
-/
structure FiberedMaskStateCollisionCertificate
    {q : Nat} [NeZero q] (Block LocalFree CommitState : Type*)
    [Fintype (Block × LocalFree)] [Zero CommitState] where
  stateOfMask : (Block × LocalFree -> ZMod q) -> CommitState
  eta : Nat
  leeMin : Nat
  state_sub :
    ∀ y₀ y₁,
      stateOfMask y₀ = stateOfMask y₁ ->
        stateOfMask (fun bf => y₀ bf - y₁ bf) = 0
  shortCollisionBound :
    Fintype.card (Block × LocalFree) * (eta + eta) < leeMin
  noLeeKernel :
    ModqNoLeeKernelVectorBelow q stateOfMask leeMin

namespace FiberedMaskStateCollisionCertificate

variable {q : Nat} [NeZero q]
variable {Block LocalFree CommitState : Type*}
variable [Fintype (Block × LocalFree)] [Zero CommitState]

def ValidMask
    (C :
      FiberedMaskStateCollisionCertificate
        (q := q) Block LocalFree CommitState)
    (mask : Block × LocalFree -> ZMod q) : Prop :=
  BoundedMaskValid (q := q) (Coord := Block × LocalFree) C.eta mask

def collisionToLeeViolation
    (C :
      FiberedMaskStateCollisionCertificate
        (q := q) Block LocalFree CommitState)
    (W :
      SameStateMaskCollision Block LocalFree CommitState C.stateOfMask
        C.ValidMask) :
    ModqLeeKernelViolation q C.stateOfMask C.leeMin :=
  W.toModqLeeKernelViolation_of_boundedMasks
    C.state_sub C.shortCollisionBound

theorem no_sameStateMaskCollision
    (C :
      FiberedMaskStateCollisionCertificate
        (q := q) Block LocalFree CommitState) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree CommitState C.stateOfMask
        C.ValidMask) := by
  rintro ⟨W⟩
  exact ModqLeeKernelViolation.impossible_of_noLeeKernelVectorBelow
    q C.noLeeKernel (C.collisionToLeeViolation W)

end FiberedMaskStateCollisionCertificate

/--
A forked round for a fibered commitment representation.

The two accepting openings are tied to the same reconstructed commitment state,
but they may be backed by different sampled masks.  If the masks are equal this
collapses to the usual `ForkedLinearRound`.  If they differ, the round itself is
a same-state mask-collision witness.
-/
structure FiberedForkedLinearRound
    {q : Nat} [NeZero q] (Block LocalFree CommitState : Type*)
    (stateOfMask : (Block × LocalFree -> ZMod q) -> CommitState)
    (ValidMask : (Block × LocalFree -> ZMod q) -> Prop)
    (secret : Block × LocalFree -> ZMod q) where
  mask₀ : Block × LocalFree -> ZMod q
  mask₁ : Block × LocalFree -> ZMod q
  valid₀ : ValidMask mask₀
  valid₁ : ValidMask mask₁
  sameState : stateOfMask mask₀ = stateOfMask mask₁
  chi₀ : Block -> ZMod q
  chi₁ : Block -> ZMod q
  z₀ : Block × LocalFree -> ZMod q
  z₁ : Block × LocalFree -> ZMod q
  hz₀ : z₀ = productBlockLinearResponse chi₀ mask₀ secret
  hz₁ : z₁ = productBlockLinearResponse chi₁ mask₁ secret

namespace FiberedForkedLinearRound

variable {q : Nat} [NeZero q]
variable {Block LocalFree CommitState : Type*}
variable {stateOfMask : (Block × LocalFree -> ZMod q) -> CommitState}
variable {ValidMask : (Block × LocalFree -> ZMod q) -> Prop}
variable {secret : Block × LocalFree -> ZMod q}

def DeltaActive
    (R :
      FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
        ValidMask secret)
    (b : Block) : Prop :=
  DeltaExtractable R.chi₀ R.chi₁ b

def maskCollision
    (R :
      FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
        ValidMask secret)
    (hdiff : R.mask₀ ≠ R.mask₁) :
    SameStateMaskCollision Block LocalFree CommitState stateOfMask
      ValidMask where
  mask₀ := R.mask₀
  mask₁ := R.mask₁
  valid₀ := R.valid₀
  valid₁ := R.valid₁
  sameState := R.sameState
  different := hdiff

def HasMaskCollision
    (R :
      FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
        ValidMask secret) : Prop :=
  Nonempty
    { C : SameStateMaskCollision Block LocalFree CommitState stateOfMask
        ValidMask // C.mask₀ = R.mask₀ ∧ C.mask₁ = R.mask₁ }

noncomputable def toForkedLinearRound
    (R :
      FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
        ValidMask secret)
    (hsame : R.mask₀ = R.mask₁) :
    ForkedLinearRound Block LocalFree secret where
  chi₀ := R.chi₀
  chi₁ := R.chi₁
  mask := R.mask₀
  z₀ := R.z₀
  z₁ := R.z₁
  hz₀ := R.hz₀
  hz₁ := by
    rw [hsame]
    exact R.hz₁

noncomputable def correctedExtractCoordinate
    (R :
      FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
        ValidMask secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    ZMod q :=
  Classical.choose hdelta *
    (R.z₀ (b, f) - R.z₁ (b, f) - (R.mask₀ (b, f) - R.mask₁ (b, f)))

theorem correctedExtractCoordinate_eq_secret
    (R :
      FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
        ValidMask secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    R.correctedExtractCoordinate b hdelta f = secret (b, f) := by
  unfold correctedExtractCoordinate
  rw [R.hz₀, R.hz₁]
  exact extract_coordinate_of_deltaExtractable_with_mask_delta
    (q := q) (bf := (b, f))
    (inv := Classical.choose hdelta) (Classical.choose_spec hdelta)

noncomputable def extractedSecret
    {Round : Type*}
    (round :
      Round ->
        FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
          ValidMask secret)
    (hsame : ∀ r, (round r).mask₀ = (round r).mask₁)
    (hcover : CoversBlocks fun r b => (round r).DeltaActive b) :
    Block × LocalFree -> ZMod q :=
  ForkedLinearRound.extractedSecret
    (fun r => (round r).toForkedLinearRound (hsame r))
    (by
      intro b
      rcases hcover b with ⟨r, hr⟩
      exact ⟨r, hr⟩)

theorem extractedSecret_eq_of_all_sameMask
    {Round : Type*}
    (round :
      Round ->
        FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
          ValidMask secret)
    (hsame : ∀ r, (round r).mask₀ = (round r).mask₁)
    (hcover : CoversBlocks fun r b => (round r).DeltaActive b) :
    extractedSecret round hsame hcover = secret := by
  unfold extractedSecret
  exact ForkedLinearRound.extractedSecret_eq
    (fun r => (round r).toForkedLinearRound (hsame r))
    (by
      intro b
      rcases hcover b with ⟨r, hr⟩
      exact ⟨r, hr⟩)

/--
Fibered fork alternative.

Under coverage, a fibered fork either behaves like the usual same-mask fork and
extracts the product-block secret, or it contains an explicit same-state mask
collision.  The concrete commitment proof must reduce the right-hand case to
the keygen/q-ary lattice breaking witness.
-/
theorem extraction_or_collision_of_cover
    {Round : Type*}
    (round :
      Round ->
        FiberedForkedLinearRound Block LocalFree CommitState stateOfMask
          ValidMask secret)
    (hcover : CoversBlocks fun r b => (round r).DeltaActive b) :
    (∃ hsame : ∀ r, (round r).mask₀ = (round r).mask₁,
        extractedSecret round hsame hcover = secret) ∨
      ∃ r : Round, (round r).HasMaskCollision := by
  classical
  by_cases hsame : ∀ r, (round r).mask₀ = (round r).mask₁
  · exact Or.inl
      ⟨hsame, extractedSecret_eq_of_all_sameMask round hsame hcover⟩
  · rcases not_forall.mp hsame with ⟨r, hdiff⟩
    exact Or.inr
      ⟨r, ⟨⟨(round r).maskCollision hdiff, by constructor <;> rfl⟩⟩⟩

end FiberedForkedLinearRound

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

/--
Active-only multi-fork round.

This is the deterministic extractor interface for compact signatures.  A draw
only serializes coordinates on blocks where its challenge scalar is a signed
unit.  The verifier-side soundness obligation supplies the response equation
for exactly those coordinates.
-/
structure CompactMultiForkLinearRound
    {q : Nat} [NeZero q] (Draw Block LocalFree : Type*)
    (secret : Block × LocalFree -> ZMod q) where
  chi : Draw -> Block -> ZMod q
  mask : Block × LocalFree -> ZMod q
  z :
    ∀ d b, (chi d b = 1 ∨ chi d b = -1) ->
      LocalFree -> ZMod q
  hz :
    ∀ d b h f,
      z d b h f = productBlockLinearResponse (chi d) mask secret (b, f)

namespace CompactMultiForkLinearRound

variable {q : Nat} [NeZero q]
variable {Draw Block LocalFree : Type*}
variable {secret : Block × LocalFree -> ZMod q}

def CompactOppositeActive
    (R : CompactMultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) : Prop :=
  ∃ d₀ d₁, OppositeSignedActive (R.chi d₀) (R.chi d₁) b

theorem left_signedUnit_of_opposite
    {R : CompactMultiForkLinearRound Draw Block LocalFree secret}
    {d₀ d₁ : Draw} {b : Block}
    (hopp : OppositeSignedActive (R.chi d₀) (R.chi d₁) b) :
    R.chi d₀ b = 1 ∨ R.chi d₀ b = -1 := by
  rcases hopp with ⟨h₀, _h₁⟩ | ⟨h₀, _h₁⟩
  · exact Or.inl h₀
  · exact Or.inr h₀

theorem right_signedUnit_of_opposite
    {R : CompactMultiForkLinearRound Draw Block LocalFree secret}
    {d₀ d₁ : Draw} {b : Block}
    (hopp : OppositeSignedActive (R.chi d₀) (R.chi d₁) b) :
    R.chi d₁ b = 1 ∨ R.chi d₁ b = -1 := by
  rcases hopp with ⟨_h₀, h₁⟩ | ⟨_h₀, h₁⟩
  · exact Or.inr h₁
  · exact Or.inl h₁

def PairOppositeActive
    (R : CompactMultiForkLinearRound Draw Block LocalFree secret)
    (d₀ d₁ : Draw) (b : Block) : Prop :=
  OppositeSignedActive (R.chi d₀) (R.chi d₁) b

noncomputable def oppositeLeft
    (R : CompactMultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) (hcompact : R.CompactOppositeActive b) : Draw :=
  Classical.choose hcompact

noncomputable def oppositeRight
    (R : CompactMultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) (hcompact : R.CompactOppositeActive b) : Draw :=
  Classical.choose (Classical.choose_spec hcompact)

theorem oppositeWitness_spec
    (R : CompactMultiForkLinearRound Draw Block LocalFree secret)
    (b : Block) (hcompact : R.CompactOppositeActive b) :
    R.PairOppositeActive (R.oppositeLeft b hcompact)
      (R.oppositeRight b hcompact) b := by
  unfold oppositeLeft oppositeRight PairOppositeActive
  exact Classical.choose_spec (Classical.choose_spec hcompact)

noncomputable def extractCoordinate
    (R : CompactMultiForkLinearRound Draw Block LocalFree secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (b : Block) (hcompact : R.CompactOppositeActive b)
    (f : LocalFree) : ZMod q :=
  let d₀ := R.oppositeLeft b hcompact
  let d₁ := R.oppositeRight b hcompact
  let hopp := R.oppositeWitness_spec b hcompact
  let h₀ := R.left_signedUnit_of_opposite hopp
  let h₁ := R.right_signedUnit_of_opposite hopp
  let inv := Classical.choose
    (deltaExtractable_of_oppositeSignedActive hinvTwo hopp)
  inv * (R.z d₀ b h₀ f - R.z d₁ b h₁ f)

theorem extractCoordinate_eq_secret
    (R : CompactMultiForkLinearRound Draw Block LocalFree secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (b : Block) (hcompact : R.CompactOppositeActive b)
    (f : LocalFree) :
    R.extractCoordinate hinvTwo b hcompact f = secret (b, f) := by
  unfold extractCoordinate
  let d₀ := R.oppositeLeft b hcompact
  let d₁ := R.oppositeRight b hcompact
  let hopp := R.oppositeWitness_spec b hcompact
  let h₀ := R.left_signedUnit_of_opposite hopp
  let h₁ := R.right_signedUnit_of_opposite hopp
  let hdelta := deltaExtractable_of_oppositeSignedActive hinvTwo hopp
  have hchoose : Classical.choose hdelta *
        (R.z d₀ b h₀ f - R.z d₁ b h₁ f) =
      Classical.choose hdelta *
        (productBlockLinearResponse (R.chi d₀) R.mask secret (b, f) -
          productBlockLinearResponse (R.chi d₁) R.mask secret (b, f)) := by
    rw [R.hz d₀ b h₀ f, R.hz d₁ b h₁ f]
  rw [hchoose]
  exact extract_coordinate_of_deltaExtractable
    (q := q) (chi₀ := R.chi d₀) (chi₁ := R.chi d₁)
    (mask := R.mask) (secret := secret) (bf := (b, f))
    (inv := Classical.choose hdelta) (Classical.choose_spec hdelta)

/--
Active-only extraction: opposite signed active compact responses covering every
block recover the product-block secret.
-/
noncomputable def extractedSecret
    {Round : Type*}
    (round : Round -> CompactMultiForkLinearRound Draw Block LocalFree secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover : CoversBlocks fun r b => (round r).CompactOppositeActive b) :
    Block × LocalFree -> ZMod q :=
  fun bf =>
    let r := Classical.choose (hcover bf.1)
    (round r).extractCoordinate hinvTwo bf.1
      (Classical.choose_spec (hcover bf.1)) bf.2

theorem extractedSecret_eq
    {Round : Type*}
    (round : Round -> CompactMultiForkLinearRound Draw Block LocalFree secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover : CoversBlocks fun r b => (round r).CompactOppositeActive b) :
    extractedSecret round hinvTwo hcover = secret := by
  funext bf
  unfold extractedSecret
  exact (round (Classical.choose (hcover bf.1))).extractCoordinate_eq_secret
    hinvTwo bf.1 (Classical.choose_spec (hcover bf.1)) bf.2

end CompactMultiForkLinearRound

/--
Full-response honest verifier equations for one repeated commitment.

This is the signable relation missing from the certificate-violation endpoint:
the signer chooses `mask`, publishes the commitment `A * mask`, answers with
`z = mask + chi * secret`, and the verifier checks `A * z = t + chi * pk`.
-/
structure HonestVerifierEquationMultiForkRound
    {q : Nat} [NeZero q]
    (Draw Block LocalPivot LocalFree : Type*)
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (secret : Block × LocalFree -> ZMod q) where
  mask : Block × LocalFree -> ZMod q
  chi : Draw -> Block -> ZMod q
  z : Draw -> Block × LocalFree -> ZMod q
  verifierEquation :
    ∀ d,
      productBlockVerifierEquation Plocal (chi d)
        (productBlockPublicKey Plocal secret)
        (productBlockCommitment Plocal mask)
        (z d)

namespace HonestVerifierEquationMultiForkRound

variable {q : Nat} [NeZero q]
variable {Draw Block LocalPivot LocalFree : Type*}
variable [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
variable {Plocal : Block -> LocalPivot -> LocalFree -> ZMod q}
variable {secret : Block × LocalFree -> ZMod q}

def toMultiForkLinearRound
    (R :
      HonestVerifierEquationMultiForkRound
        Draw Block LocalPivot LocalFree Plocal secret) :
    MultiForkLinearRound Draw Block LocalFree secret where
  chi := R.chi
  mask := R.mask
  z := R.z
  hz := by
    intro d
    exact linearResponse_of_productBlockVerifierEquation
      (Plocal := Plocal) (h := R.verifierEquation d)

def CompactOppositeActive
    (R :
      HonestVerifierEquationMultiForkRound
        Draw Block LocalPivot LocalFree Plocal secret)
    (b : Block) : Prop :=
  R.toMultiForkLinearRound.CompactOppositeActive b

noncomputable def extractedSecret
    {Round : Type*}
    (round :
      Round ->
        HonestVerifierEquationMultiForkRound
          Draw Block LocalPivot LocalFree Plocal secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover :
      CoversBlocks fun r b => (round r).CompactOppositeActive b) :
    Block × LocalFree -> ZMod q :=
  MultiForkLinearRound.extractedSecret
    (fun r => (round r).toMultiForkLinearRound)
    (MultiForkLinearRound.coversMultiDelta_of_coversCompactOpposite
      (fun r => (round r).toMultiForkLinearRound) hinvTwo hcover)

theorem extractedSecret_eq
    {Round : Type*}
    (round :
      Round ->
        HonestVerifierEquationMultiForkRound
          Draw Block LocalPivot LocalFree Plocal secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover :
      CoversBlocks fun r b => (round r).CompactOppositeActive b) :
    extractedSecret round hinvTwo hcover = secret := by
  exact MultiForkLinearRound.extractedSecret_eq_of_coversCompactOpposite
    (fun r => (round r).toMultiForkLinearRound) hinvTwo hcover

end HonestVerifierEquationMultiForkRound

/--
Active-only honest verifier equations for compact signatures.

The response contains only free coordinates on signed active blocks.  The
verifier equation still checks the local systematic block against the public
commitment and public key, so honest signatures are valid while forked accepting
transcripts continue to instantiate the compact extractor.
-/
structure CompactHonestVerifierEquationMultiForkRound
    {q : Nat} [NeZero q]
    (Draw Block LocalPivot LocalFree : Type*)
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (secret : Block × LocalFree -> ZMod q) where
  mask : Block × LocalFree -> ZMod q
  chi : Draw -> Block -> ZMod q
  z :
    ∀ d b, (chi d b = 1 ∨ chi d b = -1) ->
      LocalFree -> ZMod q
  verifierEquation :
    ∀ d,
      productBlockActiveVerifierEquation Plocal (chi d)
        (productBlockPublicKey Plocal secret)
        (productBlockCommitment Plocal mask)
        (z d)

namespace CompactHonestVerifierEquationMultiForkRound

variable {q : Nat} [NeZero q]
variable {Draw Block LocalPivot LocalFree : Type*}
variable [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
variable {Plocal : Block -> LocalPivot -> LocalFree -> ZMod q}
variable {secret : Block × LocalFree -> ZMod q}

def toCompactMultiForkLinearRound
    (R :
      CompactHonestVerifierEquationMultiForkRound
        Draw Block LocalPivot LocalFree Plocal secret) :
    CompactMultiForkLinearRound Draw Block LocalFree secret where
  chi := R.chi
  mask := R.mask
  z := R.z
  hz := by
    intro d b h f
    exact activeLinearResponse_of_productBlockActiveVerifierEquation
      (Plocal := Plocal) (h := R.verifierEquation d) b h f

def CompactOppositeActive
    (R :
      CompactHonestVerifierEquationMultiForkRound
        Draw Block LocalPivot LocalFree Plocal secret)
    (b : Block) : Prop :=
  R.toCompactMultiForkLinearRound.CompactOppositeActive b

noncomputable def extractedSecret
    {Round : Type*}
    (round :
      Round ->
        CompactHonestVerifierEquationMultiForkRound
          Draw Block LocalPivot LocalFree Plocal secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover :
      CoversBlocks fun r b => (round r).CompactOppositeActive b) :
    Block × LocalFree -> ZMod q :=
  CompactMultiForkLinearRound.extractedSecret
    (fun r => (round r).toCompactMultiForkLinearRound)
    hinvTwo hcover

theorem extractedSecret_eq
    {Round : Type*}
    (round :
      Round ->
        CompactHonestVerifierEquationMultiForkRound
          Draw Block LocalPivot LocalFree Plocal secret)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover :
      CoversBlocks fun r b => (round r).CompactOppositeActive b) :
    extractedSecret round hinvTwo hcover = secret := by
  exact CompactMultiForkLinearRound.extractedSecret_eq
    (fun r => (round r).toCompactMultiForkLinearRound) hinvTwo hcover

end CompactHonestVerifierEquationMultiForkRound

end BlockChallengeSoundness

end Descender
