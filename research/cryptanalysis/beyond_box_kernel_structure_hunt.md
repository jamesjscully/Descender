# WS4 #3 — Beyond-Box Kernel Structure Hunt

Created: 2026-06-10. Closes the third OPEN item in `ws4_empirical_probes.md`
("structure hunt on the hand-searched bounded kernel beyond the box").

## Question

The in-repo exact enumeration (`ResponseLinkExactCertificate`, ~1.12M nodes)
proves NO forgeable short response difference exists INSIDE the modeled
L-infinity box (smallest nonzero kernel-cover vector L-inf = 114 > response
difference bound 96, combined kernel dim 9). OPEN: does the kernel basis have
exploitable structure JUST OUTSIDE the box — a near-bound vector enabling an
approximate / near-collision forgery, or algebraic/Vandermonde regularity, or an
unusually short vector under a better reduction than the enumeration assumed?

## Method (reproducible)

All three scripts reconstruct the q=12289 12x23 full-response SIS matrix
deterministically from `Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1`
and rebuild the combined intersection lattice (coords {0,3} fixed to 0 mod q).
The reconstruction is cross-checked against the committed certificate:
`matrix_transcript_sha256` matches
`b21703e6381ec794407acf6313af5e347d591b65251ad1f8e7a24d06b0bb71a6`, and
combined_constraint_rank = 14, kernel_dim = 9 reproduce the cert exactly.

- `scripts/research/beyond_box_kernel_structure_probe.py` — reduction escalation,
  enumeration min-L-inf, structural regularity.
- `scripts/research/beyond_box_gap_census.py` — fine census of the (96, K] gap.
- `scripts/research/beyond_box_exact_svp_and_endpoint.py` — exact Euclidean SVP +
  the endpoint-feasibility / approximate-forgery argument.

## Findings

### (c) Stronger reduction finds NOTHING shorter than the cert

- LLL, BKZ-23/24/30/40 (auto-abort, 8 tours) all converge to the SAME shortest
  Euclidean vector: L2^2 = 80192 (L2 = 283.2), L-inf = 139. Identical to the
  cert's `lll_first_vector_stats`.
- Exact SVP by full enumeration to radius r_00 confirms the true Euclidean
  minimum is L2^2 = 80192 — there is no shorter vector under any basis.
- best/Gaussian-heuristic ratio = 0.791 (GH norm 358.0). The lattice DOES carry
  an unusually short Euclidean vector (~21% under GH), expected for a q-ary
  intersection lattice — but its L-inf is 139, far above the box, and it does
  NOT shorten the L-inf-relevant minimum.
- The TRUE L-inf minimum of the lattice is 114 (confirmed by enumeration far
  past the L2-shortest, up to L2^2 = 21*160^2). No basis change lowers it.

### (a) Gap-region census — the gap is essentially empty

Exhaustive sign-canonical enumeration (200k distinct vectors, all reachable
short vectors):

| L-inf range | count |
|---|---|
| <= 96 (the box) | 0 (certified infeasible) |
| (96, 100] | 0 |
| (96, 105] | 0 |
| (96, 110] | 0 |
| (96, 114] | 1 |
| (96, 120] | 4 |
| (96, 130] | 28 |
| (96, 140] | 117 |

The single vector in (96, 114] IS the L-inf-minimum (114). There is a clean ~18-
unit dead zone above the box before vectors begin appearing — the kernel is not
"crowding" the box boundary. The smallest above-box vector is well-separated.

### (b) No algebraic / structural regularity

- Shortest reduced rows have FULL support (21 of 21 free coords; coords 0,3 are
  fixed to 0). No low-weight / sparse codeword structure to exploit.
- gcd of the shortest vector's entries = 1 (no scalar sublattice shortcut).
- Zero duplicate matrix columns (no trivial 2-support kernel vector).
- The common support across the 9 shortest basis rows is all 21 coords — no
  shared sparse pattern, no exploitable 3x2 repeated-local-block alignment in the
  kernel basis. (The 3x2 block structure remains a STATISTICAL-leakage question
  on signature samples — WS4 OPEN #11 — not a kernel-geometry shortcut.)
- No Vandermonde regularity surfaced: the matrix is a SHA256-derived uniform
  random matrix (rejection-sampled mod q), so the intersection lattice behaves as
  a generic random q-ary lattice with no algebraic automorphism to ride.

### (a/c decisive) Why the gap is NOT exploitable — the endpoint argument

This is the load-bearing structural fact, verified against the deployed
verifier (`signature_q12289_local_delta_stateful.rs`):

- `const RESPONSE_BOUND: i16 = 48`. The verifier HARD-REJECTS any response
  coordinate outside [-48, 48] (centered-lift check at lines 4662-4664; also
  4740, 9526 negative control).
- A forgery requires two DISTINCT valid responses r != r' for the same
  commitment / challenge / public data. Both have every coord in [-48, 48].
  Their difference delta = r' - r is a nonzero kernel vector, and per coordinate
  {a - b : a, b in [-48,48]} = [-96, 96] EXACTLY. So the realizable
  response-difference body is precisely the L-inf box [-96,96]^23 — the
  `response_difference_coeff_bound = 2 * RESPONSE_BOUND = 96` (line 5358) is an
  EXACT triangle-inequality bound, not a heuristic margin.
- Therefore an approximate / near-collision forgery is realizable IFF some
  nonzero kernel vector has L-inf <= 96. The cert proves the minimum is 114.
- Concretely, for the smallest above-box kernel vector (L-inf 114), the
  coordinates exceeding 96 are: idx4=99, idx8=-99, idx9=97, idx14=-105,
  idx15=114, idx22=-111. Even the BEST cancelling endpoint leaves
  |r'_i| >= |delta_i| - 48: 51, 51, 49, 57, 66, 63 respectively — ALL exceed the
  hard bound 48, so the verifier rejects on those coordinates. There is no slack:
  the nearest above-box vector overshoots by 1 unit on idx9 (97 - 96) and up to
  18 on idx15. No "almost valid" second response exists.

## Verdict

NO beyond-box structure found. Internal probe is conclusive: the gap region just
above coeff 96 is empty up to 114 and the single 114-vector (and all higher) are
provably non-realizable as a response difference because the verifier's per-coord
[-48,48] gate makes [-96,96]^23 the EXACT difference body — there is no
approximate-forgery slack. Stronger reduction (exact SVP, BKZ-40) finds nothing
the enumeration missed; the kernel has no algebraic/Vandermonde/low-weight
regularity (SHA256-random matrix, full-support short vectors, gcd 1).

This strengthens the existing claim from "no vector inside the box" to "the box
is the exact realizability frontier and the dead-zone above it is clean and
structureless," which is the relevant beyond-box assurance.

## Residual that remains irreducibly external

This probe covers the SMALL, MODELED kernel-geometry regime exhaustively. It does
NOT and cannot settle the asymptotic / quantum beyond-box hardness of the
underlying construction — i.e. whether a fundamentally different (non-enumeration,
non-lattice-reduction) attack against the keygen SIS or the response-link code at
scale exists. That is the project's standing `open_external_estimator`: an
irreducibly external question for third-party cryptanalysis (and is consistent
with the project memory: n=32 is deliberately too small for lattice hardness;
security rests on the response-link code + this exact box exclusion, NOT on
asymptotic SIS margin). DoD-3 part-2 ("empirically no exploitable structure") is
advanced by this result for the modeled regime but the keygen-SIS asymptotic
margin + statistical-leakage battery (WS4 OPEN #2, #11) remain genuinely open and
external.
