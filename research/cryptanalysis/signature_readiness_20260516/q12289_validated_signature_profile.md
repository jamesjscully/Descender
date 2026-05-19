# q12289 Validated Signature Profile

| field | value |
|---|---:|
| profile | `validated-q12289-n48-m1-32-single-carrier` |
| q | 12289 |
| n | 48 |
| m0 view coordinates | 80 |
| m1 | 32 |
| eta | 12 |
| target Lee | 512 |
| groups | 16 |
| rows/group | 3 |
| public key bytes | 172 |
| challenge bytes | 17 |
| relation payload bytes | 140 |
| single-carrier relation signature bytes | 157 |
| implicit response bits/coordinate | 9 |
| implicit response coordinates/repetition | 32 |
| implicit response bytes/repetition | 36 |
| implicit response bound | 224 |
| implicit commitment signature bytes | 269 |
| implicit commitment verifier | `signature_q12289_implicit_verifier` |
| implicit commitment hash domain | `Descender.Signature.q12289.implicit_commitment.v1` |
| explicit commitment bytes/repetition | 140 |
| explicit commitment signature bytes | 1235 |
| checked centered pairs | 523264 |
| independent min l1 | 533 |

## Lean Endpoints

- reconstructed commitment: `SignatureDistribution.productBlockReconstructedCommitment_honest`
- implicit commitment byte profile: `N32CompletedResponseSignatureByteProfile.leadingN32_9bit_2free_implicitCommitment`
- implicit verifier bridge: `N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs`
- q12289 implicit verifier alias: `N32Q12289ImplicitCommitmentByteVerifier.Inputs`
- q12289 implicit signature bytes: `N32Q12289ImplicitCommitmentByteVerifier.serializedSignatureBytes_eq`
- q12289 implicit core payload check: `N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck`
- q12289 implicit domain tag: `N32Q12289ImplicitCommitmentByteVerifier.implicitCommitmentDomainTag`
- q12289 implicit response view: `N32Q12289ImplicitCommitmentByteVerifier.responseView`
- q12289 reconstructed commitment view: `N32Q12289ImplicitCommitmentByteVerifier.reconstructedCommitmentView`
- q12289 product-block soundness constructor: `N32Q12289ImplicitCommitmentByteVerifier.productBlockEquationSoundness`
- q12289 commitment view encoder: `N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder`
- q12289 encoded-reconstruction certificate: `N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate`
- q12289 encoded-reconstruction bridge: `N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts`
- q12289 hash frame: `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame`
- q12289 hash frame length theorem: `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length`
- q12289 hash-frame certificate: `N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate`
- q12289 hash-frame bridge: `N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts`
- q12289 production oracle agreement: `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement`
- q12289 production certificate constructor: `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.toProductionHashFrameVerifierCertificate`
- q12289 production oracle bridge: `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.verifierEquation_of_oneShotAccepts`
- q12289 bounded local Lee check: `SharedRowProductBlock12289Rows3.boundedCenteredPairCheck_eq_true`
- q12289 internal local Lee certificate: `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate`
- q12289 internal local Lee lower bound: `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower`
- q12289 concrete verifier certificate: `N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate`
- q12289 concrete verifier bridge: `N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate.verifierEquation_of_oneShotAccepts`
- implicit accepted-transcript product-block bridge: `N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_acceptsTranscript`
- implicit one-shot product-block bridge: `N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_oneShotAccepts`
- global certificate: `N32Q12289Validated48x32CanonicalBaseSecurityCertificate`
- FS-core no-failure certificate: `N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate`
- canonical no-failure certificate: `N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate`
- no-failure bound: `Pr[forgery] <= 2^-133`

## Artifacts

- sidecar: `research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.json`
- verification: `research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.verify.json`

## Concrete Implicit-Commitment Codec

- challenge: 7 rounds, 19 bits/round: 4-bit omitted block plus 15 sign bits, little-endian, top 3 bits zero
- response: 7 chunks of 36 bytes; each chunk packs 32 signed 9-bit two's-complement free-response coordinates, little-endian
- public key: 32-byte seed followed by 80 q-coordinates packed as 48 pivot then 32 free coordinates at 14 bits each
- reconstructed commitment: 7 repetitions of reconstructed t' = A*z - chi*u, serialized as 80 q-coordinates packed at 14 bits each

## Rust Conformance Fixtures

- zero-public-key challenge: `500b7e675f1970875c294ccb0d00eee306`
- zero-public-key reconstructed commitment SHA-256: `b849747fe881a40d5e534e90128d6975e470de03a308a0c82fd2bc6226cd9c85`
- zero-public-key hash frame SHA-256: `500b7e675f1970875c294ccb0d00eee386910b1a37c8ecf5b78d97e0af51993c`

## Remaining Implementation Obligations

- Supply N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement for the production verifier: SHA-256-to-challenge over q12289HashFrame agrees with the query-log oracle on accepted transcripts.
- Keep the Rust verifier conformance fixtures pinned to the Lean q12289 byte and hash-frame contract.
