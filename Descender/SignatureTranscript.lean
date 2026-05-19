import Mathlib.Analysis.MeanInequalitiesPow
import Mathlib.Tactic
import Descender.SignatureForking

namespace Descender

/-!
Canonical transcript boundary for the current small product-block signature row.

This file records the first concrete Fiat-Shamir transcript object for the
`n = 32`, `255 B` candidate.  The challenge has sixteen product blocks, omits
one block in each repetition, and signs the remaining fifteen blocks.  There
are seven repetitions, so the canonical challenge space has cardinality

  `(16 * 2^15)^7 = 2^133`.

The random-oracle replay proof can use this as the concrete blind
challenge-guess loss.
-/

namespace SignatureTranscript

open Advantage
open BlockChallengeSoundness
open SignatureForking

noncomputable section

/-- Uniform finite distribution on a nonempty finite type. -/
def uniformFiniteProbability (Ω : Type*) [Fintype Ω] [Nonempty Ω] :
    FiniteProbability Ω where
  mass := fun _ => ((Fintype.card Ω : ℝ)⁻¹)
  nonnegative := by
    intro _ω
    positivity
  total_mass := by
    simp [Finset.sum_const, nsmul_eq_mul]

/-- Product of two finite probability distributions. -/
def productFiniteProbability
    {α β : Type*} [Fintype α] [Fintype β]
    (Dα : FiniteProbability α) (Dβ : FiniteProbability β) :
    FiniteProbability (α × β) where
  mass := fun x => Dα.mass x.1 * Dβ.mass x.2
  nonnegative := by
    intro x
    exact mul_nonneg (Dα.nonnegative x.1) (Dβ.nonnegative x.2)
  total_mass := by
    rw [Fintype.sum_prod_type]
    calc
      (Finset.univ.sum fun a =>
          Finset.univ.sum fun b => Dα.mass a * Dβ.mass b) =
          Finset.univ.sum fun a =>
            Dα.mass a * (Finset.univ.sum fun b => Dβ.mass b) := by
            apply Finset.sum_congr rfl
            intro a _ha
            rw [Finset.mul_sum]
      _ = Finset.univ.sum fun a => Dα.mass a * 1 := by
            rw [Dβ.total_mass]
      _ = 1 := by
            simpa using Dα.total_mass

theorem productFiniteProbability_ignore_left_acceptProb
    {α β : Type*} [Fintype α] [Fintype β]
    (Dα : FiniteProbability α) (Dβ : FiniteProbability β)
    (A : β -> Bool) :
    (productFiniteProbability Dα Dβ).acceptProb
        (fun x : α × β => A x.2) =
      Dβ.acceptProb A := by
  unfold FiniteProbability.acceptProb productFiniteProbability
  rw [Fintype.sum_prod_type]
  calc
    (Finset.univ.sum fun a =>
        Finset.univ.sum fun b =>
          Dα.mass a * Dβ.mass b * boolIndicator (A b)) =
        Finset.univ.sum fun a =>
          Dα.mass a *
            (Finset.univ.sum fun b => Dβ.mass b * boolIndicator (A b)) := by
          apply Finset.sum_congr rfl
          intro a _ha
          rw [Finset.mul_sum]
          apply Finset.sum_congr rfl
          intro b _hb
          ring
    _ =
        (Finset.univ.sum fun a => Dα.mass a) *
          (Finset.univ.sum fun b => Dβ.mass b * boolIndicator (A b)) := by
          rw [Finset.sum_mul]
    _ = Finset.univ.sum fun b => Dβ.mass b * boolIndicator (A b) := by
          rw [Dα.total_mass]
          ring

theorem acceptProb_eq_zero_of_forall_not
    {Ω : Type*} [Fintype Ω]
    (D : FiniteProbability Ω) (A : Ω -> Bool)
    (h : ∀ ω, A ω = true -> False) :
    D.acceptProb A = 0 := by
  unfold FiniteProbability.acceptProb
  apply Finset.sum_eq_zero
  intro ω _hω
  have hfalse : A ω = false := by
    cases hA : A ω
    · rfl
    · exfalso
      exact h ω hA
  simp [hfalse]

/-- Product distribution over a finite function space. -/
def functionProductFiniteProbability
    {ι Ω : Type*} [Fintype ι] [DecidableEq ι] [Fintype Ω]
    (D : ι -> FiniteProbability Ω) :
    FiniteProbability (ι -> Ω) where
  mass := fun x => Finset.univ.prod fun i => (D i).mass (x i)
  nonnegative := by
    intro x
    exact Finset.prod_nonneg (by
      intro i _hi
      exact (D i).nonnegative (x i))
  total_mass := by
    classical
    calc
      (Finset.univ.sum fun x : ι -> Ω =>
          Finset.univ.prod fun i => (D i).mass (x i)) =
          Finset.univ.sum fun x : ι -> Ω =>
            Finset.univ.prod fun i => (D i).mass (x i) := rfl
      _ =
          Finset.univ.sum fun x : ι -> Ω =>
            Finset.univ.prod fun i => (D i).mass (x i) := rfl
      _ = Finset.univ.prod fun i =>
            Finset.univ.sum fun a => (D i).mass a := by
          rw [← Finset.sum_prod_piFinset
            (s := (Finset.univ : Finset Ω))
            (g := fun i a => (D i).mass a)]
          simp
      _ = Finset.univ.prod fun _i : ι => (1 : ℝ) := by
          apply Finset.prod_congr rfl
          intro i _hi
          exact (D i).total_mass
      _ = 1 := by simp

theorem boolIndicator_forall_eq_prod
    {ι : Type*} [Fintype ι]
    (A : ι -> Bool) :
    boolIndicator (decide (∀ i, A i = true)) =
      Finset.univ.prod fun i => boolIndicator (A i) := by
  classical
  by_cases h : ∀ i, A i = true
  · simp [h]
  · have hexists : ∃ i, A i = false := by
      by_contra hnone
      apply h
      intro i
      cases hi : A i
      · exfalso
        exact hnone ⟨i, hi⟩
      · rfl
    rcases hexists with ⟨i, hi⟩
    have hzero :
        (Finset.univ.prod fun i => boolIndicator (A i)) = 0 :=
      Finset.prod_eq_zero (Finset.mem_univ i) (by simp [hi])
    simp [h, hzero]

/--
Independent finite draws accept all coordinate tests with probability equal to
the product of their individual accepting probabilities.
-/
theorem functionProduct_acceptProb_forall_eq_prod
    {ι Ω : Type*} [Fintype ι] [DecidableEq ι] [Fintype Ω]
    (D : ι -> FiniteProbability Ω)
    (A : ι -> Ω -> Bool) :
    (functionProductFiniteProbability D).acceptProb
        (fun x => decide (∀ i, A i (x i) = true)) =
      Finset.univ.prod fun i => (D i).acceptProb (A i) := by
  classical
  unfold FiniteProbability.acceptProb functionProductFiniteProbability
  calc
    (Finset.univ.sum fun x : ι -> Ω =>
        (Finset.univ.prod fun i => (D i).mass (x i)) *
          boolIndicator (decide (∀ i, A i (x i) = true))) =
        Finset.univ.sum fun x : ι -> Ω =>
          Finset.univ.prod fun i =>
            (D i).mass (x i) * boolIndicator (A i (x i)) := by
          apply Finset.sum_congr rfl
          intro x _hx
          rw [boolIndicator_forall_eq_prod]
          rw [← Finset.prod_mul_distrib]
    _ = Finset.univ.prod fun i =>
          Finset.univ.sum fun a =>
            (D i).mass a * boolIndicator (A i a) := by
          rw [← Finset.sum_prod_piFinset
            (s := (Finset.univ : Finset Ω))
            (g := fun i a => (D i).mass a * boolIndicator (A i a))]
          simp

set_option linter.constructorNameAsVariable false in
/--
State-local product-moment identity for independent replay draws.

After cutting an execution at the challenge point, this is the finite
probability statement we need: the chance that every independently programmed
draw accepts is the state average of the product of the per-state accepting
densities.
-/
theorem productFunctionProduct_acceptProb_forall_eq_state_product
    {State Draw Challenge : Type*}
    [Fintype State] [Fintype Draw] [DecidableEq Draw] [Fintype Challenge]
    (Dσ : FiniteProbability State)
    (Ddraw : Draw -> FiniteProbability Challenge)
    (accepts : State -> Draw -> Challenge -> Bool) :
    (productFiniteProbability Dσ
        (functionProductFiniteProbability Ddraw)).acceptProb
        (fun trace =>
          decide (∀ d, accepts trace.1 d (trace.2 d) = true)) =
      Finset.univ.sum fun state =>
        Dσ.mass state *
          Finset.univ.prod fun d =>
            (Ddraw d).acceptProb (accepts state d) := by
  unfold FiniteProbability.acceptProb productFiniteProbability
  rw [Fintype.sum_prod_type]
  apply Finset.sum_congr rfl
  intro state _hstate
  calc
    (Finset.univ.sum fun draws : Draw -> Challenge =>
        Dσ.mass state * (functionProductFiniteProbability Ddraw).mass draws *
          boolIndicator
            (decide (∀ d, accepts state d (draws d) = true))) =
        Dσ.mass state *
          ((functionProductFiniteProbability Ddraw).acceptProb
            (fun draws =>
              decide (∀ d, accepts state d (draws d) = true))) := by
          unfold FiniteProbability.acceptProb
          rw [Finset.mul_sum]
          apply Finset.sum_congr rfl
          intro draws _hdraws
          ring
    _ = Dσ.mass state *
        (Finset.univ.prod fun d =>
          (Ddraw d).acceptProb (accepts state d)) := by
          have hinner :=
            functionProduct_acceptProb_forall_eq_prod Ddraw
              (fun d c => accepts state d c)
          exact congrArg (fun x => Dσ.mass state * x) hinner

set_option linter.constructorNameAsVariable false in
/--
Identical-draw specialization of the product moment.

If every draw has the same accepting set at a fixed state, the all-draw replay
success probability is the `card Draw` moment of the state-local accepting
density.
-/
theorem productFunctionProduct_identical_acceptProb_eq_state_moment
    {State Draw Challenge : Type*}
    [Fintype State] [Fintype Draw] [DecidableEq Draw] [Fintype Challenge]
    (Dσ : FiniteProbability State)
    (Dchallenge : FiniteProbability Challenge)
    (accepts : State -> Challenge -> Bool) :
    (productFiniteProbability Dσ
        (functionProductFiniteProbability
          (fun _d : Draw => Dchallenge))).acceptProb
        (fun trace =>
          decide (∀ d : Draw, accepts trace.1 (trace.2 d) = true)) =
      Finset.univ.sum fun state =>
        Dσ.mass state *
          (Dchallenge.acceptProb (accepts state)) ^ Fintype.card Draw := by
  calc
    ((productFiniteProbability Dσ
        (functionProductFiniteProbability
          (fun _d : Draw => Dchallenge))).acceptProb
        (fun trace =>
          decide (∀ d : Draw, accepts trace.1 (trace.2 d) = true))) =
        Finset.univ.sum fun state =>
          Dσ.mass state *
            Finset.univ.prod fun _d : Draw =>
              Dchallenge.acceptProb (accepts state) := by
          exact
            productFunctionProduct_acceptProb_forall_eq_state_product
              Dσ (fun _d : Draw => Dchallenge)
              (fun state _d c => accepts state c)
    _ =
        Finset.univ.sum fun state =>
          Dσ.mass state *
            (Dchallenge.acceptProb (accepts state)) ^
              Fintype.card Draw := by
          apply Finset.sum_congr rfl
          intro state _hstate
          have hprod :
              (Finset.univ.prod fun _d : Draw =>
                Dchallenge.acceptProb (accepts state)) =
              (Dchallenge.acceptProb (accepts state)) ^
                Fintype.card Draw := by
            simp [Finset.prod_const]
          rw [hprod]

/--
Finite Jensen/mean-power inequality for the lightweight probability interface.

For any nonnegative state-local density `x`, the `n`th moment dominates the
`n`th power of the mean.
-/
theorem finiteProbability_mean_pow_le_moment
    {Ω : Type*} [Fintype Ω]
    (D : FiniteProbability Ω)
    (x : Ω -> ℝ)
    (hx : ∀ ω, 0 <= x ω)
    (n : ℕ) :
    (Finset.univ.sum fun ω => D.mass ω * x ω) ^ n <=
      Finset.univ.sum fun ω => D.mass ω * (x ω) ^ n := by
  classical
  simpa using
    (Real.pow_arith_mean_le_arith_mean_pow
      (s := (Finset.univ : Finset Ω))
      (w := D.mass)
      (z := x)
      (hw := by
        intro ω _hω
        exact D.nonnegative ω)
      (hw' := by
        simpa using D.total_mass)
      (hz := by
        intro ω _hω
        exact hx ω)
      (n := n))

/--
Moment lower bound for identical independent replay draws.

If each independent draw accepts with state-local density `p[state]`, then the
all-draw replay probability is at least `E[p] ^ card Draw`.
-/
theorem productFunctionProduct_identical_acceptProb_ge_mean_pow
    {State Draw Challenge : Type*}
    [Fintype State] [Fintype Draw] [DecidableEq Draw] [Fintype Challenge]
    (Dσ : FiniteProbability State)
    (Dchallenge : FiniteProbability Challenge)
    (accepts : State -> Challenge -> Bool) :
    (Finset.univ.sum fun state =>
      Dσ.mass state * Dchallenge.acceptProb (accepts state)) ^
        Fintype.card Draw <=
      (productFiniteProbability Dσ
        (functionProductFiniteProbability
          (fun _d : Draw => Dchallenge))).acceptProb
        (fun trace =>
          decide (∀ d : Draw, accepts trace.1 (trace.2 d) = true)) := by
  have hmoment :=
    finiteProbability_mean_pow_le_moment Dσ
      (fun state => Dchallenge.acceptProb (accepts state))
      (fun state => Dchallenge.acceptProb_nonneg (accepts state))
      (Fintype.card Draw)
  rw [productFunctionProduct_identical_acceptProb_eq_state_moment]
  exact hmoment

/-- A fixed value under the uniform finite distribution is hit with probability `1 / |Ω|`. -/
theorem uniform_singleton_acceptProb
    (Ω : Type*) [Fintype Ω] [Nonempty Ω] [DecidableEq Ω]
    (target : Ω) :
    (uniformFiniteProbability Ω).acceptProb
        (fun actual => decide (actual = target)) =
      ((Fintype.card Ω : ℝ)⁻¹) := by
  unfold FiniteProbability.acceptProb uniformFiniteProbability
  rw [Finset.sum_eq_single target]
  · simp [boolIndicator]
  · intro x _hx hne
    simp [hne, boolIndicator]
  · intro hnot
    simp at hnot

/-- Uniform finite probability of a Boolean event, as a cardinality ratio. -/
theorem uniform_acceptProb_eq_card_true
    {Ω : Type*} [Fintype Ω] [Nonempty Ω]
    (A : Ω -> Bool) :
    (uniformFiniteProbability Ω).acceptProb A =
      (Fintype.card {ω : Ω // A ω = true} : ℝ) *
        ((Fintype.card Ω : ℝ)⁻¹) := by
  classical
  unfold FiniteProbability.acceptProb uniformFiniteProbability
  simp only
  rw [← Finset.mul_sum]
  rw [mul_comm]
  congr 1
  have hfilter :
      (Finset.univ.sum fun ω => boolIndicator (A ω)) =
        ((Finset.univ.filter (fun ω : Ω => A ω = true)).card : ℝ) := by
    have hconst :
        ((Finset.univ.filter (fun ω : Ω => A ω = true)).card : ℝ) =
          (Finset.univ.filter (fun ω : Ω => A ω = true)).sum
            (fun _ => (1 : ℝ)) := by
      simp
    rw [hconst]
    rw [Finset.sum_filter]
    apply Finset.sum_congr rfl
    intro ω _hω
    cases A ω <;> simp [boolIndicator]
  rw [hfilter]
  norm_num
  simp [Fintype.card_subtype]

theorem exists_true_of_acceptProb_pos
    {Ω : Type*} [Fintype Ω]
    (D : FiniteProbability Ω)
    (A : Ω -> Bool)
    (hpos : 0 < D.acceptProb A) :
    ∃ ω, A ω = true := by
  by_contra hnone
  have hzero : D.acceptProb A = 0 := by
    unfold FiniteProbability.acceptProb
    apply Finset.sum_eq_zero
    intro ω _hω
    have hfalse : A ω = false := by
      cases hA : A ω
      · rfl
      · exfalso
        exact hnone ⟨ω, hA⟩
    simp [hfalse]
  linarith

/-- Sixteen product blocks in the `n = 32` row. -/
abbrev N32Block := Fin 16

/-- Seven repetitions in the current `n = 32`, `255 B` row. -/
abbrev N32Repetition := Fin 7

/-- Six replay draws for the compact multi-fork extractor. -/
abbrev N32ReplayDraw := Fin 6

theorem n32ReplayDraw_card :
    Fintype.card N32ReplayDraw = 6 := by
  rfl

theorem n32Repetition_card :
    Fintype.card N32Repetition = 7 := by
  rfl

/-- The active blocks are all blocks except the omitted one. -/
abbrev N32ActiveBlock (omitted : N32Block) :=
  {b : N32Block // b ≠ omitted}

/--
One signed weight-15 product-block challenge.

For this exact row, choosing a weight-15 subset is equivalent to choosing the
single omitted block, then assigning one sign bit to each remaining block.
-/
abbrev N32RoundChallenge :=
  Sigma fun omitted : N32Block => N32ActiveBlock omitted -> Bool

/-- The full Fiat-Shamir challenge vector: one signed sparse challenge per repetition. -/
abbrev N32CanonicalChallenge :=
  N32Repetition -> N32RoundChallenge

namespace N32RoundChallenge

/-- The block omitted by this sparse challenge. -/
def omitted (c : N32RoundChallenge) : N32Block :=
  c.1

/-- Whether a block is active in this sparse challenge. -/
def active (c : N32RoundChallenge) (b : N32Block) : Bool :=
  decide (b ≠ c.omitted)

/-- The sign bit for an active block. -/
def sign (c : N32RoundChallenge) (b : N32ActiveBlock c.omitted) : Bool :=
  c.2 b

theorem active_iff (c : N32RoundChallenge) (b : N32Block) :
    c.active b = true ↔ b ≠ c.omitted := by
  simp [active]

/-- The signed block scalar attached to this challenge. -/
def scalar {q : Nat} [NeZero q] (c : N32RoundChallenge) (b : N32Block) :
    ZMod q :=
  if h : b = c.omitted then 0
  else if c.sign ⟨b, h⟩ then 1 else -1

def positiveAt (c : N32RoundChallenge) (b : N32Block) : Bool :=
  if h : b = c.omitted then false else c.sign ⟨b, h⟩

def negativeAt (c : N32RoundChallenge) (b : N32Block) : Bool :=
  if h : b = c.omitted then false else !c.sign ⟨b, h⟩

def noPositiveAt (c : N32RoundChallenge) (b : N32Block) : Bool :=
  !positiveAt c b

def noNegativeAt (c : N32RoundChallenge) (b : N32Block) : Bool :=
  !negativeAt c b

theorem scalar_omitted {q : Nat} [NeZero q]
    (c : N32RoundChallenge) :
    c.scalar (q := q) c.omitted = 0 := by
  simp [scalar]

theorem scalar_signedUnit_of_active {q : Nat} [NeZero q]
    (c : N32RoundChallenge) {b : N32Block}
    (hactive : b ≠ c.omitted) :
    c.scalar (q := q) b = 1 ∨ c.scalar (q := q) b = -1 := by
  by_cases hsign : c.sign ⟨b, hactive⟩
  · exact Or.inl (by simp [scalar, hactive, hsign])
  · exact Or.inr (by simp [scalar, hactive, hsign])

theorem scalar_signedUnitOrZero {q : Nat} [NeZero q]
    (c : N32RoundChallenge) :
    BlockSignedUnitOrZero (q := q) (c.scalar (q := q)) := by
  intro b
  by_cases hactive : b ≠ c.omitted
  · rcases c.scalar_signedUnit_of_active (q := q) hactive with h | h
    · exact Or.inr (Or.inl h)
    · exact Or.inr (Or.inr h)
  · have hb : b = c.omitted := by simpa using hactive
    subst hb
    exact Or.inl (c.scalar_omitted (q := q))

theorem scalar_eq_one_of_positiveAt {q : Nat} [NeZero q]
    (c : N32RoundChallenge) {b : N32Block}
    (hpos : c.positiveAt b = true) :
    c.scalar (q := q) b = 1 := by
  unfold positiveAt at hpos
  unfold scalar
  split at hpos
  · simp at hpos
  · rename_i hactive
    simp [hactive, hpos]

theorem scalar_eq_neg_one_of_negativeAt {q : Nat} [NeZero q]
    (c : N32RoundChallenge) {b : N32Block}
    (hneg : c.negativeAt b = true) :
    c.scalar (q := q) b = -1 := by
  unfold negativeAt at hneg
  unfold scalar
  split at hneg
  · simp at hneg
  · rename_i hactive
    have hsign : c.sign ⟨b, hactive⟩ = false := by
      cases hs : c.sign ⟨b, hactive⟩ <;> simp [hs] at hneg ⊢
    simp [hactive, hsign]

end N32RoundChallenge

theorem n32ActiveBlock_card (omitted : N32Block) :
    Fintype.card (N32ActiveBlock omitted) = 15 := by
  simp [N32ActiveBlock, Fintype.card_subtype_compl]

def n32ActiveBlocksPerRepetition : Nat := 15

theorem n32ActiveBlocksPerRepetition_eq (omitted : N32Block) :
    n32ActiveBlocksPerRepetition = Fintype.card (N32ActiveBlock omitted) := by
  rw [n32ActiveBlock_card]
  rfl

theorem n32RoundChallenge_card :
    Fintype.card N32RoundChallenge = 16 * 2 ^ 15 := by
  norm_num [N32RoundChallenge, N32ActiveBlock, Fintype.card_sigma,
    Fintype.card_subtype_compl]

theorem n32CanonicalChallenge_card :
    Fintype.card N32CanonicalChallenge = 2 ^ 133 := by
  norm_num [N32CanonicalChallenge, N32RoundChallenge, N32ActiveBlock,
    Fintype.card_sigma, Fintype.card_subtype_compl]

theorem n32CanonicalChallenge_uniform_mass_eq_round_product
    (c : N32CanonicalChallenge) :
    (uniformFiniteProbability N32CanonicalChallenge).mass c =
      Finset.univ.prod fun r : N32Repetition =>
        (uniformFiniteProbability N32RoundChallenge).mass (c r) := by
  unfold uniformFiniteProbability
  simp [Finset.prod_const]
  norm_num

/-- The concrete blind challenge-guess bound for the `n = 32`, `255 B` row. -/
def n32ChallengeGuessBound : ℝ :=
  ((2 : ℝ) ^ 133)⁻¹

/-- Guessing one canonical challenge under the uniform challenge law costs exactly `2^-133`. -/
theorem n32_single_challenge_guess_acceptProb
    (guess : N32CanonicalChallenge) :
    (uniformFiniteProbability N32CanonicalChallenge).acceptProb
        (fun actual => decide (actual = guess)) =
      n32ChallengeGuessBound := by
  rw [uniform_singleton_acceptProb]
  rw [n32CanonicalChallenge_card]
  norm_num [n32ChallengeGuessBound]

/--
A finite family of blind guesses succeeds with probability at most
`number_of_guesses / 2^133`.

This is the transcript-facing form of the `queryMiss` loss: if the adversary
did not query the challenge point, every output challenge must be one of these
blind guesses.
-/
theorem n32_challenge_guess_family_acceptProb_le
    {Guess : Type*} [Fintype Guess] [DecidableEq Guess]
    (guess : Guess -> N32CanonicalChallenge) :
    (uniformFiniteProbability N32CanonicalChallenge).acceptProb
        (fun actual => decide (∃ g, guess g = actual)) <=
      (Fintype.card Guess : ℝ) * n32ChallengeGuessBound := by
  have hunion :
      (uniformFiniteProbability N32CanonicalChallenge).acceptProb
          (fun actual => decide (∃ g, guess g = actual)) <=
        Finset.univ.sum fun g =>
          (uniformFiniteProbability N32CanonicalChallenge).acceptProb
            (fun actual => decide (guess g = actual)) :=
    acceptProb_le_sum_of_exists
      (uniformFiniteProbability N32CanonicalChallenge)
      (fun actual => decide (∃ g, guess g = actual))
      (fun g actual => decide (guess g = actual))
      (by
        intro actual hguess
        simp only [decide_eq_true_eq] at hguess ⊢
        exact hguess)
  calc
    (uniformFiniteProbability N32CanonicalChallenge).acceptProb
        (fun actual => decide (∃ g, guess g = actual)) <=
        Finset.univ.sum fun g =>
          (uniformFiniteProbability N32CanonicalChallenge).acceptProb
            (fun actual => decide (guess g = actual)) := hunion
    _ =
        Finset.univ.sum fun _g : Guess => n32ChallengeGuessBound := by
          apply Finset.sum_congr rfl
          intro g _hg
          simpa [eq_comm] using n32_single_challenge_guess_acceptProb (guess g)
    _ =
        (Fintype.card Guess : ℝ) * n32ChallengeGuessBound := by
          simp [Finset.sum_const, nsmul_eq_mul]

set_option linter.constructorNameAsVariable false in
/--
If the hidden challenge is sampled independently and uniformly after the
adversary-side state is fixed, one state-dependent blind guess hits with
probability exactly `2^-133`.
-/
theorem n32_product_blind_guess_acceptProb
    {σ : Type*} [Fintype σ]
    (Dσ : FiniteProbability σ)
    (guess : σ -> N32CanonicalChallenge) :
    (productFiniteProbability Dσ
        (uniformFiniteProbability N32CanonicalChallenge)).acceptProb
        (fun trace => decide (trace.2 = guess trace.1)) =
      n32ChallengeGuessBound := by
  unfold FiniteProbability.acceptProb productFiniteProbability
  rw [Fintype.sum_prod_type]
  calc
    (Finset.univ.sum fun state =>
        Finset.univ.sum fun actual =>
          Dσ.mass state *
              (uniformFiniteProbability N32CanonicalChallenge).mass actual *
            boolIndicator (decide (actual = guess state))) =
        Finset.univ.sum fun state =>
          Dσ.mass state *
            ((uniformFiniteProbability N32CanonicalChallenge).acceptProb
              (fun actual => decide (actual = guess state))) := by
          apply Finset.sum_congr rfl
          intro state _hstate
          unfold FiniteProbability.acceptProb
          rw [Finset.mul_sum]
          apply Finset.sum_congr rfl
          intro actual _hactual
          ring
    _ = Finset.univ.sum fun state => Dσ.mass state * n32ChallengeGuessBound := by
          apply Finset.sum_congr rfl
          intro state _h
          rw [n32_single_challenge_guess_acceptProb]
    _ = (Finset.univ.sum fun state => Dσ.mass state) *
        n32ChallengeGuessBound := by
          rw [Finset.sum_mul]
    _ = n32ChallengeGuessBound := by
          rw [Dσ.total_mass]
          ring

theorem boolIndicator_add_not (A : Bool) :
    boolIndicator A + boolIndicator (!A) = 1 := by
  cases A <;> simp [boolIndicator]

theorem acceptProb_add_not
    {Ω : Type*} [Fintype Ω]
    (D : FiniteProbability Ω) (A : Ω -> Bool) :
    D.acceptProb A + D.acceptProb (fun ω => !(A ω)) = 1 := by
  unfold FiniteProbability.acceptProb
  calc
    (Finset.univ.sum fun ω => D.mass ω * boolIndicator (A ω)) +
        (Finset.univ.sum fun ω => D.mass ω * boolIndicator (!A ω)) =
        Finset.univ.sum fun ω =>
          D.mass ω * boolIndicator (A ω) +
            D.mass ω * boolIndicator (!A ω) := by
          rw [Finset.sum_add_distrib]
    _ = Finset.univ.sum fun ω => D.mass ω := by
          apply Finset.sum_congr rfl
          intro ω _hω
          rw [← mul_add, boolIndicator_add_not, mul_one]
    _ = 1 := D.total_mass

theorem acceptProb_not_eq_one_sub
    {Ω : Type*} [Fintype Ω]
    (D : FiniteProbability Ω) (A : Ω -> Bool) :
    D.acceptProb (fun ω => !(A ω)) = 1 - D.acceptProb A := by
  have h := acceptProb_add_not D A
  linarith

set_option linter.constructorNameAsVariable false in
/--
If every adversary-side state accepts at least an `alpha` fraction of canonical
challenges, then an independently sampled programmed challenge misses the
state's accepting set with probability at most `1 - alpha`.
-/
theorem n32_product_acceptFailure_le_one_sub_lower
    {σ : Type*} [Fintype σ]
    (Dσ : FiniteProbability σ)
    (accepts : σ -> N32CanonicalChallenge -> Bool)
    (alpha : ℝ)
    (hα : ∀ state,
      alpha <= (uniformFiniteProbability N32CanonicalChallenge).acceptProb
        (accepts state)) :
    (productFiniteProbability Dσ
        (uniformFiniteProbability N32CanonicalChallenge)).acceptProb
        (fun trace => !(accepts trace.1 trace.2)) <= 1 - alpha := by
  unfold FiniteProbability.acceptProb productFiniteProbability
  rw [Fintype.sum_prod_type]
  calc
    (Finset.univ.sum fun state =>
        Finset.univ.sum fun actual =>
          Dσ.mass state *
              (uniformFiniteProbability N32CanonicalChallenge).mass actual *
            boolIndicator (!(accepts state actual))) =
        Finset.univ.sum fun state =>
          Dσ.mass state *
            ((uniformFiniteProbability N32CanonicalChallenge).acceptProb
              (fun actual => !(accepts state actual))) := by
          apply Finset.sum_congr rfl
          intro state _hstate
          unfold FiniteProbability.acceptProb
          rw [Finset.mul_sum]
          apply Finset.sum_congr rfl
          intro actual _hactual
          ring
    _ <= Finset.univ.sum fun state => Dσ.mass state * (1 - alpha) := by
          apply Finset.sum_le_sum
          intro state _hstate
          have hnot :
              (uniformFiniteProbability N32CanonicalChallenge).acceptProb
                  (fun actual => !(accepts state actual)) <= 1 - alpha := by
            rw [acceptProb_not_eq_one_sub]
            linarith [hα state]
          exact mul_le_mul_of_nonneg_left hnot (Dσ.nonnegative state)
    _ = (Finset.univ.sum fun state => Dσ.mass state) * (1 - alpha) := by
          rw [Finset.sum_mul]
    _ = 1 - alpha := by
          rw [Dσ.total_mass]
          ring

/-- State-local accepting-challenge density under the canonical challenge law. -/
def n32StateAcceptProbability
    {σ : Type*}
    (accepts : σ -> N32CanonicalChallenge -> Bool)
    (state : σ) : ℝ :=
  (uniformFiniteProbability N32CanonicalChallenge).acceptProb
    (accepts state)

/-- Six independently programmed replay challenges. -/
abbrev N32ReplayChallengeVector :=
  N32ReplayDraw -> N32CanonicalChallenge

abbrev N32ReplayRoundVector :=
  N32ReplayDraw -> N32RoundChallenge

abbrev N32ReplayRoundMatrix :=
  N32Repetition -> N32ReplayRoundVector

/-- Replay trace after the adversary state has been cut at the challenge point. -/
abbrev N32SixReplayTrace (State : Type*) :=
  State × N32ReplayChallengeVector

/-- Independent uniform canonical challenges for the six replay draws. -/
def n32ReplayChallengeVectorProbability :
    FiniteProbability N32ReplayChallengeVector :=
  functionProductFiniteProbability
    (fun _d : N32ReplayDraw =>
      uniformFiniteProbability N32CanonicalChallenge)

def n32ReplayRoundVectorProbability :
    FiniteProbability N32ReplayRoundVector :=
  functionProductFiniteProbability
    (fun _d : N32ReplayDraw =>
      uniformFiniteProbability N32RoundChallenge)

def n32ReplayRoundMatrixProbability :
    FiniteProbability N32ReplayRoundMatrix :=
  functionProductFiniteProbability
    (fun _r : N32Repetition => n32ReplayRoundVectorProbability)

def transposeReplayChallenges
    (challenges : N32ReplayChallengeVector) : N32ReplayRoundMatrix :=
  fun r d => challenges d r

def untransposeReplayChallenges
    (matrix : N32ReplayRoundMatrix) : N32ReplayChallengeVector :=
  fun d r => matrix r d

def replayChallengeTransposeEquiv :
    N32ReplayChallengeVector ≃ N32ReplayRoundMatrix where
  toFun := transposeReplayChallenges
  invFun := untransposeReplayChallenges
  left_inv := by intro challenges; rfl
  right_inv := by intro matrix; rfl

theorem n32ReplayChallengeVector_mass_eq_transpose
    (challenges : N32ReplayChallengeVector) :
    n32ReplayChallengeVectorProbability.mass challenges =
      n32ReplayRoundMatrixProbability.mass
        (transposeReplayChallenges challenges) := by
  unfold n32ReplayChallengeVectorProbability n32ReplayRoundMatrixProbability
    n32ReplayRoundVectorProbability transposeReplayChallenges
    functionProductFiniteProbability
  simp only
  simp_rw [n32CanonicalChallenge_uniform_mass_eq_round_product]
  exact Finset.prod_comm

theorem n32ReplayChallengeVector_mass_untranspose
    (matrix : N32ReplayRoundMatrix) :
    n32ReplayChallengeVectorProbability.mass
        (untransposeReplayChallenges matrix) =
      n32ReplayRoundMatrixProbability.mass matrix := by
  simpa [replayChallengeTransposeEquiv, transposeReplayChallenges,
    untransposeReplayChallenges] using
    n32ReplayChallengeVector_mass_eq_transpose
      (untransposeReplayChallenges matrix)

set_option linter.constructorNameAsVariable false in
theorem n32ReplayChallengeVector_acceptProb_transpose
    (event : N32ReplayRoundMatrix -> Bool) :
    n32ReplayChallengeVectorProbability.acceptProb
        (fun challenges =>
          event (transposeReplayChallenges challenges)) =
      n32ReplayRoundMatrixProbability.acceptProb event := by
  unfold FiniteProbability.acceptProb
  calc
    (Finset.univ.sum fun challenges : N32ReplayChallengeVector =>
        n32ReplayChallengeVectorProbability.mass challenges *
          boolIndicator (event (transposeReplayChallenges challenges))) =
        Finset.univ.sum fun matrix : N32ReplayRoundMatrix =>
          n32ReplayChallengeVectorProbability.mass
              (untransposeReplayChallenges matrix) *
            boolIndicator (event matrix) := by
          refine Fintype.sum_equiv replayChallengeTransposeEquiv _ _ ?_
          intro challenges
          rfl
    _ =
        Finset.univ.sum fun matrix : N32ReplayRoundMatrix =>
          n32ReplayRoundMatrixProbability.mass matrix *
            boolIndicator (event matrix) := by
          apply Finset.sum_congr rfl
          intro matrix _hmatrix
          rw [n32ReplayChallengeVector_mass_untranspose]

/-- State distribution times six independent programmed challenge draws. -/
def n32SixReplayTraceDist
    {State : Type*} [Fintype State]
    (Dσ : FiniteProbability State) :
    FiniteProbability (N32SixReplayTrace State) :=
  productFiniteProbability Dσ n32ReplayChallengeVectorProbability

/-- All six replay draws accept under their independently programmed challenges. -/
def n32SixReplayAllAccepts
    {State : Type*}
    (accepts :
      State -> N32ReplayDraw -> N32CanonicalChallenge -> Bool)
    (τ : N32SixReplayTrace State) : Bool :=
  @decide (∀ d : N32ReplayDraw, accepts τ.1 d (τ.2 d) = true)
    Fintype.decidableForallFintype

set_option linter.constructorNameAsVariable false in
/--
The mean state-local accepting density is exactly the one-shot accepting
probability under the product experiment `state × canonical challenge`.
-/
theorem n32_product_acceptProb_eq_state_acceptProbability
    {σ : Type*} [Fintype σ]
    (Dσ : FiniteProbability σ)
    (accepts : σ -> N32CanonicalChallenge -> Bool) :
    (productFiniteProbability Dσ
        (uniformFiniteProbability N32CanonicalChallenge)).acceptProb
        (fun trace => accepts trace.1 trace.2) =
      Finset.univ.sum fun state =>
        Dσ.mass state * n32StateAcceptProbability accepts state := by
  unfold FiniteProbability.acceptProb productFiniteProbability n32StateAcceptProbability
  rw [Fintype.sum_prod_type]
  calc
    (Finset.univ.sum fun state =>
        Finset.univ.sum fun actual =>
          Dσ.mass state *
              (uniformFiniteProbability N32CanonicalChallenge).mass actual *
            boolIndicator (accepts state actual)) =
        Finset.univ.sum fun state =>
          Dσ.mass state *
            (Finset.univ.sum fun actual =>
              (uniformFiniteProbability N32CanonicalChallenge).mass actual *
                boolIndicator (accepts state actual)) := by
          apply Finset.sum_congr rfl
          intro state _hstate
          rw [Finset.mul_sum]
          apply Finset.sum_congr rfl
          intro actual _hactual
          ring
    _ = Finset.univ.sum fun state =>
          Dσ.mass state *
            (Finset.univ.sum fun actual =>
              (uniformFiniteProbability N32CanonicalChallenge).mass actual *
                boolIndicator (accepts state actual)) := rfl

set_option linter.constructorNameAsVariable false in
/--
First-run bridge from forgery success to average state-local accepting density.

If every successful forgery outside `queryMiss` is accepted by the one-shot
programmed challenge test, then

`Pr[forgery] - Pr[queryMiss] <= E_state[p[state]]`.
-/
theorem n32_stateAcceptMean_ge_forgery_sub_queryMiss
    {σ : Type*} [Fintype σ]
    (Dσ : FiniteProbability σ)
    (accepts : σ -> N32CanonicalChallenge -> Bool)
    (forgery queryMiss : σ × N32CanonicalChallenge -> Bool)
    (hgood :
      ∀ trace,
        forgery trace = true ->
        queryMiss trace = false ->
        accepts trace.1 trace.2 = true) :
    (productFiniteProbability Dσ
        (uniformFiniteProbability N32CanonicalChallenge)).acceptProb forgery -
      (productFiniteProbability Dσ
        (uniformFiniteProbability N32CanonicalChallenge)).acceptProb queryMiss <=
      Finset.univ.sum fun state =>
        Dσ.mass state * n32StateAcceptProbability accepts state := by
  let D :=
    productFiniteProbability Dσ
      (uniformFiniteProbability N32CanonicalChallenge)
  have hsub :
      D.acceptProb forgery - D.acceptProb queryMiss <=
        D.acceptProb (fun trace => forgery trace && !queryMiss trace) :=
    sub_acceptProb_le_and_not D forgery queryMiss
  have hmono :
      D.acceptProb (fun trace => forgery trace && !queryMiss trace) <=
        D.acceptProb (fun trace => accepts trace.1 trace.2) :=
    acceptProb_mono D (fun trace htrace => by
      cases hforgery : forgery trace <;>
        cases hquery : queryMiss trace <;>
        simp [hforgery, hquery] at htrace ⊢
      exact hgood trace hforgery hquery)
  have heq :
      D.acceptProb (fun trace => accepts trace.1 trace.2) =
        Finset.univ.sum fun state =>
          Dσ.mass state * n32StateAcceptProbability accepts state := by
    simpa [D] using
      n32_product_acceptProb_eq_state_acceptProbability Dσ accepts
  linarith

set_option linter.constructorNameAsVariable false in
/--
N32 six-draw moment lower bound from any nonnegative lower bound on first-run
state-local accepting density.
-/
theorem n32_six_replay_identical_acceptProb_ge_lowerBound_sixth
    {State : Type*} [Fintype State]
    (Dσ : FiniteProbability State)
    (accepts : State -> N32CanonicalChallenge -> Bool)
    (lowerBound : ℝ)
    (hlower_nonneg : 0 <= lowerBound)
    (hlower :
      lowerBound <=
        Finset.univ.sum fun state =>
          Dσ.mass state * n32StateAcceptProbability accepts state) :
    lowerBound ^ 6 <=
      (n32SixReplayTraceDist Dσ).acceptProb
        (n32SixReplayAllAccepts
          (fun state _d c => accepts state c)) := by
  have hmoment :=
    productFunctionProduct_identical_acceptProb_ge_mean_pow
      (Draw := N32ReplayDraw)
      Dσ (uniformFiniteProbability N32CanonicalChallenge) accepts
  have hpow :
      lowerBound ^ 6 <=
        (Finset.univ.sum fun state =>
          Dσ.mass state * n32StateAcceptProbability accepts state) ^ 6 := by
    exact pow_le_pow_left₀ hlower_nonneg hlower 6
  have hmoment6 :
      (Finset.univ.sum fun state =>
          Dσ.mass state * n32StateAcceptProbability accepts state) ^ 6 <=
        (n32SixReplayTraceDist Dσ).acceptProb
          (n32SixReplayAllAccepts
            (fun state _d c => accepts state c)) := by
    simpa [n32SixReplayTraceDist, n32ReplayChallengeVectorProbability,
      n32SixReplayAllAccepts, n32StateAcceptProbability, n32ReplayDraw_card]
      using hmoment
  exact le_trans hpow hmoment6

/--
Threshold-heavy states for a replay/forgery predicate.

A state is `alpha`-heavy when an independently programmed canonical challenge
accepts with probability at least `alpha`.
-/
def n32StateHeavy
    {σ : Type*}
    (accepts : σ -> N32CanonicalChallenge -> Bool)
    (alpha : ℝ)
    (state : σ) : Bool :=
  decide (alpha <= n32StateAcceptProbability accepts state)

theorem n32StateHeavy_acceptProbability_lower
    {σ : Type*}
    (accepts : σ -> N32CanonicalChallenge -> Bool)
    (alpha : ℝ) {state : σ}
    (hheavy : n32StateHeavy accepts alpha state = true) :
    alpha <= n32StateAcceptProbability accepts state := by
  simpa [n32StateHeavy] using hheavy

set_option linter.constructorNameAsVariable false in
/--
If `heavy` is the canonical threshold predicate for `accepts`, then accepting
on a light state has probability at most the threshold.

This is the first formal source of an `alpha`: global forger success only
becomes a per-state replay density after the proof either conditions on heavy
states or pays this light-state term.
-/
theorem n32_product_light_acceptProb_le_threshold
    {σ : Type*} [Fintype σ]
    (Dσ : FiniteProbability σ)
    (accepts : σ -> N32CanonicalChallenge -> Bool)
    (alpha : ℝ)
    (halpha_nonneg : 0 <= alpha) :
    (productFiniteProbability Dσ
        (uniformFiniteProbability N32CanonicalChallenge)).acceptProb
        (fun trace =>
          !(n32StateHeavy accepts alpha trace.1) && accepts trace.1 trace.2) <=
      alpha := by
  unfold FiniteProbability.acceptProb productFiniteProbability
  rw [Fintype.sum_prod_type]
  calc
    (Finset.univ.sum fun state =>
        Finset.univ.sum fun actual =>
          Dσ.mass state *
              (uniformFiniteProbability N32CanonicalChallenge).mass actual *
            boolIndicator
              (!(n32StateHeavy accepts alpha state) && accepts state actual)) =
        Finset.univ.sum fun state =>
          Dσ.mass state *
            ((uniformFiniteProbability N32CanonicalChallenge).acceptProb
              (fun actual =>
                !(n32StateHeavy accepts alpha state) && accepts state actual)) := by
          apply Finset.sum_congr rfl
          intro state _hstate
          unfold FiniteProbability.acceptProb
          rw [Finset.mul_sum]
          apply Finset.sum_congr rfl
          intro actual _hactual
          ring
    _ <= Finset.univ.sum fun state => Dσ.mass state * alpha := by
          apply Finset.sum_le_sum
          intro state _hstate
          cases hheavy : n32StateHeavy accepts alpha state
          · have hnot :
                ¬ alpha <= n32StateAcceptProbability accepts state := by
              simpa [n32StateHeavy] using hheavy
            have hlight :
                (uniformFiniteProbability N32CanonicalChallenge).acceptProb
                    (fun actual => accepts state actual) <= alpha := by
              have hlt : n32StateAcceptProbability accepts state < alpha :=
                lt_of_not_ge hnot
              unfold n32StateAcceptProbability at hlt
              linarith
            simpa [hheavy] using
              mul_le_mul_of_nonneg_left hlight (Dσ.nonnegative state)
          · have hfalse :
                (uniformFiniteProbability N32CanonicalChallenge).acceptProb
                    (fun _actual : N32CanonicalChallenge => false) = 0 := by
              unfold FiniteProbability.acceptProb
              simp
            simpa [hheavy, hfalse] using
              mul_nonneg (Dσ.nonnegative state) halpha_nonneg
    _ = (Finset.univ.sum fun state => Dσ.mass state) * alpha := by
          rw [Finset.sum_mul]
    _ = alpha := by
          rw [Dσ.total_mass]
          ring

set_option linter.constructorNameAsVariable false in
/--
Heavy-state accepting failure bound.

Only heavy states are charged to this event.  Light states must be paid for by
a separate light-state replay/forgery term.
-/
theorem n32_product_heavy_acceptFailure_le_one_sub_lower
    {σ : Type*} [Fintype σ]
    (Dσ : FiniteProbability σ)
    (heavy : σ -> Bool)
    (accepts : σ -> N32CanonicalChallenge -> Bool)
    (alpha : ℝ)
    (halpha_le_one : alpha <= 1)
    (hα : ∀ state,
      heavy state = true ->
      alpha <= (uniformFiniteProbability N32CanonicalChallenge).acceptProb
        (accepts state)) :
    (productFiniteProbability Dσ
        (uniformFiniteProbability N32CanonicalChallenge)).acceptProb
        (fun trace => heavy trace.1 && !(accepts trace.1 trace.2)) <=
      1 - alpha := by
  let acceptsOrLight : σ -> N32CanonicalChallenge -> Bool :=
    fun state actual => !(heavy state) || accepts state actual
  have hlower : ∀ state,
      alpha <= (uniformFiniteProbability N32CanonicalChallenge).acceptProb
        (acceptsOrLight state) := by
    intro state
    cases hheavy : heavy state
    · unfold acceptsOrLight
      have hall :
          (fun actual : N32CanonicalChallenge =>
            !(heavy state) || accepts state actual) =
          (fun _actual : N32CanonicalChallenge => true) := by
        funext actual
        simp [hheavy]
      rw [hall]
      have htrue :
          (uniformFiniteProbability N32CanonicalChallenge).acceptProb
              (fun _actual => true) = 1 := by
        unfold FiniteProbability.acceptProb
        simp [uniformFiniteProbability, Finset.sum_const, nsmul_eq_mul]
      rw [htrue]
      exact halpha_le_one
    · unfold acceptsOrLight
      have hsame :
          (fun actual : N32CanonicalChallenge =>
            !(heavy state) || accepts state actual) =
          (fun actual : N32CanonicalChallenge => accepts state actual) := by
        funext actual
        simp [hheavy]
      rw [hsame]
      exact hα state hheavy
  have hbase :=
    n32_product_acceptFailure_le_one_sub_lower
      Dσ acceptsOrLight alpha hlower
  have hevent :
      (fun trace : σ × N32CanonicalChallenge =>
        !(acceptsOrLight trace.1 trace.2)) =
      (fun trace => heavy trace.1 && !(accepts trace.1 trace.2)) := by
    funext trace
    cases hheavy : heavy trace.1 <;>
      cases hacc : accepts trace.1 trace.2 <;>
      simp [acceptsOrLight, hheavy, hacc]
  rw [hevent] at hbase
  exact hbase

/--
The byte-level challenge point that must be hashed before sampling the
canonical product-block challenge.

`Commitment` is intentionally abstract here: the serialized verifier theorem
will later instantiate it with the concrete commitment bytes for the exact
sampler.
-/
structure N32ChallengePoint
    (PublicKey Message Commitment : Type*) where
  publicKey : PublicKey
  message : Message
  commitments : N32Repetition -> Commitment
  deriving DecidableEq

/--
One accepting Fiat-Shamir transcript for the canonical challenge space.

The response type remains abstract in this file; the serialized verifier layer
will instantiate it with compact active-only response bytes.
-/
structure N32AcceptingTranscript
    (PublicKey Message Commitment Response : Type*) where
  point : N32ChallengePoint PublicKey Message Commitment
  challenge : N32CanonicalChallenge
  response : N32Repetition -> Response

/-- Six replay transcripts bound to the same challenge point. -/
structure N32SixDrawReplay
    (PublicKey Message Commitment Response : Type*) where
  point : N32ChallengePoint PublicKey Message Commitment
  draw : N32ReplayDraw ->
    N32AcceptingTranscript PublicKey Message Commitment Response
  same_point : ∀ d, (draw d).point = point

/-- One serialized byte. -/
abbrev Byte := Fin 256

/-- Fixed-length byte vector. -/
abbrev ByteVector (n : Nat) :=
  Fin n -> Byte

/-- Byte codec for the canonical N32 challenge carried in the signature. -/
structure N32CanonicalChallengeByteCodec where
  challengeLen : Nat
  encode : N32CanonicalChallenge -> ByteVector challengeLen
  decode? : ByteVector challengeLen -> Option N32CanonicalChallenge
  encode_injective : Function.Injective encode
  decode_encode : ∀ c, decode? (encode c) = some c

namespace N32CanonicalChallengeByteCodec

def challengeBits : Nat := 133

def packedChallengeBytes : Nat := 17

def packedChallengeBits : Nat := 8 * packedChallengeBytes

theorem packedChallengeBits_eq :
    packedChallengeBits = 136 := by
  native_decide

theorem slackBits_eq :
    packedChallengeBits - challengeBits = 3 := by
  native_decide

def productionRoundBits : Nat := 19

def productionOmittedBits : Nat := 4

def productionSignBits : Nat := 15

theorem productionRoundBits_eq :
    productionRoundBits = productionOmittedBits + productionSignBits := by
  rfl

theorem productionChallengeBits_eq :
    7 * productionRoundBits = challengeBits := by
  native_decide

theorem productionSlackBits_eq :
    packedChallengeBits - 7 * productionRoundBits = 3 := by
  native_decide

def productionSignRank (omitted block : N32Block) : Nat :=
  if block.1 < omitted.1 then block.1 else block.1 - 1

def productionSignBlock (omitted : N32Block) (rank : Nat) : Nat :=
  if rank < omitted.1 then rank else rank + 1

def productionRoundEncodedSignBit
    (round : N32RoundChallenge) (block : N32Block) : Bool :=
  if hactive : block ≠ round.omitted then
    !round.sign ⟨block, hactive⟩
  else
    false

theorem productionSignRank_lt
    {omitted block : N32Block}
    (hactive : block ≠ omitted) :
    productionSignRank omitted block < productionSignBits := by
  have hne : block.1 ≠ omitted.1 := by
    intro hval
    exact hactive (Fin.ext hval)
  by_cases hlt : block.1 < omitted.1
  · simp [productionSignRank, productionSignBits, hlt]
    omega
  · simp [productionSignRank, productionSignBits, hlt]
    omega

theorem productionSignBlock_rank
    {omitted block : N32Block}
    (hactive : block ≠ omitted) :
    productionSignBlock omitted (productionSignRank omitted block) =
      block.1 := by
  have hne : block.1 ≠ omitted.1 := by
    intro hval
    exact hactive (Fin.ext hval)
  by_cases hlt : block.1 < omitted.1
  · simp [productionSignBlock, productionSignRank, hlt]
  · have hgt : omitted.1 < block.1 := by omega
    have hnlt : ¬ block.1 - 1 < omitted.1 := by omega
    simp [productionSignBlock, productionSignRank, hlt, hnlt]
    omega

def productionChallengeBit (challenge : N32CanonicalChallenge) (bit : Nat) :
    Bool :=
  if _hbit : bit < challengeBits then
    let repIdx := bit / productionRoundBits
    let offset := bit % productionRoundBits
    if hrep : repIdx < 7 then
      let round := challenge ⟨repIdx, hrep⟩
      if _hoff : offset < productionOmittedBits then
        round.omitted.1.testBit offset
      else
        let rank := offset - productionOmittedBits
        let blockNat := productionSignBlock round.omitted rank
        if hblock : blockNat < 16 then
          let block : N32Block := ⟨blockNat, hblock⟩
          productionRoundEncodedSignBit round block
        else
          false
    else
      false
  else
    false

def productionPackedByte
    (bit : Nat -> Bool) (byteIdx : Nat) : Byte :=
  ⟨Nat.ofBits (fun k : Fin 8 => bit (byteIdx * 8 + k.1)),
    Nat.ofBits_lt_two_pow (fun k : Fin 8 => bit (byteIdx * 8 + k.1))⟩

def productionEncode (challenge : N32CanonicalChallenge) :
    ByteVector packedChallengeBytes :=
  fun i => productionPackedByte (productionChallengeBit challenge) i.1

def productionByteBitLE {n : Nat} (bytes : ByteVector n) (bit : Nat) :
    Nat :=
  let byteIdx := bit / 8
  let bitIdx := bit % 8
  let byteNat :=
    if h : byteIdx < n then
      (bytes ⟨byteIdx, h⟩).1
    else
      0
  (byteNat / 2 ^ bitIdx) % 2

def productionPackedBitsNatLE {n : Nat}
    (bytes : ByteVector n) (bitOffset width : Nat) : Nat :=
  ∑ i : Fin width,
    productionByteBitLE bytes (bitOffset + i.1) * 2 ^ i.1

theorem production_nat_div_pow_mod_two_eq_testBit_toNat (n i : Nat) :
    (n / 2 ^ i) % 2 = (n.testBit i).toNat := by
  rw [Nat.testBit, Nat.shiftRight_eq_div_pow, Nat.one_and_eq_mod_two]
  have hlt : (n / 2 ^ i) % 2 < 2 := Nat.mod_lt _ Nat.zero_lt_two
  interval_cases (n / 2 ^ i) % 2 <;> rfl

theorem production_sum_bool_bits_eq_ofBits (w : Nat) (f : Fin w -> Bool) :
    (∑ i : Fin w, (f i).toNat * 2 ^ i.1) = Nat.ofBits f := by
  induction w with
  | zero => simp
  | succ w ih =>
      rw [Nat.ofBits_succ]
      simp [Fin.sum_univ_succ]
      rw [← ih (f ∘ Fin.succ)]
      simp only [Function.comp_apply, pow_succ]
      have hsum :
          (∑ x : Fin w, (f x.succ).toNat * (2 ^ x.1 * 2)) =
            2 * ∑ x : Fin w, (f x.succ).toNat * 2 ^ x.1 := by
        rw [Finset.mul_sum]
        apply Finset.sum_congr rfl
        intro x _hx
        ring
      rw [hsum]
      ring

theorem productionByteBitLE_encode
    (challenge : N32CanonicalChallenge)
    {bit : Nat}
    (hbit : bit < packedChallengeBits) :
    productionByteBitLE (productionEncode challenge) bit =
      (productionChallengeBit challenge bit).toNat := by
  unfold productionByteBitLE productionEncode productionPackedByte
  have hbyte : bit / 8 < packedChallengeBytes := by
    have hbyte17 : bit / 8 < 17 := by
      exact Nat.div_lt_of_lt_mul
        (by simpa [packedChallengeBits, packedChallengeBytes] using hbit)
    simpa [packedChallengeBytes] using hbyte17
  simp [hbyte]
  rw [production_nat_div_pow_mod_two_eq_testBit_toNat]
  rw [Nat.testBit_ofBits_lt _ _ (Nat.mod_lt _ (by decide : 0 < 8))]
  have hidx : bit / 8 * 8 + bit % 8 = bit := by
    rw [Nat.mul_comm (bit / 8) 8]
    exact Nat.div_add_mod bit 8
  simp [hidx]

theorem productionChallengeBit_omitted
    (challenge : N32CanonicalChallenge)
    (rep : N32Repetition)
    (k : Fin productionOmittedBits) :
    productionChallengeBit challenge
        (rep.1 * productionRoundBits + k.1) =
      ((challenge rep).omitted.1.testBit k.1) := by
  unfold productionChallengeBit
  have hk4 : k.1 < 4 := by
    change k.1 < 4
    exact k.2
  have hk19Nat : k.1 < 19 := by
    omega
  have hk19 : k.1 < productionRoundBits := by
    simpa [productionRoundBits] using hk19Nat
  have hbitNat :
      rep.1 * 19 + k.1 < 133 := by
    have hrep := rep.2
    omega
  have hbit :
      rep.1 * productionRoundBits + k.1 < challengeBits := by
    simpa [productionRoundBits, challengeBits] using hbitNat
  have hdiv :
      (rep.1 * productionRoundBits + k.1) / productionRoundBits =
        rep.1 := by
    change (rep.1 * 19 + k.1) / 19 = rep.1
    rw [Nat.mul_comm rep.1 19, Nat.mul_add_div (by decide : 0 < 19)]
    rw [Nat.div_eq_of_lt hk19Nat, Nat.add_zero]
  have hmod :
      (rep.1 * productionRoundBits + k.1) % productionRoundBits =
        k.1 := by
    change (rep.1 * 19 + k.1) % 19 = k.1
    rw [Nat.mul_comm rep.1 19, Nat.mul_add_mod]
    exact Nat.mod_eq_of_lt hk19Nat
  have hoff : k.1 < productionOmittedBits := k.2
  simp [hbit, hdiv, hmod, rep.2, hoff]

theorem productionPackedBitsNatLE_omitted_encode
    (challenge : N32CanonicalChallenge)
    (rep : N32Repetition) :
    productionPackedBitsNatLE (productionEncode challenge)
        (rep.1 * productionRoundBits) productionOmittedBits =
      (challenge rep).omitted.1 := by
  unfold productionPackedBitsNatLE
  calc
    (∑ i : Fin productionOmittedBits,
        productionByteBitLE (productionEncode challenge)
            (rep.1 * productionRoundBits + i.1) * 2 ^ i.1) =
        ∑ i : Fin productionOmittedBits,
          ((challenge rep).omitted.1.testBit i.1).toNat * 2 ^ i.1 := by
          apply Finset.sum_congr rfl
          intro i _hi
          rw [productionByteBitLE_encode]
          · rw [productionChallengeBit_omitted]
          · unfold packedChallengeBits productionRoundBits productionOmittedBits
            have hrep := rep.2
            have hi : i.1 < 4 := by
              change i.1 < 4
              exact i.2
            change rep.1 * 19 + i.1 < 136
            omega
    _ = Nat.ofBits (fun i : Fin productionOmittedBits =>
          (challenge rep).omitted.1.testBit i.1) := by
          rw [production_sum_bool_bits_eq_ofBits]
    _ = (challenge rep).omitted.1 := by
          rw [Nat.ofBits_testBit]
          exact Nat.mod_eq_of_lt (challenge rep).omitted.2

theorem productionChallengeBit_sign
    (challenge : N32CanonicalChallenge)
    (rep : N32Repetition)
    {block : N32Block}
    (hactive : block ≠ (challenge rep).omitted) :
    productionChallengeBit challenge
        (rep.1 * productionRoundBits + productionOmittedBits +
          productionSignRank (challenge rep).omitted block) =
      !((challenge rep).sign ⟨block, hactive⟩) := by
  let rank := productionSignRank (challenge rep).omitted block
  change productionChallengeBit challenge
      (rep.1 * productionRoundBits + productionOmittedBits + rank) =
    !((challenge rep).sign ⟨block, hactive⟩)
  unfold productionChallengeBit
  have hrank : rank < productionSignBits :=
    productionSignRank_lt hactive
  have hrankNat : rank < 15 := by
    simpa [productionSignBits] using hrank
  have hrank19Nat : 4 + rank < 19 := by
    omega
  have hrank19 : productionOmittedBits + rank < productionRoundBits := by
    simpa [productionOmittedBits, productionRoundBits] using hrank19Nat
  have hbitNat :
      rep.1 * 19 + 4 + rank < 133 := by
    have hrep := rep.2
    omega
  have hbit :
      rep.1 * productionRoundBits + productionOmittedBits + rank <
        challengeBits := by
    simpa [productionRoundBits, productionOmittedBits, challengeBits]
      using hbitNat
  have hdiv :
      (rep.1 * productionRoundBits + productionOmittedBits + rank) /
          productionRoundBits =
        rep.1 := by
    change (rep.1 * 19 + 4 + rank) / 19 = rep.1
    rw [Nat.add_assoc]
    rw [Nat.mul_comm rep.1 19, Nat.mul_add_div (by decide : 0 < 19)]
    rw [Nat.div_eq_of_lt hrank19Nat, Nat.add_zero]
  have hrepEq :
      (⟨(rep.1 * productionRoundBits + productionOmittedBits + rank) /
          productionRoundBits, by
            rw [hdiv]
            exact rep.2⟩ : N32Repetition) = rep := by
    exact Fin.ext hdiv
  have hrepIdxLt :
      (rep.1 * productionRoundBits + productionOmittedBits + rank) /
          productionRoundBits < 7 := by
    rw [hdiv]
    exact rep.2
  have hmod :
      (rep.1 * productionRoundBits + productionOmittedBits + rank) %
          productionRoundBits =
        productionOmittedBits + rank := by
    change (rep.1 * 19 + 4 + rank) % 19 = 4 + rank
    rw [Nat.add_assoc]
    rw [Nat.mul_comm rep.1 19, Nat.mul_add_mod]
    exact Nat.mod_eq_of_lt hrank19Nat
  have hoff : ¬ productionOmittedBits + rank < productionOmittedBits := by
    omega
  have hsub : productionOmittedBits + rank - productionOmittedBits = rank := by
    omega
  have hblockNat :
      productionSignBlock (challenge rep).omitted rank = block.1 := by
    simpa [rank] using productionSignBlock_rank hactive
  have hblockLt :
      productionSignBlock (challenge rep).omitted rank < 16 := by
    rw [hblockNat]
    exact block.2
  have hblockEq :
      (⟨productionSignBlock (challenge rep).omitted rank, hblockLt⟩ :
          N32Block) = block := by
    exact Fin.ext hblockNat
  have hchallengeEq
      (hlt :
        (rep.1 * productionRoundBits + productionOmittedBits + rank) /
            productionRoundBits < 7) :
      challenge
          ⟨(rep.1 * productionRoundBits + productionOmittedBits + rank) /
              productionRoundBits, hlt⟩ =
        challenge rep := by
    exact congrArg challenge (Fin.ext hdiv)
  simp [hbit, hrepIdxLt, hrepEq, hmod, hoff, hsub, hblockLt, hblockEq]
  simp [productionRoundEncodedSignBit, hactive]

def productionSlackClear (bytes : ByteVector packedChallengeBytes) : Bool :=
  decide (productionByteBitLE bytes challengeBits = 0) &&
    decide (productionByteBitLE bytes (challengeBits + 1) = 0) &&
    decide (productionByteBitLE bytes (challengeBits + 2) = 0)

def productionDecodeRound
    (bytes : ByteVector packedChallengeBytes)
    (rep : N32Repetition) : N32RoundChallenge :=
  let omittedVal :=
    productionPackedBitsNatLE bytes (rep.1 * productionRoundBits)
      productionOmittedBits % 16
  let omitted : N32Block := ⟨omittedVal, Nat.mod_lt _ (by decide)⟩
  ⟨omitted, fun block =>
    productionByteBitLE bytes
        (rep.1 * productionRoundBits + productionOmittedBits +
          productionSignRank omitted block.1) = 0⟩

def productionDecode? (bytes : ByteVector packedChallengeBytes) :
    Option N32CanonicalChallenge :=
  if productionSlackClear bytes then
    some (fun rep => productionDecodeRound bytes rep)
  else
    none

theorem productionSlackClear_encode
    (challenge : N32CanonicalChallenge) :
    productionSlackClear (productionEncode challenge) = true := by
  unfold productionSlackClear
  have h0 :
      productionByteBitLE (productionEncode challenge) challengeBits = 0 := by
    rw [productionByteBitLE_encode]
    · simp [productionChallengeBit]
    · norm_num [packedChallengeBits, packedChallengeBytes, challengeBits]
  have h1 :
      productionByteBitLE (productionEncode challenge) (challengeBits + 1) =
        0 := by
    rw [productionByteBitLE_encode]
    · simp [productionChallengeBit]
    · norm_num [packedChallengeBits, packedChallengeBytes, challengeBits]
  have h2 :
      productionByteBitLE (productionEncode challenge) (challengeBits + 2) =
        0 := by
    rw [productionByteBitLE_encode]
    · simp [productionChallengeBit]
    · norm_num [packedChallengeBits, packedChallengeBytes, challengeBits]
  simp [h0, h1, h2]

theorem productionDecodeRound_encode
    (challenge : N32CanonicalChallenge)
    (rep : N32Repetition) :
    productionDecodeRound (productionEncode challenge) rep =
      challenge rep := by
  unfold productionDecodeRound
  cases hround : challenge rep with
  | mk omitted signs =>
      have homitted :
          productionPackedBitsNatLE (productionEncode challenge)
              (rep.1 * productionRoundBits) productionOmittedBits % 16 =
            omitted.1 := by
        have h :=
          productionPackedBitsNatLE_omitted_encode challenge rep
        simp [hround] at h
        rw [h]
        exact Nat.mod_eq_of_lt omitted.2
      have hdecodedOmitted :
          (⟨productionPackedBitsNatLE (productionEncode challenge)
              (rep.1 * productionRoundBits) productionOmittedBits % 16,
            Nat.mod_lt _ (by decide)⟩ : N32Block) = omitted :=
        Fin.ext homitted
      simp [homitted]
      have hdomain :
          N32ActiveBlock omitted =
            N32ActiveBlock
              (⟨productionPackedBitsNatLE (productionEncode challenge)
                  (rep.1 * productionRoundBits) productionOmittedBits % 16,
                Nat.mod_lt _ (by decide)⟩ : N32Block) :=
        congrArg N32ActiveBlock hdecodedOmitted.symm
      refine Function.hfunext hdomain.symm ?_
      intro block block' hblock
      apply heq_of_eq
      have hp :
          ∀ b : N32Block,
            b ≠
                (⟨productionPackedBitsNatLE (productionEncode challenge)
                    (rep.1 * productionRoundBits) productionOmittedBits % 16,
                  Nat.mod_lt _ (by decide)⟩ : N32Block) ↔
              b ≠ omitted := by
        intro b
        rw [hdecodedOmitted]
      have hval : (block : N32Block) = (block' : N32Block) :=
        (Subtype.heq_iff_coe_eq hp).mp hblock
      have hbit' :
          productionByteBitLE (productionEncode challenge)
              (rep.1 * productionRoundBits + productionOmittedBits +
                productionSignRank omitted block'.1) =
            (!(signs block')).toNat := by
        rw [productionByteBitLE_encode]
        · have hactive' : block'.1 ≠ (challenge rep).omitted := by
            simpa [hround] using block'.2
          have hsign :=
            productionChallengeBit_sign challenge rep
              (block := block'.1) hactive'
          have hsignNat := congrArg Bool.toNat hsign
          have hfst : (challenge rep).omitted = omitted :=
            (Sigma.mk.inj_iff.mp hround).1
          have hsnd : (challenge rep).snd ≍ signs :=
            (Sigma.mk.inj_iff.mp hround).2
          have hpred :
              ∀ b : N32Block,
                b ≠ (challenge rep).omitted ↔ b ≠ omitted := by
            intro b
            rw [hfst]
          have harg :
              (⟨block'.1, hactive'⟩ :
                N32ActiveBlock (challenge rep).omitted) ≍ block' := by
            exact (Subtype.heq_iff_coe_eq hpred).2 rfl
          have hsignVal :
              (challenge rep).sign ⟨block'.1, hactive'⟩ =
                signs block' :=
            congr_heq hsnd harg
          have hnotNat :=
            congrArg (fun b : Bool => (!b).toNat) hsignVal
          simpa [hfst] using hsignNat.trans hnotNat
        · have hrank := productionSignRank_lt block'.2
          have hrankNat : productionSignRank omitted block'.1 < 15 := by
            simpa [productionSignBits] using hrank
          have hrep := rep.2
          change rep.1 * 19 + 4 + productionSignRank omitted block'.1 < 136
          omega
      have hbit :
          productionByteBitLE (productionEncode challenge)
              (rep.1 * productionRoundBits + productionOmittedBits +
                productionSignRank omitted block.1) =
            (!(signs block')).toNat := by
        simpa [hval] using hbit'
      cases hsign : signs block' <;>
        simp [hbit, hsign]

theorem productionDecode_encode
    (challenge : N32CanonicalChallenge) :
    productionDecode? (productionEncode challenge) = some challenge := by
  unfold productionDecode?
  rw [productionSlackClear_encode]
  simp
  funext rep
  exact productionDecodeRound_encode challenge rep

theorem productionEncode_injective :
    Function.Injective productionEncode := by
  intro c d h
  have hc := productionDecode_encode c
  have hd := productionDecode_encode d
  rw [h] at hc
  exact Option.some.inj (hc.symm.trans hd)

def productionBitLayout17 : N32CanonicalChallengeByteCodec where
  challengeLen := packedChallengeBytes
  encode := productionEncode
  decode? := productionDecode?
  encode_injective := productionEncode_injective
  decode_encode := productionDecode_encode

theorem productionBitLayout17_challengeLen_eq :
    productionBitLayout17.challengeLen = 17 := by
  rfl

theorem byteVector17_card :
    Fintype.card (ByteVector packedChallengeBytes) = 2 ^ 136 := by
  norm_num [packedChallengeBytes, ByteVector, Byte, Fintype.card_fun]

theorem challengeCodeSpace_le_byteSpace :
    2 ^ challengeBits <= 2 ^ 136 := by
  native_decide

noncomputable def challengeEquivFin :
    N32CanonicalChallenge ≃ Fin (2 ^ challengeBits) :=
  Fintype.equivFinOfCardEq (by
    change Fintype.card N32CanonicalChallenge = 2 ^ 133
    exact n32CanonicalChallenge_card)

noncomputable def byteVector17EquivFin :
    ByteVector packedChallengeBytes ≃ Fin (2 ^ 136) :=
  Fintype.equivFinOfCardEq byteVector17_card

/--
Packed 17-byte challenge encoding.

This is the proof-facing canonical packing witness: the `2^133` challenge
codes occupy the lower part of a `2^136` byte space, leaving three slack bits.
-/
noncomputable def packedEncode
    (c : N32CanonicalChallenge) : ByteVector packedChallengeBytes :=
  byteVector17EquivFin.symm
    (Fin.castLE challengeCodeSpace_le_byteSpace (challengeEquivFin c))

noncomputable def packedDecode?
    (bytes : ByteVector packedChallengeBytes) :
    Option N32CanonicalChallenge :=
  let i := byteVector17EquivFin bytes
  if h : (i : Nat) < 2 ^ challengeBits then
    some (challengeEquivFin.symm ⟨i.val, h⟩)
  else
    none

theorem packedEncode_injective :
    Function.Injective packedEncode := by
  intro c d h
  unfold packedEncode at h
  have hfin := congrArg byteVector17EquivFin h
  simp only [Equiv.apply_symm_apply] at hfin
  have hidx :
      Fin.castLE challengeCodeSpace_le_byteSpace (challengeEquivFin c) =
        Fin.castLE challengeCodeSpace_le_byteSpace (challengeEquivFin d) :=
    hfin
  have hchallenge :=
    Fin.castLE_injective challengeCodeSpace_le_byteSpace hidx
  exact challengeEquivFin.injective hchallenge

theorem packedDecode_encode
    (c : N32CanonicalChallenge) :
    packedDecode? (packedEncode c) = some c := by
  unfold packedDecode? packedEncode
  simp only [Equiv.apply_symm_apply]
  rw [dif_pos]
  · simp
  · exact (challengeEquivFin c).isLt

noncomputable def packed17 : N32CanonicalChallengeByteCodec where
  challengeLen := packedChallengeBytes
  encode := packedEncode
  decode? := packedDecode?
  encode_injective := packedEncode_injective
  decode_encode := packedDecode_encode

theorem packed17_challengeLen_eq :
    packed17.challengeLen = 17 := by
  rfl

end N32CanonicalChallengeByteCodec

def bytePackedLength (entries bits : Nat) : Nat :=
  (entries * bits + 7) / 8

/-- Byte accounting certificate for a compact active-only N32 signature row. -/
structure N32CompactSignatureByteProfile where
  localFreeCount : Nat
  responseBitsPerCoordinate : Nat
  challengeBytes : Nat
  responseCoordinatesPerRepetition : Nat
  responseBytesPerRepetition : Nat
  signatureBytes : Nat
  challengeBytes_eq :
    challengeBytes = N32CanonicalChallengeByteCodec.packedChallengeBytes
  responseCoordinates_eq :
    responseCoordinatesPerRepetition =
      n32ActiveBlocksPerRepetition * localFreeCount
  responseBytes_eq :
    responseBytesPerRepetition =
      bytePackedLength responseCoordinatesPerRepetition
        responseBitsPerCoordinate
  signatureBytes_eq :
    signatureBytes =
      challengeBytes +
        Fintype.card N32Repetition * responseBytesPerRepetition

namespace N32CompactSignatureByteProfile

def leadingN32_9bit_2free : N32CompactSignatureByteProfile where
  localFreeCount := 2
  responseBitsPerCoordinate := 9
  challengeBytes := 17
  responseCoordinatesPerRepetition := 30
  responseBytesPerRepetition := 34
  signatureBytes := 255
  challengeBytes_eq := by native_decide
  responseCoordinates_eq := by native_decide
  responseBytes_eq := by native_decide
  signatureBytes_eq := by native_decide

theorem leading_responseCoordinatesPerRepetition_eq :
    leadingN32_9bit_2free.responseCoordinatesPerRepetition = 30 := by
  rfl

theorem leading_responseBytesPerRepetition_eq :
    leadingN32_9bit_2free.responseBytesPerRepetition = 34 := by
  rfl

theorem leading_signatureBytes_eq :
    leadingN32_9bit_2free.signatureBytes = 255 := by
  rfl

end N32CompactSignatureByteProfile

/--
Byte accounting certificate for the implicit-commitment completed-response
route.

Unlike `N32CompactSignatureByteProfile`, this profile serializes free response
coordinates for every block in every repetition.  The verifier can then
reconstruct the whole Fiat-Shamir commitment as `A*z - chi*u`, so no explicit
commitment bytes are counted.
-/
structure N32CompletedResponseSignatureByteProfile where
  localFreeCount : Nat
  responseBitsPerCoordinate : Nat
  challengeBytes : Nat
  responseCoordinatesPerRepetition : Nat
  responseBytesPerRepetition : Nat
  signatureBytes : Nat
  challengeBytes_eq :
    challengeBytes = N32CanonicalChallengeByteCodec.packedChallengeBytes
  responseCoordinates_eq :
    responseCoordinatesPerRepetition =
      Fintype.card N32Block * localFreeCount
  responseBytes_eq :
    responseBytesPerRepetition =
      bytePackedLength responseCoordinatesPerRepetition
        responseBitsPerCoordinate
  signatureBytes_eq :
    signatureBytes =
      challengeBytes +
        Fintype.card N32Repetition * responseBytesPerRepetition

namespace N32CompletedResponseSignatureByteProfile

def leadingN32_9bit_2free_implicitCommitment :
    N32CompletedResponseSignatureByteProfile where
  localFreeCount := 2
  responseBitsPerCoordinate := 9
  challengeBytes := 17
  responseCoordinatesPerRepetition := 32
  responseBytesPerRepetition := 36
  signatureBytes := 269
  challengeBytes_eq := by native_decide
  responseCoordinates_eq := by native_decide
  responseBytes_eq := by native_decide
  signatureBytes_eq := by native_decide

theorem leading_responseCoordinatesPerRepetition_eq :
    leadingN32_9bit_2free_implicitCommitment.responseCoordinatesPerRepetition =
      32 := by
  rfl

theorem leading_responseBytesPerRepetition_eq :
    leadingN32_9bit_2free_implicitCommitment.responseBytesPerRepetition =
      36 := by
  rfl

theorem leading_signatureBytes_eq :
    leadingN32_9bit_2free_implicitCommitment.signatureBytes = 269 := by
  rfl

end N32CompletedResponseSignatureByteProfile

/--
Structured fixed-layout bytes for the challenge point.

The frame records a domain tag, public-key bytes, message bytes, and the seven
commitment byte strings in repetition order.  A concrete implementation can
flatten this frame into a byte string with fixed offsets.
-/
structure N32ChallengePointByteFrame
    (domainLen publicKeyLen messageLen commitmentLen : Nat) where
  domainTag : ByteVector domainLen
  publicKey : ByteVector publicKeyLen
  message : ByteVector messageLen
  commitments : N32Repetition -> ByteVector commitmentLen

/--
Byte codec for the challenge point.

The proof obligation is exactly the byte-level binding property: the byte
encodings of public keys, messages, and commitments are injective inside this
fixed-layout, domain-separated frame.
-/
structure N32ChallengePointByteCodec
    (PublicKey Message Commitment : Type*) where
  domainLen : Nat
  publicKeyLen : Nat
  messageLen : Nat
  commitmentLen : Nat
  domainTag : ByteVector domainLen
  encodePublicKey : PublicKey -> ByteVector publicKeyLen
  encodeMessage : Message -> ByteVector messageLen
  encodeCommitment : Commitment -> ByteVector commitmentLen
  encodePublicKey_injective : Function.Injective encodePublicKey
  encodeMessage_injective : Function.Injective encodeMessage
  encodeCommitment_injective : Function.Injective encodeCommitment

namespace N32ChallengePointByteCodec

def encodePoint
    {PublicKey Message Commitment : Type*}
    (C : N32ChallengePointByteCodec PublicKey Message Commitment)
    (p : N32ChallengePoint PublicKey Message Commitment) :
    N32ChallengePointByteFrame C.domainLen C.publicKeyLen
      C.messageLen C.commitmentLen where
  domainTag := C.domainTag
  publicKey := C.encodePublicKey p.publicKey
  message := C.encodeMessage p.message
  commitments := fun r => C.encodeCommitment (p.commitments r)

theorem encodePoint_injective
    {PublicKey Message Commitment : Type*}
    (C : N32ChallengePointByteCodec PublicKey Message Commitment) :
    Function.Injective C.encodePoint := by
  intro p q h
  cases p with
  | mk ppk pmsg pcom =>
  cases q with
  | mk qpk qmsg qcom =>
    simp [encodePoint] at h ⊢
    constructor
    · exact C.encodePublicKey_injective h.1
    · constructor
      · exact C.encodeMessage_injective h.2.1
      · funext r
        exact C.encodeCommitment_injective (congrFun h.2.2 r)

/-- Concrete byte-vector challenge-point codec for already-serialized scheme objects. -/
def byteVector
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (domainTag : ByteVector domainLen) :
    N32ChallengePointByteCodec
      (ByteVector publicKeyLen) (ByteVector messageLen)
      (ByteVector commitmentLen) where
  domainLen := domainLen
  publicKeyLen := publicKeyLen
  messageLen := messageLen
  commitmentLen := commitmentLen
  domainTag := domainTag
  encodePublicKey := id
  encodeMessage := id
  encodeCommitment := id
  encodePublicKey_injective := by
    intro x y h
    exact h
  encodeMessage_injective := by
    intro x y h
    exact h
  encodeCommitment_injective := by
    intro x y h
    exact h

theorem byteVector_encodePoint_injective
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (domainTag : ByteVector domainLen) :
    Function.Injective
      (byteVector
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        domainTag).encodePoint :=
  (byteVector
    (publicKeyLen := publicKeyLen)
    (messageLen := messageLen)
    (commitmentLen := commitmentLen)
    domainTag).encodePoint_injective

end N32ChallengePointByteCodec

/-- The transcript sample space for query-miss analysis. -/
abbrev N32QueryTrace (State : Type*) :=
  State × N32CanonicalChallenge

/--
The random-oracle query-log experiment for one challenge point.

The state contains every adversary-side random choice before the hidden
challenge is sampled.  The actual challenge is then sampled independently and
uniformly from `N32CanonicalChallenge`.
-/
structure N32QueryLogExperiment
    (State QueryIdx PublicKey Message Commitment : Type*)
    [Fintype State] where
  stateDist : FiniteProbability State
  outputPoint : State -> N32ChallengePoint PublicKey Message Commitment
  outputChallenge : State -> N32CanonicalChallenge
  queryPoint : State -> QueryIdx -> N32ChallengePoint PublicKey Message Commitment

namespace N32QueryLogExperiment

variable {State QueryIdx PublicKey Message Commitment : Type*}
variable [Fintype State]

def traceDist
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment) :
    FiniteProbability (N32QueryTrace State) :=
  productFiniteProbability E.stateDist
    (uniformFiniteProbability N32CanonicalChallenge)

def actualChallenge
    (_E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (τ : N32QueryTrace State) : N32CanonicalChallenge :=
  τ.2

def blindChallenge
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (τ : N32QueryTrace State) : N32CanonicalChallenge :=
  E.outputChallenge τ.1

def blindGuessEvent
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (τ : N32QueryTrace State) : Bool :=
  decide (E.actualChallenge τ = E.blindChallenge τ)

def pointQueried
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (τ : N32QueryTrace State) : Bool :=
  decide (∃ i, E.queryPoint τ.1 i = E.outputPoint τ.1)

theorem blindGuessEvent_acceptProb
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment) :
    E.traceDist.acceptProb E.blindGuessEvent = n32ChallengeGuessBound := by
  simpa [traceDist, blindGuessEvent, actualChallenge, blindChallenge] using
    n32_product_blind_guess_acceptProb E.stateDist E.outputChallenge

/--
Transcript-side certificate for query miss.

If the challenge point was not queried, a successful accepting forgery must
have guessed the hidden challenge from adversary-side state alone.
-/
structure QueryMissCertificate
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (queryMiss : N32QueryTrace State -> Bool) where
  queryMiss_not_queried :
    ∀ τ, queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ

namespace QueryMissCertificate

variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {queryMiss : N32QueryTrace State -> Bool}

theorem queryMiss_acceptProb_le
    (C : QueryMissCertificate E queryMiss) :
    E.traceDist.acceptProb queryMiss <= n32ChallengeGuessBound := by
  have hmono :
      E.traceDist.acceptProb queryMiss <=
        E.traceDist.acceptProb E.blindGuessEvent :=
    acceptProb_mono E.traceDist (fun τ hmiss => by
      have hblind := C.queryMiss_outputs_blind τ hmiss
      simpa [N32QueryLogExperiment.blindGuessEvent,
        N32QueryLogExperiment.actualChallenge,
        N32QueryLogExperiment.blindChallenge] using hblind)
  have hguess := E.blindGuessEvent_acceptProb
  linarith

end QueryMissCertificate

end N32QueryLogExperiment

/--
Serialized verifier boundary for the first-run accepting transcript.

The verifier is modeled at the transcript layer: it parses a public key,
message, seven commitments, the canonical challenge, and the compact response,
then accepts or rejects.  The byte-frame field is the concrete serialization
obligation: on an accepting transcript, the verifier's challenge-point bytes
must be exactly the fixed-layout encoding of the semantic point.  The challenge
recompute field is the Fiat-Shamir obligation: an accepting transcript's
challenge is the random-oracle value bound to that point.
-/
structure N32SerializedVerifier
    {State QueryIdx PublicKey Message Commitment Response : Type*}
    [Fintype State]
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    where
  codec : N32ChallengePointByteCodec PublicKey Message Commitment
  challengeCodec : N32CanonicalChallengeByteCodec
  response : State -> N32Repetition -> Response
  acceptsTranscript :
    N32AcceptingTranscript PublicKey Message Commitment Response -> Bool
  verifierPointFrame :
    State ->
      N32ChallengePointByteFrame codec.domainLen codec.publicKeyLen
        codec.messageLen codec.commitmentLen
  verifierPointFrame_matches_output :
    ∀ τ : N32QueryTrace State,
      acceptsTranscript
          { point := E.outputPoint τ.1
            challenge := E.outputChallenge τ.1
            response := response τ.1 } = true ->
        verifierPointFrame τ.1 = codec.encodePoint (E.outputPoint τ.1)
  recomputedChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      acceptsTranscript
          { point := E.outputPoint τ.1
            challenge := E.outputChallenge τ.1
            response := response τ.1 } = true ->
        E.outputChallenge τ.1 = E.actualChallenge τ

namespace N32SerializedVerifier

variable {State QueryIdx PublicKey Message Commitment Response : Type*}
variable [Fintype State]
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}

def outputTranscript
    (V : N32SerializedVerifier (Response := Response) E)
    (τ : N32QueryTrace State) :
    N32AcceptingTranscript PublicKey Message Commitment Response where
  point := E.outputPoint τ.1
  challenge := E.outputChallenge τ.1
  response := V.response τ.1

def programmedTranscript
    (V : N32SerializedVerifier (Response := Response) E)
    (state : State)
    (challenge : N32CanonicalChallenge) :
    N32AcceptingTranscript PublicKey Message Commitment Response where
  point := E.outputPoint state
  challenge := challenge
  response := V.response state

def outputAccepts
    (V : N32SerializedVerifier (Response := Response) E)
    (τ : N32QueryTrace State) : Bool :=
  V.acceptsTranscript (V.outputTranscript τ)

def outputChallengeBytes
    (V : N32SerializedVerifier (Response := Response) E)
    (τ : N32QueryTrace State) : ByteVector V.challengeCodec.challengeLen :=
  V.challengeCodec.encode (E.outputChallenge τ.1)

theorem outputChallengeBytes_decode
    (V : N32SerializedVerifier (Response := Response) E)
    (τ : N32QueryTrace State) :
    V.challengeCodec.decode? (V.outputChallengeBytes τ) =
      some (E.outputChallenge τ.1) :=
  V.challengeCodec.decode_encode (E.outputChallenge τ.1)

/--
The one-shot accepting set used by the density and product-moment arguments.

For a fixed adversary-side state, this predicate asks whether the serialized
verifier would accept after programming the challenge point to `challenge`.
-/
def oneShotAccepts
    (V : N32SerializedVerifier (Response := Response) E)
    (state : State)
    (challenge : N32CanonicalChallenge) : Bool :=
  V.acceptsTranscript (V.programmedTranscript state challenge)

theorem oneShotAccepts_of_outputAccepts
    (V : N32SerializedVerifier (Response := Response) E)
    (τ : N32QueryTrace State)
    (haccept : V.outputAccepts τ = true) :
    V.oneShotAccepts τ.1 τ.2 = true := by
  have hchallenge :=
    V.recomputedChallenge_matches_actual τ
      (by
        simpa [outputAccepts, outputTranscript] using haccept)
  simpa [oneShotAccepts, programmedTranscript, outputAccepts,
    outputTranscript, N32QueryLogExperiment.actualChallenge, hchallenge]
    using haccept

end N32SerializedVerifier

/--
Concrete serialized-verifier inputs for a byte-vector implementation.

Here the public key, message, commitments, and responses are already fixed
byte vectors.  The challenge-point codec is the identity codec on those byte
vectors, and the challenge codec is the canonical 17-byte N32 packing.
-/
structure N32ByteSerializedVerifierInputs
    {State QueryIdx : Type*}
    [Fintype State]
    (domainLen publicKeyLen messageLen commitmentLen responseLen : Nat)
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)) where
  domainTag : ByteVector domainLen
  response : State -> N32Repetition -> ByteVector responseLen
  acceptsTranscript :
    N32AcceptingTranscript
      (ByteVector publicKeyLen) (ByteVector messageLen)
      (ByteVector commitmentLen) (ByteVector responseLen) -> Bool
  verifierPointFrame :
    State ->
      N32ChallengePointByteFrame domainLen publicKeyLen
        messageLen commitmentLen
  verifierPointFrame_matches_output :
    ∀ τ : N32QueryTrace State,
      acceptsTranscript
          { point := E.outputPoint τ.1
            challenge := E.outputChallenge τ.1
            response := response τ.1 } = true ->
        verifierPointFrame τ.1 =
          (N32ChallengePointByteCodec.byteVector domainTag).encodePoint
            (E.outputPoint τ.1)
  recomputedChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      acceptsTranscript
          { point := E.outputPoint τ.1
            challenge := E.outputChallenge τ.1
            response := response τ.1 } = true ->
        E.outputChallenge τ.1 = E.actualChallenge τ

namespace N32ByteSerializedVerifierInputs

variable {State QueryIdx : Type*}
variable [Fintype State]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}

noncomputable def toSerializedVerifier
    (V :
      N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
        commitmentLen responseLen E) :
    N32SerializedVerifier (Response := ByteVector responseLen) E where
  codec := N32ChallengePointByteCodec.byteVector V.domainTag
  challengeCodec := N32CanonicalChallengeByteCodec.packed17
  response := V.response
  acceptsTranscript := V.acceptsTranscript
  verifierPointFrame := V.verifierPointFrame
  verifierPointFrame_matches_output := V.verifierPointFrame_matches_output
  recomputedChallenge_matches_actual := V.recomputedChallenge_matches_actual

theorem toSerializedVerifier_challengeLen_eq
    (V :
      N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
        commitmentLen responseLen E) :
    V.toSerializedVerifier.challengeCodec.challengeLen = 17 := by
  rfl

end N32ByteSerializedVerifierInputs

/--
Implicit-commitment byte verifier boundary.

This is the proof-facing shape of the concrete verifier whose signature carries
only a challenge and completed response bytes.  The verifier reconstructs the
commitment bytes from `(publicKey, challenge, response)`, checks that those
bytes are the Fiat-Shamir challenge point commitments, and then recomputes the
canonical challenge from the fixed byte frame.

The field `corePayloadCheck` is where concrete parsing and range checks live:
challenge canonicality, public-key q-coordinate canonicality, signed response
decoding, and response bounds.  The hash function itself is represented by
`recomputeChallenge`; the oracle agreement field is the standard query-log
obligation saying that, for accepted core payloads whose reconstructed
commitments are the output point, this byte frame is bound to the hidden
challenge.
-/
structure N32ImplicitCommitmentByteVerifierInputs
    {State QueryIdx : Type*}
    [Fintype State]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)) where
  domainTag : ByteVector domainLen
  response : State -> N32Repetition -> ByteVector responseLen
  corePayloadCheck :
    N32ChallengePoint
      (ByteVector publicKeyLen) (ByteVector messageLen)
      (ByteVector commitmentLen) ->
      N32CanonicalChallenge ->
      (N32Repetition -> ByteVector responseLen) -> Bool
  reconstructCommitment :
    ByteVector publicKeyLen ->
      N32CanonicalChallenge ->
      (N32Repetition -> ByteVector responseLen) ->
      N32Repetition -> ByteVector commitmentLen
  recomputeChallenge :
    N32ChallengePointByteFrame domainLen publicKeyLen messageLen
      commitmentLen ->
      N32CanonicalChallenge
  oracleChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      corePayloadCheck
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (response τ.1) = true ->
        (∀ r,
          reconstructCommitment
              (E.outputPoint τ.1).publicKey
              (E.outputChallenge τ.1) (response τ.1) r =
            (E.outputPoint τ.1).commitments r) ->
        recomputeChallenge
            ((N32ChallengePointByteCodec.byteVector domainTag).encodePoint
              (E.outputPoint τ.1)) =
          E.actualChallenge τ

namespace N32ImplicitCommitmentByteVerifierInputs

variable {State QueryIdx : Type*}
variable [Fintype State]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}

def reconstructedPoint
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    (point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (challenge : N32CanonicalChallenge)
    (response : N32Repetition -> ByteVector responseLen) :
    N32ChallengePoint
      (ByteVector publicKeyLen) (ByteVector messageLen)
      (ByteVector commitmentLen) where
  publicKey := point.publicKey
  message := point.message
  commitments :=
    I.reconstructCommitment point.publicKey challenge response

def challengePointFrame
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    (point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (challenge : N32CanonicalChallenge)
    (response : N32Repetition -> ByteVector responseLen) :
    N32ChallengePointByteFrame domainLen publicKeyLen messageLen
      commitmentLen :=
  (N32ChallengePointByteCodec.byteVector I.domainTag).encodePoint
    (I.reconstructedPoint point challenge response)

def commitmentsMatch
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    (point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (challenge : N32CanonicalChallenge)
    (response : N32Repetition -> ByteVector responseLen) :
    Bool :=
  decide
    (∀ r,
      I.reconstructCommitment point.publicKey challenge response r =
        point.commitments r)

def acceptsTranscript
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    (transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen)) :
    Bool :=
  I.corePayloadCheck transcript.point transcript.challenge
      transcript.response &&
    I.commitmentsMatch transcript.point transcript.challenge
      transcript.response &&
    decide
      (I.recomputeChallenge
          (I.challengePointFrame transcript.point transcript.challenge
            transcript.response) =
        transcript.challenge)

theorem corePayloadCheck_of_acceptsTranscript
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    {transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen)}
    (haccept : I.acceptsTranscript transcript = true) :
    I.corePayloadCheck transcript.point transcript.challenge
      transcript.response = true := by
  have h :
      (I.corePayloadCheck transcript.point transcript.challenge
            transcript.response = true ∧
        I.commitmentsMatch transcript.point transcript.challenge
            transcript.response = true) ∧
          decide
            (I.recomputeChallenge
                (I.challengePointFrame transcript.point transcript.challenge
                  transcript.response) =
              transcript.challenge) = true := by
    simpa only [acceptsTranscript, Bool.and_eq_true] using haccept
  exact h.1.1

theorem commitments_eq_of_acceptsTranscript
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    {transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen)}
    (haccept : I.acceptsTranscript transcript = true) :
    ∀ r,
      I.reconstructCommitment transcript.point.publicKey transcript.challenge
          transcript.response r =
        transcript.point.commitments r := by
  have hmatch :
      I.commitmentsMatch transcript.point transcript.challenge
        transcript.response = true := by
    have h :
        (I.corePayloadCheck transcript.point transcript.challenge
              transcript.response = true ∧
          I.commitmentsMatch transcript.point transcript.challenge
              transcript.response = true) ∧
            decide
              (I.recomputeChallenge
                  (I.challengePointFrame transcript.point transcript.challenge
                    transcript.response) =
                transcript.challenge) = true := by
      simpa only [acceptsTranscript, Bool.and_eq_true] using haccept
    exact h.1.2
  unfold commitmentsMatch at hmatch
  exact of_decide_eq_true hmatch

theorem recomputeChallenge_eq_of_acceptsTranscript
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    {transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen)}
    (haccept : I.acceptsTranscript transcript = true) :
    I.recomputeChallenge
        (I.challengePointFrame transcript.point transcript.challenge
          transcript.response) =
      transcript.challenge := by
  have hchallenge :
      decide
          (I.recomputeChallenge
              (I.challengePointFrame transcript.point transcript.challenge
                transcript.response) =
            transcript.challenge) = true := by
    have h :
        (I.corePayloadCheck transcript.point transcript.challenge
              transcript.response = true ∧
          I.commitmentsMatch transcript.point transcript.challenge
              transcript.response = true) ∧
            decide
              (I.recomputeChallenge
                  (I.challengePointFrame transcript.point transcript.challenge
                    transcript.response) =
                transcript.challenge) = true := by
      simpa only [acceptsTranscript, Bool.and_eq_true] using haccept
    exact h.2
  exact of_decide_eq_true hchallenge

theorem challengePointFrame_eq_encodePoint_of_commitments_eq
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    {point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)}
    {challenge : N32CanonicalChallenge}
    {response : N32Repetition -> ByteVector responseLen}
    (hcommit :
      ∀ r,
        I.reconstructCommitment point.publicKey challenge response r =
          point.commitments r) :
    I.challengePointFrame point challenge response =
      (N32ChallengePointByteCodec.byteVector I.domainTag).encodePoint
        point := by
  cases point with
  | mk publicKey message commitments =>
    simp [challengePointFrame, reconstructedPoint,
      N32ChallengePointByteCodec.encodePoint,
      N32ChallengePointByteCodec.byteVector, hcommit]

noncomputable def toSerializedVerifierInputs
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E) :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E where
  domainTag := I.domainTag
  response := I.response
  acceptsTranscript := I.acceptsTranscript
  verifierPointFrame := fun state =>
    I.challengePointFrame (E.outputPoint state)
      (E.outputChallenge state) (I.response state)
  verifierPointFrame_matches_output := by
    intro τ haccept
    have hcommit :
        ∀ r,
          I.reconstructCommitment
              (E.outputPoint τ.1).publicKey
              (E.outputChallenge τ.1) (I.response τ.1) r =
            (E.outputPoint τ.1).commitments r :=
      I.commitments_eq_of_acceptsTranscript
        (transcript :=
          { point := E.outputPoint τ.1
            challenge := E.outputChallenge τ.1
            response := I.response τ.1 })
        (by
          simpa [acceptsTranscript] using haccept)
    exact I.challengePointFrame_eq_encodePoint_of_commitments_eq hcommit
  recomputedChallenge_matches_actual := by
    intro τ haccept
    let transcript :
        N32AcceptingTranscript
          (ByteVector publicKeyLen) (ByteVector messageLen)
          (ByteVector commitmentLen) (ByteVector responseLen) :=
      { point := E.outputPoint τ.1
        challenge := E.outputChallenge τ.1
        response := I.response τ.1 }
    have haccept' : I.acceptsTranscript transcript = true := by
      simpa [transcript, acceptsTranscript] using haccept
    have hcore :
        I.corePayloadCheck
            (E.outputPoint τ.1) (E.outputChallenge τ.1)
            (I.response τ.1) = true :=
      I.corePayloadCheck_of_acceptsTranscript
        (transcript := transcript) haccept'
    have hcommit :
        ∀ r,
          I.reconstructCommitment
              (E.outputPoint τ.1).publicKey
              (E.outputChallenge τ.1) (I.response τ.1) r =
            (E.outputPoint τ.1).commitments r :=
      I.commitments_eq_of_acceptsTranscript
        (transcript := transcript) haccept'
    have hframe :
        I.challengePointFrame (E.outputPoint τ.1)
            (E.outputChallenge τ.1) (I.response τ.1) =
          (N32ChallengePointByteCodec.byteVector I.domainTag).encodePoint
            (E.outputPoint τ.1) :=
      I.challengePointFrame_eq_encodePoint_of_commitments_eq hcommit
    have houtput :
        I.recomputeChallenge
            ((N32ChallengePointByteCodec.byteVector I.domainTag).encodePoint
              (E.outputPoint τ.1)) =
          E.outputChallenge τ.1 := by
      have h :=
        I.recomputeChallenge_eq_of_acceptsTranscript
          (transcript := transcript) haccept'
      simpa [transcript, hframe] using h
    have hactual :
        I.recomputeChallenge
            ((N32ChallengePointByteCodec.byteVector I.domainTag).encodePoint
              (E.outputPoint τ.1)) =
          E.actualChallenge τ :=
      I.oracleChallenge_matches_actual τ hcore hcommit
    exact houtput.symm.trans hactual

noncomputable def toSerializedVerifier
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E) :
    N32SerializedVerifier (Response := ByteVector responseLen) E :=
  I.toSerializedVerifierInputs.toSerializedVerifier

theorem outputAccepts_implies_outputChallenge_eq_actual
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    {τ : N32QueryTrace State}
    (haccept :
      I.toSerializedVerifier.outputAccepts τ = true) :
    E.outputChallenge τ.1 = E.actualChallenge τ := by
  exact
    I.toSerializedVerifier.recomputedChallenge_matches_actual τ
      (by
        simpa [toSerializedVerifier, N32SerializedVerifier.outputAccepts]
          using haccept)

theorem oneShotAccepts_of_outputAccepts
    (I :
      N32ImplicitCommitmentByteVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    (τ : N32QueryTrace State)
    (haccept :
      I.toSerializedVerifier.outputAccepts τ = true) :
    I.toSerializedVerifier.oneShotAccepts τ.1 τ.2 = true :=
  I.toSerializedVerifier.oneShotAccepts_of_outputAccepts τ haccept

end N32ImplicitCommitmentByteVerifierInputs

namespace N32Q12289ImplicitCommitmentByteVerifier

def domainBytes : Nat := 49
def publicKeyBytes : Nat := 172
def commitmentBytesPerRepetition : Nat := 140
def responseBytesPerRepetition : Nat := 36
def responseBound : Nat := 224

def serializedSignatureBytes : Nat :=
  N32CanonicalChallengeByteCodec.packedChallengeBytes +
    Fintype.card N32Repetition * responseBytesPerRepetition

theorem serializedSignatureBytes_eq :
    serializedSignatureBytes = 269 := by
  native_decide

theorem profile_signatureBytes_eq :
    serializedSignatureBytes =
      N32CompletedResponseSignatureByteProfile.leadingN32_9bit_2free_implicitCommitment.signatureBytes := by
  native_decide

abbrev Inputs
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)) :=
  N32ImplicitCommitmentByteVerifierInputs
    (domainLen := domainBytes)
    (publicKeyLen := publicKeyBytes)
    (messageLen := messageLen)
    (commitmentLen := commitmentBytesPerRepetition)
    (responseLen := responseBytesPerRepetition) E

end N32Q12289ImplicitCommitmentByteVerifier

/--
Serialized-verifier implication for the product-moment route.

This packages the concrete obligations that turn the verifier's byte-level
acceptance into the finite-probability accepting set `p[state]`.  The fields
say that the one-run Fiat-Shamir trace distribution is the query-log
distribution, query misses are blind challenge guesses, and every successful
forgery is accepted by the serialized verifier output transcript.
-/
structure N32SerializedVerifierProductMomentInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Response
      Round Block LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    {secret : Block × LocalFree -> ZMod q}
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (G :
      MultiForkFiatShamirGame
        (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret)
    (R : RandomOracleMultiForkReplay G) where
  traceDist_eq : G.traceDist = E.traceDist
  queryMiss_not_queried :
    ∀ τ, R.queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, R.queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ
  verifier : N32SerializedVerifier (Response := Response) E
  forgery_accepts :
    ∀ τ, R.forgery τ = true -> verifier.outputAccepts τ = true

namespace N32SerializedVerifierProductMomentInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Response
  Round Block LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {secret : Block × LocalFree -> ZMod q}
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {G :
  MultiForkFiatShamirGame
    (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret}
variable {R : RandomOracleMultiForkReplay G}

def accepts
    (I : N32SerializedVerifierProductMomentInputs (Response := Response) E G R)
    (state : State)
    (challenge : N32CanonicalChallenge) : Bool :=
  I.verifier.oneShotAccepts state challenge

def toQueryMissCertificate
    (I : N32SerializedVerifierProductMomentInputs (Response := Response) E G R) :
    N32QueryLogExperiment.QueryMissCertificate E R.queryMiss where
  queryMiss_not_queried := I.queryMiss_not_queried
  queryMiss_outputs_blind := I.queryMiss_outputs_blind

theorem forgery_implies_accepts
    (I : N32SerializedVerifierProductMomentInputs (Response := Response) E G R)
    (τ : N32QueryTrace State)
    (hforgery : R.forgery τ = true) :
    I.accepts τ.1 τ.2 = true := by
  exact
    I.verifier.oneShotAccepts_of_outputAccepts τ
      (I.forgery_accepts τ hforgery)

theorem queryMissProb_le_guess
    (I : N32SerializedVerifierProductMomentInputs (Response := Response) E G R) :
    R.queryMissProb <= n32ChallengeGuessBound := by
  unfold RandomOracleMultiForkReplay.queryMissProb
  rw [I.traceDist_eq]
  exact
    N32QueryLogExperiment.QueryMissCertificate.queryMiss_acceptProb_le
      (I.toQueryMissCertificate)

theorem stateAcceptMean_ge_forger_sub_queryMiss
    (I : N32SerializedVerifierProductMomentInputs (Response := Response) E G R) :
    R.forgerSuccessProb - R.queryMissProb <=
      Finset.univ.sum fun state =>
        E.stateDist.mass state *
          n32StateAcceptProbability I.accepts state := by
  have hbase :=
    n32_stateAcceptMean_ge_forgery_sub_queryMiss
      E.stateDist I.accepts R.forgery R.queryMiss
      (fun τ hforgery _hqueryMiss =>
        I.forgery_implies_accepts τ hforgery)
  unfold RandomOracleMultiForkReplay.forgerSuccessProb
    RandomOracleMultiForkReplay.queryMissProb
  rw [I.traceDist_eq]
  simpa [N32QueryLogExperiment.traceDist] using hbase

theorem stateAcceptMean_ge_forger_sub_guess
    (I : N32SerializedVerifierProductMomentInputs (Response := Response) E G R) :
    R.forgerSuccessProb - n32ChallengeGuessBound <=
      Finset.univ.sum fun state =>
        E.stateDist.mass state *
          n32StateAcceptProbability I.accepts state := by
  have hmean := I.stateAcceptMean_ge_forger_sub_queryMiss
  have hquery := I.queryMissProb_le_guess
  linarith

/--
Concrete serialized-verifier product-moment endpoint.

If `lowerBound` is any nonnegative value below the observed forgery probability
after the `2^-133` query-miss loss, then six independently programmed replays
all land in the same one-shot accepting set with probability at least
`lowerBound^6`.
-/
theorem sixReplayAllAccepts_ge_lowerBound_sixth
    (I : N32SerializedVerifierProductMomentInputs (Response := Response) E G R)
    (lowerBound : ℝ)
    (hlower_nonneg : 0 <= lowerBound)
    (hlower :
      lowerBound <= R.forgerSuccessProb - n32ChallengeGuessBound) :
    lowerBound ^ 6 <=
      (n32SixReplayTraceDist E.stateDist).acceptProb
        (n32SixReplayAllAccepts
          (fun state _d challenge => I.accepts state challenge)) := by
  have hmean := I.stateAcceptMean_ge_forger_sub_guess
  exact
    n32_six_replay_identical_acceptProb_ge_lowerBound_sixth
      E.stateDist I.accepts lowerBound hlower_nonneg
      (le_trans hlower hmean)

end N32SerializedVerifierProductMomentInputs

/--
Product-moment six-replay wrapper.

This is the non-additive replay route.  The first-run verifier proof supplies
the average one-shot accepting density.  Six independent random-oracle
programmed challenges then accept with probability at least the sixth power of
that density.  The final remaining transcript obligation is semantic: whenever
all six programmed one-shot transcripts accept, the multi-fork game has the
`acceptingForks` event needed by the deterministic extractor.
-/
structure N32SixReplayProductMomentForkInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Response
      Round Block LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    {secret : Block × LocalFree -> ZMod q}
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (G :
      MultiForkFiatShamirGame
        (N32SixReplayTrace State) Round N32ReplayDraw Block LocalFree secret)
    where
  traceDist_eq : G.traceDist = n32SixReplayTraceDist E.stateDist
  forgery : N32QueryTrace State -> Bool
  queryMiss : N32QueryTrace State -> Bool
  queryMiss_not_queried :
    ∀ τ, queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ
  verifier : N32SerializedVerifier (Response := Response) E
  forgery_accepts :
    ∀ τ, forgery τ = true -> verifier.outputAccepts τ = true
  acceptingForks_of_allAccepts :
    ∀ τ,
      n32SixReplayAllAccepts
          (fun state _d challenge => verifier.oneShotAccepts state challenge)
          τ = true ->
        G.acceptingForks τ = true
  coverageFailureBound : ℝ
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace N32SixReplayProductMomentForkInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Response
  Round Block LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {secret : Block × LocalFree -> ZMod q}
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {G :
  MultiForkFiatShamirGame
    (N32SixReplayTrace State) Round N32ReplayDraw Block LocalFree secret}

def accepts
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G)
    (state : State)
    (challenge : N32CanonicalChallenge) : Bool :=
  I.verifier.oneShotAccepts state challenge

def forgerSuccessProb
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

def queryMissProb
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G) :
    ℝ :=
  E.traceDist.acceptProb I.queryMiss

def toQueryMissCertificate
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G) :
    N32QueryLogExperiment.QueryMissCertificate E I.queryMiss where
  queryMiss_not_queried := I.queryMiss_not_queried
  queryMiss_outputs_blind := I.queryMiss_outputs_blind

theorem forgery_implies_accepts
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G)
    (τ : N32QueryTrace State)
    (hforgery : I.forgery τ = true) :
    I.accepts τ.1 τ.2 = true := by
  exact
    I.verifier.oneShotAccepts_of_outputAccepts τ
      (I.forgery_accepts τ hforgery)

theorem queryMissProb_le_guess
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G) :
    I.queryMissProb <= n32ChallengeGuessBound := by
  unfold queryMissProb
  exact
    N32QueryLogExperiment.QueryMissCertificate.queryMiss_acceptProb_le
      (I.toQueryMissCertificate)

theorem stateAcceptMean_ge_forger_sub_queryMiss
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G) :
    I.forgerSuccessProb - I.queryMissProb <=
      Finset.univ.sum fun state =>
        E.stateDist.mass state *
          n32StateAcceptProbability I.accepts state := by
  have hbase :=
    n32_stateAcceptMean_ge_forgery_sub_queryMiss
      E.stateDist I.accepts I.forgery I.queryMiss
      (fun τ hforgery _hqueryMiss =>
        I.forgery_implies_accepts τ hforgery)
  simpa [forgerSuccessProb, queryMissProb,
    N32QueryLogExperiment.traceDist] using hbase

theorem stateAcceptMean_ge_forger_sub_guess
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G) :
    I.forgerSuccessProb - n32ChallengeGuessBound <=
      Finset.univ.sum fun state =>
        E.stateDist.mass state *
          n32StateAcceptProbability I.accepts state := by
  have hmean := I.stateAcceptMean_ge_forger_sub_queryMiss
  have hquery := I.queryMissProb_le_guess
  linarith

theorem allAcceptsProb_ge_lowerBound_sixth
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G)
    (lowerBound : ℝ)
    (hlower_nonneg : 0 <= lowerBound)
    (hlower :
      lowerBound <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    lowerBound ^ 6 <=
      (n32SixReplayTraceDist E.stateDist).acceptProb
        (n32SixReplayAllAccepts
          (fun state _d challenge => I.accepts state challenge)) := by
  have hmean := I.stateAcceptMean_ge_forger_sub_guess
  exact
    n32_six_replay_identical_acceptProb_ge_lowerBound_sixth
      E.stateDist I.accepts lowerBound hlower_nonneg
      (le_trans hlower hmean)

theorem forkSuccessProb_ge_lowerBound_sixth
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G)
    (lowerBound : ℝ)
    (hlower_nonneg : 0 <= lowerBound)
    (hlower :
      lowerBound <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    lowerBound ^ 6 <= G.forkSuccessProb := by
  let allAccepts : N32SixReplayTrace State -> Bool :=
    n32SixReplayAllAccepts
      (fun state _d challenge => I.accepts state challenge)
  have hmoment :
      lowerBound ^ 6 <=
        (n32SixReplayTraceDist E.stateDist).acceptProb allAccepts := by
    exact I.allAcceptsProb_ge_lowerBound_sixth
      lowerBound hlower_nonneg hlower
  have hmono :
      (n32SixReplayTraceDist E.stateDist).acceptProb allAccepts <=
        (n32SixReplayTraceDist E.stateDist).acceptProb G.acceptingForks :=
    acceptProb_mono (n32SixReplayTraceDist E.stateDist)
      (fun τ hτ => I.acceptingForks_of_allAccepts τ (by
        simpa [allAccepts, accepts] using hτ))
  have hforkDist :
      G.forkSuccessProb =
        (n32SixReplayTraceDist E.stateDist).acceptProb G.acceptingForks := by
    unfold MultiForkFiatShamirGame.forkSuccessProb
    rw [I.traceDist_eq]
  linarith

/--
End-to-end product-moment extraction lower bound with only the compact
coverage-failure term subtracted after the sixth-power replay step.
-/
theorem extractableProb_ge_lowerBound_sixth
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G)
    (lowerBound : ℝ)
    (hlower_nonneg : 0 <= lowerBound)
    (hlower :
      lowerBound <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    lowerBound ^ 6 - I.coverageFailureBound <= G.extractableProb := by
  exact
    G.extractableProb_ge_of_forking_bounds
      (I.forkSuccessProb_ge_lowerBound_sixth
        lowerBound hlower_nonneg hlower)
      I.coverageFailure_upper

theorem extractableProb_ge_forgeryMinusGuess_sixth
    (I : N32SixReplayProductMomentForkInputs (Response := Response) E G)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
        I.coverageFailureBound <=
      G.extractableProb := by
  exact I.extractableProb_ge_lowerBound_sixth
    (I.forgerSuccessProb - n32ChallengeGuessBound) hnonneg le_rfl

end N32SixReplayProductMomentForkInputs

/--
Compact active-only response decoder for the N32 product-block transcript.

The serialized response for one repetition only needs coordinates whose
challenge scalar is a signed unit.  The decoder therefore takes an explicit
proof that the requested block is active/signed.
-/
structure N32CompactResponseCodec
    {q : Nat} [NeZero q]
    (Response LocalFree : Type*) where
  decode :
    Response -> (challenge : N32RoundChallenge) ->
      (b : N32Block) ->
        (challenge.scalar (q := q) b = 1 ∨
          challenge.scalar (q := q) b = -1) ->
          LocalFree -> ZMod q

/--
Byte-backed compact response decoder.

The response object is a fixed-length byte vector.  A concrete implementation
instantiates `decode` with its parser for active response coordinates.
-/
structure N32CompactResponseByteDecoder
    {q : Nat} [NeZero q]
    (LocalFree : Type*) where
  responseLen : Nat
  decode :
    ByteVector responseLen -> (challenge : N32RoundChallenge) ->
      (b : N32Block) ->
        (challenge.scalar (q := q) b = 1 ∨
          challenge.scalar (q := q) b = -1) ->
          LocalFree -> ZMod q

namespace N32CompactResponseByteDecoder

variable {q : Nat} [NeZero q]
variable {LocalFree : Type*}

def toCompactResponseCodec
    (D : N32CompactResponseByteDecoder (q := q) LocalFree) :
    N32CompactResponseCodec (q := q) (ByteVector D.responseLen)
      LocalFree where
  decode := D.decode

def leadingResponseLen : Nat := 34

theorem leadingResponseLen_eq_profile :
    leadingResponseLen =
      N32CompactSignatureByteProfile.leadingN32_9bit_2free.responseBytesPerRepetition := by
  rfl

end N32CompactResponseByteDecoder

/-- Fixed-length variant of the byte-backed compact response decoder. -/
structure N32FixedCompactResponseByteDecoder
    {q : Nat} [NeZero q]
    (responseLen : Nat) (LocalFree : Type*) where
  decode :
    ByteVector responseLen -> (challenge : N32RoundChallenge) ->
      (b : N32Block) ->
        (challenge.scalar (q := q) b = 1 ∨
          challenge.scalar (q := q) b = -1) ->
          LocalFree -> ZMod q

namespace N32FixedCompactResponseByteDecoder

variable {q : Nat} [NeZero q]
variable {responseLen : Nat}
variable {LocalFree : Type*}

def toCompactResponseCodec
    (D : N32FixedCompactResponseByteDecoder (q := q) responseLen LocalFree) :
    N32CompactResponseCodec (q := q) (ByteVector responseLen) LocalFree where
  decode := D.decode

end N32FixedCompactResponseByteDecoder

/--
Verifier soundness for compact product-block responses.

If the serialized verifier accepts a programmed challenge, then every active
decoded response coordinate satisfies the linear equation
`z = mask + chi * secret`.  This is the concrete proof obligation that connects
compact response bytes to the active-only deterministic extractor.
-/
structure N32CompactProductBlockVerifierSoundness
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Response LocalFree : Type*}
    [Fintype State]
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (V : N32SerializedVerifier (Response := Response) E)
    (secret : N32Block × LocalFree -> ZMod q) where
  responseCodec : N32CompactResponseCodec (q := q) Response LocalFree
  mask :
    State -> N32Repetition -> N32Block × LocalFree -> ZMod q
  accepted_response_eq :
    ∀ state challenge,
      V.oneShotAccepts state challenge = true ->
        ∀ r b hactive f,
            responseCodec.decode (V.response state r) (challenge r) b
              hactive f =
            SignatureDistribution.productBlockLinearResponse
              ((challenge r).scalar (q := q))
              (mask state r) secret (b, f)

/--
Byte-vector instantiation of compact response soundness.

This is the concrete response-byte boundary: accepted byte-vector responses
decode to active coordinates satisfying the linear verifier equation.
-/
structure N32ByteCompactProductBlockVerifierSoundnessInputs
    {q : Nat} [NeZero q]
    {State QueryIdx LocalFree : Type*}
    [Fintype State]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (V :
      N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
        commitmentLen responseLen E)
    (secret : N32Block × LocalFree -> ZMod q) where
  responseDecoder :
    N32FixedCompactResponseByteDecoder (q := q) responseLen LocalFree
  mask :
    State -> N32Repetition -> N32Block × LocalFree -> ZMod q
  accepted_response_eq :
    ∀ state challenge,
      V.toSerializedVerifier.oneShotAccepts state challenge = true ->
        ∀ r b hactive f,
            responseDecoder.decode (V.response state r) (challenge r) b
              hactive f =
            SignatureDistribution.productBlockLinearResponse
              ((challenge r).scalar (q := q))
              (mask state r) secret (b, f)

namespace N32ByteCompactProductBlockVerifierSoundnessInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx LocalFree : Type*}
variable [Fintype State]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {V :
  N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
    commitmentLen responseLen E}
variable {secret : N32Block × LocalFree -> ZMod q}

noncomputable def toCompactSoundness
    (S :
      N32ByteCompactProductBlockVerifierSoundnessInputs
        (q := q) E V secret) :
    N32CompactProductBlockVerifierSoundness E V.toSerializedVerifier
      secret where
  responseCodec := S.responseDecoder.toCompactResponseCodec
  mask := S.mask
  accepted_response_eq := by
    intro state challenge haccept r b hactive f
    exact S.accepted_response_eq state challenge haccept r b hactive f

end N32ByteCompactProductBlockVerifierSoundnessInputs

namespace N32CompactProductBlockVerifierSoundness

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Response LocalFree : Type*}
variable [Fintype State]
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {V : N32SerializedVerifier (Response := Response) E}
variable {secret : N32Block × LocalFree -> ZMod q}

/-- A block is covered by opposite signed active programmed challenges. -/
def replayOppositeActive
    (trace : N32SixReplayTrace State)
    (r : N32Repetition) (b : N32Block) : Prop :=
  ∃ d₀ d₁ : N32ReplayDraw,
    OppositeSignedActive
      ((trace.2 d₀ r).scalar (q := q))
      ((trace.2 d₁ r).scalar (q := q)) b

/--
The compact active-only round obtained from one repetition of a six-replay
trace, under the assumption that all six programmed one-shot transcripts
accept.
-/
noncomputable def compactRoundOfReplay
    (S : N32CompactProductBlockVerifierSoundness E V secret)
    (trace : N32SixReplayTrace State)
    (hall :
      n32SixReplayAllAccepts
          (fun state _d challenge => V.oneShotAccepts state challenge)
          trace = true)
    (r : N32Repetition) :
    CompactMultiForkLinearRound
      N32ReplayDraw N32Block LocalFree secret where
  chi := fun d => (trace.2 d r).scalar (q := q)
  mask := S.mask trace.1 r
  z := fun d b hactive f =>
    S.responseCodec.decode (V.response trace.1 r)
      (trace.2 d r) b hactive f
  hz := by
    intro d b hactive f
    have hall_forall :
        ∀ d : N32ReplayDraw,
          V.oneShotAccepts trace.1 (trace.2 d) = true := by
      simpa [n32SixReplayAllAccepts] using hall
    exact S.accepted_response_eq trace.1 (trace.2 d)
      (hall_forall d) r b hactive f

theorem compactRound_compactOppositeActive_of_replay
    (S : N32CompactProductBlockVerifierSoundness E V secret)
    (trace : N32SixReplayTrace State)
    (hall :
      n32SixReplayAllAccepts
          (fun state _d challenge => V.oneShotAccepts state challenge)
          trace = true)
    {r : N32Repetition} {b : N32Block}
    (hactive : replayOppositeActive (q := q) trace r b) :
    (S.compactRoundOfReplay trace hall r).CompactOppositeActive b := by
  simpa [compactRoundOfReplay, replayOppositeActive,
    CompactMultiForkLinearRound.CompactOppositeActive] using hactive

/--
Concrete active-only extraction theorem for the N32 six-replay transcript.

When all six one-shot replays accept and the programmed challenges cover every
block by opposite signs across repetitions, the compact active-only extractor
recovers the product-block secret.
-/
theorem compactExtractedSecret_eq
    (S : N32CompactProductBlockVerifierSoundness E V secret)
    (trace : N32SixReplayTrace State)
    (hall :
      n32SixReplayAllAccepts
          (fun state _d challenge => V.oneShotAccepts state challenge)
          trace = true)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1)
    (hcover :
      CoversBlocks
        (fun r b => replayOppositeActive (q := q) trace r b)) :
    CompactMultiForkLinearRound.extractedSecret
        (fun r => S.compactRoundOfReplay trace hall r)
        hinvTwo
        (by
          intro b
          rcases hcover b with ⟨r, hr⟩
          exact ⟨r,
            S.compactRound_compactOppositeActive_of_replay
              trace hall hr⟩) =
      secret := by
  exact CompactMultiForkLinearRound.extractedSecret_eq
    (fun r => S.compactRoundOfReplay trace hall r)
    hinvTwo
    (by
      intro b
      rcases hcover b with ⟨r, hr⟩
      exact ⟨r,
        S.compactRound_compactOppositeActive_of_replay
          trace hall hr⟩)

end N32CompactProductBlockVerifierSoundness

/-
Concrete compact fork-delta coverage event for the N32 six-replay trace.

Coverage succeeds when every product block has, in at least one repetition, two
programmed replay challenges with opposite signed active scalars.  This is the
event required by the active-only extractor.
-/
namespace N32CompactCoverage

variable {q : Nat} [NeZero q]
variable {State : Type*}

def roundNoPositive
    (draws : N32ReplayRoundVector)
    (b : N32Block) : Bool :=
  @decide (∀ d : N32ReplayDraw,
      (draws d).noPositiveAt b = true)
    Fintype.decidableForallFintype

def roundNoNegative
    (draws : N32ReplayRoundVector)
    (b : N32Block) : Bool :=
  @decide (∀ d : N32ReplayDraw,
      (draws d).noNegativeAt b = true)
    Fintype.decidableForallFintype

def roundMissesBlockBySign
    (draws : N32ReplayRoundVector)
    (b : N32Block) : Bool :=
  @decide (∀ d₀ : N32ReplayDraw, ∀ d₁ : N32ReplayDraw,
      ¬ ((draws d₀).positiveAt b = true ∧
          (draws d₁).negativeAt b = true))
    Fintype.decidableForallFintype

def matrixMissesBlockBySign
    (matrix : N32ReplayRoundMatrix)
    (b : N32Block) : Bool :=
  @decide (∀ r : N32Repetition,
      roundMissesBlockBySign (matrix r) b = true)
    Fintype.decidableForallFintype

def vectorMissesBlockBySign
    (challenges : N32ReplayChallengeVector)
    (b : N32Block) : Bool :=
  matrixMissesBlockBySign (transposeReplayChallenges challenges) b

def missesBlock
    (trace : N32SixReplayTrace State)
    (b : N32Block) : Bool :=
  by
    classical
    exact decide (∀ r : N32Repetition,
      ¬ N32CompactProductBlockVerifierSoundness.replayOppositeActive
        (q := q) trace r b)

def failure
    (trace : N32SixReplayTrace State) : Bool :=
  by
    classical
    exact decide (∃ b : N32Block, missesBlock (q := q) trace b = true)

theorem failure_false_covers
    {trace : N32SixReplayTrace State}
    (hfailure : failure (q := q) trace = false) :
    CoversBlocks
      (fun r b =>
        N32CompactProductBlockVerifierSoundness.replayOppositeActive
          (q := q) trace r b) := by
  intro b
  by_contra hmiss
  have hmissBool : missesBlock (q := q) trace b = true := by
    simpa [missesBlock] using hmiss
  have hfail : failure (q := q) trace = true := by
    simp [failure]
    exact ⟨b, hmissBool⟩
  simp [hfail] at hfailure

theorem failure_implies_missesBlock
    {trace : N32SixReplayTrace State}
    (hfailure : failure (q := q) trace = true) :
    ∃ b : N32Block, missesBlock (q := q) trace b = true := by
  simpa [failure] using hfailure

theorem failure_acceptProb_le_sum_misses
    [Fintype State]
    (D : FiniteProbability (N32SixReplayTrace State)) :
    D.acceptProb (failure (q := q)) <=
      Finset.univ.sum fun b : N32Block =>
        D.acceptProb (fun trace => missesBlock (q := q) trace b) := by
  exact acceptProb_le_sum_of_exists D
    (failure (q := q))
    (fun b trace => missesBlock (q := q) trace b)
    (fun trace hfailure => failure_implies_missesBlock (q := q) hfailure)

theorem n32RoundChallenge_noPositive_card (b : N32Block) :
    Fintype.card {c : N32RoundChallenge //
      c.noPositiveAt b = true} = 17 * 2 ^ 14 := by
  fin_cases b <;> native_decide

theorem n32RoundChallenge_noNegative_card (b : N32Block) :
    Fintype.card {c : N32RoundChallenge //
      c.noNegativeAt b = true} = 17 * 2 ^ 14 := by
  fin_cases b <;> native_decide

theorem n32RoundChallenge_noPositive_acceptProb (b : N32Block) :
    (uniformFiniteProbability N32RoundChallenge).acceptProb
        (fun c => c.noPositiveAt b) =
      (17 : ℝ) / 32 := by
  rw [uniform_acceptProb_eq_card_true]
  rw [n32RoundChallenge_noPositive_card, n32RoundChallenge_card]
  norm_num

theorem n32RoundChallenge_noNegative_acceptProb (b : N32Block) :
    (uniformFiniteProbability N32RoundChallenge).acceptProb
        (fun c => c.noNegativeAt b) =
      (17 : ℝ) / 32 := by
  rw [uniform_acceptProb_eq_card_true]
  rw [n32RoundChallenge_noNegative_card, n32RoundChallenge_card]
  norm_num

theorem roundNoPositive_acceptProb (b : N32Block) :
    n32ReplayRoundVectorProbability.acceptProb
        (fun draws => roundNoPositive draws b) =
      ((17 : ℝ) / 32) ^ 6 := by
  have h := functionProduct_acceptProb_forall_eq_prod
    (fun _d : N32ReplayDraw =>
      uniformFiniteProbability N32RoundChallenge)
    (fun _d c => c.noPositiveAt b)
  calc
    n32ReplayRoundVectorProbability.acceptProb
        (fun draws => roundNoPositive draws b) =
        Finset.univ.prod fun _d : N32ReplayDraw =>
          (uniformFiniteProbability N32RoundChallenge).acceptProb
            (fun c => c.noPositiveAt b) := by
          exact h
    _ = ((17 : ℝ) / 32) ^ 6 := by
          simp [Finset.prod_const,
            n32RoundChallenge_noPositive_acceptProb]
          ring

theorem roundNoNegative_acceptProb (b : N32Block) :
    n32ReplayRoundVectorProbability.acceptProb
        (fun draws => roundNoNegative draws b) =
      ((17 : ℝ) / 32) ^ 6 := by
  have h := functionProduct_acceptProb_forall_eq_prod
    (fun _d : N32ReplayDraw =>
      uniformFiniteProbability N32RoundChallenge)
    (fun _d c => c.noNegativeAt b)
  calc
    n32ReplayRoundVectorProbability.acceptProb
        (fun draws => roundNoNegative draws b) =
        Finset.univ.prod fun _d : N32ReplayDraw =>
          (uniformFiniteProbability N32RoundChallenge).acceptProb
            (fun c => c.noNegativeAt b) := by
          exact h
    _ = ((17 : ℝ) / 32) ^ 6 := by
          simp [Finset.prod_const,
            n32RoundChallenge_noNegative_acceptProb]
          ring

theorem roundMissesBlockBySign_implies_noSign
    {draws : N32ReplayRoundVector} {b : N32Block}
    (hmiss : roundMissesBlockBySign draws b = true) :
    roundNoPositive draws b = true ∨
      roundNoNegative draws b = true := by
  by_cases hpos : roundNoPositive draws b = true
  · exact Or.inl hpos
  · right
    by_cases hneg : roundNoNegative draws b = true
    · exact hneg
    · exfalso
      simp only [roundNoPositive, Bool.not_eq_true] at hpos
      simp only [decide_eq_false_iff_not, not_forall] at hpos
      rcases hpos with ⟨dpos, hdpos⟩
      have hposTrue : (draws dpos).positiveAt b = true := by
        cases hp : (draws dpos).positiveAt b <;>
          simp [N32RoundChallenge.noPositiveAt, hp] at hdpos ⊢
      simp only [roundNoNegative, Bool.not_eq_true] at hneg
      simp only [decide_eq_false_iff_not, not_forall] at hneg
      rcases hneg with ⟨dneg, hdneg⟩
      have hnegTrue : (draws dneg).negativeAt b = true := by
        cases hn : (draws dneg).negativeAt b <;>
          simp [N32RoundChallenge.noNegativeAt, hn] at hdneg ⊢
      have hmissForall :
          ∀ d₀ : N32ReplayDraw, ∀ d₁ : N32ReplayDraw,
            ¬ ((draws d₀).positiveAt b = true ∧
                (draws d₁).negativeAt b = true) := by
        simpa [roundMissesBlockBySign] using hmiss
      exact hmissForall dpos dneg ⟨hposTrue, hnegTrue⟩

def roundMissBound : ℝ :=
  2 * (((17 : ℝ) / 32) ^ 6)

theorem roundMissesBlockBySign_acceptProb_le
    (b : N32Block) :
    n32ReplayRoundVectorProbability.acceptProb
        (fun draws => roundMissesBlockBySign draws b) <=
      roundMissBound := by
  let D := n32ReplayRoundVectorProbability
  have hmono :
      D.acceptProb (fun draws => roundMissesBlockBySign draws b) <=
        D.acceptProb
          (fun draws => roundNoPositive draws b ||
            roundNoNegative draws b) := by
    exact acceptProb_mono D (fun draws hdraws => by
      rcases roundMissesBlockBySign_implies_noSign
          (draws := draws) (b := b) hdraws with hpos | hneg
      · simp [hpos]
      · simp [hneg])
  have hor :
      D.acceptProb
          (fun draws => roundNoPositive draws b ||
            roundNoNegative draws b) <=
        D.acceptProb (fun draws => roundNoPositive draws b) +
          D.acceptProb (fun draws => roundNoNegative draws b) :=
    acceptProb_or_le_add D
      (fun draws => roundNoPositive draws b)
      (fun draws => roundNoNegative draws b)
  have hpos := roundNoPositive_acceptProb b
  have hneg := roundNoNegative_acceptProb b
  unfold roundMissBound
  linarith

/--
One-block miss target for the N32 compact fork-delta event.

For a fixed block in one repetition, a single replay draw is `0` with
probability `1/16`, `+1` with probability `15/32`, and `-1` with probability
`15/32`.  This certified Lean bound uses the sign-union bound: six draws fail
to contain both signs with probability at most `2 * (17/32)^6`; seven
independent repetitions give the seventh power.  The sharper
inclusion-exclusion value subtracts `(1/16)^6` per repetition, but changes this
candidate's compact coverage budget by less than `0.000014` bits.
-/
def singleBlockMissBound : ℝ :=
  roundMissBound ^ 7

theorem matrixMissesBlockBySign_acceptProb_le_singleBlockMissBound
    (b : N32Block) :
    n32ReplayRoundMatrixProbability.acceptProb
        (fun matrix => matrixMissesBlockBySign matrix b) <=
      singleBlockMissBound := by
  have hprod := functionProduct_acceptProb_forall_eq_prod
    (fun _r : N32Repetition => n32ReplayRoundVectorProbability)
    (fun _r draws => roundMissesBlockBySign draws b)
  have hle :
      (Finset.univ.prod fun _r : N32Repetition =>
        n32ReplayRoundVectorProbability.acceptProb
          (fun draws => roundMissesBlockBySign draws b)) <=
        Finset.univ.prod fun _r : N32Repetition => roundMissBound := by
    exact Finset.prod_le_prod
      (by
        intro r _hr
        exact n32ReplayRoundVectorProbability.acceptProb_nonneg
          (fun draws => roundMissesBlockBySign draws b))
      (by
        intro r _hr
        exact roundMissesBlockBySign_acceptProb_le b)
  calc
    n32ReplayRoundMatrixProbability.acceptProb
        (fun matrix => matrixMissesBlockBySign matrix b) =
        Finset.univ.prod fun _r : N32Repetition =>
          n32ReplayRoundVectorProbability.acceptProb
            (fun draws => roundMissesBlockBySign draws b) := hprod
    _ <= Finset.univ.prod fun _r : N32Repetition => roundMissBound := hle
    _ = singleBlockMissBound := by
          norm_num [singleBlockMissBound, N32Repetition, Finset.prod_const,
            nsmul_eq_mul]

theorem replayOppositeActive_of_roundSigns
    (trace : N32SixReplayTrace State)
    {r : N32Repetition} {b : N32Block}
    {d₀ d₁ : N32ReplayDraw}
    (hpos : (trace.2 d₀ r).positiveAt b = true)
    (hneg : (trace.2 d₁ r).negativeAt b = true) :
    N32CompactProductBlockVerifierSoundness.replayOppositeActive
      (q := q) trace r b := by
  exact ⟨d₀, d₁, Or.inl
    ⟨N32RoundChallenge.scalar_eq_one_of_positiveAt
        (q := q) (trace.2 d₀ r) hpos,
      N32RoundChallenge.scalar_eq_neg_one_of_negativeAt
        (q := q) (trace.2 d₁ r) hneg⟩⟩

theorem missesBlock_implies_vectorMissesBlockBySign
    {trace : N32SixReplayTrace State} {b : N32Block}
    (hmiss : missesBlock (q := q) trace b = true) :
    vectorMissesBlockBySign trace.2 b = true := by
  classical
  have hmissAll :
      ∀ r : N32Repetition,
        ¬ N32CompactProductBlockVerifierSoundness.replayOppositeActive
          (q := q) trace r b := by
    exact of_decide_eq_true (by
      simpa [missesBlock] using hmiss)
  unfold vectorMissesBlockBySign matrixMissesBlockBySign
  exact @decide_eq_true _ Fintype.decidableForallFintype (by
    intro r
    unfold roundMissesBlockBySign
    exact @decide_eq_true _ Fintype.decidableForallFintype (by
      intro d₀ d₁ hsigns
      exact hmissAll r
        (replayOppositeActive_of_roundSigns
          (q := q) trace hsigns.1 hsigns.2)))

theorem vectorMissesBlockBySign_acceptProb_le_singleBlockMissBound
    (b : N32Block) :
    n32ReplayChallengeVectorProbability.acceptProb
        (fun challenges => vectorMissesBlockBySign challenges b) <=
      singleBlockMissBound := by
  have htranspose :=
    n32ReplayChallengeVector_acceptProb_transpose
      (fun matrix => matrixMissesBlockBySign matrix b)
  have hmatrix :=
    matrixMissesBlockBySign_acceptProb_le_singleBlockMissBound b
  simpa [vectorMissesBlockBySign] using
    (le_of_eq_of_le htranspose hmatrix)

theorem missesBlock_acceptProb_le_singleBlockMissBound
    [Fintype State]
    (Dσ : FiniteProbability State)
    (b : N32Block) :
    (n32SixReplayTraceDist Dσ).acceptProb
        (fun trace => missesBlock (q := q) trace b) <=
      singleBlockMissBound := by
  let D := n32SixReplayTraceDist Dσ
  have hmono :
      D.acceptProb (fun trace => missesBlock (q := q) trace b) <=
        D.acceptProb
          (fun trace => vectorMissesBlockBySign trace.2 b) :=
    acceptProb_mono D
      (fun trace htrace =>
        missesBlock_implies_vectorMissesBlockBySign
          (q := q) (trace := trace) htrace)
  have hignore :
      D.acceptProb
          (fun trace => vectorMissesBlockBySign trace.2 b) =
        n32ReplayChallengeVectorProbability.acceptProb
          (fun challenges => vectorMissesBlockBySign challenges b) := by
    simpa [D, n32SixReplayTraceDist] using
      productFiniteProbability_ignore_left_acceptProb
        Dσ n32ReplayChallengeVectorProbability
        (fun challenges => vectorMissesBlockBySign challenges b)
  have hvec := vectorMissesBlockBySign_acceptProb_le_singleBlockMissBound b
  linarith

/-- Union-bound compact coverage target for all sixteen N32 product blocks. -/
def failureUnionBound : ℝ :=
  16 * singleBlockMissBound

theorem failure_acceptProb_le_unionBound_of_block_miss
    [Fintype State]
    (D : FiniteProbability (N32SixReplayTrace State))
    (hblock :
      ∀ b : N32Block,
        D.acceptProb (fun trace => missesBlock (q := q) trace b) <=
          singleBlockMissBound) :
    D.acceptProb (failure (q := q)) <= failureUnionBound := by
  have hsum := failure_acceptProb_le_sum_misses (q := q) D
  have hblocks :
      (Finset.univ.sum fun b : N32Block =>
        D.acceptProb (fun trace => missesBlock (q := q) trace b)) <=
      Finset.univ.sum fun _b : N32Block => singleBlockMissBound := by
    exact Finset.sum_le_sum (by
      intro b _hb
      exact hblock b)
  calc
    D.acceptProb (failure (q := q)) <=
        Finset.univ.sum fun b : N32Block =>
          D.acceptProb (fun trace => missesBlock (q := q) trace b) := hsum
    _ <= Finset.univ.sum fun _b : N32Block => singleBlockMissBound := hblocks
    _ = failureUnionBound := by
          norm_num [failureUnionBound, N32Block, Finset.sum_const,
            nsmul_eq_mul]

theorem failure_unionBound_nonneg :
    0 <= failureUnionBound := by
  norm_num [failureUnionBound, singleBlockMissBound, roundMissBound]

end N32CompactCoverage

/--
Concrete product-moment extraction wrapper for compact active-only responses.

This bypasses the full-response idealization: the replay event is six accepted
one-shot compact transcripts, and the extractor is the active-only deterministic
extractor from `CompactMultiForkLinearRound`.
-/
structure N32CompactSixReplayExtractionInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Response LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (secret : N32Block × LocalFree -> ZMod q) where
  verifier : N32SerializedVerifier (Response := Response) E
  compactSoundness :
    N32CompactProductBlockVerifierSoundness E verifier secret
  forgery : N32QueryTrace State -> Bool
  queryMiss : N32QueryTrace State -> Bool
  queryMiss_not_queried :
    ∀ τ, queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ
  forgery_accepts :
    ∀ τ, forgery τ = true -> verifier.outputAccepts τ = true
  coverageFailure : N32SixReplayTrace State -> Bool
  coverageFailure_sound :
    ∀ τ,
      coverageFailure τ = false ->
        CoversBlocks
          (fun r b =>
            N32CompactProductBlockVerifierSoundness.replayOppositeActive
              (q := q) τ r b)
  coverageFailureBound : ℝ
  coverageFailure_upper :
    (n32SixReplayTraceDist E.stateDist).acceptProb coverageFailure <=
      coverageFailureBound

namespace N32CompactSixReplayExtractionInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Response LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {secret : N32Block × LocalFree -> ZMod q}

def accepts
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (state : State)
    (challenge : N32CanonicalChallenge) : Bool :=
  I.verifier.oneShotAccepts state challenge

def forgerSuccessProb
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret) : ℝ :=
  E.traceDist.acceptProb I.forgery

def queryMissProb
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret) : ℝ :=
  E.traceDist.acceptProb I.queryMiss

def allAccepts
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32SixReplayTrace State) : Bool :=
  n32SixReplayAllAccepts
    (fun state _d challenge => I.accepts state challenge) τ

def compactExtractableEvent
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32SixReplayTrace State) : Bool :=
  I.allAccepts τ && !I.coverageFailure τ

def compactExtractableProb
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret) : ℝ :=
  (n32SixReplayTraceDist E.stateDist).acceptProb I.compactExtractableEvent

def toQueryMissCertificate
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret) :
    N32QueryLogExperiment.QueryMissCertificate E I.queryMiss where
  queryMiss_not_queried := I.queryMiss_not_queried
  queryMiss_outputs_blind := I.queryMiss_outputs_blind

theorem forgery_implies_accepts
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32QueryTrace State)
    (hforgery : I.forgery τ = true) :
    I.accepts τ.1 τ.2 = true := by
  exact
    I.verifier.oneShotAccepts_of_outputAccepts τ
      (I.forgery_accepts τ hforgery)

theorem queryMissProb_le_guess
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret) :
    I.queryMissProb <= n32ChallengeGuessBound := by
  unfold queryMissProb
  exact
    N32QueryLogExperiment.QueryMissCertificate.queryMiss_acceptProb_le
      (I.toQueryMissCertificate)

theorem stateAcceptMean_ge_forger_sub_guess
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret) :
    I.forgerSuccessProb - n32ChallengeGuessBound <=
      Finset.univ.sum fun state =>
        E.stateDist.mass state *
          n32StateAcceptProbability I.accepts state := by
  have hbase :=
    n32_stateAcceptMean_ge_forgery_sub_queryMiss
      E.stateDist I.accepts I.forgery I.queryMiss
      (fun τ hforgery _hqueryMiss =>
        I.forgery_implies_accepts τ hforgery)
  have hbase' :
      I.forgerSuccessProb - I.queryMissProb <=
        Finset.univ.sum fun state =>
          E.stateDist.mass state *
            n32StateAcceptProbability I.accepts state := by
    simpa [forgerSuccessProb, queryMissProb,
      N32QueryLogExperiment.traceDist] using hbase
  have hquery := I.queryMissProb_le_guess
  linarith

theorem allAcceptsProb_ge_lowerBound_sixth
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (lowerBound : ℝ)
    (hlower_nonneg : 0 <= lowerBound)
    (hlower :
      lowerBound <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    lowerBound ^ 6 <=
      (n32SixReplayTraceDist E.stateDist).acceptProb I.allAccepts := by
  have hmean := I.stateAcceptMean_ge_forger_sub_guess
  simpa [allAccepts, accepts] using
    n32_six_replay_identical_acceptProb_ge_lowerBound_sixth
      E.stateDist I.accepts lowerBound hlower_nonneg
      (le_trans hlower hmean)

/--
Probability of compact active-only extraction under the product-moment replay
analysis.  Only the compact coverage-failure bound is subtracted after the
sixth-power replay step.
-/
theorem compactExtractableProb_ge_lowerBound_sixth
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (lowerBound : ℝ)
    (hlower_nonneg : 0 <= lowerBound)
    (hlower :
      lowerBound <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    lowerBound ^ 6 - I.coverageFailureBound <=
      I.compactExtractableProb := by
  let D := n32SixReplayTraceDist E.stateDist
  have hall := I.allAcceptsProb_ge_lowerBound_sixth
    lowerBound hlower_nonneg hlower
  have hsub :
      D.acceptProb I.allAccepts - D.acceptProb I.coverageFailure <=
        D.acceptProb I.compactExtractableEvent := by
    simpa [compactExtractableEvent] using
      sub_acceptProb_le_and_not D I.allAccepts I.coverageFailure
  have hcov := I.coverageFailure_upper
  unfold compactExtractableProb
  linarith

theorem compactExtractableProb_ge_forgeryMinusGuess_sixth
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
        I.coverageFailureBound <=
      I.compactExtractableProb := by
  exact I.compactExtractableProb_ge_lowerBound_sixth
    (I.forgerSuccessProb - n32ChallengeGuessBound) hnonneg le_rfl

theorem rawAllAccepts_of_compactExtractable
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    {τ : N32SixReplayTrace State}
    (hτ : I.compactExtractableEvent τ = true) :
    n32SixReplayAllAccepts
        (fun state _d challenge => I.verifier.oneShotAccepts state challenge)
        τ = true := by
  have hall : I.allAccepts τ = true := by
    cases hall : I.allAccepts τ <;>
      cases hcov : I.coverageFailure τ <;>
      simp [compactExtractableEvent, hall, hcov] at hτ ⊢
  simpa [allAccepts, accepts] using hall

theorem coverageFailure_false_of_compactExtractable
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    {τ : N32SixReplayTrace State}
    (hτ : I.compactExtractableEvent τ = true) :
    I.coverageFailure τ = false := by
  cases hall : I.allAccepts τ <;>
    cases hcov : I.coverageFailure τ <;>
    simp [compactExtractableEvent, hall, hcov] at hτ ⊢

def compactCoverageOfExtractable
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32SixReplayTrace State)
    (hτ : I.compactExtractableEvent τ = true) :
    CoversBlocks
      (fun r b =>
        (I.compactSoundness.compactRoundOfReplay τ
          (I.rawAllAccepts_of_compactExtractable hτ) r).CompactOppositeActive b) := by
  intro b
  rcases I.coverageFailure_sound τ
      (I.coverageFailure_false_of_compactExtractable hτ) b with ⟨r, hr⟩
  exact ⟨r,
    I.compactSoundness.compactRound_compactOppositeActive_of_replay
      τ (I.rawAllAccepts_of_compactExtractable hτ) hr⟩

theorem extractedSecret_eq_of_compactExtractable
    (I : N32CompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32SixReplayTrace State)
    (hτ : I.compactExtractableEvent τ = true)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    CompactMultiForkLinearRound.extractedSecret
        (fun r => I.compactSoundness.compactRoundOfReplay τ
          (I.rawAllAccepts_of_compactExtractable hτ) r)
        hinvTwo (I.compactCoverageOfExtractable τ hτ) =
      secret := by
  exact CompactMultiForkLinearRound.extractedSecret_eq
    (fun r => I.compactSoundness.compactRoundOfReplay τ
      (I.rawAllAccepts_of_compactExtractable hτ) r)
    hinvTwo (I.compactCoverageOfExtractable τ hτ)

end N32CompactSixReplayExtractionInputs

/--
Concrete N32 compact extraction inputs using the canonical compact
fork-delta coverage event.

The one-block miss bound is certified by
`N32CompactCoverage.missesBlock_acceptProb_le_singleBlockMissBound`.  The
sixteen-block union bound and the coverage soundness implication are supplied
here, so callers only instantiate the serialized verifier and compact
active-response soundness obligations.  The query-miss event is the canonical
one: a forgery whose challenge point was not queried.
-/
structure N32ConcreteCompactSixReplayExtractionInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Response LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (secret : N32Block × LocalFree -> ZMod q) where
  verifier : N32SerializedVerifier (Response := Response) E
  compactSoundness :
    N32CompactProductBlockVerifierSoundness E verifier secret
  forgery : N32QueryTrace State -> Bool
  forgery_accepts :
    ∀ τ, forgery τ = true -> verifier.outputAccepts τ = true

namespace N32ConcreteCompactSixReplayExtractionInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Response LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {secret : N32Block × LocalFree -> ZMod q}

def queryMiss
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32QueryTrace State) : Bool :=
  I.forgery τ && !E.pointQueried τ

theorem queryMiss_not_queried
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32QueryTrace State)
    (hmiss : I.queryMiss τ = true) :
    E.pointQueried τ = false := by
  cases hforgery : I.forgery τ <;>
    cases hqueried : E.pointQueried τ <;>
    simp [queryMiss, hforgery, hqueried] at hmiss ⊢

theorem queryMiss_outputs_blind
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32QueryTrace State)
    (hmiss : I.queryMiss τ = true) :
    E.actualChallenge τ = E.blindChallenge τ := by
  have hforgery : I.forgery τ = true := by
    cases hforgery : I.forgery τ <;>
      cases hqueried : E.pointQueried τ <;>
      simp [queryMiss, hforgery, hqueried] at hmiss ⊢
  have haccept := I.forgery_accepts τ hforgery
  have hchallenge :=
    I.verifier.recomputedChallenge_matches_actual τ
      (by
        simpa [N32SerializedVerifier.outputAccepts,
          N32SerializedVerifier.outputTranscript] using haccept)
  simpa [N32QueryLogExperiment.actualChallenge,
    N32QueryLogExperiment.blindChallenge] using hchallenge.symm

def toCompactInputs
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret) :
    N32CompactSixReplayExtractionInputs (Response := Response) E secret where
  verifier := I.verifier
  compactSoundness := I.compactSoundness
  forgery := I.forgery
  queryMiss := I.queryMiss
  queryMiss_not_queried := I.queryMiss_not_queried
  queryMiss_outputs_blind := I.queryMiss_outputs_blind
  forgery_accepts := I.forgery_accepts
  coverageFailure := N32CompactCoverage.failure (q := q)
  coverageFailure_sound := by
    intro τ hfailure
    exact N32CompactCoverage.failure_false_covers (q := q) hfailure
  coverageFailureBound := N32CompactCoverage.failureUnionBound
  coverageFailure_upper := by
    exact
      N32CompactCoverage.failure_acceptProb_le_unionBound_of_block_miss
        (q := q) (n32SixReplayTraceDist E.stateDist)
        (fun b =>
          N32CompactCoverage.missesBlock_acceptProb_le_singleBlockMissBound
            (q := q) E.stateDist b)

def compactExtractableProb
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret) : ℝ :=
  I.toCompactInputs.compactExtractableProb

def forgerSuccessProb
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret) : ℝ :=
  I.toCompactInputs.forgerSuccessProb

theorem compactExtractableProb_ge_forgeryMinusGuess_sixth
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
        N32CompactCoverage.failureUnionBound <=
      I.compactExtractableProb := by
  simpa [compactExtractableProb, forgerSuccessProb, toCompactInputs] using
    (I.toCompactInputs.compactExtractableProb_ge_forgeryMinusGuess_sixth
      hnonneg)

theorem exists_compactExtractable_of_positive_lower_bound
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound)
    (hpositive :
      0 <
        (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
          N32CompactCoverage.failureUnionBound) :
    ∃ τ : N32SixReplayTrace State,
      I.toCompactInputs.compactExtractableEvent τ = true := by
  have hlower :=
    I.compactExtractableProb_ge_forgeryMinusGuess_sixth hnonneg
  have hprob :
      0 < I.compactExtractableProb := lt_of_lt_of_le hpositive hlower
  exact
    exists_true_of_acceptProb_pos
      (n32SixReplayTraceDist E.stateDist)
      I.toCompactInputs.compactExtractableEvent
      (by
        simpa [compactExtractableProb,
          N32CompactSixReplayExtractionInputs.compactExtractableProb] using
          hprob)

theorem extractedSecret_eq_of_compactExtractable
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (τ : N32SixReplayTrace State)
    (hτ : I.toCompactInputs.compactExtractableEvent τ = true)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    CompactMultiForkLinearRound.extractedSecret
        (fun r => I.compactSoundness.compactRoundOfReplay τ
          (I.toCompactInputs.rawAllAccepts_of_compactExtractable hτ) r)
        hinvTwo
        (I.toCompactInputs.compactCoverageOfExtractable τ hτ) =
      secret := by
  exact I.toCompactInputs.extractedSecret_eq_of_compactExtractable
    τ hτ hinvTwo

theorem exists_extractedSecret_eq_of_positive_lower_bound
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound)
    (hpositive :
      0 <
        (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
          N32CompactCoverage.failureUnionBound)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    ∃ τ : N32SixReplayTrace State,
      ∃ hτ : I.toCompactInputs.compactExtractableEvent τ = true,
        CompactMultiForkLinearRound.extractedSecret
            (fun r => I.compactSoundness.compactRoundOfReplay τ
              (I.toCompactInputs.rawAllAccepts_of_compactExtractable hτ) r)
            hinvTwo
            (I.toCompactInputs.compactCoverageOfExtractable τ hτ) =
          secret := by
  rcases I.exists_compactExtractable_of_positive_lower_bound
      hnonneg hpositive with ⟨τ, hτ⟩
  exact ⟨τ, hτ, I.extractedSecret_eq_of_compactExtractable
    τ hτ hinvTwo⟩

end N32ConcreteCompactSixReplayExtractionInputs

/--
Byte-vector concrete compact extraction inputs.

This is the canonical byte route: public key, message, commitments, and
responses are fixed-length byte vectors; the challenge codec is `packed17`;
compact response bytes are decoded by a fixed response-byte decoder.
-/
structure N32ByteConcreteCompactSixReplayExtractionInputs
    {q : Nat} [NeZero q]
    {State QueryIdx LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  compactSoundness :
    N32ByteCompactProductBlockVerifierSoundnessInputs
      (q := q) E verifier secret
  forgery : N32QueryTrace State -> Bool
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true

namespace N32ByteConcreteCompactSixReplayExtractionInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}

  noncomputable def toConcreteInputs
    (I :
      N32ByteConcreteCompactSixReplayExtractionInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret) :
    N32ConcreteCompactSixReplayExtractionInputs
      (Response := ByteVector responseLen) E secret where
  verifier := I.verifier.toSerializedVerifier
  compactSoundness := I.compactSoundness.toCompactSoundness
  forgery := I.forgery
  forgery_accepts := I.forgery_accepts

def forgerSuccessProb
    (I :
      N32ByteConcreteCompactSixReplayExtractionInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret) : ℝ :=
  I.toConcreteInputs.forgerSuccessProb

def compactExtractableProb
    (I :
      N32ByteConcreteCompactSixReplayExtractionInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret) : ℝ :=
  I.toConcreteInputs.compactExtractableProb

theorem compactExtractableProb_ge_forgeryMinusGuess_sixth
    (I :
      N32ByteConcreteCompactSixReplayExtractionInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound) :
    (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
        N32CompactCoverage.failureUnionBound <=
      I.compactExtractableProb := by
  simpa [forgerSuccessProb, compactExtractableProb] using
    I.toConcreteInputs.compactExtractableProb_ge_forgeryMinusGuess_sixth
      hnonneg

theorem exists_extractedSecret_eq_of_positive_lower_bound
    (I :
      N32ByteConcreteCompactSixReplayExtractionInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound)
    (hpositive :
      0 <
        (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
          N32CompactCoverage.failureUnionBound)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    ∃ τ : N32SixReplayTrace State,
      ∃ hτ : I.toConcreteInputs.toCompactInputs.compactExtractableEvent τ =
          true,
        CompactMultiForkLinearRound.extractedSecret
            (fun r => I.toConcreteInputs.compactSoundness.compactRoundOfReplay τ
              (N32CompactSixReplayExtractionInputs.rawAllAccepts_of_compactExtractable
                (I.toConcreteInputs.toCompactInputs) hτ) r)
            hinvTwo
            (N32CompactSixReplayExtractionInputs.compactCoverageOfExtractable
              (I.toConcreteInputs.toCompactInputs) τ hτ) =
          secret := by
  simpa [forgerSuccessProb] using
    I.toConcreteInputs.exists_extractedSecret_eq_of_positive_lower_bound
      hnonneg hpositive hinvTwo

end N32ByteConcreteCompactSixReplayExtractionInputs

/--
Abstract keygen-security target for the product-block signature proof.

The transcript/forking proof recovers the product-block secret.  A concrete
signature instantiation closes this interface by saying how a recovered
product-block secret is packaged as a breaker for its keygen security game.
-/
structure N32ProductBlockSecretRecoveryGame
    {q : Nat} [NeZero q] (LocalFree Breaker : Type*) where
  secret : N32Block × LocalFree -> ZMod q
  buildBreaker : (N32Block × LocalFree -> ZMod q) -> Breaker
  BreaksKeygen : Breaker -> Prop
  buildBreaker_sound :
    ∀ recovered, recovered = secret -> BreaksKeygen (buildBreaker recovered)

namespace N32ProductBlockSecretRecoveryGame

variable {q : Nat} [NeZero q]
variable {LocalFree : Type*}

/-- The canonical keygen-break target when breaking keygen means recovering the product-block secret. -/
def secretRecovery
    (secret : N32Block × LocalFree -> ZMod q) :
    N32ProductBlockSecretRecoveryGame (q := q) LocalFree
      (N32Block × LocalFree -> ZMod q) where
  secret := secret
  buildBreaker := id
  BreaksKeygen := fun recovered => recovered = secret
  buildBreaker_sound := by
    intro recovered hrecovered
    exact hrecovered

theorem secretRecovery_breaks_iff
    (secret recovered : N32Block × LocalFree -> ZMod q) :
    (secretRecovery (q := q) secret).BreaksKeygen recovered ↔
      recovered = secret := by
  rfl

end N32ProductBlockSecretRecoveryGame

/--
Partial keygen-break target for N32 product-block signatures.

The extracted object no longer has to be the whole product-block secret.  A
concrete keygen game can declare any useful object as a valid partial witness,
as long as that witness can be packaged as a keygen breaker.
-/
structure N32PartialKeygenBreakGame
    {q : Nat} [NeZero q]
    (LocalFree PartialWitness Breaker : Type*) where
  secret : N32Block × LocalFree -> ZMod q
  ValidWitness : PartialWitness -> Prop
  buildBreaker : PartialWitness -> Breaker
  BreaksKeygen : Breaker -> Prop
  buildBreaker_sound :
    ∀ witness, ValidWitness witness ->
      BreaksKeygen (buildBreaker witness)

namespace N32PartialKeygenBreakGame

variable {q : Nat} [NeZero q]
variable {LocalFree PartialWitness Breaker : Type*}

/-- Full-secret recovery as a partial-witness game. -/
def fullSecret
    (secret : N32Block × LocalFree -> ZMod q) :
    N32PartialKeygenBreakGame (q := q) LocalFree
      (N32Block × LocalFree -> ZMod q)
      (N32Block × LocalFree -> ZMod q) where
  secret := secret
  ValidWitness := fun recovered => recovered = secret
  buildBreaker := id
  BreaksKeygen := fun recovered => recovered = secret
  buildBreaker_sound := by
    intro recovered hrecovered
    exact hrecovered

/-- Any existing full-secret keygen game is also a partial-witness game. -/
def ofSecretRecoveryGame
    (K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker) :
    N32PartialKeygenBreakGame (q := q) LocalFree
      (N32Block × LocalFree -> ZMod q) Breaker where
  secret := K.secret
  ValidWitness := fun recovered => recovered = K.secret
  buildBreaker := K.buildBreaker
  BreaksKeygen := K.BreaksKeygen
  buildBreaker_sound := K.buildBreaker_sound

end N32PartialKeygenBreakGame

/-- Candidate partial witness: one recovered product block. -/
structure N32RecoveredBlockWitness
    {q : Nat} [NeZero q] (LocalFree : Type*) where
  block : N32Block
  value : LocalFree -> ZMod q

namespace N32RecoveredBlockWitness

variable {q : Nat} [NeZero q]
variable {LocalFree : Type*}

def Valid
    (secret : N32Block × LocalFree -> ZMod q)
    (witness : N32RecoveredBlockWitness (q := q) LocalFree) : Prop :=
  ∀ f, witness.value f = secret (witness.block, f)

/--
Canonical one-block recovery game.

This does not assert that one block is a valid break for the final scheme.  It
is the exact interface where that keygen assumption can be tested.
-/
def keygenBreakGame
    (secret : N32Block × LocalFree -> ZMod q) :
    N32PartialKeygenBreakGame (q := q) LocalFree
      (N32RecoveredBlockWitness (q := q) LocalFree)
      (N32RecoveredBlockWitness (q := q) LocalFree) where
  secret := secret
  ValidWitness := Valid secret
  buildBreaker := id
  BreaksKeygen := Valid secret
  buildBreaker_sound := by
    intro witness hvalid
    exact hvalid

theorem keygenBreakGame_breaks_iff
    (secret : N32Block × LocalFree -> ZMod q)
    (witness : N32RecoveredBlockWitness (q := q) LocalFree) :
    (keygenBreakGame (q := q) secret).BreaksKeygen witness ↔
      Valid secret witness := by
  rfl

end N32RecoveredBlockWitness

/-- Candidate partial witness: recovered product-block secrets for a block set. -/
structure N32RecoveredBlocksWitness
    {q : Nat} [NeZero q] (LocalFree : Type*) where
  blocks : Finset N32Block
  value : (b : N32Block) -> b ∈ blocks -> LocalFree -> ZMod q

namespace N32RecoveredBlocksWitness

variable {q : Nat} [NeZero q]
variable {LocalFree : Type*}

def Valid
    (secret : N32Block × LocalFree -> ZMod q)
    (minBlocks : Nat)
    (witness : N32RecoveredBlocksWitness (q := q) LocalFree) : Prop :=
  minBlocks <= witness.blocks.card ∧
    ∀ b hb f, witness.value b hb f = secret (b, f)

/--
Multi-block partial recovery game.

This is the recommended partial-secret target surface: one block is a useful
test case, but a concrete keygen assumption should usually set `minBlocks`
large enough that guessing the witness is outside the desired security level.
-/
def keygenBreakGame
    (secret : N32Block × LocalFree -> ZMod q)
    (minBlocks : Nat) :
    N32PartialKeygenBreakGame (q := q) LocalFree
      (N32RecoveredBlocksWitness (q := q) LocalFree)
      (N32RecoveredBlocksWitness (q := q) LocalFree) where
  secret := secret
  ValidWitness := Valid secret minBlocks
  buildBreaker := id
  BreaksKeygen := Valid secret minBlocks
  buildBreaker_sound := by
    intro witness hvalid
    exact hvalid

theorem keygenBreakGame_breaks_iff
    (secret : N32Block × LocalFree -> ZMod q)
    (minBlocks : Nat)
    (witness : N32RecoveredBlocksWitness (q := q) LocalFree) :
    (keygenBreakGame (q := q) secret minBlocks).BreaksKeygen witness ↔
      Valid secret minBlocks witness := by
  rfl

end N32RecoveredBlocksWitness

/--
Linear structural soundness target.

This is the practical endpoint we want future schemes to instantiate.  A
successful forgery is split into a bounded query-miss event, a bounded
scheme-specific structural failure event, or a clean trace that extracts a
keygen-breaking object.

Unlike the current six-replay certificate, this target has no moment/root loss.
-/
structure LinearStructuralSoundnessTarget
    (Trace Extracted Breaker : Type*) [Fintype Trace] where
  traceDist : FiniteProbability Trace
  forgery : Trace -> Bool
  queryMiss : Trace -> Bool
  structuralFailure : Trace -> Bool
  extract : Trace -> Option Extracted
  buildBreaker : Extracted -> Breaker
  BreaksKeygen : Breaker -> Prop
  queryMissBound : ℝ
  structuralFailureBound : ℝ
  queryMiss_acceptProb_le :
    traceDist.acceptProb queryMiss <= queryMissBound
  structuralFailure_acceptProb_le :
    traceDist.acceptProb structuralFailure <= structuralFailureBound
  cleanForgery_extracts_breaker :
    ∀ τ,
      forgery τ = true ->
      queryMiss τ = false ->
      structuralFailure τ = false ->
      ∃ extracted,
        extract τ = some extracted ∧
          BreaksKeygen (buildBreaker extracted)

namespace LinearStructuralSoundnessTarget

variable {Trace Extracted Breaker : Type*}
variable [Fintype Trace]

def forgeryProb
    (T : LinearStructuralSoundnessTarget Trace Extracted Breaker) :
    ℝ :=
  T.traceDist.acceptProb T.forgery

def cleanForgery
    (T : LinearStructuralSoundnessTarget Trace Extracted Breaker)
    (τ : Trace) : Bool :=
  (T.forgery τ && !T.queryMiss τ) && !T.structuralFailure τ

theorem cleanForgery_forgery
    (T : LinearStructuralSoundnessTarget Trace Extracted Breaker)
    {τ : Trace}
    (h : T.cleanForgery τ = true) :
    T.forgery τ = true := by
  cases hf : T.forgery τ <;>
    cases hq : T.queryMiss τ <;>
    cases hb : T.structuralFailure τ <;>
    simp [cleanForgery, hf, hq, hb] at h ⊢

theorem cleanForgery_no_queryMiss
    (T : LinearStructuralSoundnessTarget Trace Extracted Breaker)
    {τ : Trace}
    (h : T.cleanForgery τ = true) :
    T.queryMiss τ = false := by
  cases hf : T.forgery τ <;>
    cases hq : T.queryMiss τ <;>
    cases hb : T.structuralFailure τ <;>
    simp [cleanForgery, hf, hq, hb] at h ⊢

theorem cleanForgery_no_structuralFailure
    (T : LinearStructuralSoundnessTarget Trace Extracted Breaker)
    {τ : Trace}
    (h : T.cleanForgery τ = true) :
    T.structuralFailure τ = false := by
  cases hf : T.forgery τ <;>
    cases hq : T.queryMiss τ <;>
    cases hb : T.structuralFailure τ <;>
    simp [cleanForgery, hf, hq, hb] at h ⊢

theorem cleanForgery_acceptProb_eq_zero_of_no_keygenBreaker
    (T : LinearStructuralSoundnessTarget Trace Extracted Breaker)
    (hNoBreak : ∀ breaker, ¬ T.BreaksKeygen breaker) :
    T.traceDist.acceptProb T.cleanForgery = 0 := by
  apply acceptProb_eq_zero_of_forall_not
  intro τ hclean
  rcases T.cleanForgery_extracts_breaker τ
      (T.cleanForgery_forgery hclean)
      (T.cleanForgery_no_queryMiss hclean)
      (T.cleanForgery_no_structuralFailure hclean) with
    ⟨extracted, _hextract, hbreak⟩
  exact hNoBreak (T.buildBreaker extracted) hbreak

/--
The linear target theorem.

If no keygen breaker exists, then every forgery is charged directly to either
query miss or structural failure.  There is no square root or sixth root.
-/
theorem forgeryProb_le_queryMiss_add_structuralFailure_of_no_keygenBreaker
    (T : LinearStructuralSoundnessTarget Trace Extracted Breaker)
    (hNoBreak : ∀ breaker, ¬ T.BreaksKeygen breaker) :
    T.forgeryProb <= T.queryMissBound + T.structuralFailureBound := by
  have hsplitQuery :=
    acceptProb_le_and_not_add T.traceDist T.forgery T.queryMiss
  have hsplitStructural :=
    acceptProb_le_and_not_add T.traceDist
      (fun τ => T.forgery τ && !T.queryMiss τ)
      T.structuralFailure
  have hclean :
      T.traceDist.acceptProb
          (fun τ => T.forgery τ && !T.queryMiss τ && !T.structuralFailure τ) =
        T.traceDist.acceptProb T.cleanForgery := by
    rfl
  have hzero := T.cleanForgery_acceptProb_eq_zero_of_no_keygenBreaker hNoBreak
  have hstruct :
      T.traceDist.acceptProb (fun τ => T.forgery τ && !T.queryMiss τ) <=
        T.structuralFailureBound := by
    rw [hclean] at hsplitStructural
    linarith [T.structuralFailure_acceptProb_le, hzero]
  unfold forgeryProb
  linarith [hsplitQuery, hstruct, T.queryMiss_acceptProb_le]

end LinearStructuralSoundnessTarget

/--
Byte-concrete N32 instance shape for the linear structural target.

The serialized verifier discharges the canonical query-miss/blind-guess part.
The only algorithmic north-star obligation left here is:

  clean queried byte forgery, outside structural failure,
  extracts an object that breaks key generation.
-/
structure N32ByteLinearStructuralSoundnessInputs
    {State QueryIdx Extracted Breaker : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  forgery : N32QueryTrace State -> Bool
  structuralFailure : N32QueryTrace State -> Bool
  extract : N32QueryTrace State -> Option Extracted
  buildBreaker : Extracted -> Breaker
  BreaksKeygen : Breaker -> Prop
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true
  clean_queried_forgery_extracts_breaker :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      ∃ extracted,
        extract τ = some extracted ∧
          BreaksKeygen (buildBreaker extracted)

namespace N32ByteLinearStructuralSoundnessInputs

variable {State QueryIdx Extracted Breaker : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}

def queryMiss
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker))
    (τ : N32QueryTrace State) : Bool :=
  I.forgery τ && !E.pointQueried τ

def forgerSuccessProb
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker)) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

theorem queryMiss_not_queried
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker))
    {τ : N32QueryTrace State}
    (hmiss : I.queryMiss τ = true) :
    E.pointQueried τ = false := by
  cases hforgery : I.forgery τ <;>
    cases hqueried : E.pointQueried τ <;>
    simp [queryMiss, hforgery, hqueried] at hmiss ⊢

theorem queryMiss_outputs_blind
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker))
    {τ : N32QueryTrace State}
    (hmiss : I.queryMiss τ = true) :
    E.actualChallenge τ = E.blindChallenge τ := by
  have hforgery : I.forgery τ = true := by
    cases hf : I.forgery τ <;>
      cases hp : E.pointQueried τ <;>
      simp [queryMiss, hf, hp] at hmiss ⊢
  have haccept := I.forgery_accepts τ hforgery
  have hrecompute :=
    I.verifier.toSerializedVerifier.recomputedChallenge_matches_actual τ
      (by
        simpa [N32SerializedVerifier.outputAccepts] using haccept)
  simpa [N32QueryLogExperiment.blindChallenge,
    N32QueryLogExperiment.actualChallenge] using hrecompute.symm

def toQueryMissCertificate
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker)) :
    N32QueryLogExperiment.QueryMissCertificate E I.queryMiss where
  queryMiss_not_queried := fun _τ hmiss => I.queryMiss_not_queried hmiss
  queryMiss_outputs_blind := fun _τ hmiss => I.queryMiss_outputs_blind hmiss

theorem queryMiss_acceptProb_le_guess
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker)) :
    E.traceDist.acceptProb I.queryMiss <= n32ChallengeGuessBound := by
  exact
    N32QueryLogExperiment.QueryMissCertificate.queryMiss_acceptProb_le
      I.toQueryMissCertificate

theorem pointQueried_of_forgery_of_not_queryMiss
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker))
    {τ : N32QueryTrace State}
    (hforgery : I.forgery τ = true)
    (hnotMiss : I.queryMiss τ = false) :
    E.pointQueried τ = true := by
  cases hqueried : E.pointQueried τ <;>
    simp [queryMiss, hforgery, hqueried] at hnotMiss ⊢

noncomputable def toLinearTarget
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker)) :
    LinearStructuralSoundnessTarget
      (N32QueryTrace State) Extracted Breaker where
  traceDist := E.traceDist
  forgery := I.forgery
  queryMiss := I.queryMiss
  structuralFailure := I.structuralFailure
  extract := I.extract
  buildBreaker := I.buildBreaker
  BreaksKeygen := I.BreaksKeygen
  queryMissBound := n32ChallengeGuessBound
  structuralFailureBound := I.structuralFailureBound
  queryMiss_acceptProb_le := I.queryMiss_acceptProb_le_guess
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  cleanForgery_extracts_breaker := by
    intro τ hforgery hnotMiss hstructural
    exact
      I.clean_queried_forgery_extracts_breaker τ hforgery
        (I.pointQueried_of_forgery_of_not_queryMiss hforgery hnotMiss)
        hstructural

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
    (I :
      N32ByteLinearStructuralSoundnessInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E
        (Extracted := Extracted) (Breaker := Breaker))
    (hNoBreak : ∀ breaker, ¬ I.BreaksKeygen breaker) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toLinearTarget,
    LinearStructuralSoundnessTarget.forgeryProb] using
    LinearStructuralSoundnessTarget.forgeryProb_le_queryMiss_add_structuralFailure_of_no_keygenBreaker
      I.toLinearTarget hNoBreak

end N32ByteLinearStructuralSoundnessInputs

/--
Byte-concrete N32 linear target specialized to a partial keygen witness.

This is the pivot point: a scheme can now prove security by extracting any
valid partial witness on clean queried forgeries, rather than recovering the
entire product-block secret.
-/
structure N32BytePartialWitnessLinearInputs
    {q : Nat} [NeZero q]
    {State QueryIdx LocalFree PartialWitness Breaker : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (K :
      N32PartialKeygenBreakGame
        (q := q) LocalFree PartialWitness Breaker) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  forgery : N32QueryTrace State -> Bool
  structuralFailure : N32QueryTrace State -> Bool
  extract : N32QueryTrace State -> Option PartialWitness
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true
  clean_queried_forgery_extracts_valid_witness :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      ∃ witness,
        extract τ = some witness ∧
          K.ValidWitness witness

namespace N32BytePartialWitnessLinearInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx LocalFree PartialWitness Breaker : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {K :
  N32PartialKeygenBreakGame
    (q := q) LocalFree PartialWitness Breaker}

def forgerSuccessProb
    (I :
      N32BytePartialWitnessLinearInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

noncomputable def toLinearInputs
    (I :
      N32BytePartialWitnessLinearInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K) :
    N32ByteLinearStructuralSoundnessInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E
      (Extracted := PartialWitness)
      (Breaker := Breaker) where
  verifier := I.verifier
  forgery := I.forgery
  structuralFailure := I.structuralFailure
  extract := I.extract
  buildBreaker := K.buildBreaker
  BreaksKeygen := K.BreaksKeygen
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts
  clean_queried_forgery_extracts_breaker := by
    intro τ hforgery hqueried hstructural
    rcases I.clean_queried_forgery_extracts_valid_witness
        τ hforgery hqueried hstructural with
      ⟨witness, hextract, hvalid⟩
    exact ⟨witness, hextract, K.buildBreaker_sound witness hvalid⟩

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
    (I :
      N32BytePartialWitnessLinearInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K)
    (hNoBreak : ∀ breaker, ¬ K.BreaksKeygen breaker) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toLinearInputs,
    N32ByteLinearStructuralSoundnessInputs.forgerSuccessProb] using
    N32ByteLinearStructuralSoundnessInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
      I.toLinearInputs hNoBreak

end N32BytePartialWitnessLinearInputs

/--
Concrete byte-level one-view target with an extractable mask.

This is the first executable north-star instantiation: if a clean queried
forgery gives the extractor the same mask used by verifier soundness, and the
actual challenge covers all product blocks by signed active coordinates, then
one accepted compact response recovers the product-block secret.
-/
structure N32ByteKnownMaskSingleViewInputs
    {q : Nat} [NeZero q]
    {State QueryIdx LocalFree Breaker : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  compactSoundness :
    N32ByteCompactProductBlockVerifierSoundnessInputs
      (q := q) E verifier K.secret
  forgery : N32QueryTrace State -> Bool
  structuralFailure : N32QueryTrace State -> Bool
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  maskExtractor :
    State -> N32Repetition -> N32Block × LocalFree -> ZMod q
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true
  maskExtractor_eq :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      maskExtractor τ.1 = compactSoundness.mask τ.1
  activeCoverage :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      CoversBlocks fun r b =>
        (E.actualChallenge τ r).scalar (q := q) b = 1 ∨
          (E.actualChallenge τ r).scalar (q := q) b = -1

namespace N32ByteKnownMaskSingleViewInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx LocalFree Breaker : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker}

def forgerSuccessProb
    (I :
      N32ByteKnownMaskSingleViewInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

noncomputable def compactSingleRound
    (I :
      N32ByteKnownMaskSingleViewInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K)
    (τ : N32QueryTrace State)
    (hforgery : I.forgery τ = true)
    (hqueried : E.pointQueried τ = true)
    (hstructural : I.structuralFailure τ = false)
    (r : N32Repetition) :
    CompactSingleViewLinearRound
      (q := q) N32Block LocalFree K.secret where
  chi := fun b => (E.actualChallenge τ r).scalar (q := q) b
  mask := I.maskExtractor τ.1 r
  z := fun b hactive f =>
    I.compactSoundness.responseDecoder.decode (I.verifier.response τ.1 r)
      (E.actualChallenge τ r) b hactive f
  hz := by
    intro b hactive f
    have hacceptOutput := I.forgery_accepts τ hforgery
    have honeShot :
        I.verifier.toSerializedVerifier.oneShotAccepts τ.1
          (E.actualChallenge τ) = true :=
      I.verifier.toSerializedVerifier.oneShotAccepts_of_outputAccepts τ
        hacceptOutput
    have hresp :=
      I.compactSoundness.accepted_response_eq τ.1 (E.actualChallenge τ)
        honeShot r b hactive f
    have hmask := I.maskExtractor_eq τ hforgery hqueried hstructural
    rw [hmask]
    exact hresp

noncomputable def recoveredSecret
    (I :
      N32ByteKnownMaskSingleViewInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K)
    (τ : N32QueryTrace State)
    (hclean :
      I.forgery τ = true ∧ E.pointQueried τ = true ∧
        I.structuralFailure τ = false) :
    N32Block × LocalFree -> ZMod q :=
  CompactSingleViewLinearRound.extractedSecret
    (fun r =>
      I.compactSingleRound τ hclean.1 hclean.2.1 hclean.2.2 r)
    (I.activeCoverage τ hclean.1 hclean.2.1 hclean.2.2)

theorem recoveredSecret_eq_secret
    (I :
      N32ByteKnownMaskSingleViewInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K)
    (τ : N32QueryTrace State)
    (hclean :
      I.forgery τ = true ∧ E.pointQueried τ = true ∧
        I.structuralFailure τ = false) :
    I.recoveredSecret τ hclean = K.secret := by
  unfold recoveredSecret
  exact
    CompactSingleViewLinearRound.extractedSecret_eq
      (fun r =>
        I.compactSingleRound τ hclean.1 hclean.2.1 hclean.2.2 r)
      (I.activeCoverage τ hclean.1 hclean.2.1 hclean.2.2)

noncomputable def extract
    (I :
      N32ByteKnownMaskSingleViewInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K)
    (τ : N32QueryTrace State) :
    Option (N32Block × LocalFree -> ZMod q) :=
  if hclean :
      I.forgery τ = true ∧ E.pointQueried τ = true ∧
        I.structuralFailure τ = false then
    some (I.recoveredSecret τ hclean)
  else
    none

noncomputable def toLinearInputs
    (I :
      N32ByteKnownMaskSingleViewInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K) :
    N32ByteLinearStructuralSoundnessInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E
      (Extracted := N32Block × LocalFree -> ZMod q)
      (Breaker := Breaker) where
  verifier := I.verifier
  forgery := I.forgery
  structuralFailure := I.structuralFailure
  extract := I.extract
  buildBreaker := K.buildBreaker
  BreaksKeygen := K.BreaksKeygen
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts
  clean_queried_forgery_extracts_breaker := by
    intro τ hforgery hqueried hstructural
    let hclean :
        I.forgery τ = true ∧ E.pointQueried τ = true ∧
          I.structuralFailure τ = false :=
      ⟨hforgery, hqueried, hstructural⟩
    refine ⟨I.recoveredSecret τ hclean, ?_, ?_⟩
    · simp [extract, hclean]
    · exact K.buildBreaker_sound (I.recoveredSecret τ hclean)
        (I.recoveredSecret_eq_secret τ hclean)

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
    (I :
      N32ByteKnownMaskSingleViewInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K)
    (hNoBreak : ∀ breaker, ¬ K.BreaksKeygen breaker) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toLinearInputs,
    N32ByteLinearStructuralSoundnessInputs.forgerSuccessProb] using
    N32ByteLinearStructuralSoundnessInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
      I.toLinearInputs hNoBreak

end N32ByteKnownMaskSingleViewInputs

namespace N32ConcreteCompactSixReplayExtractionInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Response LocalFree
  Breaker : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}

theorem exists_keygenBreaker_of_positive_lower_bound
    (K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker)
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E K.secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound)
    (hpositive :
      0 <
        (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
          N32CompactCoverage.failureUnionBound)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    ∃ breaker : Breaker, K.BreaksKeygen breaker := by
  rcases I.exists_extractedSecret_eq_of_positive_lower_bound
      hnonneg hpositive hinvTwo with
    ⟨τ, hτ, hsecret⟩
  let recovered : N32Block × LocalFree -> ZMod q :=
    CompactMultiForkLinearRound.extractedSecret
      (fun r => I.compactSoundness.compactRoundOfReplay τ
        (I.toCompactInputs.rawAllAccepts_of_compactExtractable hτ) r)
      hinvTwo
      (I.toCompactInputs.compactCoverageOfExtractable τ hτ)
  refine ⟨K.buildBreaker recovered, ?_⟩
  exact K.buildBreaker_sound recovered (by
    simpa [recovered] using hsecret)

theorem exists_secretRecoveryBreaker_of_positive_lower_bound
    {secret : N32Block × LocalFree -> ZMod q}
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound)
    (hpositive :
      0 <
        (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
          N32CompactCoverage.failureUnionBound)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    ∃ recovered : N32Block × LocalFree -> ZMod q,
      (N32ProductBlockSecretRecoveryGame.secretRecovery
        (q := q) secret).BreaksKeygen recovered := by
  exact
    I.exists_keygenBreaker_of_positive_lower_bound
      (N32ProductBlockSecretRecoveryGame.secretRecovery (q := q) secret)
      hnonneg hpositive hinvTwo

/--
Contrapositive security form of the compact proof.

If no breaker for the keygen target exists, then the sixth-power replay lower
bound cannot rise above the certified compact coverage loss.
-/
theorem forgeryMinusGuess_sixth_le_failureUnionBound_of_no_keygenBreaker
    (K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker)
    (I : N32ConcreteCompactSixReplayExtractionInputs
      (Response := Response) E K.secret)
    (hNoBreak : ∀ breaker : Breaker, ¬ K.BreaksKeygen breaker)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 <=
      N32CompactCoverage.failureUnionBound := by
  by_contra hnot
  have hgt :
      N32CompactCoverage.failureUnionBound <
        (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 :=
    lt_of_not_ge hnot
  have hpositive :
      0 <
        (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
          N32CompactCoverage.failureUnionBound := by
    linarith
  rcases I.exists_keygenBreaker_of_positive_lower_bound
      K hnonneg hpositive hinvTwo with ⟨breaker, hbreak⟩
  exact hNoBreak breaker hbreak

end N32ConcreteCompactSixReplayExtractionInputs

namespace N32ByteConcreteCompactSixReplayExtractionInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx LocalFree Breaker : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}

theorem exists_keygenBreaker_of_positive_lower_bound
    (K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker)
    (I :
      N32ByteConcreteCompactSixReplayExtractionInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K.secret)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound)
    (hpositive :
      0 <
        (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 -
          N32CompactCoverage.failureUnionBound)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    ∃ breaker : Breaker, K.BreaksKeygen breaker := by
  simpa [forgerSuccessProb] using
    I.toConcreteInputs.exists_keygenBreaker_of_positive_lower_bound
      K hnonneg hpositive hinvTwo

theorem forgeryMinusGuess_sixth_le_failureUnionBound_of_no_keygenBreaker
    (K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker)
    (I :
      N32ByteConcreteCompactSixReplayExtractionInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K.secret)
    (hNoBreak : ∀ breaker : Breaker, ¬ K.BreaksKeygen breaker)
    (hnonneg : 0 <= I.forgerSuccessProb - n32ChallengeGuessBound)
    {invTwo : ZMod q}
    (hinvTwo : invTwo * (2 : ZMod q) = 1) :
    (I.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 <=
      N32CompactCoverage.failureUnionBound := by
  simpa [forgerSuccessProb] using
    N32ConcreteCompactSixReplayExtractionInputs.forgeryMinusGuess_sixth_le_failureUnionBound_of_no_keygenBreaker
      K I.toConcreteInputs hNoBreak hnonneg hinvTwo

end N32ByteConcreteCompactSixReplayExtractionInputs

/--
Final byte-concrete signature security certificate.

All transcript/proof obligations are packaged in `inputs`; the only remaining
cryptographic assumption is that the declared keygen target has no breaker.
-/
structure N32ByteConcreteSignatureSecurityCertificate
    {q : Nat} [NeZero q]
    {State QueryIdx LocalFree Breaker : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker) where
  inputs :
    N32ByteConcreteCompactSixReplayExtractionInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E K.secret
  noKeygenBreak : ∀ breaker : Breaker, ¬ K.BreaksKeygen breaker
  invTwo : ZMod q
  invTwo_spec : invTwo * (2 : ZMod q) = 1

namespace N32ByteConcreteSignatureSecurityCertificate

variable {q : Nat} [NeZero q]
variable {State QueryIdx LocalFree Breaker : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {K : N32ProductBlockSecretRecoveryGame (q := q) LocalFree Breaker}

theorem forgeryMinusGuess_sixth_le_failureUnionBound
    (C :
      N32ByteConcreteSignatureSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K)
    (hnonneg :
      0 <= C.inputs.forgerSuccessProb - n32ChallengeGuessBound) :
    (C.inputs.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 <=
      N32CompactCoverage.failureUnionBound := by
  exact
    C.inputs.forgeryMinusGuess_sixth_le_failureUnionBound_of_no_keygenBreaker
      K C.noKeygenBreak hnonneg C.invTwo_spec

/--
Concrete target for reading the N32 certificate as a direct forgery bound.

The six-replay extractor proves a sixth-moment statement.  To certify a
target residual advantage `targetAdv`, the concrete compact coverage failure
must therefore fit under `targetAdv ^ 6`.
-/
structure PracticalForgeryTarget where
  targetAdv : ℝ
  target_nonneg : 0 <= targetAdv
  failureUnionBound_le_target_sixth :
    N32CompactCoverage.failureUnionBound <= targetAdv ^ 6

theorem forgerSuccess_le_guess_plus_target
    (C :
      N32ByteConcreteSignatureSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E K)
    (T : PracticalForgeryTarget) :
    C.inputs.forgerSuccessProb <= n32ChallengeGuessBound + T.targetAdv := by
  by_cases hnonneg :
      0 <= C.inputs.forgerSuccessProb - n32ChallengeGuessBound
  · have hpow :
        (C.inputs.forgerSuccessProb - n32ChallengeGuessBound) ^ 6 <=
          T.targetAdv ^ 6 :=
      le_trans (C.forgeryMinusGuess_sixth_le_failureUnionBound hnonneg)
        T.failureUnionBound_le_target_sixth
    have hle :
        C.inputs.forgerSuccessProb - n32ChallengeGuessBound <= T.targetAdv :=
      (pow_le_pow_iff_left₀ hnonneg T.target_nonneg
        (by norm_num : (6 : Nat) ≠ 0)).mp hpow
    linarith
  · have hlt :
        C.inputs.forgerSuccessProb - n32ChallengeGuessBound < 0 :=
      lt_of_not_ge hnonneg
    linarith [T.target_nonneg]

end N32ByteConcreteSignatureSecurityCertificate

/--
Replay schedule diagnostics for the six-draw extractor.

A draw fails if it either does not accept or is not bound to the same challenge
point.  The global replay failure event is the union of those six per-draw
failures.
-/
structure N32ReplaySchedule (Trace : Type*) where
  drawAccepts : N32ReplayDraw -> Trace -> Bool
  sameChallengePoint : N32ReplayDraw -> Trace -> Bool

namespace N32ReplaySchedule

variable {Trace : Type*}

def drawGood (S : N32ReplaySchedule Trace) (d : N32ReplayDraw) (τ : Trace) :
    Bool :=
  S.drawAccepts d τ && S.sameChallengePoint d τ

def drawFailure (S : N32ReplaySchedule Trace) (d : N32ReplayDraw) (τ : Trace) :
    Bool :=
  !(S.drawGood d τ)

def acceptFailure (S : N32ReplaySchedule Trace) (d : N32ReplayDraw) (τ : Trace) :
    Bool :=
  !(S.drawAccepts d τ)

def pointFailure (S : N32ReplaySchedule Trace) (d : N32ReplayDraw) (τ : Trace) :
    Bool :=
  S.drawAccepts d τ && !(S.sameChallengePoint d τ)

def replayFailure (S : N32ReplaySchedule Trace) (τ : Trace) : Bool :=
  decide (∃ d, S.drawFailure d τ = true)

theorem replayFailure_implies_drawFailure
    (S : N32ReplaySchedule Trace) :
    ∀ τ, S.replayFailure τ = true -> ∃ d, S.drawFailure d τ = true := by
  intro τ h
  simpa [replayFailure] using h

theorem drawAccepts_of_not_drawFailure
    (S : N32ReplaySchedule Trace) {d : N32ReplayDraw} {τ : Trace}
    (h : S.drawFailure d τ = false) :
    S.drawAccepts d τ = true := by
  cases ha : S.drawAccepts d τ <;>
    cases hp : S.sameChallengePoint d τ <;>
    simp [drawFailure, drawGood, ha, hp] at h ⊢

theorem sameChallengePoint_of_not_drawFailure
    (S : N32ReplaySchedule Trace) {d : N32ReplayDraw} {τ : Trace}
    (h : S.drawFailure d τ = false) :
    S.sameChallengePoint d τ = true := by
  cases ha : S.drawAccepts d τ <;>
    cases hp : S.sameChallengePoint d τ <;>
    simp [drawFailure, drawGood, ha, hp] at h ⊢

theorem drawFailure_implies_accept_or_pointFailure
    (S : N32ReplaySchedule Trace) (d : N32ReplayDraw) :
    ∀ τ,
      S.drawFailure d τ = true ->
      (S.acceptFailure d τ || S.pointFailure d τ) = true := by
  intro τ h
  cases ha : S.drawAccepts d τ <;>
    cases hp : S.sameChallengePoint d τ <;>
    simp [drawFailure, drawGood, acceptFailure, pointFailure, ha, hp] at h ⊢

theorem drawFailureProb_le_components
    [Fintype Trace]
    (D : FiniteProbability Trace)
    (S : N32ReplaySchedule Trace)
    (d : N32ReplayDraw) :
    D.acceptProb (S.drawFailure d) <=
      D.acceptProb (S.acceptFailure d) +
        D.acceptProb (S.pointFailure d) := by
  have hmono :
      D.acceptProb (S.drawFailure d) <=
        D.acceptProb
          (fun τ => S.acceptFailure d τ || S.pointFailure d τ) :=
    acceptProb_mono D (S.drawFailure_implies_accept_or_pointFailure d)
  have hunion :=
    acceptProb_or_le_add D (S.acceptFailure d) (S.pointFailure d)
  exact le_trans hmono hunion

theorem acceptFailure_upper_zero_of_always
    [Fintype Trace]
    (D : FiniteProbability Trace)
    (S : N32ReplaySchedule Trace)
    (d : N32ReplayDraw)
    (hAccepts : ∀ τ, S.drawAccepts d τ = true) :
    D.acceptProb (S.acceptFailure d) <= 0 := by
  have hzero : D.acceptProb (S.acceptFailure d) = 0 := by
    unfold FiniteProbability.acceptProb
    apply Finset.sum_eq_zero
    intro τ _hτ
    simp [acceptFailure, hAccepts τ]
  exact le_of_eq hzero

theorem pointFailure_upper_zero_of_always
    [Fintype Trace]
    (D : FiniteProbability Trace)
    (S : N32ReplaySchedule Trace)
    (d : N32ReplayDraw)
    (hSamePoint : ∀ τ, S.drawAccepts d τ = true ->
      S.sameChallengePoint d τ = true) :
    D.acceptProb (S.pointFailure d) <= 0 := by
  have hzero : D.acceptProb (S.pointFailure d) = 0 := by
    unfold FiniteProbability.acceptProb
    apply Finset.sum_eq_zero
    intro τ _hτ
    cases hacc : S.drawAccepts d τ
    · simp [pointFailure, hacc]
    · simp [pointFailure, hacc, hSamePoint τ hacc]
  exact le_of_eq hzero

end N32ReplaySchedule

/--
Byte-level replay binding.

An accepting replay draw is bound to the same challenge point when the verifier
checks equality of the fixed-layout byte encodings.  Injectivity of the byte
codec then converts byte equality into semantic equality of the public key,
message, and seven commitments.
-/
structure N32ByteReplayBinding
    (Trace PublicKey Message Commitment : Type*) where
  codec : N32ChallengePointByteCodec PublicKey Message Commitment
  expectedPoint : Trace -> N32ChallengePoint PublicKey Message Commitment
  drawPoint : N32ReplayDraw -> Trace ->
    N32ChallengePoint PublicKey Message Commitment
  drawAccepts : N32ReplayDraw -> Trace -> Bool
  bytes_match_on_accept :
    ∀ d τ,
      drawAccepts d τ = true ->
      codec.encodePoint (drawPoint d τ) = codec.encodePoint (expectedPoint τ)

namespace N32ByteReplayBinding

variable {Trace PublicKey Message Commitment : Type*}

def toReplaySchedule
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (B : N32ByteReplayBinding Trace PublicKey Message Commitment) :
    N32ReplaySchedule Trace where
  drawAccepts := B.drawAccepts
  sameChallengePoint := fun d τ =>
    decide (B.drawPoint d τ = B.expectedPoint τ)

theorem sameChallengePoint_of_accepts
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (B : N32ByteReplayBinding Trace PublicKey Message Commitment)
    (d : N32ReplayDraw) (τ : Trace)
    (hAccepts : (B.toReplaySchedule).drawAccepts d τ = true) :
    (B.toReplaySchedule).sameChallengePoint d τ = true := by
  have hbytes := B.bytes_match_on_accept d τ hAccepts
  have hpoint := B.codec.encodePoint_injective hbytes
  simp [toReplaySchedule, hpoint]

theorem pointFailure_upper_zero
    [Fintype Trace]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (D : FiniteProbability Trace)
    (B : N32ByteReplayBinding Trace PublicKey Message Commitment)
    (d : N32ReplayDraw) :
    D.acceptProb ((B.toReplaySchedule).pointFailure d) <= 0 :=
  N32ReplaySchedule.pointFailure_upper_zero_of_always D B.toReplaySchedule d
    (B.sameChallengePoint_of_accepts d)

end N32ByteReplayBinding

/--
Accepting-replay density profile.

For draw `d` and adversary-side state `state`, `acceptingChallenge d state`
is the set of programmed canonical challenges that make the replay accept.
The field `acceptProbabilityLower d` is a per-draw lower bound on that set's
uniform density for every state.
-/
structure N32ReplayAcceptanceProfile
    {State QueryIdx PublicKey Message Commitment : Type*}
    [Fintype State]
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (B :
      N32ByteReplayBinding
        (N32QueryTrace State) PublicKey Message Commitment) where
  acceptingChallenge :
    N32ReplayDraw -> State -> N32CanonicalChallenge -> Bool
  drawAccepts_eq :
    ∀ d τ, B.drawAccepts d τ = acceptingChallenge d τ.1 τ.2
  acceptProbabilityLower : N32ReplayDraw -> ℝ
  acceptProbability_lower :
    ∀ d state,
      acceptProbabilityLower d <=
        (uniformFiniteProbability N32CanonicalChallenge).acceptProb
          (acceptingChallenge d state)

namespace N32ReplayAcceptanceProfile

variable {State QueryIdx PublicKey Message Commitment : Type*}
variable [Fintype State]
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {B :
  N32ByteReplayBinding
    (N32QueryTrace State) PublicKey Message Commitment}

theorem acceptFailure_upper
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (P : N32ReplayAcceptanceProfile E B)
    (d : N32ReplayDraw) :
    E.traceDist.acceptProb ((B.toReplaySchedule).acceptFailure d) <=
      1 - P.acceptProbabilityLower d := by
  have hbase :=
    n32_product_acceptFailure_le_one_sub_lower
      E.stateDist (P.acceptingChallenge d)
      (P.acceptProbabilityLower d)
      (P.acceptProbability_lower d)
  have hp :
      (fun τ => (B.toReplaySchedule).acceptFailure d τ) =
        (fun τ => !(P.acceptingChallenge d τ.1 τ.2)) := by
    funext τ
    simp [N32ByteReplayBinding.toReplaySchedule,
      N32ReplaySchedule.acceptFailure, P.drawAccepts_eq]
  change E.traceDist.acceptProb
      (fun τ => (B.toReplaySchedule).acceptFailure d τ) <=
    1 - P.acceptProbabilityLower d
  rw [hp]
  simpa [N32QueryLogExperiment.traceDist] using hbase

end N32ReplayAcceptanceProfile

/--
Accepting-replay density profile restricted to heavy states.

This is the usable replay-density interface for a Fiat-Shamir forking proof.
Global forger success does not imply that every adversary-side state has large
accepting-challenge density.  Instead the proof chooses a heavy-state predicate,
proves the density lower bound on those states, and separately pays for the
light-state replay loss.
-/
structure N32HeavyReplayAcceptanceProfile
    {State QueryIdx PublicKey Message Commitment : Type*}
    [Fintype State]
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (B :
      N32ByteReplayBinding
        (N32QueryTrace State) PublicKey Message Commitment) where
  heavyState : State -> Bool
  acceptingChallenge :
    N32ReplayDraw -> State -> N32CanonicalChallenge -> Bool
  drawAccepts_eq :
    ∀ d τ, B.drawAccepts d τ = acceptingChallenge d τ.1 τ.2
  acceptProbabilityLower : N32ReplayDraw -> ℝ
  acceptProbabilityLower_le_one :
    ∀ d, acceptProbabilityLower d <= 1
  heavy_acceptProbability_lower :
    ∀ d state,
      heavyState state = true ->
      acceptProbabilityLower d <=
        (uniformFiniteProbability N32CanonicalChallenge).acceptProb
          (acceptingChallenge d state)

namespace N32HeavyReplayAcceptanceProfile

variable {State QueryIdx PublicKey Message Commitment : Type*}
variable [Fintype State]
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {B :
  N32ByteReplayBinding
    (N32QueryTrace State) PublicKey Message Commitment}

def heavyAcceptFailure
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (P : N32HeavyReplayAcceptanceProfile E B)
    (d : N32ReplayDraw)
    (τ : N32QueryTrace State) : Bool :=
  P.heavyState τ.1 && (B.toReplaySchedule).acceptFailure d τ

def heavyDrawFailure
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (P : N32HeavyReplayAcceptanceProfile E B)
    (d : N32ReplayDraw)
    (τ : N32QueryTrace State) : Bool :=
  P.heavyState τ.1 && (B.toReplaySchedule).drawFailure d τ

theorem heavyAcceptFailure_upper
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (P : N32HeavyReplayAcceptanceProfile E B)
    (d : N32ReplayDraw) :
    E.traceDist.acceptProb (P.heavyAcceptFailure d) <=
      1 - P.acceptProbabilityLower d := by
  have hbase :=
    n32_product_heavy_acceptFailure_le_one_sub_lower
      E.stateDist P.heavyState (P.acceptingChallenge d)
      (P.acceptProbabilityLower d)
      (P.acceptProbabilityLower_le_one d)
      (P.heavy_acceptProbability_lower d)
  have hp :
      (fun τ => P.heavyAcceptFailure d τ) =
        (fun τ =>
          P.heavyState τ.1 &&
            !(P.acceptingChallenge d τ.1 τ.2)) := by
    funext τ
    simp [heavyAcceptFailure, N32ByteReplayBinding.toReplaySchedule,
      N32ReplaySchedule.acceptFailure, P.drawAccepts_eq]
  change E.traceDist.acceptProb
      (fun τ => P.heavyAcceptFailure d τ) <=
    1 - P.acceptProbabilityLower d
  rw [hp]
  simpa [N32QueryLogExperiment.traceDist] using hbase

theorem heavyDrawFailure_upper
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    (P : N32HeavyReplayAcceptanceProfile E B)
    (d : N32ReplayDraw) :
    E.traceDist.acceptProb (P.heavyDrawFailure d) <=
      1 - P.acceptProbabilityLower d := by
  have hmono :
      E.traceDist.acceptProb (P.heavyDrawFailure d) <=
      E.traceDist.acceptProb
          (fun τ =>
            P.heavyAcceptFailure d τ ||
              (B.toReplaySchedule).pointFailure d τ) :=
    acceptProb_mono E.traceDist (fun τ hdraw => by
      have hfailure :
          (B.toReplaySchedule).drawFailure d τ = true := by
        cases hheavy : P.heavyState τ.1 <;>
          cases hdf : (B.toReplaySchedule).drawFailure d τ <;>
          simp [heavyDrawFailure, hheavy, hdf] at hdraw ⊢
      have hheavyTrue : P.heavyState τ.1 = true := by
        cases hheavy : P.heavyState τ.1 <;>
          cases hdf : (B.toReplaySchedule).drawFailure d τ <;>
          simp [heavyDrawFailure, hheavy, hdf] at hdraw ⊢
      have hcomponents :=
        (B.toReplaySchedule).drawFailure_implies_accept_or_pointFailure
          d τ hfailure
      cases hacc : (B.toReplaySchedule).acceptFailure d τ <;>
        cases hp : (B.toReplaySchedule).pointFailure d τ <;>
        simp [heavyAcceptFailure, hheavyTrue, hacc, hp] at hcomponents ⊢)
  have hunion :=
    acceptProb_or_le_add E.traceDist
      (P.heavyAcceptFailure d)
      ((B.toReplaySchedule).pointFailure d)
  have haccept := P.heavyAcceptFailure_upper d
  have hpoint := B.pointFailure_upper_zero E.traceDist d
  linarith

end N32HeavyReplayAcceptanceProfile

/--
Per-draw replay bounds split into the two concrete failure mechanisms:
the draw does not accept, or it accepts but is not bound to the same challenge
point.
-/
structure N32ReplayScheduleBounds
    {Trace : Type*} [Fintype Trace]
    (D : FiniteProbability Trace)
    (S : N32ReplaySchedule Trace) where
  acceptFailureBound : N32ReplayDraw -> ℝ
  pointFailureBound : N32ReplayDraw -> ℝ
  acceptFailure_upper :
    ∀ d, D.acceptProb (S.acceptFailure d) <= acceptFailureBound d
  pointFailure_upper :
    ∀ d, D.acceptProb (S.pointFailure d) <= pointFailureBound d

namespace N32ReplayScheduleBounds

variable {Trace : Type*} [Fintype Trace]
variable {D : FiniteProbability Trace}
variable {S : N32ReplaySchedule Trace}

def drawFailureBound (B : N32ReplayScheduleBounds D S) (d : N32ReplayDraw) :
    ℝ :=
  B.acceptFailureBound d + B.pointFailureBound d

theorem drawFailure_upper
    (B : N32ReplayScheduleBounds D S) :
    ∀ d, D.acceptProb (S.drawFailure d) <= B.drawFailureBound d := by
  intro d
  have hbase := S.drawFailureProb_le_components D d
  have ha := B.acceptFailure_upper d
  have hp := B.pointFailure_upper d
  unfold drawFailureBound
  linarith

end N32ReplayScheduleBounds

/--
Concrete transcript inputs needed to instantiate the abstract six-draw forking
certificate for the `n = 32`, `255 B` challenge space.

This packages the actual query-log proof, replay-draw decomposition, per-draw
probability bounds, and compact coverage bound into the shape expected by
`SixDrawForkingBounds`.
-/
structure N32SixDrawForkingInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    {secret : Block × LocalFree -> ZMod q}
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (G :
      MultiForkFiatShamirGame
        (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret)
    (R : RandomOracleMultiForkReplay G) where
  traceDist_eq : G.traceDist = E.traceDist
  queryMiss_not_queried :
    ∀ τ, R.queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, R.queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ
  schedule : N32ReplaySchedule (N32QueryTrace State)
  replayFailure_implies_schedule :
    ∀ τ, R.replayFailure τ = true -> schedule.replayFailure τ = true
  drawFailureBound : N32ReplayDraw -> ℝ
  drawFailure_upper :
    ∀ d, G.traceDist.acceptProb (schedule.drawFailure d) <= drawFailureBound d
  coverageFailureBound : ℝ
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace N32SixDrawForkingInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {secret : Block × LocalFree -> ZMod q}
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {G :
  MultiForkFiatShamirGame
    (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret}
variable {R : RandomOracleMultiForkReplay G}

def toQueryMissCertificate
    (I : N32SixDrawForkingInputs E G R) :
    N32QueryLogExperiment.QueryMissCertificate E R.queryMiss where
  queryMiss_not_queried := I.queryMiss_not_queried
  queryMiss_outputs_blind := I.queryMiss_outputs_blind

def toSixDrawForkingBounds
    (I : N32SixDrawForkingInputs E G R) :
    RandomOracleMultiForkReplay.SixDrawForkingBounds R where
  queryGuess := E.blindGuessEvent
  drawFailure := I.schedule.drawFailure
  challengeGuessBound := n32ChallengeGuessBound
  drawFailureBound := I.drawFailureBound
  coverageFailureBound := I.coverageFailureBound
  queryMiss_implies_guess := by
    intro τ hmiss
    have hblind := I.queryMiss_outputs_blind τ hmiss
    simpa [N32QueryLogExperiment.blindGuessEvent,
      N32QueryLogExperiment.actualChallenge,
      N32QueryLogExperiment.blindChallenge] using hblind
  replayFailure_implies_drawFailure := by
    intro τ hreplay
    exact I.schedule.replayFailure_implies_drawFailure τ
      (I.replayFailure_implies_schedule τ hreplay)
  queryGuess_upper := by
    rw [I.traceDist_eq]
    exact le_of_eq E.blindGuessEvent_acceptProb
  drawFailure_upper := I.drawFailure_upper
  coverageFailure_upper := I.coverageFailure_upper

theorem extractableProb_ge
    (I : N32SixDrawForkingInputs E G R) :
    R.forgerSuccessProb - n32ChallengeGuessBound -
        (Finset.univ.sum I.drawFailureBound) - I.coverageFailureBound <=
      G.extractableProb := by
  simpa [RandomOracleMultiForkReplay.SixDrawForkingBounds.replayFailureBound,
    toSixDrawForkingBounds] using
    (RandomOracleMultiForkReplay.SixDrawForkingBounds.extractableProb_ge
      (I.toSixDrawForkingBounds))

end N32SixDrawForkingInputs

/--
Version of the six-draw transcript certificate that asks for component replay
bounds instead of a precombined `drawFailureBound`.
-/
structure N32SixDrawComponentInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    {secret : Block × LocalFree -> ZMod q}
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (G :
      MultiForkFiatShamirGame
        (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret)
    (R : RandomOracleMultiForkReplay G) where
  traceDist_eq : G.traceDist = E.traceDist
  queryMiss_not_queried :
    ∀ τ, R.queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, R.queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ
  schedule : N32ReplaySchedule (N32QueryTrace State)
  replayFailure_implies_schedule :
    ∀ τ, R.replayFailure τ = true -> schedule.replayFailure τ = true
  acceptFailureBound : N32ReplayDraw -> ℝ
  pointFailureBound : N32ReplayDraw -> ℝ
  acceptFailure_upper :
    ∀ d, G.traceDist.acceptProb (schedule.acceptFailure d) <=
      acceptFailureBound d
  pointFailure_upper :
    ∀ d, G.traceDist.acceptProb (schedule.pointFailure d) <=
      pointFailureBound d
  coverageFailureBound : ℝ
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace N32SixDrawComponentInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {secret : Block × LocalFree -> ZMod q}
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {G :
  MultiForkFiatShamirGame
    (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret}
variable {R : RandomOracleMultiForkReplay G}

def drawFailureBound
    (I : N32SixDrawComponentInputs E G R)
    (d : N32ReplayDraw) : ℝ :=
  I.acceptFailureBound d + I.pointFailureBound d

def replayScheduleBounds
    (I : N32SixDrawComponentInputs E G R) :
    N32ReplayScheduleBounds G.traceDist I.schedule where
  acceptFailureBound := I.acceptFailureBound
  pointFailureBound := I.pointFailureBound
  acceptFailure_upper := I.acceptFailure_upper
  pointFailure_upper := I.pointFailure_upper

def toSixDrawForkingInputs
    (I : N32SixDrawComponentInputs E G R) :
    N32SixDrawForkingInputs E G R where
  traceDist_eq := I.traceDist_eq
  queryMiss_not_queried := I.queryMiss_not_queried
  queryMiss_outputs_blind := I.queryMiss_outputs_blind
  schedule := I.schedule
  replayFailure_implies_schedule := I.replayFailure_implies_schedule
  drawFailureBound := I.drawFailureBound
  drawFailure_upper := by
    intro d
    exact I.replayScheduleBounds.drawFailure_upper d
  coverageFailureBound := I.coverageFailureBound
  coverageFailure_upper := I.coverageFailure_upper

theorem extractableProb_ge
    (I : N32SixDrawComponentInputs E G R) :
    R.forgerSuccessProb - n32ChallengeGuessBound -
        (Finset.univ.sum fun d =>
          I.acceptFailureBound d + I.pointFailureBound d) -
        I.coverageFailureBound <=
      G.extractableProb := by
  simpa [drawFailureBound, toSixDrawForkingInputs] using
    (N32SixDrawForkingInputs.extractableProb_ge
      (I.toSixDrawForkingInputs))

end N32SixDrawComponentInputs

/--
Six-draw transcript certificate with byte-level same-point binding.

The byte binding proof discharges every `pointFailureBound[d]` with zero, so
the only replay terms left are the six accepting-replay failure bounds.
-/
structure N32SixDrawByteBindingInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    {secret : Block × LocalFree -> ZMod q}
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (G :
      MultiForkFiatShamirGame
        (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret)
    (R : RandomOracleMultiForkReplay G) where
  traceDist_eq : G.traceDist = E.traceDist
  queryMiss_not_queried :
    ∀ τ, R.queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, R.queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ
  binding :
    N32ByteReplayBinding
      (N32QueryTrace State) PublicKey Message Commitment
  replayFailure_implies_schedule :
    ∀ τ, R.replayFailure τ = true ->
      binding.toReplaySchedule.replayFailure τ = true
  acceptFailureBound : N32ReplayDraw -> ℝ
  acceptFailure_upper :
    ∀ d,
      G.traceDist.acceptProb
          (binding.toReplaySchedule.acceptFailure d) <=
        acceptFailureBound d
  coverageFailureBound : ℝ
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace N32SixDrawByteBindingInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {secret : Block × LocalFree -> ZMod q}
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {G :
  MultiForkFiatShamirGame
    (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret}
variable {R : RandomOracleMultiForkReplay G}

def toComponentInputs
    (I : N32SixDrawByteBindingInputs E G R) :
    N32SixDrawComponentInputs E G R where
  traceDist_eq := I.traceDist_eq
  queryMiss_not_queried := I.queryMiss_not_queried
  queryMiss_outputs_blind := I.queryMiss_outputs_blind
  schedule := I.binding.toReplaySchedule
  replayFailure_implies_schedule := I.replayFailure_implies_schedule
  acceptFailureBound := I.acceptFailureBound
  pointFailureBound := fun _ => 0
  acceptFailure_upper := I.acceptFailure_upper
  pointFailure_upper := by
    intro d
    exact I.binding.pointFailure_upper_zero G.traceDist d
  coverageFailureBound := I.coverageFailureBound
  coverageFailure_upper := I.coverageFailure_upper

theorem extractableProb_ge
    (I : N32SixDrawByteBindingInputs E G R) :
    R.forgerSuccessProb - n32ChallengeGuessBound -
        (Finset.univ.sum I.acceptFailureBound) -
        I.coverageFailureBound <=
      G.extractableProb := by
  simpa [toComponentInputs] using
    (N32SixDrawComponentInputs.extractableProb_ge
      (I.toComponentInputs))

end N32SixDrawByteBindingInputs

/--
Six-draw transcript certificate where accepting-replay failures are bounded by
per-state accepting-challenge densities.
-/
structure N32SixDrawAcceptanceDensityInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    {secret : Block × LocalFree -> ZMod q}
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (G :
      MultiForkFiatShamirGame
        (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret)
    (R : RandomOracleMultiForkReplay G) where
  traceDist_eq : G.traceDist = E.traceDist
  queryMiss_not_queried :
    ∀ τ, R.queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, R.queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ
  binding :
    N32ByteReplayBinding
      (N32QueryTrace State) PublicKey Message Commitment
  replayFailure_implies_schedule :
    ∀ τ, R.replayFailure τ = true ->
      binding.toReplaySchedule.replayFailure τ = true
  acceptance :
    N32ReplayAcceptanceProfile E binding
  coverageFailureBound : ℝ
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace N32SixDrawAcceptanceDensityInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {secret : Block × LocalFree -> ZMod q}
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {G :
  MultiForkFiatShamirGame
    (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret}
variable {R : RandomOracleMultiForkReplay G}

def acceptFailureBound
    (I : N32SixDrawAcceptanceDensityInputs E G R)
    (d : N32ReplayDraw) : ℝ :=
  1 - I.acceptance.acceptProbabilityLower d

def toByteBindingInputs
    (I : N32SixDrawAcceptanceDensityInputs E G R) :
    N32SixDrawByteBindingInputs E G R where
  traceDist_eq := I.traceDist_eq
  queryMiss_not_queried := I.queryMiss_not_queried
  queryMiss_outputs_blind := I.queryMiss_outputs_blind
  binding := I.binding
  replayFailure_implies_schedule := I.replayFailure_implies_schedule
  acceptFailureBound := I.acceptFailureBound
  acceptFailure_upper := by
    intro d
    rw [I.traceDist_eq]
    exact I.acceptance.acceptFailure_upper d
  coverageFailureBound := I.coverageFailureBound
  coverageFailure_upper := I.coverageFailure_upper

theorem extractableProb_ge
    (I : N32SixDrawAcceptanceDensityInputs E G R) :
    R.forgerSuccessProb - n32ChallengeGuessBound -
        (Finset.univ.sum fun d =>
          (1 : ℝ) - I.acceptance.acceptProbabilityLower d) -
        I.coverageFailureBound <=
      G.extractableProb := by
  simpa [acceptFailureBound, toByteBindingInputs] using
    (N32SixDrawByteBindingInputs.extractableProb_ge
      (I.toByteBindingInputs))

end N32SixDrawAcceptanceDensityInputs

/--
Heavy-state six-draw certificate.

This is the replay-density form that can actually be derived from a forger.
The proof chooses a heavy-state predicate, pays one `lightReplayBound` for
traces outside that predicate, and uses the per-draw `alpha[d]` bounds only
inside heavy states.
-/
structure N32SixDrawHeavyAcceptanceInputs
    {q : Nat} [NeZero q]
    {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
    {secret : Block × LocalFree -> ZMod q}
    (E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment)
    (G :
      MultiForkFiatShamirGame
        (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret)
    (R : RandomOracleMultiForkReplay G) where
  traceDist_eq : G.traceDist = E.traceDist
  queryMiss_not_queried :
    ∀ τ, R.queryMiss τ = true -> E.pointQueried τ = false
  queryMiss_outputs_blind :
    ∀ τ, R.queryMiss τ = true -> E.actualChallenge τ = E.blindChallenge τ
  binding :
    N32ByteReplayBinding
      (N32QueryTrace State) PublicKey Message Commitment
  heavyAcceptance :
    N32HeavyReplayAcceptanceProfile E binding
  lightReplay : N32QueryTrace State -> Bool
  lightReplayBound : ℝ
  lightReplay_upper :
    G.traceDist.acceptProb lightReplay <= lightReplayBound
  replayFailure_implies_light_or_schedule :
    ∀ τ,
      R.replayFailure τ = true ->
        lightReplay τ = true ∨
          (heavyAcceptance.heavyState τ.1 = true ∧
            binding.toReplaySchedule.replayFailure τ = true)
  coverageFailureBound : ℝ
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace N32SixDrawHeavyAcceptanceInputs

variable {q : Nat} [NeZero q]
variable {State QueryIdx PublicKey Message Commitment Round Block LocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [DecidableEq (N32ChallengePoint PublicKey Message Commitment)]
variable {secret : Block × LocalFree -> ZMod q}
variable {E : N32QueryLogExperiment State QueryIdx PublicKey Message Commitment}
variable {G :
  MultiForkFiatShamirGame
    (N32QueryTrace State) Round N32ReplayDraw Block LocalFree secret}
variable {R : RandomOracleMultiForkReplay G}

def replayFailureBound
    (I : N32SixDrawHeavyAcceptanceInputs E G R) : ℝ :=
  I.lightReplayBound +
    Finset.univ.sum fun d =>
      (1 : ℝ) - I.heavyAcceptance.acceptProbabilityLower d

def heavyDrawFailureExists
    (I : N32SixDrawHeavyAcceptanceInputs E G R)
    (τ : N32QueryTrace State) : Bool :=
  decide (∃ d : N32ReplayDraw,
    I.heavyAcceptance.heavyDrawFailure d τ = true)

def toRandomOracleForkingBounds
    (I : N32SixDrawHeavyAcceptanceInputs E G R) :
    RandomOracleMultiForkReplay.RandomOracleForkingBounds R where
  queryMissBound := n32ChallengeGuessBound
  replayFailureBound := I.replayFailureBound
  coverageFailureBound := I.coverageFailureBound
  queryMiss_upper := by
    unfold RandomOracleMultiForkReplay.queryMissProb
    rw [I.traceDist_eq]
    exact
      (N32QueryLogExperiment.QueryMissCertificate.queryMiss_acceptProb_le
        ({ queryMiss_not_queried := I.queryMiss_not_queried
           queryMiss_outputs_blind := I.queryMiss_outputs_blind } :
          N32QueryLogExperiment.QueryMissCertificate E R.queryMiss))
  replayFailure_upper := by
    unfold RandomOracleMultiForkReplay.replayFailureProb
    have hmono :
        G.traceDist.acceptProb R.replayFailure <=
          G.traceDist.acceptProb
            (fun τ => I.lightReplay τ || I.heavyDrawFailureExists τ) :=
      acceptProb_mono G.traceDist (fun τ hreplay => by
        rcases I.replayFailure_implies_light_or_schedule τ hreplay with
          hlight | hheavySchedule
        · simp [hlight]
        · rcases hheavySchedule with ⟨hheavy, hschedule⟩
          rcases I.binding.toReplaySchedule.replayFailure_implies_drawFailure
              τ hschedule with ⟨d, hdraw⟩
          have hd :
              I.heavyAcceptance.heavyDrawFailure d τ = true := by
            simp [N32HeavyReplayAcceptanceProfile.heavyDrawFailure,
              hheavy, hdraw]
          simp [heavyDrawFailureExists]
          exact Or.inr ⟨d, hd⟩)
    have hunion :
        G.traceDist.acceptProb
            (fun τ => I.lightReplay τ || I.heavyDrawFailureExists τ) <=
          G.traceDist.acceptProb I.lightReplay +
            G.traceDist.acceptProb I.heavyDrawFailureExists :=
      acceptProb_or_le_add G.traceDist I.lightReplay I.heavyDrawFailureExists
    have hexists :
        G.traceDist.acceptProb I.heavyDrawFailureExists <=
          Finset.univ.sum fun d =>
            G.traceDist.acceptProb
              (fun τ => I.heavyAcceptance.heavyDrawFailure d τ) :=
      acceptProb_le_sum_of_exists G.traceDist
        I.heavyDrawFailureExists
        (fun d τ => I.heavyAcceptance.heavyDrawFailure d τ)
        (by
          intro τ hτ
          simpa [heavyDrawFailureExists] using hτ)
    have hdraws :
        (Finset.univ.sum fun d =>
            G.traceDist.acceptProb
              (fun τ => I.heavyAcceptance.heavyDrawFailure d τ)) <=
          Finset.univ.sum fun d =>
            (1 : ℝ) - I.heavyAcceptance.acceptProbabilityLower d := by
      apply Finset.sum_le_sum
      intro d _hd
      rw [I.traceDist_eq]
      exact I.heavyAcceptance.heavyDrawFailure_upper d
    have hlight := I.lightReplay_upper
    unfold replayFailureBound
    linarith
  coverageFailure_upper := I.coverageFailure_upper

theorem extractableProb_ge
    (I : N32SixDrawHeavyAcceptanceInputs E G R) :
    R.forgerSuccessProb - n32ChallengeGuessBound -
        I.lightReplayBound -
        (Finset.univ.sum fun d =>
          (1 : ℝ) - I.heavyAcceptance.acceptProbabilityLower d) -
        I.coverageFailureBound <=
      G.extractableProb := by
  have h :=
    RandomOracleMultiForkReplay.RandomOracleForkingBounds.extractableProb_ge
      (I.toRandomOracleForkingBounds)
  change R.forgerSuccessProb - n32ChallengeGuessBound -
      I.replayFailureBound - I.coverageFailureBound <=
    G.extractableProb at h
  unfold replayFailureBound at h
  linarith

end N32SixDrawHeavyAcceptanceInputs

end

end SignatureTranscript

end Descender
