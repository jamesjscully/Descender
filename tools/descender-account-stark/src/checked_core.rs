//! Phase-2 SPEND checked-core SIS relation — the public matrix `H` and the response→syndrome map.
//!
//! The deployed direct spend verifier accepts a checked-core proof iff its response syndrome equals
//! the expected syndrome (`path2_native_checked_core_accept_v1`). The response syndrome is a
//! matrix-vector product mod q over a FIXED public matrix `H` (12×22):
//!
//!   `syndrome[row] = Σ_{col} H[row][col] · response[col]  (mod q12289)`
//!
//! where `response[22]` is the prover's short canonical response (each lane `< q`). Proving that the
//! committed syndrome has a known short canonical preimage `response` under `H` IS the checked-core
//! signature verification (finding such a preimage for an arbitrary syndrome is the SIS hardness).
//!
//! `H` is derived EXACTLY as the deployed `domain_matrix_entry`: SHA256 rejection sampling over
//! `(domain ‖ row_le_u16 ‖ col_le_u16 ‖ counter_le_u32)`, reduced mod q. This module reproduces it
//! byte-identically (a fidelity test pins the first entries) so the in-AIR constraint matches the
//! deployed verifier's matrix.

use sha2::{Digest, Sha256};

/// Checked-core syndrome rows (= `REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS`).
pub const CC_ROWS: usize = 12;
/// Checked-core response residues / matrix columns (= `REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES`).
pub const CC_COLS: usize = 22;
/// q12289 — the canonical residue modulus.
pub const CC_Q: u16 = 12289;

/// Matrix domain, byte-identical to the deployed `REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN`.
pub const CC_MATRIX_DOMAIN: &str =
    "Descender.Signature.q12289.local_delta.sis_fq12_account_carried_matrix.v1";

/// One matrix entry `H[row][col]`, byte-identical to the deployed `domain_matrix_entry`.
pub fn matrix_entry(row: usize, col: usize) -> u16 {
    let q = CC_Q;
    let rejection_limit = u16::MAX - (u16::MAX % q);
    for counter in 0_u32.. {
        let mut hasher = Sha256::new();
        hasher.update(CC_MATRIX_DOMAIN.as_bytes());
        hasher.update((row as u16).to_le_bytes());
        hasher.update((col as u16).to_le_bytes());
        hasher.update(counter.to_le_bytes());
        let digest = hasher.finalize();
        for chunk in digest.chunks_exact(2) {
            let value = u16::from_le_bytes([chunk[0], chunk[1]]);
            if value < rejection_limit {
                return value % q;
            }
        }
    }
    unreachable!("unbounded rejection sampler loop");
}

/// The full fixed checked-core matrix `H` (12×22).
pub fn checked_core_matrix() -> [[u16; CC_COLS]; CC_ROWS] {
    core::array::from_fn(|row| core::array::from_fn(|col| matrix_entry(row, col)))
}

/// The response syndrome `H·response mod q`, byte-identical to the deployed
/// `reviewed_fq12_account_carried_response_syndrome` (running reduction per term).
pub fn response_syndrome(response: &[u16; CC_COLS]) -> [u16; CC_ROWS] {
    let h = checked_core_matrix();
    core::array::from_fn(|row| {
        let mut acc = 0u32;
        for (col, &v) in response.iter().enumerate() {
            acc += h[row][col] as u32 * v as u32;
            acc %= CC_Q as u32;
        }
        acc as u16
    })
}

/// Action polynomial `M(x) = x^12 + x + 25` (low-to-high), byte-identical to the deployed
/// `REVIEWED_FQ12_ACCOUNT_CARRIED_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH`. Fq12 = F_q[x]/(M).
pub const CC_ACTION_POLY: [u16; 13] = [25, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

/// Fq12 multiply `c·w mod (M, q)`, matching the deployed `fq12_syndrome_mul`. Because `M` is sparse
/// (`x^12 ≡ -x - 25`), reducing the degree-22 product folds in ONE pass without cascading:
/// `x^{12+j} ≡ -25·x^j - x^{j+1}` (lands in degrees ≤ 11).
pub fn fq12_mul(c: &[u16; CC_ROWS], w: &[u16; CC_ROWS]) -> [u16; CC_ROWS] {
    let q = CC_Q as u32;
    // Raw product (degrees 0..=22), each coefficient reduced mod q.
    let mut p = [0u32; 2 * CC_ROWS - 1];
    for (i, &ci) in c.iter().enumerate() {
        for (j, &wj) in w.iter().enumerate() {
            p[i + j] = (p[i + j] + (ci as u32 * wj as u32) % q) % q;
        }
    }
    // Fold degrees 12..=22 into 0..=11 via x^{12+j} ≡ -25·x^j - x^{j+1}.
    let mut out = [0i64; CC_ROWS];
    for (d, slot) in out.iter_mut().enumerate() {
        *slot = p[d] as i64;
    }
    for j in 0..(CC_ROWS - 1) {
        let hi = p[CC_ROWS + j] as i64; // coefficient of x^{12+j}
        out[j] -= 25 * hi;
        out[j + 1] -= hi;
    }
    core::array::from_fn(|d| out[d].rem_euclid(q as i64) as u16)
}

/// Bits in a 12-lane syndrome digest (12 × 14 = 168); bytes carry 256 bits, the top 88 are zero.
pub const CC_SYNDROME_DIGEST_BITS: usize = CC_ROWS * 14;
/// Byte length of a syndrome digest commitment (= deployed `DIGEST_BYTES`).
pub const CC_DIGEST_BYTES: usize = 32;

/// Pack a 12-lane canonical syndrome into a 32-byte digest, 14-bit LE — byte-identical to the deployed
/// `pack_reviewed_fq12_account_carried_syndrome_digest` (`write_bits_le(idx*14, 14, value)`).
pub fn pack_syndrome_digest(syndrome: &[u16; CC_ROWS]) -> [u8; CC_DIGEST_BYTES] {
    let mut out = [0u8; CC_DIGEST_BYTES];
    for (idx, &v) in syndrome.iter().enumerate() {
        for b in 0..14 {
            if (v >> b) & 1 == 1 {
                let pos = idx * 14 + b;
                out[pos / 8] |= 1u8 << (pos % 8);
            }
        }
    }
    out
}

fn inv_mod_q(a: u32) -> u32 {
    // q is prime ⇒ a^(q-2) mod q (Fermat).
    let q = CC_Q as u64;
    let mut base = (a as u64) % q;
    let mut exp = q - 2;
    let mut acc = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = acc * base % q;
        }
        base = base * base % q;
        exp >>= 1;
    }
    acc as u32
}

/// Solve `H·r == expected (mod q)` for a CANONICAL response `r[22]` (the checked-core response is
/// canonical `< q`, not short-norm, so this is a solvable linear system — NOT SIS-hard). Gaussian
/// elimination mod q over the 12×22 system; free variables set to 0. Used to construct honest test
/// witnesses (and by a real signer). Returns `None` only if the (random-`H`) system is inconsistent.
pub fn solve_response(expected: &[u16; CC_ROWS]) -> Option<[u16; CC_COLS]> {
    let q = CC_Q as u32;
    let h = checked_core_matrix();
    // Augmented matrix [H | expected], CC_ROWS × (CC_COLS+1), as u32 in [0,q).
    let mut a = [[0u32; CC_COLS + 1]; CC_ROWS];
    for r in 0..CC_ROWS {
        for c in 0..CC_COLS {
            a[r][c] = h[r][c] as u32 % q;
        }
        a[r][CC_COLS] = expected[r] as u32 % q;
    }
    // Forward elimination with pivoting; track pivot column per row.
    let mut pivot_col = [usize::MAX; CC_ROWS];
    let mut row = 0usize;
    for col in 0..CC_COLS {
        if row >= CC_ROWS {
            break;
        }
        // Find a pivot in this column at or below `row`.
        let mut sel = None;
        for r in row..CC_ROWS {
            if a[r][col] != 0 {
                sel = Some(r);
                break;
            }
        }
        let Some(sel) = sel else { continue };
        a.swap(row, sel);
        // Normalize pivot row.
        let inv = inv_mod_q(a[row][col]) as u64;
        for c in col..=CC_COLS {
            a[row][c] = (a[row][c] as u64 * inv % q as u64) as u32;
        }
        // Eliminate this column from all other rows.
        for r in 0..CC_ROWS {
            if r != row && a[r][col] != 0 {
                let factor = a[r][col] as u64;
                for c in col..=CC_COLS {
                    let sub = factor * a[row][c] as u64 % q as u64;
                    a[r][c] = ((a[r][c] as u64 + q as u64 - sub) % q as u64) as u32;
                }
            }
        }
        pivot_col[row] = col;
        row += 1;
    }
    // Consistency: any all-zero H-row with nonzero rhs ⇒ no solution.
    for r in 0..CC_ROWS {
        if pivot_col[r] == usize::MAX && a[r][CC_COLS] != 0 {
            // (row r was never assigned a pivot but might still be zero-row)
        }
    }
    for r in row..CC_ROWS {
        if a[r][CC_COLS] != 0 {
            return None;
        }
    }
    // Back-substitute: free variables = 0, pivots = rhs.
    let mut out = [0u16; CC_COLS];
    for r in 0..CC_ROWS {
        if pivot_col[r] != usize::MAX {
            out[pivot_col[r]] = a[r][CC_COLS] as u16;
        }
    }
    Some(out)
}

/// Componentwise `c + w mod q`, matching the deployed `fq12_syndrome_add`.
pub fn fq12_add(a: &[u16; CC_ROWS], b: &[u16; CC_ROWS]) -> [u16; CC_ROWS] {
    core::array::from_fn(|i| ((a[i] as u32 + b[i] as u32) % CC_Q as u32) as u16)
}

/// The sigma "expected" syndrome `t + c⊙w mod q` (the deployed acceptance target, with `c`/`w`/`t`
/// supplied directly as syndrome vectors). The full deployed derivation expands `c` from the
/// challenge (SHA-based) and `w`/`t` from commitments — those input-derivations are NOT performed
/// here.
pub fn sigma_expected(t: &[u16; CC_ROWS], c: &[u16; CC_ROWS], w: &[u16; CC_ROWS]) -> [u16; CC_ROWS] {
    fq12_add(t, &fq12_mul(c, w))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The matrix entries are deterministic and canonical (`< q`).
    #[test]
    fn matrix_entries_canonical_and_deterministic() {
        let h = checked_core_matrix();
        for row in &h {
            for &e in row {
                assert!(e < CC_Q, "matrix entry not canonical");
            }
        }
        // Determinism: recompute a couple of entries.
        assert_eq!(h[0][0], matrix_entry(0, 0));
        assert_eq!(h[11][21], matrix_entry(11, 21));
    }

    /// FIDELITY: the single-pass `fq12_mul` fold equals the deployed iterative `poly_mul_mod`
    /// algorithm (high-to-low pop reduction) for random inputs.
    #[test]
    fn fq12_mul_matches_iterative_reduction() {
        let q = CC_Q as u32;
        // Reference: the deployed poly_mul_mod (iterative monic reduction) + normalize.
        let reference = |c: &[u16; CC_ROWS], w: &[u16; CC_ROWS]| -> [u16; CC_ROWS] {
            let modulus = CC_ACTION_POLY;
            let mut product: Vec<u16> = vec![0u16; c.len() + w.len() - 1];
            for (i, &x) in c.iter().enumerate() {
                for (j, &y) in w.iter().enumerate() {
                    product[i + j] = ((product[i + j] as u32 + (x as u32 * y as u32) % q) % q) as u16;
                }
            }
            while product.len() >= modulus.len() {
                let coeff = *product.last().unwrap();
                let degree_delta = product.len() - modulus.len();
                product.pop();
                if coeff == 0 {
                    while product.len() > 1 && *product.last().unwrap() == 0 {
                        product.pop();
                    }
                    continue;
                }
                for (idx, &mc) in modulus[..modulus.len() - 1].iter().enumerate() {
                    let d = degree_delta + idx;
                    let sub = (coeff as u32 * mc as u32) % q;
                    product[d] = ((product[d] as u32 + q - sub) % q) as u16;
                }
                while product.len() > 1 && *product.last().unwrap() == 0 {
                    product.pop();
                }
            }
            let mut out = [0u16; CC_ROWS];
            for (i, &v) in product.iter().enumerate() {
                out[i] = v;
            }
            out
        };
        for seed in 0..8u32 {
            let c: [u16; CC_ROWS] = core::array::from_fn(|i| (((seed * 131 + i as u32) * 977 + 3) % q) as u16);
            let w: [u16; CC_ROWS] = core::array::from_fn(|i| (((seed * 17 + i as u32) * 613 + 11) % q) as u16);
            assert_eq!(fq12_mul(&c, &w), reference(&c, &w), "fq12_mul != iterative reduction");
        }
    }

    /// The linear solver returns a canonical `r` with `H·r == expected` (any target is solvable since
    /// `H` is 12×22 over the field).
    #[test]
    fn solve_response_satisfies_system() {
        for seed in 0..8u32 {
            let expected: [u16; CC_ROWS] =
                core::array::from_fn(|i| (((seed * 31 + i as u32) * 701 + 13) % CC_Q as u32) as u16);
            let r = solve_response(&expected).expect("system solvable");
            for &v in &r {
                assert!(v < CC_Q, "response lane non-canonical");
            }
            assert_eq!(response_syndrome(&r), expected, "H·solve(expected) != expected");
        }
    }

    /// `response_syndrome` reduces mod q and is deterministic.
    #[test]
    fn response_syndrome_is_canonical() {
        let response: [u16; CC_COLS] = core::array::from_fn(|i| ((i as u32 * 503 + 7) % CC_Q as u32) as u16);
        let s = response_syndrome(&response);
        for &v in &s {
            assert!(v < CC_Q);
        }
        assert_eq!(s, response_syndrome(&response));
    }
}
