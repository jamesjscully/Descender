# Public-SIS Lattice Reduction — Findings (WS4 #2)

Created: 2026-06-10. Author: WS4 public-SIS lattice-reduction probe (internal).
Companion artifacts:
- `scripts/research/quotient_sis_lattice_attack_review.py` (matrix rebuild + reduction harness)
- `scripts/research/verify_response_link_bounded_kernel_certificate.py` (`full_response_matrix()`)
- `research/cryptanalysis/signature_readiness_20260516/quotient_sis_lattice_attack_review_n32_q12289.json`
- `scripts/research/cryptanalysis_concrete_security_estimate.py` (WS3 estimator)

All numbers below are from an **independent fpylll reduction** I ran this session
(fpylll 0.6.4), not just a re-read of the stored JSON. The lattice-estimator was
NOT runnable (`tools/external/lattice-estimator` requires Sage; `import sage` fails
in this environment), so the asymptotic core-SVP curve is taken from the WS3
estimator's textbook formula and is *not* the load-bearing evidence — the
load-bearing evidence is the actual BKZ reduction + the exact enumeration.

## The actual public SIS instance (extracted, verified)

The public key backs a SIS relation `M x = 0 (mod q)` with:

- `q = 12289`, matrix shape `12 x 23`, `rank 12`, **kernel dimension 11**.
- Entries: SHA-256 rejection sampling over domain
  `Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1`,
  row/col/u32-counter, rejection cutoff `61445` (= `65535 - 65535 % q`).
- Reconstructed transcript SHA-256 = `b21703e6381ec794407acf6313af5e347d591b65251ad1f8e7a24d06b0bb71a6`
  — **matches** the stored review JSON exactly (so I reduced the deployed matrix,
  not a re-roll).

The integer q-kernel lattice has dimension 23,
`det = 1.186e49`, `det^(1/23) = 136.04`, Gaussian-heuristic norm `157.87`.

## Reduction result on the BARE kernel lattice (reproduced)

| reduction | wall time | shortest L2 | max-abs coeff | L2 / GH |
|-----------|-----------|-------------|---------------|---------|
| LLL       | 0.007 s   | 158.96      | 68            | 1.007   |
| BKZ-20    | 0.025 s   | 158.96      | 68            | 1.007   |
| BKZ-30    | <0.01 s   | 158.96      | 68            | 1.007   |
| BKZ-40    | <0.01 s   | 158.96      | 68            | 1.007   |
| BKZ-45    | <0.01 s   | 158.96      | 68            | 1.007   |

BKZ-45 on a dim-23 lattice is essentially full-dimension SVP. Nothing shorter than
L2 159 / coeff 68 appears at any blocksize. **L2/GH = 1.007 ≈ 1.0 → a generic
lattice with NO unusually-short vector.** The bare keygen SIS is LLL/BKZ-weak by
design: the coeff-68 short vector sits INSIDE the response-difference box (≤96),
and is rejected ONLY because its response-link coordinates `[0]=-4`, `[3]=68` are
nonzero mod q (`caught_by_response_link_code_0_3 = true`). This independently
confirms the project's "intentionally lattice-weak keygen" characterization and
the WS3/`matrix_keygen_estimator_memo` figures (coeff 68, L2 159, ratio 1.007).

## The question that matters: is there a lattice shortcut the enumeration misses?

The security claim is NOT bare SIS hardness — it is: *no forgeable short response
difference exists*, i.e. no nonzero vector that (a) lies in the kernel of `M`,
(b) ALSO vanishes on the response-link coordinates `[0,3]` mod q (the "coded"
constraint a real forgery must satisfy), and (c) has max-abs coeff ≤ 96. The
in-repo `ResponseLinkExactCertificate` proves this by EXACT enumeration
(~1.12M nodes): smallest such cover vector has coeff **114 > 96**.

I attacked this directly with lattice reduction — the natural way to find a short
coded vector OUTSIDE the enumerated box if one existed. I built the **coded
sublattice** = kernel(`M`) ∩ {coord[0] ≡ coord[3] ≡ 0 mod q} and reduced it:

- combined kernel dimension = **9** (matches the certificate exactly).
- intersecting with the two q-coordinate constraints densifies the lattice:
  `det^(1/23)` jumps `136 → 308.5`, GH norm `157.9 → 358.0`.
- LLL / BKZ-30 / BKZ-45 all return the same shortest coded vector:
  **L2 = 283.18, max-abs coeff = 139**, syndrome zero, coords [0],[3] ≡ 0 mod q
  (verified — it is a genuine coded kernel vector). L2/GH = 0.79.
- min L∞ (cover coeff) over reduced rows + signed pairwise sums = **120**,
  consistent with the enumeration's exact minimum of 114.

**Lattice reduction AGREES with the exact enumeration.** The shortest forgeable
coded vector has coeff ~114–139 (enumeration-exact 114; BKZ L2-optimal 139),
all comfortably above the bound 96. The GH ratio 0.79 (slightly below 1, no
anomalous gap) means BKZ surfaces no unexpectedly-short coded vector that the
enumeration would have missed. There is **no lattice shortcut** into the box.

## Does any security claim accidentally depend on bare-lattice hardness of n=32?

No. I checked the dependency direction in both layers:
- The bare 12×23 lattice IS weak (coeff 68 ≤ 96) and the scheme KNOWS this — the
  weak vector is treated as a negative control and rejected by the code, not relied
  upon. Nothing in the accept path requires the bare lattice to be hard.
- The accept path requires the *coded* sublattice to have no ≤96 vector, and that is
  established by exact enumeration (not by an asymptotic lattice-hardness assumption),
  and is independently corroborated here by direct BKZ on the coded sublattice.

So the "keygen-SIS is intentionally lattice-weak and security does not rest on it"
characterization is **honest and verified**. There is no accidental dependence on
n=32 lattice hardness in the modeled box.

## Honest scope / residual (irreducibly EXTERNAL)

- The exact enumeration and these reductions cover only the DEPLOYED, modeled box
  (coeff ≤ 96, the response-difference bound). Hardness of the response-link-CODED
  problem BEYOND the box — including pseudorandomness of the SHA-256 sampler that
  picks `M`, and whether the coded distribution admits any non-lattice structural
  attack — is the project's documented `open_external_estimator` item and remains
  genuinely external (third-party cryptanalysis), per the risk posture.
- The lattice-estimator could not be run here (no Sage). A third-party run of the
  estimator/SIS analysis on the CODED problem (not the bare quotient) is the right
  external next step; the WS3 estimator already states this redirect.

## Verdict

`no-internal-obstruction` for the public-SIS lattice surface within the modeled box.
The bare public lattice is weak by design; nothing in the security claim depends on
it being hard; and direct BKZ on the security-relevant CODED sublattice reproduces
the exact-enumeration exclusion (shortest forgeable coded vector coeff ≥114, BKZ
L2-min 139, both ≥ bound 96). No lattice shortcut beyond what the enumeration
already accounts for. The only residual is the external beyond-box coded-hardness +
sampler-pseudorandomness item, which is not an internal obstruction.
