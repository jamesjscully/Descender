import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

namespace Descender
namespace HiddenSourceActiveSubspace

/-!
Lean-side certificate shape for the hidden-source active projection subspace.

The review scripts may discover the selected projection lanes, but this module
keeps the durable proof object small: concrete generated modules instantiate
`ActiveSubspaceCertificate` with literal columns/coefficients and prove the
finite checks by Lean.
-/

structure ActiveLaneSymbol where
  lane : Nat
  sourceColumn : Nat
  coefficientModQ : Nat
deriving Repr, DecidableEq

def sourceColumns (symbols : List ActiveLaneSymbol) : List Nat :=
  symbols.map (fun s => s.sourceColumn)

def laneIndices (symbols : List ActiveLaneSymbol) : List Nat :=
  symbols.map (fun s => s.lane)

def coefficients (symbols : List ActiveLaneSymbol) : List Nat :=
  symbols.map (fun s => s.coefficientModQ)

def allColumnsBelow (oldProjectionColumns : Nat)
    (symbols : List ActiveLaneSymbol) : Prop :=
  ∀ s ∈ symbols, s.sourceColumn < oldProjectionColumns

def allLanesBelow (laneCount : Nat) (symbols : List ActiveLaneSymbol) : Prop :=
  ∀ s ∈ symbols, s.lane < laneCount

def oldInactiveProjectionColumns
    (oldProjectionColumns : Nat) (symbols : List ActiveLaneSymbol) : Nat :=
  oldProjectionColumns - symbols.length

def survivorExcluded
    (survivorColumn : Nat) (symbols : List ActiveLaneSymbol) : Prop :=
  survivorColumn ∉ sourceColumns symbols

/--
Each active lane reads exactly one source column scaled by a coefficient: lane
`s` evaluates to `s.coefficientModQ * x s.sourceColumn` on a source vector `x`
over the old projection columns.
-/
def laneValue (q : Nat) [NeZero q] (x : Nat -> ZMod q)
    (s : ActiveLaneSymbol) : ZMod q :=
  (s.coefficientModQ : ZMod q) * x s.sourceColumn

/--
Checkable non-degeneracy fact: every active lane coefficient is nonzero modulo
`q`.  This is the literal-data condition that makes the readout faithful (a zero
coefficient would let a source coordinate change without moving its lane).
-/
def coefficientsNonzeroModQ (q : Nat) [NeZero q]
    (symbols : List ActiveLaneSymbol) : Prop :=
  ∀ s ∈ symbols, (s.coefficientModQ : ZMod q) ≠ 0

/--
The faithful "active-lane nullity zero" property the bridge semantics intend:
if every verifier-visible lane equation reads zero, then every active source
coordinate read by a lane is itself zero.  Unlike the earlier placeholder, this
is **not** a tautology — it fails whenever some lane coefficient is `0` modulo
`q`, so proving it has real content (it pins down that the active readout cannot
silently absorb a nonzero active-column perturbation).
-/
def ActiveLaneReadoutDetects (q : Nat) [NeZero q]
    (symbols : List ActiveLaneSymbol) : Prop :=
  ∀ x : Nat -> ZMod q,
    (∀ s ∈ symbols, laneValue q x s = 0) ->
      ∀ s ∈ symbols, x s.sourceColumn = 0

/--
When `q` is prime (`ZMod q` is a field, hence has no zero divisors) and every
lane coefficient is nonzero, the active-lane readout detects every nonzero
active-column perturbation.  This is the real content the generated certificate
discharges over its literal coefficients.
-/
theorem activeLaneReadoutDetects_of_coefficientsNonzero
    (q : Nat) [NeZero q] [Fact (Nat.Prime q)]
    (symbols : List ActiveLaneSymbol)
    (hco : coefficientsNonzeroModQ q symbols) :
    ActiveLaneReadoutDetects q symbols := by
  intro x hx s hs
  rcases mul_eq_zero.mp (hx s hs) with h | h
  · exact absurd h (hco s hs)
  · exact h

/--
Concrete finite certificate facts for one generated active-subspace artifact.

This deliberately avoids importing the JSON ledger as trust.  A generated Lean
module must provide literal data and prove these finite facts inside Lean.
-/
structure ActiveSubspaceCertificate (q : Nat) [NeZero q] where
  oldProjectionColumns : Nat
  activeProjectionLanes : Nat
  survivorColumn : Nat
  symbols : List ActiveLaneSymbol
  symbols_length :
    symbols.length = activeProjectionLanes
  sourceColumns_nodup :
    (sourceColumns symbols).Nodup
  laneIndices_nodup :
    (laneIndices symbols).Nodup
  allColumns_lt :
    allColumnsBelow oldProjectionColumns symbols
  allLanes_lt :
    allLanesBelow activeProjectionLanes symbols
  survivor_excluded :
    survivorExcluded survivorColumn symbols
  inactive_columns_eq :
    oldInactiveProjectionColumns oldProjectionColumns symbols = 52
  coefficients_nonzero :
    coefficientsNonzeroModQ q symbols

namespace ActiveSubspaceCertificate

variable {q : Nat} [NeZero q]

theorem active_lane_count
    (C : ActiveSubspaceCertificate q) :
    C.symbols.length = C.activeProjectionLanes :=
  C.symbols_length

theorem old_survivor_not_active
    (C : ActiveSubspaceCertificate q) :
    C.survivorColumn ∉ sourceColumns C.symbols :=
  C.survivor_excluded

/--
The real readout-detection property for a certificate, derived from its
literal nonzero coefficients (requires `q` prime so `ZMod q` is a field).
Replaces the earlier vacuous `active_relation_only_zero`.
-/
theorem readout_detects [Fact (Nat.Prime q)]
    (C : ActiveSubspaceCertificate q) :
    ActiveLaneReadoutDetects q C.symbols :=
  activeLaneReadoutDetects_of_coefficientsNonzero q C.symbols C.coefficients_nonzero

theorem inactive_projection_column_count
    (C : ActiveSubspaceCertificate q) :
    oldInactiveProjectionColumns C.oldProjectionColumns C.symbols = 52 :=
  C.inactive_columns_eq

end ActiveSubspaceCertificate

end HiddenSourceActiveSubspace
end Descender
