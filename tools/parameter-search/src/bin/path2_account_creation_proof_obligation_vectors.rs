#![recursion_limit = "256"]

use descender_parameter_search::{
    manifest_vector_evidence::{
        path2_account_creation_accepted_proof_boundary_evidence_artifact_v1,
        path2_direct_accepted_composition_evidence_artifact_v1,
    },
    production_reference::{
        PROFILE, path2_account_creation_no_export_boundary_report_v1,
        path2_account_creation_no_export_boundary_report_v1_digest,
        path2_account_creation_proof_boundary_report_v1,
        path2_account_creation_proof_boundary_report_v1_digest,
        path2_account_stark_fri_qrom_residual_report_v1,
        path2_account_stark_fri_qrom_residual_report_v1_digest,
        path2_account_stark_payload_kat_boundary_report_v1,
        path2_account_stark_payload_kat_boundary_report_v1_digest,
        path2_direct_theorem_boundary_report_v1, path2_direct_theorem_boundary_report_v1_digest,
        validate_path2_account_creation_no_export_boundary_report_v1,
        validate_path2_account_creation_proof_boundary_report_v1,
        validate_path2_account_stark_fri_qrom_residual_report_v1,
        validate_path2_account_stark_payload_kat_boundary_report_v1,
        validate_path2_direct_theorem_boundary_report_v1,
    },
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("parameter-search crate lives under tools/")
        .to_path_buf()
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

fn manifest_body() -> Result<Value, String> {
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    let proof_digest = path2_account_creation_proof_boundary_report_v1_digest(&proof_report);
    if proof_digest != proof_report.digest {
        return Err("account-creation proof-boundary report digest mismatch".to_string());
    }

    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    if direct_digest != direct_report.digest {
        return Err("direct theorem boundary report digest mismatch".to_string());
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_report.digest {
        return Err("direct theorem report is not bound to proof-boundary report".to_string());
    }
    let accepted_proof_boundary_artifact =
        path2_account_creation_accepted_proof_boundary_evidence_artifact_v1()?;
    let direct_accepted_composition_artifact =
        path2_direct_accepted_composition_evidence_artifact_v1()?;

    let no_export_report = path2_account_creation_no_export_boundary_report_v1();
    validate_path2_account_creation_no_export_boundary_report_v1(&no_export_report)?;
    let no_export_digest =
        path2_account_creation_no_export_boundary_report_v1_digest(&no_export_report);
    if no_export_digest != no_export_report.digest {
        return Err("no-export boundary report digest mismatch".to_string());
    }
    if no_export_report.deployment_ready
        || no_export_report.simulator_theorem_instantiated
        || no_export_report.no_export_theorem_instantiated
    {
        return Err("no-export boundary unexpectedly claims theorem readiness".to_string());
    }
    let no_export_vectors_file =
        load_json("formal/kat/path2_account_creation_no_export_vectors_v1.json")?;
    let no_export_vectors = at(&no_export_vectors_file, "/corpus")?;
    let no_export_vectors_json = serde_json::to_string_pretty(no_export_vectors)
        .map_err(|err| format!("serialize no-export vector corpus body: {err}"))?;
    let no_export_vectors_digest = hex_encode(&Sha256::digest(no_export_vectors_json.as_bytes()));
    if str_at(&no_export_vectors_file, "/corpus_digest")? != no_export_vectors_digest {
        return Err("no-export vector corpus digest mismatch".to_string());
    }
    if str_at(no_export_vectors, "/schema")? != "Descender.Path2.AccountCreationNoExportVectors.v1"
        || str_at(no_export_vectors, "/status")? != "digest_pinned_no_export_audit_vectors"
        || str_at(no_export_vectors, "/boundary_report/digest")?
            != hex_encode(&no_export_report.digest)
        || u64_at(no_export_vectors, "/case_count")?
            != no_export_report.no_export_vector_case_ids.len() as u64
        || !bool_at(
            no_export_vectors,
            "/audit_summary/all_private_material_absent",
        )?
        || !bool_at(no_export_vectors, "/audit_summary/payload_denylist_absent")?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/simulator_denylist_absent",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/simulator_input_allowlist_predicate_instantiated",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_public_anchor_package_bound",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_public_anchor_simulation_boundary_bound",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_public_bundle_from_public_anchors_bound",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_simulator_input_from_public_anchors_bound",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_simulator_no_export_reduction_bound",
        )?
        || bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_simulatable_from_public_anchors",
        )?
        || bool_at(
            no_export_vectors,
            "/audit_summary/public_payload_exports_private_material",
        )?
        || bool_at(
            no_export_vectors,
            "/audit_summary/simulator_theorem_instantiated",
        )?
        || bool_at(
            no_export_vectors,
            "/audit_summary/no_export_theorem_instantiated",
        )?
        || bool_at(no_export_vectors, "/audit_summary/deployment_ready")?
    {
        return Err(
            "no-export vector corpus is not bound to the current no-export boundary".to_string(),
        );
    }

    let fri_qrom_report = path2_account_stark_fri_qrom_residual_report_v1();
    validate_path2_account_stark_fri_qrom_residual_report_v1(&fri_qrom_report)?;
    let fri_qrom_digest = path2_account_stark_fri_qrom_residual_report_v1_digest(&fri_qrom_report);
    if fri_qrom_digest != fri_qrom_report.digest {
        return Err("FRI/QROM residual report digest mismatch".to_string());
    }
    let fri_qrom_vectors_file =
        load_json("formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json")?;
    let fri_qrom_vectors = at(&fri_qrom_vectors_file, "/corpus")?;
    let fri_qrom_vectors_json = serde_json::to_string_pretty(fri_qrom_vectors)
        .map_err(|err| format!("serialize FRI/QROM domain vector corpus body: {err}"))?;
    let fri_qrom_vectors_digest = hex_encode(&Sha256::digest(fri_qrom_vectors_json.as_bytes()));
    if str_at(&fri_qrom_vectors_file, "/corpus_digest")? != fri_qrom_vectors_digest {
        return Err("FRI/QROM domain vector corpus digest mismatch".to_string());
    }
    if str_at(fri_qrom_vectors, "/schema")? != "Descender.Path2.AccountStarkFriQromDomainVectors.v1"
        || str_at(fri_qrom_vectors, "/status")? != "digest_pinned_fri_qrom_domain_audit_vectors"
        || str_at(fri_qrom_vectors, "/boundary_report/digest")?
            != hex_encode(&fri_qrom_report.digest)
        || u64_at(fri_qrom_vectors, "/case_count")?
            != fri_qrom_report.qrom_domain_vector_case_count as u64
        || !bool_at(fri_qrom_vectors, "/audit_summary/fri_schedule_pinned")?
        || !bool_at(fri_qrom_vectors, "/audit_summary/challenge_field_pinned")?
        || !bool_at(
            fri_qrom_vectors,
            "/audit_summary/checked_core_qrom_budget_pinned",
        )?
        || !bool_at(
            fri_qrom_vectors,
            "/audit_summary/account_stark_fri_qrom_transcript_reduction_bound",
        )?
        || bool_at(
            fri_qrom_vectors,
            "/audit_summary/account_stark_transcript_qrom_theorem_instantiated",
        )?
        || bool_at(
            fri_qrom_vectors,
            "/audit_summary/recursion_pricing_complete",
        )?
        || bool_at(fri_qrom_vectors, "/audit_summary/residual_pricing_complete")?
        || bool_at(fri_qrom_vectors, "/audit_summary/deployment_ready")?
    {
        return Err(
            "FRI/QROM domain vector corpus is not bound to the current residual report".to_string(),
        );
    }

    let payload_kat_report = path2_account_stark_payload_kat_boundary_report_v1();
    validate_path2_account_stark_payload_kat_boundary_report_v1(&payload_kat_report)?;
    let payload_kat_digest =
        path2_account_stark_payload_kat_boundary_report_v1_digest(&payload_kat_report);
    if payload_kat_digest != payload_kat_report.digest {
        return Err("payload KAT boundary report digest mismatch".to_string());
    }
    if payload_kat_report.deployment_ready || !payload_kat_report.frozen_corpus_complete {
        return Err(
            "payload KAT boundary must be frozen while deployment remains closed".to_string(),
        );
    }
    if payload_kat_report.payload_kat_corpus_reduction_lean_theorem
        != "selectedRustAcceptedBoundaryPayloadKatCorpusReduction_v1"
        || !payload_kat_report.payload_kat_corpus_reduction_bound
    {
        return Err("payload KAT boundary lost Lean reduction binding".to_string());
    }
    let payload_vectors_file =
        load_json("formal/kat/path2_account_stark_payload_kat_vectors_v1.json")?;
    let payload_vectors = at(&payload_vectors_file, "/corpus")?;
    let payload_vectors_json = serde_json::to_string_pretty(payload_vectors)
        .map_err(|err| format!("serialize payload KAT vector corpus body: {err}"))?;
    let payload_vectors_digest = hex_encode(&Sha256::digest(payload_vectors_json.as_bytes()));
    if str_at(&payload_vectors_file, "/vector_corpus_digest")? != payload_vectors_digest {
        return Err("payload KAT vector corpus digest mismatch".to_string());
    }
    if str_at(payload_vectors, "/schema")? != "Descender.Path2.AccountStarkPayloadKatVectors.v1"
        || str_at(payload_vectors, "/status")? != "digest_pinned_boundary_vectors"
        || str_at(payload_vectors, "/boundary_report/digest")?
            != hex_encode(&payload_kat_report.digest)
        || u64_at(payload_vectors, "/boundary_report/row_count")?
            != payload_kat_report.rows.len() as u64
        || u64_at(payload_vectors, "/rejection_case_count")? != payload_kat_report.rows.len() as u64
        || u64_at(payload_vectors, "/case_count")? != payload_kat_report.rows.len() as u64 + 1
        || !bool_at(
            payload_vectors,
            "/boundary_report/every_current_row_has_generated_vector",
        )?
    {
        return Err(
            "payload KAT vector corpus is not bound to the current boundary rows".to_string(),
        );
    }

    let cases: Vec<Value> = proof_report
        .proof_obligation_rows
        .iter()
        .map(|row| {
            json!({
                "case_id": row.obligation_id,
                "kind": "audit",
                "closure_target": row.closure_target,
                "supporting_surface": row.supporting_surface,
                "current_evidence": row.current_evidence,
                "required_closure": row.required_closure,
                "named_break": row.named_break,
                "blocker": row.blocker,
                "open": row.open
            })
        })
        .collect();

    if cases.len() != proof_report.proof_obligation_audit_vector_case_ids.len() {
        return Err("proof-obligation audit vector generator case count drifted".to_string());
    }
    for (case, expected) in cases
        .iter()
        .zip(proof_report.proof_obligation_audit_vector_case_ids.iter())
    {
        if case["case_id"] != *expected {
            return Err(format!(
                "proof-obligation audit vector case id drift: expected {expected}"
            ));
        }
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationProofObligationVectors.v1",
        "corpus_id": "Descender.Path2.AccountCreationProofObligationAuditVectors.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_proof_obligation_audit_vectors",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_proof_obligation_vectors",
            "source": "tools/parameter-search/src/bin/path2_account_creation_proof_obligation_vectors.rs"
        },
        "proof_boundary_report": {
            "domain": proof_report.domain,
            "digest": hex_encode(&proof_report.digest),
            "recomputed_digest": hex_encode(&proof_digest),
            "proof_boundary_target": proof_report.proof_boundary_target,
            "proof_boundary_conclusion": proof_report.proof_boundary_conclusion,
            "accepted_boundary_evidence_schema":
                proof_report.accepted_boundary_evidence_schema,
            "accepted_boundary_evidence_endpoint":
                proof_report.accepted_boundary_evidence_endpoint,
            "accepted_boundary_evidence_present":
                proof_report.accepted_boundary_evidence_present,
            "accepted_boundary_binding_evidence_present":
                proof_report.accepted_boundary_binding_evidence_present,
            "replay_payload_digest_binding_evidence_present":
                proof_report.replay_payload_digest_binding_evidence_present,
            "deployed_anchor_binding_evidence_present":
                proof_report.deployed_anchor_binding_evidence_present,
            "proof_obligation_rows_pinned": proof_report.proof_obligation_rows_pinned,
            "every_proof_obligation_has_named_break":
                proof_report.every_proof_obligation_has_named_break,
            "every_open_proof_obligation_blocks_deployment":
                proof_report.every_open_proof_obligation_blocks_deployment,
            "closure_dependency_count": proof_report.closure_dependency_count,
            "closure_dependency_rows": proof_report.closure_dependency_rows,
            "every_closure_dependency_has_open_obligation":
                proof_report.every_closure_dependency_has_open_obligation,
            "every_closure_dependency_has_open_support_surface":
                proof_report.every_closure_dependency_has_open_support_surface,
            "every_open_closure_dependency_blocks_deployment":
                proof_report.every_open_closure_dependency_blocks_deployment,
            "deployment_ready": proof_report.deployment_ready,
            "blocker": proof_report.blocker
        },
        "accepted_proof_boundary_evidence_artifact": accepted_proof_boundary_artifact,
        "direct_accepted_composition_evidence_artifact":
            direct_accepted_composition_artifact,
        "no_export_vector_evidence": {
            "corpus_path": "formal/kat/path2_account_creation_no_export_vectors_v1.json",
            "corpus_schema": str_at(no_export_vectors, "/schema")?,
            "corpus_status": str_at(no_export_vectors, "/status")?,
            "corpus_digest": str_at(&no_export_vectors_file, "/corpus_digest")?,
            "corpus_recomputed_digest": no_export_vectors_digest,
            "boundary_report_digest": str_at(no_export_vectors, "/boundary_report/digest")?,
            "boundary_report_recomputed_digest": hex_encode(&no_export_digest),
            "case_count": u64_at(no_export_vectors, "/case_count")?,
            "all_private_material_absent": bool_at(
                no_export_vectors,
                "/audit_summary/all_private_material_absent"
            )?,
            "payload_denylist_absent": bool_at(
                no_export_vectors,
                "/audit_summary/payload_denylist_absent"
            )?,
            "simulator_denylist_absent": bool_at(
                no_export_vectors,
                "/audit_summary/simulator_denylist_absent"
            )?,
            "public_payload_exports_private_material": bool_at(
                no_export_vectors,
                "/audit_summary/public_payload_exports_private_material"
            )?,
            "simulator_theorem_instantiated":
                no_export_report.simulator_theorem_instantiated,
            "no_export_theorem_instantiated":
                no_export_report.no_export_theorem_instantiated,
            "public_bundle_no_export_lean_theorem":
                no_export_report.public_bundle_no_export_lean_theorem,
            "public_bundle_no_export_predicate_instantiated":
                no_export_report.public_bundle_no_export_predicate_instantiated,
            "simulator_input_allowlist_lean_theorem":
                no_export_report.simulator_input_allowlist_lean_theorem,
            "simulator_input_allowlist_predicate_instantiated":
                no_export_report.simulator_input_allowlist_predicate_instantiated,
            "accepted_public_anchor_package_lean_theorem":
                no_export_report.accepted_public_anchor_package_lean_theorem,
            "accepted_public_anchor_package_bound":
                no_export_report.accepted_public_anchor_package_bound,
            "accepted_bundle_public_anchor_simulation_boundary_lean_theorem":
                no_export_report
                    .accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
            "accepted_bundle_public_anchor_simulation_boundary_bound":
                no_export_report.accepted_bundle_public_anchor_simulation_boundary_bound,
            "accepted_bundle_public_bundle_from_public_anchors_lean_theorem":
                no_export_report
                    .accepted_bundle_public_bundle_from_public_anchors_lean_theorem,
            "accepted_bundle_public_bundle_from_public_anchors_bound":
                no_export_report.accepted_bundle_public_bundle_from_public_anchors_bound,
            "accepted_bundle_simulator_input_from_public_anchors_lean_theorem":
                no_export_report
                    .accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
            "accepted_bundle_simulator_input_from_public_anchors_bound":
                no_export_report.accepted_bundle_simulator_input_from_public_anchors_bound,
            "accepted_bundle_simulator_no_export_reduction_lean_theorem":
                no_export_report.accepted_bundle_simulator_no_export_reduction_lean_theorem,
            "accepted_bundle_simulator_no_export_reduction_bound":
                no_export_report.accepted_bundle_simulator_no_export_reduction_bound,
            "accepted_boundary_simulator_no_export_reduction_lean_theorem":
                no_export_report.accepted_boundary_simulator_no_export_reduction_lean_theorem,
            "accepted_boundary_simulator_no_export_reduction_bound":
                no_export_report.accepted_boundary_simulator_no_export_reduction_bound,
            "accepted_bundle_simulatable_from_public_anchors":
                no_export_report.accepted_bundle_simulatable_from_public_anchors,
            "frozen_no_export_vectors_complete":
                no_export_report.frozen_no_export_vectors_complete,
            "deployment_ready": no_export_report.deployment_ready,
            "blocker": no_export_report.blocker
        },
        "fri_qrom_domain_vector_evidence": {
            "corpus_path": "formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json",
            "corpus_schema": str_at(fri_qrom_vectors, "/schema")?,
            "corpus_status": str_at(fri_qrom_vectors, "/status")?,
            "corpus_digest": str_at(&fri_qrom_vectors_file, "/corpus_digest")?,
            "corpus_recomputed_digest": fri_qrom_vectors_digest,
            "boundary_report_digest": str_at(fri_qrom_vectors, "/boundary_report/digest")?,
            "boundary_report_recomputed_digest": hex_encode(&fri_qrom_digest),
            "case_count": u64_at(fri_qrom_vectors, "/case_count")?,
            "poseidon2_sponge_floor_bits": fri_qrom_report.poseidon2_sponge_floor_bits,
            "checked_core_qrom_selected_transcript_term_bits":
                fri_qrom_report.checked_core_qrom_selected_transcript_term_bits,
            "fri_schedule_pinned": bool_at(
                fri_qrom_vectors,
                "/audit_summary/fri_schedule_pinned"
            )?,
            "challenge_field_pinned": bool_at(
                fri_qrom_vectors,
                "/audit_summary/challenge_field_pinned"
            )?,
            "checked_core_qrom_budget_pinned": bool_at(
                fri_qrom_vectors,
                "/audit_summary/checked_core_qrom_budget_pinned"
            )?,
            "account_stark_fri_qrom_transcript_reduction_lean_theorem":
                fri_qrom_report
                    .account_stark_fri_qrom_transcript_reduction_lean_theorem,
            "account_stark_fri_qrom_transcript_reduction_bound":
                fri_qrom_report.account_stark_fri_qrom_transcript_reduction_bound,
            "account_stark_transcript_qrom_theorem_instantiated":
                fri_qrom_report.account_stark_transcript_qrom_theorem_instantiated,
            "recursion_pricing_complete": fri_qrom_report.recursion_pricing_complete,
            "residual_pricing_complete": fri_qrom_report.residual_pricing_complete,
            "deployment_ready": fri_qrom_report.deployment_ready,
            "blocker": fri_qrom_report.blocker
        },
        "payload_kat_vector_evidence": {
            "corpus_path": "formal/kat/path2_account_stark_payload_kat_vectors_v1.json",
            "corpus_schema": str_at(payload_vectors, "/schema")?,
            "corpus_status": str_at(payload_vectors, "/status")?,
            "corpus_digest": str_at(&payload_vectors_file, "/vector_corpus_digest")?,
            "corpus_recomputed_digest": payload_vectors_digest,
            "boundary_report_digest": str_at(payload_vectors, "/boundary_report/digest")?,
            "boundary_report_recomputed_digest": hex_encode(&payload_kat_digest),
            "boundary_report_row_count": payload_kat_report.rows.len(),
            "every_current_row_has_generated_vector": bool_at(
                payload_vectors,
                "/boundary_report/every_current_row_has_generated_vector"
            )?,
            "case_count": u64_at(payload_vectors, "/case_count")?,
            "rejection_case_count": u64_at(payload_vectors, "/rejection_case_count")?,
            "honest_payload_digest": str_at(payload_vectors, "/honest_payload/payload_digest")?,
            "payload_kat_corpus_reduction_lean_theorem":
                payload_kat_report.payload_kat_corpus_reduction_lean_theorem,
            "payload_kat_corpus_reduction_bound":
                payload_kat_report.payload_kat_corpus_reduction_bound,
            "frozen_corpus_complete": payload_kat_report.frozen_corpus_complete,
            "deployment_ready": payload_kat_report.deployment_ready,
            "blocker": payload_kat_report.blocker
        },
        "direct_theorem_report": {
            "domain": direct_report.domain,
            "digest": hex_encode(&direct_report.digest),
            "recomputed_digest": hex_encode(&direct_digest),
            "account_creation_proof_boundary_report_digest":
                hex_encode(&direct_report.account_creation_proof_boundary_report_digest),
            "accepted_tuple_evidence_schema":
                direct_report.accepted_tuple_evidence_schema,
            "accepted_tuple_evidence_endpoint":
                direct_report.accepted_tuple_evidence_endpoint,
            "accepted_tuple_evidence_present":
                direct_report.accepted_tuple_evidence_present,
            "accepted_composition_evidence_schema":
                direct_report.accepted_composition_evidence_schema,
            "accepted_composition_evidence_endpoint":
                direct_report.accepted_composition_evidence_endpoint,
            "accepted_composition_evidence_present":
                direct_report.accepted_composition_evidence_present,
            "composition_theorem_instantiated":
                direct_report.composition_theorem_instantiated,
            "account_creation_proof_boundary_verified":
                direct_report.account_creation_proof_boundary_verified,
            "deployment_ready": direct_report.deployment_ready,
            "blocker": direct_report.blocker
        },
        "audit_summary": {
            "proof_obligation_count": proof_report.proof_obligation_count,
            "open_proof_obligation_count":
                proof_report.proof_obligation_rows.iter().filter(|row| row.open).count(),
            "closed_proof_obligation_count":
                proof_report.proof_obligation_rows.iter().filter(|row| !row.open).count(),
            "proof_obligation_audit_vector_case_count":
                proof_report.proof_obligation_audit_vector_case_count,
            "proof_obligation_named_break_classes":
                proof_report.proof_obligation_named_break_classes,
            "all_cases_open": proof_report.proof_obligation_rows.iter().all(|row| row.open),
            "all_cases_have_named_break":
                proof_report.every_proof_obligation_has_named_break,
            "all_open_cases_block_deployment":
                proof_report.every_open_proof_obligation_blocks_deployment,
            "simulator_theorem_instantiated": proof_report.simulator_theorem_instantiated,
            "fri_aok_extractor_theorem_instantiated":
                proof_report.fri_aok_extractor_theorem_instantiated,
            "fri_qrom_residual_priced": proof_report.fri_qrom_residual_priced,
            "current_no_export_vectors_cover_boundary_cases": true,
            "current_fri_qrom_domain_vectors_cover_boundary_cases": true,
            "frozen_kat_corpus_complete": proof_report.frozen_kat_corpus_complete,
            "current_payload_kat_vectors_cover_every_boundary_row": bool_at(
                payload_vectors,
                "/boundary_report/every_current_row_has_generated_vector"
            )?,
            "external_review_complete": proof_report.external_review_complete,
            "production_gate_open": proof_report.production_gate_open,
            "deployment_ready": proof_report.deployment_ready
        },
        "case_count": cases.len(),
        "cases": cases
    }))
}

fn main() -> Result<(), String> {
    let corpus = manifest_body()?;
    let corpus_json = serde_json::to_string_pretty(&corpus)
        .map_err(|err| format!("failed to serialize proof-obligation corpus body: {err}"))?;
    let out = json!({
        "corpus_digest": hex_encode(&Sha256::digest(corpus_json.as_bytes())),
        "corpus": corpus
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out)
            .map_err(|err| format!("failed to serialize proof-obligation corpus: {err}"))?
    );
    Ok(())
}
