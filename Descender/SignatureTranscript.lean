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

end N32ReplaySchedule

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

end

end SignatureTranscript

end Descender
