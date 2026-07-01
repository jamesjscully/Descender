#[cfg(not(feature = "account-creation-stark-verifier"))]
fn main() -> Result<(), String> {
    Err(
        "path2_direct_accepted_spend_tuple_evidence_consistency requires --features account-creation-stark-verifier"
            .to_string(),
    )
}

#[cfg(feature = "account-creation-stark-verifier")]
mod enabled {
    use descender_parameter_search::{
        manifest_vector_evidence::path2_direct_accepted_spend_tuple_evidence_artifact_v1,
        production_reference::{
            path2_direct_accepted_spend_tuple_evidence_v1_digest,
            path2_direct_accepted_spend_tuple_kat_evidence_v1,
            validate_path2_direct_accepted_spend_tuple_kat_evidence_v1,
        },
    };
    use serde_json::{Value, json, to_value};
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
                "direct accepted spend tuple manifest {pointer} mismatch: {observed} != {expected_hex}"
            ));
        }
        Ok(())
    }

    fn require_str(manifest: &Value, pointer: &str, expected: &str) -> Result<(), String> {
        let observed = str_at(manifest, pointer)?;
        if observed != expected {
            return Err(format!(
                "direct accepted spend tuple manifest {pointer} mismatch: {observed:?} != {expected:?}"
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
            load_json("formal/kat/path2_direct_accepted_spend_tuple_evidence_v1.json")?;
        let body = at(&manifest_file, "/manifest")?;
        let body_json = serde_json::to_string_pretty(body)
            .map_err(|err| format!("serialize direct accepted spend tuple body: {err}"))?;
        let recomputed_manifest_digest = hex_encode(&Sha256::digest(body_json.as_bytes()));
        if str_at(&manifest_file, "/manifest_digest")? != recomputed_manifest_digest {
            return Err("direct accepted spend tuple manifest digest mismatch".to_string());
        }
        if str_at(body, "/schema")?
            != "Descender.Path2.DirectAcceptedSpendTupleDeterministicKatEvidenceManifest.v1"
        {
            return Err("direct accepted spend tuple manifest schema mismatch".to_string());
        }
        if str_at(body, "/status")?
            != "digest_pinned_deterministic_kat_accepted_direct_spend_tuple_evidence"
        {
            return Err("direct accepted spend tuple manifest status mismatch".to_string());
        }
        if str_at(body, "/deterministic_tuple_mode/accepted_tuple_endpoint")?
            != "verify_path2_direct_single_slot_spend_authorization_with_tuple_evidence_v1"
        {
            return Err("direct accepted spend tuple endpoint mismatch".to_string());
        }
        if !bool_at(body, "/deterministic_tuple_mode/evidence_digest_recomputed")? {
            return Err(
                "direct accepted spend tuple manifest lost deterministic digest sanity check"
                    .to_string(),
            );
        }
        if u64_at(body, "/deterministic_tuple_mode/rayon_threads")? != 1 {
            return Err(
                "direct accepted spend tuple manifest is not in one-thread KAT mode".to_string(),
            );
        }

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .map_err(|err| {
                format!("failed to build deterministic single-thread KAT pool: {err}")
            })?;
        let kat = pool.install(path2_direct_accepted_spend_tuple_kat_evidence_v1)?;
        validate_path2_direct_accepted_spend_tuple_kat_evidence_v1(&kat)?;
        let evidence = kat.evidence;
        let evidence_digest = path2_direct_accepted_spend_tuple_evidence_v1_digest(&evidence);
        if evidence_digest != evidence.digest {
            return Err("direct accepted spend tuple evidence recompute mismatch".to_string());
        }

        require_hex(body, "/evidence/digest", &evidence.digest)?;
        require_hex(body, "/evidence/recomputed_digest", &evidence_digest)?;
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
            "/evidence/checked_core_signature_digest",
            &evidence.checked_core_signature_digest,
        )?;
        require_hex(
            body,
            "/evidence/checked_core_challenge_digest",
            &evidence.checked_core_challenge_digest,
        )?;
        require_hex(
            body,
            "/evidence/checked_core_proof_commitment_digest",
            &evidence.checked_core_proof_commitment_digest,
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
            "/evidence/direct_admission_namespace_record_digest",
            &evidence.direct_admission_namespace_record_digest,
        )?;
        require_hex(
            body,
            "/evidence/slot_authorization_digest",
            &evidence.slot_authorization_digest,
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
                != kat.account_creation_fixture_id
            || str_at(body, "/evidence/public_result_class_name")?
                != evidence.public_result_class_name
            || u64_at(body, "/evidence/proof_mask_seed")? != kat.proof_mask_seed
            || u64_at(body, "/evidence/slot_id")? != evidence.slot_id
            || u64_at(body, "/evidence/accepted_slot_id")? != u64::from(evidence.accepted_slot_id)
            || u64_at(body, "/evidence/accepted_tuple_field_count")?
                != evidence.accepted_tuple_fields.len() as u64
            || u64_at(body, "/evidence/acceptance_hypothesis_count")?
                != evidence.acceptance_hypotheses.len() as u64
            || at(body, "/evidence/accepted_tuple_fields")?
                != &json!(evidence.accepted_tuple_fields)
            || at(body, "/evidence/acceptance_hypotheses")?
                != &json!(evidence.acceptance_hypotheses)
        {
            return Err("direct accepted spend tuple manifest scalar field mismatch".to_string());
        }

        let expected_field_rows = to_value(&evidence.accepted_tuple_field_evidence_rows)
            .map_err(|err| format!("serialize accepted tuple field evidence rows: {err}"))?;
        if u64_at(body, "/evidence/accepted_tuple_field_evidence_count")?
            != evidence.accepted_tuple_field_evidence_count as u64
            || u64_at(
                body,
                "/evidence/accepted_tuple_field_evidence_present_count",
            )? != evidence.accepted_tuple_field_evidence_present_count as u64
            || u64_at(body, "/evidence/accepted_tuple_field_open_obligation_count")?
                != evidence.accepted_tuple_field_open_obligation_count as u64
            || u64_at(
                body,
                "/evidence/accepted_tuple_field_imported_closure_count",
            )? != evidence.accepted_tuple_field_imported_closure_count as u64
            || evidence.accepted_tuple_field_evidence_count != 11
            || evidence.accepted_tuple_field_evidence_present_count != 11
            || evidence.accepted_tuple_field_open_obligation_count != 6
            || evidence.accepted_tuple_field_imported_closure_count != 2
            || at(body, "/evidence/accepted_tuple_field_evidence_rows")? != &expected_field_rows
        {
            return Err(
                "direct accepted spend tuple manifest lost accepted-field evidence rows"
                    .to_string(),
            );
        }

        for pointer in [
            "/deterministic_tuple_mode/replay_rejected_by_consume_once",
            "/evidence/state_namespace_record_present",
            "/evidence/consumed_authorization_absent_before",
            "/evidence/consumed_authorization_present_after",
            "/evidence/consume_once_inserted_exactly_one_authorization",
            "/evidence/public_result_accepts_same_slot",
            "/evidence/tuple_fields_match_theorem_boundary",
            "/evidence/every_accepted_tuple_field_has_evidence",
            "/evidence/every_accepted_tuple_field_has_open_or_imported_direct_obligation",
            "/evidence/replay_rejected_by_consume_once",
        ] {
            if !bool_at(body, pointer)? {
                return Err(format!(
                    "direct accepted spend tuple manifest unexpectedly lost true flag {pointer}"
                ));
            }
        }
        if bool_at(
            body,
            "/evidence/every_accepted_tuple_field_has_open_direct_obligation",
        )? {
            return Err(
                "direct accepted spend tuple manifest unexpectedly kept all-open tuple-field flag"
                    .to_string(),
            );
        }
        if !kat.replay_rejected_by_consume_once {
            return Err(
                "direct accepted spend tuple KAT replay rejection helper returned false"
                    .to_string(),
            );
        }

        let payload_vectors =
            load_json("formal/kat/path2_account_stark_payload_kat_vectors_v1.json")?;
        require_hex(
            &payload_vectors,
            "/corpus/honest_payload/payload_digest",
            &evidence.selected_account_creation_payload_digest,
        )?;

        let accepted_artifact = path2_direct_accepted_spend_tuple_evidence_artifact_v1()?;
        let direct_manifest_file = load_json("formal/kat/path2_direct_theorem_boundary_v1.json")?;
        let direct_manifest =
            require_manifest_body(&direct_manifest_file, "direct theorem boundary manifest")?;
        let direct_boundary = at(direct_manifest, "/direct_theorem_boundary")?;
        let composition_manifest_file =
            load_json("formal/kat/path2_direct_accepted_composition_evidence_v1.json")?;
        let composition_manifest = require_manifest_body(
            &composition_manifest_file,
            "direct accepted composition manifest",
        )?;

        if at(
            direct_boundary,
            "/direct_accepted_spend_tuple_evidence_artifact",
        )? != &accepted_artifact
            || at(
                direct_boundary,
                "/direct_obligation_vector_corpus_evidence/direct_accepted_spend_tuple_evidence_artifact",
            )? != &accepted_artifact
            || at(
                direct_boundary,
                "/direct_accepted_composition_evidence_artifact/direct_accepted_spend_tuple_artifact",
            )? != &accepted_artifact
            || at(
                direct_boundary,
                "/direct_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/direct_accepted_spend_tuple_artifact",
            )? != &accepted_artifact
            || at(
                composition_manifest,
                "/accepted_input_artifacts/direct_accepted_spend_tuple_evidence_artifact",
            )? != &accepted_artifact
        {
            return Err(
                "direct accepted spend tuple artifact drifted across generated manifests"
                    .to_string(),
            );
        }
        require_str(
            direct_manifest,
            "/digests/direct_theorem_boundary_report_digest",
            str_at(&accepted_artifact, "/direct_theorem_boundary_report_digest")?,
        )?;
        require_str(
            direct_manifest,
            "/digests/direct_qrom_action_boundary_report_digest",
            str_at(
                &accepted_artifact,
                "/direct_qrom_action_boundary_report_digest",
            )?,
        )?;
        require_str(
            direct_manifest,
            "/digests/direct_lifecycle_boundary_report_digest",
            str_at(
                &accepted_artifact,
                "/direct_lifecycle_boundary_report_digest",
            )?,
        )?;
        require_str(
            direct_manifest,
            "/digests/account_creation_proof_boundary_report_digest",
            str_at(
                &accepted_artifact,
                "/account_creation_proof_boundary_report_digest",
            )?,
        )?;
        require_str(
            composition_manifest,
            "/evidence/direct_accepted_spend_tuple_evidence_digest",
            str_at(&accepted_artifact, "/evidence_digest")?,
        )?;
        for pointer in [
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/state_namespace_record_present",
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/consumed_authorization_absent_before",
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/consumed_authorization_present_after",
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/consume_once_inserted_exactly_one_authorization",
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/public_result_accepts_same_slot",
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/tuple_fields_match_theorem_boundary",
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/replay_rejected_by_consume_once",
            "/direct_accepted_composition_evidence_artifact/audit_summary/direct_spend_tuple_accepts",
            "/direct_accepted_composition_evidence_artifact/audit_summary/consume_once_transition_matches_tuple",
            "/direct_accepted_composition_evidence_artifact/audit_summary/public_result_accepts_same_slot",
        ] {
            if !bool_at(direct_boundary, pointer)? {
                return Err(format!(
                    "direct accepted spend tuple manifest bind lost expected true flag {pointer}"
                ));
            }
        }
        for pointer in [
            "/direct_accepted_composition_evidence_artifact/audit_summary/composition_theorem_instantiated",
            "/direct_accepted_composition_evidence_artifact/audit_summary/account_creation_proof_boundary_verified",
            "/direct_accepted_composition_evidence_artifact/audit_summary/deployment_ready",
        ] {
            if bool_at(direct_boundary, pointer)? {
                return Err(format!(
                    "direct accepted spend tuple manifest bind unexpectedly closed {pointer}"
                ));
            }
        }

        println!("Path 2 direct accepted spend tuple evidence consistency check passed.");
        Ok(())
    }
}

#[cfg(feature = "account-creation-stark-verifier")]
fn main() -> Result<(), String> {
    enabled::run()
}
