#[cfg(not(feature = "account-creation-stark-verifier"))]
fn main() -> Result<(), String> {
    Err(
        "path2_account_creation_accepted_proof_boundary_evidence_consistency requires --features account-creation-stark-verifier"
            .to_string(),
    )
}

#[cfg(feature = "account-creation-stark-verifier")]
mod enabled {
    use descender_parameter_search::{
        manifest_vector_evidence::path2_account_creation_accepted_proof_boundary_evidence_artifact_v1,
        production_reference::{
            path2_account_creation_accepted_proof_boundary_evidence_v1,
            path2_account_creation_accepted_proof_boundary_evidence_v1_digest,
        },
    };
    use serde_json::Value;
    use sha2::{Digest, Sha256};
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("parameter-search crate lives under tools/")
            .to_path_buf()
    }

    fn hex_encode(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            out.push(HEX[(byte >> 4) as usize] as char);
            out.push(HEX[(byte & 0x0f) as usize] as char);
        }
        out
    }

    fn load_json(relative_path: &str) -> Result<Value, String> {
        let path = repo_root().join(relative_path);
        let text =
            fs::read_to_string(&path).map_err(|err| format!("read {}: {err}", path.display()))?;
        serde_json::from_str(&text).map_err(|err| format!("parse {}: {err}", path.display()))
    }

    fn at<'a>(value: &'a Value, pointer: &str) -> Result<&'a Value, String> {
        value
            .pointer(pointer)
            .ok_or_else(|| format!("missing JSON pointer {pointer}"))
    }

    fn str_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
        at(value, pointer)?
            .as_str()
            .ok_or_else(|| format!("JSON pointer {pointer} is not a string"))
    }

    fn bool_at(value: &Value, pointer: &str) -> Result<bool, String> {
        at(value, pointer)?
            .as_bool()
            .ok_or_else(|| format!("JSON pointer {pointer} is not a bool"))
    }

    fn u64_at(value: &Value, pointer: &str) -> Result<u64, String> {
        at(value, pointer)?
            .as_u64()
            .ok_or_else(|| format!("JSON pointer {pointer} is not an unsigned integer"))
    }

    fn require_str(manifest: &Value, pointer: &str, expected: &str) -> Result<(), String> {
        let observed = str_at(manifest, pointer)?;
        if observed != expected {
            return Err(format!(
                "account-creation accepted proof-boundary manifest {pointer} mismatch: {observed:?} != {expected:?}"
            ));
        }
        Ok(())
    }

    fn require_hex(manifest: &Value, pointer: &str, expected: &[u8]) -> Result<(), String> {
        let observed = str_at(manifest, pointer)?;
        let expected_hex = hex_encode(expected);
        if observed != expected_hex {
            return Err(format!(
                "account-creation accepted proof-boundary manifest {pointer} mismatch: {observed} != {expected_hex}"
            ));
        }
        Ok(())
    }

    fn require_bool(manifest: &Value, pointer: &str, expected: bool) -> Result<(), String> {
        let observed = bool_at(manifest, pointer)?;
        if observed != expected {
            return Err(format!(
                "account-creation accepted proof-boundary manifest {pointer} mismatch: {observed} != {expected}"
            ));
        }
        Ok(())
    }

    fn body_digest(value: &Value, label: &str) -> Result<String, String> {
        let json = serde_json::to_string_pretty(value)
            .map_err(|err| format!("serialize {label}: {err}"))?;
        Ok(hex_encode(&Sha256::digest(json.as_bytes())))
    }

    fn require_manifest_body<'a>(file: &'a Value, label: &str) -> Result<&'a Value, String> {
        let body = at(file, "/manifest")?;
        let recomputed = body_digest(body, label)?;
        if str_at(file, "/manifest_digest")? != recomputed {
            return Err(format!("{label} manifest digest mismatch"));
        }
        Ok(body)
    }

    pub fn run() -> Result<(), String> {
        let manifest_file = load_json(
            "formal/kat/path2_account_creation_accepted_proof_boundary_evidence_v1.json",
        )?;
        let body = at(&manifest_file, "/manifest")?;
        let body_json = serde_json::to_string_pretty(body)
            .map_err(|err| format!("serialize accepted proof-boundary manifest body: {err}"))?;
        let recomputed_manifest_digest = hex_encode(&Sha256::digest(body_json.as_bytes()));
        if str_at(&manifest_file, "/manifest_digest")? != recomputed_manifest_digest {
            return Err(
                "account-creation accepted proof-boundary manifest digest mismatch".to_string(),
            );
        }
        require_str(
            body,
            "/schema",
            "Descender.Path2.AccountCreationAcceptedProofBoundaryDeterministicKatEvidenceManifest.v1",
        )?;
        require_str(
            body,
            "/status",
            "digest_pinned_deterministic_kat_accepted_account_creation_proof_boundary_evidence",
        )?;
        require_bool(
            body,
            "/deterministic_proof_byte_mode/evidence_digest_recomputed",
            true,
        )?;
        if u64_at(body, "/deterministic_proof_byte_mode/rayon_threads")? != 1 {
            return Err(
                "account-creation accepted proof-boundary manifest is not in one-thread KAT mode"
                    .to_string(),
            );
        }
        require_str(
            body,
            "/deterministic_proof_byte_mode/accepted_boundary_endpoint",
            "path2_account_creation_accepted_proof_boundary_evidence_v1",
        )?;

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .map_err(|err| {
                format!("failed to build deterministic single-thread KAT pool: {err}")
            })?;
        let evidence = pool.install(path2_account_creation_accepted_proof_boundary_evidence_v1)?;
        let evidence_digest =
            path2_account_creation_accepted_proof_boundary_evidence_v1_digest(&evidence);
        if evidence_digest != evidence.digest {
            return Err(
                "account-creation accepted proof-boundary evidence recompute mismatch".to_string(),
            );
        }

        require_hex(body, "/evidence/digest", &evidence.digest)?;
        require_hex(body, "/evidence/recomputed_digest", &evidence_digest)?;
        require_str(body, "/evidence/schema", evidence.schema)?;
        require_str(
            body,
            "/evidence/proof_boundary_target",
            evidence.proof_boundary_target,
        )?;
        require_str(
            body,
            "/evidence/proof_boundary_conclusion",
            evidence.proof_boundary_conclusion,
        )?;
        require_str(
            body,
            "/evidence/selected_payload_type",
            evidence.selected_payload_type,
        )?;
        require_str(
            body,
            "/evidence/verifier_boundary_endpoint",
            evidence.verifier_boundary_endpoint,
        )?;
        require_str(
            body,
            "/evidence/production_boundary_endpoint",
            evidence.production_boundary_endpoint,
        )?;
        require_str(
            body,
            "/evidence/replay_payload_digest_function",
            evidence.replay_payload_digest_function,
        )?;

        let requirements = at(body, "/evidence/accepting_requirements")?
            .as_array()
            .ok_or_else(|| "accepted proof-boundary requirements are not an array".to_string())?;
        if requirements.len() != evidence.accepting_requirements.len() {
            return Err(
                "account-creation accepted proof-boundary requirement count mismatch".to_string(),
            );
        }
        for (index, expected) in evidence.accepting_requirements.iter().enumerate() {
            if requirements[index].as_str() != Some(*expected) {
                return Err(format!(
                    "account-creation accepted proof-boundary requirement {index} drifted"
                ));
            }
        }

        require_hex(
            body,
            "/evidence/account_creation_proof_boundary_report_digest",
            &evidence.account_creation_proof_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/sampler_timing_report_digest",
            &evidence.sampler_timing_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/no_export_boundary_report_digest",
            &evidence.no_export_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/account_stark_residual_matrix_report_digest",
            &evidence.account_stark_residual_matrix_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/frame_kat_boundary_report_digest",
            &evidence.frame_kat_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/fri_qrom_residual_report_digest",
            &evidence.fri_qrom_residual_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/fri_aok_boundary_report_digest",
            &evidence.fri_aok_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/payload_byte_model_report_digest",
            &evidence.payload_byte_model_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/payload_kat_boundary_report_digest",
            &evidence.payload_kat_boundary_report_digest,
        )?;
        require_str(body, "/evidence/fixture_id", evidence.fixture_id)?;
        if u64_at(body, "/evidence/proof_mask_seed")? != evidence.proof_mask_seed
            || u64_at(body, "/evidence/slot_id")? != evidence.slot_id
            || u64_at(body, "/evidence/payload_frame_bytes")? != evidence.payload_frame_bytes as u64
            || u64_at(body, "/evidence/account_stark_bundle_bytes")?
                != evidence.account_stark_bundle_bytes as u64
        {
            return Err(
                "account-creation accepted proof-boundary scalar field mismatch".to_string(),
            );
        }

        for (pointer, expected) in [
            ("/evidence/account_id", &evidence.account_id[..]),
            (
                "/evidence/public_key_identity",
                &evidence.public_key_identity[..],
            ),
            (
                "/evidence/account_state_root_before",
                &evidence.account_state_root_before[..],
            ),
            (
                "/evidence/prevalidated_slot_leaf_digest",
                &evidence.prevalidated_slot_leaf_digest[..],
            ),
            (
                "/evidence/slot_authorization_digest",
                &evidence.slot_authorization_digest[..],
            ),
            (
                "/evidence/account_creation_evidence_transcript_digest",
                &evidence.account_creation_evidence_transcript_digest[..],
            ),
            (
                "/evidence/direct_replayable_relation_certificate_digest",
                &evidence.direct_replayable_relation_certificate_digest[..],
            ),
            (
                "/evidence/selected_account_creation_payload_digest",
                &evidence.selected_account_creation_payload_digest[..],
            ),
            (
                "/evidence/creation_proof_binding_digest",
                &evidence.creation_proof_binding_digest[..],
            ),
            (
                "/evidence/leaf_context_digest",
                &evidence.leaf_context_digest[..],
            ),
            (
                "/evidence/aggregation_context_digest",
                &evidence.aggregation_context_digest[..],
            ),
            (
                "/evidence/native_role_root_commitment",
                &evidence.native_role_root_commitment[..],
            ),
            (
                "/evidence/masked_projection_digest",
                &evidence.masked_projection_digest[..],
            ),
            (
                "/evidence/deployed_anchor_digest",
                &evidence.deployed_anchor_digest[..],
            ),
            (
                "/evidence/payload_frame_digest",
                &evidence.payload_frame_digest[..],
            ),
            (
                "/evidence/account_stark_bundle_digest",
                &evidence.account_stark_bundle_digest[..],
            ),
            (
                "/evidence/decoded_public_anchor_digest",
                &evidence.decoded_public_anchor_digest[..],
            ),
        ] {
            require_hex(body, pointer, expected)?;
        }

        for pointer in [
            "/evidence/verifier_boundary_accepts",
            "/evidence/production_gate_fail_closed",
            "/evidence/proof_boundary_report_validated",
            "/evidence/payload_digest_bound_to_certificate",
            "/evidence/evidence_digest_bound_to_certificate",
            "/evidence/leaf_transcript_bound_to_evidence",
            "/evidence/leaf_digest_bound_to_evidence",
            "/evidence/leaf_context_bound",
            "/evidence/binding_digest_bound",
            "/evidence/payload_binding_fields_match_report",
            "/evidence/decoded_public_anchors_match_deployed_anchors",
            "/evidence/deployed_anchor_account_id_bound",
            "/evidence/deployed_anchor_slot_id_bound",
            "/evidence/deployed_anchor_state_root_bound",
            "/evidence/deployed_anchor_lifecycle_anchor_bound",
            "/evidence/deployed_anchor_verifier_version_bound",
            "/evidence/deployed_anchor_masked_projection_bound",
            "/evidence/deployed_anchor_role_root_export_bridge_bound",
            "/evidence/deployed_anchor_packed_witness_commitment_bound",
            "/evidence/deployed_anchor_fields_match_report",
            "/evidence/account_stark_bundle_nonempty",
            "/evidence/proof_family_obligations_still_open",
        ] {
            require_bool(body, pointer, true)?;
        }
        for pointer in [
            "/evidence/public_payload_exports_private_material",
            "/evidence/deployment_ready",
        ] {
            require_bool(body, pointer, false)?;
        }

        let accepted_artifact =
            path2_account_creation_accepted_proof_boundary_evidence_artifact_v1()?;
        let proof_manifest_file =
            load_json("formal/kat/path2_account_creation_proof_boundary_v1.json")?;
        let proof_manifest = require_manifest_body(
            &proof_manifest_file,
            "account-creation proof-boundary manifest",
        )?;
        let direct_manifest_file = load_json("formal/kat/path2_direct_theorem_boundary_v1.json")?;
        let direct_manifest =
            require_manifest_body(&direct_manifest_file, "direct theorem boundary manifest")?;
        let proof_boundary = at(proof_manifest, "/account_creation_proof_boundary")?;
        let direct_boundary = at(direct_manifest, "/direct_theorem_boundary")?;

        if proof_boundary != at(direct_manifest, "/account_creation_proof_boundary")? {
            return Err(
                "accepted proof-boundary evidence lost shared proof-boundary manifest binding"
                    .to_string(),
            );
        }
        if at(proof_boundary, "/accepted_proof_boundary_evidence_artifact")? != &accepted_artifact
            || at(
                proof_boundary,
                "/proof_obligation_vector_corpus_evidence/accepted_proof_boundary_evidence_artifact",
            )? != &accepted_artifact
            || at(
                proof_boundary,
                "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/account_creation_accepted_proof_boundary_evidence_artifact",
            )? != &accepted_artifact
            || at(
                direct_boundary,
                "/direct_accepted_composition_evidence_artifact/account_creation_accepted_proof_boundary_evidence_artifact",
            )? != &accepted_artifact
            || at(
                direct_boundary,
                "/direct_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/account_creation_accepted_proof_boundary_evidence_artifact",
            )? != &accepted_artifact
        {
            return Err(
                "accepted proof-boundary evidence artifact drifted across generated manifests"
                    .to_string(),
            );
        }
        require_str(
            proof_manifest,
            "/digests/account_creation_proof_boundary_report_digest",
            str_at(
                body,
                "/evidence/account_creation_proof_boundary_report_digest",
            )?,
        )?;
        require_str(
            proof_manifest,
            "/digests/account_creation_sampler_timing_report_digest",
            str_at(body, "/evidence/sampler_timing_report_digest")?,
        )?;
        require_str(
            proof_manifest,
            "/digests/account_creation_no_export_boundary_report_digest",
            str_at(body, "/evidence/no_export_boundary_report_digest")?,
        )?;
        require_str(
            proof_manifest,
            "/digests/account_stark_residual_matrix_report_digest",
            str_at(
                body,
                "/evidence/account_stark_residual_matrix_report_digest",
            )?,
        )?;
        require_str(
            proof_manifest,
            "/digests/account_creation_frame_kat_boundary_report_digest",
            str_at(body, "/evidence/frame_kat_boundary_report_digest")?,
        )?;
        require_str(
            proof_manifest,
            "/digests/account_stark_fri_qrom_residual_report_digest",
            str_at(body, "/evidence/fri_qrom_residual_report_digest")?,
        )?;
        require_str(
            proof_manifest,
            "/digests/account_stark_fri_aok_boundary_report_digest",
            str_at(body, "/evidence/fri_aok_boundary_report_digest")?,
        )?;
        require_str(
            proof_manifest,
            "/digests/payload_kat_boundary_report_digest",
            str_at(body, "/evidence/payload_kat_boundary_report_digest")?,
        )?;
        for pointer in [
            "/accepted_proof_boundary_evidence_artifact/audit_summary/proof_family_obligations_still_open",
            "/accepted_proof_boundary_evidence_artifact/audit_summary/production_gate_fail_closed",
            "/proof_obligation_vector_corpus_evidence/audit_summary/all_cases_open",
            "/proof_obligation_vector_corpus_evidence/audit_summary/all_cases_have_named_break",
            "/proof_obligation_vector_corpus_evidence/audit_summary/all_open_cases_block_deployment",
            "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/proof_family_obligations_still_open",
        ] {
            require_bool(proof_boundary, pointer, true)?;
        }
        for pointer in [
            "/accepted_proof_boundary_evidence_artifact/audit_summary/deployment_ready",
            "/proof_obligation_vector_corpus_evidence/audit_summary/simulator_theorem_instantiated",
            "/proof_obligation_vector_corpus_evidence/audit_summary/fri_aok_extractor_theorem_instantiated",
            "/proof_obligation_vector_corpus_evidence/audit_summary/fri_qrom_residual_priced",
            "/proof_obligation_vector_corpus_evidence/audit_summary/frozen_kat_corpus_complete",
            "/proof_obligation_vector_corpus_evidence/audit_summary/production_gate_open",
            "/proof_obligation_vector_corpus_evidence/audit_summary/deployment_ready",
            "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/account_creation_proof_boundary_verified",
            "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/deployment_ready",
        ] {
            require_bool(proof_boundary, pointer, false)?;
        }

        let payload_vectors =
            load_json("formal/kat/path2_account_stark_payload_kat_vectors_v1.json")?;
        require_hex(
            &payload_vectors,
            "/corpus/honest_payload/payload_digest",
            &evidence.selected_account_creation_payload_digest,
        )?;

        println!(
            "Path 2 account-creation accepted proof-boundary evidence consistency check passed."
        );
        Ok(())
    }
}

#[cfg(feature = "account-creation-stark-verifier")]
fn main() -> Result<(), String> {
    enabled::run()
}
