#!/usr/bin/env python3
"""Fine gap-region census: enumerate ALL kernel-lattice vectors with L-inf in
(96, K] and analyze whether a near-bound vector enables an approximate forgery.

Uses fpylll enumeration with a generous L2 radius then filters by L-inf, plus a
direct exhaustive box-difference argument.
"""
from __future__ import annotations
import importlib.util, math, json
spec = importlib.util.spec_from_file_location(
    "probe", "scripts/research/beyond_box_kernel_structure_probe.py")
P = importlib.util.module_from_spec(spec); spec.loader.exec_module(P)

Q, COLS, BOUND = P.Q, P.COLS, P.BOUND
coords = P.CERT["coords"]
matrix = P.full_response_matrix()
basis, rank, kdim = P.integer_intersection_basis(matrix, coords)

from fpylll import GSO, LLL, IntegerMatrix, Enumeration

m = IntegerMatrix.from_matrix([r[:] for r in basis])
LLL.reduction(m, delta=0.99)
g = GSO.Mat(m); g.update_gso()

# Enumerate generously: cover all vectors with L-inf <= 200 we can reach.
# A vector with L-inf <= K over 21-support has L2^2 <= 21*K^2.
K = 160
radius2 = 21 * K * K
enum = Enumeration(g, nr_solutions=200000)
buckets = {}
gap_vectors = []  # L-inf in (96,130]
linf_hist = {}
seen = set()
sols = enum.enumerate(0, m.nrows, radius2, 0)
for dist, coeffs in sols:
    vec = [0]*COLS
    for i, c in enumerate(coeffs):
        ci = int(c)
        if ci:
            for j in range(COLS):
                vec[j] += ci*int(m[i, j])
    if not any(vec):
        continue
    # canonicalize sign
    for x in vec:
        if x != 0:
            if x < 0:
                vec = [-y for y in vec]
            break
    key = tuple(vec)
    if key in seen:
        continue
    seen.add(key)
    li = P.linf(vec)
    linf_hist[li] = linf_hist.get(li, 0) + 1
    if BOUND < li <= 130:
        gap_vectors.append((li, P.l2sq(vec), vec))

print("enumerated distinct (sign-canonical) vectors:", len(seen))
print("min L-inf overall:", min(linf_hist))
below = sum(c for li, c in linf_hist.items() if li <= BOUND)
print(f"vectors with L-inf <= {BOUND}: {below}")
for hi in (100, 105, 110, 114, 120, 130, 140, 160):
    c = sum(cnt for li, cnt in linf_hist.items() if BOUND < li <= hi)
    print(f"  vectors with L-inf in (96,{hi}]: {c}")

gap_vectors.sort()
print("\nSmallest-L-inf vectors above box (top 8):")
for li, l2, vec in gap_vectors[:8]:
    print(f"  Linf={li} L2^2={l2} support={sum(1 for x in vec if x)} vec={vec}")

# ---- approximate-forgery semantics check ----
# A forgery needs a SECOND valid response r' with the SAME public data, i.e.
# r' - r is a nonzero kernel vector with L-inf <= BOUND (the verifier enforces
# every coordinate <= BOUND). The gap analysis shows the smallest such delta has
# L-inf 114. An "approximate"/near-collision r' would have r'-r with L-inf 114,
# so r' = r + delta has at least one coordinate exceeding the response box by
# (114 - 96) = 18 ... UNLESS r already sits >=18 below the bound on that coord.
# Quantify: how far below 96 must r be on delta's max-coord to absorb it?
li_min = gap_vectors[0][0]
print(f"\nApproximate-forgery margin: smallest kernel delta L-inf = {li_min}")
print(f"  overshoot over box bound = {li_min - BOUND} on >=1 coordinate")
# count how many coords of the min vector are at the extreme
mv = gap_vectors[0][2]
extremes = [i for i, x in enumerate(mv) if abs(x) == li_min]
near = sum(1 for x in mv if abs(x) > BOUND)
print(f"  coords of min-delta exceeding 96: {near}; at the max ({li_min}): {len(extremes)} -> idx {extremes}")
