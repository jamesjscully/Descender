import Descender.Path2NativeAccountDesign

namespace Descender
namespace Path2HashBudget

/-!
# Path 2 Hash/QROM Budget Mirror

This file records the executable direct-mode hash/QROM budget used by the Rust
`path2_direct_query_budget_v1` and `path2_native_hash_qrom_review_v1` reference
objects.  It intentionally keeps the theorem surface narrow: the numeric
deployment margin is proved here, while domain separation and multi-target
pricing remain the surrounding active-path obligations.
-/

/-- Lean mirror of the direct-mode query-budget constants. -/
structure Path2DirectQueryBudgetV1 where
  targetSecurityBits : Nat
  deploymentQuantumHashQueryBoundLog2 : Nat
  deploymentClassicalHashQueryBoundLog2 : Nat
  qromQuadraticLossBitsAtDeploymentBound : Nat
  transcriptTermBitsAfterQromLoss : Nat
  minimumDeploymentMarginBits : Nat
  qBatchActive : Bool
  qBatchInactive : qBatchActive = false
  transcriptTermExceedsTargetPlusMargin :
    transcriptTermBitsAfterQromLoss >
      targetSecurityBits + minimumDeploymentMarginBits

/--
The active direct-mode budget pinned by `path2_direct_query_budget_v1`.

Direct mode does not activate `q_batch`; optional aggregation remains outside
this budget until a batch theorem is enabled.
-/
def path2DirectQueryBudgetV1 : Path2DirectQueryBudgetV1 where
  targetSecurityBits := 128
  deploymentQuantumHashQueryBoundLog2 := 16
  deploymentClassicalHashQueryBoundLog2 := 33
  qromQuadraticLossBitsAtDeploymentBound := 32
  transcriptTermBitsAfterQromLoss := 182
  minimumDeploymentMarginBits := 53
  qBatchActive := false
  qBatchInactive := rfl
  transcriptTermExceedsTargetPlusMargin := by native_decide

/-- Lean mirror of the native public hash/QROM review caps. -/
structure Path2NativeHashQromReviewV1 where
  digestBits : Nat
  directBudget : Path2DirectQueryBudgetV1
  targetSecurityBits : Nat
  qromQuadraticLossBitsAtDeploymentBound : Nat
  transcriptTermBitsAfterQromLoss : Nat
  minimumDeploymentMarginBits : Nat
  importsDirectBudgetCaps :
    targetSecurityBits = directBudget.targetSecurityBits ∧
      qromQuadraticLossBitsAtDeploymentBound =
        directBudget.qromQuadraticLossBitsAtDeploymentBound ∧
      transcriptTermBitsAfterQromLoss =
        directBudget.transcriptTermBitsAfterQromLoss ∧
      minimumDeploymentMarginBits =
        directBudget.minimumDeploymentMarginBits

/-- Native review object matching `path2_native_hash_qrom_review_v1`. -/
def path2NativeHashQromReviewV1 : Path2NativeHashQromReviewV1 where
  digestBits := 256
  directBudget := path2DirectQueryBudgetV1
  targetSecurityBits := path2DirectQueryBudgetV1.targetSecurityBits
  qromQuadraticLossBitsAtDeploymentBound :=
    path2DirectQueryBudgetV1.qromQuadraticLossBitsAtDeploymentBound
  transcriptTermBitsAfterQromLoss :=
    path2DirectQueryBudgetV1.transcriptTermBitsAfterQromLoss
  minimumDeploymentMarginBits :=
    path2DirectQueryBudgetV1.minimumDeploymentMarginBits
  importsDirectBudgetCaps := by
    exact ⟨rfl, rfl, rfl, rfl⟩

/-- Query-budget pricing fact supplied by the executable direct-mode object. -/
def DirectQueryBudgetPriced (B : Path2DirectQueryBudgetV1) : Prop :=
  B.qBatchActive = false ∧
    B.transcriptTermBitsAfterQromLoss >
      B.targetSecurityBits + B.minimumDeploymentMarginBits

theorem path2_direct_query_budget_keeps_q_batch_inactive :
    path2DirectQueryBudgetV1.qBatchActive = false :=
  path2DirectQueryBudgetV1.qBatchInactive

theorem path2_direct_query_budget_transcript_exceeds_target_plus_margin :
    path2DirectQueryBudgetV1.transcriptTermBitsAfterQromLoss >
      path2DirectQueryBudgetV1.targetSecurityBits +
        path2DirectQueryBudgetV1.minimumDeploymentMarginBits :=
  path2DirectQueryBudgetV1.transcriptTermExceedsTargetPlusMargin

theorem path2_direct_query_budget_priced :
    DirectQueryBudgetPriced path2DirectQueryBudgetV1 :=
  ⟨path2_direct_query_budget_keeps_q_batch_inactive,
    path2_direct_query_budget_transcript_exceeds_target_plus_margin⟩

/--
Adapter from the executable direct-mode budget into the active native hash
boundary.  The direct object fills the query-budget pricing field; checked-core
domain separation, hidden-source domain separation, and multi-target pricing
remain explicit external propositions.
-/
def toNativeHashBudgetBoundary
    (B : Path2DirectQueryBudgetV1)
    (checkedCoreFrameDomainSeparated hiddenSourceFrameDomainSeparated : Prop)
    (multiTargetBudget : Nat)
    (multiTargetLossPriced : Prop) :
    Path2NativeAccountDesign.NativeHashBudgetBoundary where
  model := Path2NativeAccountDesign.NativeHashAssumptionModel.quantumRandomOracle
  checkedCoreFrameDomainSeparated := checkedCoreFrameDomainSeparated
  hiddenSourceFrameDomainSeparated := hiddenSourceFrameDomainSeparated
  queryBudget := B.transcriptTermBitsAfterQromLoss
  multiTargetBudget := multiTargetBudget
  queryBudgetPriced := DirectQueryBudgetPriced B
  multiTargetLossPriced := multiTargetLossPriced

theorem active_direct_budget_native_boundary_query_priced
    (checkedCoreFrameDomainSeparated hiddenSourceFrameDomainSeparated : Prop)
    (multiTargetBudget : Nat)
    (multiTargetLossPriced : Prop) :
    (toNativeHashBudgetBoundary path2DirectQueryBudgetV1
      checkedCoreFrameDomainSeparated hiddenSourceFrameDomainSeparated
      multiTargetBudget multiTargetLossPriced).queryBudgetPriced :=
  path2_direct_query_budget_priced

end Path2HashBudget
end Descender
