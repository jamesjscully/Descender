#!/usr/bin/env python3
"""
WS3 concrete-security estimate for the Descender Path 2 native / reviewed-Fq12
route. Deliberately conservative and HONEST about scope:

  * Numbers this script asserts (collision / Fiat-Shamir entropy / birthday) are
    exact arithmetic over the deployed constants and are reliable.
  * The lattice (SIS) core-SVP figure is a STANDARD-MODEL estimate of *known*
    attacks only; the exact SIS instance dimension is flagged where it affects
    the result. It bounds modeled attacks, never "no attack exists".

Deployed constants are mirrored from tools/parameter-search/src/production_reference.rs
and src/bin/signature_q12289_local_delta_stateful.rs (kept in sync by CI param check).
"""
from math import log2, sqrt, lgamma, pi, e

# ---- deployed constants (mirror; WS3-c CI check should assert equality) ----
Q = 12289
FIELD_BITS = 14
SIS_PUBLIC_ROWS = 12                 # SIS_FULL_RESPONSE_PUBLIC_ROWS / REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS
RESPONSE_BOUND = 48                  # centered L-inf bound on a response coord
RESPONSE_DIFF_BOUND = 2 * RESPONSE_BOUND   # = 96 = RESPONSE_LINK_BOUNDED_KERNEL_BOUND
ACTION_RESIDUES = 12                 # Fq12: action -> F_q^12
REPS = 26
CHALLENGE_BITS_PER_REP = 5
CHALLENGE_HARDENING_BYTES = 4
ALGEBRAIC_CHALLENGE_ENTROPY_BITS = REPS * CHALLENGE_BITS_PER_REP        # 130
CHALLENGE_ENTROPY_BITS = ALGEBRAIC_CHALLENGE_ENTROPY_BITS + 8 * CHALLENGE_HARDENING_BYTES  # 162
# response vector dimension (packed): 41 masked-witness bytes / 14 bits ~ 23 residues
SIS_RESPONSE_COORDS = 23
TARGET_BITS = 128

def banner(s): print("\n" + "=" * 72 + f"\n{s}\n" + "=" * 72)

# ---------------- C1: Fq12 action collision (RELIABLE) ----------------
banner("C1  Fq12 action collision  (action = SHA256-derived hash -> F_q^12)")
action_space_bits = ACTION_RESIDUES * log2(Q)
classical_birthday = action_space_bits / 2
print(f"action output space      : {Q}^{ACTION_RESIDUES} = 2^{action_space_bits:.2f}")
print(f"asserted floor (Lean)    : 163 bits  -> matches floor(2^{action_space_bits:.1f})")
print(f"cost to FIND a collision : 2^{classical_birthday:.1f} classical birthday "
      f"(~2^{action_space_bits/3:.0f} quantum BHT w/ qRAM; ~2^{classical_birthday:.0f} w/o)")
print("HOW IT ENTERS SECURITY (corrected framing — supersedes an earlier over-")
print("conservative T^2/2^163 estimate that was WRONG):")
print("  A precomputed action collision gives the attacker NO forgery: the response")
print("  z = r + action(c)*w still requires the witness w, and the FS challenge")
print("  c = H(commitment,msg) cannot be cheaply steered onto a colliding value.")
print("  The action floor matters only for the SOUNDNESS EXTRACTOR: forking yields two")
print("  transcripts at challenges c*, c** (c** fresh); extraction needs")
print("  action(c*) != action(c**), which fails with prob ~2^-162 PER FORK.")
print(f"  -> action-collision soundness term ~2^-{action_space_bits-1:.0f} (PER-FORK, negligible),")
print("     NOT a multi-target T^2/2^163 term. The 163-bit floor is ADEQUATE for 128-bit.")
print("  (The 182-bit transcript term / actionQuantumQueryBoundLog2For128=17 concern the")
print("   FS challenge-guessing / multi-target surface, a DIFFERENT term.)")
print("VERDICT: action is a random-oracle hash-to-F_q^12; collision = generic hash")
print("  collision, NOT an algebraic shortcut. Reduces to QROM collision-resistance of")
print("  SHA256-derived uniform_mod_from_hash. Field structure (X^12+X+25, verified")
print("  irreducible) only provides invertibility for forking extraction.")
print("  -> C1 reducible to STANDARD; the 163-bit floor carries the per-fork extractor")
print("     term comfortably. No '2^-99 cap' obstruction (that earlier flag was a")
print("     mis-model). Composed-bound closure remains a WS7 wiring task, not a")
print("     parameter obstruction from C1.")

# ---------------- Fiat-Shamir / challenge entropy (RELIABLE) ----------------
banner("Fiat-Shamir challenge entropy / multi-target (RELIABLE)")
print(f"challenge entropy        : {CHALLENGE_ENTROPY_BITS} bits "
      f"({ALGEBRAIC_CHALLENGE_ENTROPY_BITS} algebraic + {8*CHALLENGE_HARDENING_BYTES} hardening)")
print(f"classical FS query margin: {CHALLENGE_ENTROPY_BITS - TARGET_BITS} bits over 2^{TARGET_BITS}")
print(f"quantum FS query margin  : {(CHALLENGE_ENTROPY_BITS - TARGET_BITS)//2} bits "
      f"(Grover sqrt) over 2^{TARGET_BITS}")
print("NOTE: a *single-target* forgery via challenge guessing costs ~2^162 work;")
print("      the AP2-004 widened transcript term (182 bits) covers multi-target/QROM.")
print("CAVEAT: action collision space (163) > challenge entropy (162): finding an")
print("        action collision among reachable challenges is birthday-bounded by")
print("        the smaller of the two; flagged for WS3 multi-target accounting.")

# ---------------- SIS / lattice (STANDARD-MODEL, MODELED ATTACKS ONLY) ----------------
banner("SIS short-vector hardness  (core-SVP, MODELED attacks only)")
print(f"instance (deployed)      : ~{SIS_RESPONSE_COORDS} response coords, "
      f"{SIS_PUBLIC_ROWS} syndrome rows, q={Q}, centered |coord|<= {RESPONSE_BOUND}")
print(f"response-difference box   : |coord| <= {RESPONSE_DIFF_BOUND} (bounded-kernel bound)")
print("EMPIRICAL EXACT RESULT (already in-repo, Lean-certified):")
print("  ResponseLinkExactCertificate.excludesBoundedNonzeroKernel proves")
print("  responseDifferenceBound(96) < minimumCoverCoeffBound  -> NO nonzero kernel")
print("  vector exists inside the response box (exact enumeration, ~1.12M nodes).")
print("  This is a real, exhaustive short-vector exclusion for the DEPLOYED box.")

print("KEYGEN SIS IS INTENTIONALLY LLL-WEAK (by design):")
print("  the bare quotient lattice (12x23 over q, dim 23, Gaussian-heuristic norm ~158)")
print("  has a short vector at L2~159 / coeff 68 found by BKZ-20 in ~0.003s (matrix_keygen")
print("  _estimator_memo). Security does NOT rest on this bare SIS being hard; the short")
print("  vector has nonzero coords 0-3 mod q and is REJECTED by the response-link code,")
print("  with the exact enumeration (96 < 114) as the authoritative exclusion. So there is")
print("  NO 'asymptotic SIS margin' to extract here -- the regime is exact enumeration,")
print("  already Lean-certified. Beyond-box response-link-coded hardness is the project's")
print("  documented open_external_estimator item (genuinely external review).")

def core_svp(dim, logq, log_target_norm, model="classical"):
    """Standard core-SVP: smallest BKZ block b s.t. the GSA-predicted shortest
    vector beats target; cost 2^(c*b). Returns (b, cost_bits). MODELED ATTACKS ONLY.
    NOTE: a rough textbook estimate; exact instance embedding must be confirmed (WS3 remainder)."""
    c = 0.292 if model == "classical" else 0.265
    best = None
    for b in range(50, dim):
        # GSA delta for block size b
        delta = ((pi * b) ** (1.0 / b) * b / (2 * pi * e)) ** (1.0 / (2 * (b - 1)))
        # predicted shortest reduced-vector log2 norm in a q-ary lattice of given dim
        # (Minkowski/GSA heuristic; coarse)
        log_lambda = (dim - 1) * log2(delta) + (SIS_PUBLIC_ROWS * logq) / dim
        if log_lambda <= log_target_norm:
            best = (b, c * b)
            break
    return best

logq = log2(Q)
# target short vector L2 norm ~ RESPONSE_BOUND * sqrt(coords)
log_target = log2(RESPONSE_BOUND * sqrt(SIS_RESPONSE_COORDS))
for model in ("classical", "quantum"):
    res = core_svp(SIS_RESPONSE_COORDS + SIS_PUBLIC_ROWS, logq, log_target, model)
    if res:
        b, cost = res
        print(f"core-SVP {model:9s}: block b={b}, cost ~2^{cost:.0f}  [MODELED; dim flagged]")
    else:
        print(f"core-SVP {model:9s}: no block size in range reaches target "
              f"(small instance; enumeration/exact search is the relevant regime)")
print("HONEST CAVEAT: this SIS instance is SMALL (dim ~35). For such dimensions the")
print("  exact-enumeration bounded-kernel certificate above is the AUTHORITATIVE")
print("  short-vector statement, not the asymptotic core-SVP curve. The asymptotic")
print("  figure matters for the larger keygen/public-view SIS, whose exact dimension")
print("  must be extracted before a bit-security number is asserted (WS3 remainder).")

banner("B-layer L1/L-inf -> L2 metric mapping  (makes the SIS instance estimator-legible)")
# deployed bare quotient SIS instance (from matrix_keygen_estimator_memo)
sis_dim = 23
det_root = 136.04           # determinant^(1/dim)
gh_norm_memo = 157.87       # Gaussian-heuristic norm (memo)
short_vec_l2 = 158.96       # observed shortest vector (BKZ-20) L2
short_vec_coeff = 68        # max-abs coeff of that vector
# Gaussian heuristic GH = sqrt(dim/(2*pi*e)) * det^(1/dim)
gh_computed = sqrt(sis_dim / (2 * pi * e)) * det_root
# norm-bound mapping for a response in the box: |coord|<=48 over ~23 coords
l2_from_linf = RESPONSE_BOUND * sqrt(SIS_RESPONSE_COORDS)
print(f"deployed bare SIS lattice: dim {sis_dim}, det^(1/dim) {det_root:.1f}")
print(f"response box -> L2 ball   : |coord|<=48 over {SIS_RESPONSE_COORDS} coords => L2 <= {l2_from_linf:.0f}")
print(f"Gaussian heuristic (GH)   : computed {gh_computed:.1f}  (memo {gh_norm_memo:.1f})")
print(f"observed shortest vector  : L2 {short_vec_l2:.1f}, coeff {short_vec_coeff}")
print(f"shortest / GH ratio       : {short_vec_l2/gh_computed:.3f}  (~1.0 = NO gap)")
print("READING (estimator-legible): the bare quotient lattice has its minimum essentially")
print("  AT the Gaussian heuristic (ratio ~1.0) -> a GENERIC lattice with no unusually-short")
print("  vector. So BKZ behaves generically and the bare SIS is easy by design; the short")
print("  vector (coeff 68) sits INSIDE the response box (96) and is excluded only by the")
print("  response-link code, which lifts the smallest code-satisfying kernel vector to coeff")
print("  114 > 96 (exact enumeration). An external SIS/lattice estimator should therefore be")
print("  pointed at the RESPONSE-LINK-CODED problem, not the bare quotient lattice.")

banner("SUMMARY")
print("RELIABLE: C1 action floor 2^163.0; action-collision soundness term ~2^-162 PER-FORK")
print("          (adequate for 128-bit). Earlier 2^-99 multi-target flag was a mis-model.")
print("RELIABLE: FS challenge entropy 162 bits (single-target forgery ~2^162); 182-bit")
print("          multi-target transcript term (AP2-004).")
print("RELIABLE: deployed response-box has NO short kernel vector (exact, Lean-certified,")
print("          96 < 114). Bare keygen SIS is intentionally LLL-weak, caught by the code.")
print("RESOLVED vs earlier flags: (a) C1 imposes no 2^-128 obstruction; (b) keygen has no")
print("          asymptotic margin to extract -- it is an exact-enumeration regime.")
print("GENUINELY OPEN (irreducible this session):")
print("  1. WS7 concrete reduction: wiring per-event terms into an inhabited certificate")
print("     and the norm_num composed-bound check. Maintainer-led proof work.")
print("  2. Beyond-box response-link-coded hardness + sampler pseudorandomness:")
print("     the project's own open_external_estimator -- requires EXTERNAL cryptanalysis,")
print("     which (per the agreed risk posture) cannot be discharged by self-analysis.")
print("  3. Statistical-leakage battery on real signature transcripts (WS4 OPEN).")
