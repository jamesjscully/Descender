namespace Descender

/-!
Compact public descriptions and evaluator-cost interfaces.

This file is intentionally operational rather than asymptotic.  It records the
finite accounting data that a Descender KEM instantiation must provide before a
"small public key / no dense public matrix" claim can be connected to the
security layer.
-/

universe u v

/-- Bit-size accounting for the public KEM surface. -/
structure PublicSizeProfile where
  publicParameterBits : Nat
  seedBits : Nat
  publicKeyBits : Nat

namespace PublicSizeProfile

/-- Componentwise size comparison against a concrete budget. -/
def BoundedBy (sizes budget : PublicSizeProfile) : Prop :=
  sizes.publicParameterBits <= budget.publicParameterBits ∧
    sizes.seedBits <= budget.seedBits ∧
    sizes.publicKeyBits <= budget.publicKeyBits

/-- Total public material that must be transmitted or stored. -/
def totalBits (sizes : PublicSizeProfile) : Nat :=
  (sizes.publicParameterBits + sizes.seedBits) + sizes.publicKeyBits

theorem totalBits_le_totalBits_of_boundedBy
    {sizes budget : PublicSizeProfile} (h : sizes.BoundedBy budget) :
    sizes.totalBits <= budget.totalBits :=
  Nat.add_le_add (Nat.add_le_add h.1 h.2.1) h.2.2

end PublicSizeProfile

/-- Public parameter size, separated out for theorem statements. -/
def publicParameterSize (sizes : PublicSizeProfile) : Nat :=
  sizes.publicParameterBits

/-- Public seed size, separated out for theorem statements. -/
def seedSize (sizes : PublicSizeProfile) : Nat :=
  sizes.seedBits

/-- Public-key size, separated out for theorem statements. -/
def publicKeySize (sizes : PublicSizeProfile) : Nat :=
  sizes.publicKeyBits

/--
Worst-case cost accounting for one public evaluator.

`work` is intended for field operations or word operations, `memory` for
working storage, and `oracleCalls` for public hash/PRG/table accesses.
-/
structure EvaluatorCost where
  work : Nat
  memory : Nat
  oracleCalls : Nat

namespace EvaluatorCost

/-- Componentwise cost comparison against a concrete budget. -/
def BoundedBy (cost budget : EvaluatorCost) : Prop :=
  cost.work <= budget.work ∧ cost.memory <= budget.memory ∧
    cost.oracleCalls <= budget.oracleCalls

/-- Combined scalar cost, useful when a theorem wants one numeric bound. -/
def total (cost : EvaluatorCost) : Nat :=
  (cost.work + cost.memory) + cost.oracleCalls

/-- Sequential composition of two evaluator-cost accounts. -/
def add (left right : EvaluatorCost) : EvaluatorCost where
  work := left.work + right.work
  memory := left.memory + right.memory
  oracleCalls := left.oracleCalls + right.oracleCalls

theorem total_le_total_of_boundedBy
    {cost budget : EvaluatorCost} (h : cost.BoundedBy budget) :
    cost.total <= budget.total :=
  Nat.add_le_add (Nat.add_le_add h.1 h.2.1) h.2.2

end EvaluatorCost

/--
A public evaluator together with the size of its description and its
worst-case execution cost.
-/
structure CompactEvaluator (Domain : Type u) (Range : Type v) where
  apply : Domain -> Range
  descriptionBits : Nat
  seedBits : Nat
  cost : EvaluatorCost

namespace CompactEvaluator

/-- Predicate form for efficient-evaluator obligations. -/
def HasCostAtMost {Domain : Type u} {Range : Type v}
    (eval : CompactEvaluator Domain Range) (budget : EvaluatorCost) : Prop :=
  eval.cost.BoundedBy budget

end CompactEvaluator

/-- Abstract two-sided public operator used by the KEM proof layer. -/
structure PublicOperator (Secret : Type u) (Measurement : Type v) where
  A : Secret -> Measurement
  AT : Measurement -> Secret

/-- The exposed public multiplication routines. -/
structure PublicEvaluators (Secret : Type u) (Measurement : Type v) where
  A_mul : CompactEvaluator Secret Measurement
  AT_mul : CompactEvaluator Measurement Secret

namespace PublicEvaluators

def descriptionBits {Secret : Type u} {Measurement : Type v}
    (evals : PublicEvaluators Secret Measurement) : Nat :=
  evals.A_mul.descriptionBits + evals.AT_mul.descriptionBits

def seedBits {Secret : Type u} {Measurement : Type v}
    (evals : PublicEvaluators Secret Measurement) : Nat :=
  evals.A_mul.seedBits + evals.AT_mul.seedBits

def cost {Secret : Type u} {Measurement : Type v}
    (evals : PublicEvaluators Secret Measurement) : EvaluatorCost :=
  evals.A_mul.cost.add evals.AT_mul.cost

def asOperator {Secret : Type u} {Measurement : Type v}
    (evals : PublicEvaluators Secret Measurement) :
    PublicOperator Secret Measurement where
  A := evals.A_mul.apply
  AT := evals.AT_mul.apply

end PublicEvaluators

/--
A compact public description of an operator: public size accounting plus the
two exposed multiplication evaluators.
-/
structure CompactOperator (Secret : Type u) (Measurement : Type v) where
  sizes : PublicSizeProfile
  evaluators : PublicEvaluators Secret Measurement

namespace CompactOperator

def descriptionBits {Secret : Type u} {Measurement : Type v}
    (op : CompactOperator Secret Measurement) : Nat :=
  op.evaluators.descriptionBits

def seedBits {Secret : Type u} {Measurement : Type v}
    (op : CompactOperator Secret Measurement) : Nat :=
  op.evaluators.seedBits

def evaluatorCost {Secret : Type u} {Measurement : Type v}
    (op : CompactOperator Secret Measurement) : EvaluatorCost :=
  op.evaluators.cost

def asOperator {Secret : Type u} {Measurement : Type v}
    (op : CompactOperator Secret Measurement) :
    PublicOperator Secret Measurement :=
  op.evaluators.asOperator

end CompactOperator

/--
Certificate that an abstract operator is represented by public evaluators whose
description, seed material, public key, and runtime costs fit declared budgets.

This is the Lean hook for the KEM skeleton claim that setup exposes `A_mul` and
`AT_mul`, with no dense matrix `A` required in the public description.
-/
structure CompactOperatorCertificate
    {Secret : Type u} {Measurement : Type v}
    (operator : PublicOperator Secret Measurement)
    (sizeBudget : PublicSizeProfile) (costBudget : EvaluatorCost) where
  compact : CompactOperator Secret Measurement
  represents_A_mul :
    ∀ s, compact.evaluators.A_mul.apply s = operator.A s
  represents_AT_mul :
    ∀ r, compact.evaluators.AT_mul.apply r = operator.AT r
  descriptionFitsPublicParameters :
    compact.descriptionBits <= compact.sizes.publicParameterBits
  evaluatorSeedsFitPublicSeed :
    compact.seedBits <= compact.sizes.seedBits
  sizeBound : compact.sizes.BoundedBy sizeBudget
  costBound : compact.evaluatorCost.BoundedBy costBudget

namespace CompactOperatorCertificate

variable {Secret : Type u} {Measurement : Type v}
variable {operator : PublicOperator Secret Measurement}
variable {sizeBudget : PublicSizeProfile} {costBudget : EvaluatorCost}

theorem A_mul_spec
    (cert : CompactOperatorCertificate operator sizeBudget costBudget)
    (s : Secret) :
    cert.compact.evaluators.A_mul.apply s = operator.A s :=
  cert.represents_A_mul s

theorem AT_mul_spec
    (cert : CompactOperatorCertificate operator sizeBudget costBudget)
    (r : Measurement) :
    cert.compact.evaluators.AT_mul.apply r = operator.AT r :=
  cert.represents_AT_mul r

theorem publicParameterSize_le_budget
    (cert : CompactOperatorCertificate operator sizeBudget costBudget) :
    publicParameterSize cert.compact.sizes <=
      publicParameterSize sizeBudget :=
  cert.sizeBound.1

theorem seedSize_le_budget
    (cert : CompactOperatorCertificate operator sizeBudget costBudget) :
    seedSize cert.compact.sizes <= seedSize sizeBudget :=
  cert.sizeBound.2.1

theorem publicKeySize_le_budget
    (cert : CompactOperatorCertificate operator sizeBudget costBudget) :
    publicKeySize cert.compact.sizes <= publicKeySize sizeBudget :=
  cert.sizeBound.2.2

theorem descriptionBits_le_publicParameterBudget
    (cert : CompactOperatorCertificate operator sizeBudget costBudget) :
    cert.compact.descriptionBits <= publicParameterSize sizeBudget :=
  Nat.le_trans cert.descriptionFitsPublicParameters cert.sizeBound.1

theorem evaluatorSeedBits_le_seedBudget
    (cert : CompactOperatorCertificate operator sizeBudget costBudget) :
    cert.compact.seedBits <= seedSize sizeBudget :=
  Nat.le_trans cert.evaluatorSeedsFitPublicSeed cert.sizeBound.2.1

theorem evaluatorCost_total_le_budget
    (cert : CompactOperatorCertificate operator sizeBudget costBudget) :
    cert.compact.evaluatorCost.total <= costBudget.total :=
  EvaluatorCost.total_le_total_of_boundedBy cert.costBound

theorem totalPublicBits_le_budget
    (cert : CompactOperatorCertificate operator sizeBudget costBudget) :
    cert.compact.sizes.totalBits <= sizeBudget.totalBits :=
  PublicSizeProfile.totalBits_le_totalBits_of_boundedBy cert.sizeBound

end CompactOperatorCertificate

end Descender
