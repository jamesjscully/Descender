//! Poseidon2 permutation + sponge over the prime field Fq, q = 12289.
//!
//! Reference: Grassi, Khovratovich, Schofnegger, "Poseidon2: A Faster Version
//! of the Poseidon Hash Function", ePrint 2023/323. Round constants use the
//! Grain LFSR construction from the original Poseidon paper (ePrint 2019/458)
//! and its reference Sage script `generate_parameters_grain.sage`.
//!
//! THIS MODULE IS THE ARITHMETIZATION-FRIENDLY HASH for an account-creation
//! STARK. CORRECTNESS and HONEST labeling are the priorities here, not speed.
//!
//! HONESTY / SECURITY SCOPE (read this before trusting any output):
//!   * The Grain LFSR round-constant generator is VALIDATED against a published
//!     external reference (circomlib BN254 t=2 constants — see
//!     `test_grain_lfsr_matches_published_bn254_reference`). That gives high
//!     confidence the constant generation is the canonical one.
//!   * The external matrix M_E uses the canonical published Poseidon2 M_4 block
//!     and the 2*M_4 / M_4 block assembly for t % 4 == 0. M_4's MDS property is
//!     COMPUTATIONALLY VERIFIED here over Fq (all square submatrices invertible).
//!   * The internal matrix M_I is the CANONICAL Poseidon2 form: all off-diagonal
//!     entries are 1 and the diagonal is mu_i (i.e. M_I = ones(t) + diag(mu_i-1)),
//!     so the partial rounds DO provide full cross-lane diffusion via the
//!     sum-of-all-lanes term. The diagonal mu_i for q=12289, t=24 is a
//!     SELF-CHOSEN deterministic candidate (there is NO published secure
//!     diagonal for this field/width); mu_i are pairwise distinct, != 1, and we
//!     verify M_I is INVERTIBLE over Fq (Gauss-Jordan). Its deep
//!     (subspace-trail / infinitely-long-trail) security is **DEFERRED** to
//!     later cryptanalysis. Do NOT claim it is proven secure.
//!   * R_P = 60 is a deliberately CONSERVATIVE partial-round count. See the
//!     `R_P` constant comment: the canonical round-number formula is invalid at
//!     this 14-bit field size; small-field cryptanalysis is DEFERRED.

// =============================================================================
// Field Fq, q = 12289
// =============================================================================

/// The prime modulus q = 12289 = 3 * 2^12 + 1 (a 14-bit NTT-friendly prime).
pub const Q: u32 = 12289;

/// State width t. Chosen 24 = 4 * 6 so the Poseidon2 external block structure
/// (which requires t divisible by 4) applies.
pub const T: usize = 24;

/// Number of FULL (external) rounds. R_F = 8 split as 4 initial + 4 terminal,
/// per the Poseidon2 construction.
pub const R_F: usize = 8;
pub const R_F_HALF: usize = R_F / 2;

/// Number of PARTIAL (internal) rounds.
///
/// R_P = 60 is a DELIBERATELY CONSERVATIVE choice and is NOT the output of the
/// canonical Poseidon round-number formula. That formula is calibrated for
/// large (~255-bit) fields; at this 14-bit field its naive small-field x^5
/// output would be ~22, which we do NOT trust because the formula's statistical
/// / algebraic-degree assumptions break down for a 14-bit field. We instead use
/// the un-capped large-field x^5 partial-round count (60) as a conservative
/// margin. Tight small-field cryptanalysis (subspace trails, Groebner-basis /
/// algebraic-degree growth at 14 bits) is DEFERRED to later work.
pub const R_P: usize = 60;

/// S-box exponent. x^5 is REQUIRED: gcd(5, q-1) = gcd(5, 12288) = 1, so x^5
/// permutes Fq. x^3 does NOT permute Fq because 3 | 12288. (Asserted in
/// `test_sbox_x5_is_permutation_x3_is_not`.)
pub const SBOX_ALPHA: u64 = 5;

/// Sponge rate (number of input/output lanes absorbed/squeezed per call).
pub const RATE: usize = 5;
/// Sponge capacity. RATE + CAPACITY == T.
pub const CAPACITY: usize = T - RATE; // = 19

/// Maximum number of lanes the sponge will squeeze. Set to 20 so the 20-lane
/// binding commitments (pad-source / opening-tuple / Merkle digest) fit. The
/// squeeze runs ceil(20/RATE)=4 reads of RATE with a permutation between reads.
/// Collision resistance is capacity-limited (~129 bits), so squeezing beyond the
/// capacity buys no extra binding strength.
pub const MAX_SQUEEZE_LANES: usize = 20;

/// Reduce a u32 into [0, Q).
#[inline]
pub fn fq_from_u32(x: u32) -> u32 {
    x % Q
}

/// Field addition.
#[inline]
pub fn fq_add(a: u32, b: u32) -> u32 {
    let s = a + b; // a,b < Q < 2^14, no overflow in u32
    if s >= Q { s - Q } else { s }
}

/// Field subtraction.
#[inline]
pub fn fq_sub(a: u32, b: u32) -> u32 {
    if a >= b { a - b } else { a + Q - b }
}

/// Field multiplication.
#[inline]
pub fn fq_mul(a: u32, b: u32) -> u32 {
    ((a as u64 * b as u64) % Q as u64) as u32
}

/// Field exponentiation by squaring.
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

/// Multiplicative inverse via Fermat: a^(q-2). Returns 0 for input 0.
fn fq_inv(a: u32) -> u32 {
    debug_assert!(a < Q);
    if a == 0 {
        return 0;
    }
    fq_pow(a, (Q - 2) as u64)
}

/// S-box: x^5 over Fq.
#[inline]
pub fn sbox(x: u32) -> u32 {
    let x2 = fq_mul(x, x);
    let x4 = fq_mul(x2, x2);
    fq_mul(x4, x)
}

// =============================================================================
// Grain LFSR round-constant generation (field-generic)
// =============================================================================
//
// This is the canonical Poseidon Grain LFSR from `generate_parameters_grain.sage`.
//
//   INIT_SEQUENCE (80 bits, in append order, all big-endian / MSB-first):
//     FIELD     : 2 bits   (1 => GF(p))
//     SBOX      : 4 bits   (0 => exponentiation S-box x^alpha)
//     FIELD_SIZE: 12 bits  (bit length n of the prime)
//     NUM_CELLS : 12 bits  (state width t)
//     R_F       : 10 bits
//     R_P       : 10 bits
//     padding   : 30 bits  (all ones)
//
//   Stepping: new_bit = s[62] ^ s[51] ^ s[38] ^ s[23] ^ s[13] ^ s[0];
//             pop front, push new_bit at back.
//   Warm-up : discard the first 160 stepped bits.
//   Bit gen : the "self-shrinking" rule — produce a bit b1; if b1 == 0, drop the
//             following bit and try again; if b1 == 1, the following bit is the
//             emitted random bit.
//   Sampling: read n bits MSB-first into an integer; reject (resample) if the
//             value is >= prime.

/// A Grain LFSR over an 80-bit window stored as a VecDeque-like ring of bools.
struct GrainLfsr {
    /// Bit window, index 0 is the "front" (s[0]).
    bits: std::collections::VecDeque<bool>,
}

impl GrainLfsr {
    /// Initialize with the canonical Poseidon init sequence.
    ///
    /// * `field`: FIELD selector (1 for GF(p)).
    /// * `sbox`:  SBOX selector (0 for exponentiation S-box).
    /// * `field_size_bits`: bit length n of the prime modulus.
    /// * `t`, `r_f`, `r_p`: width and round counts.
    fn new(field: u32, sbox: u32, field_size_bits: u32, t: u32, r_f: u32, r_p: u32) -> Self {
        let mut bits = std::collections::VecDeque::with_capacity(80);
        push_bits(&mut bits, field, 2);
        push_bits(&mut bits, sbox, 4);
        push_bits(&mut bits, field_size_bits, 12);
        push_bits(&mut bits, t, 12);
        push_bits(&mut bits, r_f, 10);
        push_bits(&mut bits, r_p, 10);
        for _ in 0..30 {
            bits.push_back(true);
        }
        debug_assert_eq!(bits.len(), 80);
        let mut lfsr = GrainLfsr { bits };
        // Warm-up: discard 160 stepped bits.
        for _ in 0..160 {
            lfsr.step();
        }
        lfsr
    }

    /// One raw LFSR step: compute feedback from taps {62,51,38,23,13,0},
    /// shift, return the new bit.
    fn step(&mut self) -> bool {
        let b = self.bits[62]
            ^ self.bits[51]
            ^ self.bits[38]
            ^ self.bits[23]
            ^ self.bits[13]
            ^ self.bits[0];
        self.bits.pop_front();
        self.bits.push_back(b);
        b
    }

    /// Emit one random bit using the self-shrinking rule.
    fn next_bit(&mut self) -> bool {
        loop {
            let b1 = self.step();
            let b2 = self.step();
            if b1 {
                return b2;
            }
            // b1 == 0: discard b2 and continue.
            let _ = b2;
        }
    }

    /// Read `n` random bits MSB-first into a big-endian byte buffer of the
    /// minimal length that holds `n` bits (most-significant bits in the high
    /// byte). Returns (bytes, n).
    fn next_field_size_bits(&mut self, n: usize) -> Vec<u8> {
        let mut collected: Vec<bool> = Vec::with_capacity(n);
        for _ in 0..n {
            collected.push(self.next_bit());
        }
        bits_to_be_bytes(&collected)
    }
}

/// Append `width` bits of `value` to `bits`, MSB-first (big-endian).
fn push_bits(bits: &mut std::collections::VecDeque<bool>, value: u32, width: u32) {
    for i in (0..width).rev() {
        bits.push_back((value >> i) & 1 == 1);
    }
}

/// Convert an MSB-first bit slice into a big-endian byte vector (the first bit
/// is the most significant bit of the resulting integer).
fn bits_to_be_bytes(bits: &[bool]) -> Vec<u8> {
    let n = bits.len();
    let nbytes = n.div_ceil(8);
    let mut out = vec![0u8; nbytes];
    // bit index i (0 = MSB) has integer weight 2^(n-1-i).
    for (i, &b) in bits.iter().enumerate() {
        if b {
            let pos = n - 1 - i; // bit position from LSB
            out[nbytes - 1 - pos / 8] |= 1 << (pos % 8);
        }
    }
    out
}

/// Compare a big-endian byte integer `a` against a big-endian byte integer `b`.
/// Returns true if a < b. Lengths may differ.
fn be_lt(a: &[u8], b: &[u8]) -> bool {
    // Strip to logical lengths by left-padding the shorter conceptually.
    let la = a.len();
    let lb = b.len();
    let max = la.max(lb);
    for i in 0..max {
        let ai = if i + la >= max { a[i + la - max] } else { 0 };
        let bi = if i + lb >= max { b[i + lb - max] } else { 0 };
        if ai != bi {
            return ai < bi;
        }
    }
    false // equal
}

/// Reduce a big-endian byte integer modulo a small u32 prime, returning the
/// residue. Standard Horner reduction over base 256.
fn be_mod_u32(a: &[u8], prime: u32) -> u32 {
    let mut acc: u64 = 0;
    let p = prime as u64;
    for &byte in a {
        acc = (acc * 256 + byte as u64) % p;
    }
    acc as u32
}

/// Generate `count` round constants in Fq using the canonical Grain LFSR with
/// rejection sampling against the field modulus.
///
/// `prime_be` is the prime modulus as a big-endian byte slice and
/// `field_size_bits` is its bit length. We read `field_size_bits` bits per
/// candidate (MSB-first), reject candidates >= prime, then reduce into Fq.
///
/// NOTE: for q = 12289 the prime is far below 2^14 so rejection is frequent —
/// that is exactly how the reference behaves, and is fine for offline constant
/// generation.
fn generate_round_constants_generic(
    field: u32,
    sbox: u32,
    field_size_bits: u32,
    t: u32,
    r_f: u32,
    r_p: u32,
    prime_be: &[u8],
    count: usize,
) -> Vec<u32> {
    let mut lfsr = GrainLfsr::new(field, sbox, field_size_bits, t, r_f, r_p);
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        loop {
            let cand = lfsr.next_field_size_bits(field_size_bits as usize);
            if be_lt(&cand, prime_be) {
                // Candidate < prime; reduce into Fq. For q=12289 the candidate
                // is already < Q, so this reduction is the identity, but we keep
                // it general/safe.
                out.push(be_mod_u32(&cand, Q));
                break;
            }
        }
    }
    out
}

/// q = 12289 as big-endian bytes.
fn q_be() -> Vec<u8> {
    Q.to_be_bytes().iter().copied().skip_while(|&b| b == 0).collect()
}

/// Bit length of q = 12289. 12289 < 2^14, >= 2^13, so n = 14.
const Q_BITS: u32 = 14;

/// All round constants for our q12289/t24 instance, laid out as
/// `[round][lane]`. Full rounds get a constant per lane; partial rounds use
/// only lane 0 (Poseidon2 adds a constant to the first state element in
/// internal rounds), but for a uniform layout we still GENERATE a full row and
/// only APPLY lane 0 in internal rounds (matching the reference, which still
/// consumes the constants from the stream — see note).
///
/// IMPORTANT (matching the canonical stream): the original Poseidon generates
/// t constants per round for ALL rounds (full and partial) and the permutation
/// only uses the relevant ones. Poseidon2 reduces internal rounds to a single
/// constant. To keep our Grain stream auditable and deterministic we generate
/// exactly the constants we APPLY: t per full round, 1 per partial round.
fn round_constants() -> (Vec<[u32; T]>, Vec<u32>) {
    let prime_be = q_be();
    let total = R_F * T + R_P; // t per full round + 1 per partial round
    let flat = generate_round_constants_generic(
        1, 0, Q_BITS, T as u32, R_F as u32, R_P as u32, &prime_be, total,
    );
    let mut full: Vec<[u32; T]> = Vec::with_capacity(R_F);
    let mut idx = 0;
    for _ in 0..R_F {
        let mut row = [0u32; T];
        for lane in row.iter_mut() {
            *lane = flat[idx];
            idx += 1;
        }
        full.push(row);
    }
    let mut partial: Vec<u32> = Vec::with_capacity(R_P);
    for _ in 0..R_P {
        partial.push(flat[idx]);
        idx += 1;
    }
    (full, partial)
}

// =============================================================================
// External matrix M_E (Poseidon2 block construction for t % 4 == 0)
// =============================================================================
//
// Canonical published M_4 (Poseidon2 / HorizenLabs reference):
//     [ 5 7 1 3 ]
//     [ 4 6 1 1 ]
//     [ 1 3 5 7 ]
//     [ 1 1 4 6 ]
// For t = 4*k, M_E is the block matrix whose (i,j) 4x4 block is:
//     2*M_4 if i == j (diagonal block), else M_4 (off-diagonal block).

const M4: [[u32; 4]; 4] = [[5, 7, 1, 3], [4, 6, 1, 1], [1, 3, 5, 7], [1, 1, 4, 6]];

/// Apply the external matrix M_E to a t-element state, in place, via the block
/// structure. This is O(t^2) but explicit and auditable (speed is not a goal).
///
/// We materialize the full t x t matrix once and multiply. The block rule:
/// block (I,J) is 2*M_4 if I==J else M_4, where I = i/4, J = j/4.
fn build_external_matrix() -> [[u32; T]; T] {
    let mut m = [[0u32; T]; T];
    let blocks = T / 4;
    for bi in 0..blocks {
        for bj in 0..blocks {
            let scale = if bi == bj { 2u32 } else { 1u32 };
            for r in 0..4 {
                for c in 0..4 {
                    m[bi * 4 + r][bj * 4 + c] = fq_mul(fq_from_u32(scale), fq_from_u32(M4[r][c]));
                }
            }
        }
    }
    m
}

pub fn apply_matrix(m: &[[u32; T]; T], state: &mut [u32; T]) {
    let mut out = [0u32; T];
    for (i, out_i) in out.iter_mut().enumerate() {
        let mut acc = 0u32;
        for j in 0..T {
            acc = fq_add(acc, fq_mul(m[i][j], state[j]));
        }
        *out_i = acc;
    }
    *state = out;
}

// =============================================================================
// Internal matrix M_I = (all-ones t x t matrix) + diag(mu_i - 1)   (CANONICAL)
// =============================================================================
//
// CANONICAL Poseidon2 internal matrix. Every OFF-diagonal entry is 1, and the
// diagonal entries are mu_i:
//
//     M_I[i][j] = 1        for i != j
//     M_I[i][i] = mu_i
//
// Equivalently M_I = ones(t) + diag(mu_i - 1). The action factorizes as
//
//     (M_I * x)_i = (sum_j x_j) + (mu_i - 1) * x_i
//
// The "sum of all lanes" term is what gives FULL diffusion in the partial
// rounds — a purely diagonal internal matrix would give none, defeating the
// whole point of Poseidon2's efficient internal rounds. We apply M_I via this
// sum trick, but the result is exactly the all-ones+diagonal matrix product.
//
// There is NO published secure mu_i for q = 12289, t = 24. We generate them
// deterministically from the CONTINUED canonical Grain stream (drawn AFTER the
// round constants). We require:
//   * mu_i pairwise DISTINCT, and
//   * M_I INVERTIBLE over Fq (det != 0, computed via Gauss-Jordan).
// If a draw collides or makes the matrix singular we deterministically nudge it
// (documented below). We ONLY verify M_I is invertible; its subspace-trail /
// long-trail security is DEFERRED. This is a CANDIDATE, NOT a proven-secure
// diagonal. See module-level honesty note.

/// Determinant over Fq of the all-ones+diagonal internal matrix with diagonal
/// `mu`. Using the matrix-determinant lemma for M_I = D' + 1*1^T where
/// D' = diag(mu_i - 1):
///     det(M_I) = (prod_i (mu_i - 1)) * (1 + sum_i 1/(mu_i - 1))
/// valid when all (mu_i - 1) != 0. We instead just build the full matrix and
/// Gauss-Jordan it for an unconditional, auditable check.
fn internal_matrix_is_invertible(mu: &[u32; T]) -> bool {
    invert_matrix(&build_internal_matrix_from_mu(mu)).is_some()
}

/// Build the full t x t internal matrix M_I from a diagonal `mu`:
/// off-diagonal = 1, diagonal[i] = mu[i].
fn build_internal_matrix_from_mu(mu: &[u32; T]) -> [[u32; T]; T] {
    let mut m = [[1u32; T]; T];
    for k in 0..T {
        m[k][k] = mu[k];
    }
    m
}

/// Produce the internal-matrix diagonal `mu_i`. Deterministic and documented
/// above: drawn from the CONTINUED canonical Grain stream, forced pairwise
/// distinct, and adjusted (if needed) so M_I is invertible over Fq.
fn internal_diagonal() -> [u32; T] {
    // Continue the SAME canonical stream past the round constants so the
    // diagonal is fully determined by the published Grain construction. We draw
    // a generous surplus so we can skip collisions / singular candidates
    // deterministically without re-seeding.
    let prime_be = q_be();
    let consumed = R_F * T + R_P; // constants drawn for round constants
    let surplus = 4 * T; // plenty of spare draws for distinctness/invertibility
    let need = consumed + surplus;
    let flat = generate_round_constants_generic(
        1, 0, Q_BITS, T as u32, R_F as u32, R_P as u32, &prime_be, need,
    );

    let mut mu = [0u32; T];
    let mut filled = 0usize;
    let mut src = consumed;
    while filled < T {
        // Next candidate from the continued stream (deterministic).
        let mut cand = if src < need {
            let c = flat[src];
            src += 1;
            c
        } else {
            // Extremely unlikely fallback: deterministic walk over the field.
            ((filled as u32) * 2 + 3) % Q
        };
        // Reject mu_i == 1: that would make M_I[i][i] - 1 == 0, weakening the
        // matrix-determinant-lemma structure (the row would equal the all-ones
        // row). Reject duplicates to keep mu pairwise distinct.
        loop {
            let dup = mu[..filled].iter().any(|&m| m == cand);
            if cand != 1 && !dup {
                break;
            }
            // Deterministic nudge until valid.
            cand = (cand + 1) % Q;
        }
        mu[filled] = cand;
        filled += 1;
    }

    // Ensure invertibility; if singular, deterministically perturb the last
    // entry until M_I is invertible (keeping it distinct and != 1).
    while !internal_matrix_is_invertible(&mu) {
        let last = T - 1;
        let mut cand = (mu[last] + 1) % Q;
        loop {
            let dup = mu[..last].iter().any(|&m| m == cand);
            if cand != 1 && !dup {
                break;
            }
            cand = (cand + 1) % Q;
        }
        mu[last] = cand;
    }
    mu
}

/// Build full t x t internal matrix M_I (all-ones off-diagonal + mu diagonal).
#[cfg(test)]
fn build_internal_matrix() -> [[u32; T]; T] {
    build_internal_matrix_from_mu(&internal_diagonal())
}

/// Apply the internal matrix in place using the sum trick:
///     (M_I * x)_i = (sum_j x_j) + (mu_i - 1) * x_i
/// This equals the all-ones+diagonal matrix product exactly.
pub fn apply_internal(mu: &[u32; T], state: &mut [u32; T]) {
    let mut sum = 0u32;
    for &s in state.iter() {
        sum = fq_add(sum, s);
    }
    for k in 0..T {
        let mu_minus_1 = fq_sub(mu[k], 1);
        state[k] = fq_add(sum, fq_mul(mu_minus_1, state[k]));
    }
}

// =============================================================================
// Precomputed parameter bundle
// =============================================================================

/// All parameters for the q12289/t24 Poseidon2 instance. Built once.
pub struct Poseidon2Params {
    pub m_e: [[u32; T]; T],
    pub internal_diag: [u32; T],
    pub rc_full: Vec<[u32; T]>,
    pub rc_partial: Vec<u32>,
}

impl Poseidon2Params {
    pub fn new() -> Self {
        let (rc_full, rc_partial) = round_constants();
        Poseidon2Params {
            m_e: build_external_matrix(),
            internal_diag: internal_diagonal(),
            rc_full,
            rc_partial,
        }
    }
}

impl Default for Poseidon2Params {
    fn default() -> Self {
        Self::new()
    }
}

/// Process-wide memoized parameter bundle. `Poseidon2Params::new()` regenerates
/// the Grain round constants (and the internal diagonal, which itself draws and
/// Gauss-Jordan-checks the stream) on EVERY call — that recomputation is what
/// made the FRI/Merkle hashing loop spend tens of seconds in constant
/// generation rather than in the permutation. We build the bundle exactly once
/// and hand out a shared reference. The bundle is purely a function of the
/// compile-time constants, so caching changes NO output value — only cost.
static PARAMS: std::sync::OnceLock<Poseidon2Params> = std::sync::OnceLock::new();

/// Borrow the process-wide Poseidon2 parameter bundle, building it on first use.
/// Use this on every hot path instead of `Poseidon2Params::new()`.
pub fn shared_params() -> &'static Poseidon2Params {
    PARAMS.get_or_init(Poseidon2Params::new)
}

// =============================================================================
// Permutation
// =============================================================================

/// The Poseidon2 permutation over Fq^t.
///
/// Structure (ePrint 2023/323):
///   1. Initial external linear layer: state := M_E * state.
///   2. R_F/2 initial full rounds:  add RC (all lanes) -> S-box all lanes -> M_E.
///   3. R_P internal rounds: add RC (lane 0) -> S-box lane 0 -> M_I.
///   4. R_F/2 terminal full rounds: add RC (all lanes) -> S-box all lanes -> M_E.
pub fn permute(params: &Poseidon2Params, state: &mut [u32; T]) {
    for s in state.iter_mut() {
        *s = fq_from_u32(*s);
    }

    // 1. Initial external linear layer.
    apply_matrix(&params.m_e, state);

    // 2. Initial full rounds.
    for r in 0..R_F_HALF {
        let rc = &params.rc_full[r];
        for k in 0..T {
            state[k] = fq_add(state[k], rc[k]);
        }
        for k in 0..T {
            state[k] = sbox(state[k]);
        }
        apply_matrix(&params.m_e, state);
    }

    // 3. Internal (partial) rounds.
    for r in 0..R_P {
        state[0] = fq_add(state[0], params.rc_partial[r]);
        state[0] = sbox(state[0]);
        apply_internal(&params.internal_diag, state);
    }

    // 4. Terminal full rounds.
    for r in 0..R_F_HALF {
        let rc = &params.rc_full[R_F_HALF + r];
        for k in 0..T {
            state[k] = fq_add(state[k], rc[k]);
        }
        for k in 0..T {
            state[k] = sbox(state[k]);
        }
        apply_matrix(&params.m_e, state);
    }
}

// =============================================================================
// Inverse permutation (for the bijectivity round-trip test)
// =============================================================================

/// Inverse S-box: x^(5^{-1} mod (q-1)). Since gcd(5, q-1)=1 there is a unique
/// inverse exponent.
fn sbox_inv_exp() -> u64 {
    // 5 * e ≡ 1 (mod q-1). q-1 = 12288. Find e.
    let modulus: i64 = (Q - 1) as i64;
    // extended euclid for inverse of 5 mod 12288
    let (mut t, mut newt): (i64, i64) = (0, 1);
    let (mut r, mut newr): (i64, i64) = (modulus, 5);
    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }
    if t < 0 {
        t += modulus;
    }
    t as u64
}

fn sbox_inv(x: u32, e: u64) -> u32 {
    fq_pow(x, e)
}

/// Invert a t x t matrix over Fq via Gauss-Jordan. Returns None if singular.
fn invert_matrix(m: &[[u32; T]; T]) -> Option<[[u32; T]; T]> {
    let mut a = *m;
    let mut inv = [[0u32; T]; T];
    for i in 0..T {
        inv[i][i] = 1;
    }
    for col in 0..T {
        // find pivot
        let mut pivot = None;
        for row in col..T {
            if a[row][col] != 0 {
                pivot = Some(row);
                break;
            }
        }
        let p = pivot?;
        a.swap(col, p);
        inv.swap(col, p);
        let pinv = fq_inv(a[col][col]);
        for j in 0..T {
            a[col][j] = fq_mul(a[col][j], pinv);
            inv[col][j] = fq_mul(inv[col][j], pinv);
        }
        for row in 0..T {
            if row == col {
                continue;
            }
            let factor = a[row][col];
            if factor == 0 {
                continue;
            }
            for j in 0..T {
                a[row][j] = fq_sub(a[row][j], fq_mul(factor, a[col][j]));
                inv[row][j] = fq_sub(inv[row][j], fq_mul(factor, inv[col][j]));
            }
        }
    }
    Some(inv)
}

/// Inverse of the Poseidon2 permutation. Used only to prove bijectivity in
/// tests; not needed for hashing.
pub fn permute_inverse(params: &Poseidon2Params, state: &mut [u32; T]) {
    let e = sbox_inv_exp();
    let m_e_inv = invert_matrix(&params.m_e).expect("M_E invertible");
    // internal matrix inverse: M_I is all-ones-off-diagonal + mu diagonal, NOT
    // diagonal, so we invert the full materialized matrix.
    let m_i = build_internal_matrix_from_mu(&params.internal_diag);
    let m_i_inv = invert_matrix(&m_i).expect("M_I invertible");

    for s in state.iter_mut() {
        *s = fq_from_u32(*s);
    }

    // invert step 4: terminal full rounds (reverse order)
    for r in (0..R_F_HALF).rev() {
        apply_matrix(&m_e_inv, state);
        for k in 0..T {
            state[k] = sbox_inv(state[k], e);
        }
        let rc = &params.rc_full[R_F_HALF + r];
        for k in 0..T {
            state[k] = fq_sub(state[k], rc[k]);
        }
    }

    // invert step 3: internal rounds (reverse order)
    for r in (0..R_P).rev() {
        apply_matrix(&m_i_inv, state);
        state[0] = sbox_inv(state[0], e);
        state[0] = fq_sub(state[0], params.rc_partial[r]);
    }

    // invert step 2: initial full rounds (reverse order)
    for r in (0..R_F_HALF).rev() {
        apply_matrix(&m_e_inv, state);
        for k in 0..T {
            state[k] = sbox_inv(state[k], e);
        }
        let rc = &params.rc_full[r];
        for k in 0..T {
            state[k] = fq_sub(state[k], rc[k]);
        }
    }

    // invert step 1: initial external layer
    apply_matrix(&m_e_inv, state);
}

// =============================================================================
// Sponge
// =============================================================================
//
// Sponge with rate r = RATE (5), capacity c = CAPACITY (19), t = 24.
//
// Domain separation & padding:
//   * The capacity is initialized from the domain string: we absorb a length-
//     prefixed SHA-free, simple injective encoding of `domain` into the capacity
//     lanes BEFORE absorbing inputs. Concretely capacity lane c0 is set to the
//     domain byte length mod Q, and the remaining capacity lanes are filled by a
//     small mixing of the domain bytes (so distinct domains start from distinct
//     capacity states). The rate lanes start at 0.
//   * Inputs (u16, each reduced mod Q) are absorbed RATE lanes at a time, each
//     absorption followed by one permutation. The LAST block is padded with the
//     10* rule in the Fq domain: append a single `1` field element, then `0`
//     field elements until the block is full. This makes the padding injective
//     over variable-length input.
//   * Output: after absorbing, squeeze `N` lanes (N <= RATE per squeeze; we
//     permute between squeezes when more than RATE lanes are requested). Each
//     output lane is in [0, Q). Up to 6 lanes are supported by squeezing twice.

/// Initialize the capacity portion of the state from a domain separator.
///
/// `pub(crate)` so the sponge-PRF AIR can pin EXACTLY the same initial capacity
/// constants as the reference sponge (no behavior change).
pub(crate) fn init_capacity(domain: &[u8]) -> [u32; T] {
    let mut state = [0u32; T];
    // capacity lanes occupy indices RATE..T
    // c0 = domain length
    state[RATE] = fq_from_u32(domain.len() as u32);
    // simple deterministic mixing of domain bytes across remaining capacity lanes
    let cap_lanes = CAPACITY - 1;
    if cap_lanes > 0 {
        let mut acc = [0u32; CAPACITY];
        for (i, &b) in domain.iter().enumerate() {
            let lane = i % cap_lanes;
            // mix: acc = acc*31 + b  (mod Q), Horner-style per lane
            acc[lane] = fq_add(fq_mul(acc[lane], fq_from_u32(31)), fq_from_u32(b as u32));
        }
        for j in 0..cap_lanes {
            state[RATE + 1 + j] = acc[j];
        }
    }
    state
}

/// Poseidon2 sponge over Fq=12289. Returns up to MAX_SQUEEZE_LANES (20) output
/// lanes.
///
/// * `domain`: domain-separation tag (arbitrary bytes).
/// * `inputs`: input lanes; each is reduced mod Q before absorption.
/// * `N`: number of output lanes to squeeze (1..=MAX_SQUEEZE_LANES).
///
/// The squeeze loop permutes between reads, so any `n_out` is supported by
/// running `ceil(n_out / RATE)` reads with permutations in between (e.g. n_out=20
/// is 4 reads of 5 with a permutation after each non-final read).
///
/// COLLISION-RESISTANCE NOTE: a binding commitment / Merkle digest must resist
/// COLLISIONS, so its strength is (output bits)/2. A 20-lane output is
/// 20*log2(12289) ≈ 272 bits, BUT collision resistance is also bounded by the
/// sponge CAPACITY (19 lanes ≈ 258 bits) — the relevant internal-collision
/// resistance is min(output, capacity)/2 ≈ 258/2 ≈ 129 bits, adequate for the
/// 128-bit target. (A 10-lane output gave only ~136/2 ≈ 68 bits — too short.)
/// Requesting more than MAX_SQUEEZE_LANES buys no extra binding (capacity cap).
pub fn poseidon2_sponge(domain: &[u8], inputs: &[u16], n_out: usize) -> Vec<u16> {
    assert!(
        n_out >= 1 && n_out <= MAX_SQUEEZE_LANES,
        "n_out must be in 1..=MAX_SQUEEZE_LANES (=20); collision resistance is capacity-limited (~129 bits)"
    );
    let params = shared_params();

    let mut state = init_capacity(domain);

    // Build the padded field-element input stream (10* padding in Fq domain).
    let mut elems: Vec<u32> = inputs.iter().map(|&x| fq_from_u32(x as u32)).collect();
    elems.push(1); // pad marker
    while elems.len() % RATE != 0 {
        elems.push(0);
    }

    // Absorb.
    for block in elems.chunks(RATE) {
        for (i, &e) in block.iter().enumerate() {
            state[i] = fq_add(state[i], e);
        }
        permute(params, &mut state);
    }

    // Squeeze.
    let mut out: Vec<u16> = Vec::with_capacity(n_out);
    let mut remaining = n_out;
    loop {
        let take = remaining.min(RATE);
        for i in 0..take {
            out.push(state[i] as u16);
        }
        remaining -= take;
        if remaining == 0 {
            break;
        }
        permute(params, &mut state);
    }
    out
}

// =============================================================================
// Projection-mask PRF (Poseidon2 replacement for the SHA-256 mask derivation)
// =============================================================================

/// Domain-separation tag for the Poseidon2 projection-mask PRF. Distinct from
/// any SHA-256 domain so the two PRF families can never collide.
pub const POSEIDON2_PROJECTION_MASK_DOMAIN: &[u8] =
    b"Descender.Signature.q12289.local_delta.poseidon2_projection_mask.v1";

/// Number of projection-mask lanes (matches the 6 hidden-source projection
/// lanes the SHA-256 PRF currently emits).
pub const PROJECTION_MASK_LANES: usize = 6;

/// Poseidon2-based projection-mask PRF: the arithmetization-friendly replacement
/// for the SHA-256 `uniform_mod_from_hash` mask derivation.
///
/// Input encoding (chosen to be injective AND AIR-friendly): every input byte is
/// absorbed as its own field element in `[0, 256)`. Small lanes mean the eventual
/// AIR opens `pad_seed`/`slot` with cheap base-256 range checks and the same
/// byte-lane layout used here, so the in-circuit PRF and this native PRF agree by
/// construction.
///   - 32 byte-lanes for `pad_seed`
///   - 8 byte-lanes for `slot_id` (little-endian u64)
/// Squeezes `PROJECTION_MASK_LANES` Fq=12289 output lanes.
///
/// NOTE: this is the native (off-circuit) reference. It is NOT yet wired into the
/// deployed signer/verifier (that swap touches the spend path and the KAT corpus
/// and is staged separately). Deep small-field cryptanalysis of the underlying
/// permutation is DEFERRED per the project decision; R_P is the conservative
/// un-capped count.
pub fn projection_masks_poseidon2(
    pad_seed: &[u8; 32],
    slot_id: u64,
) -> [u16; PROJECTION_MASK_LANES] {
    let mut inputs: Vec<u16> = Vec::with_capacity(40);
    for &b in pad_seed.iter() {
        inputs.push(b as u16);
    }
    for &b in slot_id.to_le_bytes().iter() {
        inputs.push(b as u16);
    }
    let out = poseidon2_sponge(POSEIDON2_PROJECTION_MASK_DOMAIN, &inputs, PROJECTION_MASK_LANES);
    let mut masks = [0u16; PROJECTION_MASK_LANES];
    masks.copy_from_slice(&out);
    masks
}

// =============================================================================
// Pad-source commitment (Poseidon2 replacement for the SHA pad_seed_digest)
// =============================================================================

/// Domain-separation tag for the Poseidon2 pad-source commitment. DISTINCT from
/// the projection-mask PRF domain so the two sponge families can never collide:
/// the SAME hidden `pad_seed` feeds both, but under different domains, so the
/// commitment and the mask PRF are independent functions of pad_seed.
pub const POSEIDON2_PAD_SOURCE_COMMITMENT_DOMAIN: &[u8] =
    b"Descender.AccountCreation.q12289.poseidon2_pad_source_commitment.v1";

/// Number of pad-source commitment output lanes. A COMMITMENT is a binding
/// (collision) object, so its security is (output bits)/2. 20 Fq=12289 lanes ≈
/// 20 * log2(12289) ≈ 272 bits of output, with collision resistance bounded by
/// the sponge capacity (19 lanes ≈ 258 bits) ⇒ ~129-bit collision resistance,
/// adequate for the 128-bit target. (10 lanes ≈ 136 bits gave only ~68-bit
/// collision resistance — too short.)
pub const PAD_SOURCE_COMMITMENT_LANES: usize = 20;

/// Number of blinding-nonce bytes absorbed for hiding. Hidden witness; absorbed
/// as 16 byte-lanes (each in [0, 256)) AFTER the 32 pad_seed byte-lanes.
pub const PAD_SOURCE_BLINDING_BYTES: usize = 16;

/// Poseidon2-based pad-source commitment: the arithmetization-friendly
/// replacement for the SHA-256 `pad_seed_digest` scaffold. This is obligation
/// **C1** (pad-source binding) of the account-creation relation.
///
/// Input encoding (byte-lane, injective AND AIR-friendly, mirroring
/// `projection_masks_poseidon2`): every input byte is absorbed as its own field
/// element in `[0, 256)`:
///   - 32 byte-lanes for `pad_seed`
///   - 16 byte-lanes for `blinding_nonce` (hidden, for hiding)
/// Squeezes `PAD_SOURCE_COMMITMENT_LANES` (= 20) Fq=12289 output lanes
/// (~129-bit collision resistance).
///
/// The SAME `pad_seed` feeds `projection_masks_poseidon2` (C2). Binding both to
/// one pad_seed is the algebraic root that replaces the SHA pad_seed_digest.
pub fn pad_source_commitment_poseidon2(
    pad_seed: &[u8; 32],
    blinding_nonce: &[u8; PAD_SOURCE_BLINDING_BYTES],
) -> [u16; PAD_SOURCE_COMMITMENT_LANES] {
    let mut inputs: Vec<u16> = Vec::with_capacity(32 + PAD_SOURCE_BLINDING_BYTES);
    for &b in pad_seed.iter() {
        inputs.push(b as u16);
    }
    for &b in blinding_nonce.iter() {
        inputs.push(b as u16);
    }
    let out = poseidon2_sponge(
        POSEIDON2_PAD_SOURCE_COMMITMENT_DOMAIN,
        &inputs,
        PAD_SOURCE_COMMITMENT_LANES,
    );
    let mut commit = [0u16; PAD_SOURCE_COMMITMENT_LANES];
    commit.copy_from_slice(&out);
    commit
}

// =============================================================================
// Opening-tuple commitment (Poseidon2 — obligation C6)
// =============================================================================

/// Domain-separation tag for the Poseidon2 opening-tuple commitment. DISTINCT
/// from every other Poseidon2 domain (mask PRF, pad-source commitment) so the
/// sponge families can never collide. This commitment binds the SAME
/// `account_projection` that the C4 carrier reconstructs, plus a lifecycle
/// anchor, hidden under a blinding nonce.
pub const POSEIDON2_OPENING_TUPLE_COMMITMENT_DOMAIN: &[u8] =
    b"Descender.AccountCreation.q12289.poseidon2_opening_tuple_commitment.v1";

/// Number of `account_projection` lanes absorbed as FIELD lanes (not bytes).
/// Matches the 6 hidden-source projection lanes the C4 carrier reconstructs.
pub const OPENING_TUPLE_ACCOUNT_PROJECTION_LANES: usize = 6;

/// Number of lifecycle-anchor byte-lanes (each in [0, 256)).
pub const OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES: usize = 32;

/// Number of blinding-nonce byte-lanes (each in [0, 256)), for hiding.
pub const OPENING_TUPLE_BLINDING_BYTES: usize = 16;

/// Number of opening-tuple commitment output lanes. As a binding (collision)
/// object its security is (output bits)/2. 20 Fq=12289 lanes ≈ 272 bits of
/// output, collision-bounded by the sponge capacity (~258 bits) ⇒ ~129-bit
/// collision resistance, adequate for the 128-bit target and matching the
/// pad-source commitment. (10 lanes ≈ 136 bits gave only ~68-bit collision
/// resistance — too short.)
pub const OPENING_TUPLE_COMMITMENT_LANES: usize = 20;

/// Poseidon2-based opening-tuple commitment: obligation **C6** of the
/// account-creation relation. It commits the 6-lane `account_projection`
/// (binding it into the opening tuple) plus a 32-byte `lifecycle_anchor`, hidden
/// under a 16-byte `blinding_nonce`.
///
/// Input encoding (injective AND AIR-friendly):
///   - 6 FIELD lanes for `account_projection` (each already a canonical residue
///     < q — absorbed AS field elements, NOT byte-decomposed).
///   - 32 byte-lanes for `lifecycle_anchor` (each in [0, 256)).
///   - 16 byte-lanes for `blinding_nonce` (each in [0, 256)).
/// Squeezes `OPENING_TUPLE_COMMITMENT_LANES` (= 20) Fq=12289 output lanes
/// (~129-bit collision resistance).
///
/// The SAME `account_projection` feeds the C4 base-128 carrier; binding both to
/// one `account_projection` (via the SeamOpeningAcc constraints) is what makes
/// the opening-tuple commitment a non-vacuous co-location with the carrier.
pub fn opening_tuple_commitment_poseidon2(
    account_projection: &[u16; OPENING_TUPLE_ACCOUNT_PROJECTION_LANES],
    lifecycle_anchor: &[u8; OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES],
    blinding_nonce: &[u8; OPENING_TUPLE_BLINDING_BYTES],
) -> [u16; OPENING_TUPLE_COMMITMENT_LANES] {
    let mut inputs: Vec<u16> = Vec::with_capacity(
        OPENING_TUPLE_ACCOUNT_PROJECTION_LANES
            + OPENING_TUPLE_LIFECYCLE_ANCHOR_BYTES
            + OPENING_TUPLE_BLINDING_BYTES,
    );
    // 6 account_projection FIELD lanes (each a canonical residue < q).
    for &v in account_projection.iter() {
        debug_assert!((v as u32) < Q, "account_projection lane must be canonical (< q)");
        inputs.push(v);
    }
    // 32 lifecycle_anchor byte-lanes.
    for &b in lifecycle_anchor.iter() {
        inputs.push(b as u16);
    }
    // 16 blinding_nonce byte-lanes.
    for &b in blinding_nonce.iter() {
        inputs.push(b as u16);
    }
    let out = poseidon2_sponge(
        POSEIDON2_OPENING_TUPLE_COMMITMENT_DOMAIN,
        &inputs,
        OPENING_TUPLE_COMMITMENT_LANES,
    );
    let mut commit = [0u16; OPENING_TUPLE_COMMITMENT_LANES];
    commit.copy_from_slice(&out);
    commit
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opening_tuple_commitment() {
        let acc_a: [u16; 6] = [1, 127, 128, 12288, 6000, 0];
        let mut acc_b = acc_a;
        acc_b[3] = 12287;
        let anchor_a = [11u8; 32];
        let mut anchor_b = [11u8; 32];
        anchor_b[31] = 12;
        let nonce_a = [5u8; 16];
        let mut nonce_b = [5u8; 16];
        nonce_b[0] = 6;

        let c_a = opening_tuple_commitment_poseidon2(&acc_a, &anchor_a, &nonce_a);
        // determinism
        assert_eq!(c_a, opening_tuple_commitment_poseidon2(&acc_a, &anchor_a, &nonce_a));
        // range + lane count
        assert_eq!(c_a.len(), OPENING_TUPLE_COMMITMENT_LANES);
        for &x in c_a.iter() {
            assert!(u32::from(x) < Q, "commitment lane {x} not reduced mod Q");
        }
        // sensitivity to account_projection
        assert_ne!(
            c_a,
            opening_tuple_commitment_poseidon2(&acc_b, &anchor_a, &nonce_a),
            "commitment must depend on account_projection"
        );
        // sensitivity to lifecycle_anchor
        assert_ne!(
            c_a,
            opening_tuple_commitment_poseidon2(&acc_a, &anchor_b, &nonce_a),
            "commitment must depend on lifecycle_anchor"
        );
        // sensitivity to blinding_nonce (hiding)
        assert_ne!(
            c_a,
            opening_tuple_commitment_poseidon2(&acc_a, &anchor_a, &nonce_b),
            "commitment must depend on blinding_nonce"
        );
        // distinct domain from the pad-source commitment: even reusing the same
        // 16-byte nonce, a different domain yields a different function.
        let pad_seed = [11u8; 32];
        let pad_commit = pad_source_commitment_poseidon2(&pad_seed, &nonce_a);
        assert_ne!(
            c_a, pad_commit,
            "opening-tuple commitment must differ from pad-source commitment (domain separation)"
        );
    }

    #[test]
    fn test_pad_source_commitment() {
        let pad_a = [7u8; 32];
        let mut pad_b = [7u8; 32];
        pad_b[31] = 8;
        let nonce_a = [3u8; 16];
        let mut nonce_b = [3u8; 16];
        nonce_b[0] = 4;

        let c_a = pad_source_commitment_poseidon2(&pad_a, &nonce_a);
        // determinism
        assert_eq!(c_a, pad_source_commitment_poseidon2(&pad_a, &nonce_a));
        // range
        for &x in c_a.iter() {
            assert!(u32::from(x) < Q, "commitment lane {x} not reduced mod Q");
        }
        // 10 lanes
        assert_eq!(c_a.len(), PAD_SOURCE_COMMITMENT_LANES);
        // sensitivity to pad_seed
        let c_b = pad_source_commitment_poseidon2(&pad_b, &nonce_a);
        assert_ne!(c_a, c_b, "commitment must depend on pad_seed");
        // sensitivity to blinding nonce (hiding requires nonce to change output)
        let c_n = pad_source_commitment_poseidon2(&pad_a, &nonce_b);
        assert_ne!(c_a, c_n, "commitment must depend on blinding_nonce");
        // distinct domain from the mask PRF: same pad_seed, different function.
        // (compare first 6 lanes against the 6-lane mask PRF output)
        let masks = projection_masks_poseidon2(&pad_a, 0);
        assert_ne!(
            &c_a[0..6],
            &masks[0..6],
            "pad-source commitment must differ from mask PRF (domain separation)"
        );
    }

    #[test]
    fn test_projection_masks_prf() {
        let pad_a = [7u8; 32];
        let mut pad_b = [7u8; 32];
        pad_b[31] = 8; // single-byte difference

        let m_a0 = projection_masks_poseidon2(&pad_a, 0);
        // determinism
        assert_eq!(m_a0, projection_masks_poseidon2(&pad_a, 0));
        // range: every lane is a canonical Fq element
        for &x in m_a0.iter() {
            assert!(u32::from(x) < Q, "mask lane {x} not reduced mod Q");
        }
        // sensitivity to slot_id
        let m_a1 = projection_masks_poseidon2(&pad_a, 1);
        assert_ne!(m_a0, m_a1, "masks must depend on slot_id");
        // sensitivity to pad_seed (one byte)
        let m_b0 = projection_masks_poseidon2(&pad_b, 0);
        assert_ne!(m_a0, m_b0, "masks must depend on pad_seed");
        // 6 lanes
        assert_eq!(m_a0.len(), PROJECTION_MASK_LANES);
    }

    #[test]
    fn test_field_basics() {
        assert_eq!(fq_add(Q - 1, 1), 0);
        assert_eq!(fq_sub(0, 1), Q - 1);
        assert_eq!(fq_mul(2, 3), 6);
        // inverse round-trip
        for a in 1..Q {
            let inv = fq_inv(a);
            assert_eq!(fq_mul(a, inv), 1, "inv failed for {a}");
        }
    }

    /// x^5 is a permutation of Fq; x^3 is not (3 | q-1).
    #[test]
    fn test_sbox_x5_is_permutation_x3_is_not() {
        // gcd(5, q-1) == 1
        fn gcd(mut a: u64, mut b: u64) -> u64 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }
        assert_eq!(gcd(5, (Q - 1) as u64), 1, "5 must be coprime to q-1");
        assert_eq!(gcd(3, (Q - 1) as u64), 3, "3 must divide q-1 (x^3 not a permutation)");

        // x^5 hits every residue exactly once.
        let mut seen5 = vec![false; Q as usize];
        for x in 0..Q {
            let y = fq_pow(x, 5) as usize;
            assert!(!seen5[y], "x^5 collision -> not a permutation");
            seen5[y] = true;
        }
        assert!(seen5.iter().all(|&b| b), "x^5 not surjective");

        // x^3 must collide (not a permutation).
        let mut seen3 = vec![false; Q as usize];
        let mut collided = false;
        for x in 0..Q {
            let y = fq_pow(x, 3) as usize;
            if seen3[y] {
                collided = true;
                break;
            }
            seen3[y] = true;
        }
        assert!(collided, "x^3 unexpectedly a permutation");
    }

    /// VALIDATION AGAINST PUBLISHED EXTERNAL REFERENCE.
    ///
    /// We run the Grain LFSR with the canonical arguments for BN254, t=2, x^5,
    /// R_F=8, R_P=56 (the exact `generate_parameters_grain.sage 1 0 254 2 8 56`
    /// invocation documented by circomlib) and reproduce the FIRST published
    /// round constants from circomlib's poseidon_constants (plain integer form):
    ///   C[0] = 0x09c46e9ec68e9bd4fe1faaba294cba38a71aa177534cdd1b6c7dc0dbd0abd7a7
    ///   C[1] = 0x0c0356530896eec42a97ed937f3135cfc5142b3ae405b8343c1d83ffa604cb81
    ///   C[2] = 0x1e28a1d935698ad1142e51182bb54cf4a00ea5aabd6268bd317ea977cc154a30
    /// Source: iden3/circomlibjs src/poseidon_constants.js (BN254 scalar field).
    ///
    /// Because these values are < the BN254 prime, the rejection-sampled
    /// candidate equals the constant exactly, so we compare the raw 254-bit
    /// big-endian candidate bytes (NO mod reduction, which would be identity)
    /// against the expected big-endian bytes.
    #[test]
    fn test_grain_lfsr_matches_published_bn254_reference() {
        // BN254 scalar field prime.
        let prime_hex = "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001";
        let prime_be = hex_to_be_bytes(prime_hex);
        let field_size_bits = 254u32;

        let expected = [
            "09c46e9ec68e9bd4fe1faaba294cba38a71aa177534cdd1b6c7dc0dbd0abd7a7",
            "0c0356530896eec42a97ed937f3135cfc5142b3ae405b8343c1d83ffa604cb81",
            "1e28a1d935698ad1142e51182bb54cf4a00ea5aabd6268bd317ea977cc154a30",
        ];

        let mut lfsr = GrainLfsr::new(1, 0, field_size_bits, 2, 8, 56);
        for (i, exp_hex) in expected.iter().enumerate() {
            let cand = loop {
                let c = lfsr.next_field_size_bits(field_size_bits as usize);
                if be_lt(&c, &prime_be) {
                    break c;
                }
            };
            let exp = hex_to_be_bytes(exp_hex);
            // normalize both to fixed 32-byte width for comparison
            assert_eq!(
                left_pad(&cand, 32),
                left_pad(&exp, 32),
                "Grain constant #{i} mismatch vs published circomlib BN254 reference"
            );
        }
    }

    fn hex_to_be_bytes(h: &str) -> Vec<u8> {
        let h = h.trim_start_matches("0x");
        let h = if h.len() % 2 == 1 {
            format!("0{h}")
        } else {
            h.to_string()
        };
        (0..h.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap())
            .collect()
    }

    fn left_pad(b: &[u8], width: usize) -> Vec<u8> {
        let mut out = vec![0u8; width.saturating_sub(b.len())];
        out.extend_from_slice(b);
        out
    }

    /// M_4 is MDS over Fq: every square submatrix is invertible (det != 0).
    /// Equivalently no minor vanishes. We check all 1x1..4x4 minors directly.
    #[test]
    fn test_m4_is_mds() {
        // Build M_4 over Fq.
        let m: Vec<Vec<u32>> = M4
            .iter()
            .map(|row| row.iter().map(|&x| fq_from_u32(x)).collect())
            .collect();

        // determinant of a small matrix over Fq via Laplace (n<=4).
        fn det(m: &[Vec<u32>]) -> u32 {
            let n = m.len();
            if n == 1 {
                return m[0][0];
            }
            let mut acc = 0u32;
            for c in 0..n {
                let minor: Vec<Vec<u32>> = m[1..]
                    .iter()
                    .map(|row| {
                        row.iter()
                            .enumerate()
                            .filter(|&(j, _)| j != c)
                            .map(|(_, &v)| v)
                            .collect()
                    })
                    .collect();
                let term = fq_mul(m[0][c], det(&minor));
                if c % 2 == 0 {
                    acc = fq_add(acc, term);
                } else {
                    acc = fq_sub(acc, term);
                }
            }
            acc
        }

        // all square submatrices: choose r rows and r cols, check det != 0.
        for r in 1..=4 {
            for rows in combinations(4, r) {
                for cols in combinations(4, r) {
                    let sub: Vec<Vec<u32>> = rows
                        .iter()
                        .map(|&ri| cols.iter().map(|&ci| m[ri][ci]).collect())
                        .collect();
                    assert_ne!(det(&sub), 0, "M_4 has a vanishing minor -> not MDS");
                }
            }
        }
    }

    fn combinations(n: usize, r: usize) -> Vec<Vec<usize>> {
        let mut out = vec![];
        let mut idx: Vec<usize> = (0..r).collect();
        if r == 0 {
            return vec![vec![]];
        }
        loop {
            out.push(idx.clone());
            let mut i = r;
            loop {
                if i == 0 {
                    return out;
                }
                i -= 1;
                if idx[i] != i + n - r {
                    break;
                }
            }
            idx[i] += 1;
            for j in i + 1..r {
                idx[j] = idx[j - 1] + 1;
            }
        }
    }

    /// M_E (full t x t) is invertible over Fq.
    #[test]
    fn test_external_matrix_invertible() {
        let m = build_external_matrix();
        assert!(invert_matrix(&m).is_some(), "M_E must be invertible");
        // round-trip: M_E * M_E^{-1} = I
        let inv = invert_matrix(&m).unwrap();
        for i in 0..T {
            for j in 0..T {
                let mut acc = 0u32;
                for k in 0..T {
                    acc = fq_add(acc, fq_mul(m[i][k], inv[k][j]));
                }
                let expect = if i == j { 1 } else { 0 };
                assert_eq!(acc, expect, "M_E * M_E^-1 != I at ({i},{j})");
            }
        }
    }

    /// M_I (all-ones off-diagonal + mu diagonal) is invertible over Fq, the mu
    /// entries are pairwise distinct, and none equals 1.
    #[test]
    fn test_internal_matrix_invertible() {
        let mu = internal_diagonal();
        // pairwise distinct + none == 1
        for i in 0..T {
            assert_ne!(mu[i], 1, "mu[{i}] == 1 (forbidden)");
            for j in (i + 1)..T {
                assert_ne!(mu[i], mu[j], "mu[{i}] == mu[{j}] (not distinct)");
            }
        }
        // off-diagonal must all be 1, diagonal must be mu
        let m = build_internal_matrix();
        for i in 0..T {
            for j in 0..T {
                if i == j {
                    assert_eq!(m[i][j], mu[i], "diagonal entry mismatch at {i}");
                } else {
                    assert_eq!(m[i][j], 1, "off-diagonal entry != 1 at ({i},{j})");
                }
            }
        }
        // full-matrix invertibility (det != 0 via Gauss-Jordan)
        let inv = invert_matrix(&m);
        assert!(inv.is_some(), "M_I must be invertible");
        // round-trip M_I * M_I^-1 = I
        let inv = inv.unwrap();
        for i in 0..T {
            for j in 0..T {
                let mut acc = 0u32;
                for k in 0..T {
                    acc = fq_add(acc, fq_mul(m[i][k], inv[k][j]));
                }
                let expect = if i == j { 1 } else { 0 };
                assert_eq!(acc, expect, "M_I * M_I^-1 != I at ({i},{j})");
            }
        }
        // the sum-trick apply must equal the explicit matrix multiply
        let mut s1 = [0u32; T];
        for k in 0..T {
            s1[k] = (k as u32 * 37 + 11) % Q;
        }
        let mut s2 = s1;
        apply_internal(&mu, &mut s1);
        apply_matrix(&m, &mut s2);
        assert_eq!(s1, s2, "apply_internal sum-trick != explicit M_I multiply");
    }

    /// Permutation is deterministic.
    #[test]
    fn test_permutation_deterministic() {
        let params = Poseidon2Params::new();
        let mut a = [0u32; T];
        for k in 0..T {
            a[k] = (k as u32 * 123 + 7) % Q;
        }
        let mut b = a;
        permute(&params, &mut a);
        permute(&params, &mut b);
        assert_eq!(a, b);
    }

    /// Permutation BIJECTIVITY: inverse round-trips on many random states.
    #[test]
    fn test_permutation_bijective_roundtrip() {
        let params = Poseidon2Params::new();
        // simple deterministic PRNG (xorshift) so the test is reproducible.
        let mut s: u64 = 0x1234_5678_9abc_def0;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        for _ in 0..200 {
            let mut state = [0u32; T];
            for k in 0..T {
                state[k] = (next() % Q as u64) as u32;
            }
            let orig = state;
            permute(&params, &mut state);
            permute_inverse(&params, &mut state);
            assert_eq!(state, orig, "permutation not bijective (round-trip failed)");
        }
    }

    /// FIXED KNOWN-ANSWER vector (regression guard). This pins the current
    /// construction; if any parameter/algorithm changes, this test catches it.
    ///
    /// NOTE: this KAT is SELF-GENERATED by THIS implementation (there is no
    /// external Poseidon2-q12289 reference to compare against). It only guards
    /// against future regressions, it does NOT independently validate the
    /// permutation. The Grain LFSR is the part validated externally (see
    /// `test_grain_lfsr_matches_published_bn254_reference`).
    #[test]
    fn test_known_answer_vector() {
        let params = Poseidon2Params::new();
        let mut state = [0u32; T];
        for k in 0..T {
            state[k] = k as u32; // input 0,1,2,...,23
        }
        permute(&params, &mut state);
        // Recorded output (regression anchor). All lanes < Q.
        for &v in state.iter() {
            assert!(v < Q);
        }
        // Print on failure so a deliberate update is easy.
        let expected = KAT_EXPECTED;
        assert_eq!(state, expected, "KAT regression: state = {state:?}");
    }

    /// Sponge determinism + output range + length.
    #[test]
    fn test_sponge_determinism_and_range() {
        let d = b"descender-account-creation-v1";
        let inputs: Vec<u16> = (0..13u16).collect();
        let a = poseidon2_sponge(d, &inputs, 6);
        let b = poseidon2_sponge(d, &inputs, 6);
        assert_eq!(a, b, "sponge not deterministic");
        assert_eq!(a.len(), 6);
        for &x in &a {
            assert!((x as u32) < Q, "sponge output out of range");
        }
        // domain separation: different domain -> different output (overwhelmingly)
        let c = poseidon2_sponge(b"other-domain", &inputs, 6);
        assert_ne!(a, c, "domain separation failed");
        // different input -> different output
        let mut inputs2 = inputs.clone();
        inputs2[0] = 999;
        let e = poseidon2_sponge(d, &inputs2, 6);
        assert_ne!(a, e, "input sensitivity failed");
    }

    /// q12289 round-constant sanity: all generated constants are < Q.
    #[test]
    fn test_round_constants_in_range() {
        let (full, partial) = round_constants();
        assert_eq!(full.len(), R_F);
        assert_eq!(partial.len(), R_P);
        for row in &full {
            for &c in row {
                assert!(c < Q);
            }
        }
        for &c in &partial {
            assert!(c < Q);
        }
    }
}

/// Pinned KAT output: permute([0,1,2,...,23]) for this q12289/t24 instance.
/// SELF-GENERATED regression anchor (no external Poseidon2-q12289 reference
/// exists). Generated by this implementation; recorded so future changes are
/// caught as regressions.
#[cfg(test)]
const KAT_EXPECTED: [u32; T] = [
    2057, 3937, 3448, 2299, 11170, 4033, 2164, 2811, 6970, 2315, 2108, 8185, 7185, 2295, 2942,
    7534, 7818, 10183, 7866, 10692, 236, 9767, 7793, 2044,
];
