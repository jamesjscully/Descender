import Mathlib.Data.ZMod.Basic

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
The active projection equations are represented after the bridge semantics
rewrite as one verifier-visible variable per active lane.  Nullity zero means
that setting every verifier-visible lane equation to zero forces the active
lane vector itself to be zero.
-/
def ActiveLaneNullityZero (q laneCount : Nat) [NeZero q] : Prop :=
  ∀ x : Fin laneCount -> ZMod q, (∀ lane, x lane = 0) -> x = 0

theorem activeLaneNullityZero (q laneCount : Nat) [NeZero q] :
    ActiveLaneNullityZero q laneCount := by
  intro x hzero
  funext lane
  exact hzero lane

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
  active_lane_nullity_zero :
    ActiveLaneNullityZero q activeProjectionLanes

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

theorem active_relation_only_zero
    (C : ActiveSubspaceCertificate q)
    (x : Fin C.activeProjectionLanes -> ZMod q)
    (hzero : ∀ lane, x lane = 0) :
    x = 0 :=
  C.active_lane_nullity_zero x hzero

theorem inactive_projection_column_count
    (C : ActiveSubspaceCertificate q) :
    oldInactiveProjectionColumns C.oldProjectionColumns C.symbols = 52 :=
  C.inactive_columns_eq

end ActiveSubspaceCertificate

end HiddenSourceActiveSubspace
end Descender
