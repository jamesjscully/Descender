# Red-Team: Lattice Assault on the Coded-SIS Residual (VECTOR 1)

Date: 2026-06-10. Adversarial red-team probe. Goal: REFUTE the "no forgeable short
response difference / coeff 114>96 / no structure" claim. Outcome below is reported
honestly; I did not find a break.

## Target reconstructed (verified, not re-rolled)

- `full_response_matrix()`: 12x23 over q=12289, SHA-256 rejection sampler, domain
  `Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1`.
- Matrix transcript SHA-256 = `b21703e6381ec794407acf6313af5e347d591b65251ad1f8e7a24d06b0bb71a6`
  (matches deployed certificate exactly).
- Coded sublattice = kernel(M) ∩ {coord[0] ≡ coord[3] ≡ 0 mod q}, constraint rank 14,
  kernel dim 9, embedded in 23-dim integer lattice (23x23 basis with q-vectors).
  All basis rows independently re-verified as genuine coded kernel vectors.

## (a) BKZ + native enumeration on bare AND coded lattices — NO break

Ran fpylll independently this session (not a re-read of stored JSON):

- BARE kernel lattice (dim 11): shortest L-inf=68, L2=159. This vector IS in the box
  (<=96) but has coords[0]=12285, coords[3]=68 (mod q) NONZERO -> it is NOT coded,
  so the response-link code rejects it. The scheme knows this; nothing relies on the
  bare lattice being hard.
- CODED sublattice: BKZ block sizes 20/30/40/50/60 (= full-dimension SVP for the
  effective dim-9 lattice) ALL return the same shortest-L2 vector: L-inf=139, L2=283.18.
  min L-inf over reduced rows = 136; over rows + pairwise signed sums = 120.
- INDEPENDENT native fpylll `Enumeration` of ~50,000 coded vectors inside the full
  L2 cover ball (radius^2 = 23*96^2 = 211968, which provably contains EVERY L-inf<=96
  vector): minimum L-inf found = **114**. Count with L-inf<=96 = **0**. Exactly one
  vector lands in (96,114], at the boundary 114. This reproduces the certificate's
  exact-enumeration minimum WITHOUT using the repo's enumerator.
- 8.5M random small-integer-combination L-inf-minimization attempts: nothing below 138.

I also confirmed the in-repo exact-enumeration certificate replays and verifies
(`verify_response_link_bounded_kernel_certificate.py`, status=verified).

L-inf minimization is a genuinely different problem from BKZ's L2 minimization, so I
attacked it directly (scaled combos, native enum tracking L-inf, random search). Every
method agrees: minimum coded L-inf = 114 > 96. The verifier's exact bound 96 leaves an
18-unit margin to the nearest forgeable coded difference.

## (b) lattice-estimator (Sage installed via nix) — confirms nonexistence

`SIS.estimate(n=12, q=12289, length_bound=sqrt(23)*96=460.4, m=23)` returns
`rop: 2^inf, beta: 23, d: 23`. The estimator (full-dim BKZ-23 = SVP) finds NO vector
short enough to satisfy the L2 bound. This is the asymptotic-model echo of the exact
result: a forgeable-length vector does not exist in the lattice. Note the bare lattice
DOES have a coeff-68 vector below this bound, so the 2^inf reflects the generic-lattice
GH model; the load-bearing evidence remains the exact finite enumeration, which is
sound (the L2 ball of radius sqrt(23)*96 strictly contains the L-inf<=96 box).

KEY STRUCTURAL POINT: this is a dim-23 lattice. Full SVP is trivial (seconds). The
">=128-bit" security here is NOT asymptotic lattice hardness — it is the COMBINATORIAL
nonexistence of a coded box vector (proven exhaustively). An attacker cannot find a
forgeable short response difference because one does not exist, not because it is hard
to find. Asymptotic core-SVP on dim 23 is not the relevant security notion.

## (c) SHA-256 matrix sampler — no exploitable structure

- Full rank 12 over q (no degenerate rows/cols).
- chi^2 over 32 bins = 34.3 (df=31; uniform) — no entry-value bias.
- No duplicate rows or columns; gcd of all entries = 1; mean 6488 ~ q/2.
- Coded coordinates 0 and 3 are not anomalous vs other columns (norms in-range).
No circulant/Toeplitz/repeated-block structure that would weaken SIS.

## (d) (96,114] near-collision / approximate-forgery gap

Only vectors at exactly L-inf=114 appear in (96,114]; nothing in the open interval
(96,114). The verifier uses an exact `<= 96` bound, so a 114 vector is rejected with
margin. No approximate-forgery exploit of the exact boundary.

## SHORTEST VECTORS FOUND

- Coded, L2-optimal: L-inf=139, L2=283.18, support 21
  `[0,3,-23,0,94,-111,139,-44,39,-8,52,-57,-39,-30,84,52,65,-54,47,-65,-45,-36,-25]`
- Coded, L-inf-optimal (independent native enum): L-inf=**114**, coded-verified
  `[0,-46,66,0,99,77,60,-5,-99,97,-2,48,50,13,-105,114,-85,-16,-84,74,47,-10,-111]`
- Bare (non-coded, rejected): L-inf=68, L2=159.

## VERDICT: NO BREAK on this vector

The claim "no coded kernel vector with L-inf<=96; min is 114>96" is CONFIRMED by
independent reduction, independent native enumeration, the lattice-estimator, and a
clean sampler audit. No forgery, no sub-2^128 shortcut into the box, no sampler
weakness. The exact enumeration is sound (L2 cover ball contains the L-inf box).

## RESIDUAL for a better-resourced adversary

Genuinely EXTERNAL (the project's documented open item), not an internal obstruction:
1. Beyond-box coded hardness: the enumeration only proves nonexistence WITHIN the box.
   Whether the coded-SIS distribution admits a NON-lattice structural attack (algebraic
   relations over Fq12 = F_q[X]/(X^12+X+25), the ring the scheme actually lives in) is
   untested here. A targeted attack should exploit the RING structure of the 23 columns
   if they encode ring elements — I treated the lattice as generic integer/q-ary; a
   module-lattice or ring-homomorphism attack on the actual Fq12 algebra is the right
   next escalation and was NOT attempted.
2. SHA-256 sampler pseudorandomness: I only ran statistical tests, not a structured
   pre-image/correlation attack tying matrix entries to a chosen weak instance.
3. Estimator on the CODED problem specifically (not the bare q-ary SIS) with a module
   model, by a third party with Sage + g6k tuned.
