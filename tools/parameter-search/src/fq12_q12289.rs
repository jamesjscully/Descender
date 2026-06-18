//! Degree-12 extension field GF(q^12) over Fq, q = 12289.
//!
//! This is the CHALLENGE / SOUNDNESS field for an upcoming FRI proof layer.
//! FRI over the 14-bit base field q=12289 needs challenges drawn from an
//! extension large enough to give ~128-bit+ soundness; GF(12289^12) has order
//! q^12 ≈ 2^163, comfortably above 128 bits.
//!
//! CORRECTNESS DISCIPLINE
//! ----------------------
//! The defining polynomial f(x) is NOT assumed irreducible — it is
//! IRREDUCIBILITY-VERIFIED COMPUTATIONALLY via the Rabin irreducibility test
//! (see `is_irreducible_rabin` and the test `chosen_modulus_is_irreducible`).
//! The chosen modulus is the binomial
//!
//!     f(x) = x^12 - G_MODULUS   with G_MODULUS = 11,
//!
//! i.e. f(x) = x^12 + (q - 11). This is the first binomial x^12 - g
//! (g = 2,3,5,7,11,...) that the machine Rabin test certifies irreducible over
//! F_q. (g = 2,3,5,7 all FAIL the test; documented in
//! `binomial_modulus_search_matches_chosen`.) Irreducibility is VERIFIED, not
//! assumed.
//!
//! Representation: an element c0 + c1*x + ... + c11*x^11 is stored as
//! `[u32; 12]` with each coefficient reduced into [0, q).
//!
//! Base-field ops are reused from `crate::poseidon2_q12289`.

use crate::poseidon2_q12289::{fq_add, fq_from_u32, fq_mul, fq_sub, Q};

/// Extension degree.
pub const N: usize = 12;

/// The binomial constant g in f(x) = x^12 - g. Machine-verified irreducible
/// (Rabin test); see module docs and tests.
pub const G_MODULUS: u32 = 11;

/// f(x) = x^12 - G_MODULUS, so x^12 ≡ G_MODULUS (mod f).
/// REDUCE_CONST = G_MODULUS reduced into [0, q).
const REDUCE_CONST: u32 = G_MODULUS;

// =============================================================================
// Fq12 element type
// =============================================================================

/// An element of GF(q^12) = F_q[x]/(f). Coefficients c0..c11, each in [0, q).
#[derive(Clone, Copy, Debug)]
pub struct Fq12 {
    pub c: [u32; N],
}

impl PartialEq for Fq12 {
    fn eq(&self, other: &Self) -> bool {
        // Coefficients are kept canonical (reduced mod q), so byte equality is
        // field equality.
        self.c == other.c
    }
}
impl Eq for Fq12 {}

impl Fq12 {
    /// The zero element.
    pub fn zero() -> Self {
        Fq12 { c: [0; N] }
    }

    /// The one element.
    pub fn one() -> Self {
        let mut c = [0u32; N];
        c[0] = 1;
        Fq12 { c }
    }

    /// Embed a base-field element a into GF(q^12) as the constant a.
    pub fn from_base(a: u32) -> Self {
        let mut c = [0u32; N];
        c[0] = fq_from_u32(a);
        Fq12 { c }
    }

    /// Construct directly from 12 coefficients, reducing each mod q.
    pub fn from_coeffs(raw: [u32; N]) -> Self {
        let mut c = [0u32; N];
        for i in 0..N {
            c[i] = fq_from_u32(raw[i]);
        }
        Fq12 { c }
    }

    pub fn is_zero(&self) -> bool {
        self.c.iter().all(|&v| v == 0)
    }

    /// Field addition.
    pub fn add(&self, other: &Self) -> Self {
        let mut c = [0u32; N];
        for i in 0..N {
            c[i] = fq_add(self.c[i], other.c[i]);
        }
        Fq12 { c }
    }

    /// Field subtraction.
    pub fn sub(&self, other: &Self) -> Self {
        let mut c = [0u32; N];
        for i in 0..N {
            c[i] = fq_sub(self.c[i], other.c[i]);
        }
        Fq12 { c }
    }

    /// Field negation.
    pub fn neg(&self) -> Self {
        let mut c = [0u32; N];
        for i in 0..N {
            c[i] = fq_sub(0, self.c[i]);
        }
        Fq12 { c }
    }

    /// Field multiplication: schoolbook degree-≤22 product reduced mod f.
    ///
    /// Because f(x) = x^12 - g, the reduction is the simple folding rule
    /// x^(12+j) ≡ g * x^j, applied to product coefficients of degree 12..=22.
    pub fn mul(&self, other: &Self) -> Self {
        // Full product, degrees 0..=22.
        let mut prod = [0u64; 2 * N - 1];
        for i in 0..N {
            if self.c[i] == 0 {
                continue;
            }
            for j in 0..N {
                prod[i + j] += self.c[i] as u64 * other.c[j] as u64;
            }
        }
        // Reduce each accumulator mod q first to keep things small, then fold
        // the high terms x^12..x^22 down via x^12 ≡ g.
        let g = REDUCE_CONST as u64;
        let mut c = [0u32; N];
        for k in 0..N {
            c[k] = (prod[k] % Q as u64) as u32;
        }
        for k in N..(2 * N - 1) {
            // x^k = x^(12 + (k-12)) ≡ g * x^(k-12)
            let contrib = ((prod[k] % Q as u64) * g) % Q as u64;
            c[k - N] = fq_add(c[k - N], contrib as u32);
        }
        Fq12 { c }
    }

    /// Exponentiation by a big-integer exponent given as little-endian u64 limbs.
    /// Square-and-multiply over the bit representation.
    pub fn pow(&self, exp_limbs: &[u64]) -> Self {
        let mut result = Fq12::one();
        // Find the highest set bit so we don't waste squarings on leading zeros.
        let mut hi_bit: i64 = -1;
        for (li, &limb) in exp_limbs.iter().enumerate().rev() {
            if limb != 0 {
                hi_bit = (li as i64) * 64 + (63 - limb.leading_zeros() as i64);
                break;
            }
        }
        if hi_bit < 0 {
            return result; // exponent == 0
        }
        let mut bit = hi_bit;
        while bit >= 0 {
            result = result.mul(&result);
            let limb = (bit as usize) / 64;
            let off = (bit as usize) % 64;
            if (exp_limbs[limb] >> off) & 1 == 1 {
                result = result.mul(self);
            }
            bit -= 1;
        }
        result
    }

    /// Multiplicative inverse via Fermat's little theorem: a^(q^12 - 2).
    /// Returns None for the zero element.
    pub fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }
        let exp = q12_minus_2_limbs();
        Some(self.pow(&exp))
    }

    /// Frobenius endomorphism a -> a^q.
    pub fn frobenius(&self) -> Self {
        self.pow(&[Q as u64])
    }
}

// =============================================================================
// Big-integer exponent helpers (little-endian u64 limbs)
// =============================================================================

/// q^k as little-endian u64 limbs.
fn q_pow_limbs(k: u32) -> Vec<u64> {
    let mut acc: Vec<u64> = vec![1];
    for _ in 0..k {
        mul_small(&mut acc, Q as u64);
    }
    acc
}

/// q^12 - 2 as little-endian u64 limbs.
fn q12_minus_2_limbs() -> Vec<u64> {
    let mut v = q_pow_limbs(12);
    sub_small(&mut v, 2);
    v
}

/// In-place multiply a little-endian limb vector by a small (< 2^32) scalar.
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
        if *limb >= borrow {
            *limb -= borrow;
            borrow = 0;
            break;
        } else {
            let (res, _) = limb.overflowing_sub(borrow);
            *limb = res;
            borrow = 1;
        }
    }
    debug_assert_eq!(borrow, 0, "sub_small underflow");
}

// =============================================================================
// Polynomial arithmetic over F_q (for the Rabin irreducibility test)
// =============================================================================
//
// Polynomials are Vec<u32> of coefficients (little-endian: index = degree),
// each coefficient in [0, q). The empty / all-zero vector is the zero poly.

/// Strip trailing zero coefficients (normalize representation).
fn poly_trim(p: &mut Vec<u32>) {
    while p.len() > 1 && *p.last().unwrap() == 0 {
        p.pop();
    }
    if p.is_empty() {
        p.push(0);
    }
}

/// Degree of a polynomial (degree of zero poly reported as 0 with c0==0).
fn poly_degree(p: &[u32]) -> usize {
    let mut d = p.len().saturating_sub(1);
    while d > 0 && p[d] == 0 {
        d -= 1;
    }
    d
}

fn poly_is_zero(p: &[u32]) -> bool {
    p.iter().all(|&v| v == 0)
}

/// Subtract two polynomials over F_q.
fn poly_sub(a: &[u32], b: &[u32]) -> Vec<u32> {
    let n = a.len().max(b.len());
    let mut r = vec![0u32; n];
    for i in 0..n {
        let av = if i < a.len() { a[i] } else { 0 };
        let bv = if i < b.len() { b[i] } else { 0 };
        r[i] = fq_sub(av, bv);
    }
    poly_trim(&mut r);
    r
}

/// Multiply two polynomials over F_q (schoolbook).
fn poly_mul(a: &[u32], b: &[u32]) -> Vec<u32> {
    if poly_is_zero(a) || poly_is_zero(b) {
        return vec![0];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            r[i + j] += ai as u64 * bj as u64;
        }
    }
    let mut out: Vec<u32> = r.iter().map(|&v| (v % Q as u64) as u32).collect();
    poly_trim(&mut out);
    out
}

/// Polynomial remainder a mod m over F_q (m must be nonzero; leading coeff
/// inverted via base-field Fermat inverse). Returns a with deg < deg(m).
fn poly_rem(a: &[u32], m: &[u32]) -> Vec<u32> {
    let dm = poly_degree(m);
    let lead_m = m[dm];
    let lead_inv = fq_inv(lead_m);
    let mut r: Vec<u32> = a.to_vec();
    poly_trim(&mut r);
    while !poly_is_zero(&r) && poly_degree(&r) >= dm {
        let dr = poly_degree(&r);
        let shift = dr - dm;
        // factor = r[dr] / lead_m
        let factor = fq_mul(r[dr], lead_inv);
        // r -= factor * x^shift * m
        for i in 0..=dm {
            let term = fq_mul(factor, m[i]);
            r[i + shift] = fq_sub(r[i + shift], term);
        }
        poly_trim(&mut r);
        // safety: the top coefficient must have been cleared
        if poly_degree(&r) == dr && r[dr] != 0 {
            // Shouldn't happen; avoid infinite loop.
            break;
        }
    }
    poly_trim(&mut r);
    r
}

/// Polynomial GCD over F_q (Euclidean), returned monic if nonzero.
fn poly_gcd(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut x: Vec<u32> = a.to_vec();
    let mut y: Vec<u32> = b.to_vec();
    poly_trim(&mut x);
    poly_trim(&mut y);
    while !poly_is_zero(&y) {
        let r = poly_rem(&x, &y);
        x = y;
        y = r;
    }
    // make monic
    if !poly_is_zero(&x) {
        let d = poly_degree(&x);
        let inv = fq_inv(x[d]);
        for v in x.iter_mut() {
            *v = fq_mul(*v, inv);
        }
        poly_trim(&mut x);
    }
    x
}

/// Multiply two polynomials modulo m over F_q.
fn poly_mul_mod(a: &[u32], b: &[u32], m: &[u32]) -> Vec<u32> {
    poly_rem(&poly_mul(a, b), m)
}

/// Compute base^exp mod m over F_q[x], exp given as little-endian u64 limbs.
/// Square-and-multiply.
fn poly_pow_mod(base: &[u32], exp_limbs: &[u64], m: &[u32]) -> Vec<u32> {
    let mut result = vec![1u32]; // the constant polynomial 1
    let b0 = poly_rem(base, m);
    // highest set bit
    let mut hi_bit: i64 = -1;
    for (li, &limb) in exp_limbs.iter().enumerate().rev() {
        if limb != 0 {
            hi_bit = (li as i64) * 64 + (63 - limb.leading_zeros() as i64);
            break;
        }
    }
    if hi_bit < 0 {
        return result;
    }
    let mut bit = hi_bit;
    while bit >= 0 {
        result = poly_mul_mod(&result, &result, m);
        let limb = (bit as usize) / 64;
        let off = (bit as usize) % 64;
        if (exp_limbs[limb] >> off) & 1 == 1 {
            result = poly_mul_mod(&result, &b0, m);
        }
        bit -= 1;
    }
    result
}

/// Base-field inverse via Fermat: a^(q-2). Panics on 0 (callers ensure nonzero).
fn fq_inv(a: u32) -> u32 {
    assert!(a % Q != 0, "fq_inv of zero");
    // a^(q-2)
    let mut acc: u32 = 1;
    let mut base = a % Q;
    let mut e = (Q - 2) as u64;
    while e > 0 {
        if e & 1 == 1 {
            acc = fq_mul(acc, base);
        }
        base = fq_mul(base, base);
        e >>= 1;
    }
    acc
}

// =============================================================================
// Rabin irreducibility test
// =============================================================================

/// Rabin irreducibility test for a MONIC polynomial f over F_q of degree n.
///
/// f is irreducible iff:
///   (1) x^(q^n) ≡ x (mod f), AND
///   (2) for every prime p | n, gcd(x^(q^(n/p)) - x, f) = 1.
///
/// `f` is given as little-endian coefficients (index = degree); it must be
/// monic with deg(f) = n. `n_prime_divisors` lists the distinct primes dividing
/// n (for n = 12 this is [2, 3]).
pub fn is_irreducible_rabin(f: &[u32], n: usize, n_prime_divisors: &[u32]) -> bool {
    debug_assert_eq!(poly_degree(f), n, "f must have degree n");
    let x_poly: Vec<u32> = vec![0, 1]; // the polynomial x

    // Condition (1): x^(q^n) ≡ x (mod f)
    let xqn = poly_pow_mod(&x_poly, &q_pow_limbs(n as u32), f);
    let diff_n = poly_sub(&xqn, &x_poly);
    if !poly_is_zero(&diff_n) {
        return false;
    }

    // Condition (2): for each prime p | n, gcd(x^(q^(n/p)) - x, f) == 1
    for &p in n_prime_divisors {
        debug_assert_eq!(n % (p as usize), 0);
        let k = (n / p as usize) as u32;
        let xqk = poly_pow_mod(&x_poly, &q_pow_limbs(k), f);
        let diff = poly_sub(&xqk, &x_poly);
        let g = poly_gcd(&diff, f);
        // gcd must be the constant 1 (degree 0, nonzero)
        if poly_degree(&g) != 0 || poly_is_zero(&g) {
            return false;
        }
    }
    true
}

/// Build the binomial modulus x^12 - g as a little-endian coefficient vector.
/// Used by the irreducibility tests (and as documentation of the chosen f).
#[cfg_attr(not(test), allow(dead_code))]
fn binomial_modulus(g: u32) -> Vec<u32> {
    let mut f = vec![0u32; N + 1];
    f[N] = 1; // x^12
    f[0] = fq_sub(0, fq_from_u32(g)); // -g
    f
}

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
            let mut c = [0u32; N];
            for i in 0..N {
                c[i] = self.fq();
            }
            Fq12 { c }
        }
        fn fq12_nonzero(&mut self) -> Fq12 {
            loop {
                let e = self.fq12();
                if !e.is_zero() {
                    return e;
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // Irreducibility: machine-verified for the chosen f, with reducible controls.
    // -------------------------------------------------------------------------

    #[test]
    fn fq12_chosen_modulus_is_irreducible() {
        let f = binomial_modulus(G_MODULUS);
        assert_eq!(poly_degree(&f), N);
        assert!(
            is_irreducible_rabin(&f, N, &[2, 3]),
            "chosen f = x^12 - {} must be Rabin-irreducible",
            G_MODULUS
        );
    }

    #[test]
    fn fq12_binomial_modulus_search_matches_chosen() {
        // g = 2,3,5,7 must FAIL; g = 11 is the first that passes.
        let candidates = [2u32, 3, 5, 7, 11];
        let mut first_irreducible: Option<u32> = None;
        for &g in &candidates {
            let f = binomial_modulus(g);
            let irr = is_irreducible_rabin(&f, N, &[2, 3]);
            if irr && first_irreducible.is_none() {
                first_irreducible = Some(g);
            }
        }
        assert_eq!(
            first_irreducible,
            Some(G_MODULUS),
            "first irreducible binomial g should be {}",
            G_MODULUS
        );
        // explicit: the smaller ones are reducible
        for &g in &[2u32, 3, 5, 7] {
            assert!(
                !is_irreducible_rabin(&binomial_modulus(g), N, &[2, 3]),
                "x^12 - {} should be reducible over F_q",
                g
            );
        }
    }

    #[test]
    fn fq12_reducible_controls_fail_rabin() {
        // Control 1: x^12 - 0 = x^12, obviously reducible.
        let f0 = binomial_modulus(0);
        assert!(!is_irreducible_rabin(&f0, N, &[2, 3]));

        // Control 2: a known reducible degree-12 poly: (x^6)^2 form, i.e.
        // x^12 - 1 = (x^6-1)(x^6+1), reducible.
        let mut f1 = vec![0u32; N + 1];
        f1[N] = 1;
        f1[0] = fq_sub(0, 1); // x^12 - 1
        assert!(!is_irreducible_rabin(&f1, N, &[2, 3]));

        // Control 3: x^12 - (a square that makes it split) — x^12 - 9 where 9
        // need not be; instead use a guaranteed-reducible product
        // (x^11)*(x) -> not monic-degree-12 prime; use x^12 + x^6 = x^6(x^6+1).
        let mut f2 = vec![0u32; N + 1];
        f2[N] = 1;
        f2[6] = 1;
        assert!(!is_irreducible_rabin(&f2, N, &[2, 3]));
    }

    // -------------------------------------------------------------------------
    // Field axioms
    // -------------------------------------------------------------------------

    #[test]
    fn fq12_additive_axioms() {
        let mut rng = Rng(0x1234_5678_9abc_def0);
        for _ in 0..200 {
            let a = rng.fq12();
            let b = rng.fq12();
            let c = rng.fq12();
            // commutativity
            assert_eq!(a.add(&b), b.add(&a));
            // associativity
            assert_eq!(a.add(&b).add(&c), a.add(&b.add(&c)));
            // identity
            assert_eq!(a.add(&Fq12::zero()), a);
            // inverse
            assert_eq!(a.add(&a.neg()), Fq12::zero());
            // sub consistency
            assert_eq!(a.sub(&b), a.add(&b.neg()));
        }
    }

    #[test]
    fn fq12_multiplicative_axioms() {
        let mut rng = Rng(0xdead_beef_cafe_babe);
        for _ in 0..200 {
            let a = rng.fq12();
            let b = rng.fq12();
            let c = rng.fq12();
            // commutativity
            assert_eq!(a.mul(&b), b.mul(&a));
            // associativity
            assert_eq!(a.mul(&b).mul(&c), a.mul(&b.mul(&c)));
            // identity
            assert_eq!(a.mul(&Fq12::one()), a);
            // distributivity
            assert_eq!(a.mul(&b.add(&c)), a.mul(&b).add(&a.mul(&c)));
        }
    }

    #[test]
    fn fq12_inverse_round_trip() {
        let mut rng = Rng(0x0f0f_0f0f_1234_9999);
        for _ in 0..40 {
            let a = rng.fq12_nonzero();
            let ai = a.inv().expect("nonzero has inverse");
            assert_eq!(a.mul(&ai), Fq12::one(), "a * inv(a) must be 1");
        }
        // base-field-embedded elements
        for g in [1u32, 2, 3, 5, 7, 11, 12288] {
            let a = Fq12::from_base(g);
            let ai = a.inv().expect("nonzero base elt has inverse");
            assert_eq!(a.mul(&ai), Fq12::one());
        }
        assert!(Fq12::zero().inv().is_none());
    }

    #[test]
    fn fq12_frobenius_identity_on_base_field() {
        for g in [0u32, 1, 2, 3, 7, 11, 100, 12288] {
            let a = Fq12::from_base(g);
            assert_eq!(a.frobenius(), a, "Frobenius fixes base field");
        }
    }

    #[test]
    fn fq12_frobenius_is_automorphism() {
        let mut rng = Rng(0xabcd_1234_5678_ef01);
        for _ in 0..60 {
            let a = rng.fq12();
            let b = rng.fq12();
            // additive
            assert_eq!(a.add(&b).frobenius(), a.frobenius().add(&b.frobenius()));
            // multiplicative
            assert_eq!(a.mul(&b).frobenius(), a.frobenius().mul(&b.frobenius()));
        }
    }

    #[test]
    fn fq12_x_to_qn_is_identity() {
        // The defining property of GF(q^12): a^(q^12) == a for all a.
        let mut rng = Rng(0x5555_aaaa_3333_cccc);
        let qn = q_pow_limbs(N as u32);
        for _ in 0..20 {
            let a = rng.fq12();
            assert_eq!(a.pow(&qn), a, "a^(q^12) must equal a");
        }
    }

    #[test]
    fn fq12_base_field_consistency() {
        let mut rng = Rng(0x1111_2222_3333_4444);
        for _ in 0..200 {
            let a = rng.fq();
            let b = rng.fq();
            let lhs = Fq12::from_base(a).mul(&Fq12::from_base(b));
            let rhs = Fq12::from_base(fq_mul(a, b));
            assert_eq!(lhs, rhs);
            let ladd = Fq12::from_base(a).add(&Fq12::from_base(b));
            let radd = Fq12::from_base(fq_add(a, b));
            assert_eq!(ladd, radd);
        }
    }

    #[test]
    fn fq12_x_satisfies_modulus() {
        // x^12 == G_MODULUS in the field (sanity on the reduction rule).
        let mut x = Fq12::zero();
        x.c[1] = 1; // the element x
        let mut acc = Fq12::one();
        for _ in 0..N {
            acc = acc.mul(&x);
        }
        assert_eq!(acc, Fq12::from_base(G_MODULUS), "x^12 must equal g");
    }
}
