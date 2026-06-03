import Mathlib.Tactic
import Descender.InformationSetCommitment
import Descender.BlockChallengeSoundness

namespace Descender

/-!
Locally recoverable syndrome commitments.

This file records the constructive version of the fibered 69-byte direction.
Instead of hoping for a lucky `stateOfMask`, the signer is restricted to masks
whose local block syndromes all agree with one global state.  Any active block
then reconstructs the state by construction, while the local mask can remain
hidden inside the kernel of the local syndrome map.
-/

namespace LocallyRecoverableSyndrome

open BlockChallengeSoundness

variable {q : Nat} [NeZero q]

/--
Box-bounded q-ary kernel violation.

For bounded-mask collisions the useful promise is often coordinate-wise:
the delta between two accepted masks has every centered coordinate bounded by
`eta`, which can be much sharper than only bounding the total Lee norm.
-/
structure ModqBoxKernelViolation [Fintype ι]
    (q : Nat) [NeZero q] [Zero V]
    (AT : (ι -> ZMod q) -> V) (eta : Nat) where
  r : ι -> ZMod q
  kernel : AT r = 0
  bounded : centeredResidueNoiseSupport q eta r
  nonzero : r ≠ 0

def ModqNoBoxKernelVector [Fintype ι]
    (q : Nat) [NeZero q] [Zero V]
    (AT : (ι -> ZMod q) -> V) (eta : Nat) : Prop :=
  ¬ Nonempty (ModqBoxKernelViolation q AT eta)

namespace ModqBoxKernelViolation

theorem impossible_of_noBoxKernelVector [Fintype ι]
    (q : Nat) [NeZero q] [Zero V]
    {AT : (ι -> ZMod q) -> V} {eta : Nat}
    (hCode : ModqNoBoxKernelVector q AT eta)
    (W : ModqBoxKernelViolation q AT eta) :
    False :=
  hCode ⟨W⟩

end ModqBoxKernelViolation

abbrev ProductBlockMask (q : Nat) (Block LocalFree : Type*) :=
  Block × LocalFree -> ZMod q

abbrev LocalView (q : Nat) (LocalFree : Type*) :=
  LocalFree -> ZMod q

def blockView
    {Block LocalFree : Type*}
    (mask : ProductBlockMask q Block LocalFree)
    (b : Block) : LocalView q LocalFree :=
  fun f => mask (b, f)

/--
Generic active-mask-state interface.

Unlike `InformationSetCommitment.Architecture`, the active view here is carried
by the signature and is a view of the sampled mask, not a projection of an
already public commitment value.
-/
structure ActiveMaskStateArchitecture
    (Challenge ActiveView Mask State : Type*) where
  stateOfMask : Mask -> State
  activeView : Challenge -> Mask -> ActiveView
  reconstructState : Challenge -> ActiveView -> State

namespace ActiveMaskStateArchitecture

variable {Challenge ActiveView Mask State : Type*}

def ReconstructsOn
    (A : ActiveMaskStateArchitecture Challenge ActiveView Mask State)
    (ValidMask : Mask -> Prop) : Prop :=
  ∀ c mask, ValidMask mask ->
    A.reconstructState c (A.activeView c mask) = A.stateOfMask mask

end ActiveMaskStateArchitecture

/--
Abstract locally recoverable state: every block has a local decoder into the
same global state.
-/
structure LocallyRecoverableState
    (q : Nat) [NeZero q] (Block LocalFree State : Type*) where
  stateOfMask : ProductBlockMask q Block LocalFree -> State
  localState : Block -> LocalView q LocalFree -> State

namespace LocallyRecoverableState

variable {Block LocalFree State : Type*}

def Consistent
    (S : LocallyRecoverableState q Block LocalFree State)
    (mask : ProductBlockMask q Block LocalFree) : Prop :=
  ∀ b, S.localState b (blockView mask b) = S.stateOfMask mask

def toActiveMaskStateArchitecture
    (S : LocallyRecoverableState q Block LocalFree State) :
    ActiveMaskStateArchitecture Block (LocalView q LocalFree)
      (ProductBlockMask q Block LocalFree) State where
  stateOfMask := S.stateOfMask
  activeView := fun b mask => blockView mask b
  reconstructState := S.localState

theorem reconstructsOn_consistent
    (S : LocallyRecoverableState q Block LocalFree State) :
    S.toActiveMaskStateArchitecture.ReconstructsOn S.Consistent := by
  intro b mask hconsistent
  exact hconsistent b

def LocalStateHiding
    (S : LocallyRecoverableState q Block LocalFree State) : Prop :=
  ∀ b, ¬ Function.Injective (S.localState b)

end LocallyRecoverableState

/--
Concrete one-symbol local linear syndrome.

For each block, the local state is a linear form in the local free coordinates.
Valid masks are those for which all block-local syndromes equal the anchor
block's syndrome.
-/
structure LocalLinearSyndrome
    (q : Nat) [NeZero q] (Block LocalFree : Type*) where
  coeff : Block -> LocalFree -> ZMod q

namespace LocalLinearSyndrome

variable {Block LocalFree : Type*}
variable [Fintype Block] [Fintype LocalFree]

def localState
    (S : LocalLinearSyndrome q Block LocalFree)
    (b : Block) (view : LocalView q LocalFree) : ZMod q :=
  ∑ f, S.coeff b f * view f

def stateOfMask
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    (mask : ProductBlockMask q Block LocalFree) : ZMod q :=
  S.localState default (blockView mask default)

def Consistent
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    (mask : ProductBlockMask q Block LocalFree) : Prop :=
  ∀ b, S.localState b (blockView mask b) = S.stateOfMask mask

def toLocallyRecoverableState
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree) :
    LocallyRecoverableState q Block LocalFree (ZMod q) where
  stateOfMask := S.stateOfMask
  localState := S.localState

omit [Fintype Block] in
theorem consistent_eq_state
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {mask : ProductBlockMask q Block LocalFree}
    (hconsistent : S.Consistent mask) (b : Block) :
    S.localState b (blockView mask b) = S.stateOfMask mask :=
  hconsistent b

omit [Fintype Block] in
theorem active_reconstructs_state
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree) :
    S.toLocallyRecoverableState.toActiveMaskStateArchitecture.ReconstructsOn
      S.Consistent := by
  exact S.toLocallyRecoverableState.reconstructsOn_consistent

omit [Fintype Block] in
theorem localState_zero
    (S : LocalLinearSyndrome q Block LocalFree)
    (b : Block) :
    S.localState b 0 = 0 := by
  simp [localState]

omit [Fintype Block] in
theorem localState_sub
    (S : LocalLinearSyndrome q Block LocalFree)
    (b : Block) (x y : LocalView q LocalFree) :
    S.localState b (fun f => x f - y f) =
      S.localState b x - S.localState b y := by
  simp [localState, Finset.sum_sub_distrib, mul_sub]

def HasLocalKernel
    (S : LocalLinearSyndrome q Block LocalFree)
    (b : Block) : Prop :=
  ∃ view : LocalView q LocalFree, view ≠ 0 ∧ S.localState b view = 0

omit [Fintype Block] in
theorem localState_not_injective_of_hasLocalKernel
    (S : LocalLinearSyndrome q Block LocalFree)
    {b : Block}
    (hkernel : S.HasLocalKernel b) :
    ¬ Function.Injective (S.localState b) := by
  intro hinj
  rcases hkernel with ⟨view, hview_ne, hstate⟩
  apply hview_ne
  apply hinj
  rw [hstate, S.localState_zero b]

omit [Fintype Block] in
theorem localStateHiding_of_hasLocalKernel
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    (hkernel : ∀ b, S.HasLocalKernel b) :
    S.toLocallyRecoverableState.LocalStateHiding := by
  intro b
  exact S.localState_not_injective_of_hasLocalKernel (hkernel b)

/--
The q-ary collision map for locally recoverable syndromes.

A same-state collision between two consistent masks gives a nonzero vector in
the kernel of this map.  This is the map that needs a Lee-distance certificate,
not the one-symbol `stateOfMask` alone.
-/
def collisionMap
    (S : LocalLinearSyndrome q Block LocalFree)
    (delta : ProductBlockMask q Block LocalFree) : Block -> ZMod q :=
  fun b => S.localState b (blockView delta b)

def ValidMask
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    (eta : Nat)
    (mask : ProductBlockMask q Block LocalFree) : Prop :=
  BoundedMaskValid (q := q) (Coord := Block × LocalFree) eta mask ∧
    S.Consistent mask

theorem collisionMap_maskDelta_eq_zero
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {eta : Nat}
    (C :
      SameStateMaskCollision Block LocalFree (ZMod q) S.stateOfMask
        (S.ValidMask eta)) :
    S.collisionMap C.maskDelta = 0 := by
  funext b
  change S.localState b (blockView C.maskDelta b) = 0
  have h₀ :
      S.localState b (fun f => C.mask₀ (b, f)) = S.stateOfMask C.mask₀ := by
    simpa [blockView] using C.valid₀.2 b
  have h₁ :
      S.localState b (fun f => C.mask₁ (b, f)) = S.stateOfMask C.mask₁ := by
    simpa [blockView] using C.valid₁.2 b
  unfold SameStateMaskCollision.maskDelta blockView
  rw [S.localState_sub]
  rw [h₀, h₁, C.sameState]
  simp

def collisionToLeeViolation
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {eta leeMin : Nat}
    (C :
      SameStateMaskCollision Block LocalFree (ZMod q) S.stateOfMask
        (S.ValidMask eta))
    (hbound : Fintype.card (Block × LocalFree) * (eta + eta) < leeMin) :
    ModqLeeKernelViolation q S.collisionMap leeMin where
  r := C.maskDelta
  kernel := S.collisionMap_maskDelta_eq_zero C
  nonzero := C.maskDelta_nonzero
  short := by
    exact lt_of_le_of_lt
      (centeredNormL1_sub_le_card_mul_add_of_noiseSupport
        (q := q) (eta := eta) (rho := eta)
        (x := C.mask₀) (y := C.mask₁) C.valid₀.1 C.valid₁.1)
      hbound

/--
Certificate closing the collision branch for a locally recoverable linear
syndrome construction.
-/
structure CollisionCertificate
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree) where
  eta : Nat
  leeMin : Nat
  shortCollisionBound :
    Fintype.card (Block × LocalFree) * (eta + eta) < leeMin
  noLeeKernel :
    ModqNoLeeKernelVectorBelow q S.collisionMap leeMin

namespace CollisionCertificate

variable [Inhabited Block]
variable {S : LocalLinearSyndrome q Block LocalFree}

theorem no_sameStateMaskCollision
    (Cert : S.CollisionCertificate) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree (ZMod q) S.stateOfMask
        (S.ValidMask Cert.eta)) := by
  rintro ⟨C⟩
  exact ModqLeeKernelViolation.impossible_of_noLeeKernelVectorBelow
    q Cert.noLeeKernel
    (S.collisionToLeeViolation C Cert.shortCollisionBound)

end CollisionCertificate

def collisionToBoxKernelViolation
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {eta : Nat}
    (C :
      SameStateMaskCollision Block LocalFree (ZMod q) S.stateOfMask
        (S.ValidMask eta)) :
    ModqBoxKernelViolation q S.collisionMap (eta + eta) where
  r := C.maskDelta
  kernel := S.collisionMap_maskDelta_eq_zero C
  bounded := by
    intro bf
    unfold SameStateMaskCollision.maskDelta
    exact Nat.le_trans (centeredAbs_sub_le q (C.mask₀ bf) (C.mask₁ bf))
      (Nat.add_le_add (C.valid₀.1 bf) (C.valid₁.1 bf))
  nonzero := C.maskDelta_nonzero

/--
Coordinate-wise certificate closing the locally recoverable collision branch.

This is the right target for accepted-mask deltas: if every accepted mask has
coordinates bounded by `eta`, every fork delta is bounded coordinate-wise by
`eta + eta`.
-/
structure BoxCollisionCertificate
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree) where
  eta : Nat
  noBoxKernel :
    ModqNoBoxKernelVector q S.collisionMap (eta + eta)

namespace BoxCollisionCertificate

variable [Inhabited Block]
variable {S : LocalLinearSyndrome q Block LocalFree}

theorem no_sameStateMaskCollision
    (Cert : S.BoxCollisionCertificate) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree (ZMod q) S.stateOfMask
        (S.ValidMask Cert.eta)) := by
  rintro ⟨C⟩
  exact ModqBoxKernelViolation.impossible_of_noBoxKernelVector
    q Cert.noBoxKernel (S.collisionToBoxKernelViolation C)

theorem stateOfMask_injectiveOn_validMask
    (Cert : S.BoxCollisionCertificate)
    {mask₀ mask₁ : ProductBlockMask q Block LocalFree}
    (hvalid₀ : S.ValidMask Cert.eta mask₀)
    (hvalid₁ : S.ValidMask Cert.eta mask₁)
    (hsame : S.stateOfMask mask₀ = S.stateOfMask mask₁) :
    mask₀ = mask₁ := by
  by_contra hdifferent
  exact Cert.no_sameStateMaskCollision
    ⟨{
      mask₀ := mask₀
      mask₁ := mask₁
      valid₀ := hvalid₀
      valid₁ := hvalid₁
      sameState := hsame
      different := hdifferent
    }⟩

end BoxCollisionCertificate

/--
Collision in the public commitment map.

This is the branch that should be discharged by the concrete commitment or hash
assumption in a deployable construction.
-/
structure CommitmentCollision {HiddenState PublicState : Type*}
    (commit : HiddenState -> PublicState) where
  hidden₀ : HiddenState
  hidden₁ : HiddenState
  different : hidden₀ ≠ hidden₁
  sameCommit : commit hidden₀ = commit hidden₁

def NoCommitmentCollision {HiddenState PublicState : Type*}
    (commit : HiddenState -> PublicState) : Prop :=
  ¬ Nonempty (CommitmentCollision commit)

/--
Commitment-domain version of a locally recoverable state.

The active view reconstructs a hidden commitment-domain state, then hashes or
commits it into the public state used by Fiat-Shamir.  Public-state collisions
therefore split into hidden-state collisions or commitment collisions.
-/
structure CommittedLocallyRecoverableState
    (q : Nat) [NeZero q] (Block LocalFree HiddenState PublicState : Type*) where
  hiddenStateOfMask : ProductBlockMask q Block LocalFree -> HiddenState
  localHiddenState : Block -> LocalView q LocalFree -> HiddenState
  commit : HiddenState -> PublicState

namespace CommittedLocallyRecoverableState

variable {HiddenState PublicState : Type*}

def publicStateOfMask
    (C :
      CommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState)
    (mask : ProductBlockMask q Block LocalFree) : PublicState :=
  C.commit (C.hiddenStateOfMask mask)

def Consistent
    (C :
      CommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState)
    (mask : ProductBlockMask q Block LocalFree) : Prop :=
  ∀ b, C.localHiddenState b (blockView mask b) = C.hiddenStateOfMask mask

def toActiveMaskStateArchitecture
    (C :
      CommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState) :
    ActiveMaskStateArchitecture Block (LocalView q LocalFree)
      (ProductBlockMask q Block LocalFree) PublicState where
  stateOfMask := C.publicStateOfMask
  activeView := fun b mask => blockView mask b
  reconstructState := fun b view => C.commit (C.localHiddenState b view)

omit [Fintype Block] [Fintype LocalFree] in
theorem public_reconstructsOn_consistent
    (C :
      CommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState) :
    C.toActiveMaskStateArchitecture.ReconstructsOn C.Consistent := by
  intro b mask hconsistent
  simp [toActiveMaskStateArchitecture, publicStateOfMask, hconsistent b]

noncomputable def hiddenCollisionOfPublicCollisionOrCommitmentCollision
    (C :
      CommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState)
    {ValidMask : ProductBlockMask q Block LocalFree -> Prop}
    (W :
      SameStateMaskCollision Block LocalFree PublicState C.publicStateOfMask
        ValidMask) :
    Sum
      (SameStateMaskCollision Block LocalFree HiddenState C.hiddenStateOfMask
        ValidMask)
      (CommitmentCollision C.commit) := by
  classical
  by_cases hhidden :
      C.hiddenStateOfMask W.mask₀ = C.hiddenStateOfMask W.mask₁
  · exact Sum.inl {
      mask₀ := W.mask₀
      mask₁ := W.mask₁
      valid₀ := W.valid₀
      valid₁ := W.valid₁
      sameState := hhidden
      different := W.different
    }
  · exact Sum.inr {
      hidden₀ := C.hiddenStateOfMask W.mask₀
      hidden₁ := C.hiddenStateOfMask W.mask₁
      different := hhidden
      sameCommit := by
        simpa [publicStateOfMask] using W.sameState
    }

omit [Fintype Block] [Fintype LocalFree] in
theorem no_publicCollision_of_no_hiddenCollision_noCommitmentCollision
    (C :
      CommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState)
    {ValidMask : ProductBlockMask q Block LocalFree -> Prop}
    (hNoHidden :
      ¬ Nonempty
        (SameStateMaskCollision Block LocalFree HiddenState C.hiddenStateOfMask
          ValidMask))
    (hNoCommit : NoCommitmentCollision C.commit) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree PublicState C.publicStateOfMask
        ValidMask) := by
  rintro ⟨W⟩
  cases C.hiddenCollisionOfPublicCollisionOrCommitmentCollision W with
  | inl H => exact hNoHidden ⟨H⟩
  | inr K => exact hNoCommit ⟨K⟩

end CommittedLocallyRecoverableState

def toCommittedState
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {PublicState : Type*}
    (commit : ZMod q -> PublicState) :
    CommittedLocallyRecoverableState q Block LocalFree (ZMod q) PublicState where
  hiddenStateOfMask := S.stateOfMask
  localHiddenState := S.localState
  commit := commit

omit [Fintype Block] in
theorem committed_public_reconstructs_state
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {PublicState : Type*}
    (commit : ZMod q -> PublicState) :
    (S.toCommittedState commit).toActiveMaskStateArchitecture.ReconstructsOn
      (S.toCommittedState commit).Consistent := by
  exact (S.toCommittedState commit).public_reconstructsOn_consistent

theorem no_publicStateCollision_of_boxCertificate_noCommitmentCollision
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {PublicState : Type*}
    (commit : ZMod q -> PublicState)
    (Cert : S.BoxCollisionCertificate)
    (hNoCommit : NoCommitmentCollision commit) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree PublicState
        (S.toCommittedState commit).publicStateOfMask
        (S.ValidMask Cert.eta)) :=
  (S.toCommittedState commit).no_publicCollision_of_no_hiddenCollision_noCommitmentCollision
    Cert.no_sameStateMaskCollision hNoCommit

/--
Public-only opened commitment-domain version.

Unlike `OpenedCommittedLocallyRecoverableState`, this interface does not require
the active opening to reconstruct the hidden preimage.  It reconstructs only the
public state.  This is the proof surface for trapdoor or ZK-style openings.
-/
structure PublicOpenedCommittedLocallyRecoverableState
    (q : Nat) [NeZero q]
    (Block LocalFree HiddenState PublicState Opening : Type*) where
  hiddenStateOfMask : ProductBlockMask q Block LocalFree -> HiddenState
  activeOpening : Block -> ProductBlockMask q Block LocalFree -> Opening
  openPublicState : Block -> LocalView q LocalFree -> Opening -> PublicState
  commit : HiddenState -> PublicState

namespace PublicOpenedCommittedLocallyRecoverableState

variable {HiddenState PublicState Opening : Type*}

def publicStateOfMask
    (C :
      PublicOpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    (mask : ProductBlockMask q Block LocalFree) : PublicState :=
  C.commit (C.hiddenStateOfMask mask)

def Consistent
    (C :
      PublicOpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    (mask : ProductBlockMask q Block LocalFree) : Prop :=
  ∀ b,
    C.openPublicState b (blockView mask b) (C.activeOpening b mask) =
      C.publicStateOfMask mask

def toActiveMaskStateArchitecture
    (C :
      PublicOpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening) :
    ActiveMaskStateArchitecture Block (LocalView q LocalFree × Opening)
      (ProductBlockMask q Block LocalFree) PublicState where
  stateOfMask := C.publicStateOfMask
  activeView := fun b mask => (blockView mask b, C.activeOpening b mask)
  reconstructState := fun b viewAndOpening =>
    C.openPublicState b viewAndOpening.1 viewAndOpening.2

omit [Fintype Block] [Fintype LocalFree] in
theorem public_reconstructsOn_consistent
    (C :
      PublicOpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening) :
    C.toActiveMaskStateArchitecture.ReconstructsOn C.Consistent := by
  intro b mask hconsistent
  simp [toActiveMaskStateArchitecture, hconsistent b]

noncomputable def hiddenCollisionOfPublicCollisionOrCommitmentCollision
    (C :
      PublicOpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    {ValidMask : ProductBlockMask q Block LocalFree -> Prop}
    (W :
      SameStateMaskCollision Block LocalFree PublicState C.publicStateOfMask
        ValidMask) :
    Sum
      (SameStateMaskCollision Block LocalFree HiddenState C.hiddenStateOfMask
        ValidMask)
      (CommitmentCollision C.commit) := by
  classical
  by_cases hhidden :
      C.hiddenStateOfMask W.mask₀ = C.hiddenStateOfMask W.mask₁
  · exact Sum.inl {
      mask₀ := W.mask₀
      mask₁ := W.mask₁
      valid₀ := W.valid₀
      valid₁ := W.valid₁
      sameState := hhidden
      different := W.different
    }
  · exact Sum.inr {
      hidden₀ := C.hiddenStateOfMask W.mask₀
      hidden₁ := C.hiddenStateOfMask W.mask₁
      different := hhidden
      sameCommit := by
        simpa [publicStateOfMask] using W.sameState
    }

omit [Fintype Block] [Fintype LocalFree] in
theorem no_publicCollision_of_no_hiddenCollision_noCommitmentCollision
    (C :
      PublicOpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    {ValidMask : ProductBlockMask q Block LocalFree -> Prop}
    (hNoHidden :
      ¬ Nonempty
        (SameStateMaskCollision Block LocalFree HiddenState C.hiddenStateOfMask
          ValidMask))
    (hNoCommit : NoCommitmentCollision C.commit) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree PublicState C.publicStateOfMask
        ValidMask) := by
  rintro ⟨W⟩
  cases C.hiddenCollisionOfPublicCollisionOrCommitmentCollision W with
  | inl H => exact hNoHidden ⟨H⟩
  | inr K => exact hNoCommit ⟨K⟩

/--
Information-theoretic placeholder for "the public opening does not reveal the
hidden preimage".  A deployable ZK/trapdoor instantiation should replace this
with a computational zero-knowledge or preimage-hiding statement.
-/
def NoPerfectHiddenExtractor
    (C :
      PublicOpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    (ValidMask : ProductBlockMask q Block LocalFree -> Prop) : Prop :=
  ∀ extract : Block -> LocalView q LocalFree -> Opening -> HiddenState,
    ∃ b mask,
      ValidMask mask ∧ C.Consistent mask ∧
        extract b (blockView mask b) (C.activeOpening b mask) ≠
          C.hiddenStateOfMask mask

end PublicOpenedCommittedLocallyRecoverableState

def toPublicOpenedCommittedState
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Opening PublicState : Type*}
    (activeOpening : Block -> ProductBlockMask q Block LocalFree -> Opening)
    (openPublicState : Block -> LocalView q LocalFree -> Opening -> PublicState)
    (commit : ZMod q -> PublicState) :
    PublicOpenedCommittedLocallyRecoverableState
      q Block LocalFree (ZMod q) PublicState Opening where
  hiddenStateOfMask := S.stateOfMask
  activeOpening := activeOpening
  openPublicState := openPublicState
  commit := commit

omit [Fintype Block] in
theorem publicOpened_public_reconstructs_state
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Opening PublicState : Type*}
    (activeOpening : Block -> ProductBlockMask q Block LocalFree -> Opening)
    (openPublicState : Block -> LocalView q LocalFree -> Opening -> PublicState)
    (commit : ZMod q -> PublicState) :
    ActiveMaskStateArchitecture.ReconstructsOn
      (PublicOpenedCommittedLocallyRecoverableState.toActiveMaskStateArchitecture
        (S.toPublicOpenedCommittedState activeOpening openPublicState commit))
      (PublicOpenedCommittedLocallyRecoverableState.Consistent
        (S.toPublicOpenedCommittedState activeOpening openPublicState commit)) := by
  exact PublicOpenedCommittedLocallyRecoverableState.public_reconstructsOn_consistent
    (S.toPublicOpenedCommittedState activeOpening openPublicState commit)

theorem no_publicOpenedStateCollision_of_boxCertificate_noCommitmentCollision
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Opening PublicState : Type*}
    (activeOpening : Block -> ProductBlockMask q Block LocalFree -> Opening)
    (openPublicState : Block -> LocalView q LocalFree -> Opening -> PublicState)
    (commit : ZMod q -> PublicState)
    (Cert : S.BoxCollisionCertificate)
    (hNoCommit : NoCommitmentCollision commit) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree PublicState
        (S.toPublicOpenedCommittedState activeOpening openPublicState commit).publicStateOfMask
        (S.ValidMask Cert.eta)) :=
  (S.toPublicOpenedCommittedState activeOpening openPublicState commit)
    |>.no_publicCollision_of_no_hiddenCollision_noCommitmentCollision
        Cert.no_sameStateMaskCollision hNoCommit

/--
Opened commitment-domain version.

The active transcript may carry explicit opening data in addition to the active
block view.  This is the right abstraction for salted or trapdoor preimage
commitments: the opening bytes are charged separately, and public-state binding
still splits into hidden-state binding plus commitment binding.
-/
structure OpenedCommittedLocallyRecoverableState
    (q : Nat) [NeZero q]
    (Block LocalFree HiddenState PublicState Opening : Type*) where
  hiddenStateOfMask : ProductBlockMask q Block LocalFree -> HiddenState
  localHiddenState : Block -> LocalView q LocalFree -> Opening -> HiddenState
  activeOpening : Block -> ProductBlockMask q Block LocalFree -> Opening
  commit : HiddenState -> PublicState

namespace OpenedCommittedLocallyRecoverableState

variable {HiddenState PublicState Opening : Type*}

def publicStateOfMask
    (C :
      OpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    (mask : ProductBlockMask q Block LocalFree) : PublicState :=
  C.commit (C.hiddenStateOfMask mask)

def Consistent
    (C :
      OpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    (mask : ProductBlockMask q Block LocalFree) : Prop :=
  ∀ b,
    C.localHiddenState b (blockView mask b) (C.activeOpening b mask) =
      C.hiddenStateOfMask mask

def toActiveMaskStateArchitecture
    (C :
      OpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening) :
    ActiveMaskStateArchitecture Block (LocalView q LocalFree × Opening)
      (ProductBlockMask q Block LocalFree) PublicState where
  stateOfMask := C.publicStateOfMask
  activeView := fun b mask => (blockView mask b, C.activeOpening b mask)
  reconstructState := fun b viewAndOpening =>
    C.commit (C.localHiddenState b viewAndOpening.1 viewAndOpening.2)

omit [Fintype Block] [Fintype LocalFree] in
theorem public_reconstructsOn_consistent
    (C :
      OpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening) :
    C.toActiveMaskStateArchitecture.ReconstructsOn C.Consistent := by
  intro b mask hconsistent
  simp [toActiveMaskStateArchitecture, publicStateOfMask, hconsistent b]

noncomputable def hiddenCollisionOfPublicCollisionOrCommitmentCollision
    (C :
      OpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    {ValidMask : ProductBlockMask q Block LocalFree -> Prop}
    (W :
      SameStateMaskCollision Block LocalFree PublicState C.publicStateOfMask
        ValidMask) :
    Sum
      (SameStateMaskCollision Block LocalFree HiddenState C.hiddenStateOfMask
        ValidMask)
      (CommitmentCollision C.commit) := by
  classical
  by_cases hhidden :
      C.hiddenStateOfMask W.mask₀ = C.hiddenStateOfMask W.mask₁
  · exact Sum.inl {
      mask₀ := W.mask₀
      mask₁ := W.mask₁
      valid₀ := W.valid₀
      valid₁ := W.valid₁
      sameState := hhidden
      different := W.different
    }
  · exact Sum.inr {
      hidden₀ := C.hiddenStateOfMask W.mask₀
      hidden₁ := C.hiddenStateOfMask W.mask₁
      different := hhidden
      sameCommit := by
        simpa [publicStateOfMask] using W.sameState
    }

omit [Fintype Block] [Fintype LocalFree] in
theorem no_publicCollision_of_no_hiddenCollision_noCommitmentCollision
    (C :
      OpenedCommittedLocallyRecoverableState
        q Block LocalFree HiddenState PublicState Opening)
    {ValidMask : ProductBlockMask q Block LocalFree -> Prop}
    (hNoHidden :
      ¬ Nonempty
        (SameStateMaskCollision Block LocalFree HiddenState C.hiddenStateOfMask
          ValidMask))
    (hNoCommit : NoCommitmentCollision C.commit) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree PublicState C.publicStateOfMask
        ValidMask) := by
  rintro ⟨W⟩
  cases C.hiddenCollisionOfPublicCollisionOrCommitmentCollision W with
  | inl H => exact hNoHidden ⟨H⟩
  | inr K => exact hNoCommit ⟨K⟩

end OpenedCommittedLocallyRecoverableState

def toSaltedOpenedCommittedState
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Salt PublicState : Type*}
    (saltOfMask : ProductBlockMask q Block LocalFree -> Salt)
    (commit : ZMod q × Salt -> PublicState) :
    OpenedCommittedLocallyRecoverableState
      q Block LocalFree (ZMod q × Salt) PublicState Salt where
  hiddenStateOfMask := fun mask => (S.stateOfMask mask, saltOfMask mask)
  localHiddenState := fun b view salt => (S.localState b view, salt)
  activeOpening := fun _b mask => saltOfMask mask
  commit := commit

omit [Fintype Block] in
theorem saltedOpened_consistent_iff
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Salt PublicState : Type*}
    (saltOfMask : ProductBlockMask q Block LocalFree -> Salt)
    (commit : ZMod q × Salt -> PublicState)
    (mask : ProductBlockMask q Block LocalFree) :
    (S.toSaltedOpenedCommittedState saltOfMask commit).Consistent mask ↔
      S.Consistent mask := by
  constructor
  · intro hconsistent b
    have h := hconsistent b
    exact congrArg Prod.fst h
  · intro hconsistent b
    simp [toSaltedOpenedCommittedState, hconsistent b]

omit [Fintype Block] in
theorem saltedOpened_public_reconstructs_state
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Salt PublicState : Type*}
    (saltOfMask : ProductBlockMask q Block LocalFree -> Salt)
    (commit : ZMod q × Salt -> PublicState) :
    (S.toSaltedOpenedCommittedState saltOfMask commit).toActiveMaskStateArchitecture.ReconstructsOn
      (S.toSaltedOpenedCommittedState saltOfMask commit).Consistent := by
  exact (S.toSaltedOpenedCommittedState saltOfMask commit)
    |>.public_reconstructsOn_consistent

theorem no_saltedHiddenCollision_of_boxCertificate
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Salt PublicState : Type*}
    (saltOfMask : ProductBlockMask q Block LocalFree -> Salt)
    (commit : ZMod q × Salt -> PublicState)
    (Cert : S.BoxCollisionCertificate) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree (ZMod q × Salt)
        (S.toSaltedOpenedCommittedState saltOfMask commit).hiddenStateOfMask
        (S.ValidMask Cert.eta)) := by
  rintro ⟨W⟩
  have hsameState :
      S.stateOfMask W.mask₀ = S.stateOfMask W.mask₁ :=
    congrArg Prod.fst W.sameState
  exact Cert.no_sameStateMaskCollision ⟨{
    mask₀ := W.mask₀
    mask₁ := W.mask₁
    valid₀ := W.valid₀
    valid₁ := W.valid₁
    sameState := hsameState
    different := W.different
  }⟩

theorem no_saltedPublicStateCollision_of_boxCertificate_noCommitmentCollision
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Salt PublicState : Type*}
    (saltOfMask : ProductBlockMask q Block LocalFree -> Salt)
    (commit : ZMod q × Salt -> PublicState)
    (Cert : S.BoxCollisionCertificate)
    (hNoCommit : NoCommitmentCollision commit) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree PublicState
        (S.toSaltedOpenedCommittedState saltOfMask commit).publicStateOfMask
        (S.ValidMask Cert.eta)) :=
  (S.toSaltedOpenedCommittedState saltOfMask commit)
    |>.no_publicCollision_of_no_hiddenCollision_noCommitmentCollision
        (S.no_saltedHiddenCollision_of_boxCertificate saltOfMask commit Cert)
        hNoCommit

theorem saltedHiddenState_injectiveOn_validMask_of_boxCertificate
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    {Salt PublicState : Type*}
    (saltOfMask : ProductBlockMask q Block LocalFree -> Salt)
    (commit : ZMod q × Salt -> PublicState)
    (Cert : S.BoxCollisionCertificate)
    {mask₀ mask₁ : ProductBlockMask q Block LocalFree}
    (hvalid₀ : S.ValidMask Cert.eta mask₀)
    (hvalid₁ : S.ValidMask Cert.eta mask₁)
    (hsame :
      (S.toSaltedOpenedCommittedState saltOfMask commit).hiddenStateOfMask mask₀ =
        (S.toSaltedOpenedCommittedState saltOfMask commit).hiddenStateOfMask mask₁) :
    mask₀ = mask₁ :=
  Cert.stateOfMask_injectiveOn_validMask hvalid₀ hvalid₁
    (congrArg Prod.fst hsame)

/--
Non-injective hidden domains are incompatible with the no-collision box
certificate.  This records the negative result for the "large fibers by design"
route under the current collision-closing proof.
-/
def ValidStateNonInjective
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree) (eta : Nat) : Prop :=
  ∃ mask₀ mask₁ : ProductBlockMask q Block LocalFree,
    S.ValidMask eta mask₀ ∧ S.ValidMask eta mask₁ ∧
      S.stateOfMask mask₀ = S.stateOfMask mask₁ ∧ mask₀ ≠ mask₁

theorem no_validStateNonInjective_of_boxCertificate
    [Inhabited Block]
    (S : LocalLinearSyndrome q Block LocalFree)
    (Cert : S.BoxCollisionCertificate) :
    ¬ S.ValidStateNonInjective Cert.eta := by
  rintro ⟨mask₀, mask₁, hvalid₀, hvalid₁, hsame, hdifferent⟩
  exact Cert.no_sameStateMaskCollision ⟨{
    mask₀ := mask₀
    mask₁ := mask₁
    valid₀ := hvalid₀
    valid₁ := hvalid₁
    sameState := hsame
    different := hdifferent
  }⟩

end LocalLinearSyndrome

/--
Opening data that makes same-state fiber deltas recoverable.

This is the condition under which intentionally non-injective hidden domains can
still be useful: the fork proof does not need to rule out same-state mask
collisions if the active openings let the extractor recover the exact mask
delta and use corrected extraction.
-/
structure FiberDeltaOpening
    {q : Nat} [NeZero q]
    (Block LocalFree State Opening : Type*)
    (stateOfMask : ProductBlockMask q Block LocalFree -> State) where
  openingOfMask : ProductBlockMask q Block LocalFree -> Opening
  deltaFromOpenings :
    Opening -> Opening -> ProductBlockMask q Block LocalFree
  delta_eq_of_sameState :
    ∀ {mask₀ mask₁ : ProductBlockMask q Block LocalFree},
      stateOfMask mask₀ = stateOfMask mask₁ ->
        deltaFromOpenings (openingOfMask mask₀) (openingOfMask mask₁) =
          fun bf => mask₀ bf - mask₁ bf

namespace FiberDeltaOpening

variable {State Opening : Type*}
variable {Block LocalFree : Type*}
variable {stateOfMask : ProductBlockMask q Block LocalFree -> State}
variable {ValidMask : ProductBlockMask q Block LocalFree -> Prop}
variable {secret : Block × LocalFree -> ZMod q}

noncomputable def correctedExtractCoordinate
    (F : FiberDeltaOpening Block LocalFree State Opening stateOfMask)
    (R :
      FiberedForkedLinearRound Block LocalFree State stateOfMask
        ValidMask secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    ZMod q :=
  Classical.choose hdelta *
    (R.z₀ (b, f) - R.z₁ (b, f) -
      F.deltaFromOpenings (F.openingOfMask R.mask₀)
        (F.openingOfMask R.mask₁) (b, f))

theorem correctedExtractCoordinate_eq_secret
    (F : FiberDeltaOpening Block LocalFree State Opening stateOfMask)
    (R :
      FiberedForkedLinearRound Block LocalFree State stateOfMask
        ValidMask secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    F.correctedExtractCoordinate R b hdelta f = secret (b, f) := by
  unfold correctedExtractCoordinate
  rw [F.delta_eq_of_sameState R.sameState]
  rw [R.hz₀, R.hz₁]
  exact extract_coordinate_of_deltaExtractable_with_mask_delta
    (q := q) (bf := (b, f))
    (inv := Classical.choose hdelta) (Classical.choose_spec hdelta)

end FiberDeltaOpening

/--
Local opening data that makes same-state fiber deltas recoverable.

This is the byte-oriented variant of `FiberDeltaOpening`: an active verifier
does not need a global opening, only enough data for the challenged block.
-/
structure LocalFiberDeltaOpening
    {q : Nat} [NeZero q]
    (Block LocalFree State Opening : Type*)
    (stateOfMask : ProductBlockMask q Block LocalFree -> State) where
  openingOfMask : Block -> ProductBlockMask q Block LocalFree -> Opening
  deltaFromOpenings :
    Block -> Opening -> Opening -> LocalFree -> ZMod q
  delta_eq_of_sameState :
    ∀ {mask₀ mask₁ : ProductBlockMask q Block LocalFree},
      stateOfMask mask₀ = stateOfMask mask₁ ->
        ∀ b f,
          deltaFromOpenings b (openingOfMask b mask₀) (openingOfMask b mask₁) f =
            mask₀ (b, f) - mask₁ (b, f)

namespace LocalFiberDeltaOpening

variable {State Opening : Type*}
variable {Block LocalFree : Type*}
variable {stateOfMask : ProductBlockMask q Block LocalFree -> State}
variable {ValidMask : ProductBlockMask q Block LocalFree -> Prop}
variable {secret : Block × LocalFree -> ZMod q}

noncomputable def correctedExtractCoordinate
    (F : LocalFiberDeltaOpening Block LocalFree State Opening stateOfMask)
    (R :
      FiberedForkedLinearRound Block LocalFree State stateOfMask
        ValidMask secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    ZMod q :=
  Classical.choose hdelta *
    (R.z₀ (b, f) - R.z₁ (b, f) -
      F.deltaFromOpenings b (F.openingOfMask b R.mask₀)
        (F.openingOfMask b R.mask₁) f)

theorem correctedExtractCoordinate_eq_secret
    (F : LocalFiberDeltaOpening Block LocalFree State Opening stateOfMask)
    (R :
      FiberedForkedLinearRound Block LocalFree State stateOfMask
        ValidMask secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    F.correctedExtractCoordinate R b hdelta f = secret (b, f) := by
  unfold correctedExtractCoordinate
  rw [F.delta_eq_of_sameState R.sameState b f]
  rw [R.hz₀, R.hz₁]
  exact extract_coordinate_of_deltaExtractable_with_mask_delta
    (q := q) (bf := (b, f))
    (inv := Classical.choose hdelta) (Classical.choose_spec hdelta)

/--
One-time-padded local active opening.

A single opening can be information-theoretically hidden by sampling the pad
uniformly; two openings for the same fiber cancel the shared pad and reveal only
the local mask delta needed by corrected extraction.
-/
def additiveMaskedLocalOpening
    (view pad : LocalView q LocalFree) : LocalView q LocalFree :=
  fun f => view f + pad f

def padTranslateForSameOpening
    (view₀ view₁ pad₀ : LocalView q LocalFree) : LocalView q LocalFree :=
  fun f => pad₀ f + view₀ f - view₁ f

omit [NeZero q] in
theorem additiveMaskedLocalOpening_translate_eq
    (view₀ view₁ pad₀ : LocalView q LocalFree) :
    additiveMaskedLocalOpening view₀ pad₀ =
      additiveMaskedLocalOpening view₁
        (padTranslateForSameOpening view₀ view₁ pad₀) := by
  funext f
  simp [additiveMaskedLocalOpening, padTranslateForSameOpening]
  ring

def padTranslateEquiv
    (view₀ view₁ : LocalView q LocalFree) :
    LocalView q LocalFree ≃ LocalView q LocalFree where
  toFun := padTranslateForSameOpening view₀ view₁
  invFun := padTranslateForSameOpening view₁ view₀
  left_inv := by
    intro pad
    funext f
    simp [padTranslateForSameOpening]
  right_inv := by
    intro pad
    funext f
    simp [padTranslateForSameOpening]

/--
Distribution-free hiding interface: for any two local views, pad space is in
bijection so that the produced opening is identical.

With a uniform pad distribution, this bijection is the usual one-time-pad proof
that the single opening carries no information about the view.
-/
def PerfectLocalOpeningHiding
    {Pad Opening : Type*}
    (makeOpening : LocalView q LocalFree -> Pad -> Opening) : Prop :=
  ∀ view₀ view₁,
    ∃ translate : Pad ≃ Pad,
      ∀ pad, makeOpening view₀ pad = makeOpening view₁ (translate pad)

omit [NeZero q] in
theorem additiveMaskedLocalOpening_perfectHiding :
    PerfectLocalOpeningHiding
      (fun (view : LocalView q LocalFree) pad =>
        additiveMaskedLocalOpening view pad) := by
  intro view₀ view₁
  refine ⟨padTranslateEquiv view₀ view₁, ?_⟩
  intro pad
  exact additiveMaskedLocalOpening_translate_eq view₀ view₁ pad

/--
Concrete local fiber-delta construction from a per-state one-time pad.

The `State` should be read as the fixed public fiber/commitment instance.  The
same fiber uses the same pad, so two same-state openings subtract to the local
mask delta.  A single opening remains one-time-padded when the pad is sampled
from a high-entropy distribution and kept out of the transcript.
-/
def additiveOneTimePad
    (stateOfMask : ProductBlockMask q Block LocalFree -> State)
    (padOfState : State -> Block -> LocalView q LocalFree) :
    LocalFiberDeltaOpening Block LocalFree State (LocalView q LocalFree)
      stateOfMask where
  openingOfMask := fun b mask =>
    additiveMaskedLocalOpening (blockView mask b) (padOfState (stateOfMask mask) b)
  deltaFromOpenings := fun _b opening₀ opening₁ f => opening₀ f - opening₁ f
  delta_eq_of_sameState := by
    intro mask₀ mask₁ hsame b f
    simp [additiveMaskedLocalOpening, blockView, hsame]

theorem hidden_eq_of_noCommitmentCollision
    {HiddenState PublicState : Type*}
    {commit : HiddenState -> PublicState}
    (hNoCommit : LocalLinearSyndrome.NoCommitmentCollision commit)
    {hidden₀ hidden₁ : HiddenState}
    (hsame : commit hidden₀ = commit hidden₁) :
    hidden₀ = hidden₁ := by
  by_contra hdifferent
  exact hNoCommit ⟨{
    hidden₀ := hidden₀
    hidden₁ := hidden₁
    different := hdifferent
    sameCommit := hsame
  }⟩

/--
Committed one-time-padded local opening.

The pad is bound to the hidden committed state, not derived from the public
state.  A single transcript therefore need not reveal the pad.  Across a fork,
public commitment equality plus commitment binding forces the same hidden state
and hence the same pad, so the active openings still subtract to the mask
delta.
-/
def additiveCommittedOneTimePad
    {HiddenState PublicState : Type*}
    (hiddenStateOfMask : ProductBlockMask q Block LocalFree -> HiddenState)
    (commit : HiddenState -> PublicState)
    (padOfHidden : HiddenState -> Block -> LocalView q LocalFree)
    (hNoCommit : LocalLinearSyndrome.NoCommitmentCollision commit) :
    LocalFiberDeltaOpening Block LocalFree PublicState (LocalView q LocalFree)
      (fun mask => commit (hiddenStateOfMask mask)) where
  openingOfMask := fun b mask =>
    additiveMaskedLocalOpening (blockView mask b)
      (padOfHidden (hiddenStateOfMask mask) b)
  deltaFromOpenings := fun _b opening₀ opening₁ f => opening₀ f - opening₁ f
  delta_eq_of_sameState := by
    intro mask₀ mask₁ hsame b f
    have hhidden :
        hiddenStateOfMask mask₀ = hiddenStateOfMask mask₁ :=
      hidden_eq_of_noCommitmentCollision hNoCommit hsame
    simp [additiveMaskedLocalOpening, blockView, hhidden]

/--
Abstract hash/digest binding for the prechallenge public fiber state.

For a real hash this is the deterministic side of the reduction: equal digest
without equal state is the hash/commitment collision branch.
-/
structure StateDigestBinding (State Digest : Type*) where
  digestOfState : State -> Digest
  state_eq_of_digest_eq :
    ∀ {state₀ state₁ : State},
      digestOfState state₀ = digestOfState state₁ -> state₀ = state₁

def AcceptsMask
    {Digest : Type*}
    (B : StateDigestBinding State Digest)
    (F : LocalFiberDeltaOpening Block LocalFree State Opening stateOfMask)
    (mask : ProductBlockMask q Block LocalFree)
    (b : Block) (opening : Opening) (digest : Digest) : Prop :=
  B.digestOfState (stateOfMask mask) = digest ∧
    opening = F.openingOfMask b mask

theorem sameState_of_accepts_sameDigest
    {Digest : Type*}
    {B : StateDigestBinding State Digest}
    {F : LocalFiberDeltaOpening Block LocalFree State Opening stateOfMask}
    {mask₀ mask₁ : ProductBlockMask q Block LocalFree}
    {b₀ b₁ : Block} {opening₀ opening₁ : Opening} {digest : Digest}
    (h₀ : AcceptsMask B F mask₀ b₀ opening₀ digest)
    (h₁ : AcceptsMask B F mask₁ b₁ opening₁ digest) :
    stateOfMask mask₀ = stateOfMask mask₁ := by
  apply B.state_eq_of_digest_eq
  rw [h₀.1, h₁.1]

theorem deltaFromOpenings_eq_of_accepts_sameDigest
    {Digest : Type*}
    {B : StateDigestBinding State Digest}
    {F : LocalFiberDeltaOpening Block LocalFree State Opening stateOfMask}
    {mask₀ mask₁ : ProductBlockMask q Block LocalFree}
    {b : Block} {opening₀ opening₁ : Opening} {digest : Digest}
    (h₀ : AcceptsMask B F mask₀ b opening₀ digest)
    (h₁ : AcceptsMask B F mask₁ b opening₁ digest)
    (f : LocalFree) :
    F.deltaFromOpenings b opening₀ opening₁ f =
      mask₀ (b, f) - mask₁ (b, f) := by
  have hsame :
      stateOfMask mask₀ = stateOfMask mask₁ :=
    sameState_of_accepts_sameDigest h₀ h₁
  rw [h₀.2, h₁.2]
  exact F.delta_eq_of_sameState hsame b f

/--
Forked linear round whose same-fiber condition comes from transcript binding.

Both accepting transcripts use the same prechallenge state digest.  Each local
opening is checked against that digest and the challenged block.  The digest
binding theorem turns this into same public state, and the local opening
relation turns same public state into a mask-delta correction.
-/
structure HashBoundLocalDeltaForkedRound
    {Digest : Type*}
    (B : StateDigestBinding State Digest)
    (F : LocalFiberDeltaOpening Block LocalFree State Opening stateOfMask)
    (ValidMask : ProductBlockMask q Block LocalFree -> Prop)
    (secret : Block × LocalFree -> ZMod q) where
  mask₀ : ProductBlockMask q Block LocalFree
  mask₁ : ProductBlockMask q Block LocalFree
  valid₀ : ValidMask mask₀
  valid₁ : ValidMask mask₁
  digest : Digest
  opening₀ : Block -> Opening
  opening₁ : Block -> Opening
  accepts₀ : ∀ b, AcceptsMask B F mask₀ b (opening₀ b) digest
  accepts₁ : ∀ b, AcceptsMask B F mask₁ b (opening₁ b) digest
  chi₀ : Block -> ZMod q
  chi₁ : Block -> ZMod q
  z₀ : Block × LocalFree -> ZMod q
  z₁ : Block × LocalFree -> ZMod q
  hz₀ : z₀ = SignatureDistribution.productBlockLinearResponse chi₀ mask₀ secret
  hz₁ : z₁ = SignatureDistribution.productBlockLinearResponse chi₁ mask₁ secret

namespace HashBoundLocalDeltaForkedRound

variable {Digest : Type*}
variable {B : StateDigestBinding State Digest}
variable {F : LocalFiberDeltaOpening Block LocalFree State Opening stateOfMask}

def DeltaActive
    (R : HashBoundLocalDeltaForkedRound B F ValidMask secret)
    (b : Block) : Prop :=
  DeltaExtractable R.chi₀ R.chi₁ b

theorem sameState
    (R : HashBoundLocalDeltaForkedRound B F ValidMask secret)
    (b₀ b₁ : Block) :
    stateOfMask R.mask₀ = stateOfMask R.mask₁ :=
  sameState_of_accepts_sameDigest (R.accepts₀ b₀) (R.accepts₁ b₁)

theorem deltaFromOpenings_eq
    (R : HashBoundLocalDeltaForkedRound B F ValidMask secret)
    (b : Block) (f : LocalFree) :
    F.deltaFromOpenings b (R.opening₀ b) (R.opening₁ b) f =
      R.mask₀ (b, f) - R.mask₁ (b, f) :=
  deltaFromOpenings_eq_of_accepts_sameDigest
    (R.accepts₀ b) (R.accepts₁ b) f

noncomputable def correctedExtractCoordinate
    (R : HashBoundLocalDeltaForkedRound B F ValidMask secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    ZMod q :=
  Classical.choose hdelta *
    (R.z₀ (b, f) - R.z₁ (b, f) -
      F.deltaFromOpenings b (R.opening₀ b) (R.opening₁ b) f)

theorem correctedExtractCoordinate_eq_secret
    (R : HashBoundLocalDeltaForkedRound B F ValidMask secret)
    (b : Block) (hdelta : R.DeltaActive b) (f : LocalFree) :
    R.correctedExtractCoordinate b hdelta f = secret (b, f) := by
  unfold correctedExtractCoordinate
  rw [R.deltaFromOpenings_eq b f]
  rw [R.hz₀, R.hz₁]
  exact extract_coordinate_of_deltaExtractable_with_mask_delta
    (q := q) (bf := (b, f))
    (inv := Classical.choose hdelta) (Classical.choose_spec hdelta)

end HashBoundLocalDeltaForkedRound

end LocalFiberDeltaOpening

namespace LocalLinearSyndrome

variable {Block LocalFree : Type*}
variable [Fintype Block] [Fintype LocalFree]

/-- Byte accounting for compact active transcripts with per-repetition openings. -/
structure ActiveOpeningByteProfile where
  challengeBytes : Nat
  responseBytes : Nat
  repetitions : Nat
  openingBytesPerRepetition : Nat

namespace ActiveOpeningByteProfile

def openingBytes (P : ActiveOpeningByteProfile) : Nat :=
  P.repetitions * P.openingBytesPerRepetition

def signatureBytes (P : ActiveOpeningByteProfile) : Nat :=
  P.challengeBytes + P.responseBytes + P.openingBytes

theorem signatureBytes_eq
    (P : ActiveOpeningByteProfile) :
    P.signatureBytes =
      P.challengeBytes + P.responseBytes +
        P.repetitions * P.openingBytesPerRepetition :=
  rfl

theorem signatureBytes_mono_opening
    {P Q : ActiveOpeningByteProfile}
    (hchallenge : P.challengeBytes = Q.challengeBytes)
    (hresponse : P.responseBytes = Q.responseBytes)
    (hrepetitions : P.repetitions = Q.repetitions)
    (hopening : P.openingBytesPerRepetition <= Q.openingBytesPerRepetition) :
    P.signatureBytes <= Q.signatureBytes := by
  simp [signatureBytes, openingBytes, hchallenge, hresponse, hrepetitions]
  exact Nat.mul_le_mul_left Q.repetitions hopening

end ActiveOpeningByteProfile

end LocalLinearSyndrome

namespace N32Q12289LocalSyndrome

abbrev modulus : Nat := 12289
abbrev eta : Nat := 12
abbrev deltaBound : Nat := eta + eta
abbrev slope : Nat := 50

abbrev Block := Fin 16
abbrev LocalFree := Fin 2

def coeff (_b : Block) : LocalFree -> ZMod modulus
  | ⟨0, _⟩ => 1
  | ⟨1, _⟩ => slope

def syndrome : LocalLinearSyndrome modulus Block LocalFree where
  coeff := coeff

def freeOfCenteredPair (x y : Int) : LocalView modulus LocalFree
  | ⟨0, _⟩ => (x : ZMod modulus)
  | ⟨1, _⟩ => (y : ZMod modulus)

def localKernelWitness : LocalView modulus LocalFree :=
  freeOfCenteredPair (-(slope : Int)) 1

theorem hasLocalKernel (b : Block) :
    syndrome.HasLocalKernel b := by
  refine ⟨localKernelWitness, ?_, ?_⟩
  · intro hzero
    have hcoord := congrFun hzero (⟨1, by decide⟩ : LocalFree)
    norm_num [localKernelWitness, freeOfCenteredPair, modulus] at hcoord
    have hneq : (1 : ZMod modulus) ≠ 0 := by
      native_decide
    exact hneq hcoord
  · norm_num [localKernelWitness, freeOfCenteredPair, syndrome, coeff,
      LocalLinearSyndrome.localState, Fin.sum_univ_two, modulus, slope]

theorem localStateHiding :
    syndrome.toLocallyRecoverableState.LocalStateHiding :=
  syndrome.localStateHiding_of_hasLocalKernel hasLocalKernel

abbrev localBoxWidth : Nat := 2 * deltaBound + 1

def centeredDeltaIndex (i : Fin localBoxWidth) : Int :=
  (i.1 : Int) - (deltaBound : Int)

def localEquationOfCenteredPair (x y : Int) : ZMod modulus :=
  (x : ZMod modulus) + (slope : ZMod modulus) * (y : ZMod modulus)

def localBoxKernelPasses (x y : Int) : Bool :=
  if x = 0 ∧ y = 0 then
    true
  else
    decide (localEquationOfCenteredPair x y ≠ 0)

def localBoxKernelCheck : Bool :=
  decide
    (∀ ix : Fin localBoxWidth, ∀ iy : Fin localBoxWidth,
      localBoxKernelPasses
        (centeredDeltaIndex ix) (centeredDeltaIndex iy) = true)

set_option maxHeartbeats 2000000 in
theorem localBoxKernelCheck_eq_true :
    localBoxKernelCheck = true := by
  native_decide

theorem centeredDeltaIndex_surjective_of_bounds
    {x : Int}
    (hlo : -(deltaBound : Int) <= x)
    (hhi : x <= (deltaBound : Int)) :
    ∃ ix : Fin localBoxWidth, centeredDeltaIndex ix = x := by
  change ∃ ix : Fin 49, (ix.1 : Int) - 24 = x
  have hlo' : -24 <= x := by
    simpa [deltaBound, eta] using hlo
  have hhi' : x <= 24 := by
    simpa [deltaBound, eta] using hhi
  let n : Nat := (x + 24).toNat
  have hnlt : n < 49 := by
    dsimp [n]
    have hnonneg : 0 <= x + 24 := by omega
    have hto : ((x + 24).toNat : Int) = x + 24 :=
      Int.toNat_of_nonneg hnonneg
    omega
  refine ⟨⟨n, hnlt⟩, ?_⟩
  dsimp [n]
  have hnonneg : 0 <= x + 24 := by omega
  have hto : ((x + 24).toNat : Int) = x + 24 :=
    Int.toNat_of_nonneg hnonneg
  omega

theorem localBoxKernelPasses_of_bounds
    {x y : Int}
    (hloX : -(deltaBound : Int) <= x)
    (hhiX : x <= (deltaBound : Int))
    (hloY : -(deltaBound : Int) <= y)
    (hhiY : y <= (deltaBound : Int)) :
    localBoxKernelPasses x y = true := by
  rcases centeredDeltaIndex_surjective_of_bounds
      (x := x) hloX hhiX with ⟨ix, rfl⟩
  rcases centeredDeltaIndex_surjective_of_bounds
      (x := y) hloY hhiY with ⟨iy, rfl⟩
  exact (of_decide_eq_true localBoxKernelCheck_eq_true ix iy)

theorem localBoxKernelPasses_sound
    {x y : Int}
    (hpass : localBoxKernelPasses x y = true)
    (hkernel : localEquationOfCenteredPair x y = 0) :
    x = 0 ∧ y = 0 := by
  unfold localBoxKernelPasses at hpass
  split_ifs at hpass with hzero
  · exact hzero
  · have hne : localEquationOfCenteredPair x y ≠ 0 :=
      of_decide_eq_true hpass
    exact False.elim (hne hkernel)

theorem localEquationOfCenteredPair_eq_localState
    (b : Block) (x y : Int) :
    syndrome.localState b (freeOfCenteredPair x y) =
      localEquationOfCenteredPair x y := by
  norm_num [localEquationOfCenteredPair, freeOfCenteredPair, syndrome, coeff,
    LocalLinearSyndrome.localState, Fin.sum_univ_two, modulus, slope]

theorem localBoxKernel_free_zero_of_bounded
    (b : Block) (view : LocalView modulus LocalFree)
    (hkernel : syndrome.localState b view = 0)
    (hbounded : centeredResidueNoiseSupport modulus deltaBound view) :
    view = 0 := by
  let x : Int := centeredLift modulus (view (⟨0, by decide⟩ : LocalFree))
  let y : Int := centeredLift modulus (view (⟨1, by decide⟩ : LocalFree))
  have hxabs : x.natAbs <= deltaBound := by
    simpa [x, centeredAbs, centeredLift] using
      hbounded (⟨0, by decide⟩ : LocalFree)
  have hyabs : y.natAbs <= deltaBound := by
    simpa [y, centeredAbs, centeredLift] using
      hbounded (⟨1, by decide⟩ : LocalFree)
  have hxbounds :
      -(deltaBound : Int) <= x ∧ x <= deltaBound := by
    have hxInt : (x.natAbs : Int) <= (deltaBound : Int) := by
      exact_mod_cast hxabs
    rw [← Int.abs_eq_natAbs] at hxInt
    exact abs_le.mp hxInt
  have hybounds :
      -(deltaBound : Int) <= y ∧ y <= deltaBound := by
    have hyInt : (y.natAbs : Int) <= (deltaBound : Int) := by
      exact_mod_cast hyabs
    rw [← Int.abs_eq_natAbs] at hyInt
    exact abs_le.mp hyInt
  have hview : freeOfCenteredPair x y = view := by
    funext f
    fin_cases f <;>
      simp [freeOfCenteredPair, x, y, centeredLift, ZMod.coe_valMinAbs]
  have hkernelPair : localEquationOfCenteredPair x y = 0 := by
    rw [← localEquationOfCenteredPair_eq_localState b x y]
    simpa [hview] using hkernel
  have hpass :=
    localBoxKernelPasses_of_bounds
      hxbounds.1 hxbounds.2 hybounds.1 hybounds.2
  have hxyzero := localBoxKernelPasses_sound hpass hkernelPair
  rw [← hview]
  funext f
  fin_cases f <;> simp [freeOfCenteredPair, hxyzero.1, hxyzero.2]

theorem noBoxKernel :
    ModqNoBoxKernelVector modulus syndrome.collisionMap deltaBound := by
  rintro ⟨r, hkernel, hbounded, hnonzero⟩
  apply hnonzero
  funext bf
  rcases bf with ⟨b, f⟩
  have hblockKernel : syndrome.localState b (blockView r b) = 0 := by
    have h := congrFun hkernel b
    simpa [LocalLinearSyndrome.collisionMap] using h
  have hblockBounded :
      centeredResidueNoiseSupport modulus deltaBound (blockView r b) := by
    intro f
    exact hbounded (b, f)
  have hblockZero :=
    localBoxKernel_free_zero_of_bounded b (blockView r b)
      hblockKernel hblockBounded
  have hcoord := congrFun hblockZero f
  simpa [blockView] using hcoord

def boxCollisionCertificate :
    syndrome.BoxCollisionCertificate where
  eta := eta
  noBoxKernel := by
    simpa [deltaBound, eta] using noBoxKernel

theorem no_sameStateMaskCollision :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree (ZMod modulus) syndrome.stateOfMask
        (syndrome.ValidMask eta)) :=
  boxCollisionCertificate.no_sameStateMaskCollision

theorem stateOfMask_injectiveOn_validMask
    {mask₀ mask₁ : ProductBlockMask modulus Block LocalFree}
    (hvalid₀ : syndrome.ValidMask eta mask₀)
    (hvalid₁ : syndrome.ValidMask eta mask₁)
    (hsame : syndrome.stateOfMask mask₀ = syndrome.stateOfMask mask₁) :
    mask₀ = mask₁ :=
  boxCollisionCertificate.stateOfMask_injectiveOn_validMask
    hvalid₀ hvalid₁ hsame

theorem no_publicStateCollision_of_noCommitmentCollision
    {PublicState : Type*}
    (commit : ZMod modulus -> PublicState)
    (hNoCommit : LocalLinearSyndrome.NoCommitmentCollision commit) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree PublicState
        (syndrome.toCommittedState commit).publicStateOfMask
        (syndrome.ValidMask eta)) :=
  syndrome.no_publicStateCollision_of_boxCertificate_noCommitmentCollision
    commit boxCollisionCertificate hNoCommit

theorem no_publicOpenedStateCollision_of_noCommitmentCollision
    {Opening PublicState : Type*}
    (activeOpening :
      Block -> ProductBlockMask modulus Block LocalFree -> Opening)
    (openPublicState :
      Block -> LocalView modulus LocalFree -> Opening -> PublicState)
    (commit : ZMod modulus -> PublicState)
    (hNoCommit : LocalLinearSyndrome.NoCommitmentCollision commit) :
    ¬ Nonempty
      (SameStateMaskCollision Block LocalFree PublicState
        (LocalLinearSyndrome.PublicOpenedCommittedLocallyRecoverableState.publicStateOfMask
          (syndrome.toPublicOpenedCommittedState activeOpening openPublicState commit))
        (syndrome.ValidMask eta)) :=
  syndrome.no_publicOpenedStateCollision_of_boxCertificate_noCommitmentCollision
    activeOpening openPublicState commit boxCollisionCertificate hNoCommit

theorem no_validStateNonInjective :
    ¬ syndrome.ValidStateNonInjective eta :=
  syndrome.no_validStateNonInjective_of_boxCertificate boxCollisionCertificate

abbrev compactChallengeBytes : Nat := 17
abbrev compactResponseBytes : Nat := 52
abbrev compactRepetitions : Nat := 26

def saltedOpeningByteProfile
    (openingBytesPerRepetition : Nat) :
    LocalLinearSyndrome.ActiveOpeningByteProfile where
  challengeBytes := compactChallengeBytes
  responseBytes := compactResponseBytes
  repetitions := compactRepetitions
  openingBytesPerRepetition := openingBytesPerRepetition

theorem saltedOpening_signatureBytes_eq
    (openingBytesPerRepetition : Nat) :
    (saltedOpeningByteProfile openingBytesPerRepetition).signatureBytes =
      69 + 26 * openingBytesPerRepetition := by
  simp [saltedOpeningByteProfile,
    LocalLinearSyndrome.ActiveOpeningByteProfile.signatureBytes,
    LocalLinearSyndrome.ActiveOpeningByteProfile.openingBytes,
    compactChallengeBytes, compactResponseBytes, compactRepetitions]

def salted128OpeningByteProfile :
    LocalLinearSyndrome.ActiveOpeningByteProfile :=
  saltedOpeningByteProfile 16

theorem salted128Opening_signatureBytes_eq :
    salted128OpeningByteProfile.signatureBytes = 485 := by
  native_decide

def salted256OpeningByteProfile :
    LocalLinearSyndrome.ActiveOpeningByteProfile :=
  saltedOpeningByteProfile 32

theorem salted256Opening_signatureBytes_eq :
    salted256OpeningByteProfile.signatureBytes = 901 := by
  native_decide

def localDeltaOpeningByteProfile :
    LocalLinearSyndrome.ActiveOpeningByteProfile :=
  saltedOpeningByteProfile 4

theorem localDeltaOpening_signatureBytes_eq :
    localDeltaOpeningByteProfile.signatureBytes = 173 := by
  native_decide

theorem saltedHiddenState_injectiveOn_validMask
    {Salt PublicState : Type*}
    (saltOfMask : ProductBlockMask modulus Block LocalFree -> Salt)
    (commit : ZMod modulus × Salt -> PublicState)
    {mask₀ mask₁ : ProductBlockMask modulus Block LocalFree}
    (hvalid₀ : syndrome.ValidMask eta mask₀)
    (hvalid₁ : syndrome.ValidMask eta mask₁)
    (hsame :
      (syndrome.toSaltedOpenedCommittedState saltOfMask commit).hiddenStateOfMask mask₀ =
        (syndrome.toSaltedOpenedCommittedState saltOfMask commit).hiddenStateOfMask mask₁) :
    mask₀ = mask₁ :=
  syndrome.saltedHiddenState_injectiveOn_validMask_of_boxCertificate
    saltOfMask commit boxCollisionCertificate hvalid₀ hvalid₁ hsame

end N32Q12289LocalSyndrome

end LocallyRecoverableSyndrome

end Descender
