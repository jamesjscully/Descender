//! Pinned Path 2 configuration of the account-creation ZK-STARK.
//!
//! This is the active implementation config lifted from the re-host benchmark's
//! `prod_bench.rs`:
//!
//!   * Poseidon2: the PINNED Plonky3 vetted instance
//!     `default_koalabear_poseidon2_16()` (Grain-derived `RC_16` tables). The SAME
//!     instance drives (a) the AIR Poseidon2 round constraints, (b) the witness
//!     trace generator, and (c) the Merkle/MMCS hash + the Fiat–Shamir challenger.
//!   * Hash/Merkle: Poseidon2-Merkle (recursion-friendly), NOT Keccak. QUANTUM-128
//!     profile (ADR-0008): `PaddingFreeSponge<Perm,16,7,13>` leaf (rate 7, capacity
//!     c=9) + sponge-mode `SpongeCompression` 2-to-1 (a 13-element digest cannot use a
//!     truncated permutation: `2*13 > WIDTH=16`); 13-field-element (~403-bit) digests,
//!     so the quantum (BHT) collision floor is ~403/3 ≈ 134 bits and the sponge
//!     indifferentiability floor is c/2 ≈ 139 bits.
//!   * FRI (quantum-128 CONSERVATIVE / Johnson-bound, NOT capacity-regime):
//!     `log_blowup = 4` (rate rho = 1/16; ZK requires >= 2),
//!     `num_queries = 120`, `query_proof_of_work_bits = 16`.
//!     See [`PROD_SOUNDNESS_BITS`] for the exact soundness calculation: the
//!     conservative Johnson figure is `120*(0.5*4) + 16 = 256` classical, so the
//!     Grover-halved non-interactive FS-grind floor is ~128 quantum. (Raised from the
//!     56-query digest-8 profile, whose ~64-bit quantum FS-grind floor was the binding
//!     quantum-128 blocker.)
//!   * ZK ON: `HidingFriPcs` (doubled domain + random codewords + ethSTARK R(X))
//!     over a salted-leaf `MerkleTreeHidingMmcs`.

use p3_challenger::DuplexChallenger;
use p3_commit::ExtensionMmcs;
use p3_field::extension::BinomialExtensionField;
use p3_field::Field;
use p3_fri::HidingFriPcs;
use p3_merkle_tree::MerkleTreeHidingMmcs;
use p3_symmetric::PaddingFreeSponge;
use p3_uni_stark::StarkConfig;
use rand::rngs::SmallRng;

use p3_koala_bear::{
    GenericPoseidon2LinearLayersKoalaBear, KoalaBear, Poseidon2KoalaBear,
    KOALABEAR_POSEIDON2_HALF_FULL_ROUNDS, KOALABEAR_POSEIDON2_PARTIAL_ROUNDS_16,
    KOALABEAR_S_BOX_DEGREE,
};

// ---------------------------------------------------------------------------
// Pinned scheme parameters (named consts).
// ---------------------------------------------------------------------------

/// Poseidon2 permutation width.
pub const WIDTH: usize = 16;

/// PINNED FRI: blowup exponent. Rate `rho = 1/2^PROD_LOG_BLOWUP = 1/16`.
/// Chosen at 4 (not 2) so each query contributes `0.5*log_blowup = 2` conservative
/// Johnson bits; at the quantum-128 profile's 120 queries that is `120*2 + 16 = 256`
/// classical ⇒ ~128 quantum after Grover-halving the FS-grind (see `PROD_SOUNDNESS_BITS`).
/// ZK requires `log_blowup >= 2`; 4 satisfies that.
pub const PROD_LOG_BLOWUP: usize = 4;
/// PINNED FRI: query count.
pub const PROD_NUM_QUERIES: usize = 120;
/// PINNED FRI: query proof-of-work grinding bits.
pub const PROD_QUERY_POW_BITS: usize = 16;
/// PINNED FRI: commit-phase proof-of-work bits.
pub const PROD_COMMIT_POW_BITS: usize = 0;

/// FRI soundness bits at the pinned config under the CONSERVATIVE Johnson-bound
/// estimate (NOT the optimistic up-to-capacity / proximity-gap-conjecture regime,
/// which was disproven late-2025 and is therefore unsafe to rely on).
///
/// Conservative model (ethSTARK / Johnson rule of thumb): at rate
/// `rho = 1/2^log_blowup`, each query contributes a per-query soundness factor of
/// `~ sqrt(rho)`, i.e. `-log2(sqrt(rho)) = 0.5 * log_blowup` bits — HALF the
/// optimistic capacity-regime `log_blowup` bits/query. Adding the query-phase
/// proof-of-work grinding:
///
///   conservative_bits = num_queries * (0.5 * log_blowup) + query_pow_bits
///                     = 120 * (0.5 * 4)     + 16
///                     = 120 * 2             + 16
///                     = 256.
///
/// This 256 classical figure is the quantum-128 sizing: the non-interactive FS query
/// grind is Grover-able, halving the effective budget to ~128 quantum (ADR-0008). (For
/// contrast the OPTIMISTIC capacity formula would read `num_queries*log_blowup +
/// query_pow = 120*4 + 16 = 496` bits — we do NOT claim that.) Expressed as an
/// integer-bits const, `0.5*log_blowup` is exact here because `log_blowup` is even.
///
/// IMPORTANT (2026-06-19): this is the QUERY-PHASE figure ONLY. The COMPOSED
/// knowledge-soundness error also carries the FRI commit/fold + DEEP ADDITIVE term
/// (~|L|/|F|), which is bounded by the CHALLENGE-FIELD size, not the query schedule.
/// At the former degree-4 extension that additive term capped the composed figure at
/// ~2^-104..-106 (large batches) — below this 128. The degree-8 `Challenge`
/// (see its doc) restores the composed figure to ~2^-228..-230. See
/// research/cryptanalysis/fri_aok_extractor_instantiation.md.
pub const PROD_SOUNDNESS_BITS: usize =
    PROD_NUM_QUERIES * (PROD_LOG_BLOWUP / 2) + PROD_QUERY_POW_BITS;

/// HidingFriPcs random codewords (>= opened points per matrix).
pub const NUM_RANDOM_CODEWORDS: usize = 4;
/// Salt field elems per hiding Merkle leaf (~31 bits each).
pub const SALT_ELEMS: usize = 4;
/// Poseidon2-Merkle digest width (field elements).
pub const DIGEST_ELEMS: usize = 13;

// ---------------------------------------------------------------------------
// KoalaBear Poseidon2 const-generic parameters (the pinned instance's shape).
// ---------------------------------------------------------------------------

pub const SBOX_DEGREE: u64 = KOALABEAR_S_BOX_DEGREE;
pub const SBOX_REGISTERS: usize = 0;
pub const HALF_FULL_ROUNDS: usize = KOALABEAR_POSEIDON2_HALF_FULL_ROUNDS;
pub const PARTIAL_ROUNDS: usize = KOALABEAR_POSEIDON2_PARTIAL_ROUNDS_16;

// ---------------------------------------------------------------------------
// Concrete type aliases for the pinned KoalaBear / Poseidon2-Merkle / ZK-FRI stack.
// ---------------------------------------------------------------------------

/// The base STARK field.
pub type Val = KoalaBear;
/// Degree-8 binomial extension (the challenge field, |F| = p^8 ~ 2^247.9).
///
/// Raised from degree-4 (~124 bits) on 2026-06-19 to clear the FIELD-BOUND FRI
/// knowledge-soundness term: the FRI commit/fold + DEEP additive error scales as
/// ~|L|/|F|, and at the deployed degree-4 extension the COMPOSED knowledge soundness
/// was only ~2^-104..-106 at large batch sizes (worse, ~2^-92..-100, for very
/// large batches) — short of the 2^-128 target even though the query phase hits 128.
/// A degree-5 BINOMIAL extension is impossible over KoalaBear (p-1 = 2^24*127,
/// gcd(5,p-1)=1 => x^5-W always reducible); degree-8 is the next supported binomial
/// (BinomialExtensionData<8>, EXT_TWO_ADICITY=27) and gives an unconditional +124-bit
/// field margin (composed ~2^-228..-230), batch-size- and list-constant-independent.
/// See research/cryptanalysis/{fri_aok_extractor_instantiation,degree5_field_rehost_scope}.md.
/// NOTE: the Poseidon2-sponge indifferentiability floor (c/2, capacity-bound) is
/// INDEPENDENT of this field. At the quantum-128 profile the leaf/Merkle sponge uses
/// capacity c = WIDTH - RATE = 16 - 7 = 9, so that floor is c/2 ≈ 139 bits (raised from
/// the digest-8 profile's c=8 ⇒ ~124). The FS `Challenger` below retains rate 8 / c=8;
/// whether it must also move to c=9 for the headline is tracked in
/// research/path2_quantum128_parameterization_v1.md (config assumption-boundary note).
pub type Challenge = BinomialExtensionField<Val, 8>;
/// The pinned Poseidon2 permutation.
pub type Perm = Poseidon2KoalaBear<16>;
/// The Poseidon2 linear layers for the AIR.
pub type LinLayers = GenericPoseidon2LinearLayersKoalaBear;

/// Poseidon2-Merkle leaf hash.
pub type MyHash = PaddingFreeSponge<Perm, 16, 7, DIGEST_ELEMS>;
/// Poseidon2-Merkle 2-to-1 compression.
pub type MyCompress = crate::sponge_compression::SpongeCompression<MyHash>;
/// Salted-leaf (ZK) Merkle MMCS.
pub type HidingValMmcs = MerkleTreeHidingMmcs<
    <Val as Field>::Packing,
    <Val as Field>::Packing,
    MyHash,
    MyCompress,
    SmallRng,
    2,
    DIGEST_ELEMS,
    SALT_ELEMS,
>;
/// Extension-field MMCS over the hiding base MMCS.
pub type HidingChallengeMmcs = ExtensionMmcs<Val, Challenge, HidingValMmcs>;
/// Fiat–Shamir challenger (duplex sponge over the pinned Poseidon2).
pub type Challenger = DuplexChallenger<Val, Perm, 16, 8>;
/// FFT backend.
pub type Dft = p3_dft::Radix2DitParallel<Val>;
/// The ZK polynomial-commitment scheme (HidingFriPcs).
pub type ZkPcs = HidingFriPcs<Val, Dft, HidingValMmcs, HidingChallengeMmcs, SmallRng>;
/// The full STARK config.
pub type ZkConfig = StarkConfig<ZkPcs, Challenge, Challenger>;
