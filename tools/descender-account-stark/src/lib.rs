//! # descender-account-stark
//!
//! Path 2 account-creation ZK-STARK implementation artifact for Descender: the
//! full `C1 ∧ C2 ∧ C4 ∧ C6` account-creation relation re-hosted on Plonky3 /
//! KoalaBear with the pinned vetted Poseidon2 instance and a ~128-bit-conjectured
//! ZK FRI config. Product authority still starts in `spec/`, ADRs, and
//! `TRACKER.md`; this crate is implementation evidence for that authority chain.
//!
//! The whole Plonky3 dependency surface is isolated to this crate. Signer-side
//! `tools/parameter-search` default builds do not depend on Plonky3; they
//! interact with account-creation proofs only through this library's API.
//!
//! ## What the relation proves (in zero knowledge)
//!
//! Given the secret [`AccountCreationSecrets`], there exists a witness such that:
//!   * `pad_source_commitment   = Poseidon2-commit(pad_seed ‖ nonce_c1)`        (C1)
//!   * `masks[6]                = Poseidon2-PRF(pad_seed ‖ slot) mod q12289`      (C2)
//!   * `opening_tuple_commitment = Poseidon2-commit(account_projection ‖
//!                                  lifecycle_anchor ‖ nonce_c6 ‖ account_id)`    (C6)
//!   * `masked_projection[j]    = (account_projection[j] + masks[j]) mod q12289` (C4)
//!     a CANONICAL q12289 residue (witnessed mod-q reduction), with
//!     `account_projection` canonical base-128 (< q12289),
//! and the shared `pad_seed` (C1↔C2) and `account_projection` (C4↔C6) are bound
//! by same-row seam constraints — one proof certifies all four obligations.
//!
//! ## Context binding (no universal-admission token)
//!
//! The public statement [`AccountCreationPublic`] carries the
//! `pad_source_commitment`, `opening_tuple_commitment`, `masked_projection` AND a
//! PUBLIC CONTEXT `(account_id, slot_id, verifier_version, account_state_root,
//! lifecycle_anchor)`. The proof is bound to that exact context:
//!   * `slot_id`        — the slot absorbed into the C2 PRF is constrained EQUAL
//!     to the public `slot_id` (in-circuit AIR equality).
//!   * `lifecycle_anchor`, `account_id` — absorbed into the C6 opening-tuple
//!     preimage and constrained EQUAL to the public values (in-circuit AIR
//!     equality; account_id is the opening-tuple-account-binding).
//!   * `verifier_version`, `account_state_root` — part of the public statement and
//!     observed into the Fiat–Shamir transcript (transcript-bound).
//! Changing ANY context field changes the public statement and rejects the proof,
//! so one proof is NOT reusable across accounts / slots / states / verifiers /
//! lifecycles. The `pad_seed`, `account_projection` and nonces stay private
//! (witness-hiding via salted-leaf hiding Merkle + HidingFriPcs).
//!
//! ## Public API
//!
//! Single instance:
//!   * [`prove_account_creation`] / [`verify_account_creation`]
//! Batch (N instances in one proof, bound to a single Poseidon2 digest):
//!   * [`prove_account_creation_batch`] / [`verify_account_creation_batch`]
//!
//! Proofs are [`Proof`] (re-exported Plonky3 proof, `serde`-serializable); use
//! [`proof_size_bytes`] for a compact-binary size estimate.

mod air;
mod config;
mod counting_serializer;
pub mod reduce_gadget;
pub mod sponge_compression;
pub mod pad_seed_registry; // F2 — global reorg-safe pad-seed admission registry (mask-PRF single-use, persistent across reorgs)
pub mod poseidon2_sponge; // shared in-AIR Poseidon2 sponge primitives (free-function form; spend/recursion reuse)
pub mod spend_air; // Phase-2 — SPEND leaf AIR (core): packed-witness consumption + canonical residues + statement

use p3_field::PrimeCharacteristicRing;
use p3_uni_stark::{prove as p3_prove, verify as p3_verify};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use p3_fri::FriParameters;

use p3_koala_bear::{
    default_koalabear_poseidon2_16, KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
    KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL, KOALABEAR_POSEIDON2_RC_16_INTERNAL,
};

use air::{
    chain_digest, full_public_values, generate_batch_trace, generate_full_trace, FullInstance,
    FullLayout, FullRelationAir, FullSecrets,
};

pub use air::{
    reference_c2_mask_prf, BATCH_PUBLIC_VALUES, CHAIN_ABSORB_LEN, CHAIN_DIGEST_ELEMS,
    CHAIN_PERMS_PER_ROW, CHAIN_RATE, CHECKED_CORE_ELEMS, CONTEXT_ELEMS, N_COMMIT_LANES, N_LANES,
    PACKED_LANE_BITS, PACKED_WITNESS_COMMITMENT_BYTES, PACKED_WITNESS_RESIDUES, PERM_WIDTH,
    PUBIO_ELEMS, PUBIO_NULLIFIER_OFF,
    PUBIO_ROLE_ROOT_OFF, RELATION_ID, RELATION_ID_ELEMS, ROLE_ROOT_ELEMS, SYNDROME_LANES,
    NULLIFIER_DOMAIN, NULLIFIER_ELEMS,
};
pub use config::{
    Challenge, Challenger, MyCompress, MyHash, Perm, Val, DIGEST_ELEMS, NUM_RANDOM_CODEWORDS,
    PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS, PROD_SOUNDNESS_BITS, SALT_ELEMS, WIDTH,
};

use config::{
    Dft, HidingChallengeMmcs, HidingValMmcs, LinLayers, ZkConfig, ZkPcs,
    HALF_FULL_ROUNDS, PARTIAL_ROUNDS, PROD_COMMIT_POW_BITS, SBOX_DEGREE, SBOX_REGISTERS,
};

/// The concrete account-creation relation AIR (pinned KoalaBear Poseidon2 params).
type RelationAir =
    FullRelationAir<Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS>;

/// A re-exported Plonky3 STARK proof for the pinned config. `serde`-serializable.
pub type Proof = p3_uni_stark::Proof<ZkConfig>;

// ===========================================================================
// Public statement / witness types
// ===========================================================================

/// The SECRET witness for one account-creation instance (never leaves the prover).
///
/// NOTE: `slot_id`, `lifecycle_anchor`, `account_id`, `verifier_version` and
/// `account_state_root` are ALSO the PUBLIC context the proof is bound to (see
/// [`AccountCreationPublic`]); they are listed here because the prover supplies
/// them as the values absorbed in-circuit. They are NOT hidden.
#[derive(Clone, Debug)]
pub struct AccountCreationSecrets {
    /// 32-byte pad seed (C1 commitment input + C2 PRF key material). HIDDEN.
    pub pad_seed: [u8; 32],
    /// Slot identifier (C2 PRF domain separator). PUBLIC CONTEXT (bound in-circuit).
    pub slot_id: u64,
    /// The 6 account-projection lanes (canonical base-128, each `< q12289 = 12289`). HIDDEN.
    pub account_projection: [u16; N_LANES],
    /// 32-byte lifecycle anchor (C6 commitment input). PUBLIC CONTEXT (bound in-circuit).
    pub lifecycle_anchor: [u8; 32],
    /// C1 commitment blinding nonce (16 bytes). HIDDEN.
    pub c1_nonce: [u8; 16],
    /// C6 commitment blinding nonce (16 bytes). HIDDEN.
    pub c6_nonce: [u8; 16],
    /// 32-byte account identity, folded into the C6 opening-tuple preimage.
    /// PUBLIC CONTEXT (opening-tuple-account-binding, bound in-circuit).
    pub account_id: [u8; 32],
    /// Verifier version. PUBLIC CONTEXT (transcript-bound).
    pub verifier_version: u32,
    /// 32-byte account state root. PUBLIC CONTEXT (transcript-bound).
    pub account_state_root: [u8; 32],
    /// CLASS C — checked-core SYNDROME lanes (residues 0..11 of the spend's
    /// canonical 18-residue packed-witness frame). Each MUST be a canonical
    /// q12289 residue (`< 12289`); they are PUBLIC statement inputs (bound
    /// in-circuit into `packed_witness_commitment`). The 6 masked_projection
    /// residues (12..17) are derived in-circuit from `account_projection`.
    pub syndrome: [u16; SYNDROME_LANES],
}

/// The PUBLIC statement for one account-creation instance.
///
/// The proof is bound to a SPECIFIC `(account_id, slot_id, verifier_version,
/// account_state_root, lifecycle_anchor)` context: changing ANY context field
/// changes the public statement (and the FS transcript), and for the
/// in-circuit-bound fields (`slot_id`, `lifecycle_anchor`, `account_id`) it also
/// violates an AIR equality constraint. This closes the context-free
/// universal-admission-token finding — one proof is no longer usable for any
/// account / slot / state / verifier / lifecycle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountCreationPublic {
    /// C1: `Poseidon2-commit(pad_seed ‖ c1_nonce)`, 10 squeezed lanes.
    pub pad_source_commitment: [Val; N_COMMIT_LANES],
    /// C6: `Poseidon2-commit(account_projection ‖ lifecycle_anchor ‖ c6_nonce ‖ account_id)`.
    pub opening_tuple_commitment: [Val; N_COMMIT_LANES],
    /// C4: `(account_projection[j] + masks[j]) mod q12289`, 6 canonical q-residue lanes.
    pub masked_projection: [Val; N_LANES],
    // --- PUBLIC CONTEXT (the binding that kills the universal-admission token) ---
    /// Account identity (in-circuit-bound into the C6 opening tuple).
    pub account_id: [u8; 32],
    /// Slot identifier (in-circuit-bound into the C2 PRF lanes).
    pub slot_id: u64,
    /// Verifier version (transcript-bound).
    pub verifier_version: u32,
    /// Account state root (transcript-bound).
    pub account_state_root: [u8; 32],
    /// Lifecycle anchor (in-circuit-bound into the C6 opening tuple).
    pub lifecycle_anchor: [u8; 32],
    // --- CLASS A: ROLE-RESTRICTION (role-root anchor) ---
    /// `role_root = Poseidon2(pad_source_commitment ‖ opening_tuple_commitment ‖
    /// masked_projection)`, computed IN-AIR from the commitment columns and
    /// exposed as a transcript-bound public value. Binds the proof to exactly
    /// this named set of role handles under one root (role-restriction). This is
    /// what the deployed leaf's `creation_proof_binding_digest` / native role
    /// root will later check against.
    pub role_root: [Val; ROLE_ROOT_ELEMS],
    // --- CLASS A-2: SINGLE-USE NULLIFIER (F2: pad-seed reuse under reorg) ---
    /// `nullifier = Poseidon2(DOMAIN ‖ pad_seed ‖ slot)`, computed IN-AIR and
    /// exposed as a transcript-bound public value. Nonce-FREE (unlike
    /// `pad_source_commitment`), so two proofs reusing one `pad_seed` for one
    /// slot share a nullifier — the single-use signal the `pad_seed_registry`
    /// enforces ([`crate::pad_seed_registry`]).
    pub nullifier: [Val; NULLIFIER_ELEMS],
    // --- CLASS B: PROOF-SYSTEM-BOOKKEEPING (relation_id) ---
    /// Pinned relation / proof-system id (constant [`RELATION_ID`]). Pinned
    /// IN-AIR and re-checked by the verifier so a proof for this relation version
    /// + column profile + query schedule cannot be confused with another's.
    pub relation_id: [Val; RELATION_ID_ELEMS],
    // --- CLASS C: CHECKED-CORE-RESIDUE-COMPATIBILITY (8th / final class) ---
    /// Checked-core SYNDROME lanes (residues 0..11 of the packed-witness frame),
    /// echoed as field elements. PUBLIC; range-checked `< q12289` IN-AIR and bound
    /// into `packed_witness_commitment`.
    pub syndrome: [Val; SYNDROME_LANES],
    /// The PUBLIC `packed_witness_commitment` anchor: 32 byte values packing
    /// `[syndrome[0..12] ‖ masked_projection[0..6]]` as 14-bit LITTLE-ENDIAN —
    /// IDENTICAL layout to the deployed
    /// `pack_reviewed_fq12_packed_witness_state18_from_parts`. The in-AIR packing
    /// is constrained EQUAL to these bytes. This is the anchor the deployed
    /// leaf/spend `packed_witness_commitment` will later be checked equal to (that
    /// equality-to-deployed-leaf is the wiring step, not this relation).
    pub packed_witness_commitment: [Val; PACKED_WITNESS_COMMITMENT_BYTES],
}

/// Field-embed the public context exactly as the AIR `context` block:
/// account_id[32] ‖ slot[8] ‖ verifier_version[4] ‖ state_root[32] ‖ lifecycle[32].
fn context_field_elems(p: &AccountCreationPublic) -> Vec<Val> {
    let mut v = Vec::with_capacity(air::CONTEXT_ELEMS);
    v.extend(p.account_id.iter().map(|&b| Val::from_u32(b as u32)));
    v.extend(p.slot_id.to_le_bytes().iter().map(|&b| Val::from_u32(b as u32)));
    v.extend(
        p.verifier_version
            .to_le_bytes()
            .iter()
            .map(|&b| Val::from_u32(b as u32)),
    );
    v.extend(p.account_state_root.iter().map(|&b| Val::from_u32(b as u32)));
    v.extend(p.lifecycle_anchor.iter().map(|&b| Val::from_u32(b as u32)));
    v
}

/// Errors from proving / verifying.
#[derive(Debug)]
pub enum Error {
    /// The STARK proof failed to verify (constraint, FRI, or shape error).
    Verification(String),
    /// A supplied public statement does not match what the proof attests (e.g.
    /// the batch public-IO digest mismatched, or a single-instance public did not
    /// match the recomputed honest statement).
    PublicMismatch(String),
    /// A secret is malformed (e.g. `account_projection` lane out of canonical range).
    InvalidWitness(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Verification(m) => write!(f, "stark verification failed: {m}"),
            Error::PublicMismatch(m) => write!(f, "public statement mismatch: {m}"),
            Error::InvalidWitness(m) => write!(f, "invalid witness: {m}"),
        }
    }
}
impl std::error::Error for Error {}

// ===========================================================================
// Pinned AIR / config construction (one place; used everywhere)
// ===========================================================================

const Q12289: u32 = 12289;

fn build_air(batch: bool) -> RelationAir {
    let beginning_rc = KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL;
    let partial_rc = KOALABEAR_POSEIDON2_RC_16_INTERNAL;
    let ending_rc = KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL;
    FullRelationAir::from_arrays_mode(beginning_rc, partial_rc, ending_rc, batch)
}

/// REFERENCE projection-mask PRF under the PINNED production config, fully
/// concretized (KoalaBear / `default_koalabear_poseidon2_16` / the in-proof C2
/// sponge + wide-then-reduce). Returns `(squeezed_lanes[30], masks[6])`.
///
/// This is the authoritative in-proof derivation the standalone
/// `descender-mask-prf` crate must match byte-for-byte — see
/// `tests/mask_prf_equivalence.rs`. It uses the SAME pinned constants
/// (`KOALABEAR_POSEIDON2_RC_16_*`) the production AIR uses in [`build_air`], and
/// delegates to the SAME private trace primitives, so it cannot drift from the
/// prover.
#[doc(hidden)]
pub fn reference_projection_mask_prf(
    pad_seed: &[u8; 32],
    slot_id: u64,
) -> ([u32; 30], [u32; N_LANES]) {
    let air = build_air(false);
    reference_c2_mask_prf::<
        Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
    >(pad_seed, slot_id, air.constants())
}

/// Build a ZK STARK config under a given mask seed. The mask seed drives the ZK
/// randomness ONLY (hiding-Merkle salt RNG + HidingFriPcs codeword/R(X) RNG); the
/// hash instance and FRI knobs are PINNED and identical across seeds.
fn build_config(perm: &Perm, hash: &MyHash, compress: &MyCompress, mask_seed: u64) -> ZkConfig {
    let hiding_val_mmcs = HidingValMmcs::new(
        hash.clone(),
        compress.clone(),
        0,
        SmallRng::seed_from_u64(mask_seed),
    );
    let hiding_challenge_mmcs = HidingChallengeMmcs::new(hiding_val_mmcs.clone());
    let fri_params = FriParameters {
        log_blowup: PROD_LOG_BLOWUP,
        log_final_poly_len: 0,
        max_log_arity: 1,
        num_queries: PROD_NUM_QUERIES,
        commit_proof_of_work_bits: PROD_COMMIT_POW_BITS,
        query_proof_of_work_bits: PROD_QUERY_POW_BITS,
        mmcs: hiding_challenge_mmcs,
    };
    let pcs = ZkPcs::new(
        Dft::default(),
        hiding_val_mmcs,
        fri_params,
        NUM_RANDOM_CODEWORDS,
        SmallRng::seed_from_u64(mask_seed ^ 0xABCD_EF01),
    );
    ZkConfig::new(pcs, Challenger::new(perm.clone()))
}

fn to_full_secrets(s: &AccountCreationSecrets) -> Result<FullSecrets, Error> {
    for (j, &v) in s.account_projection.iter().enumerate() {
        if (v as u32) >= Q12289 {
            return Err(Error::InvalidWitness(format!(
                "account_projection[{j}] = {v} >= q12289 ({Q12289}); not canonical base-128"
            )));
        }
    }
    // CLASS C: the checked-core syndrome lanes must be canonical q12289 residues
    // (the in-AIR canonicity gadget enforces this too; reject early with context).
    for (j, &v) in s.syndrome.iter().enumerate() {
        if (v as u32) >= Q12289 {
            return Err(Error::InvalidWitness(format!(
                "syndrome[{j}] = {v} >= q12289 ({Q12289}); checked-core residue not canonical"
            )));
        }
    }
    Ok(FullSecrets {
        pad_seed: s.pad_seed,
        slot: s.slot_id,
        account_projection: s.account_projection,
        nonce_c1: s.c1_nonce,
        lifecycle_anchor: s.lifecycle_anchor,
        nonce_c6: s.c6_nonce,
        account_id: s.account_id,
        verifier_version: s.verifier_version,
        account_state_root: s.account_state_root,
        syndrome: s.syndrome,
    })
}

fn public_from_instance(
    inst: &FullInstance<Val>,
    s: &AccountCreationSecrets,
) -> AccountCreationPublic {
    AccountCreationPublic {
        pad_source_commitment: inst.pad_source_commitment,
        opening_tuple_commitment: inst.opening_tuple_commitment,
        masked_projection: inst.masked_projection,
        account_id: s.account_id,
        slot_id: s.slot_id,
        verifier_version: s.verifier_version,
        account_state_root: s.account_state_root,
        lifecycle_anchor: s.lifecycle_anchor,
        role_root: inst.role_root,
        nullifier: inst.nullifier,
        relation_id: core::array::from_fn(|j| Val::from_u32(RELATION_ID[j])),
        syndrome: inst.syndrome,
        packed_witness_commitment: inst.packed_witness_commitment,
    }
}

fn public_values_vec(p: &AccountCreationPublic) -> Vec<Val> {
    let mut v = Vec::with_capacity(
        2 * N_COMMIT_LANES
            + N_LANES
            + CONTEXT_ELEMS
            + ROLE_ROOT_ELEMS
            + NULLIFIER_ELEMS
            + RELATION_ID_ELEMS
            + CHECKED_CORE_ELEMS,
    );
    v.extend_from_slice(&p.pad_source_commitment);
    v.extend_from_slice(&p.opening_tuple_commitment);
    v.extend_from_slice(&p.masked_projection);
    v.extend_from_slice(&context_field_elems(p));
    v.extend_from_slice(&p.role_root);
    v.extend_from_slice(&p.nullifier);
    v.extend_from_slice(&p.relation_id);
    v.extend_from_slice(&p.syndrome);
    v.extend_from_slice(&p.packed_witness_commitment);
    v
}

// ===========================================================================
// Single-instance API
// ===========================================================================

/// Prove one account-creation instance under the pinned production config (ZK on).
///
/// Returns the public statement and the STARK proof. `mask_seed` supplies the ZK
/// randomness (hiding-Merkle salts plus HidingFriPcs codewords/R(X)). Use a fresh
/// random `mask_seed` per proof in deployment. Fixed-seed KATs that pin serialized
/// proof bytes must also make proof-of-work witness selection deterministic; the
/// active account-STARK KAT generator does this by running the proof inside a
/// single-thread Rayon pool.
pub fn prove_account_creation_with_seed(
    secrets: &AccountCreationSecrets,
    mask_seed: u64,
) -> Result<(AccountCreationPublic, Proof), Error> {
    let full = to_full_secrets(secrets)?;
    let perm: Perm = default_koalabear_poseidon2_16();
    let hash = MyHash::new(perm.clone());
    let compress = MyCompress::new(MyHash::new(perm.clone()));
    let air = build_air(false);

    // Single-instance trace: 2 identical honest rows (power-of-two minimum).
    let (trace, inst) = generate_full_trace::<
        Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
    >(&full, air.constants(), 2);
    let public = public_from_instance(&inst, secrets);
    let pv = full_public_values(&inst);

    let cfg = build_config(&perm, &hash, &compress, mask_seed);
    let proof = p3_prove(&cfg, &air, trace, &pv);
    Ok((public, proof))
}

/// Prove one account-creation instance (ZK on), drawing FRESH cryptographic
/// randomness per proof for the ZK hiding channel.
///
/// This is the DEPLOYMENT path. The ZK mask seed is drawn from
/// [`fresh_mask_seed`] — a fresh 64-bit value from the OS-seeded ChaCha12 CSPRNG
/// ([`rand::rng`]) on every call — so two proofs of the SAME witness+context are
/// decorrelated (the fixed-seed `_with_seed` path is NOT). For deterministic
/// proof-byte KATs, use [`prove_account_creation_with_seed`] under an explicitly
/// deterministic proof-of-work scheduler.
pub fn prove_account_creation(
    secrets: &AccountCreationSecrets,
) -> Result<(AccountCreationPublic, Proof), Error> {
    prove_account_creation_with_seed(secrets, fresh_mask_seed())
}

/// Draw a fresh 64-bit ZK mask seed from a cryptographic RNG.
///
/// Uses [`rand::rng`] (rand 0.10 `ThreadRng`: a ChaCha12 CSPRNG seeded from the
/// operating-system entropy source via `getrandom`). This replaces the old fixed
/// `DEFAULT_MASK_SEED` non-CSPRNG path. NOTE: the 64-bit seed only diversifies the
/// ZK randomness (hiding-Merkle salt RNG + HidingFriPcs codeword/R(X) RNG); it is
/// NOT key material and does not affect the proven statement.
pub fn fresh_mask_seed() -> u64 {
    use rand::RngExt;
    rand::rng().random()
}

/// Check the CLASS-B `relation_id` of a public statement equals the pinned
/// [`RELATION_ID`]. The AIR ALSO pins it in-circuit; this is the verifier-side
/// re-check (a proof for a different relation/version cannot be passed off as
/// this one even before the STARK runs).
fn check_relation_id(p: &AccountCreationPublic) -> Result<(), Error> {
    for j in 0..RELATION_ID_ELEMS {
        if p.relation_id[j] != Val::from_u32(RELATION_ID[j]) {
            return Err(Error::PublicMismatch(format!(
                "relation_id[{j}] != pinned RELATION_ID (wrong relation/version)"
            )));
        }
    }
    Ok(())
}

/// Verify one account-creation proof against its public statement.
pub fn verify_account_creation(
    public: &AccountCreationPublic,
    proof: &Proof,
) -> Result<(), Error> {
    // CLASS B: the relation_id must be the pinned id (checked before the STARK).
    check_relation_id(public)?;
    let perm: Perm = default_koalabear_poseidon2_16();
    let hash = MyHash::new(perm.clone());
    let compress = MyCompress::new(MyHash::new(perm.clone()));
    let air = build_air(false);
    let cfg = build_config(&perm, &hash, &compress, /*verify mask seed irrelevant*/ 0);
    let pv = public_values_vec(public);
    p3_verify(&cfg, &air, proof, &pv).map_err(|e| Error::Verification(format!("{e:?}")))
}

// ===========================================================================
// Batch API
// ===========================================================================

/// A batch public-IO digest: the in-AIR Poseidon2 *chain* digest (8 field elems)
/// over all N instances' public-IO tuples, IN ROW ORDER. Unlike the old
/// off-circuit-only digest, this value is the STARK's PUBLIC value: it is
/// constrained in-AIR to the per-row pubio columns (a running sponge chain) and
/// observed into the Fiat–Shamir transcript, so the proof is bound to exactly
/// these N tuples in this order. See [`verify_account_creation_batch`].
pub type BatchDigest = [Val; CHAIN_DIGEST_ELEMS];

fn pubio_tuple(p: &AccountCreationPublic) -> [Val; PUBIO_ELEMS] {
    let mut t = [Val::ZERO; PUBIO_ELEMS];
    for j in 0..N_COMMIT_LANES {
        t[j] = p.pad_source_commitment[j];
        t[N_COMMIT_LANES + j] = p.opening_tuple_commitment[j];
    }
    for j in 0..N_LANES {
        t[2 * N_COMMIT_LANES + j] = p.masked_projection[j];
    }
    // Public context is part of the per-instance tuple -> bound by the chain.
    for (i, &e) in context_field_elems(p).iter().enumerate() {
        t[2 * N_COMMIT_LANES + N_LANES + i] = e;
    }
    // CLASS A role_root + CLASS B relation_id are part of the per-instance tuple
    // -> bound by the chain (so a tampered/forged role_root or wrong relation_id
    // changes the recomputed digest and is rejected).
    let role_off = 2 * N_COMMIT_LANES + N_LANES + CONTEXT_ELEMS;
    for j in 0..ROLE_ROOT_ELEMS {
        t[role_off + j] = p.role_root[j];
    }
    // CLASS A-2 nullifier is part of the per-instance tuple -> bound by the chain.
    let nullifier_off = role_off + ROLE_ROOT_ELEMS;
    for j in 0..NULLIFIER_ELEMS {
        t[nullifier_off + j] = p.nullifier[j];
    }
    for j in 0..RELATION_ID_ELEMS {
        t[nullifier_off + NULLIFIER_ELEMS + j] = p.relation_id[j];
    }
    // CLASS C: syndrome[12] ‖ packed_witness_commitment[32] are part of the
    // per-instance tuple -> bound by the digest chain (a tampered syndrome or
    // forged commitment changes the recomputed digest and is rejected).
    let cc_off = nullifier_off + NULLIFIER_ELEMS + RELATION_ID_ELEMS;
    for j in 0..SYNDROME_LANES {
        t[cc_off + j] = p.syndrome[j];
    }
    for j in 0..PACKED_WITNESS_COMMITMENT_BYTES {
        t[cc_off + SYNDROME_LANES + j] = p.packed_witness_commitment[j];
    }
    t
}

/// Recompute the in-AIR digest chain over the supplied publics. Mirrors the AIR
/// (`air::chain_digest`) EXACTLY, so the verifier's recompute and the prover's
/// in-circuit public digest agree iff the publics are exactly the proven rows
/// in the proven order.
fn compute_digest(air: &RelationAir, publics: &[AccountCreationPublic]) -> BatchDigest {
    let pubios: Vec<[Val; PUBIO_ELEMS]> = publics.iter().map(pubio_tuple).collect();
    chain_digest::<
        Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
    >(&pubios, air.constants())
}

/// Test-only: recompute the in-AIR digest chain over a list of public tuples
/// (the same function the verifier uses). Exposed so adversarial tests can build
/// a self-consistent-but-proof-mismatching digest. NOT part of the deployment
/// surface; the verifier computes this internally.
#[doc(hidden)]
pub fn recompute_chain_digest_for_tests(publics: &[AccountCreationPublic]) -> BatchDigest {
    let air = build_air(true);
    compute_digest(&air, publics)
}

/// The batch STARK public-value vector: `digest[8] ‖ count_n[1]`.
fn batch_public_values(digest: &BatchDigest, n: usize) -> Vec<Val> {
    let mut v = Vec::with_capacity(BATCH_PUBLIC_VALUES);
    v.extend_from_slice(digest);
    v.push(Val::from_u32(n as u32));
    v
}

/// Prove N independent account-creation instances in ONE proof (batch / aggregation).
///
/// Returns `(digest, publics, proof)` where:
///   * `publics[r]` is instance `r`'s public statement (one per input secret),
///   * `digest` is the Poseidon2 digest over all `publics` (the single STARK
///      public value — the binding the verifier checks),
///   * `proof` attests that every committed row is a valid account-creation
///      instance whose public IO equals its `pubio` columns.
///
/// The verifier is handed `digest`; the prover ships `publics` alongside the
/// proof (the irreducible per-instance payload). See
/// [`verify_account_creation_batch`].
pub fn prove_account_creation_batch_with_seed(
    secrets: &[AccountCreationSecrets],
    mask_seed: u64,
) -> Result<(BatchDigest, Vec<AccountCreationPublic>, Proof), Error> {
    if secrets.is_empty() {
        return Err(Error::InvalidWitness("empty batch".into()));
    }
    let full: Vec<FullSecrets> = secrets
        .iter()
        .map(to_full_secrets)
        .collect::<Result<_, _>>()?;

    let perm: Perm = default_koalabear_poseidon2_16();
    let hash = MyHash::new(perm.clone());
    let compress = MyCompress::new(MyHash::new(perm.clone()));
    let air = build_air(true);

    let (trace, insts, digest, n) = generate_batch_trace::<
        Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
    >(&full, air.constants());

    let publics: Vec<AccountCreationPublic> = insts
        .iter()
        .zip(secrets.iter())
        .map(|(inst, s)| public_from_instance(inst, s))
        .collect();
    debug_assert_eq!(digest, compute_digest(&air, &publics));
    debug_assert_eq!(n, publics.len());

    // The batch AIR exposes the chained digest + count N as STARK public values
    // (observed into the Fiat–Shamir transcript and constrained in-AIR).
    let pv = batch_public_values(&digest, n);
    let cfg = build_config(&perm, &hash, &compress, mask_seed);
    let proof = p3_prove(&cfg, &air, trace, &pv);
    Ok((digest, publics, proof))
}

/// Batch prove (ZK on), drawing FRESH cryptographic randomness per proof.
///
/// Deployment path: the ZK mask seed comes from [`fresh_mask_seed`] (OS-seeded
/// CSPRNG) so batch proofs are decorrelated across calls. Use
/// [`prove_account_creation_batch_with_seed`] for deterministic tests.
pub fn prove_account_creation_batch(
    secrets: &[AccountCreationSecrets],
) -> Result<(BatchDigest, Vec<AccountCreationPublic>, Proof), Error> {
    prove_account_creation_batch_with_seed(secrets, fresh_mask_seed())
}

/// Verify a batch proof: (a) the STARK verifies (every committed row is a valid
/// instance bound to its pubio columns), and (b) the digest of the supplied
/// `publics` equals `digest` (the binding the verifier was handed).
pub fn verify_account_creation_batch(
    digest: &BatchDigest,
    publics: &[AccountCreationPublic],
    proof: &Proof,
) -> Result<(), Error> {
    if publics.is_empty() {
        return Err(Error::InvalidWitness("empty batch".into()));
    }
    // CLASS B: every instance's relation_id must be the pinned id.
    for p in publics {
        check_relation_id(p)?;
    }
    let perm: Perm = default_koalabear_poseidon2_16();
    let hash = MyHash::new(perm.clone());
    let compress = MyCompress::new(MyHash::new(perm.clone()));
    let air = build_air(true);
    let cfg = build_config(&perm, &hash, &compress, 0);

    // (1) Recompute the chain digest over the SUPPLIED tuples (in order). This
    //     must equal the supplied digest — otherwise the caller's tuples are not
    //     what hashes to `digest`.
    let recomputed = compute_digest(&air, publics);
    if &recomputed != digest {
        return Err(Error::PublicMismatch(
            "recomputed public-IO chain digest != supplied digest".into(),
        ));
    }

    // (2) Verify the STARK with `digest ‖ N` AS the public values. The AIR
    //     constrains the in-circuit running chain to equal this public digest
    //     (and the running count to equal N), and the digest+N are observed
    //     into the Fiat–Shamir transcript. So the proof is bound to EXACTLY the
    //     N tuples (in this order) that chain to `digest`: supplying a different
    //     batch's tuples, garbage tuples, a reordering, or a different count
    //     changes `recomputed` (caught in step 1) and/or the public values the
    //     proof was bound to (caught here).
    let pv = batch_public_values(digest, publics.len());
    p3_verify(&cfg, &air, proof, &pv).map_err(|e| Error::Verification(format!("{e:?}")))?;

    Ok(())
}

// ===========================================================================
// Helpers
// ===========================================================================

/// Compact-binary (bincode-like) serialized size of a proof, in bytes.
pub fn proof_size_bytes(proof: &Proof) -> usize {
    use serde::Serialize;
    let mut counter = counting_serializer::ByteCounter(0);
    proof
        .serialize(&mut counter)
        .expect("proof serialization failed");
    counter.0
}

/// The trace width (columns per instance) under the pinned config. Useful for
/// sizing / diagnostics.
pub fn trace_columns() -> usize {
    let air = build_air(false);
    FullLayout::new(air.layout().perm_cols).width
}

/// Column index (within one instance's trace row) of the `pad_seed` byte `k`
/// (`k < 32`) in the C1 input-lane block. Exposed for ZK-hiding tests that probe
/// the opened value of a raw-witness column. Not needed by deployment.
pub fn padseed_witness_column(k: usize) -> usize {
    assert!(k < 32, "pad_seed has 32 bytes");
    let air = build_air(false);
    let lay = FullLayout::new(air.layout().perm_cols);
    lay.c1_off + lay.c1_lanes_off() + k
}

// ===========================================================================
// CLASS C — checked-core-residue-compatibility: ONE small-config prove/verify.
//
// RESOURCE NOTE: this is the SINGLE small-config (low blowup, few queries)
// prove/verify completeness test for the relation; it exercises real FRI/proving
// at minimal RAM so the new CLASS-C public values (syndrome + packed witness
// commitment) survive the full pipeline, not just the debug constraint checker.
// All other CLASS-C validation is via in-AIR check_constraints unit tests (no
// proving) in `air.rs`.
// ===========================================================================
#[cfg(test)]
mod small_config_classc_tests {
    use super::*;
    use config::{HidingChallengeMmcs, HidingValMmcs, MyCompress, MyHash, Perm};

    fn small_secrets() -> AccountCreationSecrets {
        AccountCreationSecrets {
            pad_seed: core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1)),
            slot_id: 0x0000_0000_0000_0042,
            account_projection: [11, 2222, 333, 4444, 555, 12288],
            lifecycle_anchor: core::array::from_fn(|i| (i as u8).wrapping_mul(5).wrapping_add(2)),
            c1_nonce: core::array::from_fn(|i| (i as u8).wrapping_mul(3).wrapping_add(5)),
            c6_nonce: core::array::from_fn(|i| (i as u8).wrapping_mul(9).wrapping_add(1)),
            account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3)),
            verifier_version: 0x0102_0304,
            account_state_root: core::array::from_fn(|i| (i as u8).wrapping_mul(17).wrapping_add(7)),
            syndrome: core::array::from_fn(|i| ((i as u32 * 1009 + 17) % 12289) as u16),
        }
    }

    /// FROZEN q128 KAT: a deterministic PRODUCTION-config (digest-13 sponge, rate-7, 120 queries,
    /// query-PoW 16) account-creation proof has a stable SHA-256 digest. Proving runs single-threaded
    /// so the FRI query-PoW grind is deterministic (the parallel grind returns a witness-order-
    /// dependent nonce). This pins the q128 proof BYTE FORMAT against drift — a frozen-byte-format KAT
    /// (DIR-STARK-001). Slow (full production prove, twice); run explicitly.
    #[test]
    #[ignore = "slow: production-config prove ×2 single-threaded; run explicitly for the frozen q128 KAT"]
    fn frozen_q128_leaf_proof_kat() {
        use sha2::{Digest, Sha256};
        let s = small_secrets();
        let digest_once = || -> [u8; 32] {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap();
            let (_public, proof) = pool
                .install(|| prove_account_creation_with_seed(&s, 0xF20D_2026_u64))
                .expect("prove");
            let bytes = bincode::serialize(&proof).expect("serialize proof");
            let mut h = Sha256::new();
            h.update(&bytes);
            h.finalize().into()
        };
        let d1 = digest_once();
        let d2 = digest_once();
        assert_eq!(d1, d2, "q128 leaf proof is not deterministic under single-thread PoW");
        let hex: String = d1.iter().map(|b| format!("{b:02x}")).collect();
        eprintln!("FROZEN-KAT q128 leaf proof sha256 = {hex}");
        // PINNED frozen q128 KAT digest (single-thread, seed 0xF20D_2026, `small_secrets`).
        // Re-pinned 2026-06-27 when CLASS A-2 (the F2 single-use `nullifier` public output) was
        // added to the leaf relation — the new public value changes the proof bytes by design.
        const PINNED: &str = "652e8b8397c0dc236f58f4ff7f182070426c3b1d135803c44f4a1e87be01e1fa";
        assert_eq!(hex, PINNED, "q128 leaf proof bytes drifted from the frozen KAT");
    }

    /// Build a DELIBERATELY SMALL ZK config: log_blowup=2 (the ZK minimum) and only
    /// 2 FRI queries, query-PoW off. NOT production sound — for completeness/RAM
    /// only.
    fn small_config(perm: &Perm, hash: &MyHash, compress: &MyCompress) -> ZkConfig {
        let mask_seed = 0xC0FF_EEu64;
        let hiding_val_mmcs =
            HidingValMmcs::new(hash.clone(), compress.clone(), 0, SmallRng::seed_from_u64(mask_seed));
        let hiding_challenge_mmcs = HidingChallengeMmcs::new(hiding_val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: 2,
            log_final_poly_len: 0,
            max_log_arity: 1,
            num_queries: 2,
            commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: 0,
            mmcs: hiding_challenge_mmcs,
        };
        let pcs = ZkPcs::new(
            Dft::default(),
            hiding_val_mmcs,
            fri_params,
            NUM_RANDOM_CODEWORDS,
            SmallRng::seed_from_u64(mask_seed ^ 0xABCD_EF01),
        );
        ZkConfig::new(pcs, Challenger::new(perm.clone()))
    }

    #[test]
    fn small_config_single_prove_verify_with_classc() {
        let s = small_secrets();
        let full = to_full_secrets(&s).expect("valid witness");
        let perm: Perm = default_koalabear_poseidon2_16();
        let hash = MyHash::new(perm.clone());
        let compress = MyCompress::new(MyHash::new(perm.clone()));
        let air = build_air(false);

        // Smallest power-of-two trace (2 identical honest rows).
        let (trace, inst) = generate_full_trace::<
            Val, LinLayers, WIDTH, SBOX_DEGREE, SBOX_REGISTERS, HALF_FULL_ROUNDS, PARTIAL_ROUNDS,
        >(&full, air.constants(), 2);
        let pv = full_public_values(&inst);

        let cfg = small_config(&perm, &hash, &compress);
        let proof = p3_prove(&cfg, &air, trace, &pv);

        // Verify with the SAME public values (which include syndrome + packed
        // witness commitment); the CLASS-C constraints are part of the proof.
        p3_verify(&cfg, &air, &proof, &pv).expect("small-config proof verifies");

        // Sanity: the public statement carries the CLASS-C values.
        let public = public_from_instance(&inst, &s);
        assert_eq!(public.syndrome.len(), SYNDROME_LANES);
        assert_eq!(public.packed_witness_commitment.len(), PACKED_WITNESS_COMMITMENT_BYTES);
    }

    /// Phase-2 SPEND leaf AIR: a real small-config STARK prove/verify of the spend core — the spend
    /// proves its PUBLIC `packed_witness_commitment` is a well-formed packing of 18 canonical q12289
    /// residues (the creation↔spend consumption invariant). Also checks the spend's commitment EQUALS
    /// the creation leaf's published `packed_witness_commitment` over the same residues (the MED-2
    /// binding the recursion/settlement layer enforces between the two proofs).
    #[test]
    fn small_config_spend_prove_verify() {
        use crate::spend_air::{generate_spend_trace, pack_spend_witness, SpendAir, SpendWitness};
        use p3_koala_bear::{
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL, KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
        };

        let perm: Perm = default_koalabear_poseidon2_16();
        let hash = MyHash::new(perm.clone());
        let compress = MyCompress::new(MyHash::new(perm.clone()));
        let air = SpendAir::from_arrays(
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
        );

        // Residues consumed by the spend = a canonical reviewed-fq12 frame; the AIR's in-circuit
        // packing reproduces the reference packer, and the in-AIR Poseidon2 frame-digest matches the
        // native recompute (run_sponge), all checked by prove/verify.
        let residues: [u16; 18] = core::array::from_fn(|i| ((i as u32 * 1009 + 17) % 12289) as u16);
        let packed = pack_spend_witness(&residues);
        let w = SpendWitness {
            residues,
            packed_witness_commitment: packed,
            account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3)),
            slot_id: 0x0000_0000_0000_0042,
            account_state_root: core::array::from_fn(|i| (i as u8).wrapping_mul(17).wrapping_add(7)),
            message_digest: core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1)),
        };
        let (trace, pv, _frame_digest) = generate_spend_trace(&air, &w);

        let cfg = small_config(&perm, &hash, &compress);
        let proof = p3_prove(&cfg, &air, trace, &pv);
        p3_verify(&cfg, &air, &proof, &pv).expect("small-config spend proof verifies");
    }
}
