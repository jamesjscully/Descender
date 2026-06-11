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
  algebraicChallengeBytes : Nat
  challengeHardeningBytes : Nat
  algebraicChallengeLastByteUsableBits : Nat
  algebraicChallengeBitsAfterMask : Nat
  totalChallengeBitsAfterMask : Nat
  actionCollisionFloorBits : Nat
  actionCollisionQuantumBitsAtUnionBound : Nat
  actionCollisionClassicalBitsAtUnionBound : Nat
  minimumDeploymentMarginBits : Nat
  qBatchActive : Bool
  qBatchInactive : qBatchActive = false
  algebraicChallengeBitsMatchTranscript :
    algebraicChallengeBitsAfterMask = transcriptTermBitsAfterQromLoss
  totalChallengeBitsIncludeHardening :
    totalChallengeBitsAfterMask =
      algebraicChallengeBitsAfterMask + 8 * challengeHardeningBytes
  actionCollisionQuantumMeetsTarget :
    targetSecurityBits ≤ actionCollisionQuantumBitsAtUnionBound
  actionCollisionClassicalMeetsTarget :
    targetSecurityBits ≤ actionCollisionClassicalBitsAtUnionBound
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
  algebraicChallengeBytes := 23
  challengeHardeningBytes := 8
  algebraicChallengeLastByteUsableBits := 6
  algebraicChallengeBitsAfterMask := 182
  totalChallengeBitsAfterMask := 246
  actionCollisionFloorBits := 163
  actionCollisionQuantumBitsAtUnionBound := 131
  actionCollisionClassicalBitsAtUnionBound := 130
  minimumDeploymentMarginBits := 53
  qBatchActive := false
  qBatchInactive := rfl
  algebraicChallengeBitsMatchTranscript := rfl
  totalChallengeBitsIncludeHardening := by native_decide
  actionCollisionQuantumMeetsTarget := by native_decide
  actionCollisionClassicalMeetsTarget := by native_decide
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

/--
The active auxiliary-Fq12 challenge shape used by the deployable Path 2 route.

The compact block/sign decoder still consumes only the legacy 26-repetition
prefix.  These facts pin the wider algebraic action and transcript hardening
budget that the checked-core Fq12 equation consumes after that prefix.
-/
def AuxiliaryFq12ChallengeShapeComplete
    (B : Path2DirectQueryBudgetV1) : Prop :=
  B.algebraicChallengeBytes = 23 ∧
    B.challengeHardeningBytes = 8 ∧
    B.algebraicChallengeLastByteUsableBits = 6 ∧
    B.algebraicChallengeBitsAfterMask = 182 ∧
    B.totalChallengeBitsAfterMask = 246 ∧
    B.algebraicChallengeBitsAfterMask =
      B.transcriptTermBitsAfterQromLoss ∧
    B.totalChallengeBitsAfterMask =
      B.algebraicChallengeBitsAfterMask + 8 * B.challengeHardeningBytes

/--
Action-collision budget for the widened Fq12 action.

The `163`-bit floor is the algebraic bad-event floor.  Under the selected
union-safe deployment query bounds it leaves 131 quantum bits and 130 classical
bits, both above the 128-bit target.
-/
def Fq12ActionCollisionBudgetPriced
    (B : Path2DirectQueryBudgetV1) : Prop :=
  B.actionCollisionFloorBits = 163 ∧
    B.actionCollisionQuantumBitsAtUnionBound = 131 ∧
    B.actionCollisionClassicalBitsAtUnionBound = 130 ∧
    B.targetSecurityBits ≤ B.actionCollisionQuantumBitsAtUnionBound ∧
    B.targetSecurityBits ≤ B.actionCollisionClassicalBitsAtUnionBound

theorem path2_direct_query_budget_keeps_q_batch_inactive :
    path2DirectQueryBudgetV1.qBatchActive = false :=
  path2DirectQueryBudgetV1.qBatchInactive

theorem path2_direct_query_budget_transcript_exceeds_target_plus_margin :
    path2DirectQueryBudgetV1.transcriptTermBitsAfterQromLoss >
      path2DirectQueryBudgetV1.targetSecurityBits +
        path2DirectQueryBudgetV1.minimumDeploymentMarginBits :=
  path2DirectQueryBudgetV1.transcriptTermExceedsTargetPlusMargin

theorem path2_direct_query_budget_challenge_bits_match_transcript :
    path2DirectQueryBudgetV1.algebraicChallengeBitsAfterMask =
      path2DirectQueryBudgetV1.transcriptTermBitsAfterQromLoss :=
  path2DirectQueryBudgetV1.algebraicChallengeBitsMatchTranscript

theorem path2_direct_query_budget_total_challenge_bits_after_mask :
    path2DirectQueryBudgetV1.totalChallengeBitsAfterMask = 246 := by
  rfl

theorem path2_direct_query_budget_auxiliary_fq12_challenge_shape_complete :
    AuxiliaryFq12ChallengeShapeComplete path2DirectQueryBudgetV1 :=
  ⟨rfl, rfl, rfl, rfl, rfl,
    path2_direct_query_budget_challenge_bits_match_transcript,
    path2DirectQueryBudgetV1.totalChallengeBitsIncludeHardening⟩

theorem path2_direct_query_budget_fq12_action_collision_budget_priced :
    Fq12ActionCollisionBudgetPriced path2DirectQueryBudgetV1 :=
  ⟨rfl, rfl, rfl,
    path2DirectQueryBudgetV1.actionCollisionQuantumMeetsTarget,
    path2DirectQueryBudgetV1.actionCollisionClassicalMeetsTarget⟩

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
