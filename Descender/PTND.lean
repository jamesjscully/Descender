import Mathlib.Tactic
import Descender.Residue

namespace Descender

/-!
Definition surface for projected target nearest-distance (P-TND).

The first PTND layer only records search and decision problem statements using
the shared centered-residue conventions.  It intentionally does not claim
average-case reductions, KEM security, or distributional theorems.
-/

variable {Coord Secret Noise : Type*}

structure P_TND_Search (q : Nat) [NeZero q]
    (Coord Secret : Type*) [Fintype Coord] where
  eval : Secret -> Coord -> ZMod q
  target : Coord -> ZMod q
  eta : Nat

namespace P_TND_Search

variable {q : Nat} [NeZero q]

def Success [Fintype Coord] (P : P_TND_Search q Coord Secret)
    (s : Secret) : Prop :=
  centeredNormLinf q (fun i => P.target i - P.eval s i) <= P.eta

def HasSolution [Fintype Coord] (P : P_TND_Search q Coord Secret) : Prop :=
  ∃ s, P.Success s

def EvalSeparated [Fintype Coord] (P : P_TND_Search q Coord Secret) : Prop :=
  ∀ s t, s ≠ t ->
    2 * P.eta < centeredNormLinf q (fun i => P.eval s i - P.eval t i)

def UniqueSolution [Fintype Coord] (P : P_TND_Search q Coord Secret) : Prop :=
  ∀ s t, P.Success s -> P.Success t -> s = t

/-- Support/Hamming-success variant for sparse-noise decoding. -/
def SupportSuccess [Fintype Coord] [DecidableEq Coord] [DecidableEq (ZMod q)]
    (P : P_TND_Search q Coord Secret) (tau : Nat) (s : Secret) : Prop :=
  weight (fun i => P.target i - P.eval s i) <= tau

/--
Support/Hamming separation variant for sparse-noise decoding.

This is deliberately separate from `EvalSeparated`, which is an amplitude
condition using centered `L∞`.
-/
def SupportEvalSeparated [Fintype Coord] [DecidableEq Coord]
    [DecidableEq (ZMod q)] (P : P_TND_Search q Coord Secret)
    (tau : Nat) : Prop :=
  ∀ s t, s ≠ t ->
    2 * tau < weight (fun i => P.eval s i - P.eval t i)

def SupportUniqueSolution [Fintype Coord] [DecidableEq Coord]
    [DecidableEq (ZMod q)] (P : P_TND_Search q Coord Secret)
    (tau : Nat) : Prop :=
  ∀ s t, P.SupportSuccess tau s -> P.SupportSuccess tau t -> s = t

theorem uniqueSolution_of_evalSeparated [Fintype Coord]
    (P : P_TND_Search q Coord Secret)
    (hSep : P.EvalSeparated) : P.UniqueSolution := by
  intro s t hs ht
  by_contra hne
  have hlarge := hSep s t hne
  have heq : (fun i => P.eval s i - P.eval t i) =
      fun i => -(P.target i - P.eval s i) + (P.target i - P.eval t i) := by
    funext i
    ring
  have hle :
      centeredNormLinf q (fun i => P.eval s i - P.eval t i) <=
        P.eta + P.eta := by
    rw [heq]
    calc
      centeredNormLinf q
          (fun i => -(P.target i - P.eval s i) + (P.target i - P.eval t i))
          <= centeredNormLinf q (fun i => -(P.target i - P.eval s i)) +
              centeredNormLinf q (fun i => P.target i - P.eval t i) := by
            exact centeredNormLinf_add_le q _ _
      _ = centeredNormLinf q (fun i => P.target i - P.eval s i) +
              centeredNormLinf q (fun i => P.target i - P.eval t i) := by
            rw [centeredNormLinf_neg]
      _ <= P.eta + P.eta := Nat.add_le_add hs ht
  have htwice : P.eta + P.eta = 2 * P.eta := by omega
  rw [htwice] at hle
  omega

theorem supportUniqueSolution_of_supportEvalSeparated [Fintype Coord]
    [DecidableEq Coord] [DecidableEq (ZMod q)]
    (P : P_TND_Search q Coord Secret) (tau : Nat)
    (hSep : P.SupportEvalSeparated tau) :
    P.SupportUniqueSolution tau := by
  intro s t hs ht
  by_contra hne
  have hlarge := hSep s t hne
  have heq : (fun i => P.eval s i - P.eval t i) =
      fun i => -(P.target i - P.eval s i) + (P.target i - P.eval t i) := by
    funext i
    ring
  have hle :
      weight (fun i => P.eval s i - P.eval t i) <= tau + tau := by
    rw [heq]
    calc
      weight
          (fun i => -(P.target i - P.eval s i) + (P.target i - P.eval t i))
          <= weight (fun i => -(P.target i - P.eval s i)) +
              weight (fun i => P.target i - P.eval t i) := by
            exact weight_add_le _ _
      _ = weight (fun i => P.target i - P.eval s i) +
              weight (fun i => P.target i - P.eval t i) := by
            rw [weight_neg]
      _ <= tau + tau := Nat.add_le_add hs ht
  have htwice : tau + tau = 2 * tau := by omega
  rw [htwice] at hle
  omega

end P_TND_Search

structure ProjectedCoboundaryFamily (q : Nat) [NeZero q]
    (Coord Secret : Type*) [Fintype Coord] where
  instances : P_TND_Search q Coord Secret -> Prop

namespace ProjectedCoboundaryFamily

variable {q : Nat} [NeZero q]

def Solver [Fintype Coord] (_W : ProjectedCoboundaryFamily q Coord Secret) :
    Type _ :=
  ∀ P : P_TND_Search q Coord Secret, _W.instances P -> Secret

def Solves [Fintype Coord] (W : ProjectedCoboundaryFamily q Coord Secret)
    (S : W.Solver) : Prop :=
  ∀ P hP, P_TND_Search.Success P (S P hP)

def Complete [Fintype Coord] (W : ProjectedCoboundaryFamily q Coord Secret) :
    Prop :=
  ∀ P, W.instances P -> P.HasSolution

theorem solves_instance [Fintype Coord]
    (W : ProjectedCoboundaryFamily q Coord Secret) {S : W.Solver}
    (hS : W.Solves S) {P : P_TND_Search q Coord Secret}
    (hP : W.instances P) :
    P_TND_Search.Success P (S P hP) := by
  unfold Solves at hS
  exact hS P hP

theorem not_solves_of_no_solution [Fintype Coord]
    (W : ProjectedCoboundaryFamily q Coord Secret)
    {P : P_TND_Search q Coord Secret} (hP : W.instances P)
    (hNo : ¬ P.HasSolution) (S : W.Solver) :
    ¬ W.Solves S := by
  intro hS
  unfold Solves at hS
  exact hNo ⟨S P hP, hS P hP⟩

theorem complete_of_solved [Fintype Coord]
    (W : ProjectedCoboundaryFamily q Coord Secret)
    {S : W.Solver} (hS : W.Solves S) :
    W.Complete := by
  intro P hP
  exact ⟨S P hP, W.solves_instance hS hP⟩

end ProjectedCoboundaryFamily

structure P_TND_Decision (q : Nat) [NeZero q]
    (Coord Secret Noise : Type*) [Fintype Coord] where
  eval : Secret -> Coord -> ZMod q
  noise : Noise -> Coord -> ZMod q
  noiseSupport : Noise -> Prop
  nullSample : Coord -> ZMod q

namespace P_TND_Decision

variable {q : Nat} [NeZero q]

/--
Add deterministic serialization/rounding error to a P-TND decision instance.

This is the proof-facing model for bit-packed public samples: decoding the
serialized sample back into `ZMod q` is treated as the original real sample plus
an extra bounded rounding vector.  Security of the serialized sample is handled
separately by deterministic post-processing/data processing; correctness pays
for this extra term through a larger effective noise radius.
-/
def withRoundingNoise [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    {Rounding : Type*}
    (rounding : Rounding -> Coord -> ZMod q)
    (roundingSupport : Rounding -> Prop) :
    P_TND_Decision q Coord Secret (Noise × Rounding) where
  eval := P.eval
  noise := fun er i => P.noise er.1 i + rounding er.2 i
  noiseSupport := fun er => P.noiseSupport er.1 ∧ roundingSupport er.2
  nullSample := P.nullSample

theorem withRoundingNoise_noiseSupport_of_bounds [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    {Rounding : Type*}
    (rounding : Rounding -> Coord -> ZMod q)
    (roundingSupport : Rounding -> Prop)
    {eta roundingEta : Nat}
    (hNoise :
      ∀ e, P.noiseSupport e ->
        centeredResidueNoiseSupport q eta (P.noise e))
    (hRounding :
      ∀ r, roundingSupport r ->
        centeredResidueNoiseSupport q roundingEta (rounding r))
    (er : Noise × Rounding)
    (her : (P.withRoundingNoise rounding roundingSupport).noiseSupport er) :
    centeredResidueNoiseSupport q (eta + roundingEta)
      ((P.withRoundingNoise rounding roundingSupport).noise er) := by
  exact centeredResidueNoiseSupport_add
    (q := q)
    (hNoise er.1 her.1)
    (hRounding er.2 her.2)

def realSample [Fintype Coord] (P : P_TND_Decision q Coord Secret Noise)
    (s : Secret) (e : Noise) : Coord -> ZMod q :=
  fun i => P.eval s i + P.noise e i

def RealNoiseSmall [Fintype Coord] (P : P_TND_Decision q Coord Secret Noise)
    (e : Noise) : Prop :=
  P.noiseSupport e

def RealNoiseSupportSmall [Fintype Coord] [DecidableEq Coord]
    [DecidableEq (ZMod q)] (P : P_TND_Decision q Coord Secret Noise)
    (tau : Nat) (e : Noise) : Prop :=
  weight (P.noise e) <= tau

def RealSampleSet [Fintype Coord] (P : P_TND_Decision q Coord Secret Noise)
    (t : Coord -> ZMod q) : Prop :=
  ∃ s e, P.noiseSupport e ∧ t = P.realSample s e

def NullSampleSet [Fintype Coord] (P : P_TND_Decision q Coord Secret Noise)
    (t : Coord -> ZMod q) : Prop :=
  t = P.nullSample

def Distinguishes [Fintype Coord] (P : P_TND_Decision q Coord Secret Noise)
    (A : (Coord -> ZMod q) -> Bool) : Prop :=
  (∀ t, P.RealSampleSet t -> A t = true) ∧
    ∀ t, P.NullSampleSet t -> A t = false

theorem distinguishes_real [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    {A : (Coord -> ZMod q) -> Bool}
    (hA : P.Distinguishes A) {s : Secret} {e : Noise}
    (he : P.noiseSupport e) :
    A (P.realSample s e) = true :=
  hA.1 (P.realSample s e) ⟨s, e, he, rfl⟩

theorem distinguishes_null [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    {A : (Coord -> ZMod q) -> Bool}
    (hA : P.Distinguishes A) :
    A P.nullSample = false :=
  hA.2 P.nullSample rfl

def realSearch [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise) (eta : Nat)
    (s : Secret) (e : Noise) : P_TND_Search q Coord Secret where
  eval := P.eval
  target := P.realSample s e
  eta := eta

theorem realSearch_success_of_centered_noise [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    (eta : Nat) (s : Secret) (e : Noise)
    (hnoise : centeredResidueNoiseSupport q eta (P.noise e)) :
    (P.realSearch eta s e).Success s := by
  unfold P_TND_Search.Success realSearch realSample
  simpa [sub_eq_add_neg, add_comm, add_left_comm, add_assoc] using
    (centeredNormLinf_le_of_noiseSupport (q := q) hnoise)

theorem realSearch_supportSuccess_of_support_noise [Fintype Coord]
    [DecidableEq Coord] [DecidableEq (ZMod q)]
    (P : P_TND_Decision q Coord Secret Noise)
    (tau : Nat) (s : Secret) (e : Noise)
    (hnoise : P.RealNoiseSupportSmall tau e) :
    (P.realSearch tau s e).SupportSuccess tau s := by
  unfold P_TND_Search.SupportSuccess RealNoiseSupportSmall realSearch realSample at *
  convert hnoise using 2
  funext i
  ring

theorem realSearch_hasSolution_of_centered_noise [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    (eta : Nat) (s : Secret) (e : Noise)
    (hnoise : centeredResidueNoiseSupport q eta (P.noise e)) :
    (P.realSearch eta s e).HasSolution :=
  ⟨s, P.realSearch_success_of_centered_noise eta s e hnoise⟩

theorem realSearch_planted_unique_of_centered_noise_and_separated [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    (eta : Nat) (s : Secret) (e : Noise)
    (hnoise : centeredResidueNoiseSupport q eta (P.noise e))
    (hSep : (P.realSearch eta s e).EvalSeparated) :
    ∀ t, (P.realSearch eta s e).Success t -> t = s := by
  intro t ht
  exact (P_TND_Search.uniqueSolution_of_evalSeparated
    (P.realSearch eta s e) hSep) t s ht
      (P.realSearch_success_of_centered_noise eta s e hnoise)

theorem withRoundingNoise_realSample [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    {Rounding : Type*}
    (rounding : Rounding -> Coord -> ZMod q)
    (roundingSupport : Rounding -> Prop)
    (s : Secret) (e : Noise) (r : Rounding) :
    (P.withRoundingNoise rounding roundingSupport).realSample s (e, r) =
      fun i => P.realSample s e i + rounding r i := by
  funext i
  simp [withRoundingNoise, realSample]
  ring

theorem withRoundingNoise_realSearch_success_of_bounds [Fintype Coord]
    (P : P_TND_Decision q Coord Secret Noise)
    {Rounding : Type*}
    (rounding : Rounding -> Coord -> ZMod q)
    (roundingSupport : Rounding -> Prop)
    {eta roundingEta : Nat}
    (hNoise :
      ∀ e, P.noiseSupport e ->
        centeredResidueNoiseSupport q eta (P.noise e))
    (hRounding :
      ∀ r, roundingSupport r ->
        centeredResidueNoiseSupport q roundingEta (rounding r))
    (s : Secret) (er : Noise × Rounding)
    (her : (P.withRoundingNoise rounding roundingSupport).noiseSupport er) :
    ((P.withRoundingNoise rounding roundingSupport).realSearch
      (eta + roundingEta) s er).Success s :=
  (P.withRoundingNoise rounding roundingSupport).realSearch_success_of_centered_noise
    (eta + roundingEta) s er
    (P.withRoundingNoise_noiseSupport_of_bounds rounding roundingSupport
      hNoise hRounding er her)

end P_TND_Decision

end Descender
