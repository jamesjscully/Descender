import Mathlib.Tactic
import Descender.Advantage
import Descender.BlockChallengeSoundness

namespace Descender

/-!
Probabilistic Fiat-Shamir multi-fork boundary for product-block signatures.

`BlockChallengeSoundness` proves the deterministic extractor: if repeated
multi-fork rounds contain opposite signed active openings covering every block,
then the product-block secret is recovered.  This file records the probability
layer that has to be supplied by a Fiat-Shamir/forking analysis.

The theorem here is deliberately finite and explicit.  A forking game provides
a finite trace distribution, an event saying the rewinding/forking procedure
obtained accepting same-commitment draws, and a coverage-failure event.  The
proved loss accounting is:

  Pr[extractable trace] >= Pr[accepting same-commitment forks]
                         - Pr[compact fork-delta coverage failure].
-/

namespace SignatureForking

open Advantage
open BlockChallengeSoundness

noncomputable section

variable {Ω : Type*} [Fintype Ω]

theorem boolIndicator_le_and_not_add
    (A B : Bool) :
    boolIndicator A <= boolIndicator (A && !B) + boolIndicator B := by
  cases A <;> cases B <;> simp [boolIndicator]

theorem boolIndicator_mono {A B : Bool}
    (h : A = true -> B = true) :
    boolIndicator A <= boolIndicator B := by
  cases A <;> cases B <;> simp [boolIndicator] at h ⊢

theorem boolIndicator_or_le_add
    (A B : Bool) :
    boolIndicator (A || B) <= boolIndicator A + boolIndicator B := by
  cases A <;> cases B <;> simp [boolIndicator]

theorem acceptProb_mono
    (D : FiniteProbability Ω) {A B : Ω -> Bool}
    (h : ∀ ω, A ω = true -> B ω = true) :
    D.acceptProb A <= D.acceptProb B := by
  unfold FiniteProbability.acceptProb
  exact Finset.sum_le_sum (by
    intro ω _hω
    exact mul_le_mul_of_nonneg_left
      (boolIndicator_mono (h ω)) (D.nonnegative ω))

theorem acceptProb_le_and_not_add
    (D : FiniteProbability Ω) (A B : Ω -> Bool) :
    D.acceptProb A <=
      D.acceptProb (fun ω => A ω && !B ω) + D.acceptProb B := by
  unfold FiniteProbability.acceptProb
  calc
    (Finset.univ.sum fun ω => D.mass ω * boolIndicator (A ω)) <=
        Finset.univ.sum fun ω =>
          D.mass ω *
            (boolIndicator (A ω && !B ω) + boolIndicator (B ω)) := by
          exact Finset.sum_le_sum (by
            intro ω _hω
            exact mul_le_mul_of_nonneg_left
              (boolIndicator_le_and_not_add (A ω) (B ω))
              (D.nonnegative ω))
    _ =
        (Finset.univ.sum fun ω =>
          D.mass ω * boolIndicator (A ω && !B ω)) +
        (Finset.univ.sum fun ω =>
          D.mass ω * boolIndicator (B ω)) := by
          simp [mul_add, Finset.sum_add_distrib]

theorem sub_acceptProb_le_and_not
    (D : FiniteProbability Ω) (A B : Ω -> Bool) :
    D.acceptProb A - D.acceptProb B <=
      D.acceptProb (fun ω => A ω && !B ω) := by
  have h := acceptProb_le_and_not_add D A B
  linarith

theorem acceptProb_or_le_add
    (D : FiniteProbability Ω) (A B : Ω -> Bool) :
    D.acceptProb (fun ω => A ω || B ω) <=
      D.acceptProb A + D.acceptProb B := by
  unfold FiniteProbability.acceptProb
  calc
    (Finset.univ.sum fun ω =>
        D.mass ω * boolIndicator (A ω || B ω)) <=
        Finset.univ.sum fun ω =>
          D.mass ω * (boolIndicator (A ω) + boolIndicator (B ω)) := by
          exact Finset.sum_le_sum (by
            intro ω _hω
            exact mul_le_mul_of_nonneg_left
              (boolIndicator_or_le_add (A ω) (B ω))
              (D.nonnegative ω))
    _ =
        (Finset.univ.sum fun ω => D.mass ω * boolIndicator (A ω)) +
        (Finset.univ.sum fun ω => D.mass ω * boolIndicator (B ω)) := by
          simp [mul_add, Finset.sum_add_distrib]

theorem boolIndicator_le_sum_of_exists
    {I : Type*} [Fintype I] [DecidableEq I]
    (A : Bool) (E : I -> Bool)
    (h : A = true -> ∃ i, E i = true) :
    boolIndicator A <= Finset.univ.sum fun i => boolIndicator (E i) := by
  cases hA : A
  · simp [boolIndicator]
  · rcases h hA with ⟨i, hi⟩
    have hsingle :
        boolIndicator (E i) <=
          Finset.univ.sum fun j => boolIndicator (E j) :=
      Finset.single_le_sum
        (by intro j _hj; exact boolIndicator_nonneg (E j))
        (Finset.mem_univ i)
    have honeE : boolIndicator (E i) = 1 := by
      simp [hi]
    have hsingle_one :
        (1 : ℝ) <= Finset.univ.sum fun j => boolIndicator (E j) := by
      simpa [honeE] using hsingle
    simpa [boolIndicator] using hsingle_one

theorem acceptProb_le_sum_of_exists
    {I : Type*} [Fintype I] [DecidableEq I]
    (D : FiniteProbability Ω) (A : Ω -> Bool) (E : I -> Ω -> Bool)
    (h : ∀ ω, A ω = true -> ∃ i, E i ω = true) :
    D.acceptProb A <=
      Finset.univ.sum fun i => D.acceptProb (E i) := by
  unfold FiniteProbability.acceptProb
  calc
    (Finset.univ.sum fun ω => D.mass ω * boolIndicator (A ω)) <=
        Finset.univ.sum fun ω =>
          D.mass ω * (Finset.univ.sum fun i => boolIndicator (E i ω)) := by
          exact Finset.sum_le_sum (by
            intro ω _hω
            exact mul_le_mul_of_nonneg_left
              (boolIndicator_le_sum_of_exists (A ω)
                (fun i => E i ω) (h ω))
              (D.nonnegative ω))
    _ =
        Finset.univ.sum fun ω =>
          Finset.univ.sum fun i =>
            D.mass ω * boolIndicator (E i ω) := by
          congr 1
          ext ω
          simpa using
            (Finset.mul_sum
              (s := Finset.univ)
              (a := D.mass ω)
              (f := fun i => boolIndicator (E i ω)))
    _ =
        Finset.univ.sum fun i =>
          Finset.univ.sum fun ω => D.mass ω * boolIndicator (E i ω) := by
          exact Finset.sum_comm

/--
Finite Fiat-Shamir multi-fork game boundary.

`acceptingForks` is the event that the forking procedure obtained the requested
accepting transcripts bound to the same commitments.  `coverageFailure` is the
separate compact fork-delta failure event measured by the sampler analysis.
When accepting forks occur and coverage does not fail, the game must supply
compact opposite-sign coverage for every block.
-/
structure MultiForkFiatShamirGame
    {q : Nat} [NeZero q]
    (Trace Round Draw Block LocalFree : Type*)
    [Fintype Trace]
    (secret : Block × LocalFree -> ZMod q) where
  traceDist : FiniteProbability Trace
  invTwo : ZMod q
  hinvTwo : invTwo * (2 : ZMod q) = 1
  round : Trace -> Round -> MultiForkLinearRound Draw Block LocalFree secret
  acceptingForks : Trace -> Bool
  coverageFailure : Trace -> Bool
  compactCoverage :
    ∀ τ,
      acceptingForks τ = true ->
      coverageFailure τ = false ->
      CoversBlocks fun r b => ((round τ r).CompactOppositeActive b)

namespace MultiForkFiatShamirGame

variable {q : Nat} [NeZero q]
variable {Trace Round Draw Block LocalFree : Type*}
variable [Fintype Trace]
variable {secret : Block × LocalFree -> ZMod q}

def extractableEvent
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret)
    (τ : Trace) : Bool :=
  G.acceptingForks τ && !G.coverageFailure τ

def forkSuccessProb
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret) :
    ℝ :=
  G.traceDist.acceptProb G.acceptingForks

def coverageFailureProb
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret) :
    ℝ :=
  G.traceDist.acceptProb G.coverageFailure

def extractableProb
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret) :
    ℝ :=
  G.traceDist.acceptProb G.extractableEvent

theorem accepting_of_extractable
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret)
    {τ : Trace}
    (h : G.extractableEvent τ = true) :
    G.acceptingForks τ = true := by
  cases hacc : G.acceptingForks τ <;>
    cases hcov : G.coverageFailure τ <;>
    simp [extractableEvent, hacc, hcov] at h ⊢

theorem no_coverageFailure_of_extractable
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret)
    {τ : Trace}
    (h : G.extractableEvent τ = true) :
    G.coverageFailure τ = false := by
  cases hacc : G.acceptingForks τ <;>
    cases hcov : G.coverageFailure τ <;>
    simp [extractableEvent, hacc, hcov] at h ⊢

theorem compactCoverage_of_extractable
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret)
    {τ : Trace}
    (h : G.extractableEvent τ = true) :
    CoversBlocks fun r b => ((G.round τ r).CompactOppositeActive b) :=
  G.compactCoverage τ
    (G.accepting_of_extractable h)
    (G.no_coverageFailure_of_extractable h)

noncomputable def extractedSecret
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret)
    (τ : Trace)
    (h : G.extractableEvent τ = true) :
    Block × LocalFree -> ZMod q :=
  MultiForkLinearRound.extractedSecret (G.round τ)
    (MultiForkLinearRound.coversMultiDelta_of_coversCompactOpposite
      (G.round τ) G.hinvTwo (G.compactCoverage_of_extractable h))

theorem extractedSecret_eq
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret)
    (τ : Trace)
    (h : G.extractableEvent τ = true) :
    G.extractedSecret τ h = secret := by
  unfold extractedSecret
  exact MultiForkLinearRound.extractedSecret_eq_of_coversCompactOpposite
    (G.round τ) G.hinvTwo (G.compactCoverage_of_extractable h)

/--
Core probabilistic loss accounting.  The event that is both accepting and not a
coverage failure has probability at least accepting-fork probability minus the
coverage-failure probability.
-/
theorem extractableProb_lower_bound
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret) :
    G.forkSuccessProb - G.coverageFailureProb <= G.extractableProb := by
  simpa [forkSuccessProb, coverageFailureProb, extractableProb,
    extractableEvent] using
    sub_acceptProb_le_and_not G.traceDist G.acceptingForks G.coverageFailure

/--
If an external Fiat-Shamir forking lemma lower-bounds accepting same-commitment
forks by `p`, and the sampler/coverage analysis upper-bounds compact
fork-delta failure by `δ`, then the extraction event has probability at least
`p - δ`.
-/
theorem extractableProb_ge_of_forking_bounds
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret)
    {p δ : ℝ}
    (hfork : p <= G.forkSuccessProb)
    (hcoverage : G.coverageFailureProb <= δ) :
    p - δ <= G.extractableProb := by
  have hbase := G.extractableProb_lower_bound
  linarith

/--
Quantitative certificate supplied by the future Fiat-Shamir forking analysis.

`forkingLoss` is the loss paid to turn a forger into accepting
same-commitment multi-forks.  `coverageFailureBound` is the compact
fork-delta miss probability from the sampler analysis.  The theorem below is
the end-to-end probability accounting around the deterministic extractor.
-/
structure MultiForkForkingCertificate
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret) where
  forgerSuccess : ℝ
  forkingLoss : ℝ
  coverageFailureBound : ℝ
  forkSuccess_lower :
    forgerSuccess - forkingLoss <= G.forkSuccessProb
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace MultiForkForkingCertificate

variable {G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret}

def extractionLowerBound (C : MultiForkForkingCertificate G) : ℝ :=
  C.forgerSuccess - C.forkingLoss - C.coverageFailureBound

theorem extractableProb_ge
    (C : MultiForkForkingCertificate G) :
    C.extractionLowerBound <= G.extractableProb := by
  unfold extractionLowerBound
  exact G.extractableProb_ge_of_forking_bounds
    C.forkSuccess_lower C.coverageFailure_upper

end MultiForkForkingCertificate

end MultiForkFiatShamirGame

/--
Random-oracle replay layer for the multi-fork game.

The event `forgery` is the original successful forgery event.  The event
`queryMiss` covers successful forgeries where the reduction did not obtain the
challenge random-oracle point it has to replay.  The event `replayFailure`
covers all failures after that point is available: bad replay schedule,
insufficient accepting replays, or transcript-binding failure.  The final field
is the standard random-oracle replay invariant: a forgery with neither loss
event gives accepting same-commitment forks.
-/
structure RandomOracleMultiForkReplay
    {q : Nat} [NeZero q]
    {Trace Round Draw Block LocalFree : Type*}
    [Fintype Trace]
    {secret : Block × LocalFree -> ZMod q}
    (G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret)
    where
  forgery : Trace -> Bool
  queryMiss : Trace -> Bool
  replayFailure : Trace -> Bool
  acceptingForks_of_good :
    ∀ τ,
      forgery τ = true ->
      queryMiss τ = false ->
      replayFailure τ = false ->
      G.acceptingForks τ = true

namespace RandomOracleMultiForkReplay

variable {q : Nat} [NeZero q]
variable {Trace Round Draw Block LocalFree : Type*}
variable [Fintype Trace]
variable {secret : Block × LocalFree -> ZMod q}
variable {G : MultiForkFiatShamirGame Trace Round Draw Block LocalFree secret}

def forkingFailure (R : RandomOracleMultiForkReplay G) (τ : Trace) : Bool :=
  R.queryMiss τ || R.replayFailure τ

def goodReplayEvent (R : RandomOracleMultiForkReplay G) (τ : Trace) : Bool :=
  R.forgery τ && !R.forkingFailure τ

def forgerSuccessProb (R : RandomOracleMultiForkReplay G) : ℝ :=
  G.traceDist.acceptProb R.forgery

def queryMissProb (R : RandomOracleMultiForkReplay G) : ℝ :=
  G.traceDist.acceptProb R.queryMiss

def replayFailureProb (R : RandomOracleMultiForkReplay G) : ℝ :=
  G.traceDist.acceptProb R.replayFailure

def forkingFailureProb (R : RandomOracleMultiForkReplay G) : ℝ :=
  G.traceDist.acceptProb R.forkingFailure

def goodReplayProb (R : RandomOracleMultiForkReplay G) : ℝ :=
  G.traceDist.acceptProb R.goodReplayEvent

theorem queryMiss_false_of_not_failure
    (R : RandomOracleMultiForkReplay G) {τ : Trace}
    (h : R.forkingFailure τ = false) :
    R.queryMiss τ = false := by
  cases hq : R.queryMiss τ <;>
    cases hr : R.replayFailure τ <;>
    simp [forkingFailure, hq, hr] at h ⊢

theorem replayFailure_false_of_not_failure
    (R : RandomOracleMultiForkReplay G) {τ : Trace}
    (h : R.forkingFailure τ = false) :
    R.replayFailure τ = false := by
  cases hq : R.queryMiss τ <;>
    cases hr : R.replayFailure τ <;>
    simp [forkingFailure, hq, hr] at h ⊢

theorem acceptingForks_of_goodReplayEvent
    (R : RandomOracleMultiForkReplay G) {τ : Trace}
    (hgood : R.goodReplayEvent τ = true) :
    G.acceptingForks τ = true := by
  have hforge : R.forgery τ = true := by
    cases hf : R.forgery τ <;>
      cases hfail : R.forkingFailure τ <;>
      simp [goodReplayEvent, hf, hfail] at hgood ⊢
  have hnotFailure : R.forkingFailure τ = false := by
    cases hf : R.forgery τ <;>
      cases hfail : R.forkingFailure τ <;>
      simp [goodReplayEvent, hf, hfail] at hgood ⊢
  exact R.acceptingForks_of_good τ hforge
    (R.queryMiss_false_of_not_failure hnotFailure)
    (R.replayFailure_false_of_not_failure hnotFailure)

theorem goodReplayProb_le_forkSuccessProb
    (R : RandomOracleMultiForkReplay G) :
    R.goodReplayProb <= G.forkSuccessProb := by
  unfold goodReplayProb MultiForkFiatShamirGame.forkSuccessProb
  exact acceptProb_mono G.traceDist
    (fun τ h => R.acceptingForks_of_goodReplayEvent h)

theorem forkingFailureProb_le_losses
    (R : RandomOracleMultiForkReplay G) :
    R.forkingFailureProb <= R.queryMissProb + R.replayFailureProb := by
  unfold forkingFailureProb queryMissProb replayFailureProb forkingFailure
  exact acceptProb_or_le_add G.traceDist R.queryMiss R.replayFailure

/--
Concrete random-oracle replay loss.  A successful forgery becomes accepting
same-commitment multi-forks except for the query-miss and replay-failure
events.
-/
theorem forkSuccessProb_lower_bound
    (R : RandomOracleMultiForkReplay G) :
    R.forgerSuccessProb - R.queryMissProb - R.replayFailureProb <=
      G.forkSuccessProb := by
  have hgoodLower :
      R.forgerSuccessProb - R.forkingFailureProb <= R.goodReplayProb := by
    simpa [forgerSuccessProb, forkingFailureProb, goodReplayProb,
      goodReplayEvent] using
      sub_acceptProb_le_and_not G.traceDist R.forgery R.forkingFailure
  have hgoodUpper := R.goodReplayProb_le_forkSuccessProb
  have hfailureUpper := R.forkingFailureProb_le_losses
  linarith

/--
External concrete bounds for the two random-oracle replay losses and the
sampler coverage loss.  For the six-draw candidate, instantiate `Draw` as
`Fin 6`; `queryMissBound` and `replayFailureBound` are the transcript-specific
random-oracle bounds.
-/
structure RandomOracleForkingBounds
    (R : RandomOracleMultiForkReplay G) where
  queryMissBound : ℝ
  replayFailureBound : ℝ
  coverageFailureBound : ℝ
  queryMiss_upper : R.queryMissProb <= queryMissBound
  replayFailure_upper : R.replayFailureProb <= replayFailureBound
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace RandomOracleForkingBounds

variable {R : RandomOracleMultiForkReplay G}

def forkingLoss (B : RandomOracleForkingBounds R) : ℝ :=
  B.queryMissBound + B.replayFailureBound

def toMultiForkForkingCertificate
    (B : RandomOracleForkingBounds R) :
    G.MultiForkForkingCertificate where
  forgerSuccess := R.forgerSuccessProb
  forkingLoss := B.forkingLoss
  coverageFailureBound := B.coverageFailureBound
  forkSuccess_lower := by
    unfold forkingLoss
    have hbase := R.forkSuccessProb_lower_bound
    have hquery := B.queryMiss_upper
    have hreplay := B.replayFailure_upper
    linarith
  coverageFailure_upper := B.coverageFailure_upper

theorem extractableProb_ge
    (B : RandomOracleForkingBounds R) :
    R.forgerSuccessProb - B.queryMissBound - B.replayFailureBound -
        B.coverageFailureBound <= G.extractableProb := by
  have h :=
    B.toMultiForkForkingCertificate.extractableProb_ge
  unfold MultiForkFiatShamirGame.MultiForkForkingCertificate.extractionLowerBound
    toMultiForkForkingCertificate forkingLoss at h
  linarith

end RandomOracleForkingBounds

/--
Concrete six-draw replay bounds.

For the leading compact candidate the replay target is six accepting draws for
one commitment, modeled by `Draw = Fin 6`.  This certificate reduces query miss
to a blind challenge-guess event and replay failure to a union bound over six
per-draw replay failures.
-/
structure SixDrawForkingBounds
    {q : Nat} [NeZero q]
    {Trace Round Block LocalFree : Type*}
    [Fintype Trace]
    {secret : Block × LocalFree -> ZMod q}
    {G : MultiForkFiatShamirGame Trace Round (Fin 6) Block LocalFree secret}
    (R : RandomOracleMultiForkReplay G) where
  queryGuess : Trace -> Bool
  drawFailure : Fin 6 -> Trace -> Bool
  challengeGuessBound : ℝ
  drawFailureBound : Fin 6 -> ℝ
  coverageFailureBound : ℝ
  queryMiss_implies_guess :
    ∀ τ, R.queryMiss τ = true -> queryGuess τ = true
  replayFailure_implies_drawFailure :
    ∀ τ, R.replayFailure τ = true -> ∃ d : Fin 6, drawFailure d τ = true
  queryGuess_upper :
    G.traceDist.acceptProb queryGuess <= challengeGuessBound
  drawFailure_upper :
    ∀ d, G.traceDist.acceptProb (drawFailure d) <= drawFailureBound d
  coverageFailure_upper :
    G.coverageFailureProb <= coverageFailureBound

namespace SixDrawForkingBounds

variable {q : Nat} [NeZero q]
variable {Trace Round Block LocalFree : Type*}
variable [Fintype Trace]
variable {secret : Block × LocalFree -> ZMod q}
variable {G : MultiForkFiatShamirGame Trace Round (Fin 6) Block LocalFree secret}
variable {R : RandomOracleMultiForkReplay G}

def replayFailureBound (B : SixDrawForkingBounds R) : ℝ :=
  Finset.univ.sum B.drawFailureBound

theorem queryMiss_upper
    (B : SixDrawForkingBounds R) :
    R.queryMissProb <= B.challengeGuessBound := by
  unfold queryMissProb
  have hmono :
      G.traceDist.acceptProb R.queryMiss <=
        G.traceDist.acceptProb B.queryGuess :=
    acceptProb_mono G.traceDist B.queryMiss_implies_guess
  have hguess := B.queryGuess_upper
  linarith

theorem replayFailure_upper
    (B : SixDrawForkingBounds R) :
    R.replayFailureProb <= B.replayFailureBound := by
  unfold replayFailureProb replayFailureBound
  have hunion :
      G.traceDist.acceptProb R.replayFailure <=
        Finset.univ.sum fun d =>
          G.traceDist.acceptProb (B.drawFailure d) :=
    acceptProb_le_sum_of_exists G.traceDist
      R.replayFailure B.drawFailure
      B.replayFailure_implies_drawFailure
  have hbounds :
      (Finset.univ.sum fun d =>
          G.traceDist.acceptProb (B.drawFailure d)) <=
        Finset.univ.sum B.drawFailureBound :=
    Finset.sum_le_sum (by
      intro d _hd
      exact B.drawFailure_upper d)
  exact le_trans hunion hbounds

def toRandomOracleForkingBounds
    (B : SixDrawForkingBounds R) :
    R.RandomOracleForkingBounds where
  queryMissBound := B.challengeGuessBound
  replayFailureBound := B.replayFailureBound
  coverageFailureBound := B.coverageFailureBound
  queryMiss_upper := B.queryMiss_upper
  replayFailure_upper := B.replayFailure_upper
  coverageFailure_upper := B.coverageFailure_upper

/--
Six-draw concrete extraction bound.  The only remaining transcript-specific
inputs are the blind challenge-guess bound, the six per-draw replay-failure
bounds, and the compact coverage-failure bound.
-/
theorem extractableProb_ge
    (B : SixDrawForkingBounds R) :
    R.forgerSuccessProb - B.challengeGuessBound -
        B.replayFailureBound - B.coverageFailureBound <=
      G.extractableProb := by
  exact B.toRandomOracleForkingBounds.extractableProb_ge

end SixDrawForkingBounds

end RandomOracleMultiForkReplay

end

end SignatureForking

end Descender
