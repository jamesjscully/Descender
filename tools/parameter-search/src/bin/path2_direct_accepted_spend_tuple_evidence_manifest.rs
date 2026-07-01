#![recursion_limit = "256"]

#[cfg(not(feature = "account-creation-stark-verifier"))]
fn main() -> Result<(), String> {
    Err(
        "path2_direct_accepted_spend_tuple_evidence_manifest requires --features account-creation-stark-verifier"
            .to_string(),
    )
}

#[cfg(feature = "account-creation-stark-verifier")]
mod enabled {
    use descender_parameter_search::production_reference::{
        PROFILE, path2_direct_accepted_spend_tuple_evidence_v1_digest,
        path2_direct_accepted_spend_tuple_kat_evidence_v1,
    };
    use serde_json::{Value, json};
    use sha2::{Digest, Sha256};

    fn hex_encode(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            out.push(HEX[(byte >> 4) as usize] as char);
            out.push(HEX[(byte & 0x0f) as usize] as char);
        }
        out
    }

    fn manifest_body() -> Result<Value, String> {
        let kat = path2_direct_accepted_spend_tuple_kat_evidence_v1()?;
        let evidence = kat.evidence;
        let recomputed_digest = path2_direct_accepted_spend_tuple_evidence_v1_digest(&evidence);
        if recomputed_digest != evidence.digest {
            return Err("direct accepted spend tuple evidence digest mismatch".to_string());
        }
        if !evidence.state_namespace_record_present
            || !evidence.consumed_authorization_absent_before
            || !evidence.consumed_authorization_present_after
            || !evidence.consume_once_inserted_exactly_one_authorization
            || !evidence.public_result_accepts_same_slot
            || !evidence.tuple_fields_match_theorem_boundary
            || evidence.accepted_tuple_field_evidence_count != 11
            || evidence.accepted_tuple_field_evidence_present_count != 11
            || evidence.accepted_tuple_field_open_obligation_count != 6
            || evidence.accepted_tuple_field_imported_closure_count != 2
            || !evidence.every_accepted_tuple_field_has_evidence
            || evidence.every_accepted_tuple_field_has_open_direct_obligation
            || !evidence.every_accepted_tuple_field_has_open_or_imported_direct_obligation
            || !kat.replay_rejected_by_consume_once
        {
            return Err("direct accepted spend tuple evidence lost acceptance flags".to_string());
        }

        Ok(json!({
            "schema": "Descender.Path2.DirectAcceptedSpendTupleDeterministicKatEvidenceManifest.v1",
            "corpus_id": "Descender.Path2.DirectAcceptedSpendTupleDeterministicKatEvidence.v1",
            "corpus_version": 1,
            "profile_id": PROFILE,
            "status": "digest_pinned_deterministic_kat_accepted_direct_spend_tuple_evidence",
            "generator": {
                "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --bin path2_direct_accepted_spend_tuple_evidence_manifest",
                "source": "tools/parameter-search/src/bin/path2_direct_accepted_spend_tuple_evidence_manifest.rs"
            },
            "deterministic_tuple_mode": {
                "accepted_tuple_endpoint": "verify_path2_direct_single_slot_spend_authorization_with_tuple_evidence_v1",
                "source_fixture_endpoint": "path2_direct_accepted_spend_tuple_kat_evidence_v1",
                "proof_byte_policy": "deterministic KAT evidence only: the generator runs inside a one-thread Rayon pool so the fixed account-STARK proof-randomness seed also fixes Plonky3 proof-of-work witness selection; deployment producers still use fresh proof randomness",
                "rayon_threads": 1,
                "evidence_digest_recomputed": true,
                "replay_rejected_by_consume_once": kat.replay_rejected_by_consume_once
            },
            "evidence": {
                "schema": evidence.schema,
                "digest": hex_encode(&evidence.digest),
                "recomputed_digest": hex_encode(&recomputed_digest),
                "account_creation_fixture_id": kat.account_creation_fixture_id,
                "proof_mask_seed": kat.proof_mask_seed,
                "theorem_target": evidence.theorem_target,
                "theorem_conclusion": evidence.theorem_conclusion,
                "accepted_tuple_fields": evidence.accepted_tuple_fields,
                "accepted_tuple_field_count": evidence.accepted_tuple_fields.len(),
                "accepted_tuple_field_evidence_rows":
                    evidence.accepted_tuple_field_evidence_rows,
                "accepted_tuple_field_evidence_count":
                    evidence.accepted_tuple_field_evidence_count,
                "accepted_tuple_field_evidence_present_count":
                    evidence.accepted_tuple_field_evidence_present_count,
                "accepted_tuple_field_open_obligation_count":
                    evidence.accepted_tuple_field_open_obligation_count,
                "accepted_tuple_field_imported_closure_count":
                    evidence.accepted_tuple_field_imported_closure_count,
                "every_accepted_tuple_field_has_evidence":
                    evidence.every_accepted_tuple_field_has_evidence,
                "every_accepted_tuple_field_has_open_direct_obligation":
                    evidence.every_accepted_tuple_field_has_open_direct_obligation,
                "every_accepted_tuple_field_has_open_or_imported_direct_obligation":
                    evidence.every_accepted_tuple_field_has_open_or_imported_direct_obligation,
                "acceptance_hypotheses": evidence.acceptance_hypotheses,
                "acceptance_hypothesis_count": evidence.acceptance_hypotheses.len(),
                "direct_qrom_action_boundary_report_digest": hex_encode(
                    &evidence.direct_qrom_action_boundary_report_digest
                ),
                "direct_lifecycle_boundary_report_digest": hex_encode(
                    &evidence.direct_lifecycle_boundary_report_digest
                ),
                "account_creation_proof_boundary_report_digest": hex_encode(
                    &evidence.account_creation_proof_boundary_report_digest
                ),
                "public_key_identity": hex_encode(&evidence.public_key_identity),
                "message_digest": hex_encode(&evidence.message_digest),
                "account_id": hex_encode(&evidence.account_id),
                "account_state_root_before": hex_encode(
                    &evidence.account_state_root_before
                ),
                "slot_id": evidence.slot_id,
                "accepted_slot_id": evidence.accepted_slot_id,
                "checked_core_signature_digest": hex_encode(
                    &evidence.checked_core_signature_digest
                ),
                "checked_core_challenge_digest": hex_encode(
                    &evidence.checked_core_challenge_digest
                ),
                "checked_core_proof_commitment_digest": hex_encode(
                    &evidence.checked_core_proof_commitment_digest
                ),
                "prevalidated_slot_leaf_digest": hex_encode(
                    &evidence.prevalidated_slot_leaf_digest
                ),
                "account_creation_evidence_transcript_digest": hex_encode(
                    &evidence.account_creation_evidence_transcript_digest
                ),
                "selected_account_creation_payload_digest": hex_encode(
                    &evidence.selected_account_creation_payload_digest
                ),
                "direct_admission_namespace_record_digest": hex_encode(
                    &evidence.direct_admission_namespace_record_digest
                ),
                "slot_authorization_digest": hex_encode(&evidence.slot_authorization_digest),
                "consume_once_state_before_digest": hex_encode(
                    &evidence.consume_once_state_before_digest
                ),
                "consume_once_state_after_digest": hex_encode(
                    &evidence.consume_once_state_after_digest
                ),
                "public_result_class_name": evidence.public_result_class_name,
                "state_namespace_record_present": evidence.state_namespace_record_present,
                "consumed_authorization_absent_before":
                    evidence.consumed_authorization_absent_before,
                "consumed_authorization_present_after":
                    evidence.consumed_authorization_present_after,
                "consume_once_inserted_exactly_one_authorization":
                    evidence.consume_once_inserted_exactly_one_authorization,
                "public_result_accepts_same_slot": evidence.public_result_accepts_same_slot,
                "tuple_fields_match_theorem_boundary":
                    evidence.tuple_fields_match_theorem_boundary,
                "replay_rejected_by_consume_once": kat.replay_rejected_by_consume_once
            }
        }))
    }

    pub fn run() -> Result<(), String> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .map_err(|err| {
                format!("failed to build deterministic single-thread KAT pool: {err}")
            })?;
        let body = pool.install(manifest_body)?;
        let body_json = serde_json::to_string_pretty(&body)
            .map_err(|err| format!("failed to serialize manifest body: {err}"))?;
        let manifest = json!({
            "manifest_digest": hex_encode(&Sha256::digest(body_json.as_bytes())),
            "manifest": body
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&manifest)
                .map_err(|err| format!("failed to serialize manifest: {err}"))?
        );
        Ok(())
    }
}

#[cfg(feature = "account-creation-stark-verifier")]
fn main() -> Result<(), String> {
    enabled::run()
}
