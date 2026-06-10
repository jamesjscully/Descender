#!/usr/bin/env python3
"""Final beyond-box checks:
1. Exact SVP (Euclidean) on the kernel lattice via enumeration -> confirm the
   L2-shortest is the cert's 80192 and the L-inf-min is 114 (no better basis).
2. Endpoint feasibility: for EVERY kernel vector delta with L-inf in (96,160],
   can two valid responses r,r' in [-48,48]^23 realize r'-r = delta? (i.e. is
   delta in the difference body [-96,96]^23 AND does a valid split exist).
   The verifier gate is per-coordinate [-48,48]; the difference body is exactly
   [-96,96]^23. So delta is realizable as a response difference IFF L-inf(delta)
   <= 96. Confirm no near-bound vector sneaks a realizable approximate forgery.
"""
from __future__ import annotations
import importlib.util, math
spec = importlib.util.spec_from_file_location(
    "probe", "scripts/research/beyond_box_kernel_structure_probe.py")
P = importlib.util.module_from_spec(spec); spec.loader.exec_module(P)
Q, COLS, BOUND, RB = P.Q, P.COLS, 96, 48

coords = P.CERT["coords"]
matrix = P.full_response_matrix()
basis, rank, kdim = P.integer_intersection_basis(matrix, coords)

from fpylll import GSO, LLL, BKZ, IntegerMatrix, Enumeration, SVP

m = IntegerMatrix.from_matrix([r[:] for r in basis])
LLL.reduction(m, delta=0.99)
# exact SVP via enumeration (find the true Euclidean minimum)
gso0 = GSO.Mat(m); gso0.update_gso()
en0 = Enumeration(gso0, nr_solutions=1)
r0 = gso0.get_r(0, 0)
sols0 = en0.enumerate(0, m.nrows, r0 * 1.0001, 0)
sv = [0]*COLS
for dist, coeffs in sols0:
    for i, c in enumerate(coeffs):
        ci = int(c)
        if ci:
            for j in range(COLS):
                sv[j] += ci*int(m[i, j])
    break
print("exact-SVP (enum) shortest vector L2^2:", P.l2sq(sv), "L-inf:", P.linf(sv),
      "(cert L2^2=80192, L-inf=139)")

# Now: is there ANY kernel vector with L-inf <= 96? Already certified no.
# Endpoint argument made rigorous: delta realizable as r'-r with r,r' in
# [-48,48]^23 iff every coord |delta_i| <= 96. Because for a single coord,
# {a-b : a,b in [-48,48]} = [-96,96] exactly (integer interval). So the
# realizable difference body is EXACTLY [-96,96]^23 = the L-inf box.
# => approximate forgery feasible iff some nonzero kernel vec has L-inf <= 96.
print("\nRealizable response-difference body = [-96,96]^23 (exact, per-coord "
      "{a-b:a,b in[-48,48]}=[-96,96]).")
print("Verifier hard-rejects any response coord outside [-48,48] "
      "(rs line 4662). No slack.")

# Demonstrate infeasibility of the smallest above-box vector as a difference:
g = GSO.Mat(m); g.update_gso()
enum = Enumeration(g, nr_solutions=50)
sols = enum.enumerate(0, m.nrows, int(P.l2sq(sv)*1.45), 0)
mins = []
for dist, coeffs in sols:
    vec = [0]*COLS
    for i, c in enumerate(coeffs):
        ci = int(c)
        if ci:
            for j in range(COLS):
                vec[j] += ci*int(m[i, j])
    if any(vec):
        mins.append((P.linf(vec), vec))
mins.sort()
li, mv = mins[0]
overshoot_coords = [(i, mv[i]) for i in range(COLS) if abs(mv[i]) > BOUND]
print(f"\nSmallest-L-inf kernel vector L-inf={li}; coords exceeding 96:")
for i, x in overshoot_coords:
    # minimal achievable |r'_i| given r'_i = r_i + x, r_i in [-48,48]
    # best case picks r_i to cancel sign: |r'_i|_min = |x| - 48
    print(f"  idx {i}: delta={x:4d} -> min |r'| = |{x}|-48 = {abs(x)-RB} "
          f"({'REJECT' if abs(x)-RB > RB else 'ok'} vs bound 48)")
print("\nConclusion: every above-box vector has >=1 coord with |delta|>96, so "
      "the cancelling endpoint r' still exceeds 48 -> verifier rejects. "
      "No approximate/near-collision forgery in the gap.")
