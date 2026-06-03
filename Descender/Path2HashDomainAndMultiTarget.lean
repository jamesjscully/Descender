import Descender.Path2HashBudget
import Descender.Path2NativeAccountDesign

namespace Descender
namespace Path2HashDomainAndMultiTarget

open Path2HashBudget

/-!
# Path 2 Hash Domain and Multi-Target Boundary

This module internalizes the two active hash-boundary obligations that are
left as explicit propositions by `Path2HashBudget`:

* checked-core and hidden-source frame domain separation;
* account-horizon multi-target pricing at the reviewed direct-query budget.

The pricing theorem is intentionally parameterized by an explicit horizon
record.  The active certificate below instantiates the selected 32-bit
aggregate-target horizon, matching the widened auxiliary-Fq12 direct budget's
deployment margin after QROM loss.
-/

/-- Concrete native hash-domain tag registry used by the active Path 2 route. -/
structure ActiveHashDomainRegistry where
  checkedCoreFrameTag : Nat
  hiddenSourceFrameTag : Nat
  projectionCarrierTag : Nat
  accountLifecycleTag : Nat
  checkedCoreFrameTag_pinned : checkedCoreFrameTag = 0
  hiddenSourceFrameTag_pinned : hiddenSourceFrameTag = 1
  projectionCarrierTag_pinned : projectionCarrierTag = 2
  accountLifecycleTag_pinned : accountLifecycleTag = 3
  checkedCore_ne_hiddenSource :
    checkedCoreFrameTag ≠ hiddenSourceFrameTag
  checkedCore_ne_projectionCarrier :
    checkedCoreFrameTag ≠ projectionCarrierTag
  checkedCore_ne_accountLifecycle :
    checkedCoreFrameTag ≠ accountLifecycleTag
  hiddenSource_ne_projectionCarrier :
    hiddenSourceFrameTag ≠ projectionCarrierTag
  hiddenSource_ne_accountLifecycle :
    hiddenSourceFrameTag ≠ accountLifecycleTag

/-- The active checked-core frame tag is disjoint from every other active tag. -/
def CheckedCoreFrameDomainSeparated
    (R : ActiveHashDomainRegistry) : Prop :=
  R.checkedCoreFrameTag ≠ R.hiddenSourceFrameTag ∧
    R.checkedCoreFrameTag ≠ R.projectionCarrierTag ∧
    R.checkedCoreFrameTag ≠ R.accountLifecycleTag

/-- The active hidden-source frame tag is disjoint from every other active tag. -/
def HiddenSourceFrameDomainSeparated
    (R : ActiveHashDomainRegistry) : Prop :=
  R.hiddenSourceFrameTag ≠ R.checkedCoreFrameTag ∧
    R.hiddenSourceFrameTag ≠ R.projectionCarrierTag ∧
    R.hiddenSourceFrameTag ≠ R.accountLifecycleTag

/--
Concrete byte-domain instantiation for the active hash registry.

The active security boundary consumes the small numeric tags above, while the
Rust implementation hashes concrete domain-prefixed byte frames. This bridge is
the theorem-facing contract that ties those byte domains to the active tags and
requires the concrete byte domains themselves to be disjoint.
-/
structure ActiveHashByteDomainInstantiation
    (R : ActiveHashDomainRegistry) where
  checkedCoreFrameDomainBytes : Nat
  hiddenSourceFrameDomainBytes : Nat
  projectionCarrierDomainBytes : Nat
  accountLifecycleDomainBytes : Nat
  tagOfDomainBytes : Nat -> Nat
  checkedCore_tag_matches :
    tagOfDomainBytes checkedCoreFrameDomainBytes = R.checkedCoreFrameTag
  hiddenSource_tag_matches :
    tagOfDomainBytes hiddenSourceFrameDomainBytes = R.hiddenSourceFrameTag
  projectionCarrier_tag_matches :
    tagOfDomainBytes projectionCarrierDomainBytes = R.projectionCarrierTag
  accountLifecycle_tag_matches :
    tagOfDomainBytes accountLifecycleDomainBytes = R.accountLifecycleTag
  checkedCore_bytes_ne_hiddenSource :
    checkedCoreFrameDomainBytes ≠ hiddenSourceFrameDomainBytes
  checkedCore_bytes_ne_projectionCarrier :
    checkedCoreFrameDomainBytes ≠ projectionCarrierDomainBytes
  checkedCore_bytes_ne_accountLifecycle :
    checkedCoreFrameDomainBytes ≠ accountLifecycleDomainBytes
  hiddenSource_bytes_ne_projectionCarrier :
    hiddenSourceFrameDomainBytes ≠ projectionCarrierDomainBytes
  hiddenSource_bytes_ne_accountLifecycle :
    hiddenSourceFrameDomainBytes ≠ accountLifecycleDomainBytes

/-- The concrete checked-core byte domain is disjoint from other active byte domains. -/
def CheckedCoreFrameByteDomainSeparated
    {R : ActiveHashDomainRegistry}
    (I : ActiveHashByteDomainInstantiation R) : Prop :=
  I.checkedCoreFrameDomainBytes ≠ I.hiddenSourceFrameDomainBytes ∧
    I.checkedCoreFrameDomainBytes ≠ I.projectionCarrierDomainBytes ∧
    I.checkedCoreFrameDomainBytes ≠ I.accountLifecycleDomainBytes

/-- The concrete hidden-source byte domain is disjoint from other active byte domains. -/
def HiddenSourceFrameByteDomainSeparated
    {R : ActiveHashDomainRegistry}
    (I : ActiveHashByteDomainInstantiation R) : Prop :=
  I.hiddenSourceFrameDomainBytes ≠ I.checkedCoreFrameDomainBytes ∧
    I.hiddenSourceFrameDomainBytes ≠ I.projectionCarrierDomainBytes ∧
    I.hiddenSourceFrameDomainBytes ≠ I.accountLifecycleDomainBytes

theorem checkedCoreFrameByteDomainSeparated_of_instantiation
    {R : ActiveHashDomainRegistry}
    (I : ActiveHashByteDomainInstantiation R) :
    CheckedCoreFrameByteDomainSeparated I :=
  ⟨I.checkedCore_bytes_ne_hiddenSource,
    I.checkedCore_bytes_ne_projectionCarrier,
    I.checkedCore_bytes_ne_accountLifecycle⟩

theorem hiddenSourceFrameByteDomainSeparated_of_instantiation
    {R : ActiveHashDomainRegistry}
    (I : ActiveHashByteDomainInstantiation R) :
    HiddenSourceFrameByteDomainSeparated I :=
  ⟨Ne.symm I.checkedCore_bytes_ne_hiddenSource,
    I.hiddenSource_bytes_ne_projectionCarrier,
    I.hiddenSource_bytes_ne_accountLifecycle⟩

/-- Pinned active-domain registry for the reviewed native active-lane route. -/
def activeHashDomainRegistryV1 : ActiveHashDomainRegistry where
  checkedCoreFrameTag := 0
  hiddenSourceFrameTag := 1
  projectionCarrierTag := 2
  accountLifecycleTag := 3
  checkedCoreFrameTag_pinned := rfl
  hiddenSourceFrameTag_pinned := rfl
  projectionCarrierTag_pinned := rfl
  accountLifecycleTag_pinned := rfl
  checkedCore_ne_hiddenSource := by native_decide
  checkedCore_ne_projectionCarrier := by native_decide
  checkedCore_ne_accountLifecycle := by native_decide
  hiddenSource_ne_projectionCarrier := by native_decide
  hiddenSource_ne_accountLifecycle := by native_decide

theorem active_checkedCoreFrameDomainSeparated :
    CheckedCoreFrameDomainSeparated activeHashDomainRegistryV1 :=
  ⟨activeHashDomainRegistryV1.checkedCore_ne_hiddenSource,
    activeHashDomainRegistryV1.checkedCore_ne_projectionCarrier,
    activeHashDomainRegistryV1.checkedCore_ne_accountLifecycle⟩

theorem active_hiddenSourceFrameDomainSeparated :
    HiddenSourceFrameDomainSeparated activeHashDomainRegistryV1 :=
  ⟨Ne.symm activeHashDomainRegistryV1.checkedCore_ne_hiddenSource,
    activeHashDomainRegistryV1.hiddenSource_ne_projectionCarrier,
    activeHashDomainRegistryV1.hiddenSource_ne_accountLifecycle⟩

/--
Explicit aggregate-target horizon for multi-target pricing.

The additive `log2` components make the account horizon and spend/message
surfaces visible to the theorem.  The aggregate cap can be stricter than the
sum; the active certificate below prices the selected 32-bit aggregate
deployment horizon.
-/
structure ActiveMultiTargetHorizon where
  accountHorizonLog2 : Nat
  spendHorizonLog2 : Nat
  messageHorizonLog2 : Nat
  aggregateTargetCountLog2 : Nat
  aggregate_covers_horizon :
    accountHorizonLog2 + spendHorizonLog2 + messageHorizonLog2 ≤
      aggregateTargetCountLog2

/--
The multi-target pricing predicate charged to the native hash boundary.

`transcriptTermBitsAfterQromLoss` is the already QROM-discounted direct-query
term.  Charging `aggregateTargetCountLog2` target bits is priced when the target
count fits inside the direct budget's deployment margin and the resulting term
still reaches the target security level.
-/
def MultiTargetHorizonLossPriced
    (B : Path2DirectQueryBudgetV1)
    (H : ActiveMultiTargetHorizon) : Prop :=
  H.accountHorizonLog2 + H.spendHorizonLog2 + H.messageHorizonLog2 ≤
      H.aggregateTargetCountLog2 ∧
    H.aggregateTargetCountLog2 ≤ B.minimumDeploymentMarginBits ∧
    B.targetSecurityBits + H.aggregateTargetCountLog2 ≤
      B.transcriptTermBitsAfterQromLoss

theorem multiTargetHorizonLossPriced_of_margin
    (B : Path2DirectQueryBudgetV1)
    (H : ActiveMultiTargetHorizon)
    (hmargin : H.aggregateTargetCountLog2 ≤ B.minimumDeploymentMarginBits) :
    MultiTargetHorizonLossPriced B H := by
  refine ⟨H.aggregate_covers_horizon, hmargin, ?_⟩
  have htarget :
      B.targetSecurityBits + H.aggregateTargetCountLog2 ≤
        B.targetSecurityBits + B.minimumDeploymentMarginBits :=
    Nat.add_le_add_left hmargin B.targetSecurityBits
  exact le_trans htarget (Nat.le_of_lt B.transcriptTermExceedsTargetPlusMargin)

/--
Active account-horizon cap for the direct native route.

The widened auxiliary-Fq12 challenge budget has 53 deployment-margin bits after
QROM loss.  This certificate spends 32 bits on the aggregate account horizon and
leaves spend and message horizons at the single-target active-lane value.
-/
def activeLaneAccountHorizonV1 : ActiveMultiTargetHorizon where
  accountHorizonLog2 := 32
  spendHorizonLog2 := 0
  messageHorizonLog2 := 0
  aggregateTargetCountLog2 := 32
  aggregate_covers_horizon := by native_decide

/--
Fifty-four-bit aggregate target horizon used as a fail-closed deployment-scale
negative control.

The current widened budget cannot price this horizon because the direct
QROM-discounted budget has 53 deployment-margin bits.
-/
def fiftyFourBitAggregateTargetHorizonV1 : ActiveMultiTargetHorizon where
  accountHorizonLog2 := 54
  spendHorizonLog2 := 0
  messageHorizonLog2 := 0
  aggregateTargetCountLog2 := 54
  aggregate_covers_horizon := by native_decide

theorem active_multiTargetLossPriced :
    MultiTargetHorizonLossPriced
      path2DirectQueryBudgetV1 activeLaneAccountHorizonV1 :=
  multiTargetHorizonLossPriced_of_margin
    path2DirectQueryBudgetV1 activeLaneAccountHorizonV1 (by native_decide)

/--
The current direct budget does not price a 54-bit aggregate target horizon.
Any deployment horizon broader than the selected widened-challenge cap requires
a parameter/query-budget redesign before deployment security can be claimed.
-/
theorem fiftyFourBitAggregateTargetHorizon_not_priced :
    ¬ MultiTargetHorizonLossPriced
      path2DirectQueryBudgetV1 fiftyFourBitAggregateTargetHorizonV1 := by
  intro h
  exact (by native_decide :
      ¬ fiftyFourBitAggregateTargetHorizonV1.aggregateTargetCountLog2 ≤
          path2DirectQueryBudgetV1.minimumDeploymentMarginBits) h.2.1

/--
Combined certificate for the three hash-boundary propositions not supplied by
the direct query-budget mirror.
-/
structure ActiveHashDomainAndMultiTargetCertificate where
  directBudget : Path2DirectQueryBudgetV1
  registry : ActiveHashDomainRegistry
  horizon : ActiveMultiTargetHorizon
  directQueryBudgetPriced :
    DirectQueryBudgetPriced directBudget
  checkedCoreFrameDomainSeparated :
    CheckedCoreFrameDomainSeparated registry
  hiddenSourceFrameDomainSeparated :
    HiddenSourceFrameDomainSeparated registry
  multiTargetLossPriced :
    MultiTargetHorizonLossPriced directBudget horizon

namespace ActiveHashDomainAndMultiTargetCertificate

/-- Convert the active domain/multi-target certificate into the native boundary. -/
def toNativeHashBudgetBoundary
    (C : ActiveHashDomainAndMultiTargetCertificate) :
    Path2NativeAccountDesign.NativeHashBudgetBoundary :=
  Path2HashBudget.toNativeHashBudgetBoundary C.directBudget
    (CheckedCoreFrameDomainSeparated C.registry)
    (HiddenSourceFrameDomainSeparated C.registry)
    C.horizon.aggregateTargetCountLog2
    (MultiTargetHorizonLossPriced C.directBudget C.horizon)

theorem native_checkedCoreFrameDomainSeparated
    (C : ActiveHashDomainAndMultiTargetCertificate) :
    C.toNativeHashBudgetBoundary.checkedCoreFrameDomainSeparated :=
  C.checkedCoreFrameDomainSeparated

theorem native_hiddenSourceFrameDomainSeparated
    (C : ActiveHashDomainAndMultiTargetCertificate) :
    C.toNativeHashBudgetBoundary.hiddenSourceFrameDomainSeparated :=
  C.hiddenSourceFrameDomainSeparated

theorem native_queryBudgetPriced
    (C : ActiveHashDomainAndMultiTargetCertificate) :
    C.toNativeHashBudgetBoundary.queryBudgetPriced :=
  C.directQueryBudgetPriced

theorem native_multiTargetLossPriced
    (C : ActiveHashDomainAndMultiTargetCertificate) :
    C.toNativeHashBudgetBoundary.multiTargetLossPriced :=
  C.multiTargetLossPriced

theorem native_hashBoundaryProps
    (C : ActiveHashDomainAndMultiTargetCertificate) :
    C.toNativeHashBudgetBoundary.checkedCoreFrameDomainSeparated ∧
      C.toNativeHashBudgetBoundary.hiddenSourceFrameDomainSeparated ∧
      C.toNativeHashBudgetBoundary.queryBudgetPriced ∧
      C.toNativeHashBudgetBoundary.multiTargetLossPriced :=
  ⟨C.native_checkedCoreFrameDomainSeparated,
    C.native_hiddenSourceFrameDomainSeparated,
    C.native_queryBudgetPriced,
    C.native_multiTargetLossPriced⟩

end ActiveHashDomainAndMultiTargetCertificate

/-- Active certificate for the reviewed Path 2 native active-lane route. -/
def activePath2HashDomainAndMultiTargetCertificate :
    ActiveHashDomainAndMultiTargetCertificate where
  directBudget := path2DirectQueryBudgetV1
  registry := activeHashDomainRegistryV1
  horizon := activeLaneAccountHorizonV1
  directQueryBudgetPriced :=
    path2_direct_query_budget_priced
  checkedCoreFrameDomainSeparated :=
    active_checkedCoreFrameDomainSeparated
  hiddenSourceFrameDomainSeparated :=
    active_hiddenSourceFrameDomainSeparated
  multiTargetLossPriced :=
    active_multiTargetLossPriced

/-- Native hash boundary with all active domain and multi-target props filled. -/
def activeNativeHashBudgetBoundary :
    Path2NativeAccountDesign.NativeHashBudgetBoundary :=
  activePath2HashDomainAndMultiTargetCertificate.toNativeHashBudgetBoundary

theorem activeNativeHashBudgetBoundary_checkedCoreFrameDomainSeparated :
    activeNativeHashBudgetBoundary.checkedCoreFrameDomainSeparated :=
  activePath2HashDomainAndMultiTargetCertificate
    |>.native_checkedCoreFrameDomainSeparated

theorem activeNativeHashBudgetBoundary_hiddenSourceFrameDomainSeparated :
    activeNativeHashBudgetBoundary.hiddenSourceFrameDomainSeparated :=
  activePath2HashDomainAndMultiTargetCertificate
    |>.native_hiddenSourceFrameDomainSeparated

theorem activeNativeHashBudgetBoundary_queryBudgetPriced :
    activeNativeHashBudgetBoundary.queryBudgetPriced :=
  activePath2HashDomainAndMultiTargetCertificate
    |>.native_queryBudgetPriced

theorem activeNativeHashBudgetBoundary_multiTargetLossPriced :
    activeNativeHashBudgetBoundary.multiTargetLossPriced :=
  activePath2HashDomainAndMultiTargetCertificate
    |>.native_multiTargetLossPriced

theorem activeNativeHashBudgetBoundary_hashBoundaryProps :
    activeNativeHashBudgetBoundary.checkedCoreFrameDomainSeparated ∧
      activeNativeHashBudgetBoundary.hiddenSourceFrameDomainSeparated ∧
      activeNativeHashBudgetBoundary.queryBudgetPriced ∧
      activeNativeHashBudgetBoundary.multiTargetLossPriced :=
  activePath2HashDomainAndMultiTargetCertificate
    |>.native_hashBoundaryProps

end Path2HashDomainAndMultiTarget
end Descender
