//! Settlement verifier — the deployable, economically-viable entry point for the O(1) block proof.
//!
//! ## Architecture decision: native settlement / sovereign-rollup verification
//!
//! The aggregate proof ([`crate::block_recursive_air`]) attests, in ONE O(1) proof, that every
//! member of a block was verified in-circuit by the full recursive verifier and chains to a single
//! `chain_state_root`. The remaining deployment question is WHERE that one proof is checked.
//!
//! [`crate::verifier_cost`] measured naive in-EVM verification at far above the L1 gas limit: a
//! KoalaBear/Poseidon2 STARK with 120 FRI queries costs tens of millions of gas per block. The
//! alternatives preserve quantum-128 security only conditionally:
//!   - **Poseidon2 precompile** — needs an EVM-client fork + an EIP (outside this repo's control).
//!   - **SNARK-wrap** — re-prove the STARK in a pairing SNARK; introduces a NON-post-quantum trust
//!     boundary, defeating the quantum-128 goal.
//!   - **Native settlement (CHOSEN)** — Descender settles as a sovereign / L2 rollup whose full
//!     nodes run THIS native verifier. Verification is O(1) per block at native speed (no EVM gas),
//!     so it is economically viable AND post-quantum end-to-end (Poseidon2/FRI throughout).
//!
//! Native settlement is the only option that is (a) buildable inside this repository with no external
//! dependency and (b) post-quantum end-to-end. This module is that verifier: the single artifact a
//! settlement-layer full node, sequencer, or DA verifier links against to accept or reject a block.
//!
//! All FRI knobs come from the leaf STARK's `PROD_*` constants — the verifier shares the leaf's
//! single source of truth, so a re-parameterization cannot silently desync the deployed verifier.

use crate::block_recursive_air::{build_trace, n_cols, trace_root, BlockRecursiveAir};
use crate::recursive_air::RecursiveConfig;
use descender_account_stark::{
    Challenge, MyCompress, MyHash, Perm, Val, DIGEST_ELEMS, PROD_LOG_BLOWUP, PROD_NUM_QUERIES,
    PROD_QUERY_POW_BITS,
};
use p3_challenger::DuplexChallenger;
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::Field;
use p3_fri::{FriParameters, TwoAdicFriPcs};
use p3_koala_bear::default_koalabear_poseidon2_16;
use p3_matrix::dense::RowMajorMatrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_uni_stark::{prove, verify, Proof, StarkConfig, VerificationError};

extern crate alloc;
use alloc::vec::Vec;

// ---- The deployed aggregate-STARK configuration (mirrors the leaf STARK's PROD knobs) -----------

type ValMmcs =
    MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 2, DIGEST_ELEMS>;
type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
type DftT = Radix2DitParallel<Val>;
type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
type SettlementChallenger = DuplexChallenger<Val, Perm, 16, 8>;

/// The deployed aggregate-STARK config: KoalaBear / Poseidon2-16, FRI at the leaf's production q128
/// knobs (`PROD_LOG_BLOWUP`, `PROD_NUM_QUERIES`, `PROD_QUERY_POW_BITS`).
pub type SettlementConfig = StarkConfig<Pcs, Challenge, SettlementChallenger>;

/// A serializable aggregate block proof — the single on-chain/on-DA artifact per block.
pub type BlockProof = Proof<SettlementConfig>;

/// Build the production settlement STARK config. Deterministic: same constants every node.
///
/// This is the verifier configuration a settlement-layer full node instantiates once at startup and
/// reuses for every block. The FRI parameters are read from the leaf STARK's `PROD_*` constants so
/// the aggregate verifier and the leaf prover can never desync.
pub fn prod_config() -> SettlementConfig {
    let perm: Perm = default_koalabear_poseidon2_16();
    let val_mmcs = ValMmcs::new(
        MyHash::new(perm.clone()),
        MyCompress::new(MyHash::new(perm.clone())),
        0,
    );
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
    let fri_params = FriParameters {
        log_blowup: PROD_LOG_BLOWUP,
        log_final_poly_len: 0,
        max_log_arity: 1,
        num_queries: PROD_NUM_QUERIES,
        commit_proof_of_work_bits: 0,
        query_proof_of_work_bits: PROD_QUERY_POW_BITS,
        mmcs: challenge_mmcs,
    };
    let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
    SettlementConfig::new(pcs, SettlementChallenger::new(perm))
}

// ---- Prove / verify a block ---------------------------------------------------------------------

/// Prove a block: aggregate `member_windows` (each a packed `recursive_air` window verifying one
/// member in-circuit) into ONE O(1) proof, returning `(proof, chain_state_root)`. The root is the
/// proof's only public value — the deployed claim the settlement layer accepts.
///
/// Sequencer/prover side. The returned root is what gets posted; the proof is what gets verified.
pub fn prove_block(
    config: &SettlementConfig,
    cfg: &RecursiveConfig,
    member_windows: &[Vec<Val>],
) -> (BlockProof, [Val; DIGEST_ELEMS]) {
    let n = member_windows.len();
    let flat = build_trace(cfg, member_windows);
    let root = trace_root(cfg, n, &flat);
    let trace = RowMajorMatrix::new(flat, n_cols(cfg));
    let proof = prove(
        config,
        &BlockRecursiveAir { cfg: cfg.clone() },
        trace,
        &root.to_vec(),
    );
    (proof, root)
}

/// Verify a block proof against a CLAIMED `chain_state_root`. This is the deployable entry point: a
/// settlement-layer full node calls exactly this to accept or reject a block. `Ok(())` means every
/// member was verified in-circuit by the full recursive verifier AND they chain to
/// `claimed_root` — at production quantum-128 FRI knobs, in O(1) verifier work per block.
///
/// The verifier re-derives nothing about the block beyond what the proof+root commit to; passing a
/// root that does not match the proven trace fails verification.
pub fn verify_block(
    config: &SettlementConfig,
    cfg: &RecursiveConfig,
    proof: &BlockProof,
    claimed_root: &[Val; DIGEST_ELEMS],
) -> Result<(), VerificationError<p3_uni_stark::PcsError<SettlementConfig>>> {
    verify(
        config,
        &BlockRecursiveAir { cfg: cfg.clone() },
        proof,
        &claimed_root.to_vec(),
    )
}

// ---- Transport ----------------------------------------------------------------------------------

/// Serialize a block proof for transport to the settlement layer / DA.
pub fn serialize_block_proof(proof: &BlockProof) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(proof)
}

/// Deserialize a block proof received by a settlement-layer verifier.
pub fn deserialize_block_proof(bytes: &[u8]) -> Result<BlockProof, bincode::Error> {
    bincode::deserialize(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_recursive_air::windows_for_settlement;
    use p3_field::PrimeCharacteristicRing;

    /// END-TO-END deployable path at production quantum-128 knobs: a sequencer proves a block, the
    /// proof is serialized for transport, a settlement-layer node deserializes it and ACCEPTS it
    /// against the claimed `chain_state_root` — and REJECTS a tampered root. This is the economic
    /// deployment artifact (native O(1) settlement verification) exercised end to end.
    #[test]
    fn settlement_roundtrip_accepts_and_rejects_q128() {
        let config = prod_config();
        let levels = 3usize;
        let cfg = crate::block_recursive_air::cfg_merkle_settlement(levels);
        let n = 4usize;
        let ws = windows_for_settlement(&cfg, levels, n);

        // Sequencer: prove + serialize for transport.
        let (proof, root) = prove_block(&config, &cfg, &ws);
        let bytes = serialize_block_proof(&proof).expect("serialize");

        // Settlement node: deserialize + verify against the claimed root.
        let received = deserialize_block_proof(&bytes).expect("deserialize");
        verify_block(&config, &cfg, &received, &root)
            .expect("settlement verifier must ACCEPT an honest block at production q128 knobs");

        // A tampered root must be rejected.
        let mut bad = root;
        bad[0] += Val::ONE;
        assert!(
            verify_block(&config, &cfg, &received, &bad).is_err(),
            "settlement verifier must REJECT a mismatched chain_state_root"
        );
    }
}
