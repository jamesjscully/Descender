//! On-chain verification cost basis — MEASURE the dominant cost of verifying the aggregate O(1)
//! block proof: the number of Poseidon2 permutations the verifier evaluates.
//!
//! A native STARK verifier's on-chain (EVM) gas is dominated by hash work: the FRI/Merkle openings
//! and the Fiat-Shamir challenger are all Poseidon2 permutations. There is no native KoalaBear
//! Poseidon2 EVM precompile, so each permutation is either a precompile (if added) or in-Solidity
//! field arithmetic. To turn "economically viable" into a budgetable number we MEASURE the
//! verifier's permutation count by wrapping the permutation in a counter and counting invocations
//! during `verify` (prove-phase calls excluded). EVM gas is then `perm_count × gas_per_perm`, where
//! `gas_per_perm` is the (separately-determined) EVM cost of one width-16 Poseidon2 permutation.

#[cfg(test)]
mod tests {
    use crate::block_accumulator::{build_trace, trace_root, BlockAccumulatorAir, BC_N_COLS};
    use core::sync::atomic::{AtomicU64, Ordering};
    use descender_account_stark::{Challenge, Perm, Val, DIGEST_ELEMS, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS};
    use p3_challenger::DuplexChallenger;
    use p3_commit::ExtensionMmcs;
    use p3_dft::Radix2DitParallel;
    use p3_field::Field;
    use p3_fri::{FriParameters, TwoAdicFriPcs};
    use p3_koala_bear::default_koalabear_poseidon2_16;
    use p3_matrix::dense::RowMajorMatrix;
    use p3_merkle_tree::MerkleTreeMmcs;
    use p3_symmetric::{CryptographicPermutation, PaddingFreeSponge, Permutation};
    use p3_uni_stark::{prove, verify, StarkConfig};

    static PERM_CALLS: AtomicU64 = AtomicU64::new(0);

    /// Counting wrapper over the pinned Poseidon2-16: every permutation (base or packed) is counted,
    /// then delegated to the real permutation, so prove/verify behaviour is byte-identical.
    #[derive(Clone, Debug)]
    struct CountingPerm {
        inner: Perm,
    }
    // Blanket over any array type the inner permutation supports (covers both the base `[Val;16]`
    // challenger perm and the packed `[Packing;16]` MMCS perm, which may or may not coincide).
    impl<A> Permutation<A> for CountingPerm
    where
        Perm: Permutation<A>,
        A: Clone,
    {
        fn permute_mut(&self, input: &mut A) {
            PERM_CALLS.fetch_add(1, Ordering::Relaxed);
            self.inner.permute_mut(input);
        }
    }
    impl<A> CryptographicPermutation<A> for CountingPerm
    where
        Perm: CryptographicPermutation<A>,
        A: Clone,
    {
    }

    type CHash = PaddingFreeSponge<CountingPerm, 16, 7, DIGEST_ELEMS>;
    type CCompress = descender_account_stark::sponge_compression::SpongeCompression<CHash>;
    type CValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, CHash, CCompress, 2, DIGEST_ELEMS>;
    type CChallengeMmcs = ExtensionMmcs<Val, Challenge, CValMmcs>;
    type DftT = Radix2DitParallel<Val>;
    type CPcs = TwoAdicFriPcs<Val, DftT, CValMmcs, CChallengeMmcs>;
    type CChallenger = DuplexChallenger<Val, CountingPerm, 16, 8>;
    type CConfig = StarkConfig<CPcs, Challenge, CChallenger>;

    fn member(seed: u32) -> [Val; DIGEST_ELEMS] {
        use p3_field::PrimeCharacteristicRing;
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1)))
    }

    /// MEASURE the verifier's Poseidon2 permutation count for the aggregate O(1) block proof at
    /// production q128 FRI knobs, across block sizes — the per-block on-chain verification work
    /// (amortized over the block's members) and the basis for an EVM gas estimate.
    #[test]
    fn measure_aggregate_verify_perm_count() {
        let cp = CountingPerm { inner: default_koalabear_poseidon2_16() };
        let mk = || -> CConfig {
            let val_mmcs = CValMmcs::new(CHash::new(cp.clone()), CCompress::new(CHash::new(cp.clone())), 0);
            let challenge_mmcs = CChallengeMmcs::new(val_mmcs.clone());
            let fri_params = FriParameters {
                log_blowup: PROD_LOG_BLOWUP,
                log_final_poly_len: 0,
                max_log_arity: 1,
                num_queries: PROD_NUM_QUERIES,
                commit_proof_of_work_bits: 0,
                query_proof_of_work_bits: PROD_QUERY_POW_BITS,
                mmcs: challenge_mmcs,
            };
            StarkConfig::new(CPcs::new(DftT::default(), val_mmcs, fri_params), CChallenger::new(cp.clone()))
        };

        for &n in &[4usize, 16, 64] {
            let members: Vec<[Val; DIGEST_ELEMS]> = (0..n as u32).map(member).collect();
            let root = trace_root(n, &build_trace(&members));
            let trace = RowMajorMatrix::new(build_trace(&members), BC_N_COLS);
            let config = mk();
            let proof = prove(&config, &BlockAccumulatorAir, trace, &root.to_vec());

            // Count ONLY the verify-phase permutations.
            PERM_CALLS.store(0, Ordering::Relaxed);
            verify(&config, &BlockAccumulatorAir, &proof, &root.to_vec()).expect("verify");
            let perms = PERM_CALLS.load(Ordering::Relaxed);
            let per_member = perms / n as u64;
            // Illustrative EVM gas band per width-16 Poseidon2 perm (precompile-ish ~ a few k gas
            // .. in-Solidity ~ tens of k gas). Reported as a band, NOT a pinned figure.
            eprintln!(
                "VERIFY-COST block n={n}: {perms} Poseidon2 perms (per-member {per_member}); \
                 EVM gas band ~{}M..{}M (@3k..30k gas/perm)",
                perms * 3 / 1000,
                perms * 30 / 1000,
            );
            assert!(perms > 0, "verify must invoke the counted permutation");
            // The per-block verification work is sub-linear in block size ⇒ per-member work falls.
            assert!(per_member < perms, "per-member work must be a fraction of the block work");
        }
    }
}
