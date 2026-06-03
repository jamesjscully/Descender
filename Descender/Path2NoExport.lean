import Descender.Path2NativeAccountDesign
import Descender.Path2NativeBoundary

namespace Descender

/-!
# Path 2 No-Export Closure

This module connects the existing public-output allowlist/denylist interface to
the active Path 2 native accept, reject, and storage output surfaces.

The concrete Rust serializers still have to instantiate the `outputConforms`
premises.  Once they do, the theorem below closes the Lean-side
`noExportClosed` proposition by deriving absence of denylisted private fields
on every active public surface.
-/

namespace Path2NoExport

open Path2NativeAccountDesign

/-- Active public-output surfaces for the native verifier path. -/
structure ActiveNativeOutputSurfaces where
  acceptOutputProduced :
    Boundary -> Nat -> Nat -> Nat -> Nat -> Prop
  rejectOutputProduced :
    Boundary -> BreakEvent -> Nat -> Prop
  storageOutputProduced :
    Boundary -> Nat -> Prop

/--
Serializer conformance for every active public-output surface.

The policy is the allowlist/denylist contract.  The three fields correspond to
accepted verifier output, fail-closed reject diagnostics, and public persistent
state or spent-ledger records.
-/
structure ActiveNativeOutputConformance
    (Policy : PublicOutputPolicy)
    (Surfaces : ActiveNativeOutputSurfaces) where
  accept_conforms :
    forall {q sig msg state output},
      Surfaces.acceptOutputProduced q sig msg state output ->
        Policy.outputConforms output
  reject_conforms :
    forall {q breakEvent output},
      Surfaces.rejectOutputProduced q breakEvent output ->
        Policy.outputConforms output
  storage_conforms :
    forall {q output},
      Surfaces.storageOutputProduced q output ->
        Policy.outputConforms output

/-- Denylisted private material is absent from all active public surfaces. -/
def ActiveNativeNoExportClosure
    (Policy : PublicOutputPolicy)
    (Surfaces : ActiveNativeOutputSurfaces) : Prop :=
    (forall {q sig msg state output},
      Surfaces.acceptOutputProduced q sig msg state output ->
        ¬ Policy.containsDenylistedField output) ∧
    (forall {q breakEvent output},
      Surfaces.rejectOutputProduced q breakEvent output ->
        ¬ Policy.containsDenylistedField output) ∧
    (forall {q output},
        Surfaces.storageOutputProduced q output ->
          ¬ Policy.containsDenylistedField output)

/--
Main allowlist/denylist closure theorem for AP2-N005/P2FS-004.

If each active accept/reject/storage serializer conforms to the public-output
policy, and conforming outputs have no denylisted field, then the active native
public-output surface is no-export closed.
-/
theorem active_native_no_export_closure_from_public_output_no_export
    {Policy : PublicOutputPolicy}
    {Surfaces : ActiveNativeOutputSurfaces}
    (NoExport : PublicOutputNoExport Policy)
    (Conforms : ActiveNativeOutputConformance Policy Surfaces) :
    ActiveNativeNoExportClosure Policy Surfaces := by
  constructor
  ·
    intro q sig msg state output hProduced
    exact NoExport.no_denylisted_field
      (Conforms.accept_conforms hProduced)
  constructor
  ·
    intro q breakEvent output hProduced
    exact NoExport.no_denylisted_field
      (Conforms.reject_conforms hProduced)
  ·
    intro q output hProduced
    exact NoExport.no_denylisted_field
      (Conforms.storage_conforms hProduced)

/--
Use an active no-export closure as the `noExportClosed` field of an accepted
spend soundness boundary.

This keeps the full-signature boundary abstract while making the P2FS-004
instantiation explicit: the field is closed by the concrete active surface
closure proposition, not by an unrelated assumption.
-/
def NativeAcceptedSpendSoundnessBoundary.withActiveNoExportClosure
    (B : NativeAcceptedSpendSoundnessBoundary)
    {Policy : PublicOutputPolicy}
    (Surfaces : ActiveNativeOutputSurfaces) :
    NativeAcceptedSpendSoundnessBoundary :=
  { B with noExportClosed := ActiveNativeNoExportClosure Policy Surfaces }

/-- The wrapped accepted-spend boundary exposes exactly the active closure. -/
theorem with_active_no_export_closure_closed
    (B : NativeAcceptedSpendSoundnessBoundary)
    {Policy : PublicOutputPolicy}
    {Surfaces : ActiveNativeOutputSurfaces}
    (Closed : ActiveNativeNoExportClosure Policy Surfaces) :
    (NativeAcceptedSpendSoundnessBoundary.withActiveNoExportClosure
      B (Policy := Policy) Surfaces).noExportClosed :=
  Closed

/--
Fiber privacy plus public-output conformance gives the stronger accepted-output
fact used by active verifier wrappers: the output is simulatable from public
inputs and contains no denylisted field, unless the verifier exports a named
break.
-/
theorem accepted_fiber_view_simulatable_and_no_denylisted_or_break
    {O : ObstructionMap}
    {P : PrivateFiberPredicates}
    {View : PublicViewPredicates}
    {V : VerifierPredicates}
    {Policy : PublicOutputPolicy}
    (FiberSound : FiberNoExportSoundness O P View V)
    (NoExport : PublicOutputNoExport Policy)
    {q : Boundary} {h : PrivateFiber} {sig msg state view : Nat}
    (hValid : LocalValid O P q h)
    (hView : View.publicViewProduced q h sig msg state view)
    (hConforms : Policy.outputConforms view) :
    (View.simulatableFromPublic q sig msg state view ∧
        ¬ Policy.containsDenylistedField view) ∨
      exists breakEvent, V.breakFor q breakEvent := by
  cases public_view_simulatable_or_break FiberSound hValid hView with
  | inl hSim =>
      exact Or.inl
        ⟨hSim, NoExport.no_denylisted_field hConforms⟩
  | inr hBreak =>
      exact Or.inr hBreak

/--
Transparent/native-boundary no-export wrappers can also be paired with the same
public-output policy.  This is useful when a reviewed Fq12 or transparent trace
view remains the proof-side source of the no-export fact, while Rust exposes a
separate serialized public output.
-/
theorem transparent_view_no_export_and_no_denylisted_or_break
    {Preds : Path2NativeBoundary.Predicates}
    {Policy : PublicOutputPolicy}
    (Sound : Path2NativeBoundary.NoExportSoundness Preds)
    (NoExport : PublicOutputNoExport Policy)
    (outputOf :
      Path2NativeBoundary.Boundary ->
        Path2NativeBoundary.TransparentPublicView ->
          Path2NativeBoundary.ColumnProfile -> Nat)
    (output_conforms :
      forall {b view profile},
        Preds.transparentAccepts b view ->
          Path2NativeBoundary.TransparentViewProjectsFrom b view ->
            Path2NativeBoundary.ColumnProfileCompatibleWithBoundary b view
              profile ->
              Policy.outputConforms (outputOf b view profile))
    {b : Path2NativeBoundary.Boundary}
    {view : Path2NativeBoundary.TransparentPublicView}
    {profile : Path2NativeBoundary.ColumnProfile}
    (hAccept : Preds.transparentAccepts b view)
    (hProjects : Path2NativeBoundary.TransparentViewProjectsFrom b view)
    (hProfile :
      Path2NativeBoundary.ColumnProfileCompatibleWithBoundary b view profile) :
    (Preds.noExportHolds b view profile ∧
        ¬ Policy.containsDenylistedField (outputOf b view profile)) ∨
      exists breakEvent, Preds.boundaryBreakFor b.context b breakEvent := by
  cases
      Path2NativeBoundary.no_export_holds_or_break
        Preds Sound hAccept hProjects hProfile with
  | inl hNoExport =>
      exact Or.inl
        ⟨hNoExport,
          NoExport.no_denylisted_field
            (output_conforms hAccept hProjects hProfile)⟩
  | inr hBreak =>
      exact Or.inr hBreak

end Path2NoExport
end Descender
