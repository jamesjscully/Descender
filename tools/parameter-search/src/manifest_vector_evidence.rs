use crate::production_reference::{
    path2_account_creation_frame_kat_boundary_report_v1,
    path2_account_creation_frame_kat_boundary_report_v1_digest,
    path2_account_creation_no_export_boundary_report_v1,
    path2_account_creation_no_export_boundary_report_v1_digest,
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    path2_account_stark_payload_kat_boundary_report_v1,
    path2_account_stark_payload_kat_boundary_report_v1_digest,
    path2_direct_lifecycle_boundary_report_v1, path2_direct_lifecycle_boundary_report_v1_digest,
    path2_direct_qrom_action_boundary_report_v1,
    path2_direct_qrom_action_boundary_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    path2_direct_theorem_boundary_report_v1_digest,
    validate_path2_account_creation_frame_kat_boundary_report_v1,
    validate_path2_account_creation_no_export_boundary_report_v1,
    validate_path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_stark_fri_qrom_residual_report_v1,
    validate_path2_account_stark_payload_kat_boundary_report_v1,
    validate_path2_direct_lifecycle_boundary_report_v1,
    validate_path2_direct_qrom_action_boundary_report_v1,
    validate_path2_direct_theorem_boundary_report_v1,
};
use serde_json::{json, Value};
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

fn body_digest(corpus: &Value, label: &str) -> Result<String, String> {
    let corpus_json =
        serde_json::to_string_pretty(corpus).map_err(|err| format!("serialize {label}: {err}"))?;
    Ok(hex_encode(&Sha256::digest(corpus_json.as_bytes())))
}

pub fn path2_account_creation_proof_obligation_vector_corpus_evidence_v1() -> Result<Value, String>
{
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
        return Err("direct theorem report lost proof-boundary digest binding".to_string());
    }

    let corpus_file =
        load_json("formal/kat/path2_account_creation_proof_obligation_vectors_v1.json")?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_digest = body_digest(corpus, "account-creation proof-obligation vector corpus")?;
    if str_at(&corpus_file, "/corpus_digest")? != corpus_digest {
        return Err("proof-obligation vector corpus digest mismatch".to_string());
    }
    if str_at(corpus, "/schema")? != "Descender.Path2.AccountCreationProofObligationVectors.v1"
        || str_at(corpus, "/status")? != "digest_pinned_proof_obligation_audit_vectors"
        || str_at(corpus, "/proof_boundary_report/digest")? != hex_encode(&proof_report.digest)
        || str_at(corpus, "/proof_boundary_report/recomputed_digest")? != hex_encode(&proof_digest)
        || str_at(corpus, "/direct_theorem_report/digest")? != hex_encode(&direct_report.digest)
        || str_at(corpus, "/direct_theorem_report/recomputed_digest")? != hex_encode(&direct_digest)
        || str_at(
            corpus,
            "/direct_theorem_report/account_creation_proof_boundary_report_digest",
        )? != hex_encode(&proof_report.digest)
        || u64_at(corpus, "/case_count")?
            != proof_report.proof_obligation_audit_vector_case_count as u64
        || u64_at(corpus, "/audit_summary/proof_obligation_count")?
            != proof_report.proof_obligation_count as u64
        || u64_at(
            corpus,
            "/audit_summary/proof_obligation_audit_vector_case_count",
        )? != proof_report.proof_obligation_audit_vector_case_count as u64
        || !bool_at(corpus, "/audit_summary/all_cases_open")?
        || !bool_at(corpus, "/audit_summary/all_cases_have_named_break")?
        || !bool_at(corpus, "/audit_summary/all_open_cases_block_deployment")?
        || bool_at(corpus, "/audit_summary/simulator_theorem_instantiated")?
        || bool_at(
            corpus,
            "/audit_summary/fri_aok_extractor_theorem_instantiated",
        )?
        || bool_at(corpus, "/audit_summary/fri_qrom_residual_priced")?
        || bool_at(corpus, "/audit_summary/frozen_kat_corpus_complete")?
        || bool_at(corpus, "/audit_summary/production_gate_open")?
        || bool_at(corpus, "/audit_summary/deployment_ready")?
    {
        return Err(
            "proof-obligation vector corpus is not bound to the current open proof boundary"
                .to_string(),
        );
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationProofObligationVectorCorpusEvidence.v1",
        "status": "digest_pinned_open_proof_obligation_vector_corpus_evidence",
        "corpus_path": "formal/kat/path2_account_creation_proof_obligation_vectors_v1.json",
        "corpus_schema": str_at(corpus, "/schema")?,
        "corpus_status": str_at(corpus, "/status")?,
        "corpus_digest": str_at(&corpus_file, "/corpus_digest")?,
        "corpus_recomputed_digest": corpus_digest,
        "proof_boundary_report_digest": str_at(corpus, "/proof_boundary_report/digest")?,
        "proof_boundary_report_recomputed_digest": hex_encode(&proof_digest),
        "direct_theorem_report_digest": str_at(corpus, "/direct_theorem_report/digest")?,
        "direct_theorem_report_recomputed_digest": hex_encode(&direct_digest),
        "case_count": u64_at(corpus, "/case_count")?,
        "audit_summary": {
            "proof_obligation_count": u64_at(
                corpus,
                "/audit_summary/proof_obligation_count"
            )?,
            "proof_obligation_audit_vector_case_count": u64_at(
                corpus,
                "/audit_summary/proof_obligation_audit_vector_case_count"
            )?,
            "all_cases_open": bool_at(corpus, "/audit_summary/all_cases_open")?,
            "all_cases_have_named_break": bool_at(
                corpus,
                "/audit_summary/all_cases_have_named_break"
            )?,
            "all_open_cases_block_deployment": bool_at(
                corpus,
                "/audit_summary/all_open_cases_block_deployment"
            )?,
            "simulator_theorem_instantiated": bool_at(
                corpus,
                "/audit_summary/simulator_theorem_instantiated"
            )?,
            "fri_aok_extractor_theorem_instantiated": bool_at(
                corpus,
                "/audit_summary/fri_aok_extractor_theorem_instantiated"
            )?,
            "fri_qrom_residual_priced": bool_at(
                corpus,
                "/audit_summary/fri_qrom_residual_priced"
            )?,
            "frozen_kat_corpus_complete": bool_at(
                corpus,
                "/audit_summary/frozen_kat_corpus_complete"
            )?,
            "production_gate_open": bool_at(
                corpus,
                "/audit_summary/production_gate_open"
            )?,
            "deployment_ready": bool_at(corpus, "/audit_summary/deployment_ready")?
        }
    }))
}

pub fn path2_direct_theorem_obligation_vector_corpus_evidence_v1() -> Result<Value, String> {
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    if direct_digest != direct_report.digest {
        return Err("direct theorem boundary report digest mismatch".to_string());
    }

    let qrom_action_report = path2_direct_qrom_action_boundary_report_v1();
    validate_path2_direct_qrom_action_boundary_report_v1(&qrom_action_report)?;
    let qrom_action_digest =
        path2_direct_qrom_action_boundary_report_v1_digest(&qrom_action_report);
    if qrom_action_digest != qrom_action_report.digest {
        return Err("direct QROM/action boundary report digest mismatch".to_string());
    }

    let lifecycle_report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&lifecycle_report)?;
    let lifecycle_digest = path2_direct_lifecycle_boundary_report_v1_digest(&lifecycle_report);
    if lifecycle_digest != lifecycle_report.digest {
        return Err("direct lifecycle boundary report digest mismatch".to_string());
    }

    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    let proof_digest = path2_account_creation_proof_boundary_report_v1_digest(&proof_report);
    if proof_digest != proof_report.digest {
        return Err("account-creation proof-boundary report digest mismatch".to_string());
    }
    if direct_report.direct_qrom_action_boundary_report_digest != qrom_action_report.digest
        || direct_report.direct_lifecycle_boundary_report_digest != lifecycle_report.digest
        || direct_report.account_creation_proof_boundary_report_digest != proof_report.digest
    {
        return Err("direct theorem report lost imported boundary digest binding".to_string());
    }

    let corpus_file = load_json("formal/kat/path2_direct_theorem_obligation_vectors_v1.json")?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_digest = body_digest(corpus, "direct theorem obligation vector corpus")?;
    if str_at(&corpus_file, "/corpus_digest")? != corpus_digest {
        return Err("direct theorem obligation vector corpus digest mismatch".to_string());
    }
    if str_at(corpus, "/schema")? != "Descender.Path2.DirectTheoremObligationVectors.v1"
        || str_at(corpus, "/status")? != "digest_pinned_direct_theorem_obligation_audit_vectors"
        || str_at(corpus, "/direct_theorem_report/digest")? != hex_encode(&direct_report.digest)
        || str_at(corpus, "/direct_theorem_report/recomputed_digest")? != hex_encode(&direct_digest)
        || str_at(corpus, "/qrom_action_boundary_report/digest")?
            != hex_encode(&qrom_action_report.digest)
        || str_at(corpus, "/qrom_action_boundary_report/recomputed_digest")?
            != hex_encode(&qrom_action_digest)
        || str_at(corpus, "/lifecycle_boundary_report/digest")?
            != hex_encode(&lifecycle_report.digest)
        || str_at(corpus, "/lifecycle_boundary_report/recomputed_digest")?
            != hex_encode(&lifecycle_digest)
        || str_at(corpus, "/account_creation_proof_boundary_report/digest")?
            != hex_encode(&proof_report.digest)
        || str_at(
            corpus,
            "/account_creation_proof_boundary_report/recomputed_digest",
        )? != hex_encode(&proof_digest)
        || u64_at(corpus, "/case_count")? != direct_report.obligation_count as u64
        || u64_at(corpus, "/audit_summary/direct_obligation_count")?
            != direct_report.obligation_count as u64
        || u64_at(
            corpus,
            "/audit_summary/direct_obligation_audit_vector_case_count",
        )? != direct_report.obligation_audit_vector_case_count as u64
        || !bool_at(corpus, "/audit_summary/all_cases_open")?
        || !bool_at(corpus, "/audit_summary/all_cases_have_named_break")?
        || !bool_at(corpus, "/audit_summary/all_open_cases_block_deployment")?
        || bool_at(corpus, "/audit_summary/composition_theorem_instantiated")?
        || bool_at(
            corpus,
            "/audit_summary/account_creation_proof_boundary_verified",
        )?
        || bool_at(
            corpus,
            "/audit_summary/lifecycle_transition_profile_verified",
        )?
        || bool_at(
            corpus,
            "/audit_summary/qrom_and_action_bounds_linked_to_theorem",
        )?
        || bool_at(corpus, "/audit_summary/deployment_ready")?
    {
        return Err(
            "direct theorem obligation vector corpus is not bound to the current open theorem"
                .to_string(),
        );
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectTheoremObligationVectorCorpusEvidence.v1",
        "status": "digest_pinned_open_direct_theorem_obligation_vector_corpus_evidence",
        "corpus_path": "formal/kat/path2_direct_theorem_obligation_vectors_v1.json",
        "corpus_schema": str_at(corpus, "/schema")?,
        "corpus_status": str_at(corpus, "/status")?,
        "corpus_digest": str_at(&corpus_file, "/corpus_digest")?,
        "corpus_recomputed_digest": corpus_digest,
        "direct_theorem_report_digest": str_at(corpus, "/direct_theorem_report/digest")?,
        "direct_theorem_report_recomputed_digest": hex_encode(&direct_digest),
        "direct_qrom_action_boundary_report_digest": str_at(
            corpus,
            "/qrom_action_boundary_report/digest"
        )?,
        "direct_qrom_action_boundary_report_recomputed_digest": hex_encode(&qrom_action_digest),
        "direct_lifecycle_boundary_report_digest": str_at(
            corpus,
            "/lifecycle_boundary_report/digest"
        )?,
        "direct_lifecycle_boundary_report_recomputed_digest": hex_encode(&lifecycle_digest),
        "account_creation_proof_boundary_report_digest": str_at(
            corpus,
            "/account_creation_proof_boundary_report/digest"
        )?,
        "account_creation_proof_boundary_report_recomputed_digest": hex_encode(&proof_digest),
        "case_count": u64_at(corpus, "/case_count")?,
        "audit_summary": {
            "direct_obligation_count": u64_at(
                corpus,
                "/audit_summary/direct_obligation_count"
            )?,
            "direct_obligation_audit_vector_case_count": u64_at(
                corpus,
                "/audit_summary/direct_obligation_audit_vector_case_count"
            )?,
            "all_cases_open": bool_at(corpus, "/audit_summary/all_cases_open")?,
            "all_cases_have_named_break": bool_at(
                corpus,
                "/audit_summary/all_cases_have_named_break"
            )?,
            "all_open_cases_block_deployment": bool_at(
                corpus,
                "/audit_summary/all_open_cases_block_deployment"
            )?,
            "composition_theorem_instantiated": bool_at(
                corpus,
                "/audit_summary/composition_theorem_instantiated"
            )?,
            "account_creation_proof_boundary_verified": bool_at(
                corpus,
                "/audit_summary/account_creation_proof_boundary_verified"
            )?,
            "lifecycle_transition_profile_verified": bool_at(
                corpus,
                "/audit_summary/lifecycle_transition_profile_verified"
            )?,
            "qrom_and_action_bounds_linked_to_theorem": bool_at(
                corpus,
                "/audit_summary/qrom_and_action_bounds_linked_to_theorem"
            )?,
            "deployment_ready": bool_at(corpus, "/audit_summary/deployment_ready")?
        }
    }))
}

pub fn path2_account_creation_requirement_vector_corpus_evidence_v1() -> Result<Value, String> {
    let frame_report = path2_account_creation_frame_kat_boundary_report_v1();
    validate_path2_account_creation_frame_kat_boundary_report_v1(&frame_report)?;
    let frame_digest = path2_account_creation_frame_kat_boundary_report_v1_digest(&frame_report);
    if frame_digest != frame_report.digest {
        return Err("frame KAT boundary report digest mismatch".to_string());
    }
    if frame_report.frozen_corpus_complete || frame_report.deployment_ready {
        return Err(
            "frame KAT boundary unexpectedly claims frozen/deployment readiness".to_string(),
        );
    }

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

    let fri_qrom_report = path2_account_stark_fri_qrom_residual_report_v1();
    validate_path2_account_stark_fri_qrom_residual_report_v1(&fri_qrom_report)?;
    let fri_qrom_digest = path2_account_stark_fri_qrom_residual_report_v1_digest(&fri_qrom_report);
    if fri_qrom_digest != fri_qrom_report.digest {
        return Err("FRI/QROM residual report digest mismatch".to_string());
    }
    if fri_qrom_report.deployment_ready
        || fri_qrom_report.account_stark_transcript_qrom_theorem_instantiated
        || fri_qrom_report.recursion_pricing_complete
        || fri_qrom_report.residual_pricing_complete
    {
        return Err("FRI/QROM residual boundary unexpectedly claims closure".to_string());
    }

    let payload_report = path2_account_stark_payload_kat_boundary_report_v1();
    validate_path2_account_stark_payload_kat_boundary_report_v1(&payload_report)?;
    let payload_digest = path2_account_stark_payload_kat_boundary_report_v1_digest(&payload_report);
    if payload_digest != payload_report.digest {
        return Err("payload KAT boundary report digest mismatch".to_string());
    }
    if payload_report.frozen_corpus_complete || payload_report.deployment_ready {
        return Err(
            "payload KAT boundary unexpectedly claims frozen/deployment readiness".to_string(),
        );
    }

    let frame_file = load_json("formal/kat/path2_account_creation_frame_kat_vectors_v1.json")?;
    let frame_vectors = at(&frame_file, "/corpus")?;
    let frame_corpus_digest = body_digest(frame_vectors, "frame KAT vector corpus body")?;
    if str_at(&frame_file, "/corpus_digest")? != frame_corpus_digest {
        return Err("frame KAT vector corpus digest mismatch".to_string());
    }
    if str_at(frame_vectors, "/schema")? != "Descender.Path2.AccountCreationFrameKatVectors.v1"
        || str_at(frame_vectors, "/status")? != "digest_pinned_boundary_vectors"
        || str_at(frame_vectors, "/boundary_report/digest")? != hex_encode(&frame_report.digest)
        || u64_at(frame_vectors, "/boundary_report/row_count")? != frame_report.rows.len() as u64
        || u64_at(frame_vectors, "/case_count")? != frame_report.rows.len() as u64 + 1
        || u64_at(frame_vectors, "/rejection_case_count")? != frame_report.rows.len() as u64
        || !bool_at(
            frame_vectors,
            "/boundary_report/every_current_row_has_generated_vector",
        )?
        || bool_at(frame_vectors, "/boundary_report/deployment_ready")?
    {
        return Err(
            "frame KAT vector corpus is not bound to the current boundary rows".to_string(),
        );
    }

    let no_export_file = load_json("formal/kat/path2_account_creation_no_export_vectors_v1.json")?;
    let no_export_vectors = at(&no_export_file, "/corpus")?;
    let no_export_corpus_digest = body_digest(no_export_vectors, "no-export vector corpus body")?;
    if str_at(&no_export_file, "/corpus_digest")? != no_export_corpus_digest {
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

    let fri_qrom_file =
        load_json("formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json")?;
    let fri_qrom_vectors = at(&fri_qrom_file, "/corpus")?;
    let fri_qrom_corpus_digest =
        body_digest(fri_qrom_vectors, "FRI/QROM domain vector corpus body")?;
    if str_at(&fri_qrom_file, "/corpus_digest")? != fri_qrom_corpus_digest {
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

    let payload_file = load_json("formal/kat/path2_account_stark_payload_kat_vectors_v1.json")?;
    let payload_vectors = at(&payload_file, "/corpus")?;
    let payload_corpus_digest = body_digest(payload_vectors, "payload KAT vector corpus body")?;
    if str_at(&payload_file, "/vector_corpus_digest")? != payload_corpus_digest {
        return Err("payload KAT vector corpus digest mismatch".to_string());
    }
    if str_at(payload_vectors, "/schema")? != "Descender.Path2.AccountStarkPayloadKatVectors.v1"
        || str_at(payload_vectors, "/status")? != "digest_pinned_boundary_vectors"
        || str_at(payload_vectors, "/boundary_report/digest")? != hex_encode(&payload_report.digest)
        || u64_at(payload_vectors, "/boundary_report/row_count")?
            != payload_report.rows.len() as u64
        || u64_at(payload_vectors, "/case_count")? != payload_report.rows.len() as u64 + 1
        || u64_at(payload_vectors, "/rejection_case_count")? != payload_report.rows.len() as u64
        || !bool_at(
            payload_vectors,
            "/boundary_report/every_current_row_has_generated_vector",
        )?
    {
        return Err(
            "payload KAT vector corpus is not bound to the current boundary rows".to_string(),
        );
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationRequirementVectorCorpusEvidence.v1",
        "status": "digest_pinned_requirement_vector_corpus_evidence",
        "frame_kat_vector_evidence": {
            "corpus_path": "formal/kat/path2_account_creation_frame_kat_vectors_v1.json",
            "corpus_schema": str_at(frame_vectors, "/schema")?,
            "corpus_status": str_at(frame_vectors, "/status")?,
            "corpus_digest": str_at(&frame_file, "/corpus_digest")?,
            "corpus_recomputed_digest": frame_corpus_digest,
            "boundary_report_digest": str_at(frame_vectors, "/boundary_report/digest")?,
            "boundary_report_recomputed_digest": hex_encode(&frame_digest),
            "boundary_report_row_count": frame_report.rows.len(),
            "every_current_row_has_generated_vector": bool_at(
                frame_vectors,
                "/boundary_report/every_current_row_has_generated_vector"
            )?,
            "case_count": u64_at(frame_vectors, "/case_count")?,
            "rejection_case_count": u64_at(frame_vectors, "/rejection_case_count")?,
            "frozen_corpus_complete": frame_report.frozen_corpus_complete,
            "deployment_ready": frame_report.deployment_ready,
            "blocker": frame_report.blocker
        },
        "no_export_vector_evidence": {
            "corpus_path": "formal/kat/path2_account_creation_no_export_vectors_v1.json",
            "corpus_schema": str_at(no_export_vectors, "/schema")?,
            "corpus_status": str_at(no_export_vectors, "/status")?,
            "corpus_digest": str_at(&no_export_file, "/corpus_digest")?,
            "corpus_recomputed_digest": no_export_corpus_digest,
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
            "simulator_theorem_instantiated": no_export_report.simulator_theorem_instantiated,
            "no_export_theorem_instantiated": no_export_report.no_export_theorem_instantiated,
            "deployment_ready": no_export_report.deployment_ready,
            "blocker": no_export_report.blocker
        },
        "fri_qrom_domain_vector_evidence": {
            "corpus_path": "formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json",
            "corpus_schema": str_at(fri_qrom_vectors, "/schema")?,
            "corpus_status": str_at(fri_qrom_vectors, "/status")?,
            "corpus_digest": str_at(&fri_qrom_file, "/corpus_digest")?,
            "corpus_recomputed_digest": fri_qrom_corpus_digest,
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
            "corpus_digest": str_at(&payload_file, "/vector_corpus_digest")?,
            "corpus_recomputed_digest": payload_corpus_digest,
            "boundary_report_digest": str_at(payload_vectors, "/boundary_report/digest")?,
            "boundary_report_recomputed_digest": hex_encode(&payload_digest),
            "boundary_report_row_count": payload_report.rows.len(),
            "every_current_row_has_generated_vector": bool_at(
                payload_vectors,
                "/boundary_report/every_current_row_has_generated_vector"
            )?,
            "case_count": u64_at(payload_vectors, "/case_count")?,
            "rejection_case_count": u64_at(payload_vectors, "/rejection_case_count")?,
            "honest_payload_digest": str_at(payload_vectors, "/honest_payload/payload_digest")?,
            "frozen_corpus_complete": payload_report.frozen_corpus_complete,
            "deployment_ready": payload_report.deployment_ready,
            "blocker": payload_report.blocker
        },
        "audit_summary": {
            "all_requirement_vector_corpora_bound": true,
            "frame_kat_vectors_cover_current_rows": true,
            "no_export_vectors_cover_boundary_cases": true,
            "fri_qrom_domain_vectors_cover_boundary_cases": true,
            "payload_kat_vectors_cover_current_rows": true,
            "any_requirement_vector_corpus_claims_theorem_ready": false,
            "any_requirement_vector_corpus_claims_deployment_ready": false,
            "frame_kat_case_count": u64_at(frame_vectors, "/case_count")?,
            "no_export_case_count": u64_at(no_export_vectors, "/case_count")?,
            "fri_qrom_domain_case_count": u64_at(fri_qrom_vectors, "/case_count")?,
            "payload_kat_case_count": u64_at(payload_vectors, "/case_count")?
        }
    }))
}
