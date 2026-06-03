import Descender.DescentInterface

namespace Descender

/-!
# Path 2 Descent/SIS Type Map

This sidecar module instantiates the lightweight descent vocabulary for the
Path 2 account-creation proof payload.  It is intentionally proof-interface
level only: concrete SIS matrix domains, binding/hiding assumptions, norm
bounds, and extractor proofs are separate obligations.

The public quotient is the six-lane masked projection vector already carried by
the canonical 18-residue packed witness commitment.  A representative supplies
hidden pad and account-projection lanes plus the public commitments/proof bytes
that a concrete `Path2DescentSisCommitmentV1` verifier will check.
-/

namespace Path2DescentSIS

abbrev Lane := Fin 6

/-- Six q12289 residues viewed as the public quotient for Path 2 descent. -/
structure MaskedProjection where
  lane : Lane -> Nat

/--
Public Path 2 creation object consumed by the descent wrapper.

The byte-level fields are represented abstractly as `Nat`; Rust owns canonical
byte framing.  This Lean sidecar only fixes which logical fields belong to the
public descent object.
-/
structure Public where
  leafDigest : Nat
  creationProofBindingDigest : Nat
  proofPayloadDigest : Nat
  relationTranscriptDigest : Nat
  maskedProjection : MaskedProjection

/-- Local representative supplied by the proof payload verifier. -/
structure Representative where
  padCommitment : Nat
  projectionCommitment : Nat
  openingTupleCommitment : Nat
  proofTranscriptCommitment : Nat
  maskedProjection : MaskedProjection

/-- Hidden account-opening data that the descent theorem should recover/glue. -/
structure HiddenOpening where
  padLane : Lane -> Nat
  accountProjectionLane : Lane -> Nat
  openingTupleWitness : Nat
  padBlinding : Nat
  projectionBlinding : Nat
  openingTupleBlinding : Nat

/-- Account/state evidence tying the hidden opening back to Path 2 admission. -/
structure AccountEvidence where
  accountId : Nat
  publicKeyIdentity : Nat
  slotId : Nat
  verifierVersionDigest : Nat
  accountStateRootBefore : Nat
  lifecycleAnchor : Nat

/-- Named break events for the Path 2 descent/SIS payload boundary. -/
inductive BreakEvent where
  | sisBindingBreak
  | samplerHidingBreak
  | publicProjectionBreak
  | normOrRangeBreak
  | fiatShamirOrDomainBreak
  | extractorSoundnessBreak
  | noExportBreak
  deriving DecidableEq

/-- Commitment roles used by the Path 2 descent/SIS payload candidate. -/
inductive CommitmentRole where
  | padSource
  | projectionRelation
  | openingTuple
  | transcriptCommitment
  deriving DecidableEq

/--
Column layout and norm/range policy for one SIS commitment role.

The Rust verifier owns byte parsing and concrete centered-lift checks. This
Lean policy pins the role partition consumed by the proof boundary.
-/
structure RoleLayout where
  columns : Nat
  semanticColumns : Nat
  blindingColumns : Nat
  rangeColumns : Nat
  blindingCenteredBound : Nat
  rangeDigitMax : Nat

/-- Selected executable role layout for the current Path 2 descent/SIS probe. -/
def layoutForRole : CommitmentRole -> RoleLayout
  | .padSource =>
      { columns := 30, semanticColumns := 6, blindingColumns := 24,
        rangeColumns := 0, blindingCenteredBound := 2048,
        rangeDigitMax := 127 }
  | .projectionRelation =>
      { columns := 30, semanticColumns := 6, blindingColumns := 24,
        rangeColumns := 0, blindingCenteredBound := 2048,
        rangeDigitMax := 127 }
  | .openingTuple =>
      { columns := 48, semanticColumns := 8, blindingColumns := 32,
        rangeColumns := 8, blindingCenteredBound := 2048,
        rangeDigitMax := 127 }
  | .transcriptCommitment =>
      { columns := 108, semanticColumns := 20, blindingColumns := 80,
        rangeColumns := 8, blindingCenteredBound := 2048,
        rangeDigitMax := 127 }

/-- Each selected role layout partitions the exact commitment column count. -/
theorem layoutForRole_partitions (role : CommitmentRole) :
    let L := layoutForRole role
    L.semanticColumns + L.blindingColumns + L.rangeColumns = L.columns := by
  cases role <;> rfl

/-- Embed a semantic lane index into the full role opening. -/
def semanticColumnIndex (role : CommitmentRole)
    (i : Fin (layoutForRole role).semanticColumns) :
    Fin (layoutForRole role).columns :=
  ⟨i.val, by
    cases role <;> simp [layoutForRole] at i ⊢ <;> omega⟩

/-- Embed a blinding lane index into the full role opening. -/
def blindingColumnIndex (role : CommitmentRole)
    (i : Fin (layoutForRole role).blindingColumns) :
    Fin (layoutForRole role).columns :=
  ⟨(layoutForRole role).semanticColumns + i.val, by
    cases role <;> simp [layoutForRole] at i ⊢ <;> omega⟩

/-- Embed a range-digit lane index into the full role opening. -/
def rangeColumnIndex (role : CommitmentRole)
    (i : Fin (layoutForRole role).rangeColumns) :
    Fin (layoutForRole role).columns :=
  match role with
  | .padSource => nomatch i
  | .projectionRelation => nomatch i
  | .openingTuple =>
      ⟨(layoutForRole .openingTuple).semanticColumns +
          (layoutForRole .openingTuple).blindingColumns + i.val, by
        simp [layoutForRole] at i ⊢
        omega⟩
  | .transcriptCommitment =>
      ⟨(layoutForRole .transcriptCommitment).semanticColumns +
          (layoutForRole .transcriptCommitment).blindingColumns + i.val, by
        simp [layoutForRole] at i ⊢
        omega⟩

/-- Per-segment semantic bounds; transcript commitments inherit constituent bounds. -/
def semanticCenteredBoundFor (role : CommitmentRole)
    (i : Fin (layoutForRole role).semanticColumns) : Nat :=
  match role with
  | .padSource => 96
  | .projectionRelation => 96
  | .openingTuple => 512
  | .transcriptCommitment =>
      if i.val < 12 then 96 else 512

/--
Abstract norm/range acceptance for a role opening.

Concrete Rust checks enforce canonical q12289 centered lifts, semantic/blinding
centered bounds, nonnegative base-128 range digits, and exact column count.
-/
structure OpeningNormRangeValid (role : CommitmentRole) where
  opening : Fin (layoutForRole role).columns -> Int
  semantic_bound :
    ∀ i : Fin (layoutForRole role).semanticColumns,
      (opening (semanticColumnIndex role i)).natAbs ≤
        semanticCenteredBoundFor role i
  blinding_bound :
    ∀ i : Fin (layoutForRole role).blindingColumns,
      (opening (blindingColumnIndex role i)).natAbs ≤
        (layoutForRole role).blindingCenteredBound
  range_digit_bound :
    ∀ i : Fin (layoutForRole role).rangeColumns,
      0 ≤ opening (rangeColumnIndex role i) ∧
      (opening (rangeColumnIndex role i)).natAbs ≤
        (layoutForRole role).rangeDigitMax

/-- The public quotient of a representative is its masked projection vector. -/
def quotientOfRepresentative (rep : Representative) : MaskedProjection :=
  rep.maskedProjection

/-- The public quotient of the public Path 2 object is its masked projection. -/
def quotientOfPublic (pub : Public) : MaskedProjection :=
  pub.maskedProjection

/--
Predicate package for the still-open concrete SIS proof obligations.

Keeping these predicates abstract lets this module close the type-map row
without smuggling in an unproved commitment theorem.
-/
structure Predicates where
  representativeOpens : Public -> Representative -> Prop
  hiddenRepresents : Public -> Representative -> HiddenOpening -> Prop
  accountBacked : Public -> HiddenOpening -> AccountEvidence -> Prop
  breakFor : Public -> Representative -> BreakEvent -> Prop

/-- Path 2 descent/SIS instance of the generic local quotient descent problem. -/
def localQuotientDescent (P : Predicates) :
    DescentInterface.LocalQuotientDescent
      Public Representative MaskedProjection HiddenOpening AccountEvidence
      BreakEvent where
  quotientOfRepresentative := quotientOfRepresentative
  quotientOfPublic := quotientOfPublic
  representativeOpens := P.representativeOpens
  hiddenRepresents := P.hiddenRepresents
  accountBacked := P.accountBacked
  breakFor := P.breakFor

/-- Local validity includes equality to the public masked-projection quotient. -/
theorem locallyValid_implies_maskedProjection_eq
    (P : Predicates)
    {pub : Public} {rep : Representative}
    (h :
      (localQuotientDescent P).LocallyValid pub rep) :
    rep.maskedProjection = pub.maskedProjection := by
  exact h.1

/--
Wrapper theorem for the Path 2 type map.

Once a concrete SIS verifier proves that acceptance yields descent data and an
effective descent certificate is supplied, accepted payloads glue to hidden
account-backed openings or export a named break.
-/
theorem accepts_glues_or_breaks
    (P : Predicates)
    (C :
      DescentInterface.EffectiveDescentCertificate
        (localQuotientDescent P))
    {accepts : Public -> Prop}
    (accepts_yields_descent :
      forall {pub : Public},
        accepts pub -> (localQuotientDescent P).DescentData pub)
    {pub : Public}
    (hAccept : accepts pub) :
    (exists representative hidden evidence,
      (localQuotientDescent P).LocallyValid pub representative ∧
        P.hiddenRepresents pub representative hidden ∧
          P.accountBacked pub hidden evidence) ∨
    (exists representative breakEvent,
      (localQuotientDescent P).LocallyValid pub representative ∧
        P.breakFor pub representative breakEvent) :=
  DescentInterface.EffectiveDescentCertificate.accepts_glues_or_breaks
    C accepts_yields_descent hAccept

end Path2DescentSIS
end Descender
