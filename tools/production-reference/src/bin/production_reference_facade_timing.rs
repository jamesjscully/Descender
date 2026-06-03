use descender_production_reference::*;
use serde::Serialize;
use std::hint::black_box;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_ITERATIONS: u64 = 10_000;

#[derive(Clone, Debug, Serialize)]
struct TimingRow {
    operation: &'static str,
    iterations: u64,
    avg_ns: u128,
    avg_us: f64,
    accepted_or_ok: bool,
}

#[derive(Clone, Debug, Serialize)]
struct SelectedProjectionTimingRow {
    operation: &'static str,
    reject_class: &'static str,
    iterations: u64,
    avg_ns: u128,
    avg_us: f64,
    ratio_to_valid_baseline: f64,
    expected_ok: bool,
    ok: bool,
    expected_public_reason: Option<&'static str>,
    observed_public_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
struct TimingReport {
    schema: &'static str,
    generated_at_unix_nanos: u128,
    iterations: u64,
    signature_bytes: usize,
    reviewed_target_signature_bytes: usize,
    reviewed_fq12_fallback_projection_bytes: usize,
    account_link_bytes: usize,
    public_key_bytes: usize,
    caveat: &'static str,
    timings: Vec<TimingRow>,
    selected_projection_caveat: &'static str,
    selected_projection_timings: Vec<SelectedProjectionTimingRow>,
}

fn canonical_digest(byte: u8) -> [u8; DIGEST_BYTES] {
    let mut digest = [0_u8; DIGEST_BYTES];
    digest[0] = byte;
    digest
}

fn test_account() -> ProductionAccountState {
    account_state_from_slot_seeds(
        [ProductionAccountSlotSeed {
            witness_commitment: canonical_digest(0),
            unused: true,
        }],
        canonical_digest(0),
    )
    .expect("test account is canonical")
}

fn test_public_key() -> ProductionPublicKey {
    ProductionPublicKey::new(vec![0_u8; PK_BYTES]).expect("zero public key is canonical")
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

fn parse_iterations() -> u64 {
    let mut args = std::env::args().skip(1);
    let mut iterations = DEFAULT_ITERATIONS;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--iterations" => {
                let value = args
                    .next()
                    .expect("--iterations requires an integer argument");
                iterations = value
                    .parse::<u64>()
                    .expect("--iterations must be an integer");
            }
            "--help" | "-h" => {
                println!("usage: production_reference_facade_timing [--iterations N]");
                std::process::exit(0);
            }
            _ => panic!("unknown argument: {arg}"),
        }
    }
    assert!(iterations > 0, "--iterations must be positive");
    iterations
}

fn bench<F>(operation: &'static str, iterations: u64, mut run: F) -> TimingRow
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    let mut ok_count = 0_u64;
    for _ in 0..iterations {
        if black_box(run()) {
            ok_count += 1;
        }
    }
    let elapsed = start.elapsed();
    let avg_ns = elapsed.as_nanos() / u128::from(iterations);
    TimingRow {
        operation,
        iterations,
        avg_ns,
        avg_us: avg_ns as f64 / 1_000.0,
        accepted_or_ok: ok_count == iterations,
    }
}

fn bench_selected<F>(
    operation: &'static str,
    reject_class: &'static str,
    iterations: u64,
    valid_baseline_ns: u128,
    expected_ok: bool,
    expected_public_reason: Option<&'static str>,
    mut run: F,
) -> SelectedProjectionTimingRow
where
    F: FnMut() -> Result<(), ProductionAccountBindingError>,
{
    let start = Instant::now();
    let mut ok_count = 0_u64;
    let mut observed_public_reason = None;
    for _ in 0..iterations {
        let result = black_box(run());
        let matched = match result {
            Ok(()) => expected_ok,
            Err(err) => {
                let reason = err.reason().to_string();
                observed_public_reason = Some(reason.clone());
                !expected_ok && expected_public_reason == Some(reason.as_str())
            }
        };
        if matched {
            ok_count += 1;
        }
    }
    let elapsed = start.elapsed();
    let avg_ns = elapsed.as_nanos() / u128::from(iterations);
    SelectedProjectionTimingRow {
        operation,
        reject_class,
        iterations,
        avg_ns,
        avg_us: avg_ns as f64 / 1_000.0,
        ratio_to_valid_baseline: avg_ns as f64 / valid_baseline_ns.max(1) as f64,
        expected_ok,
        ok: ok_count == iterations,
        expected_public_reason,
        observed_public_reason,
    }
}

fn signer_facade_row(iterations: u64) -> TimingRow {
    let account = test_account();
    let slot = account.get_slot(0).expect("slot 0 exists");
    let secret = ProductionSecretKey::new(());
    bench(
        "deployment_sign_with_secret_key_dummy_callback",
        iterations,
        || {
            deployment_sign_with_secret_key(&secret, slot, |_secret, slot_id, _commitment| {
                Ok(test_signature(slot_id))
            })
            .map(|signature| signature.len() == LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES)
            .unwrap_or(false)
        },
    )
}

fn verifier_accept_row(iterations: u64) -> TimingRow {
    let public_key = test_public_key();
    let account = test_account();
    let signature = test_signature(0);
    bench(
        "deployment_verify_with_account_binding_dummy_accept",
        iterations,
        || {
            deployment_verify_with_account_binding(
                &public_key,
                b"message",
                &account,
                &signature,
                |_public_key, _message, _account, _signature, checked| Ok(checked.slot_id() == 0),
                |accepted| *accepted,
            )
            .map(|verification| {
                *verification.report()
                    && !verification.next_account().get_slot(0).unwrap().is_unused()
            })
            .unwrap_or(false)
        },
    )
}

fn verifier_reject_callback_row(iterations: u64) -> TimingRow {
    let public_key = test_public_key();
    let account = test_account();
    let signature = test_signature(0);
    bench(
        "deployment_verify_with_account_binding_sanitized_callback_reject",
        iterations,
        || {
            deployment_verify_with_account_binding(
                &public_key,
                b"message",
                &account,
                &signature,
                |_public_key, _message, _account, _signature, _checked| {
                    Err("callback detail".to_string())
                },
                |_accepted: &bool| true,
            )
            .err()
            .map(|rejection| rejection.reason() == PRODUCTION_VERIFIER_CALLBACK_FAILURE_REASON)
            .unwrap_or(false)
        },
    )
}

fn reserved_sign_consume_row(iterations: u64) -> TimingRow {
    bench(
        "deployment_sign_reserved_and_consume_fresh_manager_dummy_callback",
        iterations,
        || {
            let mut manager = ProductionAccountLifecycleManager::new(test_account())
                .expect("manager initializes");
            let reservation = manager.reserve_slot(0).expect("slot reserves");
            deployment_sign_reserved_and_consume(
                &mut manager,
                reservation,
                |slot_id, _commitment| Ok(test_signature(slot_id)),
            )
            .map(|signature| {
                signature.decode().unwrap().slot_id == 0
                    && !manager.account().get_slot(0).unwrap().is_unused()
            })
            .unwrap_or(false)
        },
    )
}

fn selected_projection_verify(
    fixture: &descender_parameter_search::production_reference::ProductionSelectedProjectionCandidateTimingFixture,
    signature_with_carrier: &ProductionSignature,
    account: &ProductionReviewedFq12StateAssistedAccountState,
    opening: &ProductionReviewedFq12StateAssistedOpening,
    statement: &ProductionReviewedFq12HiddenSourceStatement,
    pad_seed: &[u8; REVIEWED_FQ12_PAD_SEED_BYTES],
) -> Result<(), ProductionAccountBindingError> {
    verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier(
        &fixture.public_key,
        &fixture.message,
        account,
        signature_with_carrier,
        opening,
        statement,
        pad_seed,
    )
    .map(|_| ())
}

fn selected_projection_timing_rows(iterations: u64) -> Vec<SelectedProjectionTimingRow> {
    let fixture =
        descender_parameter_search::production_reference::production_selected_projection_candidate_timing_fixture();
    assert_eq!(
        fixture.signature_with_carrier.as_bytes().len(),
        REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES
    );

    let baseline = bench_selected(
        "selected_projection_valid_314_byte_fq12_candidate",
        "valid",
        iterations,
        1,
        true,
        None,
        || {
            selected_projection_verify(
                &fixture,
                &fixture.signature_with_carrier,
                &fixture.account,
                &fixture.opening,
                &fixture.statement,
                &fixture.pad_seed,
            )
        },
    );
    let valid_baseline_ns = baseline.avg_ns;

    let mut wrong_pad_seed = fixture.pad_seed;
    wrong_pad_seed[0] ^= 1;

    let mut witness_opening = fixture.opening;
    witness_opening.witness_opening_digest[0] ^= 1;

    let mut carrier_bytes = *fixture.carrier.as_bytes();
    let lane0 = u16::from_le_bytes([carrier_bytes[0], carrier_bytes[1]]);
    carrier_bytes[0..2].copy_from_slice(&((lane0 + 1) % Q).to_le_bytes());
    let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes)
        .expect("mutated carrier remains canonical");
    let carrier_signature =
        encode_reviewed_fq12_account_carried_with_fallback_projection(&fixture.signature, &carrier)
            .expect("carrier mutation envelope encodes");

    let mut transcript_statement = fixture.statement;
    transcript_statement.message_digest[0] ^= 1;
    transcript_statement.digest =
        reviewed_fq12_hidden_source_statement_digest(&transcript_statement);

    let mut checked_core_bytes = fixture.signature_with_carrier.as_bytes().to_vec();
    let proof_offset =
        SLOT_ID_BYTES + CHALLENGE_BYTES + ACTIVE_RESPONSE_BYTES + LOCAL_OPENING_BYTES;
    checked_core_bytes[proof_offset] ^= 1;
    let checked_core_signature = ProductionSignature::new(checked_core_bytes);

    vec![
        SelectedProjectionTimingRow {
            ratio_to_valid_baseline: 1.0,
            ..baseline
        },
        bench_selected(
            "selected_projection_reject_pad_seed_rebinding_314_byte_fq12",
            "pad",
            iterations,
            valid_baseline_ns,
            false,
            Some(PRODUCTION_SELECTED_PROJECTION_INVALID_OPENING_REASON),
            || {
                selected_projection_verify(
                    &fixture,
                    &fixture.signature_with_carrier,
                    &fixture.account,
                    &fixture.opening,
                    &fixture.statement,
                    &wrong_pad_seed,
                )
            },
        ),
        bench_selected(
            "selected_projection_reject_witness_opening_digest_314_byte_fq12",
            "witness",
            iterations,
            valid_baseline_ns,
            false,
            Some(PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON),
            || {
                selected_projection_verify(
                    &fixture,
                    &fixture.signature_with_carrier,
                    &fixture.account,
                    &witness_opening,
                    &fixture.statement,
                    &fixture.pad_seed,
                )
            },
        ),
        bench_selected(
            "selected_projection_reject_carrier_lane_314_byte_fq12",
            "carrier",
            iterations,
            valid_baseline_ns,
            false,
            Some(PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON),
            || {
                selected_projection_verify(
                    &fixture,
                    &carrier_signature,
                    &fixture.account,
                    &fixture.opening,
                    &fixture.statement,
                    &fixture.pad_seed,
                )
            },
        ),
        bench_selected(
            "selected_projection_reject_transcript_message_digest_314_byte_fq12",
            "transcript",
            iterations,
            valid_baseline_ns,
            false,
            Some(PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON),
            || {
                selected_projection_verify(
                    &fixture,
                    &fixture.signature_with_carrier,
                    &fixture.account,
                    &fixture.opening,
                    &transcript_statement,
                    &fixture.pad_seed,
                )
            },
        ),
        bench_selected(
            "selected_projection_reject_checked_core_proof_commitment_314_byte_fq12",
            "checked_core",
            iterations,
            valid_baseline_ns,
            false,
            Some(PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON),
            || {
                selected_projection_verify(
                    &fixture,
                    &checked_core_signature,
                    &fixture.account,
                    &fixture.opening,
                    &fixture.statement,
                    &fixture.pad_seed,
                )
            },
        ),
    ]
}

fn generated_at_unix_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is after unix epoch")
        .as_nanos()
}

fn main() {
    let iterations = parse_iterations();
    let report = TimingReport {
        schema: "Descender.production_reference_facade_timing.v1",
        generated_at_unix_nanos: generated_at_unix_nanos(),
        iterations,
        signature_bytes: LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES,
        reviewed_target_signature_bytes: REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES,
        reviewed_fq12_fallback_projection_bytes:
            REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES,
        account_link_bytes: REVIEWED_ACCOUNT_LINK_BYTES,
        public_key_bytes: PK_BYTES,
        caveat: "Production-reference crate facade timing only: deterministic dummy callbacks, no accepting 314-byte/Fq12 projection-carrier proof semantics, and no constant-time claim.",
        timings: vec![
            signer_facade_row(iterations),
            verifier_accept_row(iterations),
            verifier_reject_callback_row(iterations),
            reserved_sign_consume_row(iterations),
        ],
        selected_projection_caveat: "Selected 314-byte/Fq12 candidate-verifier timing rows run one deterministic in-process reference fixture across valid, pad, witness, carrier, transcript, and checked-core reject mutations. They are same-harness equivalence evidence for this reference path only; they are not a hardware leakage model, dudect result, compiler audit, final production verifier proof, or constant-time claim.",
        selected_projection_timings: selected_projection_timing_rows(iterations),
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
