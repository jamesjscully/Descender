import Mathlib.Data.Finset.Card
import Mathlib.Combinatorics.Enumerative.DoubleCounting

namespace Descender

/-!
Finite hypergraphs and peelability.

The definitions here are intentionally independent of vector spaces.  They
capture the combinatorial peeling condition used by the wedge-rank layer.
-/

universe u v

noncomputable section

structure Hypergraph (V : Type u) (E : Type v) [Fintype V] [Fintype E] where
  inc : E -> Finset V

namespace Hypergraph

variable {V : Type u} {E : Type v} [Fintype V] [Fintype E]

def degreeIn (H : Hypergraph V E) (S : Finset E) (v : V) : Nat := by
  classical
  exact (S.filter fun e => v ∈ H.inc e).card

def neighbors (H : Hypergraph V E) (S : Finset E) : Finset V := by
  classical
  exact Finset.univ.filter fun v => ∃ e ∈ S, v ∈ H.inc e

def Uniform (H : Hypergraph V E) (r : Nat) : Prop :=
  ∀ e, (H.inc e).card = r

def Peelable (H : Hypergraph V E) (tauN k : Nat) : Prop :=
  ∀ S : Finset E,
    S.Nonempty ->
      S.card ≤ tauN ->
        ∃ v : V, 1 ≤ H.degreeIn S v ∧ H.degreeIn S v ≤ k

theorem peelable_of_neighbor_expansion
    (H : Hypergraph V E) {tauN k r : Nat}
    (hUniform : H.Uniform r)
    (hExpand :
      ∀ S : Finset E,
        S.Nonempty ->
          S.card ≤ tauN ->
            r * S.card < (k + 1) * (H.neighbors S).card) :
    H.Peelable tauN k := by
  classical
  intro S hSnonempty hSsmall
  by_contra hNoPeel
  push Not at hNoPeel
  let R : E -> V -> Prop := fun e v => v ∈ H.inc e
  have hLower :
      ∀ v ∈ H.neighbors S,
        k + 1 ≤ (S.bipartiteBelow R v).card := by
    intro v hv
    have hdeg_pos : 1 ≤ H.degreeIn S v := by
      rw [neighbors] at hv
      rcases (Finset.mem_filter.mp hv).2 with ⟨e, heS, hev⟩
      unfold degreeIn
      exact Finset.card_pos.mpr ⟨e, Finset.mem_filter.mpr ⟨heS, hev⟩⟩
    have hdeg_succ : k + 1 ≤ H.degreeIn S v :=
      Nat.succ_le_iff.mpr (hNoPeel v hdeg_pos)
    simpa [R, degreeIn, Finset.bipartiteBelow] using hdeg_succ
  have hUpper :
      ∀ e ∈ S,
        ((H.neighbors S).bipartiteAbove R e).card ≤ r := by
    intro e heS
    have hEq :
        (H.neighbors S).bipartiteAbove R e = H.inc e := by
      ext v
      constructor
      · intro hv
        exact (Finset.mem_filter.mp hv).2
      · intro hv
        exact Finset.mem_filter.mpr
          ⟨by
            rw [neighbors]
            exact Finset.mem_filter.mpr ⟨Finset.mem_univ v, ⟨e, heS, hv⟩⟩,
           hv⟩
    calc
      ((H.neighbors S).bipartiteAbove R e).card = (H.inc e).card := by rw [hEq]
      _ = r := hUniform e
      _ ≤ r := le_rfl
  have hCore :
      (k + 1) * (H.neighbors S).card ≤ r * S.card := by
    have hCount :
        (H.neighbors S).card • (k + 1) ≤ S.card • r :=
      Finset.card_nsmul_le_card_nsmul' (r := R) hLower hUpper
    simpa [nsmul_eq_mul, Nat.mul_comm, Nat.mul_left_comm, Nat.mul_assoc] using hCount
  exact (not_lt_of_ge hCore) (hExpand S hSnonempty hSsmall)

theorem three_uniform_peelable_of_neighbor_expansion
    (H : Hypergraph V E) {tauN k : Nat}
    (hUniform : H.Uniform 3)
    (hExpand :
      ∀ S : Finset E,
        S.Nonempty ->
          S.card ≤ tauN ->
            3 * S.card < (k + 1) * (H.neighbors S).card) :
    H.Peelable tauN k :=
  H.peelable_of_neighbor_expansion hUniform hExpand

end Hypergraph

end

end Descender
