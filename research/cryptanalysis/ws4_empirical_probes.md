# WS4 — Empirical Structural Cryptanalysis (small-scale)

Created: 2026-06-09. Uses the n=32 / q=12289 smallness offensively. RUN this
session = real, reproducible results; OPEN = next round.

## RUN (real results)

- **Action map is a random oracle, not algebraic (rules out attack class #4).**
  The deployed `reviewed_fq12_challenge_action_vector` derives each of the 12
  residues by `uniform_mod_from_hash(SHA256-domain, challenge, idx, Q)`. There is
  no algebraic relation from challenge bytes to the action output to exploit; the
  Fq12 field structure acts only on the *output* (for extraction-difference
  invertibility). Probing for an algebraic collision shortcut is therefore moot —
  the relevant attack is generic hash collision (WS3: 2^81.5 find / 2^-99 FS-gated
  soundness term).
- **Extension field is well-formed.** `X^12 + X + 25` is irreducible over F_12289
  (`scripts/research/verify_fq12_irreducibility.py`, Rabin test, exit 0). So Fq12
  is a genuine field — no zero divisors that could create spurious non-invertible
  action differences and break extraction. The Lean cert assumes this; now
  independently confirmed true.
- **Bounded-kernel short-vector search (authoritative for the deployed box).** The
  in-repo exact enumeration (`ResponseLinkExactCertificate`, ~1.12M nodes) shows
  the smallest nonzero kernel cover vector has coeff 114 > the response-difference
  bound 96, with combined kernel dimension 9 — i.e. **no forgeable short response
  difference exists in the box**. This is a completed empirical attack (#1),
  Lean-certified.

## OPEN (next round — honest gaps, NOT yet run)

- **Statistical leakage battery (#11).** No bias / correlation tests have been run
  on real signature/transcript samples, including the `3×2` repeated local-block
  structure flagged in the existing cryptanalysis plan. Needs a sampling harness
  over `production_sign` outputs + standard tests (chi-square on residues,
  autocorrelation, the block-structure-specific test).
- **Lattice reduction on keygen/public-view SIS (#2).** No BKZ/fplll run on the
  larger SIS instance that backs the public key (as opposed to the small response
  box). Needs exact instance extraction (WS3 item) then an actual reduction run.
- **Structure hunt on the hand-searched bounded kernel beyond the box.** The
  enumeration excludes survivors *inside* the box; whether the hand-searched
  kernel basis has exploitable structure *outside* the modeled box (e.g. a
  slightly-longer vector enabling a near-collision attack) is unprobed.

## Why these OPEN items matter (no silent caps)

The completed empirical results cover the *small, modeled* regime exhaustively.
They do **not** cover the larger keygen SIS or the statistical surface. Claiming
"empirically no exploitable structure" (DoD-3 part 2) requires the OPEN items;
until then WS4's verified contribution is: the algebraic-shortcut class is ruled
out, the field is sound, and the deployed response box is clean.
