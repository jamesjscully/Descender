# Theorem Dependency Audit

Date: 2026-05-19

This audit classifies the q12289 proof endpoints named in
[theorem_map.md](theorem_map.md). It is the endpoint-by-endpoint annex for
[assumption_ledger.md](assumption_ledger.md): every nontrivial dependency used
by those endpoints is classified as one of:

- `proved-in-Lean`
- `cryptographic assumption`
- `implementation/conformance assumption`
- `generated artifact/toolchain assumption`
- `out-of-scope`

This is a dependency classification audit, not an automated import graph.
“Dependency” here means “review-relevant premise needed to treat the endpoint
as part of the frozen deployable story.”

## Byte Profile And Verifier Boundary

| endpoint | proved-in-Lean | cryptographic assumption | implementation/conformance assumption | generated artifact/toolchain assumption | out-of-scope |
|---|---|---|---|---|---|
| `N32CompletedResponseSignatureByteProfile.leadingN32_9bit_2free_implicitCommitment` | The profile record and its equalities are definitional facts in `Descender/SignatureTranscript.lean`. | none | If used as a deployment claim, the implementation must actually emit and accept this `269 B` signature shape. | Reviewers still need the generated profile artifact in the frozen package to match the checked source tree. | Does not prove anything about signer distribution or security. |
| `N32Q12289ImplicitCommitmentByteVerifier.serializedSignatureBytes_eq` | `serializedSignatureBytes = 269` is proved by `native_decide`. | none | The deployed verifier and serializer must use the same `17 + 7*36` layout. | Trusted only for the reviewed Lean toolchain/source revision. | Does not show parser acceptance, only byte count. |
| `N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs` | The wrapper from the implicit verifier interface to the generic serialized verifier is proved in Lean. It derives frame binding and challenge agreement from the fields already supplied to `N32ImplicitCommitmentByteVerifierInputs`. | None in the generic theorem itself. At the q12289 endpoint, the supplied `oracleChallenge_matches_actual` field is later discharged only by `ProductionHashFrameOracleAgreement`. | The supplied `corePayloadCheck`, `reconstructCommitment`, and `recomputeChallenge` functions must match the production verifier semantics if this wrapper is used for deployment claims. | Trusted only for the reviewed Lean toolchain/source revision. | Does not prove SHA-256 security or signer behavior. |

## Concrete Parsing And Reconstruction

| endpoint | proved-in-Lean | cryptographic assumption | implementation/conformance assumption | generated artifact/toolchain assumption | out-of-scope |
|---|---|---|---|---|---|
| `N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck` | The concrete predicate is defined in Lean as `pointRangeCheck && responseRangeCheck`. | none | The Rust/prod verifier must reject exactly the same malformed public keys, commitments, and responses. | Trusted only for the reviewed Lean toolchain/source revision. | Does not prove that the implementation actually uses it. |
| `N32Q12289ImplicitCommitmentByteVerifier.responseView` | The response-byte-to-`ZMod 12289` view is defined in Lean via `signed9Coeff`. | none | The implementation must decode signed 9-bit coefficients with the same endianness and bound conventions. | Trusted only for the reviewed Lean toolchain/source revision. | Does not prove signer-side response generation. |
| `N32Q12289ImplicitCommitmentByteVerifier.reconstructedCommitmentView` | The algebraic reconstructed view `A*z - chi*u` is defined in Lean using `SharedRowProductBlock12289Rows3.localMatrix`. | none | The implementation must reconstruct commitments from the same public-key decode, challenge object, and response decode. | The fixed local matrix coefficients come from the reviewed `SharedRowProductBlock12289Rows3` constants, so the checked source tree and any generating search artifact must agree. | Does not prove the q14 byte encoding or hash step by itself. |

## q14 Commitment Encoder

| endpoint | proved-in-Lean | cryptographic assumption | implementation/conformance assumption | generated artifact/toolchain assumption | out-of-scope |
|---|---|---|---|---|---|
| `N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder` | The concrete encoder is the Lean record `encode := encodeCommitmentView`, `decode_encode := commitmentView_encodeCommitmentView`. | none | Production commitment packing must use the same q14 coefficient order and little-endian bit layout. | Trusted only for the reviewed Lean toolchain/source revision. | Does not show the implementation uses this encoder. |
| `N32Q12289ImplicitCommitmentByteVerifier.commitmentView_encodeCommitmentView` | Lean proves `commitmentView (encodeCommitmentView view) = view`. | none | The deployment claim still needs encoder/decoder conformance in code and vectors. | Trusted only for the reviewed Lean toolchain/source revision. | Does not prove injectivity beyond the defined decode path. |
| `N32Q12289ImplicitCommitmentByteVerifier.zmod12289Packed14Coeff_encodeCommitmentView` | Lean proves each decoded 14-bit coefficient recovers the source coordinate. | none | Production q14 unpacking must read the same per-coordinate bits. | Trusted only for the reviewed Lean toolchain/source revision. | Does not prove whole-verifier acceptance behavior. |

## Canonical Challenge Codec

| endpoint | proved-in-Lean | cryptographic assumption | implementation/conformance assumption | generated artifact/toolchain assumption | out-of-scope |
|---|---|---|---|---|---|
| `N32CanonicalChallengeByteCodec.productionBitLayout17` | The frozen 17-byte production codec is a concrete Lean object. | none | Production challenge bytes must use exactly this omitted-block/sign-bit/slack-bit layout. | Trusted only for the reviewed Lean toolchain/source revision and reviewed vector generator. | Does not prove the hash-to-challenge function calls this codec. |
| `N32CanonicalChallengeByteCodec.productionDecode_encode` | Lean proves the production decoder accepts `encode(c)` and recovers `c`. | none | The deployed decoder must reject and accept the same encodings. | Trusted only for the reviewed Lean toolchain/source revision. | Does not prove byte-codec use outside the verifier boundary. |
| `N32CanonicalChallengeByteCodec.productionEncode_injective` | Lean proves the production encoding is injective. | none | The implementation must not admit alternate encodings for the same canonical challenge. | Trusted only for the reviewed Lean toolchain/source revision. | Does not by itself prove all malformed inputs are rejected unless the implementation matches the Lean codec. |

## Hash Frame And Fiat-Shamir Boundary

| endpoint | proved-in-Lean | cryptographic assumption | implementation/conformance assumption | generated artifact/toolchain assumption | out-of-scope |
|---|---|---|---|---|---|
| `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame` | The exact framed byte string is defined in Lean from domain tag, profile bytes, modulus/group constants, public key, message, and reconstructed commitments. | none | Production must hash exactly this byte frame with the same labels, lengths, ordering, and commitment flattening. | The frozen profile bytes and domain string must be the reviewed ones; reviewers also need the checked source/tree and generated fixtures to agree. | Does not prove SHA-256 behaves like the modeled oracle. |
| `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length` | Lean proves the frame length is `1508 + messageLen`. | none | Production framing must preserve the same field widths and order. | Trusted only for the reviewed Lean toolchain/source revision. | Does not prove full frame-byte equality, only the length formula. |
| `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_fixtureMessage_length` | Lean proves the pinned `32 B` message fixture frame length is `1540`. | none | The pinned fixture used by conformance tests must be generated from the same framing logic. | Depends on the reviewed fixture-generation workflow matching the checked Lean source. | Does not prove digest or challenge equality. |
| `N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate` | This Lean record packages the q12289 hash frame with the proved commitment encoder and exposes the exact external premise needed for accepted transcripts. | Its field `hashFrameChallenge_matches_actual` is an external oracle-agreement premise, but at this layer it is still an abstract field rather than the final production assumption name. | The supplied `response` and `hashChallenge` implementation hooks must match deployment semantics if the certificate is used operationally. | Trusted only for the reviewed Lean toolchain/source revision. | Does not prove a concrete hash function or standard-model property. |
| `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement` | The record fixes the remaining q12289 production boundary to `q12289HashFrame` plus `commitmentViewEncoder`. | `A1`: this is the explicit Fiat-Shamir random-oracle/SHA agreement assumption for accepted transcripts. | The `response` hook and `hashChallenge` implementation still need byte-for-byte conformance to the Lean verifier boundary. | Trusted only for the reviewed Lean toolchain/source revision. | No standard-model SHA-256 proof is claimed. |

## Algebraic Verifier Equation

| endpoint | proved-in-Lean | cryptographic assumption | implementation/conformance assumption | generated artifact/toolchain assumption | out-of-scope |
|---|---|---|---|---|---|
| `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.verifierEquation_of_oneShotAccepts` | Lean proves that one-shot acceptance implies the q12289 product-block verifier equation once the production certificate is converted to the generic soundness theorem. | `A1`: the supplied production hash-frame oracle agreement is required to obtain the serialized-verifier acceptance premise. | The production verifier must match `corePayloadCheck`, response decoding, commitment reconstruction, challenge codec, and hash frame. | The proof also trusts the reviewed Lean toolchain/source revision and the frozen local matrix constants. | This endpoint stops at the verifier equation; it does not by itself prove the EUF-CMA theorem. |
| `N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_oneShotAccepts` | The generic bridge theorem is proved in Lean. | none in the generic theorem itself; q12289 gets a cryptographic premise only through the certificate instantiation feeding it. | Its deployment meaning depends on the supplied decode/reconstruct functions matching the implementation. | Trusted only for the reviewed Lean toolchain/source revision. | Does not classify keygen, signer, or side-channel behavior. |

## Local Lee Certificate And Global Bound

| endpoint | proved-in-Lean | cryptographic assumption | implementation/conformance assumption | generated artifact/toolchain assumption | out-of-scope |
|---|---|---|---|---|---|
| `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate` | Lean proves the local `3 x 2` bounded-free Lee certificate for the fixed `pairBlockMatrix`. | none | none directly; this is a mathematical certificate for the fixed matrix, not an implementation routine. | The fixed coefficients in `localCoeff` must be the reviewed coefficients for the frozen target. If those constants came from an external search/export step, that artifact must be archived and shown to match the checked source. | Does not prove that deployed keygen/signing instantiate the whole scheme correctly. |
| `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower` | Lean proves the lower-bound field of the previous certificate is at least `512`. | none | none directly. | Same artifact-integrity dependency as the previous row. | Does not on its own yield the final security statement. |
| `N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate` | The Lean structure packages the validated canonical-base verifier route with structural failure set to `false`, and its theorem `forgerSuccessProb_le_guess` proves the `2^-133` blind-guess bound. | For the fully concrete q12289 FS route, the verifier inputs ultimately rely on `A1`; the blind-guess reduction also presumes the query-log/Fiat-Shamir game interpretation captured by `A2`. | Deployment use still requires `A5` and `A7`: structural failure really must be reduced to explicit verifier checks, and the implementation must match those checks. | The package is trusted only for the reviewed Lean toolchain/source revision and the reviewed fixed q12289 matrix/profile constants. | It does not close signer distribution, side channels, or external cryptanalysis. |

## Cross-Cutting Dependencies Outside The Formal Core

These are not new theorem-map endpoints, but they remain attached to the
deployment reading of the final q12289 claim:

| assumption id | category | why it is not a Lean-closed dependency of the theorem-map endpoints |
|---|---|---|
| `A4` | cryptographic assumption | The theorem-map endpoints prove a verifier-side and certificate-side core. They do not by themselves prove that the deployed key generator only emits public views inside the intended safe family. |
| `A6` | out-of-scope for current theorem-map endpoints | Signer distribution and rejection-sampling behavior are not instantiated by the listed verifier theorems. They matter for the target EUF-CMA claim, not for the current verifier-side core. |
| `A8` | out-of-scope for current theorem-map endpoints | Side-channel, RNG, retry-observability, and secret-handling properties are deployment obligations outside the listed Lean theorems. |
| `A9` | out-of-scope for current theorem-map endpoints | Concrete attack cost and archived cryptanalysis are not part of the current Lean endpoint chain. |

## Review Conclusion

The theorem-map endpoints split cleanly into:

- pure Lean facts about codecs, framing, parsing predicates, and the verifier
  equation bridge;
- one explicit cryptographic assumption at the q12289 proof boundary:
  `ProductionHashFrameOracleAgreement` (`A1`);
- deployment conformance obligations that turn Lean byte predicates into a
  claim about real code (`A5`, `A7`);
- reproducibility/integrity obligations on the reviewed source tree and frozen
  generated artifacts (`A0`);
- broader deployment-only obligations (`A4`, `A6`, `A8`, `A9`) that do not
  alter the truth of the theorem-map endpoints themselves but still block a
  final deployment claim.
