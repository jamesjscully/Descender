//! Capacity-9 Fiat-Shamir challenger reference for the ADR-0011 migration.
//!
//! The deployed account proof uses `DuplexChallenger<Val, Perm, 16, 7>`.
//! This module pins those semantics for the recursive AIR. In particular, a
//! degree-8 extension challenge consumes all seven outputs of one rate block,
//! then performs a squeeze-only permutation for the eighth limb.

use descender_account_stark::{Challenge, Perm, Val, WIDTH};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing, PrimeField32};
use p3_koala_bear::default_koalabear_poseidon2_16;
use p3_symmetric::Permutation;

extern crate alloc;
use alloc::vec::Vec;

/// ADR-0011 Fiat-Shamir rate.
pub const C9_RATE: usize = 7;
/// Hidden capacity elements in the width-16 state.
pub const C9_CAPACITY: usize = WIDTH - C9_RATE;

/// Faithful reference for Plonky3's rate-7 duplex challenger.
#[derive(Clone)]
pub struct C9Challenger {
    permutation: Perm,
    state: [Val; WIDTH],
    input_buffer: Vec<Val>,
    output_buffer: Vec<Val>,
    permutation_count: usize,
}

impl Default for C9Challenger {
    fn default() -> Self {
        Self::new()
    }
}

impl C9Challenger {
    pub fn new() -> Self {
        Self::from_state([Val::ZERO; WIDTH])
    }

    /// Continue from a carried sponge state after prior outputs were discarded by observe.
    pub fn from_state(state: [Val; WIDTH]) -> Self {
        Self {
            permutation: default_koalabear_poseidon2_16(),
            state,
            input_buffer: Vec::with_capacity(C9_RATE),
            output_buffer: Vec::with_capacity(C9_RATE),
            permutation_count: 0,
        }
    }

    /// Continue after a full-rate absorb already populated the output buffer.
    pub fn from_ready_output_state(state: [Val; WIDTH]) -> Self {
        let mut challenger = Self::from_state(state);
        challenger
            .output_buffer
            .extend_from_slice(&state[..C9_RATE]);
        challenger
    }

    fn duplexing(&mut self) {
        let absorbed = self.input_buffer.len();
        assert!(absorbed <= C9_RATE);

        for (lane, value) in self.input_buffer.drain(..).enumerate() {
            self.state[lane] = value;
        }
        if absorbed > 0 {
            self.state[absorbed..C9_RATE].fill(Val::ZERO);
            self.state[C9_RATE] += Val::from_u8(absorbed as u8);
        }

        self.permutation.permute_mut(&mut self.state);
        self.permutation_count += 1;
        self.output_buffer.clear();
        self.output_buffer.extend_from_slice(&self.state[..C9_RATE]);
    }

    pub fn observe(&mut self, value: Val) {
        self.output_buffer.clear();
        self.input_buffer.push(value);
        if self.input_buffer.len() == C9_RATE {
            self.duplexing();
        }
    }

    pub fn observe_slice(&mut self, values: &[Val]) {
        for &value in values {
            self.observe(value);
        }
    }

    pub fn sample_base(&mut self) -> Val {
        if !self.input_buffer.is_empty() || self.output_buffer.is_empty() {
            self.duplexing();
        }
        self.output_buffer
            .pop()
            .expect("rate-7 duplexing must refill the output buffer")
    }

    pub fn sample_extension(&mut self) -> Challenge {
        Challenge::from_basis_coefficients_fn(|_| self.sample_base())
    }

    pub fn sample_bits(&mut self, bits: usize) -> usize {
        assert!(bits < usize::BITS as usize);
        assert!((1usize << bits) < Val::ORDER_U32 as usize);
        self.sample_base().as_canonical_u32() as usize & ((1usize << bits) - 1)
    }

    pub fn state(&self) -> [Val; WIDTH] {
        self.state
    }

    pub fn input_len(&self) -> usize {
        self.input_buffer.len()
    }

    pub fn output_len(&self) -> usize {
        self.output_buffer.len()
    }

    pub fn permutation_count(&self) -> usize {
        self.permutation_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_challenger::{CanObserve, CanSample, CanSampleBits, DuplexChallenger};

    type NativeC9 = DuplexChallenger<Val, Perm, WIDTH, C9_RATE>;

    fn values(seed: u32, count: usize) -> Vec<Val> {
        (0..count)
            .map(|i| {
                Val::from_u32(
                    seed.wrapping_mul(0x9e37_79b9)
                        .wrapping_add((i as u32).wrapping_mul(17))
                        .wrapping_add(1),
                )
            })
            .collect()
    }

    fn observe_both(mirror: &mut C9Challenger, native: &mut NativeC9, stream: &[Val]) {
        mirror.observe_slice(stream);
        for &value in stream {
            native.observe(value);
        }
    }

    #[test]
    fn capacity9_shape_is_pinned() {
        assert_eq!(WIDTH, 16);
        assert_eq!(C9_RATE, 7);
        assert_eq!(C9_CAPACITY, 9);
        assert_eq!(<Challenge as BasedVectorSpace<Val>>::DIMENSION, 8);
    }

    #[test]
    fn extension_sample_spans_two_rate7_output_blocks() {
        let mut challenger = C9Challenger::new();
        challenger.observe_slice(&values(11, 13));

        // One full seven-element absorb has already permuted; six inputs remain.
        assert_eq!(challenger.permutation_count(), 1);
        assert_eq!(challenger.input_len(), 6);
        assert_eq!(challenger.output_len(), 0);

        let _ = challenger.sample_extension();

        // Partial absorb, then a squeeze-only permutation for extension limb 8.
        assert_eq!(challenger.permutation_count(), 3);
        assert_eq!(challenger.input_len(), 0);
        assert_eq!(challenger.output_len(), 6);
    }

    #[test]
    fn mixed_plonky3_transcript_matches_native_rate7_challenger() {
        let mut mirror = C9Challenger::new();
        let mut native = NativeC9::new(default_koalabear_poseidon2_16());

        // Degree tags, trace commitment, and public values before alpha.
        observe_both(&mut mirror, &mut native, &values(1, 3 + 13 + 37));
        let alpha_m = mirror.sample_extension();
        let alpha_n: Challenge = native.sample();
        assert_eq!(alpha_m, alpha_n);

        // Quotient commitment before zeta.
        observe_both(&mut mirror, &mut native, &values(2, 13));
        let zeta_m = mirror.sample_extension();
        let zeta_n: Challenge = native.sample();
        assert_eq!(zeta_m, zeta_n);

        // Opened values before the FRI batch-combination challenge.
        observe_both(&mut mirror, &mut native, &values(3, 91));
        let batch_m = mirror.sample_extension();
        let batch_n: Challenge = native.sample();
        assert_eq!(batch_m, batch_n);

        // FRI commit-phase roots and folding challenges.
        for round in 0..5u32 {
            observe_both(&mut mirror, &mut native, &values(20 + round, 13));
            let beta_m = mirror.sample_extension();
            let beta_n: Challenge = native.sample();
            assert_eq!(beta_m, beta_n, "beta round {round}");
        }

        // Final polynomial and query-PoW witness, followed by query indices.
        observe_both(&mut mirror, &mut native, &values(40, 11));
        let witness = Val::from_u32(0x1234_5678);
        mirror.observe(witness);
        native.observe(witness);
        assert_eq!(mirror.sample_bits(16), native.sample_bits(16));
        for query in 0..120 {
            assert_eq!(
                mirror.sample_bits(6),
                native.sample_bits(6),
                "query index {query}"
            );
        }

        assert_eq!(mirror.state(), native.sponge_state);
        assert_eq!(mirror.input_len(), native.input_buffer.len());
        assert_eq!(mirror.output_len(), native.output_buffer.len());
    }
}
