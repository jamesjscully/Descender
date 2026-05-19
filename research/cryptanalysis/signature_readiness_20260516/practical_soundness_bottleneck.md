# Practical Soundness Bottleneck

This note records the practical reading of the current byte-concrete N32
certificate.  It is a proof bottleneck, not a discovered attack.

## Current Formal Endpoint

For the leading compact row, Lean proves the contrapositive certificate

```text
(Pr[forgery] - 2^-133)^6
  <= N32CompactCoverage.failureUnionBound.
```

The concrete compact coverage bound is

```text
N32CompactCoverage.failureUnionBound ~= 2^-27.326561.
```

Taking the sixth root gives the direct residual advantage allowed by the
current theorem:

```text
Pr[forgery] <= 2^-133 + 2^-4.554429
            ~= 2^-133 + 0.04256.
```

So the proof is mathematically meaningful, but it is not a practical
128-bit soundness proof for the cryptographic system.

## Security Target Implication

With an `a`-replay moment proof, a `k`-bit direct forgery target requires
approximately `a * k` bits of coverage failure before taking the root.

| direct residual target | arity 6 coverage needed | arity 2 coverage needed | arity 1 coverage needed |
|---:|---:|---:|---:|
| 32 bits | 192 bits | 64 bits | 32 bits |
| 40 bits | 240 bits | 80 bits | 40 bits |
| 64 bits | 384 bits | 128 bits | 64 bits |
| 80 bits | 480 bits | 160 bits | 80 bits |
| 128 bits | 768 bits | 256 bits | 128 bits |

This is why tuning compact fork-delta coverage from 24 to 32 bits does not
solve the practical problem.  The leading `255 B` N32 row has about `27.33`
coverage bits, which is only about `4.55` direct proof bits after the current
sixth-root loss.

## What Must Change

The low-hanging improvement is not another filter on the existing parameter
search.  It is a change to the algorithm or proof interface that removes the
large moment loss or makes extraction efficient by design.

The main viable routes are:

1. Reduce extractor arity.  A two-fork proof would still need about `256`
   coverage bits for a 128-bit target, but that is qualitatively closer than
   `768`.  A linear/tight proof would need coverage at the target level.
2. Change the challenge/response construction so one or two accepting views
   force the needed block information.  The current compact active-only sparse
   signed challenge omits inactive coordinates, so ordinary pairwise deltas
   cannot deterministically cover every omitted block without either a larger
   response or a different coefficient/view design.
3. Weaken the extraction target in a cryptographically justified way.  If a
   partial product-block recovery already breaks key generation, the proof can
   target the useful block set instead of whole-secret recovery.  This must be
   justified by the keygen game, not assumed from the transcript proof.
4. Prove state-local accepting structure.  The current bound treats accepting
   sets as arbitrary and pays an `epsilon^6` moment.  A stronger verifier/game
   theorem could show that accepting mass is algebraically structured enough
   to extract with a linear or near-linear loss.

## Direct Incorporation

Lean now has the target wrapper

```text
N32ByteConcreteSignatureSecurityCertificate.PracticalForgeryTarget
N32ByteConcreteSignatureSecurityCertificate.forgerSuccess_le_guess_plus_target
```

This converts the existing sixth-moment certificate into

```text
Pr[forgery] <= 2^-133 + targetAdv
```

provided the compact coverage failure is at most `targetAdv^6`.

The sampler reports now include a `direct proof bits` column.  For the current
proof shape this is

```text
compact fork-delta bits / fork challenge draws.
```

Any future candidate should be judged by this direct column unless it supplies
a new theorem with a smaller arity or a different security reduction.

## Linear Target Theorem

The north-star theorem is now formalized as

```text
LinearStructuralSoundnessTarget
LinearStructuralSoundnessTarget.forgeryProb_le_queryMiss_add_structuralFailure_of_no_keygenBreaker
```

The target packages:

```text
traceDist
forgery
queryMiss
structuralFailure
extract : Trace -> Option Extracted
buildBreaker : Extracted -> Breaker
BreaksKeygen : Breaker -> Prop
```

and requires:

```text
Pr[queryMiss] <= queryMissBound
Pr[structuralFailure] <= structuralFailureBound
clean forgery => extracted keygen breaker
```

The resulting theorem is the practical shape:

```text
no keygen breaker
=> Pr[forgery] <= queryMissBound + structuralFailureBound.
```

This is the contract future algorithm changes should target.  The extracted
object is abstract on purpose: it can be a full secret, a product-block secret,
a partial keygen-breaking relation, or a verifier-specific serialized witness.
The proof does not care which one, as long as the keygen game validates that it
breaks key generation.

## Byte-Concrete North-Star Bridge

The N32 byte transcript now also has a concrete bridge to the linear target:

```text
N32ByteLinearStructuralSoundnessInputs
N32ByteLinearStructuralSoundnessInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
```

This bridge fixes the query-miss side using the existing serialized verifier
and canonical N32 challenge-guess proof.  A candidate scheme supplies:

```text
forgery
structuralFailure
extract : Trace -> Option Extracted
buildBreaker : Extracted -> Breaker
BreaksKeygen : Breaker -> Prop
Pr[structuralFailure] <= structuralFailureBound
forgery => serialized verifier accepts
queried forgery outside structuralFailure => extracted keygen breaker
```

Lean then proves:

```text
no keygen breaker
=> Pr[forgery] <= 2^-133 + structuralFailureBound.
```

This is the concrete north-star contract.  The next algorithmic work is to
design a byte-level verifier/challenge view where the final queried-forgery
extraction obligation is true with a cryptographically tiny structural failure
probability.

## Current Algebraic Obstruction

The current response equation is affine:

```text
z = mask + chi * secret.
```

Lean now proves the one-view ambiguity directly:

```text
productBlockLinearResponse_single_view_secret_ambiguous
```

For any observed single response and any candidate secret, there is a mask that
explains the response.  So the existing compact affine view cannot satisfy the
linear north-star extraction obligation from one public view unless the scheme
adds something that prevents this mask shift.

The matching positive target is:

```text
SingleViewLinearRound.extractedSecret_eq
```

If a clean transcript makes the mask available, then a single active signed
challenge is enough to recover that block's secret coordinates; active coverage
across repetitions recovers the whole product-block secret.  This identifies
the exact structural feature needed for a linear proof:

```text
clean queried forgery
=> mask/relevant pad is extractable or bound tightly enough
=> active coverage gives the keygen-breaking object.
```

A public signature that simply reveals the mask would leak the key on honest
signatures, so this is not a drop-in fix.  It is a design constraint: the next
candidate must obtain mask extractability only from the adversary/query
structure, or switch the extracted object to a smaller keygen-breaking witness
that an honest signature does not reveal.

## Implemented One-View Code Path

The concrete mask-extractable path is now implemented as:

```text
CompactSingleViewLinearRound
CompactSingleViewLinearRound.extractedSecret_eq
N32ByteKnownMaskSingleViewInputs
N32ByteKnownMaskSingleViewInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
```

`N32ByteKnownMaskSingleViewInputs` takes the byte verifier, the compact
response soundness proof, an extracted mask function, and two clean-forgery
obligations:

```text
clean forgery => extracted mask equals verifier-soundness mask
clean forgery => actual challenge has active signed coverage
```

It defines an actual extractor:

```text
extract : Trace -> Option (N32Block × LocalFree -> ZMod q)
```

and proves that, on clean queried forgeries, the extracted function is the
product-block secret.  Plugging that into the linear byte target gives:

```text
no keygen breaker
=> Pr[forgery] <= 2^-133 + structuralFailureBound.
```

The remaining gap is now algorithmic, not proof plumbing: construct a scheme
where the extracted mask equality and active coverage failure are both
cryptographically small without making honest signatures leak the secret.

## Partial-Witness Pivot

The main target is now partial witness/keygen break, not full product-block
secret recovery.  Lean exposes this as:

```text
N32PartialKeygenBreakGame
N32BytePartialWitnessLinearInputs
N32BytePartialWitnessLinearInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
```

`N32PartialKeygenBreakGame` packages:

```text
secret
ValidWitness : PartialWitness -> Prop
buildBreaker : PartialWitness -> Breaker
BreaksKeygen : Breaker -> Prop
ValidWitness witness => BreaksKeygen (buildBreaker witness)
```

The byte-level wrapper then proves:

```text
clean queried forgery => valid partial witness
no partial keygen breaker
=> Pr[forgery] <= 2^-133 + structuralFailureBound.
```

Full-secret recovery is now just one instantiation:

```text
N32PartialKeygenBreakGame.fullSecret
N32PartialKeygenBreakGame.ofSecretRecoveryGame
```

The first concrete partial witness type is also present:

```text
N32RecoveredBlockWitness
N32RecoveredBlockWitness.keygenBreakGame
N32RecoveredBlocksWitness
N32RecoveredBlocksWitness.keygenBreakGame
```

The one-block form is useful as a minimal interface test, but it is probably
too weak as a standalone 128-bit keygen break: with `LocalFree = Fin 2`, one
block contains only two field elements, roughly 26-30 bits after accounting for
the choice among 16 blocks.  `N32RecoveredBlocksWitness` adds a `minBlocks`
threshold, so a concrete keygen game can require enough fixed/admissible
product blocks to make guessing infeasible.  The stronger target appears to be
a public-view relation, local P-TND/preimage witness, or narrow-dual/certificate
violation that contradicts the keygen/public-view certificate directly.

The public-view relation endpoint now has proof objects too:

```text
PublicViewNarrowDualWitness
PublicViewNarrowDualWitness.impossible_of_certificate
EfficientPublicViewCertificate.noPublicViewNarrowDualWitness
SystematicLeeViolation
SystematicLeeViolation.impossible_of_systematicCertificate
SystematicLeeViolation.impossible_of_publicViewLeeDualCertificate
```

`PublicViewNarrowDualWitness` is the generic extracted object: an integer
relation `z` whose residue is a nontrivial public dual relation and whose
noise-amplitude interval is narrow modulo `q`.  Any such object contradicts
`NoNarrowQaryDualRelations`, hence any `PublicViewLeeDualCertificate` or
`EfficientPublicViewCertificate`.

`SystematicLeeViolation` is the structured specialization for `A = [I | P]`:
a nonzero free vector `y` such that the systematic relation `(-P y, y)` is
shorter than the certified Lee lower bound.  It contradicts either the raw
`SystematicLeeCertificate` or the serialized public-view Lee-dual certificate.
This is the cleanest current target for a partial-witness extractor because it
turns a clean forgery into a direct violation of the keygen/view certificate,
not into a large recovered secret.

The signature-side byte theorem is now instantiated for these targets in
`Descender.SignaturePublicViewWitness`:

```text
publicViewNarrowDualKeygenBreakGame
N32BytePublicViewNarrowDualInputs
N32BytePublicViewNarrowDualInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_certificate
N32BytePublicViewNarrowDualInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_efficientCertificate
N32BytePublicViewNarrowDualSecurityCertificate
N32ByteEfficientPublicViewNarrowDualSecurityCertificate

systematicLeeViolationKeygenBreakGame
ShortSystematicKernelRelation
ShortSystematicVerifierRelation
N32SystematicRelationByteParser
N32SystematicRelationByteParser.verifierChecks
N32ExplicitSystematicRelationByteDecoder
N32FixedOffsetSystematicRelationByteLayout
N32ContiguousSystematicRelationByteLayout
N32ContiguousSystematicRelationByteLayout.exact
N32ContiguousPackedSystematicRelationByteLayout
zmod12289Packed14Coeff
zmod12289Packed14RangeCheck
N32Q12289SystematicRelationLayout.target32x32
N32Q12289SystematicRelationLayout.target32x32Decoder
N32Q12289SystematicRelationLayout.target32x32SingleCarrierSignatureBytes
N32Q12289SystematicRelationLayout.Target32x32SingleCarrierVerifierInputs
N32Q12289SystematicRelationLayout.validated48x32
N32Q12289SystematicRelationLayout.validated48x32Decoder
N32Q12289SystematicRelationLayout.validated48x32SingleCarrierSignatureBytes
N32Q12289SystematicRelationLayout.Validated48x32SingleCarrierVerifierInputs
N32RelationCheckingByteVerifierInputs
N32SingleCarrierRelationPayloadVerifierInputs
N32ByteExplicitCheckedSystematicVerifierEquationSoundness
N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
N32ByteExplicitCheckedSystematicVerifierEquationSecurityCertificate
N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
N32ByteCheckedSystematicVerifierEquationSoundness
N32ByteCheckedSystematicVerifierEquationSoundnessInputs
N32ByteCheckedSystematicVerifierEquationSecurityCertificate
N32ByteCheckedSystematicPublicViewVerifierEquationSecurityCertificate
N32ByteSystematicVerifierEquationSoundness
N32ByteSystematicVerifierEquationSoundnessInputs
N32ByteSystematicVerifierEquationSoundnessSecurityCertificate
N32ByteSystematicPublicViewVerifierEquationSoundnessSecurityCertificate
N32ByteSystematicVerifierEquationInputs
N32ByteSystematicVerifierEquationSecurityCertificate
N32ByteSystematicPublicViewVerifierEquationSecurityCertificate
N32ByteSystematicLeeViolationInputs
N32ByteSystematicLeeViolationInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
N32ByteSystematicLeeViolationInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
N32ByteSystematicLeeViolationSecurityCertificate
N32ByteSystematicPublicViewLeeViolationSecurityCertificate
N32ByteShortSystematicRelationInputs
N32ByteShortSystematicRelationSecurityCertificate
N32ByteShortSystematicPublicViewRelationSecurityCertificate
```

So the remaining algorithmic obligation has the exact clean-forgery form:

```text
forgery
point queried
not structural failure
-----------------------
extracts PublicViewNarrowDualWitness
```

or, for systematic public keys:

```text
forgery
point queried
not structural failure
-----------------------
extracts SystematicLeeViolation
```

The systematic route has now been pushed one level closer to the concrete
verifier equation.  `ShortSystematicVerifierRelation` is the extracted
pivot/free object:

```text
pivot : Pivot -> ZMod q
free  : Free -> ZMod q
∀ p, pivot[p] + sum_f P[p,f] * free[f] = 0
free != 0
centeredNormL1(pivot, free) < leeMin
```

Lean proves this implies `SystematicLeeViolation` by converting the verifier
equation into a kernel relation and then into the canonical systematic relation
`(-P free, free)`.  The byte-level wrapper
`N32ByteShortSystematicRelationInputs` now lets the concrete scheme supply only
this pivot/free relation extractor.

The top-level systematic interface is now even closer to implementation:
`N32SystematicRelationByteParser` parses pivot/free coordinates from the seven
response byte strings and the canonical challenge, and
`N32ByteSystematicVerifierEquationInputs` asks the concrete verifier proof for
exactly three clean-transcript facts:

```text
verifier equation: pivot + P * free = 0
nonzero free part: free != 0
shortness: centeredNormL1(pivot, free) < leeMin
```

Its packaged endpoints
`N32ByteSystematicVerifierEquationSecurityCertificate` and
`N32ByteSystematicPublicViewVerifierEquationSecurityCertificate` prove the same
linear bound directly from those parser obligations plus the Lee certificate.

The accepted-transcript form is now packaged too.  In
`N32ByteSystematicVerifierEquationSoundness`, the parser facts are required
from the actual serialized verifier condition:

```text
oneShotAccepts(state, challenge) = true
not structural failure
---------------------------------
parser equation, parser nonzero, parser shortness
```

`N32ByteSystematicVerifierEquationSoundnessInputs` then combines that with the
standard first-run fact `forgery => outputAccepts`, uses
`oneShotAccepts_of_outputAccepts`, and derives the same linear security bound.
This is the closest current Lean interface to a concrete implementation.

The chosen implementation-facing route is now the checked form:
`N32SystematicRelationByteParser.verifierChecks` is a single Boolean verifier
check bundling the three required conditions:

```text
equationCheck && freeNonzeroCheck && shortCheck
```

`N32ByteCheckedSystematicVerifierEquationSoundness` only asks the concrete
verifier proof to show:

```text
oneShotAccepts(state, challenge) = true
not structural failure
---------------------------------
parser.verifierChecks response challenge = true
```

Lean then derives the equation, nonzero, and shortness facts automatically.

The concrete-layout refinement is
`N32ExplicitSystematicRelationByteDecoder`.  It fixes a carrier repetition for
the systematic relation bytes and provides `decodePivot` and `decodeFree` for
that one response byte string.  Its `toParser` map feeds the checked theorem,
so the lowest current proof obligation is:

```text
oneShotAccepts(state, challenge) = true
not structural failure
---------------------------------
decoder.toParser.verifierChecks response challenge = true
```

This keeps the low-level packing decision explicit without making the global
reduction depend on a particular bit layout.

The proof now includes direct byte arrangements for this route:
`N32FixedOffsetSystematicRelationByteLayout` slices fixed-width coefficient
chunks out of the carrier response, and
`N32ContiguousSystematicRelationByteLayout.exact` is the no-gap arrangement:

```text
pivot chunks || free chunks
responseLen = (pivotCount + freeCount) * bytesPerCoeff
```

`N32RelationCheckingByteVerifierInputs` then constructs the actual serialized
verifier predicate as:

```text
baseAccepts(transcript)
&& decoder.toParser.verifierChecks(P, leeMin, response, challenge)
```

Lean proves:

```text
oneShotAccepts(state, challenge) = true
--------------------------------------
decoder.toParser.verifierChecks response challenge = true
```

This is the most direct incorporation of the structural insight: make the
verifier relation check itself the public-view certificate-violation predicate.
The only remaining packing parameter is the coefficient codec used for each
fixed-width chunk.

For the `q = 12289` candidate, the efficient arrangement is bit-packed rather
than byte-chunked.  Lean now names
`N32ContiguousPackedSystematicRelationByteLayout`, using adjacent
`bitsPerCoeff`-bit words inside the carrier response.  The concrete coefficient
codec is:

```text
zmod12289Packed14Coeff
zmod12289Packed14RangeCheck
```

The range check is the canonical-encoding guard for unused 14-bit words above
`12288`; it should be conjoined into `baseAccepts` for a deployed verifier.
The named layouts are:

```text
N32Q12289SystematicRelationLayout.target32x32
  responseLen = 112 B
  Pivot = Fin 32, Free = Fin 32

N32Q12289SystematicRelationLayout.validated48x32
  responseLen = 140 B
  Pivot = Fin 48, Free = Fin 32
```

This keeps the `n=32, m1=32` signature target and the current validated
`n=48, m1=32` coordinate-subview profile side by side without changing the
global reduction theorem.

The serialized response now has a single-carrier wrapper:
`N32SingleCarrierRelationPayloadVerifierInputs`.  It expands one payload into
the proof interface by placing it at the decoder carrier and filling all other
repetitions with zero bytes.  This keeps the existing seven-response theorem
unchanged while charging only one serialized relation payload.

The concrete single-carrier signature sizes are now named:

```text
N32Q12289SystematicRelationLayout.target32x32SingleCarrierSignatureBytes
  17 B challenge + 112 B payload = 129 B

N32Q12289SystematicRelationLayout.validated48x32SingleCarrierSignatureBytes
  17 B challenge + 140 B payload = 157 B
```

The corresponding verifier-input aliases are
`Target32x32SingleCarrierVerifierInputs` and
`Validated48x32SingleCarrierVerifierInputs`.  Each directly converts to
`N32ByteExplicitCheckedSystematicVerifierEquationSoundness`; the canonical
product-block wrapper below fixes `baseAcceptsPayload` as core verifier
acceptance plus the payload range check.

The validated coordinate-subview candidate is now instantiated as a concrete
product-block public view:

```text
SharedRowProductBlock12289Rows3
q = 12289
eta = 12
targetLee = 512
Block = Fin 16
LocalPivot = Fin 3
LocalFree = Fin 2
local block =
  [[10849, 1171],
   [1597,  1615],
   [6102,  11189]]
```

The byte layout is:

```text
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.layout
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.decoder
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.rangeCheck
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.payloadRangeCheck
N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.baseAcceptsPayload
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockSingleCarrierVerifierInputs
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockCanonicalBaseVerifierInputs
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockCanonicalBaseVerifierInputs.toSingleCarrierVerifierInputs
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockFSCoreVerifierInputs
N32Q12289SystematicRelationLayout.Validated48x32ProductBlockFSCoreVerifierInputs.toCanonicalBaseVerifierInputs
```

This connects the 16 repeated local blocks directly to the 140-byte packed
single-carrier response.  The canonical base predicate is now concrete:

```text
baseAcceptsPayload = coreAcceptsPayload && payloadRangeCheck
```

so a verifier built through
`Validated48x32ProductBlockCanonicalBaseVerifierInputs` rejects noncanonical
14-bit field encodings before the relation proof is used.
The next implementation-facing layer is
`Validated48x32ProductBlockFSCoreVerifierInputs`, which defines
`coreAcceptsPayload` from:

```text
corePayloadCheck(point, challenge, payload)
&& recomputeChallenge(encodePoint(point)) == challenge
```

and proves the byte-frame binding and Fiat-Shamir recomputation obligations
needed by the canonical base wrapper.

The final packaged endpoints are:

```text
N32Q12289Validated48x32SingleCarrierSecurityCertificate
N32Q12289Validated48x32SingleCarrierSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32SingleCarrierSecurityCertificate.forgerSuccessProb_le_guess_add_structuralFailure

N32Q12289Validated48x32CanonicalBaseSecurityCertificate
N32Q12289Validated48x32CanonicalBaseSecurityCertificate.toSingleCarrierSecurityCertificate
N32Q12289Validated48x32CanonicalBaseSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32CanonicalBaseSecurityCertificate.forgerSuccessProb_le_guess_add_structuralFailure

N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.toCanonicalBaseSecurityCertificate
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.forgerSuccessProb_le_guess

N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.toCanonicalNoFailureSecurityCertificate
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.toPublicViewSecurityCertificate
N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.forgerSuccessProb_le_guess
```

and the theorem is the desired linear bound:

```text
Pr[forgery] <= 2^-133 + structuralFailureBound.
```

The remaining gaps are now concrete artifacts, not global proof plumbing.  The
3-by-2 local Lee certificate and q14 commitment packer are internalized in
Lean, and the exact Rust-style hash frame is now exposed as `q12289HashFrame`;
the production verifier certificate is derived from
`ProductionHashFrameOracleAgreement`.  What remains is to instantiate that
SHA-256-to-challenge/query-log agreement for the actual serialized verifier and
bound the declared structural-failure event.

The local Lee item and no-failure wrapper are now discharged at the current
proof boundary:

```text
SharedRowProductBlock12289Rows3.boundedCenteredPairCheck_eq_true
SharedRowProductBlock12289Rows3.internalLocalLeeCertificate
SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower

q12289_m132_target512_rows_per_group3.verify.json
  verifier passes target: true
  artifact consistent: true
  checked centered pairs: 523264
  independent min l1: 533

N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
  structuralFailure := false
  structuralFailureBound := 0
  Pr[forgery] <= 2^-133
```

Items 2 and 3 are now isolated to the FS-core implementation fields: formalize
the concrete byte `corePayloadCheck`, prove the implemented
`recomputeChallenge` frame matches the Lean oracle, and prove accepted
payloads agree with the query-log challenge.  Byte-frame binding is derived
from the fixed `encodePoint` frame used by the FS-core wrapper.  The
canonical wrapper has already conjoined the payload range check, and the
relation-checking wrapper has already conjoined the systematic equation,
nonzero, and shortness checks.

Item 5 is recorded in:

```text
q12289_validated_signature_profile.json
q12289_validated_signature_profile.md
```

Current byte benchmark:

```text
public key = 172 B
single-carrier proof payload signature = 157 B
implicit-commitment signature = 269 B
challenge = 17 B
relation payload = 140 B
```

The 269-byte implicit-commitment verifier prototype now fixes the practical
signature byte frame: 17 bytes of challenge plus seven completed 36-byte
response chunks, with `t' = A*z - chi*u` reconstructed before hashing.
Lean now has the matching serialized verifier bridge:

```text
N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs
N32ImplicitCommitmentByteVerifierInputs.outputAccepts_implies_outputChallenge_eq_actual
N32Q12289ImplicitCommitmentByteVerifier.Inputs
N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck
N32Q12289ImplicitCommitmentByteVerifier.implicitCommitmentDomainTag
N32Q12289ImplicitCommitmentByteVerifier.responseView
N32Q12289ImplicitCommitmentByteVerifier.reconstructedCommitmentView
N32Q12289ImplicitCommitmentByteVerifier.productBlockEquationSoundness
N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder
N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate
N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts
N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame
N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length
N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate
N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts
N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement
N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.toProductionHashFrameVerifierCertificate
N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.verifierEquation_of_oneShotAccepts
SharedRowProductBlock12289Rows3.boundedCenteredPairCheck_eq_true
SharedRowProductBlock12289Rows3.internalLocalLeeCertificate
SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower
N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate
N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate.verifierEquation_of_oneShotAccepts
N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_acceptsTranscript
N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_oneShotAccepts
```

The new q12289 layer fixes the concrete range/parser predicates for the
implicit completed-response verifier and exposes the exact bridge theorem:
accepted byte transcripts imply the product-block equation.  The concrete
certificate now fixes the domain tag and `corePayloadCheck`.  The concrete
`commitmentViewEncoder` makes reconstruction correctness a q14-packer theorem:
decoding the encoded view recovers the requested coordinates, so
`EncodedReconstructionVerifierCertificate` can use it to supply the
`A*z - chi*u` reconstruction field.  The
`HashFrameEncodedReconstructionVerifierCertificate` then fixes the frame sent to
SHA-256: length-prefixed labels, concrete profile/domain bytes, scalar
parameters, public key, message, and the seven reconstructed commitments.  The
implementation side still supplies only `ProductionHashFrameOracleAgreement`,
the SHA-256-to-challenge/query-log agreement over that pinned hash frame.

With those verifier artifacts supplied, the internal public-view Lee certificate
rules out the extracted short relation, giving the linear bound
`Pr[forgery] <= 2^-133 + structuralFailureBound`.

The formal theorem skeleton is therefore complete at the reduction boundary:
Lean has the final packaged certificates and the final probability inequality.
The cryptographic scheme proof is not complete yet.  The missing theorem is not
a probability lemma; it is now a concrete implementation-coherence/extractor
obligation:

```text
concrete accepted byte transcript
serialized verifier equation
query binding
not structural failure
decoded reconstructed commitment = A*z - chi*u
--------------------------------
construct PublicViewNarrowDualWitness
```

or the systematic specialization:

```text
concrete accepted byte transcript
serialized verifier equation over A = [I | P]
query binding
not structural failure
--------------------------------
instantiate N32ByteExplicitCheckedSystematicVerifierEquationSoundness
```

This forced the next algorithmic pivot.  The honest signable relation is now
formalized separately from the certificate-violation verifier: the signer
samples a mask, publishes the product-block commitment `t = A * mask`, answers
with `z = mask + chi * secret`, and the verifier checks
`A * z = t + chi * u` for public key `u = A * secret`.  Lean proves the honest
completeness equation and bridges repeated accepting honest equations into the
existing product-block multi-fork extractor.

The compact active-only variant is also formalized.  For each signed active
block, the signature carries only that block's free response coordinates; the
verifier checks the local systematic pivot/free equations against the public
commitment and public key.  Lean converts this compact honest verifier
equation into `CompactMultiForkLinearRound`, so opposite signed active forks
still recover the product-block secret.

Current candidate ranking:

1. Public-view relation / narrow-dual violation: best formal target, because it
   can contradict existing public-view certificates directly.
2. Multi-block local preimage/partial secret: plausible if the keygen game
   explicitly treats recovery of enough fixed/admissible blocks as a break.
3. One-block secret: useful test case, likely too weak alone.
4. Known-mask full recovery: formally complete but likely too costly or leaky
   unless mask extractability comes only from adversarial query structure.

Subagent consensus matched this ranking:

- one recovered block is formalizable but low entropy;
- multi-block local preimage recovery is the plausible partial-secret target;
- public-view relation/certificate violation is the cleanest keygen-break
  target if a clean forgery can be made to yield such a relation;
- Lee/narrow-dual witnesses connect well to existing public-view certificates,
  while the honest affine signing relation now gives the separate, signable
  route whose forks recover the product-block secret.
