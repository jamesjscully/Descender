# WS5 — Adversarial Attack Catalog (the third-party hand-off dossier)

Created: 2026-06-09. Living catalog per master tracker WS5. Every attack class
the scheme must survive, with status `tried / cost / why-it-fails / evidence`.
This is the artifact a third-party cryptanalyst should be handed. It consolidates
the prior red-team notes (see Sources) into one place.

Status: `BLOCKED` = attack provably/empirically fails · `OPEN` = not yet run ·
`GATED` = fails only under a stated assumption (links to the ledger).

## Attack classes

| # | Class | Status | Cost / bound | Why it fails / what it rests on | Evidence |
|---|---|---|---|---|---|
| 1 | Lattice SVP/SIS short-vector (response box) | BLOCKED | exact | No nonzero kernel vector with max-coeff ≤ 96; smallest cover = 114 | `ResponseLinkExactCertificate.excludesBoundedNonzeroKernel` (Lean), ~1.12M-node exact enumeration |
| 2 | Lattice reduction on keygen/public-view SIS | OPEN | — | asymptotic core-SVP needs exact instance extraction | WS3 remainder; `cryptanalysis_concrete_security_estimate.py` |
| 3 | Fq12 action collision → forgery | GATED | find: 2^81.5; soundness term 2^-99 @ T=2^32 | action = SHA256 hash→F_q^12; collision generic, FS-gated; composed bound is AP2-N003 | ledger C1; estimator script |
| 4 | Algebraic shortcut on the action map | BLOCKED | — | action is random-oracle hash-derived, not algebraic in the challenge; field only used for extraction-difference invertibility | ledger C1; `verify_fq12_irreducibility.py` (X^12+X+25 irreducible) |
| 5 | Public post-challenge representative forgery | BLOCKED | — | verifier rejects representatives reconstructed from public material only | Rust regression `public_postchallenge_representative_forgery_remains_negative_control` |
| 6 | Transcript malleability / non-canonical encoding | GATED | — | canonical parsers reject; SUF-CMA target (D2) | negative-control parser vectors; deployable tracker |
| 7 | Replay / rollback / state-reuse (stateful slots) | GATED | — | consume-once slot semantics; D2 wants these breaks **proven**, not assumed | break events `slotReplayOrRollbackBreak` etc.; WS7-b |
| 8 | Multi-target / QROM Fiat–Shamir | GATED | 182-bit transcript term | widened challenge (AP2-004); single-target forgery ~2^162 | `Path2HashDomainAndMultiTarget`; estimator |
| 9 | Keygen / signing-oracle key recovery | OPEN | — | not closed (D2 allows named-but-standard); honest-flow only | AP2-005; deployable tracker |
| 10 | Hidden-source extraction unsoundness | GATED | — | abstract `Predicates` model; reduces to digest binding (C2) | ledger C2; AP2-001 |
| 11 | Statistical leakage in signatures/transcripts | OPEN | — | not yet probed (bias, `3×2` block correlation) | WS4 remainder |
| 12 | Rogue-key / related-key (aggregation) | N/A v1 | — | aggregation deferred (D1); revisit in v2 | WS6 |

## What is genuinely BLOCKED vs GATED vs OPEN

- **BLOCKED (real, verified):** #1 (exact bounded-kernel exclusion), #4 (action is
  RO, not algebraic — irreducibility verified), #5 (public-representative
  regression).
- **GATED (fails under a standard assumption):** #3, #6, #7, #8, #10 — each rests
  on a standard assumption (QROM hash CR, canonical encoding, consume-once,
  digest binding). Per D2 the conservative move is to **prove** #7's
  replay/rollback breaks rather than assume them (WS7-b).
- **OPEN (not yet run — honest gaps):** #2 (keygen-SIS asymptotic margin), #9
  (keygen/signing-oracle), #11 (statistical leakage). These are the next
  cryptanalytic priorities.

## Sources consolidated

- `audit/active_path_signature_independent_red_team_20260602.md`
- `research/cryptanalysis/review_readiness_20260519/cryptanalysis_plan.md`,
  `assumption_review.md`
- `research/path2_core_algorithm_red_team_v1.md`
- `research/cryptanalysis/signature_readiness_20260516/*`

## Completeness critic (what modality have we NOT run?)

- No lattice-reduction tool (BKZ/fplll) has actually been run on the
  keygen/public-view SIS (#2). The in-repo evidence is exact enumeration on the
  *small* response box only.
- No statistical test battery on real signature transcripts (#11).
- No fault / side-channel modeling (out of scope this phase per D2 — productization).
- These three are the OPEN items above and define the next WS3/WS4 round.
