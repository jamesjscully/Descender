# Freeze Audit: q12289 Byte/Profile Surface

Date: 2026-05-19

Scope audited:

- public key bytes: `172`
- signature bytes: `269`
- modulus `q`: `12289`
- repetitions: `7`
- challenge layout: `17 B`, `19` bits per repetition, `133` used bits total
- response bytes and bound: `36 B` per repetition, `|z| <= 224`
- hash domain/profile labels:
  `Descender.Signature.q12289.implicit_commitment.v1` and
  `validated-q12289-n48-m1-32-single-carrier`
- hash frame labels/order and pinned frame length: `1508 + messageLen`,
  fixture length `1540`
- committed vector fixture values in
  [q12289_conformance_vectors.json](./q12289_conformance_vectors.json)

Sources reconciled:

- [target_spec.md](./target_spec.md)
- [manifest.json](./manifest.json)
- [theorem_map.md](./theorem_map.md)
- [q12289_conformance_vectors.json](./q12289_conformance_vectors.json)
- [signature_q12289_implicit_verifier.rs](/home/jack/.paseo/worktrees/1ouugh8k/review-freeze-audit/tools/parameter-search/src/bin/signature_q12289_implicit_verifier.rs)
- [SignatureTranscript.lean](/home/jack/.paseo/worktrees/1ouugh8k/review-freeze-audit/Descender/SignatureTranscript.lean)
- [SignaturePublicViewWitness.lean](/home/jack/.paseo/worktrees/1ouugh8k/review-freeze-audit/Descender/SignaturePublicViewWitness.lean)

Conclusion:

The freeze claim is justified for the audited surface.

Evidence:

- The review package was created in commit `91799361a8e45a59c4208fcb9e2a74897aa65275`
  (`Document q12289 review readiness closure`).
- `git diff 9179936..HEAD --` over the audited files is empty, so there have
  been no post-package edits to the frozen spec/docs, Rust verifier constants,
  vector fixture, or Lean endpoints named in this audit.
- Current docs, verifier constants, vectors, and Lean endpoints agree on:
  `pk=172`, `sig=269`, `q=12289`, `repetitions=7`, `response_bound=224`,
  profile `validated-q12289-n48-m1-32-single-carrier`, domain
  `Descender.Signature.q12289.implicit_commitment.v1`, challenge codec
  `N32CanonicalChallengeByteCodec.productionBitLayout17`, and hash-frame
  endpoint `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame`.
- Regenerating the conformance vector bundle from the current Rust verifier
  produced a byte-for-byte match with the committed
  [q12289_conformance_vectors.json](./q12289_conformance_vectors.json).

Notes:

- The hash-frame endpoint lives in `Descender/SignaturePublicViewWitness.lean`;
  `Descender/SignatureTranscript.lean` is the matching Lean source for the
  frozen challenge codec and serialized signature byte count.
- This closes only the Freeze Gate checklist item
  `No unreviewed format changes since this package was created.`
