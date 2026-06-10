#!/usr/bin/env python3
"""WS4 #3 beyond-box kernel structure hunt.

Reconstructs the q=12289 12x23 full-response SIS matrix and the combined
intersection lattice (coords {0,3} fixed to 0 mod q), then probes for
exploitable structure JUST OUTSIDE the modeled L-infinity box (coeff > 96):

(a) gap-region census: how many lattice vectors have L-inf coeff in (96, K]?
(b) algebraic / structural regularity (repeated local blocks, Vandermonde,
    sublattice with short generators, automorphisms)
(c) stronger reduction than the cert's LLL/BKZ-23: BKZ with larger block, with
    pruning + multiple tours, and SVP via enumeration to find the TRUE L-inf
    minimum (not just Euclidean-shortest).

It cross-checks against the committed certificate hashes before drawing any
conclusion.
"""
from __future__ import annotations
import hashlib, json, math, time
from itertools import combinations

Q = 12289
ROWS = 12
COLS = 23
DOMAIN = b"Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1"
BOUND = 96

CERT = json.load(open(
    "research/cryptanalysis/signature_readiness_20260516/"
    "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"))


def matrix_entry(row, col):
    rejection_limit = 65535 - (65535 % Q)
    counter = 0
    while True:
        h = hashlib.sha256()
        h.update(DOMAIN); h.update(row.to_bytes(2, "little"))
        h.update(col.to_bytes(2, "little")); h.update(counter.to_bytes(4, "little"))
        digest = h.digest()
        for idx in range(0, len(digest), 2):
            v = int.from_bytes(digest[idx:idx+2], "little")
            if v < rejection_limit:
                return v % Q
        counter += 1


def full_response_matrix():
    return [[matrix_entry(r, c) for c in range(COLS)] for r in range(ROWS)]


def matrix_transcript_sha256(matrix):
    h = hashlib.sha256()
    for row in matrix:
        for v in row:
            h.update(v.to_bytes(2, "little"))
    return h.hexdigest()


def centered_lift(v):
    return v - Q if v > Q // 2 else v


def rref_mod_q(rows):
    work = [r[:] for r in rows]; rank = 0; pivots = []
    for col in range(COLS):
        pivot = next((r for r in range(rank, len(work)) if work[r][col] % Q), None)
        if pivot is None: continue
        work[rank], work[pivot] = work[pivot], work[rank]
        inv = pow(work[rank][col] % Q, -1, Q)
        work[rank] = [(v*inv) % Q for v in work[rank]]
        for r in range(len(work)):
            if r == rank or work[r][col] % Q == 0: continue
            f = work[r][col] % Q
            work[r] = [(work[r][i]-f*work[rank][i]) % Q for i in range(COLS)]
        pivots.append(col); rank += 1
        if rank == len(work): break
    return work, pivots


def integer_intersection_basis(matrix, coords):
    rows = [r[:] for r in matrix]
    for coord in coords:
        row = [0]*COLS; row[coord] = 1; rows.append(row)
    rref, pivots = rref_mod_q(rows)
    free = [c for c in range(COLS) if c not in pivots]
    basis = []
    for fc in free:
        vec = [0]*COLS; vec[fc] = 1
        for r, pc in enumerate(pivots):
            vec[pc] = (-rref[r][fc]) % Q
        basis.append([centered_lift(v) for v in vec])
    for pc in pivots:
        vec = [0]*COLS; vec[pc] = Q; basis.append(vec)
    return basis, len(pivots), len(free)


def linf(v): return max(abs(x) for x in v)
def l2sq(v): return sum(x*x for x in v)


def main():
    coords = CERT["coords"]
    matrix = full_response_matrix()
    mt = matrix_transcript_sha256(matrix)
    print("matrix_transcript_sha256 match:", mt == CERT["matrix_transcript_sha256"], mt)

    basis, rank, kdim = integer_intersection_basis(matrix, coords)
    print("combined_constraint_rank:", rank, "kernel_dim:", kdim,
          "(cert:", CERT["combined_constraint_rank"], CERT["combined_kernel_dimension"], ")")

    from fpylll import GSO, LLL, BKZ, IntegerMatrix, Enumeration, EnumerationError
    mat = IntegerMatrix.from_matrix([r[:] for r in basis])
    LLL.reduction(mat, delta=0.99)
    rb = [[int(mat[i, j]) for j in range(COLS)] for i in range(mat.nrows)]
    print("LLL min L-inf among basis rows:", min(linf(r) for r in rb if any(r)))

    # ---- (c) stronger reduction: BKZ escalating block sizes + tours ----
    best_l2 = min((r for r in rb if any(r)), key=l2sq)
    print("\n[c] stronger reduction")
    for bs in (24, 30, 40, 23):
        m2 = IntegerMatrix.from_matrix([r[:] for r in basis])
        par = BKZ.Param(min(bs, COLS), max_loops=8, flags=BKZ.AUTO_ABORT)
        BKZ.reduction(m2, par)
        rows = [[int(m2[i, j]) for j in range(COLS)] for i in range(m2.nrows)]
        sh = min((r for r in rows if any(r)), key=l2sq)
        print(f"  BKZ-{min(bs,COLS)}: shortest L2^2={l2sq(sh)} Linf={linf(sh)}")
        if l2sq(sh) < l2sq(best_l2):
            best_l2 = sh
    print("  best L2 vector: L2^2=%d Linf=%d (L2=%.1f)" % (l2sq(best_l2), linf(best_l2), math.sqrt(l2sq(best_l2))))
    gg = GSO.Mat(IntegerMatrix.from_matrix([r[:] for r in basis]));
    LLL.reduction(gg.B); gg.update_gso()
    logdet = sum(math.log(gg.get_r(i, i))/2 for i in range(gg.B.nrows) if gg.get_r(i, i) > 0)
    n = gg.B.nrows
    gh = math.exp(logdet/n) * math.sqrt(n/(2*math.pi*math.e))
    print("  Gaussian-heuristic norm:", round(gh, 1), " best/GH ratio:", round(math.sqrt(l2sq(best_l2))/gh, 3))

    # ---- (c2) TRUE L-inf minimum via enumeration of Euclidean ball, track L-inf ----
    # Enumerate all nonzero vectors with L2 <= some radius and report the min L-inf.
    print("\n[c2] enumerate short vectors, track min L-inf (not just L2)")
    m3 = IntegerMatrix.from_matrix([r[:] for r in basis])
    LLL.reduction(m3, delta=0.99)
    g = GSO.Mat(m3); g.update_gso()
    # radius: enumerate up to L2 of the box-corner is too big; use radius that
    # captures the LLL-short vectors region. Use 1.3x shortest^2.
    radius2 = int(l2sq(best_l2) * 1.6)
    enum = Enumeration(g, nr_solutions=4000)
    min_linf = None; cnt = 0; gap_in_96_114 = 0; below97 = 0
    try:
        sols = enum.enumerate(0, m3.nrows, radius2, 0)
        for dist, coeffs in sols:
            cnt += 1
            vec = [0]*COLS
            for i, c in enumerate(coeffs):
                if c:
                    for j in range(COLS):
                        vec[j] += int(c)*int(m3[i, j])
            if not any(vec): continue
            li = linf(vec)
            if min_linf is None or li < min_linf:
                min_linf = li
            if li <= BOUND: below97 += 1
            if BOUND < li <= 114: gap_in_96_114 += 1
    except EnumerationError as e:
        print("  enum error:", e)
    print(f"  enumerated {cnt} candidates up to L2^2={radius2}")
    print(f"  min L-inf found by enumeration: {min_linf}")
    print(f"  vectors with L-inf <= {BOUND}: {below97}")
    print(f"  vectors with L-inf in (96,114]: {gap_in_96_114}")

    # ---- (a) gap-region census using the cert's own exhaustive cover-ball enum ----
    print("\n[a] gap-region census (from committed exact-enum cert)")
    print("  cover ball radius^2:", CERT["euclidean_cover_radius_squared"])
    print("  nonzero vectors in cover ball:", CERT["nonzero_vectors_in_cover_ball"])
    print("  status:", CERT["status"], "(infeasible = NO vector with Linf<=96)")
    print("  shortest-in-ball (LLL) Linf:", CERT["shortest_vector_in_cover_ball"]["coeff_bound"])

    # ---- (b) structural / algebraic regularity ----
    print("\n[b] structural regularity probe")
    # repeated-block / Vandermonde: check matrix columns/rows for repetition,
    # check if intersection lattice has a sublattice with a short generator that
    # repeats a small local pattern.
    rb_sorted = sorted((r for r in rb if any(r)), key=l2sq)
    short = rb_sorted[0]
    print("  shortest reduced row support pattern (nonzero idx):",
          [i for i, x in enumerate(short) if x])
    # supports: do short vectors share a common small support? (would signal
    # a low-weight code structure exploitable below the box).
    sups = [tuple(i for i, x in enumerate(r) if x) for r in rb_sorted[:9]]
    common = set(sups[0])
    for s in sups[1:]: common &= set(s)
    print("  common support across 9 shortest basis rows:", sorted(common))
    print("  support sizes of 9 shortest rows:", [len(s) for s in sups])
    # automorphism: is there a coordinate permutation fixing the lattice that
    # could shorten? Check column-multiset of matrix for symmetry.
    cols = [tuple(matrix[r][c] for r in range(ROWS)) for c in range(COLS)]
    dup = len(cols) - len(set(cols))
    print("  duplicate matrix columns (would give trivial short kernel vec):", dup)
    # gcd structure of shortest vector entries
    from math import gcd
    g0 = 0
    for x in short:
        g0 = gcd(g0, abs(x))
    print("  gcd of shortest reduced row entries:", g0)


if __name__ == "__main__":
    main()
