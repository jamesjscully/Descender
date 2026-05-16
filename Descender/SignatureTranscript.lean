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

end

end SignatureTranscript

end Descender
