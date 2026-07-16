import Descender.Path2NativeAccountDesign
import Mathlib.Data.Fintype.Basic
import Mathlib.Tactic

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

/-- Query symbols tracked by the executable direct-mode budget. -/
inductive DirectQuerySymbol where
  | q_leaf
  | q_auth
  | q_evidence
  | q_stmt
  | q_pad
  | q_mask
  | q_chal
  | q_life
  | q_transparent_trace_commit
  | q_transparent_query_index
  | q_transparent_query_row
  | q_transparent_merkle_open
  | q_transparent_consistency
  | q_transparent_extractor
  | q_transparent_simulator
  | q_batch
  deriving DecidableEq

instance : Fintype DirectQuerySymbol where
  elems :=
    { DirectQuerySymbol.q_leaf,
      DirectQuerySymbol.q_auth,
      DirectQuerySymbol.q_evidence,
      DirectQuerySymbol.q_stmt,
      DirectQuerySymbol.q_pad,
      DirectQuerySymbol.q_mask,
      DirectQuerySymbol.q_chal,
      DirectQuerySymbol.q_life,
      DirectQuerySymbol.q_transparent_trace_commit,
      DirectQuerySymbol.q_transparent_query_index,
      DirectQuerySymbol.q_transparent_query_row,
      DirectQuerySymbol.q_transparent_merkle_open,
      DirectQuerySymbol.q_transparent_consistency,
      DirectQuerySymbol.q_transparent_extractor,
      DirectQuerySymbol.q_transparent_simulator,
      DirectQuerySymbol.q_batch }
  complete := by
    intro symbol
    cases symbol <;> simp

/-- Direct-mode active query-symbol set mirrored from Rust `active_query_symbols`. -/
def activeDirectQuerySymbols : Finset DirectQuerySymbol :=
  { DirectQuerySymbol.q_leaf,
    DirectQuerySymbol.q_auth,
    DirectQuerySymbol.q_evidence,
    DirectQuerySymbol.q_stmt,
    DirectQuerySymbol.q_pad,
    DirectQuerySymbol.q_mask,
    DirectQuerySymbol.q_chal,
    DirectQuerySymbol.q_life,
    DirectQuerySymbol.q_transparent_trace_commit,
    DirectQuerySymbol.q_transparent_query_index,
    DirectQuerySymbol.q_transparent_query_row,
    DirectQuerySymbol.q_transparent_merkle_open,
    DirectQuerySymbol.q_transparent_consistency,
    DirectQuerySymbol.q_transparent_extractor,
    DirectQuerySymbol.q_transparent_simulator }

/-- Direct-mode inactive query-symbol set mirrored from Rust `inactive_query_symbols`. -/
def inactiveDirectQuerySymbols : Finset DirectQuerySymbol :=
  { DirectQuerySymbol.q_batch }

/-- Active direct-mode symbols are exactly all symbols except optional aggregation. -/
def IsActiveDirectQuerySymbol (symbol : DirectQuerySymbol) : Prop :=
  symbol ≠ DirectQuerySymbol.q_batch

/-- The explicit active-symbol set is exactly the active direct-mode predicate. -/
theorem mem_activeDirectQuerySymbols_iff
    (symbol : DirectQuerySymbol) :
    symbol ∈ activeDirectQuerySymbols ↔ IsActiveDirectQuerySymbol symbol := by
  cases symbol <;> simp [activeDirectQuerySymbols, IsActiveDirectQuerySymbol]

/-- The explicit inactive-symbol set is exactly `q_batch`. -/
theorem mem_inactiveDirectQuerySymbols_iff
    (symbol : DirectQuerySymbol) :
    symbol ∈ inactiveDirectQuerySymbols ↔
      symbol = DirectQuerySymbol.q_batch := by
  cases symbol <;> simp [inactiveDirectQuerySymbols]

/-- Direct mode has fifteen active query symbols. -/
theorem activeDirectQuerySymbols_card :
    activeDirectQuerySymbols.card = 15 := by
  native_decide

/-- Direct mode has one inactive query symbol. -/
theorem inactiveDirectQuerySymbols_card :
    inactiveDirectQuerySymbols.card = 1 := by
  native_decide

/-- Optional aggregation is not active in the direct query budget. -/
theorem q_batch_not_mem_activeDirectQuerySymbols :
    DirectQuerySymbol.q_batch ∉ activeDirectQuerySymbols := by
  simp [activeDirectQuerySymbols]

/-- Optional aggregation is the inactive direct-mode query symbol. -/
theorem q_batch_mem_inactiveDirectQuerySymbols :
    DirectQuerySymbol.q_batch ∈ inactiveDirectQuerySymbols := by
  simp [inactiveDirectQuerySymbols]

/-- No direct query symbol is both active and inactive. -/
theorem no_direct_query_symbol_active_and_inactive
    (symbol : DirectQuerySymbol) :
    symbol ∈ activeDirectQuerySymbols →
      symbol ∈ inactiveDirectQuerySymbols → False := by
  intro hActive hInactive
  exact
    (mem_activeDirectQuerySymbols_iff symbol).1 hActive
      ((mem_inactiveDirectQuerySymbols_iff symbol).1 hInactive)

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
An active direct query symbol is priced by a direct budget once the executable
budget pricing proof has been supplied.
-/
def DirectQuerySymbolPricedByBudget
    (B : Path2DirectQueryBudgetV1)
    (symbol : DirectQuerySymbol) : Prop :=
  symbol ∈ activeDirectQuerySymbols ∧
    DirectQueryBudgetPriced B

/-- Every active direct-mode query symbol is covered by one direct budget. -/
def ActiveDirectQuerySymbolsPriced
    (B : Path2DirectQueryBudgetV1) : Prop :=
  ∀ symbol : DirectQuerySymbol,
    symbol ∈ activeDirectQuerySymbols →
      DirectQuerySymbolPricedByBudget B symbol

/-- A symbol priced through the active direct set is not the inactive batch symbol. -/
theorem DirectQuerySymbolPricedByBudget.not_inactive
    {B : Path2DirectQueryBudgetV1}
    {symbol : DirectQuerySymbol}
    (h : DirectQuerySymbolPricedByBudget B symbol) :
    symbol ∉ inactiveDirectQuerySymbols := by
  intro hInactive
  exact no_direct_query_symbol_active_and_inactive symbol h.1 hInactive

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

/--
Local bridge package consumed by the direct theorem boundary.

This packages only the executable direct QROM/action budget facts: active
direct query symbols, inactive optional aggregation, transcript margin, widened
Fq12 challenge shape, and action-collision pricing. It deliberately does not
instantiate external QROM review, domain-separation review, or deployment
readiness.
-/
def DirectQromActionBudgetBridge
    (B : Path2DirectQueryBudgetV1) : Prop :=
  DirectQueryBudgetPriced B ∧
    ActiveDirectQuerySymbolsPriced B ∧
      AuxiliaryFq12ChallengeShapeComplete B ∧
        Fq12ActionCollisionBudgetPriced B

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

/-- The selected direct budget prices every active direct query symbol. -/
theorem path2_direct_query_budget_activeDirectQuerySymbolsPriced :
    ActiveDirectQuerySymbolsPriced path2DirectQueryBudgetV1 := by
  intro symbol hActive
  exact ⟨hActive, path2_direct_query_budget_priced⟩

/--
Selected-Rust bridge from the executable direct query budget into the
QROM/action proof-support package.

This is a local theorem import shape. It keeps external review and the final
direct theorem linkage as separate fail-closed obligations.
-/
theorem selectedRustDirectQromActionBudgetBridge_v1 :
    DirectQromActionBudgetBridge path2DirectQueryBudgetV1 :=
  ⟨path2_direct_query_budget_priced,
    path2_direct_query_budget_activeDirectQuerySymbolsPriced,
    path2_direct_query_budget_auxiliary_fq12_challenge_shape_complete,
    path2_direct_query_budget_fq12_action_collision_budget_priced⟩

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

/-!
## Conditional account-STARK FRI/BCS QROM budget

The cited FRI/BCS result gives the ROM error shape
`Q * epsilon_rbr + 3 * (Q^2 + 1) / 2^kappa` and a further loss linear in
`Q` against bounded-query quantum adversaries.  The definitions below price
that shape for the selected direct deployment horizon.  They deliberately do
not prove the Plonky3 round-by-round error, the challenger-to-QRO mapping, or
the hidden constant in the QROM lift; those remain explicit theorem inputs.
-/

/-- Selected deployment quantum-oracle query bound: `Q = 2^16`. -/
def selectedAccountStarkFriQromQueryCount : Nat := 2 ^ 16

/-- Checked-core primitive target horizon used by the lifetime stress control. -/
def selectedAccountStarkFriQromTargetCount : Nat := 2 ^ 32

/-- `ADR-0011`: BCS adaptive soundness uses one global adversary query budget. -/
def selectedAccountStarkFriAdaptiveGlobalQueryBound : Bool := true

/-- `ADR-0011`: no independent account-count multiplier is applied to BCS. -/
def selectedAccountStarkFriBcsAccountTargetMultiplier : Nat := 1

/-- Candidate effective random-oracle output width for the degree-8 challenge. -/
def selectedAccountStarkFriQromOutputBits : Nat := 247

/-- Conditional RBR error supplied by the selected FRI theorem: at most `2^-256`. -/
def selectedAccountStarkFriQromRbrBits : Nat := 256

/-- Conservative allowance for the unspecified constant in the linear QROM lift. -/
def selectedAccountStarkFriQromLiftConstantBound : Nat := 2 ^ 32

/-- Non-selected lifetime-stress RBR contribution with the checked-core target count. -/
noncomputable def selectedAccountStarkFriQromRbrTerm : Real :=
  ((selectedAccountStarkFriQromLiftConstantBound *
      selectedAccountStarkFriQromTargetCount *
      selectedAccountStarkFriQromQueryCount ^ 2 : Nat) : Real) /
    ((2 ^ selectedAccountStarkFriQromRbrBits : Nat) : Real)

/-- Non-selected lifetime-stress BCS oracle term with the checked-core target count. -/
noncomputable def selectedAccountStarkFriQromBcsOracleTerm : Real :=
  ((selectedAccountStarkFriQromLiftConstantBound *
      selectedAccountStarkFriQromTargetCount *
      selectedAccountStarkFriQromQueryCount * 3 *
      (selectedAccountStarkFriQromQueryCount ^ 2 + 1) : Nat) : Real) /
    ((2 ^ selectedAccountStarkFriQromOutputBits : Nat) : Real)

/-- The two locally priced terms; primitive and protocol-mapping errors are separate. -/
noncomputable def selectedAccountStarkFriQromNormalizedBudget : Real :=
  selectedAccountStarkFriQromRbrTerm +
    selectedAccountStarkFriQromBcsOracleTerm

/-- Exact selected dimensions used by the conditional budget. -/
theorem selectedAccountStarkFriQromDimensions_v1 :
    selectedAccountStarkFriQromQueryCount = 2 ^ 16 ∧
      selectedAccountStarkFriQromTargetCount = 2 ^ 32 ∧
      selectedAccountStarkFriQromOutputBits = 247 ∧
      selectedAccountStarkFriQromRbrBits = 256 ∧
      selectedAccountStarkFriQromLiftConstantBound = 2 ^ 32 := by
  norm_num [selectedAccountStarkFriQromQueryCount,
    selectedAccountStarkFriQromTargetCount,
    selectedAccountStarkFriQromOutputBits,
    selectedAccountStarkFriQromRbrBits,
    selectedAccountStarkFriQromLiftConstantBound]

/-- Machine-checked policy selected by `ADR-0011`. -/
theorem selectedAccountStarkFriAdaptiveQromPolicy_v1 :
    selectedAccountStarkFriAdaptiveGlobalQueryBound = true ∧
      selectedAccountStarkFriBcsAccountTargetMultiplier = 1 := by
  exact ⟨rfl, rfl⟩

/-- The conditionally imported RBR contribution is below `2^-130`. -/
theorem selectedAccountStarkFriQromRbrTerm_le_q130_v1 :
    selectedAccountStarkFriQromRbrTerm ≤ (1 : Real) / (2 ^ 130 : Nat) := by
  norm_num [selectedAccountStarkFriQromRbrTerm,
    selectedAccountStarkFriQromLiftConstantBound,
    selectedAccountStarkFriQromTargetCount,
    selectedAccountStarkFriQromQueryCount,
    selectedAccountStarkFriQromRbrBits, pow_succ]

/-- The conditionally mapped BCS oracle-output contribution is below `2^-130`. -/
theorem selectedAccountStarkFriQromBcsOracleTerm_le_q130_v1 :
    selectedAccountStarkFriQromBcsOracleTerm ≤
      (1 : Real) / (2 ^ 130 : Nat) := by
  norm_num [selectedAccountStarkFriQromBcsOracleTerm,
    selectedAccountStarkFriQromLiftConstantBound,
    selectedAccountStarkFriQromTargetCount,
    selectedAccountStarkFriQromQueryCount,
    selectedAccountStarkFriQromOutputBits, pow_succ]

/--
The normalized selected-profile arithmetic fits q128 with two bits of additive
headroom.  This theorem is conditional and excludes the still-open primitive,
protocol-mapping, recursion, and review terms.
-/
theorem selectedAccountStarkFriQromNormalizedBudget_le_q128_v1 :
    selectedAccountStarkFriQromNormalizedBudget ≤
      (1 : Real) / (2 ^ 128 : Nat) := by
  unfold selectedAccountStarkFriQromNormalizedBudget
  calc
    selectedAccountStarkFriQromRbrTerm +
        selectedAccountStarkFriQromBcsOracleTerm ≤
      (1 : Real) / (2 ^ 130 : Nat) + (1 : Real) / (2 ^ 130 : Nat) :=
        add_le_add selectedAccountStarkFriQromRbrTerm_le_q130_v1
          selectedAccountStarkFriQromBcsOracleTerm_le_q130_v1
    _ ≤ (1 : Real) / (2 ^ 128 : Nat) := by norm_num [pow_succ]

/-!
## Published FRI RBR bound: exact single-leaf arithmetic

The conditional budget above asks for `epsilon_rbr ≤ 2^-256`.  The published
FRI theorem instead gives an explicit maximum of a field/domain term and a
query term.  The definitions below instantiate only that published arithmetic
for the selected single-account proof:

* KoalaBear challenge field `|F| = 2130706433^8`;
* ZK-extended proof `degree_bits = 2`;
* FRI blowup `2^4`, hence initial LDE domain `|L0| = 2^6 = 64`;
* analysis parameter `m = 3` and `rho = 1/16`;
* admissible `delta = 191/256`, giving `(1-delta)^120 = (65/256)^120`.

This is still a candidate theorem instantiation: relating Plonky3's exact IOP
to the paper and supplying the QROM/sponge reductions remain separate inputs.
The arithmetic does establish an important boundary: the theorem-native
adaptive-one-forgery budget clears q128, while additionally union-bounding the
same error over `2^32` lifetime targets does not follow from this bound.
-/

/-- Prime order of the selected KoalaBear base field. -/
def selectedAccountStarkKoalaBearOrder : Nat := 2130706433

/-- The degree-8 challenge-field cardinality. -/
def selectedAccountStarkChallengeFieldOrder : Nat :=
  selectedAccountStarkKoalaBearOrder ^ 8

/-- Single-leaf ZK proof degree bits (`log2(2 rows) + 1` hiding bit). -/
def selectedAccountStarkSingleLeafDegreeBits : Nat := 2

/-- Initial FRI LDE domain: `2^(degree_bits + log_blowup) = 64`. -/
def selectedAccountStarkSingleLeafFriDomainSize : Nat :=
  2 ^ (selectedAccountStarkSingleLeafDegreeBits + 4)

/--
The explicit field/domain term from the published FRI RBR theorem at `m = 3`.
Here `(1/16)^(3/2) = 1/64`, so the expression is kept rational.
-/
noncomputable def selectedAccountStarkPublishedFriRbrFieldTerm : Real :=
  (((7 : Real) / 2) ^ 7 *
      selectedAccountStarkSingleLeafFriDomainSize ^ 2 * 64) /
    (3 * selectedAccountStarkChallengeFieldOrder)

/-- Published FRI RBR query term for `delta = 191/256` and 120 queries. -/
noncomputable def selectedAccountStarkPublishedFriRbrQueryTerm : Real :=
  ((65 : Real) / 256) ^ 120

/-- Explicit candidate RBR upper bound imported from the published formula. -/
noncomputable def selectedAccountStarkPublishedFriRbrBound : Real :=
  max selectedAccountStarkPublishedFriRbrFieldTerm
    selectedAccountStarkPublishedFriRbrQueryTerm

/-- Exact local dimensions for the published-bound candidate. -/
theorem selectedAccountStarkPublishedFriRbrDimensions_v1 :
    selectedAccountStarkChallengeFieldOrder = 2130706433 ^ 8 ∧
      selectedAccountStarkSingleLeafDegreeBits = 2 ∧
      selectedAccountStarkSingleLeafFriDomainSize = 64 := by
  norm_num [selectedAccountStarkChallengeFieldOrder,
    selectedAccountStarkKoalaBearOrder,
    selectedAccountStarkSingleLeafDegreeBits,
    selectedAccountStarkSingleLeafFriDomainSize]

/-- The explicit field/domain term is at most `2^-218`. -/
theorem selectedAccountStarkPublishedFriRbrFieldTerm_le_q218_v1 :
    selectedAccountStarkPublishedFriRbrFieldTerm ≤
      (1 : Real) / (2 ^ 218 : Nat) := by
  norm_num [selectedAccountStarkPublishedFriRbrFieldTerm,
    selectedAccountStarkSingleLeafFriDomainSize,
    selectedAccountStarkSingleLeafDegreeBits,
    selectedAccountStarkChallengeFieldOrder,
    selectedAccountStarkKoalaBearOrder, div_pow]

/-- The selected 120-query term is also at most `2^-218`. -/
theorem selectedAccountStarkPublishedFriRbrQueryTerm_le_q218_v1 :
    selectedAccountStarkPublishedFriRbrQueryTerm ≤
      (1 : Real) / (2 ^ 218 : Nat) := by
  norm_num [selectedAccountStarkPublishedFriRbrQueryTerm, div_pow]

/-- The explicit candidate RBR bound is therefore at most `2^-218`. -/
theorem selectedAccountStarkPublishedFriRbrBound_le_q218_v1 :
    selectedAccountStarkPublishedFriRbrBound ≤
      (1 : Real) / (2 ^ 218 : Nat) := by
  exact max_le selectedAccountStarkPublishedFriRbrFieldTerm_le_q218_v1
    selectedAccountStarkPublishedFriRbrQueryTerm_le_q218_v1

/-- RBR contribution in the theorem-native adaptive-one-forgery game. -/
noncomputable def selectedAccountStarkPublishedFriAdaptiveRbrTerm : Real :=
  selectedAccountStarkFriQromLiftConstantBound *
    selectedAccountStarkFriQromQueryCount ^ 2 *
    selectedAccountStarkPublishedFriRbrBound

/-- BCS oracle-output contribution without an extra lifetime-target union bound. -/
noncomputable def selectedAccountStarkFriAdaptiveBcsOracleTerm : Real :=
  ((selectedAccountStarkFriQromLiftConstantBound *
      selectedAccountStarkFriQromQueryCount * 3 *
      (selectedAccountStarkFriQromQueryCount ^ 2 + 1) : Nat) : Real) /
    ((2 ^ selectedAccountStarkFriQromOutputBits : Nat) : Real)

/-- The theorem-native adaptive RBR contribution is below `2^-154`. -/
theorem selectedAccountStarkPublishedFriAdaptiveRbrTerm_le_q154_v1 :
    selectedAccountStarkPublishedFriAdaptiveRbrTerm ≤
      (1 : Real) / (2 ^ 154 : Nat) := by
  unfold selectedAccountStarkPublishedFriAdaptiveRbrTerm
  calc
    (selectedAccountStarkFriQromLiftConstantBound : Real) *
          selectedAccountStarkFriQromQueryCount ^ 2 *
          selectedAccountStarkPublishedFriRbrBound ≤
        (selectedAccountStarkFriQromLiftConstantBound : Real) *
          selectedAccountStarkFriQromQueryCount ^ 2 *
          ((1 : Real) / (2 ^ 218 : Nat)) := by
            gcongr
            exact selectedAccountStarkPublishedFriRbrBound_le_q218_v1
    _ ≤ (1 : Real) / (2 ^ 154 : Nat) := by
      norm_num [selectedAccountStarkFriQromLiftConstantBound,
        selectedAccountStarkFriQromQueryCount, pow_succ]

/-- The theorem-native adaptive BCS oracle term is below `2^-165`. -/
theorem selectedAccountStarkFriAdaptiveBcsOracleTerm_le_q165_v1 :
    selectedAccountStarkFriAdaptiveBcsOracleTerm ≤
      (1 : Real) / (2 ^ 165 : Nat) := by
  norm_num [selectedAccountStarkFriAdaptiveBcsOracleTerm,
    selectedAccountStarkFriQromLiftConstantBound,
    selectedAccountStarkFriQromQueryCount,
    selectedAccountStarkFriQromOutputBits, pow_succ]

/-- The candidate adaptive-one-forgery arithmetic clears q128. -/
theorem selectedAccountStarkPublishedFriAdaptiveBudget_le_q128_v1 :
    selectedAccountStarkPublishedFriAdaptiveRbrTerm +
        selectedAccountStarkFriAdaptiveBcsOracleTerm ≤
      (1 : Real) / (2 ^ 128 : Nat) := by
  calc
    selectedAccountStarkPublishedFriAdaptiveRbrTerm +
          selectedAccountStarkFriAdaptiveBcsOracleTerm ≤
        (1 : Real) / (2 ^ 154 : Nat) + (1 : Real) / (2 ^ 165 : Nat) :=
      add_le_add selectedAccountStarkPublishedFriAdaptiveRbrTerm_le_q154_v1
        selectedAccountStarkFriAdaptiveBcsOracleTerm_le_q165_v1
    _ ≤ (1 : Real) / (2 ^ 128 : Nat) := by norm_num [pow_succ]

/--
The published field/domain bound, if additionally union-bounded over `2^32`
lifetime targets, is larger than `2^-128` after the selected query and QROM
constant factors.  This is a failure-to-establish theorem, not a lower bound on
the protocol's actual error.
-/
theorem selectedAccountStarkPublishedFriLifetimeFieldBudget_not_le_q128_v1 :
    ¬ ((selectedAccountStarkFriQromLiftConstantBound : Real) *
          selectedAccountStarkFriQromTargetCount *
          selectedAccountStarkFriQromQueryCount ^ 2 *
          selectedAccountStarkPublishedFriRbrFieldTerm ≤
        (1 : Real) / (2 ^ 128 : Nat)) := by
  norm_num [selectedAccountStarkFriQromLiftConstantBound,
    selectedAccountStarkFriQromTargetCount,
    selectedAccountStarkFriQromQueryCount,
    selectedAccountStarkPublishedFriRbrFieldTerm,
    selectedAccountStarkSingleLeafFriDomainSize,
    selectedAccountStarkSingleLeafDegreeBits,
    selectedAccountStarkChallengeFieldOrder,
    selectedAccountStarkKoalaBearOrder, div_pow]

end Path2HashBudget
end Descender
