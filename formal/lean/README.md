# Lean 4 + Mathlib — Math + Reduction-Skeleton Layer

The Lean development lives in `../../Descender/` (this is a pointer, not a copy).

**Owns:** the supporting mathematics and the security-reduction skeleton —
- lattice/SIS/finite-field facts and the verified concrete-security estimate
  (`research/cryptanalysis/`, `scripts/research/`),
- the assumption ledger and the C1→QROM-hash-collision reduction,
- the union-bound + pricing layer (`Descender/Path2AcceptedSpendProbability.lean`)
  and the **closure skeleton** (`Descender/Path2AcceptedSpendClosure.lean`):
  `successProb ≤ 2⁻¹²⁸` proven from explicit, named per-event bounds.

**Why Lean keeps this layer:** Mathlib is a decisive advantage for the
field/lattice/real-analysis math the scheme leans on; reproving it elsewhere is
pure cost.

**Active work:** the concrete trace model `G` that feeds the closure skeleton +
discharging the per-event bounds. If EC-SPIKE returns "go", the *reduction* may
migrate to EasyCrypt while Lean retains the math; seam 1 then governs.

**Guards in place:** `lake build`, `scripts/audit_theorems.lean` (axiom/sorry
audit), and the param-sync check, all in CI (`.github/workflows/ci.yml`).
