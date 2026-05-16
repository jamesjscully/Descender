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

/-- Sixteen product blocks in the `n = 32` row. -/
abbrev N32Block := Fin 16

/-- Seven repetitions in the current `n = 32`, `255 B` row. -/
abbrev N32Repetition := Fin 7

/-- Six replay draws for the compact multi-fork extractor. -/
abbrev N32ReplayDraw := Fin 6

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

end N32RoundChallenge

theorem n32ActiveBlock_card (omitted : N32Block) :
    Fintype.card (N32ActiveBlock omitted) = 15 := by
  simp [N32ActiveBlock, Fintype.card_subtype_compl]

theorem n32RoundChallenge_card :
    Fintype.card N32RoundChallenge = 16 * 2 ^ 15 := by
  norm_num [N32RoundChallenge, N32ActiveBlock, Fintype.card_sigma,
    Fintype.card_subtype_compl]

theorem n32CanonicalChallenge_card :
    Fintype.card N32CanonicalChallenge = 2 ^ 133 := by
  norm_num [N32CanonicalChallenge, N32RoundChallenge, N32ActiveBlock,
    Fintype.card_sigma, Fintype.card_subtype_compl]

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

end

end SignatureTranscript

end Descender
