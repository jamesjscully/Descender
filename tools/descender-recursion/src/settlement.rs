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

use crate::block_ood_binding_air::{
    build_trace as build_ood_trace, terminal_state, BlockOodBindingAir, MemberOodInput,
};
use crate::block_recursive_air::{build_trace, n_cols, trace_root, BlockRecursiveAir};
use crate::fanout_anchor_air::{
    verify_fan_out_anchor_v1, FanOutAnchorAirV1, FanOutAnchorProofV1, FanOutAnchorVerificationError,
};
use crate::leaf_hash::leaf_hash_digest;
use crate::pcs_query_settlement::{
    verify_all_pcs_queries_against_anchor_root_v1, AllPcsQueriesSettlementProofV1,
    PcsQuerySettlementProfileV1, PcsQuerySettlementVerificationError,
};
use crate::recursive_air::{
    n_cols as window_n_cols, window_external_ood_end, window_external_ood_start,
    window_merkle_root, RecursiveConfig,
};
use descender_account_stark::{
    Challenge, MyCompress, MyHash, Perm, Val, DIGEST_ELEMS, PROD_LOG_BLOWUP, PROD_NUM_QUERIES,
    PROD_QUERY_POW_BITS,
};
use p3_challenger::DuplexChallenger;
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::{Field, PrimeCharacteristicRing};
use p3_fri::{FriParameters, TwoAdicFriPcs};
use p3_koala_bear::default_koalabear_poseidon2_16;
use p3_matrix::dense::RowMajorMatrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_uni_stark::{prove, verify, Proof, StarkConfig, VerificationError};
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::vec::Vec;

// ---- The deployed aggregate-STARK configuration (mirrors the leaf STARK's PROD knobs) -----------

type ValMmcs = MerkleTreeMmcs<
    <Val as Field>::Packing,
    <Val as Field>::Packing,
    MyHash,
    MyCompress,
    2,
    DIGEST_ELEMS,
>;
type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
type DftT = Radix2DitParallel<Val>;
type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
type SettlementChallenger = DuplexChallenger<Val, Perm, 16, 7>;

/// The deployed aggregate-STARK config: KoalaBear / Poseidon2-16, FRI at the leaf's production q128
/// knobs (`PROD_LOG_BLOWUP`, `PROD_NUM_QUERIES`, `PROD_QUERY_POW_BITS`).
pub type SettlementConfig = StarkConfig<Pcs, Challenge, SettlementChallenger>;

/// A serializable aggregate block proof — the single on-chain/on-DA artifact per block.
pub type BlockProof = Proof<SettlementConfig>;

/// The vertical OOD companion uses the same settlement PCS/config but a distinct AIR.
pub type OodBindingProof = Proof<SettlementConfig>;

/// Consensus-approved shape for the vertical companion. The verifier receives its expected profile
/// out of band and rejects a proof envelope that tries to select a smaller member count or stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OodBindingProfileV1 {
    pub members: u32,
    pub stream_len: u32,
}

impl OodBindingProfileV1 {
    pub fn new(members: usize, stream_len: usize) -> Result<Self, BoundBlockInputError> {
        if members == 0 {
            return Err(BoundBlockInputError::NoMembers);
        }
        if stream_len == 0 {
            return Err(BoundBlockInputError::EmptyStream);
        }
        Ok(Self {
            members: u32::try_from(members).map_err(|_| BoundBlockInputError::ShapeOverflow)?,
            stream_len: u32::try_from(stream_len)
                .map_err(|_| BoundBlockInputError::ShapeOverflow)?,
        })
    }

    fn shape(self) -> (usize, usize) {
        (self.members as usize, self.stream_len as usize)
    }
}

/// Versioned transport object for the production external-OOD settlement composition. Verification
/// accepts it only when both component proofs attest the same caller-supplied chain root.
#[derive(Serialize, Deserialize)]
pub struct BoundBlockProofV1 {
    pub profile: OodBindingProfileV1,
    pub block_proof: BlockProof,
    pub ood_binding_proof: OodBindingProof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundBlockInputError {
    ExternalOodRequired,
    NoMembers,
    MemberCountNotPowerOfTwo,
    EmptyStream,
    ShapeOverflow,
    MemberCountMismatch,
    WindowWidthMismatch { member: usize },
    StreamLengthMismatch { member: usize },
    TranscriptPcsMismatch { member: usize },
    MemberRootMismatch { member: usize },
    StartStateMismatch { member: usize },
    EndStateMismatch { member: usize },
    AggregateRootMismatch,
}

pub type SettlementVerificationError = VerificationError<p3_uni_stark::PcsError<SettlementConfig>>;

#[derive(Debug)]
pub enum BoundBlockVerificationError {
    ExternalOodRequired,
    ProfileMismatch {
        expected: OodBindingProfileV1,
        received: OodBindingProfileV1,
    },
    Block(SettlementVerificationError),
    OodBinding(SettlementVerificationError),
}

pub const TRANSCRIPT_ANCHORED_BLOCK_ROOT_DOMAIN_V1: u32 = 0x4453_5201;

/// Consensus root for the composed block claim.  It commits to the existing
/// member/state chain and to the recursive fan-out's ordered all-query
/// transcript anchor under an explicit versioned domain.
pub fn transcript_anchored_block_root_v1(
    chain_state_root: &[Val; DIGEST_ELEMS],
    transcript_anchor_root: &[Val; DIGEST_ELEMS],
) -> [Val; DIGEST_ELEMS] {
    let mut preimage = Vec::with_capacity(2 + 2 * DIGEST_ELEMS);
    preimage.push(Val::from_u32(TRANSCRIPT_ANCHORED_BLOCK_ROOT_DOMAIN_V1));
    preimage.push(Val::ONE);
    preimage.extend_from_slice(chain_state_root);
    preimage.extend_from_slice(transcript_anchor_root);
    leaf_hash_digest(&preimage)
}

#[derive(Debug)]
pub enum TranscriptAnchoredBlockVerificationError {
    SettlementRootMismatch,
    Block(SettlementVerificationError),
    BoundBlock(BoundBlockVerificationError),
    FanOutAnchor(FanOutAnchorVerificationError),
    AllQueries(PcsQuerySettlementVerificationError),
}

/// Complete diagnostic composition: verify the block root, the recursive
/// fan-out export, and every query's five-component settlement envelope against
/// the same transcript anchor.  This closes statement substitution at the API
/// boundary; the current all-query envelope is deliberately still the
/// uneconomical 600-proof diagnostic form.
pub fn verify_block_with_all_queries_v1(
    config: &SettlementConfig,
    block_cfg: &RecursiveConfig,
    block_proof: &BlockProof,
    chain_state_root: &[Val; DIGEST_ELEMS],
    fan_out_air: &FanOutAnchorAirV1,
    fan_out_proof: &FanOutAnchorProofV1,
    transcript_anchor_root: &[Val; DIGEST_ELEMS],
    expected_query_profile: &PcsQuerySettlementProfileV1,
    all_queries_proof: &AllPcsQueriesSettlementProofV1,
    claimed_settlement_root: &[Val; DIGEST_ELEMS],
) -> Result<(), TranscriptAnchoredBlockVerificationError> {
    verify_block_with_transcript_anchor_v1(
        config,
        block_cfg,
        block_proof,
        chain_state_root,
        fan_out_air,
        fan_out_proof,
        transcript_anchor_root,
        claimed_settlement_root,
    )?;
    verify_all_pcs_queries_against_anchor_root_v1(
        config,
        expected_query_profile,
        all_queries_proof,
        transcript_anchor_root,
    )
    .map_err(TranscriptAnchoredBlockVerificationError::AllQueries)
}

/// External-OOD companion of [`verify_block_with_all_queries_v1`].
pub fn verify_bound_block_with_all_queries_v1(
    config: &SettlementConfig,
    block_cfg: &RecursiveConfig,
    expected_ood_profile: OodBindingProfileV1,
    bound_proof: &BoundBlockProofV1,
    chain_state_root: &[Val; DIGEST_ELEMS],
    fan_out_air: &FanOutAnchorAirV1,
    fan_out_proof: &FanOutAnchorProofV1,
    transcript_anchor_root: &[Val; DIGEST_ELEMS],
    expected_query_profile: &PcsQuerySettlementProfileV1,
    all_queries_proof: &AllPcsQueriesSettlementProofV1,
    claimed_settlement_root: &[Val; DIGEST_ELEMS],
) -> Result<(), TranscriptAnchoredBlockVerificationError> {
    verify_bound_block_with_transcript_anchor_v1(
        config,
        block_cfg,
        expected_ood_profile,
        bound_proof,
        chain_state_root,
        fan_out_air,
        fan_out_proof,
        transcript_anchor_root,
        claimed_settlement_root,
    )?;
    verify_all_pcs_queries_against_anchor_root_v1(
        config,
        expected_query_profile,
        all_queries_proof,
        transcript_anchor_root,
    )
    .map_err(TranscriptAnchoredBlockVerificationError::AllQueries)
}

/// Verify the existing O(1) block proof and the exact-query fan-out proof, then
/// require the caller's consensus root to commit to both public roots.  This is
/// the composition boundary used while the two AIRs remain separate proofs.
pub fn verify_block_with_transcript_anchor_v1(
    config: &SettlementConfig,
    block_cfg: &RecursiveConfig,
    block_proof: &BlockProof,
    chain_state_root: &[Val; DIGEST_ELEMS],
    fan_out_air: &FanOutAnchorAirV1,
    fan_out_proof: &FanOutAnchorProofV1,
    transcript_anchor_root: &[Val; DIGEST_ELEMS],
    claimed_settlement_root: &[Val; DIGEST_ELEMS],
) -> Result<(), TranscriptAnchoredBlockVerificationError> {
    if transcript_anchored_block_root_v1(chain_state_root, transcript_anchor_root)
        != *claimed_settlement_root
    {
        return Err(TranscriptAnchoredBlockVerificationError::SettlementRootMismatch);
    }
    verify_block(config, block_cfg, block_proof, chain_state_root)
        .map_err(TranscriptAnchoredBlockVerificationError::Block)?;
    verify_fan_out_anchor_v1(config, fan_out_air, fan_out_proof, transcript_anchor_root)
        .map_err(TranscriptAnchoredBlockVerificationError::FanOutAnchor)
}

/// External-OOD production companion of
/// [`verify_block_with_transcript_anchor_v1`].  Both the block-recursive and
/// vertical-OOD proofs first share `chain_state_root`; the fan-out proof shares
/// the canonical transcript anchor; the versioned settlement root commits to
/// the pair.
pub fn verify_bound_block_with_transcript_anchor_v1(
    config: &SettlementConfig,
    block_cfg: &RecursiveConfig,
    expected_profile: OodBindingProfileV1,
    bound_proof: &BoundBlockProofV1,
    chain_state_root: &[Val; DIGEST_ELEMS],
    fan_out_air: &FanOutAnchorAirV1,
    fan_out_proof: &FanOutAnchorProofV1,
    transcript_anchor_root: &[Val; DIGEST_ELEMS],
    claimed_settlement_root: &[Val; DIGEST_ELEMS],
) -> Result<(), TranscriptAnchoredBlockVerificationError> {
    if transcript_anchored_block_root_v1(chain_state_root, transcript_anchor_root)
        != *claimed_settlement_root
    {
        return Err(TranscriptAnchoredBlockVerificationError::SettlementRootMismatch);
    }
    verify_bound_block_v1(
        config,
        block_cfg,
        expected_profile,
        bound_proof,
        chain_state_root,
    )
    .map_err(TranscriptAnchoredBlockVerificationError::BoundBlock)?;
    verify_fan_out_anchor_v1(config, fan_out_air, fan_out_proof, transcript_anchor_root)
        .map_err(TranscriptAnchoredBlockVerificationError::FanOutAnchor)
}

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

// ---- Production external-OOD composition ------------------------------------------------------

/// Prove the constant-count production composition: one block-recursive proof plus one vertical
/// OOD-binding proof, sharing the exact same public chain root. Host-side checks fail early when a
/// member window and its independently materialized transcript do not describe the same member.
pub fn prove_bound_block_v1(
    config: &SettlementConfig,
    cfg: &RecursiveConfig,
    member_windows: &[Vec<Val>],
    ood_inputs: &[MemberOodInput<'_>],
) -> Result<(BoundBlockProofV1, [Val; DIGEST_ELEMS]), BoundBlockInputError> {
    if !cfg.fs_ood_external {
        return Err(BoundBlockInputError::ExternalOodRequired);
    }
    if member_windows.is_empty() {
        return Err(BoundBlockInputError::NoMembers);
    }
    if !member_windows.len().is_power_of_two() {
        return Err(BoundBlockInputError::MemberCountNotPowerOfTwo);
    }
    if member_windows.len() != ood_inputs.len() {
        return Err(BoundBlockInputError::MemberCountMismatch);
    }
    let stream_len = ood_inputs[0].transcript_stream.len();
    let profile = OodBindingProfileV1::new(ood_inputs.len(), stream_len)?;
    let expected_width = window_n_cols(cfg);
    for (member, (window, input)) in member_windows.iter().zip(ood_inputs).enumerate() {
        if window.len() != expected_width {
            return Err(BoundBlockInputError::WindowWidthMismatch { member });
        }
        if input.transcript_stream.len() != stream_len || input.pcs_stream.len() != stream_len {
            return Err(BoundBlockInputError::StreamLengthMismatch { member });
        }
        if input.transcript_stream != input.pcs_stream {
            return Err(BoundBlockInputError::TranscriptPcsMismatch { member });
        }
        if input.member_root != window_merkle_root(cfg, window) {
            return Err(BoundBlockInputError::MemberRootMismatch { member });
        }
        if input.start != window_external_ood_start(cfg, window) {
            return Err(BoundBlockInputError::StartStateMismatch { member });
        }
        let end = terminal_state(input.start, input.transcript_stream);
        if end != window_external_ood_end(cfg, window) {
            return Err(BoundBlockInputError::EndStateMismatch { member });
        }
    }

    let (block_proof, block_root) = prove_block(config, cfg, member_windows);
    let (ood_trace, ood_root) = build_ood_trace(ood_inputs);
    if block_root != ood_root {
        return Err(BoundBlockInputError::AggregateRootMismatch);
    }
    let ood_air = BlockOodBindingAir::new(ood_inputs.len(), stream_len);
    let ood_binding_proof = prove(config, &ood_air, ood_trace, &ood_root);
    Ok((
        BoundBlockProofV1 {
            profile,
            block_proof,
            ood_binding_proof,
        },
        block_root,
    ))
}

/// Verify both production proofs against one caller-supplied root and one consensus-approved shape.
pub fn verify_bound_block_v1(
    config: &SettlementConfig,
    cfg: &RecursiveConfig,
    expected_profile: OodBindingProfileV1,
    proof: &BoundBlockProofV1,
    claimed_root: &[Val; DIGEST_ELEMS],
) -> Result<(), BoundBlockVerificationError> {
    if !cfg.fs_ood_external {
        return Err(BoundBlockVerificationError::ExternalOodRequired);
    }
    if proof.profile != expected_profile {
        return Err(BoundBlockVerificationError::ProfileMismatch {
            expected: expected_profile,
            received: proof.profile,
        });
    }
    verify_block(config, cfg, &proof.block_proof, claimed_root)
        .map_err(BoundBlockVerificationError::Block)?;
    let (members, stream_len) = expected_profile.shape();
    verify(
        config,
        &BlockOodBindingAir::new(members, stream_len),
        &proof.ood_binding_proof,
        &claimed_root.to_vec(),
    )
    .map_err(BoundBlockVerificationError::OodBinding)
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

/// Serialize the paired production settlement proof envelope.
pub fn serialize_bound_block_proof_v1(
    proof: &BoundBlockProofV1,
) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(proof)
}

/// Deserialize the paired production settlement proof envelope. Callers must still compare its
/// embedded profile to their consensus-approved profile via [`verify_bound_block_v1`].
pub fn deserialize_bound_block_proof_v1(bytes: &[u8]) -> Result<BoundBlockProofV1, bincode::Error> {
    bincode::deserialize(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_ood_binding_air::terminal_state;
    use crate::block_recursive_air::{windows_for_external_ood_settlement, windows_for_settlement};
    use crate::challenger::RATE;
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

    #[test]
    fn bound_settlement_roundtrip_rejects_shape_endpoint_and_component_mixups_q128() {
        let config = prod_config();
        let levels = 2usize;
        let cfg = RecursiveConfig {
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: 1,
            fs_ood_external: true,
            ..crate::block_recursive_air::cfg_merkle_settlement(levels)
        };
        let n = 2usize;
        let len = RATE + 3;
        let streams: Vec<Vec<Val>> = (0..n)
            .map(|member| {
                (0..len)
                    .map(|i| Val::from_usize(50_000 + member * 1000 + i * 19))
                    .collect()
            })
            .collect();

        // The minimal reference member has a deterministic post-zeta state. Pack once with
        // placeholder endpoints to read it, then pack the real terminal states exported by the
        // vertical transcripts.
        let placeholders = vec![[Val::ZERO; descender_account_stark::WIDTH]; n];
        let preliminary = windows_for_external_ood_settlement(&cfg, levels, &placeholders);
        let starts: Vec<_> = preliminary
            .iter()
            .map(|w| window_external_ood_start(&cfg, w))
            .collect();
        let ends: Vec<_> = (0..n)
            .map(|i| terminal_state(starts[i], &streams[i]))
            .collect();
        let windows = windows_for_external_ood_settlement(&cfg, levels, &ends);
        let roots: Vec<_> = windows
            .iter()
            .map(|w| window_merkle_root(&cfg, w))
            .collect();
        let inputs: Vec<_> = (0..n)
            .map(|i| MemberOodInput {
                member_root: roots[i],
                start: starts[i],
                transcript_stream: &streams[i],
                pcs_stream: &streams[i],
            })
            .collect();

        let (bundle, root) = prove_bound_block_v1(&config, &cfg, &windows, &inputs)
            .expect("honest paired settlement witness");
        let profile = OodBindingProfileV1::new(n, len).unwrap();
        assert_eq!(bundle.profile, profile);

        let bytes = serialize_bound_block_proof_v1(&bundle).expect("serialize paired proof");
        let received = deserialize_bound_block_proof_v1(&bytes).expect("deserialize paired proof");
        verify_bound_block_v1(&config, &cfg, profile, &received, &root)
            .expect("both q128 proofs must accept against their shared root");

        let mut bad_root = root;
        bad_root[0] += Val::ONE;
        assert!(verify_bound_block_v1(&config, &cfg, profile, &received, &bad_root).is_err());
        let wrong_profile = OodBindingProfileV1::new(n, len - 1).unwrap();
        assert!(matches!(
            verify_bound_block_v1(&config, &cfg, wrong_profile, &received, &root),
            Err(BoundBlockVerificationError::ProfileMismatch { .. })
        ));

        // Coherently changing the stream but retaining the member window is rejected before proof
        // generation because the externally committed terminal state no longer matches.
        let mut changed_streams = streams.clone();
        changed_streams[0][0] += Val::ONE;
        let changed_inputs: Vec<_> = (0..n)
            .map(|i| MemberOodInput {
                member_root: roots[i],
                start: starts[i],
                transcript_stream: &changed_streams[i],
                pcs_stream: &changed_streams[i],
            })
            .collect();
        assert!(matches!(
            prove_bound_block_v1(&config, &cfg, &windows, &changed_inputs),
            Err(BoundBlockInputError::EndStateMismatch { member: 0 })
        ));

        // Even if an attacker supplies a valid companion proof for another endpoint/root, it
        // cannot be mixed with the honest block proof because both verifications receive one root.
        let (alt_trace, alt_root) = build_ood_trace(&changed_inputs);
        assert_ne!(alt_root, root);
        let alt_ood = prove(
            &config,
            &BlockOodBindingAir::new(n, len),
            alt_trace,
            &alt_root,
        );
        let mixed = BoundBlockProofV1 {
            profile,
            block_proof: bundle.block_proof,
            ood_binding_proof: alt_ood,
        };
        assert!(matches!(
            verify_bound_block_v1(&config, &cfg, profile, &mixed, &root),
            Err(BoundBlockVerificationError::OodBinding(_))
        ));
    }
}
