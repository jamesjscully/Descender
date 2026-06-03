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
