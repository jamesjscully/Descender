#![recursion_limit = "256"]

#[cfg(not(feature = "account-creation-stark-verifier"))]
fn main() -> Result<(), String> {
    Err(
        "path2_account_creation_accepted_proof_boundary_evidence_manifest requires --features account-creation-stark-verifier"
            .to_string(),
    )
}

#[cfg(feature = "account-creation-stark-verifier")]
mod enabled {
    use descender_parameter_search::production_reference::{
        path2_account_creation_accepted_proof_boundary_evidence_v1,
        path2_account_creation_accepted_proof_boundary_evidence_v1_digest, PROFILE,
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
        let evidence = path2_account_creation_accepted_proof_boundary_evidence_v1()?;
        let recomputed_digest =
            path2_account_creation_accepted_proof_boundary_evidence_v1_digest(&evidence);
        if recomputed_digest != evidence.digest {
            return Err(
                "account-creation accepted proof-boundary evidence digest mismatch".to_string(),
            );
        }
        if evidence.deployment_ready
            || !evidence.verifier_boundary_accepts
            || !evidence.production_gate_fail_closed
            || !evidence.proof_boundary_report_validated
            || evidence.public_payload_exports_private_material
            || !evidence.payload_digest_bound_to_certificate
            || !evidence.evidence_digest_bound_to_certificate
            || !evidence.leaf_transcript_bound_to_evidence
            || !evidence.leaf_digest_bound_to_evidence
            || !evidence.leaf_context_bound
            || !evidence.binding_digest_bound
            || !evidence.payload_binding_fields_match_report
            || !evidence.decoded_public_anchors_match_deployed_anchors
            || !evidence.deployed_anchor_fields_match_report
            || !evidence.account_stark_bundle_nonempty
            || !evidence.proof_family_obligations_still_open
        {
            return Err(
                "account-creation accepted proof-boundary evidence lost fail-closed invariants"
                    .to_string(),
            );
        }

        Ok(json!({
            "schema":
                "Descender.Path2.AccountCreationAcceptedProofBoundaryDeterministicKatEvidenceManifest.v1",
            "corpus_id":
                "Descender.Path2.AccountCreationAcceptedProofBoundaryDeterministicKatEvidence.v1",
            "corpus_version": 1,
            "profile_id": PROFILE,
            "status":
                "digest_pinned_deterministic_kat_accepted_account_creation_proof_boundary_evidence",
            "generator": {
                "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --bin path2_account_creation_accepted_proof_boundary_evidence_manifest",
                "source": "tools/parameter-search/src/bin/path2_account_creation_accepted_proof_boundary_evidence_manifest.rs"
            },
            "deterministic_proof_byte_mode": {
                "accepted_boundary_endpoint":
                    "path2_account_creation_accepted_proof_boundary_evidence_v1",
                "proof_byte_policy": "deterministic KAT evidence only: the generator runs inside a one-thread Rayon pool so the fixed proof-randomness seed also fixes Plonky3 proof-of-work witness selection; deployment producers still use fresh proof randomness",
                "rayon_threads": 1,
                "evidence_digest_recomputed": true
            },
            "evidence": {
                "schema": evidence.schema,
                "digest": hex_encode(&evidence.digest),
                "recomputed_digest": hex_encode(&recomputed_digest),
                "proof_boundary_target": evidence.proof_boundary_target,
                "proof_boundary_conclusion": evidence.proof_boundary_conclusion,
                "selected_payload_type": evidence.selected_payload_type,
                "verifier_boundary_endpoint": evidence.verifier_boundary_endpoint,
                "production_boundary_endpoint": evidence.production_boundary_endpoint,
                "replay_payload_digest_function": evidence.replay_payload_digest_function,
                "accepting_requirements": evidence.accepting_requirements,
                "account_creation_proof_boundary_report_digest": hex_encode(
                    &evidence.account_creation_proof_boundary_report_digest
                ),
                "sampler_timing_report_digest": hex_encode(
                    &evidence.sampler_timing_report_digest
                ),
                "no_export_boundary_report_digest": hex_encode(
                    &evidence.no_export_boundary_report_digest
                ),
                "account_stark_residual_matrix_report_digest": hex_encode(
                    &evidence.account_stark_residual_matrix_report_digest
                ),
                "frame_kat_boundary_report_digest": hex_encode(
                    &evidence.frame_kat_boundary_report_digest
                ),
                "fri_qrom_residual_report_digest": hex_encode(
                    &evidence.fri_qrom_residual_report_digest
                ),
                "fri_aok_boundary_report_digest": hex_encode(
                    &evidence.fri_aok_boundary_report_digest
                ),
                "payload_byte_model_report_digest": hex_encode(
                    &evidence.payload_byte_model_report_digest
                ),
                "payload_kat_boundary_report_digest": hex_encode(
                    &evidence.payload_kat_boundary_report_digest
                ),
                "fixture_id": evidence.fixture_id,
                "proof_mask_seed": evidence.proof_mask_seed,
                "account_id": hex_encode(&evidence.account_id),
                "public_key_identity": hex_encode(&evidence.public_key_identity),
                "account_state_root_before": hex_encode(
                    &evidence.account_state_root_before
                ),
                "slot_id": evidence.slot_id,
                "prevalidated_slot_leaf_digest": hex_encode(
                    &evidence.prevalidated_slot_leaf_digest
                ),
                "slot_authorization_digest": hex_encode(
                    &evidence.slot_authorization_digest
                ),
                "account_creation_evidence_transcript_digest": hex_encode(
                    &evidence.account_creation_evidence_transcript_digest
                ),
                "direct_replayable_relation_certificate_digest": hex_encode(
                    &evidence.direct_replayable_relation_certificate_digest
                ),
                "selected_account_creation_payload_digest": hex_encode(
                    &evidence.selected_account_creation_payload_digest
                ),
                "creation_proof_binding_digest": hex_encode(
                    &evidence.creation_proof_binding_digest
                ),
                "leaf_context_digest": hex_encode(&evidence.leaf_context_digest),
                "aggregation_context_digest": hex_encode(
                    &evidence.aggregation_context_digest
                ),
                "native_role_root_commitment": hex_encode(
                    &evidence.native_role_root_commitment
                ),
                "masked_projection_digest": hex_encode(
                    &evidence.masked_projection_digest
                ),
                "deployed_anchor_digest": hex_encode(&evidence.deployed_anchor_digest),
                "payload_frame_digest": hex_encode(&evidence.payload_frame_digest),
                "payload_frame_bytes": evidence.payload_frame_bytes,
                "account_stark_bundle_digest": hex_encode(
                    &evidence.account_stark_bundle_digest
                ),
                "account_stark_bundle_bytes": evidence.account_stark_bundle_bytes,
                "decoded_public_anchor_digest": hex_encode(
                    &evidence.decoded_public_anchor_digest
                ),
                "verifier_boundary_accepts": evidence.verifier_boundary_accepts,
                "production_gate_fail_closed": evidence.production_gate_fail_closed,
                "proof_boundary_report_validated": evidence.proof_boundary_report_validated,
                "public_payload_exports_private_material":
                    evidence.public_payload_exports_private_material,
                "payload_digest_bound_to_certificate":
                    evidence.payload_digest_bound_to_certificate,
                "evidence_digest_bound_to_certificate":
                    evidence.evidence_digest_bound_to_certificate,
                "leaf_transcript_bound_to_evidence":
                    evidence.leaf_transcript_bound_to_evidence,
                "leaf_digest_bound_to_evidence": evidence.leaf_digest_bound_to_evidence,
                "leaf_context_bound": evidence.leaf_context_bound,
                "binding_digest_bound": evidence.binding_digest_bound,
                "payload_binding_fields_match_report":
                    evidence.payload_binding_fields_match_report,
                "decoded_public_anchors_match_deployed_anchors":
                    evidence.decoded_public_anchors_match_deployed_anchors,
                "deployed_anchor_account_id_bound":
                    evidence.deployed_anchor_account_id_bound,
                "deployed_anchor_slot_id_bound": evidence.deployed_anchor_slot_id_bound,
                "deployed_anchor_state_root_bound":
                    evidence.deployed_anchor_state_root_bound,
                "deployed_anchor_lifecycle_anchor_bound":
                    evidence.deployed_anchor_lifecycle_anchor_bound,
                "deployed_anchor_verifier_version_bound":
                    evidence.deployed_anchor_verifier_version_bound,
                "deployed_anchor_masked_projection_bound":
                    evidence.deployed_anchor_masked_projection_bound,
                "deployed_anchor_role_root_export_bridge_bound":
                    evidence.deployed_anchor_role_root_export_bridge_bound,
                "deployed_anchor_packed_witness_commitment_bound":
                    evidence.deployed_anchor_packed_witness_commitment_bound,
                "deployed_anchor_fields_match_report":
                    evidence.deployed_anchor_fields_match_report,
                "account_stark_bundle_nonempty": evidence.account_stark_bundle_nonempty,
                "proof_family_obligations_still_open":
                    evidence.proof_family_obligations_still_open,
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
