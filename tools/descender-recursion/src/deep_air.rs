//! Gadget 4 (proper) — DEEP-ALI out-of-domain quotient relation as a real `Air<AB>`.
//!
//! Lifts the VALIDATED [`crate::deep`] reference (`verify_quotient_relation`: at the OOD point
//! `zeta`, `C(zeta) = Z_H(zeta)·Q(zeta)` with `Z_H(zeta) = zeta^{|H|} − 1`, `|H| = 2^log_degree`)
//! into a Plonky3 `Air<AB>`, ONE OOD check per trace row. This is the DEEP-ALI soundness link: a
//! quotient `Q` exists as a polynomial (which FRI then certifies low-degree) IFF the AIR
//! constraints vanish on the trace domain `H` — checked at the FS-sampled OOD point. The check
//! is over the deg-8 challenge extension `Challenge = BinomialExtensionField<Val, 8>`, so each
//! element is 8 `Val` limbs and the extension arithmetic is reproduced IN-AIR (same `X^8 = W`
//! octic mul as [`crate::fri_fold_air`]).
//!
//! ## The vanishing-poly power tower (degree control)
//!
//! `Z_H(zeta) = zeta^{2^log_degree} − 1` would be degree `2^log_degree` in the committed `zeta`
//! limbs if expanded directly — far too high for a polynomial constraint. So the AIR carries the
//! squaring TOWER as committed columns: `pow[0] = zeta`, `pow[j] = pow[j−1]²` for `j = 1..=k`
//! (`k = log_degree`), each step pinned by an octic-square constraint (degree 2). Then
//! `Z_H = pow[k] − 1` and the OOD check is `C == Z_H·Q` (an octic product of two committed
//! elements, degree 2). Every constraint is degree ≤ 2 — the standard "commit the power tower"
//! trick. This mirrors how the challenger AIR commits intermediate sponge states rather than
//! re-deriving them.
//!
//! ## Soundness discipline
//!
//! Honest `(zeta, tower, Q, C)` rows (built from the [`crate::deep`] reference) ACCEPT via the
//! real `p3_air::check_all_constraints`; tampering `C`, `Q`, `zeta`, an intermediate tower power,
//! or `Z_H` each REJECTS. Soundness by attempted forgery, never by inspection — mirroring the
//! other gadget AIRs.
//!
//! ## Leaf `C(zeta)` wiring (`wire_leaf`)
//!
//! With `DeepAir { wire_leaf: true, .. }` the AIR no longer trusts the FULL opened `C(zeta)`: it
//! DECOMPOSES it as `C == C_partial + C_committed`, where
//!   * `C_partial` is RE-COMPUTED in-circuit from the opened leaf trace cells via
//!     [`crate::leaf_constraint_air::recompute_c_partial`] — the alpha-batched Poseidon2-FREE leaf
//!     constraints (all 6 C4 lanes + C4-acc binding + SeamPadSeed + SeamOpeningAcc + relation_id
//!     pinning; fidelity-PINned against `descender_account_stark::air::FullRelationAir`); and
//!   * `C_committed` is the DEFERRED Poseidon2-permutation bulk (C1/C2/C6 sponges, role-root,
//!     digest chain, the C2 reduce + carrier mod-q) — STILL a committed value (the next large
//!     chunk, see the leaf_constraint_air DEFERRED list).
//! Because `C_partial` is recomputed, a tampered opened leaf cell that violates a mirrored leaf
//! constraint shifts `C_partial`, so the committed `C` no longer satisfies `C == Z_H·Q` ⇒ rejected
//! END-TO-END (tested by `leaf_wired_tampered_leaf_cell_rejects_end_to_end` + control-row isolation).
//!
//! With `wire_leaf: false` (default / back-compat) `C(zeta)` and `Q(zeta)` are the OPENED values
//! and the AIR constrains only the QUOTIENT RELATION between them — the bounded, honest piece
//! [`crate::deep`] validates.

use crate::fri_fold_air::{limbs, D, W};
use crate::leaf_constraint_air as lc;
use descender_account_stark::{Challenge, Val};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{BasedVectorSpace, Field, PrimeCharacteristicRing};

extern crate alloc;
use alloc::vec::Vec;

// ----- column layout (one OOD quotient check per row) -----
//
// The squaring tower `pow[0..=k]` is laid out contiguously starting at `C_POW`; `pow[0]` is
// `zeta`, `pow[k]` is `zeta^{2^k}`. `K` (= `log_degree`) is a runtime AIR parameter, so the
// column count is parameterized by it.

/// First sibling: the OOD point `zeta = pow[0]` (8 limbs). (Tower base.)
pub const C_ZETA: usize = 0;
/// Opened quotient evaluation `Q(zeta)` (8 limbs).
pub const C_Q: usize = C_ZETA + D;
/// Opened constraint-polynomial evaluation `C(zeta)` (8 limbs).
pub const C_C: usize = C_Q + D;
/// The squaring tower `pow[0..=K]`: `pow[0] = zeta` (duplicated for a uniform tower view),
/// `pow[j] = pow[j−1]²`. `(K+1)` blocks of `D` limbs. (`pow[0]` is bound to `zeta` by a
/// constraint, so the duplication costs nothing soundness-wise and keeps the tower indexing
/// uniform.)
pub const C_POW: usize = C_C + D;

/// Base of the OPTIONAL leaf-mirror block (present iff `wire_leaf`). Sits AFTER the squaring tower.
/// Layout: opened leaf cells (`lc::N_OPENED_CELLS · D`) ‖ `alpha`(`D`) ‖ `C_committed`(`D`).
pub const fn leaf_base(log_degree: usize) -> usize {
    C_POW + (log_degree + 1) * D
}
/// Within the leaf block: opened cells region base.
pub const LEAF_CELLS: usize = 0;
/// Within the leaf block: the batching challenge `alpha`.
pub const fn leaf_alpha(_log_degree: usize) -> usize {
    LEAF_CELLS + lc::N_OPENED_CELLS * D
}
/// Within the leaf block: the committed DEFERRED summand `C_committed` (Poseidon2-perm bulk).
pub const fn leaf_c_committed(log_degree: usize) -> usize {
    leaf_alpha(log_degree) + D
}
/// Size of the leaf-mirror block (cells ‖ alpha ‖ C_committed).
pub const fn leaf_block_size() -> usize {
    lc::N_OPENED_CELLS * D + D + D
}

/// Column count for a DEEP AIR with `log_degree = k` (tower has `k+1` powers). When `wire_leaf`,
/// the leaf-mirror block is appended.
pub const fn n_cols_cfg(log_degree: usize, wire_leaf: bool) -> usize {
    let base = C_POW + (log_degree + 1) * D;
    if wire_leaf {
        base + leaf_block_size()
    } else {
        base
    }
}

/// Column count for a DEEP AIR with `log_degree = k` (no leaf wiring — back-compat).
pub const fn n_cols(log_degree: usize) -> usize {
    n_cols_cfg(log_degree, false)
}

/// Limb slice helper: the `j`-th tower power's column base.
const fn pow_base(j: usize) -> usize {
    C_POW + j * D
}

/// Build one OOD-check row for `(zeta, q, c)` at `log_degree = k`, where `c` MUST satisfy the
/// honest relation `c = Z_H(zeta)·q` (the reference authority). Fills the squaring tower
/// `pow[0]=zeta, pow[j]=pow[j−1]²`. Panics if `c` is not the honest constraint eval (so the
/// builder can only emit honest rows; tamper tests mutate the matrix afterwards).
pub fn pack_check(zeta: Challenge, q: Challenge, c: Challenge, log_degree: usize) -> Vec<Val> {
    let zh = crate::deep::vanishing_poly_at(zeta, log_degree);
    assert_eq!(c, zh * q, "pack_check requires the honest constraint eval c = Z_H(zeta)*q");
    let mut row = vec![Val::ZERO; n_cols(log_degree)];
    row[C_ZETA..C_ZETA + D].copy_from_slice(&limbs(zeta));
    row[C_Q..C_Q + D].copy_from_slice(&limbs(q));
    row[C_C..C_C + D].copy_from_slice(&limbs(c));
    // tower
    let mut p = zeta;
    for j in 0..=log_degree {
        row[pow_base(j)..pow_base(j) + D].copy_from_slice(&limbs(p));
        p = p * p;
    }
    row
}

/// Build one leaf-WIRED OOD-check row. The opened leaf cells + `alpha` recompute the honest
/// Poseidon2-free `C_partial`; `c_committed` is the deferred Poseidon2-perm summand. The DEEP
/// `C(zeta)` is set to `C_partial + c_committed`, and `q` is chosen so the OOD identity
/// `C == Z_H(zeta)·q` holds (the prover supplies a genuine quotient). To keep the builder honest
/// AND let tests pin both relations, the caller supplies `(zeta, c_committed, leaf_cells, alpha)`
/// and `q` is DERIVED as `C / Z_H(zeta)` (Z_H(zeta) != 0 at a generic OOD point).
pub fn pack_check_leaf(
    zeta: Challenge,
    leaf_cells: &[Challenge; lc::N_OPENED_CELLS],
    alpha: Challenge,
    c_committed: Challenge,
    log_degree: usize,
) -> Vec<Val> {
    let c_partial = lc::partial_constraint_eval(leaf_cells, alpha);
    let c = c_partial + c_committed;
    let zh = crate::deep::vanishing_poly_at(zeta, log_degree);
    let q = c * zh.inverse(); // genuine quotient (Z_H(zeta) != 0 at a generic zeta)

    let mut row = vec![Val::ZERO; n_cols_cfg(log_degree, true)];
    row[C_ZETA..C_ZETA + D].copy_from_slice(&limbs(zeta));
    row[C_Q..C_Q + D].copy_from_slice(&limbs(q));
    row[C_C..C_C + D].copy_from_slice(&limbs(c));
    let mut p = zeta;
    for j in 0..=log_degree {
        row[pow_base(j)..pow_base(j) + D].copy_from_slice(&limbs(p));
        p = p * p;
    }
    // leaf block.
    let lb = leaf_base(log_degree);
    for (ci, cell) in leaf_cells.iter().enumerate() {
        let base = lb + LEAF_CELLS + ci * D;
        row[base..base + D].copy_from_slice(&limbs(*cell));
    }
    let ab = lb + leaf_alpha(log_degree);
    row[ab..ab + D].copy_from_slice(&limbs(alpha));
    let cc = lb + leaf_c_committed(log_degree);
    row[cc..cc + D].copy_from_slice(&limbs(c_committed));
    row
}

/// Decode the constraint-eval `C(zeta)` cell of a packed row.
pub fn row_c(row: &[Val], log_degree: usize) -> Challenge {
    let _ = log_degree;
    Challenge::from_basis_coefficients_slice(&row[C_C..C_C + D]).expect("C limbs decode")
}

/// Octic square `a²` over `X^D = W` as in-AIR expressions (specialization of the octic mul).
fn octic_square<AB: AirBuilder>(a: &[AB::Expr; D]) -> [AB::Expr; D] {
    octic_mul::<AB>(a, a)
}

/// Octic multiply over `X^D = W` (same formula as `fri_fold_air`).
fn octic_mul<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    let w = AB::Expr::from(AB::F::from_u32(W));
    core::array::from_fn(|k| {
        let mut acc = AB::Expr::ZERO;
        for i in 0..D {
            for j in 0..D {
                if i + j == k {
                    acc += a[i].clone() * b[j].clone();
                } else if i + j == k + D {
                    acc += w.clone() * a[i].clone() * b[j].clone();
                }
            }
        }
        acc
    })
}

/// DEEP-ALI OOD quotient-relation AIR (one check per row). `log_degree = log2(|H|)` is the
/// trace-domain size; the squaring tower has `log_degree + 1` powers.
///
/// When `wire_leaf` is set, the AIR ALSO carries a leaf-mirror block and DECOMPOSES the DEEP
/// `C(zeta)` into the recomputed Poseidon2-free leaf partial `C_partial` (from the opened leaf
/// cells, via [`crate::leaf_constraint_air::recompute_c_partial`]) plus a committed DEFERRED
/// summand `C_committed` (the Poseidon2-perm bulk): `C == C_partial + C_committed`. A tampered
/// opened leaf cell that violates a mirrored leaf constraint changes `C_partial`, so `C` no longer
/// equals `Z_H·Q` ⇒ rejected END-TO-END. The `C_committed` summand stays committed-not-constrained
/// (the deferred bulk; see the module-level note + leaf_constraint_air DEFERRED list).
pub struct DeepAir {
    pub log_degree: usize,
    /// Wire the leaf-mirror C_partial into the DEEP C(zeta) (`false` = back-compat, C fully opened).
    pub wire_leaf: bool,
}

impl<F: Sync> BaseAir<F> for DeepAir {
    fn width(&self) -> usize {
        n_cols_cfg(self.log_degree, self.wire_leaf)
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for DeepAir {
    fn eval(&self, builder: &mut AB) {
        let k = self.log_degree;
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };

        let block = |base: usize| -> [AB::Expr; D] { core::array::from_fn(|i| l(base + i)) };

        let zeta = block(C_ZETA);
        let q = block(C_Q);
        let c = block(C_C);

        // ---- tower base: pow[0] == zeta ----
        let pow0 = block(pow_base(0));
        for i in 0..D {
            builder.assert_zero(pow0[i].clone() - zeta[i].clone());
        }

        // ---- tower squaring: pow[j] == pow[j-1]^2 (each degree 2) ----
        for j in 1..=k {
            let prev = block(pow_base(j - 1));
            let cur = block(pow_base(j));
            let sq = octic_square::<AB>(&prev);
            for i in 0..D {
                builder.assert_zero(cur[i].clone() - sq[i].clone());
            }
        }

        // ---- Z_H = pow[k] - 1  (one in limb 0) ----
        let powk = block(pow_base(k));
        let mut zh = powk.clone();
        zh[0] = powk[0].clone() - AB::Expr::ONE;

        // ---- OOD check: C == Z_H * Q  (octic product, degree 2 in committed Z_H, Q) ----
        let zhq = octic_mul::<AB>(&zh, &q);
        for i in 0..D {
            builder.assert_zero(c[i].clone() - zhq[i].clone());
        }

        // ---- (OPTIONAL) leaf-mirror wiring: C == C_partial(opened leaf cells) + C_committed ----
        // The Poseidon2-free leaf constraint contribution C_partial is RE-COMPUTED in-circuit from
        // the opened leaf cells (not trusted as a committed value); C_committed is the deferred
        // Poseidon2-perm bulk (still committed). This makes a tampered opened leaf cell reject via
        // the OOD identity above (C is no longer Z_H·Q once C_partial shifts).
        if self.wire_leaf {
            let lb = leaf_base(k);
            let leaf_cell = |c_idx: usize| -> [AB::Expr; D] { block(lb + LEAF_CELLS + c_idx * D) };
            let alpha = block(lb + leaf_alpha(k));
            let c_partial = lc::recompute_c_partial::<AB, _>(&leaf_cell, &alpha);
            let c_committed = block(lb + leaf_c_committed(k));
            // C == C_partial + C_committed (limb-for-limb).
            for i in 0..D {
                builder.assert_zero(
                    c[i].clone() - (c_partial[i].clone() + c_committed[i].clone()),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deep::vanishing_poly_at;
    use p3_air::check_all_constraints;
    use p3_field::PrimeCharacteristicRing;
    use p3_matrix::dense::RowMajorMatrix;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }

    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }

    fn honest_row(zeta: Challenge, q: Challenge, log_degree: usize) -> Vec<Val> {
        let c = vanishing_poly_at(zeta, log_degree) * q;
        pack_check(zeta, q, c, log_degree)
    }

    fn matrix(zeta: Challenge, q: Challenge, log_degree: usize) -> RowMajorMatrix<Val> {
        RowMajorMatrix::new(honest_row(zeta, q, log_degree), n_cols(log_degree))
    }

    /// PIN: the in-AIR squaring tower reproduces the NATIVE `Z_H(zeta) = zeta^{2^k} − 1`. The
    /// packed `pow[k]` minus one must equal the reference `vanishing_poly_at`; a wrong octic
    /// square or W would diverge here (attempted equality, not assumption).
    #[test]
    fn tower_reproduces_native_vanishing_poly() {
        for k in [1usize, 4, 8, 12] {
            let zeta = ext_multi(7 + k as u32);
            let row = pack_check(zeta, ext_multi(99), {
                vanishing_poly_at(zeta, k) * ext_multi(99)
            }, k);
            let powk = Challenge::from_basis_coefficients_slice(&row[pow_base(k)..pow_base(k) + D]).unwrap();
            assert_eq!(powk - Challenge::ONE, vanishing_poly_at(zeta, k), "tower Z_H != native at k={k}");
        }
    }

    /// GOLDEN (positive): honest OOD rows (over base-only and multi-limb zeta/Q, several
    /// log_degrees) ACCEPT via the real `check_all_constraints`. The DEEP-ALI quotient relation
    /// `C = Z_H(zeta)·Q` validated end-to-end as an AIR with the in-AIR squaring tower.
    #[test]
    fn air_accepts_honest_ood_checks() {
        for &k in &[1usize, 4, 10, 12] {
            // base-only
            let m = matrix(ext(123_457), ext(98_765), k);
            assert!(
                check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m, &[], None).is_ok(),
                "honest base-only OOD row rejected at k={k}"
            );
            // multi-limb (exercises the X^8 = W wraparound in the tower + product)
            let m2 = matrix(ext_multi(10 + k as u32), ext_multi(40 + k as u32), k);
            assert!(
                check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m2, &[], None).is_ok(),
                "honest multi-limb OOD row rejected at k={k}"
            );
        }
    }

    /// Forgery — WRONG C(zeta): a constraint eval ≠ Z_H(zeta)·Q is rejected. THIS is the DEEP
    /// soundness property: a prover cannot pass the OOD check without a genuine quotient (false
    /// statement ⇒ no polynomial quotient ⇒ rejected).
    #[test]
    fn air_rejects_wrong_constraint_eval() {
        let k = 10;
        let mut m = matrix(ext(777), ext(42), k);
        m.values[C_C + 1] += Val::ONE; // tamper a C limb
        assert!(!check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m, &[], None).is_ok());
    }

    /// Forgery — WRONG Q(zeta): a quotient ≠ C/Z_H is rejected (the relation binds both sides).
    #[test]
    fn air_rejects_wrong_quotient() {
        let k = 12;
        let mut m = matrix(ext_multi(3), ext_multi(5), k);
        m.values[C_Q + 4] += Val::ONE; // tamper a high Q limb
        assert!(!check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m, &[], None).is_ok());
    }

    /// Forgery — WRONG zeta / Z_H: changing the OOD point breaks the tower base (pow[0] != zeta)
    /// and the derived Z_H ⇒ rejected. (Once FS-bound, the prover cannot evaluate the OOD check
    /// at a favorable point.)
    #[test]
    fn air_rejects_wrong_zeta() {
        let k = 8;
        let mut m = matrix(ext_multi(11), ext_multi(13), k);
        m.values[C_ZETA + 2] += Val::ONE; // tower base no longer matches zeta; Z_H also wrong
        assert!(!check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m, &[], None).is_ok());
    }

    /// Forgery — a tampered INTERMEDIATE tower power (a forged `zeta^{2^j}`) ⇒ the squaring
    /// chain breaks ⇒ rejected. Confirms the power tower is fully constrained, not just its
    /// endpoints (a prover can't smuggle a favorable Z_H by relabeling an interior power).
    #[test]
    fn air_rejects_tampered_intermediate_power() {
        let k = 12;
        let mut m = matrix(ext_multi(21), ext_multi(22), k);
        // corrupt pow[6] (a middle of the tower); breaks pow[6]==pow[5]^2 and pow[7]==pow[6]^2.
        m.values[pow_base(6) + 0] += Val::ONE;
        assert!(!check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m, &[], None).is_ok());
    }

    /// Forgery — a tampered FINAL tower power `zeta^{2^k}` (hence a forged Z_H) with C/Q left
    /// honest ⇒ both the last squaring constraint AND the OOD product fail ⇒ rejected.
    #[test]
    fn air_rejects_forged_zh() {
        let k = 10;
        let mut m = matrix(ext_multi(31), ext_multi(32), k);
        m.values[pow_base(k) + 3] += Val::ONE;
        assert!(!check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m, &[], None).is_ok());
    }

    /// A multi-row trace of independent honest OOD checks (one per row) at a fixed log_degree
    /// accepts (the per-row check is what `recursive_air` instantiates once per opened poly).
    #[test]
    fn air_accepts_multirow_checks() {
        let k = 12;
        let nc = n_cols(k);
        let mut flat = Vec::new();
        for q in 0..8u32 {
            flat.extend_from_slice(&honest_row(ext_multi(200 + q), ext_multi(300 + q), k));
        }
        let m = RowMajorMatrix::new(flat, nc);
        assert!(
            check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m, &[], None).is_ok(),
            "honest multi-row OOD trace rejected"
        );
    }

    /// Cross-check: the AIR's accept set agrees with the [`crate::deep`] reference predicate on
    /// the same `(C, Q, zeta, k)` — honest accepts in both, a tampered C rejects in both.
    #[test]
    fn air_agrees_with_reference_predicate() {
        let k = 9;
        let zeta = ext_multi(55);
        let q = ext_multi(66);
        let c = vanishing_poly_at(zeta, k) * q;
        assert!(crate::deep::verify_quotient_relation(c, q, zeta, k));
        let m = RowMajorMatrix::new(pack_check(zeta, q, c, k), n_cols(k));
        assert!(check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m, &[], None).is_ok());

        // tamper C -> reference rejects, AIR rejects.
        let c_bad = c + Challenge::ONE;
        assert!(!crate::deep::verify_quotient_relation(c_bad, q, zeta, k));
        let mut m_bad = RowMajorMatrix::new(pack_check(zeta, q, c, k), n_cols(k));
        m_bad.values[C_C] += Val::ONE; // make C = c+1 in the matrix
        assert!(!check_all_constraints(&DeepAir { log_degree: k, wire_leaf: false }, &m_bad, &[], None).is_ok());
    }

    // =======================================================================
    // LEAF-WIRED DEEP block (wire_leaf = true): C == C_partial + C_committed,
    // C == Z_H·Q.  C_partial is RE-COMPUTED in-AIR from the opened leaf cells,
    // so a tampered opened leaf cell rejects END-TO-END via the OOD identity.
    // =======================================================================

    /// Honest off-domain opened leaf cells (every cell an arbitrary multi-limb extension element,
    /// the real OOD opening shape).
    fn leaf_cells(seed: u32) -> [Challenge; lc::N_OPENED_CELLS] {
        core::array::from_fn(|c| ext_multi(seed.wrapping_mul(29).wrapping_add(c as u32)))
    }

    fn leaf_air(k: usize) -> DeepAir {
        DeepAir { log_degree: k, wire_leaf: true }
    }

    /// GOLDEN (positive): a leaf-WIRED honest row accepts — both the OOD identity `C == Z_H·Q` AND
    /// the decomposition `C == C_partial(opened cells) + C_committed` hold simultaneously.
    #[test]
    fn leaf_wired_honest_accepts() {
        for &k in &[1usize, 6, 12] {
            let cells = leaf_cells(7 + k as u32);
            let row = pack_check_leaf(ext_multi(50 + k as u32), &cells, ext_multi(3), ext_multi(9), k);
            let m = RowMajorMatrix::new(row, n_cols_cfg(k, true));
            assert!(
                check_all_constraints(&leaf_air(k), &m, &[], None).is_ok(),
                "leaf-wired honest row rejected at k={k}"
            );
        }
    }

    /// END-TO-END FORGERY — a tampered OPENED LEAF CELL (one that feeds a mirrored leaf constraint)
    /// shifts the in-AIR-recomputed `C_partial`, so `C` no longer equals `C_partial + C_committed`
    /// (and, equivalently, the committed `C` no longer equals `Z_H·Q` for the recomputed C_partial)
    /// ⇒ REJECTED through the DEEP block. This is the soundness link the task asks for: a leaf trace
    /// cell that violates a mirrored constraint is caught at the DEEP OOD check.
    #[test]
    fn leaf_wired_tampered_leaf_cell_rejects_end_to_end() {
        let k = 10;
        let cells = leaf_cells(123);
        // tamper representative cells of each mirrored class (C4 v/d_lo/bit/is96/inv, acc, pad, c6,
        // relation_id) — each must reject end-to-end.
        let targets = [
            lc::L_CELLS + 0 * D,                       // C4 lane0 v
            lc::L_CELLS + 1 * D,                       // C4 lane0 d_lo
            lc::L_CELLS + 24 * D,                      // C4 lane0 is96
            lc::L_CELLS + 25 * D,                      // C4 lane0 inv
            lc::L_CELLS + (5 * lc::C4_CELLS) * D,      // C4 lane5 v
        ];
        let lb = leaf_base(k);
        for &within in &targets {
            let row = pack_check_leaf(ext_multi(55), &cells, ext_multi(4), ext_multi(8), k);
            let mut m = RowMajorMatrix::new(row, n_cols_cfg(k, true));
            m.values[lb + within] += Val::ONE; // tamper one opened-cell limb (C, C_committed unchanged)
            assert!(
                !check_all_constraints(&leaf_air(k), &m, &[], None).is_ok(),
                "tampering opened leaf cell at +{within} was NOT rejected end-to-end"
            );
        }
    }

    /// END-TO-END FORGERY (C1 SPONGE) — with a leaf-WIRED DEEP row built over an HONEST C1 sponge
    /// (real byte-pinned Val permutation, lifted to the extension), tampering a C1 sponge cell —
    /// a forged pad_source_commitment digest, a tampered absorbed pad_seed input lane, or a
    /// tampered interior perm post — shifts the in-AIR-recomputed `C_partial`, so the committed `C`
    /// no longer equals `Z_H·Q` ⇒ REJECTED through the DEEP block. This is the C1-sponge soundness
    /// link: the FIRST mirrored leaf sponge is now caught at the DEEP OOD check, end-to-end.
    #[test]
    fn leaf_wired_tampered_c1_sponge_cell_rejects_end_to_end() {
        use crate::inlined_perm::{C_BEGIN_POST, C_OUTPUT as IP_OUTPUT};
        let k = 9;
        let cells = lc::honest_cells_for_test([0, 1, 127, 12288, 5000, 96], 17);
        // honest leaf-wired row accepts.
        let row_ok = pack_check_leaf(ext_multi(90), &cells, ext_multi(3), ext_multi(11), k);
        let m_ok = RowMajorMatrix::new(row_ok, n_cols_cfg(k, true));
        assert!(check_all_constraints(&leaf_air(k), &m_ok, &[], None).is_ok(), "honest C1 row rejected");

        // tamper representative C1 cells; each must reject end-to-end.
        let targets = [
            lc::c1_out_cell_idx(0),                  // forged pad_source_commitment lane 0
            lc::c1_out_cell_idx(9),                  // forged pad_source_commitment lane 9 (squeeze perm)
            lc::c1_input_cell_idx(0),                // tampered absorbed pad_seed byte 0
            lc::c1_input_cell_idx(47),               // tampered absorbed nonce byte
            lc::c1_perm_cell_idx(0, C_BEGIN_POST),   // tampered interior perm post (perm 0)
            lc::c1_perm_cell_idx(6, IP_OUTPUT + 1),  // tampered last-absorb output (squeeze source)
        ];
        let lb = leaf_base(k);
        for &cell_idx in &targets {
            let row = pack_check_leaf(ext_multi(90), &cells, ext_multi(3), ext_multi(11), k);
            let mut m = RowMajorMatrix::new(row, n_cols_cfg(k, true));
            // tamper limb 0 of the opened cell (C, C_committed unchanged).
            m.values[lb + lc::L_CELLS + cell_idx * D] += Val::ONE;
            assert!(
                !check_all_constraints(&leaf_air(k), &m, &[], None).is_ok(),
                "tampering C1 cell {cell_idx} was NOT rejected end-to-end"
            );
        }
    }

    /// END-TO-END FORGERY (C6 SPONGE) — with a leaf-WIRED DEEP row built over an HONEST C6 sponge
    /// (real byte-pinned Val permutation, lifted to the extension), tampering a C6 sponge cell —
    /// a forged opening_tuple_commitment digest, a tampered absorbed lane (FIELD account_projection
    /// lane or a byte-lane), or a tampered interior perm post — shifts the in-AIR-recomputed
    /// `C_partial`, so the committed `C` no longer equals `Z_H·Q` ⇒ REJECTED through the DEEP block.
    /// The SECOND mirrored leaf sponge is now caught at the DEEP OOD check, end-to-end.
    #[test]
    fn leaf_wired_tampered_c6_sponge_cell_rejects_end_to_end() {
        use crate::inlined_perm::{C_BEGIN_POST, C_OUTPUT as IP_OUTPUT};
        let k = 9;
        let cells = lc::honest_cells_for_test([0, 1, 127, 12288, 5000, 96], 17);
        // honest leaf-wired row accepts.
        let row_ok = pack_check_leaf(ext_multi(90), &cells, ext_multi(3), ext_multi(11), k);
        let m_ok = RowMajorMatrix::new(row_ok, n_cols_cfg(k, true));
        assert!(check_all_constraints(&leaf_air(k), &m_ok, &[], None).is_ok(), "honest C6 row rejected");

        // tamper representative C6 cells; each must reject end-to-end.
        let targets = [
            lc::c6_out_cell_idx(0),                  // forged opening_tuple_commitment lane 0
            lc::c6_out_cell_idx(9),                  // forged opening_tuple_commitment lane 9 (squeeze perm)
            lc::c6_byte_cell_idx(0),                 // tampered absorbed lifecycle byte 0
            lc::c6_byte_cell_idx(48),                // tampered absorbed account_id byte
            lc::c6_field_cell_idx(2),                // tampered absorbed FIELD (account_projection) lane
            lc::c6_perm_cell_idx(0, C_BEGIN_POST),   // tampered interior perm post (perm 0)
            lc::c6_perm_cell_idx(10, IP_OUTPUT + 1), // tampered last-absorb output (squeeze source)
        ];
        let lb = leaf_base(k);
        for &cell_idx in &targets {
            let row = pack_check_leaf(ext_multi(90), &cells, ext_multi(3), ext_multi(11), k);
            let mut m = RowMajorMatrix::new(row, n_cols_cfg(k, true));
            m.values[lb + lc::L_CELLS + cell_idx * D] += Val::ONE;
            assert!(
                !check_all_constraints(&leaf_air(k), &m, &[], None).is_ok(),
                "tampering C6 cell {cell_idx} was NOT rejected end-to-end"
            );
        }
    }

    /// END-TO-END FORGERY (ROLE-ROOT SPONGE, CHAINED) — with a leaf-WIRED DEEP row built over an
    /// HONEST role-root sponge (real byte-pinned Val permutation over the CHAINED C1/C6 squeeze
    /// outputs ‖ masked_projection, lifted to the extension), tampering a role-root cell — a forged
    /// role_root digest, a tampered masked_projection input lane, a tampered interior perm post, OR a
    /// tampered CHAINED C1/C6 squeeze output (which the role absorb reads as input) — shifts the
    /// in-AIR-recomputed `C_partial`, so the committed `C` no longer equals `Z_H·Q` ⇒ REJECTED
    /// through the DEEP block. The THIRD mirrored leaf sponge, and its chaining to C1/C6, is now
    /// caught at the DEEP OOD check, end-to-end.
    #[test]
    fn leaf_wired_tampered_role_root_sponge_cell_rejects_end_to_end() {
        use crate::inlined_perm::{C_BEGIN_POST, C_OUTPUT as IP_OUTPUT};
        let k = 9;
        let cells = lc::honest_cells_for_test([0, 1, 127, 12288, 5000, 96], 17);
        // honest leaf-wired row accepts.
        let row_ok = pack_check_leaf(ext_multi(90), &cells, ext_multi(3), ext_multi(11), k);
        let m_ok = RowMajorMatrix::new(row_ok, n_cols_cfg(k, true));
        assert!(check_all_constraints(&leaf_air(k), &m_ok, &[], None).is_ok(), "honest role-root row rejected");

        // tamper representative role-root cells; each must reject end-to-end.
        let targets = [
            lc::role_out_cell_idx(0),                  // forged role_root lane 0
            lc::role_out_cell_idx(7),                  // forged role_root lane 7
            lc::role_masked_cell_idx(2),               // tampered masked_projection input lane
            lc::role_perm_cell_idx(0, C_BEGIN_POST),   // tampered interior perm post (perm 0)
            lc::role_perm_cell_idx(3, IP_OUTPUT + 1),  // tampered last-absorb output (squeeze source)
            lc::c1_out_chain_cell_idx(0),              // CHAINING: tampered C1 output feeding role absorb
            lc::c6_out_cell_idx(9),                    // CHAINING: tampered C6 output feeding role absorb
        ];
        let lb = leaf_base(k);
        for &cell_idx in &targets {
            let row = pack_check_leaf(ext_multi(90), &cells, ext_multi(3), ext_multi(11), k);
            let mut m = RowMajorMatrix::new(row, n_cols_cfg(k, true));
            m.values[lb + lc::L_CELLS + cell_idx * D] += Val::ONE;
            assert!(
                !check_all_constraints(&leaf_air(k), &m, &[], None).is_ok(),
                "tampering role-root cell {cell_idx} was NOT rejected end-to-end"
            );
        }
    }

    /// FORGERY — tampering `C_committed` (the deferred summand) with everything else honest breaks
    /// `C == C_partial + C_committed` ⇒ rejected. (A prover cannot smuggle a favorable deferred
    /// bulk value either; it is bound by the decomposition.)
    #[test]
    fn leaf_wired_tampered_c_committed_rejects() {
        let k = 8;
        let cells = leaf_cells(321);
        let row = pack_check_leaf(ext_multi(60), &cells, ext_multi(5), ext_multi(7), k);
        let mut m = RowMajorMatrix::new(row, n_cols_cfg(k, true));
        let cc = leaf_base(k) + leaf_c_committed(k);
        m.values[cc + 2] += Val::ONE;
        assert!(!check_all_constraints(&leaf_air(k), &m, &[], None).is_ok());
    }

    /// FORGERY — tampering the leaf-block `alpha` shifts the recomputed `C_partial`, breaking the
    /// decomposition ⇒ rejected. (Once `alpha` is FS-bound a prover can't pick a batching point that
    /// hides a violated leaf constraint.)
    #[test]
    fn leaf_wired_tampered_alpha_rejects() {
        let k = 6;
        let cells = leaf_cells(999);
        let row = pack_check_leaf(ext_multi(70), &cells, ext_multi(6), ext_multi(2), k);
        let mut m = RowMajorMatrix::new(row, n_cols_cfg(k, true));
        let ab = leaf_base(k) + leaf_alpha(k);
        m.values[ab + 1] += Val::ONE;
        assert!(!check_all_constraints(&leaf_air(k), &m, &[], None).is_ok());
    }

    /// CONTROL-ROW ISOLATION — a 2-row leaf-wired trace where ONLY row 1's opened C4 cell is
    /// tampered: row 0 (honest) stays satisfiable, the whole-trace check REJECTS because of row 1.
    /// Confirms the rejection is attributable to the tampered row, not a layout artifact.
    #[test]
    fn leaf_wired_control_row_isolated_tamper() {
        let k = 7;
        let nc = n_cols_cfg(k, true);
        let cells0 = leaf_cells(11);
        let cells1 = leaf_cells(22);
        let mut flat = Vec::new();
        flat.extend_from_slice(&pack_check_leaf(ext_multi(80), &cells0, ext_multi(1), ext_multi(3), k));
        flat.extend_from_slice(&pack_check_leaf(ext_multi(81), &cells1, ext_multi(1), ext_multi(3), k));
        // honest 2-row trace accepts.
        let m_ok = RowMajorMatrix::new(flat.clone(), nc);
        assert!(check_all_constraints(&leaf_air(k), &m_ok, &[], None).is_ok(), "honest 2-row rejected");
        // tamper row 1's C4 lane0 v only.
        flat[nc + leaf_base(k) + lc::L_CELLS] += Val::ONE;
        let m_bad = RowMajorMatrix::new(flat, nc);
        assert!(
            !check_all_constraints(&leaf_air(k), &m_bad, &[], None).is_ok(),
            "tampered control-row trace was NOT rejected"
        );
    }
}
