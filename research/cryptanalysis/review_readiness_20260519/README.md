# q12289 Review Readiness Package

Date: 2026-05-19

This directory is the deployment-security work package for the current
q12289 implicit-commitment signature candidate.  It is not a performance plan.
The target is frozen until the review gates below are satisfied or a security
issue forces a documented change.

## Frozen Target

- Profile: `validated-q12289-n48-m1-32-single-carrier`
- Public key: `172 B`
- Signature: `269 B`
- Modulus: `q = 12289`
- Groups: `16`
- Rows per group: `3`
- Local free coordinates per group: `2`
- View coordinates: `80`
- Repetitions: `7`
- Challenge: `17 B`
- Response chunk: `36 B` per repetition
- Hash domain: `Descender.Signature.q12289.implicit_commitment.v1`
- Hash frame: `N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame`
- Current external cryptographic assumption:
  `N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement`

The canonical generated profile is:

```text
research/cryptanalysis/signature_readiness_20260516/q12289_validated_signature_profile.md
```

## Package Contents

- [target_spec.md](target_spec.md): frozen byte-level scheme target.
- [target_theorem.md](target_theorem.md): deployable ROM theorem target and
  premise map.
- [theorem_map.md](theorem_map.md): Lean endpoint map and what each theorem buys.
- [assumption_ledger.md](assumption_ledger.md): proved facts versus assumptions.
- [assumption_review.md](assumption_review.md): detailed assumption IDs,
  risk levels, attack surfaces, and closure criteria.
- [conformance_plan.md](conformance_plan.md): byte-level test-vector and harness plan.
- [q12289_conformance_vectors.json](q12289_conformance_vectors.json):
  generated positive and negative verifier conformance vectors.
- [cryptanalysis_plan.md](cryptanalysis_plan.md): attacks and experiments before review.
- [implementation_hardening.md](implementation_hardening.md): reference implementation security gates.
- [reviewer_guide.md](reviewer_guide.md): highest-risk questions for external
  review.
- [review_gate_checklist.md](review_gate_checklist.md): exit criteria before third-party review.

## Current Trust Boundary

The formal chain now reaches from accepted serialized transcripts to the
q12289 product-block algebraic verifier equation, using the concrete q14
encoder and the concrete hash-frame definition.  The only remaining explicit
Fiat-Shamir/hash premise is that the production hash-to-challenge function
agrees with the query-log random-oracle model on the exact `q12289HashFrame`.

That is the right boundary for a random-oracle-model signature proof.  It does
not prove SHA-256 is random; it removes ambiguity about which bytes are hashed
and what accepting transcripts mean.

The full assumption review is now split out in
[assumption_review.md](assumption_review.md).  Its main conclusion is that the
formal boundary is clean, but deployment is still blocked by keygen/no-bad-view
closure, concrete signer distribution, independent byte-level conformance/CI,
and frozen-target cryptanalysis.

## No-Optimization Rule

Until the review gate is satisfied, do not optimize speed, change encodings,
change parameters, or change transcript format unless the change is required by
a security finding.  Any such change resets the relevant proof, conformance,
and cryptanalysis gates.
