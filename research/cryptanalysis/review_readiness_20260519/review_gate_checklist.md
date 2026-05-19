# Review Gate Checklist

This checklist defines "ready for third-party review."  It is not yet complete.
It is the dashboard for the review package; the detailed trackers are
[assumption_review.md](assumption_review.md), [target_theorem.md](target_theorem.md),
[conformance_plan.md](conformance_plan.md),
[cryptanalysis_plan.md](cryptanalysis_plan.md), and
[implementation_hardening.md](implementation_hardening.md).

## Freeze Gate

Details: [target_spec.md](target_spec.md), [manifest.json](manifest.json).

- [x] Frozen q12289 candidate documented.
- [x] Public key and signature sizes documented.
- [x] Hash domain documented.
- [x] Hash frame endpoint documented.
- [ ] No unreviewed format changes since this package was created.

## Formal Proof Gate

Details: [theorem_map.md](theorem_map.md),
[assumption_ledger.md](assumption_ledger.md), and
[target_theorem.md](target_theorem.md).

- [x] q14 commitment encoder theorem.
- [x] q12289 hash frame theorem.
- [x] accepted transcript to product-block verifier equation bridge.
- [x] production certificate derived from `ProductionHashFrameOracleAgreement`.
- [x] Detailed assumption review written for the frozen target.
- [x] Final theorem statement written in deployable EUF-CMA-style language.
- [x] Every theorem dependency classified in `assumption_ledger.md`.

## Conformance Gate

Details: [conformance_plan.md](conformance_plan.md) and
[q12289_conformance_vectors.json](q12289_conformance_vectors.json).

- [x] Rust verifier has pinned hash-frame fixture.
- [x] Rust verifier tests q-coordinate and response canonicality.
- [x] Canonical JSON/raw-hex test-vector bundle.
- [x] Negative test-vector bundle.
- [ ] CI conformance harness that fails on byte-frame drift.
- [ ] Independent implementation cross-check.

## Cryptanalysis Gate

Details: [cryptanalysis_plan.md](cryptanalysis_plan.md) and
the A4/A9 sections of [assumption_review.md](assumption_review.md).

- [ ] Bad-vector/keygen-break search rerun against frozen target.
- [ ] Lattice-reduction attack scripts and outputs archived.
- [ ] Meet-in-the-middle/search attacks documented.
- [ ] Transcript malleability attacks documented.
- [ ] Concrete security table written.
- [ ] Known limitations written plainly.

## Implementation Gate

Details: [implementation_hardening.md](implementation_hardening.md) and
the A6/A7/A8 sections of [assumption_review.md](assumption_review.md).

- [ ] Reference keygen.
- [ ] Reference signing.
- [x] Prototype verifier.
- [ ] Constant-time review.
- [ ] Fuzz targets.
- [ ] Randomness handling review.
- [ ] Secret zeroization review.

## Review Bundle Gate

Details: [README.md](README.md), [reviewer_guide.md](reviewer_guide.md),
and [manifest.json](manifest.json).

- [x] One spec.
- [x] One theorem map.
- [x] One assumption ledger.
- [x] One detailed assumption review.
- [x] One conformance vector suite.
- [ ] One cryptanalysis report.
- [ ] One implementation audit note.
- [x] One reviewer guide with highest-risk questions.

When every unchecked item is either completed or explicitly waived with a
security rationale, the package can be sent for third-party review.
