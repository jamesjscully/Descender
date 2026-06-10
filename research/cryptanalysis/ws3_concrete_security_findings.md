# WS3 — Concrete Security Findings (verified arithmetic + honest gaps)

Created: 2026-06-09. Companion to `scripts/research/cryptanalysis_concrete_security_estimate.py`
(run it to reproduce). Split into RELIABLE (exact arithmetic over deployed
constants) and FLAGGED (modeled-only or needs instance extraction).

## RELIABLE (exact)

- **Fq12 action output space:** `12289^12 = 2^163.0` (matches the cert's
  `actionCapacityFloorBits = 163`).
- **Action collision (find):** classical birthday `2^81.5`; quantum `~2^54` (BHT,
  needs ~`2^54` qRAM) or `~2^82` without qRAM.
- **Action-collision soundness term:** `~2^-162` **PER FORK** (negligible,
  adequate for 128-bit). *Correction (2026-06-09):* an earlier draft modeled this
  as a multi-target `T^2/2^163 = 2^-99` term and flagged a gap — that was WRONG.
  A precomputed action collision yields no forgery (the response still needs the
  witness `w`, and the FS challenge cannot be cheaply steered onto a colliding
  value); the floor matters only for the forking extractor, whose failure prob is
  `~2^-162` per fork. So the 163-bit floor imposes **no** `2^-128` obstruction.
  (The 182-bit transcript term concerns the *separate* FS multi-target surface.)
- **Fiat–Shamir challenge entropy:** `162` bits (`130` algebraic + `32`
  hardening). Single-target forgery by challenge guessing `~2^162`. Classical
  query margin `34` bits over `2^128`; quantum `17` bits.
- **Deployed SIS response box:** NO nonzero kernel vector with max-coeff ≤ `96`;
  smallest cover coeff `114`; combined kernel dimension `9`. Exact, Lean-certified
  (`ResponseLinkExactCertificate`).
- **Extension polynomial:** `X^12 + X + 25` is **irreducible over F_12289**
  (Rabin test, `verify_fq12_irreducibility.py`).

## Tooling guard (WS3-c, in place)

`scripts/research/check_estimator_param_sync.py` asserts the estimator's mirrored
constants equal `production_reference.rs` (Q, FIELD_BITS, SIS rows, RESPONSE_BOUND,
REPS, CHALLENGE_HARDENING_BYTES, action residues, and the
`2*RESPONSE_BOUND == RESPONSE_LINK_BOUNDED_KERNEL_BOUND` invariant). Wired into CI
(`.github/workflows/ci.yml` `cryptanalysis` job) with the irreducibility check and
the estimator run. Closes the constant-drift hole — the bug class that broke the
build at the start of this engagement — for the security estimate.

## RESOLVED (earlier flags that dissolved under scrutiny)

1. **"Composed bound capped at 2^-99 by the action term"** — WITHDRAWN. The
   action term is per-fork `~2^-162`, not multi-target (see above). C1 imposes no
   `2^-128` obstruction. Composed-bound *closure* remains a WS7 reduction-wiring
   task, but it is not blocked by a C1 parameter gap.
2. **"Extract keygen/public-view SIS asymptotic bit-security margin"** —
   MIS-FRAMED. The bare keygen quotient SIS is *intentionally LLL-weak*: the 12×23
   lattice (dim 23, Gaussian-heuristic norm ~158) yields a short vector at L2≈159 /
   coeff 68 under BKZ-20 in milliseconds (`matrix_keygen_estimator_memo`). Security
   does not rest on that bare SIS; it rests on the **response-link code** rejecting
   the short vector (nonzero coords 0-3 mod q) with the exact enumeration
   `96 < 114` as the authoritative exclusion. There is **no asymptotic margin to
   extract** — the regime is exact enumeration, already Lean-certified.

## GENUINELY OPEN (cannot be closed by self-analysis this session)

1. **WS7 concrete reduction (maintainer-led).** Inhabit
   `PricedAcceptedSpendNumericCertificate` with real per-event bounds and the
   `norm_num` composed-bound check. Real proof work, not a parameter obstruction.
2. **Beyond-box response-link-coded hardness + sampler pseudorandomness.** The
   project's own `open_external_estimator` item (keygen security surface). This is
   the irreducibly EXTERNAL part — per the agreed risk posture, self-cryptanalysis
   cannot discharge it; it needs a third-party lattice-estimator/SIS analysis of
   the coded distribution beyond the exactly-enumerated box.
3. ~~L1/centered-L∞ → L2 metric mapping for B1/B2~~ **DONE** (in the estimator):
   response box `|coord|≤48` over 23 coords → `L2 ≤ 230`; the bare lattice's
   shortest vector is `L2 159 ≈ Gaussian-heuristic 158` (ratio 1.007, **no gap** —
   generic lattice; independently reproduces the keygen memo's GH). Conclusion an
   external estimator needs: point the SIS/lattice estimator at the
   **response-link-coded** problem, not the bare quotient lattice (which is easy
   by design). Beyond-box coded hardness remains the `open_external_estimator`
   item.
4. **Statistical-leakage battery** (WS4): the valuable version targets the
   response/witness-hiding distribution, needs the signing oracle, and a
   session-scale null result would be weak evidence — scoped, not performed.
