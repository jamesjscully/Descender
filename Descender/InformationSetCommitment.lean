import Mathlib.Tactic
import Mathlib.Data.ZMod.Basic

namespace Descender

/-!
Information-set commitment architectures.

This file isolates the abstract obstruction behind active-only compact
Fiat-Shamir signatures.  If the verifier receives only a challenge-selected
active projection of a prechallenge commitment, then the projection must be an
information set for the allowed commitment code: it must determine the
prechallenge commitment uniquely.

The theorem below is deliberately independent of the product-block arithmetic.
It is the counting lower bound that any clever commitment representation must
beat only by changing the architecture, for example by using stateful
precommitments, extra auxiliary bytes, or a lower-dimensional commitment code.
-/

namespace InformationSetCommitment

/--
An abstract verifier architecture for reconstructing commitments from active
views.

`Codeword` is the allowed prechallenge mask/commitment state.  It may be the
full mask space, a lower-dimensional commitment code, or a stateful committed
object.  `commit` maps that state to the Fiat-Shamir commitment view.  For a
challenge `c`, `project c` is the active view carried by the signature, and
`reconstruct c` is the verifier's attempted reconstruction of the full
commitment view from that active view.
-/
structure Architecture
    (Challenge Commitment ActiveView Codeword : Type*) where
  commit : Codeword -> Commitment
  project : Challenge -> Commitment -> ActiveView
  reconstruct : Challenge -> ActiveView -> Commitment

namespace Architecture

variable {Challenge Commitment ActiveView Codeword : Type*}

/-- The active payload map induced by one challenge. -/
def activeMap
    (A : Architecture Challenge Commitment ActiveView Codeword)
    (c : Challenge) : Codeword -> ActiveView :=
  fun w => A.project c (A.commit w)

/--
For a challenge `c`, the active view is an information set for the commitment
code if the verifier reconstructs the exact committed value for every allowed
codeword.
-/
def Reconstructs
    (A : Architecture Challenge Commitment ActiveView Codeword)
    (c : Challenge) : Prop :=
  ∀ w, A.reconstruct c (A.activeMap c w) = A.commit w

/-- Binding at a challenge: distinct allowed codewords have distinct active views. -/
def BindingAt
    (A : Architecture Challenge Commitment ActiveView Codeword)
    (c : Challenge) : Prop :=
  Function.Injective (A.activeMap c)

/--
If commitments themselves identify allowed codewords, then exact
reconstruction from the active view implies active-view binding.
-/
theorem bindingAt_of_reconstructs
    (A : Architecture Challenge Commitment ActiveView Codeword)
    {c : Challenge}
    (hcommit : Function.Injective A.commit)
    (hrec : A.Reconstructs c) :
    A.BindingAt c := by
  intro w₀ w₁ hactive
  apply hcommit
  calc
    A.commit w₀ = A.reconstruct c (A.activeMap c w₀) := (hrec w₀).symm
    _ = A.reconstruct c (A.activeMap c w₁) := by rw [hactive]
    _ = A.commit w₁ := hrec w₁

/--
Information-set counting lower bound.

Any active-only stateless architecture that reconstructs the prechallenge
commitment from the active view must fit the allowed codeword space into the
active-view space.
-/
theorem card_codeword_le_activeView_of_bindingAt
    (A : Architecture Challenge Commitment ActiveView Codeword)
    {c : Challenge}
    [Fintype Codeword] [Fintype ActiveView]
    (hbind : A.BindingAt c) :
    Fintype.card Codeword ≤ Fintype.card ActiveView :=
  Fintype.card_le_of_injective (A.activeMap c) hbind

/--
Counting lower bound in the reconstruction form that verifier designs usually
supply.
-/
theorem card_codeword_le_activeView_of_reconstructs
    (A : Architecture Challenge Commitment ActiveView Codeword)
    {c : Challenge}
    [Fintype Codeword] [Fintype ActiveView]
    (hcommit : Function.Injective A.commit)
    (hrec : A.Reconstructs c) :
    Fintype.card Codeword ≤ Fintype.card ActiveView :=
  A.card_codeword_le_activeView_of_bindingAt
    (A.bindingAt_of_reconstructs hcommit hrec)

end Architecture

section Fibered

variable {Challenge Commitment ActiveView MaskState CommitState : Type*}

/--
A fibered information-set architecture separates the mask sampled by the signer
from the lower-dimensional commitment state reconstructed by the verifier.

This is the abstraction needed for a possible near-active-only stateless
design: the active view may be an information set for `CommitState`, while
`MaskState` can remain larger if many masks map to the same commitment state.
-/
structure FiberedArchitecture
    (Challenge Commitment ActiveView MaskState CommitState : Type*) where
  commit : CommitState -> Commitment
  stateOfMask : MaskState -> CommitState
  project : Challenge -> Commitment -> ActiveView
  reconstruct : Challenge -> ActiveView -> Commitment

namespace FiberedArchitecture

variable {Challenge Commitment ActiveView MaskState CommitState : Type*}

/-- Forget the mask fibers and keep only the commitment-state architecture. -/
def toArchitecture
    (F :
      FiberedArchitecture Challenge Commitment ActiveView MaskState
        CommitState) :
    Architecture Challenge Commitment ActiveView CommitState where
  commit := F.commit
  project := F.project
  reconstruct := F.reconstruct

/-- The active payload map induced on mask states. -/
def maskActiveMap
    (F :
      FiberedArchitecture Challenge Commitment ActiveView MaskState
        CommitState)
    (c : Challenge) : MaskState -> ActiveView :=
  fun y => F.project c (F.commit (F.stateOfMask y))

/--
Exact reconstruction only needs to hold on commitment states.  This is weaker
than recovering the sampled mask.
-/
def Reconstructs
    (F :
      FiberedArchitecture Challenge Commitment ActiveView MaskState
        CommitState)
    (c : Challenge) : Prop :=
  ∀ s, F.reconstruct c (F.project c (F.commit s)) = F.commit s

/--
Fibered commitment-state lower bound.

The active view must still fit the commitment-state code, but this theorem
does not bound the full mask space unless the mask-to-state map is injective.
-/
theorem card_commitState_le_activeView_of_reconstructs
    (F :
      FiberedArchitecture Challenge Commitment ActiveView MaskState
        CommitState)
    {c : Challenge}
    [Fintype CommitState] [Fintype ActiveView]
    (hcommit : Function.Injective F.commit)
    (hrec : F.Reconstructs c) :
    Fintype.card CommitState ≤ Fintype.card ActiveView := by
  exact F.toArchitecture.card_codeword_le_activeView_of_reconstructs
    hcommit hrec

/--
If the mask determines the commitment state injectively, exact reconstruction
also makes the active view injective on masks.  Transparent low-dimensional
mask-code designs therefore inherit the active-view cardinality bound.
-/
theorem maskBindingAt_of_reconstructs_of_stateInjective
    (F :
      FiberedArchitecture Challenge Commitment ActiveView MaskState
        CommitState)
    {c : Challenge}
    (hstate : Function.Injective F.stateOfMask)
    (hcommit : Function.Injective F.commit)
    (hrec : F.Reconstructs c) :
    Function.Injective (F.maskActiveMap c) := by
  intro y₀ y₁ hactive
  apply hstate
  apply hcommit
  calc
    F.commit (F.stateOfMask y₀) =
        F.reconstruct c (F.maskActiveMap c y₀) := (hrec (F.stateOfMask y₀)).symm
    _ = F.reconstruct c (F.maskActiveMap c y₁) := by rw [hactive]
    _ = F.commit (F.stateOfMask y₁) := hrec (F.stateOfMask y₁)

/--
Transparent-mask lower bound.

A near-active-only design can avoid bounding the full mask space only by using
nontrivial fibers above the reconstructed commitment state.
-/
theorem card_maskState_le_activeView_of_reconstructs_of_stateInjective
    (F :
      FiberedArchitecture Challenge Commitment ActiveView MaskState
        CommitState)
    {c : Challenge}
    [Fintype MaskState] [Fintype ActiveView]
    (hstate : Function.Injective F.stateOfMask)
    (hcommit : Function.Injective F.commit)
    (hrec : F.Reconstructs c) :
    Fintype.card MaskState ≤ Fintype.card ActiveView :=
  Fintype.card_le_of_injective (F.maskActiveMap c)
    (F.maskBindingAt_of_reconstructs_of_stateInjective hstate hcommit hrec)

end FiberedArchitecture

end Fibered

section ProductBlock

variable {q : Nat} [NeZero q]
variable {Challenge Commitment Codeword MaskState CommitState ActiveBlock LocalFree : Type*}

/--
The independent active payload coordinates carried by a product-block
active-only signature for one challenge.  Pivot coordinates are not counted
here: in a systematic block, they are determined by the local free
coordinates.
-/
abbrev ProductBlockActiveView
    (q : Nat) (ActiveBlock LocalFree : Type*) :=
  ActiveBlock × LocalFree -> ZMod q

/--
The product-block active view with `w` active blocks and `local_free` free
coordinates per block has `q^(w * local_free)` possible q-symbol payloads.
-/
theorem card_productBlockActiveView
    [DecidableEq ActiveBlock] [DecidableEq LocalFree]
    [Fintype ActiveBlock] [Fintype LocalFree] :
    Fintype.card (ProductBlockActiveView q ActiveBlock LocalFree) =
      q ^ (Fintype.card ActiveBlock * Fintype.card LocalFree) := by
  classical
  simp [ProductBlockActiveView, Fintype.card_prod, ZMod.card]

/--
Product-block information-set lower bound.

If a stateless active-only verifier reconstructs the full prechallenge
commitment from the active product-block q-symbol view, then the allowed
codeword space has at most `q^(#active blocks * #local free coordinates)`
states.
-/
theorem card_codeword_le_productBlockActiveView_of_reconstructs
    [Fintype Codeword]
    [DecidableEq ActiveBlock] [DecidableEq LocalFree]
    [Fintype ActiveBlock] [Fintype LocalFree]
    (A :
      Architecture Challenge Commitment
        (ProductBlockActiveView q ActiveBlock LocalFree) Codeword)
    {c : Challenge}
    (hcommit : Function.Injective A.commit)
    (hrec : A.Reconstructs c) :
    Fintype.card Codeword ≤
      q ^ (Fintype.card ActiveBlock * Fintype.card LocalFree) := by
  simpa [card_productBlockActiveView] using
    A.card_codeword_le_activeView_of_reconstructs hcommit hrec

/--
Named-cardinality variant of the product-block lower bound.  This is the form
used in parameter notes: if the challenge exposes `w` active blocks and each
block has `localFree` independent response coordinates, then active-only
reconstruction forces `|Codeword| <= q^(w * localFree)`.
-/
theorem card_codeword_le_productBlockActiveView_of_reconstructs_named
    [Fintype Codeword]
    [DecidableEq ActiveBlock] [DecidableEq LocalFree]
    [Fintype ActiveBlock] [Fintype LocalFree]
    (A :
      Architecture Challenge Commitment
        (ProductBlockActiveView q ActiveBlock LocalFree) Codeword)
    {c : Challenge}
    {w localFree : Nat}
    (hactive : Fintype.card ActiveBlock = w)
    (hfree : Fintype.card LocalFree = localFree)
    (hcommit : Function.Injective A.commit)
    (hrec : A.Reconstructs c) :
    Fintype.card Codeword ≤ q ^ (w * localFree) := by
  simpa [hactive, hfree] using
    card_codeword_le_productBlockActiveView_of_reconstructs
      (q := q) (A := A) hcommit hrec

/--
The concrete `w = 1`, `local_free = 2` obstruction: an active-only stateless
verifier that reconstructs the prechallenge commitment from one active
two-coordinate block can support at most `q^2` allowed codewords per round.
-/
theorem card_codeword_le_singleBlock_twoFree_of_reconstructs
    [Fintype Codeword]
    (A :
      Architecture Challenge Commitment
        (ProductBlockActiveView q (Fin 1) (Fin 2)) Codeword)
    {c : Challenge}
    (hcommit : Function.Injective A.commit)
    (hrec : A.Reconstructs c) :
    Fintype.card Codeword ≤ q ^ 2 := by
  simpa using
    card_codeword_le_productBlockActiveView_of_reconstructs
      (q := q) (A := A) hcommit hrec

/--
Fibered product-block lower bound for the reconstructed commitment state.

This is the near-69B design target: only the commitment state is bounded by the
active product-block view.
-/
theorem card_commitState_le_productBlockActiveView_of_fibered_reconstructs
    [Fintype CommitState]
    [DecidableEq ActiveBlock] [DecidableEq LocalFree]
    [Fintype ActiveBlock] [Fintype LocalFree]
    (F :
      FiberedArchitecture Challenge Commitment
        (ProductBlockActiveView q ActiveBlock LocalFree) MaskState CommitState)
    {c : Challenge}
    (hcommit : Function.Injective F.commit)
    (hrec : F.Reconstructs c) :
    Fintype.card CommitState ≤
      q ^ (Fintype.card ActiveBlock * Fintype.card LocalFree) := by
  simpa [card_productBlockActiveView] using
    F.card_commitState_le_activeView_of_reconstructs hcommit hrec

/--
Transparent-mask product-block lower bound.

If the mask-to-commitment-state map is injective, the same product-block
active-view bound applies to the sampled mask space.  A viable fibered design
therefore needs nontrivial mask fibers.
-/
theorem card_maskState_le_productBlockActiveView_of_fibered_reconstructs_of_stateInjective
    [Fintype MaskState]
    [DecidableEq ActiveBlock] [DecidableEq LocalFree]
    [Fintype ActiveBlock] [Fintype LocalFree]
    (F :
      FiberedArchitecture Challenge Commitment
        (ProductBlockActiveView q ActiveBlock LocalFree) MaskState CommitState)
    {c : Challenge}
    (hstate : Function.Injective F.stateOfMask)
    (hcommit : Function.Injective F.commit)
    (hrec : F.Reconstructs c) :
    Fintype.card MaskState ≤
      q ^ (Fintype.card ActiveBlock * Fintype.card LocalFree) := by
  simpa [card_productBlockActiveView] using
    F.card_maskState_le_activeView_of_reconstructs_of_stateInjective
      hstate hcommit hrec

/--
Concrete fibered `w = 1`, `local_free = 2` commitment-state bound.
-/
theorem card_commitState_le_singleBlock_twoFree_of_fibered_reconstructs
    [Fintype CommitState]
    (F :
      FiberedArchitecture Challenge Commitment
        (ProductBlockActiveView q (Fin 1) (Fin 2)) MaskState CommitState)
    {c : Challenge}
    (hcommit : Function.Injective F.commit)
    (hrec : F.Reconstructs c) :
    Fintype.card CommitState ≤ q ^ 2 := by
  simpa using
    card_commitState_le_productBlockActiveView_of_fibered_reconstructs
      (q := q) (F := F) hcommit hrec

/--
Concrete `w = 1`, `local_free = 2` transparent-mask obstruction.

If active reconstruction is injective on the sampled mask state, then that
mask state has at most `q^2` possibilities per round.
-/
theorem card_maskState_le_singleBlock_twoFree_of_fibered_reconstructs_of_stateInjective
    [Fintype MaskState]
    (F :
      FiberedArchitecture Challenge Commitment
        (ProductBlockActiveView q (Fin 1) (Fin 2)) MaskState CommitState)
    {c : Challenge}
    (hstate : Function.Injective F.stateOfMask)
    (hcommit : Function.Injective F.commit)
    (hrec : F.Reconstructs c) :
    Fintype.card MaskState ≤ q ^ 2 := by
  simpa using
    card_maskState_le_productBlockActiveView_of_fibered_reconstructs_of_stateInjective
      (q := q) (F := F) hstate hcommit hrec

end ProductBlock

end InformationSetCommitment

end Descender
