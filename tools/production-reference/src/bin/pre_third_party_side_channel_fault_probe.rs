use descender_production_reference::*;
use serde::Serialize;
use std::hint::black_box;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_SAMPLES: u64 = 25;
const DEFAULT_BATCH_ITERATIONS: u64 = 1_000;
const EQUALIZED_DIAGNOSTIC_CANONICAL_DECODE_PASSES: usize = 8;
const EQUALIZED_DIAGNOSTIC_MESSAGE_BYTES: usize = 1024;

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
struct ProbeReport {
    schema: &'static str,
    generated_at_unix_nanos: u128,
    samples: u64,
    batch_iterations: u64,
    signature_bytes: usize,
    reviewed_fq12_signature_bytes: usize,
    reviewed_fq12_hidden_source_bytes: usize,
    reviewed_fq12_fallback_projection_bytes: usize,
    public_key_bytes: usize,
    caveat: &'static str,
    rows: Vec<ProbeRow>,
    infeasible_fault_injections: Vec<&'static str>,
}

fn canonical_digest(byte: u8) -> [u8; DIGEST_BYTES] {
    let mut digest = [0_u8; DIGEST_BYTES];
    digest[0] = byte;
    digest
}

fn test_public_key() -> ProductionPublicKey {
    ProductionPublicKey::new(vec![0_u8; PK_BYTES]).expect("zero public key is canonical")
}

fn test_account(unused: bool) -> ProductionAccountState {
    account_state_from_slot_seeds(
        [ProductionAccountSlotSeed {
            witness_commitment: canonical_digest(0),
            unused,
        }],
        canonical_digest(0),
    )
    .expect("test account is canonical")
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
            proof_commitment: canonical_digest(0),
            masked_witness_response: [0_u8; LINEAR_SIGMA_MASKED_WITNESS_BYTES],
        },
    )
    .expect("test signature is canonical")
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
    group: &'static str,
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
        group,
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

fn verify_accept(
    public_key: &ProductionPublicKey,
    account: &ProductionAccountState,
    signature: &ProductionSignature,
) -> bool {
    deployment_verify_with_account_binding(
        public_key,
        b"message",
        account,
        signature,
        |_public_key, _message, _account, _signature, checked| Ok(checked.slot_id() == 0),
        |accepted| *accepted,
    )
    .is_ok()
}

fn equalized_decode_diagnostic(signature: &ProductionSignature, expected_ok: bool) -> bool {
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
    black_box(
        ProductionSignature::new(normalized.to_vec())
            .decode()
            .is_ok(),
    );

    let canonical = test_signature(0);
    for _ in 0..EQUALIZED_DIAGNOSTIC_CANONICAL_DECODE_PASSES {
        black_box(canonical.decode().is_ok());
    }

    direct_ok == expected_ok
}

fn equalized_challenge_hash_diagnostic(
    public_key: &ProductionPublicKey,
    message: &[u8],
    slot_id: u32,
    hash_digest: &[u8; DIGEST_BYTES],
    proof_commitment: &[u8; DIGEST_BYTES],
) -> bool {
    let mut padded_message = [0_u8; EQUALIZED_DIAGNOSTIC_MESSAGE_BYTES];
    let copy_len = message.len().min(EQUALIZED_DIAGNOSTIC_MESSAGE_BYTES);
    padded_message[..copy_len].copy_from_slice(&message[..copy_len]);
    let frame = challenge_hash_frame(
        public_key.as_bytes(),
        &padded_message,
        slot_id,
        hash_digest,
        proof_commitment,
    );
    black_box(hash_frame_to_challenge_bytes(&frame));
    true
}

fn verify_reject_reason(
    public_key: &ProductionPublicKey,
    account: &ProductionAccountState,
    signature: &ProductionSignature,
    expected: &str,
) -> bool {
    deployment_verify_with_account_binding(
        public_key,
        b"message",
        account,
        signature,
        |_public_key, _message, _account, _signature, checked| Ok(checked.slot_id() == 0),
        |accepted| *accepted,
    )
    .err()
    .map(|rejection| rejection.reason() == expected)
    .unwrap_or(false)
}

fn callback_reject_reason(
    public_key: &ProductionPublicKey,
    account: &ProductionAccountState,
    signature: &ProductionSignature,
) -> bool {
    deployment_verify_with_account_binding(
        public_key,
        b"message",
        account,
        signature,
        |_public_key, _message, _account, _signature, _checked| Err("fault detail".to_string()),
        |_accepted: &bool| true,
    )
    .err()
    .map(|rejection| rejection.reason() == PRODUCTION_VERIFIER_CALLBACK_FAILURE_REASON)
    .unwrap_or(false)
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
                    "usage: pre_third_party_side_channel_fault_probe [--samples N] [--batch-iterations N]"
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
    let public_key = test_public_key();
    let account = test_account(true);
    let consumed_account = test_account(false);
    let signature = test_signature(0);

    let short_empty = ProductionSignature::new(Vec::new());
    let short_slot_only = ProductionSignature::new(vec![0_u8; SLOT_ID_BYTES]);
    let mut truncated_bytes = signature.as_bytes().to_vec();
    truncated_bytes.pop();
    let truncated_signature = ProductionSignature::new(truncated_bytes);
    let mut extended_bytes = signature.as_bytes().to_vec();
    extended_bytes.push(0);
    let extended_signature = ProductionSignature::new(extended_bytes);
    let mut invalid_challenge_bytes = signature.as_bytes().to_vec();
    invalid_challenge_bytes[SLOT_ID_BYTES + ALGEBRAIC_CHALLENGE_BYTES - 1] = 0xff;
    let invalid_challenge_signature = ProductionSignature::new(invalid_challenge_bytes);
    let mut slot_out_of_range_bytes = signature.as_bytes().to_vec();
    slot_out_of_range_bytes[..SLOT_ID_BYTES].copy_from_slice(&7_u32.to_le_bytes());
    let slot_out_of_range_signature = ProductionSignature::new(slot_out_of_range_bytes);
    let mut proof_mismatch_bytes = signature.as_bytes().to_vec();
    let proof_start = SLOT_ID_BYTES + CHALLENGE_BYTES + ACTIVE_RESPONSE_BYTES + LOCAL_OPENING_BYTES;
    proof_mismatch_bytes[proof_start] ^= 1;
    let proof_mismatch_signature = ProductionSignature::new(proof_mismatch_bytes);

    let secret = ProductionSecretKey::new(());
    let slot = account.get_slot(0).expect("slot exists");
    let hash_digest = canonical_digest(3);
    let proof_commitment = canonical_digest(4);
    let pad_seed_zero = [0_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
    let pad_seed_one = [1_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
    let pad_seed_ff = [0xff_u8; REVIEWED_FQ12_PAD_SEED_BYTES];

    let rows = vec![
        probe(
            "parse_malformed_input",
            "valid_linear_sigma_signature",
            "ProductionSignature::decode",
            "accept",
            samples,
            batch_iterations,
            || signature.decode().is_ok(),
        ),
        probe(
            "parse_malformed_input",
            "empty_signature",
            "ProductionSignature::decode",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || short_empty.decode().is_err(),
        ),
        probe(
            "parse_malformed_input",
            "slot_only_signature",
            "ProductionSignature::decode",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || short_slot_only.decode().is_err(),
        ),
        probe(
            "parse_malformed_input",
            "truncated_by_one",
            "ProductionSignature::decode",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || truncated_signature.decode().is_err(),
        ),
        probe(
            "parse_malformed_input",
            "extended_by_one",
            "ProductionSignature::decode",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || extended_signature.decode().is_err(),
        ),
        probe(
            "parse_malformed_input",
            "invalid_challenge_mask",
            "ProductionSignature::decode",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || invalid_challenge_signature.decode().is_err(),
        ),
        probe(
            "equalized_parse_diagnostic",
            "valid_linear_sigma_signature_equalized",
            "equalized_decode_diagnostic",
            "accept",
            samples,
            batch_iterations,
            || equalized_decode_diagnostic(&signature, true),
        ),
        probe(
            "equalized_parse_diagnostic",
            "empty_signature_equalized",
            "equalized_decode_diagnostic",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || equalized_decode_diagnostic(&short_empty, false),
        ),
        probe(
            "equalized_parse_diagnostic",
            "slot_only_signature_equalized",
            "equalized_decode_diagnostic",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || equalized_decode_diagnostic(&short_slot_only, false),
        ),
        probe(
            "equalized_parse_diagnostic",
            "truncated_by_one_equalized",
            "equalized_decode_diagnostic",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || equalized_decode_diagnostic(&truncated_signature, false),
        ),
        probe(
            "equalized_parse_diagnostic",
            "extended_by_one_equalized",
            "equalized_decode_diagnostic",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || equalized_decode_diagnostic(&extended_signature, false),
        ),
        probe(
            "equalized_parse_diagnostic",
            "invalid_challenge_mask_equalized",
            "equalized_decode_diagnostic",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || equalized_decode_diagnostic(&invalid_challenge_signature, false),
        ),
        probe(
            "rejection_class",
            "valid_accept_dummy_callback",
            "deployment_verify_with_account_binding",
            "accept",
            samples,
            batch_iterations,
            || verify_accept(&public_key, &account, &signature),
        ),
        probe(
            "rejection_class",
            "verifier_callback_failure",
            "deployment_verify_with_account_binding",
            "reject_sanitized_callback_failure",
            samples,
            batch_iterations,
            || callback_reject_reason(&public_key, &account, &signature),
        ),
        probe(
            "rejection_class",
            "slot_out_of_range",
            "deployment_verify_with_account_binding",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || {
                verify_reject_reason(
                    &public_key,
                    &account,
                    &slot_out_of_range_signature,
                    "slot id is out of range",
                )
            },
        ),
        probe(
            "rejection_class",
            "slot_already_consumed",
            "deployment_verify_with_account_binding",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || {
                verify_reject_reason(
                    &public_key,
                    &consumed_account,
                    &signature,
                    "slot is already consumed",
                )
            },
        ),
        probe(
            "rejection_class",
            "proof_commitment_mismatch",
            "deployment_verify_with_account_binding",
            "reject_fail_closed",
            samples,
            batch_iterations,
            || {
                verify_reject_reason(
                    &public_key,
                    &account,
                    &proof_mismatch_signature,
                    "account randomizer commitment mismatch",
                )
            },
        ),
        probe(
            "fault_injection_callback",
            "signer_callback_failure",
            "deployment_sign_with_secret_key",
            "reject_sanitized_callback_failure",
            samples,
            batch_iterations,
            || {
                deployment_sign_with_secret_key(&secret, slot, |_secret, _slot_id, _commitment| {
                    Err("signer fault detail".to_string())
                })
                .err()
                .map(|err| err == PRODUCTION_SIGNER_CALLBACK_FAILURE_REASON)
                .unwrap_or(false)
            },
        ),
        probe(
            "hash_frame_public_input",
            "challenge_hash_baseline",
            "challenge_hash_frame + hash_frame_to_challenge_bytes",
            "deterministic_hash",
            samples,
            batch_iterations,
            || {
                let frame = challenge_hash_frame(
                    public_key.as_bytes(),
                    b"message",
                    0,
                    &hash_digest,
                    &proof_commitment,
                );
                black_box(hash_frame_to_challenge_bytes(&frame));
                true
            },
        ),
        probe(
            "equalized_hash_frame_diagnostic",
            "challenge_hash_baseline_equalized",
            "equalized_challenge_hash_frame_1024",
            "deterministic_hash",
            samples,
            batch_iterations,
            || {
                equalized_challenge_hash_diagnostic(
                    &public_key,
                    b"message",
                    0,
                    &hash_digest,
                    &proof_commitment,
                )
            },
        ),
        probe(
            "equalized_hash_frame_diagnostic",
            "challenge_hash_long_message_equalized",
            "equalized_challenge_hash_frame_1024",
            "deterministic_hash",
            samples,
            batch_iterations,
            || {
                equalized_challenge_hash_diagnostic(
                    &public_key,
                    &[7_u8; 1024],
                    0,
                    &hash_digest,
                    &proof_commitment,
                )
            },
        ),
        probe(
            "hash_frame_public_input",
            "challenge_hash_long_message",
            "challenge_hash_frame + hash_frame_to_challenge_bytes",
            "deterministic_hash",
            samples,
            batch_iterations,
            || {
                let frame = challenge_hash_frame(
                    public_key.as_bytes(),
                    &[7_u8; 1024],
                    0,
                    &hash_digest,
                    &proof_commitment,
                );
                black_box(hash_frame_to_challenge_bytes(&frame));
                true
            },
        ),
        probe(
            "prf_seed_class",
            "pad_prf_zero_seed",
            "reviewed_fq12_pad_residues_from_seed_bounded",
            "deterministic_prf",
            samples,
            batch_iterations,
            || reviewed_fq12_pad_residues_from_seed_bounded(&pad_seed_zero, 0).is_ok(),
        ),
        probe(
            "prf_seed_class",
            "pad_prf_one_seed",
            "reviewed_fq12_pad_residues_from_seed_bounded",
            "deterministic_prf",
            samples,
            batch_iterations,
            || reviewed_fq12_pad_residues_from_seed_bounded(&pad_seed_one, 0).is_ok(),
        ),
        probe(
            "prf_seed_class",
            "pad_prf_ff_seed",
            "reviewed_fq12_pad_residues_from_seed_bounded",
            "deterministic_prf",
            samples,
            batch_iterations,
            || reviewed_fq12_pad_residues_from_seed_bounded(&pad_seed_ff, 0).is_ok(),
        ),
        probe(
            "prf_projection_class",
            "fallback_pad_projection_zero_seed",
            "hidden_source_fallback_pad_prf_projection_values_from_seed",
            "deterministic_prf_projection",
            samples,
            batch_iterations,
            || hidden_source_fallback_pad_prf_projection_values_from_seed(&pad_seed_zero).is_ok(),
        ),
    ];

    let report = ProbeReport {
        schema: "Descender.pre_third_party_side_channel_fault_probe.v1",
        generated_at_unix_nanos: generated_at_unix_nanos(),
        samples,
        batch_iterations,
        signature_bytes: LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES,
        reviewed_fq12_signature_bytes: REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES,
        reviewed_fq12_hidden_source_bytes:
            REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_HIDDEN_SOURCE_PROOF_BYTES,
        reviewed_fq12_fallback_projection_bytes:
            REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES,
        public_key_bytes: PK_BYTES,
        caveat: "Local differential timing/fault probe over reference APIs only: deterministic dummy callbacks, public malformed inputs, equalized diagnostic rows for comparison only, no hardware leakage model, no constant-time proof, and no production hardening claim.",
        rows,
        infeasible_fault_injections: vec![
            "SHA-256 compression failures are not injectable through the current safe reference API.",
            "Bounded pad PRF counter exhaustion was not found as an injectable local fault without altering library internals.",
            "Secret-dependent hardware faults are outside this deterministic local reference binary.",
        ],
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
