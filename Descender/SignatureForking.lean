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

end

end SignatureForking

end Descender
