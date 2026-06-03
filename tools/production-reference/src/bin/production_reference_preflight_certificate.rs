use descender_production_reference::*;
use serde_json::json;

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn main() -> Result<(), String> {
    let report = production_deployment_preflight_report();
    let blockers = production_deployment_preflight_blockers(&report)?;
    let blocker_report = production_deployment_preflight_blocker_report(&report)?;
    let verdict = production_deployment_preflight_verdict(&report)?;
    let ready_assertion = production_deployment_preflight_ready_assertion(&report, &verdict)?;
    let certificate = production_deployment_preflight_certificate(&report)?;

    let payload = json!({
        "schema": "Descender.production_reference_preflight_certificate.v1",
        "profile": PROFILE,
        "q": Q,
        "groups": GROUPS,
        "local_pivot": LOCAL_PIVOT,
        "local_free": LOCAL_FREE,
        "repetitions": REPS,
        "deployment_ready": certificate.deployment_ready,
        "blocker_count": certificate.blocker_count,
        "primary_blocker": certificate.primary_blocker,
        "lifecycle_invariants": {
            "state_assisted_pad_seed_digest_account_invariant_present": report
                .preflight
                .reviewed_fq12_state_assisted_pad_seed_digest_account_invariant_present,
            "selected_projection_spent_pad_seed_digest_invariant_present": report
                .preflight
                .reviewed_fq12_selected_projection_spent_pad_seed_digest_invariant_present,
            "target_chain_pad_seed_digest_commit_validation_present": report
                .preflight
                .reviewed_fq12_target_chain_pad_seed_digest_commit_validation_present,
        },
        "candidate_verifiers": {
            "state_assisted_masked_projection_candidate_verifier_present": report
                .preflight
                .reviewed_fq12_state_assisted_masked_projection_candidate_verifier_present,
            "deployment_verifier_present": report
                .preflight
                .reviewed_fq12_deployment_verifier_present,
        },
        "blockers": blockers
            .iter()
            .map(|blocker| {
                json!({
                    "index": blocker.index,
                    "blocker": blocker.blocker,
                    "digest": hex_encode(&blocker.digest),
                })
            })
            .collect::<Vec<_>>(),
        "digests": {
            "preflight_report": hex_encode(&certificate.preflight_report_digest),
            "blocker_report": hex_encode(&certificate.blocker_report_digest),
            "verdict": hex_encode(&certificate.verdict_digest),
            "ready_assertion": hex_encode(&certificate.ready_assertion_digest),
            "certificate": hex_encode(&certificate.digest),
        },
        "cross_checks": {
            "preflight_report_digest_matches": certificate.preflight_report_digest == report.digest,
            "blocker_report_digest_matches": certificate.blocker_report_digest == blocker_report.digest,
            "verdict_digest_matches": certificate.verdict_digest == verdict.digest,
            "ready_assertion_digest_matches": certificate.ready_assertion_digest == ready_assertion.digest,
            "certificate_validates": validate_production_deployment_preflight_certificate(&report, &certificate).is_ok(),
        },
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&payload)
            .map_err(|err| format!("failed to serialize preflight certificate: {err}"))?
    );
    Ok(())
}
