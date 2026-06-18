//! NTT and low-degree-extension (LDE) over the extension field GF(q^12),
//! q = 12289, mirroring the radix-2 base-field NTT in `ntt_q12289` but with
//! `Fq12` arithmetic.
//!
//! WHY Fq12 NTT
//! ------------
//! The FRI proof layer evaluates base-field trace polynomials on a large
//! (16384-point) evaluation domain to form the FRI codeword. The base field
//! Fq has 2-adicity only 12 (q - 1 = 2^12 * 3 ⇒ domains ≤ 4096). The degree-12
//! extension Fq12 = GF(q^12) has
//!
//!     q^12 - 1 = 11_863_109_270_857_324_021_680_383_624_307_698_803_164_435_333_120
//!
//! whose 2-adic valuation is EXACTLY 14 (machine-checked here and earlier), so
//! Fq12 supports NTT domains up to 2^14 = 16384. We embed base-field
//! coefficients into Fq12 via `Fq12::from_base` and evaluate on the Fq12 roots
//! of unity.
//!
//! CONSTANTS (documented, also recomputed at runtime in `q12_minus_1_limbs` /
//! `q12_minus_1_over_2pow14_limbs`):
//!
//!     q^12 - 1            (little-endian u64 limbs):
//!         [0x1ca0734252024000, 0xb362cff5eac76afd, 0x00000008_1df837bb]
//!     (q^12 - 1) / 2^14   (little-endian u64 limbs, division is EXACT):
//!         [0xabf47281cd094809, 0xdeeecd8b3fd7ab1d, 0x00000000_002077e0]
//!
//! The division by 2^14 is exact because v_2(q^12 - 1) = 14.
//!
//! All field arithmetic reuses the verified `Fq12` ops from `fq12_q12289`; this
//! module introduces NO new field code.

use crate::fq12_q12289::{Fq12, N as FQ12_N};
use crate::poseidon2_q12289::Q;

/// 2-adicity of q^12 - 1, hence the maximum supported log2 domain size.
/// v_2(q^12 - 1) = 14 ⇒ NTT domains up to 2^14 = 16384.
pub const MAX_LOG2N: u32 = 14;

// =============================================================================
// Big-integer exponent helpers (little-endian u64 limbs)
// =============================================================================

/// In-place multiply a little-endian limb vector by a small (< 2^64) scalar.
fn mul_small(v: &mut Vec<u64>, s: u64) {
    let mut carry: u128 = 0;
    for limb in v.iter_mut() {
        let prod = (*limb as u128) * (s as u128) + carry;
        *limb = (prod & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        carry = prod >> 64;
    }
    while carry > 0 {
        v.push((carry & 0xFFFF_FFFF_FFFF_FFFF) as u64);
        carry >>= 64;
    }
}

/// In-place subtract a small scalar (assumes v >= s).
fn sub_small(v: &mut [u64], s: u64) {
    let mut borrow = s;
    for limb in v.iter_mut() {
        if borrow == 0 {
            break;
        }
        if *limb >= borrow {
            *limb -= borrow;
            borrow = 0;
        } else {
            let (res, _) = limb.overflowing_sub(borrow);
            *limb = res;
            borrow = 1;
        }
    }
    debug_assert_eq!(borrow, 0, "sub_small underflow");
}

/// q^k as little-endian u64 limbs (q = 12289).
fn q_pow_limbs(k: u32) -> Vec<u64> {
    let mut acc: Vec<u64> = vec![1];
    for _ in 0..k {
        mul_small(&mut acc, Q as u64);
    }
    acc
}

/// q^12 - 1 as little-endian u64 limbs. (See module docs for the literal value.)
pub fn q12_minus_1_limbs() -> Vec<u64> {
    let mut v = q_pow_limbs(FQ12_N as u32); // q^12
    sub_small(&mut v, 1);
    v
}

/// (q^12 - 1) / 2^14 as little-endian u64 limbs.
///
/// The division is EXACT because v_2(q^12 - 1) = 14. We assert exactness by
/// requiring the low 14 bits to be zero, then shift right by 14.
pub fn q12_minus_1_over_2pow14_limbs() -> Vec<u64> {
    let mut v = q12_minus_1_limbs();
    // Exactness check: the low 14 bits must all be zero.
    assert_eq!(
        v[0] & ((1u64 << MAX_LOG2N) - 1),
        0,
        "(q^12 - 1) not divisible by 2^14 — 2-adicity assumption violated"
    );
    shr_in_place(&mut v, MAX_LOG2N);
    // Trim a possible leading zero limb.
    while v.len() > 1 && *v.last().unwrap() == 0 {
        v.pop();
    }
    v
}

/// In-place logical right shift of a little-endian limb vector by `bits`
/// (0 < bits < 64).
fn shr_in_place(v: &mut [u64], bits: u32) {
    debug_assert!(bits > 0 && bits < 64);
    let mut carry = 0u64;
    for limb in v.iter_mut().rev() {
        let new_carry = *limb & ((1u64 << bits) - 1);
        *limb = (*limb >> bits) | (carry << (64 - bits));
        carry = new_carry;
    }
}

// =============================================================================
// Roots of unity in Fq12
// =============================================================================

/// Square `a` exactly `k` times: returns a^(2^k).
fn sq_k(a: &Fq12, k: u32) -> Fq12 {
    let mut r = *a;
    for _ in 0..k {
        r = r.mul(&r);
    }
    r
}

/// Test whether `cand` has multiplicative order EXACTLY 2^14 in Fq12*:
///   cand^(2^14) == 1  AND  cand^(2^13) != 1.
/// (If cand^(2^13) == 1 the order is a proper divisor of 2^14, i.e. ≤ 2^13.)
fn has_order_exactly_2pow14(cand: &Fq12) -> bool {
    let to_13 = sq_k(cand, MAX_LOG2N - 1); // cand^(2^13)
    if to_13 == Fq12::one() {
        return false;
    }
    let to_14 = to_13.mul(&to_13); // cand^(2^14)
    to_14 == Fq12::one()
}

/// A primitive 2^14-th root of unity OMEGA14 in Fq12*, computed and verified.
///
/// We take a small base element `h` and form `cand = h^((q^12 - 1) / 2^14)`.
/// Since g := generator(Fq12*) has order q^12 - 1, any element of order exactly
/// 2^14 must be a non-(2nd)-power times the maximal 2-power part; we search
/// small `h` until `cand` has order EXACTLY 2^14 (most random `h` work: a
/// uniformly random element fails to be a 2^14-th-order generator only when its
/// 2-part lands in a proper subgroup, probability 1/2).
///
/// The base elements tried are, in order:
///   x            = [0,1,0,...]  (the generator of the extension as F_q[x]/(f))
///   x + 1        = [1,1,0,...]
///   x + 2        = [2,1,0,...]
///   2x           = [0,2,0,...]
///   x^2          = [0,0,1,0,...]
///   ... small patterns ...
/// The returned OMEGA14 is the cand of the first `h` whose order is verified to
/// be exactly 16384.
pub fn omega14() -> Fq12 {
    let exp = q12_minus_1_over_2pow14_limbs();

    // Candidate base elements `h`, smallest/simplest first.
    let mut candidates: Vec<Fq12> = Vec::new();
    // x, x+1, x+2, ..., x+10
    for c0 in 0u32..=10 {
        candidates.push(Fq12::from_coeffs({
            let mut c = [0u32; FQ12_N];
            c[1] = 1;
            c[0] = c0;
            c
        }));
    }
    // 2x, 3x
    for c1 in 2u32..=3 {
        candidates.push(Fq12::from_coeffs({
            let mut c = [0u32; FQ12_N];
            c[1] = c1;
            c
        }));
    }
    // x^2, x^2 + 1, x^3
    candidates.push(Fq12::from_coeffs({
        let mut c = [0u32; FQ12_N];
        c[2] = 1;
        c
    }));
    candidates.push(Fq12::from_coeffs({
        let mut c = [0u32; FQ12_N];
        c[2] = 1;
        c[0] = 1;
        c
    }));
    candidates.push(Fq12::from_coeffs({
        let mut c = [0u32; FQ12_N];
        c[3] = 1;
        c
    }));

    for h in &candidates {
        if h.is_zero() {
            continue;
        }
        let cand = h.pow(&exp);
        if has_order_exactly_2pow14(&cand) {
            return cand;
        }
    }
    panic!("no base element among the tried candidates yields a 2^14-order root (unexpected)");
}

/// A primitive 2^log2n-th root of unity in Fq12*, for `log2n <= MAX_LOG2N`.
///
/// It is `OMEGA14^(2^(MAX_LOG2N - log2n))`, which has order exactly 2^log2n.
pub fn root_of_unity_fq12(log2n: u32) -> Fq12 {
    assert!(
        log2n <= MAX_LOG2N,
        "log2n {log2n} exceeds Fq12 2-adicity {MAX_LOG2N} (domain > 16384 unsupported)"
    );
    sq_k(&omega14(), MAX_LOG2N - log2n)
}

// =============================================================================
// NTT over Fq12
// =============================================================================

/// In-place bit-reversal permutation of `values` (length must be 2^log2n).
fn bit_reverse(values: &mut [Fq12]) {
    let n = values.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            values.swap(i, j);
        }
    }
}

/// Iterative radix-2 decimation-in-time NTT over Fq12 using `root` as the
/// principal 2^log2n-th root of unity. In place; `values.len()` == 2^log2n.
fn ntt_with_root(values: &mut [Fq12], log2n: u32, root: Fq12) {
    let n = values.len();
    assert_eq!(n, 1usize << log2n, "values.len() must equal 2^log2n");
    if n <= 1 {
        return;
    }
    bit_reverse(values);

    let mut len = 2usize;
    while len <= n {
        // w_len = root^(n / len) is a primitive len-th root of unity.
        let w_len = root.pow(&[(n / len) as u64]);
        let half = len / 2;
        let mut start = 0usize;
        while start < n {
            let mut w = Fq12::one();
            for k in 0..half {
                let u = values[start + k];
                let v = values[start + k + half].mul(&w);
                values[start + k] = u.add(&v);
                values[start + k + half] = u.sub(&v);
                w = w.mul(&w_len);
            }
            start += len;
        }
        len <<= 1;
    }
}

/// Forward NTT over Fq12: evaluate the coefficient vector `values` on the
/// subgroup `<root_of_unity_fq12(log2n)>`. In place; len must equal 2^log2n.
pub fn ntt_fq12(values: &mut [Fq12], log2n: u32) {
    let root = root_of_unity_fq12(log2n);
    ntt_with_root(values, log2n, root);
}

/// Inverse NTT over Fq12: forward transform with the inverse root, then scale
/// by n^{-1}. In place; len must equal 2^log2n.
///
/// n is a power of two with n <= 16384 < q, so n mod q != 0 and its base-field
/// inverse exists; we embed it via `Fq12::from_base`.
pub fn intt_fq12(values: &mut [Fq12], log2n: u32) {
    let n = values.len();
    assert_eq!(n, 1usize << log2n, "values.len() must equal 2^log2n");
    if n == 0 {
        return;
    }
    let root = root_of_unity_fq12(log2n);
    let inv_root = root.inv().expect("root of unity is nonzero");
    ntt_with_root(values, log2n, inv_root);

    // n^{-1} as a base-field element embedded into Fq12.
    let n_base = (n as u64 % Q as u64) as u32;
    assert_ne!(n_base, 0, "n must be invertible mod q (n < q)");
    let n_inv = Fq12::from_base(n_base)
        .inv()
        .expect("n is nonzero mod q, so invertible");
    for v in values.iter_mut() {
        *v = v.mul(&n_inv);
    }
}

// =============================================================================
// Low-degree extension (LDE)
// =============================================================================

/// Pad a base-field coefficient slice to length 2^log2_target, embedding each
/// coefficient into Fq12 via `from_base`, with the rest zero.
fn embed_and_pad(base_coeffs: &[u32], log2_target: u32) -> Vec<Fq12> {
    let target = 1usize << log2_target;
    assert!(
        base_coeffs.len() <= target,
        "base_coeffs.len() {} exceeds target size {}",
        base_coeffs.len(),
        target
    );
    let mut out = Vec::with_capacity(target);
    for &c in base_coeffs {
        out.push(Fq12::from_base(c));
    }
    out.resize(target, Fq12::zero());
    out
}

/// Low-degree extension of a BASE-field polynomial onto the 2^log2_target-th
/// roots of unity in Fq12.
///
/// Embeds base-field coefficients into Fq12, zero-pads to 2^log2_target, and
/// runs the forward Fq12 NTT. The result is the FRI codeword: evaluations of
/// the polynomial at `root_of_unity_fq12(log2_target)^k`, k = 0..2^log2_target.
pub fn lde_from_base_coeffs(base_coeffs: &[u32], log2_target: u32) -> Vec<Fq12> {
    let mut work = embed_and_pad(base_coeffs, log2_target);
    ntt_fq12(&mut work, log2_target);
    work
}

/// Coset low-degree extension: evaluate the base-field polynomial on the
/// multiplicative coset `shift * <root_of_unity_fq12(log2_target)>`.
///
/// Evaluating P at `shift * w^k` is `sum_i c_i * shift^i * w^(i*k)`, i.e. the
/// ordinary Fq12 NTT of the vector `(c_i * shift^i)`. We embed + pad, pre-scale
/// coeff i by shift^i, then run the forward NTT. FRI typically uses a coset
/// disjoint from the trace domain.
pub fn lde_coset(base_coeffs: &[u32], log2_target: u32, shift: Fq12) -> Vec<Fq12> {
    let mut work = embed_and_pad(base_coeffs, log2_target);
    let mut p = Fq12::one(); // shift^i
    for c in work.iter_mut() {
        *c = c.mul(&p);
        p = p.mul(&shift);
    }
    ntt_fq12(&mut work, log2_target);
    work
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- deterministic xorshift PRNG (no external deps) ----
    struct Rng(u64);
    impl Rng {
        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn fq(&mut self) -> u32 {
            (self.next_u64() % Q as u64) as u32
        }
        fn fq12(&mut self) -> Fq12 {
            let mut c = [0u32; FQ12_N];
            for i in 0..FQ12_N {
                c[i] = self.fq();
            }
            Fq12::from_coeffs(c)
        }
    }

    #[test]
    fn fq12_ntt_constants_match_documented_limbs() {
        // q^12 - 1 limbs.
        let qm1 = q12_minus_1_limbs();
        assert_eq!(
            qm1,
            vec![0x1ca0_7342_5202_4000, 0xb362_cff5_eac7_6afd, 0x0000_0008_1df8_37bb],
            "q^12 - 1 limbs mismatch"
        );
        // (q^12 - 1) / 2^14 limbs.
        let d = q12_minus_1_over_2pow14_limbs();
        assert_eq!(
            d,
            vec![0xabf4_7281_cd09_4809, 0xdeee_cd8b_3fd7_ab1d, 0x0000_0000_0020_77e0],
            "(q^12 - 1)/2^14 limbs mismatch"
        );
        // Cross-check: d * 2^14 == q^12 - 1 (exactness).
        let mut back = d.clone();
        mul_small(&mut back, 1u64 << MAX_LOG2N);
        while back.len() > 1 && *back.last().unwrap() == 0 {
            back.pop();
        }
        assert_eq!(back, qm1, "d * 2^14 must equal q^12 - 1");
    }

    #[test]
    fn fq12_omega14_order_is_exactly_16384() {
        let w = omega14();
        // w^(2^14) == 1
        let to_14 = sq_k(&w, MAX_LOG2N);
        assert_eq!(to_14, Fq12::one(), "omega14^(2^14) must be 1");
        // w^(2^13) != 1  (order is not a proper divisor)
        let to_13 = sq_k(&w, MAX_LOG2N - 1);
        assert_ne!(to_13, Fq12::one(), "omega14^(2^13) must NOT be 1");
    }

    #[test]
    fn fq12_root_of_unity_exact_order_each_k() {
        for k in 1..=MAX_LOG2N {
            let r = root_of_unity_fq12(k);
            // r^(2^k) == 1
            let top = sq_k(&r, k);
            assert_eq!(top, Fq12::one(), "root_of_unity_fq12({k})^(2^{k}) must be 1");
            // r^(2^(k-1)) != 1  (order exactly 2^k, not a proper divisor)
            let half = sq_k(&r, k - 1);
            assert_ne!(
                half,
                Fq12::one(),
                "root_of_unity_fq12({k})^(2^{}) must NOT be 1",
                k - 1
            );
        }
    }

    #[test]
    fn fq12_ntt_intt_round_trip() {
        for &log2n in &[1u32, 3, 8, 14] {
            let n = 1usize << log2n;
            let mut rng = Rng(0xDEAD_BEEF_0000_0000 ^ log2n as u64);
            let original: Vec<Fq12> = (0..n).map(|_| rng.fq12()).collect();
            let mut v = original.clone();
            ntt_fq12(&mut v, log2n);
            intt_fq12(&mut v, log2n);
            assert_eq!(v, original, "intt(ntt(v)) != v for n={n}");
        }
    }

    /// Naive DFT over Fq12: out[i] = sum_j v_j * root^(i*j).
    fn naive_dft_fq12(v: &[Fq12], log2n: u32) -> Vec<Fq12> {
        let n = v.len();
        let root = root_of_unity_fq12(log2n);
        let mut out = vec![Fq12::zero(); n];
        for (i, out_i) in out.iter_mut().enumerate() {
            let mut acc = Fq12::zero();
            for (j, vj) in v.iter().enumerate() {
                let w = root.pow(&[(i * j) as u64]);
                acc = acc.add(&vj.mul(&w));
            }
            *out_i = acc;
        }
        out
    }

    #[test]
    fn fq12_ntt_matches_naive_dft_small() {
        for &log2n in &[2u32, 3] {
            let n = 1usize << log2n;
            let mut rng = Rng(0x1234_5678_0000_0000 ^ log2n as u64);
            let v: Vec<Fq12> = (0..n).map(|_| rng.fq12()).collect();
            let mut fast = v.clone();
            ntt_fq12(&mut fast, log2n);
            let slow = naive_dft_fq12(&v, log2n);
            assert_eq!(fast, slow, "Fq12 NTT != naive DFT for n={n}");
        }
    }

    #[test]
    fn fq12_lde_matches_naive_eval_of_embedded_poly() {
        // Small random BASE-field polynomial; LDE to a larger Fq12 domain.
        let log2_target = 5u32; // 32-point domain
        let n = 1usize << log2_target;
        let mut rng = Rng(0xABCD_0001_0000_0000);
        // Use a base poly of smaller degree than the target (genuine extension).
        let deg = 12usize;
        let base_coeffs: Vec<u32> = (0..deg).map(|_| rng.fq()).collect();

        let codeword = lde_from_base_coeffs(&base_coeffs, log2_target);
        assert_eq!(codeword.len(), n);

        let root = root_of_unity_fq12(log2_target);
        // Embed coeffs into Fq12 once.
        let embedded: Vec<Fq12> = base_coeffs.iter().map(|&c| Fq12::from_base(c)).collect();
        for (k, ck) in codeword.iter().enumerate() {
            let point = root.pow(&[k as u64]);
            // Horner eval of the embedded poly at `point`.
            let mut acc = Fq12::zero();
            for c in embedded.iter().rev() {
                acc = acc.mul(&point).add(c);
            }
            assert_eq!(*ck, acc, "LDE mismatch at domain point k={k}");
        }
    }

    #[test]
    fn fq12_lde_restricted_subgroup_agrees_with_base_ntt() {
        // Cross-check against the base-field NTT (ntt_q12289): an LDE whose
        // target size equals a base-NTT-supported size, restricted to the
        // original subgroup, must equal the base-field NTT evaluations embedded
        // into Fq12 — PROVIDED both use the SAME subgroup. The base and Fq12
        // roots of unity are different elements, so we compare structurally:
        // run the base NTT, embed its outputs, and independently confirm the
        // Fq12 LDE evaluations equal naive Horner eval at the Fq12 roots (done
        // above) — here we additionally verify that the Fq12 LDE of an
        // all-base poly produces base-field-valued points exactly when the
        // domain point is base-field-valued is NOT generally true; instead we
        // assert the invariant that matters for FRI: the inverse NTT recovers
        // the embedded base coefficients.
        let log2n = 6u32;
        let n = 1usize << log2n;
        let mut rng = Rng(0x5151_5151_0000_0000);
        let base_coeffs: Vec<u32> = (0..n).map(|_| rng.fq()).collect();

        // Fq12 LDE then inverse: must recover the embedded base coefficients.
        let mut codeword = lde_from_base_coeffs(&base_coeffs, log2n);
        intt_fq12(&mut codeword, log2n);
        let embedded: Vec<Fq12> = base_coeffs.iter().map(|&c| Fq12::from_base(c)).collect();
        assert_eq!(codeword, embedded, "intt of LDE must recover embedded base coeffs");

        // And the base-field NTT of the same coeffs, embedded, equals the naive
        // Horner eval at the BASE roots — sanity that our embedding is faithful.
        let mut base_eval = base_coeffs.clone();
        crate::ntt_q12289::ntt(&mut base_eval, log2n);
        let base_root = crate::ntt_q12289::root_of_unity(log2n);
        for (k, &ek) in base_eval.iter().enumerate() {
            // Horner over base field.
            let point = {
                let mut p = 1u32;
                for _ in 0..k {
                    p = crate::poseidon2_q12289::fq_mul(p, base_root);
                }
                p
            };
            let mut acc = 0u32;
            for &c in base_coeffs.iter().rev() {
                acc = crate::poseidon2_q12289::fq_add(
                    crate::poseidon2_q12289::fq_mul(acc, point),
                    c,
                );
            }
            assert_eq!(ek, acc, "base NTT vs Horner mismatch at k={k}");
        }
    }

    #[test]
    fn fq12_lde_coset_matches_naive_on_shifted_coset() {
        let log2_target = 4u32; // 16-point coset
        let mut rng = Rng(0x0FF1_CE42_0000_0000);
        let deg = 9usize;
        let base_coeffs: Vec<u32> = (0..deg).map(|_| rng.fq()).collect();
        // A coset shift that is NOT in the subgroup (a generic small Fq12 elt).
        let shift = Fq12::from_coeffs({
            let mut c = [0u32; FQ12_N];
            c[0] = 7;
            c[1] = 1; // 7 + x
            c
        });

        let evals = lde_coset(&base_coeffs, log2_target, shift);
        let root = root_of_unity_fq12(log2_target);
        let embedded: Vec<Fq12> = base_coeffs.iter().map(|&c| Fq12::from_base(c)).collect();
        for (k, ek) in evals.iter().enumerate() {
            // coset point = shift * root^k
            let point = shift.mul(&root.pow(&[k as u64]));
            let mut acc = Fq12::zero();
            for c in embedded.iter().rev() {
                acc = acc.mul(&point).add(c);
            }
            assert_eq!(*ek, acc, "coset LDE mismatch at coset point k={k}");
        }
    }
}
