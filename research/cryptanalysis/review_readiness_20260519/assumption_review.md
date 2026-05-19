# Assumption Review

Date: 2026-05-19

Status: internal review artifact.  This document is intentionally stricter
than the current proof package.  Its purpose is to identify every assumption
that would matter for a deployable cryptographic claim and to say what must be
true before third-party review.

## Executive Conclusion

The proof boundary is now explicit enough to review: accepted serialized
transcripts are connected to the q12289 product-block verifier equation through
the concrete q14 commitment encoder and the concrete q12289 hash frame.  The
main formal endpoint is a no-structural-failure bound of
`n32ChallengeGuessBound = 2^-133`, conditional on the implementation-facing
certificate fields supplied to the theorem.

That is not yet the same thing as deployable cryptography.  The remaining
assumptions are now narrow, but several are still security-critical:

- the production SHA-256 challenge map must be treated as a random oracle on
  the exact `q12289HashFrame`;
- key generation must only emit public views for which the relevant short
  public-view witness does not exist, or must provide a certificate/rejection
  mechanism that makes this true by construction;
- the concrete signer distribution must be proven secret-independent after
  rejection sampling, including any retry behavior that an attacker can observe;
- the Rust byte verifier and eventual production implementation must be shown
  to implement the Lean byte predicates exactly; the frozen challenge layout is
  now internalized in Lean, but independent conformance/CI remains open.

The current state is therefore "clean proof boundary, not yet deployable."  A
third-party reviewer can now be handed precise assumptions, but those
assumptions still need cryptanalysis, independent conformance/CI, and
implementation hardening before a deployment claim should be made.

## Claim Under Review

The strongest current proof-side claim is best read as:

> In the q12289 random-oracle/query-log model, for the frozen verifier target,
> if the supplied implementation-facing verifier certificate is valid, if
> successful forgeries are accepted by that verifier, if structural failure is
> zero because every structural condition is an explicit verifier check, and if
> the public view has the internalized Lee-dual certificate, then the accepted
> forgery probability is at most `2^-133`.

This is not yet a complete EUF-CMA theorem for a deployed signature scheme
because key generation, signing-oracle distribution, randomness, and
implementation conformance are not all instantiated as closed artifacts.

## Dependency Boundary

The current proof stack separates into the following layers.

| layer | current status | primary endpoints |
|---|---|---|
| finite challenge space | proved | `N32CanonicalChallenge`, `n32CanonicalChallenge_card`, `n32ChallengeGuessBound` |
| q14 reconstructed-commitment encoder | proved | `commitmentViewEncoder`, `commitmentView_encodeCommitmentView`, `zmod12289Packed14Coeff_encodeCommitmentView` |
| q12289 hash frame | proved | `q12289HashFrame`, `q12289HashFrame_length`, `q12289HashFrame_fixtureMessage_length` |
| accepted transcript to algebraic verifier equation | proved conditional on hash/oracle agreement | `ProductionHashFrameOracleAgreement.verifierEquation_of_oneShotAccepts` |
| byte parser to systematic relation | proved once verifier inputs are supplied | `N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs.*` |
| local/global Lee certificate | proved for the fixed shared-row q12289 structure | `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate`, `internalLocalLeeCertificate_lower` |
| no-structural-failure final wrapper | proved if the verifier certificate makes structural failure false | `N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.forgerSuccessProb_le_guess`, `N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.forgerSuccessProb_le_guess` |
| production cryptographic scheme | incomplete | requires keygen, signer distribution, conformance, side-channel and RNG review |

This review classifies assumptions at the layer where they enter.  It does not
claim to be an automated Lean import-dependency audit.

## Assumption Status Table

| id | assumption | severity | status | blocks deployment? |
|---|---|---:|---|---|
| A0 | Lean/toolchain and generated-artifact integrity | medium | acceptable review assumption with reproducibility checks | yes, until checks are archived |
| A1 | random-oracle/SHA-256 challenge agreement | high | explicit cryptographic assumption | no, if the claim is clearly ROM |
| A2 | query-log model matches the production Fiat-Shamir game | high | partially formalized, needs final theorem wording | yes |
| A3 | canonical challenge byte codec matches production bit layout | high | closed for frozen layout | no, unless the format changes |
| A4 | keygen emits no bad public-view witness | critical | largest mathematical gap | yes |
| A5 | structural-failure probability is actually zero | high | proved only if all failures are verifier checks | yes |
| A6 | signer/rejection sampler distribution is secret-independent | critical | abstract theorem exists, concrete instantiation open | yes |
| A7 | byte-level verifier conformance | high | prototype fixtures exist, full suite open | yes |
| A8 | implementation randomness and side-channel behavior | high | hardening plan only | yes |
| A9 | concrete cryptanalysis/security estimate | critical | plan exists, report open | yes |
| A10 | scope exclusions are honest and explicit | medium | target statement written, final public wording open | yes |

## A0: Toolchain And Artifact Integrity

Statement:

The formal claims are trusted only to the extent that the Lean toolchain,
Mathlib dependency set, generated profile artifacts, and checked source tree are
the ones reviewed.

Current evidence:

- `Descender` currently contains no `sorry`, `admit`, `axiom`, or `unsafe`
  occurrences found by a source search.
- The target profile records the concrete theorem endpoints and byte sizes.
- The review package has a manifest naming the primary Lean file and generated
  profile.

Risk:

This is not a cryptographic assumption, but it is review-critical.  A reviewer
must be able to reproduce the same theorem names and the same generated
q12289 profile from a pinned commit.

Closure criteria:

- archive the exact commit hash;
- archive `lake build Descender` output;
- archive the generated q12289 profile and verifier fixture outputs;
- fail CI on `sorry`, `admit`, new axioms, or unsafe Lean code in the proof
  boundary unless explicitly reviewed.

Acceptability:

Acceptable for third-party review once reproducibility artifacts are archived.

## A1: Random-Oracle / SHA-256 Challenge Agreement

Statement:

For every accepted production transcript, the production hash-to-challenge
function obtained by applying SHA-256 to the exact `q12289HashFrame` must agree
with the query-log random-oracle challenge used by the proof.

Lean endpoint:

- `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement`
- `ProductionHashFrameOracleAgreement.toProductionHashFrameVerifierCertificate`
- `ProductionHashFrameOracleAgreement.verifierEquation_of_oneShotAccepts`

What Lean proves around it:

- the q12289 hash frame is a concrete length-prefixed byte string;
- its length is `1508 + messageLen`;
- the concrete q14 commitment encoder round-trips through the q12289 decoder;
- if the production hash-frame agreement record is supplied, accepted one-shot
  transcripts imply the product-block verifier equation.

What is not proved:

- SHA-256 is not proven to be a random oracle;
- the standard model security of the scheme is not proven;
- collision resistance alone is not the modeled property.  The proof needs the
  Fiat-Shamir challenge to behave like an independent random-oracle output at
  the transcript point.

Risk:

This is a normal Fiat-Shamir assumption, but it must be stated exactly.  A vague
"SHA-256 is secure" assumption is too weak because the proof requires the
challenge map, domain separation, and byte frame to match the modeled oracle.

Closure criteria:

- final theorem states the random-oracle-model claim explicitly;
- the hash domain is frozen as
  `Descender.Signature.q12289.implicit_commitment.v1`;
- the production hash function is fixed as SHA-256 over `q12289HashFrame`,
  followed by the canonical 133-bit challenge extraction;
- test vectors include the full hash frame bytes, SHA-256 digest, and challenge
  bytes for positive and negative cases.

Acceptability:

Acceptable for a ROM security claim.  Not acceptable for a standard-model
claim.

## A2: Query-Log Model Matches The Fiat-Shamir Game

Statement:

The real adversary game must correspond to the finite query-log experiment used
by the proof: a forgery either queried the exact challenge point, or it is
charged to the blind challenge-guess event bounded by `2^-133`.

Lean endpoints:

- `N32QueryLogExperiment.pointQueried`
- `N32QueryLogExperiment.blindGuess_acceptProb`
- `N32QueryLogExperiment.QueryMissCertificate.queryMiss_acceptProb_le`
- `N32CanonicalSerializedVerifierInputs.queryMiss`
- `N32CanonicalSerializedVerifierInputs.toForkingInputs`
- `n32_single_challenge_guess_acceptProb`
- `n32_challenge_guess_family_acceptProb_le`

What Lean proves:

- `Fintype.card N32CanonicalChallenge = 2^133`;
- a single blind challenge guess succeeds with probability exactly `2^-133`;
- query-miss events that output the blind challenge are bounded by the same
  challenge-guess term;
- accepted outputs are reduced to one-shot accepting transcripts.

Risk:

The proof is sound only for the Fiat-Shamir game expressed by the query-log
model.  The final scheme statement must define oracle queries, message
selection, signing queries, and forgery outputs in a way that maps cleanly to
this model.

Closure criteria:

- use `target_theorem.md` as the final theorem target and point each game event
  to the corresponding Lean object;
- explicitly account for multiple hash queries if the final game permits more
  than one candidate challenge point per forgery;
- ensure the message, public key, profile, commitments, response, and challenge
  fields used by the production verifier are exactly those modeled by the
  query-log point.

Acceptability:

Acceptable after the final theorem statement is written.  Until then this is a
proof-scoping assumption.

## A3: Canonical Challenge Byte Codec

Statement:

The 17 challenge bytes in the signature must represent exactly the same
`N32CanonicalChallenge` space used by Lean: seven repetitions, each with one
omitted block among 16 and signs on the 15 active blocks, for exactly 133 bits
of entropy and three zero slack bits.

Lean endpoints:

- `N32CanonicalChallenge`
- `N32RoundChallenge`
- `n32CanonicalChallenge_card`
- `N32CanonicalChallengeByteCodec.packed17`
- `N32CanonicalChallengeByteCodec.packedDecode_encode`
- `N32CanonicalChallengeByteCodec.productionBitLayout17`
- `N32CanonicalChallengeByteCodec.productionDecode_encode`
- `N32CanonicalChallengeByteCodec.productionEncode_injective`

Production endpoint:

- `decode_challenge`
- `encode_challenge`
- `hash_frame_to_challenge_bytes`
  in `tools/parameter-search/src/bin/signature_q12289_implicit_verifier.rs`

Current status:

The Lean `packed17` codec is proof-facing and uses a finite equivalence to pack
the challenge space into 17 bytes.  The Rust verifier uses a concrete
little-endian bit layout:

- four omitted-block bits per repetition;
- fifteen sign bits per repetition;
- three zero high bits in the final byte.

That concrete production layout is now internalized as
`N32CanonicalChallengeByteCodec.productionBitLayout17`.  Lean proves that the
production decoder accepts `encode(c)` and recovers `c`, that production
encoding is injective, and that nonzero slack bits are rejected.  The review
package also includes `q12289_conformance_vectors.json`, generated by the Rust
verifier, with one positive full hash-frame fixture and six negative reject
vectors.

Risk:

Lower for the frozen format.  A future challenge-codec or bit-order change
would not necessarily break the abstract proof, but it would break the deployed
claim unless the Lean production codec and conformance vectors were regenerated.

Closure criteria:

- keep `productionBitLayout17`, `productionDecode_encode`, and
  `productionEncode_injective` building;
- regenerate `q12289_conformance_vectors.json` from the Rust verifier whenever
  the verifier/profile/hash frame changes;
- add CI that fails on challenge-codec or hash-frame drift;
- add an independent implementation cross-check against the generated vectors.

Acceptability:

Acceptable for the frozen challenge layout.  Remaining implementation
cross-check work is tracked under A7.

## A4: Keygen / No-Bad-Public-View Witness

Statement:

Key generation must not emit a public view admitting the short/narrow witness
that the extraction proof turns a successful clean forgery into.

Lean endpoints:

- `PublicViewNarrowDualWitness`
- `SystematicLeeViolation`
- `EfficientPublicViewCertificate.noPublicViewNarrowDualWitness`
- `SystematicLeeViolation.impossible_of_publicViewLeeDualCertificate`
- `N32ByteSystematicVerifierEquationInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate`
- `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate`
- `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower`
- `N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.forgerSuccessProb_le_guess`
- `N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate.forgerSuccessProb_le_guess`

What Lean proves:

For the fixed shared-row q12289 product-block matrix, the internal local Lee
certificate composes into the public-view Lee-dual certificate used by the
final wrappers.  Clean accepted transcripts cannot produce a short systematic
Lee violation below the certified threshold.

What remains open:

The deployed key generation procedure must be pinned.  If keygen samples or
derives public-view material from seeds, it must either:

- generate only the certified fixed/repeated structure;
- verify and reject until the emitted structure satisfies the required
  certificate; or
- be backed by a concrete keygen security theorem saying the bad witness occurs
  with negligible probability.

Risk:

Critical.  This is the largest mathematical risk.  If keygen can emit a bad
public view, the verifier proof can remain true for its stated certificate
while the deployed key distribution is insecure.

Attack surface:

- lattice-reduction search for short public-view dual vectors;
- meet-in-the-middle search exploiting the repeated `3 x 2` local block;
- attacks on seed expansion or matrix generation correlations;
- attacks that exploit the view reduction from full coordinates to the 172-byte
  canonical public key;
- special-purpose searches for `SystematicLeeViolation` rather than generic
  short vectors.

Closure criteria:

- freeze the exact keygen algorithm;
- state whether the keygen target is deterministic certified structure,
  certificate-checked random structure, or distributional no-bad-vector;
- archive failed attack scripts and outputs for the frozen target;
- produce a concrete table of best known attack costs, memory, and confidence;
- if relying on rejection/certification, make keygen rejection conditions
  deterministic, testable, and included in the public-key validity rules.

Acceptability:

Not acceptable as a loose assumption for deployment.  It can be reviewed as a
specific mathematical hardness/keygen assumption, but it must be the first item
attacked.

## A5: Zero Structural Failure

Statement:

The no-failure wrapper is valid only if every condition that could otherwise be
charged to `structuralFailure` is enforced by the verifier, so the structural
failure event is literally false.

Lean endpoints:

- `N32Q12289Validated48x32CanonicalBaseSecurityCertificate`
- `N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate`
- `N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.falseStructuralFailure_acceptProb_le_zero`
- `N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.forgerSuccessProb_le_guess`
- `Validated48x32ProductBlockCanonicalBaseVerifierInputs.payloadRangeCheck_of_baseAccepts`
- `Validated48x32ProductBlockFSCoreVerifierInputs.toCanonicalBaseVerifierInputs`

Current evidence:

The formal no-failure wrapper sets `structuralFailure := fun _ => false` and
proves its probability is zero.  The canonical-base verifier conjoins payload
range checking with the core predicate.  The FS-core verifier conjoins payload
checking with recomputed challenge equality.

Risk:

High.  This is valid only if the production verifier actually rejects every
noncanonical or structurally invalid transcript:

- wrong public-key or signature length;
- noncanonical public-key q-coordinate;
- noncanonical challenge high bits;
- out-of-bound signed response coordinate;
- malformed payload/range encoding;
- challenge mismatch;
- alternate transcript frame/domain/profile encodings.

Closure criteria:

- enumerate every structural condition in the final spec;
- add a negative vector for each rejected condition;
- ensure `corePayloadCheck` in Lean and the production verifier agree on every
  condition;
- do not permit "implementation-defined" parsing behavior.

Acceptability:

Acceptable only after byte-level conformance is complete.  Until then the
formal zero-failure result is a conditional theorem, not a deployment fact.

## A6: Signer Distribution And Rejection Sampling

Statement:

The signing algorithm must produce transcripts whose observable distribution is
independent of the secret except through the public key and the message being
signed.  Rejection sampling must erase the secret-dependent shape of
`z = y + chi * s`.

Lean endpoints:

- `RejectionSamplingCertificate`
- `RejectionSamplingCertificate.conditionalAcceptedProb_eq_target`
- `RejectionSamplingCertificate.secret_independent`
- `RejectionSamplingCertificate.challenge_independent`
- `RejectionSamplingCertificate.encoded_secret_independent`

Current evidence:

The abstract rejection-sampling theorem is strong: if the certificate is
instantiated, any deterministic encoding or compact transcript projection of
accepted responses is secret-independent by data processing.

What remains open:

- concrete mask distribution;
- concrete acceptance predicate;
- acceptance probability;
- retry cap behavior;
- whether retry count or timing is observable;
- deterministic test mode versus production randomness mode;
- proof that the actual serialized signature distribution matches the target
  distribution used in the security game.

Risk:

Critical.  Earlier analysis found that naive small-mask linear transcripts can
leak secrets quickly.  The rejection sampler is therefore not an optimization;
it is part of the security proof.

Closure criteria:

- define the concrete signer relation and sampler;
- instantiate `RejectionSamplingCertificate` for that sampler;
- either hide retry counts/timing or include them in the leakage model and
  prove they do not reveal reusable information;
- include deterministic test vectors and randomized distribution tests;
- state a hard failure behavior for excessive retries.

Acceptability:

Not acceptable as an assumption for deployment.  It can remain as a formal
obligation during internal proof development, but must be closed before
third-party review.

## A7: Byte-Level Verifier Conformance

Statement:

The production verifier must implement the Lean byte-level verifier semantics:
same public-key decoding, same response decoding, same commitment
reconstruction, same q14 commitment encoder, same hash frame, same challenge
codec, same accept/reject predicate.

Lean endpoints:

- `N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck`
- `N32Q12289ImplicitCommitmentByteVerifier.responseView`
- `N32Q12289ImplicitCommitmentByteVerifier.reconstructedCommitmentView`
- `N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder`
- `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame`
- `HashFrameEncodedReconstructionVerifierCertificate`

Prototype implementation:

- `tools/parameter-search/src/bin/signature_q12289_implicit_verifier.rs`

Current evidence:

The Rust verifier has unit tests for the challenge codec, response bounds,
q-coordinate canonicality, q14 reconstructed commitment packing, hash-frame
length, and a pinned SHA-256 hash-frame fixture.  It can now emit
`q12289_conformance_vectors.json`, a generated vector bundle containing the
positive zero-public-key fixture, the full hash frame, the SHA-256 digest, the
canonical challenge bytes, reconstructed commitments, and six negative reject
cases.

Risk:

High.  A single byte-order mismatch can invalidate the deployed claim even if
the Lean theorem is correct.

Closure criteria:

- keep the canonical JSON/raw-hex vector bundle checked in and regenerated from
  the verifier;
- extend the negative vector bundle until every parser rejection is represented;
- independent implementation cross-check;
- CI gate that fails on hash-frame drift or codec drift;
- final production code must share vectors with the proof-facing prototype.

Acceptability:

Not acceptable as a hand-waved assumption.  It is an engineering obligation
that can be made reviewable with test vectors and, ideally, generated
certificates.

## A8: Randomness, Constant Time, And Secret Handling

Statement:

Production keygen and signing must use reliable randomness and must not leak
secret information through timing, memory access, retry counts, panics, logs, or
partial outputs.

Current proof status:

No side-channel theorem is claimed.  No RNG theorem is claimed.

Risk:

High for deployment.  The mathematical proof can be correct while an
implementation leaks the signing secret through rejection timing, malformed
error paths, or memory access patterns.

Closure criteria:

- define RNG interfaces and failure behavior;
- use deterministic test mode only under explicit test configuration;
- review secret-dependent branches and memory indices in keygen/signing;
- zeroize secret material where supported;
- document side-channel exclusions if constant-time proof is out of scope;
- add fuzzing for parser code and negative vectors for malformed inputs.

Acceptability:

Acceptable only as a clearly scoped implementation assumption for research
review.  Not sufficient for deployment without an implementation audit.

## A9: Concrete Cryptanalysis And Security Estimate

Statement:

The final security level must be supported by adversarial analysis of the
frozen target, not just by the formal `2^-133` challenge-guess term.

Why this matters:

The `2^-133` term bounds the query-miss/blind-challenge route once the theorem
inputs are satisfied.  It does not by itself estimate:

- keygen/public-view witness attacks;
- direct lattice attacks on the public key;
- multi-signature leakage attacks;
- transcript malleability;
- implementation parser attacks;
- side-channel attacks.

Risk:

Critical.  For deployment, a proof with an unexamined bespoke keygen assumption
is not enough.  The frozen q12289 target needs a concrete failed-attack report.

Closure criteria:

- run bounded dual/witness searches against the frozen q12289 structure;
- run lattice-reduction and meet-in-the-middle experiments;
- document transcript malleability and noncanonical encoding attacks;
- test repeated-message and repeated-randomness misuse cases;
- produce a table with attack class, best known cost, memory, data, and
  confidence;
- document estimator assumptions and caveats.

Acceptability:

Not acceptable as complete until the cryptanalysis report exists.  This is a
required third-party-review input.

## A10: Scope Of The Final Claim

Statement:

The final public claim must not imply more than the package proves.

Currently excluded:

- standard-model proof for SHA-256;
- side-channel security proof;
- third-party cryptanalysis;
- optimized implementation security;
- compatibility with future parameter or byte-format changes;
- security under alternate keygen distributions;
- security under alternate hash domains or transcript frames.

Risk:

Medium.  This does not weaken the theorem, but it can make the research project
misleading if the deployed claim is written too broadly.

Closure criteria:

- keep `target_theorem.md` aligned with the proof and final implementation;
- label it ROM, frozen q12289 target, exact byte verifier;
- distinguish "proved", "assumed", "empirically attacked", and "out of scope";
- include reviewer questions that aim at the highest-risk assumptions.

Acceptability:

Acceptable once the final theorem statement and reviewer guide are written.

## Deployment Blockers

The following items block any claim that the system is deployable
cryptography:

1. freeze and review the keygen algorithm;
2. close or sharply state the no-bad-public-view/keygen assumption;
3. instantiate the concrete signer/rejection sampler proof;
4. add CI and independent cross-checks for the Lean/Rust challenge codec;
5. extend the byte-level conformance vector suite to every parser rejection;
6. archive the cryptanalysis report for the frozen target;
7. complete implementation hardening for randomness and side channels;
8. map the `target_theorem.md` game events to the exact Lean dependency graph.

## Third-Party Review Readiness

This assumption review is sufficient to guide external review, but the package
is not yet ready to send as a completed cryptographic claim.  The right next
step is not parameter optimization.  It is assumption closure:

- make keygen and signing concrete;
- connect their concrete behavior to the Lean objects;
- attack the exact frozen target;
- preserve the current byte format unless a security finding forces a change.
