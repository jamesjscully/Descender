#[cfg(not(feature = "account-creation-stark-verifier"))]
fn main() -> Result<(), String> {
    Err(
        "path2_direct_accepted_composition_evidence_consistency requires --features account-creation-stark-verifier"
            .to_string(),
    )
}

#[cfg(feature = "account-creation-stark-verifier")]
mod enabled {
    use descender_parameter_search::production_reference::{
        path2_direct_accepted_composition_evidence_v1,
        path2_direct_accepted_composition_evidence_v1_digest,
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
            != "Descender.Path2.DirectAcceptedCompositionStableSurfaceManifest.v1"
        {
            return Err("direct accepted composition manifest schema mismatch".to_string());
        }
        if str_at(body, "/status")?
            != "stable_public_surface_pinned_accepted_direct_composition_evidence"
        {
            return Err("direct accepted composition manifest status mismatch".to_string());
        }
        if !bool_at(
            body,
            "/current_run_checks/evidence_digest_recomputed_before_surface_projection",
        )? {
            return Err(
                "direct accepted composition manifest lost current-run digest sanity check"
                    .to_string(),
            );
        }

        let evidence = path2_direct_accepted_composition_evidence_v1()?;
        let evidence_digest = path2_direct_accepted_composition_evidence_v1_digest(&evidence);
        if evidence_digest != evidence.digest {
            return Err("direct accepted composition evidence recompute mismatch".to_string());
        }

        require_hex(
            body,
            "/stable_surface/direct_theorem_boundary_report_digest",
            &evidence.direct_theorem_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/stable_surface/direct_qrom_action_boundary_report_digest",
            &evidence.direct_qrom_action_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/stable_surface/direct_lifecycle_boundary_report_digest",
            &evidence.direct_lifecycle_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/stable_surface/account_creation_proof_boundary_report_digest",
            &evidence.account_creation_proof_boundary_report_digest,
        )?;
        require_hex(
            body,
            "/stable_surface/public_key_identity",
            &evidence.public_key_identity,
        )?;
        require_hex(
            body,
            "/stable_surface/message_digest",
            &evidence.message_digest,
        )?;
        require_hex(body, "/stable_surface/account_id", &evidence.account_id)?;
        require_hex(
            body,
            "/stable_surface/account_state_root_before",
            &evidence.account_state_root_before,
        )?;
        if str_at(body, "/stable_surface/evidence_schema")? != evidence.schema
            || str_at(body, "/stable_surface/theorem_target")? != evidence.theorem_target
            || str_at(body, "/stable_surface/theorem_conclusion")? != evidence.theorem_conclusion
            || str_at(body, "/stable_surface/account_creation_fixture_id")?
                != evidence.account_creation_fixture_id
            || u64_at(body, "/stable_surface/proof_mask_seed")? != evidence.proof_mask_seed
            || u64_at(body, "/stable_surface/slot_id")? != evidence.slot_id
        {
            return Err("direct accepted composition manifest scalar field mismatch".to_string());
        }

        for pointer in [
            "/stable_surface/creation_evidence_matches_direct_tuple",
            "/stable_surface/proof_boundary_digest_matches_direct_theorem",
            "/stable_surface/qrom_action_digest_matches_direct_theorem",
            "/stable_surface/lifecycle_digest_matches_direct_theorem",
            "/stable_surface/direct_admission_namespace_matches_authorization",
            "/stable_surface/consume_once_transition_matches_tuple",
            "/stable_surface/verifier_boundary_accepts_creation",
            "/stable_surface/direct_spend_tuple_accepts",
            "/stable_surface/public_result_accepts_same_slot",
            "/stable_surface/direct_theorem_obligations_still_open",
            "/stable_surface/proof_family_obligations_still_open",
        ] {
            if !bool_at(body, pointer)? {
                return Err(format!(
                    "direct accepted composition manifest unexpectedly lost true flag {pointer}"
                ));
            }
        }
        for pointer in [
            "/stable_surface/composition_theorem_instantiated",
            "/stable_surface/account_creation_proof_boundary_verified",
            "/stable_surface/deployment_ready",
        ] {
            if bool_at(body, pointer)? {
                return Err(format!(
                    "direct accepted composition manifest unexpectedly claims closure at {pointer}"
                ));
            }
        }
        for pointer in [
            "/stable_surface/digest",
            "/stable_surface/direct_accepted_spend_tuple_evidence_digest",
            "/stable_surface/account_creation_accepted_proof_boundary_evidence_digest",
            "/stable_surface/prevalidated_slot_leaf_digest",
            "/stable_surface/account_creation_evidence_transcript_digest",
            "/stable_surface/selected_account_creation_payload_digest",
            "/stable_surface/slot_authorization_digest",
            "/stable_surface/direct_admission_namespace_record_digest",
            "/stable_surface/consume_once_state_before_digest",
            "/stable_surface/consume_once_state_after_digest",
        ] {
            if body.pointer(pointer).is_some() {
                return Err(format!(
                    "direct accepted composition manifest pins non-frozen field {pointer}"
                ));
            }
        }

        println!("Path 2 direct accepted composition evidence consistency check passed.");
        Ok(())
    }
}

#[cfg(feature = "account-creation-stark-verifier")]
fn main() -> Result<(), String> {
    enabled::run()
}
