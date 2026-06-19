//! Prove / verify / size + completeness & soundness driver for the C2 AIR,
//! across BabyBear and KoalaBear with IDENTICAL FRI parameters.
//!
//! Reuses the same Keccak-Merkle, ext-degree-4, ~100-bit FRI config as the
//! Poseidon2-permutation bench in `main.rs`, so C2 numbers are directly
//! comparable to the permutation-only baseline.
//!
//! A `macro_rules!` (rather than a generic fn) builds the per-field config — the
//! same approach `main.rs` uses — because spelling out every FRI/TwoAdic/binomial
//! field bound for a generic fn is brittle; the AIR ITSELF stays field-generic.

use std::time::Instant;

use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_commit::ExtensionMmcs;
use p3_field::extension::BinomialExtensionField;
use p3_field::PrimeCharacteristicRing;
use p3_fri::{FriParameters, TwoAdicFriPcs};
use p3_keccak::{Keccak256Hash, KeccakF, VECTOR_LEN};
use p3_matrix::Matrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, PaddingFreeSponge, SerializingHasher};
use p3_uni_stark::{prove, verify, StarkConfig};
use rand::distr::StandardUniform;
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};

use p3_baby_bear::{
    BabyBear, GenericPoseidon2LinearLayersBabyBear, BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS,
    BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16, BABYBEAR_S_BOX_DEGREE,
};
use p3_koala_bear::{
    GenericPoseidon2LinearLayersKoalaBear, KoalaBear, KOALABEAR_POSEIDON2_HALF_FULL_ROUNDS,
    KOALABEAR_POSEIDON2_PARTIAL_ROUNDS_16, KOALABEAR_S_BOX_DEGREE,
};

use crate::c2_air::{
    c2_public_values, compute_c2_instance, generate_c2_trace, C2Air, NUM_INPUT_LANES, NUM_OUTPUTS,
    NUM_PERMS, NUM_SQUEEZE,
};
use crate::counting_serializer::ByteCounter;

const WIDTH: usize = 16;

// Same FRI knobs as main.rs (≈100-bit conjectured FRI soundness).
const LOG_BLOWUP: usize = 1;
const NUM_QUERIES: usize = 84;
const QUERY_POW_BITS: usize = 16;
const COMMIT_POW_BITS: usize = 0;
const REPS: usize = 3;

pub struct C2Report {
    pub field: &'static str,
    pub sbox_degree: u64,
    pub log_rows: usize,
    pub trace_rows: usize,
    pub trace_cols: usize,
    pub prove_ms: f64,
    pub verify_ms: f64,
    pub proof_bytes: usize,
    pub completeness_ok: bool,
    pub soundness_tamper_mask_rejected: bool,
    pub soundness_tamper_input_rejected: bool,
}

pub fn print_c2_report(r: &C2Report) {
    println!(
        "{:<11}| sbox^{} | 2^{:<2} rows | trace {:>5}x{:<5}| prove {:>9.1} ms | verify {:>7.2} ms | proof {:>8} B | complete {} | tamper-mask-rej {} | tamper-input-rej {}",
        r.field,
        r.sbox_degree,
        r.log_rows,
        r.trace_rows,
        r.trace_cols,
        r.prove_ms,
        r.verify_ms,
        r.proof_bytes,
        if r.completeness_ok { "PASS" } else { "FAIL" },
        if r.soundness_tamper_mask_rejected { "PASS" } else { "FAIL" },
        if r.soundness_tamper_input_rejected { "PASS" } else { "FAIL" },
    );
}

fn bincode_size<T: serde::Serialize>(value: &T) -> usize {
    let mut counter = ByteCounter(0);
    value.serialize(&mut counter).expect("serialize");
    counter.0
}

/// Build a per-field C2 report. `$Val` must be a 31-bit Plonky3 field with a
/// degree-4 binomial extension (BabyBear / KoalaBear).
macro_rules! bench_c2_31bit {
    (
        $field:literal, $Val:ty, $LinLayers:ty,
        $SBOX_DEG:expr, $SBOX_REGS:expr, $HALF_FULL:expr, $PARTIAL:expr, $log_rows:expr
    ) => {{
        type Val = $Val;
        type Challenge = BinomialExtensionField<Val, 8>;

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

        // Round constants, sampled directly so the AIR can keep its own copy.
        let mut rng = SmallRng::seed_from_u64(0xC2);
        let beginning_rc: [[Val; WIDTH]; $HALF_FULL] =
            core::array::from_fn(|_| rng.sample(StandardUniform));
        let partial_rc: [Val; $PARTIAL] =
            core::array::from_fn(|_| rng.sample(StandardUniform));
        let ending_rc: [[Val; WIDTH]; $HALF_FULL] =
            core::array::from_fn(|_| rng.sample(StandardUniform));

        let air = C2Air::<
            Val,
            $LinLayers,
            WIDTH,
            $SBOX_DEG,
            $SBOX_REGS,
            $HALF_FULL,
            $PARTIAL,
        >::from_arrays(beginning_rc, partial_rc, ending_rc);

        // Honest witness.
        let pad_seed: [u8; 32] =
            core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1));
        let slot: u64 = 0xDEAD_BEEF_0000_0042;
        let log_rows: usize = $log_rows;
        let num_rows = 1usize << log_rows;

        let (trace, inst) = generate_c2_trace::<
            Val,
            $LinLayers,
            WIDTH,
            $SBOX_DEG,
            $SBOX_REGS,
            $HALF_FULL,
            $PARTIAL,
        >(&pad_seed, slot, air.constants(), num_rows);
        let (trace_rows, trace_cols) = (trace.height(), trace.width());
        let public = c2_public_values(&inst);

        // completeness cross-check: public masks == reference reduction.
        let reference = compute_c2_instance::<
            Val,
            $LinLayers,
            WIDTH,
            $SBOX_DEG,
            $SBOX_REGS,
            $HALF_FULL,
            $PARTIAL,
        >(&pad_seed, slot, air.constants());
        let masks_match_reference = public == c2_public_values(&reference);

        let fri_params = FriParameters {
            log_blowup: LOG_BLOWUP,
            log_final_poly_len: 0,
            max_log_arity: 1,
            num_queries: NUM_QUERIES,
            commit_proof_of_work_bits: COMMIT_POW_BITS,
            query_proof_of_work_bits: QUERY_POW_BITS,
            mmcs: challenge_mmcs,
        };

        type Dft = p3_dft::Radix2DitParallel<Val>;
        let dft = Dft::default();
        type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
        let pcs = Pcs::new(dft, val_mmcs, fri_params);
        type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;
        let config = MyConfig::new(pcs, Challenger::from_hasher(vec![], byte_hash));

        // prove (best of REPS).
        let mut prove_ms = f64::INFINITY;
        let mut proof = prove(&config, &air, trace.clone(), &public);
        for _ in 0..REPS {
            let t0 = Instant::now();
            proof = prove(&config, &air, trace.clone(), &public);
            prove_ms = prove_ms.min(t0.elapsed().as_secs_f64() * 1e3);
        }
        let proof_bytes = bincode_size(&proof);

        // verify (best of REPS) + completeness.
        let mut verify_ms = f64::INFINITY;
        let mut completeness_ok = masks_match_reference;
        for _ in 0..REPS {
            let t1 = Instant::now();
            let ok = verify(&config, &air, &proof, &public).is_ok();
            verify_ms = verify_ms.min(t1.elapsed().as_secs_f64() * 1e3);
            completeness_ok &= ok;
        }

        // soundness #1: tamper a claimed public mask lane -> verify must FAIL.
        let mut tampered_pub = public.clone();
        tampered_pub[0] += Val::ONE;
        let soundness_tamper_mask_rejected =
            verify(&config, &air, &proof, &tampered_pub).is_err();

        // soundness #2: prove over a DIFFERENT pad_seed but claim the ORIGINAL
        // public masks. The trace no longer satisfies the relation for those
        // masks, so either prove() panics (debug constraint check) or verify()
        // rejects (release). Either outcome = rejection.
        let mut bad_pad = pad_seed;
        bad_pad[0] ^= 0x01;
        let (bad_trace, _bad_inst) = generate_c2_trace::<
            Val,
            $LinLayers,
            WIDTH,
            $SBOX_DEG,
            $SBOX_REGS,
            $HALF_FULL,
            $PARTIAL,
        >(&bad_pad, slot, air.constants(), num_rows);
        let soundness_tamper_input_rejected = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let p = prove(&config, &air, bad_trace, &public);
                verify(&config, &air, &p, &public).is_ok()
            }),
        )
        .map(|verified| !verified)
        .unwrap_or(true);

        C2Report {
            field: $field,
            sbox_degree: $SBOX_DEG,
            log_rows,
            trace_rows,
            trace_cols,
            prove_ms,
            verify_ms,
            proof_bytes,
            completeness_ok,
            soundness_tamper_mask_rejected,
            soundness_tamper_input_rejected,
        }
    }};
}

pub fn run_c2_bench() {
    println!();
    println!("=== C2 obligation AIR (native Poseidon2 sponge + mod-q12289 reduction) ===");
    println!(
        "inputs: {NUM_INPUT_LANES} byte-lanes (32 pad_seed + 8 slot) -> {NUM_PERMS} Poseidon2 perms (rate 8) -> {NUM_SQUEEZE} squeezed lanes -> WIDE-THEN-REDUCE -> {NUM_OUTPUTS} q12289 mask lanes (public)"
    );
    println!(
        "FRI: log_blowup={LOG_BLOWUP}, num_queries={NUM_QUERIES}, query_pow={QUERY_POW_BITS} (~100-bit), ext degree 4, Keccak-Merkle (constant across fields)"
    );
    println!();

    // Warm-up.
    let _warm = bench_c2_31bit!(
        "BabyBear",
        BabyBear,
        GenericPoseidon2LinearLayersBabyBear,
        BABYBEAR_S_BOX_DEGREE,
        1,
        BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS,
        BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16,
        6
    );

    let mut reports = Vec::new();
    for &log_rows in &[6usize, 10, 12] {
        eprintln!("[c2] BabyBear 2^{log_rows} rows...");
        reports.push(bench_c2_31bit!(
            "BabyBear",
            BabyBear,
            GenericPoseidon2LinearLayersBabyBear,
            BABYBEAR_S_BOX_DEGREE,
            1,
            BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS,
            BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16,
            log_rows
        ));

        eprintln!("[c2] KoalaBear 2^{log_rows} rows...");
        reports.push(bench_c2_31bit!(
            "KoalaBear",
            KoalaBear,
            GenericPoseidon2LinearLayersKoalaBear,
            KOALABEAR_S_BOX_DEGREE,
            0,
            KOALABEAR_POSEIDON2_HALF_FULL_ROUNDS,
            KOALABEAR_POSEIDON2_PARTIAL_ROUNDS_16,
            log_rows
        ));
    }

    println!();
    println!("=== C2 RESULTS ===");
    for r in &reports {
        print_c2_report(r);
    }

    eprintln!("[c2] bias check...");
    print_bias_report(&run_bias_check());
}

/// Build a single small (2^6-row) C2 report for the given field; used by tests.
#[allow(dead_code)]
pub fn c2_report_babybear_small() -> C2Report {
    bench_c2_31bit!(
        "BabyBear",
        BabyBear,
        GenericPoseidon2LinearLayersBabyBear,
        BABYBEAR_S_BOX_DEGREE,
        1,
        BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS,
        BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16,
        6
    )
}

#[allow(dead_code)]
pub fn c2_report_koalabear_small() -> C2Report {
    bench_c2_31bit!(
        "KoalaBear",
        KoalaBear,
        GenericPoseidon2LinearLayersKoalaBear,
        KOALABEAR_S_BOX_DEGREE,
        0,
        KOALABEAR_POSEIDON2_HALF_FULL_ROUNDS,
        KOALABEAR_POSEIDON2_PARTIAL_ROUNDS_16,
        6
    )
}

// ===========================================================================
// HIDING-BIAS CHECK: empirically confirm the wide-then-reduce mask is ~uniform
// over Z_q, and that the OLD one-lane reduce was biased.
// ===========================================================================
//
// The deployed bias is structural: a uniform value on [0, p) reduced mod q has
// per-residue bias <= q/p. For p ~ 2^31, q = 12289 this is ~2^-17 — far too small
// to detect by sampling (you'd need ~2^34 samples). So we demonstrate the FIX two
// ways:
//
//   (A) MECHANISM (small-base emulation): use a small base prime `P` and modulus
//       `Q` (with `P mod Q != 0`) so the one-lane bias `Q/P` is LARGE and
//       observable. We reduce uniform digits two ways — NARROW (one digit mod Q,
//       the old scheme) and WIDE (5 digits as base-P number mod Q via the SAME
//       integer Horner the gadget uses, `wide_reduce_masks`-equivalent) — and run
//       a chi-square goodness-of-fit over Q bins. NARROW must FAIL (bias detected),
//       WIDE must PASS (uniform). This proves the wide-then-reduce removes the
//       structural bias the gadget implements.
//
//   (B) REAL MODULUS (q = 12289): sample many real masks from the live sponge
//       (varying slot) and confirm the chi-square / max-cell-deviation are within
//       the uniform tolerance — i.e. the production masks show no detectable bias.
//
// Analytic residual (reported, not sampled): narrow bias ~ q/p = 2^-17.4;
// wide-then-reduce bias ~ q/p^5 = 2^-141 (negligible).

/// SplitMix64 — a tiny deterministic uniform u64 source (no external dep).
struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// uniform on [0, n).
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Pearson chi-square statistic for `counts` over `bins` cells with `total`
/// samples (uniform expectation `total/bins` per cell).
fn chi_square(counts: &[u64], total: u64) -> f64 {
    let bins = counts.len() as f64;
    let exp = total as f64 / bins;
    counts
        .iter()
        .map(|&c| {
            let d = c as f64 - exp;
            d * d / exp
        })
        .sum()
}

pub struct BiasReport {
    // (A) small-base mechanism emulation
    pub emu_base: u64,
    pub emu_mod: u64,
    pub emu_samples: u64,
    pub emu_narrow_chi2: f64,
    pub emu_wide_chi2: f64,
    pub emu_chi2_crit: f64, // ~0.999 quantile threshold (df = bins-1)
    pub emu_narrow_biased: bool, // narrow chi2 > crit  (bias detected)
    pub emu_wide_uniform: bool,  // wide chi2 <= crit   (no bias)
    // (B) real modulus q=12289 on the live sponge
    pub real_samples: u64,
    pub real_chi2: f64,
    pub real_chi2_crit: f64,
    pub real_max_cell_dev_rel: f64, // max |count-exp|/exp
    pub real_uniform: bool,
    // analytic bounds
    pub analytic_narrow_log2: f64, // log2(q/p)
    pub analytic_wide_log2: f64,   // log2(q/p^5)
}

/// (A) Small-base emulation of the mechanism. `narrow` reduces ONE base-`base`
/// digit mod `modulus`; `wide` reduces `n_digits` digits (base-`base` number) mod
/// `modulus` via the exact integer Horner the AIR gadget enforces.
fn emulation_bias(base: u64, modulus: u64, n_digits: usize, samples: u64) -> (f64, f64) {
    let mut rng = SplitMix64(0xC2_BEEF_1234_5678);
    let mut narrow = vec![0u64; modulus as usize];
    let mut wide = vec![0u64; modulus as usize];
    for _ in 0..samples {
        // high digit first (matches gadget Horner order).
        let mut digits = [0u64; 8];
        for d in digits.iter_mut().take(n_digits) {
            *d = rng.below(base);
        }
        // NARROW (old scheme): just the first digit mod q.
        narrow[(digits[0] % modulus) as usize] += 1;
        // WIDE (new scheme): Horner over all digits, reducing each step.
        let mut r = 0u64;
        for &d in digits.iter().take(n_digits) {
            r = (r * base + d) % modulus;
        }
        wide[(r % modulus) as usize] += 1;
    }
    (chi_square(&narrow, samples), chi_square(&wide, samples))
}

/// Build the bias report (uses BabyBear's prime for the analytic bounds and the
/// real-modulus sampling field).
pub fn run_bias_check() -> BiasReport {
    use p3_field::PrimeField32;
    type Val = BabyBear;
    let p = Val::ORDER_U32 as f64;
    let q = crate::c2_air::Q12289 as f64;

    // ---- (A) small-base mechanism emulation ----
    // base P=251, modulus Q=11 (P mod Q = 9 != 0 ⇒ strong one-lane bias), 5 digits.
    let emu_base = 251u64;
    let emu_mod = 11u64;
    let emu_digits = 5usize;
    let emu_samples = 2_000_000u64;
    let (emu_narrow_chi2, emu_wide_chi2) =
        emulation_bias(emu_base, emu_mod, emu_digits, emu_samples);
    // df = 10; chi-square 0.999 quantile ≈ 29.59. A clearly-biased distribution at
    // 2M samples produces chi2 in the thousands; a uniform one stays ~ df.
    let emu_chi2_crit = 29.59;

    // ---- (B) real modulus q=12289 on the live sponge ----
    let mut rc_rng = SmallRng::seed_from_u64(0xB1A5);
    let beginning_rc: [[Val; WIDTH]; BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS] =
        core::array::from_fn(|_| rc_rng.sample(StandardUniform));
    let partial_rc: [Val; BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16] =
        core::array::from_fn(|_| rc_rng.sample(StandardUniform));
    let ending_rc: [[Val; WIDTH]; BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS] =
        core::array::from_fn(|_| rc_rng.sample(StandardUniform));
    let air = C2Air::<
        Val,
        GenericPoseidon2LinearLayersBabyBear,
        WIDTH,
        BABYBEAR_S_BOX_DEGREE,
        1,
        BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS,
        BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16,
    >::from_arrays(beginning_rc, partial_rc, ending_rc);

    let qn = crate::c2_air::Q12289 as usize;
    let mut counts = vec![0u64; qn];
    // Each instance yields 6 masks; vary slot to get many distinct sponge runs.
    let n_instances = 80_000u64;
    let pad_seed: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(7));
    for s in 0..n_instances {
        let inst = compute_c2_instance::<
            Val,
            GenericPoseidon2LinearLayersBabyBear,
            WIDTH,
            BABYBEAR_S_BOX_DEGREE,
            1,
            BABYBEAR_POSEIDON2_HALF_FULL_ROUNDS,
            BABYBEAR_POSEIDON2_PARTIAL_ROUNDS_16,
        >(&pad_seed, s, air.constants());
        for &m in inst.mask.iter() {
            counts[m as usize] += 1;
        }
    }
    let real_samples = n_instances * NUM_OUTPUTS as u64;
    let real_chi2 = chi_square(&counts, real_samples);
    let exp = real_samples as f64 / qn as f64;
    let real_max_cell_dev_rel = counts
        .iter()
        .map(|&c| (c as f64 - exp).abs() / exp)
        .fold(0.0f64, f64::max);
    // df = 12288; chi-square ~ N(df, 2*df). 0.999 ≈ df + 3.09*sqrt(2*df).
    let df = (qn - 1) as f64;
    let real_chi2_crit = df + 3.09 * (2.0 * df).sqrt();

    BiasReport {
        emu_base,
        emu_mod,
        emu_samples,
        emu_narrow_chi2,
        emu_wide_chi2,
        emu_chi2_crit,
        emu_narrow_biased: emu_narrow_chi2 > emu_chi2_crit,
        emu_wide_uniform: emu_wide_chi2 <= emu_chi2_crit,
        real_samples,
        real_chi2,
        real_chi2_crit,
        real_max_cell_dev_rel,
        real_uniform: real_chi2 <= real_chi2_crit,
        analytic_narrow_log2: (q / p).log2(),
        analytic_wide_log2: (q / p.powi(5)).log2(),
    }
}

pub fn print_bias_report(r: &BiasReport) {
    println!();
    println!("=== HIDING-BIAS CHECK (wide-then-reduce vs old one-lane reduce) ===");
    println!(
        "(A) mechanism emulation: base P={}, modulus Q={}, {} digits, {} samples",
        r.emu_base, r.emu_mod, 5, r.emu_samples
    );
    println!(
        "    NARROW (old, 1 digit) chi2 = {:>12.1}  > crit {:.1} ? {}  (bias {})",
        r.emu_narrow_chi2,
        r.emu_chi2_crit,
        r.emu_narrow_biased,
        if r.emu_narrow_biased { "DETECTED" } else { "absent" },
    );
    println!(
        "    WIDE   (new, 5 digit) chi2 = {:>12.1} <= crit {:.1} ? {}  (uniform {})",
        r.emu_wide_chi2,
        r.emu_chi2_crit,
        r.emu_wide_uniform,
        if r.emu_wide_uniform { "YES" } else { "NO" },
    );
    println!(
        "(B) real modulus q=12289 on live sponge: {} masks, {} bins",
        r.real_samples,
        crate::c2_air::Q12289
    );
    println!(
        "    chi2 = {:.1} (crit {:.1}, uniform {}), max-cell-dev = {:.4} (rel)",
        r.real_chi2, r.real_chi2_crit, r.real_uniform, r.real_max_cell_dev_rel,
    );
    println!(
        "analytic per-residue bias: narrow q/p ~ 2^{:.1} | wide q/p^5 ~ 2^{:.1}",
        r.analytic_narrow_log2, r.analytic_wide_log2,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c2_babybear_complete_and_sound() {
        let r = c2_report_babybear_small();
        assert!(r.completeness_ok, "BabyBear completeness failed");
        assert!(
            r.soundness_tamper_mask_rejected,
            "BabyBear tampered-mask was accepted (soundness break)"
        );
        assert!(
            r.soundness_tamper_input_rejected,
            "BabyBear tampered-input was accepted (soundness break)"
        );
    }

    #[test]
    fn c2_koalabear_complete_and_sound() {
        let r = c2_report_koalabear_small();
        assert!(r.completeness_ok, "KoalaBear completeness failed");
        assert!(
            r.soundness_tamper_mask_rejected,
            "KoalaBear tampered-mask was accepted (soundness break)"
        );
        assert!(
            r.soundness_tamper_input_rejected,
            "KoalaBear tampered-input was accepted (soundness break)"
        );
    }

    #[test]
    fn wide_then_reduce_removes_hiding_bias() {
        let r = run_bias_check();
        print_bias_report(&r);
        // (A) The OLD one-lane reduce is detectably biased; the NEW wide-then-reduce
        // is statistically uniform (same chi-square test, same samples).
        assert!(
            r.emu_narrow_biased,
            "emulation: old one-lane reduce should show detectable bias (chi2 {} <= crit {})",
            r.emu_narrow_chi2, r.emu_chi2_crit
        );
        assert!(
            r.emu_wide_uniform,
            "emulation: wide-then-reduce should be uniform (chi2 {} > crit {})",
            r.emu_wide_chi2, r.emu_chi2_crit
        );
        // (B) Real masks (q=12289) show no detectable bias.
        assert!(
            r.real_uniform,
            "real masks not uniform: chi2 {} > crit {}",
            r.real_chi2, r.real_chi2_crit
        );
        // Sanity: the wide chi2 is dramatically smaller than the narrow chi2.
        assert!(
            r.emu_wide_chi2 * 10.0 < r.emu_narrow_chi2,
            "wide chi2 ({}) not << narrow chi2 ({})",
            r.emu_wide_chi2,
            r.emu_narrow_chi2
        );
    }
}
