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
    use descender_parameter_search::production_reference::{
        path2_direct_accepted_composition_evidence_v1,
        path2_direct_accepted_composition_evidence_v1_digest, PROFILE,
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

        Ok(json!({
            "schema": "Descender.Path2.DirectAcceptedCompositionStableSurfaceManifest.v1",
            "corpus_id": "Descender.Path2.DirectAcceptedCompositionStableSurface.v1",
            "corpus_version": 1,
            "profile_id": PROFILE,
            "status": "stable_public_surface_pinned_accepted_direct_composition_evidence",
            "generator": {
                "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --bin path2_direct_accepted_composition_evidence_manifest",
                "source": "tools/parameter-search/src/bin/path2_direct_accepted_composition_evidence_manifest.rs"
            },
            "current_run_checks": {
                "accepted_composition_endpoint": "path2_direct_accepted_composition_evidence_v1",
                "evidence_digest_recomputed_before_surface_projection": true,
                "evidence_digest_policy": "current-run sanity check only; not frozen in this manifest while account-STARK proof-bundle bytes are not a stable checked-in KAT"
            },
            "stable_surface": {
                "evidence_schema": evidence.schema,
                "theorem_target": evidence.theorem_target,
                "theorem_conclusion": evidence.theorem_conclusion,
                "direct_theorem_boundary_report_digest": hex_encode(
                    &evidence.direct_theorem_boundary_report_digest
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
                "direct_theorem_obligations_still_open":
                    evidence.direct_theorem_obligations_still_open,
                "proof_family_obligations_still_open":
                    evidence.proof_family_obligations_still_open,
                "composition_theorem_instantiated":
                    evidence.composition_theorem_instantiated,
                "account_creation_proof_boundary_verified":
                    evidence.account_creation_proof_boundary_verified,
                "deployment_ready": evidence.deployment_ready
            },
            "non_frozen_current_run_fields": [
                "Path2DirectAcceptedCompositionEvidenceV1.digest",
                "Path2DirectAcceptedCompositionEvidenceV1.direct_accepted_spend_tuple_evidence_digest",
                "Path2DirectAcceptedCompositionEvidenceV1.account_creation_accepted_proof_boundary_evidence_digest",
                "Path2DirectAcceptedCompositionEvidenceV1.prevalidated_slot_leaf_digest",
                "Path2DirectAcceptedCompositionEvidenceV1.account_creation_evidence_transcript_digest",
                "Path2DirectAcceptedCompositionEvidenceV1.selected_account_creation_payload_digest",
                "Path2DirectAcceptedCompositionEvidenceV1.slot_authorization_digest",
                "Path2DirectAcceptedCompositionEvidenceV1.direct_admission_namespace_record_digest",
                "Path2DirectAcceptedCompositionEvidenceV1.consume_once_state_before_digest",
                "Path2DirectAcceptedCompositionEvidenceV1.consume_once_state_after_digest"
            ],
            "non_frozen_policy": "These fields are accepted-path current-run evidence while the account-STARK proof bundle is not a frozen deterministic byte corpus; this manifest pins only the stable public composition surface and fail-closed closure flags."
        }))
    }

    pub fn run() -> Result<(), String> {
        let body = manifest_body()?;
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
