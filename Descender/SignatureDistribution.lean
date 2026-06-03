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

/--
Leakage-bounded rejection-sampling certificate.

This is the proof surface for samplers such as the conditioned shifted
discrete-Gaussian mode.  Unlike `RejectionSamplingCertificate`, it does not
claim that every accepted-response distribution is identical to one common
target.  It instead gives a concrete total-variation style budget: every
Boolean test distinguishes the accepted response distributions for two secrets
by at most `leakage`.

The concrete sampler analysis is responsible for instantiating `leakage` from
finite per-coordinate TV/KL accounting and retry/timing discipline.  Once that
certificate is supplied, deterministic byte serialization is handled by the
data-processing theorem below.
-/
structure LeakageBoundedCertificate
    (S : RejectionSampler Secret Challenge Mask Response) where
  leakage : ℝ
  leakage_nonneg : 0 ≤ leakage
  acceptance : Secret -> Challenge -> ℝ
  acceptance_pos : ∀ s c, 0 < acceptance s c
  test_distance_le :
    ∀ s₀ s₁ c A,
      |S.conditionalAcceptedProb (acceptance s₀ c) s₀ c A -
        S.conditionalAcceptedProb (acceptance s₁ c) s₁ c A| ≤ leakage

namespace LeakageBoundedCertificate

variable {S : RejectionSampler Secret Challenge Mask Response}

/-- The accepted-response distinguishing advantage between two secrets. -/
def responseAdvantage
    (C : S.LeakageBoundedCertificate)
    (s₀ s₁ : Secret) (c : Challenge) (A : Response -> Bool) : ℝ :=
  |S.conditionalAcceptedProb (C.acceptance s₀ c) s₀ c A -
    S.conditionalAcceptedProb (C.acceptance s₁ c) s₁ c A|

theorem responseAdvantage_nonneg
    (C : S.LeakageBoundedCertificate)
    (s₀ s₁ : Secret) (c : Challenge) (A : Response -> Bool) :
    0 ≤ C.responseAdvantage s₀ s₁ c A := by
  unfold responseAdvantage
  exact abs_nonneg _

/--
Every response test distinguishes two accepted-response laws by at most the
certificate leakage budget.
-/
theorem responseAdvantage_le
    (C : S.LeakageBoundedCertificate)
    (s₀ s₁ : Secret) (c : Challenge) (A : Response -> Bool) :
    C.responseAdvantage s₀ s₁ c A ≤ C.leakage := by
  exact C.test_distance_le s₀ s₁ c A

/--
Data processing for deterministic encodings: serialization, projection, and
compact transcript formation cannot increase the bounded-leakage budget.
-/
theorem encoded_responseAdvantage_le
    (C : S.LeakageBoundedCertificate)
    {Encoded : Type*} (encode : Response -> Encoded)
    (s₀ s₁ : Secret) (c : Challenge) (A : Encoded -> Bool) :
    C.responseAdvantage s₀ s₁ c (fun r => A (encode r)) ≤ C.leakage := by
  exact C.responseAdvantage_le s₀ s₁ c (fun r => A (encode r))

/--
If the leakage budget is zero, the bounded-leakage certificate collapses to
ordinary secret independence for every Boolean response test.
-/
theorem secret_independent_of_zero_leakage
    (C : S.LeakageBoundedCertificate)
    (hzero : C.leakage = 0)
    (s₀ s₁ : Secret) (c : Challenge) (A : Response -> Bool) :
    S.conditionalAcceptedProb (C.acceptance s₀ c) s₀ c A =
      S.conditionalAcceptedProb (C.acceptance s₁ c) s₁ c A := by
  have hnonneg : 0 ≤ C.responseAdvantage s₀ s₁ c A :=
    C.responseAdvantage_nonneg s₀ s₁ c A
  have hle : C.responseAdvantage s₀ s₁ c A ≤ 0 := by
    simpa [hzero] using C.responseAdvantage_le s₀ s₁ c A
  have habs :
      |S.conditionalAcceptedProb (C.acceptance s₀ c) s₀ c A -
        S.conditionalAcceptedProb (C.acceptance s₁ c) s₁ c A| = 0 :=
    le_antisymm hle hnonneg
  exact sub_eq_zero.mp (abs_eq_zero.mp habs)

end LeakageBoundedCertificate

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
The public systematic view corresponding to a product-block free vector.

For signatures this is used in two places:

* the public key is `A * secret`, represented here as the systematic view of
  the secret free vector;
* the commitment is `A * mask`, represented as the systematic view of the
  signing mask.
-/
def productBlockPublicView
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (free : Block × LocalFree -> ZMod q) :
    Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q :=
  systematicRelation q (blockDiagonalSystematicMatrix Plocal) free

def productBlockPublicKey
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (secret : Block × LocalFree -> ZMod q) :
    Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q :=
  productBlockPublicView Plocal secret

def productBlockCommitment
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (mask : Block × LocalFree -> ZMod q) :
    Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q :=
  productBlockPublicView Plocal mask

/--
Honest verifier equation for the product-block signing relation.

Given public key `u = A * secret`, commitment `t = A * mask`, challenge `chi`,
and response `z`, the verifier checks

```text
A * z = t + chi * u.
```

The challenge action is block-local, so this is still verifier-computable for
the block-diagonal systematic public view.
-/
def productBlockVerifierEquation
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (publicKey commitment :
      Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q)
    (response : Block × LocalFree -> ZMod q) : Prop :=
  productBlockPublicView Plocal response =
    fun x => commitment x + blockScaleView chi publicKey x

/--
Active-only version of the honest verifier equation.

This is the compact-signature relation: for each signed active block the
signature carries only that block's free response coordinates.  The verifier
checks both the free coordinates and the local systematic pivot coordinates
against the public commitment and public key.
-/
def productBlockActiveVerifierEquation
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (publicKey commitment :
      Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q)
    (response :
      ∀ b, (chi b = 1 ∨ chi b = -1) ->
        LocalFree -> ZMod q) : Prop :=
  ∀ b hactive,
    (∀ f,
      response b hactive f =
        commitment (Sum.inr (b, f)) +
          chi b * publicKey (Sum.inr (b, f))) ∧
    (∀ p,
      -systematicPivotImage q (Plocal b) (response b hactive) p =
        commitment (Sum.inl (b, p)) +
          chi b * publicKey (Sum.inl (b, p)))

/--
Implicit commitment reconstructed by the verifier from a complete response.

This is the Schnorr-style commitment representation:

```text
t' = A * z - chi * u.
```

No commitment bytes are needed if the verifier can decode the complete free
response `z`; it reconstructs this value and hashes it as the Fiat-Shamir
challenge point.
-/
def productBlockReconstructedCommitment
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (publicKey :
      Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q)
    (response : Block × LocalFree -> ZMod q) :
    Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q :=
  fun x =>
    productBlockPublicView Plocal response x -
      blockScaleView chi publicKey x

theorem systematicPivotImage_add
    {q : Nat} [NeZero q]
    {Pivot Free : Type*} [Fintype Free]
    (P : Pivot -> Free -> ZMod q)
    (y z : Free -> ZMod q) :
    systematicPivotImage q P (fun f => y f + z f) =
      fun p => systematicPivotImage q P y p +
        systematicPivotImage q P z p := by
  classical
  funext p
  unfold systematicPivotImage
  rw [← Finset.sum_add_distrib]
  apply Finset.sum_congr rfl
  intro f _hf
  ring

theorem systematicRelation_add
    {q : Nat} [NeZero q]
    {Pivot Free : Type*} [Fintype Free]
    (P : Pivot -> Free -> ZMod q)
    (y z : Free -> ZMod q) :
    systematicRelation q P (fun f => y f + z f) =
      fun x => systematicRelation q P y x + systematicRelation q P z x := by
  funext x
  cases x with
  | inl p =>
      simp [systematicRelation, systematicPivotImage_add]
      ring
  | inr f =>
      simp [systematicRelation]

theorem productBlockPublicView_linearResponse
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (mask secret : Block × LocalFree -> ZMod q) :
    productBlockPublicView Plocal
        (productBlockLinearResponse chi mask secret) =
      fun x =>
        productBlockCommitment Plocal mask x +
          blockScaleView chi (productBlockPublicKey Plocal secret) x := by
  let P := blockDiagonalSystematicMatrix Plocal
  have hadd :
      systematicRelation q P
          (fun bf => mask bf + blockScaleFree chi secret bf) =
        fun x =>
          systematicRelation q P mask x +
            systematicRelation q P (blockScaleFree chi secret) x :=
    systematicRelation_add P mask (blockScaleFree chi secret)
  have hscale :
      blockScaleView chi (systematicRelation q P secret) =
        systematicRelation q P (blockScaleFree chi secret) :=
    blockScaleView_systematicRelation (q := q) Plocal chi secret
  funext x
  calc
    productBlockPublicView Plocal
        (productBlockLinearResponse chi mask secret) x =
        systematicRelation q P
          (fun bf => mask bf + blockScaleFree chi secret bf) x := by
          rfl
    _ =
        systematicRelation q P mask x +
          systematicRelation q P (blockScaleFree chi secret) x := by
          exact congrFun hadd x
    _ =
        productBlockCommitment Plocal mask x +
          blockScaleView chi (productBlockPublicKey Plocal secret) x := by
          rw [← congrFun hscale x]
          rfl

theorem productBlockVerifierEquation_honest
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (mask secret : Block × LocalFree -> ZMod q) :
    productBlockVerifierEquation Plocal chi
      (productBlockPublicKey Plocal secret)
      (productBlockCommitment Plocal mask)
      (productBlockLinearResponse chi mask secret) := by
  exact productBlockPublicView_linearResponse Plocal chi mask secret

theorem productBlockReconstructedCommitment_eq_of_verifierEquation
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    {Plocal : Block -> LocalPivot -> LocalFree -> ZMod q}
    {chi : Block -> ZMod q}
    {publicKey commitment :
      Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q}
    {response : Block × LocalFree -> ZMod q}
    (h :
      productBlockVerifierEquation Plocal chi publicKey commitment response) :
    productBlockReconstructedCommitment Plocal chi publicKey response =
      commitment := by
  funext x
  have hx := congrFun h x
  simp at hx
  simp [productBlockReconstructedCommitment, hx]

theorem productBlockVerifierEquation_of_reconstructedCommitment_eq
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    {Plocal : Block -> LocalPivot -> LocalFree -> ZMod q}
    {chi : Block -> ZMod q}
    {publicKey commitment :
      Sum (Block × LocalPivot) (Block × LocalFree) -> ZMod q}
    {response : Block × LocalFree -> ZMod q}
    (h :
      productBlockReconstructedCommitment Plocal chi publicKey response =
        commitment) :
    productBlockVerifierEquation Plocal chi publicKey commitment response := by
  funext x
  have hx := congrFun h x
  simp [productBlockReconstructedCommitment] at hx
  rw [← hx]
  ring

theorem productBlockReconstructedCommitment_honest
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (mask secret : Block × LocalFree -> ZMod q) :
    productBlockReconstructedCommitment Plocal chi
      (productBlockPublicKey Plocal secret)
      (productBlockLinearResponse chi mask secret) =
        productBlockCommitment Plocal mask := by
  exact productBlockReconstructedCommitment_eq_of_verifierEquation
    (productBlockVerifierEquation_honest Plocal chi mask secret)

theorem productBlockActiveVerifierEquation_honest
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (mask secret : Block × LocalFree -> ZMod q) :
    productBlockActiveVerifierEquation Plocal chi
      (productBlockPublicKey Plocal secret)
      (productBlockCommitment Plocal mask)
      (fun b _hactive f =>
        productBlockLinearResponse chi mask secret (b, f)) := by
  intro b hactive
  constructor
  · intro f
    simp [productBlockLinearResponse, productBlockPublicKey,
      productBlockCommitment, productBlockPublicView,
      systematicRelation, blockScaleFree]
  · intro p
    have hfull :
        productBlockVerifierEquation Plocal chi
          (productBlockPublicKey Plocal secret)
          (productBlockCommitment Plocal mask)
          (productBlockLinearResponse chi mask secret) :=
      productBlockVerifierEquation_honest Plocal chi mask secret
    have hp := congrFun hfull (Sum.inl (b, p))
    have hlocal :
        systematicPivotImage q (Plocal b)
            (fun f => productBlockLinearResponse chi mask secret (b, f)) p =
          systematicPivotImage q (blockDiagonalSystematicMatrix Plocal)
            (productBlockLinearResponse chi mask secret) (b, p) := by
      exact (blockDiagonalSystematicPivotImage_eq (q := q) Plocal b p
        (productBlockLinearResponse chi mask secret)).symm
    rw [hlocal]
    simpa [productBlockVerifierEquation, productBlockPublicView,
      productBlockPublicKey, productBlockCommitment,
      productBlockLinearResponse, systematicRelation, blockScaleView,
      blockScaleFree] using hp

theorem linearResponse_of_productBlockVerifierEquation
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    {Plocal : Block -> LocalPivot -> LocalFree -> ZMod q}
    {chi : Block -> ZMod q}
    {mask secret response : Block × LocalFree -> ZMod q}
    (h :
    productBlockVerifierEquation Plocal chi
        (productBlockPublicKey Plocal secret)
        (productBlockCommitment Plocal mask)
        response) :
    response = productBlockLinearResponse chi mask secret := by
  funext bf
  have hfree := congrFun h (Sum.inr bf)
  simpa [productBlockVerifierEquation, productBlockPublicView,
    productBlockPublicKey, productBlockCommitment,
    productBlockLinearResponse, systematicRelation, blockScaleView,
    blockScaleFree] using hfree

theorem activeLinearResponse_of_productBlockActiveVerifierEquation
    {q : Nat} [NeZero q]
    {Block LocalPivot LocalFree : Type*}
    [DecidableEq Block] [Fintype Block] [Fintype LocalFree]
    {Plocal : Block -> LocalPivot -> LocalFree -> ZMod q}
    {chi : Block -> ZMod q}
    {mask secret : Block × LocalFree -> ZMod q}
    {response :
      ∀ b, (chi b = 1 ∨ chi b = -1) ->
        LocalFree -> ZMod q}
    (h :
      productBlockActiveVerifierEquation Plocal chi
        (productBlockPublicKey Plocal secret)
        (productBlockCommitment Plocal mask)
        response)
    (b : Block) (hactive : chi b = 1 ∨ chi b = -1)
    (f : LocalFree) :
    response b hactive f =
      productBlockLinearResponse chi mask secret (b, f) := by
  have hfree := (h b hactive).1 f
  simpa [productBlockPublicKey, productBlockCommitment,
    productBlockPublicView, productBlockLinearResponse,
    systematicRelation, blockScaleView, blockScaleFree] using hfree

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

/--
Product-block bounded-leakage theorem for conditioned samplers.

For the conditioned shifted discrete-Gaussian mode, the concrete finite sampler
analysis supplies a `LeakageBoundedCertificate` whose `leakage` field is the
charged product TV/KL budget for all serialized response coordinates.  This
theorem is the formal handoff: every Boolean test on accepted product-block
responses distinguishes two secrets by at most that budget.
-/
theorem productBlock_accepted_response_leakage_bounded
    {q : Nat} [NeZero q]
    {Block LocalFree : Type*}
    [Fintype (Block × LocalFree -> ZMod q)]
    {mask target :
      FiniteProbability (Block × LocalFree -> ZMod q)}
    {accept :
      (Block × LocalFree -> ZMod q) ->
        (Block -> ZMod q) ->
        (Block × LocalFree -> ZMod q) -> Bool}
    (C :
      (productBlockRejectionSampler mask target accept).LeakageBoundedCertificate)
    (secret₀ secret₁ : Block × LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (A : (Block × LocalFree -> ZMod q) -> Bool) :
    C.responseAdvantage secret₀ secret₁ chi A ≤ C.leakage := by
  exact C.responseAdvantage_le secret₀ secret₁ chi A

/--
Serialized product-block bounded-leakage theorem.

Any deterministic encoder of the accepted response, including the concrete
byte-level response packing used by the verifier, inherits the same leakage
budget.  This is the data-processing step needed for the conditioned
discrete-Gaussian signing mode.
-/
theorem productBlock_encoded_response_leakage_bounded
    {q : Nat} [NeZero q]
    {Block LocalFree Encoded : Type*}
    [Fintype (Block × LocalFree -> ZMod q)]
    {mask target :
      FiniteProbability (Block × LocalFree -> ZMod q)}
    {accept :
      (Block × LocalFree -> ZMod q) ->
        (Block -> ZMod q) ->
        (Block × LocalFree -> ZMod q) -> Bool}
    (C :
      (productBlockRejectionSampler mask target accept).LeakageBoundedCertificate)
    (encode : (Block × LocalFree -> ZMod q) -> Encoded)
    (secret₀ secret₁ : Block × LocalFree -> ZMod q)
    (chi : Block -> ZMod q)
    (A : Encoded -> Bool) :
    C.responseAdvantage secret₀ secret₁ chi (fun r => A (encode r)) ≤
      C.leakage := by
  exact C.encoded_responseAdvantage_le encode secret₀ secret₁ chi A

end

end SignatureDistribution

end Descender
