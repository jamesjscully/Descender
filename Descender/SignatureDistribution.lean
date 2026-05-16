import Mathlib.Tactic
import Descender.Advantage
import Descender.ProductBlockSignature

namespace Descender

/-!
Distributional proof boundary for Fiat-Shamir-with-aborts signatures.

The product-block algebra and noise theorems show that sparse signed block
challenges are verifier-computable and do not enlarge the coordinate error.
This file records the distributional condition that makes the linear response
safe: after rejection sampling, every accepted-response test has exactly the
same mass as a fixed target response distribution, up to one common acceptance
factor.  The resulting conditional accepted-response distribution is therefore
independent of the secret.
-/

namespace SignatureDistribution

open Advantage

noncomputable section

theorem finiteProbability_acceptProb_true
    {Ω : Type*} [Fintype Ω] (D : FiniteProbability Ω) :
    D.acceptProb (fun _ => true) = 1 := by
  unfold FiniteProbability.acceptProb
  simpa using D.total_mass

/--
A rejection sampler for a fixed challenge.

`mask` is the pre-rejection signing-mask distribution.  `response s c y` is the
response that would be emitted for secret `s`, challenge `c`, and mask `y`.
`accept s c y` is the rejection predicate.  `target` is the desired
secret-independent response distribution after conditioning on acceptance.
-/
structure RejectionSampler
    (Secret Challenge Mask Response : Type*)
    [Fintype Mask] [Fintype Response] where
  mask : FiniteProbability Mask
  target : FiniteProbability Response
  response : Secret -> Challenge -> Mask -> Response
  accept : Secret -> Challenge -> Mask -> Bool

namespace RejectionSampler

variable {Secret Challenge Mask Response : Type*}
variable [Fintype Mask] [Fintype Response]

/-- Unnormalized mass of accepted responses that pass a Boolean response test. -/
def acceptedTestMass
    (S : RejectionSampler Secret Challenge Mask Response)
    (s : Secret) (c : Challenge) (A : Response -> Bool) : ℝ :=
  Finset.univ.sum fun y =>
    S.mask.mass y *
      boolIndicator (S.accept s c y) *
      boolIndicator (A (S.response s c y))

/-- Total acceptance mass for one secret/challenge pair. -/
def acceptedMass
    (S : RejectionSampler Secret Challenge Mask Response)
    (s : Secret) (c : Challenge) : ℝ :=
  S.acceptedTestMass s c (fun _ => true)

/-- Conditional accepted-response probability, parameterized by acceptance mass. -/
def conditionalAcceptedProb
    (S : RejectionSampler Secret Challenge Mask Response)
    (acceptance : ℝ)
    (s : Secret) (c : Challenge) (A : Response -> Bool) : ℝ :=
  S.acceptedTestMass s c A / acceptance

/--
Exact rejection-sampling certificate.

The accepted mass for every response test is `acceptance` times the target
probability of that test.  This is the formal statement that rejection sampling
has erased the secret dependence from the response distribution.
-/
structure RejectionSamplingCertificate
    (S : RejectionSampler Secret Challenge Mask Response) where
  acceptance : ℝ
  acceptance_pos : 0 < acceptance
  accepted_test_mass_eq :
    ∀ s c A,
      S.acceptedTestMass s c A =
        acceptance * S.target.acceptProb A

namespace RejectionSamplingCertificate

variable {S : RejectionSampler Secret Challenge Mask Response}

theorem acceptance_ne_zero (C : S.RejectionSamplingCertificate) :
    C.acceptance ≠ 0 :=
  ne_of_gt C.acceptance_pos

theorem acceptedMass_eq_acceptance
    (C : S.RejectionSamplingCertificate)
    (s : Secret) (c : Challenge) :
    S.acceptedMass s c = C.acceptance := by
  unfold RejectionSampler.acceptedMass
  rw [C.accepted_test_mass_eq]
  rw [finiteProbability_acceptProb_true]
  ring

theorem conditionalAcceptedProb_eq_target
    (C : S.RejectionSamplingCertificate)
    (s : Secret) (c : Challenge) (A : Response -> Bool) :
    S.conditionalAcceptedProb C.acceptance s c A =
      S.target.acceptProb A := by
  unfold RejectionSampler.conditionalAcceptedProb
  rw [C.accepted_test_mass_eq]
  field_simp [C.acceptance_ne_zero]

theorem conditionalAcceptedProb_nonneg
    (C : S.RejectionSamplingCertificate)
    (s : Secret) (c : Challenge) (A : Response -> Bool) :
    0 <= S.conditionalAcceptedProb C.acceptance s c A := by
  rw [C.conditionalAcceptedProb_eq_target]
  exact S.target.acceptProb_nonneg A

theorem conditionalAcceptedProb_le_one
    (C : S.RejectionSamplingCertificate)
    (s : Secret) (c : Challenge) (A : Response -> Bool) :
    S.conditionalAcceptedProb C.acceptance s c A <= 1 := by
  rw [C.conditionalAcceptedProb_eq_target]
  exact S.target.acceptProb_le_one A

/-- Accepted responses reveal no information about the secret. -/
theorem secret_independent
    (C : S.RejectionSamplingCertificate)
    (s₀ s₁ : Secret) (c : Challenge) (A : Response -> Bool) :
    S.conditionalAcceptedProb C.acceptance s₀ c A =
      S.conditionalAcceptedProb C.acceptance s₁ c A := by
  rw [C.conditionalAcceptedProb_eq_target,
    C.conditionalAcceptedProb_eq_target]

/-- The same proof also gives independence across fixed challenges. -/
theorem challenge_independent
    (C : S.RejectionSamplingCertificate)
    (s : Secret) (c₀ c₁ : Challenge) (A : Response -> Bool) :
    S.conditionalAcceptedProb C.acceptance s c₀ A =
      S.conditionalAcceptedProb C.acceptance s c₁ A := by
  rw [C.conditionalAcceptedProb_eq_target,
    C.conditionalAcceptedProb_eq_target]

/--
Any deterministic encoding or transcript projection of the accepted response is
also secret-independent.  This is the data-processing form needed for compact
serialization.
-/
theorem encoded_secret_independent
    (C : S.RejectionSamplingCertificate)
    {Encoded : Type*} (encode : Response -> Encoded)
    (s₀ s₁ : Secret) (c : Challenge) (A : Encoded -> Bool) :
    S.conditionalAcceptedProb C.acceptance s₀ c (fun r => A (encode r)) =
      S.conditionalAcceptedProb C.acceptance s₁ c (fun r => A (encode r)) :=
  C.secret_independent s₀ s₁ c (fun r => A (encode r))

end RejectionSamplingCertificate

end RejectionSampler

/-- Product-block linear response `z = y + chi * s`. -/
def productBlockLinearResponse
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    (chi : Block -> ZMod q)
    (mask secret : Block × LocalFree -> ZMod q) :
    Block × LocalFree -> ZMod q :=
  fun bf => mask bf + blockScaleFree chi secret bf

/--
The concrete rejection-sampler shape for product-block signatures.  The
mathematical work needed from a concrete masking distribution is exactly a
`RejectionSamplingCertificate` for this sampler.
-/
def productBlockRejectionSampler
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    [Fintype (Block × LocalFree -> ZMod q)]
    (mask target :
      FiniteProbability (Block × LocalFree -> ZMod q))
    (accept :
      (Block × LocalFree -> ZMod q) ->
        (Block -> ZMod q) ->
        (Block × LocalFree -> ZMod q) -> Bool) :
    RejectionSampler
      (Block × LocalFree -> ZMod q)
      (Block -> ZMod q)
      (Block × LocalFree -> ZMod q)
      (Block × LocalFree -> ZMod q) where
  mask := mask
  target := target
  response := fun secret chi y => productBlockLinearResponse chi y secret
  accept := accept

theorem productBlock_conditionalAcceptedProb_eq_target
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    [Fintype (Block × LocalFree -> ZMod q)]
    {mask target :
      FiniteProbability (Block × LocalFree -> ZMod q)}
    {accept :
      (Block × LocalFree -> ZMod q) ->
        (Block -> ZMod q) ->
        (Block × LocalFree -> ZMod q) -> Bool}
    (C : (productBlockRejectionSampler mask target accept).RejectionSamplingCertificate)
    (secret : Block × LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (A : (Block × LocalFree -> ZMod q) -> Bool) :
    (productBlockRejectionSampler mask target accept).conditionalAcceptedProb
        C.acceptance secret chi A =
      (productBlockRejectionSampler mask target accept).target.acceptProb A := by
  exact C.conditionalAcceptedProb_eq_target secret chi A

theorem productBlock_accepted_response_secret_independent
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    [Fintype (Block × LocalFree -> ZMod q)]
    {mask target :
      FiniteProbability (Block × LocalFree -> ZMod q)}
    {accept :
      (Block × LocalFree -> ZMod q) ->
        (Block -> ZMod q) ->
        (Block × LocalFree -> ZMod q) -> Bool}
    (C : (productBlockRejectionSampler mask target accept).RejectionSamplingCertificate)
    (secret₀ secret₁ : Block × LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (A : (Block × LocalFree -> ZMod q) -> Bool) :
    (productBlockRejectionSampler mask target accept).conditionalAcceptedProb
        C.acceptance secret₀ chi A =
      (productBlockRejectionSampler mask target accept).conditionalAcceptedProb
        C.acceptance secret₁ chi A := by
  exact C.secret_independent secret₀ secret₁ chi A

end

end SignatureDistribution

end Descender
