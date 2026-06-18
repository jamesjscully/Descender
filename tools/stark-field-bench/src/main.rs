//! Plonky3 Poseidon2-AIR STARK benchmark across FFT-friendly fields.
//!
//! Goal: empirically compare candidate fields (BabyBear, KoalaBear, Goldilocks)
//! for re-hosting the account-creation STARK, whose prover is dominated by
//! NTT/FRI + Poseidon2 hashing (~37 Poseidon2 permutations, x^5-ish S-box, in a
//! ~1024-row x ~450-col trace).
//!
//! We prove+verify the *standard* Plonky3 Poseidon2 permutation AIR
//! (`VectorizedPoseidon2Air`) sized to a comparable number of permutations and
//! report prove time / verify time / proof size / trace dims / FRI config for
//! each field, using IDENTICAL FRI parameters (blowup, queries, PoW) so the
//! fields are compared apples-to-apples.
//!
//! Merkle/sponge hash for the commitment is Keccak (same across all fields), so
//! the only thing that varies is the prime field + its NTT + Poseidon2 round
//! params + the FRI extension degree (4 for 31-bit fields, 2 for Goldilocks).
//!
//! NOTE on S-box: each field uses its NATIVE Poseidon2 S-box degree from Plonky3
//! (BabyBear x^7, KoalaBear x^3, Goldilocks x^7). The task's relation uses x^5;
//! x^5 is not native to these fields' Plonky3 params, but S-box degree only
//! changes the per-row constraint degree, a second-order effect vs NTT/FRI cost.

use std::time::Instant;

use p3_challenger::{HashChallenger, SerializingChallenger32, SerializingChallenger64};
use p3_commit::ExtensionMmcs;
use p3_field::extension::BinomialExtensionField;
use p3_fri::{FriParameters, TwoAdicFriPcs};
use p3_keccak::{Keccak256Hash, KeccakF, VECTOR_LEN};
use p3_matrix::Matrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_poseidon2_air::{RoundConstants, VectorizedPoseidon2Air};
use p3_symmetric::{CompressionFunctionFromHasher, PaddingFreeSponge, SerializingHasher};
use p3_uni_stark::{prove, verify, StarkConfig};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use p3_baby_bear::{
    BabyBear, GenericPoseidon2LinearLayersBabyBear, BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS,
    BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16, BABYBEAR_S_BOX_DEGREE,
};
use p3_goldilocks::poseidon1::GOLDILOCKS_S_BOX_DEGREE;
use p3_goldilocks::{
    GenericPoseidon2LinearLayersGoldilocks, Goldilocks, GOLDILOCKS_POSEIDON2_HALF_FULL_ROUNDS,
    GOLDILOCKS_POSEIDON2_PARTIAL_ROUNDS_16,
};
use p3_koala_bear::{
    GenericPoseidon2LinearLayersKoalaBear, KoalaBear, KOALABEAR_POSEIDON2_HALF_FULL_ROUNDS,
    KOALABEAR_POSEIDON2_PARTIAL_ROUNDS_16, KOALABEAR_S_BOX_DEGREE,
};

mod batch_bench;
mod c2_air;
mod c2_bench;
mod counting_serializer;
mod full_air;
mod full_bench;
mod prod_bench;
mod zk_full_bench;
use counting_serializer::ByteCounter;

// Poseidon2 permutation width (standard Plonky3 width).
const WIDTH: usize = 16;

// ---- FRI / sizing knobs (IDENTICAL across fields) ----
// log_blowup=1, num_queries=84, query_pow=16 => conjectured FRI soundness
//   = log_blowup*num_queries + query_pow = 84 + 16 = 100 bits.
const LOG_BLOWUP: usize = 1;
const NUM_QUERIES: usize = 84;
const QUERY_POW_BITS: usize = 16;
const COMMIT_POW_BITS: usize = 0;
const TARGET_SOUNDNESS_BITS: usize = LOG_BLOWUP * NUM_QUERIES + QUERY_POW_BITS; // 100

/// Best-of-REPS timing to suppress scheduler/allocator noise.
const REPS: usize = 3;

#[derive(Clone, Copy)]
struct SizeCase {
    name: &'static str,
    num_permutations: usize,
}

fn size_cases() -> Vec<SizeCase> {
    let vlen = VECTOR_LEN.max(1);
    vec![
        SizeCase {
            // ~37 real permutations, padded up to VECTOR_LEN * next-power-of-two
            // (the vectorized AIR requires n/VECTOR_LEN to be a power of two).
            name: "relation-proxy-~37perm",
            num_permutations: (37usize.div_ceil(vlen)).next_power_of_two() * vlen,
        },
        SizeCase {
            name: "scaled-2^10-rows",
            num_permutations: (1usize << 10) * vlen,
        },
        SizeCase {
            name: "scaled-2^12-rows",
            num_permutations: (1usize << 12) * vlen,
        },
        SizeCase {
            name: "scaled-2^14-rows",
            num_permutations: (1usize << 14) * vlen,
        },
    ]
}

struct Report {
    field: &'static str,
    case: &'static str,
    ext_degree: usize,
    sbox_degree: u64,
    num_permutations: usize,
    trace_rows: usize,
    trace_cols: usize,
    prove_ms: f64,
    verify_ms: f64,
    proof_bytes: usize,
}

fn print_report(r: &Report) {
    println!(
        "{:<11}| {:<22}| extΔ{} sbox^{} | perms {:>6} | trace {:>5}x{:<4}| prove {:>9.1} ms | verify {:>7.2} ms | proof {:>9} B",
        r.field,
        r.case,
        r.ext_degree,
        r.sbox_degree,
        r.num_permutations,
        r.trace_rows,
        r.trace_cols,
        r.prove_ms,
        r.verify_ms,
        r.proof_bytes,
    );
}

/// 31-bit field bench (BabyBear / KoalaBear): PrimeField32, ext degree 4.
macro_rules! bench_31bit {
    (
        $field:literal, $Val:ty, $LinLayers:ty,
        $SBOX_DEG:expr, $SBOX_REGS:expr, $HALF_FULL:expr, $PARTIAL:expr, $case:expr
    ) => {{
        type Val = $Val;
        type Challenge = BinomialExtensionField<Val, 4>;

        type ByteHash = Keccak256Hash;
        let byte_hash = ByteHash {};
        type U64Hash = PaddingFreeSponge<KeccakF, 25, 17, 4>;
        let u64_hash = U64Hash::new(KeccakF {});
        type FieldHash = SerializingHasher<U64Hash>;
        let field_hash = FieldHash::new(u64_hash);
        type MyCompress = CompressionFunctionFromHasher<U64Hash, 2, 4>;
        let compress = MyCompress::new(u64_hash);

        type ValMmcs =
            MerkleTreeMmcs<[Val; VECTOR_LEN], [u64; VECTOR_LEN], FieldHash, MyCompress, 2, 4>;
        let val_mmcs = ValMmcs::new(field_hash, compress, 3);
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        type Challenger = SerializingChallenger32<Val, HashChallenger<u8, ByteHash, 32>>;

        let mut rng = SmallRng::seed_from_u64(1);
        let constants = RoundConstants::from_rng(&mut rng);
        let air: VectorizedPoseidon2Air<
            Val,
            $LinLayers,
            WIDTH,
            $SBOX_DEG,
            $SBOX_REGS,
            $HALF_FULL,
            $PARTIAL,
            VECTOR_LEN,
        > = VectorizedPoseidon2Air::new(constants);

        let fri_params = FriParameters {
            log_blowup: LOG_BLOWUP,
            log_final_poly_len: 0,
            max_log_arity: 1,
            num_queries: NUM_QUERIES,
            commit_proof_of_work_bits: COMMIT_POW_BITS,
            query_proof_of_work_bits: QUERY_POW_BITS,
            mmcs: challenge_mmcs,
        };

        let case: SizeCase = $case;
        let trace =
            air.generate_vectorized_trace_rows(case.num_permutations, fri_params.log_blowup);
        let (rows, cols) = (trace.height(), trace.width());

        type Dft = p3_dft::Radix2DitParallel<Val>;
        let dft = Dft::default();
        type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
        let pcs = Pcs::new(dft, val_mmcs, fri_params);
        type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;
        let challenger = Challenger::from_hasher(vec![], byte_hash);
        let config = MyConfig::new(pcs, challenger);

        // Best-of-REPS to reduce timing noise. Re-generate trace each rep since
        // prove() consumes it.
        let mut prove_ms = f64::INFINITY;
        let mut proof = prove(&config, &air, trace, &[]);
        for _ in 0..REPS {
            let tr = air.generate_vectorized_trace_rows(case.num_permutations, LOG_BLOWUP);
            let t0 = Instant::now();
            proof = prove(&config, &air, tr, &[]);
            prove_ms = prove_ms.min(t0.elapsed().as_secs_f64() * 1e3);
        }
        let proof_bytes = bincode_size(&proof);

        let mut verify_ms = f64::INFINITY;
        for _ in 0..REPS {
            let t1 = Instant::now();
            verify(&config, &air, &proof, &[]).expect("verify failed");
            verify_ms = verify_ms.min(t1.elapsed().as_secs_f64() * 1e3);
        }

        Report {
            field: $field,
            case: case.name,
            ext_degree: 4,
            sbox_degree: $SBOX_DEG,
            num_permutations: case.num_permutations,
            trace_rows: rows,
            trace_cols: cols,
            prove_ms,
            verify_ms,
            proof_bytes,
        }
    }};
}

/// Goldilocks bench: PrimeField64, ext degree 2.
fn bench_goldilocks(case: SizeCase) -> Report {
    type Val = Goldilocks;
    // Degree-2 binomial extension: ~128-bit OOD/challenge space over a 64-bit field.
    type Challenge = BinomialExtensionField<Val, 2>;

    type ByteHash = Keccak256Hash;
    let byte_hash = ByteHash {};
    type U64Hash = PaddingFreeSponge<KeccakF, 25, 17, 4>;
    let u64_hash = U64Hash::new(KeccakF {});
    type FieldHash = SerializingHasher<U64Hash>;
    let field_hash = FieldHash::new(u64_hash);
    type MyCompress = CompressionFunctionFromHasher<U64Hash, 2, 4>;
    let compress = MyCompress::new(u64_hash);

    type ValMmcs = MerkleTreeMmcs<[Val; VECTOR_LEN], [u64; VECTOR_LEN], FieldHash, MyCompress, 2, 4>;
    let val_mmcs = ValMmcs::new(field_hash, compress, 3);
    type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
    type Challenger = SerializingChallenger64<Val, HashChallenger<u8, ByteHash, 32>>;

    let mut rng = SmallRng::seed_from_u64(1);
    let constants = RoundConstants::from_rng(&mut rng);
    let air: VectorizedPoseidon2Air<
        Val,
        GenericPoseidon2LinearLayersGoldilocks,
        WIDTH,
        { GOLDILOCKS_S_BOX_DEGREE },
        1, // SBOX_REGISTERS=1 (S-box degree 7 needs a register to bound constraint degree, as BabyBear does)
        GOLDILOCKS_POSEIDON2_HALF_FULL_ROUNDS,
        GOLDILOCKS_POSEIDON2_PARTIAL_ROUNDS_16,
        VECTOR_LEN,
    > = VectorizedPoseidon2Air::new(constants);

    let fri_params = FriParameters {
        log_blowup: LOG_BLOWUP,
        log_final_poly_len: 0,
        max_log_arity: 1,
        num_queries: NUM_QUERIES,
        commit_proof_of_work_bits: COMMIT_POW_BITS,
        query_proof_of_work_bits: QUERY_POW_BITS,
        mmcs: challenge_mmcs,
    };

    let trace = air.generate_vectorized_trace_rows(case.num_permutations, fri_params.log_blowup);
    let (rows, cols) = (trace.height(), trace.width());

    type Dft = p3_dft::Radix2DitParallel<Val>;
    let dft = Dft::default();
    type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
    let pcs = Pcs::new(dft, val_mmcs, fri_params);
    type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;
    let challenger = Challenger::from_hasher(vec![], byte_hash);
    let config = MyConfig::new(pcs, challenger);

    let mut prove_ms = f64::INFINITY;
    let mut proof = prove(&config, &air, trace, &[]);
    for _ in 0..REPS {
        let tr = air.generate_vectorized_trace_rows(case.num_permutations, LOG_BLOWUP);
        let t0 = Instant::now();
        proof = prove(&config, &air, tr, &[]);
        prove_ms = prove_ms.min(t0.elapsed().as_secs_f64() * 1e3);
    }
    let proof_bytes = bincode_size(&proof);

    let mut verify_ms = f64::INFINITY;
    for _ in 0..REPS {
        let t1 = Instant::now();
        verify(&config, &air, &proof, &[]).expect("verify failed");
        verify_ms = verify_ms.min(t1.elapsed().as_secs_f64() * 1e3);
    }

    Report {
        field: "Goldilocks",
        case: case.name,
        ext_degree: 2,
        sbox_degree: GOLDILOCKS_S_BOX_DEGREE,
        num_permutations: case.num_permutations,
        trace_rows: rows,
        trace_cols: cols,
        prove_ms,
        verify_ms,
        proof_bytes,
    }
}

/// Serialized proof size via a length-counting serde serializer (compact binary,
/// roughly what bincode would emit), avoiding an extra dependency.
fn bincode_size<T: serde::Serialize>(value: &T) -> usize {
    let mut counter = ByteCounter(0);
    value
        .serialize(&mut counter)
        .expect("proof serialization failed");
    counter.0
}

fn run_field_set(case: SizeCase, reports: &mut Vec<Report>) {
    eprintln!("[run] case={} BabyBear...", case.name);
    reports.push(bench_31bit!(
        "BabyBear",
        BabyBear,
        GenericPoseidon2LinearLayersBabyBear,
        BABYBEAR_S_BOX_DEGREE,
        1,
        BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS,
        BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16,
        case
    ));
    eprintln!("[run] case={} KoalaBear...", case.name);
    reports.push(bench_31bit!(
        "KoalaBear",
        KoalaBear,
        GenericPoseidon2LinearLayersKoalaBear,
        KOALABEAR_S_BOX_DEGREE,
        0,
        KOALABEAR_POSEIDON2_HALF_FULL_ROUNDS,
        KOALABEAR_POSEIDON2_PARTIAL_ROUNDS_16,
        case
    ));
    eprintln!("[run] case={} Goldilocks...", case.name);
    reports.push(bench_goldilocks(case));
    eprintln!("[run] case={} done.", case.name);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // `c2`  -> run only the C2-obligation AIR bench (fast, default).
    // `perm`-> run only the Poseidon2-permutation baseline bench.
    // `all` -> run both.
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("c2");

    if mode == "c2" {
        c2_bench::run_c2_bench();
        return;
    }
    if mode == "full" {
        full_bench::run_full_bench();
        return;
    }
    if mode == "zk" {
        zk_full_bench::run_zk_full_bench();
        return;
    }
    if mode == "prod" {
        prod_bench::run_prod_bench();
        return;
    }
    if mode == "batch" {
        batch_bench::run_batch_bench();
        return;
    }
    if mode == "all" {
        c2_bench::run_c2_bench();
        full_bench::run_full_bench();
        zk_full_bench::run_zk_full_bench();
        prod_bench::run_prod_bench();
    }

    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    println!("=== Plonky3 Poseidon2-AIR STARK field benchmark ===");
    println!(
        "Poseidon2 WIDTH={WIDTH}, SIMD VECTOR_LEN={VECTOR_LEN} (perms/AIR-row), rayon threads={threads}"
    );
    println!(
        "FRI: log_blowup={LOG_BLOWUP} (blowup {}), num_queries={NUM_QUERIES}, query_pow={QUERY_POW_BITS} bits => conjectured soundness ~{TARGET_SOUNDNESS_BITS} bits",
        1 << LOG_BLOWUP
    );
    println!("31-bit fields: ext degree 4; Goldilocks: ext degree 2. Merkle hash = Keccak (constant across fields).");
    println!();

    // Warm-up (allocator / thread pool) so the first timed run isn't penalized.
    {
        let mut warm = Vec::new();
        run_field_set(
            SizeCase {
                name: "warmup",
                num_permutations: (1usize << 8) * VECTOR_LEN.max(1),
            },
            &mut warm,
        );
    }

    let mut reports = Vec::new();
    for case in size_cases() {
        run_field_set(case, &mut reports);
    }

    println!();
    println!("=== RESULTS (threads={threads}) ===");
    for r in &reports {
        print_report(r);
    }
}
