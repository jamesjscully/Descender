//! FIX #4 — REAL ZK-hiding tests that attack the channel that ACTUALLY hides
//! `account_projection`.
//!
//! HONEST SCOPE: full zero-knowledge of this STARK rests on (a) Plonky3's
//! statistical-ZK argument for the FRI/trace openings (salted-leaf hiding Merkle +
//! HidingFriPcs random codewords + ethSTARK R(X) masking) AND (b) the
//! mask-uniformity of the PUBLIC `masked_projection` channel. These tests
//! DEMONSTRATE (b) empirically — that the per-lane mask is ~uniform over Z_q so the
//! public `masked_projection = account_projection + mask (mod q)` does not leak
//! `account_projection` — and that the default prover's per-proof randomization
//! actually decorrelates two proofs of the same witness. They are NOT a ZK
//! simulator proof.
//!
//! Why the mask channel is the real hiding channel: `masked_projection` is PUBLIC
//! and equals `account_projection[j] + masks[j] (mod q12289)` where
//! `masks = Poseidon2-PRF(MASK ‖ pad_seed ‖ slot) mod q12289`. So `account_projection`
//! hiding rests on the MASK being ~uniform over Z_q (a one-time pad mod q), NOT on
//! FRI opening hiding. If the mask is ~uniform, `masked_projection` is ~uniform and
//! independent of `account_projection`, so `account_projection` is unrecoverable
//! from the public statement. The residual non-uniformity is the
//! six-lane wide reduction from KoalaBear outputs. After charging all six mask
//! lanes and the selected 2^32-account horizon, that reduction term remains
//! below 2^-137.

use descender_account_stark::{
    padseed_witness_column, prove_account_creation, prove_account_creation_with_seed,
    AccountCreationSecrets, Challenge, Val, N_LANES,
};
use p3_field::{PrimeCharacteristicRing, PrimeField32};

const Q: u32 = 12289;

/// A witness with FIXED `account_projection` and everything else parameterized by
/// `seed` (so the C2 PRF mask = PRF(pad_seed ‖ slot) varies, but the projection it
/// masks is held constant — exactly the hiding hypothesis).
fn fixed_projection_secrets(seed: u64, fixed_proj: [u16; N_LANES]) -> AccountCreationSecrets {
    let b = |salt: u64| -> u8 { (seed.wrapping_mul(0x9E37_79B9).wrapping_add(salt) & 0xff) as u8 };
    AccountCreationSecrets {
        pad_seed: core::array::from_fn(|i| b(i as u64)),
        slot_id: seed.wrapping_mul(0x100_0193).wrapping_add(1),
        account_projection: fixed_proj, // HELD CONSTANT across all samples
        lifecycle_anchor: core::array::from_fn(|i| b(100 + i as u64)),
        c1_nonce: core::array::from_fn(|i| b(50 + i as u64)),
        c6_nonce: core::array::from_fn(|i| b(70 + i as u64)),
        account_id: core::array::from_fn(|i| b(150 + i as u64)),
        verifier_version: seed.wrapping_mul(0x1234_5) as u32,
        account_state_root: core::array::from_fn(|i| b(200 + i as u64)),
        syndrome: core::array::from_fn(|i| {
            (seed.wrapping_mul(13).wrapping_add(i as u64) % 12289) as u16
        }),
        checked_core_key_commitment: [0x2bu8; 32],
    }
}

// ===========================================================================
// FIX #4a — MASK-CHANNEL UNIFORMITY. Fix account_projection; vary pad_seed/slot
// over many samples; recover the per-lane mask = (masked_projection -
// account_projection) mod q and chi-square test it for uniformity over Z_q.
// If the mask is ~uniform, masked_projection is ~uniform and account_projection
// is not recoverable from the public statement.
// ===========================================================================
#[test]
fn mask_channel_is_uniform_over_zq() {
    let fixed_proj: [u16; N_LANES] = [11, 2222, 333, 4444, 555, 12288];
    let n_samples = 4000usize;

    // Collect masks pooled over all 6 lanes (PRF outputs are independent per lane).
    let n_bins = 64usize; // chi-square bins over [0, q)
    let mut counts = vec![0u64; n_bins];
    let mut total = 0u64;

    for seed in 0..n_samples as u64 {
        let s = fixed_projection_secrets(seed, fixed_proj);
        // Deterministic mask seed: ZK-opening randomness is irrelevant to the
        // PUBLIC masked_projection (the mask channel), so any value is fine.
        let (public, _proof) = prove_account_creation_with_seed(&s, 0xC0FFEE).expect("prove");
        for j in 0..N_LANES {
            let masked = public.masked_projection[j].as_canonical_u32();
            let acc = fixed_proj[j] as u32;
            let mask = (masked + Q - acc % Q) % Q; // (masked - acc) mod q
            assert!(mask < Q, "recovered mask out of range");
            let bin = (mask as u64 * n_bins as u64 / Q as u64) as usize;
            counts[bin.min(n_bins - 1)] += 1;
            total += 1;
        }
    }

    // Pearson chi-square against uniform. Bins are near-equal width (q not divisible
    // by n_bins); use per-bin expected = (bin_width/q)*total.
    let mut chi2 = 0.0f64;
    for (bi, &c) in counts.iter().enumerate() {
        let lo = (bi as u64 * Q as u64 / n_bins as u64) as f64;
        let hi = ((bi as u64 + 1) * Q as u64 / n_bins as u64) as f64;
        let width = hi - lo;
        let expected = width / Q as f64 * total as f64;
        let d = c as f64 - expected;
        chi2 += d * d / expected;
    }
    let dof = n_bins - 1;
    // 99.9% critical value for chi-square with 63 dof ~ 103.4. Use a generous
    // bound to avoid flakiness while still catching gross non-uniformity.
    let crit = 120.0;
    println!(
        "[FIX4a mask uniformity] samples(per-lane pooled)={total} bins={n_bins} chi2={chi2:.2} (dof={dof}, crit@~99.9%={crit})"
    );
    assert!(
        chi2 < crit,
        "MASK NOT UNIFORM: chi2={chi2:.2} >= {crit} — masked_projection would leak account_projection"
    );

    // Sanity: the mean recovered mask should sit near q/2.
    let mean = {
        let mut sum = 0u128;
        for seed in 0..16u64 {
            let s = fixed_projection_secrets(seed + 99_000, fixed_proj);
            let (p, _) = prove_account_creation_with_seed(&s, 7).unwrap();
            for j in 0..N_LANES {
                let masked = p.masked_projection[j].as_canonical_u32();
                let mask = (masked + Q - (fixed_proj[j] as u32) % Q) % Q;
                sum += mask as u128;
            }
        }
        sum as f64 / (16.0 * N_LANES as f64)
    };
    println!("[FIX4a] mean recovered mask = {mean:.0} (q/2 = {})", Q / 2);
}

// ===========================================================================
// FIX #4b — masked_projection is NOT constant for fixed account_projection.
// A direct corollary: if the mask were fixed/degenerate, masked_projection would
// repeat. Show it takes many distinct values across samples (the pad is live).
// ===========================================================================
#[test]
fn masked_projection_varies_for_fixed_account_projection() {
    let fixed_proj: [u16; N_LANES] = [7, 7, 7, 7, 7, 7];
    let mut seen = std::collections::HashSet::new();
    let n = 200usize;
    for seed in 0..n as u64 {
        let s = fixed_projection_secrets(seed, fixed_proj);
        let (public, _) = prove_account_creation_with_seed(&s, 1).unwrap();
        let lane0 = public.masked_projection[0].as_canonical_u32();
        seen.insert(lane0);
    }
    println!(
        "[FIX4b] distinct masked_projection[0] over {n} samples (fixed acc_proj) = {}",
        seen.len()
    );
    // With a ~uniform mask over ~12289 values, 200 samples should yield well over
    // 150 distinct values (birthday-collision expectation ~198).
    assert!(
        seen.len() > 150,
        "masked_projection barely varies for fixed account_projection — mask channel degenerate"
    );
}

// ===========================================================================
// FIX #4c — FRESH-RANDOMNESS DECORRELATION (default deployment path).
// Two proofs of the SAME witness+context via the DEFAULT prover (which now draws
// fresh CSPRNG randomness per call) must be decorrelated: their ZK-opening bytes
// must differ. The fixed-seed path is NOT decorrelated (covered by 4d).
// ===========================================================================
#[test]
fn default_path_decorrelates_proofs_of_same_witness() {
    let s = fixed_projection_secrets(42, [11, 2222, 333, 4444, 555, 12288]);

    let (pub_a, proof_a) = prove_account_creation(&s).expect("prove a (fresh rng)");
    let (pub_b, proof_b) = prove_account_creation(&s).expect("prove b (fresh rng)");

    // Same witness+context -> identical PUBLIC statement.
    assert_eq!(
        pub_a, pub_b,
        "same witness must yield same public statement"
    );

    // But the ZK randomness must differ: opened trace values and/or trace
    // commitment must not be byte-identical across the two fresh-randomness proofs.
    let opened_differ = proof_a.opened_values.trace_local != proof_b.opened_values.trace_local;
    let commit_differ = proof_a.commitments.trace != proof_b.commitments.trace;
    println!(
        "[FIX4c default-path decorrelation] opened_differ={opened_differ} commit_differ={commit_differ}"
    );
    assert!(
        opened_differ || commit_differ,
        "DEFAULT PATH NOT HIDING ACROSS PROOFS: two proofs of the same witness are byte-identical (no fresh randomness)"
    );

    // Stronger: the opened pad_seed column must differ across the two fresh proofs
    // (randomized opening), and never equal the raw witness byte.
    let k = 7usize;
    let col = padseed_witness_column(k);
    let target = Challenge::from(Val::from_u32(s.pad_seed[k] as u32));
    let ra = proof_a.opened_values.trace_local[col];
    let rb = proof_b.opened_values.trace_local[col];
    assert_ne!(ra, target, "opened pad_seed byte recovered (proof a)");
    assert_ne!(rb, target, "opened pad_seed byte recovered (proof b)");
    assert_ne!(
        ra, rb,
        "opened pad_seed column identical across fresh proofs (not randomized)"
    );
}

// ===========================================================================
// FIX #4d — FIXED-SEED DETERMINISM (the _with_seed variant kept for tests).
// Same witness + SAME seed must be byte-identical (deterministic), confirming the
// _with_seed path is the reproducible one (and, by contrast, why the default path
// must NOT use a fixed seed).
// ===========================================================================
#[test]
fn fixed_seed_path_is_deterministic() {
    let s = fixed_projection_secrets(42, [11, 2222, 333, 4444, 555, 12288]);
    let (pa, proof_a) = prove_account_creation_with_seed(&s, 0xDEADBEEF).expect("a");
    let (pb, proof_b) = prove_account_creation_with_seed(&s, 0xDEADBEEF).expect("b");
    assert_eq!(pa, pb, "same witness -> same public");
    assert_eq!(
        proof_a.opened_values.trace_local, proof_b.opened_values.trace_local,
        "FIXED-SEED NOT DETERMINISTIC: opened trace differs under identical seed"
    );
    assert_eq!(
        proof_a.commitments.trace, proof_b.commitments.trace,
        "FIXED-SEED NOT DETERMINISTIC: trace commitment differs under identical seed"
    );
    println!("[FIX4d] fixed-seed path is byte-deterministic under identical seed (as required for tests)");
}

// ===========================================================================
// FIX #4e — INJECTED-LEAK NEGATIVE CONTROL. Feed the same chi-square
// threshold a deliberately degenerate mask distribution. This does not exercise
// the prover; it proves the harness would fail closed if the mask channel were
// constant or similarly broken.
// ===========================================================================
#[test]
fn injected_mask_leak_negative_control_fails_chi_square() {
    let n_bins = 64usize;
    let total = 4000u64 * N_LANES as u64;
    let mut counts = vec![0u64; n_bins];
    counts[0] = total;

    let mut chi2 = 0.0f64;
    for (bi, &c) in counts.iter().enumerate() {
        let lo = (bi as u64 * Q as u64 / n_bins as u64) as f64;
        let hi = ((bi as u64 + 1) * Q as u64 / n_bins as u64) as f64;
        let width = hi - lo;
        let expected = width / Q as f64 * total as f64;
        let d = c as f64 - expected;
        chi2 += d * d / expected;
    }

    let crit = 120.0;
    println!("[FIX4e injected leak negative control] chi2={chi2:.2} crit={crit}");
    assert!(
        chi2 >= crit,
        "INJECTED LEAK NOT DETECTED: constant mask distribution stayed below chi-square threshold"
    );
}
