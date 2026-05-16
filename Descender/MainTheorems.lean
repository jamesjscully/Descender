import Descender.GuardedTilt
import Descender.PeelingRank

namespace Descender

/-!
Public deterministic-core theorem surface.

The declarations here are intentionally thin wrappers around the guarded bundle
and wedge-rank layers.  Cauchy-specific construction data and finite-column
representation details stay below this boundary.
-/

theorem descender_deterministic_core_dual_integral
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedDeterministicExportBundle (F := F) frame)
    {r : Vector J0 F × Vector J1 F}
    (hAT : B.A.adjoint r = zeroVec)
    (hr : r ≠ zeroPair)
    (hSmall : pairWeightOn frame.j0 frame.j1 r <= B.dualWindow) :
    B.dualWindow + 1 <=
      (B.tiltColumnBudget + 1) * pairWeightOn frame.j0 frame.j1 r :=
  B.dual_integral hAT hr hSmall

theorem descender_deterministic_core_primal
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedDeterministicExportBundle (F := F) frame) :
    PrimalDistanceAtLeastOp frame.rows frame.j0 frame.j1 B.A B.primalCutoff :=
  B.primal_distance

theorem descender_deterministic_core_primal_combined
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedCombinedDeterministicExportBundle (F := F) frame) :
    PrimalDistanceAtLeastOp frame.rows frame.j0 frame.j1 B.A B.primalCutoff :=
  B.primal_distance

structure DescenderDeterministicCore
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedDeterministicExportBundle (F := F) frame) : Prop where
  dualIntegral :
    ∀ r : Vector J0 F × Vector J1 F,
      B.A.adjoint r = zeroVec ->
      r ≠ zeroPair ->
      pairWeightOn frame.j0 frame.j1 r <= B.dualWindow ->
      B.dualWindow + 1 <=
        (B.tiltColumnBudget + 1) * pairWeightOn frame.j0 frame.j1 r
  primal :
    PrimalDistanceAtLeastOp frame.rows frame.j0 frame.j1 B.A B.primalCutoff

theorem descender_deterministic_core
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedDeterministicExportBundle (F := F) frame) :
    DescenderDeterministicCore B := by
  refine ⟨?dual, descender_deterministic_core_primal B⟩
  intro r hAT hr hSmall
  exact descender_deterministic_core_dual_integral B hAT hr hSmall

structure DescenderCombinedDeterministicCore
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedCombinedDeterministicExportBundle (F := F) frame) : Prop where
  dualIntegral :
    ∀ r : Vector J0 F × Vector J1 F,
      B.A.adjoint r = zeroVec ->
      r ≠ zeroPair ->
      pairWeightOn frame.j0 frame.j1 r <= B.dualWindow ->
      B.dualWindow + 1 <=
        (B.tiltColumnBudget + 1) * pairWeightOn frame.j0 frame.j1 r
  primal :
    PrimalDistanceAtLeastOp frame.rows frame.j0 frame.j1 B.A B.primalCutoff

theorem descender_deterministic_core_combined
    [Field F] [DecidableEq F] [DecidableEq J1]
    {frame : GuardedCoordinateFrame I J0 J1}
    (B : GuardedCombinedDeterministicExportBundle (F := F) frame) :
    DescenderCombinedDeterministicCore B := by
  refine ⟨?dual, descender_deterministic_core_primal_combined B⟩
  intro r hAT hr hSmall
  exact B.dual_integral hAT hr hSmall

theorem descender_patch_rank
    {F : Type*} [Field F]
    {Vertex Wedge Fiber : Type*}
    [Fintype Vertex] [Fintype Wedge] [Fintype Fiber]
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F)
    {tauN k : Nat}
    (hPeel : W.toHypergraph.Peelable tauN k)
    (hSupport : W.SupportedOnEndpoints h)
    (hLocal : W.LocallyIndependentUpTo h k)
    (S : Finset Wedge)
    (hS : S.card <= tauN) :
    LinearIndependent F
      (fun x : {w // w ∈ S} => WedgeSystem.column h x.1) :=
  W.peelability_implies_linear_independence h hPeel hSupport hLocal S hS

end Descender
