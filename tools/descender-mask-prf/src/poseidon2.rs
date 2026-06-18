//! Standalone width-16 Poseidon2 permutation, pinned BYTE-IDENTICAL to Plonky3's
//! `default_koalabear_poseidon2_16()`.
//!
//! Every numeric constant below is COPIED VERBATIM from
//! `p3-koala-bear-0.6.1/src/poseidon2.rs` so the two cannot diverge:
//!   * `RC16_EXTERNAL_INITIAL` == `KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL`
//!   * `RC16_EXTERNAL_FINAL`   == `KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL`
//!   * `RC16_INTERNAL`         == `KOALABEAR_POSEIDON2_RC_16_INTERNAL`
//!   * `HALF_FULL_ROUNDS = 4`, `PARTIAL_ROUNDS = 20`, alpha = 3
//!
//! Permutation structure (matches `Poseidon2::permute_mut` in
//! `p3-poseidon2-0.6.1/src/lib.rs`, the *generic* linear layers the proof's AIR
//! uses — `GenericPoseidon2LinearLayersKoalaBear`):
//!   1. external_linear_layer (MDSMat4 light permutation)               -- initial mds
//!   2. 4 initial external rounds: add RC -> sbox(all) -> external_linear_layer
//!   3. 20 internal rounds: add RC to lane 0 -> sbox(lane 0) -> internal_linear_layer
//!   4. 4 terminal external rounds: add RC -> sbox(all) -> external_linear_layer
//!
//! The external linear layer is `mds_light_permutation` with `MDSMat4`
//! ([2 3 1 1; 1 2 3 1; 1 1 2 3; 3 1 1 2]); the internal layer is
//! `s -> (1 + Diag(V))s` with the pinned width-16 optimized diagonal `V`.

use crate::field::{F, P};

/// Poseidon2 width.
pub const WIDTH: usize = 16;
/// Full rounds per half (RF/2). KoalaBear: 4.
pub const HALF_FULL_ROUNDS: usize = 4;
/// Partial (internal) rounds for width-16 KoalaBear: 20.
pub const PARTIAL_ROUNDS: usize = 20;

// ---------------------------------------------------------------------------
// PINNED ROUND CONSTANTS — copied verbatim from p3-koala-bear-0.6.1.
// ---------------------------------------------------------------------------

/// `KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL` (4 rounds x 16).
pub const RC16_EXTERNAL_INITIAL: [[u32; 16]; 4] = [
    [
        0x7ee56a48, 0x11367045, 0x12e41941, 0x7ebbc12b, 0x1970b7d5, 0x662b60e8, 0x3e4990c6,
        0x679f91f5, 0x350813bb, 0x00874ad4, 0x28a0081a, 0x18fa5872, 0x5f25b071, 0x5e5d5998,
        0x5e6fd3e7, 0x5b2e2660,
    ],
    [
        0x6f1837bf, 0x3fe6182b, 0x1edd7ac5, 0x57470d00, 0x43d486d5, 0x1982c70f, 0x0ea53af9,
        0x61d6165b, 0x51639c00, 0x2dec352c, 0x2950e531, 0x2d2cb947, 0x08256cef, 0x1a0109f6,
        0x1f51faf3, 0x5cef1c62,
    ],
    [
        0x3d65e50e, 0x33d91626, 0x133d5a1e, 0x0ff49b0d, 0x38900cd1, 0x2c22cc3f, 0x28852bb2,
        0x06c65a02, 0x7b2cf7bc, 0x68016e1a, 0x15e16bc0, 0x5248149a, 0x6dd212a0, 0x18d6830a,
        0x5001be82, 0x64dac34e,
    ],
    [
        0x5902b287, 0x426583a0, 0x0c921632, 0x3fe028a5, 0x245f8e49, 0x43bb297e, 0x7873dbd9,
        0x3cc987df, 0x286bb4ce, 0x640a8dcd, 0x512a8e36, 0x03a4cf55, 0x481837a2, 0x03d6da84,
        0x73726ac7, 0x760e7fdf,
    ],
];

/// `KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL` (4 rounds x 16).
pub const RC16_EXTERNAL_FINAL: [[u32; 16]; 4] = [
    [
        0x43e7dc24, 0x259a5d61, 0x27e85a3b, 0x1b9133fa, 0x343e5628, 0x485cd4c2, 0x16e269f5,
        0x165b60c6, 0x25f683d9, 0x124f81f9, 0x174331f9, 0x77344dc5, 0x5a821dba, 0x5fc4177f,
        0x54153bf5, 0x5e3f1194,
    ],
    [
        0x3bdbf191, 0x088c84a3, 0x68256c9b, 0x3c90bbc6, 0x6846166a, 0x03f4238d, 0x463335fb,
        0x5e3d3551, 0x6e59ae6f, 0x32d06cc0, 0x596293f3, 0x6c87edb2, 0x08fc60b5, 0x34bcca80,
        0x24f007f3, 0x62731c6f,
    ],
    [
        0x1e1db6c6, 0x0ca409bb, 0x585c1e78, 0x56e94edc, 0x16d22734, 0x18e11467, 0x7b2c3730,
        0x770075e4, 0x35d1b18c, 0x22be3db5, 0x4fb1fbb7, 0x477cb3ed, 0x7d5311c6, 0x5b62ae7d,
        0x559c5fa8, 0x77f15048,
    ],
    [
        0x3211570b, 0x490fef6a, 0x77ec311f, 0x2247171b, 0x4e0ac711, 0x2edf69c9, 0x3b5a8850,
        0x65809421, 0x5619b4aa, 0x362019a7, 0x6bf9d4ed, 0x5b413dff, 0x617e181e, 0x5e7ab57b,
        0x33ad7833, 0x3466c7ca,
    ],
];

/// `KOALABEAR_POSEIDON2_RC_16_INTERNAL` (20 scalar constants).
pub const RC16_INTERNAL: [u32; 20] = [
    0x54dfeb5d, 0x7d40afd6, 0x722cb316, 0x106a4573, 0x45a7ccdb, 0x44061375, 0x154077a5, 0x45744faa,
    0x4eb5e5ee, 0x3794e83f, 0x47c7093c, 0x5694903c, 0x69cb6299, 0x373df84c, 0x46a0df58, 0x46b8758a,
    0x3241ebcb, 0x0b09d233, 0x1af42357, 0x1e66cec2,
];

/// Modular inverse of `2^k` in F_p, for the negative/fractional diagonal entries.
fn inv_2exp(k: u32) -> F {
    F::new(2).pow(k as u64).inverse()
}

/// The pinned width-16 optimized internal diagonal `V` such that the internal
/// linear layer is `s -> (1 + Diag(V))s`, i.e. `state[i] = V[i]*state[i] + sum`.
///
/// From `p3-koala-bear-0.6.1/src/poseidon2.rs` `InternalLayerBaseParameters<.,16>`:
///   V = [-2, 1, 2, 1/2, 3, 4, -1/2, -3, -4,
///        1/2^8, 1/8, 1/2^24, -1/2^8, -1/8, -1/16, -1/2^24]
///
/// (Lane 0's `-2` matches `generic_internal_linear_layer`, which sets
/// `state[0] = part_sum - state[0] = full_sum - 2*state[0]`.)
fn internal_diag() -> [F; WIDTH] {
    let half = F::new(2).inverse(); // 1/2
    [
        F::new(2).neg(),  // -2
        F::ONE,           // 1
        F::new(2),        // 2
        half,             // 1/2
        F::new(3),        // 3
        F::new(4),        // 4
        half.neg(),       // -1/2
        F::new(3).neg(),  // -3
        F::new(4).neg(),  // -4
        inv_2exp(8),      // 1/2^8
        inv_2exp(3),      // 1/8
        inv_2exp(24),     // 1/2^24
        inv_2exp(8).neg(),  // -1/2^8
        inv_2exp(3).neg(),  // -1/8
        inv_2exp(4).neg(),  // -1/16
        inv_2exp(24).neg(), // -1/2^24
    ]
}

/// `apply_mat4`: multiply a 4-vector by `MDSMat4 = [2 3 1 1; 1 2 3 1; 1 1 2 3; 3 1 1 2]`.
/// Mirrors `p3-poseidon2-0.6.1/src/external.rs::apply_mat4`.
#[inline]
fn apply_mat4(x: &mut [F; 4]) {
    let two = F::new(2);
    let three = F::new(3);
    let x0 = x[0];
    let x1 = x[1];
    let x2 = x[2];
    let x3 = x[3];
    x[0] = two.mul(x0).add(three.mul(x1)).add(x2).add(x3);
    x[1] = x0.add(two.mul(x1)).add(three.mul(x2)).add(x3);
    x[2] = x0.add(x1).add(two.mul(x2)).add(three.mul(x3));
    x[3] = three.mul(x0).add(x1).add(x2).add(two.mul(x3));
}

/// External light MDS permutation: M_4 on each 4-chunk, then the outer circulant.
/// Mirrors `mds_light_permutation` for WIDTH=16.
fn external_linear_layer(state: &mut [F; WIDTH]) {
    for chunk_start in (0..WIDTH).step_by(4) {
        let mut chunk = [
            state[chunk_start],
            state[chunk_start + 1],
            state[chunk_start + 2],
            state[chunk_start + 3],
        ];
        apply_mat4(&mut chunk);
        state[chunk_start..chunk_start + 4].copy_from_slice(&chunk);
    }
    // Outer circulant: sums[k] = sum over j step 4 of state[j+k]; state[i] += sums[i%4].
    let mut sums = [F::ZERO; 4];
    for k in 0..4 {
        let mut s = F::ZERO;
        let mut j = 0;
        while j < WIDTH {
            s = s.add(state[j + k]);
            j += 4;
        }
        sums[k] = s;
    }
    for i in 0..WIDTH {
        state[i] = state[i].add(sums[i % 4]);
    }
}

/// Internal linear layer: `s -> (1 + Diag(V))s`.
fn internal_linear_layer(state: &mut [F; WIDTH], diag: &[F; WIDTH]) {
    let mut sum = F::ZERO;
    for &s in state.iter() {
        sum = sum.add(s);
    }
    for i in 0..WIDTH {
        state[i] = diag[i].mul(state[i]).add(sum);
    }
}

#[inline]
fn add_rc_full(state: &mut [F; WIDTH], rc: &[u32; WIDTH]) {
    for i in 0..WIDTH {
        state[i] = state[i].add(F::new(rc[i]));
    }
}

#[inline]
fn sbox_full(state: &mut [F; WIDTH]) {
    for s in state.iter_mut() {
        *s = s.sbox();
    }
}

/// The full width-16 Poseidon2 permutation, byte-identical to
/// `default_koalabear_poseidon2_16().permute_mut`.
pub fn permute(state: &mut [F; WIDTH]) {
    let diag = internal_diag();

    // 1. initial external linear layer.
    external_linear_layer(state);

    // 2. initial external rounds.
    for round in 0..HALF_FULL_ROUNDS {
        add_rc_full(state, &RC16_EXTERNAL_INITIAL[round]);
        sbox_full(state);
        external_linear_layer(state);
    }

    // 3. internal (partial) rounds.
    for round in 0..PARTIAL_ROUNDS {
        state[0] = state[0].add(F::new(RC16_INTERNAL[round]));
        state[0] = state[0].sbox();
        internal_linear_layer(state, &diag);
    }

    // 4. terminal external rounds.
    for round in 0..HALF_FULL_ROUNDS {
        add_rc_full(state, &RC16_EXTERNAL_FINAL[round]);
        sbox_full(state);
        external_linear_layer(state);
    }
}

/// Convenience: permute an array of canonical `u32` field reps.
pub fn permute_u32(input: &[u32; WIDTH]) -> [u32; WIDTH] {
    let mut state = [F::ZERO; WIDTH];
    for i in 0..WIDTH {
        state[i] = F::new(input[i]);
    }
    permute(&mut state);
    let mut out = [0u32; WIDTH];
    for i in 0..WIDTH {
        out[i] = state[i].as_u32();
    }
    out
}

// Compile-time guard that the prime never drifts.
const _: () = assert!(P == 0x7f00_0001);
