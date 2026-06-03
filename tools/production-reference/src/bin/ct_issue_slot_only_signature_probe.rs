use descender_production_reference::*;
use serde::Serialize;
use std::hint::black_box;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_SAMPLES: u64 = 25;
const DEFAULT_BATCH_ITERATIONS: u64 = 10_000;
const EQUALIZED_CANONICAL_DECODE_PASSES: usize = 1;

#[derive(Clone, Debug, Serialize)]
struct ProbeRow {
    group: &'static str,
    class: &'static str,
    operation: &'static str,
    expected_outcome: &'static str,
    samples: u64,
    batch_iterations: u64,
    avg_ns_samples: Vec<u128>,
    min_avg_ns: u128,
    max_avg_ns: u128,
    mean_avg_ns: f64,
    median_avg_ns: f64,
    ok_batches: u64,
    observed_outcome: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct RatioRow {
    numerator_class: &'static str,
    denominator_class: &'static str,
    numerator_mean_ns: f64,
    denominator_mean_ns: f64,
    mean_ratio: f64,
    numerator_median_ns: f64,
    denominator_median_ns: f64,
    median_ratio: f64,
}

#[derive(Clone, Debug, Serialize)]
struct ProbeReport {
    schema: &'static str,
    generated_at_unix_nanos: u128,
    samples: u64,
    batch_iterations: u64,
    signature_bytes: usize,
    slot_id_bytes: usize,
    caveat: &'static str,
    root_cause_probe: &'static str,
    rows: Vec<ProbeRow>,
    ratios: Vec<RatioRow>,
}

fn test_signature(slot_id: u32) -> ProductionSignature {
    encode_production_signature(
        slot_id,
        &[0_u8; CHALLENGE_BYTES],
        &ProductionCompactResponses {
            active_free: [[0_i16; LOCAL_FREE]; REPS],
        },
        &ProductionLocalOpenings {
            active_free: [[0_u16; LOCAL_FREE]; REPS],
        },
        &ProductionLinearSigmaTargetProof {
            proof_commitment: [0_u8; DIGEST_BYTES],
            masked_witness_response: [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
        },
    )
    .expect("zero test signature is canonical")
}

fn sample_avg_ns<F>(batch_iterations: u64, mut run: F) -> (u128, bool)
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    let mut ok_count = 0_u64;
    for _ in 0..batch_iterations {
        if black_box(run()) {
            ok_count += 1;
        }
    }
    let avg_ns = start.elapsed().as_nanos() / u128::from(batch_iterations);
    (avg_ns, ok_count == batch_iterations)
}

fn median(values: &[u128]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0
    } else {
        sorted[mid] as f64
    }
}

fn probe<F>(
    class: &'static str,
    operation: &'static str,
    expected_outcome: &'static str,
    samples: u64,
    batch_iterations: u64,
    mut run: F,
) -> ProbeRow
where
    F: FnMut() -> bool,
{
    let mut avg_ns_samples = Vec::with_capacity(samples as usize);
    let mut ok_batches = 0_u64;
    for _ in 0..samples {
        let (avg_ns, ok) = sample_avg_ns(batch_iterations, &mut run);
        avg_ns_samples.push(avg_ns);
        if ok {
            ok_batches += 1;
        }
    }
    let min_avg_ns = *avg_ns_samples.iter().min().unwrap_or(&0);
    let max_avg_ns = *avg_ns_samples.iter().max().unwrap_or(&0);
    let mean_avg_ns = avg_ns_samples
        .iter()
        .map(|value| *value as f64)
        .sum::<f64>()
        / avg_ns_samples.len() as f64;
    let median_avg_ns = median(&avg_ns_samples);
    let observed_outcome = if ok_batches == samples {
        "all_batches_matched"
    } else if ok_batches == 0 {
        "no_batches_matched"
    } else {
        "mixed_batches"
    };
    ProbeRow {
        group: "parse_malformed_input",
        class,
        operation,
        expected_outcome,
        samples,
        batch_iterations,
        avg_ns_samples,
        min_avg_ns,
        max_avg_ns,
        mean_avg_ns,
        median_avg_ns,
        ok_batches,
        observed_outcome,
    }
}

fn equalized_decode_shape(signature: &ProductionSignature, expected_ok: bool) -> bool {
    let direct_ok = signature.decode().is_ok();
    let bytes = signature.as_bytes();
    let mut normalized = [0_u8; LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES];
    let copy_len = bytes.len().min(normalized.len());
    normalized[..copy_len].copy_from_slice(&bytes[..copy_len]);

    let mut public_accumulator = 0_u8;
    for (index, byte) in normalized.iter().enumerate() {
        public_accumulator ^= byte.rotate_left((index & 7) as u32);
    }
    black_box(public_accumulator);

    let normalized_signature = ProductionSignature::new(normalized.to_vec());
    for _ in 0..EQUALIZED_CANONICAL_DECODE_PASSES {
        black_box(normalized_signature.decode().is_ok());
    }

    direct_ok == expected_ok
}

fn ratio(
    numerator_class: &'static str,
    denominator_class: &'static str,
    rows: &[ProbeRow],
) -> RatioRow {
    let numerator = rows
        .iter()
        .find(|row| row.class == numerator_class)
        .expect("numerator row exists");
    let denominator = rows
        .iter()
        .find(|row| row.class == denominator_class)
        .expect("denominator row exists");
    RatioRow {
        numerator_class,
        denominator_class,
        numerator_mean_ns: numerator.mean_avg_ns,
        denominator_mean_ns: denominator.mean_avg_ns,
        mean_ratio: numerator.mean_avg_ns / denominator.mean_avg_ns,
        numerator_median_ns: numerator.median_avg_ns,
        denominator_median_ns: denominator.median_avg_ns,
        median_ratio: numerator.median_avg_ns / denominator.median_avg_ns,
    }
}

fn parse_args() -> (u64, u64) {
    let mut samples = DEFAULT_SAMPLES;
    let mut batch_iterations = DEFAULT_BATCH_ITERATIONS;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--samples" => {
                samples = args
                    .next()
                    .expect("--samples requires an integer")
                    .parse()
                    .expect("--samples must be an integer");
            }
            "--batch-iterations" => {
                batch_iterations = args
                    .next()
                    .expect("--batch-iterations requires an integer")
                    .parse()
                    .expect("--batch-iterations must be an integer");
            }
            "--help" | "-h" => {
                println!(
                    "usage: ct_issue_slot_only_signature_probe [--samples N] [--batch-iterations N]"
                );
                std::process::exit(0);
            }
            _ => panic!("unknown argument: {arg}"),
        }
    }
    assert!(samples > 0, "--samples must be positive");
    assert!(batch_iterations > 0, "--batch-iterations must be positive");
    (samples, batch_iterations)
}

fn generated_at_unix_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is after unix epoch")
        .as_nanos()
}

fn main() {
    let (samples, batch_iterations) = parse_args();
    let valid_signature = test_signature(0);
    let slot_only_signature = ProductionSignature::new(vec![0_u8; SLOT_ID_BYTES]);

    let rows = vec![
        probe(
            "valid_linear_sigma_signature",
            "ProductionSignature::decode",
            "accept",
            samples,
            batch_iterations,
            || valid_signature.decode().is_ok(),
        ),
        probe(
            "slot_only_signature",
            "ProductionSignature::decode",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || slot_only_signature.decode().is_err(),
        ),
        probe(
            "valid_linear_sigma_signature_equalized",
            "equalized_decode_shape",
            "accept",
            samples,
            batch_iterations,
            || equalized_decode_shape(&valid_signature, true),
        ),
        probe(
            "slot_only_signature_equalized",
            "equalized_decode_shape",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || equalized_decode_shape(&slot_only_signature, false),
        ),
    ];
    let ratios = vec![
        ratio("slot_only_signature", "valid_linear_sigma_signature", &rows),
        ratio(
            "slot_only_signature_equalized",
            "valid_linear_sigma_signature_equalized",
            &rows,
        ),
    ];

    let report = ProbeReport {
        schema: "Descender.ct_issue_slot_only_signature_probe.v1",
        generated_at_unix_nanos: generated_at_unix_nanos(),
        samples,
        batch_iterations,
        signature_bytes: LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES,
        slot_id_bytes: SLOT_ID_BYTES,
        caveat: "Local differential timing probe over public malformed signature bytes in the reference crate. This is not a hardware leakage model, dudect test, compiler audit, or production constant-time proof.",
        root_cause_probe: "The slot-only input triggers the top-level length check in decode_production_signature and returns before slot, challenge, response, opening, or proof decoding. The equalized rows demonstrate the expected cost of a constant-shape diagnostic wrapper, not a library patch.",
        rows,
        ratios,
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
