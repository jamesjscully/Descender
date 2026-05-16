import Descender.Security

namespace Descender

/-!
Concrete assumptions for action-based self-randomization.

`Security.lean` exposes `SelfRandomizationReduction` as the point where an
average P-TND distinguisher becomes a worst-case projected-coboundary solver.
This file refines the algebraic obligations behind that interface.  It does
not assert that any particular Descender sampler has these properties; instead
it packages the reusable assumptions needed for an affine/action-based
construction.
-/

variable {q : Nat} [NeZero q]
variable {Coord Secret Noise : Type} [Fintype Coord]

/--
A public affine-style action on secrets and target vectors.

The target action is deliberately represented as an action on complete target
vectors.  This is flexible enough for affine shifts, coordinate permutations,
and other public masks while keeping the P-TND layer independent of a concrete
linear-algebra representation.
-/
structure PublicAffineAction (q : Nat) [NeZero q]
    (Coord Secret : Type) [Fintype Coord] where
  Mask : Type
  actSecret : Mask -> Secret -> Secret
  unmaskSecret : Mask -> Secret -> Secret
  actTarget : Mask -> (Coord -> ZMod q) -> Coord -> ZMod q
  unmaskTarget : Mask -> (Coord -> ZMod q) -> Coord -> ZMod q
  secret_left_inverse :
    ∀ r s, unmaskSecret r (actSecret r s) = s
  secret_right_inverse :
    ∀ r s, actSecret r (unmaskSecret r s) = s
  target_left_inverse :
    ∀ r t, unmaskTarget r (actTarget r t) = t
  target_right_inverse :
    ∀ r t, actTarget r (unmaskTarget r t) = t

namespace PublicAffineAction

variable (G : PublicAffineAction q Coord Secret)

/-- The public target produced by applying a mask to a search instance. -/
def randomizedTarget (r : G.Mask) (P : P_TND_Search q Coord Secret) :
    Coord -> ZMod q :=
  G.actTarget r P.target

/--
The corresponding masked search instance, with evaluation vectors transported
through the same public action and secrets renamed by the inverse action.
-/
def mapSearch (r : G.Mask) (P : P_TND_Search q Coord Secret) :
    P_TND_Search q Coord Secret where
  eval := fun s => G.actTarget r (P.eval (G.unmaskSecret r s))
  target := G.randomizedTarget r P
  eta := P.eta

@[simp] theorem randomizedTarget_unmask
    (r : G.Mask) (t : Coord -> ZMod q) :
    G.unmaskTarget r (G.actTarget r t) = t :=
  G.target_left_inverse r t

@[simp] theorem secret_unmask
    (r : G.Mask) (s : Secret) :
    G.unmaskSecret r (G.actSecret r s) = s :=
  G.secret_left_inverse r s

end PublicAffineAction

namespace DescenderCPACore

variable (K : DescenderCPACore q Coord Secret Noise)
variable {W : ProjectedCoboundaryFamily q Coord Secret}
variable (G : PublicAffineAction q Coord Secret)

/--
Distributional transport from public action masks into the average P-TND real
sample set.

For every worst-case instance and every mask, the acted target must be
representable as a real sample of the average decision problem with supported
noise.
-/
structure ActionTransportToReal where
  realSecret :
    G.Mask -> (P : P_TND_Search q Coord Secret) -> W.instances P -> Secret
  realNoise :
    G.Mask -> (P : P_TND_Search q Coord Secret) -> W.instances P -> Noise
  realNoise_support :
    ∀ r P hP, K.problem.noiseSupport (realNoise r P hP)
  randomized_eq_real :
    ∀ r P hP,
      G.randomizedTarget r P =
        K.problem.realSample (realSecret r P hP) (realNoise r P hP)

namespace ActionTransportToReal

variable {K G}

theorem randomizedTarget_mem_real
    (T : ActionTransportToReal (W := W) K G)
    (r : G.Mask) (P : P_TND_Search q Coord Secret)
    (hP : W.instances P) :
    K.problem.RealSampleSet (G.randomizedTarget r P) :=
  ⟨T.realSecret r P hP, T.realNoise r P hP,
    T.realNoise_support r P hP, T.randomized_eq_real r P hP⟩

end ActionTransportToReal

/--
An inverse/unmasking certificate for the acted query.

`recover` is the worst-case secret returned by the reduction.  Its soundness is
conditioned on the average adversary accepting the real transported query; the
constructor below obtains that acceptance from `K.SolvesAveragePTND`.
-/
structure ActionUnmasking where
  chooseMask :
    K.Adversary -> (P : P_TND_Search q Coord Secret) ->
      W.instances P -> G.Mask
  recover :
    K.Adversary -> (P : P_TND_Search q Coord Secret) ->
      W.instances P -> Secret
  recover_sound_of_accept :
    ∀ A P hP,
      A (G.randomizedTarget (chooseMask A P hP) P) = true ->
        P_TND_Search.Success P (recover A P hP)

/--
Concrete data sufficient to instantiate `SelfRandomizationReduction` from a
public action, a real-sample transport certificate, and an unmasking certificate.

The final two fields connect the extracted search solution to the chosen
worst-case solver representation.
-/
structure ActionSelfRandomizationCertificate
    {W : ProjectedCoboundaryFamily q Coord Secret}
    (WC : WorstCaseProjectedCoboundaryProblem W) where
  complete : W.Complete
  transport : ActionTransportToReal (W := W) K G
  unmasking : ActionUnmasking (W := W) K G
  solverAlg : K.Adversary -> WC.SolverAlg
  run_solverAlg :
    ∀ A P hP,
      WC.run (solverAlg A) P hP =
        unmasking.recover A P hP

namespace ActionSelfRandomizationCertificate

variable {K G}
variable {WC : WorstCaseProjectedCoboundaryProblem W}

/--
Build the generic `SelfRandomizationReduction` once the action, transport, and
unmasking obligations have been discharged.
-/
def toSelfRandomizationReduction
    (C : ActionSelfRandomizationCertificate (K := K) (G := G) WC) :
    SelfRandomizationReduction K WC where
  complete := C.complete
  randomize := fun A P hP =>
    G.randomizedTarget (C.unmasking.chooseMask A P hP) P
  randomize_real := by
    intro A P hP
    exact C.transport.randomizedTarget_mem_real
      (C.unmasking.chooseMask A P hP) P hP
  recover := C.unmasking.recover
  recover_sound := by
    intro A hA P hP
    exact C.unmasking.recover_sound_of_accept A P hP
      (hA.1 (G.randomizedTarget (C.unmasking.chooseMask A P hP) P)
        (C.transport.randomizedTarget_mem_real
          (C.unmasking.chooseMask A P hP) P hP))
  solverAlg := C.solverAlg
  run_solverAlg := C.run_solverAlg

theorem builds_worst_case_solver
    (C : ActionSelfRandomizationCertificate (K := K) (G := G) WC)
    {A : K.Adversary} (hA : K.SolvesAveragePTND A) :
    WC.Solves ((C.toSelfRandomizationReduction).solverAlg A) :=
  (C.toSelfRandomizationReduction).builds_worst_case_solver hA

theorem hard_average_ptnd_of_action_self_randomization
    (C : ActionSelfRandomizationCertificate (K := K) (G := G) WC)
    (hHard : Hard WC.Solves) :
    Hard K.SolvesAveragePTND :=
  (C.toSelfRandomizationReduction).hard_average_ptnd_of_worst_case_hard hHard

end ActionSelfRandomizationCertificate

end DescenderCPACore

end Descender
