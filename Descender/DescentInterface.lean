import Mathlib.Tactic

namespace Descender

/-!
# Lightweight Descent Interface

This module records the finite descent pattern used by the current
account-carried quotient signature work.  It intentionally avoids category
theory: the purpose is to name the proof shape, not to rebuild the verifier
around stacks.

The intended reading is:

* a public verifier object determines a public quotient;
* a representative is only meaningful inside the fiber over that quotient;
* local verifier equations make a representative locally valid;
* effective descent says locally valid data glues to hidden account-backed
  state, unless one of the named break events occurs.
-/

namespace DescentInterface

/--
A quotient presentation for representatives.

This is the smallest reusable vocabulary for "fiber over a public quotient".
Concrete q12289 instantiations should use the packed q-syndrome/account digest
as the quotient type.
-/
structure QuotientPresentation
    (Representative PublicQuotient : Type*) where
  quotientOf : Representative -> PublicQuotient

namespace QuotientPresentation

variable {Representative PublicQuotient : Type*}

def Fiber
    (Q : QuotientPresentation Representative PublicQuotient)
    (q : PublicQuotient) : Type _ :=
  { representative : Representative // Q.quotientOf representative = q }

theorem fiber_property
    (Q : QuotientPresentation Representative PublicQuotient)
    {q : PublicQuotient}
    (x : Q.Fiber q) :
    Q.quotientOf x.1 = q :=
  x.2

theorem fiber_nonempty_iff_exists
    (Q : QuotientPresentation Representative PublicQuotient)
    (q : PublicQuotient) :
    Nonempty (Q.Fiber q) <-> exists representative, Q.quotientOf representative = q := by
  constructor
  · intro h
    rcases h with ⟨x⟩
    exact ⟨x.1, x.2⟩
  · intro h
    rcases h with ⟨representative, hq⟩
    exact ⟨⟨representative, hq⟩⟩

end QuotientPresentation

/--
Abstract finite descent problem for the committed-sigma/account-carried path.

`representativeOpens` is the local verifier relation.  It should include byte
decoding, range checks, local equations, and quotient digest agreement.

`hiddenRepresents` and `accountBacked` split the semantic gluing claim: first a
hidden object explains the representative, then account evidence ties that
hidden object to prechallenge state.
-/
structure LocalQuotientDescent
    (Public Representative PublicQuotient Hidden AccountEvidence BreakEvent :
      Type*) where
  quotientOfRepresentative : Representative -> PublicQuotient
  quotientOfPublic : Public -> PublicQuotient
  representativeOpens : Public -> Representative -> Prop
  hiddenRepresents : Public -> Representative -> Hidden -> Prop
  accountBacked : Public -> Hidden -> AccountEvidence -> Prop
  breakFor : Public -> Representative -> BreakEvent -> Prop

namespace LocalQuotientDescent

variable {Public Representative PublicQuotient Hidden AccountEvidence
  BreakEvent : Type*}

variable
  (D :
    LocalQuotientDescent Public Representative PublicQuotient Hidden
      AccountEvidence BreakEvent)

/-- The representative lies in the fiber over the public quotient. -/
def InPublicFiber (pub : Public) (representative : Representative) : Prop :=
  D.quotientOfRepresentative representative = D.quotientOfPublic pub

/-- Local validity is fiber membership plus the concrete local verifier relation. -/
def LocallyValid (pub : Public) (representative : Representative) : Prop :=
  D.InPublicFiber pub representative ∧
    D.representativeOpens pub representative

/-- A locally valid representative glues to hidden state and account evidence. -/
def Glues (pub : Public) (representative : Representative) : Prop :=
  exists hidden evidence,
    D.hiddenRepresents pub representative hidden ∧
      D.accountBacked pub hidden evidence

/-- A locally valid representative triggers a named break event. -/
def HasBreak (pub : Public) (representative : Representative) : Prop :=
  exists breakEvent, D.breakFor pub representative breakEvent

/-- The public object has at least one locally valid representative. -/
def DescentData (pub : Public) : Prop :=
  exists representative, D.LocallyValid pub representative

/--
The existential quotient relation used by fiber-saturated witnesses.

This is the formal version of "there exists a representative in the public
fiber satisfying the representative-level predicate".
-/
def ExistentialQuotientRelation
    (representativeSemantic : Public -> Representative -> Prop)
    (pub : Public) : Prop :=
  exists representative,
    D.LocallyValid pub representative ∧
      representativeSemantic pub representative

theorem existentialQuotientRelation_intro
    {representativeSemantic : Public -> Representative -> Prop}
    {pub : Public} {representative : Representative}
    (hvalid : D.LocallyValid pub representative)
    (hsemantic : representativeSemantic pub representative) :
    D.ExistentialQuotientRelation representativeSemantic pub := by
  exact ⟨representative, hvalid, hsemantic⟩

/--
There are two distinct locally valid representatives in the same accepted
public fiber.
-/
def NontrivialAcceptedFiber (pub : Public) : Prop :=
  exists representative0 representative1,
    representative0 ≠ representative1 ∧
      D.LocallyValid pub representative0 ∧
        D.LocallyValid pub representative1

/--
If the local relation is singleton-binding in every public fiber, then it
cannot support nontrivial accepted fibers.

This captures the obstruction seen in the byte-commit-everything branch: an
injective commitment over all hidden bytes collapses the accepted fiber.
-/
theorem noNontrivialAcceptedFiber_of_subsingleton
    (hsubsingleton :
      forall {pub : Public} {representative0 representative1 : Representative},
        D.LocallyValid pub representative0 ->
          D.LocallyValid pub representative1 ->
            representative0 = representative1) :
    forall pub, ¬ D.NontrivialAcceptedFiber pub := by
  intro pub hfiber
  rcases hfiber with
    ⟨representative0, representative1, hneq, hvalid0, hvalid1⟩
  exact hneq (hsubsingleton hvalid0 hvalid1)

end LocalQuotientDescent

/--
Effective descent certificate for a finite quotient verifier.

Every locally valid representative must either glue to account-backed hidden
state or produce a named break.  This is the thin replacement for informal
"the local data descends" language.
-/
structure EffectiveDescentCertificate
    {Public Representative PublicQuotient Hidden AccountEvidence BreakEvent :
      Type*}
    (D :
      LocalQuotientDescent Public Representative PublicQuotient Hidden
        AccountEvidence BreakEvent) where
  descent_or_break :
    forall {pub : Public} {representative : Representative},
      D.LocallyValid pub representative ->
        D.Glues pub representative ∨ D.HasBreak pub representative

namespace EffectiveDescentCertificate

variable {Public Representative PublicQuotient Hidden AccountEvidence
  BreakEvent : Type*}

variable
  {D :
    LocalQuotientDescent Public Representative PublicQuotient Hidden
      AccountEvidence BreakEvent}

theorem locallyValid_glues_or_breaks
    (C : EffectiveDescentCertificate D)
    {pub : Public} {representative : Representative}
    (hvalid : D.LocallyValid pub representative) :
    D.Glues pub representative ∨ D.HasBreak pub representative :=
  C.descent_or_break hvalid

/--
Main wrapper theorem.

If verifier acceptance produces local descent data, then acceptance yields
either an account-backed hidden object or a named break event.
-/
theorem accepts_glues_or_breaks
    (C : EffectiveDescentCertificate D)
    {accepts : Public -> Prop}
    (accepts_yields_descent :
      forall {pub : Public}, accepts pub -> D.DescentData pub)
    {pub : Public}
    (haccepts : accepts pub) :
    (exists representative hidden evidence,
      D.LocallyValid pub representative ∧
        D.hiddenRepresents pub representative hidden ∧
          D.accountBacked pub hidden evidence) ∨
    (exists representative breakEvent,
      D.LocallyValid pub representative ∧
        D.breakFor pub representative breakEvent) := by
  rcases accepts_yields_descent haccepts with ⟨representative, hvalid⟩
  cases C.descent_or_break hvalid with
  | inl hglues =>
      rcases hglues with ⟨hidden, evidence, hrepresents, haccount⟩
      exact Or.inl
        ⟨representative, hidden, evidence, hvalid, hrepresents, haccount⟩
  | inr hbreak =>
      rcases hbreak with ⟨breakEvent, hbreak⟩
      exact Or.inr ⟨representative, breakEvent, hvalid, hbreak⟩

end EffectiveDescentCertificate

end DescentInterface
end Descender
