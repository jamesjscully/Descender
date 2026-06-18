//! The projection-mask PRF: the C2 sponge over `(pad_seed[32] ‖ slot[8])` byte
//! lanes, squeezed to 30 field lanes, then the wide-then-reduce gadget to 6
//! q12289 masks.
//!
//! This mirrors EXACTLY the in-proof derivation in
//! `../descender-account-stark/src/air.rs`:
//!   * data lanes  = pad_seed bytes (32) ‖ slot_id.to_le_bytes() (8)   = 40 lanes
//!   * `SpongePlan::new`: padded len = round_up(n_input + 1, RATE); a single ONE
//!     padding element is placed at index `n_input` (`build_elems`).
//!   * `run_sponge`: capacity-preserving absorb (first perm seeds rate lanes,
//!     subsequent absorb perms ADD into rate lanes and carry capacity), then read
//!     `NUM_SQUEEZE` outputs from the post-states starting at the last absorb perm
//!     (`output_source`).
//!   * `wide_reduce_masks`: `mask_j = (Σ_i lane_{j,i} * p^i) mod q12289` via Horner
//!     over the 5 lanes per mask, most-significant digit first.

use crate::field::{F, P};
use crate::poseidon2::{permute, WIDTH};

/// Sponge rate (elements absorbed/squeezed per permutation) for WIDTH=16.
pub const RATE: usize = 8;
/// q12289 modulus for the projection-mask lanes.
pub const Q12289: u32 = 12289;
/// Number of projection-mask lanes (outputs).
pub const NUM_OUTPUTS: usize = 6;
/// Squeezed field lanes per mask (base-`p` digits of the wide value).
pub const LANES_PER_MASK: usize = 5;
/// Total squeezed field lanes = 6 masks * 5 lanes = 30.
pub const NUM_SQUEEZE: usize = NUM_OUTPUTS * LANES_PER_MASK;
/// C2 input byte-lanes: pad_seed[32] ‖ slot_id[8].
pub const C2_INPUT_LANES: usize = 32 + 8;

/// Static sponge plan (mirrors `air.rs::SpongePlan`).
struct SpongePlan {
    n_input: usize,
    n_out: usize,
    n_absorb: usize,
    n_perms: usize,
}

impl SpongePlan {
    fn new(n_input: usize, n_out: usize) -> Self {
        let padded = {
            let mut len = n_input + 1;
            while len % RATE != 0 {
                len += 1;
            }
            len
        };
        let n_absorb = padded / RATE;
        let mut remaining = n_out;
        let mut n_squeeze = 0usize;
        loop {
            let take = remaining.min(RATE);
            remaining -= take;
            if remaining == 0 {
                break;
            }
            n_squeeze += 1;
        }
        SpongePlan {
            n_input,
            n_out,
            n_absorb,
            n_perms: n_absorb + n_squeeze,
        }
    }

    fn padded_len(&self) -> usize {
        self.n_absorb * RATE
    }

    /// Which post-state and lane a given output index reads from (mirrors
    /// `air.rs::SpongePlan::output_source`).
    fn output_source(&self, out: usize) -> (usize, usize) {
        let mut filled = 0usize;
        let mut read_perm = self.n_absorb - 1;
        let mut remaining = self.n_out;
        loop {
            let take = remaining.min(RATE);
            for i in 0..take {
                if filled == out {
                    return (read_perm, i);
                }
                filled += 1;
            }
            remaining -= take;
            if remaining == 0 {
                break;
            }
            read_perm += 1;
        }
        unreachable!("output index out of range")
    }
}

/// Build the absorbed `elems` (mirrors `air.rs::build_elems`): data at [0..n_input],
/// a ONE padding marker at index `n_input`, rest zero.
fn build_elems(data: &[u32], plan: &SpongePlan) -> Vec<F> {
    let mut elems = vec![F::ZERO; plan.padded_len()];
    for (i, &v) in data.iter().enumerate() {
        elems[i] = F::new(v);
    }
    elems[plan.n_input] = F::ONE;
    elems
}

/// Run the sponge over `elems` and squeeze `plan.n_out` field outputs (mirrors
/// `air.rs::run_sponge`).
fn run_sponge(elems: &[F], plan: &SpongePlan) -> Vec<F> {
    let mut posts: Vec<[F; WIDTH]> = Vec::with_capacity(plan.n_perms);
    let mut prev_post = [F::ZERO; WIDTH];
    for p in 0..plan.n_perms {
        let mut state = [F::ZERO; WIDTH];
        if p == 0 {
            for r in 0..RATE {
                state[r] = elems[r];
            }
        } else if p < plan.n_absorb {
            for r in 0..RATE {
                let pos = p * RATE + r;
                let add = if pos < plan.padded_len() {
                    elems[pos]
                } else {
                    F::ZERO
                };
                state[r] = prev_post[r].add(add);
            }
            for c in RATE..WIDTH {
                state[c] = prev_post[c];
            }
        } else {
            state = prev_post;
        }
        permute(&mut state);
        posts.push(state);
        prev_post = state;
    }

    let mut outputs = vec![F::ZERO; plan.n_out];
    for o in 0..plan.n_out {
        let (sp, lane) = plan.output_source(o);
        outputs[o] = posts[sp][lane];
    }
    outputs
}

/// Reference wide-then-reduce: `mask_j = (Σ_i lane_{j,i} * p^i) mod q12289`
/// (mirrors `reduce_gadget.rs::wide_reduce_masks`).
fn wide_reduce_masks(lanes: &[u32; NUM_SQUEEZE]) -> [u32; NUM_OUTPUTS] {
    let p = P as u64;
    let q = Q12289 as u64;
    let mut masks = [0u32; NUM_OUTPUTS];
    for j in 0..NUM_OUTPUTS {
        let mut r: u64 = 0;
        for t in 0..LANES_PER_MASK {
            let digit = lanes[j * LANES_PER_MASK + (LANES_PER_MASK - 1 - t)] as u64;
            r = (r * p + digit) % q;
        }
        masks[j] = r as u32;
    }
    masks
}

/// Build the 40-lane C2 input data: `pad_seed[32] ‖ slot_id.to_le_bytes()[8]`.
fn c2_data(pad_seed: &[u8; 32], slot_id: u64) -> Vec<u32> {
    let mut data: Vec<u32> = pad_seed.iter().map(|&b| b as u32).collect();
    data.extend(slot_id.to_le_bytes().iter().map(|&b| b as u32));
    data
}

/// The 30 squeezed field lanes of the C2 sponge (intermediate; exposed for the
/// equivalence test). Each is a canonical KoalaBear residue.
pub fn squeezed_lanes(pad_seed: &[u8; 32], slot_id: u64) -> [u32; NUM_SQUEEZE] {
    let data = c2_data(pad_seed, slot_id);
    debug_assert_eq!(data.len(), C2_INPUT_LANES);
    let plan = SpongePlan::new(C2_INPUT_LANES, NUM_SQUEEZE);
    let elems = build_elems(&data, &plan);
    let outputs = run_sponge(&elems, &plan);
    let mut lanes = [0u32; NUM_SQUEEZE];
    for (i, o) in outputs.iter().enumerate() {
        lanes[i] = o.as_u32();
    }
    lanes
}

/// The 6 q12289 projection masks as `u32` (each `< Q12289`).
pub fn projection_masks_u32(pad_seed: &[u8; 32], slot_id: u64) -> [u32; NUM_OUTPUTS] {
    let lanes = squeezed_lanes(pad_seed, slot_id);
    wide_reduce_masks(&lanes)
}

/// PUBLIC API: the 6 q12289 projection masks. Each fits in a `u16` (`< 12289`).
///
/// BYTE-IDENTICAL to the account-creation proof's mask derivation — see
/// `../descender-account-stark/tests/mask_prf_equivalence.rs`.
pub fn projection_masks(pad_seed: &[u8; 32], slot_id: u64) -> [u16; NUM_OUTPUTS] {
    let m = projection_masks_u32(pad_seed, slot_id);
    let mut out = [0u16; NUM_OUTPUTS];
    for j in 0..NUM_OUTPUTS {
        debug_assert!(m[j] < Q12289);
        out[j] = m[j] as u16;
    }
    out
}
