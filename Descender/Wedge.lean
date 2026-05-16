import Mathlib.LinearAlgebra.LinearIndependent.Basic
import Descender.Hypergraph

namespace Descender

/-!
Abstract wedge systems and local independence data.

This layer deliberately avoids simplicial-complex machinery.  A wedge has
three endpoint vertices, and a divergence column is a vector over
`Vertex × Fiber`.
-/

universe u v w f

noncomputable section

structure WedgeSystem
    (Vertex : Type u) (Wedge : Type v) [Fintype Vertex] [Fintype Wedge] where
  endpoints : Wedge -> Finset Vertex
  endpoints_card : ∀ w, (endpoints w).card = 3

namespace WedgeSystem

variable {F : Type f} [Field F]
variable {Vertex : Type u} {Wedge : Type v} {Fiber : Type w}
variable [Fintype Vertex] [Fintype Wedge] [Fintype Fiber]

def toHypergraph (W : WedgeSystem Vertex Wedge) : Hypergraph Vertex Wedge where
  inc := W.endpoints

def localVec
    (h : Wedge -> Vertex -> Fiber -> F) (v : Vertex) (w : Wedge) :
    Fiber -> F :=
  h w v

def column
    (h : Wedge -> Vertex -> Fiber -> F) (w : Wedge) :
    Vertex × Fiber -> F :=
  fun x => h w x.1 x.2

def SupportedOnEndpoints
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F) : Prop :=
  ∀ w v, v ∉ W.endpoints w -> h w v = 0

def LocallyIndependentUpTo
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F) (k : Nat) : Prop :=
  ∀ v (S : Finset Wedge),
    S.card ≤ k ->
      (∀ w, w ∈ S -> v ∈ W.endpoints w) ->
        LinearIndependent F
          (fun x : {w // w ∈ S} => localVec h v x.1)

def LocalIndependenceData
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F) (k : Nat) : Prop :=
  SupportedOnEndpoints W h ∧ LocallyIndependentUpTo W h k

end WedgeSystem

end

end Descender
