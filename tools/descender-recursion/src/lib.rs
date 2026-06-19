//! # Descender recursion / aggregation layer (Phase 1 in progress)
//!
//! Greenlit 2026-06-18. Builds the recursive verifier described in
//! `research/path2_recursion_circuit_concrete_design.md`. Plonky3 0.6.1 has NO
//! recursion / verifier-as-AIR primitive, so the inner-FRI verifier is hand-written
//! as an AIR, reusing `p3-poseidon2-air` for the Poseidon2 hash sub-circuits.
//!
//! The leaf is Poseidon2 end-to-end (AIR, Merkle, Fiat-Shamir, mask-PRF), so
//! verifying one creation proof in-circuit is ~400–600 Poseidon2 permutations — a
//! small recursion AIR. The mask-PRF swap retired the SHA-on-recursion-path risk;
//! the only remaining in-circuit SHA is the spend-side FS frame (deferred, behind
//! the redesign guardrail) and the role-root export bridge (cheap Poseidon2 re-host,
//! Phase-2 step 0).
//!
//! ## Build phases (milestones)
//!
//! - **Phase 1 — single-proof recursion (THIS crate's first milestone).** A
//!   verifier-AIR that re-verifies ONE `descender-account-stark` proof and re-exposes
//!   its public values. Decomposes into the gadgets below. Milestone:
//!   `recursive_verify(inner_proof, inner_public)` accepts iff the inner proof
//!   verifies, with ZERO in-circuit SHA (defer bridge/frame to public-input equality).
//! - **Phase 2 — {creation + spend} co-verification.** Add the in-AIR bindings:
//!   packed_witness equality (discharges audit MED-2 at the recursion level) + role →
//!   export bridge (re-host bridge to Poseidon2 first). Decision point (behind James)
//!   on re-hosting the spend FS frame to Poseidon2.
//! - **Phase 3 — batch / IVC over a block.** Fold per-pair recursive proofs into the
//!   Poseidon2 `chain_state_root` accumulator (reuse the leaf's `chain_digest`
//!   machinery) → one O(1) proof per block. Terminate in a STARK (no Groth16 wrap) to
//!   stay post-quantum.
//!
//! ## Phase-1 gadget breakdown (the hand-written verifier-AIR)
//!
//! Each is an AIR sub-circuit; the reference algorithm is the matching native
//! `p3-fri` / `p3-uni-stark` / `p3-challenger` routine, which the AIR must mirror
//! constant-for-constant against `descender_account_stark::config`.
//!
//! 1. `challenger` — in-AIR `DuplexChallenger` (Poseidon2 duplex sponge): observe
//!    inner commitments + the leaf public-IO (~190 field elems @ rate 8), sample α
//!    (DEEP/batch), per-round β fold challenges, and query indices (`sample_bits`).
//!    Mirrors `p3-challenger::DuplexChallenger`.
//! 2. `merkle_verify` — verify a salted Merkle opening (one `TruncatedPermutation` =
//!    one Poseidon2 perm per node) for the main-trace commitment and each FRI
//!    commit-phase layer, × `PROD_NUM_QUERIES` (56). Mirrors `MerkleTreeHidingMmcs`.
//! 3. `fri_fold` — per-query commit-phase consistency + `fold_row` over the degree-4
//!    extension field, for each binary folding round. Mirrors `p3-fri::verifier`.
//! 4. `constraint_eval` — DEEP / out-of-domain evaluation of the account-creation
//!    AIR constraint polynomial at the OOD point + the DEEP linear combination.
//!    Mirrors `p3-uni-stark::verifier` + the leaf `FullRelationAir` folder.
//! 5. `public_io` — re-expose the inner proof's public values as this proof's, so the
//!    recursion is statement-preserving.
//!
//! ## Open soundness obligations (carry explicitly; see the design doc)
//!
//! - Recursive-FRI **knowledge extractor** argued from scratch (round-by-round FRI
//!   extraction under the outer AIR).
//! - In-circuit Fiat-Shamir soundness / QROM seam (faithful in-AIR sponge; inner
//!   Poseidon2-FS + spend SHA-FS coexisting; FRI QROM loss un-quantified).
//! - Conservative Johnson-bound params for the OUTER STARK (recursion does NOT inherit
//!   the inner's soundness).
//! - Challenge field is now ext degree 8 (|F| = p^8 ≈ 2^247.9; raised from deg-4 in
//!   leaf commit 4c06f6b). The inner FRI-AoK field-bound term is cleared; the residual
//!   ~2^-124 creation-soundness floor is the WIDTH-16 Poseidon2 sponge capacity + 8-elem
//!   Merkle digest (industry-standard, accepted 2026-06-19 — see field-bound decision).
//!   The recursion still owns its OWN outer-STARK soundness budget (does NOT inherit the
//!   inner's); the per-round challenge union is now over the deg-8 field.

// Re-export the leaf config as the single source of truth for the verifier-AIR
// (the recursive verifier must NOT re-declare FRI/Poseidon2/field params).
pub use descender_account_stark::{
    Proof as LeafProof, AccountCreationPublic as LeafPublic, PROD_LOG_BLOWUP, PROD_NUM_QUERIES,
    PROD_QUERY_POW_BITS, PROD_SOUNDNESS_BITS,
};

// Phase-1 gadget modules are added incrementally; see the breakdown above. Each
// lands with its own negative tests (mirroring the leaf's adversarial-test
// discipline: test soundness by attempted forgery, never by substring scan).
//
pub mod challenger; // gadget 1 — reference layer (row-based DuplexChallenger) landed
pub mod merkle; // gadget 2 — Merkle path-verify reference (binary fold) landed
pub mod fri_fold; // gadget 3 — FRI commit-phase fold reference (binary) landed
pub mod deep; // gadget 4 — DEEP-ALI out-of-domain quotient relation (core) landed
//   mod fri_fold;         // gadget 3
//   mod constraint_eval;  // gadget 4
//   mod public_io;        // gadget 5
//   mod recursive_air;    // assembles 1–5 into one verifier-AIR

/// Phase-1 milestone marker (kept until `recursive_air` lands). Asserts the
/// re-exported leaf config is reachable so the crate's single-source-of-truth wiring
/// is real, not aspirational.
pub const fn phase1_target_present() -> bool {
    // Touch the re-exported constants so a future config drift breaks the build here.
    PROD_NUM_QUERIES > 0 && PROD_LOG_BLOWUP > 0 && PROD_QUERY_POW_BITS > 0 && PROD_SOUNDNESS_BITS > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase1_scaffold_wired_to_leaf_config() {
        assert!(phase1_target_present());
        // Pin the leaf params the recursive verifier-AIR must mirror; if the leaf
        // config changes, this is the first place that must be revisited.
        assert_eq!(PROD_NUM_QUERIES, 56);
        assert_eq!(PROD_LOG_BLOWUP, 4);
    }
}
