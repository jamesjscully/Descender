import Descender.QLattice
import Descender.Security

namespace Descender

/-!
Q-ary lattice and noise hardness interfaces.

This module is deliberately assumption-facing.  It names the primal lattice,
dual lattice, smoothing-style promises, and centered-noise conditions that a
future reduction must instantiate.  It does not prove smoothing, average-case
hardness, or quantum security.
-/

variable {Coord Secret Noise PrimalWitness DualTarget : Type}

/-- The residue pairing used to test integer relations against q-ary samples. -/
def residuePairing [Fintype Coord] (q : Nat) [NeZero q]
    (z : Coord -> Int) (x : Coord -> ZMod q) : ZMod q :=
  ∑ i, (z i : ZMod q) * x i

/--
The centered interval statistic used by the direct q-ary dual distinguisher.

For a q-ary dual relation `z`, real samples have statistic controlled by the
noise pairing, while uniform samples are expected to spread modulo `q`.
-/
def DualIntervalAccepts [Fintype Coord] (q : Nat) [NeZero q]
    (z : Coord -> Int) (radius : Nat) (t : Coord -> ZMod q) : Prop :=
  centeredAbs q (residuePairing q z t) <= radius

def CenteredNoisePairingBounded [Fintype Coord] (q eta : Nat) [NeZero q]
    (z : Coord -> Int) (noise : Noise -> Coord -> ZMod q)
    (support : Noise -> Prop) : Prop :=
  ∀ e, support e ->
    centeredAbs q (residuePairing q z (noise e)) <= eta * intL1Norm z

theorem centeredNoisePairingBounded_of_term_bounds [Fintype Coord]
    {q eta : Nat} [NeZero q] (z : Coord -> Int)
    (noise : Noise -> Coord -> ZMod q) (support : Noise -> Prop)
    (hTerm :
      ∀ e, support e ->
        ∀ i,
          centeredAbs q ((z i : ZMod q) * noise e i) <=
            (z i).natAbs * eta) :
    CenteredNoisePairingBounded q eta z noise support := by
  intro e he
  unfold residuePairing
  calc
    centeredAbs q (∑ i, (z i : ZMod q) * noise e i)
        <= ∑ i, centeredAbs q ((z i : ZMod q) * noise e i) :=
          centeredAbs_sum_le q (fun i => (z i : ZMod q) * noise e i)
    _ <= ∑ i, (z i).natAbs * eta := by
          exact Finset.sum_le_sum (by
            intro i _hi
            exact hTerm e he i)
    _ = eta * intL1Norm z := by
          rw [intL1Norm, ← Finset.sum_mul, Nat.mul_comm]

/--
The primal q-ary lattice attached to a public q-ary map.  An integer vector is
in the primal lattice when its residue is in the public image.
-/
def qaryPrimalLattice (q : Nat) [NeZero q]
    (A : PrimalWitness -> Coord -> ZMod q) : Set (Coord -> Int) :=
  {z | ∃ w, residueVec q z = A w}

def ShortQaryPrimalVector [Fintype Coord] (q : Nat) [NeZero q]
    (A : PrimalWitness -> Coord -> ZMod q) (d : Nat)
    (z : Coord -> Int) : Prop :=
  z ∈ qaryPrimalLattice q A ∧ NontrivialResidue q z ∧ intL1Norm z < d

theorem mem_qaryPrimalLattice_iff (q : Nat) [NeZero q]
    {A : PrimalWitness -> Coord -> ZMod q} {z : Coord -> Int} :
    z ∈ qaryPrimalLattice q A ↔ ∃ w, residueVec q z = A w :=
  Iff.rfl

/-- A no-short-vector promise for the primal q-ary lattice. -/
structure QaryPrimalLatticePromise (q : Nat) [NeZero q]
    (Coord PrimalWitness : Type) [Fintype Coord]
    (A : PrimalWitness -> Coord -> ZMod q) where
  vectorBound : Nat
  noShortPrimal :
    ¬ ∃ z : Coord -> Int, ShortQaryPrimalVector q A vectorBound z

/-- A no-short-relation promise for the dual q-ary lattice already named in `QLattice`. -/
structure QaryDualLatticePromise (q : Nat) [NeZero q]
    (Coord DualTarget : Type) [Fintype Coord] [Zero DualTarget]
    (AT : (Coord -> ZMod q) -> DualTarget) where
  relationBound : Nat
  noShortDual :
    ¬ ∃ z : Coord -> Int, ShortQaryDualRelation q AT relationBound z

/--
A q-ary dual relation is amplitude-vulnerable for centered coordinate noise of
radius `eta` when the entire possible real statistic `<z,e>` lies in a proper
centered interval modulo `q`.

If `AT (residueVec q z) = 0`, then real samples `t = A s + e` satisfy
`<z,t> = <z,e>`.  When `2 * eta * ||z||₁ + 1 < q`, the interval test
`centeredAbs(<z,t>) <= eta * ||z||₁` separates real samples from uniform
samples with nontrivial advantage.  This is the q-ary condition that matters
for the literal P-TND decision game; low Hamming weight is only a sufficient
way to find some such relations, not the target security statement.
-/
def AmplitudeVulnerableQaryDualRelation [Fintype Coord]
    (q eta : Nat) [NeZero q] [Zero DualTarget]
    (AT : (Coord -> ZMod q) -> DualTarget) (z : Coord -> Int) : Prop :=
  z ∈ qaryDualLattice q AT ∧
    NontrivialResidue q z ∧
      2 * eta * intL1Norm z + 1 < q

/--
The direct dual-distinguisher exclusion required by centered-noise P-TND:
there is no nontrivial q-ary public dual relation whose centered-noise
inner-product interval is narrow modulo `q`.
-/
def NoNarrowQaryDualRelations [Fintype Coord]
    (q eta : Nat) [NeZero q] [Zero DualTarget]
    (AT : (Coord -> ZMod q) -> DualTarget) : Prop :=
  ¬ ∃ z : Coord -> Int, AmplitudeVulnerableQaryDualRelation q eta AT z

/-- KEM-facing q-ary dual promise: no direct centered-noise dual distinguisher. -/
structure QaryDualAmplitudePromise (q : Nat) [NeZero q]
    (Coord DualTarget : Type) [Fintype Coord] [Zero DualTarget]
    (AT : (Coord -> ZMod q) -> DualTarget) where
  eta : Nat
  noNarrowDual : NoNarrowQaryDualRelations q eta AT

theorem no_narrow_qary_dual_relations_of_l1_lower_bound
    [Fintype Coord] {q eta d : Nat} [NeZero q] [Zero DualTarget]
    {AT : (Coord -> ZMod q) -> DualTarget}
    (hL1 : ∀ z : Coord -> Int,
      z ∈ qaryDualLattice q AT ->
      NontrivialResidue q z ->
      d <= intL1Norm z)
    (hWrap : q <= 2 * eta * d + 1) :
    NoNarrowQaryDualRelations q eta AT := by
  rintro ⟨z, hzDual, hzNontrivial, hzNarrow⟩
  have hdz : d <= intL1Norm z := hL1 z hzDual hzNontrivial
  have hmono : 2 * eta * d + 1 <= 2 * eta * intL1Norm z + 1 := by
    nlinarith [Nat.mul_le_mul_left (2 * eta) hdz]
  have hq : q <= 2 * eta * intL1Norm z + 1 := Nat.le_trans hWrap hmono
  exact Nat.not_lt_of_ge hq hzNarrow

theorem no_narrow_qary_dual_relations_of_lee_l1_lower_bound
    [Fintype Coord] {q eta d : Nat} [NeZero q] [Zero DualTarget]
    {AT : (Coord -> ZMod q) -> DualTarget}
    (hLee : ∀ r : Coord -> ZMod q,
      AT r = 0 ->
      r ≠ 0 ->
      d <= centeredNormL1 q r)
    (hWrap : q <= 2 * eta * d + 1) :
    NoNarrowQaryDualRelations q eta AT := by
  exact no_narrow_qary_dual_relations_of_l1_lower_bound
    (q := q) (eta := eta) (d := d)
    (AT := AT)
    (by
      intro z hzDual hzNontrivial
      exact Nat.le_trans
        (hLee (residueVec q z) hzDual hzNontrivial)
        (centeredNormL1_residueVec_le_intL1Norm q z))
    hWrap

/--
No-narrow dual security is monotone in the noise-amplitude parameter: proving
there are no relations narrow for a smaller interval also rules out relations
narrow for any larger interval.
-/
theorem no_narrow_qary_dual_relations_mono_eta
    [Fintype Coord] {q etaSmall etaBig : Nat} [NeZero q] [Zero DualTarget]
    {AT : (Coord -> ZMod q) -> DualTarget}
    (hEta : etaSmall <= etaBig)
    (hNoNarrow : NoNarrowQaryDualRelations q etaSmall AT) :
    NoNarrowQaryDualRelations q etaBig AT := by
  rintro ⟨z, hzDual, hzNontrivial, hzNarrow⟩
  apply hNoNarrow
  refine ⟨z, hzDual, hzNontrivial, ?_⟩
  have hmono :
      2 * etaSmall * intL1Norm z + 1 <=
        2 * etaBig * intL1Norm z + 1 := by
    nlinarith [Nat.mul_le_mul_right (2 * intL1Norm z) hEta]
  exact Nat.lt_of_le_of_lt hmono hzNarrow

theorem residuePairing_realSample_eq_noise_of_annihilates_eval
    [Fintype Coord] {q : Nat} [NeZero q]
    (P : P_TND_Decision q Coord Secret Noise)
    (z : Coord -> Int)
    (hAnn : ∀ s, residuePairing q z (P.eval s) = 0)
    (s : Secret) (e : Noise) :
    residuePairing q z (P.realSample s e) =
      residuePairing q z (P.noise e) := by
  unfold residuePairing P_TND_Decision.realSample
  have hAnn' := hAnn s
  unfold residuePairing at hAnn'
  calc
    ∑ i, (z i : ZMod q) * (P.eval s i + P.noise e i)
        = ∑ i, ((z i : ZMod q) * P.eval s i +
            (z i : ZMod q) * P.noise e i) := by
          simp [mul_add]
    _ = (∑ i, (z i : ZMod q) * P.eval s i) +
          ∑ i, (z i : ZMod q) * P.noise e i := by
          rw [Finset.sum_add_distrib]
    _ = ∑ i, (z i : ZMod q) * P.noise e i := by
          rw [hAnn', zero_add]

theorem dualIntervalAccepts_realSample_of_pairing_bound
    [Fintype Coord] {q eta : Nat} [NeZero q]
    (P : P_TND_Decision q Coord Secret Noise)
    (z : Coord -> Int)
    (hAnn : ∀ s, residuePairing q z (P.eval s) = 0)
    (hNoise : CenteredNoisePairingBounded q eta z P.noise P.noiseSupport)
    {s : Secret} {e : Noise} (he : P.noiseSupport e) :
    DualIntervalAccepts q z (eta * intL1Norm z) (P.realSample s e) := by
  unfold DualIntervalAccepts
  rw [residuePairing_realSample_eq_noise_of_annihilates_eval P z hAnn s e]
  exact hNoise e he

/--
The combined primal/dual q-ary promise used by later security reductions.

The `primalSmoothing` and `dualSmoothing` fields are intentionally abstract
propositions.  They are placeholders for genuine smoothing or transference
theorems, not proved facts.
-/
structure QaryPrimalDualSmoothingPromise (q : Nat) [NeZero q]
    (Coord PrimalWitness DualTarget : Type) [Fintype Coord]
    [Zero DualTarget] where
  primalMap : PrimalWitness -> Coord -> ZMod q
  dualMap : (Coord -> ZMod q) -> DualTarget
  primalBound : Nat
  dualBound : Nat
  smoothingRadius : Nat
  noiseRadius : Nat
  dualAmplitudeEta : Nat
  noShortPrimal :
    ¬ ∃ z : Coord -> Int,
      ShortQaryPrimalVector q primalMap primalBound z
  noShortDual :
    ¬ ∃ z : Coord -> Int,
      ShortQaryDualRelation q dualMap dualBound z
  noNarrowDual :
    NoNarrowQaryDualRelations q dualAmplitudeEta dualMap
  primalSmoothing : Prop
  dualSmoothing : Prop
  noiseBelowSmoothing : noiseRadius <= smoothingRadius

namespace QaryPrimalDualSmoothingPromise

variable [Fintype Coord] {q : Nat} [NeZero q] [Zero DualTarget]

def primalLattice (P :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget) :
    Set (Coord -> Int) :=
  qaryPrimalLattice q P.primalMap

def dualLattice (P :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget) :
    Set (Coord -> Int) :=
  qaryDualLattice q P.dualMap

def primalPromise (P :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget) :
    QaryPrimalLatticePromise q Coord PrimalWitness P.primalMap where
  vectorBound := P.primalBound
  noShortPrimal := P.noShortPrimal

def dualPromise (P :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget) :
    QaryDualLatticePromise q Coord DualTarget P.dualMap where
  relationBound := P.dualBound
  noShortDual := P.noShortDual

def dualAmplitudePromise (P :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget) :
    QaryDualAmplitudePromise q Coord DualTarget P.dualMap where
  eta := P.dualAmplitudeEta
  noNarrowDual := P.noNarrowDual

theorem no_narrow_dual (P :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget) :
    NoNarrowQaryDualRelations q P.dualAmplitudeEta P.dualMap :=
  P.noNarrowDual

end QaryPrimalDualSmoothingPromise

def CenteredNoiseBounded [Fintype Coord] (q eta : Nat)
    (noise : Noise -> Coord -> ZMod q) (support : Noise -> Prop) : Prop :=
  ∀ e, support e -> centeredResidueNoiseSupport q eta (noise e)

/--
Deterministic support-level anti-concentration against short integer residue
relations.  This is weaker than a probability statement, but it records the
minimum non-degeneracy needed before a probabilistic theorem is available.
-/
def CenteredNoiseAntiConcentrated [Fintype Coord]
    (q : Nat) [NeZero q] (relationBound : Nat)
    (noise : Noise -> Coord -> ZMod q) (support : Noise -> Prop) : Prop :=
  ∀ z : Coord -> Int,
    NontrivialResidue q z ->
    intL1Norm z < relationBound ->
      ∃ e₀ e₁,
        support e₀ ∧ support e₁ ∧
          residuePairing q z (noise e₀) ≠ residuePairing q z (noise e₁)

/-- Centered-noise support, radius, and anti-concentration assumptions. -/
structure CenteredNoiseSupportPromise (q : Nat) [NeZero q]
    (Coord Noise : Type) [Fintype Coord] where
  noise : Noise -> Coord -> ZMod q
  support : Noise -> Prop
  eta : Nat
  relationBound : Nat
  bounded : CenteredNoiseBounded q eta noise support
  antiConcentrated :
    CenteredNoiseAntiConcentrated q relationBound noise support

namespace CenteredNoiseSupportPromise

variable [Fintype Coord] {q : Nat} [NeZero q]

theorem support_centered
    (N : CenteredNoiseSupportPromise q Coord Noise)
    {e : Noise} (he : N.support e) :
    centeredResidueNoiseSupport q N.eta (N.noise e) :=
  N.bounded e he

end CenteredNoiseSupportPromise

/--
A q-ary/noise interface for decision P-TND hardness.

`decisionHard` is relative to the declared algorithm class `AdversaryAlg`.
This avoids claiming hardness against all mathematical functions.
-/
structure QaryPTNDDecisionHardnessInterface (q : Nat) [NeZero q]
    (Coord Secret Noise PrimalWitness DualTarget : Type)
    [Fintype Coord] [Zero DualTarget] where
  problem : P_TND_Decision q Coord Secret Noise
  qaryPromise :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget
  noisePromise : CenteredNoiseSupportPromise q Coord Noise
  dualAmplitudeEta_le_noiseEta :
    qaryPromise.dualAmplitudeEta <= noisePromise.eta
  noise_eq : ∀ e i, problem.noise e i = noisePromise.noise e i
  support_iff : ∀ e, problem.noiseSupport e ↔ noisePromise.support e
  AdversaryAlg : Type
  run : AdversaryAlg -> (Coord -> ZMod q) -> Bool
  decisionHard : ∀ A, ¬ problem.Distinguishes (run A)

namespace QaryPTNDDecisionHardnessInterface

variable [Fintype Coord] {q : Nat} [NeZero q] [Zero DualTarget]

def Breaks (C :
    QaryPTNDDecisionHardnessInterface
      q Coord Secret Noise PrimalWitness DualTarget)
    (A : C.AdversaryAlg) : Prop :=
  C.problem.Distinguishes (C.run A)

theorem hard (C :
    QaryPTNDDecisionHardnessInterface
      q Coord Secret Noise PrimalWitness DualTarget) :
    Hard C.Breaks := by
  intro A
  exact C.decisionHard A

theorem problem_noise_centered (C :
    QaryPTNDDecisionHardnessInterface
      q Coord Secret Noise PrimalWitness DualTarget)
    {e : Noise} (he : C.problem.noiseSupport e) :
    centeredResidueNoiseSupport q C.noisePromise.eta (C.problem.noise e) := by
  intro i
  rw [C.noise_eq e i]
  exact C.noisePromise.bounded e ((C.support_iff e).mp he) i

theorem no_narrow_dual_at_noise (C :
    QaryPTNDDecisionHardnessInterface
      q Coord Secret Noise PrimalWitness DualTarget) :
    NoNarrowQaryDualRelations q C.noisePromise.eta
      C.qaryPromise.dualMap :=
  no_narrow_qary_dual_relations_mono_eta
    C.dualAmplitudeEta_le_noiseEta C.qaryPromise.noNarrowDual

end QaryPTNDDecisionHardnessInterface

/--
A q-ary/noise interface for worst-case projected-coboundary hardness.

The lattice and noise promises are recorded next to the worst-case family, but
the actual hardness claim remains the existing `Hard WC.Solves` predicate.
-/
structure QaryProjectedCoboundaryHardnessInterface (q : Nat) [NeZero q]
    (Coord Secret Noise PrimalWitness DualTarget : Type)
    [Fintype Coord] [Zero DualTarget] where
  family : ProjectedCoboundaryFamily q Coord Secret
  qaryPromise :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget
  noisePromise : CenteredNoiseSupportPromise q Coord Noise
  dualAmplitudeEta_le_noiseEta :
    qaryPromise.dualAmplitudeEta <= noisePromise.eta
  worstCase : DescenderCPACore.WorstCaseProjectedCoboundaryProblem family
  worstCaseHard : Hard worstCase.Solves

namespace QaryProjectedCoboundaryHardnessInterface

variable [Fintype Coord] {q : Nat} [NeZero q] [Zero DualTarget]

theorem hard (C :
    QaryProjectedCoboundaryHardnessInterface
      q Coord Secret Noise PrimalWitness DualTarget) :
    Hard C.worstCase.Solves :=
  C.worstCaseHard

theorem no_narrow_dual_at_noise (C :
    QaryProjectedCoboundaryHardnessInterface
      q Coord Secret Noise PrimalWitness DualTarget) :
    NoNarrowQaryDualRelations q C.noisePromise.eta
      C.qaryPromise.dualMap :=
  no_narrow_qary_dual_relations_mono_eta
    C.dualAmplitudeEta_le_noiseEta C.qaryPromise.noNarrowDual

end QaryProjectedCoboundaryHardnessInterface

/--
Certificate that a q-ary/noise promise, a projected-coboundary family, and an
average-to-worst-case reduction together imply average P-TND hardness for the
declared CPA core.
-/
structure QaryAverageToWorstCaseCertificate (q : Nat) [NeZero q]
    (Coord Secret Noise PrimalWitness DualTarget : Type)
    [Fintype Coord] [Zero DualTarget] where
  core : DescenderCPACore q Coord Secret Noise
  family : ProjectedCoboundaryFamily q Coord Secret
  qaryPromise :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget
  noisePromise : CenteredNoiseSupportPromise q Coord Noise
  dualAmplitudeEta_le_noiseEta :
    qaryPromise.dualAmplitudeEta <= noisePromise.eta
  worstCase : DescenderCPACore.WorstCaseProjectedCoboundaryProblem family
  averageToWorst :
    DescenderCPACore.AverageToWorstCaseSearchReduction core worstCase
  worstCaseHard : Hard worstCase.Solves

namespace QaryAverageToWorstCaseCertificate

variable [Fintype Coord] {q : Nat} [NeZero q] [Zero DualTarget]

theorem projected_family_complete (C :
    QaryAverageToWorstCaseCertificate
      q Coord Secret Noise PrimalWitness DualTarget) :
    C.family.Complete :=
  C.averageToWorst.complete

theorem hard_average_ptnd (C :
    QaryAverageToWorstCaseCertificate
      q Coord Secret Noise PrimalWitness DualTarget) :
    Hard C.core.SolvesAveragePTND :=
  C.averageToWorst.hard_average_ptnd_of_worst_case_hard C.worstCaseHard

theorem no_narrow_dual (C :
    QaryAverageToWorstCaseCertificate
      q Coord Secret Noise PrimalWitness DualTarget) :
    NoNarrowQaryDualRelations q C.qaryPromise.dualAmplitudeEta
      C.qaryPromise.dualMap :=
  C.qaryPromise.noNarrowDual

theorem no_narrow_dual_at_noise (C :
    QaryAverageToWorstCaseCertificate
      q Coord Secret Noise PrimalWitness DualTarget) :
    NoNarrowQaryDualRelations q C.noisePromise.eta
      C.qaryPromise.dualMap :=
  no_narrow_qary_dual_relations_mono_eta
    C.dualAmplitudeEta_le_noiseEta C.qaryPromise.noNarrowDual

end QaryAverageToWorstCaseCertificate

end Descender
