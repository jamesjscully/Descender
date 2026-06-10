# Beyond-Box Hardness/Structure — RISK-FIRST VERDICT

**Scope:** the last internal could-force-redesign cryptanalysis before build-out.
Synthesizes two independent beyond-box attacks: (A) public-SIS lattice reduction
and (B) beyond-box kernel-structure hunt. Both ran against the deployed q=12289,
n=32, 12×23 full-response SIS instance.

---

## VERDICT: NO INTERNAL OBSTRUCTION (high confidence)

The internal cryptanalysis is clean. No concrete beyond-box attack or exploitable
structure exists in the modeled regime. **No redesign, no parameter change is
indicated.** The only residual is irreducibly EXTERNAL: the asymptotic/quantum
hardness of the response-link-CODED problem and the pseudorandomness of the
SHA-256 matrix sampler — the project's documented `open_external_estimator`.

This is **not** a heuristic "looks fine" verdict. The L∞ box `[-96,96]^23` is the
**EXACT** response-difference realizability frontier — a consequence of the
verifier's per-coordinate hard gate, not a margin — and the minimum forgeable
coded vector lies strictly above it (coeff ≥ 114), with a clean structureless
dead-zone in between.

---

## The decisive endpoint argument (why the box is exact, not heuristic)

The deployed verifier hard-rejects any response coordinate outside `[-48,48]`:

```
tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs:4662
  if !(-(RESPONSE_BOUND as i32)..=RESPONSE_BOUND as i32).contains(&centered) {
      return Err(format!("reference response out of bound in round {rep}"));
  }
RESPONSE_BOUND = 48  (line 42; also production_reference.rs:198)
```

A forgery needs two valid responses `r, r'`, both in `[-48,48]^23`. Their
difference is therefore in `[-96,96]^23` **exactly**, because
`{a − b : a, b ∈ [-48,48]} = [-96,96]`. So an approximate/near-collision forgery
is realizable **if and only if** a nonzero kernel-cover vector has L∞ ≤ 96. The
`response_difference_coeff_bound = 2·RESPONSE_BOUND = 96` is an exact
triangle-inequality frontier, not a tunable safety margin.

**The minimum is 114.** For the 114-vector, the coordinates exceeding 96
(99, −99, 97, −105, 114, −111) force the best cancelling endpoint to
|r′| ≥ |δ| − 48 = 51, 51, 49, 57, 66, 63 — every one exceeds the hard bound 48.
Even the 1-unit overshoot (idx9: 97 vs 96) cannot be absorbed. There is **zero
approximate-forgery slack**.

---

## Reconciliation of the two attacks (they agree, and they're independent)

Both attacks deterministically rebuilt the same instance and got the same
fingerprint: transcript SHA-256
`b21703e6381ec794407acf6313af5e347d591b65251ad1f8e7a24d06b0bb71a6` matches the
stored review JSON; rank 14, kernel dim 9 reproduce. They attack different
surfaces and corroborate:

| Surface | Attack A (lattice reduction) | Attack B (structure hunt) | Agreement |
|---|---|---|---|
| Bare 12×23 kernel lattice (dim 23) | LLL→BKZ-45: L2=158.96, coeff=68, L2/GH=1.007 (generic) | — | weak-by-design vector (68 ≤ 96) but rejected by code (coords [0]=−4, [3]=68 ≠ 0 mod q) |
| Coded sublattice (kernel ∩ {coord[0]≡coord[3]≡0 mod q}, dim 9) | LLL/BKZ-30/45: L2=283.18, coeff=139, L∞-min 120, L2/GH=0.79 | exact-SVP-by-enum: L2²=80192 (L2=283.2), L∞-min 114 | same Euclidean minimum; BKZ surfaces NO anomalous short vector enumeration misses |
| Gap (96,114] census | — | ZERO in (96,114), exactly 1 at 114, 4 by 120, 28 by 130 | clean ~18-unit dead zone, no boundary crowding |
| Structure (support/gcd/dup-cols/Vandermonde/3×2-block) | — | full 21-support, gcd 1, no dup cols, no sparse/block/automorphism pattern | SHA-256-random matrix; no algebraic shortcut |

**Key reconciliation point:** the bare lattice IS weak (coeff 68 ≤ 96), and the
scheme *knows* this — it is a negative control, rejected by the response-link
code, **not** a load-bearing hardness assumption. The accept path depends only on
the *coded* sublattice having no ≤96 vector. That exclusion is established by the
~1.12M-node ResponseLinkExactCertificate (min coeff 114) and **independently
corroborated by direct BKZ on the coded sublattice** (L2-min 139, L∞-min 120, all
≥ 96). Lattice reduction and exact enumeration agree: no forgeable coded vector
with coeff ≤ 96 exists, and no anomalous-gap vector that BKZ would surface but
enumeration would miss. There is no accidental dependence on n=32 bare-lattice
hardness anywhere in the accept path.

---

## Residual (must go to EXTERNAL review) — `open_external_estimator`

Stated precisely so the dossier can scope third-party cryptanalysis:

1. **Asymptotic/quantum hardness of the response-link-CODED problem beyond the
   deployed coeff≤96 box.** Whether a fundamentally different non-enumeration,
   non-lattice-reduction attack against the coded distribution (or keygen-SIS at
   scale) exists is unsettleable by this small modeled regime. (WS4 OPEN #2,
   keygen-SIS asymptotic margin.)
2. **Pseudorandomness of the SHA-256 rejection sampler** that fixes the public
   matrix M. The in-environment lattice-estimator (`tools/external/lattice-
   estimator`) could **not** be executed — it requires Sage, which is absent — so
   the actual BKZ runs + in-repo exact enumeration are the load-bearing evidence,
   not an asymptotic core-SVP curve. An external estimator run is the corroboration
   step we could not perform here.
3. **Statistical-leakage battery on real signature samples (WS4 OPEN #11),**
   including the 3×2 block structure. This is a sampling/statistics question, NOT
   a kernel-geometry shortcut — and this probe specifically ruled out the geometric
   exploit, leaving only the statistical channel for external review.

These are irreducibly external (third-party cryptanalysis) per the risk posture
and are explicitly out of scope for the internal modeled-box analysis.

---

## SINGLE CONCRETE NEXT ACTION

Record the **exact-frontier framing** in the WS5 dossier and **close WS4 OPEN
item #3**: the L∞ box `[-96,96]^23` is the EXACT response-difference realizability
body (forced by the verifier's per-coordinate `[-48,48]` gate, not a heuristic
margin), the dead-zone above it is structureless, and approximate-forgery slack is
zero. Then hand off the single external residual — a third-party
lattice-estimator/SIS run pointed at the **response-link-CODED problem (not the
bare quotient lattice)** plus the SHA-256 matrix-sampler pseudorandomness — under
the `open_external_estimator` item, alongside WS4 OPEN #2 and #11.

**No internal action could force a redesign or parameter change. Cleared for
build-out.**

---

*Evidence files (absolute paths):*
- `/home/jack/code/Descender/research/cryptanalysis/public_sis_lattice_reduction_findings.md`
- `/home/jack/code/Descender/research/cryptanalysis/beyond_box_kernel_structure_hunt.md`
- `/home/jack/code/Descender/research/cryptanalysis/ws3_concrete_security_findings.md`
- `/home/jack/code/Descender/research/cryptanalysis/ws4_empirical_probes.md`
- `/home/jack/code/Descender/tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs` (verifier gate, line 4662)
