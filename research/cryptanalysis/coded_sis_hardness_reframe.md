# Coded-SIS "hardness" — re-framed under the external-only principle (2026-06-18)

James's principle: the ONLY external things are (1) third-party review, (2) cited
literature, (3) software dependencies. The mathematical hardness is OURS to establish.
Applying it to the spend signature's "coded-SIS" residual, which the dossier had
labelled "irreducibly EXTERNAL," demotes it substantially. This is a synthesis +
verification pass over the existing internal cryptanalysis (beyond_box_VERDICT,
redteam_lattice_assault, beyond_box_kernel_structure_hunt) plus two load-bearing facts
verified this session.

## The two facts that collapse the "external" label

1. **The full-response matrix is a SINGLE FIXED GLOBAL PUBLIC matrix.**
   `full_response_matrix_entry(row,col) = domain_matrix_entry(SIS_FULL_RESPONSE_MATRIX_DOMAIN,
   row,col)` = `SHA256(domain ‖ row ‖ col ‖ counter) mod q` (rejection-sampled),
   deterministic, 12×23 over q=12289, same for everyone
   (signature_q12289_local_delta_stateful.rs:1467-1490). So a property certified for
   THIS matrix holds for ALL signatures forever.

2. **The instance is a PLAIN integer SIS, not a ring/module SIS.** The response /
   response-difference is `[u16; 23]` over q with a plain 12×23 matrix
   (production_reference.rs:216, 1010); there is no negacyclic/cyclotomic/ring
   multiplication anywhere. q=12289's NTT-friendliness ("splits") is a concern ONLY in
   the lattice-FOLDING / STARK-ring context, NOT here — there is no ring to split.
   ⇒ the dossier's "ring/module structure UNATTACKED = external residual" is **vacuous
   for this instance**: there is no ring/module/CRT attack surface to attack.

## What is PROVEN (not assumed): in-box forgery is impossible

A forgery needs two accepted responses r, r′ ∈ [-48,48]²³ (verifier hard gate
RESPONSE_BOUND=48); their difference lies in [-96,96]²³ EXACTLY (Minkowski difference
of the box), and must be a nonzero coded-kernel vector. So forgery ⟺ ∃ coded-kernel
vector with L∞ ≤ 96. Direct exact enumeration of the coded sublattice (kernel ∩
{coord0≡coord3≡0 mod q}, dim 9) proves the **minimum coded L∞ is 114 > 96** (~1.12M-node
ResponseLinkExactCertificate), with a clean ZERO-count dead-zone in (96,114].
Independently reproduced this cycle's red-team: fpylll native `Enumeration` over the L2
cover ball (radius²=23·96² ⊇ every L∞≤96 vector) → min L∞ 114, count ≤96 = 0; BKZ-20..60
(= full-dim SVP for dim-9) L∞-min 120; 8.5M random-combo search nothing < 138; Sage
`SIS.estimate(n=12,q=12289,length_bound=√23·96,m=23) → rop 2^inf`. The bare (uncoded)
lattice DOES have a coeff-68 ≤96 vector — but it is a negative control, rejected by the
response-link code (coord0,coord3 ≠ 0 mod q); nothing relies on the bare lattice.

⇒ **In-box forgery probability is exactly 0 for the deployed matrix, for all signatures.**
This is a finite verified computation, not a hardness assumption.

## Why the dim is not a weakness, and what "bit security" means here

The coded lattice is dim 9 (ambient 23); full SVP / exhaustive box-enumeration is
~2²⁰ and was run to completion. Security here is NOT asymptotic lattice hardness (the
lattice is trivially solvable) — it is the **combinatorial NONEXISTENCE** of a
forgeable box vector, proven by that exhaustive computation. So the lattice branch of
the SUF-CMA bound contributes **0 mass** (empty), not a core-SVP 2^(0.292β) term. The
finite bit-security of the spend therefore comes from the OTHER break branches —
message/challenge binding (HIGH-3 widened to 246 effective challenge bits; multi-target
T·ε_spr term) and the second-preimage/collision bindings — NOT from a novel lattice
assumption.

## The honest residual, sorted by James's external categories

- **Third-party review (legitimately external):** confirmation that no attack CLASS was
  missed (key-recovery, sampler-backdoor, beyond-box). Internal red-team found none
  (structure hunt: full 21-support, gcd 1, no dup-cols / block / Vandermonde /
  automorphism). This is the one irreducibly-external item — and it's external because
  it's *review*, not because the math can't be done.
- **Cited literature:** the core-SVP cost model (used only to corroborate; moot here
  since enumeration is exhaustive) and the SHA-256 SPR/CR/RO properties used by the
  NON-lattice break branches. Standard, citable.
- **Software dependencies:** fpylll + the Sage lattice-estimator (used and reproduced;
  note SECURITY_DOSSIER §"Sage absent" is STALE — redteam_lattice_assault.md ran it via
  nix and got rop 2^inf).
- **SHA-256 matrix sampler:** NOT a load-bearing hardness assumption. The matrix is
  fixed+public and certified clean by direct enumeration, so the sampler is a
  nothing-up-my-sleeve / reproducibility method (auditable domain string), not a PRG
  whose pseudorandomness security depends on. A "backdoored matrix" is excluded by the
  certificate, not by trusting the sampler.

## Net re-frame

The dossier's "irreducibly EXTERNAL coded-SIS hardness + matrix-sampler PRG" residual
is overstated. Corrected position: in-box forgery is PROVEN impossible (matrix-specific,
exhaustive, multiply-reproduced); the instance has no ring/module structure to attack;
the sampler is transparency not hardness; the spend's finite security is governed by the
message-binding / challenge terms (ours, established) + standard SHA-256 properties
(cited). The only genuinely external item is third-party REVIEW that no attack class was
missed — which is external by James's category (1), not because the hardness is beyond us.
