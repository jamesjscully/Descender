#![recursion_limit = "256"]

#[cfg(not(feature = "account-creation-stark-verifier"))]
fn main() -> Result<(), String> {
    Err(
        "path2_direct_accepted_composition_evidence_manifest requires --features account-creation-stark-verifier"
            .to_string(),
    )
}

#[cfg(feature = "account-creation-stark-verifier")]
mod enabled {
    use descender_parameter_search::{
        manifest_vector_evidence::{
            path2_account_creation_accepted_proof_boundary_evidence_artifact_v1,
            path2_direct_accepted_spend_tuple_evidence_artifact_v1,
        },
        production_reference::{
            path2_direct_accepted_composition_evidence_v1,
            path2_direct_accepted_composition_evidence_v1_digest, PROFILE,
        },
    };
    use serde_json::{json, Value};
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
        let evidence = path2_direct_accepted_composition_evidence_v1()?;
        let recomputed_digest = path2_direct_accepted_composition_evidence_v1_digest(&evidence);
        if recomputed_digest != evidence.digest {
            return Err("direct accepted composition evidence digest mismatch".to_string());
        }
        if evidence.deployment_ready
            || evidence.composition_theorem_instantiated
            || evidence.account_creation_proof_boundary_verified
        {
            return Err(
                "direct accepted composition evidence unexpectedly claims closure".to_string(),
            );
        }
        let accepted_proof_boundary_artifact =
            path2_account_creation_accepted_proof_boundary_evidence_artifact_v1()?;
        let accepted_spend_tuple_artifact =
            path2_direct_accepted_spend_tuple_evidence_artifact_v1()?;
        let expected_proof_boundary_digest =
            hex_encode(&evidence.account_creation_accepted_proof_boundary_evidence_digest);
        let expected_spend_tuple_digest =
            hex_encode(&evidence.direct_accepted_spend_tuple_evidence_digest);
        if accepted_proof_boundary_artifact
            .get("evidence_digest")
            .and_then(Value::as_str)
            != Some(expected_proof_boundary_digest.as_str())
            || accepted_spend_tuple_artifact
                .get("evidence_digest")
                .and_then(Value::as_str)
                != Some(expected_spend_tuple_digest.as_str())
        {
            return Err(
                "direct accepted composition manifest lost accepted input artifact binding"
                    .to_string(),
            );
        }

        Ok(json!({
            "schema": "Descender.Path2.DirectAcceptedCompositionDeterministicKatEvidenceManifest.v1",
            "corpus_id": "Descender.Path2.DirectAcceptedCompositionDeterministicKatEvidence.v1",
            "corpus_version": 1,
            "profile_id": PROFILE,
            "status": "digest_pinned_deterministic_kat_accepted_direct_composition_evidence",
            "generator": {
                "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --bin path2_direct_accepted_composition_evidence_manifest",
                "source": "tools/parameter-search/src/bin/path2_direct_accepted_composition_evidence_manifest.rs"
            },
            "deterministic_proof_byte_mode": {
                "accepted_composition_endpoint": "path2_direct_accepted_composition_evidence_v1",
                "proof_byte_policy": "deterministic KAT evidence only: the generator runs inside a one-thread Rayon pool so the fixed proof-randomness seed also fixes Plonky3 proof-of-work witness selection; deployment producers still use fresh proof randomness",
                "rayon_threads": 1,
                "evidence_digest_recomputed": true
            },
            "accepted_input_artifacts": {
                "account_creation_accepted_proof_boundary_evidence_artifact":
                    accepted_proof_boundary_artifact,
                "direct_accepted_spend_tuple_evidence_artifact":
                    accepted_spend_tuple_artifact
            },
            "evidence": {
                "schema": evidence.schema,
                "digest": hex_encode(&evidence.digest),
                "recomputed_digest": hex_encode(&recomputed_digest),
                "theorem_target": evidence.theorem_target,
                "theorem_conclusion": evidence.theorem_conclusion,
                "direct_theorem_boundary_report_digest": hex_encode(
                    &evidence.direct_theorem_boundary_report_digest
                ),
                "direct_accepted_spend_tuple_evidence_digest": hex_encode(
                    &evidence.direct_accepted_spend_tuple_evidence_digest
                ),
                "account_creation_accepted_proof_boundary_evidence_digest": hex_encode(
                    &evidence.account_creation_accepted_proof_boundary_evidence_digest
                ),
                "direct_qrom_action_boundary_report_digest": hex_encode(
                    &evidence.direct_qrom_action_boundary_report_digest
                ),
                "direct_lifecycle_boundary_report_digest": hex_encode(
                    &evidence.direct_lifecycle_boundary_report_digest
                ),
                "account_creation_proof_boundary_report_digest": hex_encode(
                    &evidence.account_creation_proof_boundary_report_digest
                ),
                "account_creation_fixture_id": evidence.account_creation_fixture_id,
                "proof_mask_seed": evidence.proof_mask_seed,
                "public_key_identity": hex_encode(&evidence.public_key_identity),
                "message_digest": hex_encode(&evidence.message_digest),
                "account_id": hex_encode(&evidence.account_id),
                "account_state_root_before": hex_encode(
                    &evidence.account_state_root_before
                ),
                "slot_id": evidence.slot_id,
                "prevalidated_slot_leaf_digest": hex_encode(
                    &evidence.prevalidated_slot_leaf_digest
                ),
                "account_creation_evidence_transcript_digest": hex_encode(
                    &evidence.account_creation_evidence_transcript_digest
                ),
                "selected_account_creation_payload_digest": hex_encode(
                    &evidence.selected_account_creation_payload_digest
                ),
                "slot_authorization_digest": hex_encode(&evidence.slot_authorization_digest),
                "direct_admission_namespace_record_digest": hex_encode(
                    &evidence.direct_admission_namespace_record_digest
                ),
                "consume_once_state_before_digest": hex_encode(
                    &evidence.consume_once_state_before_digest
                ),
                "consume_once_state_after_digest": hex_encode(
                    &evidence.consume_once_state_after_digest
                ),
                "creation_evidence_matches_direct_tuple":
                    evidence.creation_evidence_matches_direct_tuple,
                "proof_boundary_digest_matches_direct_theorem":
                    evidence.proof_boundary_digest_matches_direct_theorem,
                "qrom_action_digest_matches_direct_theorem":
                    evidence.qrom_action_digest_matches_direct_theorem,
                "lifecycle_digest_matches_direct_theorem":
                    evidence.lifecycle_digest_matches_direct_theorem,
                "direct_admission_namespace_matches_authorization":
                    evidence.direct_admission_namespace_matches_authorization,
                "consume_once_transition_matches_tuple":
                    evidence.consume_once_transition_matches_tuple,
                "verifier_boundary_accepts_creation":
                    evidence.verifier_boundary_accepts_creation,
                "direct_spend_tuple_accepts": evidence.direct_spend_tuple_accepts,
                "public_result_accepts_same_slot":
                    evidence.public_result_accepts_same_slot,
                "accepted_hypothesis_evidence_rows":
                    evidence.accepted_hypothesis_evidence_rows,
                "accepted_hypothesis_evidence_count":
                    evidence.accepted_hypothesis_evidence_count,
                "positive_acceptance_hypothesis_evidence_count":
                    evidence.positive_acceptance_hypothesis_evidence_count,
                "every_positive_acceptance_hypothesis_has_accepted_path_evidence":
                    evidence.every_positive_acceptance_hypothesis_has_accepted_path_evidence,
                "public_rejection_hypothesis_deferred_to_rejection_kats":
                    evidence.public_rejection_hypothesis_deferred_to_rejection_kats,
                "direct_theorem_obligations_still_open":
                    evidence.direct_theorem_obligations_still_open,
                "proof_family_obligations_still_open":
                    evidence.proof_family_obligations_still_open,
                "composition_theorem_instantiated":
                    evidence.composition_theorem_instantiated,
                "account_creation_proof_boundary_verified":
                    evidence.account_creation_proof_boundary_verified,
                "deployment_ready": evidence.deployment_ready
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
