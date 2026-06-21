# Committed-Sigma Proof Plan

> PACKAGE CONTEXT (2026-06-20): This package-scoped historical note is covered
> by `research/ARCHIVE_INDEX.md`. Current authority starts at `spec/README.md`,
> ADRs in `decisions/`, and `TRACKER.md`.

Status: new main proof direction, opened 2026-05-21.

The representative quotient verifier is no longer the target theorem.  The
new target is a stateful committed-sigma architecture:

```text
account/slot state:
  committed hidden witness state
  committed prechallenge randomizer state, or a slot commitment from which it
  is derived

signature transcript:
  prechallenge proof commitment a
  Fiat-Shamir challenge c = H(frame including a and slot commitments)
  response z = r + c*w
  local opening proof tying z to the same committed objects

verifier:
  check canonical encodings and slot state
  check L(z) = a + c*digest
  check public opening proof
  reject public post-challenge representative reconstruction
```

## Why This Replaces The Old Proof

The old executable representative verifier accepted a public conditioned-fiber
forgery.  Adding the strict equation `L(z) = a + c*digest` blocked that exact
fixed-commitment transcript, but `public_sigma_equation_forgery_probe` adapted:
it chooses `a`, derives `c`, solves the public post-challenge quotient equation
for `z`, and then chooses a local representative accepted by the current
opening predicate.

The core invariant is therefore:

```text
acceptance must imply that z is backed by prechallenge hidden witness and
randomizer commitments, not merely that z is a public solution to the linear
equation.
```

## Lean Surface

New module:

```text
Descender.CommittedSigma
```

Initial definitions:

- `LinearSigmaAlgebra`: abstract `L(z) = A(r) + c*digest(w)` algebra.
- `BindingCommitment`: commitment interface with an explicit binding-break
  event.
- `Slot`: public account slot carrying statement digest and witness commitment.
- `Prechallenge`: proof commitment plus randomizer commitment.
- `Transcript`: slot, prechallenge, challenge, response, opening.
- `BackedByHiddenWitness`: hidden witness/randomizer explanation for every
  public transcript field.
- `VerifierAccepts`: public verifier predicate, intentionally stronger than
  the linear equation alone.
- `SoundnessBreaks`: named failure events.
- `CommittedSigmaSoundnessCertificate`: target theorem:

```text
VerifierAccepts(T)
  -> BackedByHiddenWitness(T)
     or witness commitment binding breaks
     or randomizer commitment binding breaks
     or opening proof breaks
     or Fiat-Shamir frame breaks
     or post-challenge representative gate breaks
```

This is not yet the final byte-level theorem.  It is the clean replacement
surface that the byte-level Rust/Lean implementation must instantiate.

## Proof Work Items

1. Instantiate `LinearSigmaAlgebra` for the full-response quotient map:
   `relationMap = 12 x 23 q-syndrome`, `Digest = Fq12`, and
   `responseOf r c w = r + c*w` at the quotient level.

   Status: first Lean adapter complete.  `Descender.CommittedSigma` defines
   `FullResponseQuotient.statementAlgebra`, which reuses only the algebraic
   operations from `FullResponseByteOpeningAlgebra` and proves that its
   verifier equation matches the old `verifyFields` equation.  This is not a
   deployed soundness theorem; it is the quotient sigma algebra that the new
   committed opening layer must instantiate.

2. Define the prechallenge randomizer commitment:
   - minimum viable: hash commitment to randomizer seed/state,
   - preferred stateful version: account slot includes or derives a committed
     randomizer pool,
   - verifier frame binds the randomizer commitment and proof commitment before
     challenge derivation.

   Status: structural interface complete.  `Descender.CommittedSigma` now
   defines `PrechallengeRandomizerOpeningInterface`.  Its core theorem field is:

   ```text
   prechallengeWellFormed
   ∧ L(z) = a + c*digest
   ∧ openingAccepts
     => BackedByHiddenWitness or openingProofBreak
   ```

   This is the smallest acceptable soundness interface.  A bare hash
   commitment to `r` is not sufficient, and revealing `r` is not acceptable:
   from public `z`, public challenge action `c`, and revealed `r`, the verifier
   can recover the hidden witness response.  The deployed opening proof must
   link committed `r`, committed `w`, and response `z` without revealing either
   hidden value.

   Status update: `Descender.CommittedSigma` now also defines
   `RevealedRandomizerWitnessRecovery`, proving abstractly that any backed
   transcript with revealed randomizer gives public witness recovery whenever
   the response algebra has the usual inverse operation.  This makes
   "reveal `r`" a formally excluded byte strategy, not just a design preference.

3. Redesign the opening predicate so an arbitrary post-challenge public
   representative cannot satisfy it.  Candidate gates:
   - prove `z` opens against committed `r` and committed `w`,
   - prove the local openings are derived from the same hidden preimage,
   - forbid choosing the hidden seed/fiber state after `c`.

4. Prove fork extraction:
   two accepting transcripts with the same prechallenge commitments and
   distinct noncolliding challenge actions extract `w`, or produce one of the
   named break events.

5. Prove byte-realization:
   the serialized verifier implements the abstract committed-sigma verifier
   byte-for-byte: canonical q packing, frame fields, slot state, challenge
   bytes, local openings, and state consumption.

6. Keep the old representative verifier and both public forgery probes as
   regression controls.  They must fail against the new committed-sigma path.

## New Parameter Search Model

Parameter search should be redone after the abstract proof object is fixed, but
before committing to a single byte layout.  The old search optimized the wrong
object.  The new search must optimize `ParameterSearchModel` from
`Descender.CommittedSigma`:

```text
publicKeyBytes
accountSlotBytes
witnessCommitmentBytes
randomizerCommitmentBytes
challengeBytes
responseBytes
openingBytes
proofOverheadBytes
classicalSecurityBits
quantumSecurityBits
rejectsPublicPostChallengeRepresentatives
hasPrechallengeWitnessBoundCommitment
```

Structural gates are mandatory:

```text
meets128
rejectsPublicPostChallengeRepresentatives
hasPrechallengeWitnessBoundCommitment
```

Only candidates passing these gates should be ranked by bytes.

## Byte Strategy

The current best hypothesis is that the stateful model should move as much
prechallenge binding as possible into account/slot state rather than into every
signature.  The search should therefore report:

- signature bytes,
- account slot bytes,
- total account expansion per spendable slot,
- verifier work,
- signer sampling/retry cost,
- whether randomizer commitments can be batched or Merkleized.

Before ranking by bytes, the search should enumerate
`OpeningInstantiationCandidate` values and reject candidates unless:

```text
not revealsRandomizer
instantiatesPrechallengeOpeningInterface
postChallengeRepresentativeRejected
witnessHidingArgument
```

The currently excluded cheap paths are:

- reveal `r`: small, but leaks `w` from `z = r + c*w`;
- bare `Hash(r)` with no proof link: does not stop public post-challenge
  representative reconstruction;
- local openings derived solely from `z`: same adapted public-forgery failure
  as `public_sigma_equation_forgery_probe`.

Status update: these exclusions are now Lean objects, not only prose:

- `OpeningInstantiationCandidate.revealRandomizerCandidate`
- `OpeningInstantiationCandidate.bareHashRandomizerCandidate`
- `OpeningInstantiationCandidate.zOnlyOpeningCandidate`

Each has a corresponding `*_not_structurallyAdmissible` theorem.  The remaining
admissible shape is `abstractHiddenLinkProofCandidate`, which is admissible
only after supplying:

```text
instantiatesPrechallengeOpeningInterface
postChallengeRepresentativeRejected
witnessHidingArgument
```

That is now the exact target for the smallest byte-level construction.

Status update: `Descender.CommittedSigma` now includes the concrete
`N32Q12289CommittedSigmaByteFrontier`.  It pins the current byte pressure:

```text
base stateful transaction before committed-sigma proof: 181 B
proof commitment a = L(r):                            32 B
current 41-byte masked response, no hidden link:      254 B
37-byte fiber-saturated response, no hidden link:     250 B
39-byte fiber-state-preserving response, no link:     252 B
```

The important theorem is the budget boundary:

```text
transactionBytes 41 hiddenLinkOpeningBytes <= 256
  <-> hiddenLinkOpeningBytes <= 2
```

So the current 41-byte response layout cannot plausibly carry a real
hidden-link proof under a 256-byte cap.  The live low-byte route is the
fiber-saturated quotient architecture: prove the hidden-link obligations while
shrinking the masked response to 37 or 39 bytes, or consciously move the target
above 256 bytes.

The next proof object is also explicit now:
`AccountCarriedHiddenLinkInstantiation`.  It separates
`signatureHiddenLinkBytes` from `accountHiddenLinkBytes`.  This captures the
stateful blockchain tradeoff directly: the live 250-byte target is
`accountCarriedFiberSaturatedTarget`, which uses the 37-byte response and zero
extra signature bytes for the hidden-link layer.  It is still admissible only
after proving:

```text
openingInterfaceHolds
postChallengeRepresentativeRejected
witnessHidingArgument
```

So the concrete cryptographic task is no longer "make the current 254-byte
public equation sound"; it is "build an account-state hidden-link mechanism that
instantiates those three obligations for the 37-byte fiber-saturated quotient."

Status update: the verifier-facing account-state contract is now formalized as
`AccountCarriedHiddenLinkInterface`.  The consumed slot carries:

```text
statementDigest
witnessCommitment
randomizerCommitment
hiddenLinkCommitment
```

The signature still carries `proofCommitment`, challenge, response, and opening.
The interface theorem is:

```text
accountSlotWellFormed
∧ L(response) = proofCommitment + challenge * statementDigest
∧ openingAccepts
  => AccountBackedByHiddenWitness or accountLinkProofBreak
```

`AccountBackedByHiddenWitness` converts back to the generic
`BackedByHiddenWitness`, so this account-state interface is a concrete
instantiation path for the main committed-sigma soundness certificate.

Status update: the first account-link negative result is also formalized.  If a
transcript satisfies slot well-formedness, the sigma equation, and
`openingAccepts`, but is not `AccountBackedByHiddenWitness`, then
`AccountCarriedHiddenLinkInterface.unbacked_acceptance_forces_break` proves that
`accountLinkProofBreak` has occurred.  Equivalently, under no break the verifier
must reject any unbacked public post-challenge representative.  The
`bareAccountHashLinkTarget` candidate records the immediate corollary: simply
putting a hash/link value in account state is not enough unless the serialized
opening predicate proves the response is linked to that account-state object.

Status update: the account-carried target now has its own top-level certificate.
`AccountCarriedVerifierAccepts` is the public verifier predicate, and
`AccountCarriedSoundnessCertificate` states:

```text
AccountCarriedVerifierAccepts(T)
  => AccountBackedByHiddenWitness(T) or SoundnessBreaks.any
```

`AccountCarriedSoundnessCertificate.fromHiddenLinkInterface` builds that
certificate from `AccountCarriedHiddenLinkInterface`, routing
`accountLinkProofBreak` through the `openingProofBreak` slot of
`SoundnessBreaks`.  The helper
`SoundnessBreaks.any_of_openingProofBreak` now centralizes that injection so the
break ledger is less error-prone.

Status update: the concrete opening layer is now decomposed.  The type
`AccountCarriedOpening` splits the signature opening payload into:

```text
localOpening
linkProof
```

`AccountCarriedVerifierAcceptsWithAccountOpening` is the verifier shape whose
opening predicate is allowed to inspect the consumed account slot.  This is
necessary because the hidden-link proof must check against
`hiddenLinkCommitment`, which is not present in the generic transcript.
`AccountLinkProofSystem` packages the composition theorem:

```text
account slot well-formed
∧ sigma equation
∧ localOpeningAccepts
∧ linkProofAccepts
  => AccountBackedByHiddenWitness or accountLinkProofBreak
```

This is now the exact adapter a concrete link proof should instantiate.

Status update: the link-proof candidate frontier is now explicit.  Lean defines
`LinkProofInstantiationCandidate` and rejects:

```text
reveal-witness-or-randomizer-link
local-opening-only-link
bare-account-hash-link
slot-bound-first-message-only
public-projection-homomorphic-link
```

The remaining live class is `abstractAccountLinkProofCandidate`, specialized in
the n32/q12289 byte frontier as `accountCarriedLinkProofCandidate`: zero
signature link-proof bytes, account-carried link evidence, and admissible only
after supplying:

```text
instantiatesAccountLinkProofSystem
postChallengeRepresentativeRejected
nonPublicProjectionOfSigmaEquation
witnessHidingArgument
```

The Rust report mirrors this as `account_link_proof_frontier`, including the
six-byte signature-link stress target that exactly consumes the spare sub-256
budget of the 37-byte response.

Status update: `SlotBindsProofCommitment` now records the necessary stateful
first-message invariant: the proof commitment in the signature equals the
first-message value reserved in the consumed slot.  Backed transcripts satisfy
this invariant when the slot's randomizer commitment is the same map as
`proofCommitmentOfRandomizer`.  This prevents adaptive choice of `a`, but the
`slot-bound-first-message-only` candidate is still rejected because it does not
prove that the post-challenge response `z` is linked to hidden `w,r`.

Status update: the first concrete zero-signature-byte live construction class is
now formalized as `HomomorphicAccountLinkInterface`.  It models a deterministic
public response commitment and account-stored commitments satisfying:

```text
Commit(response) = Commit(r) + challenge * Commit(w)
```

`toAccountLinkProofSystem` converts this interface into the generic
`AccountLinkProofSystem` with `LinkProof = Unit`, so the link proof carries zero
signature bytes.  This does not close the cryptography: the exact obligation is
`linkEquationSoundness`, namely that passing the homomorphic link equation plus
the local opening implies `AccountBackedByHiddenWitness` or `linkProofBreak`.
The proof surface also now contains
`HomomorphicAccountLinkInterface.linkProofAccepts_of_publicProjection`, which
formalizes the main rejected shortcut: if the response and account commitments
are merely homomorphic public projections of `L(response)`, `proofCommitment`,
and `statementDigest`, then the link equation follows from the public sigma
equation alone and adds no security.
The Rust frontier now includes this row as
`homomorphic-account-link-equation`, with
`public-projection-homomorphic-link` recorded as the rejected degenerate
variant.

Status update: the homomorphic obligation is now factored into the two proof
pieces we actually need for a deterministic linear account-code instantiation:

```text
AccountBackedExceptResponse:
  account state and local opening identify the same hidden w,r
  but do not yet prove response = r + c*w

ResponseLinkSoundness:
  given those same w,r, linkProofAccepts forces
  response = r + c*w or a link-kernel break occurs
```

`AccountBackedExceptResponse.accountBacked_or_linkKernelBreak` composes these
pieces into `AccountBackedByHiddenWitness`.  This is the right next target for
linear-code experiments: the remaining concrete question is whether the chosen
deterministic commitment map has an acceptable kernel on the allowed response
box while not being a public projection of the sigma equation.

Status update: Lean now has the bounded-box bridge theorem:

```text
BoundedResponseLinkKernelCertificate.responseLinkSoundness
```

It proves `ResponseLinkSoundness` from three byte-level obligations:

- the accepted response and honest response for the same `w,r` are in the
  response-difference box,
- the deterministic link equation gives equal response-code commitments,
- equality in both `L` and the response-code map inside that box implies equal
  responses, or the bounded link-kernel assumption breaks.

Status update: the Rust report now emits a first `response_link_code_frontier`.
It records the important soundness/hiding tradeoff:

- `full-kernel-coordinate-code`: use the 11 free response coordinates as the
  deterministic link code.  This is only a 20-byte q-vector commitment and has
  rank 11 on the public quotient kernel, so `L(z)` plus `C(z)` is globally
  injective.  That makes `ResponseLinkSoundness` easy, but it is not deployable
  for public account state because `L(w)` plus `C(w)` reveals the witness.
- `bounded-box-kernel-code-target`: keep a smaller code and require a bounded
  kernel certificate instead.  This preserves the possibility of public-account
  hiding, but the missing proof is now explicit: no nonzero allowed response
  delta may lie in `ker(L) ∩ ker(C)`.  The current executable target uses the
  two-coordinate projection `[0, 3]`: it leaves a 9-dimensional algebraic
  kernel for hiding, and the LLL intersection-lattice probe currently finds
  shortest coefficient bound `139`, above the allowed response-difference box
  bound `2 * 48 = 96`, for a margin of `43`.  Comparison rows `[0, 1]`,
  `[0, 2]`, and `[1, 2]` also clear the LLL box probe with smaller margins.
  This is evidence, not yet a complete SVP proof.

Status update: the exact-check harness is now a repo artifact:

```text
scripts/research/response_link_bounded_kernel_check.py
```

It reconstructs the same SHA-256-derived matrix as the Rust report and supports
`lll`, `lattice-enum`, `exact-enum`, `z3`, `cp-sat`, `scip`,
`cp-sat-split`, and `scip-split` modes.  The fast `lll` mode reproduces the
`[0, 3]` evidence and matrix transcript hash
`b21703e6381ec794407acf6313af5e347d591b65251ad1f8e7a24d06b0bb71a6`.

The new `lattice-enum` mode enumerates the whole intersection lattice inside
the Euclidean ball of radius squared `23 * 96^2 = 211968`; this ball contains
every possible signed response-difference vector in `[-96,96]^23`.  For the
target `[0, 3]` projection it checks `99458` nonzero lattice vectors in the
cover ball and finds no vector with coefficient bound at most `96`.  The
reproducible run artifact is:

```text
research/cryptanalysis/signature_readiness_20260516/response_link_bounded_kernel_enum_n32_q12289_coords_0_3_b96.json
```

The slower `exact-enum` mode replays the same search with exact rational
Gram-Schmidt pruning and exact integer leaf-norm checks.  It reaches the same
`infeasible` conclusion with `99458` nonzero vectors checked, `1123628` search
nodes, and shortest cover-ball vector coefficient bound `139`.  Artifact:

```text
research/cryptanalysis/signature_readiness_20260516/response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json
```

Both artifacts now carry deterministic fingerprints for the reduced lattice
basis and the enumerated cover-ball vectors:

```text
canonical_basis_sha256:
  c6a0cd8f470d1ec80886be3352f6144a90642826f0962ad2d077e727217fcf69

reduced_basis_sha256:
  1348db737dfb311a4d53b3e5969774b82515a7e7f5b76689e39ee173ee9a44f0

cover_ball_vector_sha256:
  e269022d387b191cc3d926a32a746e8e87b9e89e720dd04fb1dc91f6be4a0c95
```

They also embed the reduced basis itself.  Replay checks that the embedded
basis is the same full-rank integer lattice as the canonical intersection
basis: every row satisfies the modular relation, every row is an integer
combination of the canonical basis, and the absolute determinants match:

```text
|det| =
  1791561079655532333095965150039192629866166308243408855041
```

The checker can replay either artifact directly:

```bash
python3 scripts/research/response_link_bounded_kernel_check.py \
  --verify-json research/cryptanalysis/signature_readiness_20260516/response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json \
  --timeout 180
```

The current replay status is `verified` with no mismatched certificate fields.
For the exact artifact, replay enumerates from the embedded reduced basis after
this lattice-equivalence check; it does not need to regenerate the basis with
LLL during verification.

There is also a smaller standalone verifier:

```bash
python3 scripts/research/verify_response_link_bounded_kernel_certificate.py \
  research/cryptanalysis/signature_readiness_20260516/response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json \
  --timeout 240
```

This verifier uses only Python's standard library.  It reconstructs the
canonical matrix, validates the embedded basis, and replays the exact
enumeration.  It has verified both the `exact-enum` and `lattice-enum`
artifacts with no mismatches.

The exact artifact now also has an audit manifest emitted by the standalone
verifier:

```text
research/cryptanalysis/signature_readiness_20260516/response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json
```

The manifest records the certificate file hash, verifier file hash, canonical
statement, replay command, basis validation, search counts, shortest cover-ball
vector, and cover-ball transcript hash.  Current manifest payload hash:

```text
da10b823ad8f4142dfeccdbf7bbcddbb5e7148f420d8ff36687bae412d09fbf0
```

It was produced by:

```bash
python3 scripts/research/verify_response_link_bounded_kernel_certificate.py \
  research/cryptanalysis/signature_readiness_20260516/response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json \
  --timeout 240 \
  --write-manifest research/cryptanalysis/signature_readiness_20260516/response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json
```

This changes the status of the bounded-box certificate: the concrete existence
question is closed internally by exact-rational computational enumeration for
the canonical matrix and bound.  The remaining work is to make the certificate
third-party audit-grade: have an independent reviewer replay the manifest and
optionally validate with external SVP tooling.  The MILP/SMT split modes remain
useful independent cross checks, but short runs still return
`timeout`/`unknown`.

Status update: the Lean account-carried proof now has the composition adapter
that was missing between local/account opening and response-link soundness:

```text
AccountCarriedAccountOpeningSoundnessCertificate
AccountLinkProofSystem.toAccountOpeningSoundnessCertificate
AccountPreResponseBackingSystem
AccountPreResponseBackingSystem.toAccountLinkProofSystem
AccountPreResponseBackingSystem.toAccountLinkProofSystemOfBoundedKernel
AccountPreResponseBackingSystem.toAccountOpeningSoundnessCertificateOfBoundedKernel
AccountPreResponseBackingSystem.accepts_accountBacked_of_noBreaks_ofBoundedKernel
AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations
AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations.toAccountOpeningSoundnessCertificate
```

`AccountCarriedAccountOpeningSoundnessCertificate` is the deployment-facing
certificate for account-aware openings: the opening verifier may inspect the
consumed account slot, which is necessary for hidden-link checks.
`AccountLinkProofSystem.toAccountOpeningSoundnessCertificate` converts any
account-link proof system into that top-level certificate.

`AccountPreResponseBackingSystem` is the pre-response theorem shape: the
consumed account slot and local opening identify the same hidden `w,r`, but do
not yet assert `response = responseOf r challenge w`.  Its adapters compose that
pre-response backing with either an abstract `ResponseLinkSoundness` theorem or
a concrete `BoundedResponseLinkKernelCertificate`.  The resulting object is an
`AccountLinkProofSystem`, and then the account-link proof system converts to the
account-aware top-level soundness certificate.

The final two helpers package the whole path in one step: a pre-response
backing system plus a bounded response-link kernel certificate directly yields
an `AccountCarriedAccountOpeningSoundnessCertificate`, and if neither the
pre-response backing break nor the link-kernel break occurs, every accepting
account-aware transcript is backed by the hidden account witness/randomizer.
The `BoundedKernelDeploymentObligations` object names the exact remaining
byte-level premises for a concrete instantiation: the bounded kernel
certificate, proof-commitment agreement with `A.randomizerMap`, the response
commitment link equation, and the accepted response-difference box condition.
Supplying that object is now enough to derive the account-aware top-level
certificate.

The candidate frontier now also has a direct constructor:

```text
LinkProofInstantiationCandidate.fromBoundedKernelDeploymentObligations
LinkProofInstantiationCandidate.fromBoundedKernelDeploymentObligations_structurallyAdmissible
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelLinkProofCandidate
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelLinkProofCandidate_structurallyAdmissible
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelFiberSaturatedTarget
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelFiberSaturatedTarget_transactionBytes_eq
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelFiberSaturatedTarget_structurallyAdmissible
```

So the zero-link-byte account-carried row no longer has to cite an abstract
`hSystem` placeholder.  A concrete `BoundedKernelDeploymentObligations` object
produces the `Nonempty AccountLinkProofSystem` witness needed by structural
admissibility, and the N32/q12289 byte-frontier wrapper pins the signature link
proof bytes to `0`.  The remaining non-proof-system gates are exactly
post-challenge representative rejection, non-public-projection, and witness
hiding.

The byte-frontier target itself is also now tied to the proof object:
`accountCarriedBoundedKernelFiberSaturatedTarget` packages the same bounded
deployment obligations as the opening-interface witness for the 37-byte
fiber-saturated account-carried transaction row.  Lean proves this row has
`transactionBytes = 250`, is under the 256-byte cap, and is structurally
admissible once representative rejection and witness hiding are supplied.

The remaining structural gates are now bundled as well:

```text
N32Q12289CommittedSigmaByteFrontier.BoundedKernelTargetStructuralGates
N32Q12289CommittedSigmaByteFrontier.BoundedKernelTargetStructuralBreaks
N32Q12289CommittedSigmaByteFrontier.BoundedKernelTargetStructuralReview
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelLinkProofCandidateOfGates
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelFiberSaturatedTargetOfGates
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelLinkProofCandidateOfReview
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelFiberSaturatedTargetOfReview
N32Q12289CommittedSigmaByteFrontier.ReviewedBoundedKernelAccountLinkInstantiation
N32Q12289CommittedSigmaByteFrontier.accountCarriedBoundedKernelReviewedTarget_accepts_accountBacked_and_structurallyAdmissible
```

`BoundedKernelTargetStructuralGates.allHold` is exactly the triple still needed
after the bounded deployment obligations are supplied:

```text
postChallengeRepresentativeRejected
nonPublicProjectionOfSigmaEquation
witnessHidingArgument
```

The `OfGates` constructors derive the structurally admissible zero-link-byte
candidate and the 250-byte fiber-saturated target from one gates object.
The newer `BoundedKernelTargetStructuralReview` layer is the audit-facing
version of that object: each gate is stated as either true or charged to a
named break bucket:

```text
postChallengeRepresentativeBreak
publicProjectionBreak
witnessHidingBreak
```

Lean proves `BoundedKernelTargetStructuralReview.allHold_of_noBreaks`, and the
`OfReview` constructors then derive the same structurally admissible candidate
and 250-byte target from a no-structural-break hypothesis.  This is the cleaner
form for third-party review: the proof no longer asks reviewers to accept three
anonymous Props, but asks them to review the exact games that would falsify
representative rejection, non-public projection, or witness hiding.

The current global compiled theorem is:

```text
accountCarriedBoundedKernelReviewedTarget_accepts_accountBacked_and_structurallyAdmissible
```

It combines the bounded-kernel deployment obligations, structural review object,
and no-break hypotheses.  Given an accepting account-aware transcript, it proves
all three facts we need for the low-byte row:

```text
AccountBackedByHiddenWitness
structurallyAdmissible
transactionBytes = 250
```

The account-link instantiation itself is now bundled as a single review object:

```text
ReviewedBoundedKernelAccountLinkInstantiation
ReviewedBoundedKernelAccountLinkInstantiation.anyBreak
ReviewedBoundedKernelAccountLinkInstantiation.accepts_accountBacked_and_target_of_noBreak
ConcreteReviewedAccountCommitmentRepresentation
ConcreteReviewedAccountCommitmentRepresentation.accepts_accountBacked_and_target_of_noBreak
```

This object packages the pre-response backing system, bounded-kernel deployment
obligations, and structural review gates.  Its `anyBreak` predicate is exactly
the union of the three concrete account-link failure classes:

```text
preResponseBackingBreak
linkKernelBreak
structuralReview.breaks.any
```

Under `¬ anyBreak`, Lean derives `AccountBackedByHiddenWitness`, structural
admissibility, and `transactionBytes = 250` from a single reviewed
instantiation object.  This is the form the concrete account commitment
representation should now instantiate.

The concrete byte layout for that representation is now fixed as a Lean object:

```text
reviewedAccountWitnessCommitmentBytes    = 32
reviewedAccountRandomizerCommitmentBytes = 32
reviewedAccountResponseLinkCodeBytes     = 4
reviewedAccountHiddenLinkBytes           = 68
```

`ConcreteReviewedAccountCommitmentRepresentation` ties this 68-byte account
state layout to a `ReviewedBoundedKernelAccountLinkInstantiation`.  Its main
theorem keeps the spend-signature link bytes at zero and preserves the
250-byte transaction theorem under the reviewed no-break hypothesis.  These 68
bytes are account/state bytes; they are not added to the spend signature.

The parameter-search rerank now has a focused committed-sigma cost artifact:

```text
scripts/parameter_search/committed_sigma_cost_model.py
research/parameter_search/committed_sigma_account_carried_20260521/committed_sigma_account_carried_cost_model.json
research/parameter_search/committed_sigma_account_carried_20260521/committed_sigma_account_carried_cost_model.md
```

This script ranks verifier architectures only after applying the current proof
gates.  Its current Pareto frontier has one row:

```text
reviewed-account-carried-bounded-kernel-37b:
  transaction bytes = 250
  account/state bytes = 68
  proof bytes = 69
```

The admissible fallback is the 39-byte response variant at `252 B` transaction
bytes and the same 68 account/state bytes.  The old 41-byte public sigma row is
kept in the table as a negative control and fails the representative-rejection
and non-public-projection gates.

This moves the global proof surface forward: the bounded `[0,3]` certificate is
now connected all the way to the account-aware top-level certificate.  The
remaining concrete proof obligation is no longer the composition theorem; it is
the instantiation of a pre-response backing system, a
`BoundedKernelDeploymentObligations` object, and a
`BoundedKernelTargetStructuralReview` object for the selected account
commitment representation.

The immediate byte target remains "as small as reasonably possible" rather than
a hard 177-byte target.  A 250-byte signature is acceptable only if the proof
surface is structurally sound and the account-state overhead is blockchain
practical.

## Immediate Next Steps

1. Build `Descender.CommittedSigma`.  Done.
2. Instantiate the quotient-level `LinearSigmaAlgebra`.  First adapter done.
3. Design the smallest concrete byte instantiation of
   `PrechallengeRandomizerOpeningInterface`.
4. Add a new Rust path for `committed_sigma_reference` while preserving old
   representative paths as negative controls.
5. Replace the existing parameter-search ranking with a committed-sigma schema
   once item 3 is concrete enough to price.
