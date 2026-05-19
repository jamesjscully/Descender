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
- [ ] Every theorem dependency classified in `assumption_ledger.md`.

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

Details: [cryptanalysis_plan.md](cryptanalysis_plan.md),
[cryptanalysis_report.md](cryptanalysis_report.md), and
the A4/A9 sections of [assumption_review.md](assumption_review.md).

- [ ] Bad-vector/keygen-break search rerun against frozen target.
  Evidence tracker: [cryptanalysis_report.md](cryptanalysis_report.md#1-bad-vector--keygen-break-search).
- [ ] Lattice-reduction attack scripts and outputs archived.
  Evidence tracker: [cryptanalysis_report.md](cryptanalysis_report.md#2-lattice-reduction-attack-archive).
- [ ] Meet-in-the-middle/search attacks documented.
  Evidence tracker: [cryptanalysis_report.md](cryptanalysis_report.md#3-meet-in-the-middle--search-attacks).
- [ ] Transcript malleability attacks documented.
  Evidence tracker: [cryptanalysis_report.md](cryptanalysis_report.md#4-transcript-malleability-attacks).
- [ ] Concrete security table written.
  Evidence tracker: [cryptanalysis_report.md](cryptanalysis_report.md#5-concrete-security-table).
- [ ] Known limitations written plainly.
  Evidence tracker: [cryptanalysis_report.md](cryptanalysis_report.md#6-known-limitations).

## Implementation Gate

Details: [implementation_hardening.md](implementation_hardening.md),
[implementation_audit.md](implementation_audit.md), and
the A6/A7/A8 sections of [assumption_review.md](assumption_review.md).

- [ ] Reference keygen.
  Evidence tracker: [implementation_audit.md](implementation_audit.md#1-reference-keygen).
- [ ] Reference signing.
  Evidence tracker: [implementation_audit.md](implementation_audit.md#2-reference-signing).
- [x] Prototype verifier.
- [ ] Constant-time review.
  Evidence tracker: [implementation_audit.md](implementation_audit.md#4-constant-time-review).
- [ ] Fuzz targets.
  Evidence tracker: [implementation_audit.md](implementation_audit.md#5-fuzz-targets).
- [ ] Randomness handling review.
  Evidence tracker: [implementation_audit.md](implementation_audit.md#6-randomness-handling-review).
- [ ] Secret zeroization review.
  Evidence tracker: [implementation_audit.md](implementation_audit.md#7-secret-zeroization-review).

## Review Bundle Gate

Details: [README.md](README.md), [reviewer_guide.md](reviewer_guide.md),
and [manifest.json](manifest.json).

- [x] One spec.
- [x] One theorem map.
- [x] One assumption ledger.
- [x] One detailed assumption review.
- [x] One conformance vector suite.
- [ ] One cryptanalysis report.
  Current artifact: [cryptanalysis_report.md](cryptanalysis_report.md) (skeleton only; incomplete until substantive evidence is archived).
- [ ] One implementation audit note.
  Current artifact: [implementation_audit.md](implementation_audit.md) (skeleton only; incomplete until substantive audit findings are archived).
- [x] One reviewer guide with highest-risk questions.

When every unchecked item is either completed or explicitly waived with a
security rationale, the package can be sent for third-party review.
