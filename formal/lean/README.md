# Lean 4 + Mathlib — Math + Reduction-Skeleton Layer

Status: ACTIVE_SUPPORT

This package supports current Path 2 proof closure work, but product authority
starts in `../../spec/README.md`, ADRs, and `../../TRACKER.md`.

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
discharging the per-event bounds. EC-SPIKE returned "go", so EasyCrypt may own
game-hop bookkeeping while Lean retains the math; seam 1 then governs.

**Guards in place:** `lake build`, `scripts/audit_theorems.lean` (axiom/sorry
audit), and the param-sync check, all in CI (`.github/workflows/ci.yml`).
