import Descender.PTND

namespace Descender

/-!
Generic reduction calculus for the cryptographic security layer.

This file does not assert any cryptographic hardness.  Instead it records the
interfaces that must be filled by later reductions and proves the formal
composition theorem: a CCA break is impossible if it reduces to CPA, CPA reduces
to average P-TND, average P-TND reduces to worst-case projected coboundary, and
the worst-case problem is hard.
-/

structure Reduction {A B : Type*} (PA : A -> Prop) (PB : B -> Prop) where
  map : A -> B
  sound : ∀ a, PA a -> PB (map a)

def Hard {Solver : Type*} (P : Solver -> Prop) : Prop :=
  ∀ solver, ¬ P solver

def NoBreak {Adversary : Type*} (Breaks : Adversary -> Prop) : Prop :=
  ∀ adversary, ¬ Breaks adversary

theorem hard_of_reduction {A B : Type*} {PA : A -> Prop} {PB : B -> Prop}
    (R : Reduction PA PB) (hHard : Hard PB) :
    Hard PA := by
  intro a ha
  exact hHard (R.map a) (R.sound a ha)

def composeReduction {A B C : Type*}
    {PA : A -> Prop} {PB : B -> Prop} {PC : C -> Prop}
    (R₁ : Reduction PA PB) (R₂ : Reduction PB PC) :
    Reduction PA PC where
  map := fun a => R₂.map (R₁.map a)
  sound := by
    intro a ha
    exact R₂.sound (R₁.map a) (R₁.sound a ha)

structure DescenderSecurityProblems where
  CCAAdversary : Type
  CPAAdversary : Type
  AveragePTNDSolver : Type
  WorstCaseProjectedCoboundarySolver : Type
  BreaksCCA : CCAAdversary -> Prop
  BreaksCPA : CPAAdversary -> Prop
  SolvesAveragePTND : AveragePTNDSolver -> Prop
  SolvesWorstCaseProjectedCoboundary : WorstCaseProjectedCoboundarySolver -> Prop

namespace DescenderSecurityProblems

structure ReductionPipeline (P : DescenderSecurityProblems) where
  ccaToCpa : Reduction P.BreaksCCA P.BreaksCPA
  cpaToAveragePTND : Reduction P.BreaksCPA P.SolvesAveragePTND
  averageToWorstCase :
    Reduction P.SolvesAveragePTND P.SolvesWorstCaseProjectedCoboundary

def ccaToWorstCase {P : DescenderSecurityProblems} (R : P.ReductionPipeline) :
    Reduction P.BreaksCCA P.SolvesWorstCaseProjectedCoboundary :=
  composeReduction (composeReduction R.ccaToCpa R.cpaToAveragePTND)
    R.averageToWorstCase

theorem no_cca_break_of_worst_case_hard
    {P : DescenderSecurityProblems} (R : P.ReductionPipeline)
    (hWorst : Hard P.SolvesWorstCaseProjectedCoboundary) :
    NoBreak P.BreaksCCA := by
  exact hard_of_reduction (ccaToWorstCase R) hWorst

theorem no_cpa_break_of_worst_case_hard
    {P : DescenderSecurityProblems} (R : P.ReductionPipeline)
    (hWorst : Hard P.SolvesWorstCaseProjectedCoboundary) :
    NoBreak P.BreaksCPA := by
  exact hard_of_reduction
    (composeReduction R.cpaToAveragePTND R.averageToWorstCase) hWorst

theorem no_average_ptnd_solver_of_worst_case_hard
    {P : DescenderSecurityProblems} (R : P.ReductionPipeline)
    (hWorst : Hard P.SolvesWorstCaseProjectedCoboundary) :
    Hard P.SolvesAveragePTND := by
  exact hard_of_reduction R.averageToWorstCase hWorst

end DescenderSecurityProblems

structure FOTransformProof (P : DescenderSecurityProblems) where
  reduction : Reduction P.BreaksCCA P.BreaksCPA

structure CPAToAveragePTNDProof (P : DescenderSecurityProblems) where
  reduction : Reduction P.BreaksCPA P.SolvesAveragePTND

structure AveragePTNDToWorstCaseProof (P : DescenderSecurityProblems) where
  reduction :
    Reduction P.SolvesAveragePTND P.SolvesWorstCaseProjectedCoboundary

structure DescenderKEMSecurityCertificate where
  problems : DescenderSecurityProblems
  fo : FOTransformProof problems
  cpaToPTND : CPAToAveragePTNDProof problems
  averageToWorst : AveragePTNDToWorstCaseProof problems
  worstCaseHard : Hard problems.SolvesWorstCaseProjectedCoboundary

namespace DescenderKEMSecurityCertificate

def pipeline (C : DescenderKEMSecurityCertificate) :
    C.problems.ReductionPipeline where
  ccaToCpa := C.fo.reduction
  cpaToAveragePTND := C.cpaToPTND.reduction
  averageToWorstCase := C.averageToWorst.reduction

theorem no_cca_break (C : DescenderKEMSecurityCertificate) :
    NoBreak C.problems.BreaksCCA :=
  DescenderSecurityProblems.no_cca_break_of_worst_case_hard
    C.pipeline C.worstCaseHard

theorem no_cpa_break (C : DescenderKEMSecurityCertificate) :
    NoBreak C.problems.BreaksCPA :=
  DescenderSecurityProblems.no_cpa_break_of_worst_case_hard
    C.pipeline C.worstCaseHard

theorem no_average_ptnd_solver (C : DescenderKEMSecurityCertificate) :
    Hard C.problems.SolvesAveragePTND :=
  DescenderSecurityProblems.no_average_ptnd_solver_of_worst_case_hard
    C.pipeline C.worstCaseHard

end DescenderKEMSecurityCertificate

/--
The public KEM-security boundary: a completed security certificate proves that
no CCA adversary breaks the instantiated Descender problem family.

This theorem is deliberately conditional.  The certificate must provide the
FO/CCA-to-CPA reduction, the CPA-to-average-P-TND reduction, the
average-to-worst-case reduction, and the worst-case hardness assumption.
-/
theorem descender_kem_ind_cca_secure (C : DescenderKEMSecurityCertificate) :
    NoBreak C.problems.BreaksCCA :=
  C.no_cca_break

structure PTNDCorrectnessCertificate
    (q : Nat) [NeZero q] (Coord Secret Noise : Type*) [Fintype Coord] where
  problem : P_TND_Decision q Coord Secret Noise
  eta : Nat
  noiseBound :
    ∀ e, problem.noiseSupport e ->
      centeredResidueNoiseSupport q eta (problem.noise e)
  separated :
    ∀ s e, problem.noiseSupport e ->
      (problem.realSearch eta s e).EvalSeparated

namespace PTNDCorrectnessCertificate

theorem planted_unique
    {q : Nat} [NeZero q] {Coord Secret Noise : Type*} [Fintype Coord]
    (C : PTNDCorrectnessCertificate q Coord Secret Noise)
    (s : Secret) (e : Noise) (he : C.problem.noiseSupport e) :
    ∀ t, (C.problem.realSearch C.eta s e).Success t -> t = s := by
  exact C.problem.realSearch_planted_unique_of_centered_noise_and_separated
    C.eta s e (C.noiseBound e he) (C.separated s e he)

theorem planted_has_solution
    {q : Nat} [NeZero q] {Coord Secret Noise : Type*} [Fintype Coord]
    (C : PTNDCorrectnessCertificate q Coord Secret Noise)
    (s : Secret) (e : Noise) (he : C.problem.noiseSupport e) :
    (C.problem.realSearch C.eta s e).HasSolution :=
  C.problem.realSearch_hasSolution_of_centered_noise C.eta s e
    (C.noiseBound e he)

theorem planted_success
    {q : Nat} [NeZero q] {Coord Secret Noise : Type*} [Fintype Coord]
    (C : PTNDCorrectnessCertificate q Coord Secret Noise)
    (s : Secret) (e : Noise) (he : C.problem.noiseSupport e) :
    (C.problem.realSearch C.eta s e).Success s :=
  C.problem.realSearch_success_of_centered_noise C.eta s e
    (C.noiseBound e he)

/--
Correctness under deterministic public-sample rounding.

If the original decision instance is correct for noise radius `eta`, and the
decoded serialized sample contributes an additional bounded rounding vector of
radius `roundingEta`, then the rounded instance is correct at
`eta + roundingEta`.  The caller still supplies the corresponding separation
fact at the larger effective radius.
-/
def withRoundingNoise
    {q : Nat} [NeZero q] {Coord Secret Noise : Type*} [Fintype Coord]
    (C : PTNDCorrectnessCertificate q Coord Secret Noise)
    {Rounding : Type*}
    (rounding : Rounding -> Coord -> ZMod q)
    (roundingSupport : Rounding -> Prop)
    (roundingEta : Nat)
    (hRounding :
      ∀ r, roundingSupport r ->
        centeredResidueNoiseSupport q roundingEta (rounding r))
    (hSeparated :
      ∀ s er,
        (C.problem.withRoundingNoise rounding roundingSupport).noiseSupport er ->
        ((C.problem.withRoundingNoise rounding roundingSupport).realSearch
          (C.eta + roundingEta) s er).EvalSeparated) :
    PTNDCorrectnessCertificate q Coord Secret (Noise × Rounding) where
  problem := C.problem.withRoundingNoise rounding roundingSupport
  eta := C.eta + roundingEta
  noiseBound := by
    intro er her
    exact C.problem.withRoundingNoise_noiseSupport_of_bounds
      rounding roundingSupport C.noiseBound hRounding er her
  separated := hSeparated

end PTNDCorrectnessCertificate

/--
Sparse/support-noise correctness certificate.

This is the Hamming-distance sibling of `PTNDCorrectnessCertificate`.  It is
intended for protocol variants where the noise promise is sparse support rather
than small centered amplitude.  It uses the Cauchy/MDS support-distance style
of theorem directly.
-/
structure PTNDSupportCorrectnessCertificate
    (q : Nat) [NeZero q] (Coord Secret Noise : Type*) [Fintype Coord]
    [DecidableEq Coord] [DecidableEq (ZMod q)] where
  problem : P_TND_Decision q Coord Secret Noise
  tau : Nat
  noiseBound :
    ∀ e, problem.noiseSupport e ->
      problem.RealNoiseSupportSmall tau e
  separated :
    ∀ s e, problem.noiseSupport e ->
      (problem.realSearch tau s e).SupportEvalSeparated tau

namespace PTNDSupportCorrectnessCertificate

theorem planted_support_success
    {q : Nat} [NeZero q] {Coord Secret Noise : Type*} [Fintype Coord]
    [DecidableEq Coord] [DecidableEq (ZMod q)]
    (C : PTNDSupportCorrectnessCertificate q Coord Secret Noise)
    (s : Secret) (e : Noise) (he : C.problem.noiseSupport e) :
    (C.problem.realSearch C.tau s e).SupportSuccess C.tau s :=
  C.problem.realSearch_supportSuccess_of_support_noise C.tau s e
    (C.noiseBound e he)

theorem planted_support_unique
    {q : Nat} [NeZero q] {Coord Secret Noise : Type*} [Fintype Coord]
    [DecidableEq Coord] [DecidableEq (ZMod q)]
    (C : PTNDSupportCorrectnessCertificate q Coord Secret Noise)
    (s : Secret) (e : Noise) (he : C.problem.noiseSupport e) :
    ∀ t, (C.problem.realSearch C.tau s e).SupportSuccess C.tau t -> t = s := by
  intro t ht
  exact
    (P_TND_Search.supportUniqueSolution_of_supportEvalSeparated
      (C.problem.realSearch C.tau s e) C.tau (C.separated s e he))
      t s ht (C.planted_support_success s e he)

end PTNDSupportCorrectnessCertificate

structure DescenderCPACore
    (q : Nat) [NeZero q] (Coord Secret Noise : Type) [Fintype Coord] where
  problem : P_TND_Decision q Coord Secret Noise

namespace DescenderCPACore

variable {q : Nat} [NeZero q] {Coord Secret Noise : Type} [Fintype Coord]

def Adversary (_K : DescenderCPACore q Coord Secret Noise) : Type _ :=
  (Coord -> ZMod q) -> Bool

def BreaksCPA (K : DescenderCPACore q Coord Secret Noise)
    (A : K.Adversary) : Prop :=
  K.problem.Distinguishes A

def SolvesAveragePTND (K : DescenderCPACore q Coord Secret Noise)
    (A : K.Adversary) : Prop :=
  K.problem.Distinguishes A

def cpaToAveragePTNDReduction
    (K : DescenderCPACore q Coord Secret Noise) :
    Reduction K.BreaksCPA K.SolvesAveragePTND where
  map := id
  sound := by
    intro A hA
    exact hA

def securityProblems (K : DescenderCPACore q Coord Secret Noise)
    (CCAAdversary WorstCaseProjectedCoboundarySolver : Type)
    (BreaksCCA : CCAAdversary -> Prop)
    (SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop) :
    DescenderSecurityProblems where
  CCAAdversary := CCAAdversary
  CPAAdversary := K.Adversary
  AveragePTNDSolver := K.Adversary
  WorstCaseProjectedCoboundarySolver := WorstCaseProjectedCoboundarySolver
  BreaksCCA := BreaksCCA
  BreaksCPA := K.BreaksCPA
  SolvesAveragePTND := K.SolvesAveragePTND
  SolvesWorstCaseProjectedCoboundary := SolvesWorstCaseProjectedCoboundary

def cpaToAveragePTNDProof
    (K : DescenderCPACore q Coord Secret Noise)
    (CCAAdversary WorstCaseProjectedCoboundarySolver : Type)
    (BreaksCCA : CCAAdversary -> Prop)
    (SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop) :
    CPAToAveragePTNDProof
      (K.securityProblems CCAAdversary WorstCaseProjectedCoboundarySolver
        BreaksCCA SolvesWorstCaseProjectedCoboundary) where
  reduction := K.cpaToAveragePTNDReduction

structure FOImplicitRejectionProof
    (K : DescenderCPACore q Coord Secret Noise)
    (CCAAdversary : Type) (BreaksCCA : CCAAdversary -> Prop) where
  simulate : CCAAdversary -> K.Adversary
  sound : ∀ A, BreaksCCA A -> K.BreaksCPA (simulate A)

namespace FOImplicitRejectionProof

def toFOTransformProof
    {K : DescenderCPACore q Coord Secret Noise}
    {CCAAdversary : Type} {BreaksCCA : CCAAdversary -> Prop}
    (FO : FOImplicitRejectionProof K CCAAdversary BreaksCCA)
    (WorstCaseProjectedCoboundarySolver : Type)
    (SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop) :
    FOTransformProof
      (K.securityProblems CCAAdversary WorstCaseProjectedCoboundarySolver
        BreaksCCA SolvesWorstCaseProjectedCoboundary) where
  reduction := {
    map := FO.simulate
    sound := FO.sound
  }

end FOImplicitRejectionProof

structure AverageToWorstCaseReduction
    (K : DescenderCPACore q Coord Secret Noise)
    (WorstCaseProjectedCoboundarySolver : Type)
    (SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop) where
  build : K.Adversary -> WorstCaseProjectedCoboundarySolver
  sound : ∀ A, K.SolvesAveragePTND A ->
    SolvesWorstCaseProjectedCoboundary (build A)

namespace AverageToWorstCaseReduction

def toAveragePTNDToWorstCaseProof
    {K : DescenderCPACore q Coord Secret Noise}
    {WorstCaseProjectedCoboundarySolver : Type}
    {SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop}
    (R : AverageToWorstCaseReduction K
      WorstCaseProjectedCoboundarySolver SolvesWorstCaseProjectedCoboundary)
    (CCAAdversary : Type) (BreaksCCA : CCAAdversary -> Prop) :
    AveragePTNDToWorstCaseProof
      (K.securityProblems CCAAdversary WorstCaseProjectedCoboundarySolver
        BreaksCCA SolvesWorstCaseProjectedCoboundary) where
  reduction := {
    map := R.build
    sound := R.sound
  }

end AverageToWorstCaseReduction

structure WorstCaseProjectedCoboundaryProblem
    (W : ProjectedCoboundaryFamily q Coord Secret) where
  SolverAlg : Type
  run : SolverAlg -> W.Solver

namespace WorstCaseProjectedCoboundaryProblem

def Solves {W : ProjectedCoboundaryFamily q Coord Secret}
    (WC : WorstCaseProjectedCoboundaryProblem W)
    (S : WC.SolverAlg) : Prop :=
  W.Solves (WC.run S)

theorem solves_instance {W : ProjectedCoboundaryFamily q Coord Secret}
    (WC : WorstCaseProjectedCoboundaryProblem W)
    {S : WC.SolverAlg} (hS : WC.Solves S)
    {P : P_TND_Search q Coord Secret} (hP : W.instances P) :
    P_TND_Search.Success P (WC.run S P hP) :=
  W.solves_instance hS hP

end WorstCaseProjectedCoboundaryProblem

structure AverageToWorstCaseSearchReduction
    (K : DescenderCPACore q Coord Secret Noise)
    {W : ProjectedCoboundaryFamily q Coord Secret}
    (WC : WorstCaseProjectedCoboundaryProblem W) where
  complete : W.Complete
  build : K.Adversary -> WC.SolverAlg
  sound : ∀ A, K.SolvesAveragePTND A -> WC.Solves (build A)

namespace AverageToWorstCaseSearchReduction

def toAverageToWorstCaseReduction
    {K : DescenderCPACore q Coord Secret Noise}
    {W : ProjectedCoboundaryFamily q Coord Secret}
    {WC : WorstCaseProjectedCoboundaryProblem W}
    (R : AverageToWorstCaseSearchReduction K WC) :
    AverageToWorstCaseReduction K WC.SolverAlg WC.Solves where
  build := R.build
  sound := R.sound

def toAveragePTNDToWorstCaseProof
    {K : DescenderCPACore q Coord Secret Noise}
    {W : ProjectedCoboundaryFamily q Coord Secret}
    {WC : WorstCaseProjectedCoboundaryProblem W}
    (R : AverageToWorstCaseSearchReduction K WC)
    (CCAAdversary : Type) (BreaksCCA : CCAAdversary -> Prop) :
    AveragePTNDToWorstCaseProof
      (K.securityProblems CCAAdversary WC.SolverAlg BreaksCCA WC.Solves) :=
  R.toAverageToWorstCaseReduction.toAveragePTNDToWorstCaseProof
    CCAAdversary BreaksCCA

theorem builds_worst_case_solver
    {K : DescenderCPACore q Coord Secret Noise}
    {W : ProjectedCoboundaryFamily q Coord Secret}
    {WC : WorstCaseProjectedCoboundaryProblem W}
    (R : AverageToWorstCaseSearchReduction K WC)
    {A : K.Adversary} (hA : K.SolvesAveragePTND A) :
    WC.Solves (R.build A) :=
  R.sound A hA

theorem hard_average_ptnd_of_worst_case_hard
    {K : DescenderCPACore q Coord Secret Noise}
    {W : ProjectedCoboundaryFamily q Coord Secret}
    {WC : WorstCaseProjectedCoboundaryProblem W}
    (R : AverageToWorstCaseSearchReduction K WC)
    (hHard : Hard WC.Solves) :
    Hard K.SolvesAveragePTND := by
  intro A hA
  exact hHard (R.build A) (R.sound A hA)

end AverageToWorstCaseSearchReduction

structure SelfRandomizationReduction
    (K : DescenderCPACore q Coord Secret Noise)
    {W : ProjectedCoboundaryFamily q Coord Secret}
    (WC : WorstCaseProjectedCoboundaryProblem W) where
  complete : W.Complete
  randomize :
    K.Adversary -> (P : P_TND_Search q Coord Secret) ->
      W.instances P -> Coord -> ZMod q
  randomize_real :
    ∀ A P hP, K.problem.RealSampleSet (randomize A P hP)
  recover :
    K.Adversary -> (P : P_TND_Search q Coord Secret) ->
      W.instances P -> Secret
  recover_sound :
    ∀ A, K.SolvesAveragePTND A ->
      ∀ P hP, P_TND_Search.Success P (recover A P hP)
  solverAlg : K.Adversary -> WC.SolverAlg
  run_solverAlg :
    ∀ A P hP, WC.run (solverAlg A) P hP = recover A P hP

namespace SelfRandomizationReduction

def toAverageToWorstCaseSearchReduction
    {K : DescenderCPACore q Coord Secret Noise}
    {W : ProjectedCoboundaryFamily q Coord Secret}
    {WC : WorstCaseProjectedCoboundaryProblem W}
    (R : SelfRandomizationReduction K WC) :
    AverageToWorstCaseSearchReduction K WC where
  complete := R.complete
  build := R.solverAlg
  sound := by
    intro A hA
    unfold WorstCaseProjectedCoboundaryProblem.Solves ProjectedCoboundaryFamily.Solves
    intro P hP
    rw [R.run_solverAlg A P hP]
    exact R.recover_sound A hA P hP

theorem average_query_is_real
    {K : DescenderCPACore q Coord Secret Noise}
    {W : ProjectedCoboundaryFamily q Coord Secret}
    {WC : WorstCaseProjectedCoboundaryProblem W}
    (R : SelfRandomizationReduction K WC)
    {A : K.Adversary} (hA : K.SolvesAveragePTND A)
    (P : P_TND_Search q Coord Secret) (hP : W.instances P) :
    A (R.randomize A P hP) = true :=
  hA.1 (R.randomize A P hP) (R.randomize_real A P hP)

theorem builds_worst_case_solver
    {K : DescenderCPACore q Coord Secret Noise}
    {W : ProjectedCoboundaryFamily q Coord Secret}
    {WC : WorstCaseProjectedCoboundaryProblem W}
    (R : SelfRandomizationReduction K WC)
    {A : K.Adversary} (hA : K.SolvesAveragePTND A) :
    WC.Solves (R.solverAlg A) :=
  R.toAverageToWorstCaseSearchReduction.sound A hA

theorem hard_average_ptnd_of_worst_case_hard
    {K : DescenderCPACore q Coord Secret Noise}
    {W : ProjectedCoboundaryFamily q Coord Secret}
    {WC : WorstCaseProjectedCoboundaryProblem W}
    (R : SelfRandomizationReduction K WC)
    (hHard : Hard WC.Solves) :
    Hard K.SolvesAveragePTND :=
  R.toAverageToWorstCaseSearchReduction.hard_average_ptnd_of_worst_case_hard hHard

end SelfRandomizationReduction

def securityCertificate
    (K : DescenderCPACore q Coord Secret Noise)
    (CCAAdversary WorstCaseProjectedCoboundarySolver : Type)
    (BreaksCCA : CCAAdversary -> Prop)
    (SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop)
    (fo :
      FOTransformProof
        (K.securityProblems CCAAdversary WorstCaseProjectedCoboundarySolver
          BreaksCCA SolvesWorstCaseProjectedCoboundary))
    (averageToWorst :
      AveragePTNDToWorstCaseProof
        (K.securityProblems CCAAdversary WorstCaseProjectedCoboundarySolver
          BreaksCCA SolvesWorstCaseProjectedCoboundary))
    (worstCaseHard : Hard SolvesWorstCaseProjectedCoboundary) :
    DescenderKEMSecurityCertificate where
  problems :=
    K.securityProblems CCAAdversary WorstCaseProjectedCoboundarySolver
      BreaksCCA SolvesWorstCaseProjectedCoboundary
  fo := fo
  cpaToPTND :=
    K.cpaToAveragePTNDProof CCAAdversary WorstCaseProjectedCoboundarySolver
      BreaksCCA SolvesWorstCaseProjectedCoboundary
  averageToWorst := averageToWorst
  worstCaseHard := worstCaseHard

def securityCertificateFromConcreteReductions
    (K : DescenderCPACore q Coord Secret Noise)
    (CCAAdversary WorstCaseProjectedCoboundarySolver : Type)
    (BreaksCCA : CCAAdversary -> Prop)
    (SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop)
    (fo : FOImplicitRejectionProof K CCAAdversary BreaksCCA)
    (averageToWorst :
      AverageToWorstCaseReduction K WorstCaseProjectedCoboundarySolver
        SolvesWorstCaseProjectedCoboundary)
    (worstCaseHard : Hard SolvesWorstCaseProjectedCoboundary) :
    DescenderKEMSecurityCertificate :=
  K.securityCertificate CCAAdversary WorstCaseProjectedCoboundarySolver
    BreaksCCA SolvesWorstCaseProjectedCoboundary
    (fo.toFOTransformProof WorstCaseProjectedCoboundarySolver
      SolvesWorstCaseProjectedCoboundary)
    (averageToWorst.toAveragePTNDToWorstCaseProof CCAAdversary BreaksCCA)
    worstCaseHard

theorem no_cca_break_of_concrete_cpa
    (K : DescenderCPACore q Coord Secret Noise)
    (CCAAdversary WorstCaseProjectedCoboundarySolver : Type)
    (BreaksCCA : CCAAdversary -> Prop)
    (SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop)
    (fo :
      FOTransformProof
        (K.securityProblems CCAAdversary WorstCaseProjectedCoboundarySolver
          BreaksCCA SolvesWorstCaseProjectedCoboundary))
    (averageToWorst :
      AveragePTNDToWorstCaseProof
        (K.securityProblems CCAAdversary WorstCaseProjectedCoboundarySolver
          BreaksCCA SolvesWorstCaseProjectedCoboundary))
    (worstCaseHard : Hard SolvesWorstCaseProjectedCoboundary) :
    NoBreak BreaksCCA :=
  descender_kem_ind_cca_secure
    (K.securityCertificate CCAAdversary WorstCaseProjectedCoboundarySolver
      BreaksCCA SolvesWorstCaseProjectedCoboundary fo averageToWorst
      worstCaseHard)

theorem no_cca_break_of_concrete_reductions
    (K : DescenderCPACore q Coord Secret Noise)
    (CCAAdversary WorstCaseProjectedCoboundarySolver : Type)
    (BreaksCCA : CCAAdversary -> Prop)
    (SolvesWorstCaseProjectedCoboundary :
      WorstCaseProjectedCoboundarySolver -> Prop)
    (fo : FOImplicitRejectionProof K CCAAdversary BreaksCCA)
    (averageToWorst :
      AverageToWorstCaseReduction K WorstCaseProjectedCoboundarySolver
        SolvesWorstCaseProjectedCoboundary)
    (worstCaseHard : Hard SolvesWorstCaseProjectedCoboundary) :
    NoBreak BreaksCCA :=
  descender_kem_ind_cca_secure
    (K.securityCertificateFromConcreteReductions CCAAdversary
      WorstCaseProjectedCoboundarySolver BreaksCCA
      SolvesWorstCaseProjectedCoboundary fo averageToWorst worstCaseHard)

def securityCertificateFromSearchReduction
    (K : DescenderCPACore q Coord Secret Noise)
    (W : ProjectedCoboundaryFamily q Coord Secret)
    (WC : WorstCaseProjectedCoboundaryProblem W)
    (CCAAdversary : Type) (BreaksCCA : CCAAdversary -> Prop)
    (fo : FOImplicitRejectionProof K CCAAdversary BreaksCCA)
    (averageToWorst : AverageToWorstCaseSearchReduction K WC)
    (worstCaseHard : Hard WC.Solves) :
    DescenderKEMSecurityCertificate :=
  K.securityCertificate CCAAdversary WC.SolverAlg BreaksCCA WC.Solves
    (fo.toFOTransformProof WC.SolverAlg WC.Solves)
    (averageToWorst.toAveragePTNDToWorstCaseProof CCAAdversary BreaksCCA)
    worstCaseHard

theorem no_cca_break_of_average_to_worst_search
    (K : DescenderCPACore q Coord Secret Noise)
    (W : ProjectedCoboundaryFamily q Coord Secret)
    (WC : WorstCaseProjectedCoboundaryProblem W)
    (CCAAdversary : Type) (BreaksCCA : CCAAdversary -> Prop)
    (fo : FOImplicitRejectionProof K CCAAdversary BreaksCCA)
    (averageToWorst : AverageToWorstCaseSearchReduction K WC)
    (worstCaseHard : Hard WC.Solves) :
    NoBreak BreaksCCA :=
  descender_kem_ind_cca_secure
    (K.securityCertificateFromSearchReduction W WC CCAAdversary BreaksCCA
      fo averageToWorst worstCaseHard)

def securityCertificateFromSelfRandomization
    (K : DescenderCPACore q Coord Secret Noise)
    (W : ProjectedCoboundaryFamily q Coord Secret)
    (WC : WorstCaseProjectedCoboundaryProblem W)
    (CCAAdversary : Type) (BreaksCCA : CCAAdversary -> Prop)
    (fo : FOImplicitRejectionProof K CCAAdversary BreaksCCA)
    (selfRandomization : SelfRandomizationReduction K WC)
    (worstCaseHard : Hard WC.Solves) :
    DescenderKEMSecurityCertificate :=
  K.securityCertificateFromSearchReduction W WC CCAAdversary BreaksCCA fo
    selfRandomization.toAverageToWorstCaseSearchReduction worstCaseHard

theorem no_cca_break_of_self_randomization
    (K : DescenderCPACore q Coord Secret Noise)
    (W : ProjectedCoboundaryFamily q Coord Secret)
    (WC : WorstCaseProjectedCoboundaryProblem W)
    (CCAAdversary : Type) (BreaksCCA : CCAAdversary -> Prop)
    (fo : FOImplicitRejectionProof K CCAAdversary BreaksCCA)
    (selfRandomization : SelfRandomizationReduction K WC)
    (worstCaseHard : Hard WC.Solves) :
    NoBreak BreaksCCA :=
  descender_kem_ind_cca_secure
    (K.securityCertificateFromSelfRandomization W WC CCAAdversary BreaksCCA
      fo selfRandomization worstCaseHard)

end DescenderCPACore

end Descender
