//! Base-field NTT over Fq, q = 12289.
//!
//! q - 1 = 12288 = 2^12 * 3, so Fq* has 2-adicity 12: there is a subgroup of
//! order 2^k for every k <= 12, hence NTT domains of size up to 4096 = 2^12.
//! (Domains larger than 4096 need the degree-12 extension field Fq12 and are a
//! SEPARATE later brick — see `fq12_q12289`. This module is base-field only,
//! sizes n <= 4096.)
//!
//! All field arithmetic reuses the existing, audited Fq ops from
//! `poseidon2_q12289` (`fq_add`, `fq_sub`, `fq_mul`, `fq_from_u32`, `Q`); this
//! module introduces NO new field code.

use crate::poseidon2_q12289::{fq_add, fq_from_u32, fq_mul, fq_sub, Q};

/// Maximum supported log2 domain size. q - 1 = 2^12 * 3 ⇒ 2-adicity 12.
pub const MAX_LOG2N: u32 = 12;

/// Exponentiation by squaring in Fq (local copy: the one in `poseidon2_q12289`
/// is private). Computes `base^exp mod q`.
fn fq_pow(mut base: u32, mut exp: u64) -> u32 {
    let mut acc: u32 = 1;
    base %= Q;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = fq_mul(acc, base);
        }
        base = fq_mul(base, base);
        exp >>= 1;
    }
    acc
}

/// Multiplicative inverse via Fermat: `a^(q-2)`. Panics on 0 (no inverse).
fn fq_inv(a: u32) -> u32 {
    assert!(a != 0, "fq_inv(0) is undefined");
    fq_pow(a % Q, (Q - 2) as u64)
}

/// A smallest multiplicative generator `g` of Fq* (order exactly q - 1 = 12288).
///
/// We search `g = 2, 3, 4, ...` and accept the first whose multiplicative order
/// is EXACTLY q - 1. By Lagrange, ord(g) | q-1; ord(g) == q-1 iff for every
/// prime divisor p of q-1 we have g^((q-1)/p) != 1. The prime divisors of
/// 12288 = 2^12 * 3 are {2, 3}.
pub fn generator() -> u32 {
    let qm1 = (Q - 1) as u64; // 12288
    // Prime divisors of 12288 = 2^12 * 3.
    let prime_divisors = [2u64, 3u64];
    let mut g = 2u32;
    loop {
        let is_gen = prime_divisors
            .iter()
            .all(|&p| fq_pow(g, qm1 / p) != 1);
        if is_gen {
            return g;
        }
        g += 1;
        assert!(g < Q, "no generator found (impossible for a prime field)");
    }
}

/// A primitive 2^MAX_LOG2N-th (= 4096th) root of unity `omega`.
///
/// omega = g^((q-1)/4096) = g^3. Since g generates Fq*, g^3 has order exactly
/// (q-1)/gcd(3, q-1) = 12288/3 = 4096 (3 | 12288), i.e. EXACTLY 4096.
pub fn omega() -> u32 {
    let qm1 = (Q - 1) as u64; // 12288
    let exp = qm1 / (1u64 << MAX_LOG2N); // 12288 / 4096 = 3
    fq_pow(generator(), exp)
}

/// A primitive 2^log2n-th root of unity, for `log2n <= MAX_LOG2N`.
///
/// It is `omega^(2^(MAX_LOG2N - log2n))`, which has order exactly 2^log2n.
pub fn root_of_unity(log2n: u32) -> u32 {
    assert!(
        log2n <= MAX_LOG2N,
        "log2n {log2n} exceeds base-field 2-adicity {MAX_LOG2N} (size > 4096 needs Fq12)"
    );
    let step = 1u64 << (MAX_LOG2N - log2n);
    fq_pow(omega(), step)
}

/// In-place bit-reversal permutation of `values` (length must be 2^log2n).
fn bit_reverse(values: &mut [u32], log2n: u32) {
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
    let _ = log2n;
}

/// Iterative radix-2 decimation-in-time NTT using `root` as the principal
/// 2^log2n-th root of unity. In place; `values.len()` must equal 2^log2n.
fn ntt_with_root(values: &mut [u32], log2n: u32, root: u32) {
    let n = values.len();
    assert_eq!(n, 1usize << log2n, "values.len() must equal 2^log2n");
    if n <= 1 {
        return;
    }
    bit_reverse(values, log2n);

    // Stages of size 2, 4, 8, ..., n.
    let mut len = 2usize;
    while len <= n {
        // w_len = root^(n / len) is a primitive len-th root of unity.
        let w_len = fq_pow(root, (n / len) as u64);
        let half = len / 2;
        let mut start = 0usize;
        while start < n {
            let mut w = 1u32;
            for k in 0..half {
                let u = values[start + k];
                let v = fq_mul(values[start + k + half], w);
                values[start + k] = fq_add(u, v);
                values[start + k + half] = fq_sub(u, v);
                w = fq_mul(w, w_len);
            }
            start += len;
        }
        len <<= 1;
    }
}

/// Forward NTT: evaluate the coefficient vector `values` on the subgroup
/// `<root_of_unity(log2n)>`. In place; `values.len()` must equal 2^log2n.
pub fn ntt(values: &mut [u32], log2n: u32) {
    let root = root_of_unity(log2n);
    ntt_with_root(values, log2n, root);
}

/// Inverse NTT: forward transform with the inverse root, then scale by n^{-1}.
/// In place; `values.len()` must equal 2^log2n.
pub fn intt(values: &mut [u32], log2n: u32) {
    let n = values.len();
    assert_eq!(n, 1usize << log2n, "values.len() must equal 2^log2n");
    if n == 0 {
        return;
    }
    let root = root_of_unity(log2n);
    let inv_root = fq_inv(root);
    ntt_with_root(values, log2n, inv_root);
    let n_inv = fq_inv(fq_from_u32(n as u32));
    for v in values.iter_mut() {
        *v = fq_mul(*v, n_inv);
    }
}

/// Interpolate: given evaluations on the subgroup `<root_of_unity(log2n)>`,
/// recover the coefficient vector. This is exactly the inverse NTT.
pub fn interpolate(evals_on_subgroup: &[u32]) -> Vec<u32> {
    let log2n = log2_exact(evals_on_subgroup.len());
    let mut out = evals_on_subgroup.to_vec();
    intt(&mut out, log2n);
    out
}

/// Evaluate: given a coefficient vector, produce evaluations on the subgroup
/// `<root_of_unity(log2n)>`. This is exactly the forward NTT.
pub fn evaluate(coeffs: &[u32], log2n: u32) -> Vec<u32> {
    assert_eq!(coeffs.len(), 1usize << log2n, "coeffs.len() must equal 2^log2n");
    let mut out = coeffs.to_vec();
    ntt(&mut out, log2n);
    out
}

/// Coset NTT: evaluate the coefficient vector `coeffs` on the multiplicative
/// coset `shift * <omega_n>` (omega_n = root_of_unity(log2n)).
///
/// Evaluating poly P at `shift * w^k` is `sum_i c_i * shift^i * w^(i*k)`, i.e.
/// the ordinary NTT of the vector `(c_i * shift^i)`. So we pre-scale coeff i by
/// shift^i, then run the forward NTT. In place; len must be 2^log2n.
pub fn coset_ntt(coeffs: &mut [u32], log2n: u32, shift: u32) {
    let n = coeffs.len();
    assert_eq!(n, 1usize << log2n, "coeffs.len() must equal 2^log2n");
    let shift = fq_from_u32(shift);
    let mut p = 1u32; // shift^i
    for c in coeffs.iter_mut() {
        *c = fq_mul(*c, p);
        p = fq_mul(p, shift);
    }
    ntt(coeffs, log2n);
}

/// Coset evaluate (allocating wrapper around `coset_ntt`): returns the
/// evaluations of `coeffs` on the coset `shift * <root_of_unity(log2n)>`.
pub fn coset_evaluate(coeffs: &[u32], log2n: u32, shift: u32) -> Vec<u32> {
    let mut out = coeffs.to_vec();
    coset_ntt(&mut out, log2n, shift);
    out
}

/// Compute log2 of a power-of-two length, asserting it IS a power of two.
fn log2_exact(n: usize) -> u32 {
    assert!(n.is_power_of_two(), "length {n} is not a power of two");
    n.trailing_zeros()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Multiplicative order of `a` in Fq* (a != 0): smallest k>0 with a^k == 1.
    /// Computed by checking divisors of q-1 (so it is exact, not a naive loop).
    fn mult_order(a: u32) -> u64 {
        assert!(a != 0);
        let qm1 = (Q - 1) as u64;
        // factor q-1 = 2^12 * 3
        // order divides q-1; find it by dividing out prime factors.
        let mut order = qm1;
        for &p in &[2u64, 3u64] {
            while order % p == 0 && fq_pow(a, order / p) == 1 {
                order /= p;
            }
        }
        order
    }

    #[test]
    fn generator_order_is_exactly_q_minus_1() {
        let g = generator();
        assert_eq!(mult_order(g), (Q - 1) as u64, "generator order must be 12288");
        // smallest generator for q=12289 is 11 (documented anchor below).
        assert_eq!(g, 11, "smallest generator of Fq* for q=12289 is 11");
    }

    #[test]
    fn omega_order_is_exactly_4096() {
        let w = omega();
        // omega^4096 == 1
        assert_eq!(fq_pow(w, 1 << MAX_LOG2N), 1, "omega^4096 must be 1");
        // omega^2048 != 1  (so order is not a proper divisor)
        assert_ne!(fq_pow(w, 1 << (MAX_LOG2N - 1)), 1, "omega^2048 must not be 1");
        // exact order
        assert_eq!(mult_order(w), 1 << MAX_LOG2N, "omega order must be exactly 4096");
    }

    #[test]
    fn root_of_unity_has_exact_order_for_each_k() {
        for k in 1..=MAX_LOG2N {
            let r = root_of_unity(k);
            assert_eq!(fq_pow(r, 1u64 << k), 1, "root_of_unity({k})^(2^{k}) must be 1");
            assert_ne!(
                fq_pow(r, 1u64 << (k - 1)),
                1,
                "root_of_unity({k})^(2^{}) must not be 1 (order not exact)",
                k - 1
            );
            assert_eq!(mult_order(r), 1u64 << k, "root_of_unity({k}) order must be exactly 2^{k}");
        }
    }

    /// Simple reproducible xorshift PRNG so tests are deterministic.
    fn rng_seed(seed: u64) -> impl FnMut() -> u32 {
        let mut s = seed;
        move || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            (s % Q as u64) as u32
        }
    }

    #[test]
    fn ntt_intt_round_trip() {
        for &log2n in &[1u32, 3, 6, 10, 12] {
            let n = 1usize << log2n;
            let mut next = rng_seed(0xDEAD_BEEF ^ log2n as u64);
            let original: Vec<u32> = (0..n).map(|_| next()).collect();
            let mut v = original.clone();
            ntt(&mut v, log2n);
            intt(&mut v, log2n);
            assert_eq!(v, original, "intt(ntt(v)) != v for n={n}");
        }
    }

    /// Naive DFT: out[i] = sum_j v_j * root^(i*j), root = root_of_unity(log2n).
    fn naive_dft(v: &[u32], log2n: u32) -> Vec<u32> {
        let n = v.len();
        let root = root_of_unity(log2n);
        let mut out = vec![0u32; n];
        for (i, out_i) in out.iter_mut().enumerate() {
            let mut acc = 0u32;
            for (j, &vj) in v.iter().enumerate() {
                let w = fq_pow(root, (i * j) as u64);
                acc = fq_add(acc, fq_mul(vj, w));
            }
            *out_i = acc;
        }
        out
    }

    #[test]
    fn ntt_matches_naive_dft_small() {
        for &log2n in &[2u32, 3] {
            let n = 1usize << log2n;
            let mut next = rng_seed(0x1234_5678 ^ log2n as u64);
            let v: Vec<u32> = (0..n).map(|_| next()).collect();
            let fast = evaluate(&v, log2n);
            let slow = naive_dft(&v, log2n);
            assert_eq!(fast, slow, "NTT != naive DFT for n={n}");
        }
    }

    #[test]
    fn evaluate_matches_naive_polynomial_evaluation() {
        // P(x) = sum_i coeff_i x^i, evaluated at each domain point root^k.
        let log2n = 3u32;
        let n = 1usize << log2n;
        let mut next = rng_seed(0xABCD_0001);
        let coeffs: Vec<u32> = (0..n).map(|_| next()).collect();
        let evals = evaluate(&coeffs, log2n);
        let root = root_of_unity(log2n);
        for (k, &ek) in evals.iter().enumerate() {
            let point = fq_pow(root, k as u64);
            // Horner evaluation of P at `point`.
            let mut acc = 0u32;
            for &c in coeffs.iter().rev() {
                acc = fq_add(fq_mul(acc, point), c);
            }
            assert_eq!(ek, acc, "evaluate mismatch at domain point k={k}");
        }
    }

    #[test]
    fn coset_evaluate_matches_naive_on_shifted_coset() {
        let log2n = 3u32;
        let n = 1usize << log2n;
        let shift = 7u32; // a coset shift (any nonzero field element)
        let mut next = rng_seed(0x0FF1_CE42);
        let coeffs: Vec<u32> = (0..n).map(|_| next()).collect();
        let evals = coset_evaluate(&coeffs, log2n, shift);
        let root = root_of_unity(log2n);
        for (k, &ek) in evals.iter().enumerate() {
            // domain point is shift * root^k
            let point = fq_mul(fq_from_u32(shift), fq_pow(root, k as u64));
            let mut acc = 0u32;
            for &c in coeffs.iter().rev() {
                acc = fq_add(fq_mul(acc, point), c);
            }
            assert_eq!(ek, acc, "coset_evaluate mismatch at coset point k={k}");
        }
    }

    #[test]
    fn interpolate_is_inverse_of_evaluate() {
        let log2n = 6u32;
        let n = 1usize << log2n;
        let mut next = rng_seed(0x5151_5151);
        let coeffs: Vec<u32> = (0..n).map(|_| next()).collect();
        let evals = evaluate(&coeffs, log2n);
        let recovered = interpolate(&evals);
        assert_eq!(recovered, coeffs, "interpolate(evaluate(c)) != c");
    }
}
