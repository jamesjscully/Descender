#[cfg(not(feature = "account-creation-stark-verifier"))]
fn main() -> Result<(), String> {
    Err(
        "path2_direct_accepted_composition_evidence_consistency requires --features account-creation-stark-verifier"
            .to_string(),
    )
}

#[cfg(feature = "account-creation-stark-verifier")]
mod enabled {
    use descender_parameter_search::{
        manifest_vector_evidence::path2_direct_accepted_composition_evidence_artifact_v1,
        production_reference::{
            path2_direct_accepted_composition_evidence_v1,
            path2_direct_accepted_composition_evidence_v1_digest,
            validate_path2_direct_accepted_composition_evidence_v1,
        },
    };
    use serde_json::{Value, to_value};
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

    fn require_hex(manifest: &Value, pointer: &str, expected: &[u8]) -> Result<(), String> {
        let observed = str_at(manifest, pointer)?;
        let expected_hex = hex_encode(expected);
        if observed != expected_hex {
            return Err(format!(
                "direct accepted composition manifest {pointer} mismatch: {observed} != {expected_hex}"
            ));
        }
        Ok(())
    }

    fn require_str(manifest: &Value, pointer: &str, expected: &str) -> Result<(), String> {
        let observed = str_at(manifest, pointer)?;
        if observed != expected {
            return Err(format!(
                "direct accepted composition manifest {pointer} mismatch: {observed:?} != {expected:?}"
            ));
        }
        Ok(())
    }

    fn require_bool(manifest: &Value, pointer: &str, expected: bool) -> Result<(), String> {
        let observed = bool_at(manifest, pointer)?;
        if observed != expected {
            return Err(format!(
                "direct accepted composition manifest {pointer} mismatch: {observed} != {expected}"
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
        let manifest_file =
            load_json("formal/kat/path2_direct_accepted_composition_evidence_v1.json")?;
        let body = at(&manifest_file, "/manifest")?;
        let body_json = serde_json::to_string_pretty(body)
            .map_err(|err| format!("serialize direct accepted composition body: {err}"))?;
        let recomputed_manifest_digest = hex_encode(&Sha256::digest(body_json.as_bytes()));
        if str_at(&manifest_file, "/manifest_digest")? != recomputed_manifest_digest {
            return Err("direct accepted composition manifest digest mismatch".to_string());
        }
        if str_at(body, "/schema")?
            != "Descender.Path2.DirectAcceptedCompositionDeterministicKatEvidenceManifest.v1"
        {
            return Err("direct accepted composition manifest schema mismatch".to_string());
        }
        if str_at(body, "/status")?
            != "digest_pinned_deterministic_kat_accepted_direct_composition_evidence"
        {
            return Err("direct accepted composition manifest status mismatch".to_string());
        }
        if !bool_at(
            body,
            "/deterministic_proof_byte_mode/evidence_digest_recomputed",
        )? {
            return Err(
                "direct accepted composition manifest lost deterministic digest sanity check"
                    .to_string(),
            );
        }
        if u64_at(body, "/deterministic_proof_byte_mode/rayon_threads")? != 1 {
            return Err(
                "direct accepted composition manifest is not in one-thread KAT mode".to_string(),
            );
        }

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .map_err(|err| {
                format!("failed to build deterministic single-thread KAT pool: {err}")
            })?;
        let evidence = pool.install(path2_direct_accepted_composition_evidence_v1)?;
        validate_path2_direct_accepted_composition_evidence_v1(&evidence)?;
        let evidence_digest = path2_direct_accepted_composition_evidence_v1_digest(&evidence);
        if evidence_digest != evidence.digest {
            return Err("direct accepted composition evidence recompute mismatch".to_string());
        }

        require_hex(body, "/evidence/digest", &evidence.digest)?;
        require_hex(body, "/evidence/recomputed_digest", &evidence_digest)?;
        require_hex(
            body,
            "/evidence/direct_theorem_boundary_report_digest",
            &evidence.direct_theorem_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/direct_accepted_spend_tuple_evidence_digest",
            &evidence.direct_accepted_spend_tuple_evidence_digest,
        )?;
        require_hex(
            body,
            "/evidence/account_creation_accepted_proof_boundary_evidence_digest",
            &evidence.account_creation_accepted_proof_boundary_evidence_digest,
        )?;
        require_hex(
            body,
            "/evidence/direct_qrom_action_boundary_report_digest",
            &evidence.direct_qrom_action_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/direct_lifecycle_boundary_report_digest",
            &evidence.direct_lifecycle_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/account_creation_proof_boundary_report_digest",
            &evidence.account_creation_proof_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/evidence/public_key_identity",
            &evidence.public_key_identity,
        )?;
        require_hex(body, "/evidence/message_digest", &evidence.message_digest)?;
        require_hex(body, "/evidence/account_id", &evidence.account_id)?;
        require_hex(
            body,
            "/evidence/account_state_root_before",
            &evidence.account_state_root_before,
        )?;
        require_hex(
            body,
            "/evidence/prevalidated_slot_leaf_digest",
            &evidence.prevalidated_slot_leaf_digest,
        )?;
        require_hex(
            body,
            "/evidence/account_creation_evidence_transcript_digest",
            &evidence.account_creation_evidence_transcript_digest,
        )?;
        require_hex(
            body,
            "/evidence/selected_account_creation_payload_digest",
            &evidence.selected_account_creation_payload_digest,
        )?;
        require_hex(
            body,
            "/evidence/slot_authorization_digest",
            &evidence.slot_authorization_digest,
        )?;
        require_hex(
            body,
            "/evidence/direct_admission_namespace_record_digest",
            &evidence.direct_admission_namespace_record_digest,
        )?;
        require_hex(
            body,
            "/evidence/consume_once_state_before_digest",
            &evidence.consume_once_state_before_digest,
        )?;
        require_hex(
            body,
            "/evidence/consume_once_state_after_digest",
            &evidence.consume_once_state_after_digest,
        )?;
        if str_at(body, "/evidence/schema")? != evidence.schema
            || str_at(body, "/evidence/theorem_target")? != evidence.theorem_target
            || str_at(body, "/evidence/theorem_conclusion")? != evidence.theorem_conclusion
            || str_at(body, "/evidence/account_creation_fixture_id")?
                != evidence.account_creation_fixture_id
            || u64_at(body, "/evidence/proof_mask_seed")? != evidence.proof_mask_seed
            || u64_at(body, "/evidence/slot_id")? != evidence.slot_id
        {
            return Err("direct accepted composition manifest scalar field mismatch".to_string());
        }
        let accepted_boundary = load_json(
            "formal/kat/path2_account_creation_accepted_proof_boundary_evidence_v1.json",
        )?;
        let accepted_boundary_body = at(&accepted_boundary, "/manifest")?;
        let accepted_boundary_json = serde_json::to_string_pretty(accepted_boundary_body)
            .map_err(|err| format!("serialize accepted proof-boundary manifest body: {err}"))?;
        let accepted_boundary_recomputed_digest =
            hex_encode(&Sha256::digest(accepted_boundary_json.as_bytes()));
        if str_at(&accepted_boundary, "/manifest_digest")? != accepted_boundary_recomputed_digest {
            return Err(
                "account-creation accepted proof-boundary manifest digest mismatch".to_string(),
            );
        }
        if str_at(accepted_boundary_body, "/schema")?
            != "Descender.Path2.AccountCreationAcceptedProofBoundaryDeterministicKatEvidenceManifest.v1"
            || str_at(accepted_boundary_body, "/status")?
                != "digest_pinned_deterministic_kat_accepted_account_creation_proof_boundary_evidence"
            || !bool_at(
                accepted_boundary_body,
                "/deterministic_proof_byte_mode/evidence_digest_recomputed",
            )?
            || u64_at(
                accepted_boundary_body,
                "/deterministic_proof_byte_mode/rayon_threads",
            )? != 1
        {
            return Err(
                "account-creation accepted proof-boundary manifest mode drifted".to_string(),
            );
        }
        if str_at(
            body,
            "/evidence/account_creation_accepted_proof_boundary_evidence_digest",
        )? != str_at(accepted_boundary_body, "/evidence/digest")?
            || str_at(body, "/evidence/account_creation_fixture_id")?
                != str_at(accepted_boundary_body, "/evidence/fixture_id")?
            || u64_at(body, "/evidence/proof_mask_seed")?
                != u64_at(accepted_boundary_body, "/evidence/proof_mask_seed")?
            || str_at(body, "/evidence/public_key_identity")?
                != str_at(accepted_boundary_body, "/evidence/public_key_identity")?
            || str_at(body, "/evidence/account_id")?
                != str_at(accepted_boundary_body, "/evidence/account_id")?
            || str_at(body, "/evidence/account_state_root_before")?
                != str_at(
                    accepted_boundary_body,
                    "/evidence/account_state_root_before",
                )?
            || u64_at(body, "/evidence/slot_id")?
                != u64_at(accepted_boundary_body, "/evidence/slot_id")?
            || str_at(body, "/evidence/prevalidated_slot_leaf_digest")?
                != str_at(
                    accepted_boundary_body,
                    "/evidence/prevalidated_slot_leaf_digest",
                )?
            || str_at(
                body,
                "/evidence/account_creation_evidence_transcript_digest",
            )? != str_at(
                accepted_boundary_body,
                "/evidence/account_creation_evidence_transcript_digest",
            )?
            || str_at(body, "/evidence/selected_account_creation_payload_digest")?
                != str_at(
                    accepted_boundary_body,
                    "/evidence/selected_account_creation_payload_digest",
                )?
            || str_at(body, "/evidence/slot_authorization_digest")?
                != str_at(
                    accepted_boundary_body,
                    "/evidence/slot_authorization_digest",
                )?
            || str_at(
                body,
                "/evidence/account_creation_proof_boundary_report_digest",
            )? != str_at(
                accepted_boundary_body,
                "/evidence/account_creation_proof_boundary_report_digest",
            )?
            || bool_at(body, "/evidence/verifier_boundary_accepts_creation")?
                != bool_at(
                    accepted_boundary_body,
                    "/evidence/verifier_boundary_accepts",
                )?
            || bool_at(body, "/evidence/proof_family_obligations_still_open")?
                != bool_at(
                    accepted_boundary_body,
                    "/evidence/proof_family_obligations_still_open",
                )?
        {
            return Err(
                "direct accepted composition manifest drifted from standalone accepted proof-boundary artifact"
                    .to_string(),
            );
        }
        if bool_at(
            accepted_boundary_body,
            "/evidence/public_payload_exports_private_material",
        )? || !bool_at(
            accepted_boundary_body,
            "/evidence/production_gate_fail_closed",
        )? || bool_at(accepted_boundary_body, "/evidence/deployment_ready")?
        {
            return Err(
                "accepted proof-boundary manifest lost fail-closed/no-export evidence".to_string(),
            );
        }
        let embedded_boundary_artifact = at(
            body,
            "/accepted_input_artifacts/account_creation_accepted_proof_boundary_evidence_artifact",
        )?;
        if str_at(embedded_boundary_artifact, "/artifact_manifest_digest")?
            != str_at(&accepted_boundary, "/manifest_digest")?
            || str_at(
                embedded_boundary_artifact,
                "/artifact_manifest_recomputed_digest",
            )? != accepted_boundary_recomputed_digest
            || str_at(embedded_boundary_artifact, "/evidence_digest")?
                != str_at(accepted_boundary_body, "/evidence/digest")?
            || str_at(embedded_boundary_artifact, "/evidence_recomputed_digest")?
                != str_at(accepted_boundary_body, "/evidence/recomputed_digest")?
            || str_at(
                embedded_boundary_artifact,
                "/account_creation_proof_boundary_report_digest",
            )? != str_at(
                accepted_boundary_body,
                "/evidence/account_creation_proof_boundary_report_digest",
            )?
            || bool_at(
                embedded_boundary_artifact,
                "/audit_summary/verifier_boundary_accepts",
            )? != bool_at(
                accepted_boundary_body,
                "/evidence/verifier_boundary_accepts",
            )?
            || bool_at(
                embedded_boundary_artifact,
                "/audit_summary/proof_family_obligations_still_open",
            )? != bool_at(
                accepted_boundary_body,
                "/evidence/proof_family_obligations_still_open",
            )?
            || bool_at(
                embedded_boundary_artifact,
                "/audit_summary/public_payload_exports_private_material",
            )? != bool_at(
                accepted_boundary_body,
                "/evidence/public_payload_exports_private_material",
            )?
            || bool_at(
                embedded_boundary_artifact,
                "/audit_summary/deployment_ready",
            )? != bool_at(accepted_boundary_body, "/evidence/deployment_ready")?
        {
            return Err(
                "direct accepted composition manifest embedded proof-boundary artifact drifted"
                    .to_string(),
            );
        }

        for pointer in [
            "/evidence/creation_evidence_matches_direct_tuple",
            "/evidence/proof_boundary_digest_matches_direct_theorem",
            "/evidence/qrom_action_digest_matches_direct_theorem",
            "/evidence/qrom_action_budget_bridge_imported",
            "/evidence/qrom_action_theorem_linkage_reduction_imported",
            "/evidence/qrom_action_review_hypothesis_explicit",
            "/evidence/qrom_action_external_review_still_open",
            "/evidence/lifecycle_digest_matches_direct_theorem",
            "/evidence/direct_admission_namespace_matches_authorization",
            "/evidence/consume_once_transition_matches_tuple",
            "/evidence/verifier_boundary_accepts_creation",
            "/evidence/direct_spend_tuple_accepts",
            "/evidence/public_result_accepts_same_slot",
            "/evidence/every_positive_acceptance_hypothesis_has_accepted_path_evidence",
            "/evidence/direct_theorem_obligations_still_open",
            "/evidence/proof_family_obligations_still_open",
        ] {
            if !bool_at(body, pointer)? {
                return Err(format!(
                    "direct accepted composition manifest unexpectedly lost true flag {pointer}"
                ));
            }
        }
        let expected_hypothesis_rows = to_value(&evidence.accepted_hypothesis_evidence_rows)
            .map_err(|err| format!("serialize accepted hypothesis evidence rows: {err}"))?;
        if u64_at(body, "/evidence/accepted_hypothesis_evidence_count")?
            != evidence.accepted_hypothesis_evidence_count as u64
            || u64_at(
                body,
                "/evidence/positive_acceptance_hypothesis_evidence_count",
            )? != evidence.positive_acceptance_hypothesis_evidence_count as u64
            || evidence.accepted_hypothesis_evidence_count != 9
            || evidence.positive_acceptance_hypothesis_evidence_count != 8
            || at(body, "/evidence/accepted_hypothesis_evidence_rows")? != &expected_hypothesis_rows
        {
            return Err(
                "direct accepted composition manifest lost accepted-hypothesis evidence rows"
                    .to_string(),
            );
        }
        for pointer in [
            "/evidence/public_rejection_hypothesis_deferred_to_rejection_kats",
            "/evidence/composition_theorem_instantiated",
            "/evidence/account_creation_proof_boundary_verified",
            "/evidence/deployment_ready",
        ] {
            if bool_at(body, pointer)? {
                return Err(format!(
                    "direct accepted composition manifest unexpectedly claims closure at {pointer}"
                ));
            }
        }
        let accepted_tuple =
            load_json("formal/kat/path2_direct_accepted_spend_tuple_evidence_v1.json")?;
        let accepted_tuple_body = at(&accepted_tuple, "/manifest")?;
        let accepted_tuple_json = serde_json::to_string_pretty(accepted_tuple_body)
            .map_err(|err| format!("serialize accepted tuple manifest body: {err}"))?;
        let accepted_tuple_recomputed_digest =
            hex_encode(&Sha256::digest(accepted_tuple_json.as_bytes()));
        if str_at(&accepted_tuple, "/manifest_digest")? != accepted_tuple_recomputed_digest {
            return Err("direct accepted spend tuple manifest digest mismatch".to_string());
        }
        if str_at(
            body,
            "/evidence/direct_accepted_spend_tuple_evidence_digest",
        )? != str_at(accepted_tuple_body, "/evidence/digest")?
            || str_at(body, "/evidence/public_key_identity")?
                != str_at(accepted_tuple_body, "/evidence/public_key_identity")?
            || str_at(body, "/evidence/message_digest")?
                != str_at(accepted_tuple_body, "/evidence/message_digest")?
            || str_at(body, "/evidence/account_id")?
                != str_at(accepted_tuple_body, "/evidence/account_id")?
            || str_at(body, "/evidence/account_state_root_before")?
                != str_at(accepted_tuple_body, "/evidence/account_state_root_before")?
            || u64_at(body, "/evidence/slot_id")?
                != u64_at(accepted_tuple_body, "/evidence/slot_id")?
            || str_at(body, "/evidence/prevalidated_slot_leaf_digest")?
                != str_at(
                    accepted_tuple_body,
                    "/evidence/prevalidated_slot_leaf_digest",
                )?
            || str_at(
                body,
                "/evidence/account_creation_evidence_transcript_digest",
            )? != str_at(
                accepted_tuple_body,
                "/evidence/account_creation_evidence_transcript_digest",
            )?
            || str_at(body, "/evidence/selected_account_creation_payload_digest")?
                != str_at(
                    accepted_tuple_body,
                    "/evidence/selected_account_creation_payload_digest",
                )?
            || str_at(body, "/evidence/slot_authorization_digest")?
                != str_at(accepted_tuple_body, "/evidence/slot_authorization_digest")?
            || str_at(body, "/evidence/direct_admission_namespace_record_digest")?
                != str_at(
                    accepted_tuple_body,
                    "/evidence/direct_admission_namespace_record_digest",
                )?
            || str_at(body, "/evidence/consume_once_state_before_digest")?
                != str_at(
                    accepted_tuple_body,
                    "/evidence/consume_once_state_before_digest",
                )?
            || str_at(body, "/evidence/consume_once_state_after_digest")?
                != str_at(
                    accepted_tuple_body,
                    "/evidence/consume_once_state_after_digest",
                )?
            || !bool_at(
                accepted_tuple_body,
                "/evidence/replay_rejected_by_consume_once",
            )?
        {
            return Err(
                "direct accepted composition manifest drifted from standalone accepted spend tuple artifact"
                    .to_string(),
            );
        }
        let embedded_tuple_artifact = at(
            body,
            "/accepted_input_artifacts/direct_accepted_spend_tuple_evidence_artifact",
        )?;
        if str_at(embedded_tuple_artifact, "/artifact_manifest_digest")?
            != str_at(&accepted_tuple, "/manifest_digest")?
            || str_at(
                embedded_tuple_artifact,
                "/artifact_manifest_recomputed_digest",
            )? != accepted_tuple_recomputed_digest
            || str_at(embedded_tuple_artifact, "/evidence_digest")?
                != str_at(accepted_tuple_body, "/evidence/digest")?
            || str_at(embedded_tuple_artifact, "/evidence_recomputed_digest")?
                != str_at(accepted_tuple_body, "/evidence/recomputed_digest")?
            || str_at(
                embedded_tuple_artifact,
                "/direct_theorem_boundary_report_digest",
            )? != str_at(body, "/evidence/direct_theorem_boundary_report_digest")?
            || str_at(
                embedded_tuple_artifact,
                "/account_creation_proof_boundary_report_digest",
            )? != str_at(
                body,
                "/evidence/account_creation_proof_boundary_report_digest",
            )?
            || bool_at(
                embedded_tuple_artifact,
                "/audit_summary/replay_rejected_by_consume_once",
            )? != bool_at(
                accepted_tuple_body,
                "/evidence/replay_rejected_by_consume_once",
            )?
            || bool_at(
                embedded_tuple_artifact,
                "/audit_summary/tuple_fields_match_theorem_boundary",
            )? != bool_at(
                accepted_tuple_body,
                "/evidence/tuple_fields_match_theorem_boundary",
            )?
        {
            return Err(
                "direct accepted composition manifest embedded spend tuple artifact drifted"
                    .to_string(),
            );
        }

        let accepted_composition_artifact =
            path2_direct_accepted_composition_evidence_artifact_v1()?;
        let direct_manifest_file = load_json("formal/kat/path2_direct_theorem_boundary_v1.json")?;
        let direct_manifest =
            require_manifest_body(&direct_manifest_file, "direct theorem boundary manifest")?;
        let proof_manifest_file =
            load_json("formal/kat/path2_account_creation_proof_boundary_v1.json")?;
        let proof_manifest = require_manifest_body(
            &proof_manifest_file,
            "account-creation proof-boundary manifest",
        )?;
        let direct_boundary = at(direct_manifest, "/direct_theorem_boundary")?;
        let proof_boundary = at(proof_manifest, "/account_creation_proof_boundary")?;
        if proof_boundary != at(direct_manifest, "/account_creation_proof_boundary")? {
            return Err(
                "direct accepted composition evidence lost shared proof-boundary manifest binding"
                    .to_string(),
            );
        }
        require_bool(
            direct_boundary,
            "/coverage/every_closure_dependency_has_open_obligation",
            true,
        )?;
        require_bool(
            direct_boundary,
            "/coverage/every_closure_dependency_has_open_support_surface",
            true,
        )?;
        require_bool(
            direct_boundary,
            "/coverage/every_open_closure_dependency_blocks_deployment",
            true,
        )?;
        if u64_at(direct_boundary, "/coverage/closure_dependency_count")? != 9 {
            return Err(
                "direct accepted composition evidence lost direct closure dependency count"
                    .to_string(),
            );
        }
        for pointer in [
            "/coverage/every_closure_dependency_has_open_obligation",
            "/coverage/every_closure_dependency_has_open_support_surface",
            "/coverage/every_open_closure_dependency_blocks_deployment",
        ] {
            require_bool(proof_boundary, pointer, true)?;
        }
        for (observed, label) in [
            (
                at(
                    direct_boundary,
                    "/direct_accepted_composition_evidence_artifact",
                )?,
                "direct theorem boundary composition artifact",
            ),
            (
                at(
                    direct_boundary,
                    "/direct_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact",
                )?,
                "direct theorem obligation corpus composition artifact",
            ),
            (
                at(
                    direct_manifest,
                    "/account_creation_proof_boundary/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact",
                )?,
                "direct manifest proof-obligation corpus composition artifact",
            ),
            (
                at(
                    proof_boundary,
                    "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact",
                )?,
                "proof manifest proof-obligation corpus composition artifact",
            ),
        ] {
            if observed != &accepted_composition_artifact {
                return Err(format!(
                    "{label} drifted from standalone direct accepted composition artifact"
                ));
            }
        }
        require_str(
            direct_manifest,
            "/digests/direct_theorem_boundary_report_digest",
            str_at(
                &accepted_composition_artifact,
                "/direct_theorem_boundary_report_digest",
            )?,
        )?;
        require_str(
            direct_manifest,
            "/digests/account_creation_proof_boundary_report_digest",
            str_at(
                &accepted_composition_artifact,
                "/account_creation_proof_boundary_report_digest",
            )?,
        )?;
        require_str(
            proof_manifest,
            "/digests/account_creation_proof_boundary_report_digest",
            str_at(
                &accepted_composition_artifact,
                "/account_creation_proof_boundary_report_digest",
            )?,
        )?;
        let payload_vectors =
            load_json("formal/kat/path2_account_stark_payload_kat_vectors_v1.json")?;
        require_hex(
            &payload_vectors,
            "/corpus/honest_payload/payload_digest",
            &evidence.selected_account_creation_payload_digest,
        )?;

        println!("Path 2 direct accepted composition evidence consistency check passed.");
        Ok(())
    }
}

#[cfg(feature = "account-creation-stark-verifier")]
fn main() -> Result<(), String> {
    enabled::run()
}
